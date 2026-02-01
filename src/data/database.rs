use rusqlite::{Connection, Result as SqliteResult};
use thiserror::Error;

/// Database error types
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Query failed: {0}")]
    QueryError(String),

    #[error("Migration failed: {0}")]
    MigrationError(String),

    #[error("Serialization failed: {0}")]
    SerializationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
}

/// Database connection manager
pub struct Database {
    pub conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: &str) -> Result<Self, DatabaseError> {
        let conn = Connection::open(path)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(Self { conn })
    }

    /// Create an in-memory database (for testing)
    pub fn in_memory() -> Result<Self, DatabaseError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        // Enable foreign keys
        conn.execute("PRAGMA foreign_keys = ON", [])
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(Self { conn })
    }

    /// Run database migrations
    pub fn run_migrations(&self) -> Result<(), DatabaseError> {
        // Create tables
        self.create_tables()?;
        Ok(())
    }

    /// Create all database tables
    fn create_tables(&self) -> Result<(), DatabaseError> {
        // Leagues table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS leagues (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                current_round INTEGER NOT NULL DEFAULT 0,
                total_rounds INTEGER NOT NULL
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Teams table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS teams (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                league_id TEXT NOT NULL,
                budget INTEGER NOT NULL,
                formation TEXT NOT NULL,
                attacking_mentality INTEGER NOT NULL,
                defensive_height TEXT NOT NULL,
                passing_style TEXT NOT NULL,
                tempo TEXT NOT NULL,
                FOREIGN KEY (league_id) REFERENCES leagues(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Players table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS players (
                id TEXT PRIMARY KEY,
                team_id TEXT,
                name TEXT NOT NULL,
                age INTEGER NOT NULL,
                nationality TEXT NOT NULL,
                position TEXT NOT NULL,
                second_positions TEXT,
                preferred_foot TEXT NOT NULL,
                height INTEGER NOT NULL,
                weight INTEGER NOT NULL,
                corners INTEGER DEFAULT 50,
                crossing INTEGER DEFAULT 50,
                dribbling INTEGER DEFAULT 50,
                finishing INTEGER DEFAULT 50,
                heading INTEGER DEFAULT 50,
                long_shots INTEGER DEFAULT 50,
                long_throws INTEGER DEFAULT 50,
                marking INTEGER DEFAULT 50,
                passing INTEGER DEFAULT 50,
                penalties INTEGER DEFAULT 50,
                tackling INTEGER DEFAULT 50,
                technique INTEGER DEFAULT 50,
                aggression INTEGER DEFAULT 50,
                anticipation INTEGER DEFAULT 50,
                bravery INTEGER DEFAULT 50,
                creativity INTEGER DEFAULT 50,
                decisions INTEGER DEFAULT 50,
                concentration INTEGER DEFAULT 50,
                positioning INTEGER DEFAULT 50,
                off_the_ball INTEGER DEFAULT 50,
                work_rate INTEGER DEFAULT 50,
                pressure INTEGER DEFAULT 50,
                teamwork INTEGER DEFAULT 50,
                vision INTEGER DEFAULT 50,
                acceleration INTEGER DEFAULT 50,
                agility INTEGER DEFAULT 50,
                balance INTEGER DEFAULT 50,
                pace INTEGER DEFAULT 50,
                stamina INTEGER DEFAULT 50,
                strength INTEGER DEFAULT 50,
                aerial_reach INTEGER DEFAULT 50,
                command_of_area INTEGER DEFAULT 50,
                communication INTEGER DEFAULT 50,
                eccentricity INTEGER DEFAULT 50,
                handling INTEGER DEFAULT 50,
                kicking INTEGER DEFAULT 50,
                throwing INTEGER DEFAULT 50,
                reflexes INTEGER DEFAULT 50,
                rushing_out INTEGER DEFAULT 50,
                gk_positioning INTEGER DEFAULT 50,
                potential_ability INTEGER DEFAULT 100,
                current_ability INTEGER DEFAULT 100,
                adaptability INTEGER DEFAULT 100,
                ambition INTEGER DEFAULT 100,
                professionalism INTEGER DEFAULT 100,
                loyalty INTEGER DEFAULT 100,
                injury_proneness INTEGER DEFAULT 50,
                controversy INTEGER DEFAULT 50,
                match_fitness INTEGER DEFAULT 100,
                morale INTEGER DEFAULT 50,
                status TEXT DEFAULT 'healthy',
                injury_days INTEGER DEFAULT 0,
                fatigue INTEGER DEFAULT 0,
                wage INTEGER NOT NULL,
                contract_years INTEGER NOT NULL,
                market_value INTEGER NOT NULL,
                FOREIGN KEY (team_id) REFERENCES teams(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Matches table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS matches (
                id TEXT PRIMARY KEY,
                league_id TEXT NOT NULL,
                home_team_id TEXT NOT NULL,
                away_team_id TEXT NOT NULL,
                home_score INTEGER NOT NULL,
                away_score INTEGER NOT NULL,
                match_mode TEXT NOT NULL,
                events TEXT NOT NULL,
                home_possession REAL,
                away_possession REAL,
                home_shots INTEGER,
                away_shots INTEGER,
                home_shots_on_target INTEGER,
                away_shots_on_target INTEGER,
                home_passes INTEGER,
                away_passes INTEGER,
                home_pass_accuracy REAL,
                away_pass_accuracy REAL,
                home_corners INTEGER,
                away_corners INTEGER,
                home_fouls INTEGER,
                away_fouls INTEGER,
                home_offsides INTEGER,
                away_offsides INTEGER,
                played_at INTEGER NOT NULL,
                round INTEGER NOT NULL,
                FOREIGN KEY (league_id) REFERENCES leagues(id),
                FOREIGN KEY (home_team_id) REFERENCES teams(id),
                FOREIGN KEY (away_team_id) REFERENCES teams(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Transfer market table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS transfer_market (
                player_id TEXT PRIMARY KEY,
                asking_price INTEGER NOT NULL,
                listed_at INTEGER NOT NULL,
                reason TEXT,
                FOREIGN KEY (player_id) REFERENCES players(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Game metadata table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS game_metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Scheduled matches table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS scheduled_matches (
                id TEXT PRIMARY KEY,
                league_id TEXT NOT NULL,
                round_number INTEGER NOT NULL,
                home_team_id TEXT NOT NULL,
                away_team_id TEXT NOT NULL,
                played INTEGER DEFAULT 0,
                FOREIGN KEY (league_id) REFERENCES leagues(id),
                FOREIGN KEY (home_team_id) REFERENCES teams(id),
                FOREIGN KEY (away_team_id) REFERENCES teams(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Lineups table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS lineups (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                team_id TEXT NOT NULL,
                player_id TEXT NOT NULL,
                position TEXT NOT NULL,
                role TEXT NOT NULL,
                duty TEXT NOT NULL,
                is_starting INTEGER DEFAULT 1,
                position_index INTEGER,
                UNIQUE (team_id, player_id),
                FOREIGN KEY (team_id) REFERENCES teams(id),
                FOREIGN KEY (player_id) REFERENCES players(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        // Team statistics table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS team_statistics (
                team_id TEXT PRIMARY KEY,
                matches_played INTEGER DEFAULT 0,
                wins INTEGER DEFAULT 0,
                draws INTEGER DEFAULT 0,
                losses INTEGER DEFAULT 0,
                goals_for INTEGER DEFAULT 0,
                goals_against INTEGER DEFAULT 0,
                points INTEGER DEFAULT 0,
                league_position INTEGER,
                FOREIGN KEY (team_id) REFERENCES teams(id)
            )",
            [],
        ).map_err(|e| DatabaseError::MigrationError(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_database() {
        let db = Database::in_memory().unwrap();
        // Test that database was created successfully by querying
        let result: Result<i64, _> = db.conn.query_row("SELECT 1", [], |row| row.get(0));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_create_tables() {
        let db = Database::in_memory().unwrap();
        db.run_migrations().unwrap();

        // Check if tables exist
        let tables: Vec<String> = db.conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<SqliteResult<Vec<_>>>()
            .unwrap();

        assert!(tables.contains(&"leagues".to_string()));
        assert!(tables.contains(&"teams".to_string()));
        assert!(tables.contains(&"players".to_string()));
        assert!(tables.contains(&"matches".to_string()));
    }
}
