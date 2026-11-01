<p align="center">🌐 <b>언어</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms（Rust 창고 관리 시스템）

**Rust + Axum + SeaORM** 으로 작성된 **로컬 우선, 단일 바이너리, 외부 데이터베이스 불필요** 창고 관리 시스템(WMS)입니다.
**SQLite** 를 내장하고 완전한 웹 프런트엔드(HTML/CSS/JS)와 JWT 인증을 제공합니다. **바로 사용 가능하며 단일 바이너리로 배포**되고 Windows / macOS / Linux 를 지원합니다.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> 로컬 우선: 데이터는 로컬 SQLite 파일에 저장되며, MySQL/PostgreSQL 설치나 클라우드 계정이 필요 없습니다. 그냥 실행하면 됩니다.

---

## rwms 를 선택하는 이유

- **단일 바이너리 + SQLite 내장**: 외부 DB도 미들웨어도 없이 `cargo build --release` 로 만든 exe 하나로 실행됩니다.
- **완전한 Web UI**: 로그인, 대시보드, 자재 / 분류 / 창고 / 공급업체 / 고객 / 입고 / 출고 / 재고 화면을 바로 사용할 수 있습니다.
- **DDD 4계층 아키텍처**: interfaces → application → domain ← infrastructure, 경계가 명확해 유지보수와 확장이 쉽습니다.
- **JWT 인증 + 미들웨어**: 공개 라우트와 보호 라우트를 분리하고 Bearer 토큰을 검증합니다.

---

## 기능

- **인증**: `POST /api/login` 으로 JWT 발급, `POST /api/logout` 로그아웃, 보호 API는 JWT 미들웨어로 검증.
- **자재 관리**: 목록 / 상세 / 추가 / 수정 / 삭제(페이징).
- **분류 관리**: 자재 분류 CRUD.
- **창고 관리**: 다중 창고 정보 관리.
- **공급업체 관리**: 공급업체 정보 관리.
- **고객 관리**: 고객 정보 관리.
- **입고 전표**: 생성(상세 포함), 목록, 상세, 심사(승인/거부), 삭제, 상세 조회.
- **출고 전표**: 생성(상세 포함), 목록, 상세, 심사(승인/거부), 삭제, 상세 조회.
- **재고 조회**: 다중 조건 필터 + 페이징.
- **헬스 체크**: `GET /health` 가 `{"status":"UP"}` 를 반환하여 컨테이너/K8s 생존 확인에 사용.

## 기술 스택 및 아키텍처

| 계층 | 기술 / 설명 |
|---|---|
| 웹 프레임워크 | `axum 0.7` (라우팅, 추출기, 미들웨어) |
| 비동기 런타임 | `tokio 1` (full) |
| ORM / 데이터베이스 | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, 완전 로컬) |
| 직렬화 | `serde` / `serde_json` |
| 인증 | `jsonwebtoken` (JWT) + `auth_middleware` |
| 설정 | `application.yml` + `dotenvy` |
| 로깅 | `tracing` / `tracing-subscriber` / `tracing-appender` (롤링 파일 로그) |
| 프런트엔드 | 순수 HTML5 + CSS3 + JavaScript (`static/`, `ServeDir` 로 제공) |
| 설치 프로그램 | Inno Setup (Windows, 선택) |

**DDD 4계층 아키텍처**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (Service 유스케이스 오케스트레이션)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (SeaORM Repo 구현 / 설정 / 데이터베이스)
```

## 프런트엔드 페이지

`static/` 내 페이지: `login.html`(로그인), `index.html`(홈), `material.html`(자재), `category.html`(분류), `warehouse.html`(창고), `supplier.html`(공급업체), `customer.html`(고객), `inorder.html`(입고), `outorder.html`(출고), `stock.html`(재고).

## 지원 플랫폼

| 플랫폼 | 상태 | 설명 |
|---|---|---|
| Windows x64 | ✅ | 권장, Inno Setup 설치 스크립트도 제공 |
| macOS(Intel / Apple Silicon) | ✅ CI 빌드 | `build.yml` 로 유니버설 바이너리 생성 |
| Linux x64 | ✅ CI 빌드 | 기본 데이터 디렉터리 `/wms`(볼륨 마운트 가능) |

## 빌드

Rust **1.85+** 필요(`edition = "2024"`).

```bash
cargo build --release
```

### Windows(MSVC)

```powershell
# Visual Studio Build Tools(C++ 및 Windows SDK) 필요
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## 사전 빌드 / 크로스 플랫폼 산출물

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **CI 매트릭스**: `.github/workflows/ci.yml` 이 3개 플랫폼에서 빌드, `build.yml` 이 산출물을 패키징하여 태그 시 Release에 첨부.
- **컨테이너 이미지(GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# 브라우저에서 http://localhost:8080/login.html 열기
```

## 사용법

```bash
rwms                 # 서버 시작(기본 0.0.0.0:8080)
```

- 접속 URL: <http://localhost:8080/login.html>
- 기본 계정: `admin` / `admin` (**운영 환경에서는 반드시 변경**)
- 헬스 체크: <http://localhost:8080/health>

시작 시 데이터 디렉터리에 `wms.db` 를 생성하고 테이블을 자동 생성합니다(최초 시작 시 스키마 생성 및 마이그레이션 수행).

## 설정

설정은 `application.yml` 에서 읽습니다(`APP_ENV` 로 전환). 주요 환경 변수:

| 변수 | 설명 |
|---|---|
| `PORT` | 수신 포트(기본값은 `application.yml` 의 `server.port`, 즉 8080) |
| `JWT_SECRET` | JWT 시크릿(`jwt.secret` 덮어씀, **운영 시 필수**) |
| `APP_ENV` | 실행 환경(dev / test / prod) |
| `RUST_LOG` | 로그 레벨 필터(예: `info`, `debug`) |

## 데이터베이스 및 데이터 디렉터리

SQLite 파일 `wms.db` 위치는 플랫폼에 따라 결정됩니다:

| 플랫폼 | 경로 |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker는 `/wms` 에 볼륨 마운트 권장) |

로그 파일은 데이터베이스와 같은 디렉터리에 기록됩니다(`wms.log`, 일 단위 롤링).

## API 개요

**공개 라우트(인증 불필요)**: `GET /`, `GET /health`, `POST /api/login`.

**보호 라우트(`Authorization: Bearer <token>` 필요)**:

| 리소스 | 엔드포인트 |
|---|---|
| 로그아웃 | `POST /api/logout` |
| 자재 | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| 분류 | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| 창고 | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| 공급업체 | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| 고객 | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| 입고 | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| 출고 | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| 재고 | `GET /api/stocks` |

## 프로젝트 구조

```
src/
  main.rs                  진입점: 설정 로드, 로그/DB 초기화, 라우트 구성, 서버 시작
  state.rs                 공유 앱 상태(서비스 인스턴스)
  interfaces/              인터페이스 계층: routes / handlers / DTO
  application/             애플리케이션 계층: 각 Service(인증/자재/분류/창고/공급업체/고객/입고/출고/재고)
  domain/                  도메인 계층: aggregate / repository trait / domain_service
  infrastructure/          인프라 계층: SeaORM model & repository 구현 / 설정 / 스키마 / JWT
static/                    프런트엔드 페이지(HTML/CSS/JS)
application.yml            앱 설정
installer.iss / setup.iss  Inno Setup 설치 스크립트(Windows)
```

## 검증

- `GET /health` 가 `{"status":"UP","service":"WMS"}` 를 반환.
- `GET /` 가 시작 성공 메시지를 반환.
- CI 가 3개 플랫폼(Windows / macOS / Ubuntu)에서 빌드 통과.

## 자주 묻는 질문(FAQ)

**Q: 데이터베이스를 설치해야 하나요?**
아니요. SQLite 가 내장되어 있으며 최초 시작 시 `wms.db` 를 생성하고 테이블을 구축합니다.

**Q: 기본 계정은 무엇인가요?**
`admin` / `admin`. 실제 사용 전에 비밀번호와 `JWT_SECRET` 을 변경하세요.

**Q: 포트가 사용 중이면?**
`PORT=9090` 으로 실행하거나 `application.yml` 의 `server.port` 를 변경하세요.

**Q: 데이터 디렉터리는 어디인가요?**
"데이터베이스 및 데이터 디렉터리" 참고. Windows는 `C:\ProgramData\wms`, Linux/macOS는 `/wms`.

## 피드백(Issues)

문제나 제안이 있으면 GitHub Issues를 이용해 주세요: <https://github.com/shuying-code/rwms/issues/new/choose>. 제출 전 **[피드백 가이드](docs/FEEDBACK.md)** 를 읽어 주세요.

- **🐞 버그 리포트**: 재현 가능한 오류, 크래시, 실패
- **💡 기능 제안**: 신규 기능 또는 개선
- **❓ 질문 / 피드백**: 사용법, 설정, 일반 의견

> 관리자는 보통 **주 1회** issue를 일괄 처리합니다. 보고가 구체적일수록(버전, OS, 재현 단계, 오류, 최소 예제) 더 빠르게 수정됩니다.

## 업데이트 요약

- 최초 공개 버전: WMS 백엔드 API 전체 + 내장 웹 프런트엔드 + JWT 인증.
- 크로스 플랫폼 빌드(Windows / macOS / Linux) + 컨테이너 이미지(GHCR).
- 자세한 내용은 `CHANGELOG.md` 와 `DEPLOYMENT.md` 참고.

## 문서

| 파일 | 내용 |
|---|---|
| `DEPLOYMENT.md` | 배포 및 패키징 |
| `docs/FEEDBACK.md` | 피드백 가이드 |
| `docs/README.md` | 문서 색인 |

## 라이선스

[MIT](LICENSE)
