<div align="center">

# Auth API Reference

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `generate_jwt(payload)` | Generate JWT token | `payload: impl Serialize` | `DMSCResult<String>` |
| `verify_jwt(token)` | Verify JWT token | `token: &str` | `DMSCResult<T>` |
| `generate_jwt_with_refresh(payload)` | Generate JWT and refresh token | `payload: impl Serialize` | `DMSCResult<(String, String)>` |
| `refresh_jwt(refresh_token)` | Get new JWT using refresh token | `refresh_token: &str` | `DMSCResult<String>` |
| `check_permission(role, permission)` | Check role permission | `role: &str`, `permission: &str` | `DMSCResult<bool>` |
| `check_resource_permission(role, resource_type, resource_id, action)` | Check resource permission | `role: &str`, `resource_type: &str`, `resource_id: &str`, `action: &str` | `DMSCResult<bool>` |
| `oauth_config(provider)` | Get OAuth configuration | `provider: &str` | `DMSCResult<DMSCOAuthConfig>` |
| `oauth_authorization_url(provider, state)` | Generate OAuth authorization URL | `provider: &str`, `state: &str` | `DMSCResult<String>` |
| `oauth_exchange_token(provider, code)` | Exchange OAuth token | `provider: &str`, `code: &str` | `DMSCResult<String>` |
| `oauth_get_user_info(provider, token)` | Get OAuth user information | `provider: &str`, `token: &str` | `DMSCResult<DMSCUserInfo>` |

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

// Generate JWT token
let user = User {
    id: 1,
    username: "test_user".to_string(),
    role: "admin".to_string(),
};

let jwt = ctx.auth().generate_jwt(&user).await?;

// Verify JWT token
let decoded_user: User = ctx.auth().verify_jwt(&jwt).await?;

// Check permission
let has_access = ctx.auth().check_permission(&user.role, "admin").await?;

// OAuth2 flow
let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;
let token = ctx.auth().oauth_exchange_token("github", "code123").await?;
let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
```

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

OAuth configuration structure.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `client_id` | `String` | OAuth client ID |
| `client_secret` | `String` | OAuth client secret |
| `redirect_uri` | `String` | Redirect URI |
| `scope` | `Vec<String>` | Permission scope |
| `authorization_url` | `Option<String>` | Authorization URL (optional) |
| `token_url` | `Option<String>` | Token URL (optional) |

### DMSCUserInfo

User information structure.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `id` | `String` | User ID |
| `username` | `String` | Username |
| `email` | `Option<String>` | Email |
| `avatar_url` | `Option<String>` | Avatar URL |
| `provider` | `String` | Provider |

<div align="center">

## JWT Features

</div>

### Token Generation

```rust
// Generate standard JWT
let jwt = ctx.auth().generate_jwt(&user).await?;

// Generate JWT with expiration time
let jwt = ctx.auth().generate_jwt_with_expiry(&user, 7200).await?;

// Generate refresh token
let (access_token, refresh_token) = ctx.auth().generate_jwt_with_refresh(&user).await?;
```

### Token Verification

```rust
// Verify JWT and decode
let user: User = ctx.auth().verify_jwt(&token).await?;

// Verify JWT expiration time
let is_valid = ctx.auth().verify_jwt_expiry(&token).await?;

// Refresh access token
let new_access_token = ctx.auth().refresh_jwt(&refresh_token).await?;
```

<div align="center">

## OAuth2 Features

</div>

### Supported Providers

- **GitHub**: GitHub OAuth2
- **Google**: Google OAuth2
- **Microsoft**: Microsoft OAuth2
- **Custom**: Support for custom OAuth2 providers

### Authorization Flow

```rust
// 1. Generate authorization URL
let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;

// 2. After user authorization, exchange token
let token = ctx.auth().oauth_exchange_token("github", "code123").await?;

// 3. Get user information
let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
```

<div align="center">

## Permission Management

</div>

### RBAC Model

```rust
// Define role permissions
let role_permissions = vec![
    ("admin", vec!["create", "read", "update", "delete"]),
    ("editor", vec!["create", "read", "update"]),
    ("viewer", vec!["read"]),
];

// Check role permissions
let can_create = ctx.auth().check_permission("admin", "create").await?;
let can_delete = ctx.auth().check_permission("editor", "delete").await?;
```

### Resource Permissions

```rust
// Check specific resource permission
let can_edit_post = ctx.auth().check_resource_permission(
    "user", "post", "123", "edit"
).await?;

// Check batch permissions
let permissions = vec!["read", "write", "delete"];
let results = ctx.auth().check_permissions("admin", &permissions).await?;
```

<div align="center">

## Session Management

</div>

### Session Creation

```rust
// Create session
let session_id = ctx.auth().create_session(&user).await?;

// Set session data
ctx.auth().set_session_data(&session_id, "key", "value").await?;
```

### Session Verification

```rust
// Verify session
let is_valid = ctx.auth().validate_session(&session_id).await?;

// Get session data
let value = ctx.auth().get_session_data(&session_id, "key").await?;

// Destroy session
ctx.auth().destroy_session(&session_id).await?;
```

<div align="center">

## Security Configuration

</div>

### Key Management

```rust
// Use environment variables to store keys
let jwt_secret = std::env::var("DMSC_JWT_SECRET")?;
let oauth_client_secret = std::env::var("DMSC_OAUTH_CLIENT_SECRET")?;

// Rotate keys regularly
ctx.auth().rotate_jwt_secret().await?;
```

### Security Policies

```rust
// Set password policy
ctx.auth().set_password_policy(PasswordPolicy {
    min_length: 8,
    require_uppercase: true,
    require_lowercase: true,
    require_numbers: true,
    require_special_chars: true,
}).await?;

// Enable two-factor authentication
ctx.auth().enable_2fa(&user_id).await?;
```

<div align="center">

## Error Handling

</div>

### Authentication Error Codes

| Error Code | Description |
|:--------|:-------------|
| `INVALID_TOKEN` | Invalid token |
| `TOKEN_EXPIRED` | Token expired |
| `INSUFFICIENT_PERMISSIONS` | Insufficient permissions |
| `OAUTH_ERROR` | OAuth authentication error |
| `SESSION_INVALID` | Invalid session |

### Error Handling Example

```rust
match ctx.auth().verify_jwt::<User>(&token).await {
    Ok(user) => {
        // Verification successful
        ctx.logger().info("auth", &format!("User authenticated: {}", user.username))?;
    }
    Err(DMSCError { code, .. }) if code == "TOKEN_EXPIRED" => {
        // Token expired, try to refresh
        let new_token = ctx.auth().refresh_jwt(&refresh_token).await?;
    }
    Err(e) => {
        // Other errors
        ctx.logger().error("auth", &format!("Authentication failed: {}", e))?;
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Secure Key Storage**: Use environment variables or key management services to store JWT keys and OAuth credentials
2. **Reasonable Expiration Time**: Set appropriate token expiration times based on application requirements
3. **Use HTTPS**: Always use HTTPS to transmit authentication information in production environments
4. **Implement Token Refresh**: Implement token refresh mechanism for long-running applications
5. **Regular Key Rotation**: Regularly rotate JWT keys and OAuth credentials
6. **Log Authentication Events**: Log all authentication and authorization events for auditing

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [config](./config.md): Configuration module, managing authentication configuration and key settings
- [cache](./cache.md): Cache module, providing multi-backend cache abstraction, caching user sessions and permission data
- [database](./database.md): Database module, providing user data persistence and query functionality
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [observability](./observability.md): Observability module, monitoring authentication performance and security events
- [security](./security.md): Security module, providing encryption, hashing, and verification functionality
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
