<p align="center">🌐 <b>Langue</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Système de gestion d'entrepôt en Rust)

Un **système de gestion d'entrepôt (WMS) local, mono-binaire et sans base de données externe**, écrit en **Rust + Axum + SeaORM**.
Il intègre **SQLite**, un frontend web complet (HTML/CSS/JS) et l'authentification JWT — **prêt à l'emploi, déployable en un seul binaire** — sous Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Local d'abord : les données résident dans un fichier SQLite local — pas d'installation de MySQL/PostgreSQL, pas de compte cloud ; lancez-le simplement.

---

## Pourquoi rwms

- **Mono-binaire + SQLite intégré** : aucune base externe ni middleware — un seul exe issu de `cargo build --release` suffit.
- **Interface web complète** : connexion, tableau de bord, pages matériaux / catégories / entrepôts / fournisseurs / clients / entrées / sorties / stock prêtes à l'emploi.
- **Architecture DDD en 4 couches** : interfaces → application → domaine ← infrastructure ; frontières claires, maintenance et extension faciles.
- **Authentification JWT + middleware** : routes publiques et protégées séparées, validation du Bearer token.

---

## Fonctionnalités

- **Authentification** : `POST /api/login` pour obtenir un JWT ; `POST /api/logout` ; les API protégées passent par le middleware JWT.
- **Matériaux** : liste / détail / création / mise à jour / suppression (paginé).
- **Catégories** : CRUD des catégories de matériaux.
- **Entrepôts** : gestion de plusieurs entrepôts.
- **Fournisseurs** : gestion des informations fournisseurs.
- **Clients** : gestion des informations clients.
- **Bons d'entrée** : création (avec lignes), liste, détail, validation (approuver/rejeter), suppression, consultation des lignes.
- **Bons de sortie** : création (avec lignes), liste, détail, validation (approuver/rejeter), suppression, consultation des lignes.
- **Consultation du stock** : filtrage multi-critères + pagination.
- **Contrôle de santé** : `GET /health` renvoie `{"status":"UP"}` pour les sondes conteneur/K8s.

## Technologies et architecture

| Couche | Technologie / Notes |
|---|---|
| Framework web | `axum 0.7` (routing, extracteurs, middleware) |
| Runtime asynchrone | `tokio 1` (full) |
| ORM / base de données | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, entièrement local) |
| Sérialisation | `serde` / `serde_json` |
| Authentification | `jsonwebtoken` (JWT) + `auth_middleware` |
| Configuration | `application.yml` + `dotenvy` |
| Journalisation | `tracing` / `tracing-subscriber` / `tracing-appender` (logs rotatifs) |
| Frontend | HTML5 + CSS3 + JavaScript natifs (`static/`, servi par `ServeDir`) |
| Installateur | Inno Setup (Windows, optionnel) |

**Architecture DDD en 4 couches**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (orchestration des cas d'usage Service)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (implémentation SeaORM Repo / configuration / base de données)
```

## Pages du frontend

Pages dans `static/` : `login.html` (connexion), `index.html` (accueil), `material.html` (matériaux), `category.html` (catégories), `warehouse.html` (entrepôts), `supplier.html` (fournisseurs), `customer.html` (clients), `inorder.html` (entrées), `outorder.html` (sorties), `stock.html` (stock).

## Plateformes prises en charge

| Plateforme | État | Notes |
|---|---|---|
| Windows x64 | ✅ | Recommandé ; script d'installateur Inno Setup inclus |
| macOS (Intel / Apple Silicon) | ✅ build CI | Binaire universel produit par `build.yml` |
| Linux x64 | ✅ build CI | Répertoire de données par défaut `/wms` (volume montable) |

## Compilation

Requiert Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Nécessite Visual Studio Build Tools (C++ et Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Artefacts précompilés / multiplateformes

- **Releases** : <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **Matrice CI** : `.github/workflows/ci.yml` compile sur trois plateformes ; `build.yml` empaquette les artefacts et les joint à un Release lors du tag.
- **Image conteneur (GHCR)** :

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# ouvrir http://localhost:8080/login.html
```

## Utilisation

```bash
rwms                 # démarre le serveur (par défaut 0.0.0.0:8080)
```

- URL : <http://localhost:8080/login.html>
- Compte par défaut : `admin` / `admin` (**à changer en production**)
- Contrôle de santé : <http://localhost:8080/health>

Au démarrage, il crée `wms.db` dans le répertoire de données et génère automatiquement les tables (schéma et migrations au premier lancement).

## Configuration

La configuration provient de `application.yml` (bascule via `APP_ENV`). Variables d'environnement courantes :

| Variable | Description |
|---|---|
| `PORT` | Port d'écoute (par défaut `server.port` dans `application.yml`, soit 8080) |
| `JWT_SECRET` | Secret JWT (remplace `jwt.secret` ; **obligatoire en production**) |
| `APP_ENV` | Environnement (dev / test / prod) |
| `RUST_LOG` | Filtre de niveau de log (p. ex. `info`, `debug`) |

## Base de données et répertoire de données

L'emplacement du fichier SQLite `wms.db` dépend de la plateforme :

| Plateforme | Chemin |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker : monter un volume sur `/wms`) |

Les fichiers de logs se trouvent dans le même répertoire que la base (`wms.log`, rotation quotidienne).

## Aperçu de l'API

**Routes publiques (sans authentification)** : `GET /`, `GET /health`, `POST /api/login`.

**Routes protégées (nécessitent `Authorization: Bearer <token>`)** :

| Ressource | Endpoints |
|---|---|
| Déconnexion | `POST /api/logout` |
| Matériaux | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Catégories | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Entrepôts | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Fournisseurs | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Clients | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Entrées | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Sorties | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Stock | `GET /api/stocks` |

## Structure du projet

```
src/
  main.rs                  entrée : charge la config, init logs/DB, monte les routes, démarre le serveur
  state.rs                 état partagé de l'app (instances de services)
  interfaces/              couche interfaces : routes / handlers / DTO
  application/             couche application : services (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  couche domaine : aggregate / repository trait / domain_service
  infrastructure/          infrastructure : implémentation SeaORM model & repository / config / schéma / JWT
static/                    pages du frontend (HTML/CSS/JS)
application.yml            configuration de l'app
installer.iss / setup.iss  scripts d'installateur Inno Setup (Windows)
```

## Vérification

- `GET /health` renvoie `{"status":"UP","service":"WMS"}`.
- `GET /` renvoie un message de démarrage réussi.
- La CI compile sur les trois plateformes (Windows / macOS / Ubuntu).

## FAQ

**Q : Faut-il installer une base de données ?**
Non. SQLite est intégré ; `wms.db` est créé et les tables construites au premier démarrage.

**Q : Quel est le compte par défaut ?**
`admin` / `admin`. Changez le mot de passe et `JWT_SECRET` avant toute utilisation réelle.

**Q : Le port est occupé, que faire ?**
Démarrez avec `PORT=9090` (ou modifiez `server.port` dans `application.yml`).

**Q : Où se trouve le répertoire de données ?**
Voir « Base de données et répertoire de données » ; Windows : `C:\ProgramData\wms`, Linux/macOS : `/wms`.

## Retour d'information (Issues)

Un problème ou une suggestion ? Utilisez GitHub Issues : <https://github.com/shuying-code/rwms/issues/new/choose>. Lisez le **[guide de retour](docs/FEEDBACK.md)** avant de soumettre.

- **🐞 Rapport de bug** : erreurs reproductibles, plantages, échecs
- **💡 Demande de fonctionnalité** : nouvelles fonctions ou améliorations
- **❓ Question / retour** : utilisation, configuration, avis général

> Les mainteneurs traitent généralement les issues **une fois par semaine**. Plus le rapport est précis (version, OS, étapes, erreurs, exemple minimal), plus la correction est rapide.

## Résumé des mises à jour

- Première version publique : API backend WMS complète + frontend web intégré + authentification JWT.
- Compilations multiplateformes (Windows / macOS / Linux) + image conteneur (GHCR).
- Voir `CHANGELOG.md` et `DEPLOYMENT.md` pour plus de détails.

## Documentation

| Fichier | Contenu |
|---|---|
| `DEPLOYMENT.md` | Déploiement et empaquetage |
| `docs/FEEDBACK.md` | Guide de retour |
| `docs/README.md` | Index de la documentation |

## Licence

[MIT](LICENSE)
