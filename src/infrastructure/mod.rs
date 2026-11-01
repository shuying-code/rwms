// ============================================
// Infrastructure 基础设施层
// 包含：持久化模型、仓储实现、基础设施配置
// ============================================

pub mod model;          // 持久化模型模块：SeaORM 实体定义，对应数据库表结构
pub mod repository;     // 仓储实现模块：Domain 层 Repository trait 的具体实现
pub mod config;         // 配置模块：应用配置、数据库连接、JWT 工具、认证中间件
