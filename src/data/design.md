# Data Module Design

## 概述

数据层模块负责所有数据持久化和访问操作，使用Repository模式提供抽象接口，其他模块通过此模块进行CRUD操作。

## 架构

### 模式：Repository Pattern

使用trait定义数据访问接口，SQLite作为具体实现，便于未来切换数据库。

### 文件结构

```
data/
├── mod.rs              # 模块导出
├── database.rs         # 数据库连接管理
├── repository.rs       # Repository trait定义
├── team_repo.rs        # 球队数据访问实现
├── player_repo.rs      # 球员数据访问实现
├── league_repo.rs      # 联赛数据访问实现
├── match_repo.rs       # 比赛记录数据访问实现
├── transfer_repo.rs    # 转会市场数据访问实现
└── save_manager.rs     # 存档管理实现
```

## 核心组件

### 1. Database (database.rs)

数据库连接和schema管理。

**职责**:
- 创建和管理SQLite连接
- 运行数据库迁移（refinery）
- 提供Repository实例

**API**:
```rust
pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, DatabaseError>
    pub fn in_memory() -> Result<Self, DatabaseError>
    pub fn run_migrations(&self) -> Result<(), DatabaseError>
    pub fn team_repo(&self) -> SqliteTeamRepository
    pub fn player_repo(&self) -> SqlitePlayerRepository
    pub fn league_repo(&self) -> SqliteLeagueRepository
    pub fn match_repo(&self) -> SqliteMatchRepository
    pub fn transfer_repo(&self) -> SqliteTransferRepository
    pub fn scheduled_match_repo(&self) -> SqliteScheduledMatchRepository
    pub fn lineup_repo(&self) -> SqliteLineupRepository
    pub fn team_statistics_repo(&self) -> SqliteTeamStatisticsRepository
}
```

### 2. Repository Traits (repository.rs)

定义数据访问接口。

**TeamRepository**:
```rust
pub trait TeamRepository {
    fn create(&self, team: &Team) -> Result<(), DatabaseError>
    fn get_by_id(&self, id: &str) -> Result<Team, DatabaseError>
    fn get_all(&self) -> Result<Vec<Team>, DatabaseError>
    fn update(&self, team: &Team) -> Result<(), DatabaseError>
    fn delete(&self, id: &str) -> Result<(), DatabaseError>
    fn get_by_league(&self, league_id: &str) -> Result<Vec<Team>, DatabaseError>
}
```

**PlayerRepository**:
```rust
pub trait PlayerRepository {
    fn create(&self, player: &Player) -> Result<(), DatabaseError>
    fn get_by_id(&self, id: &str) -> Result<Player, DatabaseError>
    fn get_by_team(&self, team_id: &str) -> Result<Vec<Player>, DatabaseError>
    fn update(&self, player: &Player) -> Result<(), DatabaseError>
    fn delete(&self, id: &str) -> Result<(), DatabaseError>
    fn get_free_agents(&self) -> Result<Vec<Player>, DatabaseError>
    fn search(&self, filters: PlayerFilters) -> Result<Vec<Player>, DatabaseError>
}
```

**LeagueRepository**:
```rust
pub trait LeagueRepository {
    fn create(&self, league: &League) -> Result<(), DatabaseError>
    fn get_by_id(&self, id: &str) -> Result<League, DatabaseError>
    fn update(&self, league: &League) -> Result<(), DatabaseError>
}
```

**MatchRepository**:
```rust
pub trait MatchRepository {
    fn save(&self, match_result: &MatchResult) -> Result<(), DatabaseError>
    fn get_by_id(&self, id: &str) -> Result<MatchResult, DatabaseError>
    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<MatchResult>, DatabaseError>
    fn get_by_league(&self, league_id: &str, round: u32) -> Result<Vec<MatchResult>, DatabaseError>
}
```

**ScheduledMatchRepository**:
```rust
pub trait ScheduledMatchRepository {
    fn create(&self, scheduled_match: &ScheduledMatch) -> Result<(), DatabaseError>
    fn get_by_id(&self, id: &str) -> Result<ScheduledMatch, DatabaseError>
    fn get_by_league(&self, league_id: &str) -> Result<Vec<ScheduledMatch>, DatabaseError>
    fn get_by_round(&self, league_id: &str, round: u32) -> Result<Vec<ScheduledMatch>, DatabaseError>
    fn mark_as_played(&self, id: &str) -> Result<(), DatabaseError>
    fn delete_by_league(&self, league_id: &str) -> Result<(), DatabaseError>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledMatch {
    pub id: String,
    pub league_id: String,
    pub round_number: u32,
    pub home_team_id: String,
    pub away_team_id: String,
    pub played: bool,
}
```

**LineupRepository**:
```rust
pub trait LineupRepository {
    fn save_lineup(&self, team_id: &str, lineup: &[PlayerSlot], is_starting: bool) -> Result<(), DatabaseError>
    fn get_starting_11(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError>
    fn get_bench(&self, team_id: &str) -> Result<Vec<PlayerSlot>, DatabaseError>
    fn clear_lineup(&self, team_id: &str) -> Result<(), DatabaseError>
}
```

**TeamStatisticsRepository**:
```rust
pub trait TeamStatisticsRepository {
    fn create(&self, team_id: &str) -> Result<(), DatabaseError>
    fn get_by_team(&self, team_id: &str) -> Result<TeamStatistics, DatabaseError>
    fn update(&self, team_id: &str, stats: &TeamStatistics) -> Result<(), DatabaseError>
    fn get_league_standings(&self, league_id: &str) -> Result<Vec<(String, TeamStatistics)>, DatabaseError>
}
```

**TransferMarketRepository**:
```rust
pub trait TransferMarketRepository {
    fn add_to_market(&self, player_id: &str, price: u32) -> Result<(), DatabaseError>
    fn remove_from_market(&self, player_id: &str) -> Result<(), DatabaseError>
    fn get_market_players(&self) -> Result<Vec<Player>, DatabaseError>
    fn get_market_listing(&self, player_id: &str) -> Result<Option<MarketListing>, DatabaseError>
    fn update_price(&self, player_id: &str, new_price: u32) -> Result<(), DatabaseError>
}

#[derive(Debug, Clone)]
pub struct MarketListing {
    pub player_id: String,
    pub asking_price: u32,
    pub listed_at: u64,
    pub reason: Option<String>,
}
```

## Database Schema

### teams 表
```sql
CREATE TABLE teams (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    league_id TEXT NOT NULL,
    budget INTEGER NOT NULL,
    formation TEXT NOT NULL,
    attacking_mentality INTEGER NOT NULL,  -- 0-100
    defensive_height TEXT NOT NULL,       -- low/medium/high
    passing_style TEXT NOT NULL,          -- short/mixed/long
    tempo TEXT NOT NULL,                  -- slow/medium/fast
    FOREIGN KEY (league_id) REFERENCES leagues(id)
)
```

### players 表
```sql
CREATE TABLE players (
    id TEXT PRIMARY KEY,
    team_id TEXT,  -- NULL for free agents

    -- 基本信息
    name TEXT NOT NULL,
    age INTEGER NOT NULL,
    nationality TEXT NOT NULL,
    position TEXT NOT NULL,
    second_positions TEXT,  -- JSON array
    preferred_foot TEXT NOT NULL,  -- left/right/both
    height INTEGER NOT NULL,
    weight INTEGER NOT NULL,

    -- 技术属性 (0-200)
    corners INTEGER DEFAULT 50,
    crossing INTEGER DEFAULT 50,
    dribbling INTEGER DEFAULT 50,
    finishing INTEGER DEFAULT 50,
    heading INTEGER DEFAULT 50,
    long_shots INTEGER DEFAULT 50,
    long_throws INTEGER DEFAULT 50,
    marking INTEGER DEFAULT 50,
    passing INTEGER DEFAULT 50,
    penalties INTEGER DEFAULT 50,
    tackling INTEGER DEFAULT 50,
    technique INTEGER DEFAULT 50,

    -- 精神属性 (0-200)
    aggression INTEGER DEFAULT 50,
    anticipation INTEGER DEFAULT 50,
    bravery INTEGER DEFAULT 50,
    creativity INTEGER DEFAULT 50,
    decisions INTEGER DEFAULT 50,
    concentration INTEGER DEFAULT 50,
    positioning INTEGER DEFAULT 50,
    off_the_ball INTEGER DEFAULT 50,
    work_rate INTEGER DEFAULT 50,
    pressure INTEGER DEFAULT 50,
    teamwork INTEGER DEFAULT 50,
    vision INTEGER DEFAULT 50,

    -- 身体属性 (0-200)
    acceleration INTEGER DEFAULT 50,
    agility INTEGER DEFAULT 50,
    balance INTEGER DEFAULT 50,
    pace INTEGER DEFAULT 50,
    stamina INTEGER DEFAULT 50,
    strength INTEGER DEFAULT 50,

    -- 门将属性 (0-200)
    aerial_reach INTEGER DEFAULT 50,
    command_of_area INTEGER DEFAULT 50,
    communication INTEGER DEFAULT 50,
    eccentricity INTEGER DEFAULT 50,
    handling INTEGER DEFAULT 50,
    kicking INTEGER DEFAULT 50,
    throwing INTEGER DEFAULT 50,
    reflexes INTEGER DEFAULT 50,
    rushing_out INTEGER DEFAULT 50,
    gk_positioning INTEGER DEFAULT 50,

    -- 隐藏属性
    potential_ability INTEGER DEFAULT 100,
    current_ability INTEGER DEFAULT 100,
    adaptability INTEGER DEFAULT 100,
    ambition INTEGER DEFAULT 100,
    professionalism INTEGER DEFAULT 100,
    loyalty INTEGER DEFAULT 100,
    injury_proneness INTEGER DEFAULT 50,
    controversy INTEGER DEFAULT 50,

    -- 状态
    match_fitness INTEGER DEFAULT 100,
    morale INTEGER DEFAULT 50,
    status TEXT DEFAULT 'healthy',  -- healthy/injured/fatigued/suspended
    injury_days INTEGER DEFAULT 0,
    fatigue INTEGER DEFAULT 0,

    -- 合同
    wage INTEGER NOT NULL,
    contract_years INTEGER NOT NULL,
    market_value INTEGER NOT NULL,

    FOREIGN KEY (team_id) REFERENCES teams(id)
)
```

### leagues 表
```sql
CREATE TABLE leagues (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    current_round INTEGER NOT NULL DEFAULT 0,
    total_rounds INTEGER NOT NULL
)
```

### matches 表
```sql
CREATE TABLE matches (
    id TEXT PRIMARY KEY,
    league_id TEXT NOT NULL,
    home_team_id TEXT NOT NULL,
    away_team_id TEXT NOT NULL,
    home_score INTEGER NOT NULL,
    away_score INTEGER NOT NULL,
    match_mode TEXT NOT NULL,  -- live/quick
    events TEXT NOT NULL,       -- JSON of match events
    home_possession REAL,
    away_possession REAL,
    home_shots INTEGER,
    away_shots INTEGER,
    played_at INTEGER NOT NULL,  -- timestamp or game tick
    round INTEGER NOT NULL,

    FOREIGN KEY (league_id) REFERENCES leagues(id),
    FOREIGN KEY (home_team_id) REFERENCES teams(id),
    FOREIGN KEY (away_team_id) REFERENCES teams(id)
)
```

### transfer_market 表
```sql
CREATE TABLE transfer_market (
    player_id TEXT PRIMARY KEY,
    asking_price INTEGER NOT NULL,
    listed_at INTEGER NOT NULL,  -- timestamp
    reason TEXT,                 -- transfer_listed/loan_listed

    FOREIGN KEY (player_id) REFERENCES players(id)
)
```

### game_metadata 表（存档元数据）

存储 GameState 中的非数据库字段，用于存档系统：

```sql
CREATE TABLE game_metadata (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
```

存储内容示例：
```rust
// 存储为 key-value 对
INSERT INTO game_metadata VALUES 
    ('game_id', 'uuid-string'),
    ('game_date_year', '2026'),
    ('game_date_month', '8'),
    ('game_date_day', '15'),
    ('player_team_id', 'team-uuid'),
    ('player_name', 'Player Name'),
    ('current_screen', 'TeamManagement'),
    ('difficulty', 'Normal'),
    ('match_mode_preference', 'Quick'),
    ('saved_at', '1706784622'),
    ('play_time', '3600'),
    ('save_version', '1.0');
```

### scheduled_matches 表（赛程）
```sql
CREATE TABLE scheduled_matches (
    id TEXT PRIMARY KEY,
    league_id TEXT NOT NULL,
    round_number INTEGER NOT NULL,
    home_team_id TEXT NOT NULL,
    away_team_id TEXT NOT NULL,
    played INTEGER DEFAULT 0,  -- 0=未进行, 1=已进行
    
    FOREIGN KEY (league_id) REFERENCES leagues(id),
    FOREIGN KEY (home_team_id) REFERENCES teams(id),
    FOREIGN KEY (away_team_id) REFERENCES teams(id)
)
```

### lineups 表（首发阵容和替补）
```sql
CREATE TABLE lineups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    team_id TEXT NOT NULL,
    player_id TEXT NOT NULL,
    position TEXT NOT NULL,       -- GK/CB/LB/RB/WB/DM/CM/AM/LW/RW/ST/CF
    role TEXT NOT NULL,           -- PlayerRole 枚举值
    duty TEXT NOT NULL,           -- Attack/Support/Defend/Stopper/Cover
    is_starting INTEGER DEFAULT 1,  -- 1=首发, 0=替补
    position_index INTEGER,       -- 在阵型中的位置索引
    
    UNIQUE (team_id, player_id),
    FOREIGN KEY (team_id) REFERENCES teams(id),
    FOREIGN KEY (player_id) REFERENCES players(id)
)
```

### team_statistics 表（球队统计）
```sql
CREATE TABLE team_statistics (
    team_id TEXT PRIMARY KEY,
    matches_played INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    draws INTEGER DEFAULT 0,
    losses INTEGER DEFAULT 0,
    goals_for INTEGER DEFAULT 0,
    goals_against INTEGER DEFAULT 0,
    points INTEGER DEFAULT 0,
    league_position INTEGER,
    
    FOREIGN KEY (team_id) REFERENCES teams(id)
)
```

### player_season_stats 表（球员赛季统计）
```sql
CREATE TABLE player_season_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    player_id TEXT NOT NULL,
    season TEXT NOT NULL,           -- 如 "2026-27"
    team_id TEXT NOT NULL,
    
    -- 出场数据
    appearances INTEGER DEFAULT 0,
    starts INTEGER DEFAULT 0,
    minutes_played INTEGER DEFAULT 0,
    
    -- 进攻数据
    goals INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    shots INTEGER DEFAULT 0,
    shots_on_target INTEGER DEFAULT 0,
    
    -- 纪律数据
    yellow_cards INTEGER DEFAULT 0,
    red_cards INTEGER DEFAULT 0,
    
    -- 评分
    total_rating REAL DEFAULT 0,
    average_rating REAL DEFAULT 0,
    man_of_the_match INTEGER DEFAULT 0,
    
    UNIQUE (player_id, season),
    FOREIGN KEY (player_id) REFERENCES players(id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
)
```

### team_finance 表（球队财务状态）
```sql
CREATE TABLE team_finance (
    team_id TEXT PRIMARY KEY,
    balance INTEGER NOT NULL DEFAULT 0,
    wage_budget INTEGER NOT NULL DEFAULT 0,
    transfer_budget INTEGER NOT NULL DEFAULT 0,
    
    FOREIGN KEY (team_id) REFERENCES teams(id)
)
```

### finance_transactions 表（财务交易记录）
```sql
CREATE TABLE finance_transactions (
    id TEXT PRIMARY KEY,
    team_id TEXT NOT NULL,
    transaction_type TEXT NOT NULL,
    amount INTEGER NOT NULL,
    description TEXT,
    date_year INTEGER NOT NULL,
    date_month INTEGER NOT NULL,
    date_day INTEGER NOT NULL,
    related_player_id TEXT,
    
    FOREIGN KEY (team_id) REFERENCES teams(id),
    FOREIGN KEY (related_player_id) REFERENCES players(id)
)
```

### season_finance_reports 表（赛季财务报告）
```sql
CREATE TABLE season_finance_reports (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    season TEXT NOT NULL,
    team_id TEXT NOT NULL,
    transfer_income INTEGER DEFAULT 0,
    matchday_income INTEGER DEFAULT 0,
    sponsorship INTEGER DEFAULT 0,
    tv_revenue INTEGER DEFAULT 0,
    prize_money INTEGER DEFAULT 0,
    transfer_expenses INTEGER DEFAULT 0,
    wage_expenses INTEGER DEFAULT 0,
    bonus_expenses INTEGER DEFAULT 0,
    staff_wages INTEGER DEFAULT 0,
    facilities INTEGER DEFAULT 0,
    youth_academy INTEGER DEFAULT 0,
    
    UNIQUE (season, team_id),
    FOREIGN KEY (team_id) REFERENCES teams(id)
)
```

### indexes（用于性能优化）
```sql
CREATE INDEX idx_players_team ON players(team_id);
CREATE INDEX idx_players_position ON players(position);
CREATE INDEX idx_matches_league ON matches(league_id);
CREATE INDEX idx_matches_team ON matches(home_team_id);
CREATE INDEX idx_matches_team_away ON matches(away_team_id);
CREATE INDEX idx_teams_league ON teams(league_id);
CREATE INDEX idx_scheduled_matches_league ON scheduled_matches(league_id);
CREATE INDEX idx_scheduled_matches_round ON scheduled_matches(round_number);
CREATE INDEX idx_lineups_team ON lineups(team_id);
CREATE INDEX idx_player_season_stats_player ON player_season_stats(player_id);
CREATE INDEX idx_player_season_stats_season ON player_season_stats(season);
CREATE INDEX idx_player_season_stats_team ON player_season_stats(team_id);
CREATE INDEX idx_finance_transactions_team ON finance_transactions(team_id);
CREATE INDEX idx_finance_transactions_date ON finance_transactions(date_year, date_month, date_day);
CREATE INDEX idx_season_finance_reports_season ON season_finance_reports(season);
```

**PlayerSeasonStatsRepository**:
```rust
pub trait PlayerSeasonStatsRepository {
    fn create(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError>;
    fn get_by_player(&self, player_id: &str, season: &str) -> Result<PlayerSeasonStats, DatabaseError>;
    fn get_by_team(&self, team_id: &str, season: &str) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn update(&self, stats: &PlayerSeasonStats) -> Result<(), DatabaseError>;
    fn get_or_create(&self, player_id: &str, season: &str, team_id: &str) -> Result<PlayerSeasonStats, DatabaseError>;
    
    // 排行榜查询
    fn get_top_scorers(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn get_top_assists(&self, season: &str, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
    fn get_top_rated(&self, season: &str, min_appearances: u32, limit: usize) -> Result<Vec<PlayerSeasonStats>, DatabaseError>;
}
```

**TeamFinanceRepository**:
```rust
pub trait TeamFinanceRepository {
    fn get(&self, team_id: &str) -> Result<TeamFinance, DatabaseError>;
    fn update(&self, finance: &TeamFinance) -> Result<(), DatabaseError>;
    fn create(&self, finance: &TeamFinance) -> Result<(), DatabaseError>;
}
```

**FinanceTransactionRepository**:
```rust
pub trait FinanceTransactionRepository {
    fn create(&self, transaction: &FinanceTransaction) -> Result<(), DatabaseError>;
    fn get_by_team(&self, team_id: &str, limit: usize) -> Result<Vec<FinanceTransaction>, DatabaseError>;
    fn get_by_date_range(&self, team_id: &str, start: &GameDate, end: &GameDate) -> Result<Vec<FinanceTransaction>, DatabaseError>;
    fn get_by_type(&self, team_id: &str, t_type: TransactionType) -> Result<Vec<FinanceTransaction>, DatabaseError>;
}
```

**SeasonFinanceReportRepository**:
```rust
pub trait SeasonFinanceReportRepository {
    fn get(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError>;
    fn get_or_create(&self, team_id: &str, season: &str) -> Result<SeasonFinanceReport, DatabaseError>;
    fn update(&self, report: &SeasonFinanceReport) -> Result<(), DatabaseError>;
    fn get_history(&self, team_id: &str, limit: usize) -> Result<Vec<SeasonFinanceReport>, DatabaseError>;
}
```

## 错误处理

```rust
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Query failed: {0}")]
    QueryError(String),

    #[error("Migration failed: {0}")]
    MigrationError(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Unique constraint violated: {0}")]
    UniqueViolation(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}
```

## 存档系统

### 存档存储方案

**方案：每个存档一个独立的 SQLite 文件**

每个存档使用一个独立的 SQLite 数据库文件，文件名包含存档标识和元信息。这种方案的优势：
- 完全隔离：每个存档独立，互不影响
- 易于管理：备份/删除/移动只需操作单个文件
- 性能好：只加载需要的存档数据
- 符合现有设计：GameState 已有 `db_path` 字段

### 目录结构

```
saves/
├── save_001_20260201_143022_MyTeam.db
├── save_002_20260201_150000_OtherTeam.db
└── .metadata.json  # 可选：用于快速查询存档列表
```

### 文件命名规则

格式：`save_{slot:03d}_{timestamp}_{team_name}.db`

- `slot`: 存档槽位号（001-999）
- `timestamp`: 保存时间戳（YYYYMMDD_HHMMSS）
- `team_name`: 玩家球队名称（清理后的文件名安全字符）

示例：
- `save_001_20260201_143022_Manchester_United.db`
- `save_002_20260201_150000_Real_Madrid.db`

### 存档元数据索引（可选优化）

为了快速列出存档而不扫描所有文件，可以维护一个轻量级索引：

```json
// saves/.metadata.json
{
  "saves": [
    {
      "slot": 1,
      "file_path": "save_001_20260201_143022_MyTeam.db",
      "player_team_name": "My Team",
      "current_round": 5,
      "saved_at": 1706784622,
      "play_time": 3600,
      "game_id": "uuid-string"
    }
  ],
  "version": "1.0"
}
```

### SaveManager API

```rust
pub struct SaveManager {
    saves_dir: PathBuf,
}

/// 存档元数据（用于存档列表显示）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveMetadata {
    pub slot: u8,
    pub save_id: String,
    pub file_path: String,
    pub player_team_name: String,
    pub current_round: u32,
    pub saved_at: u64,      // Unix timestamp
    pub play_time: u64,     // 游戏时间（秒）
    pub game_id: String,
    pub save_version: String,
}

impl SaveManager {
    /// 创建存档管理器
    pub fn new(saves_dir: impl AsRef<Path>) -> Result<Self, SaveError>;

    /// 保存游戏
    pub fn save_game(
        &self,
        slot: u8,
        state: &GameState,
        db: &Database,
    ) -> Result<(), SaveError>;

    /// 加载游戏
    pub fn load_game(&self, slot: u8) -> Result<GameState, SaveError>;

    /// 列出所有存档
    pub fn list_saves(&self) -> Result<Vec<SaveMetadata>, SaveError>;

    /// 删除存档
    pub fn delete_save(&self, slot: u8) -> Result<(), SaveError>;

    /// 备份存档到指定目录
    pub fn backup_save(
        &self,
        slot: u8,
        backup_dir: impl AsRef<Path>
    ) -> Result<PathBuf, SaveError>;

    /// 生成存档文件名
    fn generate_filename(&self, slot: u8, state: &GameState) -> String;

    /// 从数据库读取 GameState 元数据
    fn load_metadata_from_db(&self, db: &Database) -> Result<GameState, SaveError>;

    /// 将 GameState 元数据保存到数据库
    fn save_metadata_to_db(&self, db: &Database, state: &GameState) -> Result<(), SaveError>;
}
```

### 存档流程

#### 保存流程

1. 生成文件名（包含槽位、时间戳、球队名）
2. 保存 GameState 元数据到当前数据库的 `game_metadata` 表
3. 复制数据库文件到存档目录（或直接使用当前数据库文件）
4. 更新元数据索引文件（如果使用）

#### 加载流程

1. 根据槽位查找存档文件
2. 打开存档数据库
3. 从 `game_metadata` 表恢复 GameState
4. 其他数据（teams, players 等）通过 Repository 从数据库加载

### 数据库迁移

每个存档数据库都需要运行迁移。在创建新存档时：

```rust
pub fn create_new_save(slot: u8, initial_state: GameState) -> Result<Database, SaveError> {
    let save_manager = SaveManager::new("saves")?;
    let filename = save_manager.generate_filename(slot, &initial_state)?;
    let db_path = save_manager.saves_dir.join(&filename);

    // 创建新数据库
    let db = Database::new(db_path.to_str().unwrap())?;
    
    // 运行迁移
    db.run_migrations()?;

    // 保存初始状态
    save_manager.save_metadata(&db, &initial_state)?;

    Ok(db)
}
```

### 版本兼容性

在 `game_metadata` 表中添加版本字段：

```sql
INSERT INTO game_metadata VALUES ('save_version', '1.0');
```

加载存档时检查版本，支持不同版本的存档格式迁移。

### 性能考虑

1. **延迟加载**: 只加载当前需要的存档数据
2. **索引优化**: 使用元数据索引文件避免扫描所有存档
3. **压缩**: 可选，对不常用的存档进行 SQLite 压缩（VACUUM）

## 迁移策略

使用 refinery 管理数据库版本：

```
migrations/
├── V1__initial_schema.sql        # teams, players, leagues, matches 基础表
├── V2__add_game_metadata.sql     # game_metadata 表
├── V3__add_transfer_market.sql   # transfer_market 表
├── V4__add_scheduled_matches.sql # scheduled_matches 表
├── V5__add_lineups.sql           # lineups 表
├── V6__add_team_statistics.sql   # team_statistics 表
└── ...
```

### 迁移文件示例

```sql
-- V4__add_scheduled_matches.sql
CREATE TABLE scheduled_matches (
    id TEXT PRIMARY KEY,
    league_id TEXT NOT NULL,
    round_number INTEGER NOT NULL,
    home_team_id TEXT NOT NULL,
    away_team_id TEXT NOT NULL,
    played INTEGER DEFAULT 0,
    FOREIGN KEY (league_id) REFERENCES leagues(id),
    FOREIGN KEY (home_team_id) REFERENCES teams(id),
    FOREIGN KEY (away_team_id) REFERENCES teams(id)
);

CREATE INDEX idx_scheduled_matches_league ON scheduled_matches(league_id);
CREATE INDEX idx_scheduled_matches_round ON scheduled_matches(round_number);
```

## 错误处理

```rust
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Query failed: {0}")]
    QueryError(String),

    #[error("Migration failed: {0}")]
    MigrationError(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Unique constraint violated: {0}")]
    UniqueViolation(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, thiserror::Error)]
pub enum SaveError {
    #[error("存档文件不存在: {0}")]
    SaveNotFound(u8),

    #[error("存档文件损坏: {0}")]
    CorruptedSave(String),

    #[error("不支持的存档版本: {0}")]
    UnsupportedVersion(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("数据库错误: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("序列化错误: {0}")]
    SerializationError(String),
}
```

## 测试策略

1. **单元测试**: 每个Repository的CRUD操作
2. **集成测试**: 使用内存数据库测试完整流程
3. **迁移测试**: 确保schema迁移正确
4. **存档测试**: SaveManager 的各个方法，完整的保存-加载循环
5. **兼容性测试**: 不同版本存档的加载
6. **性能测试**: 大量存档的列表和加载性能
