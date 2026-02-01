use crate::game::{GameState, Screen};
use crate::team::MatchMode;
use crate::ui::i18n::Language;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Line},
    widgets::{Block, Borders, List, Paragraph, Wrap, ListState},
    Frame,
};

/// Settings screen
#[derive(Debug, Clone, Default)]
pub struct SettingsScreen {
    selected_index: usize,
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),     // Settings list
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Settings")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White).bold());

        frame.render_widget(title, chunks[0]);

        // Settings list
        let items = vec![
            "Language: English".to_string(),
            format!("Match Mode: {:?}", state.match_mode_preference),
        ];

        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index));

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(
            list,
            chunks[1],
            &mut list_state,
        );

        // Help
        let help = Paragraph::new("↑↓: Select | Enter: Toggle | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

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
                self.selected_index += 1;
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                // Toggle setting (placeholder - would need state mutation)
                None
            }
            _ => None,
        }
    }
}

/// Save/Load screen
#[derive(Debug, Clone, Default)]
pub struct SaveLoadScreen {
    selected_index: usize,
    save_slots: Vec<String>, // Slot 1-10
    save_metadata: Vec<(u8, bool)>, // (slot_number, is_empty)
}

impl SaveLoadScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            save_slots: vec![
                "Slot 1 - Empty".to_string(),
                "Slot 2 - Empty".to_string(),
                "Slot 3 - Empty".to_string(),
                "Slot 4 - Empty".to_string(),
                "Slot 5 - Empty".to_string(),
            ],
            save_metadata: vec![(1, true), (2, true), (3, true), (4, true), (5, true)],
        }
    }

    /// Update save slots with actual metadata
    pub fn update_save_slots(&mut self, saves: &[crate::data::SaveMetadata]) {
        self.save_slots.clear();
        self.save_metadata.clear();

        // Always show 5 slots
        for slot in 1..=5 {
            if let Some(save) = saves.iter().find(|s| s.slot == slot) {
                self.save_slots.push(format!(
                    "Slot {} - {} | {} | {}",
                    slot, save.team_name, save.date, save.season
                ));
                self.save_metadata.push((slot, false));
            } else {
                self.save_slots.push(format!("Slot {} - Empty", slot));
                self.save_metadata.push((slot, true));
            }
        }
    }

    /// Get selected slot number
    pub fn selected_slot(&self) -> u8 {
        if self.selected_index < self.save_metadata.len() {
            self.save_metadata[self.selected_index].0
        } else {
            1
        }
    }

    /// Check if selected slot is empty
    pub fn is_selected_slot_empty(&self) -> bool {
        if self.selected_index < self.save_metadata.len() {
            self.save_metadata[self.selected_index].1
        } else {
            true
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(15),     // Save slots
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Save / Load Game")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White).bold());

        frame.render_widget(title, chunks[0]);

        // Save slots list
        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index));

        let list = List::new(self.save_slots.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(
            list,
            chunks[1],
            &mut list_state,
        );

        // Help
        let help = Paragraph::new("↑↓: Select | Enter: Save/Load | Del: Delete | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

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
                if self.selected_index < self.save_slots.len() - 1 {
                    self.selected_index += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                // Return to main menu after save/load operation
                // The actual save/load will be handled by the main game loop
                Some(Screen::MainMenu)
            }
            KeyCode::Delete => {
                // Delete save at selected slot
                // The actual deletion will be handled by the main game loop
                if !self.is_selected_slot_empty() {
                    // Refresh save slots after deletion
                    // In real implementation, would call SaveManager::delete_save
                }
                None
            }
            _ => None,
        }
    }
}

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

/// League table (standings) screen
#[derive(Debug, Clone, Default)]
pub struct LeagueTableScreen {
    selected_tab: usize, // 0: Standings, 1: Top Scorers
}

impl LeagueTableScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),     // Title + Tabs
                Constraint::Min(10),         // Table content
                Constraint::Length(3),     // Help
            ])
            .split(area);

        // Title with tabs
        let tabs = vec!["Standings", "Top Scorers"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tabs_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tabs_paragraph, chunks[0]);

        // Table content (placeholder - would show actual standings)
        let content = if self.selected_tab == 0 {
            // Standings
            let header = vec![
                "#  Team        P  W  D  L  GF  GA  Pts",
                "----------------------------------------",
                "1  Team A      3  2  1  0   5  2   7",
                "2  Team B      3  1  1  1   3  4   4",
                "3  Team C      3  0  2  1   2  5   2",
            ];
            header.join("\n")
        } else {
            // Top Scorers
            "Top Scorers:\n\n  1. Player A (Team A) - 5 goals\n  2. Player B (Team B) - 3 goals".to_string()
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 1 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            _ => None,
        }
    }
}

/// Match mode selection screen
#[derive(Debug, Clone, Default)]
pub struct MatchModeSelectionScreen {
    selected_mode: usize, // 0: Live, 1: Quick
}

impl MatchModeSelectionScreen {
    pub fn new() -> Self {
        Self {
            selected_mode: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Select Match Mode")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White).bold());

        frame.render_widget(title, chunks[0]);

        // Match modes
        let modes = vec![
            ("Live Text", "Watch match unfold with real-time commentary"),
            ("Quick Sim", "Instant result with detailed statistics"),
        ];

        let mode_text: Vec<Line> = modes.iter()
            .enumerate()
            .map(|(i, (name, desc))| {
                let style = if i == self.selected_mode {
                    Style::default()
                        .fg(Color::Yellow)
                        .bold()
                        .add_modifier(Modifier::REVERSED)
                } else {
                    Style::default().fg(Color::White)
                };
                Line::from(vec![
                    Span::styled(format!("{}:", name), style),
                    Span::styled(format!(" {}", desc), Style::default().fg(Color::Gray)),
                ])
            })
            .collect();

        let modes_paragraph = Paragraph::new(mode_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(modes_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("↑↓: Select | Enter: Confirm | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Up => {
                if self.selected_mode > 0 {
                    self.selected_mode -= 1;
                }
                None
            }
            KeyCode::Down => {
                if self.selected_mode < 1 {
                    self.selected_mode += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                let mode = if self.selected_mode == 0 {
                    MatchMode::Live
                } else {
                    MatchMode::Quick
                };
                // Start match with selected mode
                // The match_id should be provided by the game state or passed during screen creation
                // For now, use a placeholder that the main loop will replace with actual match_id
                Some(Screen::MatchLive { match_id: "next_match".to_string() })
            }
            _ => None,
        }
    }
}

/// Player detail screen
#[derive(Debug, Clone, Default)]
pub struct PlayerDetailScreen {
    player_id: String,
    selected_tab: usize, // 0: Overview, 1: Attributes, 2: Contract
}

impl PlayerDetailScreen {
    pub fn new(player_id: String) -> Self {
        Self {
            player_id,
            selected_tab: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        // Placeholder: Player data would be fetched from database in real implementation
        let player_found = true; // Simulating player exists

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Header with tabs
        let tabs = vec!["Overview", "Attributes", "Contract"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tabs_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tabs_paragraph, chunks[0]);

        // Content based on selected tab (placeholder data)
        let content = if player_found {
            match self.selected_tab {
                0 => format!(
                    "Player ID: {}\n\n[Placeholder player data]\nName: Player Name\nAge: 25\nPosition: ST\nOverall Ability: 80",
                    self.player_id
                ),
                1 => "Technical attributes: [placeholder]\n• Finishing: 85\n• Passing: 75\n• Dribbling: 80\n\nMental attributes: [placeholder]\n• Decisions: 78\n• Teamwork: 82\n\nPhysical attributes: [placeholder]\n• Pace: 84\n• Stamina: 79".to_string(),
                2 => format!(
                    "Wage: £50K/week\nContract: 3 years\nMarket Value: £5M\n\n[Contract details placeholder]"
                ),
                _ => "Invalid tab".to_string(),
            }
        } else {
            "Player not found".to_string()
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 2 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::TeamManagement),
            _ => None,
        }
    }
}

/// Match result screen
#[derive(Debug, Clone, Default)]
pub struct MatchResultScreen {
    match_id: String,
    selected_tab: usize, // 0: Overview, 1: Statistics, 2: Player Ratings
}

impl MatchResultScreen {
    pub fn new(match_id: String) -> Self {
        Self {
            match_id,
            selected_tab: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Header + Tabs
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Header with tabs
        let tabs = vec!["Overview", "Statistics", "Player Ratings"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tabs_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tabs_paragraph, chunks[0]);

        // Content
        let content = match self.selected_tab {
            0 => "Final Score: 2 - 1\n[Match overview placeholder]",
            1 => "Possession: 52% - 48%\nShots: 15 - 10\n[Statistics placeholder]",
            2 => "[Player ratings placeholder]",
            _ => "Invalid tab",
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | Enter: Continue | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 2 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => Some(Screen::MainMenu),
            _ => None,
        }
    }
}

/// Season summary screen
#[derive(Debug, Clone, Default)]
pub struct SeasonSummaryScreen {
    season: String,
}

impl SeasonSummaryScreen {
    pub fn new(season: String) -> Self {
        Self { season }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let content = Paragraph::new(format!("Season {} Summary\n\n[Season summary placeholder]", self.season))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center);

        frame.render_widget(content, area);

        // Help
        let help = Paragraph::new("Enter: Continue | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        let help_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(area);

        frame.render_widget(help, help_area[1]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        match key_event.code {
            crossterm::event::KeyCode::Enter => Some(Screen::MainMenu),
            crossterm::event::KeyCode::Esc => Some(Screen::MainMenu),
            _ => None,
        }
    }
}

/// Match history screen
#[derive(Debug, Clone, Default)]
pub struct MatchHistoryScreen {
    selected_filter: usize, // 0: All, 1: Wins, 2: Draws, 3: Losses
    selected_match_index: usize,
    match_ids: Vec<String>, // Store match IDs for navigation
}

impl MatchHistoryScreen {
    pub fn new() -> Self {
        Self {
            selected_filter: 0,
            selected_match_index: 0,
            match_ids: Vec::new(),
        }
    }

    /// Update match list with IDs from game state
    pub fn update_match_list(&mut self, match_ids: Vec<String>) {
        self.match_ids = match_ids;
        self.selected_match_index = 0;
    }

    /// Get selected match ID
    pub fn selected_match_id(&self) -> Option<String> {
        if self.selected_match_index < self.match_ids.len() {
            Some(self.match_ids[self.selected_match_index].clone())
        } else {
            None
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title + Filter
                Constraint::Min(10),     // Match list
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title with filter
        let filters = vec!["All", "Wins", "Draws", "Losses"];
        let filter_line: Vec<Line> = filters.iter()
            .enumerate()
            .map(|(i, filter)| {
                let style = if i == self.selected_filter {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *filter), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let filter_paragraph = Paragraph::new(filter_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(filter_paragraph, chunks[0]);

        // Match list
        let content = "[Match history placeholder]\n\nMatch 1: Team A vs Team B (2-1)\nMatch 2: Team C vs Team D (1-1)";

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Filter | ↑↓: Select | Enter: View Match | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_filter > 0 {
                    self.selected_filter -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_filter < 3 {
                    self.selected_filter += 1;
                }
                None
            }
            KeyCode::Up => {
                if self.selected_match_index > 0 {
                    self.selected_match_index -= 1;
                }
                None
            }
            KeyCode::Down => {
                if !self.match_ids.is_empty() && self.selected_match_index < self.match_ids.len() - 1 {
                    self.selected_match_index += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                // Open match detail for selected match
                if let Some(match_id) = self.selected_match_id() {
                    Some(Screen::MatchResult { match_id })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Notifications screen
#[derive(Debug, Clone, Default)]
pub struct NotificationsScreen {
    selected_index: usize,
    notification_ids: Vec<String>, // Store notification IDs for marking as read
}

impl NotificationsScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            notification_ids: Vec::new(),
        }
    }

    /// Update notification list with IDs from game state
    pub fn update_notification_list(&mut self, notifications: &[crate::game::Notification]) {
        self.notification_ids = notifications.iter().map(|n| n.id.clone()).collect();
        self.selected_index = 0;
    }

    /// Get selected notification ID
    pub fn selected_notification_id(&self) -> Option<String> {
        if self.selected_index < self.notification_ids.len() {
            Some(self.notification_ids[self.selected_index].clone())
        } else {
            None
        }
    }

    /// Mark notification at current index as read (called by main loop)
    pub fn mark_current_as_read(&mut self) {
        if self.selected_index < self.notification_ids.len() {
            // The actual read status is in GameState, this just advances to next
            if self.selected_index < self.notification_ids.len() - 1 {
                self.selected_index += 1;
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        // Update notification IDs from state
        self.update_notification_list(&state.notifications);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title
                Constraint::Min(10),     // Notifications
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title
        let title = Paragraph::new("Notifications")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White).bold());

        frame.render_widget(title, chunks[0]);

        // Notifications list
        let notifications = &state.notifications;
        let items: Vec<String> = if notifications.is_empty() {
            vec!["No notifications".to_string()]
        } else {
            notifications.iter().take(20).enumerate().map(|(i, n)| {
                let prefix = if n.read { "  " } else { "* " };
                format!("{}[{}] {}: {}", prefix, i + 1, n.notification_type, n.message)
            }).collect()
        };

        let mut list_state = ListState::default();
        list_state.select(Some(self.selected_index));

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD));

        frame.render_stateful_widget(
            list,
            chunks[1],
            &mut list_state,
        );

        // Help
        let help = Paragraph::new("↑↓: Select | Enter: Mark as Read | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

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
                if !self.notification_ids.is_empty() && self.selected_index < self.notification_ids.len() - 1 {
                    self.selected_index += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                // Mark selected notification as read
                // The actual marking will be handled by the main game loop
                // which has mutable access to GameState
                if let Some(_notification_id) = self.selected_notification_id() {
                    // Signal that notification should be marked as read
                    // The main loop will call GameState::mark_notification_read
                }
                None
            }
            _ => None,
        }
    }
}

/// Tactics screen
#[derive(Debug, Clone, Default)]
pub struct TacticsScreen {
    selected_area: usize, // 0: Formation, 1: Player Roles, 2: Team Instructions
}

impl TacticsScreen {
    pub fn new() -> Self {
        Self {
            selected_area: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Title + Area tabs
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Title with tabs
        let areas = vec!["Formation", "Player Roles", "Team Instructions"];
        let area_line: Vec<Line> = areas.iter()
            .enumerate()
            .map(|(i, area)| {
                let style = if i == self.selected_area {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *area), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let area_paragraph = Paragraph::new(area_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(area_paragraph, chunks[0]);

        // Content
        let team = state.teams.get(&state.player_team_id);
        let content = if let Some(team) = team {
            match self.selected_area {
                0 => format!(
                    "Current Formation: {}\n\n[Formation diagram placeholder]",
                    team.tactic.formation
                ),
                1 => "[Player roles placeholder - position by position]\n".to_string(),
                2 => format!(
                    "Attacking Mentality: {}\nDefensive Height: {}\nPassing Style: {}\nTempo: {}",
                    team.tactic.attacking_mentality,
                    team.tactic.defensive_height,
                    team.tactic.passing_style,
                    team.tactic.tempo
                ),
                _ => "Invalid area".to_string(),
            }
        } else {
            "Team not found".to_string()
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Area | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_area > 0 {
                    self.selected_area -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_area < 2 {
                    self.selected_area += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            _ => None,
        }
    }
}

/// Transfer market screen
#[derive(Debug, Clone, Default)]
pub struct TransferMarketScreen {
    selected_tab: usize, // 0: Browse, 1: Sell
}

impl TransferMarketScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Header + Tabs
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Header with tabs
        let tabs = vec!["Browse Market", "Sell Players"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tabs_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tabs_paragraph, chunks[0]);

        // Content
        let content = if self.selected_tab == 0 {
            "[Transfer market players list placeholder]\n\n1. Player A - ST - OVR 85 - £5M\n2. Player B - MC - OVR 78 - £3M"
        } else {
            "[Your squad placeholder - Sell players list]\n\nBudget: £10M"
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | Enter: Action | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 1 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            _ => None,
        }
    }
}

/// Team management screen
#[derive(Debug, Clone, Default)]
pub struct TeamManagementScreen {
    selected_tab: usize, // 0: Squad, 1: Tactics, 2: Statistics
    selected_index: usize,
}

impl TeamManagementScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
            selected_index: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Header + Tabs
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Header with tabs
        let tabs = vec!["Squad", "Tactics", "Statistics"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tab_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tab_paragraph, chunks[0]);

        // Content (placeholder - team.players contains player IDs, not Player objects)
        let team = state.teams.get(&state.player_team_id);
        let content = if let Some(team) = team {
            match self.selected_tab {
                0 => {
                    // Squad - show placeholder player list
                    let player_count = team.players.len();
                    format!(
                        "Squad List ({} players)\n\n[Player data requires database access]\n\n1. Player A - ST - OVR 85\n2. Player B - MC - OVR 78\n3. Player C - CB - OVR 82\n... and {} more players",
                        player_count, player_count.saturating_sub(3)
                    )
                }
                1 => format!("Tactics placeholder\nFormation: {}", team.tactic.formation),
                2 => format!(
                    "Statistics placeholder\nMatches: {}\nGoals: {}/{}",
                    team.statistics.matches_played,
                    team.statistics.goals_for,
                    team.statistics.goals_for + team.statistics.goals_against
                ),
                _ => "Invalid tab".to_string(),
            }
        } else {
            "Team not found".to_string()
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | ↑↓: Select | Enter: Action | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 2 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                None
            }
            KeyCode::Down => {
                self.selected_index += 1;
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            KeyCode::Enter => {
                match self.selected_tab {
                    0 => Some(Screen::PlayerDetail { player_id: "placeholder".to_string() }),
                    1 => Some(Screen::Tactics),
                    2 => None,
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

/// Match live screen
#[derive(Debug, Clone, Default)]
pub struct MatchLiveScreen {
    match_id: String,
    is_paused: bool,
    current_minute: u32,
}

impl MatchLiveScreen {
    pub fn new(match_id: String) -> Self {
        Self {
            match_id,
            is_paused: false,
            current_minute: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, _state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(5),  // Score + Time
                Constraint::Min(10),     // Events
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Score and time
        let score_time = format!(
            "{}  {} - {}  |  {}'\"",
            if self.is_paused { "[PAUSED]" } else { "Live  " },
            "Team A",
            2,
            self.current_minute
        );

        let header = Paragraph::new(score_time)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green)),
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White).bold());

        frame.render_widget(header, chunks[0]);

        // Events
        let events = "[Match events placeholder]\n\n12': Goal! Team A scores\n35': Yellow card - Team B\n\n[More events would appear here]";

        let events_paragraph = Paragraph::new(events)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left);

        frame.render_widget(events_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("Space: Pause/Resume | Esc: End Match | Enter: Continue")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Char(' ') => {
                self.is_paused = !self.is_paused;
                None
            }
            KeyCode::Esc => Some(Screen::MatchResult { match_id: self.match_id.clone() }),
            KeyCode::Enter => {
                self.current_minute += 1; // Simulate time passing
                if self.current_minute >= 90 {
                    Some(Screen::MatchResult { match_id: self.match_id.clone() })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Finance report screen
#[derive(Debug, Clone, Default)]
pub struct FinanceReportScreen {
    selected_tab: usize, // 0: Overview, 1: Income, 2: Expenses, 3: History
}

impl FinanceReportScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: 0,
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, state: &GameState, _language: Language) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([
                Constraint::Length(3),  // Header + Tabs
                Constraint::Min(10),     // Content
                Constraint::Length(3),  // Help
            ])
            .split(area);

        // Header with tabs
        let tabs = vec!["Overview", "Income", "Expenses", "History"];
        let tab_line: Vec<Line> = tabs.iter()
            .enumerate()
            .map(|(i, tab)| {
                let style = if i == self.selected_tab {
                    Style::default().fg(Color::Yellow).bold()
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(vec![
                    Span::styled(format!(" {} ", *tab), style),
                    Span::styled("|", Style::default().fg(Color::DarkGray)),
                ])
            })
            .collect();

        let tab_paragraph = Paragraph::new(tab_line)
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(tab_paragraph, chunks[0]);

        // Content
        let team = state.teams.get(&state.player_team_id);
        let content = if let Some(team) = team {
            match self.selected_tab {
                0 => format!(
                    "Budget: £{}\n\nBalance: £{}",
                    team.budget,
                    team.budget // Placeholder for balance
                ),
                1 => "[Income breakdown placeholder]\n• Ticket sales: £500K\n• TV revenue: £2M".to_string(),
                2 => "[Expenses breakdown placeholder]\n• Wages: £10M\n• Transfer fees: £5M".to_string(),
                3 => "[Financial history placeholder]".to_string(),
                _ => "Invalid tab".to_string(),
            }
        } else {
            "Team not found".to_string()
        };

        let content_paragraph = Paragraph::new(content)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });

        frame.render_widget(content_paragraph, chunks[1]);

        // Help
        let help = Paragraph::new("←→: Switch Tab | Esc: Back")
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center);

        frame.render_widget(help, chunks[2]);
    }

    pub fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> Option<Screen> {
        use crossterm::event::KeyCode;

        match key_event.code {
            KeyCode::Left => {
                if self.selected_tab > 0 {
                    self.selected_tab -= 1;
                }
                None
            }
            KeyCode::Right => {
                if self.selected_tab < 3 {
                    self.selected_tab += 1;
                }
                None
            }
            KeyCode::Esc => Some(Screen::MainMenu),
            _ => None,
        }
    }
}
