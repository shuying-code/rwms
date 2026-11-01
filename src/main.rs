// ============================================
// WMS 仓库管理系统 - Rust + Axum 实现
// 采用 DDD 领域驱动设计 4 层架构
// Interface(接口层) → Application(应用层) → Domain(领域层) ← Infrastructure(基础设施层)
// ============================================

// Windows 子系统设置：release 模式下不弹控制台黑窗，纯后台运行
// debug 模式（IDE 调试）保持控制台输出
#![cfg_attr(all(windows, not(debug_assertions), target_env = "msvc"), windows_subsystem = "windows")]

mod infrastructure;
mod domain;
mod application;
mod interfaces;
mod state;

use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main] // Tokio 异步运行时入口标记，将同步 main 函数转换为异步运行时
async fn main() {
    // ===== 第1步：加载 YAML 配置文件 =====
    let config = infrastructure::config::app_config::AppConfig::load_with_env() // 从 application.yml 加载配置，支持 APP_ENV 环境变量切换配置文件
        .expect("配置文件加载失败，请检查 application.yml 是否存在且格式正确"); // 配置加载失败时直接 panic 终止程序

    // ===== 第2步：初始化结构化日志 =====
    // debug 编译模式（IDE 调试）：日志输出到控制台，方便开发调试
    // release 编译模式（打包 exe）：日志输出到文件，exe 启动无控制台窗口

    let log_dir = infrastructure::config::database::get_db_directory(); // 获取数据库目录，日志文件与其同级存放
    std::fs::create_dir_all(&log_dir).ok(); // 确保日志目录存在

    if cfg!(debug_assertions) {
        // === 开发模式：控制台输出（IDE 调试可见） ===
        tracing_subscriber::fmt()
            .with_max_level(config.log_level())
            .with_target(true) // 显示模块路径，便于定位日志来源
            .with_file(true)   // 显示源码文件名
            .with_line_number(true) // 显示行号
            .init();
    } else {
        // === 生产模式：文件输出（打包 exe 无控制台窗口） ===
        let file_appender = tracing_appender::rolling::daily(&log_dir, "wms.log");
        tracing_subscriber::fmt()
            .with_writer(file_appender) // 日志写入数据库目录下的 wms.log 文件
            .with_max_level(config.log_level())
            .with_ansi(false) // 文件输出不需要 ANSI 颜色码
            .with_target(true)
            .init();
    }

    tracing::info!("Starting {} v{}", config.app.name, config.app.version); // 输出应用名称和版本号到日志
    tracing::info!("Environment: {}", env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string())); // 输出当前运行环境（dev/test/prod）

    // ===== 第3步：连接 SQLite 数据库 =====
    let db = infrastructure::config::database::connect_db_with_config(&config.database) // 根据配置连接 SQLite 数据库，返回 DatabaseConnection 实例
        .await // 异步等待连接建立
        .expect("数据库连接失败，请确认 wms.db 文件可访问"); // 连接失败时 panic 终止

    // ===== 第4步：初始化数据库表结构 =====
    infrastructure::config::database_init::init_tables(&db) // 执行 DDL 建表语句，创建 sys_user、material、stock 等业务表
        .await // 异步等待建表完成
        .expect("数据库表初始化失败"); // 建表失败时 panic 终止

    // ===== 第5步：准备 JWT 密钥 =====
    let jwt_secret = env::var("JWT_SECRET") // 尝试从环境变量读取 JWT 密钥
        .unwrap_or_else(|_| config.jwt.secret.clone()); // 若环境变量未设置，则使用配置文件中的密钥

    // ===== 第6步：组装应用状态 =====
    let state = state::AppState::new(db, jwt_secret); // 创建 AppState 实例，内部初始化所有应用层服务

    // ===== 第7步：配置 CORS =====
    let cors = if config.cors.allow_origins == "*" { // 判断配置是否允许所有来源
        CorsLayer::new() // 创建新的 CORS 中间件层
            .allow_origin(Any) // 允许任意来源的跨域请求
            .allow_methods(Any) // 允许任意 HTTP 方法（GET/POST/PUT/DELETE 等）
            .allow_headers(Any) // 允许任意请求头（包括 Authorization）
    } else {
        CorsLayer::new() // 当前实现：即使配置了具体来源也使用 Any（后续可优化为精确配置）
            .allow_origin(Any) // 允许任意来源
            .allow_methods(Any) // 允许任意方法
            .allow_headers(Any) // 允许任意请求头
    };

    // ===== 第8步：构建 Axum 路由 =====
    let app = interfaces::routes::create_router(state) // 调用路由工厂函数，创建包含公开路由和受保护路由的完整路由表
        .layer(cors) // 叠加 CORS 中间件层，处理跨域请求
        .layer(TraceLayer::new_for_http()); // 叠加 HTTP 请求追踪中间件层，自动记录请求/响应日志

    // ===== 第9步：绑定 TCP 监听端口 =====
    let port = env::var("PORT") // 尝试从环境变量读取端口号（适用于 Docker/云平台部署）
        .ok() // 将 Result 转为 Option，忽略环境变量不存在的错误
        .and_then(|p| p.parse().ok()) // 尝试将字符串解析为整数，解析失败则返回 None
        .unwrap_or(config.server.port); // 若环境变量未设置或解析失败，使用配置文件中的端口（默认 8080）

    let addr = format!("{}:{}", config.server.host, port); // 拼接监听地址，如 "0.0.0.0:8080"

    tracing::info!("Server will start on {}", addr); // 输出即将监听的地址

    // ===== 第10步：启动 TCP 服务器 =====
    let listener = tokio::net::TcpListener::bind(&addr) // 绑定 TCP 端口，开始监听
        .await // 异步等待绑定完成
        .expect("端口被占用或无权限，请检查 8080 端口是否已被使用"); // 绑定失败时 panic 终止

    tracing::info!("🚀 {} is running on {}", config.app.name, addr); // 输出服务启动成功信息
    tracing::info!("📝 Description: {}", config.app.description); // 输出应用描述信息
    tracing::info!("🌐 Access URL: http://localhost:{}/login.html", port); // 输出前端访问地址

    // ===== 第11步（可选）：自动打开浏览器 =====
    open_browser(&format!("http://localhost:{}/login.html", port)); // 调用系统命令自动打开默认浏览器

    // ===== 第12步：运行 Axum HTTP 服务器 =====
    let svc = app.into_make_service(); // 将 Router 转换为 Tower Service，支持连接复用
    axum::serve(listener, svc) // 启动 Axum HTTP 服务器，开始接受客户端连接
        .await // 异步等待服务器运行（通常不会返回，除非出错）
        .expect("HTTP 服务器异常退出"); // 服务器异常退出时 panic
}

/// 自动打开默认浏览器访问指定 URL
fn open_browser(url: &str) {
    use std::process::Command; // 导入进程命令模块，用于启动外部程序
    let (cmd, args): (&str, Vec<&str>) = if cfg!(target_os = "windows") { // 编译期判断操作系统类型
        ("cmd", vec!["/C", "start", url]) // Windows：使用 cmd /C start 命令打开浏览器
    } else if cfg!(target_os = "macos") {
        ("open", vec![url]) // macOS：使用 open 命令打开浏览器
    } else {
        ("xdg-open", vec![url]) // Linux：使用 xdg-open 命令打开浏览器
    };
    if Command::new(cmd).args(&args).spawn().is_ok() { // 异步启动浏览器进程，不阻塞主线程
        tracing::info!("已自动打开浏览器访问登录页面"); // 浏览器启动成功日志
    } else {
        tracing::warn!("自动打开浏览器失败，请手动访问: {}", url); // 浏览器启动失败警告
    }
}
