<p align="center">🌐 <b>語言</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms（Rust 倉庫管理系統）

一套以 **Rust + Axum + SeaORM** 撰寫的**本地化、單一檔案、零外部資料庫依賴**的倉庫管理系統（WMS）。
內嵌 **SQLite**，內建完整 Web 前端（HTML/CSS/JS）與 JWT 認證，**開箱即用、單一二進位檔部署**，支援 Windows / macOS / Linux。

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> 本地優先：資料存於本機 SQLite，無需安裝 MySQL/PostgreSQL，無需雲端帳號；啟動即用。

---

## 為何選擇 rwms

- **單一二進位檔 + 內嵌 SQLite**：無外部資料庫、無中介軟體，`cargo build --release` 後一個 exe 即可執行。
- **完整 Web UI**：登入、首頁儀表板、物資 / 分類 / 倉庫 / 供應商 / 客戶 / 入庫 / 出庫 / 庫存等頁面開箱即用。
- **DDD 四層架構**：介面層 → 應用層 → 領域層 ← 基礎設施層，邊界清晰、易維護、易擴充。
- **JWT 認證 + 中介軟體**：公開路由與受保護路由分離，Bearer Token 驗證。

---

## 功能

- **認證**：`POST /api/login` 取得 JWT；`POST /api/logout` 登出；受保護介面統一由 JWT 中介軟體校驗。
- **物資管理**：物資列表 / 詳情 / 新增 / 更新 / 刪除（分頁）。
- **分類管理**：物資分類的增刪改查。
- **倉庫管理**：多倉庫資訊維護。
- **供應商管理**：供應商資訊維護。
- **客戶管理**：客戶資訊維護。
- **入庫單**：建立（含明細）、列表、詳情、稽核（通過/拒絕）、刪除、明細查詢。
- **出庫單**：建立（含明細）、列表、詳情、稽核（通過/拒絕）、刪除、明細查詢。
- **庫存查詢**：多條件過濾 + 分頁查詢庫存。
- **健康檢查**：`GET /health` 回傳 `{"status":"UP"}`，便於容器/K8s 探活。

## 技術棧與架構

| 層 | 技術 / 說明 |
|---|---|
| Web 框架 | `axum 0.7`（路由、擷取器、中介軟體） |
| 非同步執行環境 | `tokio 1`（full） |
| ORM / 資料庫 | `sea-orm 0.12` + SQLite（`sqlx-sqlite`，純本地） |
| 序列化 | `serde` / `serde_json` |
| 認證 | `jsonwebtoken`（JWT）+ `auth_middleware` |
| 設定 | `application.yml` + `dotenvy` |
| 日誌 | `tracing` / `tracing-subscriber` / `tracing-appender`（滾動檔案日誌） |
| 前端 | 原生 HTML5 + CSS3 + JavaScript（`static/`，由 `ServeDir` 託管） |
| 安裝程式 | Inno Setup（Windows，可選） |

**DDD 四層架構**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (Service 用例編排)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM Repo 實作 / 設定 / 資料庫)
```

## 前端頁面

`static/` 內建頁面：`login.html`（登入）、`index.html`（首頁）、`material.html`（物資）、`category.html`（分類）、`warehouse.html`（倉庫）、`supplier.html`（供應商）、`customer.html`（客戶）、`inorder.html`（入庫單）、`outorder.html`（出庫單）、`stock.html`（庫存）。

## 支援平台

| 平台 | 狀態 | 說明 |
|---|---|---|
| Windows x64 | ✅ | 推薦；亦提供 Inno Setup 安裝程式腳本 |
| macOS（Intel / Apple Silicon） | ✅ CI 建置 | 通用二進位檔由 `build.yml` 產出 |
| Linux x64 | ✅ CI 建置 | 資料目錄預設 `/wms`（可掛載磁碟區） |

## 建置

需要 Rust **1.85+**（`edition = "2024"`）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# 需 Visual Studio Build Tools（含 C++ 與 Windows SDK）
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## 預編譯 / 跨平台產物

- **Releases**：<https://github.com/shuying-code/rwms/releases>（Windows / macOS universal / Linux x64）。
- **CI 建置矩陣**：`.github/workflows/ci.yml` 三平台建置；`build.yml` 打包產物，打 tag 時自動附加至 Release。
- **容器映像（GHCR）**：

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# 瀏覽器開啟 http://localhost:8080/login.html
```

## 使用

```bash
rwms                 # 啟動服務（預設 0.0.0.0:8080）
```

- 存取位址：<http://localhost:8080/login.html>
- 預設帳號：`admin` / `admin`（**生產環境請務必修改**）
- 健康檢查：<http://localhost:8080/health>

啟動時會在資料目錄建立 `wms.db` 並自動建表（首次啟動自動完成建表與遷移）。

## 設定

設定來自 `application.yml`（可用 `APP_ENV` 切換），常用環境變數：

| 變數 | 說明 |
|---|---|
| `PORT` | 監聽埠（預設取 `application.yml` 的 `server.port`，即 8080） |
| `JWT_SECRET` | JWT 金鑰（覆寫設定中的 `jwt.secret`，**生產必須設定**） |
| `APP_ENV` | 執行環境標識（dev / test / prod） |
| `RUST_LOG` | 日誌等級過濾（如 `info`、`debug`） |

## 資料庫與資料目錄

SQLite 檔案 `wms.db` 的位置由程式依平台決定：

| 平台 | 路徑 |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db`（Docker 建議掛載磁碟區至 `/wms`） |

日誌檔與資料庫同目錄（`wms.log`，按天滾動）。

## API 概覽

**公開路由（無需認證）**：`GET /`、`GET /health`、`POST /api/login`。

**受保護路由（需 `Authorization: Bearer <token>`）**：

| 資源 | 端點 |
|---|---|
| 登出 | `POST /api/logout` |
| 物資 | `GET/POST /api/materials`、`GET/DELETE /api/materials/:id`、`POST /api/materials/update` |
| 分類 | `GET/POST /api/categories`、`GET/DELETE /api/categories/:id`、`POST /api/categories/update` |
| 倉庫 | `GET/POST /api/warehouses`、`GET/DELETE /api/warehouses/:id`、`POST /api/warehouses/update` |
| 供應商 | `GET/POST /api/suppliers`、`GET/DELETE /api/suppliers/:id`、`POST /api/suppliers/update` |
| 客戶 | `GET/POST /api/customers`、`GET/DELETE /api/customers/:id`、`POST /api/customers/update` |
| 入庫單 | `GET/POST /api/in-orders`、`GET/DELETE /api/in-orders/:id`、`POST /api/in-orders/audit`、`GET /api/in-orders/:id/items` |
| 出庫單 | `GET/POST /api/out-orders`、`GET/DELETE /api/out-orders/:id`、`POST /api/out-orders/audit`、`GET /api/out-orders/:id/items` |
| 庫存 | `GET /api/stocks` |

## 專案結構

```
src/
  main.rs                  程式入口：載入設定、初始化日誌/資料庫、組裝路由、啟動服務
  state.rs                 應用共享狀態（各應用層服務實例）
  interfaces/              介面層：routes / handlers / DTO
  application/             應用層：各業務 Service（認證/物資/分類/倉庫/供應商/客戶/入庫/出庫/庫存）
  domain/                  領域層：aggregate / repository trait / domain_service
  infrastructure/          基礎設施層：SeaORM model & repository 實作 / 設定 / 建表 / JWT
static/                    前端頁面（HTML/CSS/JS）
application.yml            應用設定
installer.iss / setup.iss  Inno Setup 安裝程式腳本（Windows）
```

## 驗收

- 健康檢查 `GET /health` 回傳 `{"status":"UP","service":"WMS"}`。
- `GET /` 回傳啟動成功訊息。
- CI 三平台（Windows / macOS / Ubuntu）建置通過。

## 常見問題（FAQ）

**Q：需要安裝資料庫嗎？**
不需要。內嵌 SQLite，首次啟動自動建立 `wms.db` 並建表。

**Q：預設帳號是什麼？**
`admin` / `admin`。正式使用前請修改密碼與 `JWT_SECRET`。

**Q：連接埠被佔用怎麼辦？**
用 `PORT=9090`（或修改 `application.yml` 的 `server.port`）啟動。

**Q：資料目錄在哪？**
見「資料庫與資料目錄」；Windows 為 `C:\ProgramData\wms`，Linux/macOS 為 `/wms`。

## 問題回報（Issues）

遇到問題或有建議？歡迎透過 GitHub Issues 回報：<https://github.com/shuying-code/rwms/issues/new/choose>。提交前請先閱讀 **[使用回報指南](docs/FEEDBACK.md)**。

- **🐞 Bug / 問題回報**：可重現的功能異常、崩潰、錯誤
- **💡 功能建議**：新功能或改進
- **❓ 使用諮詢 / 回報**：用法、設定與一般性回報

> 維護者通常**每週集中處理一次** issue。回報越具體（版本、系統、重現步驟、錯誤、最小範例），修復越快。

## 更新摘要

- 首個公開版本：完整 WMS 後端 API + 內建 Web 前端 + JWT 認證。
- 跨平台建置（Windows / macOS / Linux）+ 容器映像（GHCR）。
- 詳見 `CHANGELOG.md` 與 `DEPLOYMENT.md`。

## 文件

| 檔案 | 內容 |
|---|---|
| `DEPLOYMENT.md` | 部署與打包說明 |
| `docs/FEEDBACK.md` | 使用回報指南 |
| `docs/README.md` | 文件索引 |

## 授權

[MIT](LICENSE)
