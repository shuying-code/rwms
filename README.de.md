<p align="center">🌐 <b>Sprache</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Lagerverwaltungssystem in Rust)

Ein **lokales Lagerverwaltungssystem (WMS) als einzelne Binärdatei ohne externe Datenbank**, geschrieben in **Rust + Axum + SeaORM**.
Es enthält **SQLite**, ein vollständiges Web-Frontend (HTML/CSS/JS) und JWT-Authentifizierung — **sofort einsatzbereit, als ein einziges Binary bereitstellbar** — unter Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Local-first: Die Daten liegen in einer lokalen SQLite-Datei — keine MySQL-/PostgreSQL-Installation, kein Cloud-Konto; einfach starten.

---

## Warum rwms

- **Einzelnes Binary + integriertes SQLite**: keine externe DB, keine Middleware — eine exe aus `cargo build --release` genügt.
- **Vollständige Web-UI**: Anmeldung, Dashboard sowie Seiten für Materialien / Kategorien / Lager / Lieferanten / Kunden / Eingang / Ausgang / Bestand sofort nutzbar.
- **DDD-4-Schichten-Architektur**: interfaces → application → domain ← infrastructure; klare Grenzen, leicht zu warten und zu erweitern.
- **JWT-Authentifizierung + Middleware**: öffentliche und geschützte Routen getrennt, Bearer-Token-Prüfung.

---

## Funktionen

- **Authentifizierung**: `POST /api/login` für ein JWT; `POST /api/logout`; geschützte APIs laufen über die JWT-Middleware.
- **Materialien**: Liste / Detail / Anlegen / Ändern / Löschen (paginiert).
- **Kategorien**: CRUD für Materialkategorien.
- **Lager**: Verwaltung mehrerer Lager.
- **Lieferanten**: Verwaltung von Lieferantendaten.
- **Kunden**: Verwaltung von Kundendaten.
- **Eingangsbelege**: Anlegen (mit Positionen), Liste, Detail, Prüfung (genehmigen/ablehnen), Löschen, Positionsabfrage.
- **Ausgangsbelege**: Anlegen (mit Positionen), Liste, Detail, Prüfung (genehmigen/ablehnen), Löschen, Positionsabfrage.
- **Bestandsabfrage**: Filter nach mehreren Kriterien + Paginierung.
- **Health-Check**: `GET /health` liefert `{"status":"UP"}` für Container-/K8s-Probes.

## Technologie-Stack und Architektur

| Schicht | Technologie / Hinweis |
|---|---|
| Web-Framework | `axum 0.7` (Routing, Extractor, Middleware) |
| Async-Runtime | `tokio 1` (full) |
| ORM / Datenbank | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, vollständig lokal) |
| Serialisierung | `serde` / `serde_json` |
| Authentifizierung | `jsonwebtoken` (JWT) + `auth_middleware` |
| Konfiguration | `application.yml` + `dotenvy` |
| Logging | `tracing` / `tracing-subscriber` / `tracing-appender` (rotierende Logdateien) |
| Frontend | Reines HTML5 + CSS3 + JavaScript (`static/`, ausgeliefert via `ServeDir`) |
| Installer | Inno Setup (Windows, optional) |

**DDD-4-Schichten-Architektur**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (Service-Use-Case-Orchestrierung)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM-Repo-Implementierung / Konfiguration / Datenbank)
```

## Frontend-Seiten

Seiten in `static/`: `login.html` (Anmeldung), `index.html` (Start), `material.html` (Materialien), `category.html` (Kategorien), `warehouse.html` (Lager), `supplier.html` (Lieferanten), `customer.html` (Kunden), `inorder.html` (Eingang), `outorder.html` (Ausgang), `stock.html` (Bestand).

## Unterstützte Plattformen

| Plattform | Status | Hinweis |
|---|---|---|
| Windows x64 | ✅ | Empfohlen; Inno-Setup-Installer-Skript enthalten |
| macOS (Intel / Apple Silicon) | ✅ CI-Build | Universal-Binary via `build.yml` |
| Linux x64 | ✅ CI-Build | Standard-Datenverzeichnis `/wms` (mountbares Volume) |

## Build

Erfordert Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Benötigt Visual Studio Build Tools (C++ und Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Vorgefertigte / plattformübergreifende Artefakte

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **CI-Matrix**: `.github/workflows/ci.yml` baut auf drei Plattformen; `build.yml` packt Artefakte und hängt sie bei einem Tag an ein Release.
- **Container-Image (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# http://localhost:8080/login.html im Browser öffnen
```

## Verwendung

```bash
rwms                 # Server starten (Standard 0.0.0.0:8080)
```

- URL: <http://localhost:8080/login.html>
- Standardkonto: `admin` / `admin` (**in Produktion unbedingt ändern**)
- Health-Check: <http://localhost:8080/health>

Beim Start wird `wms.db` im Datenverzeichnis angelegt und Tabellen automatisch erstellt (Schema und Migrationen beim ersten Start).

## Konfiguration

Die Konfiguration stammt aus `application.yml` (umschaltbar via `APP_ENV`). Häufige Umgebungsvariablen:

| Variable | Beschreibung |
|---|---|
| `PORT` | Listen-Port (Standard `server.port` aus `application.yml`, also 8080) |
| `JWT_SECRET` | JWT-Secret (überschreibt `jwt.secret`; **in Produktion erforderlich**) |
| `APP_ENV` | Umgebung (dev / test / prod) |
| `RUST_LOG` | Log-Level-Filter (z. B. `info`, `debug`) |

## Datenbank und Datenverzeichnis

Der Speicherort der SQLite-Datei `wms.db` hängt von der Plattform ab:

| Plattform | Pfad |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker: Volume nach `/wms` mounten) |

Logdateien liegen im selben Verzeichnis wie die Datenbank (`wms.log`, tägliche Rotation).

## API-Überblick

**Öffentliche Routen (ohne Auth)**: `GET /`, `GET /health`, `POST /api/login`.

**Geschützte Routen (erfordern `Authorization: Bearer <token>`)**:

| Ressource | Endpunkte |
|---|---|
| Abmelden | `POST /api/logout` |
| Materialien | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Kategorien | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Lager | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Lieferanten | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Kunden | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Eingang | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Ausgang | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Bestand | `GET /api/stocks` |

## Projektstruktur

```
src/
  main.rs                  Einstieg: Konfiguration laden, Logging/DB initialisieren, Routen verdrahten, Server starten
  state.rs                 gemeinsamer App-Status (Service-Instanzen)
  interfaces/              Interface-Schicht: routes / handlers / DTO
  application/             Anwendungsschicht: Services (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  Domänenschicht: aggregate / repository trait / domain_service
  infrastructure/          Infrastruktur: SeaORM model & repository-Implementierung / Konfiguration / Schema / JWT
static/                    Frontend-Seiten (HTML/CSS/JS)
application.yml            App-Konfiguration
installer.iss / setup.iss  Inno-Setup-Installer-Skripte (Windows)
```

## Verifikation

- `GET /health` liefert `{"status":"UP","service":"WMS"}`.
- `GET /` liefert eine Erfolgsmeldung beim Start.
- CI baut auf allen drei Plattformen (Windows / macOS / Ubuntu) erfolgreich.

## FAQ

**F: Muss ich eine Datenbank installieren?**
Nein. SQLite ist integriert; `wms.db` wird beim ersten Start erstellt und Tabellen werden angelegt.

**F: Wie lautet das Standardkonto?**
`admin` / `admin`. Vor dem echten Einsatz Passwort und `JWT_SECRET` ändern.

**F: Der Port ist belegt — was nun?**
Mit `PORT=9090` starten (oder `server.port` in `application.yml` ändern).

**F: Wo ist das Datenverzeichnis?**
Siehe „Datenbank und Datenverzeichnis"; Windows: `C:\ProgramData\wms`, Linux/macOS: `/wms`.

## Feedback (Issues)

Problem oder Vorschlag? Nutze GitHub Issues: <https://github.com/shuying-code/rwms/issues/new/choose>. Lies vor dem Einreichen den **[Feedback-Leitfaden](docs/FEEDBACK.md)**.

- **🐞 Bug-Report**: reproduzierbare Fehler, Abstürze, Ausfälle
- **💡 Funktionswunsch**: neue Funktionen oder Verbesserungen
- **❓ Frage / Feedback**: Nutzung, Konfiguration, allgemeines Feedback

> Maintainer bearbeiten Issues in der Regel **einmal pro Woche**. Je konkreter der Bericht (Version, OS, Schritte, Fehler, Minimalbeispiel), desto schneller die Behebung.

## Änderungsübersicht

- Erste öffentliche Version: vollständige WMS-Backend-API + integriertes Web-Frontend + JWT-Authentifizierung.
- Plattformübergreifende Builds (Windows / macOS / Linux) + Container-Image (GHCR).
- Details siehe `CHANGELOG.md` und `DEPLOYMENT.md`.

## Dokumentation

| Datei | Inhalt |
|---|---|
| `DEPLOYMENT.md` | Deployment und Paketierung |
| `docs/FEEDBACK.md` | Feedback-Leitfaden |
| `docs/README.md` | Dokumentationsindex |

## Lizenz

[MIT](LICENSE)
