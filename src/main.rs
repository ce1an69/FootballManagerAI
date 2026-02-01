use football_manager_ai::game::GameState;
use football_manager_ai::game::init::{quick_start, load_game};
use football_manager_ai::ui::TuiApp;
use football_manager_ai::ui::{
    MainMenuScreen,
    SettingsScreen,
    SaveLoadScreen,
    LeagueTableScreen,
    MatchModeSelectionScreen,
    PlayerDetailScreen,
    MatchResultScreen,
    SeasonSummaryScreen,
    MatchHistoryScreen,
    NotificationsScreen,
    TacticsScreen,
    TransferMarketScreen,
    TeamManagementScreen,
    MatchLiveScreen,
    FinanceReportScreen,
};
use football_manager_ai::data::SaveManager;
use std::io;
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create or load game
    let game_state = setup_game()?;

    // Create TUI app
    let mut app = TuiApp::new(game_state);

    // Run application
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut TuiApp,
) -> io::Result<()> {

    loop {
        // Render current screen
        terminal.draw(|f| {
            let size = f.size();

            // Draw title bar
            let title = ratatui::widgets::Paragraph::new(app.get_title())
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan));
            f.render_widget(title, ratatui::layout::Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: 1,
            });

            // Draw help bar at bottom
            let help = ratatui::widgets::Paragraph::new(app.get_help_text())
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::Gray));
            f.render_widget(help, ratatui::layout::Rect {
                x: 0,
                y: size.height.saturating_sub(1),
                width: size.width,
                height: 1,
            });

            // Draw main content area
            let content_area = ratatui::layout::Rect {
                x: 0,
                y: 1,
                width: size.width,
                height: size.height.saturating_sub(2),
            };

            // Render based on current screen
            match &app.game_state.current_screen {
                football_manager_ai::game::Screen::MainMenu => {
                    let mut screen = MainMenuScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::Settings => {
                    let mut screen = SettingsScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::SaveLoad => {
                    let mut screen = SaveLoadScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::LeagueTable => {
                    let mut screen = LeagueTableScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::TeamManagement => {
                    let mut screen = TeamManagementScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::Tactics => {
                    let mut screen = TacticsScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::TransferMarket => {
                    let mut screen = TransferMarketScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::MatchModeSelection => {
                    let mut screen = MatchModeSelectionScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::PlayerDetail { .. } => {
                    let mut screen = PlayerDetailScreen::new("player_1".to_string());
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::MatchResult { .. } => {
                    let mut screen = MatchResultScreen::new("match_1".to_string());
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::SeasonSummary { .. } => {
                    let mut screen = SeasonSummaryScreen::new("2026-27".to_string());
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::MatchHistory => {
                    let mut screen = MatchHistoryScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::Notifications => {
                    let mut screen = NotificationsScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::MatchLive { .. } => {
                    let mut screen = MatchLiveScreen::new("match_1".to_string());
                    screen.render(f, content_area, &app.game_state, app.language);
                }
                football_manager_ai::game::Screen::FinanceReport => {
                    let mut screen = FinanceReportScreen::new();
                    screen.render(f, content_area, &app.game_state, app.language);
                }
            }
        })?;

        // Handle input
        use crossterm::event;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = event::read()? {
                app.handle_key_event(key);

                if !app.running {
                    return Ok(());
                }
            }
        }
    }
}

fn setup_game() -> Result<GameState, Box<dyn std::error::Error>> {
    // Check for existing saves
    let save_manager = SaveManager::default();
    let saves = save_manager.list_saves()?;

    if !saves.is_empty() {
        // Load the most recent save
        println!("Loading saved game from slot {}...", saves[0].slot);
        let (game_state, _db) = load_game(saves[0].slot)?;
        println!("Game loaded successfully!");
        return Ok(game_state);
    }

    // No saves found, create new game using quick_start
    println!("No saved games found. Creating new game...");
    let (game_state, _db) = quick_start()?;
    println!("Game created successfully!");

    Ok(game_state)
}
