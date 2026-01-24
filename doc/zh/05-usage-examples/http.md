<div align="center">

# HTTP服务使用示例

**Version: 0.1.6**

**Last modified date: 2026-01-24**

本示例展示如何使用DMSC的http模块进行HTTP服务器、客户端、路由管理、中间件、WebSocket和文件上传下载功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- HTTP服务器配置和路由管理
- RESTful API设计和实现
- 中间件和请求处理
- WebSocket实时通信
- 文件上传下载处理
- 错误处理和响应格式化

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解HTTP协议和RESTful API概念
- （可选）Postman或curl用于API测试

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-http-example
cd dms-http-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-http-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

http:
  server:
    host: "0.0.0.0"
    port: 8080
    max_connections: 1000
    request_timeout: 30
    keep_alive_timeout: 60
    max_request_size: 10485760  # 10MB
    enable_compression: true
    compression_threshold: 1024
    enable_cors: true
    cors_origins: ["*"]
    cors_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    cors_headers: ["Content-Type", "Authorization"]
    enable_websocket: true
    websocket_ping_interval: 30
    rate_limit_enabled: true
    rate_limit_requests_per_minute: 60
    rate_limit_burst: 10
    enable_request_logging: true
    log_request_body: false
    log_response_body: false
```

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde_json::json;
use chrono::Utc;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_http(DMSCHttpConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC HTTP Example started")?;
        
        // 初始化HTTP服务器
        initialize_http_server(&ctx).await?;
        
        // 配置API路由
        setup_api_routes(&ctx).await?;
        
        // 启动HTTP服务器
        ctx.logger().info("service", "HTTP server is running on http://localhost:8080")?;
        
        // 保持服务运行
        tokio::signal::ctrl_c().await?;
        ctx.logger().info("service", "Shutting down HTTP server")?;
        
        Ok(())
    }).await
}

async fn initialize_http_server(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("http", "Initializing HTTP server")?;
    
    // 创建HTTP服务器配置
    let server_config = DMSCHttpServerConfig {
        host: "0.0.0.0".to_string(),
        port: 8080,
        max_connections: 1000,
        request_timeout: Duration::from_secs(30),
        keep_alive_timeout: Duration::from_secs(60),
        max_request_size: 10 * 1024 * 1024, // 10MB
        enable_compression: true,
        compression_threshold: 1024, // 1KB
        enable_cors: true,
        cors_origins: vec!["*".to_string()],
        cors_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
        cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        enable_websocket: true,
        websocket_ping_interval: Duration::from_secs(30),
        rate_limit_enabled: true,
        rate_limit_requests_per_minute: 60,
        rate_limit_burst: 10,
        enable_request_logging: true,
        log_request_body: false,
        log_response_body: false,
    };
    
    // 初始化HTTP服务器
    ctx.http().init_server(server_config).await?;
    ctx.logger().info("http", "HTTP server initialized on port 8080")?;
    
    Ok(())
}

async fn setup_api_routes(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("http", "Setting up API routes")?;
    
    // GET请求处理 - 根路径
    ctx.http().get("/", |req, ctx| async move {
        Ok(DMSCHttpResponse::ok(json!({
            "message": "Welcome to DMSC API",
            "version": "1.0.0",
            "timestamp": Utc::now().to_rfc3339(),
        })))
    });
    
    // GET请求处理 - 获取用户信息
    ctx.http().get("/users/:id", |req, ctx| async move {
        let user_id = req.params.get("id")
            .and_then(|id| id.parse::<i32>().ok())
            .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
        
        // 从数据库获取用户信息
        match ctx.database().query_one(
            "SELECT id, name, email, created_at FROM users WHERE id = $1",
            vec![user_id.into()]
        ).await? {
            Some(user_data) => Ok(DMSCHttpResponse::ok(json!({
                "id": user_data.get::<i32>("id"),
                "name": user_data.get::<String>("name"),
                "email": user_data.get::<String>("email"),
                "created_at": user_data.get::<String>("created_at"),
            }))),
            None => Ok(DMSCHttpResponse::not_found(json!({
                "error": "User not found",
                "user_id": user_id,
            }))),
        }
    });
    
    // POST请求处理 - 创建用户
    ctx.http().post("/users", |req, ctx| async move {
        let body = req.json::<serde_json::Value>()
            .await
            .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
        
        // 验证必需字段
        let name = body["name"].as_str()
            .ok_or_else(|| DMSCError::bad_request("Name is required".to_string()))?;
        let email = body["email"].as_str()
            .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
        
        // 验证邮箱格式
        if !email.contains('@') {
            return Ok(DMSCHttpResponse::bad_request(json!({
                "error": "Invalid email format"
            })));
        }
        
        // 插入用户数据
        let user_id = ctx.database()
            .execute(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                vec![name.into(), email.into()]
            )
            .await?;
        
        Ok(DMSCHttpResponse::created(json!({
            "id": user_id,
            "name": name,
            "email": email,
            "message": "User created successfully"
        })))
    });
    
    ctx.logger().info("http", "API routes configured successfully")?;
    
    Ok(())
}
```

<div align="center">

## 代码解析

</div>

http模块提供HTTP服务器、客户端、路由管理、中间件、WebSocket和文件上传下载功能的使用示例。

## HTTP服务器

### 基本服务器配置

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建HTTP服务器配置
let server_config = DMSCHttpServerConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    max_connections: 1000,
    request_timeout: Duration::from_secs(30),
    keep_alive_timeout: Duration::from_secs(60),
    max_request_size: 10 * 1024 * 1024, // 10MB
    enable_compression: true,
    compression_threshold: 1024, // 1KB
    enable_cors: true,
    cors_origins: vec!["*".to_string()],
    cors_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
    cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
    enable_websocket: true,
    websocket_ping_interval: Duration::from_secs(30),
    rate_limit_enabled: true,
    rate_limit_requests_per_minute: 60,
    rate_limit_burst: 10,
    enable_request_logging: true,
    log_request_body: false,
    log_response_body: false,
};

// 初始化HTTP服务器
ctx.http().init_server(server_config).await?;
ctx.log().info("HTTP server initialized on port 8080");
```

### 基本路由

```rust
use dmsc::prelude::*;
use serde_json::json;

// GET请求处理
ctx.http().get("/", |req, ctx| async move {
    Ok(DMSCHttpResponse::ok(json!({
        "message": "Welcome to DMSC API",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
});

// 带参数的GET请求
ctx.http().get("/users/:id", |req, ctx| async move {
    let user_id = req.params.get("id")
        .and_then(|id| id.parse::<i32>().ok())
        .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
    
    // 从数据库获取用户信息
    match ctx.database().query_one(
        "SELECT id, name, email, created_at FROM users WHERE id = $1",
        vec![user_id.into()]
    ).await? {
        Some(user_data) => Ok(DMSCHttpResponse::ok(json!({
            "id": user_data.get::<i32>("id"),
            "name": user_data.get::<String>("name"),
            "email": user_data.get::<String>("email"),
            "created_at": user_data.get::<String>("created_at"),
        }))),
        None => Ok(DMSCHttpResponse::not_found(json!({
            "error": "User not found",
            "user_id": user_id,
        }))),
    }
});

// POST请求处理
ctx.http().post("/users", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    // 验证必需字段
    let name = body["name"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Name is required".to_string()))?;
    let email = body["email"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
    
    // 验证邮箱格式
    if !email.contains('@') {
        return Ok(DMSCHttpResponse::bad_request(json!({
            "error": "Invalid email format"
        })));
    }
    
    // 插入用户数据
    let user_id = ctx.database()
        .execute(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
            vec![name.into(), email.into()]
        )
        .await?;
    
    Ok(DMSCHttpResponse::created(json!({
        "id": user_id,
        "name": name,
        "email": email,
        "message": "User created successfully"
    })))
});

// PUT请求处理
ctx.http().put("/users/:id", |req, ctx| async move {
    let user_id = req.params.get("id")
        .and_then(|id| id.parse::<i32>().ok())
        .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
    
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    let name = body["name"].as_str();
    let email = body["email"].as_str();
    let age = body["age"].as_i64();
    
    // 构建动态更新查询
    let mut updates = Vec::new();
    let mut params = Vec::new();
    
    if let Some(name) = name {
        updates.push("name = $1");
        params.push(name.into());
    }
    
    if let Some(email) = email {
        updates.push(format!("email = ${}", params.len() + 1));
        params.push(email.into());
    }
    
    if let Some(age) = age {
        updates.push(format!("age = ${}", params.len() + 1));
        params.push(age.into());
    }
    
    if updates.is_empty() {
        return Ok(DMSCHttpResponse::bad_request(json!({
            "error": "No fields to update"
        })));
    }
    
    // 添加WHERE条件
    params.push(user_id.into());
    
    let query = format!(
        "UPDATE users SET {} WHERE id = ${} RETURNING *",
        updates.join(", "),
        params.len()
    );
    
    match ctx.database().query_one(query, params).await? {
        Some(updated_user) => Ok(DMSCHttpResponse::ok(json!({
            "id": updated_user.get::<i32>("id"),
            "name": updated_user.get::<String>("name"),
            "email": updated_user.get::<String>("email"),
            "age": updated_user.get::<i32>("age"),
            "message": "User updated successfully"
        }))),
        None => Ok(DMSCHttpResponse::not_found(json!({
            "error": "User not found",
            "user_id": user_id,
        }))),
    }
});

// DELETE请求处理
ctx.http().delete("/users/:id", |req, ctx| async move {
    let user_id = req.params.get("id")
        .and_then(|id| id.parse::<i32>().ok())
        .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
    
    let deleted_rows = ctx.database()
        .execute("DELETE FROM users WHERE id = $1", vec![user_id.into()])
        .await?;
    
    if deleted_rows > 0 {
        Ok(DMSCHttpResponse::ok(json!({
            "message": "User deleted successfully",
            "user_id": user_id,
        })))
    } else {
        Ok(DMSCHttpResponse::not_found(json!({
            "error": "User not found",
            "user_id": user_id,
        })))
    }
});
```

### 路由组

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建API路由组
let api_routes = ctx.http().group("/api/v1");

// 在路由组中添加路由
api_routes.get("/health", |req, ctx| async move {
    let health_status = json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "database": ctx.database().ping().await.is_ok(),
        "cache": ctx.cache().ping().await.is_ok(),
        "version": "1.0.0",
    });
    
    Ok(DMSCHttpResponse::ok(health_status))
});

api_routes.get("/stats", |req, ctx| async move {
    // 获取系统统计信息
    let user_count = ctx.database()
        .query_one("SELECT COUNT(*) as count FROM users", vec![])
        .await?
        .and_then(|row| row.get::<i64>("count"))
        .unwrap_or(0);
    
    let order_count = ctx.database()
        .query_one("SELECT COUNT(*) as count FROM orders", vec![])
        .await?
        .and_then(|row| row.get::<i64>("count"))
        .unwrap_or(0);
    
    let stats = json!({
        "users": user_count,
        "orders": order_count,
        "server_time": chrono::Utc::now().to_rfc3339(),
    });
    
    Ok(DMSCHttpResponse::ok(stats))
});

// 用户管理路由组
let user_routes = api_routes.group("/users");

user_routes.get("/", |req, ctx| async move {
    let page = req.query.get("page")
        .and_then(|p| p.parse::<i32>().ok())
        .unwrap_or(1);
    let limit = req.query.get("limit")
        .and_then(|l| l.parse::<i32>().ok())
        .unwrap_or(10);
    let offset = (page - 1) * limit;
    
    let users = ctx.database()
        .query(
            "SELECT id, name, email, created_at FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            vec![limit.into(), offset.into()]
        )
        .await?;
    
    let user_list: Vec<serde_json::Value> = users.iter().map(|user| {
        json!({
            "id": user.get::<i32>("id"),
            "name": user.get::<String>("name"),
            "email": user.get::<String>("email"),
            "created_at": user.get::<String>("created_at"),
        })
    }).collect();
    
    let total_count = ctx.database()
        .query_one("SELECT COUNT(*) as count FROM users", vec![])
        .await?
        .and_then(|row| row.get::<i64>("count"))
        .unwrap_or(0);
    
    let response = json!({
        "users": user_list,
        "pagination": {
            "page": page,
            "limit": limit,
            "total": total_count,
            "pages": (total_count as f64 / limit as f64).ceil() as i32,
        }
    });
    
    Ok(DMSCHttpResponse::ok(response))
});

// 认证路由组
let auth_routes = api_routes.group("/auth");

auth_routes.post("/login", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    let email = body["email"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
    let password = body["password"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Password is required".to_string()))?;
    
    // 验证用户凭据
    let user = ctx.database()
        .query_one(
            "SELECT id, name, email, password_hash FROM users WHERE email = $1",
            vec![email.into()]
        )
        .await?;
    
    if let Some(user_data) = user {
        let stored_hash = user_data.get::<String>("password_hash").unwrap_or_default();
        
        // 验证密码（使用bcrypt等密码哈希库）
        if verify_password(password, &stored_hash)? {
            // 生成JWT令牌
            let token = ctx.auth().generate_jwt(
                user_data.get::<i32>("id").unwrap_or(0),
                Duration::from_hours(24)
            ).await?;
            
            Ok(DMSCHttpResponse::ok(json!({
                "token": token,
                "user": {
                    "id": user_data.get::<i32>("id"),
                    "name": user_data.get::<String>("name"),
                    "email": user_data.get::<String>("email"),
                }
            })))
        } else {
            Ok(DMSCHttpResponse::unauthorized(json!({
                "error": "Invalid credentials"
            })))
        }
    } else {
        Ok(DMSCHttpResponse::unauthorized(json!({
            "error": "Invalid credentials"
        })))
    }
});

auth_routes.post("/register", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    let name = body["name"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Name is required".to_string()))?;
    let email = body["email"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
    let password = body["password"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Password is required".to_string()))?;
    
    // 验证输入
    if password.len() < 8 {
        return Ok(DMSCHttpResponse::bad_request(json!({
            "error": "Password must be at least 8 characters long"
        })));
    }
    
    // 检查邮箱是否已存在
    let existing_user = ctx.database()
        .query_one("SELECT id FROM users WHERE email = $1", vec![email.into()])
        .await?;
    
    if existing_user.is_some() {
        return Ok(DMSCHttpResponse::conflict(json!({
            "error": "Email already registered"
        })));
    }
    
    // 哈希密码
    let password_hash = hash_password(password)?;
    
    // 创建用户
    let user_id = ctx.database()
        .execute(
            "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
            vec![name.into(), email.into(), password_hash.into()]
        )
        .await?;
    
    Ok(DMSCHttpResponse::created(json!({
        "id": user_id,
        "name": name,
        "email": email,
        "message": "User registered successfully"
    })))
});
```

## HTTP客户端

### 基本客户端使用

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建HTTP客户端配置
let client_config = DMSCHttpClientConfig {
    timeout: Duration::from_secs(30),
    max_redirects: 5,
    user_agent: "DMSC-Client/1.0".to_string(),
    enable_compression: true,
    enable_cookies: true,
    max_connections: 100,
    connection_timeout: Duration::from_secs(10),
    idle_timeout: Duration::from_secs(60),
    retry_attempts: 3,
    retry_delay: Duration::from_millis(1000),
    enable_metrics: true,
};

// 初始化HTTP客户端
ctx.http().init_client(client_config).await?;

// GET请求
let response = ctx.http().get("https://api.github.com/users/octocat")
    .header("Accept", "application/json")
    .header("User-Agent", "DMSC-Client/1.0")
    .send()
    .await?;

if response.status.is_success() {
    let user_data = response.json::<serde_json::Value>().await?;
    ctx.log().info(format!("GitHub user: {:?}", user_data));
} else {
    ctx.log().error(format!("Request failed with status: {}", response.status));
}

// POST请求
let create_response = ctx.http().post("https://api.example.com/users")
    .json(&json!({
        "name": "John Doe",
        "email": "john@example.com",
        "age": 30
    }))
    .send()
    .await?;

if create_response.status.is_success() {
    let created_user = create_response.json::<serde_json::Value>().await?;
    ctx.log().info(format!("Created user: {:?}", created_user));
}
```

### 高级客户端功能

```rust
use dmsc::prelude::*;
use serde_json::json;

// 表单数据提交
let form_response = ctx.http().post("https://httpbin.org/post")
    .form(&json!({
        "username": "testuser",
        "password": "testpass",
        "remember": "true"
    }))
    .send()
    .await?;

// 文件上传
let upload_response = ctx.http().post("https://httpbin.org/post")
    .multipart_form()
    .file("file", "./data/document.pdf", "application/pdf")
    .field("description", "Important document")
    .field("category", "documents")
    .send()
    .await?;

// 自定义头部和认证
let auth_response = ctx.http().get("https://api.example.com/protected")
    .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")
    .header("X-API-Key", "your-api-key")
    .header("X-Request-ID", "unique-request-id")
    .send()
    .await?;

// 流式下载大文件
let download_response = ctx.http().get("https://example.com/large-file.zip")
    .send_stream()
    .await?;

let mut file = std::fs::File::create("./downloads/large-file.zip")?;
let mut stream = download_response.stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk)?;
}

// 并发请求
let urls = vec![
    "https://api.github.com/users/octocat",
    "https://api.github.com/users/torvalds",
    "https://api.github.com/users/dhh",
];

let mut handles = Vec::new();
for url in urls {
    let handle = ctx.http().get(url)
        .header("Accept", "application/json")
        .send();
    handles.push(handle);
}

let results = futures::future::join_all(handles).await;
for (i, result) in results.iter().enumerate() {
    match result {
        Ok(response) => {
            ctx.log().info(format!("Request {} completed with status: {}", i, response.status));
        }
        Err(e) => {
            ctx.log().error(format!("Request {} failed: {}", i, e));
        }
    }
}
```

## 中间件

### 内置中间件

```rust
use dmsc::prelude::*;
use serde_json::json;

// 日志中间件
ctx.http().use_middleware(DMSCHttpMiddleware::logging());

// CORS中间件
ctx.http().use_middleware(DMSCHttpMiddleware::cors()
    .allow_origin("https://example.com")
    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allow_headers(vec!["Content-Type", "Authorization"])
    .max_age(3600)
);

// 认证中间件
ctx.http().use_middleware(DMSCHttpMiddleware::auth()
    .exclude_paths(vec!["/api/v1/auth/login", "/api/v1/auth/register"])
    .token_header("Authorization")
    .token_prefix("Bearer ")
);

// 速率限制中间件
ctx.http().use_middleware(DMSCHttpMiddleware::rate_limit()
    .requests_per_minute(60)
    .burst(10)
    .key_extractor(|req| {
        // 基于IP地址进行速率限制
        req.remote_addr.to_string()
    })
);

// 压缩中间件
ctx.http().use_middleware(DMSCHttpMiddleware::compression()
    .threshold(1024)
    .level(6)
);

// 请求ID中间件
ctx.http().use_middleware(DMSCHttpMiddleware::request_id()
    .header_name("X-Request-ID")
    .generator(|| uuid::Uuid::new_v4().to_string())
);
```

### 自定义中间件

```rust
use dmsc::prelude::*;
use serde_json::json;

// 自定义认证中间件
async fn auth_middleware(
    req: DMSCHttpRequest,
    ctx: DMSCContext,
    next: DMSCNext,
) -> DMSCResult<DMSCHttpResponse> {
    // 跳过不需要认证的路径
    let skip_auth = vec!["/api/v1/auth/login", "/api/v1/auth/register", "/health"];
    if skip_auth.iter().any(|path| req.path.starts_with(path)) {
        return next.run(req, ctx).await;
    }
    
    // 检查认证头
    let auth_header = req.headers.get("Authorization")
        .ok_or_else(|| DMSCError::unauthorized("Missing Authorization header".to_string()))?;
    
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| DMSCError::unauthorized("Invalid Authorization format".to_string()))?;
    
    // 验证JWT令牌
    match ctx.auth().validate_jwt(token).await {
        Ok(user_id) => {
            // 将用户信息添加到请求上下文
            let mut req = req;
            req.context.insert("user_id", user_id.to_string());
            
            // 继续处理请求
            next.run(req, ctx).await
        }
        Err(e) => {
            Ok(DMSCHttpResponse::unauthorized(json!({
                "error": "Invalid or expired token",
                "details": e.to_string()
            })))
        }
    }
}

// 自定义日志中间件
async fn logging_middleware(
    req: DMSCHttpRequest,
    ctx: DMSCContext,
    next: DMSCNext,
) -> DMSCResult<DMSCHttpResponse> {
    let start_time = std::time::Instant::now();
    let request_id = req.headers.get("X-Request-ID")
        .unwrap_or(&"unknown".to_string())
        .clone();
    
    ctx.log().info(format!(
        "[{}] {} {} - Started",
        request_id,
        req.method,
        req.path
    ));
    
    // 执行请求
    let response = next.run(req, ctx).await?;
    
    let duration = start_time.elapsed();
    ctx.log().info(format!(
        "[{}] {} {} - Completed {} in {:?}",
        request_id,
        response.status.method(),
        response.status.as_str(),
        response.status.as_u16(),
        duration
    ));
    
    Ok(response)
}

// 自定义错误处理中间件
async fn error_handling_middleware(
    req: DMSCHttpRequest,
    ctx: DMSCContext,
    next: DMSCNext,
) -> DMSCResult<DMSCHttpResponse> {
    match next.run(req, ctx.clone()).await {
        Ok(response) => Ok(response),
        Err(error) => {
            ctx.log().error(format!("Request error: {}", error));
            
            let error_response = match error {
                DMSCError::BadRequest(msg) => {
                    DMSCHttpResponse::bad_request(json!({
                        "error": "Bad Request",
                        "message": msg,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }))
                }
                DMSCError::Unauthorized(msg) => {
                    DMSCHttpResponse::unauthorized(json!({
                        "error": "Unauthorized",
                        "message": msg,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }))
                }
                DMSCError::NotFound(msg) => {
                    DMSCHttpResponse::not_found(json!({
                        "error": "Not Found",
                        "message": msg,
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }))
                }
                DMSCError::InternalError(msg) => {
                    DMSCHttpResponse::internal_server_error(json!({
                        "error": "Internal Server Error",
                        "message": "An internal error occurred",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }))
                }
                _ => {
                    DMSCHttpResponse::internal_server_error(json!({
                        "error": "Internal Server Error",
                        "message": "An unexpected error occurred",
                        "timestamp": chrono::Utc::now().to_rfc3339(),
                    }))
                }
            };
            
            Ok(error_response)
        }
    }
}

// 使用自定义中间件
ctx.http().use_custom_middleware(auth_middleware);
ctx.http().use_custom_middleware(logging_middleware);
ctx.http().use_custom_middleware(error_handling_middleware);
```

## WebSocket支持

### WebSocket服务器

```rust
use dmsc::prelude::*;
use serde_json::json;
use futures::{StreamExt, SinkExt};

// WebSocket连接处理
ctx.http().websocket("/ws", |ws_stream, ctx| async move {
    let (mut sender, mut receiver) = ws_stream.split();
    let client_id = uuid::Uuid::new_v4().to_string();
    
    ctx.log().info(format!("WebSocket client connected: {}", client_id));
    
    // 发送欢迎消息
    let welcome_msg = json!({
        "type": "welcome",
        "client_id": client_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    sender.send(DMSCWebSocketMessage::Text(welcome_msg.to_string())).await?;
    
    // 处理接收到的消息
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(DMSCWebSocketMessage::Text(text)) => {
                ctx.log().info(format!("Received message from {}: {}", client_id, text));
                
                // 解析消息
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(data) => {
                        let response = match data["type"].as_str() {
                            Some("ping") => json!({
                                "type": "pong",
                                "timestamp": chrono::Utc::now().to_rfc3339(),
                            }),
                            Some("echo") => json!({
                                "type": "echo_response",
                                "data": &data["data"],
                                "timestamp": chrono::Utc::now().to_rfc3339(),
                            }),
                            Some("get_stats") => {
                                let user_count = ctx.database()
                                    .query_one("SELECT COUNT(*) as count FROM users", vec![])
                                    .await?
                                    .and_then(|row| row.get::<i64>("count"))
                                    .unwrap_or(0);
                                
                                json!({
                                    "type": "stats_response",
                                    "data": {
                                        "users": user_count,
                                        "timestamp": chrono::Utc::now().to_rfc3339(),
                                    }
                                })
                            }
                            _ => json!({
                                "type": "error",
                                "message": "Unknown message type",
                            })
                        };
                        
                        sender.send(DMSCWebSocketMessage::Text(response.to_string())).await?;
                    }
                    Err(e) => {
                        let error_response = json!({
                            "type": "error",
                            "message": format!("Invalid JSON: {}", e),
                        });
                        sender.send(DMSCWebSocketMessage::Text(error_response.to_string())).await?;
                    }
                }
            }
            Ok(DMSCWebSocketMessage::Binary(data)) => {
                ctx.log().info(format!("Received binary data from {}: {} bytes", client_id, data.len()));
                
                // 回显二进制数据
                sender.send(DMSCWebSocketMessage::Binary(data)).await?;
            }
            Ok(DMSCWebSocketMessage::Close(reason)) => {
                ctx.log().info(format!("WebSocket client {} disconnected: {:?}", client_id, reason));
                break;
            }
            Err(e) => {
                ctx.log().error(format!("WebSocket error from {}: {}", client_id, e));
                break;
            }
        }
    }
    
    ctx.log().info(format!("WebSocket connection with {} closed", client_id));
});

// WebSocket聊天室
ctx.http().websocket("/chat/:room", |ws_stream, ctx| async move {
    let (mut sender, mut receiver) = ws_stream.split();
    let room_id = ctx.params.get("room").unwrap_or("default").to_string();
    let user_id = ctx.context.get("user_id").unwrap_or(&"anonymous".to_string()).to_string();
    
    // 将用户添加到聊天室
    ctx.cache().add_to_set(&format!("chat_room:{}", room_id), &user_id).await?;
    
    // 广播用户加入消息
    let join_msg = json!({
        "type": "user_joined",
        "user_id": user_id,
        "room_id": room_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    broadcast_to_room(&room_id, join_msg, &ctx).await?;
    
    // 处理消息
    while let Some(msg) = receiver.next().await {
        if let Ok(DMSCWebSocketMessage::Text(text)) = msg {
            let chat_msg = json!({
                "type": "chat_message",
                "user_id": user_id,
                "room_id": room_id,
                "message": text,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            });
            
            broadcast_to_room(&room_id, chat_msg, &ctx).await?;
        }
    }
    
    // 用户离开聊天室
    ctx.cache().remove_from_set(&format!("chat_room:{}", room_id), &user_id).await?;
    
    let leave_msg = json!({
        "type": "user_left",
        "user_id": user_id,
        "room_id": room_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    broadcast_to_room(&room_id, leave_msg, &ctx).await?;
});

// 广播函数
async fn broadcast_to_room(room_id: &str, message: serde_json::Value, ctx: &DMSCContext) -> DMSCResult<()> {
    let room_key = format!("chat_room:{}:connections", room_id);
    let connections = ctx.cache().get_set_members(&room_key).await?;
    
    for connection_id in connections {
        if let Some(sender) = get_websocket_sender(&connection_id) {
            sender.send(DMSCWebSocketMessage::Text(message.to_string())).await?;
        }
    }
    
    Ok(())
}
```

### WebSocket客户端

```rust
use dmsc::prelude::*;
use futures::{StreamExt, SinkExt};

// 连接到WebSocket服务器
let ws_client = ctx.http().websocket_client("wss://echo.websocket.org").await?;
let (mut sender, mut receiver) = ws_client.split();

// 发送消息
sender.send(DMSCWebSocketMessage::Text("Hello WebSocket!".to_string())).await?;

// 接收消息
while let Some(msg) = receiver.next().await {
    match msg {
        Ok(DMSCWebSocketMessage::Text(text)) => {
            ctx.log().info(format!("Received: {}", text));
        }
        Ok(DMSCWebSocketMessage::Close(reason)) => {
            ctx.log().info(format!("WebSocket closed: {:?}", reason));
            break;
        }
        Err(e) => {
            ctx.log().error(format!("WebSocket error: {}", e));
            break;
        }
        _ => {}
    }
}
```

## 文件上传下载

### 文件上传

```rust
use dmsc::prelude::*;
use serde_json::json;

// 单文件上传
ctx.http().post("/upload/single", |req, ctx| async move {
    let mut multipart = req.multipart()?;
    
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default().to_string();
        let filename = field.file_name().unwrap_or_default().to_string();
        let content_type = field.content_type().unwrap_or_default().to_string();
        
        ctx.log().info(format!("Uploading file: {} ({})", filename, content_type));
        
        // 读取文件数据
        let data = field.bytes().await?;
        
        // 保存到存储系统
        let file_id = ctx.storage().upload(
            &filename,
            &data,
            &content_type,
            None
        ).await?;
        
        ctx.log().info(format!("File uploaded with ID: {}", file_id));
        
        return Ok(DMSCHttpResponse::ok(json!({
            "file_id": file_id,
            "filename": filename,
            "size": data.len(),
            "content_type": content_type,
        })));
    }
    
    Ok(DMSCHttpResponse::bad_request(json!({
        "error": "No file uploaded"
    })))
});

// 多文件上传
ctx.http().post("/upload/multiple", |req, ctx| async move {
    let mut multipart = req.multipart()?;
    let mut uploaded_files = Vec::new();
    
    while let Some(field) = multipart.next_field().await? {
        if let Some(filename) = field.file_name() {
            let content_type = field.content_type().unwrap_or_default().to_string();
            let data = field.bytes().await?;
            
            let file_id = ctx.storage().upload(
                filename,
                &data,
                &content_type,
                None
            ).await?;
            
            uploaded_files.push(json!({
                "file_id": file_id,
                "filename": filename,
                "size": data.len(),
                "content_type": content_type,
            }));
        }
    }
    
    Ok(DMSCHttpResponse::ok(json!({
        "uploaded_files": uploaded_files,
        "total_count": uploaded_files.len()
    })))
});

// 分块上传（大文件）
ctx.http().post("/upload/chunk", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    let upload_id = body["upload_id"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Upload ID is required".to_string()))?;
    let chunk_index = body["chunk_index"].as_i64()
        .ok_or_else(|| DMSCError::bad_request("Chunk index is required".to_string()))? as i32;
    let total_chunks = body["total_chunks"].as_i64()
        .ok_or_else(|| DMSCError::bad_request("Total chunks is required".to_string()))? as i32;
    let chunk_data = body["data"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Chunk data is required".to_string()))?;
    
    // 解码base64数据
    let data = base64::decode(chunk_data)
        .map_err(|e| DMSCError::bad_request(format!("Invalid base64 data: {}", e)))?;
    
    // 上传分块
    ctx.storage().upload_chunk(
        upload_id,
        chunk_index,
        &data,
        total_chunks
    ).await?;
    
    // 检查是否所有分块都已上传
    if chunk_index == total_chunks - 1 {
        let file_id = ctx.storage().complete_multipart_upload(upload_id).await?;
        
        Ok(DMSCHttpResponse::ok(json!({
            "status": "completed",
            "file_id": file_id,
            "upload_id": upload_id
        })))
    } else {
        Ok(DMSCHttpResponse::ok(json!({
            "status": "chunk_uploaded",
            "chunk_index": chunk_index,
            "upload_id": upload_id
        })))
    }
});
```

### 文件下载

```rust
use dmsc::prelude::*;
use serde_json::json;

// 文件下载
ctx.http().get("/download/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    // 获取文件信息
    let file_info = ctx.storage().get_file_info(file_id).await?;
    
    // 设置响应头
    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), file_info.content_type.clone());
    headers.insert("Content-Disposition".to_string(), 
        format!("attachment; filename=\"{}\"", file_info.filename));
    
    // 下载文件数据
    let file_data = ctx.storage().download(file_id).await?;
    
    Ok(DMSCHttpResponse::ok_with_headers(file_data, headers))
});

// 断点续传下载
ctx.http().get("/download/range/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    // 解析Range头
    let range_header = req.headers.get("Range")
        .ok_or_else(|| DMSCError::bad_request("Range header is required".to_string()))?;
    
    let (start, end) = parse_range_header(range_header)?;
    
    // 获取文件信息
    let file_info = ctx.storage().get_file_info(file_id).await?;
    
    // 验证范围
    if start >= file_info.size as u64 {
        return Ok(DMSCHttpResponse::range_not_satisfiable());
    }
    
    let end = end.unwrap_or(file_info.size as u64 - 1);
    let content_length = end - start + 1;
    
    // 下载指定范围
    let data = ctx.storage().download_range(file_id, start, end).await?;
    
    // 设置响应头
    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), file_info.content_type.clone());
    headers.insert("Content-Range".to_string(), 
        format!("bytes {}-{}/{}", start, end, file_info.size));
    headers.insert("Content-Length".to_string(), content_length.to_string());
    headers.insert("Accept-Ranges".to_string(), "bytes".to_string());
    
    Ok(DMSCHttpResponse::partial_content_with_headers(data, headers))
});

// 临时下载链接
ctx.http().get("/download/temp/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    let expires_in = req.query.get("expires")
        .and_then(|e| e.parse::<u64>().ok())
        .unwrap_or(3600); // 默认1小时
    
    // 生成临时下载链接
    let temp_url = ctx.storage().generate_temp_download_url(
        file_id,
        Duration::from_secs(expires_in)
    ).await?;
    
    Ok(DMSCHttpResponse::ok(json!({
        "download_url": temp_url,
        "expires_in": expires_in,
        "file_id": file_id
    })))
});
```

## 高级功能

### 代理请求

```rust
use dmsc::prelude::*;

// HTTP代理
ctx.http().get("/proxy/*path", |req, ctx| async move {
    let target_path = req.params.get("path")
        .ok_or_else(|| DMSCError::bad_request("Target path is required".to_string()))?;
    
    let target_url = format!("https://api.target-service.com/{}", target_path);
    
    // 转发请求
    let proxy_response = ctx.http().client()
        .request(req.method.clone(), &target_url)
        .headers(req.headers.clone())
        .body(req.body.clone())
        .send()
        .await?;
    
    // 返回代理响应
    Ok(DMSCHttpResponse::new(
        proxy_response.status,
        proxy_response.headers,
        proxy_response.body
    ))
});
```

### 服务器发送事件 (SSE)

```rust
use dmsc::prelude::*;
use serde_json::json;

// SSE端点
ctx.http().get("/events", |req, ctx| async move {
    let mut event_stream = DMSCHttpResponse::event_stream();
    
    // 发送初始事件
    event_stream.send_event("connected", json!({
        "message": "Connected to event stream",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })).await?;
    
    // 定期发送事件
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        let event_data = json!({
            "type": "heartbeat",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "server_status": "running",
        });
        
        match event_stream.send_event("heartbeat", event_data).await {
            Ok(_) => continue,
            Err(_) => break, // 客户端断开连接
        }
    }
    
    Ok(event_stream)
});

// 发送自定义事件
async fn send_custom_event(event_type: &str, data: serde_json::Value, ctx: &DMSCContext) -> DMSCResult<()> {
    let event = json!({
        "type": event_type,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    // 广播到所有连接的客户端
    ctx.http().broadcast_event("notification", event).await?;
    
    Ok(())
}
```

## 错误处理

### HTTP错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 错误处理示例
match ctx.http().get("https://api.example.com/data").send().await {
    Ok(response) => {
        if response.status.is_success() {
            let data = response.json::<serde_json::Value>().await?;
            ctx.log().info(format!("Data received: {:?}", data));
        } else if response.status.is_client_error() {
            ctx.log().warn(format!("Client error: {}", response.status));
            // 处理客户端错误
        } else if response.status.is_server_error() {
            ctx.log().error(format!("Server error: {}", response.status));
            // 处理服务器错误
        }
    }
    Err(DMSCError::HttpTimeoutError(e)) => {
        ctx.log().error(format!("Request timeout: {}", e));
        // 重试或降级处理
    }
    Err(DMSCError::HttpConnectionError(e)) => {
        ctx.log().error(format!("Connection error: {}", e));
        // 检查网络连接或切换备用服务
    }
    Err(DMSCError::HttpRedirectError(e)) => {
        ctx.log().warn(format!("Too many redirects: {}", e));
        // 处理重定向循环
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected HTTP error: {}", e));
        return Err(e);
    }
}

// 重试机制
async fn retry_request(url: &str, max_retries: u32, ctx: &DMSCContext) -> DMSCResult<DMSCHttpResponse> {
    let mut retry_count = 0;
    
    loop {
        match ctx.http().get(url).send().await {
            Ok(response) => {
                if response.status.is_success() {
                    return Ok(response);
                } else if response.status.is_server_error() && retry_count < max_retries {
                    retry_count += 1;
                    let delay = Duration::from_millis(1000 * 2_u64.pow(retry_count));
                    ctx.log().warn(format!("Request failed with {}, retrying in {:?} (attempt {})", 
                        response.status, delay, retry_count));
                    sleep(delay).await;
                    continue;
                } else {
                    return Ok(response);
                }
            }
            Err(e) if retry_count < max_retries => {
                retry_count += 1;
                let delay = Duration::from_millis(1000 * 2_u64.pow(retry_count));
                ctx.log().warn(format!("Request failed with {}, retrying in {:?} (attempt {})", 
                    e, delay, retry_count));
                sleep(delay).await;
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
```

## 性能优化

### 连接池优化

```rust
use dmsc::prelude::*;

// HTTP客户端连接池配置
let client_config = DMSCHttpClientConfig {
    max_connections: 200,              // 增加最大连接数
    connection_timeout: Duration::from_secs(5),
    idle_timeout: Duration::from_secs(30), // 减少空闲超时时间
    retry_attempts: 3,
    retry_delay: Duration::from_millis(500),
    enable_connection_pooling: true,
    pool_max_idle_per_host: 20,        // 每个主机的最大空闲连接数
    pool_max_lifetime: Duration::from_secs(300), // 连接最大生命周期
};

ctx.http().init_client(client_config).await?;
```

### 缓存策略

```rust
use dmsc::prelude::*;
use serde_json::json;

// HTTP响应缓存
async fn cached_request(url: &str, cache_key: &str, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    // 尝试从缓存获取
    if let Some(cached_data) = ctx.cache().get_json(cache_key).await? {
        ctx.log().debug(format!("Cache hit for key: {}", cache_key));
        return Ok(cached_data);
    }
    
    // 缓存未命中，发起请求
    let response = ctx.http().get(url).send().await?;
    let data = response.json::<serde_json::Value>().await?;
    
    // 缓存响应数据（5分钟）
    ctx.cache().set_json(cache_key, &data, Duration::from_minutes(5)).await?;
    
    ctx.log().debug(format!("Cache miss for key: {}, fetched from API", cache_key));
    Ok(data)
}

// 条件请求（ETag/Last-Modified）
async fn conditional_request(url: &str, etag_key: &str, ctx: &DMSCContext) -> DMSCResult<Option<serde_json::Value>> {
    // 获取缓存的ETag
    let cached_etag = ctx.cache().get(etag_key).await?;
    
    let mut request = ctx.http().get(url);
    
    if let Some(etag) = cached_etag {
        request = request.header("If-None-Match", &etag);
    }
    
    let response = request.send().await?;
    
    match response.status {
        StatusCode::NOT_MODIFIED => {
            ctx.log().debug("Resource not modified, using cached version");
            Ok(None)
        }
        StatusCode::OK => {
            // 更新缓存的ETag
            if let Some(etag) = response.headers.get("ETag") {
                ctx.cache().set(etag_key, etag, Duration::from_hours(1)).await?;
            }
            
            let data = response.json::<serde_json::Value>().await?;
            Ok(Some(data))
        }
        _ => {
            Err(DMSCError::http_error(format!("Unexpected status: {}", response.status)))
        }
    }
}
```

<div align="center">

## 运行步骤

</div>

1. **创建项目**: 使用Cargo创建新的Rust项目
2. **添加依赖**: 在Cargo.toml中添加必要的依赖项
3. **创建配置**: 创建config.yaml配置文件
4. **编写代码**: 实现HTTP服务器和客户端功能
5. **运行应用**: 使用cargo run启动应用
6. **测试API**: 使用curl或Postman测试HTTP端点

<div align="center">

## 预期结果

</div>

运行成功后，你将看到以下输出：

```
[INFO] DMSC HTTP Example started
[INFO] HTTP server initialized on port 8080
[INFO] API routes configured successfully
[INFO] HTTP server is running on http://localhost:8080
```

API测试示例：

```bash
# 测试根路径
curl http://localhost:8080/

# 获取用户信息
curl http://localhost:8080/users/1

# 创建用户
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'

# 文件上传
curl -X POST http://localhost:8080/upload/single \
  -F "file=@document.pdf"

# WebSocket连接
websocat ws://localhost:8080/ws
```

<div align="center">

## 扩展功能

</div>

### 实现负载均衡支持

```rust
use dmsc::prelude::*;
use serde_json::json;

async fn setup_load_balancer(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    let backend_servers = vec![
        "http://backend1.example.com:8080",
        "http://backend2.example.com:8080",
        "http://backend3.example.com:8080",
    ];
    
    let lb_config = DMSCLoadBalancerConfig {
        algorithm: DMSCLoadBalancerAlgorithm::RoundRobin,
        health_check_interval: Duration::from_secs(30),
        health_check_timeout: Duration::from_secs(5),
        unhealthy_threshold: 3,
        healthy_threshold: 2,
        max_connections_per_server: 100,
    };
    
    ctx.http().setup_load_balancer(backend_servers, lb_config).await?;
    
    // 健康检查端点
    ctx.http().get("/health", |req, ctx| async move {
        let health_status = json!({
            "status": "healthy",
            "backend_servers": ctx.http().get_backend_health().await?,
            "active_connections": ctx.http().get_active_connections().await?,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });
        
        Ok(DMSCHttpResponse::ok(health_status))
    });
    
    Ok(())
}
```

### 实现API网关

```rust
use dmsc::prelude::*;
use serde_json::json;

async fn setup_api_gateway(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    let gateway_config = DMSCApiGatewayConfig {
        rate_limiting: DMSCRateLimitConfig {
            requests_per_minute: 1000,
            burst: 100,
            key_extractor: |req| req.headers.get("X-API-Key").unwrap_or(&"anonymous".to_string()).clone(),
        },
        circuit_breaker: DMSCCircuitBreakerConfig {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(60),
            success_threshold: 2,
        },
        request_timeout: Duration::from_secs(30),
        enable_cors: true,
        enable_logging: true,
    };
    
    ctx.http().setup_api_gateway(gateway_config).await?;
    
    // 路由配置
    let routes = vec![
        ("/api/users/*", "http://user-service:8080"),
        ("/api/orders/*", "http://order-service:8080"),
        ("/api/products/*", "http://product-service:8080"),
        ("/api/notifications/*", "http://notification-service:8080"),
    ];
    
    for (path, target) in routes {
        ctx.http().proxy(path, target).await?;
    }
    
    // API文档端点
    ctx.http().get("/api/docs", |req, ctx| async move {
        let api_docs = json!({
            "openapi": "3.0.0",
            "info": {
                "title": "DMSC API Gateway",
                "version": "1.0.0",
                "description": "Unified API gateway for microservices",
            },
            "servers": [
                {"url": "https://api.example.com", "description": "Production server"}
            ],
            "paths": {
                "/api/users": {
                    "get": {
                        "summary": "Get users",
                        "responses": {
                            "200": {"description": "Success"}
                        }
                    }
                }
            }
        });
        
        Ok(DMSCHttpResponse::ok(api_docs))
    });
    
    Ok(())
}
```

### 实现GraphQL支持

```rust
use dmsc::prelude::*;
use async_graphql::*;
use serde_json::json;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: i32) -> FieldResult<User> {
        let dms_ctx = ctx.data::<DMSCServiceContext>()?;
        let user_data = dms_ctx.database()
            .query_one("SELECT id, name, email FROM users WHERE id = $1", vec![id.into()])
            .await?
            .ok_or_else(|| FieldError::new("User not found"))?;
        
        Ok(User {
            id: user_data.get::<i32>("id"),
            name: user_data.get::<String>("name"),
            email: user_data.get::<String>("email"),
        })
    }
    
    async fn users(&self, ctx: &Context<'_>, limit: Option<i32>) -> FieldResult<Vec<User>> {
        let dms_ctx = ctx.data::<DMSCServiceContext>()?;
        let limit = limit.unwrap_or(10);
        
        let users_data = dms_ctx.database()
            .query("SELECT id, name, email FROM users LIMIT $1", vec![limit.into()])
            .await?;
        
        let users: Vec<User> = users_data.iter().map(|user| User {
            id: user.get::<i32>("id"),
            name: user.get::<String>("name"),
            email: user.get::<String>("email"),
        }).collect();
        
        Ok(users)
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, name: String, email: String) -> FieldResult<User> {
        let dms_ctx = ctx.data::<DMSCServiceContext>()?;
        
        let user_id = dms_ctx.database()
            .execute(
                "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
                vec![name.clone().into(), email.clone().into()]
            )
            .await?;
        
        Ok(User {
            id: user_id,
            name,
            email,
        })
    }
}

async fn setup_graphql(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(ctx.clone())
        .finish();
    
    ctx.http().post("/graphql", |req, ctx| async move {
        let body = req.json::<serde_json::Value>().await?;
        let query = body["query"].as_str().unwrap_or("");
        let variables = body["variables"].as_object().cloned().unwrap_or_default();
        
        let request = async_graphql::Request::new(query)
            .variables(variables);
        
        let response = schema.execute(request).await;
        
        Ok(DMSCHttpResponse::ok(serde_json::to_value(response)?))
    });
    
    // GraphQL Playground
    ctx.http().get("/graphql", |req, ctx| async move {
        Ok(DMSCHttpResponse::ok(async_graphql::http::playground_source(
            GraphQLPlaygroundConfig::new("/graphql")
        )))
    });
    
    Ok(())
}
```

### 实现实时分析

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
struct HttpAnalytics {
    request_count: Arc<RwLock<u64>>,
    response_times: Arc<RwLock<Vec<Duration>>>,
    error_count: Arc<RwLock<u64>>,
    active_connections: Arc<RwLock<u32>>,
}

impl HttpAnalytics {
    fn new() -> Self {
        Self {
            request_count: Arc::new(RwLock::new(0)),
            response_times: Arc::new(RwLock::new(Vec::new())),
            error_count: Arc::new(RwLock::new(0)),
            active_connections: Arc::new(RwLock::new(0)),
        }
    }
    
    async fn record_request(&self) {
        let mut count = self.request_count.write().await;
        *count += 1;
    }
    
    async fn record_response_time(&self, duration: Duration) {
        let mut times = self.response_times.write().await;
        times.push(duration);
        
        // 保持最近1000个响应时间
        if times.len() > 1000 {
            times.remove(0);
        }
    }
    
    async fn record_error(&self) {
        let mut count = self.error_count.write().await;
        *count += 1;
    }
    
    async fn get_stats(&self) -> serde_json::Value {
        let request_count = *self.request_count.read().await;
        let error_count = *self.error_count.read().await;
        let active_connections = *self.active_connections.read().await;
        
        let response_times = self.response_times.read().await;
        let avg_response_time = if !response_times.is_empty() {
            response_times.iter().sum::<Duration>() / response_times.len() as u32
        } else {
            Duration::from_secs(0)
        };
        
        json!({
            "request_count": request_count,
            "error_count": error_count,
            "error_rate": if request_count > 0 { error_count as f64 / request_count as f64 } else { 0.0 },
            "active_connections": active_connections,
            "average_response_time_ms": avg_response_time.as_millis(),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        })
    }
}

async fn setup_analytics(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    let analytics = HttpAnalytics::new();
    
    // 分析中间件
    ctx.http().use_custom_middleware(|req, ctx, next| async move {
        let start_time = std::time::Instant::now();
        
        // 记录请求
        if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
            analytics.record_request().await;
        }
        
        // 执行请求
        let result = next.run(req, ctx.clone()).await;
        
        // 记录响应时间和错误
        if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
            let duration = start_time.elapsed();
            analytics.record_response_time(duration).await;
            
            if let Err(_) = &result {
                analytics.record_error().await;
            }
        }
        
        result
    });
    
    // 分析API端点
    ctx.http().get("/analytics", |req, ctx| async move {
        if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
            let stats = analytics.get_stats().await;
            Ok(DMSCHttpResponse::ok(stats))
        } else {
            Ok(DMSCHttpResponse::internal_server_error(json!({
                "error": "Analytics not available"
            })))
        }
    });
    
    // 实时仪表板WebSocket
    ctx.http().websocket("/analytics/live", |ws_stream, ctx| async move {
        let (mut sender, mut receiver) = ws_stream.split();
        
        // 定期发送分析数据
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
                        let stats = analytics.get_stats().await;
                        
                        if sender.send(DMSCWebSocketMessage::Text(stats.to_string())).await.is_err() {
                            break;
                        }
                    }
                }
                
                msg = receiver.next() => {
                    match msg {
                        Some(Ok(DMSCWebSocketMessage::Close(_))) => break,
                        Some(Err(_)) => break,
                        _ => {}
                    }
                }
            }
        }
        
        Ok(())
    });
    
    Ok(())
}
```

<div align="center">

## 最佳实践

</div>

1. **错误处理**: 妥善处理HTTP错误，实现重试机制
2. **超时设置**: 为所有请求设置合理的超时时间
3. **连接池**: 使用连接池提高性能
4. **认证安全**: 安全地处理认证信息
5. **输入验证**: 验证所有用户输入
6. **日志记录**: 记录重要的请求和响应信息
7. **监控指标**: 收集HTTP性能指标
8. **限流保护**: 实施速率限制防止滥用
9. **压缩支持**: 启用响应压缩减少带宽使用
10. **缓存策略**: 合理使用缓存提高性能

<div align="center">

## 总结

</div>

本示例全面展示了DMSC框架的HTTP服务功能，包括服务器配置、路由管理、客户端使用、中间件、WebSocket通信、文件上传下载等核心功能。通过实际代码示例，你可以学习如何：

- 配置和启动HTTP服务器
- 实现RESTful API接口
- 使用HTTP客户端进行外部请求
- 实现自定义中间件
- 处理WebSocket实时通信
- 管理文件上传下载
- 实现高级功能如负载均衡、API网关、GraphQL等
- 进行性能优化和错误处理

这些功能为构建现代化的Web应用和微服务架构提供了强大的支持。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用

- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信