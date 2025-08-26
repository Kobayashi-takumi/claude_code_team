# Claude Code Team 🤖

> **AI 組織による協業開発システム** - 複数の Claude エージェントが連携して開発タスクを自動実行

[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Claude Code](https://img.shields.io/badge/Powered%20by-Claude%20Code-blue.svg)](https://claude.ai/code)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 🌟 概要

Claude Code Team は、複数の AI エージェントが自動的に役割分担し、協業してソフトウェア開発を行う Revolutionary なツールです。

**特徴:**

- 🧠 **ワークフローベース AI 開発チーム**: Brain、Arch、Dev、QA の 4 エージェント構成
- 🔄 **自動ワークフロー実行**: 要求分析 → 設計 → 実装 → テスト → レビューの完全自動化
- 📺 **リアルタイム協業**: tmux による並列実行とエージェント間コミュニケーション
- ⚡ **高品質開発**: 設計レビューとテスト検証による堅牢なソフトウェア開発
- 🔀 **マルチセッション対応**: 複数の AI 開発チームを同時に起動・管理可能

## 🚀 クイックスタート（5 分で体験）

```bash
# 1. インストール
cargo install --path .

# 2. コマンドの初期化
claude-code-team set-up

# 3. AI開発チーム起動
claude-code-team start

# 3. AI開発チームのウィンドウをアタッチ
claude-code-team attach claude-code-team

# 4. 開発指示を送信(もしくは、左上のウィンドウで、"/brain ユーザー認証機能付きのWebアプリを作成してください")
claude-code-team send brain "ユーザー認証機能付きのWebアプリを作成してください"

```

### 実行結果例

```
⏳ TMUXセッション 'claude-code-team' を作成中...
✓ セッション作成完了
⏳ 4つのペインを設定中...
  ├ ペイン 0: Brain (ワークフロー管理) (claude-code-team:1.0)
  ├ ペイン 1: Arch (システム設計) (claude-code-team:1.1)
  ├ ペイン 2: Dev (実装) (claude-code-team:1.2)
  └ ペイン 3: QA (品質保証) (claude-code-team:1.3)
✓ 全ペイン作成完了
✓ エージェントチーム起動完了
```

## 📦 インストール

### 前提条件

- **Rust** (1.70+): [rustup.rs](https://rustup.rs/)
- **Claude Code CLI**: [インストール手順](https://docs.anthropic.com/en/docs/claude-code)
- **tmux**: マルチペイン管理

  ```bash
  # macOS
  brew install tmux

  # Ubuntu/Debian
  sudo apt install tmux

  # CentOS/RHEL
  sudo yum install tmux
  ```

### ビルド & インストール

```bash
# リポジトリクローン
git clone https://github.com/your-org/claude-code-team.git
cd claude-code-team

# 依存関係とテストの確認
make check

# インストール
cargo install --path .

# 動作確認
claude-code-team --version
# 出力例: claude-code-team 0.1.0
```

## 💼 基本的な使い方

### 1. コマンドの初期化

```bash
claude-code-team set-up
```

これにより以下が作成されます：

- カスタムコマンド: `~/.claude/commands/brain.md・arch.md・dev.md・qa.md`

### 2. ワークフローベース チーム構成

| エージェント | 役割             | 責任範囲                                 |
| ------------ | ---------------- | ---------------------------------------- |
| **Brain**    | ワークフロー管理 | 要求分析、タスク分解、進捗管理、完了確認 |
| **Arch**     | アーキテクト     | システム設計、技術選択、コードレビュー   |
| **Dev**      | 開発者           | 実装、コーディング、修正対応             |
| **QA**       | 品質保証         | テスト、検証、品質確認                   |

### 3. 開発フロー

```bash
# Step 1: チーム起動
claude-code-team start

# Step 2: ウィンドウをアタッチ
claude-code-team attach claude-code-team

# Step 3: 要件をBrainに伝達(必要であれば、何度でも)
claude-code-team send brain "ECサイトを作ってください。ユーザー登録、商品一覧、決済機能が必要です"

# Step 4: 完了後の停止
claude-code-team stop
```

#### ワークフローの詳細な動作

```mermaid
graph LR
    A[Start] --> B[Brain: タスク分解]
    B --> C[Arch: 設計]
    C --> D[Brain: 設計判断]
    D --> E[Dev: 開発]
    E --> F[Brain: 開発判断]
    F --> G[QA: テスト]
    G --> H[Brain: テスト判断]
    H --> I[Arch: レビュー]
    I --> J[Brain: レビュー判断]
    J --> K[Done]
```

Brain エージェントがワークフローを制御し、各エージェントの成果物を確認して次のステップへ進みます。

## 📊 コマンドリファレンス

### `set-up` - コマンドのセットアップ

```bash
claude-code-team set-up
```

### `start` - チーム起動

```bash
claude-code-team start [OPTIONS]
```

**オプション:**

- `--session-name <NAME>`: TMUX セッション名を指定（デフォルト: claude-code-team）
- `--dangerously-skip-permissions`: Claude CLI の権限チェックをスキップ

TMUX セッションを作成し、4 つのエージェントを並列起動します。各エージェントは専用のペインで動作し、カスタムコマンド（/brain、/arch、/dev、/qa）を使用してメッセージを受信します。

### `attach` - ウィンドウのアタッチ

```bash
claude-code-team attach <SESSION_NAME>
```

**引数:**

- `<SESSION_NAME>`: TMUX セッション名（必須）

### `send` - メッセージ送信

```bash
claude-code-team send <AGENT> "<MESSAGE>" [OPTIONS]
```

**引数:**

- `<AGENT>`: 送信先エージェント (brain, arch, dev, qa)
- `<MESSAGE>`: 送信するメッセージ（改行文字\n をサポート）

**オプション:**

- `--session-name <NAME>`: TMUX セッション名を指定

**例:**

```bash
# 基本はbrainにのみ送る
claude-code-team send brain "ブログシステムを作成してください"

# 複数行のメッセージ
claude-code-team send brain "以下の機能を実装してください:\n1. ユーザー認証\n2. 記事投稿\n3. コメント機能"

# 特定セッションへの送信
claude-code-team send brain "修正をお願いします" --session-name my-project
```

**注意事項:**

- シェル特殊文字は自動的にエスケープされます
- TMUX の send-keys コマンドを使用してメッセージを送信

### `stop` - チーム停止

```bash
claude-code-team stop [OPTIONS]
```

**オプション:**

- `--session-name <NAME>`: TMUX セッション名を指定

TMUX セッションを終了し、すべてのエージェントを停止します。

## 🔍 内部動作の詳細

### メッセージ送信の仕組み

1. **MessageSender 構造体**が送信を管理
2. TMUX の`send-keys`コマンドでメッセージを送信
3. シェル特殊文字（$、`、\、"など）を自動エスケープ
4. 改行文字（\n）を実際の改行に変換
5. セッション名とエージェントタイプからペイン ID を動的に解決

### エラーハンドリング

`src/error/types.rs`で統一された AppError 型を定義：

- Configuration: 設定エラー
- IO: ファイル I/O エラー
- Tmux: TMUX コマンドエラー
- Agent: エージェント関連エラー
- Communication: 通信エラー

### セッション管理

- **マルチセッション対応**: 異なるセッション名で複数のチームを同時実行可能
- **セッション分離**: 各セッションは独立したワークスペースを持つ
- **動的ペイン解決**: セッション名に基づいてペインを特定

## ⚙️ 設定とカスタマイズ

### 環境変数設定 (`.env`)

```bash
# Claude Code CLI の設定
CLAUDE_CLI_PATH=claude

# TMUXセッション名
TMUX_SESSION_NAME=claude-code-team

# ワークスペースディレクトリ
WORKSPACE_DIR=./workspace

# ログ出力先
LOG_DIR=./logs

# ログレベル
RUST_LOG=info
```

## 🔧 トラブルシューティング

### よくある問題と解決法

#### 1. TMUX エラー: "can't find window"

```bash
# 既存セッションの確認と削除
tmux list-sessions
tmux kill-session -t claude-code-team

# 再起動
claude-code-team start
```

#### 2. Claude Code CLI が見つからない

```bash
# パスの確認
which claude

# .envファイルでパス指定
echo "CLAUDE_CLI_PATH=/path/to/claude" >> .env
```

#### 3. エージェントが応答しない

```bash
# ログでエラー確認
claude-code-team logs --agent brain

# TMUXセッションに直接アクセス
tmux attach-session -t claude-code-team
```

#### 4. 権限エラー

```bash
# ワークスペースディレクトリの権限確認
ls -la ./workspace/

# 必要に応じて権限変更
chmod -R 755 ./workspace/
```

#### 5. パフォーマンス最適化

```bash
# 詳細ログを有効化
export RUST_LOG=debug
claude-code-team start

# 特定エージェントのメモリ使用量確認
claude-code-team logs --agent brain | grep -i memory
```

## 🛠️ 開発・コントリビューション

### 開発環境セットアップ

```bash
# リポジトリクローン
git clone https://github.com/your-org/claude-code-team.git
cd claude-code-team

# 開発依存関係インストール
cargo build

# テスト実行
make test

# リント実行
make lint

# フォーマット
make fmt
```

### アーキテクチャ

```
src/
├── main.rs           # エントリーポイント
├── lib.rs            # ライブラリルート
├── error/            # エラー定義・ハンドリング
├── config/           # 設定管理
├── agent/            # エージェント管理
├── communication/    # エージェント間通信
├── session/          # TMUXセッション管理
└── cli/              # CLIインターフェース
```

### コーディング規約

- **TDD 採用**: テストファーストで開発
- **エラーハンドリング**: `./src/error` で統一管理
- **設定管理**: 環境変数は `.env` → `./src/config`
- **コマンド管理**: `Makefile` で開発コマンドを統一

### プルリクエスト

1. Fork this repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📈 実践例・ユースケース

### 🌐 Web アプリケーション開発

```bash
claude-code-team send brain "React + Node.js でタスク管理アプリを作成してください"
```

### 📱 モバイル API 開発

```bash
claude-code-team send brain "REST API サーバーを作ってください。ユーザー認証、データCRUD機能付きで"
```

### 🔧 CLI ツール開発

```bash
claude-code-team send brain "ファイル変換CLIツールを作成してください。JSON ↔ YAML ↔ TOML対応で"
```

### 🎯 マイクロサービス

```bash
claude-code-team send brain "Dockerコンテナ化されたマイクロサービスを作成してください"
```

## 📄 ライセンス

MIT License - 詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 🙏 謝辞

- [Anthropic](https://www.anthropic.com/) - Claude Code CLI の提供
- [Rust Community](https://www.rust-lang.org/) - 素晴らしい開発体験
- [tmux](https://github.com/tmux/tmux) - マルチペインターミナル管理

---

**Claude Code Team** で、AI 協業開発の新しい時代を体験してください！ 🚀

[![Star this repo](https://img.shields.io/github/stars/your-org/claude-code-team?style=social)](https://github.com/your-org/claude-code-team)
