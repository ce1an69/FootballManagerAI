use crate::team::Player;
use crate::data::{Database, PlayerRepository, TeamRepository, LeagueRepository};
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
        // Verify player belongs to team
        let player_repo = self.database.player_repo();
        let mut player = player_repo.get_by_id(player_id)
            .map_err(|e| TransferError::DatabaseError(e.to_string()))?;

        if player.team_id.as_deref() != Some(team_id) {
            return Err(TransferError::InvalidOffer(
                format!("Player {} is not on team {}", player_id, team_id)
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

    #[test]
    fn test_check_expiring_contracts() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        let manager = ContractManager::new(db);
        // Would need to add test players with contracts
        let result = manager.check_expiring_contracts("team1", 3);
        assert!(result.is_ok());
    }

    #[test]
    fn test_salary_budget() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        // Create a league first (required by FK constraint)
        let league = crate::team::League::new("league1".to_string(), "Test League".to_string(), vec![]);
        db.league_repo().create(&league).unwrap();

        // Create a team
        let team = crate::team::Team::new("team1".to_string(), "Test Team".to_string(), "league1".to_string(), 10_000_000);
        db.team_repo().create(&team).unwrap();

        // Create a player with the team_id set
        let mut player1 = crate::team::Player::new("p1".to_string(), "Player 1".to_string(), crate::team::Position::ST);
        player1.team_id = Some("team1".to_string());
        player1.wage = 50_000; // 50k per week
        db.player_repo().create(&player1).unwrap();

        let manager = ContractManager::new(db);
        let result = manager.get_salary_budget("team1");
        assert!(result.is_ok());
        // Budget should be 20% of 10M = 2M per week, minus 50k wages = 1.95M remaining
        assert!(result.unwrap() == 1_950_000);
    }
}
