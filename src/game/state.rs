use crate::team::{League, Team, MatchResult, MatchMode};
use std::collections::HashMap;
use uuid::Uuid;

/// Game state structure
#[derive(Debug, Clone)]
pub struct GameState {
    // Basic game information
    pub game_id: String,
    pub is_new_game: bool,
    pub current_date: GameDate,

    // Player information
    pub player_team_id: String,
    pub player_name: Option<String>,

    // League and teams
    pub league: League,
    pub teams: HashMap<String, Team>,

    // UI state
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,

    // Notifications
    pub notifications: Vec<Notification>,

    // Game configuration
    pub difficulty: Difficulty,
    pub match_mode_preference: MatchMode,

    // Database path for persistence
    pub db_path: String,
}

/// In-game date system
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct GameDate {
    pub year: u16,    // e.g., 2026
    pub month: u8,    // 1-12
    pub day: u8,      // 1-31
}

impl GameDate {
    /// Create new date (default season start: August 1st)
    pub fn new_season_start(year: u16) -> Self {
        Self { year, month: 8, day: 1 }
    }

    /// Advance by one day
    pub fn advance_day(&mut self) {
        self.day += 1;
        let days_in_month = self.days_in_current_month();
        if self.day > days_in_month {
            self.day = 1;
            self.month += 1;
            if self.month > 12 {
                self.month = 1;
                self.year += 1;
            }
        }
    }

    /// Advance by one week
    pub fn advance_week(&mut self) {
        for _ in 0..7 {
            self.advance_day();
        }
    }

    /// Advance to next match day (simplified: 3-4 days)
    pub fn advance_to_next_matchday(&mut self) {
        for _ in 0..3 {
            self.advance_day();
        }
    }

    /// Check if in summer transfer window (June 1 - August 31)
    pub fn is_summer_transfer_window(&self) -> bool {
        self.month >= 6 && self.month <= 8
    }

    /// Check if in winter transfer window (January 1 - January 31)
    pub fn is_winter_transfer_window(&self) -> bool {
        self.month == 1
    }

    /// Check if in any transfer window
    pub fn is_transfer_window(&self) -> bool {
        self.is_summer_transfer_window() || self.is_winter_transfer_window()
    }

    /// Check if season ended (May 31st)
    pub fn is_season_end(&self) -> bool {
        self.month == 5 && self.day == 31
    }

    /// Check if new season started (August 1st)
    pub fn is_season_start(&self) -> bool {
        self.month == 8 && self.day == 1
    }

    /// Get days in current month
    fn days_in_current_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => 30,
        }
    }

    /// Check if leap year
    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    /// Get day of week (0=Sunday, 1=Monday, ..., 6=Saturday)
    pub fn weekday(&self) -> u8 {
        let mut y = self.year as i32;
        let mut m = self.month as i32;
        if m < 3 {
            m += 12;
            y -= 1;
        }
        let d = self.day as i32;
        let w = (d + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
        ((w + 6) % 7) as u8
    }

    /// Format date display
    pub fn format(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Format date with weekday
    pub fn format_with_weekday(&self) -> String {
        let weekday_names = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"];
        let weekday = self.weekday() as usize;
        format!("{}年{}月{}日 {}", self.year, self.month, self.day, weekday_names[weekday])
    }

    /// Get current season string (e.g., "2026-27")
    pub fn season_string(&self) -> String {
        if self.month >= 8 {
            format!("{}-{}", self.year, (self.year + 1) % 100)
        } else {
            format!("{}-{}", self.year - 1, self.year % 100)
        }
    }
}

/// UI Screen types
#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    TeamManagement,
    PlayerDetail { player_id: String },
    Tactics,
    TransferMarket,
    MatchModeSelection,
    MatchLive { match_id: String },
    MatchResult { match_id: String },
    LeagueTable,
    SeasonSummary { season: String },
    MatchHistory,
    FinanceReport,
    Notifications,
    SaveLoad,
    Settings,
}

/// Game difficulty settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

/// Notification structure
#[derive(Debug, Clone)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: NotificationPriority,
    pub read: bool,
    pub created_at: u64,
}

/// Notification types
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationType {
    Transfer,
    Injury,
    Contract,
    Match,
    Finance,
    PlayerMorale,
    Achievement,
    News,
    System,
}

/// Notification priority
#[derive(Debug, Clone, PartialEq)]
pub enum NotificationPriority {
    Urgent,
    High,
    Normal,
    Low,
}

impl GameState {
    /// Create a new game state
    pub fn new(player_team_id: String, league: League, teams: Vec<Team>, start_year: u16) -> Self {
        let mut teams_map = HashMap::new();
        for team in teams {
            teams_map.insert(team.id.clone(), team);
        }

        Self {
            game_id: Uuid::new_v4().to_string(),
            is_new_game: true,
            current_date: GameDate::new_season_start(start_year),
            player_team_id,
            player_name: None,
            league,
            teams: teams_map,
            current_screen: Screen::MainMenu,
            screen_stack: vec![],
            notifications: Vec::new(),
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        }
    }

    /// Get player's team
    pub fn get_player_team(&self) -> Option<&Team> {
        self.teams.get(&self.player_team_id)
    }

    /// Get any team by ID
    pub fn get_team(&self, team_id: &str) -> Option<&Team> {
        self.teams.get(team_id)
    }

    /// Navigate to screen
    pub fn navigate_to(&mut self, screen: Screen) {
        self.screen_stack.push(self.current_screen.clone());
        self.current_screen = screen;
    }

    /// Go back to previous screen
    pub fn go_back(&mut self) {
        if let Some(screen) = self.screen_stack.pop() {
            self.current_screen = screen;
        }
    }

    /// Advance time to next match day
    pub fn advance_time(&mut self) {
        self.current_date.advance_to_next_matchday();

        // Check season end
        if self.current_date.is_season_end() {
            self.on_season_end();
        }
    }

    /// Handle season end
    fn on_season_end(&mut self) {
        // Player aging will be handled in ai module
        // Trigger season summary event here
    }

    /// Add notification
    pub fn add_notification(&mut self, title: String, message: String, notification_type: NotificationType, priority: NotificationPriority) {
        let notification = Notification {
            id: Uuid::new_v4().to_string(),
            title,
            message,
            notification_type,
            priority,
            read: false,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.notifications.push(notification);
    }

    /// Get unread notifications count
    pub fn unread_notifications_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    /// Mark notification as read
    pub fn mark_notification_read(&mut self, id: &str) {
        if let Some(notification) = self.notifications.iter_mut().find(|n| n.id == id) {
            notification.read = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_date_creation() {
        let date = GameDate::new_season_start(2026);
        assert_eq!(date.year, 2026);
        assert_eq!(date.month, 8);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn test_advance_day() {
        let mut date = GameDate::new_season_start(2026);
        date.advance_day();
        assert_eq!(date.day, 2);
        assert_eq!(date.month, 8);
    }

    #[test]
    fn test_advance_month() {
        let mut date = GameDate { year: 2026, month: 1, day: 31 };
        date.advance_day();
        assert_eq!(date.day, 1);
        assert_eq!(date.month, 2);
    }

    #[test]
    fn test_advance_year() {
        let mut date = GameDate { year: 2026, month: 12, day: 31 };
        date.advance_day();
        assert_eq!(date.year, 2027);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1);
    }

    #[test]
    fn test_transfer_window() {
        let summer = GameDate { year: 2026, month: 7, day: 15 };
        assert!(summer.is_summer_transfer_window());
        assert!(summer.is_transfer_window());

        let winter = GameDate { year: 2026, month: 1, day: 15 };
        assert!(winter.is_winter_transfer_window());
        assert!(winter.is_transfer_window());

        let non_window = GameDate { year: 2026, month: 4, day: 15 };
        assert!(!non_window.is_transfer_window());
    }

    #[test]
    fn test_season_string() {
        let date1 = GameDate { year: 2026, month: 9, day: 1 };
        assert_eq!(date1.season_string(), "2026-27");

        let date2 = GameDate { year: 2027, month: 2, day: 1 };
        assert_eq!(date2.season_string(), "2026-27");
    }

    #[test]
    fn test_navigation() {
        let mut state = GameState {
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
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        };

        state.navigate_to(Screen::TeamManagement);
        assert_eq!(state.current_screen, Screen::TeamManagement);
        assert_eq!(state.screen_stack.len(), 1);

        state.go_back();
        assert_eq!(state.current_screen, Screen::MainMenu);
        assert_eq!(state.screen_stack.len(), 0);
    }

    #[test]
    fn test_notifications() {
        let mut state = GameState {
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
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        };

        state.add_notification(
            "Test".to_string(),
            "Test message".to_string(),
            NotificationType::System,
            NotificationPriority::Normal,
        );

        assert_eq!(state.unread_notifications_count(), 1);
    }
}
