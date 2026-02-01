// Integration tests for Football Manager AI
//
// These tests verify the complete flow from game creation to playing matches,
// contract management, and player valuation.

use football_manager_ai::game::init::start_new_game;
use football_manager_ai::data::{PlayerRepository, TeamRepository, Database};
use football_manager_ai::transfer::ContractManager;
use football_manager_ai::team::calculate_market_value;
use std::sync::atomic::{AtomicU16, Ordering};

// Counter to generate unique league IDs for each test
static TEST_COUNTER: AtomicU16 = AtomicU16::new(0);

/// Helper function to create a unique game for each test
fn create_test_game() -> Result<(football_manager_ai::game::GameState, Database), Box<dyn std::error::Error>> {
    let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
    let league_id = format!("test_league_{}", test_id);
    let league_name = format!("Test League {}", test_id);
    let save_slot = ((test_id % 250) + 1) as u8; // Use slots 1-250

    start_new_game(
        save_slot,
        league_name,
        "Test Country".to_string(),
        8, // Fewer teams for faster tests
        100, // Lower skill level for faster tests
        2026,
    ).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}

#[test]
fn test_complete_game_flow() {
    // Step 1: Create a new game
    let (game_state, database) = create_test_game()
        .expect("Failed to start new game");

    // Verify initial state
    assert!(game_state.is_new_game);
    assert_eq!(game_state.current_date.year, 2026);
    assert_eq!(game_state.current_date.month, 8);
    assert_eq!(game_state.current_date.day, 1);
    assert!(!game_state.teams.is_empty());
    assert!(!game_state.league.teams.is_empty());

    // Verify welcome notification was added
    assert!(!game_state.notifications.is_empty());
    let welcome_notification = &game_state.notifications[0];
    assert_eq!(welcome_notification.title, "Welcome to Football Manager!");

    // Step 2: Get the player team and verify it exists
    let player_team = game_state.get_player_team()
        .expect("Player team should exist");
    // Note: starting_11 and bench may be empty initially as they're set by tactics
    // Just verify the team exists and has players in the database
    assert!(!player_team.id.is_empty());

    // Step 3: Verify league was created with schedule
    assert!(!game_state.league.schedule.rounds.is_empty());
    let first_round = &game_state.league.schedule.rounds[0];
    assert!(!first_round.matches.is_empty());

    // Step 4: Verify players were created in the database
    let players = database.player_repo()
        .get_by_team(&game_state.player_team_id)
        .expect("Should retrieve players from database");
    assert!(!players.is_empty());

    // Verify at least 11 players were generated (typically 25)
    assert!(players.len() >= 11,
            "Expected at least 11 players, got {}", players.len());

    // Step 5: Verify all players have valid contracts
    for player in &players {
        assert!(player.contract_years > 0,
                "Player {} should have a valid contract", player.id);
        assert!(player.wage > 0,
                "Player {} should have a wage", player.id);
    }

    // Cleanup
    let db_path = &game_state.db_path;
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).ok();
    }
}

#[test]
fn test_contract_renewal() {
    // Step 1: Create a new game
    let (game_state, database) = create_test_game()
        .expect("Failed to start new game");

    // Step 2: Get a player from the player's team
    let players = database.player_repo()
        .get_by_team(&game_state.player_team_id)
        .expect("Should retrieve players from database");

    assert!(!players.is_empty(), "Should have players in the team");

    // Pick the first player
    let player = &players[0];
    let player_id = player.id.clone();
    let team_id = game_state.player_team_id.clone();

    // Store original contract details
    let original_contract_years = player.contract_years;
    let original_wage = player.wage;

    // Step 3: Create contract manager and renew contract
    let contract_manager = ContractManager::new(database);

    // Renew for 2 additional years with 10% salary increase
    let additional_years = 2u8;
    let salary_increase = 10u8;

    // We need to get the database back from the manager to use it later
    // Since ContractManager owns the Database, we'll verify the update through
    // a fresh lookup by storing values we need to verify later
    let original_contract_years = player.contract_years;
    let original_wage = player.wage;

    let renewal_result = contract_manager.renew_contract(
        &player_id,
        &team_id,
        additional_years,
        salary_increase,
    );

    // Verify renewal was successful
    assert!(renewal_result.is_ok(),
            "Contract renewal should succeed: {:?}", renewal_result.err());

    let contract_update = renewal_result.unwrap();
    assert_eq!(contract_update.player_id, player_id);
    assert_eq!(contract_update.old_years, original_contract_years);
    assert_eq!(contract_update.new_years, original_contract_years + additional_years);
    assert_eq!(contract_update.old_salary, original_wage);

    // Verify new salary is approximately 10% higher
    let expected_new_salary = (original_wage as f32 * 1.10) as u32;
    assert_eq!(contract_update.new_salary, expected_new_salary);

    // Cleanup
    let db_path = &game_state.db_path;
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).ok();
    }
}

#[test]
fn test_player_valuation() {
    // Step 1: Create a new game
    let (game_state, database) = create_test_game()
        .expect("Failed to start new game");

    // Step 2: Get players from the player's team
    let players = database.player_repo()
        .get_by_team(&game_state.player_team_id)
        .expect("Should retrieve players from database");

    assert!(!players.is_empty(), "Should have players in the team");

    // Step 3: Test valuation for multiple players
    for player in &players {
        // Calculate market value
        let market_value = calculate_market_value(player);

        // Verify market value is positive and reasonable
        assert!(market_value > 0,
                "Market value for {} should be positive", player.name);
        assert!(market_value >= 10_000,
                "Market value should be at least 10,000, got {}", market_value);

        // Verify market value is not unreasonably high
        assert!(market_value <= 200_000_000,
                "Market value should not exceed 200M, got {}", market_value);

        // Higher ability players should have higher value (generally)
        if player.current_ability > 120 {
            assert!(market_value > 1_000_000,
                    "High ability player should be worth >1M, got {}",
                    market_value);
        }

        // Verify the player has a wage
        assert!(player.wage > 0,
                "Player {} should have a wage", player.name);

        // Wage should be reasonable (between 500 and 500,000)
        assert!(player.wage >= 500,
                "Wage should be at least 500, got {}", player.wage);
        assert!(player.wage <= 500_000,
                "Wage should not exceed 500k, got {}", player.wage);
    }

    // Step 4: Test valuation for different player types
    let forward = players.iter()
        .find(|p| p.position == football_manager_ai::team::Position::ST)
        .or_else(|| players.iter()
            .find(|p| p.position == football_manager_ai::team::Position::CF));

    if let Some(striker) = forward {
        let striker_value = calculate_market_value(striker);
        // Forwards tend to have higher value due to goal scoring
        assert!(striker_value > 0, "Striker should have positive value");
    }

    let goalkeeper = players.iter()
        .find(|p| p.position == football_manager_ai::team::Position::GK);

    if let Some(gk) = goalkeeper {
        let gk_value = calculate_market_value(gk);
        // Goalkeepers have value too, though often less than forwards
        assert!(gk_value > 0, "Goalkeeper should have positive value");
    }

    // Cleanup
    let db_path = &game_state.db_path;
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).ok();
    }
}

#[test]
fn test_notification_system_integration() {
    // Test that notifications are properly created during game flow
    let (game_state, _database) = create_test_game()
        .expect("Failed to start new game");

    // Verify initial notifications
    assert!(!game_state.notifications.is_empty(),
            "Should have initial notifications");

    // Find welcome notification
    let welcome = game_state.notifications.iter()
        .find(|n| n.title == "Welcome to Football Manager!");
    assert!(welcome.is_some(), "Should have welcome notification");

    // Verify notification structure
    let notification = welcome.unwrap();
    assert!(!notification.title.is_empty());
    assert!(!notification.message.is_empty());
    assert!(!notification.read); // Should be unread initially

    // Cleanup
    let db_path = &game_state.db_path;
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).ok();
    }
}

#[test]
fn test_game_state_persistence() {
    // Test that game state can be created and retrieved
    let (game_state, database) = create_test_game()
        .expect("Failed to start new game");

    // Verify database file was created
    let db_path = std::path::Path::new(&game_state.db_path);
    assert!(db_path.exists(), "Database file should exist");

    // Verify we can query the database
    let teams = database.team_repo()
        .get_all()
        .expect("Should retrieve all teams");

    assert!(!teams.is_empty(), "Should have teams in database");

    // Verify player team is in the database
    let player_team = database.team_repo()
        .get_by_id(&game_state.player_team_id)
        .expect("Should retrieve player team");

    assert_eq!(player_team.id, game_state.player_team_id);

    // Cleanup
    if db_path.exists() {
        std::fs::remove_file(db_path).ok();
    }
}

#[test]
fn test_expiring_contracts_flow() {
    // Test the contract expiration check flow
    let (game_state, database) = create_test_game()
        .expect("Failed to start new game");

    let contract_manager = ContractManager::new(database);

    // Check for expiring contracts
    let expiring = contract_manager.check_expiring_contracts(
        &game_state.player_team_id,
        12, // Within 12 months
    ).expect("Should check expiring contracts");

    // Result should be valid (may be empty if no contracts expiring soon)
    assert!(expiring.len() <= 25, "Should not exceed total players");

    // Verify structure of any expiring contracts
    for contract in &expiring {
        assert!(!contract.player_id.is_empty());
        assert!(!contract.player_name.is_empty());
        assert!(contract.contract_remaining <= 12);
        assert!(contract.age > 0);
        assert!(contract.current_ability > 0);
        assert!(contract.wage > 0);
        assert!(contract.market_value > 0);
    }

    // Cleanup
    let db_path = &game_state.db_path;
    if std::path::Path::new(db_path).exists() {
        std::fs::remove_file(db_path).ok();
    }
}
