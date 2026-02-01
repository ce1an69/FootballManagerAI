# Team Module Design

## 概述

Team模块负责定义游戏中的核心数据模型：球队、球员、战术等。这些数据结构被整个应用程序使用。

## 架构

### 文件结构

```
team/
├── mod.rs          # 模块导出
├── player.rs       # Player 数据模型
├── team.rs         # Team 数据模型
├── league.rs       # League 数据模型
├── match_result.rs # MatchResult 数据模型
├── tactics.rs      # 战术系统（阵型、角色、职责）
└── attributes.rs   # 球员属性定义和计算
```

### 模块导出 (mod.rs)

```rust
// team/mod.rs
mod player;
mod team;
mod league;
mod match_result;
mod tactics;
mod attributes;

// Re-export all public types
pub use player::{Player, Position, Foot, PlayerStatus};
pub use team::{Team, PlayerSlot, TeamStatistics};
pub use league::{League, MatchSchedule, Round, ScheduledMatch};
pub use match_result::{MatchResult, MatchEvent, MatchMode};
pub use tactics::{Tactic, Formation, DefensiveHeight, PassingStyle, Tempo, PlayerRole, Duty, PlayerRoleAssignment};
pub use attributes::{calculate_market_value, calculate_wage};
```

这样其他模块可以通过 `crate::team::Player` 或 `crate::team::*` 导入所需类型。

### 模块依赖关系

每个核心数据模型都维护在单独的文件中，便于独立开发和维护：

- **player.rs**: 独立的 Player 模型，包含 Position、Foot、PlayerStatus 等枚举
- **team.rs**: Team 模型，依赖 `player::Position` 和 `tactics::*`
- **league.rs**: League 模型，独立的数据结构
- **match_result.rs**: MatchResult 模型，依赖 `team::Team`（通过 ID 引用）
- **tactics.rs**: 战术系统，依赖 `player::Position`

**依赖图**:
```
player.rs (基础)
  ↑
  ├── team.rs
  ├── tactics.rs
  └── match_result.rs (间接依赖)
```

**文件组织原则**:
- 每个核心数据模型独立维护在一个文件中，便于：
  - 独立开发和测试
  - 清晰的职责划分
  - 减少文件冲突
  - 更好的代码可维护性
- 每个文件包含：
  - 该模型的结构体定义
  - 相关的枚举类型
  - 该模型的实现方法
  - 相关的辅助函数（如果只属于该模型）

## 核心数据模型

### 1. Player (player.rs)

球员的完整数据模型。

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    // 唯一标识
    pub id: String,
    pub team_id: Option<String>,

    // 基本信息
    pub name: String,
    pub age: u8,
    pub nationality: String,
    pub position: Position,
    pub second_positions: Vec<Position>,
    pub preferred_foot: Foot,
    pub height: u8,  // cm
    pub weight: u8,  // kg

    // 技术属性 (0-200)
    pub corners: u16,
    pub crossing: u16,
    pub dribbling: u16,
    pub finishing: u16,
    pub heading: u16,
    pub long_shots: u16,
    pub long_throws: u16,
    pub marking: u16,
    pub passing: u16,
    pub penalties: u16,
    pub tackling: u16,
    pub technique: u16,

    // 精神属性 (0-200)
    pub aggression: u16,
    pub anticipation: u16,
    pub bravery: u16,
    pub creativity: u16,
    pub decisions: u16,
    pub concentration: u16,
    pub positioning: u16,
    pub off_the_ball: u16,
    pub work_rate: u16,
    pub pressure: u16,
    pub teamwork: u16,
    pub vision: u16,

    // 身体属性 (0-200)
    pub acceleration: u16,
    pub agility: u16,
    pub balance: u16,
    pub pace: u16,
    pub stamina: u16,
    pub strength: u16,

    // 门将属性 (0-200)
    pub aerial_reach: u16,
    pub command_of_area: u16,
    pub communication: u16,
    pub eccentricity: u16,
    pub handling: u16,
    pub kicking: u16,
    pub throwing: u16,
    pub reflexes: u16,
    pub rushing_out: u16,
    pub gk_positioning: u16,

    // 隐藏属性
    pub potential_ability: u16,
    pub current_ability: u16,
    pub adaptability: u16,
    pub ambition: u16,
    pub professionalism: u16,
    pub loyalty: u16,
    pub injury_proneness: u16,
    pub controversy: u16,

    // 状态
    pub match_fitness: u8,      // 0-100
    pub morale: u8,             // 0-100
    pub status: PlayerStatus,
    pub injury_days: Option<u8>,
    pub fatigue: u8,            // 0-100

    // 合同
    pub wage: u32,              // 周薪
    pub contract_years: u8,
    pub market_value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Position {
    GK,
    CB, LB, RB, WB,
    DM,
    CM, AM,
    LW, RW,
    ST, CF,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Foot {
    Left,
    Right,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerStatus {
    Healthy,
    Injured,
    Fatigued,
    Suspended,
}
```

**Player 方法**:
```rust
impl Player {
    pub fn new(id: String, name: String, position: Position) -> Self;
    pub fn age_player(&mut self);  // 增加年龄，更新能力
    pub fn recover_fatigue(&mut self, amount: u8);
    pub fn injure(&mut self, days: u8);
    pub fn heal(&mut self);
    pub fn calculate_overall_ability(&self) -> u16;
    pub fn get_position_rating(&self, position: &Position) -> u16;
    pub fn is_gk(&self) -> bool;
    pub fn can_play_position(&self, position: &Position) -> bool;
}
```

### 2. Team (team.rs)

球队数据模型。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub league_id: String,
    pub budget: u32,
    pub players: Vec<String>,  // Player IDs
    pub starting_11: Vec<PlayerSlot>,
    pub bench: Vec<PlayerSlot>,
    pub tactic: Tactic,
    pub statistics: TeamStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSlot {
    pub player_id: String,
    pub position: Position,
    pub role: PlayerRole,
    pub duty: Duty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStatistics {
    pub matches_played: u32,
    pub wins: u32,
    pub draws: u32,
    pub losses: u32,
    pub goals_for: u32,
    pub goals_against: u32,
    pub points: u32,
    pub league_position: Option<u32>,
}
```

**Team 方法**:
```rust
impl Team {
    pub fn new(id: String, name: String, league_id: String, budget: u32) -> Self;
    pub fn add_player(&mut self, player_id: String);
    pub fn remove_player(&mut self, player_id: &str);
    pub fn set_starting_11(&mut self, players: Vec<PlayerSlot>);
    pub fn get_team_strength(&self, players: &[Player]) -> u16;
    pub fn calculate_attack_strength(&self, players: &[Player]) -> u16;
    pub fn calculate_defense_strength(&self, players: &[Player]) -> u16;
    pub fn calculate_midfield_strength(&self, players: &[Player]) -> u16;
    pub fn update_statistics(&mut self, won: bool, drawn: bool, goals_for: u32, goals_against: u32);
}
```

### 3. League (league.rs)

联赛数据模型。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct League {
    pub id: String,
    pub name: String,
    pub current_round: u32,
    pub total_rounds: u32,
    pub teams: Vec<String>,  // Team IDs
    pub schedule: MatchSchedule,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSchedule {
    pub rounds: Vec<Round>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Round {
    pub round_number: u32,
    pub matches: Vec<ScheduledMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMatch {
    pub id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub played: bool,
}
```

### 5. Tactic (tactics.rs)

战术系统。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tactic {
    pub formation: Formation,
    pub attacking_mentality: u8,  // 0-100
    pub defensive_height: DefensiveHeight,
    pub passing_style: PassingStyle,
    pub tempo: Tempo,
    pub player_roles: Vec<PlayerRoleAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Formation {
    FourFourTwo,
    FourThreeThree,
    FourTwoThreeOne,
    FourOneFourOne,
    FourFiveOne,
    ThreeFiveTwo,
    ThreeFourTwoOne,
    FiveThreeTwo,
    FiveTwoTwoOne,
}

impl Formation {
    pub fn positions(&self) -> Vec<Position>;
    pub fn name(&self) -> &str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DefensiveHeight {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PassingStyle {
    Short,
    Mixed,
    Long,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tempo {
    Slow,
    Medium,
    Fast,
}

impl Tactic {
    /// 获取战术风格描述（用于 UI 显示）
    pub fn style_description(&self) -> &str {
        // 战术风格由 ai 模块的 infer_tactical_style 计算
        // 这里提供简单的描述
        if matches!(self.defensive_height, DefensiveHeight::High) && self.attacking_mentality > 70 {
            "高位逼抢"
        } else if matches!(self.defensive_height, DefensiveHeight::Low) 
            && matches!(self.tempo, Tempo::Fast) {
            "防守反击"
        } else if matches!(self.passing_style, PassingStyle::Short) {
            "控球为主"
        } else if matches!(self.passing_style, PassingStyle::Long) {
            "长传冲吊"
        } else {
            "平衡战术"
        }
    }
    
    /// 获取战术强度评估（0-100）
    pub fn intensity(&self) -> u8 {
        let press_intensity = match self.defensive_height {
            DefensiveHeight::High => 30,
            DefensiveHeight::Medium => 20,
            DefensiveHeight::Low => 10,
        };
        let tempo_intensity = match self.tempo {
            Tempo::Fast => 30,
            Tempo::Medium => 20,
            Tempo::Slow => 10,
        };
        let mentality_intensity = self.attacking_mentality / 3;
        
        (press_intensity + tempo_intensity + mentality_intensity).min(100)
    }
}
```

### 6. Player Roles (tactics.rs)

球员角色系统（参考FM）。

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlayerRole {
    // 前锋角色
    AdvancedForward,
    CompleteForward,
    DeepLyingForward,
    TargetMan,
    Poacher,
    FalseNine,
    Trequartista,

    // 中场角色
    BoxToBox,
    CentralMidfielder,
    BallWinningMidfielder,
    DefensiveMidfielder,
    DeepLyingPlaymaker,
    Regista,
    AdvancedPlaymaker,
    WideMidfielder,
    Winger,
    InsideForward,
    WideTargetMan,

    // 后卫角色
    FullBack,
    WingBack,
    CompleteWingBack,
    InvertedWingBack,
    CentralDefender,
    BallPlayingDefender,
    Libero,
    NoNonsenseDefender,
    WideCentreBack,

    // 门将角色
    Goalkeeper,
    SweeperKeeper,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Duty {
    Attack,
    Support,
    Defend,
    Stopper,
    Cover,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRoleAssignment {
    pub position_index: usize,  // 在阵型中的位置索引
    pub role: PlayerRole,
    pub duty: Duty,
}
```

**PlayerRole 方法**:
```rust
impl PlayerRole {
    pub fn get_required_attributes(&self) -> Vec<&str>;
    pub fn get_description(&self) -> &str;
    pub fn is_suitable_for_position(&self, position: &Position) -> bool;
}
```

### 4. Match Result (match_result.rs)

比赛结果数据模型。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchResult {
    pub id: String,
    pub league_id: String,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_score: u8,
    pub away_score: u8,
    pub match_mode: MatchMode,
    pub events: Vec<MatchEvent>,
    pub statistics: MatchStatistics,  // 详细统计
    pub played_at: u64,
    pub round: u32,
}

/// 比赛详细统计
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MatchStatistics {
    // 控球
    pub home_possession: f32,
    pub away_possession: f32,
    
    // 射门
    pub home_shots: u16,
    pub away_shots: u16,
    pub home_shots_on_target: u16,
    pub away_shots_on_target: u16,
    
    // 传球
    pub home_passes: u16,
    pub away_passes: u16,
    pub home_pass_accuracy: f32,     // 传球成功率 (0-100)
    pub away_pass_accuracy: f32,
    
    // 其他
    pub home_corners: u8,
    pub away_corners: u8,
    pub home_fouls: u8,
    pub away_fouls: u8,
    pub home_offsides: u8,
    pub away_offsides: u8,
    
    // 球员评分
    pub home_player_ratings: Vec<PlayerMatchRating>,
    pub away_player_ratings: Vec<PlayerMatchRating>,
}

/// 球员单场评分
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMatchRating {
    pub player_id: String,
    pub player_name: String,
    pub position: Position,
    pub rating: f32,           // 6.0 - 10.0
    pub minutes_played: u8,
    pub goals: u8,
    pub assists: u8,
}

impl MatchStatistics {
    /// 获取主队射正率
    pub fn home_shot_accuracy(&self) -> f32 {
        if self.home_shots == 0 { 0.0 } 
        else { (self.home_shots_on_target as f32 / self.home_shots as f32) * 100.0 }
    }
    
    /// 获取客队射正率
    pub fn away_shot_accuracy(&self) -> f32 {
        if self.away_shots == 0 { 0.0 } 
        else { (self.away_shots_on_target as f32 / self.away_shots as f32) * 100.0 }
    }
    
    /// 获取全场最佳球员
    pub fn man_of_the_match(&self) -> Option<&PlayerMatchRating> {
        self.home_player_ratings.iter()
            .chain(self.away_player_ratings.iter())
            .max_by(|a, b| a.rating.partial_cmp(&b.rating).unwrap())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchMode {
    Live,
    Quick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatchEvent {
    Goal { team: String, player_id: String, minute: u8 },
    OwnGoal { team: String, player_id: String, minute: u8 },
    Penalty { team: String, player_id: String, minute: u8, scored: bool },
    YellowCard { team: String, player_id: String, minute: u8 },
    RedCard { team: String, player_id: String, minute: u8 },
    Injury { team: String, player_id: String, minute: u8, severity: u8 },
    Substitution { team: String, player_out: String, player_in: String, minute: u8 },
}
```

### 5. 球员赛季统计 (player_stats.rs)

记录球员在单赛季的表现数据。

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSeasonStats {
    pub player_id: String,
    pub season: String,          // 如 "2026-27"
    pub team_id: String,
    
    // 出场数据
    pub appearances: u32,        // 出场次数
    pub starts: u32,             // 首发次数
    pub minutes_played: u32,     // 上场时间（分钟）
    
    // 进攻数据
    pub goals: u32,              // 进球数
    pub assists: u32,            // 助攻数
    pub shots: u32,              // 射门次数
    pub shots_on_target: u32,    // 射正次数
    
    // 纪律数据
    pub yellow_cards: u32,       // 黄牌数
    pub red_cards: u32,          // 红牌数
    
    // 评分
    pub total_rating: f32,       // 累计评分
    pub average_rating: f32,     // 平均评分
    
    // 最佳表现
    pub man_of_the_match: u32,   // 全场最佳次数
}

impl PlayerSeasonStats {
    pub fn new(player_id: String, season: String, team_id: String) -> Self {
        Self {
            player_id,
            season,
            team_id,
            ..Default::default()
        }
    }
    
    /// 记录一场比赛的数据
    pub fn record_match(
        &mut self,
        minutes: u32,
        started: bool,
        goals: u32,
        assists: u32,
        yellow: bool,
        red: bool,
        rating: f32,
    ) {
        self.appearances += 1;
        if started {
            self.starts += 1;
        }
        self.minutes_played += minutes;
        self.goals += goals;
        self.assists += assists;
        if yellow {
            self.yellow_cards += 1;
        }
        if red {
            self.red_cards += 1;
        }
        self.total_rating += rating;
        self.average_rating = self.total_rating / self.appearances as f32;
    }
    
    /// 计算进球率（每90分钟进球数）
    pub fn goals_per_90(&self) -> f32 {
        if self.minutes_played == 0 {
            return 0.0;
        }
        (self.goals as f32 / self.minutes_played as f32) * 90.0
    }
    
    /// 计算助攻率（每90分钟助攻数）
    pub fn assists_per_90(&self) -> f32 {
        if self.minutes_played == 0 {
            return 0.0;
        }
        (self.assists as f32 / self.minutes_played as f32) * 90.0
    }
}
```

### 6. 球队财务 (finance.rs)

记录球队的财务状况。

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamFinance {
    pub team_id: String,
    pub balance: i64,              // 当前余额（可为负）
    pub wage_budget: i64,          // 薪资预算
    pub transfer_budget: i64,      // 转会预算
}

/// 财务交易记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinanceTransaction {
    pub id: String,
    pub team_id: String,
    pub transaction_type: TransactionType,
    pub amount: i64,               // 正数为收入，负数为支出
    pub description: String,
    pub date: GameDate,
    pub related_player_id: Option<String>,  // 关联球员（如转会）
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionType {
    // 收入
    TransferIncome,     // 转会收入
    MatchdayIncome,     // 比赛日收入
    Sponsorship,        // 赞助
    TVRevenue,          // 电视转播
    PrizeMoneyWin,      // 奖金
    
    // 支出
    TransferExpense,    // 转会支出
    WagePayment,        // 薪资支出
    BonusPayment,       // 奖金支出
    StaffWages,         // 教练组薪资
    Facilities,         // 设施维护
    YouthAcademy,       // 青训投入
}

impl TransactionType {
    pub fn is_income(&self) -> bool {
        matches!(self, 
            TransactionType::TransferIncome |
            TransactionType::MatchdayIncome |
            TransactionType::Sponsorship |
            TransactionType::TVRevenue |
            TransactionType::PrizeMoneyWin
        )
    }
}

/// 赛季财务报告
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeasonFinanceReport {
    pub season: String,
    pub team_id: String,
    
    // 收入
    pub transfer_income: i64,
    pub matchday_income: i64,
    pub sponsorship: i64,
    pub tv_revenue: i64,
    pub prize_money: i64,
    
    // 支出
    pub transfer_expenses: i64,
    pub wage_expenses: i64,
    pub bonus_expenses: i64,
    pub staff_wages: i64,
    pub facilities: i64,
    pub youth_academy: i64,
}

impl SeasonFinanceReport {
    pub fn total_income(&self) -> i64 {
        self.transfer_income + self.matchday_income + self.sponsorship 
            + self.tv_revenue + self.prize_money
    }
    
    pub fn total_expenses(&self) -> i64 {
        self.transfer_expenses + self.wage_expenses + self.bonus_expenses 
            + self.staff_wages + self.facilities + self.youth_academy
    }
    
    pub fn net_balance(&self) -> i64 {
        self.total_income() - self.total_expenses()
    }
    
    /// 记录一笔交易
    pub fn record(&mut self, transaction: &FinanceTransaction) {
        match transaction.transaction_type {
            TransactionType::TransferIncome => self.transfer_income += transaction.amount,
            TransactionType::MatchdayIncome => self.matchday_income += transaction.amount,
            TransactionType::Sponsorship => self.sponsorship += transaction.amount,
            TransactionType::TVRevenue => self.tv_revenue += transaction.amount,
            TransactionType::PrizeMoneyWin => self.prize_money += transaction.amount,
            TransactionType::TransferExpense => self.transfer_expenses += transaction.amount.abs(),
            TransactionType::WagePayment => self.wage_expenses += transaction.amount.abs(),
            TransactionType::BonusPayment => self.bonus_expenses += transaction.amount.abs(),
            TransactionType::StaffWages => self.staff_wages += transaction.amount.abs(),
            TransactionType::Facilities => self.facilities += transaction.amount.abs(),
            TransactionType::YouthAcademy => self.youth_academy += transaction.amount.abs(),
        }
    }
}

impl TeamFinance {
    pub fn new(team_id: String, initial_balance: i64) -> Self {
        Self {
            team_id,
            balance: initial_balance,
            wage_budget: initial_balance / 2,
            transfer_budget: initial_balance / 2,
        }
    }
    
    /// 检查是否有足够资金
    pub fn can_afford(&self, amount: i64) -> bool {
        self.balance >= amount
    }
    
    /// 执行交易
    pub fn process_transaction(&mut self, amount: i64) -> bool {
        if amount < 0 && !self.can_afford(amount.abs()) {
            return false;
        }
        self.balance += amount;
        true
    }
    
    /// 计算每周薪资支出
    pub fn weekly_wage_bill(&self, players: &[Player]) -> i64 {
        players.iter()
            .filter(|p| p.team_id.as_ref() == Some(&self.team_id))
            .map(|p| p.wage as i64)
            .sum()
    }
}
```

## 数据计算

### 球员能力计算

```rust
impl Player {
    /// 计算球员在该位置的综合评分
    pub fn get_position_rating(&self, position: &Position) -> u16 {
        match position {
            Position::ST => {
                (self.finishing * 3 +
                 self.off_the_ball * 2 +
                 self.pace +
                 self.heading +
                 self.technique) / 8
            }
            Position::CM => {
                (self.passing * 2 +
                 self.vision * 2 +
                 self.stamina +
                 self.teamwork +
                 self.decisions +
                 self.technique) / 8
            }
            // ... 其他位置
        }
    }

    /// 计算当前整体能力
    pub fn calculate_overall_ability(&self) -> u16 {
        // 根据位置计算相关属性平均值
        let relevant_attrs = self.get_relevant_attributes();
        relevant_attrs.iter().sum::<u16>() / relevant_attrs.len() as u16
    }
}
```

### 球队实力计算

```rust
impl Team {
    pub fn get_team_strength(&self, players: &[Player]) -> u16 {
        let starting: Vec<&Player> = self.starting_11.iter()
            .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
            .collect();

        let total: u16 = starting.iter()
            .map(|p| p.calculate_overall_ability())
            .sum();

        total / 11
    }

    pub fn calculate_attack_strength(&self, players: &[Player]) -> u16 {
        let attackers: Vec<&Player> = self.starting_11.iter()
            .filter(|slot| matches!(slot.position, Position::ST | Position::CF | Position::LW | Position::RW))
            .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
            .collect();

        if attackers.is_empty() {
            return 0;
        }

        attackers.iter()
            .map(|p| p.get_position_rating(&p.position))
            .sum::<u16>() / attackers.len() as u16
    }

    pub fn calculate_defense_strength(&self, players: &[Player]) -> u16 {
        let defenders: Vec<&Player> = self.starting_11.iter()
            .filter(|slot| matches!(slot.position, Position::GK | Position::CB | Position::LB | Position::RB | Position::WB))
            .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
            .collect();

        if defenders.is_empty() {
            return 0;
        }

        defenders.iter()
            .map(|p| p.get_position_rating(&p.position))
            .sum::<u16>() / defenders.len() as u16
    }

    pub fn calculate_midfield_strength(&self, players: &[Player]) -> u16 {
        let midfielders: Vec<&Player> = self.starting_11.iter()
            .filter(|slot| matches!(slot.position, Position::DM | Position::CM | Position::AM))
            .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
            .collect();

        if midfielders.is_empty() {
            return 0;
        }

        midfielders.iter()
            .map(|p| p.get_position_rating(&p.position))
            .sum::<u16>() / midfielders.len() as u16
    }
}
```

## 辅助函数

### 属性辅助 (attributes.rs)

```rust
pub fn calculate_market_value(player: &Player) -> u32 {
    let base = player.current_ability as u32 * 1000;

    let age_modifier = match player.age {
        age if age <= 21 => 1.5,
        age if age <= 28 => 1.0,
        age if age <= 33 => 0.7,
        _ => 0.4,
    };

    let potential_bonus = (player.potential_ability - player.current_ability) as u32 * 500;

    (base as f32 * age_modifier) as u32 + potential_bonus
}

pub fn calculate_wage(player: &Player) -> u32 {
    player.current_ability as u32 * 50 + player.age as u32 * 20
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
        assert_eq!(player.name, "Test");
        assert_eq!(player.position, Position::ST);
    }

    #[test]
    fn test_team_strength_calculation() {
        // 测试球队实力计算
    }
}
```

## Serde 支持

所有主要结构都实现 `Serialize` 和 `Deserialize`，以便：
- 存储到数据库（JSON格式）
- 存档系统
- 网络传输（未来可能的多人模式）
