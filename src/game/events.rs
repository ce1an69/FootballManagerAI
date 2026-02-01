use crate::game::state::{Screen, Difficulty, GameState};
use crate::team::{MatchMode, Tactic};

/// Game events
#[derive(Debug, Clone)]
pub enum GameEvent {
    // Player action events
    NavigateTo { screen: Screen },
    GoBack,
    QuitGame,

    // Game flow events
    StartNewGame { player_team_id: String },
    LoadGame { save_id: String },
    SaveGame { slot: u8 },
    AdvanceToNextMatch,

    // Match related
    SelectMatchMode { mode: MatchMode },
    StartMatch { match_id: String },
    MakeSubstitution { player_out: String, player_in: String },
    AdjustTactics { new_tactic: Tactic },

    // Transfer related
    BuyPlayer { player_id: String },
    SellPlayer { player_id: String },
    ListPlayer { player_id: String, price: u32 },

    // Settings
    ChangeDifficulty { difficulty: Difficulty },
    ChangeMatchPreference { mode: MatchMode },

    // System events
    Error { message: String },
    Info { message: String },
}

/// Event handling effects
#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    Render,
    None,
}

/// Game error types
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("Team not found: {0}")]
    TeamNotFound(String),

    #[error("Player not found: {0}")]
    PlayerNotFound(String),

    #[error("Match not found: {0}")]
    MatchNotFound(String),

    #[error("No more matches available")]
    NoMoreMatches,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Invalid action: {0}")]
    InvalidAction(String),
}

/// Event handler
pub struct EventHandler;

impl EventHandler {
    /// Handle a game event
    pub fn handle_event(
        &self,
        event: GameEvent,
        state: &mut GameState,
    ) -> Result<Effect, GameError> {
        match event {
            GameEvent::NavigateTo { screen } => {
                state.navigate_to(screen);
                Ok(Effect::Render)
            }
            GameEvent::GoBack => {
                state.go_back();
                Ok(Effect::Render)
            }
            GameEvent::QuitGame => {
                Ok(Effect::None)
            }
            GameEvent::StartNewGame { player_team_id } => {
                // Will be implemented when game initialization is ready
                state.player_team_id = player_team_id;
                state.navigate_to(Screen::TeamManagement);
                Ok(Effect::Render)
            }
            GameEvent::LoadGame { save_id } => {
                // Will be implemented when save/load system is ready
                Err(GameError::DatabaseError(format!("Loading game {}", save_id)))
            }
            GameEvent::SaveGame { slot } => {
                // Will be implemented when save/load system is ready
                Err(GameError::DatabaseError(format!("Saving to slot {}", slot)))
            }
            GameEvent::AdvanceToNextMatch => {
                state.advance_time();
                Ok(Effect::Render)
            }
            GameEvent::SelectMatchMode { mode } => {
                state.match_mode_preference = mode;
                Ok(Effect::Render)
            }
            GameEvent::StartMatch { match_id } => {
                state.navigate_to(Screen::MatchLive { match_id });
                Ok(Effect::Render)
            }
            GameEvent::BuyPlayer { player_id: _ } => {
                // Will be implemented when transfer system is ready
                Ok(Effect::None)
            }
            GameEvent::SellPlayer { player_id: _ } => {
                // Will be implemented when transfer system is ready
                Ok(Effect::None)
            }
            GameEvent::ListPlayer { player_id: _, price: _ } => {
                // Will be implemented when transfer system is ready
                Ok(Effect::None)
            }
            GameEvent::ChangeDifficulty { difficulty } => {
                state.difficulty = difficulty;
                Ok(Effect::Render)
            }
            GameEvent::ChangeMatchPreference { mode } => {
                state.match_mode_preference = mode;
                Ok(Effect::Render)
            }
            GameEvent::Error { message } => {
                Err(GameError::InvalidAction(message))
            }
            GameEvent::Info { .. } => {
                Ok(Effect::None)
            }
            _ => Ok(Effect::None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::League;

    #[test]
    fn test_navigate_event() {
        let handler = EventHandler;
        let mut state = GameState {
            game_id: "test".to_string(),
            is_new_game: true,
            current_date: crate::game::state::GameDate::new_season_start(2026),
            player_team_id: "team1".to_string(),
            player_name: None,
            league: League::new("1".to_string(), "Test".to_string(), vec![]),
            teams: std::collections::HashMap::new(),
            current_screen: Screen::MainMenu,
            screen_stack: vec![],
            notifications: vec![],
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        };

        let event = GameEvent::NavigateTo {
            screen: Screen::TeamManagement,
        };

        let effect = handler.handle_event(event, &mut state).unwrap();
        assert_eq!(effect, Effect::Render);
        assert_eq!(state.current_screen, Screen::TeamManagement);
    }

    #[test]
    fn test_go_back_event() {
        let handler = EventHandler;
        let mut state = GameState {
            game_id: "test".to_string(),
            is_new_game: true,
            current_date: crate::game::state::GameDate::new_season_start(2026),
            player_team_id: "team1".to_string(),
            player_name: None,
            league: League::new("1".to_string(), "Test".to_string(), vec![]),
            teams: std::collections::HashMap::new(),
            current_screen: Screen::TeamManagement,
            screen_stack: vec![Screen::MainMenu],
            notifications: vec![],
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        };

        let effect = handler.handle_event(GameEvent::GoBack, &mut state).unwrap();
        assert_eq!(effect, Effect::Render);
        assert_eq!(state.current_screen, Screen::MainMenu);
    }

    #[test]
    fn test_change_difficulty() {
        let handler = EventHandler;
        let mut state = GameState {
            game_id: "test".to_string(),
            is_new_game: true,
            current_date: crate::game::state::GameDate::new_season_start(2026),
            player_team_id: "team1".to_string(),
            player_name: None,
            league: League::new("1".to_string(), "Test".to_string(), vec![]),
            teams: std::collections::HashMap::new(),
            current_screen: Screen::MainMenu,
            screen_stack: vec![],
            notifications: vec![],
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        };

        let effect = handler.handle_event(
            GameEvent::ChangeDifficulty { difficulty: Difficulty::Hard },
            &mut state,
        ).unwrap();

        assert_eq!(effect, Effect::Render);
        assert_eq!(state.difficulty, Difficulty::Hard);
    }
}
