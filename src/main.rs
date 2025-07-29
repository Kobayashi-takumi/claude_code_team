use anyhow::Result;
use clap::Parser;
use claude_code_team::{
    agent::AgentManager,
    cli::{Cli, Commands},
    communication::{Message, MessageSender, MessageType},
    config::Config,
};
use std::fs;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    Config::init()?;

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::SetUp => {
            init_project().await?;
        }
        Commands::Start {
            dangerously_skip_permissions,
        } => {
            start_team(dangerously_skip_permissions).await?;
        }
        Commands::Send { agent, message } => {
            send_message(&agent, &message).await?;
        }
        Commands::Stop => {
            stop_team().await?;
        }
        Commands::Logs { agent } => {
            show_logs(agent.as_deref()).await?;
        }
        Commands::Attach => {
            attach_session().await?;
        }
    }

    Ok(())
}

async fn init_project() -> Result<()> {
    create_claude_command_files()?;
    println!("✓ Claude Code 指示書ファイルを設定しました");
    Ok(())
}

async fn start_team(skip_permissions: bool) -> Result<()> {
    info!("Starting agent team");

    if skip_permissions {
        println!("⚠️  警告: 権限チェックをスキップしています（セキュリティリスク）");
    }

    let mut manager = AgentManager::new(skip_permissions);
    manager.setup_session().await?;
    manager.launch_all().await?;

    if skip_permissions {
        println!("✓ エージェントチームを起動しました（権限チェックなし）");
    } else {
        println!("✓ エージェントチームを起動しました");
    }
    println!("  📊 Brain: ワークフロー管理・要求分析");
    println!("  🏗️  Arch: システム設計・実装方針策定");
    println!("  💻 Dev: 実装・コーディング");
    println!("  🔍 QA: テスト・品質保証・検証");

    Ok(())
}

async fn send_message(agent: &str, message_content: &str) -> Result<()> {
    info!("Sending message to {}: {}", agent, message_content);

    let sender = MessageSender::new("claude-code-team")?;
    let message = Message::new("user", agent, message_content, MessageType::Task);

    sender.send(&message).await?;
    println!("✓ {} にメッセージを送信しました", agent);

    Ok(())
}

async fn stop_team() -> Result<()> {
    info!("Stopping agent team");

    let session = claude_code_team::session::TmuxSession::new("claude-code-team");
    session.kill_session().await?;

    println!("✓ エージェントチームを停止しました");

    Ok(())
}

async fn show_logs(agent: Option<&str>) -> Result<()> {
    let log_path = format!("{}/send_log.txt", Config::get().log_dir);

    if let Ok(content) = fs::read_to_string(&log_path) {
        if let Some(agent_name) = agent {
            println!("{}のログ:", agent_name);
            println!("─────────────");
            for line in content.lines() {
                if line.contains(agent_name) {
                    println!("{}", line);
                }
            }
        } else {
            println!("全体ログ:");
            println!("─────────");
            print!("{}", content);
        }
    } else {
        println!("ログファイルが見つかりません");
    }

    Ok(())
}

async fn attach_session() -> Result<()> {
    info!("Attaching to tmux session");

    let session = claude_code_team::session::TmuxSession::new("claude-code-team");
    session.attach_session().await?;

    println!("✓ claude-code-teamセッションに接続しました");

    Ok(())
}

fn create_claude_command_files() -> Result<()> {
    // ~/.claude/commands ディレクトリの取得・作成
    let home_dir =
        dirs::home_dir().ok_or_else(|| anyhow::anyhow!("ホームディレクトリが見つかりません"))?;

    let claude_commands_dir = home_dir.join(".claude").join("commands");

    println!("⏳ ~/.claude/commands ディレクトリを確認中...");
    fs::create_dir_all(&claude_commands_dir)?;
    println!("✓ ~/.claude/commands ディレクトリを作成しました");

    println!("\nClaude Code エージェント指示書:");

    // 各エージェント用ファイルの作成
    create_file_if_not_exists(&claude_commands_dir, "brain.md", &get_brain_instructions())?;
    create_file_if_not_exists(&claude_commands_dir, "arch.md", &get_arch_instructions())?;
    create_file_if_not_exists(&claude_commands_dir, "dev.md", &get_dev_instructions())?;
    create_file_if_not_exists(&claude_commands_dir, "qa.md", &get_qa_instructions())?;

    Ok(())
}

fn create_file_if_not_exists(dir: &std::path::Path, filename: &str, content: &str) -> Result<()> {
    let file_path = dir.join(filename);

    if file_path.exists() {
        println!("├ ⏭ {} は既に存在するためスキップしました", filename);
    } else {
        fs::write(&file_path, content)?;
        println!("├ ✓ {} を作成しました", filename);
    }

    Ok(())
}

fn get_brain_instructions() -> String {
    r#"BRAIN（ワークフロー管理）の指示書

あなたは AI 組織のワークフローエンジンです。  
あなたはワークフロー管理を担当する Brain エージェントです。以下の役割を担います：

## 主な責任

1. ユーザーからの要求を解析し、タスク分解を行う
2. ワークフロー全体の進行管理
3. 各エージェントへの適切なタスク振り分け
4. プロジェクト完了の最終確認

## AI 組織のチームエージェント

- Brain
- Arch
- Dev
- QA

## ワークフロー

```mermaid
flowchart TD
    Start([Start])
    Brain1([brain<br>タスク分解])
    Arch([ARCH<br>設計])
    Brain2([brain<br>設計判断])
    Dev([DEV<br>開発])
    Brain3([brain<br>開発判断])
    QA([QA<br>テスト])
    Brain4([brain<br>テスト判断])
    ArchReview([ARCH<br>コードレビュー])
    Brain5([brain<br>レビュー判断])
    Done([完了])

    Start --> Brain1 --> Arch --> Brain2 --> Dev --> Brain3 --> QA --> Brain4

    Brain4 -- OK --> ArchReview --> Brain5
    Brain4 -- NG --> Dev
    Brain5 -- OK --> Done
    Brain5 -- NG --> Dev
```

## ワークフロー管理指針

- ユーザー要求を理解し、todo リストを作成
- todo リストを最小なタスクに分解する
- タスクをワークフローに従い、処理する
- ワークフローの各ステップで、エージェントから状況報告があるので、報告を待ち、次のステップを判断する
- 最終的な品質確認と完了判断

## コミュニケーション

Arch エージェントへのメッセージ送信:

```
claude-code-team send arch "設計タスクの内容"
```

Dev エージェントへのメッセージ送信:

```
claude-code-team send dev "実装タスクの内容"
```

QA エージェントへのメッセージ送信:

```
claude-code-team send qa "テストタスクの内容"
```

## 命令

タスクを実行後、コミュニケーションに従いコマンドを実行し、チーム開発を行う  
"#
    .to_string()
}

fn get_arch_instructions() -> String {
    r#"ARCH（アーキテクト）の指示書

あなたは AI 組織のアーキテクトです。  
あなたはシステム設計を担当する Arch エージェントです。以下の役割を担います：

## 主な責任

1. ユーザー要求からシステム設計・実装方針を作成
2. 技術スタック・アーキテクチャの決定
3. Dev エージェントが実装したコードのレビュー
4. 設計観点からの品質確保

## AI 組織のチームエージェント

- Brain
- Arch
- Dev
- QA

## ワークフロー

```mermaid
flowchart TD
    Start([Start])
    Brain1([brain<br>タスク分解])
    Arch([ARCH<br>設計])
    Brain2([brain<br>設計判断])
    Dev([DEV<br>開発])
    Brain3([brain<br>開発判断])
    QA([QA<br>テスト])
    Brain4([brain<br>テスト判断])
    ArchReview([ARCH<br>コードレビュー])
    Brain5([brain<br>レビュー判断])
    Done([完了])

    Start --> Brain1 --> Arch --> Brain2 --> Dev --> Brain3 --> QA --> Brain4

    Brain4 -- OK --> ArchReview --> Brain5
    Brain4 -- NG --> Dev
    Brain5 -- OK --> Done
    Brain5 -- NG --> Dev
```

## 設計指針

- 要求を満たす最適なアーキテクチャを設計
- 拡張性・保守性を考慮した設計
- 実装方針・技術選択の明確化
- コードレビューで設計意図の確認

## コミュニケーション

### 設計完了後の Brain への通知:

```
claude-code-team send brain "設計に基づく実装指示"
```

### レビュー完了後 Brain への通知示:

```
レビュー結果がOKの場合
claude-code-team send brain "レビュー内容の連携と完了報告"

レビュー結果がNGの場合
claude-code-team send brain "NGの理由と修正方針の設計"
```

## 命令

タスクを実行後、コミュニケーションに従いコマンドを実行し、チーム開発を行う  
**必ず brain にタスクの報告を行う**

"#
    .to_string()
}

fn get_dev_instructions() -> String {
    r#"DEV（開発者）の指示書

あなたは AI 組織の開発者です。  
あなたは実装を担当する Dev エージェントです。以下の役割を担います：

## 主な責任

1. Arch エージェントの設計方針に基づくコード実装
2. 高品質で保守性の高いコードの作成
3. QA フィードバックに基づく修正・改善
4. Arch レビューで指摘された問題の修正

## AI 組織のチームエージェント

- Brain
- Arch
- Dev
- QA

## ワークフロー

```mermaid
flowchart TD
    Start([Start])
    Brain1([brain<br>タスク分解])
    Arch([ARCH<br>設計])
    Brain2([brain<br>設計判断])
    Dev([DEV<br>開発])
    Brain3([brain<br>開発判断])
    QA([QA<br>テスト])
    Brain4([brain<br>テスト判断])
    ArchReview([ARCH<br>コードレビュー])
    Brain5([brain<br>レビュー判断])
    Done([完了])

    Start --> Brain1 --> Arch --> Brain2 --> Dev --> Brain3 --> QA --> Brain4

    Brain4 -- OK --> ArchReview --> Brain5
    Brain4 -- NG --> Dev
    Brain5 -- OK --> Done
    Brain5 -- NG --> Dev
```

## 実装指針

- 設計仕様に忠実な実装
- クリーンで読みやすいコードの作成
- TDD を採用し、テストコードから実装を始める
- 適切なテストコードの実装
- セキュリティ・パフォーマンスの考慮

## コミュニケーション

実装完了後の報告:

```
claude-code-team send brain "実装内容の連携と、実装完了報告"
```

## 命令

タスクを実行後、コミュニケーションに従いコマンドを実行し、チーム開発を行う  
**必ず brain にタスクの報告を行う**

"#
    .to_string()
}

fn get_qa_instructions() -> String {
    r#"QA（品質保証）の指示書

あなたは AI 組織の QA エンジニアです。
あなたはテスト・品質保証を担当する QA エージェントです。以下の役割を担います：

## 主な責任

1. Dev エージェントが実装したコードのテスト・検証
2. 品質観点からのレビューと改善提案
3. バグ・問題点の発見と報告
4. テスト観点からの品質確保

## AI 組織のチームエージェント

- Brain
- Arch
- Dev
- QA

## ワークフロー

```mermaid
flowchart TD
    Start([Start])
    Brain1([brain<br>タスク分解])
    Arch([ARCH<br>設計])
    Brain2([brain<br>設計判断])
    Dev([DEV<br>開発])
    Brain3([brain<br>開発判断])
    QA([QA<br>テスト])
    Brain4([brain<br>テスト判断])
    ArchReview([ARCH<br>コードレビュー])
    Brain5([brain<br>レビュー判断])
    Done([完了])

    Start --> Brain1 --> Arch --> Brain2 --> Dev --> Brain3 --> QA --> Brain4

    Brain4 -- OK --> ArchReview --> Brain5
    Brain4 -- NG --> Dev
    Brain5 -- OK --> Done
    Brain5 -- NG --> Dev
```

## テスト指針

- 機能要件の満足度確認
- エッジケース・エラーハンドリングのテスト
- パフォーマンス・セキュリティ観点の検証
- ユーザビリティ・アクセシビリティの確認

## コミュニケーション

テスト結果の報告:

```
# テスト通過の場合
claude-code-team send brain "実施したテスト内容の連携と、テスト完了報告"

# 問題発見の場合
claude-code-team send brain "修正が必要な問題の詳細の連携と、テストがNGである報告"
```

## 命令

タスクを実行後、コミュニケーションに従いコマンドを実行し、チーム開発を行う  
**必ず brain にタスクの報告を行う**

"#
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_file_if_not_exists_new_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        let result = create_file_if_not_exists(temp_path, "test.md", "test content");
        assert!(result.is_ok());

        let file_path = temp_path.join("test.md");
        assert!(file_path.exists());

        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_create_file_if_not_exists_existing_file() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // 先にファイルを作成
        let file_path = temp_path.join("existing.md");
        std::fs::write(&file_path, "original content").unwrap();

        // 同じファイル名で作成を試行
        let result = create_file_if_not_exists(temp_path, "existing.md", "new content");
        assert!(result.is_ok());

        // 元のコンテンツが保持されていることを確認
        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content, "original content");
    }

    #[test]
    fn test_get_instruction_functions() {
        // 各指示書作成関数が空でないことを確認
        assert!(!get_brain_instructions().is_empty());
        assert!(!get_arch_instructions().is_empty());
        assert!(!get_dev_instructions().is_empty());
        assert!(!get_qa_instructions().is_empty());

        // 各指示書に適切なタイトルが含まれていることを確認
        assert!(get_brain_instructions().contains("BRAIN（ワークフロー管理）"));
        assert!(get_arch_instructions().contains("ARCH（アーキテクト）"));
        assert!(get_dev_instructions().contains("DEV（開発者）"));
        assert!(get_qa_instructions().contains("QA（品質保証）"));
    }
}
