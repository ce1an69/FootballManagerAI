# LLM 模块集成设计文档

**日期:** 2026-02-01
**作者:** Claude & User
**状态:** 设计阶段

## 1. 概述

为 FootballManagerAI 添加 OpenAI 兼容的大语言模型支持，实现真正的 AI 功能。主要用途包括：

- 游戏启动时动态生成所有游戏数据（球员、球队、联赛等）
- 生成动态事件和新闻故事
- 参与比赛模拟和生成解说

## 2. 模块结构

```
src/ai/
├── mod.rs                # 现有 AI 模块导出 + LLM 导出
├── generator.rs          # 现有：随机生成器（作为回退）
├── match_sim.rs          # 现有：比赛模拟
├── tactical.rs           # 现有：战术
├── substitution.rs       # 现有：换人
├── progression.rs        # 现有：球员成长
├── transfer_decision.rs  # 现有：转会决策
├── events.rs             # 现有：事件
└── llm/
    ├── mod.rs            # LLM 模块导出
    ├── client.rs         # LLM 客户端
    ├── config.rs         # 配置管理
    ├── error.rs          # 错误类型
    ├── prompts.rs        # 提示词模板
    └── generators/
        ├── mod.rs
        ├── player_gen.rs
        ├── team_gen.rs
        └── game_gen.rs   # 统一生成入口
```

## 3. 核心组件

### 3.1 LLM 客户端 (`ai/llm/client.rs`)

```rust
pub struct LLMClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl LLMClient {
    // 常规请求
    pub async fn chat(&self, messages: &[Message]) -> Result<String>;

    // 流式请求 - 返回异步流
    pub async fn chat_stream(&self, messages: &[Message])
        -> impl Stream<Item = Result<String>>;
}
```

### 3.2 配置管理 (`ai/llm/config.rs`)

**配置文件 (`config.json`):**

```json
{
  "url": "https://api.deepseek.com/v1",
  "apiKey": "sk-xxxxxxx",
  "modelName": "deepseek-chat"
}
```

**配置结构体:**

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct LlmConfig {
    pub url: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    pub model_name: String,
}
```

**配置加载优先级:**
1. 环境变量 `LLM_API_KEY`（最高优先级）
2. `config.json` 中的配置
3. 默认值（最低优先级）

**硬编码参数:**
- timeout: 30秒
- max_retries: 3次
- temperature: 0.7
- stream_enabled: true

### 3.3 提示词模板 (`ai/llm/prompts.rs`)

```rust
pub struct PromptTemplates;

impl PromptTemplates {
    pub fn generate_player(position: &str, skill_level: u16, age: u8) -> String;
    pub fn generate_team(league_name: &str) -> String;
    pub fn match_commentary(event: &str, minute: u8, teams: &str) -> String;
}
```

### 3.4 数据生成器 (`ai/llm/generators/`)

```rust
pub struct PlayerGenerator {
    client: LLMClient,
}

pub struct GameDataGenerator {
    client: LLMClient,
    player_gen: PlayerGenerator,
    team_gen: TeamGenerator,
}

impl GameDataGenerator {
    pub async fn generate_all(&mut self) -> Result<(GameState, Db)>;
    pub async fn generate_all_players(&self, teams: &[Team])
        -> impl Stream<Item = Player>;
}
```

### 3.5 错误处理 (`ai/llm/error.rs`)

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("配置错误: {0}")]
    Config(String),

    #[error("API 请求失败: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API 返回错误: {message}")]
    ApiError { message: String, code: Option<String> },

    #[error("JSON 解析失败: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("流式传输中断")]
    StreamClosed,

    #[error("重试次数耗尽")]
    MaxRetriesExceeded,
}
```

## 4. 游戏集成

### 4.1 游戏启动流程改造

```rust
// game/init.rs
pub async fn quick_start_with_llm() -> Result<(GameState, Db)> {
    let config = LlmConfig::load_or_default()?;

    if !config.is_valid() {
        return quick_start(); // 回退到随机生成
    }

    let llm_client = LLMClient::new(config);
    let mut generator = GameDataGenerator::new(llm_client);
    generator.generate_all().await
}
```

### 4.2 TUI 进度显示

```rust
// ui/screens/generation_screen.rs
pub struct GenerationScreen {
    progress: Arc<AtomicUsize>,
    total: usize,
    current_output: String,
}

impl GenerationScreen {
    pub async fn run(&mut self, mut player_stream: impl Stream<Item = Player>);
}
```

## 5. 依赖管理

**Cargo.toml 新增依赖:**

```toml
[dependencies]
async-openai = "0.26"  # 或 reqwest + serde
tokio = { version = "1", features = ["full"] }
futures = "0.3"
```

## 6. 流式输出应用场景

1. **游戏启动数据生成** - 显示进度条，实时显示正在生成的内容
2. **比赛解说** - 实时生成解说文字，打字机效果显示
3. **新闻和事件** - 逐字显示新闻内容

## 7. 错误处理与重试

- 网络超时：自动重试 3 次，指数退避
- API 限流：等待后重试
- Token 超限：自动截断或分批处理
- API 不可用：回退到随机生成器

## 8. 安全性考虑

- API Key 支持环境变量覆盖：`LLM_API_KEY`
- 配置文件中敏感信息可选，优先从环境变量读取
- 添加 `.gitignore` 防止误提交配置文件

## 9. 测试策略

1. **单元测试** - 提示词生成、配置解析
2. **集成测试** - Mock API 响应，测试完整流程
3. **可选的真实 API 测试** - 通过环境变量控制，CI 中跳过
