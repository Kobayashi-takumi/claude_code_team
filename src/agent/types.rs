use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_role_descriptions() {
        assert_eq!(Role::Brain.description(), "ワークフロー管理・要求分析");
        assert_eq!(Role::Arch.description(), "システム設計・実装方針策定");
        assert_eq!(Role::Dev.description(), "実装・コーディング");
        assert_eq!(Role::QA.description(), "テスト・品質保証・検証");
    }

    #[test]
    fn test_new_agent_creation() {
        let brain = Agent::new("brain", Role::Brain);
        assert_eq!(brain.name, "brain");
        assert!(matches!(brain.role, Role::Brain));
        assert_eq!(brain.tmux_pane, "claude-code-team:0.0");

        let dev = Agent::new("dev", Role::Dev);
        assert_eq!(dev.tmux_pane, "claude-code-team:0.2");
    }

    #[test]
    fn test_instruction_files() {
        assert_eq!(Role::Brain.instruction_file(), "instructions/brain.md");
        assert_eq!(Role::Arch.instruction_file(), "instructions/arch.md");
        assert_eq!(Role::Dev.instruction_file(), "instructions/dev.md");
        assert_eq!(Role::QA.instruction_file(), "instructions/qa.md");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    Brain,
    Arch,
    Dev,
    QA,
}

impl Role {
    pub fn description(&self) -> &'static str {
        match self {
            Role::Brain => "ワークフロー管理・要求分析",
            Role::Arch => "システム設計・実装方針策定",
            Role::Dev => "実装・コーディング",
            Role::QA => "テスト・品質保証・検証",
        }
    }

    pub fn instruction_file(&self) -> &'static str {
        match self {
            Role::Brain => "instructions/brain.md",
            Role::Arch => "instructions/arch.md",
            Role::Dev => "instructions/dev.md",
            Role::QA => "instructions/qa.md",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub role: Role,
    pub tmux_pane: String,
    pub status: AgentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgentStatus {
    Idle,
    Working,
    WaitingForResponse,
    Completed,
    Failed(String),
}

impl Agent {
    pub fn new(name: &str, role: Role) -> Self {
        let pane_index = match &role {
            Role::Brain => "0.0",
            Role::Arch => "0.1",
            Role::Dev => "0.2",
            Role::QA => "0.3",
        };

        Self {
            name: name.to_string(),
            role,
            tmux_pane: format!("claude-code-team:{}", pane_index),
            status: AgentStatus::Idle,
        }
    }
}
