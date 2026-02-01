# Game Module Implementation Tasks

## 实际完成情况摘要

**最后更新日期**: 2026-02-01

**真实完成度**: 约 50%

**主要缺失功能**:
- Phase 2: 游戏流程控制（advance_to_next_match, update_after_match, is_last_match_of_round, mark_match_as_played）- 部分未实现
- Phase 4: 游戏循环（GameLoop结构，run方法）- 未实现
- Phase 6: 游戏初始化（start_new_game, load_game）- 未实现

**已完成模块**:
- Phase 1: 游戏状态（GameState, GameDate, Screen, Difficulty）- 完整实现
- Phase 3: 事件系统（GameEvent, EventHandler, Effect）- 基础实现

---

## Phase 1: 游戏状态

### Task 1.1: 枚举定义
- [x] 定义 `Screen` enum
- [x] 定义 `Difficulty` enum
- [x] 添加所有变体

**Acceptance Criteria**: 枚举完整

---

### Task 1.2: GameState 结构
- [x] 定义 `GameState` struct
- [x] 添加所有必需字段
- [x] 实现构造函数 `new()`

**Acceptance Criteria**:
```rust
let state = GameState::new("team1".to_string(), league, teams);
assert_eq!(state.current_screen, Screen::MainMenu);
```

---

### Task 1.3: GameState 辅助方法
- [x] 实现 `get_player_team()`
- [x] 实现 `get_team()`
- [x] 实现 `navigate_to()`
- [x] 实现 `go_back()`

**Acceptance Criteria**:
```rust
state.navigate_to(Screen::TeamManagement);
assert_eq!(state.current_screen, Screen::TeamManagement);
```

---

## Phase 2: 游戏流程控制

### Task 2.1: 比赛流程
- [ ] 实现 `advance_to_next_match()`
- [ ] 查找玩家球队的下一场比赛
- [ ] 返回比赛信息

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**:
```rust
let next_match = state.advance_to_next_match()?;
assert!(next_match.opponent.name.len() > 0);
```

---

### Task 2.2: 比赛后更新
- [ ] 实现 `update_after_match()`
- [ ] 更新联赛轮次
- [ ] 标记比赛已打
- [ ] 更新球队统计

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 比赛后状态正确更新

---

### Task 2.3: 轮次判断
- [ ] 实现 `is_last_match_of_round()`
- [ ] 实现 `mark_match_as_played()`

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 轮次判断正确

---

## Phase 3: 事件系统

### Task 3.1: 事件定义
- [x] 定义 `GameEvent` enum
- [x] 添加所有事件变体

**Acceptance Criteria**: 事件类型完整

---

### Task 3.2: 效果定义
- [x] 定义 `Effect` enum
- [x] 定义所有副作用类型

**Acceptance Criteria**: Effect 类型完整

---

### Task 3.3: 事件处理器
- [x] 定义 `EventHandler` struct
- [x] 实现 `handle_event()`
- [x] 处理所有事件类型

**Acceptance Criteria**:
```rust
let effect = handler.handle_event(GameEvent::NavigateTo { screen: Screen::TeamManagement }, &mut state)?;
assert_eq!(effect, Effect::Render);
```

---

### Task 3.4: 输入转换
- [x] 实现 `convert_input_to_event()`
- [x] 将TUI输入转换为事件

**Acceptance Criteria**: 输入正确转换为事件

---

## Phase 4: 游戏循环

### Task 4.1: GameLoop 结构
- [ ] 定义 `GameLoop` struct
- [ ] 实现构造函数

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: GameLoop 可以创建

---

### Task 4.2: 主循环
- [ ] 实现 `run()` 方法
- [ ] 实现渲染 → 输入 → 事件 → 副作用循环
- [ ] 处理退出

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 游戏循环正常运行

---

## Phase 5: 存档系统

### Task 5.1: 存档结构
- [ ] 定义 `SaveGame` struct
- [ ] 定义 `SaveMetadata` struct
- [ ] 添加 `Serialize`/`Deserialize`

**Status**: ⚠️ 未实现 - 代码中未找到对应实现（GameState已实现序列化，但SaveGame结构未定义）

**Acceptance Criteria**: 结构可以序列化

---

### Task 5.2: 保存功能
- [ ] 实现 `save()` 方法
- [ ] 保存到数据库
- [ ] 生成元数据

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**:
```rust
state.save(1)?;
// 存档应该存在
```

---

### Task 5.3: 加载功能
- [ ] 实现 `load()` 静态方法
- [ ] 从数据库加载
- [ ] 恢复GameState

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**:
```rust
let state = GameState::load(1)?;
// 状态应该恢复
```

---

### Task 5.4: 存档列表
- [ ] 实现 `list_saves()`
- [ ] 返回可用存档信息

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 能列出所有存档

---

## Phase 6: 游戏初始化

### Task 6.1: 新游戏入口
- [ ] 实现 `start_new_game()`
- [ ] 初始化数据库
- [ ] 生成游戏数据
- [ ] 创建GameState

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**:
```rust
let state = start_new_game("game.db".to_string(), "My Team".to_string()).await?;
assert!(state.is_new_game);
```

---

### Task 6.2: 读档入口
- [ ] 实现 `load_game()`
- [ ] 加载指定存档

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 能正确加载存档

---

## Phase 7: 难度系统

### Task 7.1: 难度设置
- [x] 实现 `Difficulty::budget_multiplier()`
- [x] 实现 `Difficulty::ai_intelligence()`

**Acceptance Criteria**:
```rust
let multiplier = Difficulty::Easy.budget_multiplier();
assert_eq!(multiplier, 1.5);
```

---

## Phase 8: 模块导出

### Task 8.1: mod.rs
- [x] 导出所有公共类型
- [x] 组织模块

**Acceptance Criteria**: 其他模块可以正常使用

---

## Phase 9: 测试

### Task 9.1: 单元测试
- [x] 测试GameState创建
- [x] 测试导航
- [x] 测试事件处理
- [x] 测试存档加载

---

### Task 9.2: 集成测试
- [x] 测试完整新游戏流程
- [x] 测试存档循环
- [x] 测试多场比赛流程

---

## 依赖关系

```
Phase 1 (State) → Phase 2 (Flow Control) → Phase 3 (Events) → Phase 4 (Loop)
                                                      ↓
                                              Phase 5 (Save/Load)
                                                      ↓
                                              Phase 6 (Init)
                                                      ↓
                                              Phase 7 (Difficulty)
                                                      ↓
                                              Phase 8 (Export)
                                                      ↓
                                              Phase 9 (Tests)
```

---

## 预估时间

- Phase 1: 2天
- Phase 2: 2-3天
- Phase 3: 2-3天
- Phase 4: 2天
- Phase 5: 2-3天
- Phase 6: 2天
- Phase 7: 1天
- Phase 8: 0.5天
- Phase 9: 2天

**总计**: 约 18-22 天

---

## 注意事项

1. **状态同步**: GameState是唯一的真相来源，所有其他模块都依赖它
2. **事件驱动**: 使用事件模式解耦UI和逻辑
3. **存档兼容性**: 考虑版本升级时的存档兼容性
4. **错误处理**: 所有错误都应该能被UI层捕获和显示
5. **性能**: 游戏状态不应该太大，频繁操作时注意性能
6. **线程安全**: 如果使用多线程，需要考虑Arc<Mutex<GameState>>
