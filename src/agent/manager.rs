use super::types::{Agent, Role};
use crate::error::{AppError, Result};
use crate::session::TmuxSession;
use std::collections::HashMap;

pub struct AgentManager {
    agents: HashMap<String, Agent>,
    session: TmuxSession,
    skip_permissions: bool,
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new(false)
    }
}

impl AgentManager {
    pub fn new(skip_permissions: bool) -> Self {
        let mut agents = HashMap::new();

        agents.insert("brain".to_string(), Agent::new("brain", Role::Brain));
        agents.insert("arch".to_string(), Agent::new("arch", Role::Arch));
        agents.insert("dev".to_string(), Agent::new("dev", Role::Dev));
        agents.insert("qa".to_string(), Agent::new("qa", Role::QA));

        Self {
            agents,
            session: TmuxSession::new("claude-code-team"),
            skip_permissions,
        }
    }

    pub async fn setup_session(&mut self) -> Result<()> {
        println!("⏳ TMUXセッション '{}' を作成中...", self.session.name());

        // 4つのエージェント用にセッションとペインを設定
        self.session.setup_layout(self.agents.len()).await?;

        println!("✓ セッション作成完了");
        println!("⏳ {}つのペインを設定中...", self.agents.len());

        // ペイン確認
        if let Ok(panes) = self.session.list_panes().await {
            for (i, pane) in panes.iter().enumerate() {
                let agent_name = match i {
                    0 => "Brain (ワークフロー管理)",
                    1 => "Arch (システム設計)",
                    2 => "Dev (実装)",
                    3 => "QA (品質保証)",
                    _ => "Unknown",
                };
                println!("  ├ ペイン {}: {} ({})", i, agent_name, pane);
            }
        }

        println!("✓ 全ペイン作成完了");
        Ok(())
    }

    pub async fn launch_agent(&self, agent_name: &str) -> Result<()> {
        let agent = self
            .agents
            .get(agent_name)
            .ok_or_else(|| AppError::AgentError(format!("Agent {} not found", agent_name)))?;

        // 動的にペイン一覧を取得して適切なペインを選択
        let panes = self.session.list_panes().await?;
        let agent_index = self.get_agent_index(agent_name);

        let target_pane = if agent_index < panes.len() {
            &panes[agent_index]
        } else {
            // フォールバック：元の静的ペイン指定
            &agent.tmux_pane
        };

        let _instruction_path = agent.role.instruction_file();
        
        let claude_command = if self.skip_permissions {
            format!(
                "{} --dangerously-skip-permissions",
                crate::config::Config::get().claude_cli_path,
            )
        } else {
            crate::config::Config::get().claude_cli_path.to_string()
        };

        self.session.send_keys(target_pane, &claude_command).await?;

        Ok(())
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

    pub async fn launch_all(&self) -> Result<()> {
        println!("⏳ エージェントを起動中...");

        for agent_name in self.agents.keys() {
            println!("  ├ {}を起動中...", agent_name);
            if let Err(e) = self.launch_agent(agent_name).await {
                println!("  ⚠ {}の起動に失敗: {}", agent_name, e);
                continue;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }

        println!("✓ エージェントチーム起動完了");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_manager_new_with_permissions() {
        let manager_secure = AgentManager::new(false);
        assert!(!manager_secure.skip_permissions);
        
        let manager_insecure = AgentManager::new(true);
        assert!(manager_insecure.skip_permissions);
    }

    #[test]
    fn test_agent_manager_default() {
        let manager = AgentManager::default();
        assert!(!manager.skip_permissions); // デフォルトはセキュア
    }
}
