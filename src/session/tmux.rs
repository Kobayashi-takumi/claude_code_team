use crate::error::{AppError, Result};
use tokio::process::Command;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_session_creation_and_layout() {
        // tmuxが利用可能かチェック
        if tokio::process::Command::new("tmux").arg("-V").output().await.is_err() {
            println!("tmux not available, skipping test");
            return;
        }

        let session = TmuxSession::new("test-session");
        
        // セッション削除（クリーンアップ）
        let _ = session.kill_session().await;
        
        // セッション作成
        if session.create().await.is_err() {
            println!("Session creation failed, skipping test");
            return;
        }
        
        // ペイン分割のテスト（4回分割して5ペインにする）
        for i in 0..4 {
            session.split_pane().await.unwrap_or_else(|e| {
                println!("Split pane {} failed: {}", i, e);
            });
        }
        
        // クリーンアップ
        let _ = session.kill_session().await;
    }

    #[tokio::test]
    async fn test_send_keys_with_explicit_pane() {
        // tmuxが利用可能かチェック
        if tokio::process::Command::new("tmux").arg("-V").output().await.is_err() {
            println!("tmux not available, skipping test");
            return;
        }

        let session = TmuxSession::new("test-keys-session");
        
        // クリーンアップ
        let _ = session.kill_session().await;
        
        // セッション作成
        if session.create().await.is_err() {
            println!("Session creation failed, skipping test");
            return;
        }
        
        // キー送信テスト
        let result = session.send_keys("test-keys-session", "echo test").await;
        assert!(result.is_ok(), "Key sending should succeed");
        
        // クリーンアップ
        let _ = session.kill_session().await;
    }

    #[tokio::test]
    async fn test_attach_session_nonexistent() {
        // tmuxが利用可能かチェック
        if tokio::process::Command::new("tmux").arg("-V").output().await.is_err() {
            println!("tmux not available, skipping test");
            return;
        }

        let session = TmuxSession::new("nonexistent-session");
        
        // 存在しないセッションへのattachはエラーになることを確認
        let result = session.attach_session().await;
        assert!(result.is_err(), "Attach to nonexistent session should fail");
        
        // エラーメッセージの確認
        if let Err(AppError::TmuxError(msg)) = result {
            assert!(msg.contains("が見つかりません"), "Error message should indicate session not found");
        }
    }

    #[tokio::test]
    async fn test_send_enter_key() {
        // tmuxが利用可能かチェック
        if tokio::process::Command::new("tmux").arg("-V").output().await.is_err() {
            println!("tmux not available, skipping test");
            return;
        }

        let session = TmuxSession::new("test-enter-session");
        
        // 存在しないセッションのペインへのEnter送信はエラーになることを確認
        let result = session.send_enter_key("nonexistent:0.0").await;
        assert!(result.is_err(), "Send enter key to nonexistent pane should fail");
    }

    #[test]
    fn test_is_inside_tmux() {
        // 環境変数TMUX_PANEまたはTMUXが設定されている場合、tmux内と判定
        std::env::set_var("TMUX_PANE", "%1");
        assert!(is_inside_tmux(), "Should detect tmux environment with TMUX_PANE");
        
        std::env::remove_var("TMUX_PANE");
        std::env::set_var("TMUX", "/tmp/tmux-1000/default,12345,0");
        assert!(is_inside_tmux(), "Should detect tmux environment with TMUX");
        
        std::env::remove_var("TMUX");
        assert!(!is_inside_tmux(), "Should not detect tmux environment without variables");
    }

    #[tokio::test]
    async fn test_get_current_tmux_session() {
        // tmuxが利用可能かチェック
        if tokio::process::Command::new("tmux").arg("-V").output().await.is_err() {
            println!("tmux not available, skipping test");
            return;
        }

        // tmux環境外では取得できないことを確認
        std::env::remove_var("TMUX");
        std::env::remove_var("TMUX_PANE");
        let result = get_current_tmux_session().await;
        assert!(result.is_err(), "Should fail to get session outside tmux");
    }
}

pub struct TmuxSession {
    name: String,
}

impl TmuxSession {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn create(&self) -> Result<()> {
        let output = Command::new("tmux")
            .args(["new-session", "-d", "-s", &self.name])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to create session: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            if !error.contains("duplicate session") {
                return Err(AppError::TmuxError(error.to_string()));
            }
        }

        Ok(())
    }

    pub async fn split_pane(&self) -> Result<()> {
        let output = Command::new("tmux")
            .args(["split-window", "-h", "-t", &self.name])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to split pane: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub async fn setup_layout(&self, pane_count: usize) -> Result<()> {
        // 最初のセッション作成
        self.create().await?;
        
        // 必要な数だけペインを分割
        for i in 1..pane_count {
            if let Err(e) = self.split_pane().await {
                tracing::warn!("Failed to create pane {}: {}", i, e);
                // 分割失敗時は垂直分割を試行
                self.split_pane_vertical().await?;
            }
        }
        
        // レイアウトを均等に調整
        self.select_layout("tiled").await?;
        
        Ok(())
    }

    pub async fn split_pane_vertical(&self) -> Result<()> {
        let output = Command::new("tmux")
            .args(["split-window", "-v", "-t", &self.name])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to split pane vertically: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub async fn select_layout(&self, layout: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["select-layout", "-t", &self.name, layout])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to select layout: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            tracing::warn!("Layout selection failed: {}", error);
            // レイアウト選択失敗は致命的ではないため、ログ出力のみ
        }

        Ok(())
    }

    pub async fn send_keys(&self, pane: &str, keys: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["send-keys", "-t", pane, keys, "Enter"])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to send keys: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub async fn send_enter_key(&self, pane: &str) -> Result<()> {
        let output = Command::new("tmux")
            .args(["send-keys", "-t", pane, "Enter"])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to send enter key: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }

    pub async fn list_panes(&self) -> Result<Vec<String>> {
        let output = Command::new("tmux")
            .args(["list-panes", "-t", &self.name, "-F", "#{session_name}:#{window_index}.#{pane_index}"])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to list panes: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        let panes = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|line| line.trim().to_string())
            .collect();

        Ok(panes)
    }

    pub async fn kill_session(&self) -> Result<()> {
        let _ = Command::new("tmux")
            .args(["kill-session", "-t", &self.name])
            .output()
            .await;
        Ok(())
    }

    pub async fn attach_session(&self) -> Result<()> {
        // セッション存在確認
        let output = Command::new("tmux")
            .args(["has-session", "-t", &self.name])
            .output()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to check session: {}", e)))?;

        if !output.status.success() {
            return Err(AppError::TmuxError(format!(
                "セッション '{}' が見つかりません。'claude-code-team start' でセッションを開始してください",
                &self.name
            )));
        }

        // セッションに接続
        let status = Command::new("tmux")
            .args(["attach-session", "-t", &self.name])
            .status()
            .await
            .map_err(|e| AppError::TmuxError(format!("Failed to attach session: {}", e)))?;

        if !status.success() {
            return Err(AppError::TmuxError(format!(
                "セッション '{}' への接続に失敗しました",
                &self.name
            )));
        }

        Ok(())
    }
}

pub fn is_inside_tmux() -> bool {
    std::env::var("TMUX").is_ok() || std::env::var("TMUX_PANE").is_ok()
}

pub async fn get_current_tmux_session() -> Result<String> {
    if !is_inside_tmux() {
        return Err(AppError::TmuxError(
            "tmuxセッション内で実行するか、--sessionでセッション名を指定してください".to_string()
        ));
    }

    let output = Command::new("tmux")
        .args(["display-message", "-p", "#{session_name}"])
        .output()
        .await
        .map_err(|e| AppError::TmuxError(format!("Failed to get current session: {}", e)))?;

    if !output.status.success() {
        return Err(AppError::TmuxError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let session_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(session_name)
}