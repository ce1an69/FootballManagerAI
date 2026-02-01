use crate::data::{Database, DatabaseError};
use crate::game::GameState;
use crate::team::{League, Team};
use std::path::{Path, PathBuf};
use std::fs;
use rusqlite::params;

/// Save metadata
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveMetadata {
    pub slot: u8,
    pub team_name: String,
    pub date: String,
    pub season: String,
    pub saved_at: u64,
    pub play_time_hours: u32,
    pub db_path: String,
}

/// Save manager
pub struct SaveManager {
    saves_dir: PathBuf,
}

impl SaveManager {
    /// Create new save manager
    pub fn new(saves_dir: PathBuf) -> Self {
        // Create saves directory if it doesn't exist
        fs::create_dir_all(&saves_dir).unwrap_or_else(|e| {
            eprintln!("Failed to create saves directory: {}", e);
        });

        Self { saves_dir }
    }

    /// Get saves directory (default: ./saves)
    pub fn default() -> Self {
        Self::new(PathBuf::from("saves"))
    }

    /// List all save files
    pub fn list_saves(&self) -> Result<Vec<SaveMetadata>, DatabaseError> {
        let mut saves = Vec::new();

        if !self.saves_dir.exists() {
            return Ok(saves);
        }

        let entries = fs::read_dir(&self.saves_dir)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to read saves directory: {}", e)))?;

        for entry in entries {
            let entry = entry.map_err(|e| DatabaseError::QueryError(format!("Failed to read entry: {}", e)))?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) != Some("db") {
                continue;
            }

            // Extract slot from filename
            let filename = path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("");

            if let Some(slot) = Self::parse_slot_from_filename(filename) {
                // Try to read metadata from database
                if let Ok(metadata) = self.read_save_metadata(&path) {
                    saves.push(metadata);
                } else {
                    // Create basic metadata from filename
                    saves.push(SaveMetadata {
                        slot,
                        team_name: "Unknown".to_string(),
                        date: "Unknown".to_string(),
                        season: "Unknown".to_string(),
                        saved_at: 0,
                        play_time_hours: 0,
                        db_path: path.to_string_lossy().to_string(),
                    });
                }
            }
        }

        saves.sort_by(|a, b| b.saved_at.cmp(&a.saved_at));
        Ok(saves)
    }

    /// Parse slot number from filename (save_001_xxx.db -> 1)
    fn parse_slot_from_filename(filename: &str) -> Option<u8> {
        if filename.starts_with("save_") {
            let parts: Vec<&str> = filename.split('_').collect();
            if parts.len() >= 2 {
                parts.get(1).and_then(|s| s.parse::<u8>().ok())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Read metadata from save database
    fn read_save_metadata(&self, db_path: &Path) -> Result<SaveMetadata, DatabaseError> {
        let db = Database::new(db_path.to_str().unwrap())?;

        // Read from game_metadata table
        let team_name = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'player_name'",
            [],
            |row| row.get::<_, String>(0),
        ).unwrap_or_else(|_| "Unknown".to_string());

        let date = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'game_date'",
            [],
            |row| row.get::<_, String>(0),
        ).unwrap_or_else(|_| "Unknown".to_string());

        let season = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'season'",
            [],
            |row| row.get::<_, String>(0),
        ).unwrap_or_else(|_| "Unknown".to_string());

        let saved_at: u64 = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'saved_at'",
            [],
            |row| {
                let s: String = row.get(0)?;
                Ok(s.parse().unwrap_or(0))
            },
        ).unwrap_or(0);

        let slot: u8 = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'save_slot'",
            [],
            |row| {
                let s: String = row.get(0)?;
                Ok(s.parse().unwrap_or(0))
            },
        ).unwrap_or(0);

        let play_time_hours: u32 = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'play_time'",
            [],
            |row| {
                let s: String = row.get(0)?;
                Ok(s.parse().unwrap_or(0))
            },
        ).unwrap_or(0);

        Ok(SaveMetadata {
            slot,
            team_name,
            date,
            season,
            saved_at,
            play_time_hours,
            db_path: db_path.to_string_lossy().to_string(),
        })
    }

    /// Generate save filename
    pub fn generate_save_filename(slot: u8, team_name: &str, timestamp: u64) -> String {
        let team_name_clean = team_name.chars()
            .map(|c| if c.is_alphanumeric() { c } else { '_' })
            .collect::<String>();

        format!("save_{:03}_{}_{}.db", slot, timestamp, team_name_clean)
    }

    /// Get full save path
    pub fn get_save_path(&self, slot: u8, team_name: &str, timestamp: u64) -> PathBuf {
        self.saves_dir.join(Self::generate_save_filename(slot, team_name, timestamp))
    }

    /// Delete save file
    pub fn delete_save(&self, db_path: &str) -> Result<(), DatabaseError> {
        fs::remove_file(db_path)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to delete save: {}", e)))
    }

    /// Auto-save slot number
    pub const AUTOSAVE_SLOT: u8 = 0;

    /// Check if autosave exists
    pub fn has_autosave(&self) -> bool {
        self.saves_dir.join(format!("save_{:03}_autosave.db", Self::AUTOSAVE_SLOT)).exists()
    }

    /// Get autosave path
    pub fn get_autosave_path(&self) -> PathBuf {
        self.saves_dir.join(format!("save_{:03}_autosave.db", Self::AUTOSAVE_SLOT))
    }

    /// Save game to database
    pub fn save_game(
        &self,
        slot: u8,
        state: &GameState,
        db: &Database,
    ) -> Result<PathBuf, DatabaseError> {
        // Get current timestamp
        let saved_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to get time: {}", e)))?
            .as_secs();

        // Get team name from state
        let team_name = state.teams.get(&state.player_team_id)
            .map(|t| t.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Generate save path
        let save_path = self.get_save_path(slot, &team_name, saved_at);

        // Copy current database to save location
        let current_db_path = std::path::Path::new(&state.db_path);
        if current_db_path.exists() {
            fs::copy(&state.db_path, &save_path)
                .map_err(|e| DatabaseError::QueryError(format!("Failed to copy database: {}", e)))?;
        } else {
            // Create new database at save location
            let save_db = Database::new(save_path.to_str().unwrap())?;
            save_db.run_migrations()?;
        }

        // Open the save database and write metadata
        let save_db = Database::new(save_path.to_str().unwrap())?;

        // Save game state as JSON in game_metadata table
        let state_json = serde_json::to_string(state)
            .map_err(|e| DatabaseError::SerializationError(e.to_string()))?;

        // Save metadata
        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["game_state", state_json],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        // Save individual metadata fields for easier querying
        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["player_name", state.player_name.as_deref().unwrap_or("Player")],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["game_date", format!("{}", state.current_date)],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["season", format!("Year {}", state.current_date.year)],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["saved_at", saved_at.to_string()],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["save_slot", slot.to_string()],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        save_db.conn.write().unwrap().execute(
            "INSERT OR REPLACE INTO game_metadata (key, value) VALUES (?, ?)",
            rusqlite::params!["save_version", "1.0"],
        ).map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(save_path)
    }

    /// Load game from database
    pub fn load_game(&self, slot: u8) -> Result<GameState, DatabaseError> {
        // Find save file for this slot
        let saves = self.list_saves()?;
        let save_metadata = saves.iter()
            .find(|s| s.slot == slot)
            .ok_or_else(|| DatabaseError::NotFound(format!("Save slot {}", slot)))?;

        // Open the save database
        let db = Database::new(&save_metadata.db_path)?;

        // Load game state from game_metadata table
        let state_json: String = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'game_state'",
            [],
            |row| row.get(0),
        ).map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                DatabaseError::NotFound("game_state metadata".to_string())
            },
            _ => DatabaseError::QueryError(e.to_string()),
        })?;

        // Deserialize game state
        let state: GameState = serde_json::from_str(&state_json)
            .map_err(|e| DatabaseError::SerializationError(format!("Failed to deserialize state: {}", e)))?;

        Ok(state)
    }

    /// Create a new save with initial state
    pub fn create_new_save(
        &self,
        slot: u8,
        initial_state: &GameState,
        db: &Database,
    ) -> Result<PathBuf, DatabaseError> {
        self.save_game(slot, initial_state, db)
    }

    /// Backup save to specified directory
    pub fn backup_save(&self, slot: u8, backup_dir: &str) -> Result<PathBuf, DatabaseError> {
        // Find save file for this slot
        let saves = self.list_saves()?;
        let save_metadata = saves.iter()
            .find(|s| s.slot == slot)
            .ok_or_else(|| DatabaseError::NotFound(format!("Save slot {}", slot)))?;

        // Create backup directory if it doesn't exist
        fs::create_dir_all(backup_dir)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to create backup directory: {}", e)))?;

        // Generate backup filename
        let source_path = std::path::Path::new(&save_metadata.db_path);
        let filename = source_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| DatabaseError::QueryError("Invalid save filename".to_string()))?;

        let backup_path = std::path::Path::new(backup_dir).join(filename);

        // Copy save file to backup location
        fs::copy(&save_metadata.db_path, &backup_path)
            .map_err(|e| DatabaseError::QueryError(format!("Failed to backup save: {}", e)))?;

        Ok(backup_path)
    }

    /// Get save version
    pub fn get_save_version(&self, slot: u8) -> Result<String, DatabaseError> {
        // Find save file for this slot
        let saves = self.list_saves()?;
        let save_metadata = saves.iter()
            .find(|s| s.slot == slot)
            .ok_or_else(|| DatabaseError::NotFound(format!("Save slot {}", slot)))?;

        // Open the save database
        let db = Database::new(&save_metadata.db_path)?;

        // Get version
        let version: String = db.conn.read().unwrap().query_row(
            "SELECT value FROM game_metadata WHERE key = 'save_version'",
            [],
            |row| row.get(0),
        ).unwrap_or_else(|_| "1.0".to_string());

        Ok(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_parse_slot_from_filename() {
        assert_eq!(SaveManager::parse_slot_from_filename("save_001_team_123.db"), Some(1));
        assert_eq!(SaveManager::parse_slot_from_filename("save_010_team_456.db"), Some(10));
        assert_eq!(SaveManager::parse_slot_from_filename("save_255_team.db"), Some(255));
        assert_eq!(SaveManager::parse_slot_from_filename("other_file.db"), None);
        assert_eq!(SaveManager::parse_slot_from_filename("save_abc.db"), None);
    }

    #[test]
    fn test_generate_save_filename() {
        let filename = SaveManager::generate_save_filename(1, "Test Team", 1234567890);
        assert!(filename.starts_with("save_001_"));
        assert!(filename.contains("Test_Team"));
        assert!(filename.ends_with(".db"));
    }

    #[test]
    fn test_save_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SaveManager::new(temp_dir.path().to_path_buf());
        assert!(manager.saves_dir.exists());
    }

    #[test]
    fn test_list_saves_empty() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SaveManager::new(temp_dir.path().to_path_buf());
        let saves = manager.list_saves().unwrap();
        assert_eq!(saves.len(), 0);
    }

    #[test]
    fn test_autosave_slot() {
        assert_eq!(SaveManager::AUTOSAVE_SLOT, 0);
    }

    #[test]
    fn test_get_save_path() {
        let temp_dir = TempDir::new().unwrap();
        let manager = SaveManager::new(temp_dir.path().to_path_buf());
        let path = manager.get_save_path(1, "Test Team", 1234567890);
        assert!(path.to_string_lossy().contains("save_001_"));
    }

    #[test]
    fn test_filename_sanitization() {
        let filename = SaveManager::generate_save_filename(1, "Test/Team\\Name", 123);
        assert!(!filename.contains('/'));
        assert!(!filename.contains('\\'));
        assert!(filename.contains("Test_Team_Name"));
    }
}
