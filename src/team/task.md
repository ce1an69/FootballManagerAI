# Team Module Implementation Tasks

## Phase 1: 基础数据模型

### Task 1.1: Position 和基础枚举 (player.rs)
- [x] 创建 `player.rs` 文件
- [x] 定义 `Position` enum (GK, CB, LB, RB, etc.)
- [x] 定义 `Foot` enum (Left, Right, Both)
- [x] 定义 `PlayerStatus` enum (Healthy, Injured, etc.)
- [x] 添加 `Serialize` 和 `Deserialize` derive

**注意**: `MatchMode` 枚举应在 `match_result.rs` 中定义

**Acceptance Criteria**:
```rust
let pos = Position::ST;
assert_eq!(pos, Position::ST);
let serialized = serde_json::to_string(&pos).unwrap();
```

---

### Task 1.2: Player 结构定义 (player.rs)
- [x] 在 `player.rs` 中定义 `Player` struct
- [x] 添加所有基本字段
- [x] 添加所有技术属性字段
- [x] 添加所有精神属性字段
- [x] 添加所有身体属性字段
- [x] 添加门将专属属性字段
- [x] 添加隐藏属性字段
- [x] 添加状态和合同字段

**Acceptance Criteria**: Player 结构包含所有必需字段

---

### Task 1.3: Player 基础方法 (player.rs)
- [x] 在 `player.rs` 中实现 `Player::new()`
- [x] 实现 `age_player()` - 增加年龄，调整能力
- [x] 实现 `recover_fatigue()` - 恢复疲劳
- [x] 实现 `injure()` - 设置伤病
- [x] 实现 `heal()` - 治疗伤病
- [x] 实现 `is_gk()` - 判断是否门将
- [x] 实现 `can_play_position()` - 判断能否踢某位置

**Acceptance Criteria**:
```rust
let mut player = Player::new("1".to_string(), "Test".to_string(), Position::ST);
player.age_player();
assert_eq!(player.age, 17);  // 假设初始16岁
```

---

### Task 1.4: Player 能力计算 (player.rs)
- [x] 在 `player.rs` 中实现 `calculate_overall_ability()`
- [x] 实现 `get_position_rating()` - 计算特定位置评分
- [x] 实现各个位置的属性权重逻辑
  - ST: Finishing, Off The Ball, Pace
  - CM: Passing, Vision, Stamina
  - CB: Marking, Tackling, Positioning
  - GK: Handling, Reflexes, Positioning
  - 等等

**Acceptance Criteria**:
```rust
let rating = player.get_position_rating(&Position::ST);
assert!(rating >= 0 && rating <= 200);
```

---

## Phase 2: 球队模型

### Task 2.1: Team 结构定义 (team.rs)
- [x] 创建 `team.rs` 文件
- [x] 定义 `Team` struct
- [x] 定义 `PlayerSlot` struct
- [x] 定义 `TeamStatistics` struct
- [x] 添加必需字段
- [x] 导入 `player::Position` 和 `tactics::*`

**Acceptance Criteria**: Team 结构正确定义

---

### Task 2.2: Team 方法实现 (team.rs)
- [x] 在 `team.rs` 中实现 `Team::new()`
- [x] 实现 `add_player()` - 添加球员ID到球队
- [x] 实现 `remove_player()` - 移除球员
- [x] 实现 `set_starting_11()` - 设置首发阵容
- [x] 实现 `update_statistics()` - 更新统计

**Acceptance Criteria**:
```rust
let mut team = Team::new("1".to_string(), "Test".to_string(), "league1".to_string(), 1000000);
team.add_player("player1".to_string());
assert!(team.players.contains(&"player1".to_string()));
```

---

### Task 2.3: Team 实力计算 (team.rs)
- [x] 在 `team.rs` 中实现 `get_team_strength()` - 计算球队整体实力
- [x] 实现 `calculate_attack_strength()` - 进攻实力
- [x] 实现 `calculate_defense_strength()` - 防守实力
- [x] 需要依赖 `PlayerRepository` 来获取球员数据
- [x] 导入 `player::Player`

**Acceptance Criteria**: 实力计算返回合理的数值 (0-200)

---

## Phase 3: 联赛模型

### Task 3.1: League 结构定义 (league.rs)
- [x] 创建 `league.rs` 文件
- [x] 定义 `League` struct
- [x] 定义 `MatchSchedule` struct
- [x] 定义 `Round` struct
- [x] 定义 `ScheduledMatch` struct

**Acceptance Criteria**: 联赛数据结构定义完成

---

### Task 3.2: League 方法 (league.rs)
- [x] 在 `league.rs` 中实现 `League::new()`
- [x] 实现 `generate_schedule()` - 生成赛程（双循环）
- [x] 实现 `get_current_round_matches()` - 获取当前轮次比赛
- [x] 实现 `advance_round()` - 进入下一轮

**Acceptance Criteria**:
```rust
let league = League::new("1".to_string(), "Test League", 20);
let schedule = league.generate_schedule();
assert_eq!(schedule.rounds.len(), 38);  // 20队双循环
```

---

## Phase 4: 战术系统

### Task 4.1: 战术枚举定义
- [x] 定义 `Formation` enum 及其所有变体
- [x] 定义 `DefensiveHeight` enum
- [x] 定义 `PassingStyle` enum
- [x] 定义 `Tempo` enum

**Acceptance Criteria**: 所有战术枚举定义完成

---

### Task 4.2: Formation 实现
- [x] 实现 `Formation::positions()` - 返回该阵型的位置列表
- [x] 实现 `Formation::name()` - 返回阵型名称
- [x] 定义每个阵型的位置布局
  - 4-4-2: GK, LB, CB, CB, RB, LM, CM, CM, RM, ST, ST
  - 4-3-3: GK, LB, CB, CB, RB, CM, CM, CM, LW, ST, RW
  - 等等

**Acceptance Criteria**:
```rust
let positions = Formation::FourFourTwo.positions();
assert_eq!(positions.len(), 11);
assert_eq!(positions[0], Position::GK);
```

---

### Task 4.3: Player Role 定义
- [x] 定义 `PlayerRole` enum（所有FM角色）
- [x] 定义 `Duty` enum (Attack, Support, Defend, Stopper, Cover)
- [x] 定义 `PlayerRoleAssignment` struct

**Acceptance Criteria**: 角色系统定义完成

---

### Task 4.4: Tactic 结构
- [x] 定义 `Tactic` struct
- [x] 实现 `Tactic::new()` - 默认战术
- [x] 实现战术验证方法 `validate()`

**Acceptance Criteria**: 战术结构可用

---

### Task 4.5: PlayerRole 方法
- [x] 实现 `get_required_attributes()` - 返回角色所需关键属性
- [x] 实现 `get_description()` - 角色描述
- [x] 实现 `is_suitable_for_position()` - 角色是否适合某位置

**Acceptance Criteria**:
```rust
let attrs = PlayerRole::AdvancedForward.get_required_attributes();
assert!(attrs.contains(&"pace"));
assert!(attrs.contains(&"finishing"));
```

---

## Phase 5: 比赛结果模型

### Task 5.1: MatchResult 结构 (match_result.rs)
- [x] 创建 `match_result.rs` 文件
- [x] 定义 `MatchResult` struct
- [x] 定义 `MatchMode` enum (Live, Quick)
- [x] 定义 `MatchEvent` enum
- [x] 添加所有必需字段

**Acceptance Criteria**: 比赛结果数据结构定义完成

---

## Phase 6: 辅助计算

### Task 6.1: 市场价值计算
- [x] 实现 `calculate_market_value()`
- [x] 实现年龄加成逻辑
- [x] 实现潜力加成逻辑

**Acceptance Criteria**:
```rust
let value = calculate_market_value(&player);
assert!(value > 0);
```

---

### Task 6.2: 薪资计算
- [x] 实现 `calculate_wage()`
- [x] 基于能力和年龄

**Acceptance Criteria**: 薪资计算合理

---

## Phase 7: 模块导出

### Task 7.1: mod.rs
- [x] 创建 `mod.rs` 文件
- [x] 导出 `player` 模块
- [x] 导出 `team` 模块
- [x] 导出 `league` 模块
- [x] 导出 `match_result` 模块
- [x] 导出 `tactics` 模块
- [x] 导出 `attributes` 模块
- [x] 重新导出常用类型：`pub use player::{Player, Position, Foot, PlayerStatus};`
- [x] 重新导出常用类型：`pub use team::{Team, PlayerSlot, TeamStatistics};`
- [x] 重新导出常用类型：`pub use league::{League, MatchSchedule, Round, ScheduledMatch};`
- [x] 重新导出常用类型：`pub use match_result::{MatchResult, MatchMode, MatchEvent};`

**Acceptance Criteria**: 其他模块可以 `use team::{Player, Team, League, MatchResult}`

---

## Phase 8: 测试

### Task 8.1: 单元测试
- [x] Player 相关测试
- [x] Team 相关测试
- [x] League 相关测试
- [x] Tactic 相关测试

**Acceptance Criteria**: `cargo test --lib team` 全部通过

---

## 依赖关系

```
Phase 1 (Player) → Phase 2 (Team) → Phase 3 (League) → Phase 4 (Tactic) → Phase 5 (MatchResult)
                                                              ↓
                                                      Phase 6 (Helpers)
                                                              ↓
                                                      Phase 7 (Exports)
                                                              ↓
                                                      Phase 8 (Tests)
```

---

## 预估时间

- Phase 1: 3-4天
- Phase 2: 2-3天
- Phase 3: 2天
- Phase 4: 3-4天
- Phase 5: 1天
- Phase 6: 1天
- Phase 7: 0.5天
- Phase 8: 2天

**总计**: 约 15-18 天

---

## 注意事项

1. **Serde**: 确保所有需要序列化的类型都添加 derive
2. **Clone**: 大多数数据需要 Clone（用于比赛模拟）
3. **默认值**: 属性默认值为 50（中等水平）
4. **位置适配性**: 球员踢非本职位置时评分应该降低
5. **年龄成长**: 青年球员能力每年增长，老年球员衰退
