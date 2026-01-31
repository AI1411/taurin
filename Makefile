.PHONY: dev build clean install fmt lint test

# 開発サーバー起動（ホットリロード対応）
dev:
	cargo tauri dev

# リリースビルド
build:
	cargo tauri build

# ビルド成果物のクリーンアップ
clean:
	cargo clean
	rm -rf dist

# 依存関係のインストール
install:
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	cargo install tauri-cli

# コードフォーマット
fmt:
	cargo fmt --all

# Lint チェック
lint:
	cargo clippy --all-targets --all-features

# テスト実行
test:
	cargo test --all

# フロントエンドのみビルド
frontend:
	trunk build

# フロントエンド開発サーバー（Tauriなし）
frontend-dev:
	trunk serve

# Tauri設定の確認
info:
	cargo tauri info
