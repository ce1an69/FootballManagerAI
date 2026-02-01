use crate::team::Player;
use crate::data::TransferHistoryRepository;
use rand::Rng;

/// Evaluate player's current market value
///
/// Considers age, ability, position, and form
pub fn evaluate_player_value(player: &Player) -> u32 {
    let mut base_value = player.current_ability as u32 * 10000;

    // Age factor
    let age_multiplier = match player.age {
        16..=21 => 1.2,      // Young players have potential value
        22..=26 => 1.0,      // Peak years
        27..=30 => 0.9,      // Slight decline
        31..=33 => 0.7,      // Decline phase
        _ => 0.5,            // Late career
    };

    base_value = (base_value as f32 * age_multiplier) as u32;

    // Position factor (some positions more valuable)
    let position_multiplier = match player.position {
        crate::team::Position::ST | crate::team::Position::CF => 1.3,
        crate::team::Position::AM | crate::team::Position::LW | crate::team::Position::RW => 1.2,
        crate::team::Position::CM => 1.1,
        crate::team::Position::CB => 1.05,
        crate::team::Position::GK => 1.0,
        _ => 1.0,
    };

    base_value = (base_value as f32 * position_multiplier) as u32;

    // Form/morale factor
    let form_factor = if player.morale > 70 {
        1.0 + ((player.morale - 70) as f32 / 100.0 * 0.2) // Up to +6%
    } else {
        0.9 + ((player.morale as f32 / 70.0) * 0.1) // Down to 10% reduction
    };

    base_value = (base_value as f32 * form_factor) as u32;

    // Injury reduces value
    if let Some(injury_days) = player.injury_days {
        let injury_penalty = (injury_days as f32 / 365.0) * 0.5; // Max 50% penalty for year-long injury
        base_value = (base_value as f32 * (1.0 - injury_penalty)) as u32;
    }

    // Contract years remaining (players with expiring contracts are cheaper)
    if player.contract_years > 2 {
        base_value = (base_value as f32 * 1.1) as u32; // +10% for long-term security
    } else if player.contract_years == 0 {
        base_value = (base_value as f32 * 0.7) as u32; // -30% for expiring contract
    }

    // Add random variance (+/- 5%)
    let mut rng = rand::thread_rng();
    let variance = rng.gen_range(-5..=5);
    base_value = (base_value as f32 * (1.0 + variance as f32 / 100.0)) as u32;

    base_value.max(100_000) // Minimum value
}

/// Predict player's future potential value
///
/// Estimates what player could be worth in future based on age and potential ability
pub fn predict_potential_value(player: &Player, future_years: u8) -> u32 {
    // Current value
    let current_value = evaluate_player_value(player);

    // Age development curve
    let development_multiplier = match player.age {
        age if age + future_years as u8 <= 21 => {
            // Young player will develop
            let growth = (player.potential_ability as f32 - player.current_ability as f32) / 200.0;
            1.0 + growth.min(0.5) // Up to +50% increase
        }
        age if age + future_years as u8 <= 26 => {
            // Peak years - stable or slight growth
            1.05
        }
        age if age + future_years as u8 <= 30 => {
            // Decline phase
            let decline = ((age + future_years as u8 - 30) as f32) * 0.1; // -10% per year over 30
            (1.0 - decline).max(0.5)
        }
        _ => {
            // Late career - significant decline expected
            0.5
        }
    };

    let predicted_value = (current_value as f32 * development_multiplier) as u32;

    // Add uncertainty based on prediction distance
    let uncertainty_factor = match future_years {
        1 => 1.0,
        2 => 0.95,
        3 => 0.88,
        4 => 0.8,
        _ => 0.7,
    };

    ((predicted_value as f32) * uncertainty_factor) as u32
}

/// Transfer history entry
#[derive(Debug, Clone)]
pub struct TransferHistoryEntry {
    pub id: String,
    pub player_id: String,
    pub player_name: String,
    pub from_team_id: String,
    pub to_team_id: Option<String>, // None if released
    pub transfer_type: TransferHistoryType,
    pub fee: u32,
    pub date: String,
}

/// Transfer history type
#[derive(Debug, Clone, PartialEq)]
pub enum TransferHistoryType {
    Purchase,
    Sale,
    Loan,
    Release,
    FreeTransfer,
    Exchange,
}

/// Transfer history manager
pub struct TransferHistory {
    database: Option<crate::data::Database>,
    entries: Vec<TransferHistoryEntry>,
}

impl TransferHistory {
    /// Create new transfer history
    pub fn new() -> Self {
        Self {
            database: None,
            entries: Vec::new(),
        }
    }

    /// Create with database connection
    pub fn with_database(database: crate::data::Database) -> Self {
        let entries = Self::load_from_database(&database).unwrap_or_default();
        Self {
            database: Some(database),
            entries,
        }
    }

    /// Record a transfer
    pub fn record_transfer(
        &mut self,
        player_id: &str,
        player_name: &str,
        from_team_id: &str,
        to_team_id: Option<&str>,
        transfer_type: TransferHistoryType,
        fee: u32,
        date: &str,
    ) -> Result<(), crate::data::DatabaseError> {
        let entry = TransferHistoryEntry {
            id: uuid::Uuid::new_v4().to_string(),
            player_id: player_id.to_string(),
            player_name: player_name.to_string(),
            from_team_id: from_team_id.to_string(),
            to_team_id: to_team_id.map(|s| s.to_string()),
            transfer_type,
            fee,
            date: date.to_string(),
        };

        // Add to in-memory history
        self.entries.push(entry.clone());

        // Save to database if available
        if let Some(db) = &self.database {
            let repo = db.transfer_history_repo();
            repo.create(&entry)?;
        }

        Ok(())
    }

    /// Get transfer history for a player
    pub fn get_player_history(&self, player_id: &str) -> Vec<&TransferHistoryEntry> {
        self.entries.iter()
            .filter(|e| e.player_id == player_id)
            .collect()
    }

    /// Get transfer history for a team
    pub fn get_team_history(&self, team_id: &str) -> Vec<&TransferHistoryEntry> {
        self.entries.iter()
            .filter(|e| e.from_team_id == team_id || e.to_team_id.as_deref() == Some(team_id))
            .collect()
    }

    /// Get recent transfers
    pub fn get_recent_transfers(&self, limit: usize) -> Vec<&TransferHistoryEntry> {
        let mut sorted: Vec<&TransferHistoryEntry> = self.entries.iter().collect();
        sorted.sort_by(|a, b| b.date.cmp(&a.date)); // Most recent first
        sorted.into_iter().take(limit).collect()
    }

    /// Calculate total transfer spending for a team
    pub fn calculate_spending(&self, team_id: &str) -> u32 {
        self.entries.iter()
            .filter(|e| e.to_team_id.as_deref() == Some(team_id))
            .filter(|e| matches!(e.transfer_type, TransferHistoryType::Purchase | TransferHistoryType::Exchange))
            .map(|e| e.fee)
            .sum()
    }

    /// Calculate total transfer income for a team
    pub fn calculate_income(&self, team_id: &str) -> u32 {
        self.entries.iter()
            .filter(|e| e.from_team_id == team_id)
            .filter(|e| matches!(e.transfer_type, TransferHistoryType::Sale | TransferHistoryType::Exchange))
            .map(|e| e.fee)
            .sum()
    }

    /// Load transfer history from database
    fn load_from_database(database: &crate::data::Database) -> Result<Vec<TransferHistoryEntry>, crate::data::DatabaseError> {
        let repo = database.transfer_history_repo();
        // Load recent transfers (limit to 1000 for performance)
        repo.get_recent(1000)
    }

    /// Get transfer statistics for a team
    pub fn get_team_statistics(&self, team_id: &str) -> TransferStatistics {
        let history = self.get_team_history(team_id);

        let purchases = history.iter()
            .filter(|e| e.to_team_id.as_deref() == Some(team_id))
            .filter(|e| matches!(e.transfer_type, TransferHistoryType::Purchase | TransferHistoryType::Exchange))
            .count();

        let sales = history.iter()
            .filter(|e| e.from_team_id == team_id)
            .filter(|e| matches!(e.transfer_type, TransferHistoryType::Sale))
            .count();

        let total_spent = self.calculate_spending(team_id);
        let total_income = self.calculate_income(team_id);

        let biggest_purchase = history.iter()
            .filter(|e| e.from_team_id == team_id)
            .filter_map(|e| {
                if matches!(e.transfer_type, TransferHistoryType::Purchase | TransferHistoryType::Exchange) {
                    Some(e.fee)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        let biggest_sale = history.iter()
            .filter(|e| e.from_team_id == team_id)
            .filter_map(|e| {
                if matches!(e.transfer_type, TransferHistoryType::Sale) {
                    Some(e.fee)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        TransferStatistics {
            total_purchases: purchases,
            total_sales: sales,
            total_spent,
            total_income,
            biggest_purchase,
            biggest_sale,
            net_spending: total_spent as i32 - total_income as i32,
        }
    }
}

/// Transfer statistics for a team
#[derive(Debug, Clone)]
pub struct TransferStatistics {
    pub total_purchases: usize,
    pub total_sales: usize,
    pub total_spent: u32,
    pub total_income: u32,
    pub biggest_purchase: u32,
    pub biggest_sale: u32,
    pub net_spending: i32,
}

impl Default for TransferHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::Player;
    use crate::team::Position;

    #[test]
    fn test_evaluate_player_value() {
        let mut player = Player::new("test".to_string(), "Test Player".to_string(), Position::CM);
        player.age = 25;
        player.current_ability = 120;
        player.morale = 80;
        player.contract_years = 3;

        let value = evaluate_player_value(&player);
        assert!(value > 1_000_000); // Should be worth at least this
    }

    #[test]
    fn test_predict_potential_value() {
        let mut player = Player::new("test".to_string(), "Test Player".to_string(), Position::ST);
        player.age = 18; // Very young player
        player.current_ability = 100;
        player.potential_ability = 160; // High potential

        let current = evaluate_player_value(&player);
        let future = predict_potential_value(&player, 1); // 1 year prediction

        assert!(future > current); // Young player should develop
    }

    #[test]
    fn test_transfer_history() {
        let mut history = TransferHistory::new();

        history.record_transfer(
            "player1",
            "John Doe",
            "team1",
            Some("team2"),
            TransferHistoryType::Purchase,
            5_000_000,
            "2026-01-15"
        ).unwrap();

        assert_eq!(history.entries.len(), 1);
        assert_eq!(history.get_player_history("player1").len(), 1);
    }

    #[test]
    fn test_team_statistics() {
        let mut history = TransferHistory::new();

        // Add some transfers
        // team1 buys from team2 (from team2's perspective, to team1)
        history.record_transfer("p1", "Player 1", "team2", Some("team1"),
            TransferHistoryType::Purchase, 5_000_000, "2026-01-15").unwrap();
        // team1 sells to team3 (from team1's perspective, to team3)
        history.record_transfer("p2", "Player 2", "team1", Some("team3"),
            TransferHistoryType::Sale, 3_000_000, "2026-01-16").unwrap();

        let stats = history.get_team_statistics("team1");
        assert_eq!(stats.total_purchases, 1);
        assert_eq!(stats.total_sales, 1);
        assert_eq!(stats.total_spent, 5_000_000);
        assert_eq!(stats.total_income, 3_000_000);
        assert_eq!(stats.net_spending, 2_000_000);
    }
}
