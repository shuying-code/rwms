<p align="center">🌐 <b>Язык</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Rust — система управления складом)

**Локальная, однобинарная система управления складом (WMS) без внешней базы данных**, написанная на **Rust + Axum + SeaORM**.
Встроенный **SQLite**, полноценный веб-интерфейс (HTML/CSS/JS) и аутентификация JWT — **готово к работе, развёртывается одним бинарником**; работает на Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Локальный подход: данные хранятся в локальном файле SQLite — не нужно устанавливать MySQL/PostgreSQL и не нужен облачный аккаунт; просто запустите.

---

## Почему rwms

- **Один бинарник + встроенный SQLite**: без внешней БД и промежуточного ПО — достаточно одного exe из `cargo build --release`.
- **Полноценный веб-интерфейс**: вход, дашборд, страницы материалов / категорий / складов / поставщиков / клиентов / прихода / расхода / остатков — «из коробки».
- **4-слойная архитектура DDD**: interfaces → application → domain ← infrastructure; понятные границы, легко поддерживать и расширять.
- **Аутентификация JWT + middleware**: публичные и защищённые маршруты разделены, проверка Bearer-токена.

---

## Возможности

- **Аутентификация**: `POST /api/login` для получения JWT; `POST /api/logout`; защищённые API проверяются middleware JWT.
- **Материалы**: список / детали / создание / изменение / удаление (с пагинацией).
- **Категории**: CRUD категорий материалов.
- **Склады**: управление несколькими складами.
- **Поставщики**: управление данными поставщиков.
- **Клиенты**: управление данными клиентов.
- **Приходные ордера**: создание (с позициями), список, детали, аудит (принять/отклонить), удаление, просмотр позиций.
- **Расходные ордера**: создание (с позициями), список, детали, аудит (принять/отклонить), удаление, просмотр позиций.
- **Остатки**: фильтрация по нескольким условиям + пагинация.
- **Проверка работоспособности**: `GET /health` возвращает `{"status":"UP"}` для проб контейнера/K8s.

## Технологии и архитектура

| Слой | Технология / описание |
|---|---|
| Веб-фреймворк | `axum 0.7` (маршрутизация, extractor'ы, middleware) |
| Асинхронный рантайм | `tokio 1` (full) |
| ORM / база данных | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, полностью локально) |
| Сериализация | `serde` / `serde_json` |
| Аутентификация | `jsonwebtoken` (JWT) + `auth_middleware` |
| Конфигурация | `application.yml` + `dotenvy` |
| Логирование | `tracing` / `tracing-subscriber` / `tracing-appender` (ротация логов) |
| Фронтенд | Чистый HTML5 + CSS3 + JavaScript (`static/`, раздаётся через `ServeDir`) |
| Установщик | Inno Setup (Windows, опционально) |

**4-слойная архитектура DDD**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (оркестрация сценариев Service)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (реализация SeaORM Repo / конфигурация / база данных)
```

## Страницы фронтенда

Страницы в `static/`: `login.html` (вход), `index.html` (главная), `material.html` (материалы), `category.html` (категории), `warehouse.html` (склады), `supplier.html` (поставщики), `customer.html` (клиенты), `inorder.html` (приход), `outorder.html` (расход), `stock.html` (остатки).

## Поддерживаемые платформы

| Платформа | Статус | Примечание |
|---|---|---|
| Windows x64 | ✅ | Рекомендуется; также есть скрипт установщика Inno Setup |
| macOS (Intel / Apple Silicon) | ✅ сборка CI | Универсальный бинарник собирается `build.yml` |
| Linux x64 | ✅ сборка CI | Каталог данных по умолчанию `/wms` (можно смонтировать том) |

## Сборка

Требуется Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Требуются Visual Studio Build Tools (C++ и Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Готовые / кроссплатформенные артефакты

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **Матрица CI**: `.github/workflows/ci.yml` собирает на трёх платформах; `build.yml` упаковывает артефакты и прикрепляет их к Release по тегу.
- **Образ контейнера (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# откройте http://localhost:8080/login.html
```

## Использование

```bash
rwms                 # запуск сервера (по умолчанию 0.0.0.0:8080)
```

- Адрес: <http://localhost:8080/login.html>
- Учётная запись по умолчанию: `admin` / `admin` (**обязательно смените в продакшене**)
- Проверка работоспособности: <http://localhost:8080/health>

При запуске создаётся `wms.db` в каталоге данных и автоматически создаются таблицы (схема и миграции выполняются при первом запуске).

## Конфигурация

Конфигурация берётся из `application.yml` (переключение через `APP_ENV`). Основные переменные окружения:

| Переменная | Описание |
|---|---|
| `PORT` | Порт прослушивания (по умолчанию `server.port` из `application.yml`, т.е. 8080) |
| `JWT_SECRET` | Секрет JWT (переопределяет `jwt.secret`; **обязателен в продакшене**) |
| `APP_ENV` | Имя окружения (dev / test / prod) |
| `RUST_LOG` | Фильтр уровня логов (например, `info`, `debug`) |

## База данных и каталог данных

Расположение файла SQLite `wms.db` зависит от платформы:

| Платформа | Путь |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker: смонтируйте том в `/wms`) |

Файлы логов находятся в том же каталоге, что и база данных (`wms.log`, ротация по дням).

## Обзор API

**Публичные маршруты (без аутентификации)**: `GET /`, `GET /health`, `POST /api/login`.

**Защищённые маршруты (требуется `Authorization: Bearer <token>`)**:

| Ресурс | Эндпоинты |
|---|---|
| Выход | `POST /api/logout` |
| Материалы | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Категории | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Склады | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Поставщики | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Клиенты | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Приход | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Расход | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Остатки | `GET /api/stocks` |

## Структура проекта

```
src/
  main.rs                  точка входа: загрузка конфигурации, инициализация логов/БД, маршруты, запуск сервера
  state.rs                 общее состояние приложения (экземпляры сервисов)
  interfaces/              слой интерфейсов: routes / handlers / DTO
  application/             слой приложения: сервисы (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  доменный слой: aggregate / repository trait / domain_service
  infrastructure/          инфраструктура: реализация SeaORM model & repository / конфигурация / схема / JWT
static/                    страницы фронтенда (HTML/CSS/JS)
application.yml            конфигурация приложения
installer.iss / setup.iss  скрипты установщика Inno Setup (Windows)
```

## Проверка

- `GET /health` возвращает `{"status":"UP","service":"WMS"}`.
- `GET /` возвращает сообщение об успешном запуске.
- CI-сборка проходит на трёх платформах (Windows / macOS / Ubuntu).

## Частые вопросы (FAQ)

**Вопрос: Нужно ли устанавливать базу данных?**
Нет. SQLite встроена; `wms.db` создаётся и таблицы строятся при первом запуске.

**Вопрос: Какая учётная запись по умолчанию?**
`admin` / `admin`. Перед реальным использованием смените пароль и `JWT_SECRET`.

**Вопрос: Порт занят — что делать?**
Запустите с `PORT=9090` (или измените `server.port` в `application.yml`).

**Вопрос: Где каталог данных?**
См. «База данных и каталог данных»; Windows — `C:\ProgramData\wms`, Linux/macOS — `/wms`.

## Обратная связь (Issues)

Нашли проблему или есть предложение? Используйте GitHub Issues: <https://github.com/shuying-code/rwms/issues/new/choose>. Перед отправкой прочитайте **[руководство по обратной связи](docs/FEEDBACK.md)**.

- **🐞 Баг-репорт**: воспроизводимые ошибки, падения, сбои
- **💡 Предложение 기능**: новые функции или улучшения
- **❓ Вопрос / отзыв**: использование, конфигурация, общие отзывы

> Сопровождающие обычно обрабатывают issues **раз в неделю**. Чем конкретнее отчёт (версия, ОС, шаги, ошибки, минимальный пример), тем быстрее исправление.

## Краткое описание обновлений

- Первый публичный релиз: полный backend API WMS + встроенный веб-фронтенд + аутентификация JWT.
- Кроссплатформенные сборки (Windows / macOS / Linux) + образ контейнера (GHCR).
- Подробности см. в `CHANGELOG.md` и `DEPLOYMENT.md`.

## Документация

| Файл | Содержание |
|---|---|
| `DEPLOYMENT.md` | Развёртывание и упаковка |
| `docs/FEEDBACK.md` | Руководство по обратной связи |
| `docs/README.md` | Индекс документации |

## Лицензия

[MIT](LICENSE)
