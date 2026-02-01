// Game module - Game state and event handling

mod state;
mod events;
mod r#loop;
mod flow;

// Re-exports
pub use state::{
    GameState, GameDate, Screen, Difficulty,
    Notification, NotificationType, NotificationPriority
};
pub use events::{GameEvent, EventHandler, Effect, GameError};
pub use r#loop::{GameLoop, GameLoopError};
pub use flow::{advance_to_next_match, update_after_match, is_last_match_of_round, is_season_complete, MatchInfo, MatchUpdateResult, MatchFlowError};
