use crate::game::GameState;
use crate::ai::{generate_league, generate_team};
use crate::data::{
    Database, SaveManager,
    TeamRepository, PlayerRepository, LeagueRepository,
    ScheduledMatchRepository, TeamStatisticsRepository
};

/// Game initialization error types
#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] crate::data::DatabaseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Start a new game
///
/// Creates all necessary game data including league, teams, and players
pub fn start_new_game(
    save_slot: u8,
    league_name: String,
    country: String,
    num_teams: usize,
    skill_level: u16,
    start_year: u16,
) -> Result<(GameState, Database), InitError> {
    // Create database in saves directory
    let save_manager = SaveManager::default();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs();
    let save_path = save_manager.get_save_path(save_slot, &league_name, timestamp);

    // Initialize database
    let db = Database::new(save_path.to_str().unwrap())?;
    db.run_migrations()?;

    // Generate league and teams
    let league = generate_league(num_teams, skill_level);

    // Generate team objects for all teams in league
    let mut teams = Vec::new();
    for team_id in &league.teams {
        let team = generate_team(
            team_id.clone(),
            format!("Team {}", team_id),
            skill_level
        );
        teams.push(team);
    }

    // Select player team (first team by default)
    let player_team_id = league.teams.get(0)
        .ok_or_else(|| InitError::DatabaseError(
            crate::data::DatabaseError::QueryError("League has no teams".to_string())
        ))?
        .clone();

    // Create game state
    let mut game_state = GameState::new(player_team_id.clone(), league, teams, start_year);
    game_state.db_path = save_path.to_string_lossy().to_string();
    game_state.is_new_game = true;

    // Save league to database first (required by foreign key constraint)
    db.league_repo().create(&game_state.league)?;

    // Save teams to database
    let team_repo = db.team_repo();
    for (_team_id, team) in &game_state.teams {
        team_repo.create(team)?;
    }

    // Generate players for each team
    let player_repo = db.player_repo();
    for (_team_id, team) in &game_state.teams {
        // Generate 25 players for each team
        for i in 0..25 {
            let position = get_position_for_index(i);
            let player = crate::ai::generate_player(
                Some(team.id.clone()),
                position,
                skill_level,
                18..28
            );

            // Set team_id
            let mut player_with_team = player;
            player_with_team.team_id = Some(team.id.clone());

            player_repo.create(&player_with_team)?;
        }
    }

    // Generate league schedule
    game_state.league.generate_schedule();

    // Save scheduled matches to database
    for round in &game_state.league.schedule.rounds {
        for scheduled_match in &round.matches {
            let match_data = crate::data::ScheduledMatchData {
                id: uuid::Uuid::new_v4().to_string(),
                league_id: game_state.league.id.clone(),
                round_number: round.round_number,
                home_team_id: scheduled_match.home_team_id.clone(),
                away_team_id: scheduled_match.away_team_id.clone(),
                played: scheduled_match.played,
            };

            db.scheduled_match_repo().create(&match_data)?;
        }
    }

    // Create initial team statistics
    let stats_repo = db.team_statistics_repo();
    for team_id in &game_state.league.teams {
        stats_repo.create(team_id)?;
    }

    // Save initial game state
    save_manager.create_new_save(save_slot, &game_state, &db)?;

    // Add welcome notification
    game_state.add_notification(
        "Welcome to Football Manager!".to_string(),
        format!("You are managing {}", game_state.get_player_team()
            .map(|t| t.name.as_str())
            .unwrap_or("your team")
        ),
        crate::game::NotificationType::System,
        crate::game::NotificationPriority::Normal,
    );

    Ok((game_state, db))
}

/// Load an existing game
///
/// Loads game state and database from specified save slot
pub fn load_game(slot: u8) -> Result<(GameState, Database), InitError> {
    let save_manager = SaveManager::default();

    // Load game state
    let game_state = save_manager.load_game(slot)?;

    // Open database
    let db_path = &game_state.db_path;
    let db = Database::new(db_path)?;

    // Verify database exists and is valid
    if !std::path::Path::new(db_path).exists() {
        return Err(InitError::DatabaseError(
            crate::data::DatabaseError::QueryError(
                format!("Database file not found: {}", db_path)
            )
        ));
    }

    Ok((game_state, db))
}

/// Get position for squad index
fn get_position_for_index(index: usize) -> crate::team::Position {
    use crate::team::Position;

    match index {
        0 => Position::GK,
        1..=4 => Position::CB,
        5 => Position::LB,
        6 => Position::RB,
        7..=10 => Position::CM,
        11..=14 => Position::ST,
        15..=17 => Position::LW,
        18..=20 => Position::RW,
        21..=23 => Position::AM,
        _ => Position::DM,
    }
}

/// Create a new save with default settings
pub fn quick_start() -> Result<(GameState, Database), InitError> {
    start_new_game(
        1,
        "Premier League".to_string(),
        "England".to_string(),
        20,
        120,
        2026
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_start() {
        let result = quick_start();
        if let Err(e) = &result {
            eprintln!("Error: {}", e);
        }
        assert!(result.is_ok());

        let (state, _db) = result.unwrap();
        assert!(state.is_new_game);
        assert!(state.current_date.year == 2026);
        assert!(state.teams.len() == 20);

        // Check database was created
        let path = std::path::Path::new(&state.db_path);
        assert!(path.exists());

        // Cleanup
        std::fs::remove_file(&state.db_path).ok();
    }

    #[test]
    fn test_position_generation() {
        let pos = get_position_for_index(0);
        assert_eq!(format!("{:?}", pos), "GK");

        let pos = get_position_for_index(5);
        assert_eq!(format!("{:?}", pos), "LB");
    }
}
