<div align="center">

# Auth API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

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
| `oauth_provider(provider)` | Get OAuth provider | `provider: &str` | `Option<Arc<DMSCOAuthProvider>>` |

#### Usage Example

```rust
use dms::prelude::*;
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

let jwt = jwt_manager.generate_jwt(&user).await?;

// Verify JWT token
let decoded_user: User = jwt_manager.verify_jwt(&jwt).await?;

// Access permission manager through module
let permission_manager = ctx.module::<DMSCAuthModule>().await?
    .permission_manager();

// Check permission
let has_access = permission_manager.check_permission(&user.role, "admin").await?;

// OAuth2 flow
let auth_url = ctx.module::<DMSCAuthModule>().await?
    .oauth_provider("github")
    .unwrap()
    .get_authorization_url("state123").await?;
```

### DMSCJWTManager

JWT token manager, responsible for JWT generation and verification.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(secret)` | Create JWT manager | `secret: String` | `Self` |
| `generate_jwt(payload)` | Generate JWT token | `payload: impl Serialize` | `DMSCResult<String>` |
| `verify_jwt(token)` | Verify JWT token | `token: &str` | `DMSCResult<T>` |
| `generate_jwt_with_refresh(payload)` | Generate JWT and refresh token | `payload: impl Serialize` | `DMSCResult<(String, String)>` |
| `refresh_jwt(refresh_token)` | Get new JWT using refresh token | `refresh_token: &str` | `DMSCResult<String>` |

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
| `jwt_expires_in` | `u64` | JWT expiration time (seconds) | 3600 |
| `jwt_refresh_expires_in` | `u64` | Refresh token expiration time (seconds) | 86400 |
| `oauth_providers` | `HashMap<String, DMSCOAuthConfig>` | OAuth provider configuration | Empty |
| `permission_rules` | `HashMap<String, Vec<String>>` | Permission rules | Default rules |

#### Usage Example

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
