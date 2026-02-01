use crate::data::{DatabaseError, PlayerSeasonStatsRepository};
use crate::team::PlayerSeasonStats;
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of PlayerSeasonStatsRepository
pub struct SqlitePlayerSeasonStatsRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqlitePlayerSeasonStatsRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }
}

impl PlayerSeasonStatsRepository for SqlitePlayerSeasonStatsRepository {
    fn create(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "INSERT INTO player_season_stats (id, player_id, season, team_id, appearances, goals, assists,
                                             yellow_cards, red_cards, minutes_played, average_rating)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                uuid::Uuid::new_v4().to_string(),
                stats.player_id,
                stats.season,
                stats.team_id,
                stats.appearances,
                stats.goals,
                stats.assists,
                stats.yellow_cards,
                stats.red_cards,
                stats.minutes_played,
                stats.average_rating,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_player(&self, player_id: &str, season: &str) -> Result<PlayerSeasonStats, DatabaseError> {
        let conn = self.conn.read().unwrap();

        conn.query_row(
            "SELECT player_id, season, team_id, appearances, goals, assists,
                    yellow_cards, red_cards, minutes_played, average_rating
             FROM player_season_stats
             WHERE player_id = ? AND season = ?",
            params![player_id, season],
            |row| {
                let appearances: u32 = row.get(3)?;
                let avg_rating: f32 = row.get(9)?;
                Ok(PlayerSeasonStats {
                    player_id: row.get(0)?,
                    season: row.get(1)?,
                    team_id: row.get(2)?,
                    appearances,
                    starts: 0, // Not tracked separately
                    minutes_played: row.get(8)?,
                    goals: row.get(4)?,
                    assists: row.get(5)?,
                    shots: 0,
                    shots_on_target: 0,
                    yellow_cards: row.get(6)?,
                    red_cards: row.get(7)?,
                    total_rating: avg_rating * appearances as f32,
                    average_rating: avg_rating,
                    man_of_the_match: 0,
                })
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                DatabaseError::NotFound(format!("{}-{}", player_id, season))
            }
            _ => DatabaseError::QueryError(e.to_string()),
        })
    }

    fn get_by_team(&self, team_id: &str, season: &str) -> Result<Vec<PlayerSeasonStats>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, season, team_id, appearances, goals, assists,
                    yellow_cards, red_cards, minutes_played, average_rating
             FROM player_season_stats
             WHERE team_id = ? AND season = ?
             ORDER BY goals DESC, assists DESC"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let stats_list = stmt.query_map(params![team_id, season], |row| {
            let appearances: u32 = row.get(3)?;
            let avg_rating: f32 = row.get(9)?;
            Ok(PlayerSeasonStats {
                player_id: row.get(0)?,
                season: row.get(1)?,
                team_id: row.get(2)?,
                appearances,
                starts: 0,
                minutes_played: row.get(8)?,
                goals: row.get(4)?,
                assists: row.get(5)?,
                shots: 0,
                shots_on_target: 0,
                yellow_cards: row.get(6)?,
                red_cards: row.get(7)?,
                total_rating: avg_rating * appearances as f32,
                average_rating: avg_rating,
                man_of_the_match: 0,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(stats_list)
    }

    fn update(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        conn.execute(
            "UPDATE player_season_stats
             SET appearances = ?, goals = ?, assists = ?,
                 yellow_cards = ?, red_cards = ?, minutes_played = ?, average_rating = ?
             WHERE player_id = ? AND season = ?",
            params![
                stats.appearances,
                stats.goals,
                stats.assists,
                stats.yellow_cards,
                stats.red_cards,
                stats.minutes_played,
                stats.average_rating,
                stats.player_id,
                stats.season,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_or_create(&self, player_id: &str, season: &str, team_id: &str) -> Result<PlayerSeasonStats, DatabaseError> {
        match self.get_by_player(player_id, season) {
            Ok(stats) => Ok(stats),
            Err(DatabaseError::NotFound(_)) => {
                // Create new stats
                let new_stats = PlayerSeasonStats::new(
                    player_id.to_string(),
                    season.to_string(),
                    team_id.to_string(),
                );
                let conn = self.conn.write().unwrap();

                conn.execute(
                    "INSERT INTO player_season_stats (id, player_id, season, team_id, appearances, goals, assists,
                                                     yellow_cards, red_cards, minutes_played, average_rating)
                     VALUES (?, ?, ?, ?, 0, 0, 0, 0, 0, 0, 6.0)",
                    params![uuid::Uuid::new_v4().to_string(), player_id, season, team_id],
                ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

                Ok(new_stats)
            }
            Err(e) => Err(e),
        }
    }

    fn get_top_scorers(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, season, team_id, appearances, goals, assists,
                    yellow_cards, red_cards, minutes_played, average_rating
             FROM player_season_stats
             WHERE season = ?
             ORDER BY goals DESC
             LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let stats_list = stmt.query_map(params![season, limit], |row| {
            let appearances: u32 = row.get(3)?;
            let avg_rating: f32 = row.get(9)?;
            Ok(PlayerSeasonStats {
                player_id: row.get(0)?,
                season: row.get(1)?,
                team_id: row.get(2)?,
                appearances,
                starts: 0,
                minutes_played: row.get(8)?,
                goals: row.get(4)?,
                assists: row.get(5)?,
                shots: 0,
                shots_on_target: 0,
                yellow_cards: row.get(6)?,
                red_cards: row.get(7)?,
                total_rating: avg_rating * appearances as f32,
                average_rating: avg_rating,
                man_of_the_match: 0,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(stats_list)
    }

    fn get_top_assists(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, season, team_id, appearances, goals, assists,
                    yellow_cards, red_cards, minutes_played, average_rating
             FROM player_season_stats
             WHERE season = ?
             ORDER BY assists DESC
             LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let stats_list = stmt.query_map(params![season, limit], |row| {
            let appearances: u32 = row.get(3)?;
            let avg_rating: f32 = row.get(9)?;
            Ok(PlayerSeasonStats {
                player_id: row.get(0)?,
                season: row.get(1)?,
                team_id: row.get(2)?,
                appearances,
                starts: 0,
                minutes_played: row.get(8)?,
                goals: row.get(4)?,
                assists: row.get(5)?,
                shots: 0,
                shots_on_target: 0,
                yellow_cards: row.get(6)?,
                red_cards: row.get(7)?,
                total_rating: avg_rating * appearances as f32,
                average_rating: avg_rating,
                man_of_the_match: 0,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(stats_list)
    }

    fn get_top_rated(
        &self,
        season: &str,
        min_appearances: u32,
        limit: usize,
    ) -> Result<Vec<PlayerSeasonStats>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT player_id, season, team_id, appearances, goals, assists,
                    yellow_cards, red_cards, minutes_played, average_rating
             FROM player_season_stats
             WHERE season = ? AND appearances >= ?
             ORDER BY average_rating DESC
             LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let stats_list = stmt.query_map(params![season, min_appearances, limit], |row| {
            let appearances: u32 = row.get(3)?;
            let avg_rating: f32 = row.get(9)?;
            Ok(PlayerSeasonStats {
                player_id: row.get(0)?,
                season: row.get(1)?,
                team_id: row.get(2)?,
                appearances,
                starts: 0,
                minutes_played: row.get(8)?,
                goals: row.get(4)?,
                assists: row.get(5)?,
                shots: 0,
                shots_on_target: 0,
                yellow_cards: row.get(6)?,
                red_cards: row.get(7)?,
                total_rating: avg_rating * appearances as f32,
                average_rating: avg_rating,
                man_of_the_match: 0,
            })
        }).map_err(|e| DatabaseError::QueryError(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(stats_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    #[test]
    fn test_create_and_get_stats() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqlitePlayerSeasonStatsRepository::new(conn.clone());

        // Create the league first (foreign key requirement)
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team and player first (foreign key requirements)
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["player1", "team1", "Player 1", 25, "England", "ST", "Right", 180, 75, 10000, 3, 5000000],
        ).unwrap();

        let stats = PlayerSeasonStats::new(
            "player1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        repo.create(&stats).unwrap();

        let retrieved = repo.get_by_player("player1", "2026-27").unwrap();
        assert_eq!(retrieved.player_id, "player1");
        assert_eq!(retrieved.season, "2026-27");
        assert_eq!(retrieved.goals, 0);
    }

    #[test]
    fn test_update_stats() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqlitePlayerSeasonStatsRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team and player first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["player1", "team1", "Player 1", 25, "England", "ST", "Right", 180, 75, 10000, 3, 5000000],
        ).unwrap();

        let mut stats = PlayerSeasonStats::new(
            "player1".to_string(),
            "2026-27".to_string(),
            "team1".to_string(),
        );

        repo.create(&stats).unwrap();

        stats.goals = 10;
        stats.assists = 5;
        stats.appearances = 15;
        repo.update(&stats).unwrap();

        let retrieved = repo.get_by_player("player1", "2026-27").unwrap();
        assert_eq!(retrieved.goals, 10);
        assert_eq!(retrieved.assists, 5);
        assert_eq!(retrieved.appearances, 15);
    }

    #[test]
    fn test_get_or_create() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqlitePlayerSeasonStatsRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create the team and player first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO players (id, team_id, name, age, nationality, position, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["player1", "team1", "Player 1", 25, "England", "ST", "Right", 180, 75, 10000, 3, 5000000],
        ).unwrap();

        // Should create new
        let stats = repo.get_or_create("player1", "2026-27", "team1").unwrap();
        assert_eq!(stats.player_id, "player1");
        assert_eq!(stats.goals, 0);

        // Should return existing
        let stats2 = repo.get_or_create("player1", "2026-27", "team1").unwrap();
        assert_eq!(stats2.player_id, "player1");
    }

    #[test]
    fn test_get_top_scorers() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = db.conn.clone();
        let repo = SqlitePlayerSeasonStatsRepository::new(conn.clone());

        // Create the league first
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        // Create teams and players first
        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        for (player_id, team_id) in [("player1", "team1"), ("player2", "team1"), ("player3", "team2")] {
            conn.write().unwrap().execute(
                "INSERT INTO players (id, team_id, name, age, nationality, position, preferred_foot, height, weight, wage, contract_years, market_value) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                rusqlite::params![player_id, team_id, format!("Player {}", player_id), 25, "England", "ST", "Right", 180, 75, 10000, 3, 5000000],
            ).unwrap();
        }

        // Create multiple players
        let mut stats1 = PlayerSeasonStats::new("player1".to_string(), "2026-27".to_string(), "team1".to_string());
        stats1.goals = 20;

        let mut stats2 = PlayerSeasonStats::new("player2".to_string(), "2026-27".to_string(), "team1".to_string());
        stats2.goals = 15;

        let mut stats3 = PlayerSeasonStats::new("player3".to_string(), "2026-27".to_string(), "team2".to_string());
        stats3.goals = 25;

        repo.create(&stats1).unwrap();
        repo.create(&stats2).unwrap();
        repo.create(&stats3).unwrap();

        let top_scorers = repo.get_top_scorers("2026-27", 10).unwrap();
        assert_eq!(top_scorers.len(), 3);
        assert_eq!(top_scorers[0].goals, 25); // player3
        assert_eq!(top_scorers[1].goals, 20); // player1
        assert_eq!(top_scorers[2].goals, 15); // player2
    }
}
