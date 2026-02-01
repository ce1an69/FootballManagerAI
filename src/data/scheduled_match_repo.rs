use crate::data::{DatabaseError, ScheduledMatchRepository, ScheduledMatchData};
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of ScheduledMatchRepository
pub struct SqliteScheduledMatchRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteScheduledMatchRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl ScheduledMatchRepository for SqliteScheduledMatchRepository {
    fn create(&self, scheduled_match: &ScheduledMatchData) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT INTO scheduled_matches (id, league_id, round_number, home_team_id, away_team_id, played) VALUES (?, ?, ?, ?, ?, ?)",
            params![
                scheduled_match.id,
                scheduled_match.league_id,
                scheduled_match.round_number,
                scheduled_match.home_team_id,
                scheduled_match.away_team_id,
                scheduled_match.played,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<ScheduledMatchData, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let scheduled_match = conn.query_row(
            "SELECT * FROM scheduled_matches WHERE id = ?",
            params![id],
            |row| {
                Ok(ScheduledMatchData {
                    id: row.get("id")?,
                    league_id: row.get("league_id")?,
                    round_number: row.get("round_number")?,
                    home_team_id: row.get("home_team_id")?,
                    away_team_id: row.get("away_team_id")?,
                    played: row.get::<_, i32>("played").ok().map(|v| v != 0).unwrap_or(false),
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        Ok(scheduled_match)
    }

    fn get_by_league(&self, league_id: &str) -> Result<Vec<ScheduledMatchData>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id FROM scheduled_matches WHERE league_id = ? ORDER BY round_number, home_team_id"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let match_ids = stmt.query_map(params![league_id], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut matches = Vec::new();
        for match_id in match_ids {
            matches.push(self.get_by_id(&match_id)?);
        }

        Ok(matches)
    }

    fn get_by_round(&self, league_id: &str, round: u32) -> Result<Vec<ScheduledMatchData>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id FROM scheduled_matches WHERE league_id = ? AND round_number = ? ORDER BY home_team_id"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let match_ids = stmt.query_map(params![league_id, round], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut matches = Vec::new();
        for match_id in match_ids {
            matches.push(self.get_by_id(&match_id)?);
        }

        Ok(matches)
    }

    fn mark_as_played(&self, id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE scheduled_matches SET played = 1 WHERE id = ?",
            params![id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn delete_by_league(&self, league_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "DELETE FROM scheduled_matches WHERE league_id = ?",
            params![league_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    fn create_test_scheduled_match(id: &str, league_id: &str, round: u32) -> ScheduledMatchData {
        ScheduledMatchData {
            id: id.to_string(),
            league_id: league_id.to_string(),
            round_number: round,
            home_team_id: "team1".to_string(),
            away_team_id: "team2".to_string(),
            played: false,
        }
    }

    #[test]
    fn test_create_and_get_scheduled_match() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteScheduledMatchRepository::new(conn.clone());

        // Create league
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create teams
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let scheduled_match = create_test_scheduled_match("match1", "league1", 1);

        repo.create(&scheduled_match).unwrap();
        let retrieved = repo.get_by_id("match1").unwrap();

        assert_eq!(retrieved.id, "match1");
        assert_eq!(retrieved.league_id, "league1");
        assert_eq!(retrieved.round_number, 1);
        assert_eq!(retrieved.home_team_id, "team1");
        assert_eq!(retrieved.away_team_id, "team2");
        assert_eq!(retrieved.played, false);
    }

    #[test]
    fn test_get_by_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteScheduledMatchRepository::new(conn.clone());

        // Create leagues
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league2", "League 2", 0, 38],
        ).unwrap();

        // Create teams
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team3", "Team 3", "league2", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match1 = ScheduledMatchData {
            id: "match1".to_string(),
            league_id: "league1".to_string(),
            round_number: 1,
            home_team_id: "team1".to_string(),
            away_team_id: "team2".to_string(),
            played: false,
        };
        let match2 = ScheduledMatchData {
            id: "match2".to_string(),
            league_id: "league1".to_string(),
            round_number: 2,
            home_team_id: "team2".to_string(),
            away_team_id: "team1".to_string(),
            played: false,
        };

        repo.create(&match1).unwrap();
        repo.create(&match2).unwrap();

        let matches = repo.get_by_league("league1").unwrap();
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_get_by_round() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteScheduledMatchRepository::new(conn.clone());

        // Create league
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create teams
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match1 = create_test_scheduled_match("match1", "league1", 1);
        let match2 = ScheduledMatchData {
            id: "match2".to_string(),
            league_id: "league1".to_string(),
            round_number: 1,
            home_team_id: "team2".to_string(),
            away_team_id: "team1".to_string(),
            played: false,
        };
        let match3 = create_test_scheduled_match("match3", "league1", 2);

        repo.create(&match1).unwrap();
        repo.create(&match2).unwrap();
        repo.create(&match3).unwrap();

        let round1_matches = repo.get_by_round("league1", 1).unwrap();
        assert_eq!(round1_matches.len(), 2);

        let round2_matches = repo.get_by_round("league1", 2).unwrap();
        assert_eq!(round2_matches.len(), 1);
    }

    #[test]
    fn test_mark_as_played() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteScheduledMatchRepository::new(conn.clone());

        // Create league
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create teams
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let scheduled_match = create_test_scheduled_match("match1", "league1", 1);
        repo.create(&scheduled_match).unwrap();

        repo.mark_as_played("match1").unwrap();
        let retrieved = repo.get_by_id("match1").unwrap();

        assert_eq!(retrieved.played, true);
    }

    #[test]
    fn test_delete_by_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqliteScheduledMatchRepository::new(conn.clone());

        // Create leagues
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league2", "League 2", 0, 38],
        ).unwrap();

        // Create teams
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team3", "Team 3", "league2", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team4", "Team 4", "league2", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match1 = create_test_scheduled_match("match1", "league1", 1);
        let match2 = ScheduledMatchData {
            id: "match2".to_string(),
            league_id: "league2".to_string(),
            round_number: 1,
            home_team_id: "team3".to_string(),
            away_team_id: "team4".to_string(),
            played: false,
        };

        repo.create(&match1).unwrap();
        repo.create(&match2).unwrap();

        repo.delete_by_league("league1").unwrap();

        let league1_matches = repo.get_by_league("league1").unwrap();
        assert_eq!(league1_matches.len(), 0);

        let league2_matches = repo.get_by_league("league2").unwrap();
        assert_eq!(league2_matches.len(), 1);
    }
}
