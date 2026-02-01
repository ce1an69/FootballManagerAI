use crate::data::{DatabaseError, PlayerRepository};
use crate::team::Player;

/// SQLite implementation of PlayerRepository
pub struct SqlitePlayerRepository;

impl PlayerRepository for SqlitePlayerRepository {
    fn create(&self, _player: &Player) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_by_id(&self, _id: &str) -> Result<Player, DatabaseError> {
        Err(DatabaseError::NotFound("Not implemented".to_string()))
    }

    fn get_by_team(&self, _team_id: &str) -> Result<Vec<Player>, DatabaseError> {
        Ok(Vec::new())
    }

    fn update(&self, _player: &Player) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete(&self, _id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_free_agents(&self) -> Result<Vec<Player>, DatabaseError> {
        Ok(Vec::new())
    }
}
