// Team module - Core data models for Football Manager AI

mod player;
mod team;
mod league;
mod match_result;
mod tactics;
mod attributes;
mod player_stats;
mod finance;

// Re-export all public types
pub use player::{Player, Position, Foot, PlayerStatus};
pub use team::{Team, PlayerSlot, TeamStatistics};
pub use league::{League, MatchSchedule, Round, ScheduledMatch};
pub use match_result::{MatchResult, MatchEvent, MatchMode, MatchStatistics, PlayerMatchRating};
pub use tactics::{Tactic, Formation, DefensiveHeight, PassingStyle, Tempo, PlayerRole, Duty, PlayerRoleAssignment};
pub use attributes::{calculate_market_value, calculate_wage};
pub use player_stats::PlayerSeasonStats;
pub use finance::{TeamFinance, FinanceTransaction, TransactionType, SeasonFinanceReport};

// Re-export GameDate from game module for backward compatibility
pub use crate::game::GameDate;
