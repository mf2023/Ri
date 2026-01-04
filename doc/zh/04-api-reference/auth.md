<div align="center">

# Auth API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

auth模块提供认证与授权功能，支持JWT、OAuth2和基于角色的访问控制。

## 模块概述

</div>

auth模块包含以下子模块：

- **jwt**: JWT令牌生成和验证
- **oauth**: OAuth2授权流程
- **permissions**: 权限管理和RBAC
- **session**: 会话管理

<div align="center">

## 核心组件

</div>

### DMSCAuthModule

认证模块主接口，提供统一的认证服务访问。

**注意**：此类提供对各个认证子模块的访问入口，具体的JWT和权限操作请参考对应的Manager类。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `jwt_manager()` | 获取JWT管理器 | 无 | `Arc<DMSCJWTManager>` |
| `permission_manager()` | 获取权限管理器 | 无 | `Arc<DMSCPermissionManager>` |
| `session_manager()` | 获取会话管理器 | 无 | `Arc<DMSCSessionManager>` |
| `oauth_provider(provider)` | 获取OAuth提供商 | `provider: &str` | `Option<Arc<DMSCOAuthProvider>>` |

#### 使用示例

```rust
use dms::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    role: String,
}

// 通过module访问JWT管理器
let jwt_manager = ctx.module::<DMSCAuthModule>().await?
    .jwt_manager();
    
// 生成JWT令牌
let user = User {
    id: 1,
    username: "test_user".to_string(),
    role: "admin".to_string(),
};

let jwt = jwt_manager.generate_jwt(&user).await?;

// 验证JWT令牌
let decoded_user: User = jwt_manager.verify_jwt(&jwt).await?;

// 通过module访问权限管理器
let permission_manager = ctx.module::<DMSCAuthModule>().await?
    .permission_manager();

// 检查权限
let has_access = permission_manager.check_permission(&user.role, "admin").await?;

// OAuth2流程
let auth_url = ctx.module::<DMSCAuthModule>().await?
    .oauth_provider("github")
    .unwrap()
    .get_authorization_url("state123").await?;
```

### DMSCJWTManager

JWT令牌管理器，负责JWT的生成和验证。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(secret)` | 创建JWT管理器 | `secret: String` | `Self` |
| `generate_jwt(payload)` | 生成JWT令牌 | `payload: impl Serialize` | `DMSCResult<String>` |
| `verify_jwt(token)` | 验证JWT令牌 | `token: &str` | `DMSCResult<T>` |
| `generate_jwt_with_refresh(payload)` | 生成JWT和刷新令牌 | `payload: impl Serialize` | `DMSCResult<(String, String)>` |
| `refresh_jwt(refresh_token)` | 使用刷新令牌获取新JWT | `refresh_token: &str` | `DMSCResult<String>` |

### DMSCPermissionManager

权限管理器，负责角色权限检查和资源访问控制。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建权限管理器 | 无 | `Self` |
| `check_permission(role, permission)` | 检查角色权限 | `role: &str`, `permission: &str` | `DMSCResult<bool>` |
| `check_resource_permission(role, resource_type, resource_id, action)` | 检查资源权限 | `role: &str`, `resource_type: &str`, `resource_id: &str`, `action: &str` | `DMSCResult<bool>` |

### DMSCAuthConfig

认证模块配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `jwt_secret` | `String` | JWT签名密钥 | 自动生成 |
| `jwt_issuer` | `String` | JWT签发者 | "dms" |
| `jwt_expires_in` | `u64` | JWT过期时间（秒） | 3600 |
| `jwt_refresh_expires_in` | `u64` | 刷新令牌过期时间（秒） | 86400 |
| `oauth_providers` | `HashMap<String, DMSCOAuthConfig>` | OAuth提供商配置 | 空 |
| `permission_rules` | `HashMap<String, Vec<String>>` | 权限规则 | 默认规则 |

#### 使用示例

```rust
let auth_config = DMSCAuthConfig {
    jwt_secret: "your-secret-key".to_string(),
    jwt_issuer: "dms".to_string(),
    jwt_expires_in: 3600,
    jwt_refresh_expires_in: 86400,
    oauth_providers: HashMap::new(),
    permission_rules: HashMap::new(),
};
```
