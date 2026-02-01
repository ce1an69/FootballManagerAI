//! LLM API client
//!
//! This module provides a client for interacting with LLM APIs that are compatible
//! with the OpenAI chat completions API format.

use crate::ai::llm::{LlmConfig, LlmError, Result};
use async_stream::stream;
use futures::stream::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::time::Duration;

/// Chat message with role and content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    /// Create a user message
    pub fn user(content: &str) -> Self {
        Self {
            role: "user".to_string(),
            content: content.to_string(),
        }
    }

    /// Create a system message
    pub fn system(content: &str) -> Self {
        Self {
            role: "system".to_string(),
            content: content.to_string(),
        }
    }

    /// Create an assistant message
    pub fn assistant(content: &str) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.to_string(),
        }
    }
}

/// Chat request payload
#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u16,
}

/// Chat response from API
#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

/// Single choice in chat response
#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

/// LLM API client
///
/// This client handles communication with LLM APIs using the OpenAI-compatible
/// chat completions format.
pub struct LLMClient {
    client: Client,
    config: LlmConfig,
}

impl LLMClient {
    /// Create a new LLM client with the given configuration
    pub fn new(config: LlmConfig) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    /// Get the model name being used
    pub fn model(&self) -> &str {
        &self.config.model_name
    }

    /// Send a chat completion request
    ///
    /// This method sends a list of messages to the LLM API and returns
    /// the assistant's response content.
    pub async fn chat(&self, messages: &[Message]) -> Result<String> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or(LlmError::MissingApiKey)?;

        let url = format!("{}/chat/completions", self.config.url);

        let request = ChatRequest {
            model: self.config.model_name.clone(),
            messages: messages.to_vec(),
            temperature: 0.7,
            max_tokens: 2000,
        };

        let response = self
            .client
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
        let content = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| LlmError::ApiError {
                message: "API returned empty response".to_string(),
                code: None,
            })?;

        Ok(content)
    }

    /// Send a streaming chat completion request
    ///
    /// This method sends a list of messages to the LLM API and returns
    /// a stream of content chunks as they arrive from the API.
    pub fn chat_stream(
        &self,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = Result<String>> + Send>> {
        let api_key = match self.config.api_key.as_ref() {
            Some(key) => key.clone(),
            None => {
                return Box::pin(stream! {
                    yield Err(LlmError::MissingApiKey);
                });
            }
        };

        let url = format!("{}/chat/completions", self.config.url);
        let request = ChatRequest {
            model: self.config.model_name.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 2000,
        };

        let client = self.client.clone();

        Box::pin(stream! {
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

            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.map_err(|e| LlmError::Request(e))?;

                let text = String::from_utf8_lossy(&chunk);
                for line in text.lines() {
                    if line.starts_with("data: ") {
                        let data = &line[6..];
                        if data == "[DONE]" {
                            return;
                        }

                        if let Ok(chunk_data) = serde_json::from_str::<StreamChunk>(data) {
                            if let Some(choice) = chunk_data.choices.first() {
                                if !choice.delta.content.is_empty() {
                                    yield Ok(choice.delta.content.clone());
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}

/// Streaming response chunk
#[derive(Debug, Deserialize)]
struct StreamChunk {
    choices: Vec<StreamChoice>,
}

/// Single choice in streaming response
#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: Delta,
}

/// Delta content in streaming response
#[derive(Debug, Deserialize)]
struct Delta {
    #[serde(default)]
    content: String,
}

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

    #[test]
    fn test_message_creation() {
        let msg = Message::user("Hello");
        assert_eq!(msg.role, "user");
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_system_message() {
        let msg = Message::system("You are a helpful assistant");
        assert_eq!(msg.role, "system");
        assert_eq!(msg.content, "You are a helpful assistant");
    }

    #[test]
    fn test_assistant_message() {
        let msg = Message::assistant("Hi there!");
        assert_eq!(msg.role, "assistant");
        assert_eq!(msg.content, "Hi there!");
    }

    #[test]
    fn test_multiple_messages() {
        let messages = vec![
            Message::system("You are a helpful assistant"),
            Message::user("What is 2+2?"),
        ];
        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
    }
}
