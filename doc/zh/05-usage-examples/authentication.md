<div align="center">

# 认证与授权示例

**Version: 0.1.4**

**Last modified date: 2026-01-15**

本示例展示如何使用DMSC的auth模块进行JWT和OAuth认证与授权。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- JWT令牌生成和验证
- OAuth2授权流程
- 基于角色的访问控制

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解JWT和OAuth2基本概念

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-auth-example
cd dms-auth-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

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

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde::{Deserialize, Serialize};

// 用户信息结构
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: String,
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_auth(DMSCAuthConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Auth Example started")?;
        
        // 创建示例用户
        let user = User {
            id: 1,
            username: "test_user",
            email: "test@example.com",
            role: "admin",
        };
        
        // 生成JWT令牌
        let jwt_manager = ctx.module::<DMSCAuthModule>().await?.jwt_manager();
        let jwt = jwt_manager.generate_token("user123", vec!["admin"], vec!["read", "write"])?;
        ctx.logger().info("jwt", &format!("Generated JWT: {}", jwt))?;
        
        // 验证JWT令牌
        let claims = jwt_manager.validate_token(&jwt)?;
        ctx.logger().info("jwt", &format!("Decoded claims: {:?}", claims))?;
        
        // 检查权限
        let permission_manager = ctx.module::<DMSCAuthModule>().await?.permission_manager();
        let has_admin_access = permission_manager.check_permission("admin", "admin").await?;
        let has_user_access = permission_manager.check_permission("admin", "user").await?;
        
        ctx.logger().info("auth", &format!("Has admin access: {}", has_admin_access))?;
        ctx.logger().info("auth", &format!("Has user access: {}", has_user_access))?;
        
        // OAuth2配置示例
        let oauth_manager = ctx.module::<DMSCAuthModule>().await?.oauth_manager("github");
        if let Some(oauth) = oauth_manager {
            let auth_url = oauth.get_auth_url("state123").await?;
            ctx.logger().info("oauth", &format!("GitHub auth URL: {}", auth_url))?;
        }
        
        ctx.logger().info("service", "DMSC Auth Example completed")?;
        
        Ok(())
    }).await
}
```

<div align="center">

## 代码解析

</div>

### 1. 导入依赖

```rust
use dmsc::prelude::*;
use serde::{Deserialize, Serialize};
```

导入DMSC的核心组件和Serde库用于序列化和反序列化。

### 2. 用户信息结构

```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
    role: String,
}
```

定义用户信息结构，用于JWT令牌的生成和验证。

### 3. 构建应用

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_auth(DMSCAuthConfig::default())?
    .build()?;
```

构建应用并启用auth模块。

### 4. JWT操作

```rust
let jwt_manager = ctx.module::<DMSCAuthModule>().await?.jwt_manager();
let jwt = jwt_manager.generate_token("user123", vec!["admin"], vec!["read", "write"])?;

// 验证JWT令牌
let claims = jwt_manager.validate_token(&jwt)?;
```

- `generate_token()`：生成带用户ID、角色和权限的JWT令牌
- `validate_token()`：验证JWT令牌并返回声明

### 5. 权限检查

```rust
let permission_manager = ctx.module::<DMSCAuthModule>().await?.permission_manager();
let has_admin_access = permission_manager.check_permission("admin", "admin").await?;
let has_user_access = permission_manager.check_permission("admin", "user").await?;
```

`check_permission()`：检查角色是否具有特定权限。

### 6. OAuth2操作

```rust
let oauth_manager = ctx.module::<DMSCAuthModule>().await?.oauth_manager("github")?;
let auth_url = oauth_manager.get_auth_url("state123").await?;
```

- `oauth_manager()`：获取特定OAuth提供商的管理器
- `get_auth_url()`：生成OAuth授权URL

<div align="center">

## 运行步骤

</div>

### 1. 构建项目

```bash
cargo build
```

### 2. 运行项目

```bash
cargo run
```

## 预期结果

运行示例后，您应该会看到类似以下的输出：

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

## 扩展功能

</div>

### 1. 实现完整的OAuth2回调处理

```rust
// 在实际应用中，您需要实现一个HTTP端点来处理OAuth2回调
async fn handle_oauth_callback(ctx: &DMSCServiceContext, code: &str) -> DMSCResult<String> {
    let token = ctx.auth().oauth_exchange_token("github", code).await?;
    let user_info = ctx.auth().oauth_get_user_info("github", &token).await?;
    Ok(user_info)
}
```

### 2. 实现刷新令牌机制

```rust
// 生成包含刷新令牌的JWT
let (access_token, refresh_token) = ctx.auth().generate_jwt_with_refresh(&user).await?;

// 使用刷新令牌获取新的访问令牌
let new_access_token = ctx.auth().refresh_jwt(&refresh_token).await?;
```

### 3. 实现更复杂的权限模型

```rust
// 定义更复杂的权限规则
let permission_rules = vec![
    ("admin", vec!["create", "read", "update", "delete"]),
    ("user", vec!["read"]),
];

// 检查特定资源的权限
let can_edit_resource = ctx.auth().check_resource_permission(
    &user.role, 
    "resource_type", 
    "resource_id", 
    "edit"
).await?;
```

<div align="center">

## 最佳实践

</div>

1. **安全存储密钥**：在生产环境中，不要将JWT密钥和OAuth凭证硬编码在配置文件中，使用环境变量或安全的密钥管理服务

2. **合理设置过期时间**：根据应用需求设置合适的JWT过期时间，平衡安全性和用户体验

3. **使用HTTPS**：在生产环境中，始终使用HTTPS传输JWT令牌和OAuth凭证

4. **实现令牌刷新机制**：对于长期运行的应用，实现令牌刷新机制，避免用户频繁登录

5. **定期轮换密钥**：定期轮换JWT密钥和OAuth凭证，提高安全性

6. **记录认证事件**：记录所有认证和授权事件，便于审计和调试

<div align="center">

## 总结

</div>

本示例展示了如何使用DMSC的auth模块进行认证与授权，包括：

- JWT令牌的生成和验证
- OAuth2授权流程
- 基于角色的访问控制

通过本示例，您应该已经了解了DMSC auth模块的基本使用方式。您可以在此基础上进一步实现更复杂的认证和授权逻辑。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权