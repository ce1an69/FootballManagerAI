# UI Module Design

## 概述

UI模块负责游戏的TUI（Terminal User Interface）界面，使用ratatui框架构建。

## 架构

### 文件结构

```
ui/
├── mod.rs              # 模块导出
├── app.rs              # 主应用结构
├── screens.rs          # 各个界面实现
├── components.rs       # 可复用UI组件
├── event.rs            # 输入事件处理
└── i18n.rs             # 国际化支持
```

## 1. 国际化支持 (i18n.rs)

### 1.1 语言枚举

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn from_code(code: &str) -> Self {
        match code {
            "zh" | "zh-CN" | "zh-TW" => Language::Chinese,
            "en" | "en-US" | "en-GB" => Language::English,
            _ => Language::English, // 默认英文
        }
    }

    pub fn code(&self) -> &str {
        match self {
            Language::Chinese => "zh",
            Language::English => "en",
        }
    }
}
```

### 1.2 翻译键定义

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TranslationKey {
    // 通用
    Confirm,
    Cancel,
    Back,
    Quit,
    Save,
    Load,
    Delete,
    Edit,
    Search,
    
    // 主菜单
    MainMenu,
    TeamManagement,
    Tactics,
    TransferMarket,
    NextMatch,
    LeagueTable,
    SaveLoad,
    ExitGame,
    
    // 球队管理
    Squad,
    Statistics,
    PlayerName,
    Position,
    Age,
    Ability,
    Status,
    PlayerList,
    
    // 比赛
    MatchModeSelection,
    LiveText,
    QuickSimulation,
    MatchEvents,
    Pause,
    Resume,
    Score,
    Time,
    
    // 转会市场
    Browse,
    SellPlayers,
    Buy,
    Sell,
    Price,
    Budget,
    
    // 联赛
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
    
    // 提示信息
    PressSpaceToPause,
    PressQToQuit,
    PressEscToBack,
    
    // 其他
    GameTitle,
    TeamInfo,
    
    // 日期相关
    CurrentDate,
    Season,
    TransferWindowOpen,
    TransferWindowClosed,
    SummerTransferWindow,
    WinterTransferWindow,
    
    // 星期
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    
    // 通知系统
    Notifications,
    UnreadNotifications,
    MarkAllRead,
    NoNotifications,
    NotificationTypeTransfer,
    NotificationTypeInjury,
    NotificationTypeContract,
    NotificationTypeMatch,
    NotificationTypeFinance,
    NotificationTypeNews,
    PriorityUrgent,
    PriorityHigh,
    PriorityNormal,
    PriorityLow,
    
    // 球员详情
    PlayerDetail,
    Overview,
    Attributes,
    Contract,
    TechnicalAttributes,
    MentalAttributes,
    PhysicalAttributes,
    GoalkeeperAttributes,
    Condition,
    Fatigue,
    Morale,
    MatchFitness,
    Wage,
    ContractYears,
    MarketValue,
    Potential,
    
    // 技术属性名
    AttrCorners,
    AttrCrossing,
    AttrDribbling,
    AttrFinishing,
    AttrHeading,
    AttrLongShots,
    AttrPassing,
    AttrTackling,
    AttrTechnique,
    
    // 精神属性名
    AttrAggression,
    AttrAnticipation,
    AttrBravery,
    AttrCreativity,
    AttrDecisions,
    AttrConcentration,
    AttrPositioning,
    AttrOffTheBall,
    AttrWorkRate,
    AttrTeamwork,
    AttrVision,
    
    // 身体属性名
    AttrAcceleration,
    AttrAgility,
    AttrBalance,
    AttrPace,
    AttrStamina,
    AttrStrength,
    
    // 换人建议
    SubstitutionSuggestions,
    ViewSuggestions,
    SubsRemaining,
    ReasonLowFitness,
    ReasonHighFatigue,
    ReasonInjured,
    ReasonTacticalAttack,
    ReasonTacticalDefense,
    
    // 设置界面
    Settings,
    LanguageSettings,
    MatchSettings,
    SelectLanguage,
    Chinese,
    English,
    DefaultMatchMode,
    AutoSave,
    AutoSaveOn,
    AutoSaveOff,
    SettingsSaved,
    RestoreDefaults,
    
    // 筛选功能
    Filter,
    FilterByPosition,
    FilterByAge,
    FilterByAbility,
    SortBy,
    SortByName,
    SortByAge,
    SortByAbility,
    SortByPosition,
    AgeRange,
    AbilityRange,
    AllPositions,
    ClearFilter,
    ResultsCount,
    
    // 赛季总结
    SeasonSummary,
    FinalStandings,
    TopScorers,
    TopAssists,
    BestPlayers,
    SeasonReview,
    YourPerformance,
    LeaguePosition,
    TotalGoals,
    TotalAssists,
    CleanSheets,
    
    // 比赛历史
    MatchHistory,
    RecentMatches,
    AllMatches,
    ViewDetails,
    
    // 财务报告
    FinanceReport,
    Income,
    Expenses,
    TransferIncome,
    TransferExpenses,
    WageBill,
    MatchdayIncome,
    Sponsorship,
    NetBalance,
    SeasonFinance,
}
```

### 1.3 翻译管理器

```rust
use std::collections::HashMap;

pub struct I18n {
    current_language: Language,
    translations: HashMap<Language, HashMap<TranslationKey, String>>,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        let mut i18n = Self {
            current_language: language,
            translations: HashMap::new(),
        };
        i18n.init_translations();
        i18n
    }

    fn init_translations(&mut self) {
        // 中文翻译
        let mut zh_translations = HashMap::new();
        zh_translations.insert(TranslationKey::MainMenu, "主菜单".to_string());
        zh_translations.insert(TranslationKey::TeamManagement, "球队管理".to_string());
        zh_translations.insert(TranslationKey::Tactics, "战术设置".to_string());
        zh_translations.insert(TranslationKey::TransferMarket, "转会市场".to_string());
        zh_translations.insert(TranslationKey::NextMatch, "下一场比赛".to_string());
        zh_translations.insert(TranslationKey::LeagueTable, "联赛积分榜".to_string());
        zh_translations.insert(TranslationKey::SaveLoad, "存档/读取".to_string());
        zh_translations.insert(TranslationKey::ExitGame, "退出游戏".to_string());
        zh_translations.insert(TranslationKey::Squad, "阵容".to_string());
        zh_translations.insert(TranslationKey::Statistics, "统计".to_string());
        zh_translations.insert(TranslationKey::PlayerName, "姓名".to_string());
        zh_translations.insert(TranslationKey::Position, "位置".to_string());
        zh_translations.insert(TranslationKey::Age, "年龄".to_string());
        zh_translations.insert(TranslationKey::Ability, "能力".to_string());
        zh_translations.insert(TranslationKey::Status, "状态".to_string());
        zh_translations.insert(TranslationKey::PlayerList, "球员名单".to_string());
        zh_translations.insert(TranslationKey::MatchModeSelection, "选择比赛模式".to_string());
        zh_translations.insert(TranslationKey::LiveText, "文本直播".to_string());
        zh_translations.insert(TranslationKey::QuickSimulation, "快速模拟".to_string());
        zh_translations.insert(TranslationKey::MatchEvents, "比赛事件".to_string());
        zh_translations.insert(TranslationKey::Pause, "暂停".to_string());
        zh_translations.insert(TranslationKey::Resume, "继续".to_string());
        zh_translations.insert(TranslationKey::Score, "比分".to_string());
        zh_translations.insert(TranslationKey::Time, "时间".to_string());
        zh_translations.insert(TranslationKey::Browse, "浏览".to_string());
        zh_translations.insert(TranslationKey::SellPlayers, "出售球员".to_string());
        zh_translations.insert(TranslationKey::Buy, "购买".to_string());
        zh_translations.insert(TranslationKey::Sell, "出售".to_string());
        zh_translations.insert(TranslationKey::Price, "价格".to_string());
        zh_translations.insert(TranslationKey::Budget, "预算".to_string());
        zh_translations.insert(TranslationKey::LeagueStandings, "联赛积分榜".to_string());
        zh_translations.insert(TranslationKey::Team, "球队".to_string());
        zh_translations.insert(TranslationKey::Played, "场".to_string());
        zh_translations.insert(TranslationKey::Won, "胜".to_string());
        zh_translations.insert(TranslationKey::Drawn, "平".to_string());
        zh_translations.insert(TranslationKey::Lost, "负".to_string());
        zh_translations.insert(TranslationKey::GoalsFor, "进".to_string());
        zh_translations.insert(TranslationKey::GoalsAgainst, "失".to_string());
        zh_translations.insert(TranslationKey::GoalDifference, "净".to_string());
        zh_translations.insert(TranslationKey::Points, "分".to_string());
        zh_translations.insert(TranslationKey::PressSpaceToPause, "按 [空格] 暂停".to_string());
        zh_translations.insert(TranslationKey::PressQToQuit, "按 [q] 退出".to_string());
        zh_translations.insert(TranslationKey::PressEscToBack, "按 [Esc] 返回".to_string());
        zh_translations.insert(TranslationKey::GameTitle, "Football Manager AI".to_string());
        zh_translations.insert(TranslationKey::TeamInfo, "球队".to_string());
        zh_translations.insert(TranslationKey::Confirm, "确认".to_string());
        zh_translations.insert(TranslationKey::Cancel, "取消".to_string());
        zh_translations.insert(TranslationKey::Back, "返回".to_string());
        zh_translations.insert(TranslationKey::Quit, "退出".to_string());
        zh_translations.insert(TranslationKey::Save, "保存".to_string());
        zh_translations.insert(TranslationKey::Load, "读取".to_string());
        // 日期相关
        zh_translations.insert(TranslationKey::CurrentDate, "当前日期".to_string());
        zh_translations.insert(TranslationKey::Season, "赛季".to_string());
        zh_translations.insert(TranslationKey::TransferWindowOpen, "转会窗口开放".to_string());
        zh_translations.insert(TranslationKey::TransferWindowClosed, "转会窗口关闭".to_string());
        zh_translations.insert(TranslationKey::SummerTransferWindow, "夏季转会窗口".to_string());
        zh_translations.insert(TranslationKey::WinterTransferWindow, "冬季转会窗口".to_string());
        zh_translations.insert(TranslationKey::Sunday, "周日".to_string());
        zh_translations.insert(TranslationKey::Monday, "周一".to_string());
        zh_translations.insert(TranslationKey::Tuesday, "周二".to_string());
        zh_translations.insert(TranslationKey::Wednesday, "周三".to_string());
        zh_translations.insert(TranslationKey::Thursday, "周四".to_string());
        zh_translations.insert(TranslationKey::Friday, "周五".to_string());
        zh_translations.insert(TranslationKey::Saturday, "周六".to_string());
        // 通知系统
        zh_translations.insert(TranslationKey::Notifications, "通知".to_string());
        zh_translations.insert(TranslationKey::UnreadNotifications, "未读通知".to_string());
        zh_translations.insert(TranslationKey::MarkAllRead, "全部标为已读".to_string());
        zh_translations.insert(TranslationKey::NoNotifications, "暂无通知".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeTransfer, "转会".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeInjury, "伤病".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeContract, "合同".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeMatch, "比赛".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeFinance, "财务".to_string());
        zh_translations.insert(TranslationKey::NotificationTypeNews, "新闻".to_string());
        zh_translations.insert(TranslationKey::PriorityUrgent, "紧急".to_string());
        zh_translations.insert(TranslationKey::PriorityHigh, "重要".to_string());
        zh_translations.insert(TranslationKey::PriorityNormal, "普通".to_string());
        zh_translations.insert(TranslationKey::PriorityLow, "次要".to_string());
        // 球员详情
        zh_translations.insert(TranslationKey::PlayerDetail, "球员详情".to_string());
        zh_translations.insert(TranslationKey::Overview, "概览".to_string());
        zh_translations.insert(TranslationKey::Attributes, "属性".to_string());
        zh_translations.insert(TranslationKey::Contract, "合同".to_string());
        zh_translations.insert(TranslationKey::TechnicalAttributes, "技术属性".to_string());
        zh_translations.insert(TranslationKey::MentalAttributes, "精神属性".to_string());
        zh_translations.insert(TranslationKey::PhysicalAttributes, "身体属性".to_string());
        zh_translations.insert(TranslationKey::GoalkeeperAttributes, "门将属性".to_string());
        zh_translations.insert(TranslationKey::Condition, "状态".to_string());
        zh_translations.insert(TranslationKey::Fatigue, "疲劳".to_string());
        zh_translations.insert(TranslationKey::Morale, "士气".to_string());
        zh_translations.insert(TranslationKey::MatchFitness, "比赛体能".to_string());
        zh_translations.insert(TranslationKey::Wage, "周薪".to_string());
        zh_translations.insert(TranslationKey::ContractYears, "合同剩余".to_string());
        zh_translations.insert(TranslationKey::MarketValue, "市场价值".to_string());
        zh_translations.insert(TranslationKey::Potential, "潜力".to_string());
        // 技术属性
        zh_translations.insert(TranslationKey::AttrCorners, "角球".to_string());
        zh_translations.insert(TranslationKey::AttrCrossing, "传中".to_string());
        zh_translations.insert(TranslationKey::AttrDribbling, "盘带".to_string());
        zh_translations.insert(TranslationKey::AttrFinishing, "射门".to_string());
        zh_translations.insert(TranslationKey::AttrHeading, "头球".to_string());
        zh_translations.insert(TranslationKey::AttrLongShots, "远射".to_string());
        zh_translations.insert(TranslationKey::AttrPassing, "传球".to_string());
        zh_translations.insert(TranslationKey::AttrTackling, "抢断".to_string());
        zh_translations.insert(TranslationKey::AttrTechnique, "技术".to_string());
        // 精神属性
        zh_translations.insert(TranslationKey::AttrAggression, "侵略性".to_string());
        zh_translations.insert(TranslationKey::AttrAnticipation, "预判".to_string());
        zh_translations.insert(TranslationKey::AttrBravery, "勇敢".to_string());
        zh_translations.insert(TranslationKey::AttrCreativity, "创造力".to_string());
        zh_translations.insert(TranslationKey::AttrDecisions, "决断".to_string());
        zh_translations.insert(TranslationKey::AttrConcentration, "专注".to_string());
        zh_translations.insert(TranslationKey::AttrPositioning, "位置感".to_string());
        zh_translations.insert(TranslationKey::AttrOffTheBall, "无球跑动".to_string());
        zh_translations.insert(TranslationKey::AttrWorkRate, "工作投入".to_string());
        zh_translations.insert(TranslationKey::AttrTeamwork, "团队合作".to_string());
        zh_translations.insert(TranslationKey::AttrVision, "视野".to_string());
        // 身体属性
        zh_translations.insert(TranslationKey::AttrAcceleration, "加速".to_string());
        zh_translations.insert(TranslationKey::AttrAgility, "灵活".to_string());
        zh_translations.insert(TranslationKey::AttrBalance, "平衡".to_string());
        zh_translations.insert(TranslationKey::AttrPace, "速度".to_string());
        zh_translations.insert(TranslationKey::AttrStamina, "耐力".to_string());
        zh_translations.insert(TranslationKey::AttrStrength, "力量".to_string());
        // 换人建议
        zh_translations.insert(TranslationKey::SubstitutionSuggestions, "换人建议".to_string());
        zh_translations.insert(TranslationKey::ViewSuggestions, "查看建议".to_string());
        zh_translations.insert(TranslationKey::SubsRemaining, "换人剩余".to_string());
        zh_translations.insert(TranslationKey::ReasonLowFitness, "体能过低".to_string());
        zh_translations.insert(TranslationKey::ReasonHighFatigue, "疲劳过高".to_string());
        zh_translations.insert(TranslationKey::ReasonInjured, "球员受伤".to_string());
        zh_translations.insert(TranslationKey::ReasonTacticalAttack, "加强进攻".to_string());
        zh_translations.insert(TranslationKey::ReasonTacticalDefense, "加强防守".to_string());
        // 设置界面
        zh_translations.insert(TranslationKey::Settings, "设置".to_string());
        zh_translations.insert(TranslationKey::LanguageSettings, "语言设置".to_string());
        zh_translations.insert(TranslationKey::MatchSettings, "比赛设置".to_string());
        zh_translations.insert(TranslationKey::SelectLanguage, "选择语言".to_string());
        zh_translations.insert(TranslationKey::Chinese, "中文".to_string());
        zh_translations.insert(TranslationKey::English, "English".to_string());
        zh_translations.insert(TranslationKey::DefaultMatchMode, "默认比赛模式".to_string());
        zh_translations.insert(TranslationKey::AutoSave, "自动保存".to_string());
        zh_translations.insert(TranslationKey::AutoSaveOn, "开启".to_string());
        zh_translations.insert(TranslationKey::AutoSaveOff, "关闭".to_string());
        zh_translations.insert(TranslationKey::SettingsSaved, "设置已保存".to_string());
        zh_translations.insert(TranslationKey::RestoreDefaults, "恢复默认".to_string());
        // 筛选功能
        zh_translations.insert(TranslationKey::Filter, "筛选".to_string());
        zh_translations.insert(TranslationKey::FilterByPosition, "按位置".to_string());
        zh_translations.insert(TranslationKey::FilterByAge, "按年龄".to_string());
        zh_translations.insert(TranslationKey::FilterByAbility, "按能力".to_string());
        zh_translations.insert(TranslationKey::SortBy, "排序".to_string());
        zh_translations.insert(TranslationKey::SortByName, "按姓名".to_string());
        zh_translations.insert(TranslationKey::SortByAge, "按年龄".to_string());
        zh_translations.insert(TranslationKey::SortByAbility, "按能力".to_string());
        zh_translations.insert(TranslationKey::SortByPosition, "按位置".to_string());
        zh_translations.insert(TranslationKey::AgeRange, "年龄范围".to_string());
        zh_translations.insert(TranslationKey::AbilityRange, "能力范围".to_string());
        zh_translations.insert(TranslationKey::AllPositions, "全部位置".to_string());
        zh_translations.insert(TranslationKey::ClearFilter, "清除筛选".to_string());
        zh_translations.insert(TranslationKey::ResultsCount, "结果数量".to_string());
        // 赛季总结
        zh_translations.insert(TranslationKey::SeasonSummary, "赛季总结".to_string());
        zh_translations.insert(TranslationKey::FinalStandings, "最终排名".to_string());
        zh_translations.insert(TranslationKey::TopScorers, "最佳射手".to_string());
        zh_translations.insert(TranslationKey::TopAssists, "助攻王".to_string());
        zh_translations.insert(TranslationKey::BestPlayers, "最佳球员".to_string());
        zh_translations.insert(TranslationKey::SeasonReview, "赛季回顾".to_string());
        zh_translations.insert(TranslationKey::YourPerformance, "你的表现".to_string());
        zh_translations.insert(TranslationKey::LeaguePosition, "联赛排名".to_string());
        zh_translations.insert(TranslationKey::TotalGoals, "总进球".to_string());
        zh_translations.insert(TranslationKey::TotalAssists, "总助攻".to_string());
        zh_translations.insert(TranslationKey::CleanSheets, "零封场次".to_string());
        // 比赛历史
        zh_translations.insert(TranslationKey::MatchHistory, "比赛历史".to_string());
        zh_translations.insert(TranslationKey::RecentMatches, "最近比赛".to_string());
        zh_translations.insert(TranslationKey::AllMatches, "全部比赛".to_string());
        zh_translations.insert(TranslationKey::ViewDetails, "查看详情".to_string());
        // 财务报告
        zh_translations.insert(TranslationKey::FinanceReport, "财务报告".to_string());
        zh_translations.insert(TranslationKey::Income, "收入".to_string());
        zh_translations.insert(TranslationKey::Expenses, "支出".to_string());
        zh_translations.insert(TranslationKey::TransferIncome, "转会收入".to_string());
        zh_translations.insert(TranslationKey::TransferExpenses, "转会支出".to_string());
        zh_translations.insert(TranslationKey::WageBill, "薪资支出".to_string());
        zh_translations.insert(TranslationKey::MatchdayIncome, "比赛日收入".to_string());
        zh_translations.insert(TranslationKey::Sponsorship, "赞助收入".to_string());
        zh_translations.insert(TranslationKey::NetBalance, "净余额".to_string());
        zh_translations.insert(TranslationKey::SeasonFinance, "赛季财务".to_string());
        
        // 英文翻译
        let mut en_translations = HashMap::new();
        en_translations.insert(TranslationKey::MainMenu, "Main Menu".to_string());
        en_translations.insert(TranslationKey::TeamManagement, "Team Management".to_string());
        en_translations.insert(TranslationKey::Tactics, "Tactics".to_string());
        en_translations.insert(TranslationKey::TransferMarket, "Transfer Market".to_string());
        en_translations.insert(TranslationKey::NextMatch, "Next Match".to_string());
        en_translations.insert(TranslationKey::LeagueTable, "League Table".to_string());
        en_translations.insert(TranslationKey::SaveLoad, "Save/Load".to_string());
        en_translations.insert(TranslationKey::ExitGame, "Exit Game".to_string());
        en_translations.insert(TranslationKey::Squad, "Squad".to_string());
        en_translations.insert(TranslationKey::Statistics, "Statistics".to_string());
        en_translations.insert(TranslationKey::PlayerName, "Name".to_string());
        en_translations.insert(TranslationKey::Position, "Position".to_string());
        en_translations.insert(TranslationKey::Age, "Age".to_string());
        en_translations.insert(TranslationKey::Ability, "Ability".to_string());
        en_translations.insert(TranslationKey::Status, "Status".to_string());
        en_translations.insert(TranslationKey::PlayerList, "Player List".to_string());
        en_translations.insert(TranslationKey::MatchModeSelection, "Select Match Mode".to_string());
        en_translations.insert(TranslationKey::LiveText, "Live Text".to_string());
        en_translations.insert(TranslationKey::QuickSimulation, "Quick Simulation".to_string());
        en_translations.insert(TranslationKey::MatchEvents, "Match Events".to_string());
        en_translations.insert(TranslationKey::Pause, "Pause".to_string());
        en_translations.insert(TranslationKey::Resume, "Resume".to_string());
        en_translations.insert(TranslationKey::Score, "Score".to_string());
        en_translations.insert(TranslationKey::Time, "Time".to_string());
        en_translations.insert(TranslationKey::Browse, "Browse".to_string());
        en_translations.insert(TranslationKey::SellPlayers, "Sell Players".to_string());
        en_translations.insert(TranslationKey::Buy, "Buy".to_string());
        en_translations.insert(TranslationKey::Sell, "Sell".to_string());
        en_translations.insert(TranslationKey::Price, "Price".to_string());
        en_translations.insert(TranslationKey::Budget, "Budget".to_string());
        en_translations.insert(TranslationKey::LeagueStandings, "League Standings".to_string());
        en_translations.insert(TranslationKey::Team, "Team".to_string());
        en_translations.insert(TranslationKey::Played, "P".to_string());
        en_translations.insert(TranslationKey::Won, "W".to_string());
        en_translations.insert(TranslationKey::Drawn, "D".to_string());
        en_translations.insert(TranslationKey::Lost, "L".to_string());
        en_translations.insert(TranslationKey::GoalsFor, "GF".to_string());
        en_translations.insert(TranslationKey::GoalsAgainst, "GA".to_string());
        en_translations.insert(TranslationKey::GoalDifference, "GD".to_string());
        en_translations.insert(TranslationKey::Points, "Pts".to_string());
        en_translations.insert(TranslationKey::PressSpaceToPause, "Press [Space] to pause".to_string());
        en_translations.insert(TranslationKey::PressQToQuit, "Press [q] to quit".to_string());
        en_translations.insert(TranslationKey::PressEscToBack, "Press [Esc] to go back".to_string());
        en_translations.insert(TranslationKey::GameTitle, "Football Manager AI".to_string());
        en_translations.insert(TranslationKey::TeamInfo, "Team".to_string());
        en_translations.insert(TranslationKey::Confirm, "Confirm".to_string());
        en_translations.insert(TranslationKey::Cancel, "Cancel".to_string());
        en_translations.insert(TranslationKey::Back, "Back".to_string());
        en_translations.insert(TranslationKey::Quit, "Quit".to_string());
        en_translations.insert(TranslationKey::Save, "Save".to_string());
        en_translations.insert(TranslationKey::Load, "Load".to_string());
        // 日期相关
        en_translations.insert(TranslationKey::CurrentDate, "Current Date".to_string());
        en_translations.insert(TranslationKey::Season, "Season".to_string());
        en_translations.insert(TranslationKey::TransferWindowOpen, "Transfer Window Open".to_string());
        en_translations.insert(TranslationKey::TransferWindowClosed, "Transfer Window Closed".to_string());
        en_translations.insert(TranslationKey::SummerTransferWindow, "Summer Transfer Window".to_string());
        en_translations.insert(TranslationKey::WinterTransferWindow, "Winter Transfer Window".to_string());
        en_translations.insert(TranslationKey::Sunday, "Sun".to_string());
        en_translations.insert(TranslationKey::Monday, "Mon".to_string());
        en_translations.insert(TranslationKey::Tuesday, "Tue".to_string());
        en_translations.insert(TranslationKey::Wednesday, "Wed".to_string());
        en_translations.insert(TranslationKey::Thursday, "Thu".to_string());
        en_translations.insert(TranslationKey::Friday, "Fri".to_string());
        en_translations.insert(TranslationKey::Saturday, "Sat".to_string());
        // 通知系统
        en_translations.insert(TranslationKey::Notifications, "Notifications".to_string());
        en_translations.insert(TranslationKey::UnreadNotifications, "Unread".to_string());
        en_translations.insert(TranslationKey::MarkAllRead, "Mark All Read".to_string());
        en_translations.insert(TranslationKey::NoNotifications, "No notifications".to_string());
        en_translations.insert(TranslationKey::NotificationTypeTransfer, "Transfer".to_string());
        en_translations.insert(TranslationKey::NotificationTypeInjury, "Injury".to_string());
        en_translations.insert(TranslationKey::NotificationTypeContract, "Contract".to_string());
        en_translations.insert(TranslationKey::NotificationTypeMatch, "Match".to_string());
        en_translations.insert(TranslationKey::NotificationTypeFinance, "Finance".to_string());
        en_translations.insert(TranslationKey::NotificationTypeNews, "News".to_string());
        en_translations.insert(TranslationKey::PriorityUrgent, "Urgent".to_string());
        en_translations.insert(TranslationKey::PriorityHigh, "High".to_string());
        en_translations.insert(TranslationKey::PriorityNormal, "Normal".to_string());
        en_translations.insert(TranslationKey::PriorityLow, "Low".to_string());
        // 球员详情
        en_translations.insert(TranslationKey::PlayerDetail, "Player Detail".to_string());
        en_translations.insert(TranslationKey::Overview, "Overview".to_string());
        en_translations.insert(TranslationKey::Attributes, "Attributes".to_string());
        en_translations.insert(TranslationKey::Contract, "Contract".to_string());
        en_translations.insert(TranslationKey::TechnicalAttributes, "Technical".to_string());
        en_translations.insert(TranslationKey::MentalAttributes, "Mental".to_string());
        en_translations.insert(TranslationKey::PhysicalAttributes, "Physical".to_string());
        en_translations.insert(TranslationKey::GoalkeeperAttributes, "Goalkeeper".to_string());
        en_translations.insert(TranslationKey::Condition, "Condition".to_string());
        en_translations.insert(TranslationKey::Fatigue, "Fatigue".to_string());
        en_translations.insert(TranslationKey::Morale, "Morale".to_string());
        en_translations.insert(TranslationKey::MatchFitness, "Match Fitness".to_string());
        en_translations.insert(TranslationKey::Wage, "Wage".to_string());
        en_translations.insert(TranslationKey::ContractYears, "Contract Left".to_string());
        en_translations.insert(TranslationKey::MarketValue, "Market Value".to_string());
        en_translations.insert(TranslationKey::Potential, "Potential".to_string());
        // 技术属性
        en_translations.insert(TranslationKey::AttrCorners, "Corners".to_string());
        en_translations.insert(TranslationKey::AttrCrossing, "Crossing".to_string());
        en_translations.insert(TranslationKey::AttrDribbling, "Dribbling".to_string());
        en_translations.insert(TranslationKey::AttrFinishing, "Finishing".to_string());
        en_translations.insert(TranslationKey::AttrHeading, "Heading".to_string());
        en_translations.insert(TranslationKey::AttrLongShots, "Long Shots".to_string());
        en_translations.insert(TranslationKey::AttrPassing, "Passing".to_string());
        en_translations.insert(TranslationKey::AttrTackling, "Tackling".to_string());
        en_translations.insert(TranslationKey::AttrTechnique, "Technique".to_string());
        // 精神属性
        en_translations.insert(TranslationKey::AttrAggression, "Aggression".to_string());
        en_translations.insert(TranslationKey::AttrAnticipation, "Anticipation".to_string());
        en_translations.insert(TranslationKey::AttrBravery, "Bravery".to_string());
        en_translations.insert(TranslationKey::AttrCreativity, "Creativity".to_string());
        en_translations.insert(TranslationKey::AttrDecisions, "Decisions".to_string());
        en_translations.insert(TranslationKey::AttrConcentration, "Concentration".to_string());
        en_translations.insert(TranslationKey::AttrPositioning, "Positioning".to_string());
        en_translations.insert(TranslationKey::AttrOffTheBall, "Off The Ball".to_string());
        en_translations.insert(TranslationKey::AttrWorkRate, "Work Rate".to_string());
        en_translations.insert(TranslationKey::AttrTeamwork, "Teamwork".to_string());
        en_translations.insert(TranslationKey::AttrVision, "Vision".to_string());
        // 身体属性
        en_translations.insert(TranslationKey::AttrAcceleration, "Acceleration".to_string());
        en_translations.insert(TranslationKey::AttrAgility, "Agility".to_string());
        en_translations.insert(TranslationKey::AttrBalance, "Balance".to_string());
        en_translations.insert(TranslationKey::AttrPace, "Pace".to_string());
        en_translations.insert(TranslationKey::AttrStamina, "Stamina".to_string());
        en_translations.insert(TranslationKey::AttrStrength, "Strength".to_string());
        // 换人建议
        en_translations.insert(TranslationKey::SubstitutionSuggestions, "Substitution Suggestions".to_string());
        en_translations.insert(TranslationKey::ViewSuggestions, "View Suggestions".to_string());
        en_translations.insert(TranslationKey::SubsRemaining, "Subs Remaining".to_string());
        en_translations.insert(TranslationKey::ReasonLowFitness, "Low Fitness".to_string());
        en_translations.insert(TranslationKey::ReasonHighFatigue, "High Fatigue".to_string());
        en_translations.insert(TranslationKey::ReasonInjured, "Player Injured".to_string());
        en_translations.insert(TranslationKey::ReasonTacticalAttack, "More Attacking".to_string());
        en_translations.insert(TranslationKey::ReasonTacticalDefense, "More Defensive".to_string());
        // 设置界面
        en_translations.insert(TranslationKey::Settings, "Settings".to_string());
        en_translations.insert(TranslationKey::LanguageSettings, "Language".to_string());
        en_translations.insert(TranslationKey::MatchSettings, "Match Settings".to_string());
        en_translations.insert(TranslationKey::SelectLanguage, "Select Language".to_string());
        en_translations.insert(TranslationKey::Chinese, "中文".to_string());
        en_translations.insert(TranslationKey::English, "English".to_string());
        en_translations.insert(TranslationKey::DefaultMatchMode, "Default Match Mode".to_string());
        en_translations.insert(TranslationKey::AutoSave, "Auto Save".to_string());
        en_translations.insert(TranslationKey::AutoSaveOn, "On".to_string());
        en_translations.insert(TranslationKey::AutoSaveOff, "Off".to_string());
        en_translations.insert(TranslationKey::SettingsSaved, "Settings Saved".to_string());
        en_translations.insert(TranslationKey::RestoreDefaults, "Restore Defaults".to_string());
        // 筛选功能
        en_translations.insert(TranslationKey::Filter, "Filter".to_string());
        en_translations.insert(TranslationKey::FilterByPosition, "By Position".to_string());
        en_translations.insert(TranslationKey::FilterByAge, "By Age".to_string());
        en_translations.insert(TranslationKey::FilterByAbility, "By Ability".to_string());
        en_translations.insert(TranslationKey::SortBy, "Sort".to_string());
        en_translations.insert(TranslationKey::SortByName, "By Name".to_string());
        en_translations.insert(TranslationKey::SortByAge, "By Age".to_string());
        en_translations.insert(TranslationKey::SortByAbility, "By Ability".to_string());
        en_translations.insert(TranslationKey::SortByPosition, "By Position".to_string());
        en_translations.insert(TranslationKey::AgeRange, "Age Range".to_string());
        en_translations.insert(TranslationKey::AbilityRange, "Ability Range".to_string());
        en_translations.insert(TranslationKey::AllPositions, "All Positions".to_string());
        en_translations.insert(TranslationKey::ClearFilter, "Clear Filter".to_string());
        en_translations.insert(TranslationKey::ResultsCount, "Results".to_string());
        // 赛季总结
        en_translations.insert(TranslationKey::SeasonSummary, "Season Summary".to_string());
        en_translations.insert(TranslationKey::FinalStandings, "Final Standings".to_string());
        en_translations.insert(TranslationKey::TopScorers, "Top Scorers".to_string());
        en_translations.insert(TranslationKey::TopAssists, "Top Assists".to_string());
        en_translations.insert(TranslationKey::BestPlayers, "Best Players".to_string());
        en_translations.insert(TranslationKey::SeasonReview, "Season Review".to_string());
        en_translations.insert(TranslationKey::YourPerformance, "Your Performance".to_string());
        en_translations.insert(TranslationKey::LeaguePosition, "League Position".to_string());
        en_translations.insert(TranslationKey::TotalGoals, "Total Goals".to_string());
        en_translations.insert(TranslationKey::TotalAssists, "Total Assists".to_string());
        en_translations.insert(TranslationKey::CleanSheets, "Clean Sheets".to_string());
        // 比赛历史
        en_translations.insert(TranslationKey::MatchHistory, "Match History".to_string());
        en_translations.insert(TranslationKey::RecentMatches, "Recent Matches".to_string());
        en_translations.insert(TranslationKey::AllMatches, "All Matches".to_string());
        en_translations.insert(TranslationKey::ViewDetails, "View Details".to_string());
        // 财务报告
        en_translations.insert(TranslationKey::FinanceReport, "Finance Report".to_string());
        en_translations.insert(TranslationKey::Income, "Income".to_string());
        en_translations.insert(TranslationKey::Expenses, "Expenses".to_string());
        en_translations.insert(TranslationKey::TransferIncome, "Transfer Income".to_string());
        en_translations.insert(TranslationKey::TransferExpenses, "Transfer Expenses".to_string());
        en_translations.insert(TranslationKey::WageBill, "Wage Bill".to_string());
        en_translations.insert(TranslationKey::MatchdayIncome, "Matchday Income".to_string());
        en_translations.insert(TranslationKey::Sponsorship, "Sponsorship".to_string());
        en_translations.insert(TranslationKey::NetBalance, "Net Balance".to_string());
        en_translations.insert(TranslationKey::SeasonFinance, "Season Finance".to_string());
        
        self.translations.insert(Language::Chinese, zh_translations);
        self.translations.insert(Language::English, en_translations);
    }

    pub fn t(&self, key: TranslationKey) -> &str {
        self.translations
            .get(&self.current_language)
            .and_then(|lang_map| lang_map.get(&key))
            .map(|s| s.as_str())
            .unwrap_or_else(|| {
                // 如果当前语言没有翻译，尝试使用英文作为后备
                self.translations
                    .get(&Language::English)
                    .and_then(|lang_map| lang_map.get(&key))
                    .map(|s| s.as_str())
                    .unwrap_or("[MISSING]")
            })
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> Language {
        self.current_language
    }
}

// 便捷宏，用于简化翻译调用
#[macro_export]
macro_rules! t {
    ($i18n:expr, $key:expr) => {
        $i18n.t($key)
    };
}
```

### 1.4 格式化翻译（支持参数）

```rust
impl I18n {
    // 支持格式化字符串，例如：format!("球队: {} | 预算: ${}", team_name, budget)
    pub fn t_format(&self, key: TranslationKey, args: &[&str]) -> String {
        let template = self.t(key);
        // 简单的占位符替换，例如："{0} | Budget: ${1}"
        let mut result = template.to_string();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }
}
```

## 2. 主应用 (app.rs)

### 2.1 App 结构

```rust
use ratatui::{Frame, Terminal};
use crossterm::event::{KeyEvent, MouseEvent};
use crate::game::GameState;
use crate::ui::i18n::{I18n, Language};

pub struct App {
    pub state: GameState,
    pub should_quit: bool,
    pub current_screen: Box<dyn Screen>,
    pub i18n: I18n,  // 国际化管理器
}

impl App {
    pub fn new(state: GameState, language: Option<Language>) -> Self {
        let lang = language.unwrap_or_else(|| {
            // 可以从环境变量或配置文件读取默认语言
            std::env::var("LANG")
                .ok()
                .and_then(|l| l.split('.').next())
                .map(|l| Language::from_code(l))
                .unwrap_or(Language::English)
        });
        
        let current_screen = Box::new(MainMenuScreen::new());
        Self {
            state,
            should_quit: false,
            current_screen,
            i18n: I18n::new(lang),
        }
    }

    pub fn run(&mut self, terminal: &mut Terminal<impl Backend>) -> Result<(), UiError> {
        while !self.should_quit {
            terminal.draw(|f| self.render(f))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render(&self, f: &mut Frame) {
        self.current_screen.render(f, &self.state, &self.i18n);
    }

    fn handle_events(&mut self) -> Result<(), UiError> {
        // 处理键盘事件
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => self.should_quit = true,
                KeyCode::Esc => self.navigate_back(),
                _ => self.current_screen.handle_key(key, &mut self.state)?,
            }
        }
        Ok(())
    }

    fn navigate_back(&mut self) {
        // 实现返回逻辑
    }
}
```

## 2. 界面 (screens.rs)

### 2.1 Screen Trait

```rust
use ratatui::Frame;

pub trait Screen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n);
    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError>;
    fn on_enter(&mut self, state: &mut GameState) {}
    fn on_exit(&mut self, state: &mut GameState) {}
}
```

### 2.2 主菜单界面

```rust
pub struct MainMenuScreen {
    selected_index: usize,
    menu_items: Vec<MenuItem>,
}

struct MenuItem {
    translation_key: TranslationKey,
    action: MenuAction,
}

enum MenuAction {
    NavigateTo(ScreenType),
    Quit,
}

impl MainMenuScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            menu_items: vec![
                MenuItem {
                    translation_key: TranslationKey::TeamManagement,
                    action: MenuAction::NavigateTo(ScreenType::TeamManagement),
                },
                MenuItem {
                    translation_key: TranslationKey::Tactics,
                    action: MenuAction::NavigateTo(ScreenType::Tactics),
                },
                MenuItem {
                    translation_key: TranslationKey::TransferMarket,
                    action: MenuAction::NavigateTo(ScreenType::TransferMarket),
                },
                MenuItem {
                    translation_key: TranslationKey::NextMatch,
                    action: MenuAction::NavigateTo(ScreenType::MatchModeSelection),
                },
                MenuItem {
                    translation_key: TranslationKey::LeagueTable,
                    action: MenuAction::NavigateTo(ScreenType::LeagueTable),
                },
                MenuItem {
                    translation_key: TranslationKey::Notifications,
                    action: MenuAction::NavigateTo(ScreenType::Notifications),
                },
                MenuItem {
                    translation_key: TranslationKey::Settings,
                    action: MenuAction::NavigateTo(ScreenType::Settings),
                },
                MenuItem {
                    translation_key: TranslationKey::SaveLoad,
                    action: MenuAction::NavigateTo(ScreenType::SaveLoad),
                },
                MenuItem {
                    translation_key: TranslationKey::ExitGame,
                    action: MenuAction::Quit,
                },
            ],
        }
    }
}

impl Screen for MainMenuScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();

        // 标题
        let title = Paragraph::new(i18n.t(TranslationKey::GameTitle))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(title, Rect { x: 0, y: 2, width: size.width, height: 3 });

        // 日期和赛季信息
        let date_str = state.current_date.format_with_weekday();
        let season_str = state.current_date.season_string();
        let date_label = i18n.t(TranslationKey::CurrentDate);
        let season_label = i18n.t(TranslationKey::Season);
        let date_info = Paragraph::new(format!("{}: {} | {}: {}", date_label, date_str, season_label, season_str))
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center);
        f.render_widget(date_info, Rect { x: 0, y: 5, width: size.width, height: 1 });

        // 球队信息
        if let Some(team) = state.get_player_team() {
            let team_label = i18n.t(TranslationKey::TeamInfo);
            let budget_label = i18n.t(TranslationKey::Budget);
            let info = Paragraph::new(format!("{}: {} | {}: ${}", team_label, team.name, budget_label, team.budget))
                .alignment(Alignment::Center);
            f.render_widget(info, Rect { x: 0, y: 7, width: size.width, height: 1 });
        }

        // 菜单
        let unread_count = state.notifications.unread_count();
        let menu_items: Vec<Line> = self.menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut label = i18n.t(item.translation_key).to_string();
                
                // 通知项显示未读数量
                if item.translation_key == TranslationKey::Notifications && unread_count > 0 {
                    let style_suffix = if state.notifications.has_urgent() {
                        format!(" [{}!]", unread_count)  // 有紧急通知
                    } else {
                        format!(" [{}]", unread_count)
                    };
                    label.push_str(&style_suffix);
                }
                
                if i == self.selected_index {
                    let style = if item.translation_key == TranslationKey::Notifications 
                        && state.notifications.has_urgent() {
                        Style::default().fg(Color::Red)  // 紧急通知红色高亮
                    } else {
                        Style::default().fg(Color::Yellow)
                    };
                    Line::from(format!("> {}", label)).style(style)
                } else {
                    Line::from(format!("  {}", label))
                }
            })
            .collect();

        let menu = List::new(menu_items)
            .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::MainMenu)));
        f.render_widget(menu, Rect { x: size.width / 4, y: 8, width: size.width / 2, height: 14 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.menu_items.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                let action = &self.menu_items[self.selected_index].action;
                match action {
                    MenuAction::NavigateTo(screen_type) => {
                        state.navigate_to(screen_type.to_screen());
                    }
                    MenuAction::Quit => {
                        return Err(UiError::QuitRequested);
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
```

### 2.3 球队管理界面

```rust
pub struct TeamManagementScreen {
    selected_tab: TeamTab,
    selected_player_index: usize,
    // 筛选状态
    filter: PlayerFilter,
    sort_by: SortOption,
    show_filter_panel: bool,
}

enum TeamTab {
    Squad,
    Tactics,
    Statistics,
}

/// 球员筛选条件
#[derive(Debug, Clone, Default)]
pub struct PlayerFilter {
    pub position: Option<Position>,    // 按位置筛选
    pub min_age: Option<u8>,           // 最小年龄
    pub max_age: Option<u8>,           // 最大年龄
    pub min_ability: Option<u16>,      // 最小能力值
    pub max_ability: Option<u16>,      // 最大能力值
}

impl PlayerFilter {
    pub fn is_empty(&self) -> bool {
        self.position.is_none() 
            && self.min_age.is_none() 
            && self.max_age.is_none()
            && self.min_ability.is_none()
            && self.max_ability.is_none()
    }
    
    pub fn matches(&self, player: &Player) -> bool {
        if let Some(pos) = &self.position {
            if player.position != *pos && !player.second_positions.contains(pos) {
                return false;
            }
        }
        if let Some(min) = self.min_age {
            if player.age < min { return false; }
        }
        if let Some(max) = self.max_age {
            if player.age > max { return false; }
        }
        if let Some(min) = self.min_ability {
            if player.current_ability < min { return false; }
        }
        if let Some(max) = self.max_ability {
            if player.current_ability > max { return false; }
        }
        true
    }
    
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

/// 排序选项
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortOption {
    Name,
    Age,
    Ability,
    Position,
}

impl Default for SortOption {
    fn default() -> Self { SortOption::Position }
}

impl TeamManagementScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: TeamTab::Squad,
            selected_player_index: 0,
            filter: PlayerFilter::default(),
            sort_by: SortOption::default(),
            show_filter_panel: false,
        }
    }
    
    /// 应用筛选和排序，返回处理后的球员列表
    fn get_filtered_players<'a>(&self, players: &'a [Player]) -> Vec<&'a Player> {
        let mut filtered: Vec<&Player> = players.iter()
            .filter(|p| self.filter.matches(p))
            .collect();
        
        // 排序
        match self.sort_by {
            SortOption::Name => filtered.sort_by(|a, b| a.name.cmp(&b.name)),
            SortOption::Age => filtered.sort_by(|a, b| a.age.cmp(&b.age)),
            SortOption::Ability => filtered.sort_by(|a, b| b.current_ability.cmp(&a.current_ability)),
            SortOption::Position => filtered.sort_by(|a, b| {
                let pos_order = |p: &Position| match p {
                    Position::GK => 0,
                    Position::CB => 1, Position::LB => 2, Position::RB => 3, Position::WB => 4,
                    Position::DM => 5, Position::CM => 6, Position::AM => 7,
                    Position::LW => 8, Position::RW => 9,
                    Position::ST => 10, Position::CF => 11,
                };
                pos_order(&a.position).cmp(&pos_order(&b.position))
            }),
        }
        
        filtered
    }
}

impl Screen for TeamManagementScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();

        // 标题栏
        let header = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::TeamManagement));
        f.render_widget(header, size);

        // 标签页
        let tabs = vec![
            i18n.t(TranslationKey::Squad),
            i18n.t(TranslationKey::Tactics),
            i18n.t(TranslationKey::Statistics),
        ];
        let tab_names: Vec<Line> = tabs
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let index = match self.selected_tab {
                    TeamTab::Squad => 0,
                    TeamTab::Tactics => 1,
                    TeamTab::Statistics => 2,
                };
                if i == index {
                    Line::from(*name).style(Style::default().fg(Color::Yellow))
                } else {
                    Line::from(*name)
                }
            })
            .collect();

        let tab_widget = Tabs::new(tab_names);
        f.render_widget(tab_widget, Rect { x: 1, y: 1, width: size.width - 2, height: 3 });

        // 内容区域
        match self.selected_tab {
            TeamTab::Squad => self.render_squad(f, state, i18n),
            TeamTab::Tactics => self.render_tactics(f, state, i18n),
            TeamTab::Statistics => self.render_statistics(f, state, i18n),
        }
        
        // 筛选面板（如果展开）
        if self.show_filter_panel {
            self.render_filter_panel(f, state, i18n);
        }
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        // 如果筛选面板打开，优先处理
        if self.show_filter_panel {
            match key.code {
                KeyCode::Esc => self.show_filter_panel = false,
                KeyCode::Char('c') | KeyCode::Char('C') => {
                    self.filter.clear();
                    self.show_filter_panel = false;
                }
                // 筛选面板内的导航...
                _ => {}
            }
            return Ok(());
        }
        
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                // 切换标签
                self.selected_tab = match (&self.selected_tab, key.code) {
                    (TeamTab::Squad, KeyCode::Right) => TeamTab::Tactics,
                    (TeamTab::Tactics, KeyCode::Right) => TeamTab::Statistics,
                    (TeamTab::Statistics, KeyCode::Right) => TeamTab::Squad,
                    (TeamTab::Squad, KeyCode::Left) => TeamTab::Statistics,
                    (TeamTab::Tactics, KeyCode::Left) => TeamTab::Squad,
                    (TeamTab::Statistics, KeyCode::Left) => TeamTab::Tactics,
                    _ => self.selected_tab.clone(),
                };
                self.selected_player_index = 0;
            }
            KeyCode::Up => {
                if self.selected_player_index > 0 {
                    self.selected_player_index -= 1;
                }
            }
            KeyCode::Down => {
                // 需要检查列表长度
                self.selected_player_index += 1;
            }
            KeyCode::Enter => {
                // 查看球员详情
                // state.navigate_to(Screen::PlayerDetail { player_id: selected_id });
            }
            KeyCode::Char('f') | KeyCode::Char('F') => {
                // 打开筛选面板
                self.show_filter_panel = true;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // 切换排序方式
                self.sort_by = match self.sort_by {
                    SortOption::Position => SortOption::Name,
                    SortOption::Name => SortOption::Age,
                    SortOption::Age => SortOption::Ability,
                    SortOption::Ability => SortOption::Position,
                };
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl TeamManagementScreen {
    fn render_squad(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let team = match state.get_player_team() {
            Some(t) => t,
            None => return,
        };

        // 球员列表表格
        let rows = vec![
            Row::new(vec!["Name", "Pos", "Age", "Ability", "Status"]),
            // 实际球员数据...
        ];

        let header = Row::new(vec![
            i18n.t(TranslationKey::PlayerName),
            i18n.t(TranslationKey::Position),
            i18n.t(TranslationKey::Age),
            i18n.t(TranslationKey::Ability),
            i18n.t(TranslationKey::Status),
        ]).style(Style::default().fg(Color::Yellow));

        let table = Table::new(rows)
            .header(header)
            .widths(&[Constraint::Percentage(25), Constraint::Percentage(15), Constraint::Percentage(10), Constraint::Percentage(15), Constraint::Percentage(15)])
            .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::PlayerList)));

        f.render_widget(table, Rect { x: 2, y: 5, width: 60, height: 20 });
        
        // 筛选/排序状态栏
        let filter_status = if self.filter.is_empty() {
            format!("[F] {} | [S] {}: {:?}", 
                i18n.t(TranslationKey::Filter),
                i18n.t(TranslationKey::SortBy),
                self.sort_by)
        } else {
            format!("[F] {} (已启用) | [S] {}: {:?} | [C] {}", 
                i18n.t(TranslationKey::Filter),
                i18n.t(TranslationKey::SortBy),
                self.sort_by,
                i18n.t(TranslationKey::ClearFilter))
        };
        let status_para = Paragraph::new(filter_status)
            .style(Style::default().fg(Color::Gray));
        f.render_widget(status_para, Rect { x: 2, y: 26, width: 70, height: 1 });
    }
    
    fn render_tactics(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        // 战术标签的内容
    }
    
    fn render_statistics(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        // 统计标签的内容
    }
    
    /// 渲染筛选面板（弹出层）
    fn render_filter_panel(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 半透明背景
        let panel_width = 50;
        let panel_height = 16;
        let panel_x = (size.width - panel_width) / 2;
        let panel_y = (size.height - panel_height) / 2;
        
        let panel_area = Rect { x: panel_x, y: panel_y, width: panel_width, height: panel_height };
        
        // 清除背景
        f.render_widget(Clear, panel_area);
        
        // 面板框
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(i18n.t(TranslationKey::Filter));
        f.render_widget(block, panel_area);
        
        // 筛选选项
        let options = vec![
            format!("{}: {:?}", i18n.t(TranslationKey::FilterByPosition), 
                self.filter.position.as_ref().map(|p| format!("{:?}", p)).unwrap_or_else(|| i18n.t(TranslationKey::AllPositions).to_string())),
            format!("{}: {}-{}", i18n.t(TranslationKey::AgeRange),
                self.filter.min_age.unwrap_or(16),
                self.filter.max_age.unwrap_or(40)),
            format!("{}: {}-{}", i18n.t(TranslationKey::AbilityRange),
                self.filter.min_ability.unwrap_or(0),
                self.filter.max_ability.unwrap_or(200)),
        ];
        
        for (i, opt) in options.iter().enumerate() {
            let para = Paragraph::new(opt.as_str());
            f.render_widget(para, Rect { 
                x: panel_x + 2, 
                y: panel_y + 2 + i as u16, 
                width: panel_width - 4, 
                height: 1 
            });
        }
        
        // 位置快捷选择
        let positions = "GK CB LB RB DM CM AM LW RW ST";
        let pos_hint = Paragraph::new(positions)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(pos_hint, Rect { 
            x: panel_x + 2, y: panel_y + 7, width: panel_width - 4, height: 1 
        });
        
        // 操作提示
        let hints = Paragraph::new("[1-9] 选择位置 | [0] 全部 | [C] 清除 | [Esc] 关闭")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { 
            x: panel_x + 2, y: panel_y + panel_height - 3, width: panel_width - 4, height: 1 
        });
    }
}
```

### 2.4 战术设置界面

```rust
pub struct TacticsScreen {
    selected_section: TacticalSection,
    selected_formation: usize,
}

enum TacticalSection {
    Formation,
    PlayerRoles,
    TeamInstructions,
}

impl Screen for TacticsScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        // 显示当前阵型
        // 显示球员角色
        // 显示团队指令（进攻倾向、防守高度等）
        // 所有文本使用 i18n.t() 获取翻译
    }
}
```

### 2.5 转会市场界面

```rust
pub struct TransferMarketScreen {
    selected_tab: TransferTab,
    selected_player_index: usize,
    filters: MarketFilters,
}

enum TransferTab {
    Browse,
    SellPlayers,
}

impl Screen for TransferMarketScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        match self.selected_tab {
            TransferTab::Browse => self.render_browse(f, state, i18n),
            TransferTab::SellPlayers => self.render_sell(f, state, i18n),
        }
    }
}
```

### 2.6 比赛模式选择界面

```rust
pub struct MatchModeSelectionScreen {
    selected_mode: MatchMode,
}

impl Screen for MatchModeSelectionScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();

        let modes = vec![
            i18n.t(TranslationKey::LiveText),
            i18n.t(TranslationKey::QuickSimulation),
        ];
        let items: Vec<Line> = modes
            .iter()
            .enumerate()
            .map(|(i, mode)| {
                let is_selected = match (self.selected_mode, i) {
                    (MatchMode::Live, 0) => true,
                    (MatchMode::Quick, 1) => true,
                    _ => false,
                };
                if is_selected {
                    Line::from(format!("> {}", mode)).style(Style::default().fg(Color::Yellow))
                } else {
                    Line::from(format!("  {}", mode))
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::MatchModeSelection)));
        f.render_widget(list, Rect { x: size.width / 3, y: 10, width: size.width / 3, height: 6 });
    }
}
```

### 2.7 文本直播界面

```rust
use crate::ai::match_sim::{SubstitutionSuggestion, SubstitutionAdvisor, SuggestionUrgency};

pub struct MatchLiveScreen {
    match_id: String,
    events: Vec<String>,
    scroll_offset: usize,
    paused: bool,
    current_minute: u8,
    home_score: u8,
    away_score: u8,
    subs_remaining: u8,
    suggestions: Vec<SubstitutionSuggestion>,  // AI 换人建议
    show_suggestions_panel: bool,              // 是否显示建议面板
    selected_suggestion: usize,                // 选中的建议索引
}

impl MatchLiveScreen {
    pub fn new(match_id: String) -> Self {
        Self {
            match_id,
            events: vec![],
            scroll_offset: 0,
            paused: false,
            current_minute: 0,
            home_score: 0,
            away_score: 0,
            subs_remaining: 3,
            suggestions: vec![],
            show_suggestions_panel: false,
            selected_suggestion: 0,
        }
    }
    
    /// 更新 AI 换人建议（每隔一段时间调用）
    pub fn update_suggestions(
        &mut self,
        team: &Team,
        starting_players: &[Player],
        bench_players: &[Player],
        is_home: bool,
    ) {
        let score_diff = if is_home {
            self.home_score as i8 - self.away_score as i8
        } else {
            self.away_score as i8 - self.home_score as i8
        };
        
        self.suggestions = SubstitutionAdvisor::analyze(
            self.current_minute,
            team,
            starting_players,
            bench_players,
            score_diff,
            self.subs_remaining,
        );
    }
}

impl Screen for MatchLiveScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();

        // 比分显示
        let score = Paragraph::new(format!("Home {} - {} Away", self.home_score, self.away_score))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(score, Rect { x: 0, y: 0, width: size.width, height: 3 });

        // 时间 + 换人剩余
        let time_info = format!("{}' | 换人剩余: {}", self.current_minute, self.subs_remaining);
        let time = Paragraph::new(time_info)
            .alignment(Alignment::Center);
        f.render_widget(time, Rect { x: 0, y: 4, width: size.width, height: 1 });

        // 事件列表区域（如果有建议面板则缩小）
        let events_height = if self.show_suggestions_panel { 
            size.height - 18 
        } else { 
            size.height - 10 
        };
        
        let events: Vec<Line> = self.events
            .iter()
            .map(|e| Line::from(e.as_str()))
            .collect();

        let event_list = List::new(events)
            .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::MatchEvents)));
        f.render_widget(event_list, Rect { x: 2, y: 6, width: size.width - 4, height: events_height });

        // === AI 换人建议提示 ===
        if !self.suggestions.is_empty() && !self.show_suggestions_panel {
            let urgent_count = self.suggestions.iter()
                .filter(|s| s.urgency >= SuggestionUrgency::High)
                .count();
            
            let hint_style = if urgent_count > 0 {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Yellow)
            };
            
            let hint_text = if urgent_count > 0 {
                format!("⚠ 有 {} 条换人建议 ({}条紧急) - 按 [S] 查看", 
                    self.suggestions.len(), urgent_count)
            } else {
                format!("💡 有 {} 条换人建议 - 按 [S] 查看", self.suggestions.len())
            };
            
            let suggestion_hint = Paragraph::new(hint_text).style(hint_style);
            f.render_widget(suggestion_hint, Rect { 
                x: 2, 
                y: 6 + events_height, 
                width: size.width - 4, 
                height: 1 
            });
        }

        // === 建议面板（展开时显示） ===
        if self.show_suggestions_panel && !self.suggestions.is_empty() {
            let panel_y = 6 + events_height + 1;
            let panel_block = Block::default()
                .borders(Borders::ALL)
                .title("换人建议 (按 Enter 执行换人，Esc 关闭)")
                .border_style(Style::default().fg(Color::Yellow));
            f.render_widget(panel_block, Rect { 
                x: 2, y: panel_y, width: size.width - 4, height: 7 
            });
            
            let suggestion_items: Vec<Line> = self.suggestions.iter()
                .enumerate()
                .map(|(i, s)| {
                    let urgency_icon = match s.urgency {
                        SuggestionUrgency::Critical => "🔴",
                        SuggestionUrgency::High => "🟠",
                        SuggestionUrgency::Medium => "🟡",
                        SuggestionUrgency::Low => "🟢",
                    };
                    
                    let text = format!(
                        "{} {} → {} ({})",
                        urgency_icon,
                        s.player_out_name,
                        s.player_in_name,
                        s.reason.description()
                    );
                    
                    if i == self.selected_suggestion {
                        Line::from(format!("> {}", text))
                            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    } else {
                        Line::from(format!("  {}", text))
                    }
                })
                .collect();
            
            let suggestions_list = Paragraph::new(suggestion_items);
            f.render_widget(suggestions_list, Rect { 
                x: 4, y: panel_y + 1, width: size.width - 8, height: 5 
            });
        }

        // 控制提示
        let pause_hint = i18n.t(TranslationKey::PressSpaceToPause);
        let quit_hint = i18n.t(TranslationKey::PressQToQuit);
        let subs_hint = if !self.suggestions.is_empty() { " | [S] 建议" } else { "" };
        let help = Paragraph::new(format!("{} | {}{}", pause_hint, quit_hint, subs_hint))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(help, Rect { x: 0, y: size.height - 2, width: size.width, height: 1 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Char(' ') => {
                self.paused = !self.paused;
            }
            KeyCode::Char('q') => {
                state.go_back();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                // 切换建议面板显示
                if !self.suggestions.is_empty() {
                    self.show_suggestions_panel = !self.show_suggestions_panel;
                    self.selected_suggestion = 0;
                }
            }
            KeyCode::Up if self.show_suggestions_panel => {
                if self.selected_suggestion > 0 {
                    self.selected_suggestion -= 1;
                }
            }
            KeyCode::Down if self.show_suggestions_panel => {
                if self.selected_suggestion < self.suggestions.len().saturating_sub(1) {
                    self.selected_suggestion += 1;
                }
            }
            KeyCode::Enter if self.show_suggestions_panel => {
                // 执行选中的换人（实际实现需要调用换人逻辑）
                if let Some(suggestion) = self.suggestions.get(self.selected_suggestion) {
                    // TODO: 执行换人
                    // execute_substitution(suggestion.player_out_id, suggestion.player_in_id);
                    self.subs_remaining = self.subs_remaining.saturating_sub(1);
                    self.suggestions.remove(self.selected_suggestion);
                    self.selected_suggestion = 0;
                    
                    if self.suggestions.is_empty() {
                        self.show_suggestions_panel = false;
                    }
                }
            }
            KeyCode::Esc if self.show_suggestions_panel => {
                self.show_suggestions_panel = false;
            }
            _ => {}
        }
        Ok(())
    }
}
```

### 2.8 比赛结果界面

```rust
pub struct MatchResultScreen {
    match_id: String,
    selected_tab: MatchResultTab,
}

#[derive(Debug, Clone, PartialEq)]
enum MatchResultTab {
    Overview,    // 概览：比分 + 主要统计
    Statistics,  // 详细统计
    Ratings,     // 球员评分
}

impl MatchResultScreen {
    pub fn new(match_id: String) -> Self {
        Self {
            match_id,
            selected_tab: MatchResultTab::Overview,
        }
    }
}

impl Screen for MatchResultScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 假设已获取 match_result: MatchResult
        // let match_result = match_repo.get_by_id(&self.match_id)?;
        
        // === 顶部：比分 ===
        let score_text = "北京国安  2 - 1  上海申花";  // 示例
        let score = Paragraph::new(score_text)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(score, Rect { x: 0, y: 1, width: size.width, height: 2 });
        
        // 全场最佳
        let motm = Paragraph::new("⭐ 全场最佳: 张三 (8.5)")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center);
        f.render_widget(motm, Rect { x: 0, y: 3, width: size.width, height: 1 });

        // === 标签页 ===
        let tabs = vec!["概览", "统计", "评分"];
        let tab_items: Vec<Line> = tabs.iter().enumerate().map(|(i, t)| {
            let is_selected = match (&self.selected_tab, i) {
                (MatchResultTab::Overview, 0) => true,
                (MatchResultTab::Statistics, 1) => true,
                (MatchResultTab::Ratings, 2) => true,
                _ => false,
            };
            if is_selected {
                Line::from(format!("[{}]", t)).style(Style::default().fg(Color::Yellow))
            } else {
                Line::from(format!(" {} ", t))
            }
        }).collect();
        let tabs_widget = Tabs::new(tab_items);
        f.render_widget(tabs_widget, Rect { x: 2, y: 5, width: size.width - 4, height: 1 });

        // === 内容区域 ===
        let content_area = Rect { x: 2, y: 7, width: size.width - 4, height: size.height - 11 };
        
        match self.selected_tab {
            MatchResultTab::Overview => self.render_overview(f, content_area),
            MatchResultTab::Statistics => self.render_statistics(f, content_area),
            MatchResultTab::Ratings => self.render_ratings(f, content_area),
        }

        // === 底部提示 ===
        let hints = Paragraph::new("[←/→] 切换标签 | [Enter] 继续 | [Esc] 返回主菜单")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Left => {
                self.selected_tab = match self.selected_tab {
                    MatchResultTab::Overview => MatchResultTab::Ratings,
                    MatchResultTab::Statistics => MatchResultTab::Overview,
                    MatchResultTab::Ratings => MatchResultTab::Statistics,
                };
            }
            KeyCode::Right => {
                self.selected_tab = match self.selected_tab {
                    MatchResultTab::Overview => MatchResultTab::Statistics,
                    MatchResultTab::Statistics => MatchResultTab::Ratings,
                    MatchResultTab::Ratings => MatchResultTab::Overview,
                };
            }
            KeyCode::Enter | KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl MatchResultScreen {
    fn render_overview(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("比赛概览");
        f.render_widget(block, area);
        
        // 主要统计数据对比
        let stats = vec![
            ("控球率", "58%", "42%"),
            ("射门", "12", "8"),
            ("射正", "5", "3"),
            ("角球", "6", "4"),
            ("犯规", "10", "12"),
        ];
        
        let col_width = (area.width - 6) / 3;
        
        // 表头
        let header = Line::from(vec![
            Span::styled(format!("{:^width$}", "主队", width = col_width as usize), Style::default().fg(Color::Cyan)),
            Span::raw(format!("{:^12}", "")),
            Span::styled(format!("{:^width$}", "客队", width = col_width as usize), Style::default().fg(Color::Magenta)),
        ]);
        let header_para = Paragraph::new(header);
        f.render_widget(header_para, Rect { x: area.x + 2, y: area.y + 1, width: area.width - 4, height: 1 });
        
        // 统计行
        for (i, (label, home, away)) in stats.iter().enumerate() {
            let row = Line::from(vec![
                Span::styled(format!("{:>width$}", home, width = col_width as usize / 2), Style::default().fg(Color::Cyan)),
                Span::raw(format!("{:^12}", label)),
                Span::styled(format!("{:<width$}", away, width = col_width as usize / 2), Style::default().fg(Color::Magenta)),
            ]);
            let row_para = Paragraph::new(row);
            f.render_widget(row_para, Rect { 
                x: area.x + 2, y: area.y + 3 + i as u16, 
                width: area.width - 4, height: 1 
            });
        }
    }
    
    fn render_statistics(&self, f: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).title("详细统计");
        f.render_widget(block, area);
        
        let stats = vec![
            ("控球率", "58%", "42%"),
            ("射门", "12", "8"),
            ("射正", "5", "3"),
            ("射正率", "42%", "38%"),
            ("传球", "456", "312"),
            ("传球成功率", "85%", "78%"),
            ("角球", "6", "4"),
            ("越位", "2", "3"),
            ("犯规", "10", "12"),
            ("黄牌", "1", "2"),
            ("红牌", "0", "0"),
        ];
        
        for (i, (label, home, away)) in stats.iter().enumerate() {
            let y = area.y + 1 + i as u16;
            if y >= area.y + area.height - 1 { break; }
            
            let row = format!("{:>8}  {:^16}  {:<8}", home, label, away);
            let row_para = Paragraph::new(row).alignment(Alignment::Center);
            f.render_widget(row_para, Rect { x: area.x + 2, y, width: area.width - 4, height: 1 });
        }
    }
    
    fn render_ratings(&self, f: &mut Frame, area: Rect) {
        // 分两列显示主客队球员评分
        let half_width = (area.width - 2) / 2;
        
        // 主队
        let home_block = Block::default().borders(Borders::ALL).title("主队评分");
        f.render_widget(home_block, Rect { x: area.x, y: area.y, width: half_width, height: area.height });
        
        let home_ratings = vec![
            ("张三", "ST", "8.5 ⭐"),
            ("李四", "CM", "7.8"),
            ("王五", "CB", "7.2"),
            ("赵六", "GK", "7.5"),
        ];
        for (i, (name, pos, rating)) in home_ratings.iter().enumerate() {
            let y = area.y + 1 + i as u16;
            let text = format!("{} ({}) - {}", name, pos, rating);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect { x: area.x + 2, y, width: half_width - 4, height: 1 });
        }
        
        // 客队
        let away_block = Block::default().borders(Borders::ALL).title("客队评分");
        f.render_widget(away_block, Rect { x: area.x + half_width, y: area.y, width: half_width, height: area.height });
        
        let away_ratings = vec![
            ("陈七", "ST", "6.8"),
            ("刘八", "CM", "7.0"),
            ("周九", "CB", "6.5"),
        ];
        for (i, (name, pos, rating)) in away_ratings.iter().enumerate() {
            let y = area.y + 1 + i as u16;
            let text = format!("{} ({}) - {}", name, pos, rating);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect { x: area.x + half_width + 2, y, width: half_width - 4, height: 1 });
        }
    }
}
```

### 2.9 联赛积分榜界面

```rust
pub struct LeagueTableScreen {
    scroll_offset: usize,
}

impl Screen for LeagueTableScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let header = Row::new(vec![
            "#",
            i18n.t(TranslationKey::Team),
            i18n.t(TranslationKey::Played),
            i18n.t(TranslationKey::Won),
            i18n.t(TranslationKey::Drawn),
            i18n.t(TranslationKey::Lost),
            i18n.t(TranslationKey::GoalsFor),
            i18n.t(TranslationKey::GoalsAgainst),
            i18n.t(TranslationKey::GoalDifference),
            i18n.t(TranslationKey::Points),
        ]);
        
        let rows = vec![
            header,
            // 实际数据...
        ];

        let table = Table::new(rows)
            .widths(&[
                Constraint::Percentage(5),
                Constraint::Percentage(30),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
            ])
            .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::LeagueStandings)));

        f.render_widget(table, Rect { x: 5, y: 3, width: 70, height: 25 });
    }
}
```

### 2.10 赛季总结界面

```rust
pub struct SeasonSummaryScreen {
    season: String,
    selected_tab: SeasonSummaryTab,
}

#[derive(Debug, Clone, PartialEq)]
enum SeasonSummaryTab {
    Overview,    // 概览
    Standings,   // 最终排名
    TopScorers,  // 射手榜
    Awards,      // 奖项
}

impl SeasonSummaryScreen {
    pub fn new(season: String) -> Self {
        Self {
            season,
            selected_tab: SeasonSummaryTab::Overview,
        }
    }
}

impl Screen for SeasonSummaryScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 标题
        let title = format!("{} - {}", i18n.t(TranslationKey::SeasonSummary), self.season);
        let title_para = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title_para, Rect { x: 0, y: 1, width: size.width, height: 2 });
        
        // 标签页
        let tabs = vec!["概览", "排名", "射手榜", "奖项"];
        // ... 标签页渲染 ...
        
        let content_area = Rect { x: 2, y: 5, width: size.width - 4, height: size.height - 9 };
        
        match self.selected_tab {
            SeasonSummaryTab::Overview => self.render_overview(f, content_area, state, i18n),
            SeasonSummaryTab::Standings => self.render_standings(f, content_area, state, i18n),
            SeasonSummaryTab::TopScorers => self.render_scorers(f, content_area, state, i18n),
            SeasonSummaryTab::Awards => self.render_awards(f, content_area, state, i18n),
        }
        
        // 底部提示
        let hints = Paragraph::new("[←/→] 切换标签 | [Enter] 继续下赛季 | [Esc] 返回")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }
    
    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                // 切换标签
            }
            KeyCode::Enter => {
                // 继续下赛季
                state.start_new_season();
                state.navigate_to(Screen::MainMenu);
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl SeasonSummaryScreen {
    fn render_overview(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::YourPerformance));
        f.render_widget(block, area);
        
        // 玩家球队赛季表现
        let lines = vec![
            format!("{}: 第 {} 名", i18n.t(TranslationKey::LeaguePosition), 3),
            format!("比赛: 38 场 (胜 20, 平 10, 负 8)"),
            format!("{}: 65", i18n.t(TranslationKey::TotalGoals)),
            format!("{}: 40", i18n.t(TranslationKey::TotalAssists)),
            format!("{}: 12", i18n.t(TranslationKey::CleanSheets)),
            format!("积分: 70"),
        ];
        
        for (i, line) in lines.iter().enumerate() {
            let para = Paragraph::new(line.as_str());
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 2 + i as u16, width: area.width - 4, height: 1 });
        }
    }
    
    fn render_standings(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::FinalStandings));
        f.render_widget(block, area);
        
        // 显示最终积分榜前10名
        let standings = vec![
            ("1", "北京国安", "38", "25", "8", "5", "78", "32", "83"),
            ("2", "上海申花", "38", "23", "9", "6", "71", "38", "78"),
            ("3", "广州恒大", "38", "20", "10", "8", "65", "40", "70"),
            // ...
        ];
        
        // 表头
        let header = "# | 球队 | 场 | 胜 | 平 | 负 | 进 | 失 | 分";
        let header_para = Paragraph::new(header).style(Style::default().fg(Color::Yellow));
        f.render_widget(header_para, Rect { x: area.x + 2, y: area.y + 1, width: area.width - 4, height: 1 });
        
        for (i, (pos, team, p, w, d, l, gf, ga, pts)) in standings.iter().enumerate() {
            let row = format!("{:>2} | {:12} | {:2} | {:2} | {:2} | {:2} | {:2} | {:2} | {:2}", 
                pos, team, p, w, d, l, gf, ga, pts);
            let style = if i < 3 {
                Style::default().fg(Color::Green)  // 前三名绿色
            } else {
                Style::default()
            };
            let para = Paragraph::new(row).style(style);
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 3 + i as u16, width: area.width - 4, height: 1 });
        }
    }
    
    fn render_scorers(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::TopScorers));
        f.render_widget(block, area);
        
        let scorers = vec![
            ("1", "张三", "北京国安", "28"),
            ("2", "李四", "上海申花", "22"),
            ("3", "王五", "广州恒大", "19"),
            ("4", "赵六", "山东鲁能", "17"),
            ("5", "孙七", "江苏苏宁", "15"),
        ];
        
        let header = "# | 球员 | 球队 | 进球";
        let header_para = Paragraph::new(header).style(Style::default().fg(Color::Yellow));
        f.render_widget(header_para, Rect { x: area.x + 2, y: area.y + 1, width: area.width - 4, height: 1 });
        
        for (i, (pos, name, team, goals)) in scorers.iter().enumerate() {
            let row = format!("{:>2} | {:8} | {:12} | {:2}", pos, name, team, goals);
            let para = Paragraph::new(row);
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 3 + i as u16, width: area.width - 4, height: 1 });
        }
    }
    
    fn render_awards(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::BestPlayers));
        f.render_widget(block, area);
        
        let awards = vec![
            ("🏆 联赛冠军", "北京国安"),
            ("⚽ 最佳射手", "张三 (28球)"),
            ("🎯 助攻王", "王五 (18助)"),
            ("⭐ 最佳球员", "张三"),
            ("🧤 最佳门将", "李门神"),
            ("🌟 最佳新人", "小明"),
        ];
        
        for (i, (award, winner)) in awards.iter().enumerate() {
            let text = format!("{}: {}", award, winner);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 2 + i as u16 * 2, width: area.width - 4, height: 1 });
        }
    }
}
```

### 2.11 比赛历史界面

```rust
pub struct MatchHistoryScreen {
    selected_index: usize,
    scroll_offset: usize,
    filter: MatchHistoryFilter,
}

#[derive(Debug, Clone, PartialEq)]
enum MatchHistoryFilter {
    All,
    Wins,
    Draws,
    Losses,
}

impl MatchHistoryScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            scroll_offset: 0,
            filter: MatchHistoryFilter::All,
        }
    }
}

impl Screen for MatchHistoryScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 标题
        let title = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::MatchHistory));
        f.render_widget(title, size);
        
        // 筛选选项
        let filter_text = format!(
            "[A] 全部 | [W] 胜利 | [D] 平局 | [L] 失败 | 当前: {:?}",
            self.filter
        );
        let filter_para = Paragraph::new(filter_text)
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(filter_para, Rect { x: 2, y: 2, width: size.width - 4, height: 1 });
        
        // 比赛列表
        let matches = vec![
            ("第38轮", "2026-05-20", "北京国安", "2", "1", "上海申花", true),
            ("第37轮", "2026-05-13", "广州恒大", "1", "1", "北京国安", false),
            ("第36轮", "2026-05-06", "北京国安", "3", "0", "山东鲁能", true),
            // ...
        ];
        
        // 表头
        let header = "轮次 | 日期 | 主队 | 比分 | 客队";
        let header_para = Paragraph::new(header)
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(header_para, Rect { x: 2, y: 4, width: size.width - 4, height: 1 });
        
        let list_area = Rect { x: 2, y: 6, width: size.width - 4, height: size.height - 10 };
        
        for (i, (round, date, home, hs, as_, away, is_home)) in matches.iter().enumerate() {
            if i < self.scroll_offset { continue; }
            let y = list_area.y + (i - self.scroll_offset) as u16;
            if y >= list_area.y + list_area.height { break; }
            
            let is_selected = i == self.selected_index;
            let row = format!("{:6} | {} | {:12} {:>2} - {:<2} {:12}", 
                round, date, home, hs, as_, away);
            
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).fg(Color::White)
            } else {
                Style::default()
            };
            
            let para = Paragraph::new(row).style(style);
            f.render_widget(para, Rect { x: list_area.x, y, width: list_area.width, height: 1 });
        }
        
        // 底部提示
        let hints = Paragraph::new("[↑/↓] 选择 | [Enter] 查看详情 | [Esc] 返回")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }
    
    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                    if self.selected_index < self.scroll_offset {
                        self.scroll_offset = self.selected_index;
                    }
                }
            }
            KeyCode::Down => {
                self.selected_index += 1;
                // 需要检查列表长度
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.filter = MatchHistoryFilter::All;
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.filter = MatchHistoryFilter::Wins;
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.filter = MatchHistoryFilter::Draws;
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                self.filter = MatchHistoryFilter::Losses;
            }
            KeyCode::Enter => {
                // 查看比赛详情
                // state.navigate_to(Screen::MatchResult { match_id: selected_match.id });
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}
```

### 2.12 财务报告界面

```rust
pub struct FinanceReportScreen {
    selected_tab: FinanceTab,
}

#[derive(Debug, Clone, PartialEq)]
enum FinanceTab {
    Overview,   // 概览
    Income,     // 收入明细
    Expenses,   // 支出明细
    History,    // 历史记录
}

impl FinanceReportScreen {
    pub fn new() -> Self {
        Self {
            selected_tab: FinanceTab::Overview,
        }
    }
}

impl Screen for FinanceReportScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 标题
        let title = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::FinanceReport));
        f.render_widget(title, size);
        
        // 标签页
        let tabs = vec!["概览", "收入", "支出", "历史"];
        // ... 标签页渲染 ...
        
        let content_area = Rect { x: 2, y: 5, width: size.width - 4, height: size.height - 9 };
        
        match self.selected_tab {
            FinanceTab::Overview => self.render_overview(f, content_area, state, i18n),
            FinanceTab::Income => self.render_income(f, content_area, state, i18n),
            FinanceTab::Expenses => self.render_expenses(f, content_area, state, i18n),
            FinanceTab::History => self.render_history(f, content_area, state, i18n),
        }
        
        // 底部提示
        let hints = Paragraph::new("[←/→] 切换标签 | [Esc] 返回")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }
    
    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Left | KeyCode::Right => {
                // 切换标签
                self.selected_tab = match (&self.selected_tab, key.code) {
                    (FinanceTab::Overview, KeyCode::Right) => FinanceTab::Income,
                    (FinanceTab::Income, KeyCode::Right) => FinanceTab::Expenses,
                    (FinanceTab::Expenses, KeyCode::Right) => FinanceTab::History,
                    (FinanceTab::History, KeyCode::Right) => FinanceTab::Overview,
                    (FinanceTab::Overview, KeyCode::Left) => FinanceTab::History,
                    (FinanceTab::Income, KeyCode::Left) => FinanceTab::Overview,
                    (FinanceTab::Expenses, KeyCode::Left) => FinanceTab::Income,
                    (FinanceTab::History, KeyCode::Left) => FinanceTab::Expenses,
                    _ => self.selected_tab.clone(),
                };
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl FinanceReportScreen {
    fn render_overview(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::SeasonFinance));
        f.render_widget(block, area);
        
        // 余额
        let balance = "¥ 150,000,000";
        let balance_para = Paragraph::new(format!("当前余额: {}", balance))
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
        f.render_widget(balance_para, Rect { x: area.x + 2, y: area.y + 2, width: area.width - 4, height: 1 });
        
        // 本赛季收支
        let summary = vec![
            (i18n.t(TranslationKey::Income), "¥ 80,000,000", Color::Green),
            (i18n.t(TranslationKey::Expenses), "¥ 65,000,000", Color::Red),
            (i18n.t(TranslationKey::NetBalance), "¥ +15,000,000", Color::Cyan),
        ];
        
        for (i, (label, value, color)) in summary.iter().enumerate() {
            let text = format!("{}: {}", label, value);
            let para = Paragraph::new(text).style(Style::default().fg(*color));
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 5 + i as u16, width: area.width - 4, height: 1 });
        }
        
        // 收支对比条形图（简化版）
        let bar_y = area.y + 10;
        let income_bar = "收入 [████████████████████         ] 80M";
        let expense_bar = "支出 [████████████████             ] 65M";
        
        let income_para = Paragraph::new(income_bar).style(Style::default().fg(Color::Green));
        f.render_widget(income_para, Rect { x: area.x + 2, y: bar_y, width: area.width - 4, height: 1 });
        
        let expense_para = Paragraph::new(expense_bar).style(Style::default().fg(Color::Red));
        f.render_widget(expense_para, Rect { x: area.x + 2, y: bar_y + 1, width: area.width - 4, height: 1 });
    }
    
    fn render_income(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::Income));
        f.render_widget(block, area);
        
        let items = vec![
            (i18n.t(TranslationKey::TransferIncome), "¥ 30,000,000", "37.5%"),
            (i18n.t(TranslationKey::MatchdayIncome), "¥ 25,000,000", "31.3%"),
            (i18n.t(TranslationKey::Sponsorship), "¥ 20,000,000", "25.0%"),
            ("电视转播", "¥ 5,000,000", "6.2%"),
        ];
        
        for (i, (label, value, percent)) in items.iter().enumerate() {
            let text = format!("{:16} {:>15} ({:>5})", label, value, percent);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 2 + i as u16, width: area.width - 4, height: 1 });
        }
        
        // 总计
        let total = format!("{:16} {:>15}", "总计", "¥ 80,000,000");
        let total_para = Paragraph::new(total)
            .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD));
        f.render_widget(total_para, Rect { x: area.x + 2, y: area.y + 8, width: area.width - 4, height: 1 });
    }
    
    fn render_expenses(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::Expenses));
        f.render_widget(block, area);
        
        let items = vec![
            (i18n.t(TranslationKey::WageBill), "¥ 40,000,000", "61.5%"),
            (i18n.t(TranslationKey::TransferExpenses), "¥ 20,000,000", "30.8%"),
            ("运营费用", "¥ 3,000,000", "4.6%"),
            ("青训投入", "¥ 2,000,000", "3.1%"),
        ];
        
        for (i, (label, value, percent)) in items.iter().enumerate() {
            let text = format!("{:16} {:>15} ({:>5})", label, value, percent);
            let para = Paragraph::new(text);
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 2 + i as u16, width: area.width - 4, height: 1 });
        }
        
        // 总计
        let total = format!("{:16} {:>15}", "总计", "¥ 65,000,000");
        let total_para = Paragraph::new(total)
            .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
        f.render_widget(total_para, Rect { x: area.x + 2, y: area.y + 8, width: area.width - 4, height: 1 });
    }
    
    fn render_history(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default().borders(Borders::ALL).title("历史记录");
        f.render_widget(block, area);
        
        let history = vec![
            ("2026-05-20", "比赛日收入", "+¥ 800,000", Color::Green),
            ("2026-05-15", "球员转入", "-¥ 15,000,000", Color::Red),
            ("2026-05-10", "球员转出", "+¥ 20,000,000", Color::Green),
            ("2026-05-01", "月薪支出", "-¥ 3,500,000", Color::Red),
            ("2026-04-20", "比赛日收入", "+¥ 750,000", Color::Green),
        ];
        
        let header = "日期 | 描述 | 金额";
        let header_para = Paragraph::new(header).style(Style::default().fg(Color::Yellow));
        f.render_widget(header_para, Rect { x: area.x + 2, y: area.y + 1, width: area.width - 4, height: 1 });
        
        for (i, (date, desc, amount, color)) in history.iter().enumerate() {
            let text = format!("{} | {:16} | {:>15}", date, desc, amount);
            let para = Paragraph::new(text).style(Style::default().fg(*color));
            f.render_widget(para, Rect { x: area.x + 2, y: area.y + 3 + i as u16, width: area.width - 4, height: 1 });
        }
    }
}
```

### 2.13 通知列表界面

```rust
pub struct NotificationsScreen {
    selected_index: usize,
    scroll_offset: usize,
}

impl NotificationsScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            scroll_offset: 0,
        }
    }
}

impl Screen for NotificationsScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        let notifications = state.notifications.get_all(self.scroll_offset, 15);

        // 标题栏：显示未读数量
        let unread = state.notifications.unread_count();
        let title = if unread > 0 {
            format!("{} - {} {}", 
                i18n.t(TranslationKey::Notifications),
                unread,
                i18n.t(TranslationKey::UnreadNotifications))
        } else {
            i18n.t(TranslationKey::Notifications).to_string()
        };

        if notifications.is_empty() {
            // 无通知
            let empty_msg = Paragraph::new(i18n.t(TranslationKey::NoNotifications))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title(title));
            f.render_widget(empty_msg, Rect { x: 2, y: 2, width: size.width - 4, height: size.height - 4 });
            return;
        }

        // 通知列表
        let items: Vec<Line> = notifications
            .iter()
            .enumerate()
            .map(|(i, n)| {
                // 图标根据类型
                let icon = match n.notification_type {
                    NotificationType::Transfer => "💰",
                    NotificationType::Injury => "🏥",
                    NotificationType::Contract => "📝",
                    NotificationType::Match => "⚽",
                    NotificationType::Finance => "💵",
                    NotificationType::PlayerMorale => "😊",
                    NotificationType::Achievement => "🏆",
                    NotificationType::News => "📰",
                    NotificationType::System => "⚙️",
                };

                // 优先级颜色
                let priority_color = match n.priority {
                    Priority::Urgent => Color::Red,
                    Priority::High => Color::Yellow,
                    Priority::Normal => Color::White,
                    Priority::Low => Color::Gray,
                };

                // 已读/未读标记
                let read_mark = if n.read { "  " } else { "● " };

                let text = format!("{}{} {}", read_mark, icon, n.title);
                
                if i == self.selected_index {
                    Line::from(format!("> {}", text))
                        .style(Style::default().fg(priority_color).add_modifier(Modifier::BOLD))
                } else {
                    Line::from(format!("  {}", text))
                        .style(Style::default().fg(if n.read { Color::DarkGray } else { priority_color }))
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title(title));
        f.render_widget(list, Rect { x: 2, y: 2, width: size.width - 4, height: size.height - 8 });

        // 选中通知的详细内容
        if let Some(selected) = notifications.get(self.selected_index) {
            let detail = Paragraph::new(selected.message.clone())
                .wrap(Wrap { trim: true })
                .block(Block::default().borders(Borders::ALL).title("详情"));
            f.render_widget(detail, Rect { x: 2, y: size.height - 6, width: size.width - 4, height: 4 });
        }

        // 操作提示
        let hints = Paragraph::new(format!(
            "[Enter] 标为已读 | [A] {} | [Esc] {}",
            i18n.t(TranslationKey::MarkAllRead),
            i18n.t(TranslationKey::Back)
        )).style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        let count = state.notifications.get_all(0, 100).len();
        
        match key.code {
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < count.saturating_sub(1) {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => {
                // 标记选中的通知为已读
                if let Some(n) = state.notifications.get_all(0, 100).get(self.selected_index) {
                    let id = n.id.clone();
                    state.notifications.mark_read(&id);
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                // 全部标为已读
                state.notifications.mark_all_read();
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}
```

### 2.11 球员详情界面

```rust
pub struct PlayerDetailScreen {
    player_id: String,
    selected_tab: PlayerDetailTab,
}

#[derive(Debug, Clone, PartialEq)]
enum PlayerDetailTab {
    Overview,     // 概览：基本信息 + 状态
    Attributes,   // 属性：技术/精神/身体
    Contract,     // 合同：薪资/剩余年限/市场价值
}

impl PlayerDetailScreen {
    pub fn new(player_id: String) -> Self {
        Self {
            player_id,
            selected_tab: PlayerDetailTab::Overview,
        }
    }
}

impl Screen for PlayerDetailScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 获取球员数据（实际实现需要从 Repository 获取）
        // let player = player_repo.get_by_id(&self.player_id)?;
        // 这里假设已获取到 player
        
        // === 顶部：球员基本信息 ===
        let header_area = Rect { x: 2, y: 1, width: size.width - 4, height: 5 };
        let header_block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::PlayerDetail));
        f.render_widget(header_block, header_area);
        
        // 球员名字、位置、年龄、国籍
        // let info = format!("{} | {} | {} 岁 | {}", player.name, player.position, player.age, player.nationality);
        let info = Paragraph::new("球员名字 | ST | 25 岁 | 中国")  // 示例
            .alignment(Alignment::Center);
        f.render_widget(info, Rect { x: 4, y: 2, width: size.width - 8, height: 1 });
        
        // 能力值显示
        // let ability_info = format!("能力: {} / 潜力: {}", player.current_ability, player.potential_ability);
        let ability_info = Paragraph::new("能力: 85 / 潜力: 150")
            .alignment(Alignment::Center);
        f.render_widget(ability_info, Rect { x: 4, y: 4, width: size.width - 8, height: 1 });

        // === 标签页 ===
        let tabs = vec![
            i18n.t(TranslationKey::Overview),
            i18n.t(TranslationKey::Attributes),
            i18n.t(TranslationKey::Contract),
        ];
        let tab_titles: Vec<Line> = tabs.iter().enumerate().map(|(i, t)| {
            let is_selected = match (&self.selected_tab, i) {
                (PlayerDetailTab::Overview, 0) => true,
                (PlayerDetailTab::Attributes, 1) => true,
                (PlayerDetailTab::Contract, 2) => true,
                _ => false,
            };
            if is_selected {
                Line::from(format!("[{}]", t)).style(Style::default().fg(Color::Yellow))
            } else {
                Line::from(format!(" {} ", t))
            }
        }).collect();
        
        let tabs_widget = Tabs::new(tab_titles);
        f.render_widget(tabs_widget, Rect { x: 2, y: 6, width: size.width - 4, height: 1 });

        // === 内容区域 ===
        let content_area = Rect { x: 2, y: 8, width: size.width - 4, height: size.height - 12 };
        
        match self.selected_tab {
            PlayerDetailTab::Overview => self.render_overview(f, content_area, i18n),
            PlayerDetailTab::Attributes => self.render_attributes(f, content_area, i18n),
            PlayerDetailTab::Contract => self.render_contract(f, content_area, i18n),
        }

        // === 底部操作提示 ===
        let hints = Paragraph::new("[←/→] 切换标签 | [Esc] 返回")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Left => {
                self.selected_tab = match self.selected_tab {
                    PlayerDetailTab::Overview => PlayerDetailTab::Contract,
                    PlayerDetailTab::Attributes => PlayerDetailTab::Overview,
                    PlayerDetailTab::Contract => PlayerDetailTab::Attributes,
                };
            }
            KeyCode::Right => {
                self.selected_tab = match self.selected_tab {
                    PlayerDetailTab::Overview => PlayerDetailTab::Attributes,
                    PlayerDetailTab::Attributes => PlayerDetailTab::Contract,
                    PlayerDetailTab::Contract => PlayerDetailTab::Overview,
                };
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl PlayerDetailScreen {
    /// 渲染概览标签
    fn render_overview(&self, f: &mut Frame, area: Rect, i18n: &I18n) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::Overview));
        f.render_widget(block, area);
        
        // 状态信息
        let status_items = vec![
            Line::from(format!("{}: {} ({})", 
                i18n.t(TranslationKey::Condition), 
                "良好",  // 使用 get_player_condition_rating()
                "75/100")),
            Line::from(format!("{}: {}/100", i18n.t(TranslationKey::Fatigue), 20)),
            Line::from(format!("{}: {}/100", i18n.t(TranslationKey::Morale), 80)),
            Line::from(format!("{}: {}/100", i18n.t(TranslationKey::MatchFitness), 90)),
            Line::from(""),
            Line::from(format!("{}: {:?}", i18n.t(TranslationKey::Status), "健康")),
            Line::from(format!("{}: {:?}", i18n.t(TranslationKey::Position), "ST")),
        ];
        
        let status = Paragraph::new(status_items);
        f.render_widget(status, Rect { 
            x: area.x + 2, 
            y: area.y + 1, 
            width: area.width - 4, 
            height: area.height - 2 
        });
    }

    /// 渲染属性标签
    fn render_attributes(&self, f: &mut Frame, area: Rect, i18n: &I18n) {
        // 分三列显示：技术 | 精神 | 身体
        let col_width = (area.width - 6) / 3;
        
        // 技术属性
        let tech_area = Rect { x: area.x, y: area.y, width: col_width + 2, height: area.height };
        let tech_block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::TechnicalAttributes));
        f.render_widget(tech_block, tech_area);
        
        let tech_attrs = vec![
            format!("{}: 85", i18n.t(TranslationKey::AttrFinishing)),
            format!("{}: 80", i18n.t(TranslationKey::AttrDribbling)),
            format!("{}: 75", i18n.t(TranslationKey::AttrPassing)),
            format!("{}: 70", i18n.t(TranslationKey::AttrHeading)),
            format!("{}: 78", i18n.t(TranslationKey::AttrTechnique)),
            format!("{}: 72", i18n.t(TranslationKey::AttrCrossing)),
            format!("{}: 65", i18n.t(TranslationKey::AttrTackling)),
        ];
        let tech_text = Paragraph::new(tech_attrs.join("\n"));
        f.render_widget(tech_text, Rect { 
            x: tech_area.x + 1, y: tech_area.y + 1, 
            width: tech_area.width - 2, height: tech_area.height - 2 
        });
        
        // 精神属性
        let mental_area = Rect { x: area.x + col_width + 2, y: area.y, width: col_width + 2, height: area.height };
        let mental_block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::MentalAttributes));
        f.render_widget(mental_block, mental_area);
        
        let mental_attrs = vec![
            format!("{}: 82", i18n.t(TranslationKey::AttrDecisions)),
            format!("{}: 85", i18n.t(TranslationKey::AttrOffTheBall)),
            format!("{}: 78", i18n.t(TranslationKey::AttrAnticipation)),
            format!("{}: 75", i18n.t(TranslationKey::AttrConcentration)),
            format!("{}: 80", i18n.t(TranslationKey::AttrVision)),
            format!("{}: 70", i18n.t(TranslationKey::AttrTeamwork)),
            format!("{}: 76", i18n.t(TranslationKey::AttrWorkRate)),
        ];
        let mental_text = Paragraph::new(mental_attrs.join("\n"));
        f.render_widget(mental_text, Rect { 
            x: mental_area.x + 1, y: mental_area.y + 1, 
            width: mental_area.width - 2, height: mental_area.height - 2 
        });
        
        // 身体属性
        let phys_area = Rect { x: area.x + (col_width + 2) * 2, y: area.y, width: col_width, height: area.height };
        let phys_block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::PhysicalAttributes));
        f.render_widget(phys_block, phys_area);
        
        let phys_attrs = vec![
            format!("{}: 88", i18n.t(TranslationKey::AttrPace)),
            format!("{}: 85", i18n.t(TranslationKey::AttrAcceleration)),
            format!("{}: 78", i18n.t(TranslationKey::AttrStamina)),
            format!("{}: 72", i18n.t(TranslationKey::AttrStrength)),
            format!("{}: 80", i18n.t(TranslationKey::AttrAgility)),
            format!("{}: 75", i18n.t(TranslationKey::AttrBalance)),
        ];
        let phys_text = Paragraph::new(phys_attrs.join("\n"));
        f.render_widget(phys_text, Rect { 
            x: phys_area.x + 1, y: phys_area.y + 1, 
            width: phys_area.width - 2, height: phys_area.height - 2 
        });
    }

    /// 渲染合同标签
    fn render_contract(&self, f: &mut Frame, area: Rect, i18n: &I18n) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::Contract));
        f.render_widget(block, area);
        
        let contract_info = vec![
            Line::from(format!("{}: $50,000/周", i18n.t(TranslationKey::Wage))),
            Line::from(format!("{}: 2 年", i18n.t(TranslationKey::ContractYears))),
            Line::from(format!("{}: $15,000,000", i18n.t(TranslationKey::MarketValue))),
            Line::from(""),
            Line::from(format!("{}: 150", i18n.t(TranslationKey::Potential))),
        ];
        
        let info = Paragraph::new(contract_info);
        f.render_widget(info, Rect { 
            x: area.x + 2, 
            y: area.y + 1, 
            width: area.width - 4, 
            height: area.height - 2 
        });
    }
}
```

### 2.12 设置界面

```rust
pub struct SettingsScreen {
    selected_section: SettingsSection,
    selected_item: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum SettingsSection {
    Language,
    Match,
}

#[derive(Debug, Clone)]
struct SettingItem {
    key: TranslationKey,
    setting_type: SettingType,
}

#[derive(Debug, Clone)]
enum SettingType {
    Language,           // 语言选择
    MatchMode,          // 比赛模式
    AutoSave,           // 自动保存开关
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            selected_section: SettingsSection::Language,
            selected_item: 0,
        }
    }
}

impl Screen for SettingsScreen {
    fn render(&self, f: &mut Frame, state: &GameState, i18n: &I18n) {
        let size = f.size();
        
        // 标题
        let title = Paragraph::new(i18n.t(TranslationKey::Settings))
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center);
        f.render_widget(title, Rect { x: 0, y: 1, width: size.width, height: 1 });

        // 左侧：设置分类
        let sections = vec![
            (SettingsSection::Language, i18n.t(TranslationKey::LanguageSettings)),
            (SettingsSection::Match, i18n.t(TranslationKey::MatchSettings)),
        ];
        
        let section_items: Vec<Line> = sections.iter().map(|(sec, label)| {
            if *sec == self.selected_section {
                Line::from(format!("> {}", label))
                    .style(Style::default().fg(Color::Yellow))
            } else {
                Line::from(format!("  {}", label))
            }
        }).collect();
        
        let section_list = List::new(section_items)
            .block(Block::default().borders(Borders::ALL).title("分类"));
        f.render_widget(section_list, Rect { x: 2, y: 3, width: 20, height: 10 });

        // 右侧：设置内容
        let content_area = Rect { x: 24, y: 3, width: size.width - 26, height: size.height - 7 };
        
        match self.selected_section {
            SettingsSection::Language => self.render_language_settings(f, content_area, state, i18n),
            SettingsSection::Match => self.render_match_settings(f, content_area, state, i18n),
        }

        // 底部操作提示
        let hints = Paragraph::new("[↑/↓] 选择 | [←/→] 切换分类 | [Enter] 修改 | [Esc] 返回")
            .style(Style::default().fg(Color::Gray));
        f.render_widget(hints, Rect { x: 2, y: size.height - 2, width: size.width - 4, height: 1 });
    }

    fn handle_key(&mut self, key: KeyEvent, state: &mut GameState) -> Result<(), UiError> {
        match key.code {
            KeyCode::Left => {
                self.selected_section = SettingsSection::Language;
                self.selected_item = 0;
            }
            KeyCode::Right => {
                self.selected_section = SettingsSection::Match;
                self.selected_item = 0;
            }
            KeyCode::Up => {
                if self.selected_item > 0 {
                    self.selected_item -= 1;
                }
            }
            KeyCode::Down => {
                let max_items = match self.selected_section {
                    SettingsSection::Language => 2,  // 中文、English
                    SettingsSection::Match => 2,     // 文本直播、快速模拟
                };
                if self.selected_item < max_items - 1 {
                    self.selected_item += 1;
                }
            }
            KeyCode::Enter => {
                match self.selected_section {
                    SettingsSection::Language => {
                        // 切换语言
                        let new_lang = match self.selected_item {
                            0 => Language::Chinese,
                            _ => Language::English,
                        };
                        // 实际实现需要更新 App 中的 i18n
                        // app.i18n.set_language(new_lang);
                    }
                    SettingsSection::Match => {
                        // 切换默认比赛模式
                        let new_mode = match self.selected_item {
                            0 => MatchMode::Live,
                            _ => MatchMode::Quick,
                        };
                        state.match_mode_preference = new_mode;
                    }
                }
            }
            KeyCode::Esc => {
                state.go_back();
            }
            _ => {}
        }
        Ok(())
    }
}

impl SettingsScreen {
    /// 渲染语言设置
    fn render_language_settings(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::LanguageSettings));
        f.render_widget(block, area);
        
        let current_lang = i18n.current_language();
        
        let items = vec![
            ("中文", Language::Chinese),
            ("English", Language::English),
        ];
        
        let lang_items: Vec<Line> = items.iter().enumerate().map(|(i, (label, lang))| {
            let is_current = *lang == current_lang;
            let prefix = if is_current { "● " } else { "○ " };
            let select_marker = if i == self.selected_item { "> " } else { "  " };
            
            let style = if i == self.selected_item {
                Style::default().fg(Color::Yellow)
            } else if is_current {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            
            Line::from(format!("{}{}{}", select_marker, prefix, label)).style(style)
        }).collect();
        
        let lang_list = Paragraph::new(lang_items);
        f.render_widget(lang_list, Rect { 
            x: area.x + 2, y: area.y + 2, 
            width: area.width - 4, height: area.height - 4 
        });
        
        // 说明文字
        let hint = Paragraph::new("按 Enter 切换语言，切换后立即生效")
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(hint, Rect { 
            x: area.x + 2, y: area.y + area.height - 3, 
            width: area.width - 4, height: 1 
        });
    }

    /// 渲染比赛设置
    fn render_match_settings(&self, f: &mut Frame, area: Rect, state: &GameState, i18n: &I18n) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(i18n.t(TranslationKey::MatchSettings));
        f.render_widget(block, area);
        
        let current_mode = &state.match_mode_preference;
        
        let items = vec![
            (i18n.t(TranslationKey::LiveText), MatchMode::Live),
            (i18n.t(TranslationKey::QuickSimulation), MatchMode::Quick),
        ];
        
        let mode_items: Vec<Line> = items.iter().enumerate().map(|(i, (label, mode))| {
            let is_current = *mode == *current_mode;
            let prefix = if is_current { "● " } else { "○ " };
            let select_marker = if i == self.selected_item { "> " } else { "  " };
            
            let style = if i == self.selected_item {
                Style::default().fg(Color::Yellow)
            } else if is_current {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };
            
            Line::from(format!("{}{}{}", select_marker, prefix, label)).style(style)
        }).collect();
        
        let mode_list = Paragraph::new(mode_items);
        f.render_widget(mode_list, Rect { 
            x: area.x + 2, y: area.y + 2, 
            width: area.width - 4, height: area.height - 4 
        });
        
        // 模式说明
        let description = match current_mode {
            MatchMode::Live => "文本直播：逐分钟显示比赛事件，可暂停、换人",
            MatchMode::Quick => "快速模拟：一次性显示比赛结果，适合批量比赛",
        };
        let desc = Paragraph::new(description)
            .style(Style::default().fg(Color::DarkGray))
            .wrap(Wrap { trim: true });
        f.render_widget(desc, Rect { 
            x: area.x + 2, y: area.y + area.height - 4, 
            width: area.width - 4, height: 2 
        });
    }
}
```

## 3. 可复用组件 (components.rs)

### 3.1 球员信息卡片

```rust
pub fn render_player_card(f: &mut Frame, area: Rect, player: &Player, i18n: &I18n) {
    let name_label = i18n.t(TranslationKey::PlayerName);
    let age_label = i18n.t(TranslationKey::Age);
    let position_label = i18n.t(TranslationKey::Position);
    let ability_label = i18n.t(TranslationKey::Ability);
    let status_label = i18n.t(TranslationKey::Status);
    
    let info = vec![
        Line::from(format!("{}: {}", name_label, player.name)),
        Line::from(format!("{}: {}", age_label, player.age)),
        Line::from(format!("{}: {:?}", position_label, player.position)),
        Line::from(format!("{}: {}", ability_label, player.current_ability)),
        Line::from(format!("{}: {:?}", status_label, player.status)),
    ];

    let paragraph = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL).title(i18n.t(TranslationKey::PlayerList)));
    f.render_widget(paragraph, area);
}
```

### 3.2 进度条

```rust
pub fn render_progress_bar(f: &mut Frame, area: Rect, value: u16, max: u16, label: &str) {
    let percentage = (value as f32 / max as f32 * 100.0) as u16;
    let filled = area.width * percentage / 100;

    let bar = Paragraph::new(format!("{}: {}%", label, percentage))
        .style(Style::default().fg(Color::Green));
    f.render_widget(bar, area);
}
```

### 3.3 确认对话框

```rust
pub fn render_confirmation_dialog(
    f: &mut Frame,
    area: Rect,
    message: &str,
    confirmed: &mut bool,
) -> bool {
    // 显示确认对话框
    // 返回用户选择
}
```

## 4. 事件处理 (event.rs)

```rust
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

pub fn read() -> Result<Event, std::io::Error> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                return Ok(Event::Key(key));
            }
        }
    }
}
```

## 5. 样式系统

```rust
pub struct Theme {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub highlight_color: Color,
    pub error_color: Color,
    pub success_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary_color: Color::Cyan,
            secondary_color: Color::Blue,
            highlight_color: Color::Yellow,
            error_color: Color::Red,
            success_color: Color::Green,
        }
    }
}
```

## 6. 语言切换

### 6.1 运行时语言切换

可以在设置界面或通过快捷键切换语言：

```rust
impl App {
    pub fn toggle_language(&mut self) {
        let new_lang = match self.i18n.current_language() {
            Language::Chinese => Language::English,
            Language::English => Language::Chinese,
        };
        self.i18n.set_language(new_lang);
        // 触发界面重绘
    }
}
```

### 6.2 语言检测

应用启动时可以从以下来源检测语言：
1. 环境变量 `LANG` 或 `LC_ALL`
2. 配置文件设置
3. 系统语言设置（如果可用）

```rust
fn detect_language() -> Language {
    // 优先检查配置文件
    if let Some(lang) = load_language_from_config() {
        return Language::from_code(&lang);
    }
    
    // 检查环境变量
    if let Ok(lang) = std::env::var("LANG") {
        if let Some(code) = lang.split('.').next() {
            return Language::from_code(code);
        }
    }
    
    // 默认英文
    Language::English
}
```

## 依赖

- `ratatui`: TUI框架
- `crossterm`: 终端操作
- `game` 模块：游戏状态
- `team` 模块：数据模型

## 7. 国际化最佳实践

### 7.1 使用翻译键

- ✅ 正确：`i18n.t(TranslationKey::TeamManagement)`
- ❌ 错误：硬编码字符串 `"球队管理"`

### 7.2 格式化字符串

对于需要动态内容的字符串，使用格式化函数：

```rust
// 使用格式化
let message = format!("{}: {} | {}: ${}", 
    i18n.t(TranslationKey::TeamInfo), 
    team.name,
    i18n.t(TranslationKey::Budget),
    team.budget
);
```

### 7.3 添加新翻译

1. 在 `TranslationKey` 枚举中添加新键
2. 在 `I18n::init_translations()` 中添加中文和英文翻译
3. 在代码中使用 `i18n.t(TranslationKey::YourKey)`

### 7.4 注意事项

- 所有用户可见的文本都应该使用翻译系统
- 日志信息可以使用英文（根据用户规则）
- 确保翻译键命名清晰且有意义
- 保持翻译文本简洁，适合 TUI 界面显示

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_rendering() {
        // 测试界面渲染（使用mock Frame）
    }
    
    #[test]
    fn test_i18n_translation() {
        let mut i18n = I18n::new(Language::Chinese);
        assert_eq!(i18n.t(TranslationKey::MainMenu), "主菜单");
        
        i18n.set_language(Language::English);
        assert_eq!(i18n.t(TranslationKey::MainMenu), "Main Menu");
    }
    
    #[test]
    fn test_language_detection() {
        assert_eq!(Language::from_code("zh"), Language::Chinese);
        assert_eq!(Language::from_code("en"), Language::English);
        assert_eq!(Language::from_code("unknown"), Language::English); // 默认
    }
}
```
