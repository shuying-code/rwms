// ============================================
// 用户仓储 SeaORM 实现
// ============================================

use async_trait::async_trait;  // 异步 trait 宏
use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, DatabaseConnection, Set};
// EntityTrait: 提供 find / find_by_id 等查询方法
// QueryFilter: WHERE 条件过滤
// ColumnTrait: 列级操作（eq, is_in 等）
// ActiveModelTrait: update 等写操作
// DatabaseConnection: SeaORM 数据库连接
// Set: ActiveModel 字段赋值包装器

use crate::domain::aggregate::sys_user::SysUser;                           // 领域实体：系统用户
use crate::domain::aggregate::user_role::UserRole;                         // 领域值对象：用户角色
use crate::domain::aggregate::account_status::AccountStatus;               // 领域值对象：账号状态
use crate::domain::aggregate::phone::Phone;                                // 领域值对象：电话
use crate::domain::repository::user_repo::UserRepository;               // 领域 trait：用户仓储接口
use crate::domain::repository::{RepositoryError, RepositoryResult};     // 仓储层错误类型与 Result 别名

use super::super::model::sys_user as sea_user;  // SeaORM 自动生成的用户表 Model 模块

/// SeaORM 实现的用户仓储
/// 持有数据库连接，实现 UserRepository trait
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,  // SeaORM 数据库连接实例
}

impl SeaOrmUserRepository {
    /// 构造函数，接收数据库连接创建仓储实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

// === ORM Model → 领域实体 转换 ===
impl From<sea_user::Model> for SysUser {
    fn from(m: sea_user::Model) -> Self {
        SysUser::reconstitute(
            m.user_id,                                                          // 用户ID，主键（字符串格式，如 UUID）
            m.username,                                                         // 登录用户名
            m.password,                                                         // 加密后的密码
            m.real_name,                                                        // 真实姓名
            UserRole::from_str(&m.role).unwrap_or(UserRole::Operator),          // 用户角色（String → UserRole 枚举）
            m.phone.map(|s| Phone::new(s).unwrap()),                            // 联系电话（String → Phone 值对象）
            m.token,                                                            // 登录令牌（JWT Token），可选
            m.create_time,                                                      // 账号创建时间
            AccountStatus::from_i8(m.status.unwrap_or(1)),                      // 账号状态（Option<i8> → AccountStatus 枚举）
        )
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    /// 根据用户ID查询单条用户记录
    /// 返回 Option<SysUser>，不存在则返回 None
    async fn find_by_id(&self, user_id: &str) -> RepositoryResult<Option<SysUser>> {
        sea_user::Entity::find_by_id(user_id)        // SELECT * FROM sys_user WHERE user_id = ?
            .one(&self.db).await                      // 取至多一条记录
            .map(|opt| opt.map(SysUser::from))        // 若存在则转换为领域实体
            .map_err(RepositoryError::from)           // SeaORM 错误 → 仓储层错误
    }

    /// 根据多个用户ID批量查询
    /// 对应 SQL: SELECT * FROM sys_user WHERE user_id IN (...)
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<SysUser>> {
        if ids.is_empty() { return Ok(vec![]); }  // 空ID列表直接返回空结果，避免无效 SQL
        sea_user::Entity::find()
            .filter(sea_user::Column::UserId.is_in(ids.to_vec()))  // WHERE user_id IN (...)
            .all(&self.db).await
            .map(|list| list.into_iter().map(SysUser::from).collect())
            .map_err(RepositoryError::from)
    }

    /// 根据用户名查询用户记录
    /// 用于登录验证，用户名应具有唯一约束
    /// 对应 SQL: SELECT * FROM sys_user WHERE username = ? LIMIT 1
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<SysUser>> {
        sea_user::Entity::find()
            .filter(sea_user::Column::Username.eq(username))  // WHERE username = ?
            .one(&self.db).await                               // 取至多一条记录
            .map(|opt| opt.map(SysUser::from))                // 若存在则转换为领域实体
            .map_err(RepositoryError::from)
    }

    /// 更新用户登录令牌（Token）
    /// 登录成功后调用，将新生成的 JWT Token 保存到数据库
    /// 先根据 user_id 查询用户，找到后更新 token 字段
    async fn update_token(&self, user_id: &str, token: &str) -> RepositoryResult<()> {
        if let Some(user) = sea_user::Entity::find_by_id(user_id).one(&self.db).await.map_err(RepositoryError::from)? {
            // 找到用户记录，转换为 ActiveModel 以进行部分字段更新
            let mut active: sea_user::ActiveModel = user.into();  // Model → ActiveModel
            active.token = Set(Some(token.to_string()));          // 仅更新 token 字段，其它字段保持不变
            active.update(&self.db).await.map_err(RepositoryError::from)?;  // UPDATE sys_user SET token = ? WHERE user_id = ?
        }
        // 若未找到用户则静默跳过，不报错
        Ok(())
    }
}
