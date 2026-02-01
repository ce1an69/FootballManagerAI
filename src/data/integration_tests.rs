/// End-to-end integration tests
///
/// These tests verify that all modules work together correctly
#[cfg(test)]
mod integration_tests {
    use crate::data::{Database, PlayerRepository, TeamStatisticsRepository};
    use std::sync::atomic::{AtomicU8, Ordering};

    // Static counter for unique save slots
    static SAVE_SLOT_COUNTER: AtomicU8 = AtomicU8::new(1);

    /// Helper: Create a complete test game
    fn create_test_game() -> (crate::game::GameState, Database) {
        // Get unique save slot (1-200, cycling)
        let save_slot = SAVE_SLOT_COUNTER.fetch_add(1, Ordering::SeqCst) % 200 + 1;
        crate::game::init::start_new_game(
            save_slot,
            format!("Test League {}", save_slot),
            "Test Country".to_string(),
            20,
            120,
            2026,
        ).expect("Failed to create test game")
    }

    #[test]
    fn test_complete_game_flow() {
        // This test verifies the complete game creation flow
        let (state, db) = create_test_game();

        // Verify game state
        assert!(!state.player_team_id.is_empty());
        assert_eq!(state.teams.len(), 20); // Quick start creates 20 teams
        assert!(state.current_date.year >= 2026);

        // Verify database exists
        assert!(!state.db_path.is_empty());
        assert!(std::path::Path::new(&state.db_path).exists());

        // Verify league was created
        assert_eq!(state.league.teams.len(), 20);

        // Verify scheduled matches were generated
        assert!(!state.league.schedule.rounds.is_empty());
        assert_eq!(state.league.schedule.rounds[0].matches.len(), 10); // 20 teams, 10 matches per round

        // Verify database has all tables
        let conn = db.conn.read().unwrap();
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"players".to_string()));
        assert!(tables.contains(&"teams".to_string()));
        assert!(tables.contains(&"leagues".to_string()));
        assert!(tables.contains(&"matches".to_string()));
        assert!(tables.contains(&"scheduled_matches".to_string()));
    }

    #[test]
    fn test_save_load_cycle() {
        // Test complete save and load cycle
        let (original_state, original_db) = create_test_game();

        // Save the game
        let save_manager = crate::data::SaveManager::default();

        let save_path = save_manager.save_game(1, &original_state, &original_db)
            .expect("Failed to save game");

        // Verify save file exists
        assert!(std::path::Path::new(&save_path).exists());

        // Load the game
        let loaded_state = save_manager.load_game(1)
            .expect("Failed to load game");

        // Verify state was preserved
        assert_eq!(loaded_state.player_team_id, original_state.player_team_id);
        assert_eq!(loaded_state.teams.len(), original_state.teams.len());
        assert_eq!(loaded_state.league.teams.len(), original_state.league.teams.len());

        // Cleanup
        std::fs::remove_file(&save_path).ok();
    }

    #[test]
    fn test_league_schedule_generation() {
        let (state, _db) = create_test_game();

        // Verify schedule structure
        let schedule = &state.league.schedule;
        assert!(!schedule.rounds.is_empty());

        // For 20 teams in double round-robin:
        // Each team plays 19 home + 19 away = 38 matches
        // Total rounds = 38 (20 - 1) * 2
        // Matches per round = 10 (20 teams / 2)
        let expected_rounds = 38;
        assert_eq!(schedule.rounds.len(), expected_rounds);

        // Verify first round has correct number of matches
        let first_round = &schedule.rounds[0];
        assert_eq!(first_round.matches.len(), 10); // 20 teams / 2

        // Verify matches are not played initially
        assert!(!first_round.matches.iter().any(|m| m.played));
    }

    #[test]
    fn test_player_generation() {
        let (state, db) = create_test_game();

        // Get player team
        let player_team = state.get_player_team().expect("Player team not found");

        // Get players for team
        let players = db.player_repo()
            .get_by_team(&player_team.id)
            .expect("Failed to get players");

        // Verify team has players
        assert!(!players.is_empty());
        assert!(players.len() >= 11); // At least starting 11

        // Verify player structure
        for player in &players {
            assert!(!player.id.is_empty());
            assert!(!player.name.is_empty());
            assert!(player.age >= 16);
            assert!(player.age <= 40);
            assert!(player.current_ability > 0);
            assert!(player.potential_ability >= player.current_ability);
            assert!(player.wage > 0);
        }
    }

    #[test]
    fn test_player_valuation() {
        let (state, db) = create_test_game();

        // Get players
        let player_team = state.get_player_team().expect("Player team not found");
        let players = db.player_repo()
            .get_by_team(&player_team.id)
            .expect("Failed to get players");

        if !players.is_empty() {
            // Test player valuation
            let test_player = &players[0];
            let value = crate::transfer::evaluate_player_value(test_player);

            assert!(value > 0);
            assert!(value >= 100_000); // Minimum value

            // Test potential value prediction
            let future_value = crate::transfer::predict_potential_value(test_player, 1);
            assert!(future_value > 0);
        }
    }

    #[test]
    fn test_notification_system() {
        let (mut state, _db) = create_test_game();

        // Initially should have welcome notification
        let initial_count = state.notifications.len();

        // Add test notification
        state.add_notification(
            "Test Notification".to_string(),
            "This is a test notification".to_string(),
            crate::game::NotificationType::System,
            crate::game::NotificationPriority::Normal,
        );

        // Verify notification was added
        assert_eq!(state.notifications.len(), initial_count + 1);

        // Verify latest notification
        let latest = &state.notifications[state.notifications.len() - 1];
        assert_eq!(latest.title, "Test Notification");
        assert_eq!(latest.notification_type, crate::game::NotificationType::System);
    }

    #[test]
    fn test_game_state_navigation() {
        let (mut state, _db) = create_test_game();

        // Test navigation stack - clone initial screen for comparison
        let initial_screen = state.current_screen.clone();

        // Navigate to team management
        state.navigate_to(crate::game::Screen::TeamManagement);
        assert_eq!(state.current_screen, crate::game::Screen::TeamManagement);

        // Navigate to transfer market
        state.navigate_to(crate::game::Screen::TransferMarket);
        assert_eq!(state.current_screen, crate::game::Screen::TransferMarket);

        // Go back
        state.go_back();
        assert_eq!(state.current_screen, crate::game::Screen::TeamManagement);

        // Go back again
        state.go_back();
        assert_eq!(state.current_screen, initial_screen);
    }

    #[test]
    fn test_team_statistics_initial() {
        let (state, db) = create_test_game();

        // Get player team
        let player_team = state.get_player_team().expect("Player team not found");

        // Verify team statistics exist
        let stats = db.team_statistics_repo()
            .get_by_team(&player_team.id)
            .expect("Failed to get team statistics");

        assert_eq!(stats.matches_played, 0);
        assert_eq!(stats.wins, 0);
        assert_eq!(stats.draws, 0);
        assert_eq!(stats.losses, 0);
        assert_eq!(stats.goals_for, 0);
        assert_eq!(stats.goals_against, 0);
        assert_eq!(stats.points, 0);
    }

    #[test]
    fn test_contract_management() {
        let (state, db) = create_test_game();

        // Get player team
        let player_team = state.get_player_team().expect("Player team not found");

        // Check that players have contracts
        let players = db.player_repo()
            .get_by_team(&player_team.id)
            .expect("Failed to get players");

        // Verify each player has valid contract data
        for player in &players {
            assert!(player.wage > 0);
        }
    }

    #[test]
    fn test_database_indexes_created() {
        let (state, _db) = create_test_game();

        // Verify database path exists
        assert!(std::path::Path::new(&state.db_path).exists());

        // Open database connection to verify indexes
        if let Ok(conn) = rusqlite::Connection::open(&state.db_path) {
            let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'").unwrap();
            let indexes: Vec<String> = stmt.query_map([], |row| row.get::<_, String>(0))
                .unwrap()
                .collect::<Result<Vec<_>, _>>()
                .unwrap();

            // Should have created indexes
            assert!(indexes.len() > 0);
            assert!(indexes.iter().any(|n| n.contains("idx_players_team_id")));
            assert!(indexes.iter().any(|n| n.contains("idx_teams_league_id")));
        }
    }

    #[test]
    fn test_all_tables_have_data() {
        let (state, db) = create_test_game();

        // Verify players exist
        let player_team = state.get_player_team().expect("Player team not found");
        let players = db.player_repo()
            .get_by_team(&player_team.id)
            .expect("Failed to get players");
        assert!(!players.is_empty());

        // Verify all teams have statistics
        for team_id in &state.league.teams {
            let _stats = db.team_statistics_repo()
                .get_by_team(team_id)
                .expect(&format!("Failed to get stats for {}", team_id));
            // Statistics exist for this team
        }

        // Verify schedule is generated
        assert!(!state.league.schedule.rounds.is_empty());
    }
}
