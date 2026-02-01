use crate::team::{Team, Player, MatchResult, MatchEvent, MatchMode, MatchStatistics, Position, PlayerMatchRating};
use rand::Rng;
use rand::seq::SliceRandom;

/// Match simulator
pub struct MatchSimulator;

impl MatchSimulator {
    /// Create new match simulator
    pub fn new() -> Self {
        Self
    }

    /// Simulate a match between two teams
    pub fn simulate_match(
        &mut self,
        home_team: &Team,
        away_team: &Team,
        home_players: &[Player],
        away_players: &[Player],
        mode: MatchMode,
    ) -> MatchResult {
        let home_strength = self.calculate_team_strength(home_team, home_players);
        let away_strength = self.calculate_team_strength(away_team, away_players);

        // Calculate goals based on strength difference
        let (home_goals, away_goals) = self.calculate_score(
            home_strength,
            away_strength,
            &mut rand::thread_rng(),
        );

        // Generate events
        let events = self.generate_match_events(
            home_team,
            away_team,
            home_players,
            away_players,
            home_goals,
            away_goals,
        );

        // Generate statistics
        let statistics = self.generate_match_statistics(
            home_team,
            away_team,
            home_players,
            away_players,
            home_goals,
            away_goals,
            &events,
        );

        // Generate player ratings
        let (home_ratings, away_ratings) = self.generate_player_ratings(
            home_players,
            away_players,
            home_goals,
            away_goals,
        );

        let mut result_stats = statistics;
        result_stats.home_player_ratings = home_ratings;
        result_stats.away_player_ratings = away_ratings;

        MatchResult {
            id: uuid::Uuid::new_v4().to_string(),
            league_id: home_team.league_id.clone(),
            home_team_id: home_team.id.clone(),
            away_team_id: away_team.id.clone(),
            home_score: home_goals,
            away_score: away_goals,
            match_mode: mode,
            events,
            statistics: result_stats,
            played_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            round: 0,
        }
    }

    /// Calculate overall team strength
    fn calculate_team_strength(&self, team: &Team, players: &[Player]) -> u16 {
        if team.starting_11.is_empty() {
            return 50;
        }

        let mut total_strength = 0;
        let mut count = 0;

        for slot in &team.starting_11 {
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                let rating = player.get_position_rating(&slot.position);
                total_strength += rating;
                count += 1;
            }
        }

        if count == 0 {
            50
        } else {
            total_strength / count
        }
    }

    /// Calculate score based on team strengths
    fn calculate_score(&mut self, home_strength: u16, away_strength: u16, rng: &mut rand::rngs::ThreadRng) -> (u8, u8) {
        let strength_diff = home_strength as i32 - away_strength as i32;

        // Base goals calculation
        let home_expected = if strength_diff > 0 {
            2.0 + (strength_diff as f32 / 50.0) // Stronger team scores more
        } else {
            1.0 + ((strength_diff.abs() as f32) / 100.0) // Weaker team might still score
        };

        let away_expected = if strength_diff < 0 {
            2.0 + (strength_diff.abs() as f32 / 50.0)
        } else {
            1.0 + ((strength_diff as f32) / 100.0)
        };

        // Add randomness
        let home_goals = (home_expected + rng.gen_range(-0.5..0.5)).floor().max(0.0).min(5.0) as u8;
        let away_goals = (away_expected + rng.gen_range(-0.5..0.5)).floor().max(0.0).min(5.0) as u8;

        (home_goals, away_goals)
    }

    /// Generate match events
    fn generate_match_events(
        &mut self,
        home_team: &Team,
        away_team: &Team,
        home_players: &[Player],
        away_players: &[Player],
        home_goals: u8,
        away_goals: u8,
    ) -> Vec<MatchEvent> {
        let mut events = Vec::new();
        let total_goals = home_goals + away_goals;
        let mut rng = rand::thread_rng();

        if total_goals == 0 {
            return events;
        }

        // Distribute goals throughout the match
        let goal_minutes = self.generate_goal_minutes(total_goals, &mut rng);

        // Assign goals to teams
        let mut home_goals_left = home_goals;
        let mut away_goals_left = away_goals;

        for minute in goal_minutes {
            if home_goals_left > 0 && (away_goals_left == 0 || rng.gen_bool(0.5)) {
                let scorer = self.select_random_scorer(home_team, home_players, &mut rng);
                events.push(MatchEvent::Goal {
                    team: home_team.id.clone(),
                    player_id: scorer,
                    minute,
                });
                home_goals_left -= 1;
            } else if away_goals_left > 0 {
                let scorer = self.select_random_scorer(away_team, away_players, &mut rng);
                events.push(MatchEvent::Goal {
                    team: away_team.id.clone(),
                    player_id: scorer,
                    minute,
                });
                away_goals_left -= 1;
            }
        }

        // Add some yellow cards
        let yellow_cards = rng.gen_range(0..4);
        for _ in 0..yellow_cards {
            let minute = rng.gen_range(1..91);
            let team = if rng.gen_bool(0.5) { &home_team.id } else { &away_team.id };
            let player = if rng.gen_bool(0.5) {
                self.select_random_scorer(home_team, home_players, &mut rng)
            } else {
                self.select_random_scorer(away_team, away_players, &mut rng)
            };

            events.push(MatchEvent::YellowCard {
                team: team.clone(),
                player_id: player,
                minute,
            });
        }

        events
    }

    /// Generate random goal minutes
    fn generate_goal_minutes(&self, total_goals: u8, rng: &mut rand::rngs::ThreadRng) -> Vec<u8> {
        let mut minutes = Vec::new();
        let mut used_minutes = vec![false; 91]; // 0-90 minutes

        for _ in 0..total_goals {
            loop {
                let minute = rng.gen_range(1..91);
                if !used_minutes[minute as usize] {
                    used_minutes[minute as usize] = true;
                    minutes.push(minute);
                    break;
                }
            }
        }

        minutes.sort();
        minutes
    }

    /// Select random scorer from team
    fn select_random_scorer(&self, team: &Team, players: &[Player], rng: &mut rand::rngs::ThreadRng) -> String {
        if team.starting_11.is_empty() {
            return players.first().map(|p| p.id.clone()).unwrap_or_default();
        }

        let attacking_slots: Vec<_> = team.starting_11.iter()
            .filter(|slot| {
                matches!(slot.position, Position::ST | Position::CF | Position::LW | Position::RW | Position::AM)
            })
            .collect();

        if let Some(slot) = attacking_slots.choose(rng) {
            // Try to get the player at this position
            if let Some(player) = players.iter().find(|p| p.id == slot.player_id) {
                return player.id.clone();
            }
        }

        // Fallback to any player
        team.starting_11.get(0)
            .map(|s| s.player_id.clone())
            .or_else(|| players.first().map(|p| p.id.clone()))
            .unwrap_or_default()
    }

    /// Generate match statistics
    fn generate_match_statistics(
        &self,
        home_team: &Team,
        away_team: &Team,
        home_players: &[Player],
        away_players: &[Player],
        home_goals: u8,
        away_goals: u8,
        _events: &[MatchEvent],
    ) -> MatchStatistics {
        let mut rng = rand::thread_rng();

        // Calculate possession based on team strength
        let home_strength = self.calculate_team_strength(home_team, home_players);
        let away_strength = self.calculate_team_strength(away_team, away_players);
        let total_strength = home_strength + away_strength;

        let home_possession = if total_strength > 0 {
            (home_strength * 100) / total_strength
        } else {
            50
        };
        let away_possession = 100 - home_possession;

        // Generate shots based on possession and goals
        let home_shots = home_possession + rng.gen_range(5..15);
        let away_shots = away_possession + rng.gen_range(5..15);

        // Some shots on target
        let home_shots_on_target = (home_shots / 3) + home_goals as u16;
        let away_shots_on_target = (away_shots / 3) + away_goals as u16;

        // Passes
        let home_passes = 100 + rng.gen_range(50..150);
        let away_passes = 100 + rng.gen_range(50..150);

        // Other stats
        let home_corners = rng.gen_range(2..8);
        let away_corners = rng.gen_range(2..8);
        let home_fouls = rng.gen_range(10..20);
        let away_fouls = rng.gen_range(10..20);

        MatchStatistics {
            home_possession: home_possession as f32,
            away_possession: away_possession as f32,
            home_shots: home_shots as u16,
            away_shots: away_shots as u16,
            home_shots_on_target: home_shots_on_target as u16,
            away_shots_on_target: away_shots_on_target as u16,
            home_passes: home_passes as u16,
            away_passes: away_passes as u16,
            home_pass_accuracy: if home_passes > 0 {
                ((home_passes / 3) * 100 / home_passes) as f32
            } else { 0.0 },
            away_pass_accuracy: if away_passes > 0 {
                ((away_passes / 3) * 100 / away_passes) as f32
            } else { 0.0 },
            home_corners,
            away_corners,
            home_fouls,
            away_fouls,
            home_offsides: rng.gen_range(1..6),
            away_offsides: rng.gen_range(1..6),
            home_player_ratings: Vec::new(),
            away_player_ratings: Vec::new(),
        }
    }

    /// Generate player ratings for the match
    fn generate_player_ratings(
        &self,
        home_players: &[Player],
        away_players: &[Player],
        home_goals: u8,
        away_goals: u8,
    ) -> (Vec<PlayerMatchRating>, Vec<PlayerMatchRating>) {
        let home_ratings = home_players.iter().map(|player| {
            PlayerMatchRating {
                player_id: player.id.clone(),
                player_name: player.name.clone(),
                position: player.position.clone(),
                rating: self.calculate_player_rating(player, false, home_goals),
                minutes_played: 90,
                goals: 0,
                assists: 0,
            }
        }).collect();

        let away_ratings = away_players.iter().map(|player| {
            PlayerMatchRating {
                player_id: player.id.clone(),
                player_name: player.name.clone(),
                position: player.position.clone(),
                rating: self.calculate_player_rating(player, true, away_goals),
                minutes_played: 90,
                goals: 0,
                assists: 0,
            }
        }).collect();

        (home_ratings, away_ratings)
    }

    /// Calculate player rating for the match
    fn calculate_player_rating(&self, player: &Player, is_away: bool, team_goals: u8) -> f32 {
        let mut rating = player.calculate_overall_ability() as f32 / 20.0; // Base 0-10

        // Adjust for performance
        if player.is_gk() {
            // Goalkeepers: better if they conceded less
            let goals_conceded = if is_away { team_goals } else { team_goals };
            rating -= (goals_conceded as f32) * 0.5;
        } else {
            // Outfield players: better if involved in goals
            let team_goals = if is_away { team_goals } else { team_goals };
            rating += (team_goals as f32) * 0.3;
        }

        // Fatigue impact
        rating -= (player.fatigue as f32) * 0.03;

        // Morale impact
        rating += (player.morale as f32 - 50.0) * 0.02;

        // Clamp to 6.0-10.0 range
        rating.max(6.0).min(10.0)
    }
}

impl Default for MatchSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Position;

    fn create_test_team(id: &str, name: &str) -> Team {
        Team::new(id.to_string(), name.to_string(), "league1".to_string(), 1000000)
    }

    fn create_test_player(id: &str, name: &str, position: Position, finishing: u16) -> Player {
        let mut player = Player::new(id.to_string(), name.to_string(), position);
        player.finishing = finishing;
        player.pace = 100;
        player.stamina = 100;
        player.current_ability = 100;
        player
    }

    #[test]
    fn test_match_simulation() {
        let mut simulator = MatchSimulator::new();

        let home_team = create_test_team("team1", "Home");
        let away_team = create_test_team("team2", "Away");

        let home_players = vec![
            create_test_player("p1", "Player 1", Position::ST, 120),
            create_test_player("p2", "Player 2", Position::ST, 100),
        ];

        let away_players = vec![
            create_test_player("p3", "Player 3", Position::ST, 90),
            create_test_player("p4", "Player 4", Position::ST, 95),
        ];

        let result = simulator.simulate_match(&home_team, &away_team, &home_players, &away_players, MatchMode::Quick);

        assert_eq!(result.home_team_id, "team1");
        assert_eq!(result.away_team_id, "team2");
        // Events are only generated when goals are scored or cards are given
        // 0-0 draws with no cards are possible, so we don't assert events > 0
    }

    #[test]
    fn test_score_calculation() {
        let mut simulator = MatchSimulator::new();

        // Strong home team
        let home_strength = 150;
        let away_strength = 100;

        let (home, away) = simulator.calculate_score(home_strength, away_strength, &mut rand::thread_rng());

        assert!(home >= away);
        assert!(home <= 5); // Max reasonable goals
    }

    #[test]
    fn test_goal_generation() {
        let simulator = MatchSimulator::new();
        let mut rng = rand::thread_rng();

        // Test 3 goals
        let minutes = simulator.generate_goal_minutes(3, &mut rng);

        assert_eq!(minutes.len(), 3);
        assert!(minutes[0] < minutes[1] && minutes[1] < minutes[2]); // Sorted
        assert!(minutes.iter().all(|m| *m >= 1 && *m <= 90));
    }

    #[test]
    fn test_player_rating() {
        let simulator = MatchSimulator::new();

        let mut player = create_test_player("p1", "Player", Position::ST, 120);
        player.morale = 80;
        player.fatigue = 10;

        let rating = simulator.calculate_player_rating(&player, false, 2);
        assert!(rating >= 6.0 && rating <= 10.0);
    }
}
