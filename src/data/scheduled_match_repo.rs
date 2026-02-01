use crate::data::{DatabaseError, ScheduledMatchRepository, ScheduledMatchData};

/// SQLite implementation of ScheduledMatchRepository
pub struct SqliteScheduledMatchRepository;

impl ScheduledMatchRepository for SqliteScheduledMatchRepository {
    fn create(&self, _scheduled_match: &ScheduledMatchData) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_by_id(&self, _id: &str) -> Result<ScheduledMatchData, DatabaseError> {
        Err(DatabaseError::NotFound("Not implemented".to_string()))
    }

    fn get_by_league(&self, _league_id: &str) -> Result<Vec<ScheduledMatchData>, DatabaseError> {
        Ok(Vec::new())
    }

    fn get_by_round(&self, _league_id: &str, _round: u32) -> Result<Vec<ScheduledMatchData>, DatabaseError> {
        Ok(Vec::new())
    }

    fn mark_as_played(&self, _id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn delete_by_league(&self, _league_id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }
}
