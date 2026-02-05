# Taurin

開発者向けのオールインワン・デスクトップツールキット。Tauri + Rust + Yew で構築された、軽量で高速なネイティブアプリケーションです。

## 特徴

- **軽量・高速**: Rust + WebAssembly による高いパフォーマンス
- **クロスプラットフォーム**: macOS / Windows / Linux 対応
- **オフライン動作**: インターネット接続不要で全機能が利用可能
- **ドラッグ&ドロップ**: ファイルをドロップするだけで自動的に適切なツールが起動

## 機能一覧

### Media

| ツール | 説明 |
|--------|------|
| **Image Compress** | 画像圧縮。WebP, AVIF, PNG, JPEG 形式に対応。品質調整可能 |
| **Image Edit** | 画像編集。リサイズ、回転、フィルター適用（グレースケール、セピア、ぼかし等） |

### Documents

| ツール | 説明 |
|--------|------|
| **CSV Viewer** | CSV/TSV ファイルの閲覧。ソート、フィルター、検索機能付き |
| **PDF Tools** | PDF の情報表示、テキスト抽出、ページ分割 |
| **Markdown** | Markdown から PDF への変換。リアルタイムプレビュー付き |
| **Text Diff** | 2つのテキストの差分比較。行単位・文字単位のハイライト表示 |
| **JSON Formatter** | JSON の整形、圧縮、バリデーション、ツリービュー表示、検索 |

### Generators

| ツール | 説明 |
|--------|------|
| **UUID** | UUID v4 / v7 の生成。一括生成、コピー機能付き |
| **Password** | セキュアなパスワード生成。長さ、文字種、除外文字の指定可能 |
| **Unit Converter** | 単位変換。長さ、重さ、温度、データサイズなど多数のカテゴリ対応 |
| **Regex Tester** | 正規表現のテスト。マッチ結果のハイライト、グループキャプチャ表示 |

### Productivity

| ツール | 説明 |
|--------|------|
| **Kanban Board** | シンプルなカンバンボード。タスク管理、ドラッグ&ドロップ対応 |
| **Notes** | メモ帳（スクラッチパッド）。複数ノート管理、自動保存、エクスポート機能 |

## 技術スタック

- **Backend**: [Tauri](https://tauri.app/) v2 + [Rust](https://www.rust-lang.org/)
- **Frontend**: [Yew](https://yew.rs/) (Rust WebAssembly フレームワーク)
- **Build**: [Trunk](https://trunkrs.dev/) (WASM バンドラー)

## 必要要件

- Rust 1.70+
- Node.js 18+ (開発時のみ)

## インストール

### 依存関係のインストール

```bash
# WASM ターゲットの追加
rustup target add wasm32-unknown-unknown

# Trunk のインストール
cargo install trunk

# Tauri CLI のインストール
cargo install tauri-cli
```

### 開発

```bash
# 開発サーバーの起動（ホットリロード対応）
make dev
# または
cargo tauri dev
```

### ビルド

```bash
# リリースビルド
make build
# または
cargo tauri build
```

ビルド成果物は `src-tauri/target/release/bundle/` に生成されます。

## コマンド一覧

| コマンド | 説明 |
|----------|------|
| `make dev` | 開発サーバー起動 |
| `make build` | リリースビルド |
| `make clean` | ビルド成果物の削除 |
| `make fmt` | コードフォーマット |
| `make lint` | Lint チェック |
| `make test` | テスト実行 |
| `make info` | Tauri 設定の確認 |

## プロジェクト構成

```
taurin/
├── src/                    # フロントエンド (Yew)
│   ├── app.rs             # メインアプリケーション
│   └── components/        # UIコンポーネント
├── src-tauri/             # バックエンド (Tauri/Rust)
│   ├── src/
│   │   ├── lib.rs        # Tauri コマンド定義
│   │   └── *.rs          # 各機能のモジュール
│   └── tauri.conf.json   # Tauri 設定
├── styles.css             # グローバルスタイル
└── index.html             # エントリーポイント
```

## ライセンス

MIT License - 詳細は [LICENSE](LICENSE) を参照してください。
