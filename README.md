# rwms（Rust Warehouse Management System）

一个用 **Rust + Axum + SeaORM** 编写的**本地化、单文件、零外部数据库依赖**的仓库管理系统（WMS）。
内嵌 **SQLite**，内置完整 Web 前端（HTML/CSS/JS）与 JWT 认证，**开箱即用、单二进制部署**，支持 Windows / macOS / Linux。

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> 本地优先：数据存于本机 SQLite，无需安装 MySQL/PostgreSQL，无需云端账号；启动即用。

---

## 为什么选择 rwms

- **单二进制 + 内嵌 SQLite**：无外部数据库、无中间件，`cargo build --release` 后一个 exe 即可运行。
- **完整 Web UI**：登录、首页仪表盘、物资 / 分类 / 仓库 / 供应商 / 客户 / 入库 / 出库 / 库存等页面开箱即用。
- **DDD 四层架构**：接口层 → 应用层 → 领域层 ← 基础设施层，边界清晰、易维护、易扩展。
- **JWT 认证 + 中间件**：公开路由与受保护路由分离，Bearer Token 校验。

---

## 功能

- **认证**：`POST /api/login` 获取 JWT；`POST /api/logout` 登出；受保护接口统一 JWT 中间件校验。
- **物资管理**：物资列表 / 详情 / 新增 / 更新 / 删除（分页）。
- **分类管理**：物资分类的增删改查。
- **仓库管理**：多仓库信息维护。
- **供应商管理**：供应商信息维护。
- **客户管理**：客户信息维护。
- **入库单**：入库单创建（含明细）、列表、详情、审核（通过/拒绝）、删除、明细查询。
- **出库单**：出库单创建（含明细）、列表、详情、审核（通过/拒绝）、删除、明细查询。
- **库存查询**：按多条件过滤 + 分页查询库存。
- **健康检查**：`GET /health` 返回 `{"status":"UP"}`，便于容器/K8s 探活。

## 技术栈与架构

| 层 | 技术 / 说明 |
|---|---|
| Web 框架 | `axum 0.7`（路由、提取器、中间件） |
| 异步运行时 | `tokio 1`（full） |
| ORM / 数据库 | `sea-orm 0.12` + SQLite（`sqlx-sqlite`，纯本地） |
| 序列化 | `serde` / `serde_json` |
| 认证 | `jsonwebtoken`（JWT）+ `auth_middleware` |
| 配置 | `application.yml` + `dotenvy` |
| 日志 | `tracing` / `tracing-subscriber` / `tracing-appender`（滚动文件日志） |
| 前端 | 原生 HTML5 + CSS3 + JavaScript（`static/`，由 `ServeDir` 托管） |
| 安装包 | Inno Setup（Windows，可选） |

**DDD 四层架构**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (Service 用例编排)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM Repo 实现 / 配置 / 数据库)
```

## 前端页面

`static/` 内置页面：`login.html`（登录）、`index.html`（首页）、`material.html`（物资）、`category.html`（分类）、`warehouse.html`（仓库）、`supplier.html`（供应商）、`customer.html`（客户）、`inorder.html`（入库单）、`outorder.html`（出库单）、`stock.html`（库存）。

## 支持的平台

| 平台 | 状态 | 说明 |
|---|---|---|
| Windows x64 | ✅ | 推荐；亦提供 Inno Setup 安装包脚本 |
| macOS（Intel / Apple Silicon） | ✅ CI 构建 | 通用二进制由 `build.yml` 产出 |
| Linux x64 | ✅ CI 构建 | 数据库目录默认 `/wms`（可用挂载卷） |

## 构建

需要 Rust **1.85+**（`edition = "2024"`）。

```bash
cargo build --release
```

### Windows（MSVC）

```powershell
# 需 Visual Studio Build Tools（含 C++ 与 Windows SDK）
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## 预编译 / 跨平台产物

- **Releases**：<https://github.com/shuying-code/rwms/releases>（Windows / macOS universal / Linux x64）。
- **CI 构建矩阵**：`.github/workflows/ci.yml` 三平台构建；`build.yml` 打包产物，打 tag 时自动附加到 Release。
- **容器镜像（GHCR）**：

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# 浏览器访问 http://localhost:8080/login.html
```

## 使用

```bash
rwms                 # 启动服务（默认 0.0.0.0:8080）
```

- 访问地址：<http://localhost:8080/login.html>
- 默认账号：`admin` / `admin`（**生产环境请务必修改**）
- 健康检查：<http://localhost:8080/health>

启动时会在数据库目录创建 `wms.db` 并自动建表（首次启动自动完成建表与迁移）。

## 配置

配置来自 `application.yml`（可用 `APP_ENV` 切换），常用环境变量：

| 变量 | 说明 |
|---|---|
| `PORT` | 监听端口（默认取 `application.yml` 的 `server.port`，即 8080） |
| `JWT_SECRET` | JWT 密钥（覆盖配置中的 `jwt.secret`，**生产必须设置**） |
| `APP_ENV` | 运行环境标识（dev / test / prod） |
| `RUST_LOG` | 日志级别过滤（如 `info`、`debug`） |

## 数据库与数据目录

SQLite 文件 `wms.db` 的位置由代码按平台决定：

| 平台 | 路径 |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db`（Docker 建议挂载卷到 `/wms`） |

日志文件与数据库同目录（`wms.log`，按天滚动）。

## API 概览

**公开路由（无需认证）**：`GET /`、`GET /health`、`POST /api/login`。

**受保护路由（需 `Authorization: Bearer <token>`）**：

| 资源 | 端点 |
|---|---|
| 登出 | `POST /api/logout` |
| 物资 | `GET/POST /api/materials`、`GET/DELETE /api/materials/:id`、`POST /api/materials/update` |
| 分类 | `GET/POST /api/categories`、`GET/DELETE /api/categories/:id`、`POST /api/categories/update` |
| 仓库 | `GET/POST /api/warehouses`、`GET/DELETE /api/warehouses/:id`、`POST /api/warehouses/update` |
| 供应商 | `GET/POST /api/suppliers`、`GET/DELETE /api/suppliers/:id`、`POST /api/suppliers/update` |
| 客户 | `GET/POST /api/customers`、`GET/DELETE /api/customers/:id`、`POST /api/customers/update` |
| 入库单 | `GET/POST /api/in-orders`、`GET/DELETE /api/in-orders/:id`、`POST /api/in-orders/audit`、`GET /api/in-orders/:id/items` |
| 出库单 | `GET/POST /api/out-orders`、`GET/DELETE /api/out-orders/:id`、`POST /api/out-orders/audit`、`GET /api/out-orders/:id/items` |
| 库存 | `GET /api/stocks` |

## 项目结构

```
src/
  main.rs                  程序入口：加载配置、初始化日志/数据库、装配路由、启动服务
  state.rs                 应用共享状态（各应用层服务实例）
  interfaces/              接口层：routes / handlers / DTO
  application/             应用层：各业务 Service（认证/物资/分类/仓库/供应商/客户/入库/出库/库存）
  domain/                  领域层：aggregate / repository trait / domain_service
  infrastructure/          基础设施层：SeaORM model & repository 实现 / 配置 / 建表 / JWT
static/                    前端页面（HTML/CSS/JS）
application.yml            应用配置
installer.iss / setup.iss  Inno Setup 安装包脚本（Windows）
```

## 验收

- 健康检查 `GET /health` 返回 `{"status":"UP","service":"WMS"}`。
- `GET /` 返回启动成功信息。
- CI 三平台（Windows / macOS / Ubuntu）构建通过。

## 常见问题（FAQ）

**Q：需要安装数据库吗？**
不需要。内嵌 SQLite，首次启动自动创建 `wms.db` 并建表。

**Q：默认账号是什么？**
`admin` / `admin`。正式使用前请修改密码与 `JWT_SECRET`。

**Q：端口被占用怎么办？**
用 `PORT=9090`（或修改 `application.yml` 的 `server.port`）启动。

**Q：数据目录在哪？**
见「数据库与数据目录」；Windows 为 `C:\ProgramData\wms`，Linux/macOS 为 `/wms`。

## 问题反馈（Issues）

遇到问题或有建议？欢迎通过 GitHub Issues 反馈：<https://github.com/shuying-code/rwms/issues/new/choose>。提交前请先阅读 **[使用反馈指南](docs/FEEDBACK.md)**。

- **🐞 Bug / 问题反馈**：可复现的功能异常、崩溃、报错
- **💡 功能建议**：新功能或改进
- **❓ 使用咨询 / 反馈**：用法、配置与一般性反馈

> 维护者通常**每周集中处理一次** issue。反馈越具体（版本、系统、复现步骤、报错、最小样例），修复越快。

## 更新摘要

- 首个公开版本：完整 WMS 后端 API + 内置 Web 前端 + JWT 认证。
- 跨平台构建（Windows / macOS / Linux）+ 容器镜像（GHCR）。
- 详见 `CHANGELOG.md` 与 `DEPLOYMENT.md`。

## 文档

| 文件 | 内容 |
|---|---|
| `DEPLOYMENT.md` | 部署与打包说明 |
| `docs/FEEDBACK.md` | 使用反馈指南 |
| `docs/README.md` | 文档索引 |

## 许可

[MIT](LICENSE)
