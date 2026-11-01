<p align="center">🌐 <b>言語</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms（Rust 倉庫管理システム）

**Rust + Axum + SeaORM** で書かれた**ローカルファースト・単一バイナリ・外部データベース不要**の倉庫管理システム（WMS）です。
**SQLite** を内蔵し、完全な Web フロントエンド（HTML/CSS/JS）と JWT 認証を備え、**すぐに使えて単一バイナリで配布**、Windows / macOS / Linux に対応します。

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> ローカルファースト：データはローカルの SQLite に保存され、MySQL/PostgreSQL のインストールもクラウドアカウントも不要。起動するだけです。

---

## rwms を選ぶ理由

- **単一バイナリ + SQLite 内蔵**：外部 DB もミドルウェアも不要。`cargo build --release` で得た 1 つの exe だけで動作します。
- **充実した Web UI**：ログイン、ダッシュボード、資材 / 分類 / 倉庫 / 仕入先 / 顧客 / 入庫 / 出庫 / 在庫などの画面をすぐに利用できます。
- **DDD 4 層アーキテクチャ**：interface → application → domain ← infrastructure。境界が明確で保守・拡張が容易です。
- **JWT 認証 + ミドルウェア**：公開ルートと保護ルートを分離し、Bearer トークンを検証します。

---

## 機能

- **認証**：`POST /api/login` で JWT を取得、`POST /api/logout` でログアウト。保護 API は JWT ミドルウェアで検証。
- **資材管理**：一覧 / 詳細 / 追加 / 更新 / 削除（ページング）。
- **分類管理**：資材カテゴリの CRUD。
- **倉庫管理**：複数倉庫の情報管理。
- **仕入先管理**：仕入先情報の管理。
- **顧客管理**：顧客情報の管理。
- **入庫伝票**：作成（明細付き）、一覧、詳細、承認（承認/却下）、削除、明細照会。
- **出庫伝票**：作成（明細付き）、一覧、詳細、承認（承認/却下）、削除、明細照会。
- **在庫照会**：複数条件での絞り込み + ページング。
- **ヘルスチェック**：`GET /health` が `{"status":"UP"}` を返し、コンテナ/K8s の存活監視に利用できます。

## 技術スタックとアーキテクチャ

| レイヤ | 技術 / 説明 |
|---|---|
| Web フレームワーク | `axum 0.7`（ルーティング、エクストラクタ、ミドルウェア） |
| 非同期ランタイム | `tokio 1`（full） |
| ORM / データベース | `sea-orm 0.12` + SQLite（`sqlx-sqlite`、完全ローカル） |
| シリアライズ | `serde` / `serde_json` |
| 認証 | `jsonwebtoken`（JWT）+ `auth_middleware` |
| 設定 | `application.yml` + `dotenvy` |
| ログ | `tracing` / `tracing-subscriber` / `tracing-appender`（ローテーションログ） |
| フロントエンド | 素の HTML5 + CSS3 + JavaScript（`static/`、`ServeDir` で配信） |
| インストーラ | Inno Setup（Windows、任意） |

**DDD 4 層アーキテクチャ**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (Service ユースケース編成)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM Repo 実装 / 設定 / データベース)
```

## フロントエンドページ

`static/` 内のページ：`login.html`（ログイン）、`index.html`（ホーム）、`material.html`（資材）、`category.html`（分類）、`warehouse.html`（倉庫）、`supplier.html`（仕入先）、`customer.html`（顧客）、`inorder.html`（入庫）、`outorder.html`（出庫）、`stock.html`（在庫）。

## 対応プラットフォーム

| プラットフォーム | 状態 | 説明 |
|---|---|---|
| Windows x64 | ✅ | 推奨。Inno Setup インストーラスクリプトも同梱 |
| macOS（Intel / Apple Silicon） | ✅ CI ビルド | ユニバーサルバイナリを `build.yml` で生成 |
| Linux x64 | ✅ CI ビルド | 既定のデータディレクトリは `/wms`（ボリュームをマウント可能） |

## ビルド

Rust **1.85+** が必要です（`edition = "2024"`）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# Visual Studio Build Tools（C++ と Windows SDK）が必要
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## ビルド済み / クロスプラットフォーム成果物

- **Releases**：<https://github.com/shuying-code/rwms/releases>（Windows / macOS universal / Linux x64）。
- **CI マトリクス**：`.github/workflows/ci.yml` が 3 プラットフォームでビルド。`build.yml` が成果物をパッケージし、タグ時に Release へ添付します。
- **コンテナイメージ（GHCR）**：

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# ブラウザで http://localhost:8080/login.html を開く
```

## 使い方

```bash
rwms                 # サーバー起動（既定 0.0.0.0:8080）
```

- URL：<http://localhost:8080/login.html>
- 既定アカウント：`admin` / `admin`（**本番では必ず変更してください**）
- ヘルスチェック：<http://localhost:8080/health>

起動時にデータディレクトリへ `wms.db` を作成し、テーブルを自動作成します（初回起動でスキーマ作成とマイグレーションを実行）。

## 設定

設定は `application.yml` から読み込みます（`APP_ENV` で切り替え可能）。主な環境変数：

| 変数 | 説明 |
|---|---|
| `PORT` | 待受ポート（既定は `application.yml` の `server.port`、すなわち 8080） |
| `JWT_SECRET` | JWT シークレット（`jwt.secret` を上書き、**本番では必須**） |
| `APP_ENV` | 実行環境（dev / test / prod） |
| `RUST_LOG` | ログレベルフィルタ（例：`info`、`debug`） |

## データベースとデータディレクトリ

SQLite ファイル `wms.db` の位置はプラットフォームごとに決まります：

| プラットフォーム | パス |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db`（Docker では `/wms` にボリュームをマウント推奨） |

ログファイルはデータベースと同じディレクトリに出力されます（`wms.log`、日次ローテーション）。

## API 概要

**公開ルート（認証不要）**：`GET /`、`GET /health`、`POST /api/login`。

**保護ルート（`Authorization: Bearer <token>` が必要）**：

| リソース | エンドポイント |
|---|---|
| ログアウト | `POST /api/logout` |
| 資材 | `GET/POST /api/materials`、`GET/DELETE /api/materials/:id`、`POST /api/materials/update` |
| 分類 | `GET/POST /api/categories`、`GET/DELETE /api/categories/:id`、`POST /api/categories/update` |
| 倉庫 | `GET/POST /api/warehouses`、`GET/DELETE /api/warehouses/:id`、`POST /api/warehouses/update` |
| 仕入先 | `GET/POST /api/suppliers`、`GET/DELETE /api/suppliers/:id`、`POST /api/suppliers/update` |
| 顧客 | `GET/POST /api/customers`、`GET/DELETE /api/customers/:id`、`POST /api/customers/update` |
| 入庫 | `GET/POST /api/in-orders`、`GET/DELETE /api/in-orders/:id`、`POST /api/in-orders/audit`、`GET /api/in-orders/:id/items` |
| 出庫 | `GET/POST /api/out-orders`、`GET/DELETE /api/out-orders/:id`、`POST /api/out-orders/audit`、`GET /api/out-orders/:id/items` |
| 在庫 | `GET /api/stocks` |

## プロジェクト構成

```
src/
  main.rs                  エントリ：設定読込、ログ/DB 初期化、ルート構成、サーバー起動
  state.rs                 共有アプリ状態（各サービスインスタンス）
  interfaces/              インターフェース層：routes / handlers / DTO
  application/             アプリケーション層：各 Service（認証/資材/分類/倉庫/仕入先/顧客/入庫/出庫/在庫）
  domain/                  ドメイン層：aggregate / repository trait / domain_service
  infrastructure/          インフラ層：SeaORM model & repository 実装 / 設定 / スキーマ / JWT
static/                    フロントエンドページ（HTML/CSS/JS）
application.yml            アプリ設定
installer.iss / setup.iss  Inno Setup インストーラスクリプト（Windows）
```

## 動作確認

- `GET /health` が `{"status":"UP","service":"WMS"}` を返す。
- `GET /` が起動成功メッセージを返す。
- CI が 3 プラットフォーム（Windows / macOS / Ubuntu）でビルド成功。

## よくある質問（FAQ）

**Q：データベースのインストールは必要ですか？**
不要です。SQLite を内蔵し、初回起動時に `wms.db` を作成してテーブルを構築します。

**Q：既定アカウントは？**
`admin` / `admin`。本番運用前にパスワードと `JWT_SECRET` を変更してください。

**Q：ポートが使用中です。どうすれば？**
`PORT=9090` で起動するか、`application.yml` の `server.port` を変更してください。

**Q：データディレクトリはどこ？**
「データベースとデータディレクトリ」を参照。Windows は `C:\ProgramData\wms`、Linux/macOS は `/wms`。

## フィードバック（Issues）

問題や提案は GitHub Issues へ：<https://github.com/shuying-code/rwms/issues/new/choose>。投稿前に **[フィードバックガイド](docs/FEEDBACK.md)** をお読みください。

- **🐞 バグ報告**：再現可能な不具合、クラッシュ、エラー
- **💡 機能要望**：新機能や改善
- **❓ 質問 / フィードバック**：使い方、設定、一般的な意見

> メンテナは通常**週に一度** issue をまとめて対応します。報告が具体的（バージョン、OS、再現手順、エラー、最小サンプル）なほど早く修正できます。

## 更新サマリ

- 初回公開版：WMS バックエンド API 一式 + Web フロントエンド内蔵 + JWT 認証。
- クロスプラットフォームビルド（Windows / macOS / Linux）+ コンテナイメージ（GHCR）。
- 詳細は `CHANGELOG.md` と `DEPLOYMENT.md` を参照。

## ドキュメント

| ファイル | 内容 |
|---|---|
| `DEPLOYMENT.md` | デプロイとパッケージング |
| `docs/FEEDBACK.md` | フィードバックガイド |
| `docs/README.md` | ドキュメント索引 |

## ライセンス

[MIT](LICENSE)
