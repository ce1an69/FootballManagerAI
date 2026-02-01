# AI Module Design

## 概述

AI模块是游戏的核心模拟引擎，负责：
1. 游戏开始时生成随机队伍/球员/联赛
2. 比赛中向后推进更新球员数据
3. 模拟比赛过程/结束
4. 随机生成事件

## 架构

### 文件结构

```
ai/
├── mod.rs          # 模块导出
├── generator.rs    # 数据生成（联赛、球队、球员）
├── progression.rs  # 球员数据更新（成长、衰老、状态变化）
├── match_sim.rs    # 比赛模拟核心逻辑
└── events.rs       # 随机事件生成
```

## 1. 数据生成器 (generator.rs)

### 职责

在开始新游戏时生成所有初始数据。

### 核心功能

#### 1.1 生成联赛

```rust
pub fn generate_league() -> League {
    League {
        id: generate_uuid(),
        name: "National League".to_string(),
        current_round: 0,
        total_rounds: 38,  // 20队双循环
        teams: vec![],     // 稍后填充
        schedule: MatchSchedule { rounds: vec![] },
    }
}
```

#### 1.2 生成球队

```rust
pub enum TeamStyle {
    Balanced,
    Attacking,
    Defending,
    CounterAttack,
    YouthDevelopment,
}

pub fn generate_team(style: TeamStyle, league_id: String) -> Team {
    let name = generate_team_name();
    let budget = match style {
        TeamStyle::YouthDevelopment => 5_000_000..15_000_000,
        TeamStyle::Balanced => 10_000_000..30_000_000,
        TeamStyle::Attacking => 20_000_000..50_000_000,
        TeamStyle::Defending => 15_000_000..35_000_000,
        TeamStyle::CounterAttack => 10_000_000..25_000_000,
    }.pick_random();

    Team {
        id: generate_uuid(),
        name,
        league_id,
        budget,
        players: vec![],
        starting_11: vec![],
        bench: vec![],
        tactic: generate_tactic_for_style(style),
        statistics: TeamStatistics::default(),
    }
}
```

#### 1.3 生成球员

```rust
pub fn generate_players_for_team(
    team_id: String,
    team_level: u16,  // 50-200，球队整体实力水平
    count: usize,
) -> Vec<Player> {
    let mut players = Vec::new();

    // 生成门将 (2-3个)
    players.extend(generate_goalkeepers(team_id.clone(), team_level, 3));

    // 生成后卫 (6-8个)
    players.extend(generate_defenders(team_id.clone(), team_level, 7));

    // 生成中场 (6-8个)
    players.extend(generate_midfielders(team_id.clone(), team_level, 7));

    // 生成前锋 (4-6个)
    players.extend(generate_forwards(team_id.clone(), team_level, 5));

    players
}

fn generate_player(
    team_id: String,
    position: Position,
    skill_level: u16,
    age_range: Range<u8>,
) -> Player {
    let age = age_range.pick_random();
    let potential = generate_potential(skill_level, age);

    // 根据位置生成关键属性
    let mut rng = rand::thread_rng();
    let mut player = Player::new(
        generate_uuid(),
        generate_name(),
        position,
    );
    player.team_id = Some(team_id);
    player.age = age;
    player.potential_ability = potential;
    player.current_ability = skill_level;

    // 根据位置生成特定属性
    set_position_attributes(&mut player, position, skill_level);
    set_random_variation(&mut player);  // 增加随机性

    // 生成合约
    player.wage = calculate_wage(&player);
    player.contract_years = rng.gen_range(1..5);
    player.market_value = calculate_market_value(&player);

    player
}
```

#### 1.4 生成赛程

```rust
pub fn generate_schedule(teams: Vec<String>) -> MatchSchedule {
    let n = teams.len();
    let total_rounds = (n - 1) * 2;  // 双循环
    let mut rounds = vec![];

    for round in 0..total_rounds {
        let mut round_matches = vec![];
        let half = n / 2;

        for i in 0..half {
            let home = if round % 2 == 0 { i } else { (n - 1) - i };
            let away = if round % 2 == 0 { (n - 1) - i } else { i };

            round_matches.push(ScheduledMatch {
                id: generate_uuid(),
                home_team_id: teams[home].clone(),
                away_team_id: teams[away].clone(),
                played: false,
            });
        }

        rounds.push(Round {
            round_number: round as u32,
            matches: round_matches,
        });
    }

    MatchSchedule { rounds }
}
```

#### 1.5 随机名称生成

```rust
static FIRST_NAMES: &[&str] = &[
    "James", "John", "Robert", "Michael", "William", "David", "Richard", "Joseph",
    "Thomas", "Charles", "Christopher", "Daniel", "Matthew", "Anthony", "Mark",
    // ... 更多名字
];

static LAST_NAMES: &[&str] = &[
    "Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis",
    "Rodriguez", "Martinez", "Hernandez", "Lopez", "Gonzalez", "Wilson", "Anderson",
    // ... 更多姓氏
];

static NATIONALITIES: &[&str] = &[
    "England", "Spain", "France", "Germany", "Italy", "Brazil", "Argentina",
    "Portugal", "Netherlands", "Belgium",
];

fn generate_name() -> String {
    let first = FIRST_NAMES.choose().unwrap();
    let last = LAST_NAMES.choose().unwrap();
    format!("{} {}", first, last)
}
```

## 2. 进度系统 (progression.rs)

### 职责

随着游戏进行更新球员数据和状态。

### 核心功能

#### 2.1 比赛后更新

```rust
pub fn update_players_after_match(
    players: &mut [Player],
    minutes_played: HashMap<String, u32>,
) {
    for player in players.iter_mut() {
        if let Some(mins) = minutes_played.get(&player.id) {
            // 增加疲劳
            let fatigue_gain = match mins {
                0..=45 => 10,
                46..=70 => 20,
                71..=90 => 30,
                _ => 35,
            };
            player.fatigue = (player.fatigue + fatigue_gain).min(100);

            // 可能受伤
            if rand::thread_rng().gen_bool(0.03) {
                injure_player(player);
            }

            // 胜利提升士气
            if minutes_played.get(&player.id).is_some() {
                player.morale = (player.morale + 5).min(100);
            }
        } else {
            // 未上场：恢复疲劳，可能降低士气
            player.fatigue = (player.fatigue.saturating_sub(20)).max(0);
            if player.age < 23 {
                player.morale = (player.morale.saturating_sub(3)).max(0);
            }
        }

        // 恢复体能
        player.match_fitness = (player.match_fitness - 5).max(50);
    }
}
```

#### 2.2 休息时更新

```rust
pub fn update_players_during_break(players: &mut [Player], days: u32) {
    for player in players.iter_mut() {
        // 恢复疲劳
        let recovery = (days * 10).min(100);
        player.fatigue = player.fatigue.saturating_sub(recovery as u8);

        // 恢复体能
        player.match_fitness = (player.match_fitness + days as u8 * 2).min(100);

        // 伤病恢复
        if let Some(injury_days) = player.injury_days {
            if injury_days <= days as u8 {
                player.heal();
            } else {
                player.injury_days = Some(injury_days - days as u8);
            }
        }
    }
}
```

#### 2.3 年龄增长

```rust
pub fn age_players(players: &mut [Player]) {
    for player in players.iter_mut() {
        player.age += 1;

        // 能力变化
        match player.age {
            16..=21 => {
                // 成长期：能力可能提升
                if player.current_ability < player.potential_ability {
                    let growth = rand::thread_rng().gen_range(1..=5);
                    player.current_ability = (player.current_ability + growth).min(player.potential_ability);
                }
            }
            22..=28 => {
                // 巅峰期：小幅波动
                let change = rand::thread_rng().gen_range(-2..=2);
                player.current_ability = (player.current_ability as i16 + change).max(0).min(200) as u16;
            }
            29..=33 => {
                // 衰退初期
                let decline = rand::thread_rng().gen_range(1..=3);
                player.current_ability = player.current_ability.saturating_sub(decline);
            }
            _ => {
                // 快速衰退
                let decline = rand::thread_rng().gen_range(3..=8);
                player.current_ability = player.current_ability.saturating_sub(decline);
            }
        }

        // 身体属性随年龄下降
        if player.age >= 30 {
            player.pace = player.pace.saturating_sub(2);
            player.acceleration = player.acceleration.saturating_sub(2);
            player.stamina = player.stamina.saturating_sub(1);
        }
    }
}
```

## 3. 比赛模拟器 (match_sim.rs)

### 职责

模拟比赛过程和结果。

### 核心功能

#### 3.0 战术克制系统

```rust
/// 战术风格（从战术设置推断）
#[derive(Debug, Clone, PartialEq)]
pub enum TacticalStyle {
    Possession,      // 控球战术：短传 + 慢节奏
    CounterAttack,   // 反击战术：长传 + 快节奏 + 低防线
    HighPress,       // 高位逼抢：高防线 + 高进攻心态
    DirectPlay,      // 长传冲吊：长传为主
    Balanced,        // 平衡战术：默认
}

/// 战术克制关系图
/// 
/// ```
///     HighPress
///       ↓ 克制
///   Possession
///       ↓ 克制
///  CounterAttack
///       ↓ 克制
///     HighPress (形成循环)
/// 
/// DirectPlay → 克制 HighPress（绕过中场）
/// ```

/// 计算战术克制修正值
/// 
/// # 返回值
/// - 正值：主队战术占优
/// - 负值：客队战术占优
/// - 范围：-0.25 到 +0.25
pub fn calculate_tactical_modifier(
    home_style: &TacticalStyle,
    away_style: &TacticalStyle,
) -> f64 {
    use TacticalStyle::*;
    
    match (home_style, away_style) {
        // 高位逼抢克制控球（破坏传球节奏）
        (HighPress, Possession) => 0.20,
        (Possession, HighPress) => -0.20,
        
        // 控球克制反击（减少反击机会）
        (Possession, CounterAttack) => 0.15,
        (CounterAttack, Possession) => -0.15,
        
        // 反击克制高位逼抢（利用身后空间）
        (CounterAttack, HighPress) => 0.25,
        (HighPress, CounterAttack) => -0.25,
        
        // 长传冲吊克制高位逼抢（绕过中场）
        (DirectPlay, HighPress) => 0.15,
        (HighPress, DirectPlay) => -0.15,
        
        // 长传冲吊被控球克制（丢球后难以组织）
        (DirectPlay, Possession) => -0.10,
        (Possession, DirectPlay) => 0.10,
        
        // 相同战术或平衡战术：无修正
        _ => 0.0,
    }
}

/// 从 Tactic 推断战术风格
pub fn infer_tactical_style(tactic: &Tactic) -> TacticalStyle {
    use crate::team::{PassingStyle, Tempo, DefensiveHeight};
    
    // 高位逼抢：高防线 + 高进攻心态(>70)
    if matches!(tactic.defensive_height, DefensiveHeight::High) 
        && tactic.attacking_mentality > 70 
    {
        return TacticalStyle::HighPress;
    }
    
    // 反击战术：低防线 + 快节奏 + 长传
    if matches!(tactic.defensive_height, DefensiveHeight::Low)
        && matches!(tactic.tempo, Tempo::Fast)
    {
        return TacticalStyle::CounterAttack;
    }
    
    // 控球战术：短传 + 慢/中等节奏
    if matches!(tactic.passing_style, PassingStyle::Short)
        && !matches!(tactic.tempo, Tempo::Fast)
    {
        return TacticalStyle::Possession;
    }
    
    // 长传冲吊：长传为主
    if matches!(tactic.passing_style, PassingStyle::Long) {
        return TacticalStyle::DirectPlay;
    }
    
    // 默认平衡
    TacticalStyle::Balanced
}
```

#### 3.1 球员状态影响系统

```rust
/// 计算球员当前比赛的有效能力值
/// 
/// 考虑因素：
/// - 疲劳（fatigue）：连续比赛累积，降低表现
/// - 士气（morale）：近期战绩影响，影响发挥稳定性
/// - 体能（match_fitness）：比赛适应度
/// 
/// # 返回值
/// 修正后的能力值（可能低于或高于 current_ability）
pub fn calculate_effective_ability(player: &Player) -> u16 {
    let base = player.current_ability as f64;
    
    // 疲劳影响：疲劳值 0-100
    // 疲劳 0 -> 100%，疲劳 50 -> 85%，疲劳 100 -> 70%
    let fatigue_modifier = 1.0 - (player.fatigue as f64 * 0.003);
    
    // 士气影响：士气 0-100
    // 士气 0 -> 85%，士气 50 -> 100%，士气 100 -> 110%
    let morale_modifier = 0.85 + (player.morale as f64 * 0.0025);
    
    // 体能影响：match_fitness 0-100
    // 体能 50 -> 90%，体能 75 -> 95%，体能 100 -> 100%
    let fitness_modifier = 0.80 + (player.match_fitness as f64 * 0.002);
    
    // 伤病状态：受伤球员能力大幅下降
    let injury_modifier = if player.status == PlayerStatus::Injured {
        0.5  // 带伤上场能力减半
    } else {
        1.0
    };
    
    // 综合修正
    let total_modifier = fatigue_modifier * morale_modifier * fitness_modifier * injury_modifier;
    
    // 限制范围：最低 50%，最高 115%
    let clamped_modifier = total_modifier.clamp(0.50, 1.15);
    
    (base * clamped_modifier).round() as u16
}

/// 计算球队有效攻击力（考虑球员状态）
pub fn calculate_effective_attack(team: &Team, players: &[Player]) -> u16 {
    let attackers: Vec<&Player> = team.starting_11.iter()
        .filter(|slot| matches!(slot.position, 
            Position::ST | Position::CF | Position::LW | Position::RW))
        .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
        .collect();

    if attackers.is_empty() { return 0; }

    attackers.iter()
        .map(|p| calculate_effective_ability(p))
        .sum::<u16>() / attackers.len() as u16
}

/// 计算球队有效防守力（考虑球员状态）
pub fn calculate_effective_defense(team: &Team, players: &[Player]) -> u16 {
    let defenders: Vec<&Player> = team.starting_11.iter()
        .filter(|slot| matches!(slot.position, 
            Position::GK | Position::CB | Position::LB | Position::RB | Position::WB))
        .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
        .collect();

    if defenders.is_empty() { return 0; }

    defenders.iter()
        .map(|p| calculate_effective_ability(p))
        .sum::<u16>() / defenders.len() as u16
}

/// 计算球队有效中场控制力（考虑球员状态）
pub fn calculate_effective_midfield(team: &Team, players: &[Player]) -> u16 {
    let midfielders: Vec<&Player> = team.starting_11.iter()
        .filter(|slot| matches!(slot.position, 
            Position::DM | Position::CM | Position::AM))
        .filter_map(|slot| players.iter().find(|p| p.id == slot.player_id))
        .collect();

    if midfielders.is_empty() { return 0; }

    midfielders.iter()
        .map(|p| calculate_effective_ability(p))
        .sum::<u16>() / midfielders.len() as u16
}

/// 获取球员状态评级（用于 UI 显示）
pub fn get_player_condition_rating(player: &Player) -> &'static str {
    let fatigue_score = 100 - player.fatigue;
    let combined = (fatigue_score as u16 + player.morale as u16 + player.match_fitness as u16) / 3;
    
    match combined {
        90..=100 => "极佳",
        75..=89 => "良好",
        50..=74 => "一般",
        25..=49 => "疲惫",
        _ => "糟糕",
    }
}
```

#### 3.2 比赛模拟主函数

```rust
pub fn simulate_match(
    home_team: &Team,
    away_team: &Team,
    home_players: &[Player],
    away_players: &[Player],
    mode: MatchMode,
) -> MatchResult {
    let mut events = vec![];
    let mut home_score = 0;
    let mut away_score = 0;

    // 计算有效实力（考虑球员状态：疲劳、士气、体能）
    let home_attack = calculate_effective_attack(home_team, home_players);
    let home_defense = calculate_effective_defense(home_team, home_players);
    let away_attack = calculate_effective_attack(away_team, away_players);
    let away_defense = calculate_effective_defense(away_team, away_players);

    // 计算战术克制修正
    let home_style = infer_tactical_style(&home_team.tactic);
    let away_style = infer_tactical_style(&away_team.tactic);
    let tactical_modifier = calculate_tactical_modifier(&home_style, &away_style);
    
    // 应用战术修正到攻防值
    // 正值对主队有利，负值对客队有利
    let home_attack_modified = (home_attack as f64 * (1.0 + tactical_modifier)) as u16;
    let home_defense_modified = (home_defense as f64 * (1.0 + tactical_modifier * 0.5)) as u16;
    let away_attack_modified = (away_attack as f64 * (1.0 - tactical_modifier)) as u16;
    let away_defense_modified = (away_defense as f64 * (1.0 - tactical_modifier * 0.5)) as u16;

    match mode {
        MatchMode::Quick => {
            // 快速模拟：一次性计算结果
            let (home_goals, away_goals, match_events) = simulate_quick_match(
                home_attack_modified, 
                home_defense_modified, 
                away_attack_modified, 
                away_defense_modified,
            );
            home_score = home_goals;
            away_score = away_goals;
            events = match_events;
        }
        MatchMode::Live => {
            // 文本直播：逐步生成事件
            for minute in (1..=90).step_by(2) {
                let (home_events, away_events) = simulate_minute(
                    minute,
                    home_attack_modified,
                    home_defense_modified,
                    away_attack_modified,
                    away_defense_modified,
                    home_players,
                    away_players,
                );
                events.extend(home_events);
                events.extend(away_events);
            }
            home_score = events.iter().filter(|e| matches!(e, MatchEvent::Goal { .. })).count() as u8;
            // 实际需要根据team统计
        }
    }

    // 计算控球率和射门
    let (home_poss, away_poss) = calculate_possession(home_attack, away_attack);
    let (home_shots, away_shots) = calculate_shots(&events, home_attack, away_attack);

    MatchResult {
        id: generate_uuid(),
        league_id: home_team.league_id.clone(),
        home_team_id: home_team.id.clone(),
        away_team_id: away_team.id.clone(),
        home_score,
        away_score,
        match_mode: mode,
        events,
        home_possession: home_poss,
        away_possession: away_poss,
        home_shots: home_shots,
        away_shots: away_shots,
        played_at: get_timestamp(),
        round: 0,  // 从外部设置
    }
}
```

#### 3.2 快速模拟

```rust
fn simulate_quick_match(
    home_attack: u16,
    home_defense: u16,
    away_attack: u16,
    away_defense: u16,
) -> (u8, u8, Vec<MatchEvent>) {
    let mut home_goals = 0;
    let mut away_goals = 0;
    let mut events = vec![];

    // 45个回合（每个回合2分钟）
    for _ in 0..45 {
        // 主队进攻
        if rand::thread_rng().gen_bool(
            calculate_goal_chance(home_attack, away_defense)
        ) {
            home_goals += 1;
            events.push(generate_goal_event("home".to_string(), rand::random()));
        }

        // 客队进攻
        if rand::thread_rng().gen_bool(
            calculate_goal_chance(away_attack, home_defense)
        ) {
            away_goals += 1;
            events.push(generate_goal_event("away".to_string(), rand::random()));
        }
    }

    (home_goals, away_goals, events)
}
```

#### 3.3 进球概率计算

```rust
fn calculate_goal_chance(attack: u16, defense: u16) -> f64 {
    let base_chance = 0.03;  // 基础3%
    let strength_diff = (attack as f64 - defense as f64) / 100.0;
    let adjusted = base_chance + strength_diff * 0.01;

    // 随机波动 0.8 ~ 1.2
    let variance = rand::thread_rng().gen_range(0.8..1.2);

    (adjusted * variance).clamp(0.005, 0.10)
}
```

#### 3.4 随机事件生成

```rust
fn generate_match_events(
    minute: u8,
    attack_team: &str,
    defend_team: &str,
) -> Vec<MatchEvent> {
    let mut events = vec![];
    let rng = rand::thread_rng();

    let event_roll = rng.gen_range(0.0..1.0);

    match event_roll {
        x if x < 0.0002 => {
            // 乌龙球
            events.push(MatchEvent::OwnGoal {
                team: defend_team.to_string(),
                minute,
            });
        }
        x if x < 0.0050 => {
            // 点球
            let scored = rng.gen_bool(0.75);  // 75%得分率
            events.push(MatchEvent::Penalty {
                team: attack_team.to_string(),
                player_id: generate_uuid(),
                minute,
                scored,
            });
        }
        x if x < 0.0550 => {
            // 黄牌
            events.push(MatchEvent::YellowCard {
                team: defend_team.to_string(),
                player_id: generate_uuid(),
                minute,
            });
        }
        x if x < 0.0700 => {
            // 伤病
            let severity = rng.gen_range(1..=4);  // 1-4周
            events.push(MatchEvent::Injury {
                team: attack_team.to_string(),
                player_id: generate_uuid(),
                minute,
                severity,
            });
        }
        x if x < 0.1000 => {
            // 进球
            events.push(MatchEvent::Goal {
                team: attack_team.to_string(),
                player_id: generate_uuid(),
                minute,
            });
        }
        _ => {
            // 普通回合，无事件
        }
    }

    events
}
```

#### 3.5 换人建议系统（仅建议，不自动执行）

**设计理念**：AI 只提供建议，玩家完全控制是否采纳。

```rust
/// 换人建议
#[derive(Debug, Clone)]
pub struct SubstitutionSuggestion {
    pub player_out_id: String,
    pub player_out_name: String,
    pub player_in_id: String,
    pub player_in_name: String,
    pub reason: SubstitutionReason,
    pub urgency: SuggestionUrgency,
}

#[derive(Debug, Clone)]
pub enum SubstitutionReason {
    LowFitness { current: u8 },      // 体能过低
    HighFatigue { current: u8 },     // 疲劳过高
    Injured,                          // 受伤
    TacticalAttacking,               // 战术建议：加强进攻
    TacticalDefensive,               // 战术建议：加强防守
}

impl SubstitutionReason {
    /// 获取建议原因的描述（用于 UI 显示）
    pub fn description(&self) -> String {
        match self {
            SubstitutionReason::LowFitness { current } => 
                format!("体能过低 ({}%)", current),
            SubstitutionReason::HighFatigue { current } => 
                format!("疲劳过高 ({}%)", current),
            SubstitutionReason::Injured => 
                "球员受伤".to_string(),
            SubstitutionReason::TacticalAttacking => 
                "加强进攻".to_string(),
            SubstitutionReason::TacticalDefensive => 
                "加强防守".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SuggestionUrgency {
    Low = 0,       // 可以考虑
    Medium = 1,    // 建议换人
    High = 2,      // 强烈建议
    Critical = 3,  // 紧急（如受伤）
}

impl SuggestionUrgency {
    pub fn label(&self) -> &'static str {
        match self {
            SuggestionUrgency::Low => "可选",
            SuggestionUrgency::Medium => "建议",
            SuggestionUrgency::High => "强烈建议",
            SuggestionUrgency::Critical => "紧急",
        }
    }
}

/// AI 换人建议生成器
/// 
/// **重要**：只生成建议，不执行任何操作。
/// 所有决策权在玩家手中。
pub struct SubstitutionAdvisor;

impl SubstitutionAdvisor {
    /// 分析当前比赛状态，生成换人建议
    /// 
    /// # 参数
    /// - `minute`: 当前比赛分钟
    /// - `team`: 玩家球队
    /// - `starting_players`: 场上球员
    /// - `bench_players`: 替补席球员
    /// - `score_diff`: 比分差（正值领先，负值落后）
    /// - `subs_remaining`: 剩余换人名额
    /// 
    /// # 返回
    /// 建议列表，按紧急程度排序（最紧急在前）
    pub fn analyze(
        minute: u8,
        team: &Team,
        starting_players: &[Player],
        bench_players: &[Player],
        score_diff: i8,
        subs_remaining: u8,
    ) -> Vec<SubstitutionSuggestion> {
        if subs_remaining == 0 { 
            return vec![]; 
        }
        
        let mut suggestions = vec![];
        
        // 检查场上球员状态
        for slot in &team.starting_11 {
            if let Some(player) = starting_players.iter().find(|p| p.id == slot.player_id) {
                // 情况1：球员受伤 -> 紧急建议
                if player.status == PlayerStatus::Injured {
                    if let Some(replacement) = Self::find_best_replacement(
                        player, bench_players, &slot.position
                    ) {
                        suggestions.push(SubstitutionSuggestion {
                            player_out_id: player.id.clone(),
                            player_out_name: player.name.clone(),
                            player_in_id: replacement.id.clone(),
                            player_in_name: replacement.name.clone(),
                            reason: SubstitutionReason::Injured,
                            urgency: SuggestionUrgency::Critical,
                        });
                    }
                }
                
                // 情况2：体能过低 (<35) 且比赛过半 -> 强烈建议
                else if player.match_fitness < 35 && minute > 55 {
                    if let Some(replacement) = Self::find_best_replacement(
                        player, bench_players, &slot.position
                    ) {
                        suggestions.push(SubstitutionSuggestion {
                            player_out_id: player.id.clone(),
                            player_out_name: player.name.clone(),
                            player_in_id: replacement.id.clone(),
                            player_in_name: replacement.name.clone(),
                            reason: SubstitutionReason::LowFitness { 
                                current: player.match_fitness 
                            },
                            urgency: SuggestionUrgency::High,
                        });
                    }
                }
                
                // 情况3：疲劳过高 (>75) -> 建议
                else if player.fatigue > 75 && minute > 60 {
                    if let Some(replacement) = Self::find_best_replacement(
                        player, bench_players, &slot.position
                    ) {
                        suggestions.push(SubstitutionSuggestion {
                            player_out_id: player.id.clone(),
                            player_out_name: player.name.clone(),
                            player_in_id: replacement.id.clone(),
                            player_in_name: replacement.name.clone(),
                            reason: SubstitutionReason::HighFatigue { 
                                current: player.fatigue 
                            },
                            urgency: SuggestionUrgency::Medium,
                        });
                    }
                }
            }
        }
        
        // 战术性建议（较低优先级）
        // 落后2球以上且比赛后期 -> 可以考虑加强进攻
        if score_diff <= -2 && minute > 70 && subs_remaining > 0 {
            // 找一个防守球员换成攻击球员
            if let Some((def_player, atk_replacement)) = 
                Self::find_tactical_swap(team, starting_players, bench_players, true) 
            {
                suggestions.push(SubstitutionSuggestion {
                    player_out_id: def_player.id.clone(),
                    player_out_name: def_player.name.clone(),
                    player_in_id: atk_replacement.id.clone(),
                    player_in_name: atk_replacement.name.clone(),
                    reason: SubstitutionReason::TacticalAttacking,
                    urgency: SuggestionUrgency::Low,
                });
            }
        }
        
        // 领先2球以上且比赛后期 -> 可以考虑加强防守
        if score_diff >= 2 && minute > 75 && subs_remaining > 0 {
            if let Some((atk_player, def_replacement)) = 
                Self::find_tactical_swap(team, starting_players, bench_players, false) 
            {
                suggestions.push(SubstitutionSuggestion {
                    player_out_id: atk_player.id.clone(),
                    player_out_name: atk_player.name.clone(),
                    player_in_id: def_replacement.id.clone(),
                    player_in_name: def_replacement.name.clone(),
                    reason: SubstitutionReason::TacticalDefensive,
                    urgency: SuggestionUrgency::Low,
                });
            }
        }
        
        // 按紧急程度排序
        suggestions.sort_by(|a, b| b.urgency.cmp(&a.urgency));
        
        suggestions
    }
    
    /// 在替补席中找到最合适的同位置替换球员
    fn find_best_replacement<'a>(
        player_out: &Player,
        bench: &'a [Player],
        position: &Position,
    ) -> Option<&'a Player> {
        bench.iter()
            .filter(|p| {
                p.can_play_position(position) 
                && p.status == PlayerStatus::Healthy
                && p.match_fitness > 50  // 替补也需要有基本体能
            })
            .max_by_key(|p| p.get_position_rating(position))
    }
    
    /// 找到战术性换人组合
    /// `more_attacking`: true=换上攻击球员, false=换上防守球员
    fn find_tactical_swap<'a>(
        team: &Team,
        starting: &'a [Player],
        bench: &'a [Player],
        more_attacking: bool,
    ) -> Option<(&'a Player, &'a Player)> {
        if more_attacking {
            // 找一个体能低的防守球员
            let defender = starting.iter()
                .filter(|p| matches!(p.position, 
                    Position::CB | Position::LB | Position::RB | Position::DM))
                .filter(|p| p.match_fitness < 60)
                .min_by_key(|p| p.match_fitness)?;
            
            // 找一个攻击型替补
            let attacker = bench.iter()
                .filter(|p| matches!(p.position, 
                    Position::ST | Position::CF | Position::LW | Position::RW | Position::AM))
                .filter(|p| p.status == PlayerStatus::Healthy)
                .max_by_key(|p| p.current_ability)?;
            
            Some((defender, attacker))
        } else {
            // 找一个攻击球员
            let attacker = starting.iter()
                .filter(|p| matches!(p.position, 
                    Position::ST | Position::CF | Position::LW | Position::RW))
                .min_by_key(|p| p.match_fitness)?;
            
            // 找一个防守型替补
            let defender = bench.iter()
                .filter(|p| matches!(p.position, 
                    Position::CB | Position::LB | Position::RB | Position::DM | Position::CM))
                .filter(|p| p.status == PlayerStatus::Healthy)
                .max_by_key(|p| p.current_ability)?;
            
            Some((attacker, defender))
        }
    }
}
```

## 4. 事件生成器 (events.rs)

### 职责

生成随机游戏事件（伤病、突发新闻等）。

### 核心功能

```rust
pub enum GameEvent {
    PlayerInjury {
        player_id: String,
        injury_type: InjuryType,
        duration_weeks: u8,
    },
    TransferOffer {
        player_id: String,
        from_team: String,
        amount: u32,
    },
    PlayerWantsToLeave {
        player_id: String,
        reason: String,
    },
    BoardMeeting {
        topic: String,
        expectations: String,
    },
    MediaStory {
        title: String,
        impact: f32,  // 对士气的影响
    },
}

pub fn generate_random_event() -> Option<GameEvent> {
    let rng = rand::thread_rng();
    let roll = rng.gen_range(0.0..1.0);

    match roll {
        x if x < 0.01 => Some(GameEvent::PlayerInjury { /* ... */ }),
        x if x < 0.03 => Some(GameEvent::TransferOffer { /* ... */ }),
        x if x < 0.04 => Some(GameEvent::PlayerWantsToLeave { /* ... */ }),
        _ => None,
    }
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_league() {
        let league = generate_league();
        assert_eq!(league.current_round, 0);
        assert_eq!(league.total_rounds, 38);
    }

    #[test]
    fn test_generate_players_count() {
        let players = generate_players_for_team("team1".to_string(), 100, 22);
        assert_eq!(players.len(), 22);
    }

    #[test]
    fn test_match_simulation_returns_valid_score() {
        let home = create_test_team();
        let away = create_test_team();
        let result = simulate_match(&home, &away, &[], &[], MatchMode::Quick);

        assert!(result.home_score <= 10);
        assert!(result.away_score <= 10);
    }
}
```

## 依赖

- `team` 模块：数据模型（通过 `crate::team::*` 导入）
- `data` 模块：数据持久化
- `rand`：随机数生成

## 模块导入示例

```rust
use crate::team::{
    Player, Team, League, Position, Foot, PlayerStatus,
    MatchResult, MatchEvent, MatchMode,
    Tactic, Formation, PlayerRole, Duty,
};
use crate::data::{Database, PlayerRepository, TeamRepository};
```
