use crate::data::{DatabaseError, TeamStatisticsRepository};
use crate::team::TeamStatistics;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of TeamStatisticsRepository
pub struct SqliteTeamStatisticsRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteTeamStatisticsRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl TeamStatisticsRepository for SqliteTeamStatisticsRepository {
    fn create(&self, team_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT OR IGNORE INTO team_statistics (team_id, matches_played, wins, draws, losses, goals_for, goals_against, points) VALUES (?, 0, 0, 0, 0, 0, 0, 0)",
            params![team_id],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_team(&self, team_id: &str) -> Result<TeamStatistics, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let stats = conn.query_row(
            "SELECT * FROM team_statistics WHERE team_id = ?",
            params![team_id],
            |row| {
                Ok(TeamStatistics {
                    matches_played: row.get("matches_played")?,
                    wins: row.get("wins")?,
                    draws: row.get("draws")?,
                    losses: row.get("losses")?,
                    goals_for: row.get("goals_for")?,
                    goals_against: row.get("goals_against")?,
                    points: row.get("points")?,
                    league_position: row.get("league_position").ok(),
                })
            },
        );

        match stats {
            Ok(s) => Ok(s),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(TeamStatistics::default()),
            Err(e) => Err(DatabaseError::QueryError(e.to_string())),
        }
    }

    fn update(&self, team_id: &str, stats: &TeamStatistics) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE team_statistics SET matches_played = ?, wins = ?, draws = ?, losses = ?, goals_for = ?, goals_against = ?, points = ?, league_position = ? WHERE team_id = ?",
            params![
                stats.matches_played,
                stats.wins,
                stats.draws,
                stats.losses,
                stats.goals_for,
                stats.goals_against,
                stats.points,
                stats.league_position,
                team_id,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_league_standings(&self, league_id: &str) -> Result<Vec<(String, TeamStatistics)>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT ts.team_id, ts.matches_played, ts.wins, ts.draws, ts.losses, ts.goals_for, ts.goals_against, ts.points, ts.league_position
             FROM team_statistics ts
             INNER JOIN teams t ON ts.team_id = t.id
             WHERE t.league_id = ?
             ORDER BY ts.points DESC, (ts.goals_for - ts.goals_against) DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let standings = stmt.query_map(params![league_id], |row| {
            Ok((
                row.get::<_, String>("team_id")?,
                TeamStatistics {
                    matches_played: row.get("matches_played")?,
                    wins: row.get("wins")?,
                    draws: row.get("draws")?,
                    losses: row.get("losses")?,
                    goals_for: row.get("goals_for")?,
                    goals_against: row.get("goals_against")?,
                    points: row.get("points")?,
                    league_position: row.get("league_position").ok(),
                }
            ))
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(standings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    #[test]
    fn test_create_and_get_statistics() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamStatisticsRepository::new(conn.clone());

        repo.create("team1").unwrap();
        let stats = repo.get_by_team("team1").unwrap();

        assert_eq!(stats.matches_played, 0);
        assert_eq!(stats.wins, 0);
        assert_eq!(stats.points, 0);
    }

    #[test]
    fn test_update_statistics() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamStatisticsRepository::new(conn.clone());

        repo.create("team1").unwrap();

        let new_stats = TeamStatistics {
            matches_played: 10,
            wins: 5,
            draws: 3,
            losses: 2,
            goals_for: 15,
            goals_against: 8,
            points: 18,
            league_position: Some(2),
        };

        repo.update("team1", &new_stats).unwrap();
        let retrieved = repo.get_by_team("team1").unwrap();

        assert_eq!(retrieved.matches_played, 10);
        assert_eq!(retrieved.wins, 5);
        assert_eq!(retrieved.points, 18);
        assert_eq!(retrieved.league_position, Some(2));
    }

    #[test]
    fn test_get_league_standings() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteTeamStatisticsRepository::new(conn.clone());

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

        // Create statistics
        let stats1 = TeamStatistics {
            matches_played: 10,
            wins: 6,
            draws: 2,
            losses: 2,
            goals_for: 20,
            goals_against: 10,
            points: 20,
            league_position: Some(1),
        };

        let stats2 = TeamStatistics {
            matches_played: 10,
            wins: 4,
            draws: 3,
            losses: 3,
            goals_for: 15,
            goals_against: 12,
            points: 15,
            league_position: Some(2),
        };

        repo.create("team1").unwrap();
        repo.create("team2").unwrap();
        repo.update("team1", &stats1).unwrap();
        repo.update("team2", &stats2).unwrap();

        let standings = repo.get_league_standings("league1").unwrap();
        assert_eq!(standings.len(), 2);
        assert_eq!(standings[0].0, "team1"); // Team 1 has more points
        assert_eq!(standings[1].0, "team2");
    }
}
