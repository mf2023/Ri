<div align="center">

# Auth API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `generate_jwt(payload)` | 生成JWT令牌 | `payload: impl Serialize` | `DMSCResult<String>` |
| `verify_jwt(token)` | 验证JWT令牌 | `token: &str` | `DMSCResult<T>` |
| `generate_jwt_with_refresh(payload)` | 生成JWT和刷新令牌 | `payload: impl Serialize` | `DMSCResult<(String, String)>` |
| `refresh_jwt(refresh_token)` | 使用刷新令牌获取新JWT | `refresh_token: &str` | `DMSCResult<String>` |
| `check_permission(role, permission)` | 检查角色权限 | `role: &str`, `permission: &str` | `DMSCResult<bool>` |
| `check_resource_permission(role, resource_type, resource_id, action)` | 检查资源权限 | `role: &str`, `resource_type: &str`, `resource_id: &str`, `action: &str` | `DMSCResult<bool>` |
| `oauth_config(provider)` | 获取OAuth配置 | `provider: &str` | `DMSCResult<DMSCOAuthConfig>` |
| `oauth_authorization_url(provider, state)` | 生成OAuth授权URL | `provider: &str`, `state: &str` | `DMSCResult<String>` |
| `oauth_exchange_token(provider, code)` | 交换OAuth令牌 | `provider: &str`, `code: &str` | `DMSCResult<String>` |
| `oauth_get_user_info(provider, token)` | 获取OAuth用户信息 | `provider: &str`, `token: &str` | `DMSCResult<DMSCUserInfo>` |

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

// 生成JWT令牌
let user = User {
    id: 1,
    username: "test_user".to_string(),
    role: "admin".to_string(),
};

let jwt = ctx.auth().generate_jwt(&user).await?;

// 验证JWT令牌
let decoded_user: User = ctx.auth().verify_jwt(&jwt).await?;

// 检查权限
let has_access = ctx.auth().check_permission(&user.role, "admin").await?;

// OAuth2流程
let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;
let token = ctx.auth().oauth_exchange_token("github", "code123").await?;
let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
```

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
    jwt_issuer: "my-app".to_string(),
    jwt_expires_in: 7200,
    jwt_refresh_expires_in: 172800,
    oauth_providers: {
        let mut providers = HashMap::new();
        providers.insert("github".to_string(), DMSCOAuthConfig {
            client_id: "your-client-id".to_string(),
            client_secret: "your-client-secret".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
            scope: vec!["user:email".to_string()],
        });
        providers
    },
    permission_rules: {
        let mut rules = HashMap::new();
        rules.insert("admin".to_string(), vec!["create".to_string(), "read".to_string(), "update".to_string(), "delete".to_string()]);
        rules.insert("user".to_string(), vec!["read".to_string()]);
        rules
    },
};
```

### DMSCOAuthConfig

OAuth配置结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `client_id` | `String` | OAuth客户端ID |
| `client_secret` | `String` | OAuth客户端密钥 |
| `redirect_uri` | `String` | 重定向URI |
| `scope` | `Vec<String>` | 权限范围 |
| `authorization_url` | `Option<String>` | 授权URL（可选） |
| `token_url` | `Option<String>` | 令牌URL（可选） |

### DMSCUserInfo

用户信息结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `id` | `String` | 用户ID |
| `username` | `String` | 用户名 |
| `email` | `Option<String>` | 邮箱 |
| `avatar_url` | `Option<String>` | 头像URL |
| `provider` | `String` | 提供商 |

<div align="center">

## JWT功能

</div>

### 令牌生成

```rust
// 生成标准JWT
let jwt = ctx.auth().generate_jwt(&user).await?;

// 生成带过期时间的JWT
let jwt = ctx.auth().generate_jwt_with_expiry(&user, 7200).await?;

// 生成刷新令牌
let (access_token, refresh_token) = ctx.auth().generate_jwt_with_refresh(&user).await?;
```

### 令牌验证

```rust
// 验证JWT并解码
let user: User = ctx.auth().verify_jwt(&token).await?;

// 验证JWT过期时间
let is_valid = ctx.auth().verify_jwt_expiry(&token).await?;

// 刷新访问令牌
let new_access_token = ctx.auth().refresh_jwt(&refresh_token).await?;
```

<div align="center">

## OAuth2功能

</div>

### 支持的提供商

- **GitHub**: GitHub OAuth2
- **Google**: Google OAuth2
- **Microsoft**: Microsoft OAuth2
- **自定义**: 支持自定义OAuth2提供商

### 授权流程

```rust
// 1. 生成授权URL
let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;

// 2. 用户授权后，交换令牌
let token = ctx.auth().oauth_exchange_token("github", "code123").await?;

// 3. 获取用户信息
let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
```

<div align="center">

## 权限管理

</div>

### RBAC模型

```rust
// 定义角色权限
let role_permissions = vec![
    ("admin", vec!["create", "read", "update", "delete"]),
    ("editor", vec!["create", "read", "update"]),
    ("viewer", vec!["read"]),
];

// 检查角色权限
let can_create = ctx.auth().check_permission("admin", "create").await?;
let can_delete = ctx.auth().check_permission("editor", "delete").await?;
```

### 资源权限

```rust
// 检查特定资源的权限
let can_edit_post = ctx.auth().check_resource_permission(
    "user", "post", "123", "edit"
).await?;

// 检查批量权限
let permissions = vec!["read", "write", "delete"];
let results = ctx.auth().check_permissions("admin", &permissions).await?;
```

<div align="center">

## 会话管理

</div>

### 会话创建

```rust
// 创建会话
let session_id = ctx.auth().create_session(&user).await?;

// 设置会话数据
ctx.auth().set_session_data(&session_id, "key", "value").await?;
```

### 会话验证

```rust
// 验证会话
let is_valid = ctx.auth().validate_session(&session_id).await?;

// 获取会话数据
let value = ctx.auth().get_session_data(&session_id, "key").await?;

// 销毁会话
ctx.auth().destroy_session(&session_id).await?;
```

<div align="center">

## 安全配置

</div>

### 密钥管理

```rust
// 使用环境变量存储密钥
let jwt_secret = std::env::var("DMSC_JWT_SECRET")?;
let oauth_client_secret = std::env::var("DMSC_OAUTH_CLIENT_SECRET")?;

// 定期轮换密钥
ctx.auth().rotate_jwt_secret().await?;
```

### 安全策略

```rust
// 设置密码策略
ctx.auth().set_password_policy(PasswordPolicy {
    min_length: 8,
    require_uppercase: true,
    require_lowercase: true,
    require_numbers: true,
    require_special_chars: true,
}).await?;

// 启用双因素认证
ctx.auth().enable_2fa(&user_id).await?;
```

<div align="center">

## 错误处理

</div>

### 认证错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `INVALID_TOKEN` | 无效的令牌 |
| `TOKEN_EXPIRED` | 令牌已过期 |
| `INSUFFICIENT_PERMISSIONS` | 权限不足 |
| `OAUTH_ERROR` | OAuth认证错误 |
| `SESSION_INVALID` | 会话无效 |

### 错误处理示例

```rust
match ctx.auth().verify_jwt::<User>(&token).await {
    Ok(user) => {
        // 验证成功
        ctx.logger().info("auth", &format!("User authenticated: {}", user.username))?;
    }
    Err(DMSCError { code, .. }) if code == "TOKEN_EXPIRED" => {
        // 令牌过期，尝试刷新
        let new_token = ctx.auth().refresh_jwt(&refresh_token).await?;
    }
    Err(e) => {
        // 其他错误
        ctx.logger().error("auth", &format!("Authentication failed: {}", e))?;
        return Err(e);
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **安全存储密钥**: 使用环境变量或密钥管理服务存储JWT密钥和OAuth凭证
2. **合理设置过期时间**: 根据应用需求设置合适的令牌过期时间
3. **使用HTTPS**: 在生产环境中始终使用HTTPS传输认证信息
4. **实现令牌刷新**: 对于长期运行的应用，实现令牌刷新机制
5. **定期轮换密钥**: 定期轮换JWT密钥和OAuth凭证
6. **记录认证事件**: 记录所有认证和授权事件，便于审计

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置
- [cache](./cache.md): 缓存模块，提供多后端缓存抽象，缓存用户会话和权限数据
- [database](./database.md): 数据库模块，提供用户数据持久化和查询功能
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [security](./security.md): 安全模块，提供加密、哈希和验证功能
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据