# 更新日志 (Changelog)

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.1.0] - 2026-11-01

### 新增
- 首个公开版本：Rust + Axum + SeaORM 的仓库管理系统（WMS）。
- 业务模块：认证（JWT）、物资、分类、仓库、供应商、客户、入库单、出库单、库存查询。
- 内置 Web 前端（`static/`）与静态资源托管；`GET /health` 健康检查。
- DDD 四层架构（interfaces / application / domain / infrastructure）。
- 内嵌 SQLite，首次启动自动建表与初始化。
- 跨平台构建（Windows / macOS / Linux）与 GHCR 容器镜像。
- Inno Setup（Windows）安装包脚本 `installer.iss` / `setup.iss`。

[0.1.0]: https://github.com/shuying-code/rwms/releases/tag/v0.1.0
