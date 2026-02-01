# LLM Integration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 为 FootballManagerAI 添加 OpenAI 兼容的大语言模型支持，实现真正的 AI 功能（数据生成、事件生成、比赛解说）

**Architecture:** 创建 `ai/llm` 模块，使用 OpenAI 兼容 API。支持流式输出，通过 `config.json` 配置，API 不可用时回退到随机生成器。

**Tech Stack:** Rust, async-openai (或 reqwest+serde), tokio, futures, serde_json

---

## Task 1: 添加依赖到 Cargo.toml

**Files:**
- Modify: `Cargo.toml`

**Step 1: 添加 LLM 相关依赖**

```toml
[dependencies]
# 现有依赖保持不变...

# LLM 相关 - 使用 reqwest + serde 实现 OpenAI 兼容 API
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
futures = "0.3"
async-stream = "0.3"
```

**Step 2: 运行 cargo build 验证依赖解析**

Run: `cargo build --release`
Expected: 成功下载和编译依赖

**Step 3: 提交**

```bash
git add Cargo.toml Cargo.lock
git commit -m "feat(llm): add reqwest, tokio, futures dependencies

Prepare for LLM integration with OpenAI-compatible APIs.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 2: 创建 LLM 错误类型

**Files:**
- Create: `src/ai/llm/error.rs`

**Step 1: 创建错误类型文件**

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

    #[error("API Key 未配置")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, LlmError>;
```

**Step 2: 创建 LLM 模块基础文件**

创建: `src/ai/llm/mod.rs`

```rust
mod error;
mod client;
mod config;
mod prompts;

pub use error::{LlmError, Result};
pub use client::LLMClient;
pub use config::LlmConfig;
```

**Step 3: 更新 ai/mod.rs 导出**

修改: `src/ai/mod.rs`

在文件末尾添加：

```rust
// 新增 LLM 模块
pub mod llm;
```

**Step 4: 运行 cargo build 验证编译**

Run: `cargo build --release`
Expected: 编译失败（client 和 config 还不存在）

**Step 5: 提交**

```bash
git add src/ai/llm/
git commit -m "feat(llm): add error types and module structure

Define LlmError with all error cases for API integration.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 3: 实现配置管理

**Files:**
- Create: `src/ai/llm/config.rs`
- Create: `config.json.example`

**Step 1: 编写配置测试**

创建: `src/ai/llm/config_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = LlmConfig::default();
        assert_eq!(config.url, "https://api.openai.com/v1");
        assert_eq!(config.model_name, "gpt-3.5-turbo");
        assert!(config.api_key.is_none());
    }

    #[test]
    fn test_load_from_file() {
        // 测试从文件加载配置
    }

    #[test]
    fn test_is_valid() {
        let mut config = LlmConfig::default();
        assert!(!config.is_valid());

        config.api_key = Some("test-key".to_string());
        assert!(config.is_valid());
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test --lib llm::config_tests`
Expected: FAIL - LlmConfig 不存在

**Step 3: 实现配置结构体**

在: `src/ai/llm/config.rs`

```rust
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmConfig {
    pub url: String,
    #[serde(skip_serializing)]
    pub api_key: Option<String>,
    pub model_name: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            url: "https://api.openai.com/v1".to_string(),
            api_key: None,
            model_name: "gpt-3.5-turbo".to_string(),
        }
    }
}

impl LlmConfig {
    /// 从 config.json 加载配置
    pub fn load_or_default() -> Result<Self> {
        // 优先从环境变量读取 API Key
        let api_key = std::env::var("LLM_API_KEY").ok();

        if let Ok(content) = fs::read_to_string("config.json") {
            let mut config: LlmConfig = serde_json::from_str(&content)
                .map_err(|e| LlmError::Config(format!("解析配置文件失败: {}", e)))?;

            // 环境变量优先级更高
            if api_key.is_some() {
                config.api_key = api_key;
            }

            Ok(config)
        } else {
            // 文件不存在，返回默认配置
            let mut config = LlmConfig::default();
            config.api_key = api_key;
            Ok(config)
        }
    }

    /// 验证配置是否有效
    pub fn is_valid(&self) -> bool {
        !self.url.is_empty()
            && !self.model_name.is_empty()
            && self.api_key.is_some()
            && self.api_key.as_ref().unwrap().len() > 10
    }

    /// 创建示例配置文件
    pub fn create_example() -> Result<()> {
        if Path::new("config.json").exists() {
            return Err(LlmError::Config("config.json 已存在".to_string()));
        }

        let example = LlmConfig {
            url: "https://api.deepseek.com/v1".to_string(),
            api_key: Some("sk-xxxxxxxxxxxxxxxx".to_string()),
            model_name: "deepseek-chat".to_string(),
        };

        let json = serde_json::to_string_pretty(&example)
            .map_err(|e| LlmError::Config(format!("序列化失败: {}", e)))?;

        fs::write("config.json", json)
            .map_err(|e| LlmError::Config(format!("写入文件失败: {}", e)))?;

        Ok(())
    }
}
```

**Step 4: 在 mod.rs 中导出**

修改: `src/ai/llm/mod.rs`

```rust
mod error;
mod client;
mod config;
mod prompts;

pub use error::{LlmError, Result};
pub use client::LLMClient;
pub use config::LlmConfig;
```

**Step 5: 运行测试验证通过**

Run: `cargo test --lib llm::config_tests`
Expected: PASS

**Step 6: 创建配置文件示例**

创建: `config.json.example`

```json
{
  "url": "https://api.deepseek.com/v1",
  "apiKey": "sk-xxxxxxxxxxxxxxxx",
  "modelName": "deepseek-chat"
}
```

**Step 7: 更新 .gitignore**

修改: `.gitignore`

添加:
```
config.json
```

**Step 8: 提交**

```bash
git add src/ai/llm/config.rs config.json.example .gitignore
git commit -m "feat(llm): add configuration management

- Support loading from config.json
- Environment variable LLM_API_KEY takes precedence
- Validate configuration before use
- Create example config file

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 4: 实现 LLM 客户端（基础功能）

**Files:**
- Create: `src/ai/llm/client.rs`

**Step 1: 编写客户端测试**

创建: `src/ai/llm/client_tests.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = LlmConfig {
            url: "https://api.example.com/v1".to_string(),
            api_key: Some("test-key-123456789012".to_string()),
            model_name: "test-model".to_string(),
        };

        let client = LLMClient::new(config);
        assert_eq!(client.model(), "test-model");
    }
}
```

**Step 2: 运行测试验证失败**

Run: `cargo test --lib llm::client_tests`
Expected: FAIL - LLMClient 不存在

**Step 3: 实现客户端基础结构**

在: `src/ai/llm/client.rs`

```rust
use crate::ai::llm::{LlmConfig, LlmError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }

    pub fn system(content: &str) -> Self {
        Self {
            role: "system".to_string(),
            content: content.to_string(),
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u16,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

pub struct LLMClient {
    client: Client,
    config: LlmConfig,
}

impl LLMClient {
    pub fn new(config: LlmConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("创建 HTTP 客户端失败");

        Self { client, config }
    }

    pub fn model(&self) -> &str {
        &self.config.model_name
    }

    /// 发送聊天请求
    pub async fn chat(&self, messages: &[Message]) -> Result<String> {
        let api_key = self.config.api_key.as_ref()
            .ok_or(LlmError::MissingApiKey)?;

        let url = format!("{}/chat/completions", self.config.url);

        let request = ChatRequest {
            model: self.config.model_name.clone(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 2000,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(LlmError::ApiError {
                message: format!("HTTP {}: {}", status, error_text),
                code: Some(status.as_u16().to_string()),
            });
        }

        let chat_response: ChatResponse = response.json().await?;
        let content = chat_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::ApiError {
                message: "API 返回空响应".to_string(),
                code: None,
            })?;

        Ok(content)
    }
}
```

**Step 4: 运行测试验证通过**

Run: `cargo test --lib llm::client_tests`
Expected: PASS

**Step 5: 提交**

```bash
git add src/ai/llm/client.rs
git commit -m "feat(llm): add basic LLM client

- Implement OpenAI-compatible chat API client
- Support configurable endpoint and model
- Error handling for API failures
- Timeout configuration (30s)

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 5: 实现流式输出

**Files:**
- Modify: `src/ai/llm/client.rs`

**Step 1: 添加流式输出测试**

在: `src/ai/llm/client_tests.rs`

```rust
    #[tokio::test]
    async fn test_stream_response() {
        // Mock API 测试
        // 实际测试需要 mock server
    }
```

**Step 2: 实现流式输出方法**

在: `src/ai/llm/client.rs` 添加：

```rust
use futures::stream::{Stream, StreamExt};
use async_stream::stream;

#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(default)]
    content: String,
}

impl LLMClient {
    /// 发送流式聊天请求
    pub async fn chat_stream(&self, messages: &[Message])
        -> impl Stream<Item = Result<String>> {
        let api_key = match self.config.api_key.as_ref() {
            Some(key) => key.clone(),
            None => return stream! { yield Err(LlmError::MissingApiKey); },
        };

        let url = format!("{}/chat/completions", self.config.url);
        let request = ChatRequest {
            model: self.config.model_name.clone(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 2000,
        };

        let client = self.client.clone();

        stream! {
            let response = client
                .post(&url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .header("Accept", "text/event-stream")
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_default();
                yield Err(LlmError::ApiError {
                    message: format!("HTTP {}: {}", status, error_text),
                    code: Some(status.as_u16().to_string()),
                });
                return;
            }

            let mut stream = response.bytes_stream();

            use futures::StreamExt;
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.map_err(|e| LlmError::Request(e))?;

                let text = String::from_utf8_lossy(&chunk);
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            return;
                        }

                        if let Ok(chunk) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(choice) = chunk.choices.first() {
                                if !choice.delta.content.is_empty() {
                                    yield Ok(choice.delta.content.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

**Step 3: 提交**

```bash
git add src/ai/llm/client.rs
git commit -m "feat(llm): add streaming response support

- Implement chat_stream method returning Stream
- Parse SSE (Server-Sent Events) format
- Yield content chunks as they arrive
- Error handling for stream interruptions

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 6: 实现提示词模板

**Files:**
- Create: `src/ai/llm/prompts.rs`

**Step 1: 实现提示词模板**

```rust
/// 提示词模板
pub struct PromptTemplates;

impl PromptTemplates {
    /// 生成球员数据的提示词
    pub fn generate_player(position: &str, skill_level: u16, age: u8) -> String {
        format!(
            "你是一个足球经理游戏的数据生成器。请生成一名球员的JSON数据：

位置: {}
能力等级: {} (1-200)
年龄: {}

返回JSON格式（只返回JSON，不要其他文字）：
{{
  \"name\": \"球员姓名\",
  \"nationality\": \"国籍\",
  \"attributes\": {{
    \"finishing\": 1-200,
    \"passing\": 1-200,
    \"pace\": 1-200,
    \"stamina\": 1-200,
    \"strength\": 1-200
  }}
}}",
            position, skill_level, age
        )
    }

    /// 生成球队信息的提示词
    pub fn generate_team(league_name: &str) -> String {
        format!(
            "生成一个位于{}的足球队JSON数据：

返回JSON格式（只返回JSON，不要其他文字）：
{{
  \"name\": \"球队名称\",
  \"abbreviation\": \"缩写\",
  \"founded_year\": 年份,
  \"stadium\": {{
    \"name\": \"球场名称\",
    \"capacity\": 容量
  }}
}}",
            league_name
        )
    }

    /// 生成比赛解说的提示词
    pub fn match_commentary(event: &str, minute: u8, teams: &str) -> String {
        format!(
            "你是足球解说员。请为以下比赛事件生成一句解说：

事件: {}
时间: {}分钟
对阵: {}

要求：简短有力，15-30字，只返回解说文字，不要其他内容。",
            event, minute, teams
        )
    }

    /// 生成新闻事件的提示词
    pub fn generate_news(event_type: &str, team_name: &str) -> String {
        format!(
            "你是体育新闻记者。请为以下事件生成一则新闻：

事件类型: {}
球队: {}

要求：简短标题（10-20字）+ 正文（50-100字），只返回新闻内容，不要其他内容。",
            event_type, team_name
        )
    }
}
```

**Step 2: 提交**

```bash
git add src/ai/llm/prompts.rs
git commit -m "feat(llm): add prompt templates

- Templates for player generation
- Templates for team generation
- Templates for match commentary
- Templates for news events

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 7: 创建数据生成器基础结构

**Files:**
- Create: `src/ai/llm/generators/mod.rs`

**Step 1: 创建生成器模块**

```rust
pub mod player_gen;
pub mod team_gen;
pub mod game_gen;

pub use player_gen::PlayerGenerator;
pub use team_gen::TeamGenerator;
pub use game_gen::GameDataGenerator;
```

**Step 2: 更新 llm/mod.rs**

修改: `src/ai/llm/mod.rs`

```rust
mod error;
mod client;
mod config;
mod prompts;
pub mod generators;

pub use error::{LlmError, Result};
pub use client::{LLMClient, Message};
pub use config::LlmConfig;
pub use generators::{PlayerGenerator, TeamGenerator, GameDataGenerator};
```

**Step 3: 提交**

```bash
git add src/ai/llm/generators/
git commit -m "feat(llm): create generators module structure

Prepare for player, team, and game data generators.

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 8: 实现球员生成器

**Files:**
- Create: `src/ai/llm/generators/player_gen.rs`

**Step 1: 编写球员生成测试**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_player() {
        // 需要 mock API
    }

    #[test]
    fn test_parse_player_response() {
        let json = r#"{"name":"张三","nationality":"中国","attributes":{"finishing":150,"passing":140,"pace":160,"stamina":145,"strength":155}}"#;
        let result = parse_player_json(json).unwrap();
        assert_eq!(result.name, "张三");
    }
}
```

**Step 2: 实现球员生成器**

```rust
use crate::ai::llm::{LLMClient, Result, LlmError, PromptTemplates};
use crate::team::Player;
use crate::team::Position;
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct PlayerAttributes {
    finishing: u16,
    passing: u16,
    pace: u16,
    stamina: u16,
    strength: u16,
}

#[derive(Debug, Deserialize)]
struct LlmPlayerData {
    name: String,
    nationality: String,
    attributes: PlayerAttributes,
}

pub struct PlayerGenerator {
    client: LLMClient,
}

impl PlayerGenerator {
    pub fn new(client: LLMClient) -> Self {
        Self { client }
    }

    pub async fn generate(&self, position: Position, skill: u16, age: u8)
        -> Result<Player> {
        let prompt = PromptTemplates::generate_player(
            &position.to_string(),
            skill,
            age
        );

        let response = self.client.chat(&[
            Message::system("你是足球经理游戏的数据生成助手。"),
            Message::user(&prompt)
        ]).await?;

        self.parse_response(&response, position)
    }

    fn parse_response(&self, json: &str, position: Position) -> Result<Player> {
        // 尝试提取 JSON（去除可能的 markdown 代码块）
        let json = extract_json(json)?;

        let data: LlmPlayerData = serde_json::from_str(json)
            .map_err(|e| LlmError::JsonParse(e))?;

        let mut player = Player::new(
            uuid::Uuid::new_v4().to_string(),
            data.name,
            position,
        );

        // 设置 LLM 生成的属性
        player.finishing = data.attributes.finishing;
        player.passing = data.attributes.passing;
        player.pace = data.attributes.pace;
        player.stamina = data.attributes.stamina;
        player.strength = data.attributes.strength;

        Ok(player)
    }
}

fn extract_json(text: &str) -> Result<&str> {
    let text = text.trim();

    // 去除 markdown 代码块
    if text.starts_with("```json") {
        let start = 7; // "```json".len()
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else if text.starts_with("```") {
        let start = 3;
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else {
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_from_code_block() {
        let input = r#"```json
{"name": "Test"}
```"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result.trim(), r#"{"name": "Test"}"#);
    }

    #[test]
    fn test_extract_json_plain() {
        let input = r#"{"name": "Test"}"#;
        let result = extract_json(input).unwrap();
        assert_eq!(result, r#"{"name": "Test"}"#);
    }
}
```

**Step 3: 提交**

```bash
git add src/ai/llm/generators/player_gen.rs
git commit -m "feat(llm): add player generator using LLM

- Generate players via LLM API
- Parse JSON response
- Handle markdown code blocks
- Set player attributes from LLM response

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 9: 实现球队生成器

**Files:**
- Create: `src/ai/llm/generators/team_gen.rs`

**Step 1: 实现球队生成器**

```rust
use crate::ai::llm::{LLMClient, Result, LlmError, PromptTemplates};
use crate::team::Team;

#[derive(Debug, serde::Deserialize)]
struct StadiumData {
    name: String,
    capacity: u32,
}

#[derive(Debug, serde::Deserialize)]
struct LlmTeamData {
    name: String,
    abbreviation: String,
    founded_year: u32,
    stadium: StadiumData,
}

pub struct TeamGenerator {
    client: LLMClient,
}

impl TeamGenerator {
    pub fn new(client: LLMClient) -> Self {
        Self { client }
    }

    pub async fn generate(&self, league_name: &str, team_id: String)
        -> Result<Team> {
        let prompt = PromptTemplates::generate_team(league_name);

        let response = self.client.chat(&[
            Message::system("你是足球经理游戏的数据生成助手。"),
            Message::user(&prompt)
        ]).await?;

        self.parse_response(&response, team_id, league_name)
    }

    fn parse_response(&self, json: &str, team_id: String, league_id: &str)
        -> Result<Team> {
        let json = extract_json(json)?;

        let data: LlmTeamData = serde_json::from_str(json)
            .map_err(|e| LlmError::JsonParse(e))?;

        let mut team = Team::new(
            team_id,
            data.name,
            league_id.to_string(),
            10_000_000, // 默认预算
        );

        // 可以扩展 Team 结构体来存储更多信息
        // team.abbreviation = data.abbreviation;
        // team.founded_year = data.founded_year;
        // team.stadium_name = data.stadium.name;
        // team.stadium_capacity = data.stadium.capacity;

        Ok(team)
    }
}

fn extract_json(text: &str) -> Result<&str> {
    let text = text.trim();

    if text.starts_with("```json") {
        let start = 7;
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else if text.starts_with("```") {
        let start = 3;
        let end = text.find("```").unwrap_or(text.len());
        Ok(text[start..end].trim())
    } else {
        Ok(text)
    }
}
```

**Step 2: 提交**

```bash
git add src/ai/llm/generators/team_gen.rs
git commit -m "feat(llm): add team generator using LLM

- Generate teams via LLM API
- Parse team data including stadium info
- Extract JSON from markdown code blocks

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 10: 实现统一游戏数据生成器

**Files:**
- Create: `src/ai/llm/generators/game_gen.rs`

**Step 1: 实现游戏数据生成器**

```rust
use crate::ai::llm::{LLMClient, Result, PlayerGenerator, TeamGenerator};
use crate::team::{Team, League, Position};
use crate::game::GameState;
use futures::stream::{Stream, StreamExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct GameDataGenerator {
    client: LLMClient,
    player_gen: PlayerGenerator,
    team_gen: TeamGenerator,
}

impl GameDataGenerator {
    pub fn new(client: LLMClient) -> Self {
        let player_gen = PlayerGenerator::new(client.clone());
        let team_gen = TeamGenerator::new(client.clone());

        Self {
            client,
            player_gen,
            team_gen,
        }
    }

    /// 生成所有游戏数据
    pub async fn generate_all(&mut self) -> Result<(GameState, crate::data::Db)> {
        // 1. 生成联赛
        let league = self.generate_league().await?;

        // 2. 生成所有球队
        let teams = self.generate_teams(&league, 20).await?;

        // 3. 生成所有球员
        let players = self.generate_all_players(&teams).await?;

        // 4. 初始化数据库
        let db = crate::data::Db::new(":memory:")?;

        // 5. 创建游戏状态
        let game_state = GameState::new(league, teams, players);

        Ok((game_state, db))
    }

    async fn generate_league(&self) -> Result<League> {
        let league_name = "Premier League".to_string();
        let mut teams = Vec::new();

        for i in 0..20 {
            teams.push(format!("team-{}", i));
        }

        let mut league = League::new(
            "league1".to_string(),
            league_name,
            teams,
        );

        league.generate_schedule();
        Ok(league)
    }

    async fn generate_teams(&self, league: &League, count: usize)
        -> Result<Vec<Team>> {
        let mut teams = Vec::new();

        for i in 0..count {
            let team_id = format!("team-{}", i);
            let team = self.team_gen.generate(&league.name, team_id).await?;
            teams.push(team);
        }

        Ok(teams)
    }

    async fn generate_all_players(&self, teams: &[Team])
        -> Result<Vec<crate::team::Player>> {
        let mut players = Vec::new();

        for team in teams {
            for i in 0..25 {
                let position = match i {
                    0 => Position::GK,
                    1..=4 => Position::CB,
                    5 => Position::LB,
                    6 => Position::RB,
                    7..=10 => Position::CM,
                    11 => Position::LW,
                    12 => Position::RW,
                    13..=15 => Position::ST,
                    _ => Position::CM,
                };

                let player = self.player_gen.generate(position, 100, 25).await?;
                players.push(player);
            }
        }

        Ok(players)
    }
}
```

**Step 2: 提交**

```bash
git add src/ai/llm/generators/game_gen.rs
git commit -m "feat(llm): add unified game data generator

- Generate league, teams, and players via LLM
- Coordinate all generation steps
- Return complete GameState

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 11: 更新游戏初始化以支持 LLM

**Files:**
- Modify: `src/game/init.rs`

**Step 1: 添加 LLM 初始化函数**

```rust
use crate::ai::llm::{LlmConfig, GameDataGenerator};

/// 使用 LLM 生成游戏数据
pub async fn quick_start_with_llm() -> Result<(GameState, Db>, Box<dyn std::error::Error>> {
    let config = LlmConfig::load_or_default()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    // 检查配置是否有效
    if !config.is_valid() {
        println!("LLM 配置无效，回退到随机生成...");
        return quick_start();
    }

    println!("使用 LLM 生成游戏数据...");

    let client = LLMClient::new(config);
    let mut generator = GameDataGenerator::new(client);

    match generator.generate_all().await {
        Ok(result) => {
            println!("游戏数据生成完成！");
            Ok(result)
        }
        Err(e) => {
            println!("LLM 生成失败: {}, 回退到随机生成...", e);
            quick_start()
        }
    }
}
```

**Step 2: 提交**

```bash
git add src/game/init.rs
git commit -m "feat(game): add LLM-based game initialization

- Add quick_start_with_llm function
- Fallback to random generation on error
- Validate configuration before use

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 12: 集成到 main 函数

**Files:**
- Modify: `src/main.rs`

**Step 1: 更新 main 函数使用 LLM**

```rust
fn setup_game() -> Result<GameState, Box<dyn std::error::Error>> {
    // Check for existing saves
    let save_manager = SaveManager::default();
    let saves = save_manager.list_saves()?;

    if !saves.is_empty() {
        // Load the most recent save
        println!("Loading saved game from slot {}...", saves[0].slot);
        let (game_state, _db) = load_game(saves[0].slot)?;
        println!("Game loaded successfully!");
        return Ok(game_state);
    }

    // No saves found, create new game
    println!("No saved games found. Creating new game...");

    // 尝试使用 LLM，需要运行时
    let rt = tokio::runtime::Runtime::new()?;
    let (game_state, _db) = rt.block_on(football_manager_ai::game::init::quick_start_with_llm())?;

    println!("Game created successfully!");

    Ok(game_state)
}
```

**Step 2: 提交**

```bash
git add src/main.rs
git commit -m "feat(main): integrate LLM game initialization

- Use LLM for new game generation
- Keep tokio runtime for async operations
- Fallback to random generation on error

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 13: 编写文档和示例

**Files:**
- Create: `docs/llm-integration.md`
- Update: `README.md`

**Step 1: 创建 LLM 集成文档**

创建: `docs/llm-integration.md`

```markdown
# LLM 集成指南

## 配置

1. 复制 `config.json.example` 为 `config.json`
2. 填入你的 API 信息：

\`\`\`json
{
  "url": "https://api.deepseek.com/v1",
  "apiKey": "sk-your-api-key",
  "modelName": "deepseek-chat"
}
\`\`\`

## 环境变量

也可以使用环境变量（优先级更高）：

\`\`\`bash
export LLM_API_KEY="sk-your-api-key"
\`\`\`

## 支持的 API

任何 OpenAI 兼容的 API：
- DeepSeek
- 通义千问
- Azure OpenAI
- 其他第三方服务

## 回退机制

如果 LLM 不可用，游戏会自动回退到随机生成。
```

**Step 2: 更新 README**

添加 LLM 功能说明到 README.md

**Step 3: 提交**

```bash
git add docs/llm-integration.md README.md
git commit -m "docs: add LLM integration documentation

- Configuration guide
- Supported APIs
- Fallback mechanism explanation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## Task 14: 最终测试和验证

**Step 1: 构建项目**

Run: `cargo build --release`
Expected: 成功编译

**Step 2: 测试配置加载**

Run: `cargo test --lib llm::config_tests`
Expected: 所有测试通过

**Step 3: 测试客户端**

Run: `cargo test --lib llm::client_tests`
Expected: 所有测试通过

**Step 4: 测试球员生成器**

Run: `cargo test --lib player_gen`
Expected: 所有测试通过

**Step 5: 手动测试（可选）**

1. 创建 `config.json` 填入真实 API 密钥
2. 运行程序验证游戏启动
3. 验证生成的数据质量

**Step 6: 最终提交**

```bash
git add .
git commit -m "feat(llm): complete LLM integration

Full OpenAI-compatible LLM integration:
- Configuration management
- Streaming response support
- Player, team, and game data generators
- Integrated with game initialization
- Fallback to random generation
- Documentation

Co-Authored-By: Claude Sonnet 4.5 <noreply@anthropic.com>"
```

---

## 后续优化建议（不在本次实现范围）

1. **性能优化**
   - 批量生成减少 API 调用
   - 缓存已生成数据

2. **功能扩展**
   - 比赛实时解说
   - 动态事件生成
   - 新闻系统

3. **用户体验**
   - 进度条显示
   - 打字机效果
   - 生成参数配置

4. **测试**
   - 添加 mock server 测试
   - 集成测试覆盖
