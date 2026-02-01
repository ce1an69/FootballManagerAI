use crate::team::Player;
use crate::data::{Database, PlayerRepository, TeamRepository};
use crate::transfer::market::TransferError;
use rand::Rng;

/// Contract management
pub struct ContractManager {
    database: Database,
}

impl ContractManager {
    /// Create new contract manager
    pub fn new(database: Database) -> Self {
        Self { database }
    }

    /// Renew player contract
    ///
    /// Increases contract years and adjusts salary based on player performance
    pub fn renew_contract(
        &self,
        player_id: &str,
        team_id: &str,
        additional_years: u8,
        salary_increase_percent: u8,
    ) -> Result<ContractUpdate, TransferError> {
        // Validate additional_years
        if additional_years == 0 {
            return Err(TransferError::InvalidOffer(
                "Additional years must be greater than 0".to_string()
            ));
        }
        if additional_years > 5 {
            return Err(TransferError::InvalidOffer(
                "Additional years cannot exceed 5".to_string()
            ));
        }

        // Verify player belongs to team
        let player_repo = self.database.player_repo();
        let mut player = player_repo.get_by_id(player_id)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

        if player.team_id.as_deref() != Some(team_id) {
            return Err(TransferError::InvalidOffer(
                format!("Player {} is not on team {}", player_id, team_id)
            ));
        }

        // Check if contract has already expired
        if player.contract_years == 0 {
            return Err(TransferError::InvalidOffer(
                "Cannot renew an expired contract".to_string()
            ));
        }

        // Check if player wants to renew
        if !self.player_wants_renew(&player) {
            return Err(TransferError::InvalidOffer(
                format!("Player {} rejected renewal: wants to leave the club", player_id)
            ));
        }

        // Calculate new salary
        let old_salary = player.wage;
        let new_salary = (old_salary as f32 * (1.0 + salary_increase_percent as f32 / 100.0)) as u32;

        // Update contract
        let old_years = player.contract_years;
        player.contract_years = player.contract_years.saturating_add(additional_years);
        player.wage = new_salary;

        // Save to database
        player_repo.update(&player)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

        Ok(ContractUpdate {
            player_id: player_id.to_string(),
            old_salary,
            new_salary,
            old_years,
            new_years: player.contract_years,
        })
    }

    /// Check if player is willing to renew contract
    fn player_wants_renew(&self, player: &Player) -> bool {
        // Factors that affect willingness to renew:
        // 1. Playing time (simulated - would need match data)
        // 2. Team performance
        // 3. Morale
        // 4. Age and potential
        // 5. Current wage vs market value

        let mut willingness_score = 50; // Base score

        // Age factor: younger players more willing to renew
        if player.age < 24 {
            willingness_score += 20;
        } else if player.age > 30 {
            willingness_score -= 10;
        }

        // Morale factor
        willingness_score += player.morale as i32 / 2;

        // Ability factor: better players have more options
        if player.current_ability > 140 {
            willingness_score -= 15;
        } else if player.current_ability < 100 {
            willingness_score += 15;
        }

        // Random factor
        let mut rng = rand::thread_rng();
        willingness_score += rng.gen_range(-10..=10);

        willingness_score > 50
    }

    /// Check contracts expiring soon
    ///
    /// Returns players with contracts ending in specified months
    pub fn check_expiring_contracts(
        &self,
        team_id: &str,
        within_months: u8,
    ) -> Result<Vec<ExpiringContract>, TransferError> {
        let player_repo = self.database.player_repo();
        let players = player_repo.get_by_team(team_id)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

        let mut expiring = Vec::new();

        for player in players {
            if player.contract_years <= within_months {
                expiring.push(ExpiringContract {
                    player_id: player.id.clone(),
                    player_name: player.name.clone(),
                    contract_remaining: player.contract_years,
                    age: player.age,
                    current_ability: player.current_ability,
                    wage: player.wage,
                    market_value: player.market_value,
                });
            }
        }

        // Sort by contract remaining (ascending)
        expiring.sort_by(|a, b| a.contract_remaining.cmp(&b.contract_remaining));

        Ok(expiring)
    }

    /// Process contract expiration at end of season
    ///
    /// Players with expiring contracts may leave, become free agents, or demand higher wages
    pub fn process_contract_expiration(
        &self,
        team_id: &str,
    ) -> Result<ContractExpirationResult, TransferError> {
        let expiring = self.check_expiring_contracts(team_id, 0)?;

        let mut leaving = Vec::new();
        let mut renewing = Vec::new();
        let mut demanding = Vec::new();

        for contract in expiring {
            let player_repo = self.database.player_repo();
            let player = player_repo.get_by_id(&contract.player_id)
                .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

            // Determine outcome based on player attributes
            let outcome = self.determine_contract_outcome(&player);

            match outcome {
                ContractOutcome::Leaves => {
                    // Make player free agent
                    let mut updated_player = player.clone();
                    updated_player.team_id = None;
                    updated_player.contract_years = 0;
                    player_repo.update(&updated_player)
                        .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

                    leaving.push(contract.player_id.clone());
                }
                ContractOutcome::Renews => {
                    // Renew contract for 1-2 years
                    let new_years = rand::thread_rng().gen_range(1..=3);
                    let mut updated_player = player.clone();
                    updated_player.contract_years = new_years;
                    player_repo.update(&updated_player)
                        .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

                    renewing.push(contract.player_id.clone());
                }
                ContractOutcome::DemandsRaise => {
                    // Demands salary increase
                    let raise_percent = rand::thread_rng().gen_range(10..=50);
                    demanding.push(Demand {
                        player_id: contract.player_id.clone(),
                        player_name: contract.player_name.clone(),
                        current_wage: contract.wage,
                        demanded_increase: raise_percent,
                    });
                }
            }
        }

        Ok(ContractExpirationResult {
            leaving,
            renewing,
            demanding,
        })
    }

    /// Determine what happens when contract expires
    fn determine_contract_outcome(&self, player: &Player) -> ContractOutcome {
        let mut rng = rand::thread_rng();

        // Calculate probability of leaving
        let leave_probability = if player.age > 32 {
            40 // Older players more likely to retire or leave
        } else if player.current_ability > 150 {
            30 // Star players may want bigger clubs
        } else if player.morale < 50 {
            50 // Unhappy players likely to leave
        } else if player.contract_years == 0 {
            100 // Contract expired, must decide
        } else {
            20 // Most players renew
        };

        let roll = rng.gen_range(0..=100);

        if roll < leave_probability {
            ContractOutcome::Leaves
        } else if player.morale < 60 || player.current_ability > 140 {
            ContractOutcome::DemandsRaise
        } else {
            ContractOutcome::Renews
        }
    }

    /// Get salary budget for a team
    pub fn get_salary_budget(&self, team_id: &str) -> Result<u32, TransferError> {
        let team = self.database.team_repo().get_by_id(team_id)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

        // Calculate total wage bill
        let player_repo = self.database.player_repo();
        let players = player_repo.get_by_team(team_id)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;
        let total_wages = players.iter().map(|p| p.wage).sum::<u32>();

        // Weekly budget = 20% of team budget (simplified)
        let weekly_budget = (team.budget as f32 * 0.2) as u32;

        Ok(weekly_budget.saturating_sub(total_wages))
    }

    /// Can team afford player with given wage
    pub fn can_afford_player(
        &self,
        team_id: &str,
        wage: u32,
    ) -> Result<bool, TransferError> {
        let remaining_budget = self.get_salary_budget(team_id)?;
        Ok(remaining_budget >= wage)
    }
}

/// Contract update result
#[derive(Debug, Clone)]
pub struct ContractUpdate {
    pub player_id: String,
    pub old_salary: u32,
    pub new_salary: u32,
    pub old_years: u8,
    pub new_years: u8,
}

/// Expiring contract information
#[derive(Debug, Clone)]
pub struct ExpiringContract {
    pub player_id: String,
    pub player_name: String,
    pub contract_remaining: u8,
    pub age: u8,
    pub current_ability: u16,
    pub wage: u32,
    pub market_value: u32,
}

/// Contract demand during negotiations
#[derive(Debug, Clone)]
pub struct Demand {
    pub player_id: String,
    pub player_name: String,
    pub current_wage: u32,
    pub demanded_increase: u8, // Percentage
}

/// Contract expiration result
#[derive(Debug, Clone)]
pub struct ContractExpirationResult {
    pub leaving: Vec<String>,   // Player IDs leaving team
    pub renewing: Vec<String>,  // Player IDs renewing
    pub demanding: Vec<Demand>,  // Players demanding raises
}

/// Contract outcome when expiring
#[derive(Debug, Clone, PartialEq)]
enum ContractOutcome {
    Leaves,
    Renews,
    DemandsRaise,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    fn setup_test_data() -> Database {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        // Create a league first (required by FK constraint)
        let league = crate::team::League::new("league1".to_string(), "Test League".to_string(), vec![]);
        db.league_repo().create(&league).unwrap();

        // Create a team
        let team = crate::team::Team::new("team1".to_string(), "Test Team".to_string(), "league1".to_string(), 10_000_000);
        db.team_repo().create(&team).unwrap();

        db
    }

    #[test]
    fn test_renew_contract() {
        let db = setup_test_data();

        // Create a player with a contract
        let mut player = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player.team_id = Some("team1".to_string());
        player.contract_years = 2;
        player.wage = 50_000;
        player.morale = 80; // High morale to ensure they want to renew
        player.age = 25; // Prime age
        db.player_repo().create(&player).unwrap();

        let manager = ContractManager::new(db);

        // Test successful renewal
        let result = manager.renew_contract("p1", "team1", 2, 10);
        assert!(result.is_ok());

        let update = result.unwrap();
        assert_eq!(update.player_id, "p1");
        assert_eq!(update.old_salary, 50_000);
        assert_eq!(update.new_salary, 55_000); // 50k + 10%
        assert_eq!(update.old_years, 2);
        assert_eq!(update.new_years, 4); // 2 + 2
    }

    #[test]
    fn test_renew_contract_invalid_years_zero() {
        let db = setup_test_data();

        let mut player = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player.team_id = Some("team1".to_string());
        player.contract_years = 2;
        player.wage = 50_000;
        db.player_repo().create(&player).unwrap();

        let manager = ContractManager::new(db);

        // Test that 0 years is rejected
        let result = manager.renew_contract("p1", "team1", 0, 10);
        assert!(result.is_err());
        match result.unwrap_err() {
            TransferError::InvalidOffer(msg) => {
                assert!(msg.contains("greater than 0") || msg.contains("must be greater"));
            }
            _ => panic!("Expected InvalidOffer error"),
        }
    }

    #[test]
    fn test_renew_contract_invalid_years_too_many() {
        let db = setup_test_data();

        let mut player = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player.team_id = Some("team1".to_string());
        player.contract_years = 2;
        player.wage = 50_000;
        db.player_repo().create(&player).unwrap();

        let manager = ContractManager::new(db);

        // Test that >5 years is rejected
        let result = manager.renew_contract("p1", "team1", 6, 10);
        assert!(result.is_err());
        match result.unwrap_err() {
            TransferError::InvalidOffer(msg) => {
                assert!(msg.contains("cannot exceed 5") || msg.contains("exceed"));
            }
            _ => panic!("Expected InvalidOffer error"),
        }
    }

    #[test]
    fn test_renew_contract_expired_contract() {
        let db = setup_test_data();

        let mut player = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player.team_id = Some("team1".to_string());
        player.contract_years = 0; // Already expired
        player.wage = 50_000;
        db.player_repo().create(&player).unwrap();

        let manager = ContractManager::new(db);

        // Test that expired contract cannot be renewed
        let result = manager.renew_contract("p1", "team1", 2, 10);
        assert!(result.is_err());
        match result.unwrap_err() {
            TransferError::InvalidOffer(msg) => {
                assert!(msg.contains("expired") || msg.contains("Cannot renew"));
            }
            _ => panic!("Expected InvalidOffer error for expired contract"),
        }
    }

    #[test]
    fn test_renew_contract_wrong_team() {
        let db = setup_test_data();

        // Create another team
        let team2 = crate::team::Team::new("team2".to_string(), "Team 2".to_string(), "league1".to_string(), 5_000_000);
        db.team_repo().create(&team2).unwrap();

        let mut player = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player.team_id = Some("team2".to_string()); // Player is on team2
        player.contract_years = 2;
        player.wage = 50_000;
        db.player_repo().create(&player).unwrap();

        let manager = ContractManager::new(db);

        // Try to renew from team1 (should fail)
        let result = manager.renew_contract("p1", "team1", 2, 10);
        assert!(result.is_err());
        match result.unwrap_err() {
            TransferError::InvalidOffer(msg) => {
                assert!(msg.contains("not on team"));
            }
            _ => panic!("Expected InvalidOffer error for wrong team"),
        }
    }

    #[test]
    fn test_check_expiring_contracts() {
        let db = setup_test_data();

        // Create players with different contract lengths
        let mut player1 = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player1.team_id = Some("team1".to_string());
        player1.contract_years = 12; // 1 year
        player1.age = 25;
        player1.current_ability = 120;
        player1.wage = 50_000;
        player1.market_value = 5_000_000;

        let mut player2 = crate::team::Player::new("p2".to_string(), "Player 2".to_string(), crate::team::Position::CM);
        player2.team_id = Some("team1".to_string());
        player2.contract_years = 6; // 6 months
        player2.age = 28;
        player2.current_ability = 135;
        player2.wage = 80_000;
        player2.market_value = 10_000_000;

        let mut player3 = crate::team::Player::new("p3".to_string(), "Player 3".to_string(), crate::team::Position::CB);
        player3.team_id = Some("team1".to_string());
        player3.contract_years = 36; // 3 years - should not appear
        player3.age = 24;
        player3.current_ability = 110;
        player3.wage = 40_000;
        player3.market_value = 3_000_000;

        db.player_repo().create(&player1).unwrap();
        db.player_repo().create(&player2).unwrap();
        db.player_repo().create(&player3).unwrap();

        let manager = ContractManager::new(db);

        // Check for contracts expiring within 12 months
        let result = manager.check_expiring_contracts("team1", 12);
        assert!(result.is_ok());

        let expiring = result.unwrap();
        assert_eq!(expiring.len(), 2); // Only player1 and player2

        // Should be sorted by contract remaining (ascending)
        assert_eq!(expiring[0].player_id, "p2"); // 6 months
        assert_eq!(expiring[1].player_id, "p1"); // 12 months

        // Verify player2 details
        assert_eq!(expiring[0].contract_remaining, 6);
        assert_eq!(expiring[0].age, 28);
        assert_eq!(expiring[0].current_ability, 135);
        assert_eq!(expiring[0].wage, 80_000);
        assert_eq!(expiring[0].market_value, 10_000_000);
    }

    #[test]
    fn test_check_expiring_contracts_empty_team() {
        let db = setup_test_data();
        let manager = ContractManager::new(db);

        // Check for team with no players
        let result = manager.check_expiring_contracts("team1", 12);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_salary_budget() {
        let db = setup_test_data();

        // Create players with the team_id set
        let mut player1 = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player1.team_id = Some("team1".to_string());
        player1.wage = 50_000; // 50k per week

        let mut player2 = crate::team::Player::new("p2".to_string(), "Player 2".to_string(), crate::team::Position::CM);
        player2.team_id = Some("team1".to_string());
        player2.wage = 80_000; // 80k per week

        db.player_repo().create(&player1).unwrap();
        db.player_repo().create(&player2).unwrap();

        let manager = ContractManager::new(db);
        let result = manager.get_salary_budget("team1");
        assert!(result.is_ok());
        // Budget should be 20% of 10M = 2M per week, minus 130k wages = 1.87M remaining
        assert_eq!(result.unwrap(), 1_870_000);
    }

    #[test]
    fn test_can_afford_player() {
        let db = setup_test_data();

        let mut player1 = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player1.team_id = Some("team1".to_string());
        player1.wage = 50_000;
        db.player_repo().create(&player1).unwrap();

        let manager = ContractManager::new(db);

        // Should be able to afford a 100k wage player
        assert!(manager.can_afford_player("team1", 100_000).unwrap());

        // Should not be able to afford a 3M wage player
        assert!(!manager.can_afford_player("team1", 3_000_000).unwrap());
    }
}
