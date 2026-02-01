# Game Module Design

## 概述

Game模块负责管理游戏的整体状态和流程控制。它是连接所有其他模块的中央协调器。

## 架构

### 文件结构

```
game/
├── mod.rs          # 模块导出
├── state.rs        # 游戏状态管理
└── events.rs       # 游戏事件定义
```

## 1. 游戏状态 (state.rs)

### 1.1 GameState 结构

```rust
use crate::team::models::{Team, Player, League};
use crate::ai::match_sim::MatchResult;
use std::collections::HashMap;

pub struct GameState {
    // 基本游戏信息
    pub game_id: String,
    pub is_new_game: bool,
    pub current_date: GameDate,  // 游戏内日期

    // 玩家信息
    pub player_team_id: String,
    pub player_name: Option<String>,

    // 联赛和球队
    pub league: League,
    pub teams: HashMap<String, Team>,  // 所有球队

    // UI状态
    pub current_screen: Screen,
    pub screen_stack: Vec<Screen>,  // 用于导航历史

    // 通知系统
    pub notifications: NotificationManager,

    // 游戏配置
    pub difficulty: Difficulty,
    pub match_mode_preference: MatchMode,

    // 数据库连接（用于持久化）
    pub db_path: String,
}

/// 游戏内日期系统
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDate {
    pub year: u16,    // 如 2026
    pub month: u8,    // 1-12
    pub day: u8,      // 1-31
}

impl GameDate {
    /// 创建新日期（默认赛季开始：8月1日）
    pub fn new_season_start(year: u16) -> Self {
        Self { year, month: 8, day: 1 }
    }

    /// 推进一天
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

    /// 推进一周
    pub fn advance_week(&mut self) {
        for _ in 0..7 {
            self.advance_day();
        }
    }

    /// 推进到下一个比赛日（通常是周末）
    pub fn advance_to_next_matchday(&mut self) {
        // 简化实现：每次推进 3-4 天
        for _ in 0..3 {
            self.advance_day();
        }
    }

    /// 是否在夏季转会窗口（6月1日 - 8月31日）
    pub fn is_summer_transfer_window(&self) -> bool {
        self.month >= 6 && self.month <= 8
    }

    /// 是否在冬季转会窗口（1月1日 - 1月31日）
    pub fn is_winter_transfer_window(&self) -> bool {
        self.month == 1
    }

    /// 是否在任意转会窗口
    pub fn is_transfer_window(&self) -> bool {
        self.is_summer_transfer_window() || self.is_winter_transfer_window()
    }

    /// 是否赛季结束（5月31日）
    pub fn is_season_end(&self) -> bool {
        self.month == 5 && self.day == 31
    }

    /// 是否新赛季开始（8月1日）
    pub fn is_season_start(&self) -> bool {
        self.month == 8 && self.day == 1
    }

    /// 获取当前月份的天数
    fn days_in_current_month(&self) -> u8 {
        match self.month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => if self.is_leap_year() { 29 } else { 28 },
            _ => 30,
        }
    }

    /// 是否闰年
    fn is_leap_year(&self) -> bool {
        (self.year % 4 == 0 && self.year % 100 != 0) || (self.year % 400 == 0)
    }

    /// 获取星期几（0=周日, 1=周一, ..., 6=周六）
    /// 使用 Zeller 公式简化版
    pub fn weekday(&self) -> u8 {
        let mut y = self.year as i32;
        let mut m = self.month as i32;
        if m < 3 {
            m += 12;
            y -= 1;
        }
        let d = self.day as i32;
        let w = (d + (13 * (m + 1)) / 5 + y + y / 4 - y / 100 + y / 400) % 7;
        ((w + 6) % 7) as u8  // 调整为 0=周日
    }

    /// 格式化日期显示
    pub fn format(&self) -> String {
        format!("{}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// 格式化日期显示（带星期）
    pub fn format_with_weekday(&self) -> String {
        let weekday_names = ["周日", "周一", "周二", "周三", "周四", "周五", "周六"];
        let weekday = self.weekday() as usize;
        format!("{}年{}月{}日 {}", self.year, self.month, self.day, weekday_names[weekday])
    }

    /// 获取当前赛季字符串（如 "2026-27"）
    pub fn season_string(&self) -> String {
        if self.month >= 8 {
            format!("{}-{}", self.year, (self.year + 1) % 100)
        } else {
            format!("{}-{}", self.year - 1, self.year % 100)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    MainMenu,
    TeamManagement,
    PlayerDetail { player_id: String },  // 球员详情界面
    Tactics,
    TransferMarket,
    MatchModeSelection,
    MatchLive { match_id: String },
    MatchResult { match_id: String },
    LeagueTable,
    SeasonSummary { season: String },    // 赛季总结界面
    MatchHistory,                        // 比赛历史界面
    FinanceReport,                       // 财务报告界面
    Notifications,                       // 通知列表界面
    SaveLoad,
    Settings,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl GameState {
    pub fn new(player_team_id: String, league: League, teams: Vec<Team>, start_year: u16) -> Self {
        let mut teams_map = HashMap::new();
        for team in teams {
            teams_map.insert(team.id.clone(), team);
        }

        Self {
            game_id: generate_uuid(),
            is_new_game: true,
            current_date: GameDate::new_season_start(start_year),
            player_team_id,
            player_name: None,
            league,
            teams: teams_map,
            current_screen: Screen::MainMenu,
            screen_stack: vec![],
            notifications: NotificationManager::new(),
            difficulty: Difficulty::Normal,
            match_mode_preference: MatchMode::Quick,
            db_path: String::new(),
        }
    }

    /// 推进游戏时间到下一个比赛日
    pub fn advance_time(&mut self) {
        self.current_date.advance_to_next_matchday();
        
        // 检查赛季结束
        if self.current_date.is_season_end() {
            self.on_season_end();
        }
    }

    /// 赛季结束时的处理
    fn on_season_end(&mut self) {
        // 球员年龄增长会在 ai/progression 中处理
        // 这里可以触发赛季总结事件
    }

    pub fn get_player_team(&self) -> Option<&Team> {
        self.teams.get(&self.player_team_id)
    }

    pub fn get_team(&self, team_id: &str) -> Option<&Team> {
        self.teams.get(team_id)
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.screen_stack.push(self.current_screen.clone());
        self.current_screen = screen;
    }

    pub fn go_back(&mut self) {
        if let Some(screen) = self.screen_stack.pop() {
            self.current_screen = screen;
        }
    }
}
```

### 1.2 游戏流程控制

```rust
impl GameState {
    /// 进入下一场比赛
    pub fn advance_to_next_match(&mut self) -> Result<NextMatchInfo, GameError> {
        let player_team = self.get_player_team()
            .ok_or(GameError::TeamNotFound(self.player_team_id.clone()))?;

        // 获取当前轮次
        let current_round = self.league.current_round;

        // 查找玩家球队的下一场比赛
        let next_match = self.league.schedule.rounds
            .get(current_round as usize)
            .ok_or(GameError::NoMoreMatches)?
            .matches
            .iter()
            .find(|m| {
                m.home_team_id == self.player_team_id
                    || m.away_team_id == self.player_team_id
            })
            .ok_or(GameError::MatchNotFound)?;

        let opponent_id = if next_match.home_team_id == self.player_team_id {
            &next_match.away_team_id
        } else {
            &next_match.home_team_id
        };

        let opponent = self.get_team(opponent_id)
            .ok_or(GameError::TeamNotFound(opponent_id.clone()))?;

        Ok(NextMatchInfo {
            match_id: next_match.id.clone(),
            opponent: opponent.clone(),
            is_home: next_match.home_team_id == self.player_team_id,
            round: current_round,
        })
    }

    /// 比赛结束后更新状态
    pub fn update_after_match(
        &mut self,
        match_result: &MatchResult,
        players_repo: &dyn PlayerRepository,
    ) -> Result<(), GameError> {
        // 更新联赛轮次
        if self.is_last_match_of_round(match_result) {
            self.league.current_round += 1;
        }

        // 更新比赛状态为已打
        self.mark_match_as_played(&match_result.id)?;

        // 更新球队统计
        self.update_team_statistics(match_result)?;

        // 更新球员状态
        self.update_player_states(match_result, players_repo)?;

        // 触发随机事件
        self.trigger_random_events()?;

        Ok(())
    }
}

pub struct NextMatchInfo {
    pub match_id: String,
    pub opponent: Team,
    pub is_home: bool,
    pub round: u32,
}
```

## 2. 游戏事件 (events.rs)

### 2.1 事件定义

```rust
#[derive(Debug, Clone)]
pub enum GameEvent {
    // 玩家操作事件
    NavigateTo { screen: Screen },
    GoBack,
    QuitGame,

    // 游戏流程事件
    StartNewGame { player_team_id: String },
    LoadGame { save_id: String },
    SaveGame { slot: u8 },
    AdvanceToNextMatch,

    // 比赛相关
    SelectMatchMode { mode: MatchMode },
    StartMatch { match_id: String },
    MakeSubstitution { player_out: String, player_in: String },
    AdjustTactics { new_tactic: Tactic },

    // 转会相关
    BuyPlayer { player_id: String },
    SellPlayer { player_id: String },
    ListPlayer { player_id: String, price: u32 },

    // 设置
    ChangeDifficulty { difficulty: Difficulty },
    ChangeMatchPreference { mode: MatchMode },

    // 系统事件
    Error { message: String },
    Info { message: String },
}
```

### 2.2 事件处理

```rust
pub struct EventHandler {
    // 依赖项
}

impl EventHandler {
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
            GameEvent::AdvanceToNextMatch => {
                let next_match = state.advance_to_next_match()?;
                Ok(Effect::NavigateTo(Screen::MatchModeSelection))
            }
            // ... 其他事件处理
            GameEvent::Error { message } => {
                Ok(Effect::ShowError(message))
            }
            _ => Ok(Effect::None),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Effect {
    None,
    Render,
    NavigateTo(Screen),
    ShowError(String),
    ShowInfo(String),
    Quit,
}
```

### 2.3 通知系统

```rust
use serde::{Serialize, Deserialize};

/// 游戏通知
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub priority: Priority,
    pub read: bool,
    pub created_at: u64,           // Unix timestamp（游戏内时间）
    pub expires_at: Option<u64>,   // 可选过期时间
    pub action: Option<NotificationAction>,  // 可选的快捷操作
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NotificationType {
    Transfer,       // 转会相关（报价、完成）
    Injury,         // 伤病通知
    Contract,       // 合同到期提醒
    Match,          // 比赛提醒/结果
    Finance,        // 财务通知（预算变动）
    PlayerMorale,   // 球员士气变化
    Achievement,    // 成就解锁
    News,           // 新闻动态
    System,         // 系统消息
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Urgent = 3,
}

/// 通知关联的快捷操作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationAction {
    ViewPlayer { player_id: String },
    ViewTeam { team_id: String },
    ViewMatch { match_id: String },
    GoToScreen(Screen),
}

/// 通知管理器
pub struct NotificationManager {
    notifications: Vec<Notification>,
    max_notifications: usize,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: Vec::new(),
            max_notifications: 100,  // 最多保留 100 条
        }
    }

    /// 添加新通知
    pub fn add(&mut self, notification: Notification) {
        self.notifications.insert(0, notification);  // 新通知在前
        
        // 超过上限时删除旧通知
        if self.notifications.len() > self.max_notifications {
            self.notifications.truncate(self.max_notifications);
        }
    }

    /// 创建并添加通知（便捷方法）
    pub fn notify(
        &mut self,
        title: impl Into<String>,
        message: impl Into<String>,
        notification_type: NotificationType,
        priority: Priority,
    ) {
        let notification = Notification {
            id: generate_uuid(),
            title: title.into(),
            message: message.into(),
            notification_type,
            priority,
            read: false,
            created_at: current_timestamp(),
            expires_at: None,
            action: None,
        };
        self.add(notification);
    }

    /// 获取所有未读通知
    pub fn get_unread(&self) -> Vec<&Notification> {
        self.notifications.iter().filter(|n| !n.read).collect()
    }

    /// 获取指定类型的通知
    pub fn get_by_type(&self, t: NotificationType) -> Vec<&Notification> {
        self.notifications.iter()
            .filter(|n| n.notification_type == t)
            .collect()
    }

    /// 获取所有通知（分页）
    pub fn get_all(&self, offset: usize, limit: usize) -> Vec<&Notification> {
        self.notifications.iter()
            .skip(offset)
            .take(limit)
            .collect()
    }

    /// 标记单条通知为已读
    pub fn mark_read(&mut self, id: &str) {
        if let Some(n) = self.notifications.iter_mut().find(|n| n.id == id) {
            n.read = true;
        }
    }

    /// 标记所有通知为已读
    pub fn mark_all_read(&mut self) {
        for n in &mut self.notifications {
            n.read = true;
        }
    }

    /// 未读通知数量
    pub fn unread_count(&self) -> usize {
        self.notifications.iter().filter(|n| !n.read).count()
    }

    /// 是否有紧急通知
    pub fn has_urgent(&self) -> bool {
        self.notifications.iter()
            .any(|n| !n.read && n.priority == Priority::Urgent)
    }

    /// 清除过期通知
    pub fn clear_expired(&mut self, current_time: u64) {
        self.notifications.retain(|n| {
            n.expires_at.map_or(true, |exp| exp > current_time)
        });
    }

    /// 删除指定通知
    pub fn delete(&mut self, id: &str) {
        self.notifications.retain(|n| n.id != id);
    }
}

/// 通知工厂方法（便捷创建常用通知）
impl NotificationManager {
    /// 创建伤病通知
    pub fn notify_injury(&mut self, player_name: &str, days: u8) {
        self.notify(
            format!("球员受伤: {}", player_name),
            format!("{} 因伤将缺阵 {} 天", player_name, days),
            NotificationType::Injury,
            Priority::High,
        );
    }

    /// 创建转会报价通知
    pub fn notify_transfer_offer(&mut self, player_name: &str, from_team: &str, amount: u32) {
        self.notify(
            format!("收到转会报价: {}", player_name),
            format!("{} 对 {} 提出 ${} 的报价", from_team, player_name, amount),
            NotificationType::Transfer,
            Priority::High,
        );
    }

    /// 创建合同到期提醒
    pub fn notify_contract_expiring(&mut self, player_name: &str, months_left: u8) {
        self.notify(
            format!("合同即将到期: {}", player_name),
            format!("{} 的合同将在 {} 个月后到期", player_name, months_left),
            NotificationType::Contract,
            if months_left <= 3 { Priority::High } else { Priority::Normal },
        );
    }

    /// 创建比赛结果通知
    pub fn notify_match_result(&mut self, home: &str, away: &str, home_score: u8, away_score: u8) {
        self.notify(
            "比赛结束",
            format!("{} {} - {} {}", home, home_score, away_score, away),
            NotificationType::Match,
            Priority::Normal,
        );
    }
}
```

## 3. 游戏循环

### 3.1 主循环结构

```rust
pub struct GameLoop {
    state: GameState,
    event_handler: EventHandler,
    ui: UiRenderer,
}

impl GameLoop {
    pub fn new(state: GameState) -> Self {
        Self {
            state,
            event_handler: EventHandler::new(),
            ui: UiRenderer::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), GameError> {
        loop {
            // 1. 渲染当前界面
            self.ui.render(&self.state)?;

            // 2. 等待用户输入
            let input = self.ui.wait_for_input()?;

            // 3. 将输入转换为事件
            let event = self.convert_input_to_event(input)?;

            // 4. 处理事件
            let effect = self.event_handler.handle_event(event, &mut self.state)?;

            // 5. 应用副作用
            match effect {
                Effect::Quit => break,
                Effect::ShowError(msg) => self.ui.show_error(&msg)?,
                _ => {}
            }
        }

        Ok(())
    }
}
```

## 4. 存档系统

### 4.1 存档结构

存档元数据定义在 `data` 模块中（`SaveMetadata`），`game` 模块通过引用使用。
存档系统采用"每个存档一个独立 SQLite 文件"的方案，详见 `data/design.md`。

```rust
// 引用 data 模块的 SaveMetadata
use crate::data::SaveMetadata;

// GameState 中需要持久化的字段会保存到 game_metadata 表
// 其他数据（teams, players 等）通过 Repository 从数据库加载
```

### 4.2 存档管理

存档操作委托给 `data` 模块的 `SaveManager`：

```rust
use crate::data::{SaveManager, Database};

impl GameState {
    /// 保存当前游戏
    pub fn save(&self, slot: u8) -> Result<(), GameError> {
        let save_manager = SaveManager::new("saves")?;
        let db = Database::new(&self.db_path)?;
        save_manager.save_game(slot, self, &db)?;
        Ok(())
    }

    /// 加载游戏
    pub fn load(slot: u8) -> Result<Self, GameError> {
        let save_manager = SaveManager::new("saves")?;
        let state = save_manager.load_game(slot)?;
        Ok(state)
    }

    /// 列出所有存档
    pub fn list_saves() -> Result<Vec<crate::data::SaveMetadata>, GameError> {
        let save_manager = SaveManager::new("saves")?;
        let saves = save_manager.list_saves()?;
        Ok(saves)
    }

    /// 删除存档
    pub fn delete_save(slot: u8) -> Result<(), GameError> {
        let save_manager = SaveManager::new("saves")?;
        save_manager.delete_save(slot)?;
        Ok(())
    }
}
```

**注意**: `SaveMetadata` 定义在 `data` 模块中，`game` 模块通过引用使用。

## 5. 游戏初始化

### 5.1 新游戏流程

```rust
pub async fn start_new_game(
    db_path: String,
    player_team_name: String,
) -> Result<GameState, GameError> {
    // 1. 初始化数据库
    let db = Database::new(&db_path)?;
    db.run_migrations().await?;

    // 2. 生成游戏数据
    let league = ai::generator::generate_league();
    let teams = generate_teams_for_league(league.id.clone()).await?;

    // 3. 找到玩家选择的球队
    let player_team = teams
        .iter()
        .find(|t| t.name == player_team_name)
        .ok_or(GameError::TeamNotFound(player_team_name))?;

    // 4. 保存到数据库
    for team in &teams {
        db.team_repo().create(team)?;
    }
    db.league_repo().create(&league)?;

    // 5. 创建游戏状态
    let mut state = GameState::new(player_team.id.clone(), league, teams);
    state.db_path = db_path;

    Ok(state)
}

async fn generate_teams_for_league(
    league_id: String,
) -> Result<Vec<Team>, GameError> {
    let mut teams = vec![];

    // 生成20支球队
    for i in 0..20 {
        let style = pick_random_team_style();
        let mut team = ai::generator::generate_team(style, league_id.clone());
        let players = ai::generator::generate_players_for_team(
            team.id.clone(),
            100,
            25,
        );

        // 保存球员
        for player in &players {
            db.player_repo().create(player)?;
        }

        team.players = players.iter().map(|p| p.id.clone()).collect();
        teams.push(team);
    }

    Ok(teams)
}
```

## 6. 难度设置

### 6.1 难度影响

```rust
impl Difficulty {
    pub fn budget_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.5,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 0.7,
        }
    }

    pub fn ai_intelligence(&self) -> u8 {
        match self {
            Difficulty::Easy => 50,
            Difficulty::Normal => 100,
            Difficulty::Hard => 150,
        }
    }
}
```

## 7. 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum GameError {
    #[error("球队不存在: {0}")]
    TeamNotFound(String),

    #[error("球员不存在: {0}")]
    PlayerNotFound(String),

    #[error("没有更多比赛")]
    NoMoreMatches,

    #[error("比赛不存在")]
    MatchNotFound,

    #[error("联赛不存在")]
    LeagueNotFound,

    #[error("无效操作: {0}")]
    InvalidOperation(String),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] crate::data::DatabaseError),

    #[error("转会错误: {0}")]
    TransferError(#[from] crate::transfer::TransferError),

    #[error("存档错误: {0}")]
    SaveError(#[from] crate::data::SaveError),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}
```

## 依赖

- `team` 模块：数据模型
- `data` 模块：数据持久化
- `ai` 模块：数据生成和模拟
- `transfer` 模块：转会市场
- `ui` 模块：用户界面

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_creation() {
        let state = GameState::new("team1".to_string(), league, teams);
        assert_eq!(state.current_screen, Screen::MainMenu);
    }

    #[test]
    fn test_navigation() {
        let mut state = create_test_state();
        state.navigate_to(Screen::TeamManagement);
        assert_eq!(state.current_screen, Screen::TeamManagement);
        state.go_back();
        assert_eq!(state.current_screen, Screen::MainMenu);
    }
}
```
