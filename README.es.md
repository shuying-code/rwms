<p align="center">🌐 <b>Idioma</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Sistema de Gestión de Almacenes en Rust)

Un **sistema de gestión de almacenes (WMS) local, de un solo binario y sin base de datos externa**, escrito en **Rust + Axum + SeaORM**.
Incorpora **SQLite**, un frontend web completo (HTML/CSS/JS) y autenticación JWT — **listo para usar y desplegar como un único binario** — en Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Local primero: los datos se almacenan en un archivo SQLite local — sin instalar MySQL/PostgreSQL ni cuenta en la nube; simplemente ejecútalo.

---

## Por qué rwms

- **Un solo binario + SQLite integrado**: sin base de datos externa ni middleware — basta un exe de `cargo build --release`.
- **Interfaz web completa**: inicio de sesión, panel, páginas de materiales / categorías / almacenes / proveedores / clientes / entradas / salidas / inventario listas para usar.
- **Arquitectura DDD de 4 capas**: interfaces → aplicación → dominio ← infraestructura; límites claros, fácil de mantener y ampliar.
- **Autenticación JWT + middleware**: rutas públicas y protegidas separadas, con validación de Bearer token.

---

## Funciones

- **Autenticación**: `POST /api/login` para obtener un JWT; `POST /api/logout`; las API protegidas pasan por el middleware JWT.
- **Materiales**: listar / detalle / crear / actualizar / eliminar (paginado).
- **Categorías**: CRUD de categorías de materiales.
- **Almacenes**: gestión de múltiples almacenes.
- **Proveedores**: gestión de información de proveedores.
- **Clientes**: gestión de información de clientes.
- **Órdenes de entrada**: crear (con líneas), listar, detalle, auditar (aprobar/rechazar), eliminar, consultar líneas.
- **Órdenes de salida**: crear (con líneas), listar, detalle, auditar (aprobar/rechazar), eliminar, consultar líneas.
- **Consulta de inventario**: filtros por múltiples condiciones + paginación.
- **Verificación de estado**: `GET /health` devuelve `{"status":"UP"}` para sondas de contenedor/K8s.

## Tecnologías y arquitectura

| Capa | Tecnología / Notas |
|---|---|
| Framework web | `axum 0.7` (rutas, extractores, middleware) |
| Runtime asíncrono | `tokio 1` (full) |
| ORM / base de datos | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, totalmente local) |
| Serialización | `serde` / `serde_json` |
| Autenticación | `jsonwebtoken` (JWT) + `auth_middleware` |
| Configuración | `application.yml` + `dotenvy` |
| Registro | `tracing` / `tracing-subscriber` / `tracing-appender` (logs rotativos) |
| Frontend | HTML5 + CSS3 + JavaScript nativos (`static/`, servido por `ServeDir`) |
| Instalador | Inno Setup (Windows, opcional) |

**Arquitectura DDD de 4 capas**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (orquestación de casos de uso Service)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (implementación SeaORM Repo / configuración / base de datos)
```

## Páginas del frontend

Páginas en `static/`: `login.html` (inicio de sesión), `index.html` (inicio), `material.html` (materiales), `category.html` (categorías), `warehouse.html` (almacenes), `supplier.html` (proveedores), `customer.html` (clientes), `inorder.html` (entradas), `outorder.html` (salidas), `stock.html` (inventario).

## Plataformas compatibles

| Plataforma | Estado | Notas |
|---|---|---|
| Windows x64 | ✅ | Recomendado; también incluye script de instalador Inno Setup |
| macOS (Intel / Apple Silicon) | ✅ compilación CI | Binario universal generado por `build.yml` |
| Linux x64 | ✅ compilación CI | Directorio de datos por defecto `/wms` (volumen montable) |

## Compilación

Requiere Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Requiere Visual Studio Build Tools (C++ y Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Artefactos precompilados / multiplataforma

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **Matriz CI**: `.github/workflows/ci.yml` compila en tres plataformas; `build.yml` empaqueta los artefactos y los adjunta a un Release al etiquetar.
- **Imagen de contenedor (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# abre http://localhost:8080/login.html
```

## Uso

```bash
rwms                 # inicia el servidor (por defecto 0.0.0.0:8080)
```

- URL: <http://localhost:8080/login.html>
- Cuenta por defecto: `admin` / `admin` (**cámbiala en producción**)
- Verificación de estado: <http://localhost:8080/health>

Al iniciar crea `wms.db` en el directorio de datos y genera las tablas automáticamente (esquema y migraciones en el primer arranque).

## Configuración

La configuración proviene de `application.yml` (cambia con `APP_ENV`). Variables de entorno habituales:

| Variable | Descripción |
|---|---|
| `PORT` | Puerto de escucha (por defecto `server.port` en `application.yml`, es decir, 8080) |
| `JWT_SECRET` | Secreto JWT (sobrescribe `jwt.secret`; **obligatorio en producción**) |
| `APP_ENV` | Entorno (dev / test / prod) |
| `RUST_LOG` | Filtro de nivel de log (p. ej. `info`, `debug`) |

## Base de datos y directorio de datos

La ubicación del archivo SQLite `wms.db` depende de la plataforma:

| Plataforma | Ruta |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker: monta un volumen en `/wms`) |

Los logs se guardan en el mismo directorio que la base de datos (`wms.log`, rotación diaria).

## Resumen de la API

**Rutas públicas (sin autenticación)**: `GET /`, `GET /health`, `POST /api/login`.

**Rutas protegidas (requieren `Authorization: Bearer <token>`)**:

| Recurso | Endpoints |
|---|---|
| Cierre de sesión | `POST /api/logout` |
| Materiales | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Categorías | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Almacenes | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Proveedores | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Clientes | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Entradas | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Salidas | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Inventario | `GET /api/stocks` |

## Estructura del proyecto

```
src/
  main.rs                  entrada: carga config, inicializa logs/DB, monta rutas, inicia servidor
  state.rs                 estado compartido de la app (instancias de servicios)
  interfaces/              capa de interfaces: routes / handlers / DTO
  application/             capa de aplicación: servicios (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  capa de dominio: aggregate / repository trait / domain_service
  infrastructure/          infraestructura: implementación SeaORM model & repository / config / esquema / JWT
static/                    páginas del frontend (HTML/CSS/JS)
application.yml            configuración de la app
installer.iss / setup.iss  scripts de instalador Inno Setup (Windows)
```

## Verificación

- `GET /health` devuelve `{"status":"UP","service":"WMS"}`.
- `GET /` devuelve un mensaje de inicio correcto.
- La CI compila en las tres plataformas (Windows / macOS / Ubuntu).

## Preguntas frecuentes (FAQ)

**P: ¿Necesito instalar una base de datos?**
No. SQLite está integrado; `wms.db` se crea y las tablas se construyen al primer arranque.

**P: ¿Cuál es la cuenta por defecto?**
`admin` / `admin`. Cambia la contraseña y `JWT_SECRET` antes del uso real.

**P: El puerto está ocupado, ¿qué hago?**
Inicia con `PORT=9090` (o cambia `server.port` en `application.yml`).

**P: ¿Dónde está el directorio de datos?**
Ver «Base de datos y directorio de datos»; Windows es `C:\ProgramData\wms`, Linux/macOS es `/wms`.

## Comentarios (Issues)

¿Encontraste un problema o tienes una sugerencia? Usa GitHub Issues: <https://github.com/shuying-code/rwms/issues/new/choose>. Lee la **[guía de comentarios](docs/FEEDBACK.md)** antes de enviar.

- **🐞 Reporte de error**: errores reproducibles, fallos, crashes
- **💡 Solicitud de función**: nuevas funciones o mejoras
- **❓ Consulta / comentarios**: uso, configuración, opiniones generales

> Los mantenedores suelen atender los issues **una vez por semana**. Cuanto más específico sea el informe (versión, SO, pasos, errores, ejemplo mínimo), más rápida será la corrección.

## Resumen de cambios

- Primera versión pública: API backend WMS completa + frontend web integrado + autenticación JWT.
- Compilaciones multiplataforma (Windows / macOS / Linux) + imagen de contenedor (GHCR).
- Ver `CHANGELOG.md` y `DEPLOYMENT.md` para más detalles.

## Documentación

| Archivo | Contenido |
|---|---|
| `DEPLOYMENT.md` | Despliegue y empaquetado |
| `docs/FEEDBACK.md` | Guía de comentarios |
| `docs/README.md` | Índice de documentación |

## Licencia

[MIT](LICENSE)
