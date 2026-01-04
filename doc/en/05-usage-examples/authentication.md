<div align="center">

# Authentication and Authorization Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC's auth module for JWT and OAuth authentication and authorization.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- JWT token generation and verification
- OAuth2 authorization flow
- Role-based access control

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of JWT and OAuth2 basic concepts

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-auth-example
cd dms-auth-example
```

### 2. Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root:

```yaml
service:
  name: "dms-auth-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

auth:
  jwt:
    secret: "your-secret-key-here"
    issuer: "dms-auth-example"
    expires_in: 3600
  oauth:
    github:
      client_id: "your-github-client-id"
      client_secret: "your-github-client-secret"
      redirect_uri: "http://localhost:8080/callback"
```

### 4. Write Main Code

Replace the content of `src/main.rs` with the following:

```rust
use dms::prelude::*;
use serde::{Deserialize, Serialize};

// User information structure
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: String,
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_auth(DMSCAuthConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Auth Example started")?;
        
        // Create sample user
        let user = User {
            id: 1,
            username: "test_user",
            email: "test@example.com",
            role: "admin",
        };
        
        // Generate JWT token
        let jwt = ctx.auth().generate_jwt(&user).await?;
        ctx.logger().info("jwt", &format!("Generated JWT: {}", jwt))?;
        
        // Verify JWT token
        let decoded_user: User = ctx.auth().verify_jwt(&jwt).await?;
        ctx.logger().info("jwt", &format!("Decoded user: {:?}", decoded_user))?;
        
        // Check permissions
        let has_admin_access = ctx.auth().check_permission(&decoded_user.role, "admin").await?;
        let has_user_access = ctx.auth().check_permission(&decoded_user.role, "user").await?;
        
        ctx.logger().info("auth", &format!("Has admin access: {}", has_admin_access))?;
        ctx.logger().info("auth", &format!("Has user access: {}", has_user_access))?;
        
        // OAuth2 configuration example
        let oauth_config = ctx.auth().oauth_config("github").await?;
        ctx.logger().info("oauth", &format!("GitHub OAuth config: {:?}", oauth_config))?;
        
        // Generate OAuth authorization URL
        let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;
        ctx.logger().info("oauth", &format!("GitHub auth URL: {}", auth_url))?;
        
        ctx.logger().info("service", "DMSC Auth Example completed")?;
        
        Ok(())
    }).await
}
```

<div align="center">

## Code Explanation

</div>

### 1. Import Dependencies

```rust
use dms::prelude::*;
use serde::{Deserialize, Serialize};
```

Import DMSC's core components and Serde library for serialization and deserialization.

### 2. User Information Structure

```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: String,
}
```

Define the user information structure for JWT token generation and verification.

### 3. Build Application

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_auth(DMSCAuthConfig::default())?
    .build()?;
```

Build the application and enable the auth module.

### 4. JWT Operations

```rust
// Generate JWT token
let jwt = ctx.auth().generate_jwt(&user).await?;

// Verify JWT token
let decoded_user: User = ctx.auth().verify_jwt(&jwt).await?;
```

- `generate_jwt()`: Generate JWT token based on user information
- `verify_jwt()`: Verify JWT token and decode to user information

### 5. Permission Check

```rust
let has_admin_access = ctx.auth().check_permission(&decoded_user.role, "admin").await?;
let has_user_access = ctx.auth().check_permission(&decoded_user.role, "user").await?;
```

`check_permission()`: Check if the user role has specific permissions.

### 6. OAuth2 Operations

```rust
// Get OAuth configuration
let oauth_config = ctx.auth().oauth_config("github").await?;

// Generate authorization URL
let auth_url = ctx.auth().oauth_authorization_url("github", "state123").await?;
```

- `oauth_config()`: Get configuration for specific OAuth provider
- `oauth_authorization_url()`: Generate OAuth authorization URL

<div align="center">

## Running Steps

</div>

### 1. Build Project

```bash
cargo build
```

### 2. Run Project

```bash
cargo run
```

## Expected Results

After running the example, you should see output similar to the following:

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC Auth Example started"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"jwt","message":"Generated JWT: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"jwt","message":"Decoded user: User { id: 1, username: \"test_user\", email: \"test@example.com\", role: \"admin\" }"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"auth","message":"Has admin access: true"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"auth","message":"Has user access: true"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"oauth","message":"GitHub OAuth config: ..."}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"oauth","message":"GitHub auth URL: https://github.com/login/oauth/authorize?client_id=..."}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC Auth Example completed"}
```

<div align="center">

## Extended Features

</div>

### 1. Implement Complete OAuth2 Callback Handling

```rust
// In a real application, you need to implement an HTTP endpoint to handle OAuth2 callbacks
async fn handle_oauth_callback(ctx: &DMSCServiceContext, code: &str) -> DMSCResult<String> {
    let token = ctx.auth().oauth_exchange_token("github", code).await?;
    let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
    Ok(user_info)
}
```

### 2. Implement Refresh Token Mechanism

```rust
// Generate JWT with refresh token
let (access_token, refresh_token) = ctx.auth().generate_jwt_with_refresh(&user).await?;

// Use refresh token to get new access token
let new_access_token = ctx.auth().refresh_jwt(&refresh_token).await?;
```

### 3. Implement More Complex Permission Models

```rust
// Define more complex permission rules
let permission_rules = vec![
    ("admin", vec!["create", "read", "update", "delete"]),
    ("user", vec!["read"]),
];

// Check permissions for specific resources
let can_edit_resource = ctx.auth().check_resource_permission(
    &user.role, 
    "resource_type", 
    "resource_id", 
    "edit"
).await?;
```

<div align="center">

## Best Practices

</div>

1. **Securely store keys**: In production environments, do not hardcode JWT keys and OAuth credentials in configuration files. Use environment variables or secure key management services.

2. **Set appropriate expiration times**: Set suitable JWT expiration times based on application requirements, balancing security and user experience.

3. **Use HTTPS**: Always use HTTPS to transmit JWT tokens and OAuth credentials in production environments.

4. **Implement token refresh mechanism**: For long-running applications, implement token refresh mechanisms to avoid frequent user logins.

5. **Regularly rotate keys**: Regularly rotate JWT keys and OAuth credentials to improve security.

6. **Log authentication events**: Record all authentication and authorization events for auditing and debugging.

<div align="center">

## Summary

</div>

This example demonstrates how to use DMSC's auth module for authentication and authorization, including:

- JWT token generation and verification
- OAuth2 authorization flow
- Role-based access control

Through this example, you should have understood the basic usage of the DMSC auth module. You can build upon this foundation to implement more complex authentication and authorization logic.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching example, understand how to use the caching module to improve application performance
- [database](./database.md): Database example, learn database connection and query operations
- [http](./http.md): HTTP service example, build web applications and RESTful APIs
- [mq](./mq.md): Message queue example, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability example, monitor application performance and health status
- [security](./security.md): Security example, encryption, hashing, and security best practices
- [storage](./storage.md): Storage example, file upload/download and storage management
- [validation](./validation.md): Validation example, data validation and sanitization operations
- [authentication](./authentication.md): Authentication example, learn JWT, OAuth2, and RBAC authentication and authorization