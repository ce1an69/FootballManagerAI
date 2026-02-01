// Data module - Database access and persistence

mod database;
mod repository;
mod team_repo;
mod player_repo;
mod league_repo;
mod match_repo;
mod scheduled_match_repo;
mod lineup_repo;
mod team_statistics_repo;
mod transfer_repo;
mod transfer_history_repo;
mod team_finance_repo;
mod finance_transaction_repo;
mod season_finance_report_repo;
mod player_season_stats_repo;
mod save_manager;
mod indexes;

#[cfg(test)]
mod integration_tests;

// Re-exports
pub use database::{Database, DatabaseError};
pub use repository::{
    TeamRepository, PlayerRepository, LeagueRepository, MatchRepository,
    ScheduledMatchRepository, LineupRepository, TeamStatisticsRepository,
    TransferMarketRepository, ScheduledMatchData, MarketListing,
    TeamFinanceRepository, FinanceTransactionRepository, SeasonFinanceReportRepository,
    PlayerSeasonStatsRepository,
};
pub use team_repo::SqliteTeamRepository;
pub use player_repo::SqlitePlayerRepository;
pub use league_repo::SqliteLeagueRepository;
pub use match_repo::SqliteMatchRepository;
pub use scheduled_match_repo::SqliteScheduledMatchRepository;
pub use lineup_repo::SqliteLineupRepository;
pub use team_statistics_repo::SqliteTeamStatisticsRepository;
pub use transfer_repo::SqliteTransferMarketRepository;
pub use transfer_history_repo::{TransferHistoryRepository, SqliteTransferHistoryRepository};
pub use team_finance_repo::SqliteTeamFinanceRepository;
pub use finance_transaction_repo::SqliteFinanceTransactionRepository;
pub use season_finance_report_repo::SqliteSeasonFinanceReportRepository;
pub use player_season_stats_repo::SqlitePlayerSeasonStatsRepository;
pub use save_manager::{SaveManager, SaveMetadata};
