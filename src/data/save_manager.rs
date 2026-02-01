use crate::data::{Database, DatabaseError};
use crate::game::GameState;
use crate::team::{League, Team};
use std::path::{Path, PathBuf};
use std::fs;

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
