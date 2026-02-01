use crate::data::{DatabaseError, TeamStatisticsRepository};
use crate::team::TeamStatistics;

/// SQLite implementation of TeamStatisticsRepository
pub struct SqliteTeamStatisticsRepository;

impl TeamStatisticsRepository for SqliteTeamStatisticsRepository {
    fn create(&self, _team_id: &str) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_by_team(&self, _team_id: &str) -> Result<TeamStatistics, DatabaseError> {
        Ok(TeamStatistics::default())
    }

    fn update(&self, _team_id: &str, _stats: &TeamStatistics) -> Result<(), DatabaseError> {
        Ok(())
    }

    fn get_league_standings(&self, _league_id: &str) -> Result<Vec<(String, TeamStatistics)>, DatabaseError> {
        Ok(Vec::new())
    }
}
