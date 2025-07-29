use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "claude-code-team")]
#[command(about = "AI組織による開発チーム管理ツール")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "ツールの初期化（Claude Code カスタムフック作成）")]
    SetUp,

    #[command(about = "エージェントチームの起動")]
    Start {
        #[arg(short, long, help = "セッション名（デフォルト: claude-code-team）")]
        session: Option<String>,

        #[arg(
            long,
            help = "⚠️  権限チェックをスキップ（セキュリティリスク：本番環境非推奨）"
        )]
        dangerously_skip_permissions: bool,
    },

    #[command(about = "エージェントへのメッセージ送信")]
    Send {
        #[arg(
            short,
            long,
            help = "セッション名（未指定時は現在のセッションを自動検出）"
        )]
        session: Option<String>,

        #[arg(help = "送信先エージェント名")]
        agent: String,

        #[arg(help = "メッセージ内容（改行は\\nで指定可能）")]
        message: String,
    },

    #[command(about = "エージェントチームの停止")]
    Stop {
        #[arg(short, long, help = "停止するセッション名")]
        session_name: Option<String>,
    },

    #[command(about = "ログの表示")]
    Logs {
        #[arg(short, long, help = "特定のエージェントのログを表示")]
        agent: Option<String>,
    },

    #[command(about = "tmuxセッションに接続")]
    Attach {
        #[arg(short, long, help = "接続するセッション名")]
        session_name: String,
    },
}
