// Game module - Game state and event handling

mod state;
mod events;
mod r#loop;
mod flow;
mod init;
mod save;

// Re-exports
pub use state::{
    GameState, GameDate, Screen, Difficulty,
    Notification, NotificationType, NotificationPriority
};
pub use events::{GameEvent, EventHandler, Effect, GameError};
pub use r#loop::{GameLoop, GameLoopError};
pub use flow::{advance_to_next_match, update_after_match, is_last_match_of_round, is_season_complete, MatchInfo, MatchUpdateResult, MatchFlowError};
pub use init::{start_new_game, load_game, quick_start, InitError};
pub use save::{
    quick_save, save_to_slot, autosave, load_from_slot, list_save_games,
    delete_save_slot, backup_save, get_save_metadata, has_autosave, load_autosave,
    validate_save, get_save_version, SaveError
};
