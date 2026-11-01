// ============================================
// 用户仓储接口
// ============================================

use async_trait::async_trait;
use super::RepositoryResult;
use crate::domain::aggregate::sys_user::SysUser;

/// 用户仓储 trait（Domain 层接口，Infrastructure 层实现）
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, user_id: &str) -> RepositoryResult<Option<SysUser>>;                  // 根据用户ID查询单条
    async fn find_by_ids(&self, ids: &[String]) -> RepositoryResult<Vec<SysUser>>;                   // 批量查询（用于名称关联）
    async fn find_by_username(&self, username: &str) -> RepositoryResult<Option<SysUser>>;           // 根据用户名查询（用于登录）
    async fn update_token(&self, user_id: &str, token: &str) -> RepositoryResult<()>;                // 更新用户Token（登录/登出）
}
