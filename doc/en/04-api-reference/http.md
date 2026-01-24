<div align="center">

# HTTP API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-24**

The http module provides HTTP client and server functionality, supporting routing, middleware, WebSocket, and file upload/download.

## Module Overview

</div>

The http module contains the following sub-modules:

- **server**: HTTP server
- **client**: HTTP client
- **router**: Route management
- **middleware**: Middleware
- **websocket**: WebSocket support
- **upload**: File upload/download

<div align="center">

## Core Components

</div>

### DMSCHttpServer

HTTP server interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create HTTP server | `config: DMSCHttpServerConfig` | `Self` |
| `route(method, path, handler)` | Add route | `method: HttpMethod`, `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `get(path, handler)` | Add GET route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `post(path, handler)` | Add POST route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `put(path, handler)` | Add PUT route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `delete(path, handler)` | Add DELETE route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `use_middleware(middleware)` | Use middleware | `middleware: impl HttpMiddleware` | `&Self` |
| `listen(addr)` | Start server listening | `addr: &str` | `DMSCResult<()>` |
| `shutdown()` | Shutdown server | None | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Create HTTP server configuration
let server_config = DMSCHttpServerConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    max_connections: 1000,
    request_timeout: Duration::from_secs(30),
    ..Default::default()
};

// Create HTTP server
let server = DMSCHttpServer::new(server_config);

// Add routes
server.get("/", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    res.status(200).json(serde_json::json!({
        "message": "Hello, World!"
    }))
});

server.get("/users/:id", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let user_id = req.params.get("id").unwrap();
    
    res.status(200).json(serde_json::json!({
        "user_id": user_id,
        "name": "John Doe",
        "email": "john@example.com"
    }))
});

// Start server
server.listen("0.0.0.0:8080").await?;
```

### DMSCHttpClient

HTTP client interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create HTTP client | `config: DMSCHttpClientConfig` | `Self` |
| `get(url)` | Send GET request | `url: &str` | `DMSCResult<DMSCHttpResponse>` |
| `post(url, body)` | Send POST request | `url: &str`, `body: impl Serialize` | `DMSCResult<DMSCHttpResponse>` |
| `put(url, body)` | Send PUT request | `url: &str`, `body: impl Serialize` | `DMSCResult<DMSCHttpResponse>` |
| `delete(url)` | Send DELETE request | `url: &str` | `DMSCResult<DMSCHttpResponse>` |
| `request(method, url, body)` | Send custom request | `method: HttpMethod`, `url: &str`, `body: Option<impl Serialize>` | `DMSCResult<DMSCHttpResponse>` |
| `set_header(key, value)` | Set request header | `key: &str`, `value: &str` | `&Self` |
| `set_timeout(timeout)` | Set timeout | `timeout: Duration` | `&Self` |
| `set_auth(auth)` | Set authentication | `auth: HttpAuth` | `&Self` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Create HTTP client
let client = DMSCHttpClient::new(DMSCHttpClientConfig::default());

// Send GET request
let response = client.get("https://api.example.com/users").await?;
let users: Vec<User> = response.json().await?;

// Send POST request
let new_user = serde_json::json!({
    "name": "Jane Doe",
    "email": "jane@example.com"
});

let response = client.post("https://api.example.com/users", new_user).await?;
let created_user: User = response.json().await?;

// Set authentication
let client = DMSCHttpClient::new(DMSCHttpClientConfig::default())
    .set_auth(HttpAuth::Bearer("your-api-token".to_string()));

let response = client.get("https://api.example.com/protected").await?;
```
<div align="center">

## Route Management

</div>

### DMSCRouter

Router interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create router | None | `Self` |
| `get(path, handler)` | Add GET route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `post(path, handler)` | Add POST route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `put(path, handler)` | Add PUT route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `delete(path, handler)` | Add DELETE route | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `use_middleware(middleware)` | Use middleware | `middleware: impl HttpMiddleware` | `&Self` |
| `group(prefix)` | Create route group | `prefix: &str` | `DMSCRouteGroup` |

#### Route Parameters

```rust
use dmsc::prelude::*;

// Path parameters
router.get("/users/:id", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let user_id = req.params.get("id").unwrap();
    // Handle logic
});

// Query parameters
router.get("/search", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let query = req.query.get("q").unwrap_or("");
    let limit = req.query.get("limit").unwrap_or("10").parse::<usize>().unwrap_or(10);
    
    // Handle search logic
});

// Wildcard routes
router.get("/files/*path", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let file_path = req.params.get("path").unwrap();
    // Handle file request
});
```

### Route Groups

```rust
use dmsc::prelude::*;

// Create route group
let api_router = router.group("/api/v1");

api_router.get("/users", get_users_handler);
api_router.post("/users", create_user_handler);
api_router.get("/users/:id", get_user_handler);
api_router.put("/users/:id", update_user_handler);
api_router.delete("/users/:id", delete_user_handler);

// Nested route groups
let admin_router = router.group("/admin");
admin_router.use_middleware(auth_middleware);

let users_admin_router = admin_router.group("/users");
users_admin_router.get("/", admin_get_users_handler);
users_admin_router.delete("/:id", admin_delete_user_handler);
```

<div align="center">

## Middleware

</div>  

### DMSCHttpMiddleware

Middleware interface.

```rust
use dmsc::prelude::*;

// Logging middleware
struct LoggingMiddleware;

impl HttpMiddleware for LoggingMiddleware {
    async fn handle(&self, req: DMSCHttpRequest, next: Next) -> DMSCResult<DMSCHttpResponse> {
        let start = std::time::Instant::now();
        
        ctx.log().info(format!("Request: {} {}", req.method, req.path));
        
        let response = next.run(req).await?;
        
        let duration = start.elapsed();
        ctx.log().info(format!(
            "Response: {} {} - {} ({:?})",
            response.status_code, req.method, req.path, duration
        ));
        
        Ok(response)
    }
}

// CORS middleware
struct CorsMiddleware {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<String>,
    allowed_headers: Vec<String>,
}

impl HttpMiddleware for CorsMiddleware {
    async fn handle(&self, req: DMSCHttpRequest, next: Next) -> DMSCResult<DMSCHttpResponse> {
        let mut response = next.run(req).await?;
        
        response.headers.insert("Access-Control-Allow-Origin", self.allowed_origins.join(", "));
        response.headers.insert("Access-Control-Allow-Methods", self.allowed_methods.join(", "));
        response.headers.insert("Access-Control-Allow-Headers", self.allowed_headers.join(", "));
        
        Ok(response)
    }
}

// Use middleware
server.use_middleware(LoggingMiddleware);
server.use_middleware(CorsMiddleware {
    allowed_origins: vec!["*".to_string()],
    allowed_methods: vec!["GET", "POST", "PUT", "DELETE".to_string()],
    allowed_headers: vec!["Content-Type", "Authorization".to_string()],
});
```

### Built-in Middleware

```rust
use dmsc::prelude::*;

// Authentication middleware
server.use_middleware(AuthMiddleware::new());

// Rate limiting middleware
server.use_middleware(RateLimitMiddleware::new()
    .set_limit(100)  // 100 requests per minute
    .set_window(Duration::from_secs(60))
);

// Compression middleware
server.use_middleware(CompressionMiddleware::new()
    .set_threshold(1024)  // Enable compression for files larger than 1KB
    .set_level(6)  // Compression level
);

// Static file middleware
server.use_middleware(StaticFileMiddleware::new("./public"));
```

<div align="center">

## WebSocket Support

</div>

### DMSCWebSocket

WebSocket interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `accept()` | Accept WebSocket connection | None | `DMSCResult<DMSCWebSocketConnection>` |
| `send(message)` | Send message | `message: impl Into<String>` | `DMSCResult<()>` |
| `receive()` | Receive message | None | `DMSCResult<Option<String>>` |
| `close()` | Close connection | None | `DMSCResult<()>` |

#### WebSocket Server

```rust
use dmsc::prelude::*;

// WebSocket route
server.get("/ws", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // Upgrade HTTP connection to WebSocket
    let ws = res.upgrade_to_websocket(req)?;
    
    // Accept WebSocket connection
    let mut connection = ws.accept().await?;
    
    // Handle WebSocket messages
    while let Some(message) = connection.receive().await? {
        match message {
            WebSocketMessage::Text(text) => {
                ctx.log().info(format!("Received: {}", text));
                
                // Echo message
                connection.send(format!("Echo: {}", text)).await?;
            }
            WebSocketMessage::Binary(data) => {
                ctx.log().info(format!("Received binary data: {} bytes", data.len()));
            }
            WebSocketMessage::Close(reason) => {
                ctx.log().info(format!("WebSocket closed: {:?}", reason));
                break;
            }
        }
    }
    
    // Close connection
    connection.close().await?;
    
    Ok(())
});
```

#### WebSocket Client

```rust
use dmsc::prelude::*;

// Create WebSocket client
let ws_client = DMSCWebSocketClient::new();

// Connect to WebSocket server
let mut connection = ws_client.connect("ws://localhost:8080/ws").await?;

// Send message
connection.send("Hello, WebSocket!").await?;

// Receive message
if let Some(message) = connection.receive().await? {
    println!("Received: {}", message);
}

// Close connection
connection.close().await?;
```

<div align="center">

## File Upload/Download

</div>

### File Upload

```rust
use dmsc::prelude::*;

// File upload handler
server.post("/upload", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // Parse multipart form data
    let multipart = req.parse_multipart()?;
    
    for field in multipart.fields {
        match field {
            MultipartField::File(file) => {
                let filename = file.filename.unwrap_or("unknown".to_string());
                let content_type = file.content_type.unwrap_or("application/octet-stream".to_string());
                
                ctx.log().info(format!("Uploading file: {} ({})", filename, content_type));
                
                // Save file
                let save_path = format!("./uploads/{}", filename);
                file.save(&save_path).await?;
                
                ctx.log().info(format!("File saved to: {}", save_path));
            }
            MultipartField::Field(field) => {
                ctx.log().info(format!("Form field: {} = {}", field.name, field.value));
            }
        }
    }
    
    res.status(200).json(serde_json::json!({
        "message": "Upload successful"
    }))
});
```

### File Download

```rust
use dmsc::prelude::*;

// File download handler
server.get("/download/:filename", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let filename = req.params.get("filename").unwrap();
    let file_path = format!("./uploads/{}", filename);
    
    // Check if file exists
    if !std::path::Path::new(&file_path).exists() {
        return res.status(404).json(serde_json::json!({
            "error": "File not found"
        }));
    }
    
    // Set response headers
    res.set_header("Content-Type", "application/octet-stream");
    res.set_header("Content-Disposition", format!("attachment; filename=\"{}\"", filename));
    
    // Send file
    res.send_file(&file_path).await
});
```

### Large File Handling

```rust
use dmsc::prelude::*;

// Large file upload (chunked upload)
server.post("/upload/chunked", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // Get upload information
    let upload_id = req.headers.get("X-Upload-ID").unwrap();
    let chunk_index = req.headers.get("X-Chunk-Index").unwrap().parse::<usize>().unwrap();
    let total_chunks = req.headers.get("X-Total-Chunks").unwrap().parse::<usize>().unwrap();
    
    // Save chunk
    let chunk_data = req.body;
    let chunk_path = format!("./uploads/temp/{}_chunk_{}", upload_id, chunk_index);
    
    std::fs::write(&chunk_path, chunk_data)?;
    
    // Check if all chunks have been uploaded
    if chunk_index + 1 == total_chunks {
        // Merge chunks
        let final_path = format!("./uploads/{}", upload_id);
        merge_chunks(&final_path, upload_id, total_chunks).await?;
        
        // Clean up temporary files
        cleanup_temp_chunks(upload_id, total_chunks).await?;
        
        ctx.log().info(format!("Upload completed: {}", upload_id));
    }
    
    res.status(200).json(serde_json::json!({
        "message": "Chunk uploaded successfully",
        "chunk_index": chunk_index
    }))
});
```

<div align="center">

## Configuration

</div>

### DMSCHttpServerConfig

HTTP server configuration struct.

#### Fields

| Field | Type | Description | Default Value |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | Server host | `"0.0.0.0"` |
| `port` | `u16` | Server port | `8080` |
| `max_connections` | `usize` | Maximum number of connections | `1000` |
| `request_timeout` | `Duration` | Request timeout | `30s` |
| `keep_alive_timeout` | `Duration` | Keep-alive timeout | `60s` |
| `max_request_size` | `usize` | Maximum request size | `10MB` |
| `enable_compression` | `bool` | Enable compression | `true` |
| `enable_cors` | `bool` | Enable CORS | `true` |

### DMSCHttpClientConfig

HTTP client configuration struct.

#### Fields

| Field | Type | Description | Default Value |
|:--------|:-----|:-------------|:-------|
| `timeout` | `Duration` | Request timeout | `30s` |
| `max_redirects` | `usize` | Maximum number of redirects | `5` |
| `user_agent` | `String` | User-Agent header | `"DMSC-HTTP-Client/1.0"` |
| `enable_cookies` | `bool` | Enable cookies | `true` |
| `enable_compression` | `bool` | Enable compression | `true` |
| `pool_idle_timeout` | `Duration` | Connection pool idle timeout | `90s` |
| `pool_max_idle_per_host` | `usize` | Maximum idle connections per host | `10` |

<div align="center">

## Error Handling

</div>
### HTTP Error Codes

| Error Code | Description |
|:--------|:-------------|
| `HTTP_SERVER_ERROR` | HTTP server error |
| `HTTP_CLIENT_ERROR` | HTTP client error |
| `HTTP_REQUEST_ERROR` | HTTP request error |
| `HTTP_RESPONSE_ERROR` | HTTP response error |
| `WEBSOCKET_ERROR` | WebSocket error |

### Error Handling Example

```rust
use dmsc::prelude::*;

match client.get("https://api.example.com/users").await {
    Ok(response) => {
        if response.status_code == 200 {
            let users: Vec<User> = response.json().await?;
            println!("Users: {:?}", users);
        } else {
            println!("API error: {} - {}", response.status_code, response.body);
        }
    }
    Err(DMSCError { code, .. }) if code == "HTTP_CLIENT_ERROR" => {
        // Client error, possibly network issue
        ctx.log().error("HTTP client error, retrying...");
        
        // Retry request
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = client.get("https://api.example.com/users").await?;
    }
    Err(e) => {
        // Other errors
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Use Middleware**: Use middleware to handle cross-cutting concerns
2. **Set Appropriate Timeouts**: Set appropriate request and connection timeouts
3. **Handle Errors**: Properly handle HTTP errors and exceptions
4. **Use Connection Pooling**: Reuse HTTP connections to improve performance
5. **Validate Input**: Validate and sanitize user input
6. **Use HTTPS**: Use HTTPS in production environments
7. **Limit Request Size**: Set reasonable request size limits
8. **Monitor Performance**: Monitor HTTP request performance metrics

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
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
