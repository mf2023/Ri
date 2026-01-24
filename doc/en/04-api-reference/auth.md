<div align="center">

# Auth API Reference

**Version: 0.1.5**

**Last modified date: 2026-01-24**

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
| `permission_manager()` | Get permission manager | None | `Arc<DMSCPermissionManager>` |
| `session_manager()` | Get session manager | None | `Arc<DMSCSessionManager>` |
| `oauth_manager(provider)` | Get OAuth manager | `provider: &str` | `Option<Arc<DMSCOAuthManager>>` |

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
let auth_url = ctx.module::<DMSCAuthModule>().await?
    .oauth_manager("github")
    .unwrap()
    .get_auth_url("state123").await?;
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
| `jwt_secret` | `String` | JWT signing key | Auto-generated |
| `jwt_issuer` | `String` | JWT issuer | "dms" |
| `jwt_expiry_secs` | `u64` | JWT expiration time (seconds) | 3600 |
| `jwt_refresh_expiry_secs` | `u64` | Refresh token expiration time (seconds) | 86400 |
| `oauth_managers` | `HashMap<String, DMSCOAuthConfig>` | OAuth manager configuration | Empty |
| `permission_rules` | `HashMap<String, Vec<String>>` | Permission rules | Default rules |

#### Usage Example

```rust
let auth_config = DMSCAuthConfig {
    jwt_secret: "your-secret-key".to_string(),
    jwt_issuer: "dms".to_string(),
    jwt_expiry_secs: 3600,
    jwt_refresh_expiry_secs: 86400,
    oauth_managers: HashMap::new(),
    permission_rules: HashMap::new(),
};
```

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
