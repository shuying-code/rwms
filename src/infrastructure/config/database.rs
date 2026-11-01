// ============================================
// 数据库连接模块
// 对应 Java 的 DataSourceConfig / HikariDataSource
// 负责 SQLite 数据库连接池的初始化和跨平台路径处理
// ============================================

// SeaORM 核心类型：Database 用于建立连接，DatabaseConnection 是连接池句柄，DbErr 是数据库错误
use sea_orm::{Database, DatabaseConnection, DbErr};
// 数据库配置结构体，从 application.yml 解析而来
use crate::infrastructure::config::app_config::DatabaseConfig;
// 标准库路径处理：Path 用于跨平台文件路径操作
use std::path::Path;
// 标准库文件系统操作：fs 用于创建目录和文件
use std::fs;

/// 数据库文件名常量
const DB_FILE_NAME: &str = "wms.db";           // SQLite 数据库文件名，固定为 wms.db

/// Linux/macOS 数据库目录（系统级共享路径，需要 root/管理员权限）
/// 注意：macOS 上 /wms 不是标准数据目录，实际部署可能需要调整
const DB_DIR_UNIX: &str = "/wms";              // Unix 系统下数据库文件存放目录

/// 获取当前操作系统名称字符串
///
/// 用于日志输出，帮助排查跨平台问题
fn get_os_name() -> &'static str {
    match std::env::consts::OS {  // 编译期常量，零开销，在编译时即确定目标平台
        "windows" => "Windows",    // Windows 系统
        "linux" => "Linux",        // Linux 系统
        "macos" => "macOS",        // macOS 系统
        _ => "Unknown",            // 其他 Unix 系统（如 FreeBSD、NetBSD 等）
    }
}

/// 获取数据库文件路径（根据操作系统动态决定）
///
/// - Windows: C:\ProgramData\wms\wms.db（系统级公共数据目录，所有用户可读写）
/// - Linux/macOS: /wms/wms.db（需要 root 权限或手动设置目录权限）
fn get_db_path() -> String {
    match std::env::consts::OS {     // 编译期判断操作系统类型
        "windows" => {
            // 读取 Windows 环境变量 PROGRAMDATA（通常为 C:\ProgramData）
            let program_data = std::env::var("PROGRAMDATA")
                .unwrap_or_else(|_| "C:/ProgramData".to_string());  // 默认值兜底，环境变量不存在时使用
            // 拼接路径：C:/ProgramData/wms/wms.db，统一使用正斜杠避免转义问题
            format!("{}/wms/{}", program_data, DB_FILE_NAME)
                .replace('\\', "/")  // 反斜杠替换为正斜杠，SQLite 驱动兼容性更好
        }
        // 非 Windows 系统：使用 /wms/wms.db
        _ => format!("{}/{}", DB_DIR_UNIX, DB_FILE_NAME),  // 拼接为 /wms/wms.db
    }
}

/// 构建跨平台数据库连接 URL
///
/// SQLite 连接 URL 格式：sqlite:<绝对路径>?mode=rwc
/// mode=rwc 表示 Read-Write-Create（读写+不存在则创建）
fn build_db_url() -> String {
    // 获取当前平台的数据库文件路径
    let db_path = get_db_path();        // 根据操作系统返回对应路径
    let os_name = get_os_name();        // 获取操作系统名称，用于日志

    tracing::info!("Detected OS: {}", os_name);   // 输出检测到的操作系统

    // 第1步：确保数据库文件所在目录存在（不存在则递归创建）
    ensure_db_directory(&db_path);      // 必须先创建目录，否则后续创建文件会失败

    // 第2步：确保数据库文件存在（不存在则创建空文件，表结构由 database_init 模块后续创建）
    ensure_db_file(&db_path);           // SQLite 要求文件存在才能打开连接

    // 第3步：构建连接 URL，将路径中的反斜杠统一为正斜杠
    let normalized_path = db_path.replace('\\', "/");   // Windows 路径兼容处理
    tracing::info!("Database path: {}", normalized_path);  // 输出最终数据库路径

    // 拼接 SQLite 连接 URL：sqlite:绝对路径?mode=rwc
    format!("sqlite:{}?mode=rwc", normalized_path)  // mode=rwc: 读写模式，文件不存在则自动创建
}

/// 检查数据库文件是否存在，不存在则创建空文件
///
/// SQLite 首次连接空文件后，由 database_init 模块负责创建所有业务表结构
fn ensure_db_file(db_path: &str) {
    let path = Path::new(db_path);  // 将字符串路径转为 Path 对象，支持跨平台路径操作

    if path.exists() {
        // 数据库文件已存在，无需操作
        tracing::info!("Database file exists: {}", db_path);
    } else {
        // 数据库文件不存在，创建空文件
        tracing::info!("Database file not found, creating: {}", db_path);
        match fs::File::create(path) {  // 创建空文件（如果目录不存在会失败，因此之前必须先 ensure_db_directory）
            Ok(_) => tracing::info!("Database file created successfully"),       // 创建成功
            Err(e) => tracing::error!("Failed to create database file: {}", e),  // 创建失败（权限不足等）
        }
    }
}

/// 确保数据库文件所在目录存在
///
/// 如果目录不存在，递归创建所有父目录（类似 mkdir -p）
fn ensure_db_directory(db_path: &str) {
    // 统一路径分隔符，避免 Windows 下反斜杠导致的 Path 解析问题
    let normalized_path = db_path.replace('\\', "/");   // 反斜杠→正斜杠
    let path = Path::new(&normalized_path);              // 转为 Path 对象

    // 获取父目录路径
    if let Some(parent) = path.parent() {               // parent() 返回 Option<&Path>，根目录的 parent 为 None
        let parent_str = parent.to_string_lossy();       // 路径可能包含非 UTF-8 字符，使用 Cow<str> 兼容处理
        // 跳过空路径和当前目录（"."），这两种情况无需创建
        if !parent_str.is_empty() && parent_str != "." {
            let parent_path = Path::new(parent_str.as_ref());  // 将 Cow<str> 转回 Path 引用
            if !parent_path.exists() {                          // 父目录不存在
                // 父目录不存在，递归创建（包含所有祖先目录）
                tracing::info!("Creating database directory: {}", parent_str);
                if let Err(e) = fs::create_dir_all(parent_path) {   // create_dir_all 等同于 mkdir -p
                    tracing::error!("Failed to create database directory: {}", e);  // 创建失败（权限不足等）
                }
            }
        }
    }
}

/// 获取数据库文件路径（供外部模块查询数据库位置）
#[allow(dead_code)]
pub fn get_db_file_path() -> String {
    get_db_path()  // 委托给 get_db_path()，根据操作系统返回对应路径
}

/// 获取数据库所在目录路径（不含文件名）
///
/// 用于将日志文件等辅助数据存放在与数据库相同的目录下
/// - Windows: C:\ProgramData\wms\
/// - Linux/macOS: /wms/
pub fn get_db_directory() -> String {
    let db_path = get_db_path();
    // 统一路径分隔符后取父目录
    let normalized = db_path.replace('\\', "/");
    if let Some(parent) = Path::new(&normalized).parent() {
        parent.to_string_lossy().to_string()
    } else {
        // 兜底：如果取不到父目录，返回当前目录
        ".".to_string()
    }
}

/// 从配置初始化数据库连接池
///
/// 对应 Java 的 DataSourceConfig.dataSource()
/// 优先使用 application.yml 中配置的 URL，未配置则使用跨平台默认路径
///
/// # 参数
/// * `config` - 数据库配置（包含 url、pool 等配置项）
///
/// # 返回
/// * `Ok(DatabaseConnection)` - 连接池句柄，可克隆共享（内部是 Arc<Pool>）
/// * `Err(DbErr)` - 连接失败（路径不可访问、文件权限不足等）
pub async fn connect_db_with_config(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    // 优先使用配置文件中的 URL，如果没有配置才使用跨平台默认路径
    let db_url = if !config.url.is_empty() {       // 检查配置中是否指定了数据库 URL
        tracing::info!("Using database URL from config: {}", config.url);
        config.url.clone()  // 使用配置文件中的 URL（如 "sqlite:./wms.db"）
    } else {
        tracing::info!("Config database.url is empty, using cross-platform default path");
        build_db_url()  // 自动构建跨平台数据库 URL（含目录和文件自动创建）
    };

    tracing::info!("Connecting to database: {}", db_url);   // 输出实际使用的连接 URL

    // 建立数据库连接（SQLite 是本地文件数据库，连接即打开文件）
    let db = Database::connect(&db_url).await?;    // SeaORM 异步连接，? 失败则向上抛出 DbErr

    tracing::info!("Database connection established");  // 连接成功
    // 打印连接池配置信息
    tracing::info!("   Pool size: {} - {}",
        config.pool.min_connections,  // 最小连接数
        config.pool.max_connections   // 最大连接数
    );

    Ok(db)  // 返回连接池句柄（DatabaseConnection 内部使用 Arc，可跨任务共享）
}

/// 初始化数据库连接池（兼容旧版本，从环境变量读取 DATABASE_URL）
///
/// 当不使用 application.yml 配置时，可通过 .env 文件提供连接信息
#[allow(dead_code)]
pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    use dotenvy::dotenv;  // .env 文件加载库，将 .env 中键值对注入环境变量
    use std::env;

    // 加载 .env 文件中的环境变量到进程环境
    dotenv().ok();        // ok() 忽略 .env 文件不存在的错误

    // 从环境变量获取数据库 URL（.env 文件中必须配置 DATABASE_URL）
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");   // 不存在则 panic，强制要求配置

    tracing::info!("Connecting to database: {}", database_url);

    // 建立数据库连接
    let db = Database::connect(&database_url).await?;   // SeaORM 异步连接

    tracing::info!("Database connection established");

    Ok(db)  // 返回连接池句柄
}

/// 获取数据库 URL（仅从环境变量读取，不自动连接）
#[allow(dead_code)]
pub fn get_database_url() -> String {
    use dotenvy::dotenv;    // .env 文件加载库
    use std::env;

    dotenv().ok();  // 加载 .env，忽略文件不存在的错误
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")  // 读取并返回，不存在则 panic
}
