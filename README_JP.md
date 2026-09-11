<p align="center">
  <img src="docs/images/logo.png" alt="Evoury Logo" width="120" height="120" style="border-radius: 20px;">
</p>

<h1 align="center">Evoury</h1>

<p align="center">
  <strong>オフラインファーストのクリエイティブアセットマネージャー</strong>
</p>

<p align="center">
  <a href="#features">機能</a> •
  <a href="#installation">インストール</a> •
  <a href="#development">開発</a> •
  <a href="#architecture">アーキテクチャ</a> •
  <a href="#contributing">貢献</a> •
  <a href="#license">ライセンス</a>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/version-0.1.0-blue.svg" alt="バージョン">
  <img src="https://img.shields.io/badge/rust-2021-orange.svg" alt="Rust バージョン">
  <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="ライセンス">
  <img src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg" alt="プラットフォーム">
</p>

---

## 概要

Evouryは、Tauri、React、Rustで構築された強力なオフラインファーストのクリエイティブアセットマネージャーです。パフォーマンスやプライバシーを妥協せずに、デジタルアセットへの高速で信頼性の高いアクセスを必要とするクリエイティブプロフェッショナルのために設計されています。

### Evouryを選ぶ理由

- **オフラインファースト**：アセットはローカルに保存されます。クラウド依存なし。
- **高速パフォーマンス**：Rustで構築され、ライブラリの規模に応じた性能。
- **モジュラーアーキテクチャ**：最大限の柔軟性を提供する40以上の専用クレート。
- **美しいUI**：ReactとTailwind CSSで構築されたモダンでレスポンシブなインターフェース。

---

## 機能

### コアエンジン

- **マルチフォーマット対応**：画像、動画、3Dモデル、音声、ドキュメントなど
- **スマートアセットペアリング**：関連ファイルを自動グループ化
- **アセットステートマシン**：発見、検証、インデックスからアーカイブまでのライフサイクル追跡
- **イベント駆動アーキテクチャ**：イベントバスによるサービス間の疎結合通信

### ライブラリ管理

- **高度なスキャナー**：フル、インクリメンタル、フォルダ指定、バックグラウンドスキャンモード
- **ファイルシステムウォッチャー**：手動リフレッシュなしのリアルタイム同期
- **メタデータパイプライン**：自動抽出、正規化、検証、キャッシュ
- **重複検出**：SHA256、パーセプチュアルハッシュ、メタデータベースの検出

### 検索と組織

- **永続化検索インデックス**：FTS5による超高速全文検索
- **スマートコレクション**：ルールベースの自動更新コレクション
- **高度なクエリ言語**：タイプ、タグ、評価、日付、カメラなどでフィルタリング
- **検索プロファイル**：検索設定の保存と切り替え

### ワークスペースシステム

- **永続化ワークスペース**：セッション全体の状態を記憶
- **複数ワークスペース**：異なるプロジェクトコンテキスト間の切り替え
- **ワークステーション**：レイアウト、ツール、ショートカット、テーマのプリセット
- **ドッキング可能なパネル**：完全にカスタマイズ可能なレイアウトエンジン

### 健康とメンテナンス

- **ヘルスエンジン**：ファイルシステム、データベース、キャッシュ、メタデータの整合性チェック
- **自動修復**：検出された問題のワンクリック修復
- **セッションリカバリ**：予期しないシャットダウン後のワークスペース復元
- **スリープモード**：アイドル時のリソース使用量最小化

---

## スクリーンショット

<p align="center">
  <img src="public/images/main_dark.webp" alt="メインインターフェース" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>メインインターフェース - ギャラリービュー</em>
</p>

<p align="center">
  <img src="public/images/main_light.webp" alt="インスペクターパネル" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>インスペクターパネル - アセット詳細</em>
</p>

<p align="center">
  <img src="public/images/settings.webp" alt="検索インターフェース" width="800" style="border-radius: 8px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
</p>

<p align="center">
  <em>高度な検索インターフェース</em>
</p>

---

## インストール

### 前提条件

- [Rust](https://www.rust-lang.org/tools/install)（最新安定版）
- [Node.js](https://nodejs.org/)（v18以上）
- [pnpm](https://pnpm.io/)（v8以上）

### ダウンロード

[Releases](https://github.com/mh3nj/evoury/releases)ページから最新バージョンをダウンロードしてください。

### ソースからビルド

```bash
# リポジトリをクローン
git clone https://github.com/mh3nj/evoury.git
cd evoury

# 依存関係をインストール
pnpm install

# 開発サーバーを起動
pnpm tauri dev

# 本番ビルド
pnpm tauri build
```

---

## 開発

### 利用可能なコマンド

```bash
# 開発
pnpm dev              # Vite開発サーバーを起動
pnpm tauri dev        # Tauri開発モードで起動

# ビルド
pnpm build            # フロントエンドをビルド
pnpm tauri build      # 本番アプリをビルド

# テスト
pnpm test             # フロントエンドテストを実行
cargo test            # Rustテストを実行

# リント
pnpm lint             # ESLintを実行
cargo clippy          # Clippyを実行

# フォーマット
pnpm format           # フロントエンドコードをフォーマット
cargo fmt             # Rustコードをフォーマット
```

---

## 技術スタック

### バックエンド

- **Rust** - システムプログラミング言語
- **Tauri** - デスクトップアプリケーションフレームワーク
- **SQLite** - ローカルデータベース
- **Crossbeam** - 並行プログラミングプリミティブ

### フロントエンド

- **React** - UIライブラリ
- **TypeScript** - 型安全なJavaScript
- **Tailwind CSS** - ユーティリティファーストのCSSフレームワーク
- **Zustand** - 状態管理
- **Vite** - ビルドツールと開発サーバー

---

## ロードマップ

詳細は[ROADMAP.md](ROADMAP.md)を参照してください。

---

## 貢献

貢献は大歓迎です！まず[CONTRIBUTING.md](CONTRIBUTING.md)をお読みください。

---

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細は[LICENSE](LICENSE)ファイルを参照してください。

---

## サポート

- **問題報告**：[GitHub Issues](https://github.com/mh3nj/evoury/issues)
- **ディスカッション**：[GitHub Discussions](https://github.com/mh3nj/evoury/discussions)

---

<p align="center">
  <a href="https://github.com/mh3nj">Mohsen Jafari</a> が ❤️ を込めて制作
</p>
