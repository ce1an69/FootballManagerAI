use crate::game::{GameState, Screen};
use crate::ui::i18n::{Language, TranslationKey, t};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Main TUI application
#[derive(Debug, Clone)]
pub struct TuiApp {
    pub running: bool,
    pub game_state: GameState,
    pub language: Language,
    pub should_quit: bool,
}

impl TuiApp {
    /// Create new TUI app
    pub fn new(game_state: GameState) -> Self {
        Self {
            running: true,
            game_state,
            language: Language::English,
            should_quit: false,
        }
    }

    /// Handle key event
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    self.quit();
                } else if self.game_state.current_screen == Screen::MainMenu {
                    self.quit();
                }
            }
            KeyCode::Esc => {
                self.game_state.go_back();
            }
            _ => {}
        }
    }

    /// Quit application
    pub fn quit(&mut self) {
        self.running = false;
        self.should_quit = true;
    }

    /// Get current screen title
    pub fn get_title(&self) -> String {
        match self.game_state.current_screen {
            Screen::MainMenu => t(TranslationKey::MainMenu, self.language).to_string(),
            Screen::TeamManagement => t(TranslationKey::TeamManagement, self.language).to_string(),
            Screen::Tactics => t(TranslationKey::Tactics, self.language).to_string(),
            Screen::TransferMarket => t(TranslationKey::TransferMarket, self.language).to_string(),
            Screen::LeagueTable => t(TranslationKey::LeagueTable, self.language).to_string(),
            Screen::Notifications => t(TranslationKey::Notifications, self.language).to_string(),
            Screen::SaveLoad => t(TranslationKey::SaveLoad, self.language).to_string(),
            Screen::MatchModeSelection => t(TranslationKey::MatchModeSelection, self.language).to_string(),
            Screen::MatchLive { .. } => "Match".to_string(),
            Screen::MatchResult { .. } => "Match Result".to_string(),
            Screen::PlayerDetail { .. } => "Player Details".to_string(),
            Screen::SeasonSummary { .. } => "Season Summary".to_string(),
            Screen::MatchHistory => "Match History".to_string(),
            Screen::FinanceReport => "Finance Report".to_string(),
            Screen::Settings => "Settings".to_string(),
        }
    }

    /// Check if on main menu
    pub fn is_on_main_menu(&self) -> bool {
        self.game_state.current_screen == Screen::MainMenu
    }

    /// Set language
    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }

    /// Get help text for current screen
    pub fn get_help_text(&self) -> String {
        match self.game_state.current_screen {
            Screen::MainMenu => {
                let q = t(TranslationKey::Quit, self.language);
                format!("↑↓: Navigate | Enter: Select | q: {}", q)
            }
            _ => {
                let esc = t(TranslationKey::Back, self.language);
                format!("Esc: {} | Ctrl+C: Quit", esc)
            }
        }
    }
}

/// Render state for UI
#[derive(Debug, Clone, PartialEq)]
pub enum RenderState {
    Idle,
    Processing,
    Error(String),
}

impl Default for RenderState {
    fn default() -> Self {
        Self::Idle
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameDate;
    use crate::team::League;
    use std::collections::HashMap;

    fn create_test_game_state() -> GameState {
        GameState {
            game_id: "test".to_string(),
            is_new_game: true,
            current_date: GameDate::new_season_start(2026),
            player_team_id: "team1".to_string(),
            player_name: None,
            league: League::new("1".to_string(), "Test".to_string(), vec![]),
            teams: HashMap::new(),
            current_screen: Screen::MainMenu,
            screen_stack: vec![],
            notifications: vec![],
            difficulty: crate::game::Difficulty::Normal,
            match_mode_preference: crate::team::MatchMode::Quick,
            db_path: String::new(),
        }
    }

    #[test]
    fn test_app_creation() {
        let state = create_test_game_state();
        let app = TuiApp::new(state);
        assert!(app.running);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_app_quit() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);
        app.quit();
        assert!(!app.running);
        assert!(app.should_quit);
    }

    #[test]
    fn test_get_title() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);

        app.language = Language::English;
        assert_eq!(app.get_title(), "Main Menu");

        app.language = Language::Chinese;
        assert_eq!(app.get_title(), "主菜单");
    }

    #[test]
    fn test_is_on_main_menu() {
        let state = create_test_game_state();
        let app = TuiApp::new(state);
        assert!(app.is_on_main_menu());
    }

    #[test]
    fn test_set_language() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);
        app.set_language(Language::Chinese);
        assert_eq!(app.language, Language::Chinese);
    }

    #[test]
    fn test_handle_esc() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);
        app.game_state.navigate_to(Screen::TeamManagement);

        let key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        app.handle_key_event(key_event);

        assert_eq!(app.game_state.current_screen, Screen::MainMenu);
    }

    #[test]
    fn test_handle_q_on_main_menu() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);

        let key_event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        app.handle_key_event(key_event);

        assert!(!app.running);
    }

    #[test]
    fn test_get_help_text() {
        let state = create_test_game_state();
        let mut app = TuiApp::new(state);

        app.language = Language::English;
        let help = app.get_help_text();
        assert!(help.contains("Quit"));

        app.language = Language::Chinese;
        let help = app.get_help_text();
        assert!(help.contains("退出"));
    }
}
