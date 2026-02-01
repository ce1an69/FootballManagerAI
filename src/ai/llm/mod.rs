mod error;
mod config;
mod client;      // TODO: Will be implemented in Task 4
// mod prompts;     // TODO: Will be implemented in Task 6

pub use error::{LlmError, Result};
pub use config::LlmConfig;
pub use client::LLMClient;
