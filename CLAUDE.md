# Football Manager AI

这是一个用 Rust 开发的足球经理模拟游戏（TUI）。

## 快速理解项目

### 技术栈
- **语言**: Rust
- **UI**: ratatui (TUI)
- **数据库**: SQLite (rusqlite)
- **国际化**: 中英文双语

### 项目结构
```
src/
├── team/       # 数据模型（Player, Team, League, MatchResult, Tactic）
├── data/       # 数据持久化（Repository Pattern + SQLite）
├── ai/         # 游戏模拟（数据生成、比赛模拟、球员成长）
├── transfer/   # 转会市场
├── game/       # 核心逻辑（GameState、事件系统）
└── ui/         # TUI 界面（国际化支持）
```

## 模块职责

| 模块 | 职责 | 依赖 |
|------|------|------|
| **team** | 定义所有数据模型 | 无 |
| **data** | SQLite 数据访问 | team |
| **ai** | 生成数据、模拟比赛 | team, data |
| **transfer** | 转会市场逻辑 | team, data |
| **game** | 游戏状态和流程控制 | team, ai, data |
| **ui** | 用户界面 | game, team |

## 关键设计

### 数据访问
- 使用 **Repository Pattern**（trait 定义，SQLite 实现）
- 每个 Repository trait 在 `repository.rs` 中定义
- 实现在 `*_repo.rs` 中（如 `team_repo.rs`）

### 存档系统
- **每个存档一个独立的 SQLite 文件**
- 文件命名：`save_{slot:03d}_{timestamp}_{team_name}.db`
- 使用 `game_metadata` 表存储 GameState
- SaveManager 负责任务：`save_game()`, `load_game()`, `list_saves()`

### 国际化
- **UI 代码禁止硬编码中文字符串**
- 必须使用：`i18n.t(TranslationKey::XXX)`
- 翻译键定义在 `ui/i18n.rs`

### 数据模型
- 所有需要持久化的类型都实现了 `Serialize`/`Deserialize`
- Player 有大量属性（技术、精神、身体、门将属性）
- 位置使用 `Position` 枚举（GK, CB, LB, RB, WB, DM, CM, AM, LW, RW, ST, CF）

### 游戏时间系统
- 使用 `GameDate` 管理游戏内日期（年/月/日）
- 赛季从 8 月 1 日开始，5 月 31 日结束
- 转会窗口：夏窗(6-8月)、冬窗(1月)
- 转会操作必须在窗口开放期间进行

## 开发规范

### 命名
- 文件：小写+下划线 (`team_repo.rs`)
- 类型：大驼峰 (`TeamRepository`)
- 函数：小写+下划线 (`get_team_by_id`)

### 错误处理
- 使用 `thiserror` 定义错误类型
- 函数返回 `Result<T, Error>`
- UI 层负责显示用户友好的错误

### 测试
- 单元测试放在各模块的 `tests` 模块
- 使用 `:memory:` 数据库测试（避免创建文件）
- 命令：`cargo test --lib <module>`

## 设计文档位置

各模块的详细设计在 `src/<module>/` 目录：
- `design.md` - 架构和 API 设计
- `task.md` - 实现任务清单

**优先阅读**: `src/team/design.md`（理解数据模型）

## 开发顺序建议

1. **team** → 数据模型（基础）
2. **data** → 数据持久化
3. **game** → 核心框架
4. **ai** → 模拟引擎
5. **transfer** → 转会系统
6. **ui** → 用户界面

## 重要提示

- 存档系统在 `src/data/task.md` 的 Phase 4.5
- 国际化键定义在 `src/ui/design.md` 的 i18n 部分
- 比赛模拟逻辑在 `src/ai/design.md` 的 match_sim.rs 部分
- 数据库 Schema 在 `src/data/design.md`
- 数据库表：`teams`, `players`, `leagues`, `matches`, `transfer_market`, `game_metadata`, `scheduled_matches`, `lineups`, `team_statistics`

## 调试

```bash
# 查看存档数据库
sqlite3 saves/save_001_xxx.db
.tables
SELECT * FROM players LIMIT 5;
```

---

**当前状态**: 设计阶段完成，准备开始实现
**最后更新**: 2026-02-01

## 设计文档修订记录

### 2026-02-01 设计审查修复
- 统一 Position 枚举（添加 WB）
- 添加 OwnGoal 到 MatchEvent
- 统一 Team 方法签名（使用 `&[Player]`）
- 将 transfer 模块的 async 方法改为同步
- 添加缺失的数据库表：scheduled_matches, lineups, team_statistics
- 添加缺失的 Repository trait：ScheduledMatchRepository, LineupRepository, TeamStatisticsRepository
- 修复 UI 硬编码问题，使用 TranslationKey
- 统一存档系统职责（SaveManager 在 data 模块）
- 添加完整的 GameError 定义

### 2026-02-01 优化：游戏内日期系统
- 添加 `GameDate` 结构到 game 模块（年/月/日、星期、赛季计算）
- `GameState` 使用 `current_date` 替代 `current_tick`
- transfer 模块添加 `TransferWindow` 枚举和转会窗口检查
- UI 添加日期相关翻译键
- data 模块更新 game_metadata 存储日期字段

### 2026-02-01 优化：战术克制系统
- ai 模块添加 `TacticalStyle` 枚举（Possession, CounterAttack, HighPress, DirectPlay, Balanced）
- 添加 `calculate_tactical_modifier()` 计算战术克制修正（±25%）
- 添加 `infer_tactical_style()` 从 Tactic 推断战术风格
- 比赛模拟应用战术修正到攻防值
- team 模块 Tactic 添加 `style_description()` 和 `intensity()` 方法

### 2026-02-01 优化：球员状态影响系统
- 添加 `calculate_effective_ability()` 计算球员有效能力（考虑疲劳/士气/体能/伤病）
- 添加 `calculate_effective_attack/defense/midfield()` 计算球队有效实力
- 添加 `get_player_condition_rating()` 获取状态评级（极佳/良好/一般/疲惫/糟糕）
- 比赛模拟使用有效能力值替代基础能力值

### 2026-02-01 优化：通知系统
- game 模块添加 `Notification` 结构和 `NotificationManager`
- 支持通知类型：Transfer/Injury/Contract/Match/Finance/PlayerMorale/Achievement/News/System
- 支持优先级：Urgent/High/Normal/Low
- GameState 添加 `notifications` 字段，Screen 枚举添加 `Notifications`
- UI 添加 `NotificationsScreen` 通知列表界面
- 主菜单显示未读通知数量，紧急通知红色高亮

### 2026-02-01 优化：球员详情界面
- Screen 枚举添加 `PlayerDetail { player_id }`
- UI 添加 `PlayerDetailScreen`，包含三个标签页：
  - Overview：状态信息（疲劳/士气/体能/健康）
  - Attributes：技术/精神/身体属性分列显示
  - Contract：薪资/合同剩余/市场价值/潜力
- 添加大量属性翻译键（中英文）

### 2026-02-01 优化：AI 换人建议系统
- **设计理念**：AI 只提供建议，玩家完全控制是否采纳
- ai 模块添加 `SubstitutionAdvisor`、`SubstitutionSuggestion`、`SubstitutionReason`、`SuggestionUrgency`
- 建议触发条件：受伤(Critical)、体能<35(High)、疲劳>75(Medium)、战术调整(Low)
- UI `MatchLiveScreen` 添加建议提示和建议面板
- 紧急建议红色高亮，玩家按 [S] 查看、[Enter] 执行、[Esc] 忽略

### 2026-02-01 优化：设置界面
- 添加 `SettingsScreen`，分两个设置分类：
  - 语言设置：中文/English 切换
  - 比赛设置：默认比赛模式（文本直播/快速模拟）
- 主菜单添加设置入口
- 当前选中项黄色高亮，已启用选项绿色标记

### 2026-02-01 优化：赛季统计系统
- team 模块添加 `PlayerSeasonStats`：出场/进球/助攻/黄红牌/评分等
- data 模块添加 `player_season_stats` 表和 `PlayerSeasonStatsRepository`
- 支持每90分钟进球率/助攻率计算

### 2026-02-01 优化：比赛统计详情
- `MatchResult` 重构，用 `MatchStatistics` 替代散落字段
- `MatchStatistics` 包含：控球率、射门/射正、传球/成功率、角球、犯规、越位、球员评分
- `PlayerMatchRating` 记录球员单场评分
- `MatchResultScreen` 三标签页：概览/详细统计/球员评分

### 2026-02-01 优化：球员搜索筛选
- `TeamManagementScreen` 添加 `PlayerFilter` 和 `SortOption`
- 按位置/年龄/能力筛选，按姓名/年龄/能力/位置排序
- [F] 打开筛选面板，[S] 切换排序，[C] 清除筛选

### 2026-02-01 优化：赛季总结界面
- Screen 枚举添加 `SeasonSummary { season }`
- `SeasonSummaryScreen` 四标签页：概览/最终排名/射手榜/奖项
- 显示玩家球队表现、联赛排名、进球/助攻/零封等统计

### 2026-02-01 优化：比赛历史界面
- Screen 枚举添加 `MatchHistory`
- `MatchHistoryScreen` 支持按胜/平/负筛选历史比赛
- 可查看任意历史比赛详情

### 2026-02-01 优化：财务系统
- team 模块添加 `TeamFinance`、`FinanceTransaction`、`TransactionType`、`SeasonFinanceReport`
- data 模块添加 `team_finance`、`finance_transactions`、`season_finance_reports` 表
- `FinanceReportScreen` 四标签页：概览/收入明细/支出明细/历史记录
- 收入类型：转会/比赛日/赞助/电视转播/奖金
- 支出类型：转会/薪资/奖金/教练组/设施/青训
