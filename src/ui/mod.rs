// UI module - TUI interface using ratatui

// This module will handle all UI rendering
// Placeholder for now

/// TUI application state
#[derive(Debug, Clone)]
pub struct TuiApp {
    pub running: bool,
}

impl TuiApp {
    pub fn new() -> Self {
        Self {
            running: true,
        }
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tui_app_creation() {
        let app = TuiApp::new();
        assert!(app.running);
    }

    #[test]
    fn test_tui_app_quit() {
        let mut app = TuiApp::new();
        app.quit();
        assert!(!app.running);
    }
}
