/// Language support
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    /// Convert from language code
    pub fn from_code(code: &str) -> Self {
        match code {
            "zh" | "zh-CN" | "zh-TW" => Language::Chinese,
            "en" | "en-US" | "en-GB" => Language::English,
            _ => Language::English,
        }
    }

    /// Get language code
    pub fn code(&self) -> &str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }
}

/// Translation keys
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // Common
    Confirm,
    Cancel,
    Back,
    Quit,
    Save,
    Load,
    Delete,
    Edit,
    Search,

    // Main menu
    MainMenu,
    TeamManagement,
    Tactics,
    TransferMarket,
    NextMatch,
    LeagueTable,
    SaveLoad,
    Settings,
    ExitGame,

    // Team management
    Squad,
    Statistics,
    PlayerName,
    Position,
    Age,
    Ability,
    Status,
    PlayerList,

    // Match
    MatchModeSelection,
    LiveText,
    QuickSimulation,
    Pause,
    Resume,
    Score,
    Time,

    // Transfer market
    Browse,
    Buy,
    Sell,
    Price,
    Budget,
    AskingPrice,

    // League
    LeagueStandings,
    Team,
    Played,
    Won,
    Drawn,
    Lost,
    GoalsFor,
    GoalsAgainst,
    GoalDifference,
    Points,

    // Notifications
    Notifications,
    NoNotifications,
}

/// Get translation for key
pub fn t(key: TranslationKey, lang: Language) -> &'static str {
    match lang {
        Language::Chinese => zh(key),
        Language::English => en(key),
    }
}

/// English translations
fn en(key: TranslationKey) -> &'static str {
    match key {
        TranslationKey::Confirm => "Confirm",
        TranslationKey::Cancel => "Cancel",
        TranslationKey::Back => "Back",
        TranslationKey::Quit => "Quit",
        TranslationKey::Save => "Save",
        TranslationKey::Load => "Load",
        TranslationKey::Delete => "Delete",
        TranslationKey::Edit => "Edit",
        TranslationKey::Search => "Search",

        TranslationKey::MainMenu => "Main Menu",
        TranslationKey::TeamManagement => "Team Management",
        TranslationKey::Tactics => "Tactics",
        TranslationKey::TransferMarket => "Transfer Market",
        TranslationKey::NextMatch => "Next Match",
        TranslationKey::LeagueTable => "League Table",
        TranslationKey::SaveLoad => "Save/Load",
        TranslationKey::Settings => "Settings",
        TranslationKey::ExitGame => "Exit Game",

        TranslationKey::Squad => "Squad",
        TranslationKey::Statistics => "Statistics",
        TranslationKey::PlayerName => "Name",
        TranslationKey::Position => "Position",
        TranslationKey::Age => "Age",
        TranslationKey::Ability => "Ability",
        TranslationKey::Status => "Status",
        TranslationKey::PlayerList => "Player List",

        TranslationKey::MatchModeSelection => "Match Mode Selection",
        TranslationKey::LiveText => "Live Text",
        TranslationKey::QuickSimulation => "Quick Simulation",
        TranslationKey::Pause => "Pause",
        TranslationKey::Resume => "Resume",
        TranslationKey::Score => "Score",
        TranslationKey::Time => "Time",

        TranslationKey::Browse => "Browse",
        TranslationKey::Buy => "Buy",
        TranslationKey::Sell => "Sell",
        TranslationKey::Price => "Price",
        TranslationKey::Budget => "Budget",
        TranslationKey::AskingPrice => "Asking Price",

        TranslationKey::LeagueStandings => "League Standings",
        TranslationKey::Team => "Team",
        TranslationKey::Played => "P",
        TranslationKey::Won => "W",
        TranslationKey::Drawn => "D",
        TranslationKey::Lost => "L",
        TranslationKey::GoalsFor => "GF",
        TranslationKey::GoalsAgainst => "GA",
        TranslationKey::GoalDifference => "GD",
        TranslationKey::Points => "Pts",

        TranslationKey::Notifications => "Notifications",
        TranslationKey::NoNotifications => "No notifications",
    }
}

/// Chinese translations
fn zh(key: TranslationKey) -> &'static str {
    match key {
        TranslationKey::Confirm => "确认",
        TranslationKey::Cancel => "取消",
        TranslationKey::Back => "返回",
        TranslationKey::Quit => "退出",
        TranslationKey::Save => "保存",
        TranslationKey::Load => "加载",
        TranslationKey::Delete => "删除",
        TranslationKey::Edit => "编辑",
        TranslationKey::Search => "搜索",

        TranslationKey::MainMenu => "主菜单",
        TranslationKey::TeamManagement => "球队管理",
        TranslationKey::Tactics => "战术设置",
        TranslationKey::TransferMarket => "转会市场",
        TranslationKey::NextMatch => "下一场比赛",
        TranslationKey::LeagueTable => "联赛积分榜",
        TranslationKey::SaveLoad => "存档/读档",
        TranslationKey::Settings => "设置",
        TranslationKey::ExitGame => "退出游戏",

        TranslationKey::Squad => "阵容",
        TranslationKey::Statistics => "统计",
        TranslationKey::PlayerName => "姓名",
        TranslationKey::Position => "位置",
        TranslationKey::Age => "年龄",
        TranslationKey::Ability => "能力",
        TranslationKey::Status => "状态",
        TranslationKey::PlayerList => "球员列表",

        TranslationKey::MatchModeSelection => "比赛模式选择",
        TranslationKey::LiveText => "文字直播",
        TranslationKey::QuickSimulation => "快速模拟",
        TranslationKey::Pause => "暂停",
        TranslationKey::Resume => "继续",
        TranslationKey::Score => "比分",
        TranslationKey::Time => "时间",

        TranslationKey::Browse => "浏览",
        TranslationKey::Buy => "购买",
        TranslationKey::Sell => "出售",
        TranslationKey::Price => "价格",
        TranslationKey::Budget => "预算",
        TranslationKey::AskingPrice => "要价",

        TranslationKey::LeagueStandings => "联赛积分榜",
        TranslationKey::Team => "球队",
        TranslationKey::Played => "场",
        TranslationKey::Won => "胜",
        TranslationKey::Drawn => "平",
        TranslationKey::Lost => "负",
        TranslationKey::GoalsFor => "进",
        TranslationKey::GoalsAgainst => "失",
        TranslationKey::GoalDifference => "差",
        TranslationKey::Points => "分",

        TranslationKey::Notifications => "通知",
        TranslationKey::NoNotifications => "暂无通知",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("zh"), Language::Chinese);
        assert_eq!(Language::from_code("en"), Language::English);
        assert_eq!(Language::from_code("unknown"), Language::English);
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::Chinese.code(), "zh");
        assert_eq!(Language::English.code(), "en");
    }

    #[test]
    fn test_english_translations() {
        assert_eq!(t(TranslationKey::MainMenu, Language::English), "Main Menu");
        assert_eq!(t(TranslationKey::Confirm, Language::English), "Confirm");
    }

    #[test]
    fn test_chinese_translations() {
        assert_eq!(t(TranslationKey::MainMenu, Language::Chinese), "主菜单");
        assert_eq!(t(TranslationKey::Confirm, Language::Chinese), "确认");
    }
}
