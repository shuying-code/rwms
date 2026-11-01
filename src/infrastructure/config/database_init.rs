// ============================================
// 数据库初始化与迁移模块
// 对应 Java 的 Flyway / Liquibase 数据库迁移工具
// 负责：创建表结构、检测缺失列并自动补充、记录迁移版本
// ============================================

// SeaORM 核心类型：ConnectionTrait 提供执行 SQL 的能力，DatabaseConnection 是连接池句柄
use sea_orm::{ConnectionTrait, DatabaseConnection, DbErr, FromQueryResult, Statement};
// HashSet 用于存储列名集合，快速判断列是否存在
use std::collections::HashSet;

/// PRAGMA table_info 查询结果结构体
///
/// 从 SQLite PRAGMA table_info 命令解析列名
#[derive(Debug, Clone, FromQueryResult)]   // FromQueryResult 允许从 SQL 查询结果自动映射
struct PragmaColumnInfo {
    name: String,                       // 列名，对应 PRAGMA table_info 输出的 name 字段
}

/// Schema 版本号查询结果结构体
#[derive(Debug, Clone, FromQueryResult)]   // FromQueryResult 允许从 SQL 查询结果自动映射
struct VersionRow {
    version: i64,                       // 迁移版本号，对应 schema_migrations 表的 version 列
}

/// 表名查询结果结构体（用于 sqlite_master 查询）
#[derive(Debug, Clone, FromQueryResult)]   // FromQueryResult 允许从 SQL 查询结果自动映射
#[allow(dead_code)]
struct TableNameRow {
    name: String,                       // 表名，对应 sqlite_master 系统表的 name 字段
}

/// 表定义结构
///
/// 描述一张数据库表的完整元信息，用于自动建表和列迁移
#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub name: &'static str,                       // 表名（如 "sys_user"、"warehouse"）
    pub create_sql: &'static str,                 // 建表 SQL（CREATE TABLE IF NOT EXISTS ...）
    pub expected_columns: Vec<ColumnDefinition>,  // 期望的列定义列表，用于检测缺失列并自动补充
}

/// 列定义结构
///
/// 描述表中一列的完整定义，用于 ALTER TABLE ADD COLUMN 迁移
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    pub name: &'static str,                       // 列名（如 "user_id"、"username"）
    pub column_type: &'static str,                // 列类型（如 "TEXT"、"INTEGER"、"REAL"）
    pub notnull: bool,                            // 是否非空（NOT NULL 约束）
    pub default_value: Option<&'static str>,      // 默认值（None 表示无默认值，Some("1") 表示 DEFAULT 1）
}

/// 当前数据库 Schema 版本号
///
/// 每次修改表结构时递增，用于判断是否需要执行迁移
const CURRENT_SCHEMA_VERSION: i64 = 1;            // 版本 1：初始表结构，包含全部 11 张表

/// 获取所有表的定义
///
/// 返回系统中全部 11 张表的定义，包括表名、建表 SQL 和期望的列列表
fn get_all_tables() -> Vec<TableDefinition> {
    vec![
        // ========== 1. 系统用户表 ==========
        // 存储所有用户账号信息，包括管理员和普通用户
        TableDefinition {
            name: "sys_user",                     // 表名：系统用户表
            create_sql: r#"CREATE TABLE IF NOT EXISTS sys_user (
                user_id TEXT NOT NULL PRIMARY KEY,       -- 用户ID（主键，如 "admin001"）
                username TEXT NOT NULL UNIQUE,            -- 用户名（唯一，用于登录）
                password TEXT NOT NULL,                   -- 密码（当前明文存储，生产环境应加密）
                real_name TEXT,                           -- 真实姓名
                role TEXT NOT NULL,                       -- 角色（如 "超级管理员"、"仓库管理员"）
                phone TEXT,                               -- 手机号
                token TEXT,                               -- JWT Token（用于单点登录控制）
                create_time TEXT,                         -- 创建时间
                status INTEGER DEFAULT 1                  -- 状态（1=启用，0=禁用）
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应，用于列迁移检测
                ColumnDefinition { name: "user_id", column_type: "TEXT", notnull: true, default_value: None },          // 用户ID，主键
                ColumnDefinition { name: "username", column_type: "TEXT", notnull: true, default_value: None },         // 用户名，唯一约束
                ColumnDefinition { name: "password", column_type: "TEXT", notnull: true, default_value: None },         // 密码
                ColumnDefinition { name: "real_name", column_type: "TEXT", notnull: false, default_value: None },       // 真实姓名，可为空
                ColumnDefinition { name: "role", column_type: "TEXT", notnull: true, default_value: None },             // 角色
                ColumnDefinition { name: "phone", column_type: "TEXT", notnull: false, default_value: None },           // 手机号，可为空
                ColumnDefinition { name: "token", column_type: "TEXT", notnull: false, default_value: None },           // JWT Token，可为空
                ColumnDefinition { name: "create_time", column_type: "TEXT", notnull: false, default_value: None },     // 创建时间，可为空
                ColumnDefinition { name: "status", column_type: "INTEGER", notnull: false, default_value: Some("1") },  // 状态，默认 1（启用）
            ],
        },
        // ========== 2. 仓库表 ==========
        // 存储仓库基本信息和容量
        TableDefinition {
            name: "warehouse",                      // 表名：仓库表
            create_sql: r#"CREATE TABLE IF NOT EXISTS warehouse (
                warehouse_id TEXT NOT NULL PRIMARY KEY,   -- 仓库ID（主键）
                warehouse_code TEXT NOT NULL UNIQUE,       -- 仓库编码（唯一）
                manager TEXT,                               -- 负责人
                max_space INTEGER NOT NULL,                 -- 最大容量
                remain_space INTEGER NOT NULL,              -- 剩余容量
                create_time TEXT                            -- 创建时间
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "warehouse_id", column_type: "TEXT", notnull: true, default_value: None },     // 仓库ID，主键
                ColumnDefinition { name: "warehouse_code", column_type: "TEXT", notnull: true, default_value: None },   // 仓库编码，唯一约束
                ColumnDefinition { name: "manager", column_type: "TEXT", notnull: false, default_value: None },         // 负责人，可为空
                ColumnDefinition { name: "max_space", column_type: "INTEGER", notnull: true, default_value: None },     // 最大容量
                ColumnDefinition { name: "remain_space", column_type: "INTEGER", notnull: true, default_value: None },  // 剩余容量
                ColumnDefinition { name: "create_time", column_type: "TEXT", notnull: false, default_value: None },     // 创建时间，可为空
            ],
        },
        // ========== 3. 物资分类表 ==========
        // 物资的分类字典（如 "电子产品"、"办公用品"）
        TableDefinition {
            name: "material_category",              // 表名：物资分类表
            create_sql: r#"CREATE TABLE IF NOT EXISTS material_category (
                category_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 分类ID（自增主键）
                category_name TEXT NOT NULL UNIQUE,    -- 分类名称（唯一）
                remark TEXT                             -- 备注
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "category_id", column_type: "INTEGER", notnull: true, default_value: None },   // 分类ID，自增主键
                ColumnDefinition { name: "category_name", column_type: "TEXT", notnull: true, default_value: None },    // 分类名称，唯一约束
                ColumnDefinition { name: "remark", column_type: "TEXT", notnull: false, default_value: None },          // 备注，可为空
            ],
        },
        // ========== 4. 物资表 ==========
        // 存储物资/商品基本信息和价格
        TableDefinition {
            name: "material",                        // 表名：物资表
            create_sql: r#"CREATE TABLE IF NOT EXISTS material (
                material_id TEXT NOT NULL PRIMARY KEY,   -- 物资ID（主键）
                material_name TEXT NOT NULL,              -- 物资名称
                category_id INTEGER NOT NULL,             -- 分类ID（外键关联 material_category）
                unit TEXT NOT NULL,                       -- 计量单位（如 "个"、"箱"）
                purchase_price REAL NOT NULL,             -- 采购价格
                sell_price REAL NOT NULL,                 -- 销售价格
                total_stock INTEGER DEFAULT 0,            -- 总库存数量
                create_time TEXT,                         -- 创建时间
                update_time TEXT                          -- 更新时间
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "material_id", column_type: "TEXT", notnull: true, default_value: None },      // 物资ID，主键
                ColumnDefinition { name: "material_name", column_type: "TEXT", notnull: true, default_value: None },    // 物资名称
                ColumnDefinition { name: "category_id", column_type: "INTEGER", notnull: true, default_value: None },   // 分类ID，外键
                ColumnDefinition { name: "unit", column_type: "TEXT", notnull: true, default_value: None },             // 计量单位
                ColumnDefinition { name: "purchase_price", column_type: "REAL", notnull: true, default_value: None },   // 采购价格
                ColumnDefinition { name: "sell_price", column_type: "REAL", notnull: true, default_value: None },       // 销售价格
                ColumnDefinition { name: "total_stock", column_type: "INTEGER", notnull: false, default_value: Some("0") }, // 总库存，默认 0
                ColumnDefinition { name: "create_time", column_type: "TEXT", notnull: false, default_value: None },     // 创建时间，可为空
                ColumnDefinition { name: "update_time", column_type: "TEXT", notnull: false, default_value: None },     // 更新时间，可为空
            ],
        },
        // ========== 5. 供应商表 ==========
        // 存储供应商联系信息
        TableDefinition {
            name: "supplier",                        // 表名：供应商表
            create_sql: r#"CREATE TABLE IF NOT EXISTS supplier (
                supplier_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 供应商ID（自增主键）
                supplier_name TEXT NOT NULL UNIQUE,   -- 供应商名称（唯一）
                contact TEXT,                           -- 联系人
                phone TEXT,                             -- 联系电话
                remark TEXT                             -- 备注
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "supplier_id", column_type: "INTEGER", notnull: true, default_value: None },   // 供应商ID，自增主键
                ColumnDefinition { name: "supplier_name", column_type: "TEXT", notnull: true, default_value: None },    // 供应商名称，唯一约束
                ColumnDefinition { name: "contact", column_type: "TEXT", notnull: false, default_value: None },         // 联系人，可为空
                ColumnDefinition { name: "phone", column_type: "TEXT", notnull: false, default_value: None },           // 联系电话，可为空
                ColumnDefinition { name: "remark", column_type: "TEXT", notnull: false, default_value: None },          // 备注，可为空
            ],
        },
        // ========== 6. 客户表 ==========
        // 存储客户联系信息
        TableDefinition {
            name: "customer",                        // 表名：客户表
            create_sql: r#"CREATE TABLE IF NOT EXISTS customer (
                customer_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 客户ID（自增主键）
                customer_name TEXT NOT NULL UNIQUE,   -- 客户名称（唯一）
                contact TEXT,                           -- 联系人
                phone TEXT,                             -- 联系电话
                remark TEXT                             -- 备注
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "customer_id", column_type: "INTEGER", notnull: true, default_value: None },   // 客户ID，自增主键
                ColumnDefinition { name: "customer_name", column_type: "TEXT", notnull: true, default_value: None },    // 客户名称，唯一约束
                ColumnDefinition { name: "contact", column_type: "TEXT", notnull: false, default_value: None },         // 联系人，可为空
                ColumnDefinition { name: "phone", column_type: "TEXT", notnull: false, default_value: None },           // 联系电话，可为空
                ColumnDefinition { name: "remark", column_type: "TEXT", notnull: false, default_value: None },          // 备注，可为空
            ],
        },
        // ========== 7. 入库单表 ==========
        // 存储入库单主信息（不含明细项）
        TableDefinition {
            name: "in_order",                        // 表名：入库单表
            create_sql: r#"CREATE TABLE IF NOT EXISTS in_order (
                order_id TEXT NOT NULL PRIMARY KEY,     -- 入库单ID（主键）
                warehouse_id TEXT NOT NULL,              -- 仓库ID（外键关联 warehouse）
                supplier_id INTEGER NOT NULL,            -- 供应商ID（外键关联 supplier）
                operator_id TEXT NOT NULL,               -- 操作员ID（外键关联 sys_user）
                contact TEXT,                             -- 联系人
                phone TEXT,                               -- 联系电话
                order_status TEXT NOT NULL,               -- 单据状态（待审核/已审核/已拒绝）
                order_type TEXT DEFAULT '入库',           -- 单据类型（默认"入库"）
                purchase_time TEXT,                      -- 采购时间
                complete_time TEXT,                      -- 完成时间（审核通过时更新）
                remark TEXT                              -- 备注
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "order_id", column_type: "TEXT", notnull: true, default_value: None },         // 入库单ID，主键
                ColumnDefinition { name: "warehouse_id", column_type: "TEXT", notnull: true, default_value: None },     // 仓库ID，外键
                ColumnDefinition { name: "supplier_id", column_type: "INTEGER", notnull: true, default_value: None },   // 供应商ID，外键
                ColumnDefinition { name: "operator_id", column_type: "TEXT", notnull: true, default_value: None },      // 操作员ID，外键
                ColumnDefinition { name: "contact", column_type: "TEXT", notnull: false, default_value: None },         // 联系人，可为空
                ColumnDefinition { name: "phone", column_type: "TEXT", notnull: false, default_value: None },           // 联系电话，可为空
                ColumnDefinition { name: "order_status", column_type: "TEXT", notnull: true, default_value: None },     // 单据状态
                ColumnDefinition { name: "order_type", column_type: "TEXT", notnull: false, default_value: Some("'入库'") }, // 单据类型，默认"入库"
                ColumnDefinition { name: "purchase_time", column_type: "TEXT", notnull: false, default_value: None },   // 采购时间，可为空
                ColumnDefinition { name: "complete_time", column_type: "TEXT", notnull: false, default_value: None },   // 完成时间，可为空
                ColumnDefinition { name: "remark", column_type: "TEXT", notnull: false, default_value: None },          // 备注，可为空
            ],
        },
        // ========== 8. 入库单明细表 ==========
        // 存储入库单中的物资明细项
        TableDefinition {
            name: "in_order_item",                   // 表名：入库单明细表
            create_sql: r#"CREATE TABLE IF NOT EXISTS in_order_item (
                item_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 明细ID（自增主键）
                order_id TEXT NOT NULL,               -- 入库单ID（外键关联 in_order）
                material_id TEXT NOT NULL,             -- 物资ID（外键关联 material）
                num INTEGER NOT NULL,                  -- 数量
                purchase_price REAL NOT NULL,          -- 采购单价
                sell_price REAL NOT NULL,              -- 销售单价
                total_amount REAL GENERATED ALWAYS AS (num * purchase_price) STORED  -- 计算列：总金额 = 数量 * 采购价
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "item_id", column_type: "INTEGER", notnull: true, default_value: None },       // 明细ID，自增主键
                ColumnDefinition { name: "order_id", column_type: "TEXT", notnull: true, default_value: None },         // 入库单ID，外键
                ColumnDefinition { name: "material_id", column_type: "TEXT", notnull: true, default_value: None },      // 物资ID，外键
                ColumnDefinition { name: "num", column_type: "INTEGER", notnull: true, default_value: None },           // 数量
                ColumnDefinition { name: "purchase_price", column_type: "REAL", notnull: true, default_value: None },   // 采购单价
                ColumnDefinition { name: "sell_price", column_type: "REAL", notnull: true, default_value: None },       // 销售单价
                ColumnDefinition { name: "total_amount", column_type: "REAL", notnull: false, default_value: None },    // 总金额（计算列）
            ],
        },
        // ========== 9. 出库单表 ==========
        // 存储出库单主信息（不含明细项）
        TableDefinition {
            name: "out_order",                       // 表名：出库单表
            create_sql: r#"CREATE TABLE IF NOT EXISTS out_order (
                order_id TEXT NOT NULL PRIMARY KEY,     -- 出库单ID（主键）
                warehouse_id TEXT NOT NULL,              -- 仓库ID（外键关联 warehouse）
                customer_id INTEGER NOT NULL,            -- 客户ID（外键关联 customer）
                operator_id TEXT NOT NULL,               -- 操作员ID（外键关联 sys_user）
                contact TEXT,                             -- 联系人
                phone TEXT,                               -- 联系电话
                order_status TEXT NOT NULL,               -- 单据状态（待审核/已审核/已拒绝）
                order_type TEXT DEFAULT '出库',           -- 单据类型（默认"出库"）
                out_time TEXT,                            -- 出库时间
                complete_time TEXT,                      -- 完成时间（审核通过时更新）
                remark TEXT                              -- 备注
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "order_id", column_type: "TEXT", notnull: true, default_value: None },         // 出库单ID，主键
                ColumnDefinition { name: "warehouse_id", column_type: "TEXT", notnull: true, default_value: None },     // 仓库ID，外键
                ColumnDefinition { name: "customer_id", column_type: "INTEGER", notnull: true, default_value: None },   // 客户ID，外键
                ColumnDefinition { name: "operator_id", column_type: "TEXT", notnull: true, default_value: None },      // 操作员ID，外键
                ColumnDefinition { name: "contact", column_type: "TEXT", notnull: false, default_value: None },         // 联系人，可为空
                ColumnDefinition { name: "phone", column_type: "TEXT", notnull: false, default_value: None },           // 联系电话，可为空
                ColumnDefinition { name: "order_status", column_type: "TEXT", notnull: true, default_value: None },     // 单据状态
                ColumnDefinition { name: "order_type", column_type: "TEXT", notnull: false, default_value: Some("'出库'") }, // 单据类型，默认"出库"
                ColumnDefinition { name: "out_time", column_type: "TEXT", notnull: false, default_value: None },        // 出库时间，可为空
                ColumnDefinition { name: "complete_time", column_type: "TEXT", notnull: false, default_value: None },   // 完成时间，可为空
                ColumnDefinition { name: "remark", column_type: "TEXT", notnull: false, default_value: None },          // 备注，可为空
            ],
        },
        // ========== 10. 出库单明细表 ==========
        // 存储出库单中的物资明细项
        TableDefinition {
            name: "out_order_item",                  // 表名：出库单明细表
            create_sql: r#"CREATE TABLE IF NOT EXISTS out_order_item (
                item_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 明细ID（自增主键）
                order_id TEXT NOT NULL,               -- 出库单ID（外键关联 out_order）
                material_id TEXT NOT NULL,             -- 物资ID（外键关联 material）
                num INTEGER NOT NULL,                  -- 数量
                purchase_price REAL NOT NULL,          -- 采购单价
                sell_price REAL NOT NULL,              -- 销售单价
                total_amount REAL GENERATED ALWAYS AS (num * sell_price) STORED  -- 计算列：总金额 = 数量 * 销售价
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应
                ColumnDefinition { name: "item_id", column_type: "INTEGER", notnull: true, default_value: None },       // 明细ID，自增主键
                ColumnDefinition { name: "order_id", column_type: "TEXT", notnull: true, default_value: None },         // 出库单ID，外键
                ColumnDefinition { name: "material_id", column_type: "TEXT", notnull: true, default_value: None },      // 物资ID，外键
                ColumnDefinition { name: "num", column_type: "INTEGER", notnull: true, default_value: None },           // 数量
                ColumnDefinition { name: "purchase_price", column_type: "REAL", notnull: true, default_value: None },   // 采购单价
                ColumnDefinition { name: "sell_price", column_type: "REAL", notnull: true, default_value: None },       // 销售单价
                ColumnDefinition { name: "total_amount", column_type: "REAL", notnull: false, default_value: None },    // 总金额（计算列）
            ],
        },
        // ========== 11. 库存表 ==========
        // 存储物资在各仓库的实时库存，UNIQUE(material_id, warehouse_id) 确保同一物资在同一仓库只有一条记录
        TableDefinition {
            name: "stock",                           // 表名：库存表
            create_sql: r#"CREATE TABLE IF NOT EXISTS stock (
                stock_id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,  -- 库存ID（自增主键）
                material_id TEXT NOT NULL,            -- 物资ID（外键关联 material）
                warehouse_id TEXT NOT NULL,            -- 仓库ID（外键关联 warehouse）
                stock_num INTEGER NOT NULL,            -- 库存数量
                in_time TEXT,                           -- 入库时间
                update_time TEXT,                      -- 更新时间
                UNIQUE(material_id, warehouse_id)      -- 联合唯一约束：同一物资在同一仓库只有一条记录
            )"#,
            expected_columns: vec![                     // 期望列定义：与建表 SQL 一一对应（UNIQUE 约束不在列定义中）
                ColumnDefinition { name: "stock_id", column_type: "INTEGER", notnull: true, default_value: None },      // 库存ID，自增主键
                ColumnDefinition { name: "material_id", column_type: "TEXT", notnull: true, default_value: None },      // 物资ID，外键
                ColumnDefinition { name: "warehouse_id", column_type: "TEXT", notnull: true, default_value: None },     // 仓库ID，外键
                ColumnDefinition { name: "stock_num", column_type: "INTEGER", notnull: true, default_value: None },     // 库存数量
                ColumnDefinition { name: "in_time", column_type: "TEXT", notnull: false, default_value: None },         // 入库时间，可为空
                ColumnDefinition { name: "update_time", column_type: "TEXT", notnull: false, default_value: None },     // 更新时间，可为空
            ],
        },
    ]
}

/// 初始化数据库表结构（带迁移支持）
///
/// 应用启动时调用，确保所有表和列都存在
/// 如果表不存在则创建，已存在则检测并补充缺失的列
pub async fn init_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    run_migrations(db).await  // 委托给 run_migrations 执行完整的迁移流程
}

/// 运行数据库迁移
///
/// 迁移流程：
/// 1. 创建迁移版本记录表（schema_migrations）
/// 2. 同步所有表结构（建表 + 补列）
/// 3. 记录当前迁移版本号
pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), DbErr> {
    tracing::info!("Starting database migrations...");   // 开始迁移日志

    // 第1步：创建迁移记录表（记录已执行的迁移版本）
    create_migrations_table(db).await?;

    // 第2步：同步所有表结构（不存在则创建，已存在则补充缺失列）
    sync_all_tables(db).await?;

    // 第3步：检查并记录迁移版本
    let current_version = get_current_version(db).await?;  // 获取当前已执行的版本号，0 表示未执行过迁移
    if current_version < CURRENT_SCHEMA_VERSION {            // 当前版本低于最新版本
        // 当前版本低于最新版本，记录新版本
        record_migration(db, CURRENT_SCHEMA_VERSION).await?;  // 写入新版本号到 schema_migrations 表
        tracing::info!("Database migration to version {} completed", CURRENT_SCHEMA_VERSION);
    } else {
        // 版号已是最新，无需迁移
        tracing::info!("Database is up to date");            // 数据库结构已是最新的
    }

    Ok(())  // 迁移完成
}

/// 创建迁移记录表
///
/// schema_migrations 表记录已执行的迁移版本，防止重复执行
async fn create_migrations_table(db: &DatabaseConnection) -> Result<(), DbErr> {
    let sql = r#"CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY,     -- 迁移版本号（主键，每个版本只记录一次）
        applied_at TEXT NOT NULL         -- 执行时间（RFC3339 格式，如 "2024-01-01T00:00:00Z"）
    )"#;

    // 执行建表 SQL（IF NOT EXISTS 确保幂等，多次调用不会报错）
    db.execute(Statement::from_string(
        sea_orm::DatabaseBackend::Sqlite,  // 指定 SQLite 方言
        sql.to_string(),                    // 将 &str 转为 String
    ))
    .await?;  // 执行失败则向上抛出 DbErr

    Ok(())
}

/// 获取当前数据库 Schema 版本号
///
/// 查询 schema_migrations 表中的最大版本号
/// 如果表为空，返回 0（表示未执行过迁移）
async fn get_current_version(db: &DatabaseConnection) -> Result<i64, DbErr> {
    let sql = "SELECT COALESCE(MAX(version), 0) as version FROM schema_migrations";  // COALESCE 将 NULL 转为 0
    let result = VersionRow::find_by_statement(                            // 使用 FromQueryResult 查询
        Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql.to_string()),
    )
    .one(db)          // 只取一行结果（聚合查询最多返回一行）
    .await?;          // 查询失败则向上抛出 DbErr

    Ok(result.map(|r| r.version).unwrap_or(0))  // 有结果则取 version 字段，无结果返回 0
}

/// 记录迁移版本
///
/// 将版本号和执行时间写入 schema_migrations 表（使用参数化查询防止 SQL 注入）
async fn record_migration(db: &DatabaseConnection, version: i64) -> Result<(), DbErr> {
    let applied_at = chrono::Utc::now().to_rfc3339();   // 获取当前 UTC 时间，格式化为 RFC3339 字符串
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Sqlite,               // 指定 SQLite 方言
        "INSERT INTO schema_migrations (version, applied_at) VALUES (?, ?)",  // 参数化 INSERT
        [version.into(), applied_at.into()],            // 参数值：版本号 + 执行时间
    );

    db.execute(stmt).await?;  // 执行插入，失败则向上抛出 DbErr

    Ok(())
}

/// 检查表是否存在
///
/// 查询 SQLite 的 sqlite_master 系统表判断指定表是否已创建（使用参数化查询）
async fn table_exists(db: &DatabaseConnection, table_name: &str) -> Result<bool, DbErr> {
    let stmt = Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::Sqlite,               // 指定 SQLite 方言
        "SELECT name FROM sqlite_master WHERE type='table' AND name=?",  // 查询系统表，参数化防注入
        [table_name.into()],                            // 参数值：表名
    );
    let result = TableNameRow::find_by_statement(stmt).one(db).await?;  // 查询，只取一行

    Ok(result.is_some())  // Some 表示表存在，None 表示表不存在
}

/// 同步所有表结构
///
/// 遍历所有表定义：
/// - 表不存在 → 执行 CREATE TABLE
/// - 表已存在 → 检查并补充缺失的列（ALTER TABLE ADD COLUMN）
async fn sync_all_tables(db: &DatabaseConnection) -> Result<(), DbErr> {
    let tables = get_all_tables();          // 获取全部 11 张表的定义
    let mut tables_created = 0;             // 新建的表计数
    let mut tables_synced = 0;              // 同步了列的表计数

    for table in &tables {                  // 遍历每张表的定义
        let exists = table_exists(db, table.name).await?;  // 检查表是否已存在于数据库中

        if !exists {
            // 表不存在，执行建表 SQL
            tracing::info!("Creating table: {}", table.name);
            db.execute(Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,           // 指定 SQLite 方言
                table.create_sql.to_string(),                // 建表 SQL（含 IF NOT EXISTS 保证幂等）
            ))
            .await?;  // 执行失败则向上抛出 DbErr
            tables_created += 1;  // 新建计数 +1
        } else {
            // 表已存在，检查是否有缺失的列需要补充
            tracing::debug!("Table {} exists, checking schema...", table.name);
            let synced = sync_table_columns(db, table).await?;  // 同步列结构
            if synced {
                tables_synced += 1;  // 有变更的表计数 +1
            }
        }
    }

    // 打印迁移汇总日志
    if tables_created > 0 {                  // 有新建的表
        tracing::info!("Created {} new tables", tables_created);
    }
    if tables_synced > 0 {                   // 有列变更的表
        tracing::info!("Synchronized {} tables with new columns", tables_synced);
    }

    Ok(())
}

/// 同步单个表的列结构：添加缺失的列
///
/// 对比期望列列表与实际列列表，对于缺失的列执行 ALTER TABLE ADD COLUMN
///
/// # 返回
/// * `Ok(true)` - 有列被添加
/// * `Ok(false)` - 表结构完整，无需变更
async fn sync_table_columns(db: &DatabaseConnection, table: &TableDefinition) -> Result<bool, DbErr> {
    // 获取表中现有的列名集合
    let existing_cols = get_existing_columns_raw(db, table.name).await?;  // 通过 PRAGMA table_info 查询
    let mut has_changes = false;  // 标记是否有列变更

    // 遍历期望的每一列，检查是否存在
    for col in &table.expected_columns {
        if !existing_cols.contains(col.name) {  // 列名不在现有集合中，说明缺失
            // 列不存在，需要添加
            tracing::info!("Adding missing column '{}' to table '{}'", col.name, table.name);

            // 构建 ALTER TABLE ADD COLUMN SQL
            let mut add_sql = format!("ALTER TABLE {} ADD COLUMN {} {}",
                table.name, col.name, col.column_type);  // 基础 SQL：ALTER TABLE 表名 ADD COLUMN 列名 类型

            // 添加 NOT NULL 约束
            if col.notnull {
                add_sql.push_str(" NOT NULL");  // 拼接 NOT NULL 约束
            }
            // 添加 DEFAULT 默认值
            if let Some(default) = col.default_value {
                add_sql.push_str(&format!(" DEFAULT {}", default));  // 拼接 DEFAULT 默认值
            }

            // 执行 ALTER TABLE SQL
            let result = db.execute(Statement::from_string(
                sea_orm::DatabaseBackend::Sqlite,   // 指定 SQLite 方言
                add_sql,                              // 动态构建的 ALTER TABLE SQL
            )).await;

            match result {
                Ok(_) => {
                    tracing::info!("  -> Column '{}' added successfully", col.name);
                    has_changes = true;  // 标记有变更
                }
                Err(e) => {
                    tracing::warn!("  -> Failed to add column '{}': {}", col.name, e);
                    // 添加失败不中断，继续处理下一列（可能是 SQLite 的 ALTER TABLE 限制）
                }
            }
        }
    }

    Ok(has_changes)  // 返回是否有列变更
}

/// 获取表的已有列名集合
///
/// 通过 SQLite PRAGMA table_info 命令查询表中已有的列名
async fn get_existing_columns_raw(db: &DatabaseConnection, table_name: &str) -> Result<HashSet<String>, DbErr> {
    let sql = format!("PRAGMA table_info('{}')", table_name);  // SQLite 专有命令，返回表的列信息
    let results = PragmaColumnInfo::find_by_statement(
        Statement::from_string(sea_orm::DatabaseBackend::Sqlite, sql),
    )
    .all(db)          // 获取所有列的信息（一行代表一列）
    .await?;          // 查询失败则向上抛出 DbErr

    Ok(results.into_iter().map(|r| r.name).collect())  // 提取列名，收集到 HashSet 中用于快速查找
}

