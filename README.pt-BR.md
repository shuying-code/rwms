<p align="center">🌐 <b>Idioma</b>：<a href="README.md">简体中文</a> · <a href="README.en.md">English</a> · <a href="README.zh-TW.md">繁體中文</a> · <a href="README.ja.md">日本語</a> · <a href="README.ko.md">한국어</a> · <a href="README.ru.md">Русский</a> · <a href="README.es.md">Español</a> · <a href="README.fr.md">Français</a> · <a href="README.de.md">Deutsch</a> · <a href="README.pt-BR.md">Português</a></p>

# rwms (Sistema de Gestão de Armazém em Rust)

Um **sistema de gestão de armazém (WMS) local, de binário único e sem banco de dados externo**, escrito em **Rust + Axum + SeaORM**.
Inclui **SQLite**, um frontend web completo (HTML/CSS/JS) e autenticação JWT — **pronto para usar e implantar como um único binário** — no Windows / macOS / Linux.

<p>
  <img alt="CI" src="https://github.com/shuying-code/rwms/actions/workflows/ci.yml/badge.svg">
  <img alt="build" src="https://github.com/shuying-code/rwms/actions/workflows/build.yml/badge.svg">
  <img alt="Docker" src="https://github.com/shuying-code/rwms/actions/workflows/docker.yml/badge.svg">
  <img alt="Rust" src="https://img.shields.io/badge/rust-1.85%2B-orange">
  <img alt="Platform" src="https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-2f6fed">
  <img alt="License" src="https://img.shields.io/badge/license-MIT-green">
  <img alt="Release" src="https://img.shields.io/github/v/release/shuying-code/rwms?display_name=tag">
</p>

> Local primeiro: os dados ficam em um arquivo SQLite local — sem instalar MySQL/PostgreSQL, sem conta na nuvem; basta iniciar.

---

## Por que rwms

- **Binário único + SQLite embutido**: sem banco externo nem middleware — um único exe gerado por `cargo build --release` é suficiente.
- **UI web completa**: login, painel e páginas de materiais / categorias / armazéns / fornecedores / clientes / entradas / saídas / estoque prontas para uso.
- **Arquitetura DDD em 4 camadas**: interfaces → aplicação → domínio ← infraestrutura; limites claros, fácil de manter e expandir.
- **Autenticação JWT + middleware**: rotas públicas e protegidas separadas, validação de Bearer token.

---

## Recursos

- **Autenticação**: `POST /api/login` para obter um JWT; `POST /api/logout`; APIs protegidas passam pelo middleware JWT.
- **Materiais**: listar / detalhe / criar / atualizar / excluir (paginado).
- **Categorias**: CRUD de categorias de materiais.
- **Armazéns**: gestão de vários armazéns.
- **Fornecedores**: gestão de informações de fornecedores.
- **Clientes**: gestão de informações de clientes.
- **Pedidos de entrada**: criar (com itens), listar, detalhe, auditar (aprovar/rejeitar), excluir, consultar itens.
- **Pedidos de saída**: criar (com itens), listar, detalhe, auditar (aprovar/rejeitar), excluir, consultar itens.
- **Consulta de estoque**: filtro por múltiplas condições + paginação.
- **Verificação de integridade**: `GET /health` retorna `{"status":"UP"}` para sondas de contêiner/K8s.

## Tecnologias e arquitetura

| Camada | Tecnologia / Observação |
|---|---|
| Framework web | `axum 0.7` (rotas, extractors, middleware) |
| Runtime assíncrono | `tokio 1` (full) |
| ORM / banco de dados | `sea-orm 0.12` + SQLite (`sqlx-sqlite`, totalmente local) |
| Serialização | `serde` / `serde_json` |
| Autenticação | `jsonwebtoken` (JWT) + `auth_middleware` |
| Configuração | `application.yml` + `dotenvy` |
| Logs | `tracing` / `tracing-subscriber` / `tracing-appender` (logs rotativos) |
| Frontend | HTML5 + CSS3 + JavaScript puros (`static/`, servido por `ServeDir`) |
| Instalador | Inno Setup (Windows, opcional) |

**Arquitetura DDD em 4 camadas**

```
interfaces (HTTP/Router/DTO/Handlers)
      │
      ▼
application (orquestração de casos de uso Service)
      │
      ▼
domain (Aggregate / Repository trait / Domain Service)
      ▲
      │
infrastructure (implementação SeaORM Repo / configuração / banco de dados)
```

## Páginas do frontend

Páginas em `static/`: `login.html` (login), `index.html` (início), `material.html` (materiais), `category.html` (categorias), `warehouse.html` (armazéns), `supplier.html` (fornecedores), `customer.html` (clientes), `inorder.html` (entradas), `outorder.html` (saídas), `stock.html` (estoque).

## Plataformas suportadas

| Plataforma | Status | Observação |
|---|---|---|
| Windows x64 | ✅ | Recomendado; inclui script de instalador Inno Setup |
| macOS (Intel / Apple Silicon) | ✅ build CI | Binário universal gerado por `build.yml` |
| Linux x64 | ✅ build CI | Diretório de dados padrão `/wms` (volume montável) |

## Compilação

Requer Rust **1.85+** (`edition = "2024"`).

```bash
cargo build --release
```

### Windows (MSVC)

```powershell
# Requer Visual Studio Build Tools (C++ e Windows SDK)
cargo build --release
```

### macOS / Linux

```bash
cargo build --release
```

## Artefatos pré-compilados / multiplataforma

- **Releases**: <https://github.com/shuying-code/rwms/releases> (Windows / macOS universal / Linux x64).
- **Matriz CI**: `.github/workflows/ci.yml` compila em três plataformas; `build.yml` empacota os artefatos e os anexa a um Release ao criar a tag.
- **Imagem de contêiner (GHCR)**:

```bash
docker pull ghcr.io/shuying-code/rwms:latest
docker run --rm -p 8080:8080 -v rwms-data:/wms ghcr.io/shuying-code/rwms:latest
# abra http://localhost:8080/login.html
```

## Uso

```bash
rwms                 # inicia o servidor (padrão 0.0.0.0:8080)
```

- URL: <http://localhost:8080/login.html>
- Conta padrão: `admin` / `admin` (**altere em produção**)
- Verificação de integridade: <http://localhost:8080/health>

Ao iniciar, cria `wms.db` no diretório de dados e gera as tabelas automaticamente (esquema e migrações na primeira execução).

## Configuração

A configuração vem de `application.yml` (alternável via `APP_ENV`). Variáveis de ambiente comuns:

| Variável | Descrição |
|---|---|
| `PORT` | Porta de escuta (padrão `server.port` em `application.yml`, ou seja, 8080) |
| `JWT_SECRET` | Segredo JWT (sobrescreve `jwt.secret`; **obrigatório em produção**) |
| `APP_ENV` | Ambiente (dev / test / prod) |
| `RUST_LOG` | Filtro de nível de log (ex.: `info`, `debug`) |

## Banco de dados e diretório de dados

A localização do arquivo SQLite `wms.db` depende da plataforma:

| Plataforma | Caminho |
|---|---|
| Windows | `C:\ProgramData\wms\wms.db` |
| Linux / macOS | `/wms/wms.db` (Docker: monte um volume em `/wms`) |

Os logs ficam no mesmo diretório do banco de dados (`wms.log`, rotação diária).

## Visão geral da API

**Rotas públicas (sem autenticação)**: `GET /`, `GET /health`, `POST /api/login`.

**Rotas protegidas (exigem `Authorization: Bearer <token>`)**:

| Recurso | Endpoints |
|---|---|
| Logout | `POST /api/logout` |
| Materiais | `GET/POST /api/materials`, `GET/DELETE /api/materials/:id`, `POST /api/materials/update` |
| Categorias | `GET/POST /api/categories`, `GET/DELETE /api/categories/:id`, `POST /api/categories/update` |
| Armazéns | `GET/POST /api/warehouses`, `GET/DELETE /api/warehouses/:id`, `POST /api/warehouses/update` |
| Fornecedores | `GET/POST /api/suppliers`, `GET/DELETE /api/suppliers/:id`, `POST /api/suppliers/update` |
| Clientes | `GET/POST /api/customers`, `GET/DELETE /api/customers/:id`, `POST /api/customers/update` |
| Entradas | `GET/POST /api/in-orders`, `GET/DELETE /api/in-orders/:id`, `POST /api/in-orders/audit`, `GET /api/in-orders/:id/items` |
| Saídas | `GET/POST /api/out-orders`, `GET/DELETE /api/out-orders/:id`, `POST /api/out-orders/audit`, `GET /api/out-orders/:id/items` |
| Estoque | `GET /api/stocks` |

## Estrutura do projeto

```
src/
  main.rs                  entrada: carrega config, inicializa logs/DB, monta rotas, inicia servidor
  state.rs                 estado compartilhado do app (instâncias de serviços)
  interfaces/              camada de interfaces: routes / handlers / DTO
  application/             camada de aplicação: serviços (auth/material/category/warehouse/supplier/customer/inbound/outbound/stock)
  domain/                  camada de domínio: aggregate / repository trait / domain_service
  infrastructure/          infraestrutura: implementação SeaORM model & repository / config / esquema / JWT
static/                    páginas do frontend (HTML/CSS/JS)
application.yml            configuração do app
installer.iss / setup.iss  scripts de instalador Inno Setup (Windows)
```

## Verificação

- `GET /health` retorna `{"status":"UP","service":"WMS"}`.
- `GET /` retorna uma mensagem de início bem-sucedido.
- A CI compila nas três plataformas (Windows / macOS / Ubuntu).

## Perguntas frequentes (FAQ)

**P: Preciso instalar um banco de dados?**
Não. O SQLite é embutido; `wms.db` é criado e as tabelas são montadas na primeira execução.

**P: Qual é a conta padrão?**
`admin` / `admin`. Altere a senha e o `JWT_SECRET` antes do uso real.

**P: A porta está em uso, e agora?**
Inicie com `PORT=9090` (ou altere `server.port` em `application.yml`).

**P: Onde fica o diretório de dados?**
Veja «Banco de dados e diretório de dados»; Windows é `C:\ProgramData\wms`, Linux/macOS é `/wms`.

## Feedback (Issues)

Encontrou um problema ou tem uma sugestão? Use o GitHub Issues: <https://github.com/shuying-code/rwms/issues/new/choose>. Leia o **[guia de feedback](docs/FEEDBACK.md)** antes de enviar.

- **🐞 Relato de bug**: erros reproduzíveis, travamentos, falhas
- **💡 Sugestão de recurso**: novos recursos ou melhorias
- **❓ Dúvida / feedback**: uso, configuração, opiniões gerais

> Os mantenedores costumam tratar as issues **uma vez por semana**. Quanto mais específico o relato (versão, SO, passos, erros, exemplo mínimo), mais rápida a correção.

## Resumo de atualizações

- Primeira versão pública: API backend WMS completa + frontend web embutido + autenticação JWT.
- Builds multiplataforma (Windows / macOS / Linux) + imagem de contêiner (GHCR).
- Veja `CHANGELOG.md` e `DEPLOYMENT.md` para detalhes.

## Documentação

| Arquivo | Conteúdo |
|---|---|
| `DEPLOYMENT.md` | Implantação e empacotamento |
| `docs/FEEDBACK.md` | Guia de feedback |
| `docs/README.md` | Índice de documentação |

## Licença

[MIT](LICENSE)
