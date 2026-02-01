# Data Module Implementation Tasks

## Phase 1: 基础设施 (Foundation)

### Task 1.1: 项目依赖配置
- [x] 添加 `rusqlite` 依赖
- [x] 添加 `thiserror` 依赖
- [x] 添加 `serde` 和 `serde_json` 依赖
- [x] 配置 Cargo.toml

**Acceptance Criteria**: `cargo build` 成功，所有依赖正确配置

---

### Task 1.2: 错误类型定义
- [x] 创建 `database.rs` 文件
- [x] 定义 `DatabaseError` enum
- [x] 实现 `std::fmt::Display` 和 `std::error::Error`
- [x] 添加从 `rusqlite::Error` 的转换

**Acceptance Criteria**:
```rust
let err = DatabaseError::QueryError("test".to_string());
assert_eq!(format!("{}", err), "Query failed: test");
```

---

### Task 1.3: Database 连接管理
- [x] 实现 `Database::new(path: &str)`
- [x] 实现 `Database::in_memory()` (用于测试)

**Acceptance Criteria**:
```rust
let db = Database::new("test.db")?;
assert!(db.conn.is_open());
```

---

## Phase 2: 数据库 Schema (Schema)

### Task 2.1: 初始 Migration
- [x] 创建 `migrations/` 目录 (内嵌在代码中)
- [x] 编写初始 schema
- [x] 创建 leagues 表
- [x] 创建 teams 表
- [x] 创建 players 表
- [x] 创建 matches 表
- [x] 创建 transfer_market 表
- [x] 创建 scheduled_matches 表
- [x] 创建 lineups 表
- [x] 创建 team_statistics 表
- [x] 创建 game_metadata 表

**Acceptance Criteria**:
- Migration 可以成功运行
- 所有表和索引正确创建

---

### Task 2.2: Migration Runner
- [x] 实现 `Database::run_migrations()`
- [x] 添加 embedded migrations

**Acceptance Criteria**:
```rust
let db = Database::in_memory()?;
db.run_migrations()?;
// 所有表应该存在
```

---

## Phase 3: Repository Traits (接口定义)

### Task 3.1: 定义 Repository Trait
- [x] 创建 `repository.rs`
- [x] 定义 `TeamRepository` trait
- [x] 定义 `PlayerRepository` trait
- [x] 定义 `LeagueRepository` trait
- [x] 定义 `MatchRepository` trait
- [x] 定义 `TransferMarketRepository` trait
- [x] 定义 `ScheduledMatchRepository` trait
- [x] 定义 `LineupRepository` trait
- [x] 定义 `TeamStatisticsRepository` trait

**Acceptance Criteria**: 所有 trait 方法签名定义完成

---

## Phase 4: SQLite 实现

### Task 4.1: TeamRepository 实现
- [x] 创建 `team_repo.rs`
- [x] 实现 `SqliteTeamRepository`
- [x] 实现 `create()`
- [x] 实现 `get_by_id()`
- [x] 实现 `get_all()`
- [x] 实现 `update()`
- [x] 实现 `delete()`
- [x] 实现 `get_by_league()`

**Acceptance Criteria**:
```rust
let repo = db.team_repo();
repo.create(&team)?;
let retrieved = repo.get_by_id(&team.id)?;
assert_eq!(retrieved.name, team.name);
```

---

### Task 4.2: PlayerRepository 实现
- [x] 创建 `player_repo.rs`
- [x] 实现 `SqlitePlayerRepository`
- [x] 实现 `create()`
- [x] 实现 `get_by_id()`
- [x] 实现 `get_by_team()`
- [x] 实现 `update()`
- [x] 实现 `delete()`
- [x] 实现 `get_free_agents()`
- [x] 实现 `search()` (可选，MVP之后)

**Acceptance Criteria**:
```rust
let repo = db.player_repo();
repo.create(&player)?;
let players = repo.get_by_team(&team_id)?;
assert_eq!(players.len(), 1);
```

---

### Task 4.3: LeagueRepository 实现
- [x] 创建 `league_repo.rs`
- [x] 实现 `SqliteLeagueRepository`
- [x] 实现 `create()`
- [x] 实现 `get_by_id()`
- [x] 实现 `update()`

**Acceptance Criteria**: CRUD 操作正常工作

---

### Task 4.4: MatchRepository 实现
- [x] 创建 `match_repo.rs`
- [x] 实现 `SqliteMatchRepository`
- [x] 实现 `save()`
- [x] 实现 `get_by_id()`
- [x] 实现 `get_by_team()`
- [x] 实现 `get_by_league()`

**Acceptance Criteria**: 可以保存和查询比赛记录

---

### Task 4.5: TransferMarketRepository 实现
- [x] 创建 `transfer_repo.rs`
- [x] 实现 `SqliteTransferMarketRepository`
- [x] 实现 `add_to_market()`
- [x] 实现 `remove_from_market()`
- [x] 实现 `get_market_players()`
- [x] 实现 `update_price()`

**Acceptance Criteria**: 转会市场CRUD操作正常

---

### Task 4.6: ScheduledMatchRepository 实现
- [x] 创建 `scheduled_match_repo.rs`
- [x] 实现 `SqliteScheduledMatchRepository`
- [x] 实现 `create()`
- [x] 实现 `get_by_id()`
- [x] 实现 `get_by_league()`
- [x] 实现 `get_by_round()`
- [x] 实现 `mark_as_played()`
- [x] 实现 `delete_by_league()`

**Acceptance Criteria**: 可管理赛程

---

### Task 4.7: LineupRepository 实现
- [x] 创建 `lineup_repo.rs`
- [x] 实现 `SqliteLineupRepository`
- [x] 实现 `save_lineup()`
- [x] 实现 `get_starting_11()`
- [x] 实现 `get_bench()`
- [x] 实现 `clear_lineup()`

**Acceptance Criteria**: 可管理阵容

---

### Task 4.8: TeamStatisticsRepository 实现
- [x] 创建 `team_statistics_repo.rs`
- [x] 实现 `SqliteTeamStatisticsRepository`
- [x] 实现 `create()`
- [x] 实现 `get_by_team()`
- [x] 实现 `update()`
- [x] 实现 `get_league_standings()`

**Acceptance Criteria**: 可管理球队统计

---

### Task 4.9: Database 提供 Repository 实例
- [x] 在 `Database` 中添加 `team_repo()` 方法
- [x] 在 `Database` 中添加 `player_repo()` 方法
- [x] 在 `Database` 中添加 `league_repo()` 方法
- [x] 在 `Database` 中添加 `match_repo()` 方法
- [x] 在 `Database` 中添加 `scheduled_match_repo()` 方法
- [x] 在 `Database` 中添加 `lineup_repo()` 方法
- [x] 在 `Database` 中添加 `team_statistics_repo()` 方法
- [x] 在 `Database` 中添加 `transfer_market_repo()` 方法
- [x] 在 `Database` 中添加 `save_manager()` 方法

**Acceptance Criteria**:
```rust
let db = Database::new("game.db")?;
let team_repo = db.team_repo();
let save_manager = db.save_manager();
// 可以正常使用 repository 和 save_manager
```

---

## Phase 4.5: 存档系统

### Task 4.5.1: SaveManager 结构
- [x] 创建 `save_manager.rs` 文件
- [x] 定义 `SaveManager` struct
- [x] 定义 `SaveMetadata` struct
- [x] 实现 `SaveManager::new()`

**Acceptance Criteria**:
```rust
let save_manager = SaveManager::new("saves")?;
assert_eq!(save_manager.saves_dir(), PathBuf::from("saves"));
```

---

### Task 4.5.2: 存档文件命名
- [x] 实现 `generate_filename()` - 生成存档文件名
- [x] 格式：`save_{slot:03d}_{timestamp}_{team_name}.db`
- [x] 实现 `sanitize_team_name()` - 清理球队名为文件名安全字符

**Acceptance Criteria**:
```rust
let filename = save_manager.generate_filename(1, "My Team")?;
assert!(filename.contains("save_001_"));
assert!(filename.contains(".db"));
```

---

### Task 4.5.3: 保存游戏
- [x] 实现 `save_game()` 方法
- [x] 保存 GameState 元数据到 game_metadata 表
- [x] 复制或使用当前数据库文件
- [x] 更新元数据索引（如果使用）

**Acceptance Criteria**:
```rust
save_manager.save_game(1, &state, &db)?;
// 存档文件应该存在
// game_metadata 表应该包含状态数据
```

---

### Task 4.5.4: 加载游戏
- [x] 实现 `load_game()` 方法
- [x] 根据槽位查找存档文件
- [x] 从 game_metadata 表恢复 GameState
- [x] 验证存档版本

**Acceptance Criteria**:
```rust
let loaded_state = save_manager.load_game(1)?;
assert_eq!(loaded_state.player_team_id, original_state.player_team_id);
```

---

### Task 4.5.5: 列出存档
- [x] 实现 `list_saves()` 方法
- [x] 扫描存档目录
- [x] 读取每个存档的元数据
- [x] 返回存档信息列表
- [x] 可选：使用 .metadata.json 索引优化性能

**Acceptance Criteria**:
```rust
let saves = save_manager.list_saves()?;
assert!(!saves.is_empty());
```

---

### Task 4.5.6: 删除存档
- [x] 实现 `delete_save()` 方法
- [x] 删除指定槽位的存档文件
- [x] 更新元数据索引（如果使用）

**Acceptance Criteria**:
```rust
save_manager.delete_save(1)?;
// 存档文件应该被删除
```

---

### Task 4.5.7: 存档备份
- [x] 实现 `backup_save()` 方法
- [x] 复制存档到指定目录
- [x] 生成备份文件名

**Acceptance Criteria**:
```rust
let backup_path = save_manager.backup_save(1, "backups")?;
assert!(backup_path.exists());
```

---

### Task 4.5.8: 创建新存档
- [x] 实现 `create_new_save()` 辅助函数
- [x] 创建新数据库文件
- [x] 运行数据库迁移
- [x] 保存初始状态

**Acceptance Criteria**:
```rust
let db = create_new_save(1, initial_state)?;
// 新数据库应该存在并包含所有表
```

---

### Task 4.5.9: 版本兼容性
- [x] 在 game_metadata 中添加 save_version 字段
- [x] 实现版本检查逻辑
- [x] 添加版本迁移支持（如果需要）

**Acceptance Criteria**:
```rust
let version = save_manager.get_save_version(1)?;
assert_eq!(version, "1.0");
```

---

## Phase 5: 模块导出

### Task 5.1: mod.rs 导出
- [x] 创建 `mod.rs`
- [x] 导出所有 repository traits
- [x] 导出 `DatabaseError`
- [x] 导出 `SaveError` (未使用，已移除)
- [x] 导出 `Database` struct
- [x] 导出 `SaveManager` struct
- [x] 导出 `SaveMetadata` struct

**Acceptance Criteria**: 其他模块可以通过 `use data::*` 访问

---

## Phase 6: 测试

### Task 6.1: 单元测试
- [x] 为 `TeamRepository` 编写测试
- [x] 为 `PlayerRepository` 编写测试
- [x] 为 `LeagueRepository` 编写测试
- [x] 为 `MatchRepository` 编写测试
- [x] 为 `ScheduledMatchRepository` 编写测试
- [x] 为 `LineupRepository` 编写测试
- [x] 为 `TeamStatisticsRepository` 编写测试
- [x] 为 `TransferMarketRepository` 编写测试

**Acceptance Criteria**: `cargo test --lib data` 全部通过

**Result**: ✅ 140 tests passing

---

### Task 6.2: 集成测试
- [ ] 创建完整流程测试
  - 创建联赛
  - 创建球队
  - 创建球员
  - 模拟比赛
  - 保存比赛结果
  - 查询统计数据

**Acceptance Criteria**: 端到端流程测试通过

**Status**: ⏳ 待实现

---

### Task 6.3: 存档系统测试
- [x] 测试保存游戏功能
- [x] 测试加载游戏功能
- [x] 测试完整的保存-加载循环
- [x] 测试列出所有存档
- [x] 测试删除存档
- [x] 测试存档备份功能
- [x] 测试版本兼容性

**Acceptance Criteria**:
```rust
// 保存游戏
save_manager.save_game(1, &state, &db)?;

// 加载游戏
let loaded_state = save_manager.load_game(1)?;
assert_eq!(loaded_state.player_team_id, state.player_team_id);

// 列出存档
let saves = save_manager.list_saves()?;
assert_eq!(saves.len(), 1);

// 删除存档
save_manager.delete_save(1)?;
let saves_after = save_manager.list_saves()?;
assert_eq!(saves_after.len(), 0);
```

**Status**: ✅ 基础测试完成，完整集成测试待实现

---

## Phase 7: 优化 (可选，MVP之后)

### Task 7.1: 性能优化
- [ ] 添加 prepared statements 缓存
- [ ] 添加批量操作方法
- [ ] 添加连接池

### Task 7.2: 添加索引
- [ ] 分析查询模式
- [ ] 添加必要的索引
- [ ] 验证索引效果

---

## 依赖关系

```
Phase 1 → Phase 2 → Phase 3 → Phase 4 → Phase 4.5 (存档) → Phase 5 → Phase 6
                              ↓
                         Phase 7 (可选)
```

---

## 预估时间

- Phase 1: 1天
- Phase 2: 2天
- Phase 3: 1天
- Phase 4: 5-7天
- Phase 4.5: 3-4天 (存档系统)
- Phase 5: 0.5天
- Phase 6: 2-3天

**总计**: 约 15-19 天

---

## 注意事项

1. **先写测试**: 每个 Repository 实现前先写测试
2. **使用内存数据库**: 测试时使用 `:memory:` 避免创建文件
3. **事务支持**: 考虑在复杂操作中使用事务
4. **JSON 序列化**: 复杂对象使用 JSON 存储到 TEXT 字段
5. **ID 生成**: 使用 UUID 或简单的字符串 ID
6. **存档隔离**: 每个存档使用独立的 SQLite 文件，确保完全隔离
7. **存档迁移**: 加载存档时检查版本，支持版本升级迁移
8. **元数据索引**: 如果存档数量多，考虑使用 .metadata.json 加速列表加载
