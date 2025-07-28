#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let err = AppError::ConfigError("Invalid config".to_string());
        assert_eq!(err.to_string(), "Configuration error: Invalid config");
    }

    #[test]
    fn test_error_conversion_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::IoError(_)));
    }

    #[test]
    fn test_tmux_error() {
        let err = AppError::TmuxError("Session not found".to_string());
        assert_eq!(err.to_string(), "Tmux error: Session not found");
    }

    #[test]
    fn test_agent_error() {
        let err = AppError::AgentError("Dev failed".to_string());
        assert_eq!(err.to_string(), "Agent error: Dev failed");
    }
}

pub mod types;

pub use types::*;
