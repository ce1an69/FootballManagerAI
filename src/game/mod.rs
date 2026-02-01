// Game module - Game state and event handling

mod state;
mod events;

// Re-exports
pub use state::{
    GameState, GameDate, Screen, Difficulty,
    Notification, NotificationType, NotificationPriority
};
pub use events::{GameEvent, EventHandler, Effect, GameError};
