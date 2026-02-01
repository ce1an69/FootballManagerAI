use crate::data::{DatabaseError, LeagueRepository};
use crate::team::League;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of LeagueRepository
pub struct SqliteLeagueRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteLeagueRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl LeagueRepository for SqliteLeagueRepository {
    fn create(&self, league: &League) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            params![
                league.id,
                league.name,
                league.current_round,
                league.total_rounds,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<League, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let league = conn.query_row(
            "SELECT * FROM leagues WHERE id = ?",
            params![id],
            |row| {
                Ok(League {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    current_round: row.get("current_round")?,
                    total_rounds: row.get("total_rounds")?,
                    teams: Vec::new(),  // Would need to load from teams table
                    schedule: crate::team::MatchSchedule { rounds: Vec::new() },  // Would need to load from scheduled_matches table
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        Ok(league)
    }

    fn update(&self, league: &League) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE leagues SET name = ?, current_round = ?, total_rounds = ? WHERE id = ?",
            params![
                league.name,
                league.current_round,
                league.total_rounds,
                league.id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_all(&self) -> Result<Vec<League>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare("SELECT id FROM leagues ORDER BY name")
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let league_ids = stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut leagues = Vec::new();
        for league_id in league_ids {
            leagues.push(self.get_by_id(&league_id)?);
        }

        Ok(leagues)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    fn create_test_league(id: &str, name: &str) -> League {
        League::new(
            id.to_string(),
            name.to_string(),
            vec!["team1".to_string(), "team2".to_string()],
        )
    }

    #[test]
    fn test_create_and_get_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteLeagueRepository::new(conn.clone());

        let league = create_test_league("league1", "Test League");

        repo.create(&league).unwrap();
        let retrieved = repo.get_by_id("league1").unwrap();

        assert_eq!(retrieved.id, "league1");
        assert_eq!(retrieved.name, "Test League");
        assert_eq!(retrieved.current_round, 0);
        assert_eq!(retrieved.total_rounds, 2); // (2 teams - 1) * 2
    }

    #[test]
    fn test_get_all_leagues() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteLeagueRepository::new(conn.clone());

        let league1 = create_test_league("league1", "League A");
        let league2 = create_test_league("league2", "League B");

        repo.create(&league1).unwrap();
        repo.create(&league2).unwrap();

        let leagues = repo.get_all().unwrap();
        assert_eq!(leagues.len(), 2);
    }

    #[test]
    fn test_update_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteLeagueRepository::new(conn.clone());

        let mut league = create_test_league("league1", "Test League");
        repo.create(&league).unwrap();

        league.name = "Updated League".to_string();
        league.current_round = 5;
        repo.update(&league).unwrap();

        let retrieved = repo.get_by_id("league1").unwrap();
        assert_eq!(retrieved.name, "Updated League");
        assert_eq!(retrieved.current_round, 5);
    }
}
