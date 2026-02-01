// UI module - TUI interface using ratatui

mod app;
mod i18n;
mod event;
mod screens;

// Re-exports
pub use app::{TuiApp, RenderState};
pub use i18n::{Language, TranslationKey, t};
pub use event::{Event, should_process_key_event, is_char_key, is_enter_key, is_esc_key, is_up_key, is_down_key, is_left_key, is_right_key};
pub use screens::MainMenuScreen;