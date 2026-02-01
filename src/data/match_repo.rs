use crate::data::{DatabaseError, MatchRepository};
use crate::team::{MatchResult, MatchMode, MatchEvent, MatchStatistics, PlayerMatchRating};
use rusqlite::params;
use std::sync::{Arc, RwLock};

/// SQLite implementation of MatchRepository
pub struct SqliteMatchRepository {
    conn: Arc<RwLock<rusqlite::Connection>>,
}

impl SqliteMatchRepository {
    /// Create new repository with connection
    pub fn new(conn: Arc<RwLock<rusqlite::Connection>>) -> Self {
        Self { conn }
    }

    fn parse_match_mode(s: &str) -> MatchMode {
        match s {
            "Live" => MatchMode::Live,
            "Quick" => MatchMode::Quick,
            _ => MatchMode::Quick,
        }
    }
}

impl MatchRepository for SqliteMatchRepository {
    fn save(&self, match_result: &MatchResult) -> Result<(), DatabaseError> {
        let conn = self.conn.write().unwrap();

        // Serialize events to JSON
        let events_json = serde_json::to_string(&match_result.events)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        // Serialize player ratings
        let home_ratings_json = serde_json::to_string(&match_result.statistics.home_player_ratings)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;
        let away_ratings_json = serde_json::to_string(&match_result.statistics.away_player_ratings)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        conn.execute(
            "INSERT INTO matches (id, league_id, home_team_id, away_team_id, home_score, away_score, match_mode, events, home_possession, away_possession, home_shots, away_shots, home_shots_on_target, away_shots_on_target, home_passes, away_passes, home_pass_accuracy, away_pass_accuracy, home_corners, away_corners, home_fouls, away_fouls, home_offsides, away_offsides, home_player_ratings, away_player_ratings, played_at, round) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                match_result.id,
                match_result.league_id,
                match_result.home_team_id,
                match_result.away_team_id,
                match_result.home_score,
                match_result.away_score,
                format!("{:?}", match_result.match_mode),
                events_json,
                match_result.statistics.home_possession,
                match_result.statistics.away_possession,
                match_result.statistics.home_shots,
                match_result.statistics.away_shots,
                match_result.statistics.home_shots_on_target,
                match_result.statistics.away_shots_on_target,
                match_result.statistics.home_passes,
                match_result.statistics.away_passes,
                match_result.statistics.home_pass_accuracy,
                match_result.statistics.away_pass_accuracy,
                match_result.statistics.home_corners,
                match_result.statistics.away_corners,
                match_result.statistics.home_fouls,
                match_result.statistics.away_fouls,
                match_result.statistics.home_offsides,
                match_result.statistics.away_offsides,
                home_ratings_json,
                away_ratings_json,
                match_result.played_at,
                match_result.round,
            ],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    fn get_by_id(&self, id: &str) -> Result<MatchResult, DatabaseError> {
        let conn = self.conn.read().unwrap();

        // Fetch all raw data including JSON strings
        let (
            id, league_id, home_team_id, away_team_id, home_score, away_score,
            match_mode_str, events_str, home_ratings_str, away_ratings_str,
            home_possession, away_possession,
            home_shots, away_shots, home_shots_on_target, away_shots_on_target,
            home_passes, away_passes, home_pass_accuracy, away_pass_accuracy,
            home_corners, away_corners, home_fouls, away_fouls,
            home_offsides, away_offsides,
            played_at, round
        ) = conn.query_row(
            "SELECT id, league_id, home_team_id, away_team_id, home_score, away_score, match_mode, events, home_player_ratings, away_player_ratings, home_possession, away_possession, home_shots, away_shots, home_shots_on_target, away_shots_on_target, home_passes, away_passes, home_pass_accuracy, away_pass_accuracy, home_corners, away_corners, home_fouls, away_fouls, home_offsides, away_offsides, played_at, round FROM matches WHERE id = ?",
            params![id],
            |row| {
                Ok((
                    row.get::<_, String>("id")?,
                    row.get::<_, String>("league_id")?,
                    row.get::<_, String>("home_team_id")?,
                    row.get::<_, String>("away_team_id")?,
                    row.get::<_, i32>("home_score")?,
                    row.get::<_, i32>("away_score")?,
                    row.get::<_, String>("match_mode")?,
                    row.get::<_, String>("events")?,
                    row.get::<_, String>("home_player_ratings")?,
                    row.get::<_, String>("away_player_ratings")?,
                    row.get::<_, Option<f64>>("home_possession")?,
                    row.get::<_, Option<f64>>("away_possession")?,
                    row.get::<_, Option<i32>>("home_shots")?,
                    row.get::<_, Option<i32>>("away_shots")?,
                    row.get::<_, Option<i32>>("home_shots_on_target")?,
                    row.get::<_, Option<i32>>("away_shots_on_target")?,
                    row.get::<_, Option<i32>>("home_passes")?,
                    row.get::<_, Option<i32>>("away_passes")?,
                    row.get::<_, Option<f64>>("home_pass_accuracy")?,
                    row.get::<_, Option<f64>>("away_pass_accuracy")?,
                    row.get::<_, Option<i32>>("home_corners")?,
                    row.get::<_, Option<i32>>("away_corners")?,
                    row.get::<_, Option<i32>>("home_fouls")?,
                    row.get::<_, Option<i32>>("away_fouls")?,
                    row.get::<_, Option<i32>>("home_offsides")?,
                    row.get::<_, Option<i32>>("away_offsides")?,
                    row.get::<_, i64>("played_at")?,
                    row.get::<_, i32>("round")?,
                ))
            },
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => DatabaseError::NotFound(id.to_string()),
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        // Deserialize JSON strings outside the closure
        let events: Vec<MatchEvent> = serde_json::from_str(&events_str)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        let home_player_ratings: Vec<PlayerMatchRating> = serde_json::from_str(&home_ratings_str)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        let away_player_ratings: Vec<PlayerMatchRating> = serde_json::from_str(&away_ratings_str)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        // Build the MatchResult with proper type conversions
        Ok(MatchResult {
            id,
            league_id,
            home_team_id,
            away_team_id,
            home_score: home_score as u8,
            away_score: away_score as u8,
            match_mode: Self::parse_match_mode(&match_mode_str),
            events,
            statistics: MatchStatistics {
                home_possession: home_possession.unwrap_or(0.0) as f32,
                away_possession: away_possession.unwrap_or(0.0) as f32,
                home_shots: home_shots.unwrap_or(0) as u16,
                away_shots: away_shots.unwrap_or(0) as u16,
                home_shots_on_target: home_shots_on_target.unwrap_or(0) as u16,
                away_shots_on_target: away_shots_on_target.unwrap_or(0) as u16,
                home_passes: home_passes.unwrap_or(0) as u16,
                away_passes: away_passes.unwrap_or(0) as u16,
                home_pass_accuracy: home_pass_accuracy.unwrap_or(0.0) as f32,
                away_pass_accuracy: away_pass_accuracy.unwrap_or(0.0) as f32,
                home_corners: home_corners.unwrap_or(0) as u8,
                away_corners: away_corners.unwrap_or(0) as u8,
                home_fouls: home_fouls.unwrap_or(0) as u8,
                away_fouls: away_fouls.unwrap_or(0) as u8,
                home_offsides: home_offsides.unwrap_or(0) as u8,
                away_offsides: away_offsides.unwrap_or(0) as u8,
                home_player_ratings,
                away_player_ratings,
            },
            played_at: played_at as u64,
            round: round as u32,
        })
    }

    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<MatchResult>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id FROM matches WHERE home_team_id = ? OR away_team_id = ? ORDER BY played_at DESC LIMIT ?"
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let match_ids = stmt.query_map(params![team_id, team_id, limit as i32], |row| row.get::<_, String>(0))
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()?;

        let mut matches = Vec::new();
        for match_id in match_ids {
            matches.push(self.get_by_id(&match_id)?);
        }

        Ok(matches)
    }

    fn get_by_league(&self, league_id: &str, round: u32) -> Result<Vec<MatchResult>, DatabaseError> {
        let conn = self.conn.read().unwrap();

        let mut stmt = conn.prepare(
            "SELECT id FROM matches WHERE league_id = ? AND round = ? ORDER BY home_team_id"
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Database;

    fn create_test_match(id: &str, league_id: &str, home_team: &str, away_team: &str) -> MatchResult {
        MatchResult {
            id: id.to_string(),
            league_id: league_id.to_string(),
            home_team_id: home_team.to_string(),
            away_team_id: away_team.to_string(),
            home_score: 2,
            away_score: 1,
            match_mode: MatchMode::Quick,
            events: vec![],
            statistics: MatchStatistics::default(),
            played_at: 1234567890,
            round: 1,
        }
    }

    #[test]
    fn test_save_and_get_match() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteMatchRepository::new(conn.clone());

        // Create league and teams
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match_result = create_test_match("match1", "league1", "team1", "team2");

        repo.save(&match_result).unwrap();
        let retrieved = repo.get_by_id("match1").unwrap();

        assert_eq!(retrieved.id, "match1");
        assert_eq!(retrieved.home_team_id, "team1");
        assert_eq!(retrieved.away_team_id, "team2");
        assert_eq!(retrieved.home_score, 2);
        assert_eq!(retrieved.away_score, 1);
    }

    #[test]
    fn test_get_by_team() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteMatchRepository::new(conn.clone());

        // Create league and teams
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match1 = create_test_match("match1", "league1", "team1", "team2");
        let match2 = MatchResult {
            id: "match2".to_string(),
            ..match1.clone()
        };

        repo.save(&match1).unwrap();
        repo.save(&match2).unwrap();

        let matches = repo.get_by_team("team1", 10).unwrap();
        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_get_by_league() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();
        let conn = Arc::new(RwLock::new(db.conn));
        let repo = SqliteMatchRepository::new(conn.clone());

        // Create league and teams
        conn.write().unwrap().execute(
            "INSERT INTO leagues (id, name, current_round, total_rounds) VALUES (?, ?, ?, ?)",
            rusqlite::params!["league1", "League 1", 0, 38],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team1", "Team 1", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        conn.write().unwrap().execute(
            "INSERT INTO teams (id, name, league_id, budget, formation, attacking_mentality, defensive_height, passing_style, tempo) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params!["team2", "Team 2", "league1", 1000000, "4-4-2", 50, "Medium", "Mixed", "Medium"],
        ).unwrap();

        let match1 = create_test_match("match1", "league1", "team1", "team2");
        repo.save(&match1).unwrap();

        let matches = repo.get_by_league("league1", 1).unwrap();
        assert_eq!(matches.len(), 1);
    }
}
