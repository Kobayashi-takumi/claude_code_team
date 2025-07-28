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
        #[arg(
            long,
            help = "⚠️  権限チェックをスキップ（セキュリティリスク：本番環境非推奨）"
        )]
        dangerously_skip_permissions: bool,
    },

    #[command(about = "エージェントへのメッセージ送信")]
    Send {
        #[arg(help = "送信先エージェント名")]
        agent: String,

        #[arg(help = "メッセージ内容")]
        message: String,
    },

    #[command(about = "エージェントチームの停止")]
    Stop,

    #[command(about = "ログの表示")]
    Logs {
        #[arg(short, long, help = "特定のエージェントのログを表示")]
        agent: Option<String>,
    },

    #[command(about = "tmuxセッションに接続")]
    Attach,
}
