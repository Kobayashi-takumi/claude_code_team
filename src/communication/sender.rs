#[cfg(test)]
mod tests {
    use super::*;
    use crate::communication::MessageType;

    #[test]
    fn test_message_formatting() {
        let sender = MessageSender {
            session: TmuxSession::new("test-session"),
            log_dir: "./test-logs".to_string(),
        };

        let message = Message::new(
            "user",
            "brain",
            "TODOアプリを作ってください",
            MessageType::Task,
        );
        let formatted = sender.format_safe_message(&message);

        // メッセージが適切にフォーマットされていることを確認
        assert!(!formatted.is_empty());

        // 特殊文字がエスケープされていることを確認
        assert!(!formatted.contains("【"));
        assert!(!formatted.contains("】"));

        // 改行文字がリテラルでないことを確認
        assert!(!formatted.contains("\\n"));
    }

    #[test]
    fn test_shell_escape() {
        let sender = MessageSender {
            session: TmuxSession::new("test-session"),
            log_dir: "./test-logs".to_string(),
        };

        let result = sender.escape_shell_string("test; rm -rf /");
        assert!(result.contains("\\;")); // セミコロンがエスケープされていることを確認
        assert!(result.contains("test"));
        assert!(result.contains("\\; rm")); // エスケープされた形で含まれていることを確認
    }
}

use super::Message;
use crate::error::Result;
use crate::session::TmuxSession;
use chrono::Local;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;

pub struct MessageSender {
    session: TmuxSession,
    log_dir: String,
}

impl MessageSender {
    pub fn new(session_name: &str) -> Result<Self> {
        let log_dir = crate::config::Config::get().log_dir.clone();
        create_dir_all(&log_dir)?;

        Ok(Self {
            session: TmuxSession::new(session_name),
            log_dir,
        })
    }

    pub async fn send(&self, message: &Message) -> Result<()> {
        let formatted = self.format_safe_message(message);
        let command = self.create_command(message);

        let command_message = format!("{} {}", command, formatted);

        // 動的にペイン一覧を取得
        let panes = self.session.list_panes().await?;
        let agent_index = self.get_agent_index(&message.to);

        let target_pane = if agent_index < panes.len() {
            &panes[agent_index]
        } else {
            // フォールバック：静的ペイン指定
            &self.get_agent_pane(&message.to)
        };

        self.session
            .send_keys(target_pane, &command_message)
            .await?;

        // 短い遅延後に追加のEnterキー送信
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.session.send_enter_key(target_pane).await?;
        self.log_message(message)?;

        Ok(())
    }

    fn format_safe_message(&self, message: &Message) -> String {
        self.escape_shell_string(&message.content)
    }

    fn create_command(&self, message: &Message) -> String {
        match message.to.as_str() {
            "brain" => "/brain".to_string(),
            "arch" => "/arch".to_string(),
            "dev" => "/dev".to_string(),
            "qa" => "/qa".to_string(),
            _ => String::new(),
        }
    }

    fn escape_shell_string(&self, input: &str) -> String {
        input
            .replace('\\', "\\\\") // バックスラッシュをエスケープ
            .replace('"', "\\\"") // ダブルクォートをエスケープ
            .replace('$', "\\$") // ドル記号をエスケープ
            .replace('`', "\\`") // バッククォートをエスケープ
            .replace('!', "\\!") // エクスクラメーションをエスケープ
            .replace(';', "\\;") // セミコロンをエスケープ
            .replace('&', "\\&") // アンパサンドをエスケープ
            .replace('|', "\\|") // パイプをエスケープ
            .replace('<', "\\<") // 小なりをエスケープ
            .replace('>', "\\>") // 大なりをエスケープ
            .replace('(', "\\(") // 左括弧をエスケープ
            .replace(')', "\\)") // 右括弧をエスケープ
            .replace('[', "\\[") // 左角括弧をエスケープ
            .replace(']', "\\]") // 右角括弧をエスケープ
            .replace('{', "\\{") // 左波括弧をエスケープ
            .replace('}', "\\}") // 右波括弧をエスケープ
            .replace('*', "\\*") // アスタリスクをエスケープ
            .replace('?', "\\?") // クエスチョンをエスケープ
    }

    fn get_agent_index(&self, agent_name: &str) -> usize {
        match agent_name {
            "brain" => 0,
            "arch" => 1,
            "dev" => 2,
            "qa" => 3,
            _ => 0,
        }
    }

    fn get_agent_pane(&self, agent_name: &str) -> String {
        let pane_index = match agent_name {
            "brain" => "1.0",
            "arch" => "1.1",
            "dev" => "1.2",
            "qa" => "1.3",
            _ => "1.0",
        };

        format!("claude-code-team:{}", pane_index)
    }

    fn log_message(&self, message: &Message) -> Result<()> {
        let log_path = format!("{}/send_log.txt", self.log_dir);
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;

        let log_entry = format!(
            "[{}] {} -> {}: {}\\n",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            message.from,
            message.to,
            message.content
        );

        file.write_all(log_entry.as_bytes())?;
        Ok(())
    }
}
