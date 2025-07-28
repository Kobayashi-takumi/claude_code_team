use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Tmux error: {0}")]
    TmuxError(String),

    #[error("Agent error: {0}")]
    AgentError(String),

    #[error("Communication error: {0}")]
    CommunicationError(String),

    #[error("Task error: {0}")]
    TaskError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Environment error: {0}")]
    EnvError(#[from] std::env::VarError),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Other error: {0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
