// ============================================
// 应用配置模块
// 对应 Java 的 @ConfigurationProperties + @Configuration
// 从 application.yml 文件加载配置，支持环境变量覆盖
// ============================================

// serde 反序列化 trait：将 YAML/JSON 字符串自动映射为 Rust 结构体
use serde::Deserialize;
// 标准库文件系统：读取 application.yml 文件内容
use std::fs;
// 标准库环境变量：读取 DATABASE_URL、JWT_SECRET、PORT 等环境变量
use std::env;

/// 应用总配置结构
///
/// 对应 Spring Boot 的 @ConfigurationProperties(prefix = "")
/// 与 application.yml 的顶层结构一一对应
#[derive(Debug, Deserialize, Clone)]  // Debug 用于日志打印，Deserialize 用于 YAML 反序列化，Clone 用于多处使用
pub struct AppConfig {
    /// 服务器配置（端口、主机地址）
    pub server: ServerConfig,           // 对应 yml 中 server 节点
    /// 应用信息（名称、版本、描述）
    pub app: AppInfo,                   // 对应 yml 中 app 节点
    /// 数据库配置（URL、连接池）
    pub database: DatabaseConfig,       // 对应 yml 中 database 节点
    /// JWT 配置（密钥、过期时间、签发者）
    pub jwt: JwtConfig,                 // 对应 yml 中 jwt 节点
    /// 日志配置（级别、格式）
    pub logging: LoggingConfig,         // 对应 yml 中 logging 节点
    /// CORS 跨域配置（允许来源、方法、头部）
    pub cors: CorsConfig,               // 对应 yml 中 cors 节点
    /// 业务配置（分页、库存预警、订单超时）
    #[serde(default)]           // YAML 中未配置 business 节点时，使用 Default 默认值
    #[allow(dead_code)]         // 当前代码未直接读取，保留供将来使用
    pub business: BusinessConfig,       // 对应 yml 中 business 节点
}

/// 服务器配置
///
/// 对应 application.yml 中的 server 节点
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// 监听端口号（如 8080）
    pub port: u16,                      // u16 类型，范围 0-65535
    /// 绑定主机地址（如 "0.0.0.0" 表示所有网卡，"127.0.0.1" 仅本机）
    pub host: String,                   // 字符串类型，支持 IP 地址或域名
}

/// 应用信息
///
/// 对应 application.yml 中的 app 节点，用于启动日志和健康检查
#[derive(Debug, Deserialize, Clone)]
pub struct AppInfo {
    /// 应用名称（如 "WMS 仓库管理系统"）
    pub name: String,                   // 应用显示名称
    /// 版本号（如 "1.0.0"）
    pub version: String,                // 语义化版本号，遵循 semver 规范
    /// 应用描述
    pub description: String,            // 应用功能简述
}

/// 数据库配置
///
/// 对应 application.yml 中的 database 节点
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    /// 数据库连接 URL（如 "sqlite:./wms.db"），为空时使用跨平台默认路径
    #[serde(default)]           // YAML 中未配置 url 时默认为空字符串
    pub url: String,                    // 为空字符串时，connect_db_with_config 会自动构建跨平台路径
    /// 连接池配置
    #[serde(default)]           // YAML 中未配置 pool 时使用 PoolConfig::default()
    pub pool: PoolConfig,               // 包含最大/最小连接数、超时时间等
}

/// 连接池配置
///
/// 对应 application.yml 中的 database.pool 节点
/// SQLite 本身不支持多连接并发写入，但 SeaORM 仍需要连接池配置
#[derive(Debug, Deserialize, Clone)]
pub struct PoolConfig {
    /// 最大连接数（如 10）
    pub max_connections: u32,           // 连接池上限，防止数据库连接数过多
    /// 最小连接数（如 5），启动时预创建
    pub min_connections: u32,           // 连接池下限，保持热连接减少延迟
    /// 连接超时时间（秒），超时未获取连接则报错
    #[allow(dead_code)]
    pub connect_timeout: u64,           // 获取连接的最大等待时间
    /// 空闲连接超时时间（秒），超时后自动回收
    #[allow(dead_code)]
    pub idle_timeout: u64,              // 空闲连接的最大存活时间，释放无用资源
}

/// 连接池默认配置
///
/// 当 application.yml 未配置 pool 节点时使用
impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10,       // 最大 10 个连接
            min_connections: 5,        // 最少保持 5 个连接
            connect_timeout: 30,       // 连接超时 30 秒
            idle_timeout: 600,         // 空闲超时 10 分钟（600 秒）
        }
    }
}

/// JWT 配置
///
/// 对应 application.yml 中的 jwt 节点
#[derive(Debug, Deserialize, Clone)]
pub struct JwtConfig {
    /// JWT 签名密钥（必须保密，生产环境应通过环境变量注入）
    pub secret: String,                 // HMAC-SHA256 密钥，长度建议至少 32 字节
    /// Token 有效期（秒），当前代码中未使用此字段，由 AuthService 硬编码 86400
    #[allow(dead_code)]
    pub expiration: u64,                // 单位为秒，如 86400 = 24小时
    /// Token 签发者标识（如 "wms-system"）
    #[serde(default = "default_issuer")]  // YAML 中未配置时使用默认值
    #[allow(dead_code)]
    pub issuer: String,                 // 用于 Token 的 iss 字段，标识签发方
}

/// JWT 签发者默认值
fn default_issuer() -> String {
    "wms-system".to_string()            // 默认签发者标识
}

/// 日志配置
///
/// 对应 application.yml 中的 logging 节点
#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    /// 日志级别（TRACE/DEBUG/INFO/WARN/ERROR）
    pub level: String,                  // 字符串类型，运行时转为 tracing::Level 枚举
    /// 日志输出格式（如 "pretty" 控制台友好格式）
    #[serde(default = "default_log_format")]  // YAML 中未配置时使用默认值
    #[allow(dead_code)]
    pub format: String,                 // 支持 "pretty"、"json"、"compact" 等格式
}

/// 日志格式默认值
fn default_log_format() -> String {
    "pretty".to_string()  // pretty 格式：彩色、多行，适合开发环境
}

/// CORS 跨域配置
///
/// 对应 application.yml 中的 cors 节点
/// 控制浏览器跨域请求策略
#[derive(Debug, Deserialize, Clone)]
pub struct CorsConfig {
    /// 允许的来源域名（"*" 表示允许所有来源，生产环境应指定具体域名）
    pub allow_origins: String,          // 对应 Access-Control-Allow-Origin 响应头
    /// 允许的 HTTP 方法（如 "GET,POST,PUT,DELETE"）
    #[allow(dead_code)]
    pub allow_methods: String,          // 对应 Access-Control-Allow-Methods 响应头
    /// 允许的请求头（如 "Content-Type,Authorization"）
    #[allow(dead_code)]
    pub allow_headers: String,          // 对应 Access-Control-Allow-Headers 响应头
}

/// 业务配置
///
/// 对应 application.yml 中的 business 节点
/// 包含分页、库存预警、订单超时等业务参数
#[derive(Debug, Deserialize, Clone, Default)]
pub struct BusinessConfig {
    /// 分页配置
    #[serde(default)]
    #[allow(dead_code)]
    pub pagination: PaginationConfig,   // 控制列表查询的默认每页条数和上限
    /// 库存预警配置
    #[serde(default)]
    #[allow(dead_code)]
    pub stock: StockConfig,             // 库存低于阈值时触发预警
    /// 订单超时配置
    #[serde(default)]
    #[allow(dead_code)]
    pub order: OrderConfig,             // 订单超时后自动完成
}

/// 分页配置
///
/// 控制列表查询的默认分页参数
#[derive(Debug, Deserialize, Clone)]
pub struct PaginationConfig {
    /// 默认每页条数
    #[allow(dead_code)]
    pub default_page_size: i32,         // 未指定 pageSize 时的默认值
    /// 每页最大条数（防止一次查询过多数据）
    #[allow(dead_code)]
    pub max_page_size: i32,             // 客户端传入超大 pageSize 时会被截断为此值
}

/// 分页默认配置
impl Default for PaginationConfig {
    fn default() -> Self {
        Self {
            default_page_size: 10,     // 默认每页 10 条
            max_page_size: 100,        // 最多每页 100 条，避免一次性加载过多数据
        }
    }
}

/// 库存预警配置
///
/// 当库存数量低于阈值时触发预警
#[derive(Debug, Deserialize, Clone)]
pub struct StockConfig {
    /// 库存预警阈值（如 10 表示库存低于 10 时预警）
    #[allow(dead_code)]
    pub warning_threshold: i32,         // 阈值以下触发库存不足提醒
}

/// 库存预警默认配置
impl Default for StockConfig {
    fn default() -> Self {
        Self {
            warning_threshold: 10,     // 库存低于 10 时预警
        }
    }
}

/// 订单配置
///
/// 控制订单的自动完成等行为
#[derive(Debug, Deserialize, Clone)]
pub struct OrderConfig {
    /// 订单自动完成时间（小时），超时后自动标记为已完成
    #[allow(dead_code)]
    pub auto_complete_hours: u64,       // 单位为小时
}

/// 订单默认配置
impl Default for OrderConfig {
    fn default() -> Self {
        Self {
            auto_complete_hours: 24,   // 24 小时后自动完成
        }
    }
}

impl AppConfig {
    /// 从 YAML 文件加载配置
    ///
    /// # 参数
    /// * `path` - 配置文件路径（如 "application.yml"）
    ///
    /// # 返回
    /// * `Ok(AppConfig)` - 解析成功
    /// * `Err(Box<dyn Error>)` - 文件不存在或 YAML 格式错误
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;                    // 读取文件全部内容为字符串，失败则向上抛出 IO 错误
        let config: AppConfig = serde_yaml::from_str(&content)?;   // 将 YAML 反序列化为 AppConfig 结构体，失败则抛出解析错误
        Ok(config)                                                  // 返回解析后的配置实例
    }

    /// 加载配置并支持环境变量覆盖
    ///
    /// 对应 Spring Boot 的 @Value + @RefreshScope
    /// 环境变量优先级高于 application.yml，适合 Docker/K8s 部署时动态注入配置
    pub fn load_with_env() -> Result<Self, Box<dyn std::error::Error>> {
        // 先从 application.yml 加载基础配置
        let mut config = Self::from_file("application.yml")?;      // mut 声明为可变，后续环境变量需要修改字段

        // 环境变量覆盖：DATABASE_URL（数据库连接地址）
        if let Ok(db_url) = env::var("DATABASE_URL") {             // 尝试读取环境变量，Ok 说明存在
            config.database.url = db_url;    // 覆盖 yml 中的 database.url
        }

        // 环境变量覆盖：JWT_SECRET（JWT 签名密钥，生产环境必须通过环境变量注入）
        if let Ok(jwt_secret) = env::var("JWT_SECRET") {          // 尝试读取环境变量，Ok 说明存在
            config.jwt.secret = jwt_secret;  // 覆盖 yml 中的 jwt.secret，生产环境安全要求
        }

        // 环境变量覆盖：PORT（服务监听端口）
        if let Ok(port) = env::var("PORT") {                       // 尝试读取环境变量，Ok 说明存在
            config.server.port = port.parse().unwrap_or(config.server.port);  // 解析为 u16，失败则保留 yml 中的值
        }

        Ok(config)                                                 // 返回最终配置（可能已被环境变量覆盖）
    }

    /// 获取完整的服务器地址（host:port 格式）
    #[allow(dead_code)]
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)  // 拼接为 "0.0.0.0:8080" 格式，供 TCP 监听使用
    }

    /// 获取日志级别
    ///
    /// 将配置文件中的字符串（如 "INFO"）转换为 tracing::Level 枚举
    pub fn log_level(&self) -> tracing::Level {
        match self.logging.level.to_uppercase().as_str() {  // 转大写后匹配，配置文件不区分大小写
            "TRACE" => tracing::Level::TRACE,    // 最详细级别：包含所有调试信息
            "DEBUG" => tracing::Level::DEBUG,    // 调试级别：包含调试信息
            "INFO" => tracing::Level::INFO,      // 信息级别：一般运行信息（默认）
            "WARN" => tracing::Level::WARN,      // 警告级别：潜在问题
            "ERROR" => tracing::Level::ERROR,    // 错误级别：严重问题
            _ => tracing::Level::INFO,           // 未识别的级别默认为 INFO，保证程序不会因配置错误而崩溃
        }
    }
}
