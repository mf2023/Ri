<div align="center">

# HTTP API参考

**Version: 0.1.4**

**Last modified date: 2026-01-15**

http模块提供HTTP客户端与服务器功能，支持路由、中间件、WebSocket与文件上传下载。

## 模块概述

</div>

http模块包含以下子模块：

- **server**: HTTP服务器
- **client**: HTTP客户端
- **router**: 路由管理
- **middleware**: 中间件
- **websocket**: WebSocket支持
- **upload**: 文件上传下载

<div align="center">

## 核心组件

</div>

### DMSCHttpServer

HTTP服务器接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建HTTP服务器 | `config: DMSCHttpServerConfig` | `Self` |
| `route(method, path, handler)` | 添加路由 | `method: HttpMethod`, `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `get(path, handler)` | 添加GET路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `post(path, handler)` | 添加POST路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `put(path, handler)` | 添加PUT路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `delete(path, handler)` | 添加DELETE路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `use_middleware(middleware)` | 使用中间件 | `middleware: impl HttpMiddleware` | `&Self` |
| `listen(addr)` | 启动服务器监听 | `addr: &str` | `DMSCResult<()>` |
| `shutdown()` | 关闭服务器 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建HTTP服务器配置
let server_config = DMSCHttpServerConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    max_connections: 1000,
    request_timeout: Duration::from_secs(30),
    ..Default::default()
};

// 创建HTTP服务器
let server = DMSCHttpServer::new(server_config);

// 添加路由
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

// 启动服务器
server.listen("0.0.0.0:8080").await?;
```

### DMSCHttpClient

HTTP客户端接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建HTTP客户端 | `config: DMSCHttpClientConfig` | `Self` |
| `get(url)` | 发送GET请求 | `url: &str` | `DMSCResult<DMSCHttpResponse>` |
| `post(url, body)` | 发送POST请求 | `url: &str`, `body: impl Serialize` | `DMSCResult<DMSCHttpResponse>` |
| `put(url, body)` | 发送PUT请求 | `url: &str`, `body: impl Serialize` | `DMSCResult<DMSCHttpResponse>` |
| `delete(url)` | 发送DELETE请求 | `url: &str` | `DMSCResult<DMSCHttpResponse>` |
| `request(method, url, body)` | 发送自定义请求 | `method: HttpMethod`, `url: &str`, `body: Option<impl Serialize>` | `DMSCResult<DMSCHttpResponse>` |
| `set_header(key, value)` | 设置请求头 | `key: &str`, `value: &str` | `&Self` |
| `set_timeout(timeout)` | 设置超时 | `timeout: Duration` | `&Self` |
| `set_auth(auth)` | 设置认证 | `auth: HttpAuth` | `&Self` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建HTTP客户端
let client = DMSCHttpClient::new(DMSCHttpClientConfig::default());

// 发送GET请求
let response = client.get("https://api.example.com/users").await?;
let users: Vec<User> = response.json().await?;

// 发送POST请求
let new_user = serde_json::json!({
    "name": "Jane Doe",
    "email": "jane@example.com"
});

let response = client.post("https://api.example.com/users", new_user).await?;
let created_user: User = response.json().await?;

// 设置认证
let client = DMSCHttpClient::new(DMSCHttpClientConfig::default())
    .set_auth(HttpAuth::Bearer("your-api-token".to_string()));

let response = client.get("https://api.example.com/protected").await?;
```
<div align="center">

## 路由管理

</div>

### DMSCRouter

路由器接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建路由器 | 无 | `Self` |
| `get(path, handler)` | 添加GET路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `post(path, handler)` | 添加POST路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `put(path, handler)` | 添加PUT路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `delete(path, handler)` | 添加DELETE路由 | `path: &str`, `handler: impl HttpHandler` | `&Self` |
| `use_middleware(middleware)` | 使用中间件 | `middleware: impl HttpMiddleware` | `&Self` |
| `group(prefix)` | 创建路由组 | `prefix: &str` | `DMSCRouteGroup` |

#### 路由参数

```rust
use dmsc::prelude::*;

// 路径参数
router.get("/users/:id", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let user_id = req.params.get("id").unwrap();
    // 处理逻辑
});

// 查询参数
router.get("/search", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let query = req.query.get("q").unwrap_or("");
    let limit = req.query.get("limit").unwrap_or("10").parse::<usize>().unwrap_or(10);
    
    // 处理搜索逻辑
});

// 通配符路由
router.get("/files/*path", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let file_path = req.params.get("path").unwrap();
    // 处理文件请求
});
```

### 路由组

```rust
use dmsc::prelude::*;

// 创建路由组
let api_router = router.group("/api/v1");

api_router.get("/users", get_users_handler);
api_router.post("/users", create_user_handler);
api_router.get("/users/:id", get_user_handler);
api_router.put("/users/:id", update_user_handler);
api_router.delete("/users/:id", delete_user_handler);

// 嵌套路由组
let admin_router = router.group("/admin");
admin_router.use_middleware(auth_middleware);

let users_admin_router = admin_router.group("/users");
users_admin_router.get("/", admin_get_users_handler);
users_admin_router.delete("/:id", admin_delete_user_handler);
```

<div align="center">

## 中间件

</div>  

### DMSCHttpMiddleware

中间件接口。

```rust
use dmsc::prelude::*;

// 日志中间件
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

// CORS中间件
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

// 使用中间件
server.use_middleware(LoggingMiddleware);
server.use_middleware(CorsMiddleware {
    allowed_origins: vec!["*".to_string()],
    allowed_methods: vec!["GET", "POST", "PUT", "DELETE".to_string()],
    allowed_headers: vec!["Content-Type", "Authorization".to_string()],
});
```

### 内置中间件

```rust
use dmsc::prelude::*;

// 认证中间件
server.use_middleware(AuthMiddleware::new());

// 限流中间件
server.use_middleware(RateLimitMiddleware::new()
    .set_limit(100)  // 每分钟100次请求
    .set_window(Duration::from_secs(60))
);

// 压缩中间件
server.use_middleware(CompressionMiddleware::new()
    .set_threshold(1024)  // 1KB以上启用压缩
    .set_level(6)  // 压缩级别
);

// 静态文件中间件
server.use_middleware(StaticFileMiddleware::new("./public"));
```

<div align="center">

## WebSocket支持

</div>

### DMSCWebSocket

WebSocket接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `accept()` | 接受WebSocket连接 | 无 | `DMSCResult<DMSCWebSocketConnection>` |
| `send(message)` | 发送消息 | `message: impl Into<String>` | `DMSCResult<()>` |
| `receive()` | 接收消息 | 无 | `DMSCResult<Option<String>>` |
| `close()` | 关闭连接 | 无 | `DMSCResult<()>` |

#### WebSocket服务器

```rust
use dmsc::prelude::*;

// WebSocket路由
server.get("/ws", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // 升级HTTP连接到WebSocket
    let ws = res.upgrade_to_websocket(req)?;
    
    // 接受WebSocket连接
    let mut connection = ws.accept().await?;
    
    // 处理WebSocket消息
    while let Some(message) = connection.receive().await? {
        match message {
            WebSocketMessage::Text(text) => {
                ctx.log().info(format!("Received: {}", text));
                
                // 回显消息
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
    
    // 关闭连接
    connection.close().await?;
    
    Ok(())
});
```

#### WebSocket客户端

```rust
use dmsc::prelude::*;

// 创建WebSocket客户端
let ws_client = DMSCWebSocketClient::new();

// 连接到WebSocket服务器
let mut connection = ws_client.connect("ws://localhost:8080/ws").await?;

// 发送消息
connection.send("Hello, WebSocket!").await?;

// 接收消息
if let Some(message) = connection.receive().await? {
    println!("Received: {}", message);
}

// 关闭连接
connection.close().await?;
```

<div align="center">

## 文件上传下载

</div>

### 文件上传

```rust
use dmsc::prelude::*;

// 文件上传处理
server.post("/upload", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // 解析multipart表单数据
    let multipart = req.parse_multipart()?;
    
    for field in multipart.fields {
        match field {
            MultipartField::File(file) => {
                let filename = file.filename.unwrap_or("unknown".to_string());
                let content_type = file.content_type.unwrap_or("application/octet-stream".to_string());
                
                ctx.log().info(format!("Uploading file: {} ({})", filename, content_type));
                
                // 保存文件
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

### 文件下载

```rust
use dmsc::prelude::*;

// 文件下载处理
server.get("/download/:filename", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    let filename = req.params.get("filename").unwrap();
    let file_path = format!("./uploads/{}", filename);
    
    // 检查文件是否存在
    if !std::path::Path::new(&file_path).exists() {
        return res.status(404).json(serde_json::json!({
            "error": "File not found"
        }));
    }
    
    // 设置响应头
    res.set_header("Content-Type", "application/octet-stream");
    res.set_header("Content-Disposition", format!("attachment; filename=\"{}\"", filename));
    
    // 发送文件
    res.send_file(&file_path).await
});
```

### 大文件处理

```rust
use dmsc::prelude::*;

// 大文件上传（分块上传）
server.post("/upload/chunked", |req: DMSCHttpRequest, res: DMSCHttpResponse| async move {
    // 获取上传信息
    let upload_id = req.headers.get("X-Upload-ID").unwrap();
    let chunk_index = req.headers.get("X-Chunk-Index").unwrap().parse::<usize>().unwrap();
    let total_chunks = req.headers.get("X-Total-Chunks").unwrap().parse::<usize>().unwrap();
    
    // 保存分块
    let chunk_data = req.body;
    let chunk_path = format!("./uploads/temp/{}_chunk_{}", upload_id, chunk_index);
    
    std::fs::write(&chunk_path, chunk_data)?;
    
    // 检查是否所有分块都已上传
    if chunk_index + 1 == total_chunks {
        // 合并分块
        let final_path = format!("./uploads/{}", upload_id);
        merge_chunks(&final_path, upload_id, total_chunks).await?;
        
        // 清理临时文件
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

## 配置

</div>

### DMSCHttpServerConfig

HTTP服务器配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | 服务器主机 | `"0.0.0.0"` |
| `port` | `u16` | 服务器端口 | `8080` |
| `max_connections` | `usize` | 最大连接数 | `1000` |
| `request_timeout` | `Duration` | 请求超时 | `30s` |
| `keep_alive_timeout` | `Duration` | Keep-alive超时 | `60s` |
| `max_request_size` | `usize` | 最大请求大小 | `10MB` |
| `enable_compression` | `bool` | 启用压缩 | `true` |
| `enable_cors` | `bool` | 启用CORS | `true` |

### DMSCHttpClientConfig

HTTP客户端配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `timeout` | `Duration` | 请求超时 | `30s` |
| `max_redirects` | `usize` | 最大重定向次数 | `5` |
| `user_agent` | `String` | User-Agent头 | `"DMSC-HTTP-Client/1.0"` |
| `enable_cookies` | `bool` | 启用Cookie | `true` |
| `enable_compression` | `bool` | 启用压缩 | `true` |
| `pool_idle_timeout` | `Duration` | 连接池空闲超时 | `90s` |
| `pool_max_idle_per_host` | `usize` | 每主机最大空闲连接 | `10` |

<div align="center">

## 错误处理

</div>
### HTTP错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `HTTP_SERVER_ERROR` | HTTP服务器错误 |
| `HTTP_CLIENT_ERROR` | HTTP客户端错误 |
| `HTTP_REQUEST_ERROR` | HTTP请求错误 |
| `HTTP_RESPONSE_ERROR` | HTTP响应错误 |
| `WEBSOCKET_ERROR` | WebSocket错误 |

### 错误处理示例

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
        // 客户端错误，可能是网络问题
        ctx.log().error("HTTP client error, retrying...");
        
        // 重试请求
        tokio::time::sleep(Duration::from_secs(1)).await;
        let response = client.get("https://api.example.com/users").await?;
    }
    Err(e) => {
        // 其他错误
        return Err(e);
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **使用中间件**: 使用中间件处理横切关注点
2. **合理设置超时**: 设置适当的请求和连接超时
3. **处理错误**: 正确处理HTTP错误和异常情况
4. **使用连接池**: 复用HTTP连接提高性能
5. **验证输入**: 验证和清理用户输入
6. **使用HTTPS**: 在生产环境使用HTTPS
7. **限制请求大小**: 设置合理的请求大小限制
8. **监控性能**: 监控HTTP请求的性能指标

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
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信