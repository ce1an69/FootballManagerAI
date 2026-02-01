use crate::data::{DatabaseError, LeagueRepository};
use crate::team::League;

/// SQLite implementation of LeagueRepository
pub struct SqliteLeagueRepository;

impl LeagueRepository for SqliteLeagueRepository {
    fn create(&self, _league: &League) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_by_id(&self, _id: &str) -> Result<League, DatabaseError> {
        Err(DatabaseError::NotFound("Not implemented".to_string()))
    }

    fn update(&self, _league: &League) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_all(&self) -> Result<Vec<League>, DatabaseError> {
        Ok(Vec::new())
    }
}
