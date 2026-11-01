<p align="center">🌐 <b>Language</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Rust Warehouse Management System)

> 📘 **[User Manual](docs/用户操作手册.en.md)** · [简体中文手册](docs/用户操作手册.md)

A **local-first, single-binary, zero-external-database** Warehouse Management System (WMS) built with **Rust + Axum + SeaORM**. It embeds **SQLite**, ships a complete web frontend (HTML/CSS/JS) and JWT authentication — **ready out of the box, deploy as one binary** — on Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Local-first: data lives in a local SQLite file — no MySQL/PostgreSQL to install, no cloud account; just start it.

---

## Why rwms

- **Single binary + embedded SQLite**: no external DB, no middleware — one exe from `cargo build --release` is enough.
- **Complete Web UI**: login, dashboard, materials / categories / warehouses / suppliers / customers / inbound / outbound / stock pages out of the box.
- **DDD 4-layer architecture**: interfaces → application → domain ← infrastructure; clear boundaries, easy to maintain and extend.
- **JWT auth + middleware**: public and protected routes are separated; Bearer token validation.

---

## Features

- **Authentication**: `POST /api/login` to obtain a JWT; `POST /api/logout`; protected routes guarded by JWT middleware.
- **Materials**: list / detail / create / update / delete (paginated).
- **Categories**: CRUD for material categories.
- **Warehouses**: manage multiple warehouses.
- **Suppliers**: manage supplier information.
- **Customers**: manage customer information.
- **Inbound orders**: create (with items), list, detail, audit (approve/reject), delete, item query.
- **Outbound orders**: create (with items), list, detail, audit (approve/reject), delete, item query.
- **Stock query**: filter by multiple conditions + pagination.
- **Health check**: `GET /health` returns `{"status":"UP"}` for container/K8s liveness.

## Tech stack & architecture

| Layer | Tech / Notes |
|---|---|
| Web framework | `axum 0.7` (routing, extractors, middleware) |
| Async runtime | `tokio 1` (full) |
| ORM / database | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, fully local) |
| Serialization | `serde` / `serde_json` |
| Auth | `jsonwebtoken` (JWT) + `auth_middleware` |
| Config | `application.yml` + `dotenvy` |
| Logging | `tracing` / `tracing-subscriber` / `tracing-appender` (rolling file logs) |
| Frontend | Vanilla HTML5 + CSS3 + JavaScript (`static/`, served by `ServeDir`) |
| Installer | Inno Setup (Windows, optional) |

**DDD 4-layer architecture**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (service use-case orchestration)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM repository impl / config / database)
```

## Frontend pages

Pages in `static/`: `login.html` (login), `index.html` (home), `material.html` (materials), `category.html` (categories), `warehouse.html` (warehouses), `supplier.html` (suppliers), `customer.html` (customers), `inorder.html` (inbound), `outorder.html` (outbound), `stock.html` (stock).

## Supported platforms

| Platform | Status | Notes |
|---|---|---|
| Windows x64 | ✅ | Recommended; also ships an Inno Setup installer script |
| macOS (Intel / Apple Silicon) | ✅ CI build | Universal binary produced by `build.yml` |
| Linux x64 | ✅ CI build | Default data dir `/wms` (mountable volume) |

## Build

Requires Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Requires Visual Studio Build Tools (C++ and Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Prebuilt / cross-platform artifacts

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **CI matrix**: `.github/workflows/ci.yml` builds on three platforms; `build.yml` packages artifacts and attaches them to a Release on tag.
- **Container image (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# open http://localhost:8080/login.html
```

## Usage

```bash
rwms                 # start the server (default 0.0.0.0:8080)
```

- URL: <http://localhost:8080/login.html>
- Default account: `admin` / `admin` (**change it in production**)
- Health check: <http://localhost:8080/health>

On startup it creates `wms.db` in the data directory and auto-creates tables (schema and migrations run on first start).

## Configuration

Config comes from `application.yml` (switch with `APP_ENV`). Common environment variables:

| Variable | Description |
|---|---|
| `PORT` | Listen port (defaults to `server.port` in `application.yml`, i.e. 8080) |
| `JWT_SECRET` | JWT secret (overrides `jwt.secret`; **required in production**) |
| `APP_ENV` | Environment name (dev / test / prod) |
| `RUST_LOG` | Log level filter (e.g. `info`, `debug`) |

## Database & data directory

The location of the SQLite file `wms.db` is chosen by platform:

| Platform | Path |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker: mount a volume to `/wms`) |

Log files live in the same directory as the database (`wms.log`, rotated daily).

## API overview

**Public routes (no auth)**: `GET /`, `GET /health`, `POST /api/login`.

**Protected routes (require `Authorization: Bearer <token>`)**:

| Resource | Endpoints |
|---|---|
| Logout | `POST /api/logout` |
| Materials | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Categories | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Warehouses | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Suppliers | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Customers | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Inbound | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Outbound | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Stock | `GET /api/stocks` |

## Project structure

```
src/
  main.rs                  entry: load config, init logging/db, wire routes, start server
  state.rs                 shared app state (service instances)
  interfaces/              interface layer: routes / handlers / DTO
  application/             application layer: services (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  domain layer: aggregate / repository trait / domain_service
  infrastructure/          infrastructure: SeaORM model & repository impl / config / schema / JWT
static/                    frontend pages (HTML/CSS/JS)
application.yml            app config
installer.iss / setup.iss  Inno Setup installer scripts (Windows)
```

## Verification

- `GET /health` returns `{"status":"UP","service":"WMS"}`.
- `GET /` returns a startup success message.
- CI builds pass on all three platforms (Windows / macOS / Ubuntu).

## FAQ

**Q: Do I need to install a database?**
No. SQLite is embedded; `wms.db` is created and tables are built on first start.

**Q: What is the default account?**
`admin` / `admin`. Change the password and `JWT_SECRET` before real use.

**Q: The port is taken — what now?**
Start with `PORT=9090` (or change `server.port` in `application.yml`).

**Q: Where is the data directory?**
See "Database & data directory"; Windows is `C:\ProgramData\wms`, Linux/macOS is `/wms`.

## Feedback (Issues)

Found a problem or have a suggestion? Please use GitHub Issues: <https://github.com/shuying-code/rwms/issues/new/choose>. Read the **[feedback guide](docs/FEEDBACK.md)** before submitting.

- **🐞 Bug report**: reproducible errors, crashes, failures
- **💡 Feature request**: new features or improvements
- **❓ Question / feedback**: usage, configuration, general feedback

> Maintainers usually triage issues **once per week**. The more specific the report (version, OS, steps, errors, minimal sample), the faster the fix.

## Changelog summary

- First public release: full WMS backend API + built-in web frontend + JWT auth.
- Cross-platform builds (Windows / macOS / Linux) + container image (GHCR).
- See `CHANGELOG.md` and `DEPLOYMENT.md` for details.

## Documentation

| File | Content |
|---|---|
| `DEPLOYMENT.md` | Deployment & packaging |
| `docs/FEEDBACK.md` | Feedback guide |
| `docs/README.md` | Docs index |

## License

[MIT](LICENSE)
