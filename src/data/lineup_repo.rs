use crate::data::{DatabaseError, LineupRepository};
use crate::team::PlayerSlot;

/// SQLite implementation of LineupRepository
pub struct SqliteLineupRepository;

impl LineupRepository for SqliteLineupRepository {
    fn save_lineup(&self, _team_id: &str, _lineup: &[PlayerSlot], _is_starting: bool) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_starting_11(&self, _team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError> {
        Ok(Vec::new())
    }

    fn get_bench(&self, _team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError> {
        Ok(Vec::new())
    }

    fn clear_lineup(&self, _team_id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }
}
