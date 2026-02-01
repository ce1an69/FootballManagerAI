use crate::game::{GameState, Screen, GameDate, NotificationType, NotificationPriority};
use crate::team::{League, MatchResult, Team};
use crate::data::{
    Database, ScheduledMatchData, ScheduledMatchRepository,
    MatchRepository, TeamStatisticsRepository, PlayerRepository,
};
use crate::ai::events::generate_injury_event;

/// Match flow error types
#[derive(Debug, thiserror::Error)]
pub enum MatchFlowError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::data::DatabaseError),

    #[error("No match scheduled")]
    NoMatchScheduled,

    #[error("Match not found: {0}")]
    MatchNotFound(String),

    #[error("Invalid game state")]
    InvalidState,
}

/// Advance to next match for player's team
pub fn advance_to_next_match(
    game_state: &mut GameState,
    database: &Database,
) -> Result<MatchInfo, MatchFlowError> {
    // Find next scheduled match for player team
    let scheduled_match_repo = database.scheduled_match_repo();
    let all_matches = scheduled_match_repo.get_by_league(&game_state.league.id)?;

    // Find next unplayed match involving player team
    let next_match = all_matches.into_iter()
        .filter(|m| !m.played)
        .find(|m| {
            m.home_team_id == game_state.player_team_id ||
            m.away_team_id == game_state.player_team_id
        })
        .ok_or(MatchFlowError::NoMatchScheduled)?;

    // Get team information (clone needed to avoid borrow issues)
    let home_team_id = next_match.home_team_id.clone();
    let away_team_id = next_match.away_team_id.clone();
    let player_team_id = game_state.player_team_id.clone();

    let home_team = game_state.teams.get(&home_team_id)
        .ok_or(MatchFlowError::InvalidState)?;
    let away_team = game_state.teams.get(&away_team_id)
        .ok_or(MatchFlowError::InvalidState)?;

    let home_team_name = home_team.name.clone();
    let away_team_name = away_team.name.clone();
    let is_home = home_team_id == player_team_id;

    // Advance date to match day
    while !is_match_day(&game_state.current_date, &next_match) {
        game_state.advance_time();

        // Check for expiring contracts on the first day of each month
        if game_state.current_date.day == 1 {
            check_and_notify_expiring_contracts(game_state, database)?;
        }
    }

    // Navigate to match mode selection
    game_state.navigate_to(Screen::MatchModeSelection);

    Ok(MatchInfo {
        match_id: next_match.id,
        home_team_id,
        away_team_id,
        home_team_name,
        away_team_name,
        is_home,
        round: next_match.round_number,
    })
}

/// Update game state after match completion
pub fn update_after_match(
    game_state: &mut GameState,
    database: &Database,
    match_result: &MatchResult,
) -> Result<MatchUpdateResult, MatchFlowError> {
    // Update statistics for both teams
    update_team_statistics(database, match_result)?;

    // Update league standings
    update_league_standings(game_state, database)?;

    // Check if this was the last match of the round
    let is_last_of_round = is_last_match_of_round(database, &match_result.home_team_id, &match_result.away_team_id)?;

    // Check if season is complete
    let is_season_end = is_season_complete(database, &game_state.league.id)?;

    // Add notification for match result
    let player_team = game_state.get_player_team().unwrap();
    let is_player_home = match_result.home_team_id == game_state.player_team_id;
    let (player_score, opponent_score) = if is_player_home {
        (match_result.home_score, match_result.away_score)
    } else {
        (match_result.away_score, match_result.home_score)
    };

    let result_text = if player_score > opponent_score {
        format!("Won {}-{}", player_score, opponent_score)
    } else if player_score == opponent_score {
        format!("Drew {}-{}", player_score, opponent_score)
    } else {
        format!("Lost {}-{}", player_score, opponent_score)
    };

    game_state.add_notification(
        "Match Result".to_string(),
        format!("{} {} vs {}: {}",
            player_team.name,
            match_result.home_team_id,
            match_result.away_team_id,
            result_text
        ),
        crate::game::NotificationType::Match,
        crate::game::NotificationPriority::Normal,
    );

    // Generate injury events for home team players
    if let Some(home_team) = game_state.teams.get(&match_result.home_team_id) {
        for player_slot in &home_team.starting_11 {
            if let Ok(player) = database.player_repo().get_by_id(&player_slot.player_id) {
                if let Some(event) = generate_injury_event(&player) {
                    let notification = event.to_notification();
                    game_state.notifications.push(notification);
                }
            }
        }
    }

    // Check for season end
    if is_season_end {
        handle_season_end(game_state, database)?;
    } else if is_last_of_round {
        // Advance to next round
        advance_to_next_round(game_state)?;
    }

    Ok(MatchUpdateResult {
        is_season_end,
        is_last_of_round,
        league_position: get_league_position(database, &game_state.player_team_id)?,
    })
}

/// Check if current date is match day
fn is_match_day(current_date: &GameDate, _scheduled_match: &ScheduledMatchData) -> bool {
    // Simplified: assume match is on current day
    // In full implementation, would check scheduled date
    true
}

/// Check if this is the last match of the round
pub fn is_last_match_of_round(
    database: &Database,
    home_team_id: &str,
    away_team_id: &str,
) -> Result<bool, MatchFlowError> {
    // Get the match to find its round
    let match_repo = database.match_repo();

    // Try to find a match between these teams
    let matches_for_teams: Vec<_> = match_repo.get_by_team(home_team_id, 100)?
        .into_iter()
        .filter(|m| &m.away_team_id == away_team_id)
        .collect();

    if matches_for_teams.is_empty() {
        return Ok(false);
    }

    // Get all matches in the league
    let scheduled_repo = database.scheduled_match_repo();
    let league_id = &matches_for_teams[0].league_id;
    let all_matches = scheduled_repo.get_by_league(league_id)?;

    // Find the round of this match
    let this_match = all_matches.iter()
        .find(|m| &m.home_team_id == home_team_id && &m.away_team_id == away_team_id);

    if let Some(m) = this_match {
        let round = m.round_number;

        // Check if all matches in this round are played
        let round_matches: Vec<_> = all_matches.iter()
            .filter(|m| m.round_number == round)
            .collect();

        let all_played = round_matches.iter().all(|m| m.played);
        Ok(all_played)
    } else {
        Ok(false)
    }
}

/// Check if season is complete
pub fn is_season_complete(
    database: &Database,
    league_id: &str,
) -> Result<bool, MatchFlowError> {
    let scheduled_repo = database.scheduled_match_repo();
    let all_matches = scheduled_repo.get_by_league(league_id)?;

    let all_played = all_matches.iter().all(|m| m.played);
    Ok(all_played)
}

/// Update team statistics after match
fn update_team_statistics(
    database: &Database,
    match_result: &MatchResult,
) -> Result<(), MatchFlowError> {
    let stats_repo = database.team_statistics_repo();

    // Update home team
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

    // Update away team
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

/// Update league standings
fn update_league_standings(
    game_state: &GameState,
    database: &Database,
) -> Result<(), MatchFlowError> {
    let stats_repo = database.team_statistics_repo();
    let mut all_standings = Vec::new();

    for team_id in &game_state.league.teams {
        let stats = stats_repo.get_by_team(team_id)?;
        all_standings.push((team_id.clone(), stats));
    }

    // Sort by points (descending), then goal difference
    all_standings.sort_by(|a, b| {
        let a_gd = a.1.goals_for as i32 - a.1.goals_against as i32;
        let b_gd = b.1.goals_for as i32 - b.1.goals_against as i32;

        b.1.points.cmp(&a.1.points)
            .then_with(|| b_gd.cmp(&a_gd))
    });

    // Update positions
    for (position, (team_id, stats)) in all_standings.into_iter().enumerate() {
        let mut stats = stats;
        stats.league_position = Some(position as u32 + 1);
        stats_repo.update(&team_id, &stats)?;
    }

    Ok(())
}

/// Get current league position for a team
fn get_league_position(
    database: &Database,
    team_id: &str,
) -> Result<Option<u32>, MatchFlowError> {
    let stats_repo = database.team_statistics_repo();
    let stats = stats_repo.get_by_team(team_id)?;
    Ok(stats.league_position)
}

/// Advance to next round
fn advance_to_next_round(
    game_state: &mut GameState,
) -> Result<(), MatchFlowError> {
    // Advance time by one week
    game_state.current_date.advance_week();

    // Add notification
    game_state.add_notification(
        "Next Round".to_string(),
        format!("Week {} - Check for upcoming matches", game_state.current_date.weekday()),
        crate::game::NotificationType::News,
        crate::game::NotificationPriority::Low,
    );

    Ok(())
}

/// Handle season end
fn handle_season_end(
    game_state: &mut GameState,
    database: &Database,
) -> Result<(), MatchFlowError> {
    // Add season summary notification
    let stats_repo = database.team_statistics_repo();
    let stats = stats_repo.get_by_team(&game_state.player_team_id)?;
    let player_position = stats.league_position.unwrap_or(0);

    game_state.add_notification(
        "Season End".to_string(),
        format!("Season {} completed! Final position: {}",
            game_state.current_date.season_string(),
            player_position
        ),
        crate::game::NotificationType::Achievement,
        crate::game::NotificationPriority::High,
    );

    // Navigate to season summary screen
    game_state.navigate_to(Screen::SeasonSummary {
        season: game_state.current_date.season_string(),
    });

    Ok(())
}

/// Check for expiring contracts and create notifications
fn check_and_notify_expiring_contracts(
    game_state: &mut GameState,
    database: &Database,
) -> Result<(), MatchFlowError> {
    // Get players from player's team
    let player_repo = database.player_repo();
    let players = player_repo.get_by_team(&game_state.player_team_id)?;

    // Check for contracts expiring within 1 month
    for player in players {
        if player.contract_years <= 1 {
            let urgency = if player.contract_years == 0 {
                NotificationPriority::Urgent
            } else {
                NotificationPriority::High
            };

            let message = if player.contract_years == 0 {
                format!("{}'s contract has expired! Age: {}, Ability: {}",
                    player.name,
                    player.age,
                    player.current_ability
                )
            } else {
                format!("{}'s contract expires in {} month{}. Age: {}, Ability: {}, Wage: {}",
                    player.name,
                    player.contract_years,
                    if player.contract_years == 1 { "" } else { "s" },
                    player.age,
                    player.current_ability,
                    player.wage
                )
            };

            game_state.add_notification(
                "Contract Expiring".to_string(),
                message,
                NotificationType::Contract,
                urgency,
            );
        }
    }

    Ok(())
}

/// Information about a match
#[derive(Debug, Clone)]
pub struct MatchInfo {
    pub match_id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_team_name: String,
    pub away_team_name: String,
    pub is_home: bool,
    pub round: u32,
}

/// Result of updating after a match
#[derive(Debug, Clone)]
pub struct MatchUpdateResult {
    pub is_season_end: bool,
    pub is_last_of_round: bool,
    pub league_position: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::{Team, League};
    use crate::data::Database;

    #[test]
    fn test_advance_to_next_match() {
        let league = League::new("test_league".to_string(), "Test".to_string(), vec![]);
        let team1 = Team::new("team1".to_string(), "Team 1".to_string(), "test_league".to_string(), 1000000);
        let team2 = Team::new("team2".to_string(), "Team 2".to_string(), "test_league".to_string(), 1000000);

        let mut game_state = GameState::new(
            "team1".to_string(),
            league,
            vec![team1, team2],
            2026
        );

        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        // This should fail as no matches are scheduled
        let result = advance_to_next_match(&mut game_state, &db);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_last_match_of_round() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        // No matches scheduled, should return false or Ok
        let result = is_last_match_of_round(&db, "team1", "team2");
        // Result depends on implementation
        assert!(result.is_ok());
    }
}
