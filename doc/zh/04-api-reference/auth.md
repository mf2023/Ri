<div align="center">

# Auth API参考

**Version: 0.1.8**

**Last modified date: 2026-02-20**

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
| `permission_manager()` | 获取权限管理器 | 无 | `Arc<RwLock<DMSCPermissionManager>>` |
| `session_manager()` | 获取会话管理器 | 无 | `Arc<DMSCSessionManager>` |
| `oauth_manager()` | 获取OAuth管理器 | 无 | `Arc<RwLock<DMSCOAuthManager>>` |

#### 使用示例

```rust
use dmsc::prelude::*;
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

let jwt = jwt_manager.generate_token("user123", vec!["admin"], vec!["read", "write"])?;

// 验证JWT令牌
let claims = jwt_manager.validate_token(&jwt)?;

// 通过module访问权限管理器
let permission_manager = ctx.module::<DMSCAuthModule>().await?
    .permission_manager();

// 检查权限
let has_access = permission_manager.check_permission(&user.role, "admin").await?;

// OAuth2流程
let oauth_manager = ctx.module::<DMSCAuthModule>().await?
    .oauth_manager();
    
let auth_url = oauth_manager.write().await
    .get_auth_url("github", "state123").await?;
```

### DMSCJWTManager

JWT令牌管理器，负责JWT的生成和验证。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(secret, expiry_secs)` | 创建JWT管理器 | `secret: String`, `expiry_secs: u64` | `Self` |
| `generate_token(user_id, roles, permissions)` | 生成JWT令牌 | `user_id: &str`, `roles: Vec<String>`, `permissions: Vec<String>` | `DMSCResult<String>` |
| `validate_token(token)` | 验证JWT令牌 | `token: &str` | `DMSCResult<DMSCJWTClaims>` |
| `get_token_expiry()` | 获取令牌过期时间 | 无 | `u64` |
| `get_secret()` | 获取密钥 | 无 | `&str` |

### DMSCJWTClaims

JWT声明结构，包含令牌负载。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `sub` | `String` | 主题声明 - 用户标识符 |
| `exp` | `u64` | 过期时间（Unix时间戳） |
| `iat` | `u64` | 签发时间（Unix时间戳） |
| `roles` | `Vec<String>` | RBAC角色标识符列表 |
| `permissions` | `Vec<String>` | 细粒度访问控制的权限标识符列表 |

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
| `enabled` | `bool` | 是否启用认证 | `true` |
| `jwt_secret` | `String` | JWT签名密钥 | 自动生成 |
| `jwt_expiry_secs` | `u64` | JWT过期时间（秒） | 3600 |
| `session_timeout_secs` | `u64` | 会话超时时间（秒） | 86400 |
| `oauth_providers` | `Vec<String>` | OAuth提供商列表 | 空列表 |
| `enable_api_keys` | `bool` | 是否启用API密钥认证 | `true` |
| `enable_session_auth` | `bool` | 是否启用会话认证 | `true` |
| `oauth_cache_backend_type` | `DMSCCacheBackendType` | OAuth缓存后端类型 | `Memory` |
| `oauth_cache_redis_url` | `String` | OAuth缓存Redis URL | `"redis://127.0.0.1:6379"` |

#### 使用示例

```rust
let auth_config = DMSCAuthConfig {
    enabled: true,
    jwt_secret: "your-secret-key".to_string(),
    jwt_expiry_secs: 3600,
    session_timeout_secs: 86400,
    oauth_providers: vec![],
    enable_api_keys: true,
    enable_session_auth: true,
    #[cfg(feature = "cache")]
    oauth_cache_backend_type: crate::cache::DMSCCacheBackendType::Memory,
    #[cfg(feature = "cache")]
    oauth_cache_redis_url: "redis://127.0.0.1:6379".to_string(),
};
```

### DMSCOAuthManager

OAuth管理器，用于处理多个身份提供商。

**重要说明**：OAuth令牌交换、用户信息获取、令牌刷新和令牌撤销操作需要启用`http_client`特性。如果没有启用此特性，这些方法将返回错误。

要启用`http_client`特性，请在`Cargo.toml`中添加：

```toml
[dependencies]
dmsc = { version = "0.1.8", features = ["http_client"] }
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 | 所需特性 |
|:--------|:-------------|:--------|:--------|:--------|
| `register_provider(provider)` | 注册OAuth提供商 | `provider: DMSCOAuthProvider` | `DMSCResult<()>` | 无 |
| `get_provider(provider_id)` | 根据ID获取提供商 | `provider_id: &str` | `DMSCResult<Option<DMSCOAuthProvider>>` | 无 |
| `get_auth_url(provider_id, state)` | 获取认证URL | `provider_id: &str`, `state: &str` | `DMSCResult<Option<String>>` | 无 |
| `exchange_code_for_token(provider_id, code, redirect_uri)` | 用授权码交换令牌 | `provider_id: &str`, `code: &str`, `redirect_uri: &str` | `DMSCResult<Option<DMSCOAuthToken>>` | `http_client` |
| `get_user_info(provider_id, access_token)` | 获取用户信息 | `provider_id: &str`, `access_token: &str` | `DMSCResult<Option<DMSCOAuthUserInfo>>` | `http_client` |
| `refresh_token(provider_id, refresh_token)` | 刷新访问令牌 | `provider_id: &str`, `refresh_token: &str` | `DMSCResult<Option<DMSCOAuthToken>>` | `http_client` |
| `revoke_token(provider_id, access_token)` | 撤销访问令牌 | `provider_id: &str`, `access_token: &str` | `DMSCResult<bool>` | `http_client` |
| `list_providers()` | 列出所有提供商 | 无 | `DMSCResult<Vec<DMSCOAuthProvider>>` | 无 |

### DMSCOAuthProvider

OAuth提供商配置结构。

#### 字段

| 字段 | 类型 | 描述 | 是否必需 |
|:--------|:--------|:-------------|:--------|
| `id` | `String` | 唯一提供商标识符 | 是 |
| `name` | `String` | 人类可读的提供商名称 | 是 |
| `client_id` | `String` | OAuth客户端ID | 是 |
| `client_secret` | `String` | OAuth客户端密钥 | 是 |
| `auth_url` | `String` | 授权端点URL | 是 |
| `token_url` | `String` | 令牌端点URL | 是 |
| `user_info_url` | `String` | 用户信息端点URL | 是 |
| `scopes` | `Vec<String>` | OAuth作用域 | 是 |
| `enabled` | `bool` | 是否启用提供商 | 是 |
| `redirect_uri` | `Option<String>` | OAuth回调的重定向URI | 否（默认为`http://localhost:8080/auth/callback`） |

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
