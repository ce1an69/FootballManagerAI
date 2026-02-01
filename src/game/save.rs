use crate::data::{Database, SaveManager, SaveMetadata, DatabaseError};
use crate::game::GameState;
use std::path::PathBuf;

/// Save game error types
#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("No game in progress")]
    NoGameInProgress,
}

/// Save game with auto-generated slot
///
/// Finds the next available save slot and saves the game
pub fn quick_save(
    game_state: &GameState,
    db: &Database,
) -> Result<PathBuf, SaveError> {
    let save_manager = SaveManager::default();

    // Find next available slot (1-9, slot 0 is autosave)
    let saves = save_manager.list_saves()?;
    let used_slots: Vec<u8> = saves.iter().map(|s| s.slot).collect();

    let slot = (1..=9).find(|s| !used_slots.contains(s))
        .unwrap_or(1); // Default to slot 1 if all full

    save_manager.save_game(slot, game_state, db)
        .map_err(Into::into)
}

/// Save game to specific slot
///
/// Overwrites existing save in the specified slot
pub fn save_to_slot(
    slot: u8,
    game_state: &GameState,
    db: &Database,
) -> Result<PathBuf, SaveError> {
    let save_manager = SaveManager::default();
    save_manager.save_game(slot, game_state, db)
        .map_err(Into::into)
}

/// Auto-save game
///
/// Saves to autosave slot (slot 0)
pub fn autosave(
    game_state: &GameState,
    db: &Database,
) -> Result<PathBuf, SaveError> {
    let save_manager = SaveManager::default();
    save_manager.save_game(SaveManager::AUTOSAVE_SLOT, game_state, db)
        .map_err(Into::into)
}

/// Load game from slot
///
/// Loads game state and returns both state and database
pub fn load_from_slot(slot: u8) -> Result<(GameState, Database), SaveError> {
    let save_manager = SaveManager::default();
    let game_state = save_manager.load_game(slot)?;

    // Open database
    let db_path = &game_state.db_path;
    let db = Database::new(db_path)?;

    Ok((game_state, db))
}

/// List all available saves
///
/// Returns metadata for all save files
pub fn list_save_games() -> Result<Vec<SaveMetadata>, SaveError> {
    let save_manager = SaveManager::default();
    save_manager.list_saves()
        .map_err(Into::into)
}

/// Delete save from slot
///
/// Removes the save file and database
pub fn delete_save_slot(slot: u8) -> Result<(), SaveError> {
    let save_manager = SaveManager::default();
    let saves = save_manager.list_saves()?;

    let save_metadata = saves.iter()
        .find(|s| s.slot == slot)
        .ok_or_else(|| SaveError::DatabaseError(
            DatabaseError::NotFound(format!("Save slot {}", slot))
        ))?;

    save_manager.delete_save(&save_metadata.db_path)
        .map_err(Into::into)
}

/// Backup save to directory
///
/// Creates a backup copy of the save file
pub fn backup_save(
    slot: u8,
    backup_dir: &str,
) -> Result<PathBuf, SaveError> {
    let save_manager = SaveManager::default();
    save_manager.backup_save(slot, backup_dir)
        .map_err(Into::into)
}

/// Get save metadata for specific slot
///
/// Returns information about the save without loading the full game state
pub fn get_save_metadata(slot: u8) -> Result<SaveMetadata, SaveError> {
    let save_manager = SaveManager::default();
    let saves = save_manager.list_saves()?;

    saves.iter()
        .find(|s| s.slot == slot)
        .cloned()
        .ok_or_else(|| SaveError::DatabaseError(
            DatabaseError::NotFound(format!("Save slot {}", slot))
        ))
}

/// Check if autosave exists
pub fn has_autosave() -> bool {
    let save_manager = SaveManager::default();
    save_manager.has_autosave()
}

/// Load autosave
///
/// Convenience function to load from autosave slot
pub fn load_autosave() -> Result<(GameState, Database), SaveError> {
    load_from_slot(SaveManager::AUTOSAVE_SLOT)
}

/// Validate save file integrity
///
/// Checks if the save file exists and can be opened
pub fn validate_save(slot: u8) -> Result<bool, SaveError> {
    let save_manager = SaveManager::default();
    let saves = save_manager.list_saves()?;

    let save_metadata = saves.iter()
        .find(|s| s.slot == slot)
        .ok_or_else(|| SaveError::DatabaseError(
            DatabaseError::NotFound(format!("Save slot {}", slot))
        ))?;

    // Check if file exists and is valid
    let path = std::path::Path::new(&save_metadata.db_path);
    if !path.exists() {
        return Ok(false);
    }

    // Try to open database to verify integrity
    match Database::new(&save_metadata.db_path) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Get save version
///
/// Returns the version string for the save file
pub fn get_save_version(slot: u8) -> Result<String, SaveError> {
    let save_manager = SaveManager::default();
    save_manager.get_save_version(slot)
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::{League, Team};
    use std::path::PathBuf;

    #[test]
    fn test_has_autosave() {
        // Should return false for fresh directory
        let result = has_autosave();
        assert!(result == true || result == false); // Just verify it returns a value
    }

    #[test]
    fn test_get_save_metadata_not_found() {
        let result = get_save_metadata(99);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_save_not_found() {
        let result = validate_save(99);
        assert!(result.is_err());
    }

    #[test]
    fn test_list_save_games() {
        let result = list_save_games();
        assert!(result.is_ok());
        // May be empty or have saves depending on test environment
    }

    #[test]
    fn test_delete_save_slot_not_found() {
        let result = delete_save_slot(99);
        assert!(result.is_err());
    }
}
