#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_from_env() {
        // 一時的に.envの影響を避けるため、環境変数を強制的に上書き
        dotenv::dotenv().ok();

        env::set_var("CLAUDE_CLI_PATH", "test-claude");
        env::set_var("TMUX_SESSION_NAME", "test-session");
        env::set_var("WORKSPACE_DIR", "/test");
        env::set_var("LOG_DIR", "./test-logs");

        let config = Config::from_env().unwrap();
        assert_eq!(config.claude_cli_path, "test-claude");
        assert_eq!(config.tmux_session_name, "test-session");
        assert_eq!(config.workspace_dir, "/test");
        assert_eq!(config.log_dir, "./test-logs");

        // テスト後のクリーンアップ
        env::remove_var("CLAUDE_CLI_PATH");
        env::remove_var("TMUX_SESSION_NAME");
        env::remove_var("WORKSPACE_DIR");
        env::remove_var("LOG_DIR");
    }

    #[test]
    fn test_config_defaults() {
        // 他のテストの影響を避けるため、前回の環境変数をクリア
        env::remove_var("CLAUDE_CLI_PATH");
        env::remove_var("TMUX_SESSION_NAME");  
        env::remove_var("WORKSPACE_DIR");
        env::remove_var("LOG_DIR");

        // .envファイルの読み込みを無効化してテスト専用環境にする
        let config = Config {
            claude_cli_path: env::var("CLAUDE_CLI_PATH").unwrap_or_else(|_| "claude".to_string()),
            tmux_session_name: env::var("TMUX_SESSION_NAME").unwrap_or_else(|_| "claude-code-team".to_string()),
            workspace_dir: env::var("WORKSPACE_DIR").unwrap_or_else(|_| "./workspace".to_string()),
            log_dir: env::var("LOG_DIR").unwrap_or_else(|_| "./logs".to_string()),
        };

        assert_eq!(config.claude_cli_path, "claude");
        assert_eq!(config.tmux_session_name, "claude-code-team");
        assert_eq!(config.workspace_dir, "./workspace");
        assert_eq!(config.log_dir, "./logs");
    }
}

pub mod env;

pub use self::env::*;
