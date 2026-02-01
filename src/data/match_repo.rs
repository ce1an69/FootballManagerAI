use crate::data::{DatabaseError, MatchRepository};
use crate::team::MatchResult;

/// SQLite implementation of MatchRepository
pub struct SqliteMatchRepository;

impl MatchRepository for SqliteMatchRepository {
    fn save(&self, _match_result: &MatchResult) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_by_id(&self, _id: &str) -> Result<MatchResult, DatabaseError> {
        Err(DatabaseError::NotFound("Not implemented".to_string()))
    }

    fn get_by_team(&self, _team_id: &str, _limit: usize) -> Result<Vec<MatchResult>, DatabaseError> {
        Ok(Vec::new())
    }

    fn get_by_league(&self, _league_id: &str, _round: u32) -> Result<Vec<MatchResult>, DatabaseError> {
        Ok(Vec::new())
    }
}
