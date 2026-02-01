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
            let size = f.area();
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
            app.render_current_screen(f, content_area);
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
