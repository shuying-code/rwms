// ============================================
// Infrastructure 配置模块
// 包含应用配置、数据库连接、JWT 工具、认证中间件
// ============================================

pub mod app_config;        // 应用配置模块：从 application.yml 加载配置，支持环境变量覆盖
pub mod database;          // 数据库连接模块：SQLite 连接池初始化和跨平台路径处理
pub mod database_init;     // 数据库初始化模块：建表、列迁移、Schema 版本管理
pub mod jwt_utils;         // JWT 工具模块：Token 生成与验证
pub mod auth_middleware;    // 认证中间件模块：拦截请求验证 JWT Token
