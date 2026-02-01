//! LLM integration error types
//!
//! This module defines all error types that can occur during LLM API interactions,
//! including configuration errors, request failures, and response parsing issues.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LlmError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("API request failed: {0}")]
    Request(#[from] reqwest::Error),

    #[error("API error (code: {code:?}): {message}")]
    ApiError { message: String, code: Option<String> },

    #[error("JSON parse failed: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Stream closed unexpectedly")]
    StreamClosed,

    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    #[error("API key not configured")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, LlmError>;
