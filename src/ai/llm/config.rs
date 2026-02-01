//! LLM configuration management
//!
//! This module handles loading and validating LLM configuration from both
//! config.json files and environment variables.

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::ai::llm::error::{LlmError, Result};

/// Configuration for LLM API connections
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmConfig {
    /// API base URL (e.g., "https://api.openai.com/v1")
    pub url: String,
    /// API key for authentication (optional in config, can be from env)
    #[serde(skip_serializing, rename = "apiKey")]
    pub api_key: Option<String>,
    /// Model name to use (e.g., "gpt-3.5-turbo", "deepseek-chat")
    #[serde(rename = "modelName")]
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
    /// Load configuration from config.json or return defaults
    ///
    /// Priority order:
    /// 1. Environment variable LLM_API_KEY (highest)
    /// 2. config.json file
    /// 3. Default values
    pub fn load_or_default() -> Result<Self> {
        // Check environment variable first
        let api_key = std::env::var("LLM_API_KEY").ok();

        if let Ok(content) = fs::read_to_string("config.json") {
            let mut config: LlmConfig = serde_json::from_str(&content)
                .map_err(|e| LlmError::Config(format!("Failed to parse config file: {}", e)))?;

            // Environment variable takes precedence over file
            if api_key.is_some() {
                config.api_key = api_key;
            }

            Ok(config)
        } else {
            // File doesn't exist, return default with env var override
            let mut config = LlmConfig::default();
            config.api_key = api_key;
            Ok(config)
        }
    }

    /// Validate that this configuration has all required fields
    pub fn is_valid(&self) -> bool {
        !self.url.is_empty()
            && !self.model_name.is_empty()
            && self.api_key.is_some()
            && self.api_key.as_ref().unwrap().len() > 10
    }

    /// Create an example config.json file
    ///
    /// Returns an error if config.json already exists
    pub fn create_example() -> Result<()> {
        if Path::new("config.json").exists() {
            return Err(LlmError::Config("config.json already exists".to_string()));
        }

        let example = LlmConfig {
            url: "https://api.deepseek.com/v1".to_string(),
            api_key: Some("sk-xxxxxxxxxxxxxxxx".to_string()),
            model_name: "deepseek-chat".to_string(),
        };

        let json = serde_json::to_string_pretty(&example)
            .map_err(|e| LlmError::Config(format!("Serialization failed: {}", e)))?;

        fs::write("config.json", json)
            .map_err(|e| LlmError::Config(format!("Failed to write file: {}", e)))?;

        Ok(())
    }
}

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
    fn test_is_valid() {
        let mut config = LlmConfig::default();
        assert!(!config.is_valid()); // No API key

        config.api_key = Some("test-key-123456789012".to_string());
        assert!(config.is_valid()); // Has API key

        config.api_key = Some("short".to_string());
        assert!(!config.is_valid()); // API key too short
    }

    #[test]
    fn test_is_valid_empty_fields() {
        let mut config = LlmConfig::default();
        config.api_key = Some("test-key-123456789012".to_string());

        config.url = String::new();
        assert!(!config.is_valid());

        config.url = "https://api.example.com".to_string();
        config.model_name = String::new();
        assert!(!config.is_valid());
    }
}
