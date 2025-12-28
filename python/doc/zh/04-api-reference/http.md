<div align="center">

# HTTP API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

http模块提供统一的HTTP服务接口，支持多种Web框架和中间件。

## 模块概述

</div>

http模块包含以下子模块：

- **server**: HTTP服务器实现
- **client**: HTTP客户端实现
- **routing**: 路由管理
- **middleware**: 中间件系统
- **validation**: 请求验证
- **websocket**: WebSocket支持
- **static**: 静态文件服务
- **cors**: 跨域支持
- **compression**: 压缩支持

<div align="center">

## 核心组件

</div>

### DMSCHTTPConfig

HTTP配置类，用于配置HTTP服务。

#### 构造函数

```python
DMSCHTTPConfig(
    framework: str = "fastapi",
    host: str = "0.0.0.0",
    port: int = 8000,
    workers: int = 1,
    reload: bool = False,
    debug: bool = False,
    cors_enabled: bool = True,
    cors_origins: List[str] = ["*"],
    cors_methods: List[str] = ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
    cors_headers: List[str] = ["*"],
    cors_credentials: bool = True,
    cors_max_age: int = 86400,
    ssl_enabled: bool = False,
    ssl_certfile: str = "",
    ssl_keyfile: str = "",
    ssl_ca_certs: str = "",
    ssl_ciphers: str = "TLSv1.2",
    max_request_size: int = 10485760,  # 10MB
    keepalive_timeout: int = 5,
    request_timeout: int = 30,
    response_timeout: int = 30,
    enable_compression: bool = True,
    compression_threshold: int = 1024,
    enable_logging: bool = True,
    log_level: str = "INFO",
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    rate_limit_enabled: bool = False,
    rate_limit_requests: int = 100,
    rate_limit_window: int = 60,
    enable_health_check: bool = True,
    health_check_path: str = "/health",
    enable_docs: bool = True,
    docs_path: str = "/docs",
    enable_redoc: bool = True,
    redoc_path: str = "/redoc"
)
```

### DMSCAppBuilder

应用构建器，用于创建和配置HTTP应用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `with_config(config)` | 设置配置 | `config: DMSCHTTPConfig` | `DMSCAppBuilder` |
| `with_middleware(middleware)` | 添加中间件 | `middleware: Callable` | `DMSCAppBuilder` |
| `with_exception_handler(handler)` | 添加异常处理器 | `handler: Callable` | `DMSCAppBuilder` |
| `with_route(path, method, handler)` | 添加路由 | `path: str`, `method: str`, `handler: Callable` | `DMSCAppBuilder` |
| `with_router(router)` | 添加路由组 | `router: DMSCRouter` | `DMSCAppBuilder` |
| `build()` | 构建应用 | `None` | `Any` |
| `run()` | 运行应用 | `None` | `None` |

#### 使用示例

```python
from dmsc import DMSCAppBuilder, DMSCHTTPConfig

# 创建HTTP配置
config = DMSCHTTPConfig(
    framework="fastapi",
    host="0.0.0.0",
    port=8000,
    debug=True,
    cors_enabled=True
)

# 构建应用
app = (DMSCAppBuilder()
    .with_config(config)
    .with_middleware(auth_middleware)
    .with_exception_handler(error_handler)
    .build()
)

# 定义路由
@app.get("/")
async def root():
    return {"message": "Hello World"}

@app.get("/users/{user_id}")
async def get_user(user_id: int):
    return {"user_id": user_id}

@app.post("/users")
async def create_user(user: dict):
    return {"user": user}

# 运行应用
if __name__ == "__main__":
    app.run()
```

### DMSCRouter

路由器，用于管理路由组。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(path, handler)` | 添加GET路由 | `path: str`, `handler: Callable` | `None` |
| `post(path, handler)` | 添加POST路由 | `path: str`, `handler: Callable` | `None` |
| `put(path, handler)` | 添加PUT路由 | `path: str`, `handler: Callable` | `None` |
| `delete(path, handler)` | 添加DELETE路由 | `path: str`, `handler: Callable` | `None` |
| `patch(path, handler)` | 添加PATCH路由 | `path: str`, `handler: Callable` | `None` |
| `head(path, handler)` | 添加HEAD路由 | `path: str`, `handler: Callable` | `None` |
| `options(path, handler)` | 添加OPTIONS路由 | `path: str`, `handler: Callable` | `None` |
| `include_router(router, prefix)` | 包含子路由 | `router: DMSCRouter`, `prefix: str` | `None` |
| `add_middleware(middleware)` | 添加中间件 | `middleware: Callable` | `None` |
| `add_exception_handler(handler)` | 添加异常处理器 | `handler: Callable` | `None` |

#### 使用示例

```python
from dmsc import DMSCRouter

# 创建路由器
user_router = DMSCRouter(prefix="/users")

# 添加路由
@user_router.get("/")
async def get_users():
    return {"users": []}

@user_router.get("/{user_id}")
async def get_user(user_id: int):
    return {"user_id": user_id}

@user_router.post("/")
async def create_user(user: dict):
    return {"user": user}

@user_router.put("/{user_id}")
async def update_user(user_id: int, user: dict):
    return {"user_id": user_id, "user": user}

@user_router.delete("/{user_id}")
async def delete_user(user_id: int):
    return {"message": f"User {user_id} deleted"}

# 包含路由器到主应用
app.include_router(user_router, prefix="/api/v1")
```

### DMSCHTTPClient

HTTP客户端，提供统一的HTTP请求接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(url, params=None, headers=None)` | GET请求 | `url: str`, `params: Dict`, `headers: Dict` | `DMSCResponse` |
| `post(url, data=None, json=None, headers=None)` | POST请求 | `url: str`, `data: Any`, `json: Dict`, `headers: Dict` | `DMSCResponse` |
| `put(url, data=None, json=None, headers=None)` | PUT请求 | `url: str`, `data: Any`, `json: Dict`, `headers: Dict` | `DMSCResponse` |
| `delete(url, headers=None)` | DELETE请求 | `url: str`, `headers: Dict` | `DMSCResponse` |
| `patch(url, data=None, json=None, headers=None)` | PATCH请求 | `url: str`, `data: Any`, `json: Dict`, `headers: Dict` | `DMSCResponse` |
| `head(url, headers=None)` | HEAD请求 | `url: str`, `headers: Dict` | `DMSCResponse` |
| `options(url, headers=None)` | OPTIONS请求 | `url: str`, `headers: Dict` | `DMSCResponse` |
| `request(method, url, **kwargs)` | 通用请求 | `method: str`, `url: str`, `**kwargs` | `DMSCResponse` |
| `set_timeout(timeout)` | 设置超时 | `timeout: int` | `None` |
| `set_headers(headers)` | 设置默认头 | `headers: Dict` | `None` |
| `set_auth(auth)` | 设置认证 | `auth: Any` | `None` |
| `set_proxy(proxy)` | 设置代理 | `proxy: str` | `None` |

#### 使用示例

```python
from dmsc import DMSCHTTPClient

# 创建HTTP客户端
client = DMSCHTTPClient(
    timeout=30,
    headers={"User-Agent": "DMSC-Client/1.0"},
    max_retries=3
)

# GET请求
response = client.get("https://api.example.com/users")
print(f"Status: {response.status_code}")
print(f"Data: {response.json()}")

# POST请求
user_data = {"name": "John Doe", "email": "john@example.com"}
response = client.post("https://api.example.com/users", json=user_data)
print(f"Created user: {response.json()}")

# 带参数的GET请求
params = {"page": 1, "limit": 10}
response = client.get("https://api.example.com/users", params=params)
print(f"Users: {response.json()}")

# 带认证头的请求
headers = {"Authorization": "Bearer token123"}
response = client.get("https://api.example.com/profile", headers=headers)
print(f"Profile: {response.json()}")

# 错误处理
try:
    response = client.get("https://api.example.com/nonexistent")
    response.raise_for_status()
except Exception as e:
    print(f"Request failed: {e}")
```

### DMSCWebSocketManager

WebSocket管理器，提供WebSocket支持。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `connect(url, headers=None)` | 连接WebSocket | `url: str`, `headers: Dict` | `DMSCWebSocket` |
| `disconnect()` | 断开连接 | `None` | `bool` |
| `send(data)` | 发送数据 | `data: Any` | `bool` |
| `receive()` | 接收数据 | `None` | `Any` |
| `broadcast(data)` | 广播数据 | `data: Any` | `bool` |
| `get_connections()` | 获取所有连接 | `None` | `List[DMSCWebSocket]` |

#### 使用示例

```python
from dmsc import DMSCWebSocketManager

# 创建WebSocket管理器
ws_manager = DMSCWebSocketManager()

# WebSocket路由处理
@app.websocket("/ws")
async def websocket_endpoint(websocket):
    await websocket.accept()
    
    try:
        while True:
            # 接收消息
            data = await websocket.receive_text()
            print(f"Received: {data}")
            
            # 发送消息
            await websocket.send_text(f"Echo: {data}")
            
    except Exception as e:
        print(f"WebSocket error: {e}")
    finally:
        await websocket.close()

# 客户端连接
async def client_example():
    ws = await ws_manager.connect("ws://localhost:8000/ws")
    
    # 发送消息
    await ws.send("Hello Server!")
    
    # 接收消息
    response = await ws.receive()
    print(f"Received: {response}")
    
    # 关闭连接
    await ws.disconnect()
```

### DMSCMiddleware

中间件基类，用于创建自定义中间件。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `__init__(**kwargs)` | 初始化中间件 | `**kwargs` | `None` |
| `process_request(request)` | 处理请求 | `request: Request` | `Request` |
| `process_response(response)` | 处理响应 | `response: Response` | `Response` |
| `process_exception(exception)` | 处理异常 | `exception: Exception` | `Response` |

#### 使用示例

```python
from dmsc import DMSCMiddleware

# 自定义中间件
class LoggingMiddleware(DMSCMiddleware):
    async def process_request(self, request):
        print(f"Request: {request.method} {request.url}")
        return request
    
    async def process_response(self, response):
        print(f"Response: {response.status_code}")
        return response

# 自定义认证中间件
class AuthMiddleware(DMSCMiddleware):
    def __init__(self, secret_key):
        self.secret_key = secret_key
    
    async def process_request(self, request):
        # 验证认证头
        auth_header = request.headers.get("Authorization")
        if not auth_header:
            raise Exception("Missing authorization header")
        
        # 验证令牌
        token = auth_header.replace("Bearer ", "")
        if not self.verify_token(token):
            raise Exception("Invalid token")
        
        return request
    
    def verify_token(self, token):
        # 实现令牌验证逻辑
        return token == self.secret_key

# 使用中间件
app.add_middleware(LoggingMiddleware)
app.add_middleware(AuthMiddleware, secret_key="my-secret-key")
```