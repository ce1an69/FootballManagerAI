use crate::data::{DatabaseError, TeamFinanceRepository};
use crate::team::TeamFinance;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of TeamFinanceRepository
pub struct SqliteTeamFinanceRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteTeamFinanceRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl TeamFinanceRepository for SqliteTeamFinanceRepository {
    fn get(&self, team_id: &str) -> Result<TeamFinance, DatabaseError> {
        let conn = self.conn.read().unwrap();

        conn.query_row(
            "SELECT team_id, balance, wage_budget, transfer_budget FROM team_finance WHERE team_id = ?",
            params![team_id],
            |row| {
                Ok(TeamFinance {
                    team_id: row.get(0)?,
                    balance: row.get(1)?,
                    wage_budget: row.get(2)?,
                    transfer_budget: row.get(3)?,
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(team_id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })
    }

    fn update(&self, finance: &TeamFinance) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE team_finance SET balance = ?, wage_budget = ?, transfer_budget = ? WHERE team_id = ?",
            params![
                finance.balance,
                finance.wage_budget,
                finance.transfer_budget,
                finance.team_id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn create(&self, finance: &TeamFinance) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT INTO team_finance (team_id, balance, wage_budget, transfer_budget) VALUES (?, ?, ?, ?)",
            params![
                finance.team_id,
                finance.balance,
                finance.wage_budget,
                finance.transfer_budget,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    #[test]
    fn test_create_and_get_finance() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteTeamFinanceRepository::new(conn.clone());

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

        let finance = TeamFinance::new("team1".to_string(), 1_000_000, 100_000, 500_000);
        repo.create(&finance).unwrap();

        let retrieved = repo.get("team1").unwrap();
        assert_eq!(retrieved.team_id, "team1");
        assert_eq!(retrieved.balance, 1_000_000);
        assert_eq!(retrieved.wage_budget, 100_000);
        assert_eq!(retrieved.transfer_budget, 500_000);
    }

    #[test]
    fn test_update_finance() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteTeamFinanceRepository::new(conn.clone());

        // Create the league first (foreign key requirement)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let mut finance = TeamFinance::new("team1".to_string(), 1_000_000, 100_000, 500_000);
        repo.create(&finance).unwrap();

        finance.balance = 900_000;
        finance.transfer_budget = 400_000;
        repo.update(&finance).unwrap();

        let retrieved = repo.get("team1").unwrap();
        assert_eq!(retrieved.balance, 900_000);
        assert_eq!(retrieved.transfer_budget, 400_000);
    }

    #[test]
    fn test_get_nonexistent_finance() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteTeamFinanceRepository::new(conn.clone());

        let result = repo.get("nonexistent");
        assert!(result.is_err());
    }
}
