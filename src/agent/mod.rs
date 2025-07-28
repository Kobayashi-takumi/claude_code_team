#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let agent = Agent::new("brain", Role::Brain);
        assert_eq!(agent.name, "brain");
        assert!(matches!(agent.role, Role::Brain));
        assert_eq!(agent.tmux_pane, "claude-code-team:0.0");
    }

    #[test]
    fn test_role_description() {
        assert_eq!(Role::Brain.description(), "ワークフロー管理・要求分析");
        assert_eq!(Role::Arch.description(), "システム設計・実装方針策定");
        assert_eq!(Role::Dev.description(), "実装・コーディング");
        assert_eq!(Role::QA.description(), "テスト・品質保証・検証");
    }
}

pub mod manager;
pub mod types;

pub use manager::*;
pub use types::*;
