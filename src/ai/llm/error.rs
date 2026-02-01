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
