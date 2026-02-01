use crate::game::{GameState, Screen};
use crate::ui::i18n::{Language, TranslationKey, t};
use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Main menu screen
#[derive(Debug, Clone, Default)]
pub struct MainMenuScreen {
    selected_index: usize,
    menu_items: Vec<MenuAction>,
}

#[derive(Debug, Clone, Copy)]
enum MenuAction {
    TeamManagement,
    Tactics,
    TransferMarket,
    NextMatch,
    LeagueTable,
    SaveLoad,
    Settings,
    ExitGame,
}

impl MainMenuScreen {
    /// Create new main menu screen
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            menu_items: vec![
                MenuAction::TeamManagement,
                MenuAction::Tactics,
                MenuAction::TransferMarket,
                MenuAction::NextMatch,
                MenuAction::LeagueTable,
                MenuAction::SaveLoad,
                MenuAction::Settings,
                MenuAction::ExitGame,
            ],
        }
    }

    /// Render main menu
    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        // Split into title and menu sections
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(7),  // Title
                Constraint::Length(16), // Menu items
                Constraint::Min(0),      // Help text
            ])
            .split(area);

        // Render title
        let team_name = state.teams.get(&state.player_team_id)
            .map(|t| t.name.as_str())
            .unwrap_or("Unknown");

        let date_str = format!("{}", state.current_date);

        let title = vec![
            Line::from(vec![
                Span::styled("Football Manager", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("AI", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(format!("Team: {}", team_name), Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled(format!("Date: {}", date_str), Style::default().fg(Color::Gray)),
            ]),
        ];

        let title_paragraph = Paragraph::new(title)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green))
                    .title("Main Menu"),
            )
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(title_paragraph, chunks[0]);

        // Render menu items
        let menu_lines: Vec<Line> = self.menu_items
            .iter()
            .enumerate()
            .map(|(i, action)| {
                let label = self.get_menu_label(*action);
                let style = if i == self.selected_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(Color::White)
                };
                Line::from(vec![Span::styled(format!("  {}  ", label), style)])
            })
            .collect();

        let menu_paragraph = Paragraph::new(menu_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(menu_paragraph, chunks[1]);

        // Render help text
        let help_text = vec![
            Line::from(vec![
                Span::styled("↑↓: Navigate | Enter: Select | q: Quit", Style::default().fg(Color::Gray)),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(help_paragraph, chunks[2]);
    }

    /// Get menu label for action
    fn get_menu_label(&self, action: MenuAction) -> &str {
        match action {
            MenuAction::TeamManagement => "Team Management",
            MenuAction::Tactics => "Tactics",
            MenuAction::TransferMarket => "Transfer Market",
            MenuAction::NextMatch => "Next Match",
            MenuAction::LeagueTable => "League Table",
            MenuAction::SaveLoad => "Save / Load",
            MenuAction::Settings => "Settings",
            MenuAction::ExitGame => "Exit Game",
        }
    }

    /// Handle key event
    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                None
            }
            KeyCode::Down => {
                if self.selected_index < self.menu_items.len() - 1 {
                    self.selected_index += 1;
                }
                None
            }
            KeyCode::Enter => {
                match self.menu_items[self.selected_index] {
                    MenuAction::TeamManagement => Some(Screen::TeamManagement),
                    MenuAction::Tactics => Some(Screen::Tactics),
                    MenuAction::TransferMarket => Some(Screen::TransferMarket),
                    MenuAction::NextMatch => Some(Screen::MatchModeSelection),
                    MenuAction::LeagueTable => Some(Screen::LeagueTable),
                    MenuAction::SaveLoad => Some(Screen::SaveLoad),
                    MenuAction::Settings => Some(Screen::Settings),
                    MenuAction::ExitGame => None, // Signal to quit
                }
            }
            KeyCode::Char('q') => None, // Signal to quit
            _ => None,
        }
    }

    /// Get selected action
    pub fn selected_action(&self) -> MenuAction {
        self.menu_items[self.selected_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_menu_creation() {
        let menu = MainMenuScreen::new();
        assert_eq!(menu.selected_index, 0);
        assert_eq!(menu.menu_items.len(), 8);
    }

    #[test]
    fn test_menu_navigation() {
        let mut menu = MainMenuScreen::new();
        use crossterm::event::{KeyEvent, KeyCode};

        // Test down
        menu.handle_key_event(KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE));
        assert_eq!(menu.selected_index, 1);

        // Test up
        menu.handle_key_event(KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE));
        assert_eq!(menu.selected_index, 0);
    }

    #[test]
    fn test_menu_boundaries() {
        let mut menu = MainMenuScreen::new();
        use crossterm::event::{KeyEvent, KeyCode};

        // Try to go up at top
        menu.handle_key_event(KeyEvent::new(KeyCode::Up, crossterm::event::KeyModifiers::NONE));
        assert_eq!(menu.selected_index, 0);

        // Try to go down at bottom
        menu.selected_index = 7;
        menu.handle_key_event(KeyEvent::new(KeyCode::Down, crossterm::event::KeyModifiers::NONE));
        assert_eq!(menu.selected_index, 7);
    }
}
