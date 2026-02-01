use crate::data::{DatabaseError, FinanceTransactionRepository};
use crate::team::{FinanceTransaction, TransactionType};
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of FinanceTransactionRepository
pub struct SqliteFinanceTransactionRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteFinanceTransactionRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    /// Helper to parse transaction type from string
    fn parse_transaction_type(s: &str) -> Result<TransactionType, DatabaseError> {
        match s {
            "TransferIncome" => Ok(TransactionType::TransferIncome),
            "MatchdayIncome" => Ok(TransactionType::MatchdayIncome),
            "Sponsorship" => Ok(TransactionType::Sponsorship),
            "TVRevenue" => Ok(TransactionType::TVRevenue),
            "PrizeMoneyWin" => Ok(TransactionType::PrizeMoneyWin),
            "TransferExpense" => Ok(TransactionType::TransferExpense),
            "WagePayment" => Ok(TransactionType::WagePayment),
            "BonusPayment" => Ok(TransactionType::BonusPayment),
            "StaffWages" => Ok(TransactionType::StaffWages),
            "Facilities" => Ok(TransactionType::Facilities),
            "YouthAcademy" => Ok(TransactionType::YouthAcademy),
            _ => Err(DatabaseError::QueryError(format!("Unknown transaction type: {}", s))),
        }
    }

    /// Helper to convert transaction type to string
    fn transaction_type_to_string(t: &TransactionType) -> &'static str {
        match t {
            TransactionType::TransferIncome => "TransferIncome",
            TransactionType::MatchdayIncome => "MatchdayIncome",
            TransactionType::Sponsorship => "Sponsorship",
            TransactionType::TVRevenue => "TVRevenue",
            TransactionType::PrizeMoneyWin => "PrizeMoneyWin",
            TransactionType::TransferExpense => "TransferExpense",
            TransactionType::WagePayment => "WagePayment",
            TransactionType::BonusPayment => "BonusPayment",
            TransactionType::StaffWages => "StaffWages",
            TransactionType::Facilities => "Facilities",
            TransactionType::YouthAcademy => "YouthAcademy",
        }
    }
}

impl FinanceTransactionRepository for SqliteFinanceTransactionRepository {
    fn create(&self, transaction: &FinanceTransaction) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        let type_str = Self::transaction_type_to_string(&transaction.transaction_type);

        conn.execute(
            "INSERT INTO finance_transactions (id, team_id, transaction_type, amount, description, date_year, date_month, date_day, related_player_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                transaction.id,
                transaction.team_id,
                type_str,
                transaction.amount,
                transaction.description,
                transaction.date_year,
                transaction.date_month,
                transaction.date_day,
                transaction.related_player_id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<FinanceTransaction>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, team_id, transaction_type, amount, description, date_year, date_month, date_day, related_player_id
             FROM finance_transactions
             WHERE team_id = ?
             ORDER BY date_year DESC, date_month DESC, date_day DESC
             LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let transactions = stmt.query_map(params![team_id, limit], |row| {
            let type_str: String = row.get(2)?;
            let transaction_type = Self::parse_transaction_type(&type_str)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(FinanceTransaction {
                id: row.get(0)?,
                team_id: row.get(1)?,
                transaction_type,
                amount: row.get(3)?,
                description: row.get(4)?,
                date_year: row.get(5)?,
                date_month: row.get(6)?,
                date_day: row.get(7)?,
                related_player_id: row.get(8)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(transactions)
    }

    fn get_by_date_range(
        &self,
        team_id: &str,
        start_year: u32,
        start_month: u8,
        start_day: u8,
        end_year: u32,
        end_month: u8,
        end_day: u8,
    ) -> Result<Vec<FinanceTransaction>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id, team_id, transaction_type, amount, description, date_year, date_month, date_day, related_player_id
             FROM finance_transactions
             WHERE team_id = ?
             AND date_year >= ? AND date_month >= ? AND date_day >= ?
             AND date_year <= ? AND date_month <= ? AND date_day <= ?
             ORDER BY date_year DESC, date_month DESC, date_day DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let transactions = stmt.query_map(
            params![
                team_id,
                start_year, start_month, start_day,
                end_year, end_month, end_day
            ],
            |row| {
                let type_str: String = row.get(2)?;
                let transaction_type = Self::parse_transaction_type(&type_str)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

                Ok(FinanceTransaction {
                    id: row.get(0)?,
                    team_id: row.get(1)?,
                    transaction_type,
                    amount: row.get(3)?,
                    description: row.get(4)?,
                    date_year: row.get(5)?,
                    date_month: row.get(6)?,
                    date_day: row.get(7)?,
                    related_player_id: row.get(8)?,
                })
            }
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(transactions)
    }

    fn get_by_type(
        &self,
        team_id: &str,
        t_type: &TransactionType,
    ) -> Result<Vec<FinanceTransaction>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let type_str = Self::transaction_type_to_string(t_type);

        let mut stmt = conn.prepare(
            "SELECT id, team_id, transaction_type, amount, description, date_year, date_month, date_day, related_player_id
             FROM finance_transactions
             WHERE team_id = ? AND transaction_type = ?
             ORDER BY date_year DESC, date_month DESC, date_day DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let transactions = stmt.query_map(params![team_id, type_str], |row| {
            let type_str: String = row.get(2)?;
            let transaction_type = Self::parse_transaction_type(&type_str)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

            Ok(FinanceTransaction {
                id: row.get(0)?,
                team_id: row.get(1)?,
                transaction_type,
                amount: row.get(3)?,
                description: row.get(4)?,
                date_year: row.get(5)?,
                date_month: row.get(6)?,
                date_day: row.get(7)?,
                related_player_id: row.get(8)?,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(transactions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;
    use uuid::Uuid;

    #[test]
    fn test_create_and_get_transaction() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteFinanceTransactionRepository::new(conn.clone());

        // Create the league first (foreign key requirement)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first (foreign key requirement)
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        // Create a player for the transaction reference (optional)
        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["player1", "team1", "Player 1", 25, "England", "ST", "Right", 180, 75, 10000, 3, 5000000],
        ).unwrap();

        let transaction = FinanceTransaction {
            id: Uuid::new_v4().to_string(),
            team_id: "team1".to_string(),
            transaction_type: TransactionType::TransferIncome,
            amount: 5000000,
            description: "Player sale".to_string(),
            date_year: 2026,
            date_month: 7,
            date_day: 15,
            related_player_id: Some("player1".to_string()),
        };

        repo.create(&transaction).unwrap();

        let transactions = repo.get_by_team("team1", 10).unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].amount, 5000000);
    }

    #[test]
    fn test_get_by_type() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteFinanceTransactionRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let transaction1 = FinanceTransaction {
            id: Uuid::new_v4().to_string(),
            team_id: "team1".to_string(),
            transaction_type: TransactionType::TransferIncome,
            amount: 5000000,
            description: "Player sale".to_string(),
            date_year: 2026,
            date_month: 7,
            date_day: 15,
            related_player_id: None,
        };

        let transaction2 = FinanceTransaction {
            id: Uuid::new_v4().to_string(),
            team_id: "team1".to_string(),
            transaction_type: TransactionType::WagePayment,
            amount: -100000,
            description: "Weekly wages".to_string(),
            date_year: 2026,
            date_month: 7,
            date_day: 16,
            related_player_id: None,
        };

        repo.create(&transaction1).unwrap();
        repo.create(&transaction2).unwrap();

        let income_transactions = repo.get_by_type("team1", &TransactionType::TransferIncome).unwrap();
        assert_eq!(income_transactions.len(), 1);
        assert_eq!(income_transactions[0].transaction_type, TransactionType::TransferIncome);
    }
}
