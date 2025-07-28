use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub claude_cli_path: String,
    pub tmux_session_name: String,
    pub workspace_dir: String,
    pub log_dir: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let config = Self {
            claude_cli_path: std::env::var("CLAUDE_CLI_PATH")
                .unwrap_or_else(|_| "claude".to_string()),
            tmux_session_name: std::env::var("TMUX_SESSION_NAME")
                .unwrap_or_else(|_| "claude-code-team".to_string()),
            workspace_dir: std::env::var("WORKSPACE_DIR")
                .unwrap_or_else(|_| "./workspace".to_string()),
            log_dir: std::env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
        };

        Ok(config)
    }

    pub fn init() -> Result<()> {
        let config = Self::from_env()?;
        CONFIG.set(config).map_err(|_| {
            crate::error::AppError::ConfigError("Config already initialized".to_string())
        })?;
        Ok(())
    }

    pub fn get() -> &'static Config {
        CONFIG.get().expect("Config not initialized")
    }
}
