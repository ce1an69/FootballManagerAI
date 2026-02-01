use crate::data::DatabaseError;
use crate::team::{
    Player, Team, League, MatchResult, PlayerSlot, TeamStatistics, Position, PlayerRole, Duty,
    TeamFinance, FinanceTransaction, TransactionType, SeasonFinanceReport, PlayerSeasonStats,
};

/// Repository for Team data access
pub trait TeamRepository {
    fn create(&self, team: &Team) -> Result<(), DatabaseError>;
    fn get_by_id(&self, id: &str) -> Result<Team, DatabaseError>;
    fn get_all(&self) -> Result<Vec<Team>, DatabaseError>;
    fn update(&self, team: &Team) -> Result<(), DatabaseError>;
    fn delete(&self, id: &str) -> Result<(), DatabaseError>;
    fn get_by_league(&self, league_id: &str) -> Result<Vec<Team>, DatabaseError>;
}

/// Repository for Player data access
pub trait PlayerRepository {
    fn create(&self, player: &Player) -> Result<(), DatabaseError>;
    fn create_batch(&self, players: &[Player]) -> Result<(), DatabaseError>;
    fn get_by_id(&self, id: &str) -> Result<Player, DatabaseError>;
    fn get_by_team(&self, team_id: &str) -> Result<Vec<Player>, DatabaseError>;
    fn update(&self, player: &Player) -> Result<(), DatabaseError>;
    fn delete(&self, id: &str) -> Result<(), DatabaseError>;
    fn delete_batch(&self, ids: &[String]) -> Result<(), DatabaseError>;
    fn get_free_agents(&self) -> Result<Vec<Player>, DatabaseError>;
}

/// Repository for League data access
pub trait LeagueRepository {
    fn create(&self, league: &League) -> Result<(), DatabaseError>;
    fn get_by_id(&self, id: &str) -> Result<League, DatabaseError>;
    fn update(&self, league: &League) -> Result<(), DatabaseError>;
    fn get_all(&self) -> Result<Vec<League>, DatabaseError>;
}

/// Repository for MatchResult data access
pub trait MatchRepository {
    fn save(&self, match_result: &MatchResult) -> Result<(), DatabaseError>;
    fn get_by_id(&self, id: &str) -> Result<MatchResult, DatabaseError>;
    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<MatchResult>, DatabaseError>;
    fn get_by_league(&self, league_id: &str, round: u32) -> Result<Vec<MatchResult>, DatabaseError>;
}

/// Scheduled match data
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledMatchData {
    pub id: String,
    pub league_id: String,
    pub round_number: u32,
    pub home_team_id: String,
    pub away_team_id: String,
    pub played: bool,
}

/// Repository for scheduled matches
pub trait ScheduledMatchRepository {
    fn create(&self, scheduled_match: &ScheduledMatchData) -> Result<(), DatabaseError>;
    fn get_by_id(&self, id: &str) -> Result<ScheduledMatchData, DatabaseError>;
    fn get_by_league(&self, league_id: &str) -> Result<Vec<ScheduledMatchData>, DatabaseError>;
    fn get_by_round(&self, league_id: &str, round: u32) -> Result<Vec<ScheduledMatchData>, DatabaseError>;
    fn mark_as_played(&self, id: &str) -> Result<(), DatabaseError>;
    fn delete_by_league(&self, league_id: &str) -> Result<(), DatabaseError>;
}

/// Repository for team lineups
pub trait LineupRepository {
    fn save_lineup(&self, team_id: &str, lineup: &[PlayerSlot], is_starting: bool) -> Result<(), DatabaseError>;
    fn get_starting_11(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError>;
    fn get_bench(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError>;
    fn clear_lineup(&self, team_id: &str) -> Result<(), DatabaseError>;
}

/// Repository for team statistics
pub trait TeamStatisticsRepository {
    fn create(&self, team_id: &str) -> Result<(), DatabaseError>;
    fn get_by_team(&self, team_id: &str) -> Result<TeamStatistics, DatabaseError>;
    fn update(&self, team_id: &str, stats: &TeamStatistics) -> Result<(), DatabaseError>;
    fn get_league_standings(&self, league_id: &str) -> Result<Vec<(String, TeamStatistics)>, DatabaseError>;
}

/// Market listing for transfer market
#[derive(Debug, Clone)]
pub struct MarketListing {
    pub player_id: String,
    pub asking_price: u32,
    pub listed_at: u64,
    pub reason: Option<String>,
}

/// Repository for transfer market
pub trait TransferMarketRepository {
    fn add_to_market(&self, player_id: &str, price: u32) -> Result<(), DatabaseError>;
    fn remove_from_market(&self, player_id: &str) -> Result<(), DatabaseError>;
    fn get_market_players(&self) -> Result<Vec<Player>, DatabaseError>;
    fn get_market_listing(&self, player_id: &str) -> Result<Option<MarketListing>, DatabaseError>;
    fn update_price(&self, player_id: &str, new_price: u32) -> Result<(), DatabaseError>;
}

/// Repository for team finance
pub trait TeamFinanceRepository {
    fn get(&self, team_id: &str) -> Result<TeamFinance, DatabaseError>;
    fn update(&self, finance: &TeamFinance) -> Result<(), DatabaseError>;
    fn create(&self, finance: &TeamFinance) -> Result<(), DatabaseError>;
}

/// Repository for finance transactions
pub trait FinanceTransactionRepository {
    fn create(&self, transaction: &FinanceTransaction) -> Result<(), DatabaseError>;
    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<FinanceTransaction>, DatabaseError>;
    fn get_by_date_range(
        &self,
        team_id: &str,
        start_year: u32,
        start_month: u8,
        start_day: u8,
        end_year: u32,
        end_month: u8,
        end_day: u8,
    ) -> Result<Vec<FinanceTransaction>, DatabaseError>;
    fn get_by_type(
        &self,
        team_id: &str,
        t_type: &TransactionType,
    ) -> Result<Vec<FinanceTransaction>, DatabaseError>;
}

/// Repository for season finance reports
pub trait SeasonFinanceReportRepository {
    fn get(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError>;
    fn get_or_create(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError>;
    fn update(&self, report: &SeasonFinanceReport) -> Result<(), DatabaseError>;
    fn get_history(&self, team_id: &str, limit: usize) -> Result<Vec<SeasonFinanceReport>, DatabaseError>;
}

/// Repository for player season statistics
pub trait PlayerSeasonStatsRepository {
    fn create(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError>;
    fn get_by_player(&self, player_id: &str, season: &str) -> Result<PlayerSeasonStats, DatabaseError>;
    fn get_by_team(&self, team_id: &str, season: &str) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn update(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError>;
    fn get_or_create(&self, player_id: &str, season: &str, team_id: &str) -> Result<PlayerSeasonStats, DatabaseError>;
    fn get_top_scorers(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn get_top_assists(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn get_top_rated(
        &self,
        season: &str,
        min_appearances: u32,
        limit: usize,
    ) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
}
