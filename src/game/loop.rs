use crate::game::{GameState, Screen, GameDate};
use crate::team::{League, MatchResult, MatchMode};
use crate::ai::MatchSimulator;
use crate::data::{Database, SaveManager, ScheduledMatchData, ScheduledMatchRepository, PlayerRepository, MatchRepository, TeamStatisticsRepository, TeamRepository};
use std::time::Duration;
use std::thread;

/// Game loop error types
#[derive(Debug, thiserror::Error)]
pub enum GameLoopError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::data::DatabaseError),

    #[error("No match scheduled")]
    NoMatchScheduled,

    #[error("Invalid game state")]
    InvalidState,
}

/// Main game loop controller
pub struct GameLoop {
    game_state: GameState,
    database: Database,
    save_manager: SaveManager,
    auto_save_interval: u32, // Auto-save every N matches
    matches_since_save: u32,
}

impl GameLoop {
    /// Create new game loop
    pub fn new(game_state: GameState, database: Database) -> Result<Self, GameLoopError> {
        let save_manager = SaveManager::default();

        Ok(Self {
            game_state,
            database,
            save_manager,
            auto_save_interval: 5, // Auto-save every 5 matches
            matches_since_save: 0,
        })
    }

    /// Run the main game loop
    pub fn run(&mut self) -> Result<(), GameLoopError> {
        while self.game_state.current_screen != Screen::MainMenu || self.game_state.is_new_game {
            // Check if we need to advance to next match
            if self.should_advance_to_match() {
                self.advance_to_next_match()?;
            }

            // Auto-save if needed
            if self.matches_since_save >= self.auto_save_interval {
                self.auto_save()?;
                self.matches_since_save = 0;
            }

            // Check for season end
            if self.game_state.current_date.is_season_end() {
                self.handle_season_end()?;
            }

            // Small delay to prevent tight loop
            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    /// Check if should advance to next match
    fn should_advance_to_match(&self) -> bool {
        matches!(self.game_state.current_screen,
            Screen::MatchModeSelection |
            Screen::MatchLive { .. } |
            Screen::MatchResult { .. })
    }

    /// Advance to next match day
    pub fn advance_to_next_match(&mut self) -> Result<(), GameLoopError> {
        // Find next scheduled match for player's team
        let scheduled_match = self.find_next_match()?
            .ok_or(GameLoopError::NoMatchScheduled)?;

        // Advance date to match day
        while !self.is_match_day(&scheduled_match) {
            self.game_state.advance_time();
        }

        // Navigate to match mode selection
        self.game_state.navigate_to(Screen::MatchModeSelection);

        Ok(())
    }

    /// Find next scheduled match for player team
    fn find_next_match(&self) -> Result<Option<ScheduledMatchData>, GameLoopError> {
        let scheduled_match_repo = self.database.scheduled_match_repo();
        let league_id = &self.game_state.league.id;

        let all_matches = scheduled_match_repo.get_by_league(league_id)?;

        // Find first unplayed match involving player team
        let next_match = all_matches.into_iter()
            .filter(|m| !m.played)
            .find(|m| {
                m.home_team_id == self.game_state.player_team_id ||
                m.away_team_id == self.game_state.player_team_id
            });

        Ok(next_match)
    }

    /// Check if current date is match day
    fn is_match_day(&self, _scheduled_match: &ScheduledMatchData) -> bool {
        // Simplified: assume match is on current day
        true
    }

    /// Play match with selected mode
    pub fn play_match(&mut self, match_id: String, mode: MatchMode) -> Result<MatchResult, GameLoopError> {
        // Get match details
        let scheduled_match = self.database.scheduled_match_repo().get_by_id(&match_id)?;

        // Get teams
        let home_team = self.game_state.teams.get(&scheduled_match.home_team_id)
            .ok_or(GameLoopError::InvalidState)?
            .clone();
        let away_team = self.game_state.teams.get(&scheduled_match.away_team_id)
            .ok_or(GameLoopError::InvalidState)?
            .clone();

        // Get players from both teams from database
        let home_players = self.database.player_repo().get_by_team(&home_team.id)?;
        let away_players = self.database.player_repo().get_by_team(&away_team.id)?;

        // Simulate match
        let mut simulator = MatchSimulator::new();
        let match_result = simulator.simulate_match(&home_team, &away_team, &home_players, &away_players, mode);

        // Save match result
        let match_repo = self.database.match_repo();
        match_repo.save(&match_result)?;

        // Mark scheduled match as played
        self.database.scheduled_match_repo().mark_as_played(&match_id)?;

        // Update after match
        self.update_after_match(&match_result)?;

        // Increment matches counter
        self.matches_since_save += 1;

        Ok(match_result)
    }

    /// Update game state after match
    fn update_after_match(&mut self, match_result: &MatchResult) -> Result<(), GameLoopError> {
        // Update team statistics
        self.update_team_statistics(match_result)?;

        // Update player fatigue and injuries
        self.update_player_status_after_match(match_result)?;

        // Check for season end
        if self.is_all_matches_played()? {
            self.handle_season_end()?;
        }

        Ok(())
    }

    /// Update team statistics after match
    fn update_team_statistics(&mut self, match_result: &MatchResult) -> Result<(), GameLoopError> {
        let stats_repo = self.database.team_statistics_repo();

        // Update home team stats
        let mut home_stats = stats_repo.get_by_team(&match_result.home_team_id)?;
        home_stats.matches_played += 1;
        home_stats.goals_for += match_result.home_score as u32;
        home_stats.goals_against += match_result.away_score as u32;

        if match_result.home_score > match_result.away_score {
            home_stats.wins += 1;
            home_stats.points += 3;
        } else if match_result.home_score == match_result.away_score {
            home_stats.draws += 1;
            home_stats.points += 1;
        } else {
            home_stats.losses += 1;
        }

        stats_repo.update(&match_result.home_team_id, &home_stats)?;

        // Update away team stats
        let mut away_stats = stats_repo.get_by_team(&match_result.away_team_id)?;
        away_stats.matches_played += 1;
        away_stats.goals_for += match_result.away_score as u32;
        away_stats.goals_against += match_result.home_score as u32;

        if match_result.away_score > match_result.home_score {
            away_stats.wins += 1;
            away_stats.points += 3;
        } else if match_result.away_score == match_result.home_score {
            away_stats.draws += 1;
            away_stats.points += 1;
        } else {
            away_stats.losses += 1;
        }

        stats_repo.update(&match_result.away_team_id, &away_stats)?;

        Ok(())
    }

    /// Update player fatigue and injuries after match
    fn update_player_status_after_match(&mut self, match_result: &MatchResult) -> Result<(), GameLoopError> {
        use crate::ai::progression;

        // Get players from both teams
        let mut home_players = self.database.player_repo().get_by_team(&match_result.home_team_id)?;
        let mut away_players = self.database.player_repo().get_by_team(&match_result.away_team_id)?;

        // Collect player IDs and minutes played (simplified - all 90 mins)
        let mut player_minutes: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        for player in &home_players {
            player_minutes.insert(player.id.clone(), 90);
        }
        for player in &away_players {
            player_minutes.insert(player.id.clone(), 90);
        }

        // Update home players
        let home_updates = progression::update_players_after_match(&mut home_players, &player_minutes);
        for player in &home_players {
            self.database.player_repo().update(player)?;
        }

        // Update away players
        let away_updates = progression::update_players_after_match(&mut away_players, &player_minutes);

        // Create notifications for injuries and other significant updates
        for update in home_updates.iter().chain(away_updates.iter()) {
            match &update.update_type {
                crate::ai::PlayerUpdateType::Injured(days) => {
                    if let Some(player) = home_players.iter().chain(away_players.iter())
                        .find(|p| p.id == update.player_id) {
                        self.game_state.add_notification(
                            "Injury".to_string(),
                            format!("{} has been injured for {} days!", player.name, days),
                            crate::game::NotificationType::Injury,
                            crate::game::NotificationPriority::High,
                        );
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Check if all matches in season are played
    fn is_all_matches_played(&self) -> Result<bool, GameLoopError> {
        let scheduled_matches = self.database.scheduled_match_repo()
            .get_by_league(&self.game_state.league.id)?;

        let all_played = scheduled_matches.iter().all(|m| m.played);
        Ok(all_played)
    }

    /// Handle season end
    fn handle_season_end(&mut self) -> Result<(), GameLoopError> {
        // Age all players
        self.age_all_players()?;

        // Generate new season schedule
        let new_league = self.generate_new_season_schedule()?;

        // Reset statistics
        self.reset_season_statistics()?;

        // Update game date to new season start
        let new_year = self.game_state.current_date.year + 1;
        self.game_state.current_date = GameDate::new_season_start(new_year);

        // Add notification
        self.game_state.add_notification(
            "Season End".to_string(),
            format!("Season {} has ended. New season starting!", self.game_state.current_date.season_string()),
            crate::game::NotificationType::Achievement,
            crate::game::NotificationPriority::High,
        );

        Ok(())
    }

    /// Age all players by one year
    fn age_all_players(&mut self) -> Result<(), GameLoopError> {
        use crate::ai::progression;

        // Get all teams in the league
        let teams = self.database.team_repo().get_all()?;

        // Collect all players from all teams
        let mut all_players = Vec::new();
        for team in &teams {
            let mut team_players = self.database.player_repo().get_by_team(&team.id)?;
            all_players.append(&mut team_players);
        }

        // Age each player and save back to database
        let updates = progression::age_players(&mut all_players);

        // Process retirement updates and filter out retired players
        let mut retired_players = Vec::new();
        for (player, update) in all_players.iter().zip(updates.iter()) {
            self.database.player_repo().update(player)?;

            // Track retired players
            if updates.iter().any(|u| u.player_id == player.id && matches!(u.update_type, crate::ai::PlayerUpdateType::Retired)) {
                retired_players.push(player.id.clone());
            }
        }

        // Create retirement notifications
        for player_id in retired_players {
            if let Some(player) = all_players.iter().find(|p| p.id == player_id) {
                self.game_state.add_notification(
                    "Retirement".to_string(),
                    format!("{} has retired from football at age {}", player.name, player.age),
                    crate::game::NotificationType::News,
                    crate::game::NotificationPriority::Normal,
                );
            }
        }

        Ok(())
    }

    /// Generate new season schedule
    fn generate_new_season_schedule(&mut self) -> Result<League, GameLoopError> {
        // Clear old scheduled matches
        self.database.scheduled_match_repo()
            .delete_by_league(&self.game_state.league.id)?;

        // Reset league schedule
        self.game_state.league.new_season();

        // Generate new schedule
        self.game_state.league.generate_schedule();

        // Save new scheduled matches to database
        for round in &self.game_state.league.schedule.rounds {
            for scheduled_match in &round.matches {
                let scheduled_match_data = ScheduledMatchData {
                    id: scheduled_match.id.clone(),
                    league_id: self.game_state.league.id.clone(),
                    round_number: round.round_number,
                    home_team_id: scheduled_match.home_team_id.clone(),
                    away_team_id: scheduled_match.away_team_id.clone(),
                    played: scheduled_match.played,
                };

                self.database.scheduled_match_repo().create(&scheduled_match_data)?;
            }
        }

        Ok(self.game_state.league.clone())
    }

    /// Reset season statistics
    fn reset_season_statistics(&mut self) -> Result<(), GameLoopError> {
        let stats_repo = self.database.team_statistics_repo();

        for team_id in &self.game_state.league.teams {
            let mut stats = stats_repo.get_by_team(team_id)?;
            stats.matches_played = 0;
            stats.wins = 0;
            stats.draws = 0;
            stats.losses = 0;
            stats.goals_for = 0;
            stats.goals_against = 0;
            stats.points = 0;
            stats.league_position = None;

            stats_repo.update(team_id, &stats)?;
        }

        Ok(())
    }

    /// Auto-save game
    fn auto_save(&mut self) -> Result<(), GameLoopError> {
        let autosave_slot = crate::data::SaveManager::AUTOSAVE_SLOT;
        self.save_manager.save_game(
            autosave_slot,
            &self.game_state,
            &self.database,
        )?;

        self.game_state.add_notification(
            "Auto-saved".to_string(),
            format!("Game auto-saved to slot {}", autosave_slot),
            crate::game::NotificationType::System,
            crate::game::NotificationPriority::Low,
        );

        Ok(())
    }

    /// Manual save game
    pub fn save_game(&mut self, slot: u8) -> Result<(), GameLoopError> {
        self.save_manager.save_game(
            slot,
            &self.game_state,
            &self.database,
        )?;

        self.game_state.add_notification(
            "Game Saved".to_string(),
            format!("Game saved to slot {}", slot),
            crate::game::NotificationType::System,
            crate::game::NotificationPriority::Normal,
        );

        Ok(())
    }

    /// Load game
    pub fn load_game(&mut self, slot: u8) -> Result<(), GameLoopError> {
        let loaded_state = self.save_manager.load_game(slot)?;

        // Update database path
        let db_path = loaded_state.db_path.clone();
        self.database = Database::new(&db_path)?;
        self.game_state = loaded_state;

        Ok(())
    }

    /// Get current game state
    pub fn game_state(&self) -> &GameState {
        &self.game_state
    }

    /// Get mutable game state
    pub fn game_state_mut(&mut self) -> &mut GameState {
        &mut self.game_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Team;

    #[test]
    fn test_game_loop_creation() {
        let league = League::new("test_league".to_string(), "Test League".to_string(), vec![]);
        let team = Team::new("team1".to_string(), "Test Team".to_string(), "test_league".to_string(), 1000000);
        let game_state = GameState::new("team1".to_string(), league, vec![team], 2026);
        let db = Database::in_memory().unwrap();

        let game_loop = GameLoop::new(game_state, db);
        assert!(game_loop.is_ok());
    }

    #[test]
    fn test_auto_save_interval() {
        let league = League::new("test_league".to_string(), "Test League".to_string(), vec![]);
        let team = Team::new("team1".to_string(), "Test Team".to_string(), "test_league".to_string(), 1000000);
        let game_state = GameState::new("team1".to_string(), league, vec![team], 2026);
        let db = Database::in_memory().unwrap();

        let game_loop = GameLoop::new(game_state, db).unwrap();
        assert_eq!(game_loop.auto_save_interval, 5);
        assert_eq!(game_loop.matches_since_save, 0);
    }
}
