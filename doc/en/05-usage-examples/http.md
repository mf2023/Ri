<div align="center">

# HTTP Service Usage Example

**Version: 0.1.6**

**Last modified date: 2026-01-30**

This example demonstrates how to use the DMSC http module for HTTP server, client, routing management, middleware, WebSocket, and file upload/download functionality.

## Example Overview

</div>

This example will create a DMSC application with the following features:

- HTTP server configuration and routing management
- RESTful API design and implementation
- Middleware and request processing
- WebSocket real-time communication
- File upload/download processing
- Error handling and response formatting

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of HTTP protocol and RESTful API concepts
- (Optional) Postman or curl for API testing

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-http-example
cd dms-http-example
```

### 2. Add Dependencies

Add the following dependencies to the `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

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

### 4. Write Main Code

Replace the `src/main.rs` file with the following content:

```rust
use dmsc::prelude::*;
use serde_json::json;
use chrono::Utc;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_http(DMSCHttpConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC HTTP Example started")?;
        
        // Initialize HTTP server
        initialize_http_server(&ctx).await?;
        
        // Configure API routes
        setup_api_routes(&ctx).await?;
        
        // Start HTTP server
        ctx.logger().info("service", "HTTP server is running on http://localhost:8080")?;
        
        // Keep service running
        tokio::signal::ctrl_c().await?;
        ctx.logger().info("service", "Shutting down HTTP server")?;
        
        Ok(())
    }).await
}

async fn initialize_http_server(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("http", "Initializing HTTP server")?;
    
    // Create HTTP server configuration
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
    
    // Initialize HTTP server
    ctx.http().init_server(server_config).await?;
    ctx.logger().info("http", "HTTP server initialized on port 8080")?;
    
    Ok(())
}

async fn setup_api_routes(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("http", "Setting up API routes")?;
    
    // GET request handler - Root path
    ctx.http().get("/", |req, ctx| async move {
        Ok(DMSCHttpResponse::ok(json!({
            "message": "Welcome to DMSC API",
            "version": "1.0.0",
            "timestamp": Utc::now().to_rfc3339(),
        })))
    });
    
    // GET request handler - Get user information
    ctx.http().get("/users/:id", |req, ctx| async move {
        let user_id = req.params.get("id")
            .and_then(|id| id.parse::<i32>().ok())
            .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
        
        // Get user information from database
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
    
    // POST request handler - Create user
    ctx.http().post("/users", |req, ctx| async move {
        let body = req.json::<serde_json::Value>()
            .await
            .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
        
        // Validate required fields
        let name = body["name"].as_str()
            .ok_or_else(|| DMSCError::bad_request("Name is required".to_string()))?;
        let email = body["email"].as_str()
            .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
        
        // Validate email format
        if !email.contains('@') {
            return Ok(DMSCHttpResponse::bad_request(json!({
                "error": "Invalid email format"
            })));
        }
        
        // Insert user data
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

## Code Analysis

The HTTP module provides usage examples for HTTP server, client, routing management, middleware, WebSocket, and file upload/download functionality.

## HTTP Server

### Basic Server Configuration

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create HTTP server configuration
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

// Initialize HTTP server
ctx.http().init_server(server_config).await?;
ctx.log().info("HTTP server initialized on port 8080");
```

### Basic Routing

```rust
use dmsc::prelude::*;
use serde_json::json;

// GET request handler
ctx.http().get("/", |req, ctx| async move {
    Ok(DMSCHttpResponse::ok(json!({
        "message": "Welcome to DMSC API",
        "version": "1.0.0",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })))
});

// GET request with parameters
ctx.http().get("/users/:id", |req, ctx| async move {
    let user_id = req.params.get("id")
        .and_then(|id| id.parse::<i32>().ok())
        .ok_or_else(|| DMSCError::bad_request("Invalid user ID".to_string()))?;
    
    // Get user information from database
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

// POST request handler
ctx.http().post("/users", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    // Validate required fields
    let name = body["name"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Name is required".to_string()))?;
    let email = body["email"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
    
    // Validate email format
    if !email.contains('@') {
        return Ok(DMSCHttpResponse::bad_request(json!({
            "error": "Invalid email format"
        })));
    }
    
    // Insert user data
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

// PUT request handler
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
    
    // Build dynamic update query
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
    
    // Add WHERE condition
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

// DELETE request handler
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

### Route Groups

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create API route group
let api_routes = ctx.http().group("/api/v1");

// Add routes to route group
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
    // Get system statistics
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

// User management route group
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

// Authentication route group
let auth_routes = api_routes.group("/auth");

auth_routes.post("/login", |req, ctx| async move {
    let body = req.json::<serde_json::Value>()
        .await
        .map_err(|e| DMSCError::bad_request(format!("Invalid JSON: {}", e)))?;
    
    let email = body["email"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Email is required".to_string()))?;
    let password = body["password"].as_str()
        .ok_or_else(|| DMSCError::bad_request("Password is required".to_string()))?;
    
    // Validate user credentials
    let user = ctx.database()
        .query_one(
            "SELECT id, name, email, password_hash FROM users WHERE email = $1",
            vec![email.into()]
        )
        .await?;
    
    if let Some(user_data) = user {
        let stored_hash = user_data.get::<String>("password_hash").unwrap_or_default();
        
        // Validate password (using bcrypt or similar password hashing library)
        if verify_password(password, &stored_hash)? {
            // Generate JWT token
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
    
    // Validate input
    if password.len() < 8 {
        return Ok(DMSCHttpResponse::bad_request(json!({
            "error": "Password must be at least 8 characters long"
        })));
    }
    
    // Check if email already exists
    let existing_user = ctx.database()
        .query_one("SELECT id FROM users WHERE email = $1", vec![email.into()])
        .await?;
    
    if existing_user.is_some() {
        return Ok(DMSCHttpResponse::conflict(json!({
            "error": "Email already registered"
        })));
    }
    
    // Hash password
    let password_hash = hash_password(password)?;
    
    // Create user
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

## HTTP Client

### Basic Client Usage

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create HTTP client configuration
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

// Initialize HTTP client
ctx.http().init_client(client_config).await?;

// GET request
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

// POST request
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

### Advanced Client Features

```rust
use dmsc::prelude::*;
use serde_json::json;

// Form data submission
let form_response = ctx.http().post("https://httpbin.org/post")
    .form(&json!({
        "username": "testuser",
        "password": "testpass",
        "remember": "true"
    }))
    .send()
    .await?;

// File upload
let upload_response = ctx.http().post("https://httpbin.org/post")
    .multipart_form()
    .file("file", "./data/document.pdf", "application/pdf")
    .field("description", "Important document")
    .field("category", "documents")
    .send()
    .await?;

// Custom headers and authentication
let auth_response = ctx.http().get("https://api.example.com/protected")
    .header("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")
    .header("X-API-Key", "your-api-key")
    .header("X-Request-ID", "unique-request-id")
    .send()
    .await?;

// Stream download large files
let download_response = ctx.http().get("https://example.com/large-file.zip")
    .send_stream()
    .await?;

let mut file = std::fs::File::create("./downloads/large-file.zip")?;
let mut stream = download_response.stream();

while let Some(chunk) = stream.next().await {
    let chunk = chunk?;
    file.write_all(&chunk)?;
}

// Concurrent requests
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

## Middleware

### Built-in Middleware

```rust
use dmsc::prelude::*;
use serde_json::json;

// Logging middleware
ctx.http().use_middleware(DMSCHttpMiddleware::logging());

// CORS middleware
ctx.http().use_middleware(DMSCHttpMiddleware::cors()
    .allow_origin("https://example.com")
    .allow_methods(vec!["GET", "POST", "PUT", "DELETE"])
    .allow_headers(vec!["Content-Type", "Authorization"])
    .max_age(3600)
);

// Authentication middleware
ctx.http().use_middleware(DMSCHttpMiddleware::auth()
    .exclude_paths(vec!["/api/v1/auth/login", "/api/v1/auth/register"])
    .token_header("Authorization")
    .token_prefix("Bearer ")
);

// Rate limiting middleware
ctx.http().use_middleware(DMSCHttpMiddleware::rate_limit()
    .requests_per_minute(60)
    .burst(10)
    .key_extractor(|req| {
        // Rate limiting based on IP address
        req.remote_addr.to_string()
    })
);

// Compression middleware
ctx.http().use_middleware(DMSCHttpMiddleware::compression()
    .threshold(1024)
    .level(6)
);

// Request ID middleware
ctx.http().use_middleware(DMSCHttpMiddleware::request_id()
    .header_name("X-Request-ID")
    .generator(|| uuid::Uuid::new_v4().to_string())
);
```

### Custom Middleware

```rust
use dmsc::prelude::*;
use serde_json::json;

// Custom authentication middleware
async fn auth_middleware(
    req: DMSCHttpRequest,
    ctx: DMSCContext,
    next: DMSCNext,
) -> DMSCResult<DMSCHttpResponse> {
    // Skip paths that don't require authentication
    let skip_auth = vec!["/api/v1/auth/login", "/api/v1/auth/register", "/health"];
    if skip_auth.iter().any(|path| req.path.starts_with(path)) {
        return next.run(req, ctx).await;
    }
    
    // Check authentication header
    let auth_header = req.headers.get("Authorization")
        .ok_or_else(|| DMSCError::unauthorized("Missing Authorization header".to_string()))?;
    
    let token = auth_header.strip_prefix("Bearer ")
        .ok_or_else(|| DMSCError::unauthorized("Invalid Authorization format".to_string()))?;
    
    // Validate JWT token
    match ctx.auth().validate_jwt(token).await {
        Ok(user_id) => {
            // Add user information to request context
            let mut req = req;
            req.context.insert("user_id", user_id.to_string());
            
            // Continue processing request
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

// Custom logging middleware
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
    
    // Execute request
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

// Custom error handling middleware
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

// Use custom middleware
ctx.http().use_custom_middleware(auth_middleware);
ctx.http().use_custom_middleware(logging_middleware);
ctx.http().use_custom_middleware(error_handling_middleware);
```

## WebSocket Support

### WebSocket Server

```rust
use dmsc::prelude::*;
use serde_json::json;
use futures::{StreamExt, SinkExt};

// WebSocket connection handling
ctx.http().websocket("/ws", |ws_stream, ctx| async move {
    let (mut sender, mut receiver) = ws_stream.split();
    let client_id = uuid::Uuid::new_v4().to_string();
    
    ctx.log().info(format!("WebSocket client connected: {}", client_id));
    
    // Send welcome message
    let welcome_msg = json!({
        "type": "welcome",
        "client_id": client_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    sender.send(DMSCWebSocketMessage::Text(welcome_msg.to_string())).await?;
    
    // Process received messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(DMSCWebSocketMessage::Text(text)) => {
                ctx.log().info(format!("Received message from {}: {}", client_id, text));
                
                // Parse message
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
                
                // Echo binary data
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

// WebSocket Chat Room
ctx.http().websocket("/chat/:room", |ws_stream, ctx| async move {
    let (mut sender, mut receiver) = ws_stream.split();
    let room_id = ctx.params.get("room").unwrap_or("default").to_string();
    let user_id = ctx.context.get("user_id").unwrap_or(&"anonymous".to_string()).to_string();
    
    // Add user to chat room
    ctx.cache().add_to_set(&format!("chat_room:{}", room_id), &user_id).await?;
    
    // Broadcast user join message
    let join_msg = json!({
        "type": "user_joined",
        "user_id": user_id,
        "room_id": room_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    broadcast_to_room(&room_id, join_msg, &ctx).await?;
    
    // Process messages
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
    
    // User leaves chat room
    ctx.cache().remove_from_set(&format!("chat_room:{}", room_id), &user_id).await?;
    
    let leave_msg = json!({
        "type": "user_left",
        "user_id": user_id,
        "room_id": room_id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    broadcast_to_room(&room_id, leave_msg, &ctx).await?;
});

// Broadcast function
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

### WebSocket Client

```rust
use dmsc::prelude::*;
use futures::{StreamExt, SinkExt};

// Connect to WebSocket server
let ws_client = ctx.http().websocket_client("wss://echo.websocket.org").await?;
let (mut sender, mut receiver) = ws_client.split();

// Send message
sender.send(DMSCWebSocketMessage::Text("Hello WebSocket!".to_string())).await?;

// Receive messages
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

## File Upload/Download

### File Upload

```rust
use dmsc::prelude::*;
use serde_json::json;

// Single file upload
ctx.http().post("/upload/single", |req, ctx| async move {
    let mut multipart = req.multipart()?;
    
    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default().to_string();
        let filename = field.file_name().unwrap_or_default().to_string();
        let content_type = field.content_type().unwrap_or_default().to_string();
        
        ctx.log().info(format!("Uploading file: {} ({})", filename, content_type));
        
        // Read file data
        let data = field.bytes().await?;
        
        // Save to storage system
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

// Multiple file upload
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

// Chunked upload (large files)
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
    
    // Decode base64 data
    let data = base64::decode(chunk_data)
        .map_err(|e| DMSCError::bad_request(format!("Invalid base64 data: {}", e)))?;
    
    // Upload chunk
    ctx.storage().upload_chunk(
        upload_id,
        chunk_index,
        &data,
        total_chunks
    ).await?;
    
    // Check if all chunks have been uploaded
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

### File Download

```rust
use dmsc::prelude::*;
use serde_json::json;

// File download
ctx.http().get("/download/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    // Get file information
    let file_info = ctx.storage().get_file_info(file_id).await?;
    
    // Set response headers
    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), file_info.content_type.clone());
    headers.insert("Content-Disposition".to_string(), 
        format!("attachment; filename=\"{}\"", file_info.filename));
    
    // Download file data
    let file_data = ctx.storage().download(file_id).await?;
    
    Ok(DMSCHttpResponse::ok_with_headers(file_data, headers))
});

// Resumable download (range request)
ctx.http().get("/download/range/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    // Parse Range header
    let range_header = req.headers.get("Range")
        .ok_or_else(|| DMSCError::bad_request("Range header is required".to_string()))?;
    
    let (start, end) = parse_range_header(range_header)?;
    
    // Get file information
    let file_info = ctx.storage().get_file_info(file_id).await?;
    
    // Validate range
    if start >= file_info.size as u64 {
        return Ok(DMSCHttpResponse::range_not_satisfiable());
    }
    
    let end = end.unwrap_or(file_info.size as u64 - 1);
    let content_length = end - start + 1;
    
    // Download specified range
    let data = ctx.storage().download_range(file_id, start, end).await?;
    
    // Set response headers
    let mut headers = std::collections::HashMap::new();
    headers.insert("Content-Type".to_string(), file_info.content_type.clone());
    headers.insert("Content-Range".to_string(), 
        format!("bytes {}-{}/{}", start, end, file_info.size));
    headers.insert("Content-Length".to_string(), content_length.to_string());
    headers.insert("Accept-Ranges".to_string(), "bytes".to_string());
    
    Ok(DMSCHttpResponse::partial_content_with_headers(data, headers))
});

// Temporary download link
ctx.http().get("/download/temp/:file_id", |req, ctx| async move {
    let file_id = req.params.get("file_id")
        .ok_or_else(|| DMSCError::bad_request("File ID is required".to_string()))?;
    
    let expires_in = req.query.get("expires")
        .and_then(|e| e.parse::<u64>().ok())
        .unwrap_or(3600); // Default 1 hour
    
    // Generate temporary download link
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

## Advanced Features

### Proxy Requests

```rust
use dmsc::prelude::*;

// HTTP proxy
ctx.http().get("/proxy/*path", |req, ctx| async move {
    let target_path = req.params.get("path")
        .ok_or_else(|| DMSCError::bad_request("Target path is required".to_string()))?;
    
    let target_url = format!("https://api.target-service.com/{}", target_path);
    
    // Forward request
    let proxy_response = ctx.http().client()
        .request(req.method.clone(), &target_url)
        .headers(req.headers.clone())
        .body(req.body.clone())
        .send()
        .await?;
    
    // Return proxy response
    Ok(DMSCHttpResponse::new(
        proxy_response.status,
        proxy_response.headers,
        proxy_response.body
    ))
});
```

### Server-Sent Events (SSE)

```rust
use dmsc::prelude::*;
use serde_json::json;

// SSE endpoint
ctx.http().get("/events", |req, ctx| async move {
    let mut event_stream = DMSCHttpResponse::event_stream();
    
    // Send initial event
    event_stream.send_event("connected", json!({
        "message": "Connected to event stream",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })).await?;
    
    // Send periodic events
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
            Err(_) => break, // Client disconnected
        }
    }
    
    Ok(event_stream)
});

// Send custom event
async fn send_custom_event(event_type: &str, data: serde_json::Value, ctx: &DMSCContext) -> DMSCResult<()> {
    let event = json!({
        "type": event_type,
        "data": data,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    // Broadcast to all connected clients
    ctx.http().broadcast_event("notification", event).await?;
    
    Ok(())
}
```

## Error Handling

### HTTP Error Handling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Error handling example
match ctx.http().get("https://api.example.com/data").send().await {
    Ok(response) => {
        if response.status.is_success() {
            let data = response.json::<serde_json::Value>().await?;
            ctx.log().info(format!("Data received: {:?}", data));
        } else if response.status.is_client_error() {
            ctx.log().warn(format!("Client error: {}", response.status));
            // Handle client error
        } else if response.status.is_server_error() {
            ctx.log().error(format!("Server error: {}", response.status));
            // Handle server error
        }
    }
    Err(DMSCError::HttpTimeoutError(e)) => {
        ctx.log().error(format!("Request timeout: {}", e));
        // Retry or fallback handling
    }
    Err(DMSCError::HttpConnectionError(e)) => {
        ctx.log().error(format!("Connection error: {}", e));
        // Check network connection or switch to backup service
    }
    Err(DMSCError::HttpRedirectError(e)) => {
        ctx.log().warn(format!("Too many redirects: {}", e));
        // Handle redirect loop
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected HTTP error: {}", e));
        return Err(e);
    }
}

// Retry mechanism
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

## Performance Optimization

### Connection Pool Optimization

```rust
use dmsc::prelude::*;

// HTTP client connection pool configuration
let client_config = DMSCHttpClientConfig {
    max_connections: 200,              // Increase maximum connections
    connection_timeout: Duration::from_secs(5),
    idle_timeout: Duration::from_secs(30), // Reduce idle timeout
    retry_attempts: 3,
    retry_delay: Duration::from_millis(500),
    enable_connection_pooling: true,
    pool_max_idle_per_host: 20,        // Maximum idle connections per host
    pool_max_lifetime: Duration::from_secs(300), // Connection maximum lifetime
};

ctx.http().init_client(client_config).await?;
```

### Cache Strategy

```rust
use dmsc::prelude::*;
use serde_json::json;

// HTTP response cache
async fn cached_request(url: &str, cache_key: &str, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    // Try to get from cache
    if let Some(cached_data) = ctx.cache().get_json(cache_key).await? {
        ctx.log().debug(format!("Cache hit for key: {}", cache_key));
        return Ok(cached_data);
    }
    
    // Cache miss, make request
    let response = ctx.http().get(url).send().await?;
    let data = response.json::<serde_json::Value>().await?;
    
    // Cache response data (5 minutes)
    ctx.cache().set_json(cache_key, &data, Duration::from_minutes(5)).await?;
    
    ctx.log().debug(format!("Cache miss for key: {}, fetched from API", cache_key));
    Ok(data)
}

// Conditional request (ETag/Last-Modified)
async fn conditional_request(url: &str, etag_key: &str, ctx: &DMSCContext) -> DMSCResult<Option<serde_json::Value>> {
    // Get cached ETag
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
            // Update cached ETag
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

## Running Steps

</div>

1. **Create Project**: Use Cargo to create a new Rust project
2. **Add Dependencies**: Add necessary dependencies in Cargo.toml
3. **Create Configuration**: Create config.yaml configuration file
4. **Write Code**: Implement HTTP server and client functionality
5. **Run Application**: Start application using cargo run
6. **Test API**: Test HTTP endpoints using curl or Postman

<div align="center">

## Expected Results

</div>

After successful execution, you will see the following output:

```
[INFO] DMSC HTTP Example started
[INFO] HTTP server initialized on port 8080
[INFO] API routes configured successfully
[INFO] HTTP server is running on http://localhost:8080
```

API testing examples:

```bash
# Test root path
curl http://localhost:8080/

# Get user information
curl http://localhost:8080/users/1

# Create user
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'

# File upload
curl -X POST http://localhost:8080/upload/single \
  -F "file=@document.pdf"

# WebSocket connection
websocat ws://localhost:8080/ws
```

<div align="center">

## Extended Features

</div>

### Implement Load Balancing Support

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
    
    // Health check endpoint
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

### Implement API Gateway

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
    
    // Route configuration
    let routes = vec![
        ("/api/users/*", "http://user-service:8080"),
        ("/api/orders/*", "http://order-service:8080"),
        ("/api/products/*", "http://product-service:8080"),
        ("/api/notifications/*", "http://notification-service:8080"),
    ];
    
    for (path, target) in routes {
        ctx.http().proxy(path, target).await?;
    }
    
    // API documentation endpoint
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

### Implement GraphQL Support

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

### Implement Real-time Analytics

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
        
        // Keep the most recent 1000 response times
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
    
    // Analytics middleware
    ctx.http().use_custom_middleware(|req, ctx, next| async move {
        let start_time = std::time::Instant::now();
        
        // Record request
        if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
            analytics.record_request().await;
        }
        
        // Execute request
        let result = next.run(req, ctx.clone()).await;
        
        // Record response time and errors
        if let Some(analytics) = ctx.get_extension::<HttpAnalytics>() {
            let duration = start_time.elapsed();
            analytics.record_response_time(duration).await;
            
            if let Err(_) = &result {
                analytics.record_error().await;
            }
        }
        
        result
    });
    
    // Analytics API endpoint
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
    
    // Real-time dashboard WebSocket
    ctx.http().websocket("/analytics/live", |ws_stream, ctx| async move {
        let (mut sender, mut receiver) = ws_stream.split();
        
        // Send analytics data periodically
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

## Best Practices

</div>

1. **Error Handling**: Handle HTTP errors properly and implement retry mechanisms
2. **Timeout Settings**: Set reasonable timeout times for all requests
3. **Connection Pool**: Use connection pools to improve performance
4. **Authentication Security**: Handle authentication information securely
5. **Input Validation**: Validate all user inputs
6. **Logging**: Record important request and response information
7. **Monitoring Metrics**: Collect HTTP performance metrics
8. **Rate Limiting**: Implement rate limiting to prevent abuse
9. **Compression Support**: Enable response compression to reduce bandwidth usage
10. **Caching Strategy**: Use caching appropriately to improve performance

<div align="center">

## Summary

</div>

This example comprehensively demonstrates the HTTP service functionality of the DMSC framework, including server configuration, route management, client usage, middleware, WebSocket communication, file upload/download, and other core features. Through practical code examples, you can learn how to:

- Configure and start HTTP servers
- Implement RESTful API interfaces
- Use HTTP clients for external requests
- Implement custom middleware
- Handle WebSocket real-time communication
- Manage file upload and download
- Implement advanced features like load balancing, API gateway, GraphQL, etc.
- Perform performance optimization and error handling

These features provide powerful support for building modern web applications and microservice architectures.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, understand how to use caching module to improve application performance
- [database](./database.md): Database examples, learn database connection and query operations
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication

- [mq](./mq.md): Message queue examples, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing and security best practices
- [storage](./storage.md): Storage examples, file upload/download and storage management
- [validation](./validation.md): Validation examples, data validation and cleanup operations