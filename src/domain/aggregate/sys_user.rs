// ============================================
// 系统用户领域实体（富领域模型）
// DDD 聚合根（Aggregate Root）
// 封装用户身份验证和状态管理
// ============================================

use chrono::{DateTime, Utc}; // 时间类型
use crate::domain::{DomainError, DomainResult}; // 领域错误
use super::account_status::AccountStatus; // 账号状态值对象
use super::user_role::UserRole; // 用户角色值对象
use super::phone::Phone; // 电话号码值对象

/// DDD 聚合根 - 系统用户领域实体
/// 私有字段 + getter，封装密码安全、状态管理
#[derive(Clone, Debug)]
pub struct SysUser {
    user_id: String,                      // 用户编号（业务主键）
    username: String,                     // 用户名（登录账号）
    password: String,                     // 密码（私有，安全敏感字段，不可直接读取）
    real_name: Option<String>,            // 真实姓名（可选）
    role: UserRole,                       // 用户角色（值对象，替代原 String）
    phone: Option<Phone>,                 // 手机号（值对象，可选）
    token: Option<String>,                // 当前 JWT Token（私有，安全敏感字段）
    create_time: Option<DateTime<Utc>>,   // 创建时间
    status: AccountStatus,                // 账号状态（值对象，替代原 Option<i8>）
}

impl SysUser {
    /// 工厂方法：创建新用户
    /// 自动设置 status = Active，create_time = 当前时间
    /// 初始无 Token
    #[allow(dead_code)]
    pub fn new(
        user_id: String,          // 用户编号（业务主键）
        username: String,         // 用户名（登录账号）
        password: String,         // 密码（明文，应在上层先做哈希处理）
        real_name: Option<String>,// 真实姓名
        role: UserRole,           // 用户角色（值对象，已通过验证）
        phone: Option<Phone>,     // 手机号（值对象，可选）
    ) -> DomainResult<Self> {
        if username.trim().is_empty() {  // 用户名不能为空
            return Err(DomainError::InvalidOperation("用户名不能为空".to_string()));
        }
        if password.trim().is_empty() {  // 密码不能为空
            return Err(DomainError::InvalidOperation("密码不能为空".to_string()));
        }
        Ok(Self {
            user_id,                     // 用户编号
            username: username.trim().to_string(), // 用户名（去除首尾空格）
            password,                    // 密码
            real_name,                   // 真实姓名
            role,                        // 用户角色
            phone,                       // 手机号
            token: None,                 // 新用户无 Token
            create_time: Some(Utc::now()), // 创建时间 = 当前 UTC
            status: AccountStatus::Active, // 新用户默认启用
        })
    }

    /// 从仓储重建聚合根（用于从数据库加载已有数据）
    /// 不走工厂方法，因为已有数据已经满足业务约束
    pub fn reconstitute(
        user_id: String,                  // 用户编号
        username: String,                 // 用户名
        password: String,                 // 密码
        real_name: Option<String>,        // 真实姓名
        role: UserRole,                   // 用户角色
        phone: Option<Phone>,             // 手机号
        token: Option<String>,            // JWT Token
        create_time: Option<DateTime<Utc>>, // 创建时间
        status: AccountStatus,            // 账号状态
    ) -> Self {
        Self {
            user_id,                      // 用户编号
            username,                     // 用户名
            password,                     // 密码
            real_name,                    // 真实姓名
            role,                         // 用户角色
            phone,                        // 手机号
            token,                        // Token
            create_time,                  // 创建时间
            status,                       // 账号状态
        }
    }

    // === Getter 方法（只读访问，封装字段） ===

    /// 获取用户编号（只读）
    pub fn user_id(&self) -> &str {
        &self.user_id                     // 用户编号
    }

    /// 获取用户名（只读）
    pub fn username(&self) -> &str {
        &self.username                    // 登录账号名
    }

    /// 获取密码（只读，仅供认证服务验证）
    #[allow(dead_code)]
    pub fn password(&self) -> &str {
        &self.password                    // 密码（调用方负责安全处理）
    }

    /// 获取真实姓名（只读）
    pub fn real_name(&self) -> &Option<String> {
        &self.real_name                   // 真实姓名（可能为空）
    }

    /// 获取用户角色（只读）
    pub fn role(&self) -> &UserRole {
        &self.role                        // 用户角色值对象
    }

    /// 获取手机号（只读）
    #[allow(dead_code)]
    pub fn phone(&self) -> &Option<Phone> {
        &self.phone                       // 手机号值对象（可能为空）
    }

    /// 获取 JWT Token（只读）
    #[allow(dead_code)]
    pub fn token(&self) -> &Option<String> {
        &self.token                       // Token（可能为空）
    }

    /// 获取创建时间（只读）
    #[allow(dead_code)]
    pub fn create_time(&self) -> &Option<DateTime<Utc>> {
        &self.create_time                 // 创建时间
    }

    /// 获取账号状态（只读）
    #[allow(dead_code)]
    pub fn status(&self) -> AccountStatus {
        self.status                       // 账号状态（值对象，Copy）
    }

    /// 判断账号是否处于启用状态（领域查询）
    /// 用于登录前的状态校验
    pub fn is_active(&self) -> bool {
        self.status.is_active()           // 委托值对象方法
    }

    /// 验证密码（领域行为）
    /// 将密码比对逻辑封装在领域实体中
    /// 当前为明文比对（生产环境应改为哈希比对）
    pub fn verify_password(&self, password: &str) -> bool {
        self.password == password         // 密码比对（明文，待升级为 bcrypt/argon2）
    }

    /// 设置 JWT Token（用于登录/登出时更新令牌）
    #[allow(dead_code)]
    pub fn set_token(&mut self, token: Option<String>) {
        self.token = token;               // 更新 Token
    }

    /// 更新用户基本信息（领域行为）
    #[allow(dead_code)]
    pub fn update(&mut self, real_name: Option<String>, role: UserRole, phone: Option<Phone>) -> DomainResult<()> {
        self.real_name = real_name;        // 更新真实姓名
        self.role = role;                  // 更新角色（值对象，已通过验证）
        self.phone = phone;                // 更新手机号（值对象，可选）
        Ok(())
    }
}
