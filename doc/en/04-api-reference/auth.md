<div align="center">

# Auth API Reference

**Version: 0.1.7**

**Last modified date: 2026-02-13**

The auth module provides authentication and authorization functionality, supporting JWT, OAuth2, and role-based access control.

## Module Overview

</div>

The auth module includes the following sub-modules:

- **jwt**: JWT token generation and verification
- **oauth**: OAuth2 authorization flow
- **permissions**: Permission management and RBAC
- **session**: Session management

<div align="center">

## Core Components

</div>

### DMSCAuthModule

The main interface for the authentication module, providing unified access to authentication services.

**Note**: This class provides access to authentication sub-modules. For specific JWT and permission operations, please refer to the corresponding Manager classes.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `jwt_manager()` | Get JWT manager | None | `Arc<DMSCJWTManager>` |
| `permission_manager()` | Get permission manager | None | `Arc<RwLock<DMSCPermissionManager>>` |
| `session_manager()` | Get session manager | None | `Arc<DMSCSessionManager>` |
| `oauth_manager()` | Get OAuth manager | None | `Arc<RwLock<DMSCOAuthManager>>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    role: String,
}

// Access JWT manager through module
let jwt_manager = ctx.module::<DMSCAuthModule>().await?
    .jwt_manager();
    
// Generate JWT token
let user = User {
    id: 1,
    username: "test_user".to_string(),
    role: "admin".to_string(),
};

let jwt = jwt_manager.generate_token("user123", vec!["admin"], vec!["read", "write"])?;

// Verify JWT token
let claims = jwt_manager.validate_token(&jwt)?;

// Access permission manager through module
let permission_manager = ctx.module::<DMSCAuthModule>().await?
    .permission_manager();

// Check permission
let has_access = permission_manager.check_permission(&user.role, "admin").await?;

// OAuth2 flow
let oauth_manager = ctx.module::<DMSCAuthModule>().await?
    .oauth_manager();
    
let auth_url = oauth_manager.write().await
    .get_auth_url("github", "state123").await?;
```

### DMSCJWTManager

JWT token manager, responsible for JWT generation and verification.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(secret, expiry_secs)` | Create JWT manager | `secret: String`, `expiry_secs: u64` | `Self` |
| `generate_token(user_id, roles, permissions)` | Generate JWT token | `user_id: &str`, `roles: Vec<String>`, `permissions: Vec<String>` | `DMSCResult<String>` |
| `validate_token(token)` | Verify JWT token | `token: &str` | `DMSCResult<DMSCJWTClaims>` |
| `get_token_expiry()` | Get token expiry time | None | `u64` |
| `get_secret()` | Get secret key | None | `&str` |

### DMSCJWTClaims

JWT claims structure containing the token payload.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `sub` | `String` | Subject claim - user identifier |
| `exp` | `u64` | Expiration time (Unix timestamp) |
| `iat` | `u64` | Issued at time (Unix timestamp) |
| `roles` | `Vec<String>` | List of role identifiers for RBAC |
| `permissions` | `Vec<String>` | List of permission identifiers for fine-grained access control |

### DMSCPermissionManager

Permission manager, responsible for role permission checking and resource access control.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create permission manager | None | `Self` |
| `check_permission(role, permission)` | Check role permission | `role: &str`, `permission: &str` | `DMSCResult<bool>` |
| `check_resource_permission(role, resource_type, resource_id, action)` | Check resource permission | `role: &str`, `resource_type: &str`, `resource_id: &str`, `action: &str` | `DMSCResult<bool>` |

### DMSCAuthConfig

Authentication module configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:--------|:-------------|:--------|
| `enabled` | `bool` | Whether authentication is enabled | `true` |
| `jwt_secret` | `String` | JWT signing key | Auto-generated |
| `jwt_expiry_secs` | `u64` | JWT expiration time (seconds) | 3600 |
| `session_timeout_secs` | `u64` | Session timeout (seconds) | 86400 |
| `oauth_providers` | `Vec<String>` | List of OAuth providers | Empty list |
| `enable_api_keys` | `bool` | Whether API key authentication is enabled | `true` |
| `enable_session_auth` | `bool` | Whether session authentication is enabled | `true` |
| `oauth_cache_backend_type` | `DMSCCacheBackendType` | OAuth cache backend type | `Memory` |
| `oauth_cache_redis_url` | `String` | OAuth cache Redis URL | `"redis://127.0.0.1:6379"` |

#### Usage Example

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

OAuth manager for handling multiple identity providers.

**Important**: OAuth token exchange, user info retrieval, token refresh, and token revocation operations require the `http_client` feature to be enabled. Without this feature, these methods will return an error.

To enable the `http_client` feature, add it to your `Cargo.toml`:

```toml
[dependencies]
dmsc = { version = "0.1.7", features = ["http_client"] }
```

#### Methods

| Method | Description | Parameters | Return Value | Feature Required |
|:--------|:-------------|:--------|:--------|:--------|
| `register_provider(provider)` | Register OAuth provider | `provider: DMSCOAuthProvider` | `DMSCResult<()>` | None |
| `get_provider(provider_id)` | Get provider by ID | `provider_id: &str` | `DMSCResult<Option<DMSCOAuthProvider>>` | None |
| `get_auth_url(provider_id, state)` | Get authentication URL | `provider_id: &str`, `state: &str` | `DMSCResult<Option<String>>` | None |
| `exchange_code_for_token(provider_id, code, redirect_uri)` | Exchange code for token | `provider_id: &str`, `code: &str`, `redirect_uri: &str` | `DMSCResult<Option<DMSCOAuthToken>>` | `http_client` |
| `get_user_info(provider_id, access_token)` | Get user information | `provider_id: &str`, `access_token: &str` | `DMSCResult<Option<DMSCOAuthUserInfo>>` | `http_client` |
| `refresh_token(provider_id, refresh_token)` | Refresh access token | `provider_id: &str`, `refresh_token: &str` | `DMSCResult<Option<DMSCOAuthToken>>` | `http_client` |
| `revoke_token(provider_id, access_token)` | Revoke access token | `provider_id: &str`, `access_token: &str` | `DMSCResult<bool>` | `http_client` |
| `list_providers()` | List all providers | None | `DMSCResult<Vec<DMSCOAuthProvider>>` | None |

### DMSCOAuthProvider

OAuth provider configuration structure.

#### Fields

| Field | Type | Description | Required |
|:--------|:--------|:-------------|:--------|
| `id` | `String` | Unique provider identifier | Yes |
| `name` | `String` | Human-readable provider name | Yes |
| `client_id` | `String` | OAuth client ID | Yes |
| `client_secret` | `String` | OAuth client secret | Yes |
| `auth_url` | `String` | Authorization endpoint URL | Yes |
| `token_url` | `String` | Token endpoint URL | Yes |
| `user_info_url` | `String` | User info endpoint URL | Yes |
| `scopes` | `Vec<String>` | OAuth scopes | Yes |
| `enabled` | `bool` | Whether provider is enabled | Yes |
| `redirect_uri` | `Option<String>` | Redirect URI for OAuth callback | No (defaults to `http://localhost:8080/auth/callback`) |

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [queue](./queue.md): Message queue module providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
