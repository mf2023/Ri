<div align="center">

# HTTP服务使用示例

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

本示例展示如何使用DMSC Python的http模块进行HTTP服务器、客户端、路由管理、中间件、WebSocket和文件上传下载功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC Python应用，实现以下功能：

- HTTP服务器配置和路由管理
- RESTful API设计和实现
- 中间件和请求处理
- WebSocket实时通信
- 文件上传下载处理
- 错误处理和响应格式化

<div align="center">

## 前置要求

</div>

- Python 3.8+
- pip 20.0+
- 基本的Python编程知识
- 了解HTTP协议和RESTful API概念
- （可选）Postman或curl用于API测试

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-http-example
cd dms-http-example
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate
```

### 2. 添加依赖

创建`requirements.txt`文件：

```txt
dmsc>=1.0.0
aiofiles>=23.0.0
python-multipart>=0.0.6
websockets>=11.0.0
```

安装依赖：

```bash
pip install -r requirements.txt
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
  host: "0.0.0.0"
  port: 8080
  workers: 2
  cors:
    enabled: true
    origins: ["*"]
    methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    headers: ["*"]
  rate_limit:
    enabled: true
    requests_per_minute: 60
  upload:
    max_file_size: 10485760  # 10MB
    allowed_extensions: [".jpg", ".jpeg", ".png", ".pdf", ".txt"]
  websocket:
    enabled: true
    max_connections: 100
    heartbeat_interval: 30
```

### 4. 编写主代码

创建`main.py`文件：

```python
import asyncio
import json
import os
from datetime import datetime
from pathlib import Path
from dmsc import DMSCAppBuilder, DMSCHTTPConfig

# 模拟数据存储
users_db = {}
products_db = {}
websocket_connections = set()

async def main():
    """主函数"""
    # 构建服务运行时
    app = DMSCAppBuilder()
    
    # 配置HTTP服务
    http_config = DMSCHTTPConfig(
        host="0.0.0.0",
        port=8080,
        workers=2,
        cors_enabled=True,
        rate_limit_enabled=True
    )
    
    # 构建应用
    dms_app = (app
               .with_http(http_config)
               .with_config("config.yaml")
               .build())
    
    # 初始化模拟数据
    await init_sample_data()
    
    # 定义业务逻辑
    async def business_logic(ctx):
        """业务逻辑函数"""
        ctx.logger.info("http_demo", "=== HTTP服务使用示例开始 ===")
        
        # 获取HTTP服务
        http_server = ctx.http
        
        # 注册路由
        await register_routes(ctx, http_server)
        
        # 注册中间件
        await register_middlewares(ctx, http_server)
        
        # 注册WebSocket处理
        await register_websocket_handlers(ctx, http_server)
        
        # 启动HTTP服务
        ctx.logger.info("http_demo", f"HTTP服务启动在 http://localhost:8080")
        ctx.logger.info("http_demo", "可用端点:")
        ctx.logger.info("http_demo", "  GET    / - 根路径")
        ctx.logger.info("http_demo", "  GET    /health - 健康检查")
        ctx.logger.info("http_demo", "  GET    /api/users - 获取用户列表")
        ctx.logger.info("http_demo", "  GET    /api/users/{id} - 获取特定用户")
        ctx.logger.info("http_demo", "  POST   /api/users - 创建用户")
        ctx.logger.info("http_demo", "  PUT    /api/users/{id} - 更新用户")
        ctx.logger.info("http_demo", "  DELETE /api/users/{id} - 删除用户")
        ctx.logger.info("http_demo", "  GET    /api/products - 获取产品列表")
        ctx.logger.info("http_demo", "  POST   /api/upload - 文件上传")
        ctx.logger.info("http_demo", "  GET    /api/download/{filename} - 文件下载")
        ctx.logger.info("http_demo", "  WS     /ws - WebSocket连接")
        
        # 模拟一些HTTP请求
        await simulate_http_requests(ctx)
        
        # 保持服务运行
        try:
            await asyncio.Event().wait()  # 永远等待，直到被中断
        except KeyboardInterrupt:
            ctx.logger.info("http_demo", "HTTP服务停止中...")
        
        return "HTTP服务停止"
    
    # 运行应用
    result = await dms_app.run_async(business_logic)
    print(f"结果: {result}")

async def init_sample_data():
    """初始化示例数据"""
    global users_db, products_db
    
    # 用户数据
    users_db = {
        "1": {"id": "1", "name": "Alice", "email": "alice@example.com", "age": 25},
        "2": {"id": "2", "name": "Bob", "email": "bob@example.com", "age": 30},
        "3": {"id": "3", "name": "Charlie", "email": "charlie@example.com", "age": 35},
    }
    
    # 产品数据
    products_db = {
        "1": {"id": "1", "name": "iPhone 15", "price": 999.99, "category": "Electronics", "stock": 100},
        "2": {"id": "2", "name": "MacBook Pro", "price": 1999.99, "category": "Electronics", "stock": 50},
        "3": {"id": "3", "name": "Coffee Maker", "price": 79.99, "category": "Kitchen", "stock": 200},
    }

async def register_routes(ctx, http_server):
    """注册HTTP路由"""
    
    # 根路径
    @http_server.get("/")
    async def root_handler(request):
        return {
            "message": "Welcome to DMSC HTTP Example",
            "version": "1.0.0",
            "timestamp": datetime.now().isoformat(),
            "endpoints": [
                "/health",
                "/api/users",
                "/api/products",
                "/api/upload",
                "/ws"
            ]
        }
    
    # 健康检查
    @http_server.get("/health")
    async def health_handler(request):
        return {
            "status": "healthy",
            "timestamp": datetime.now().isoformat(),
            "uptime": "1h 23m 45s",
            "version": "1.0.0"
        }
    
    # 用户相关路由
    @http_server.get("/api/users")
    async def get_users_handler(request):
        """获取用户列表"""
        # 支持分页和过滤
        page = int(request.query_params.get("page", 1))
        limit = int(request.query_params.get("limit", 10))
        search = request.query_params.get("search", "")
        
        # 过滤用户
        filtered_users = list(users_db.values())
        if search:
            filtered_users = [u for u in filtered_users if search.lower() in u["name"].lower()]
        
        # 分页
        total = len(filtered_users)
        start = (page - 1) * limit
        end = start + limit
        paginated_users = filtered_users[start:end]
        
        return {
            "users": paginated_users,
            "pagination": {
                "page": page,
                "limit": limit,
                "total": total,
                "pages": (total + limit - 1) // limit
            }
        }
    
    @http_server.get("/api/users/{user_id}")
    async def get_user_handler(request, user_id):
        """获取特定用户"""
        user = users_db.get(user_id)
        if not user:
            return {"error": "User not found"}, 404
        return user
    
    @http_server.post("/api/users")
    async def create_user_handler(request):
        """创建用户"""
        try:
            data = await request.json()
            
            # 数据验证
            if not data.get("name") or not data.get("email"):
                return {"error": "Name and email are required"}, 400
            
            # 生成新ID
            new_id = str(max(int(k) for k in users_db.keys()) + 1)
            
            # 创建用户
            new_user = {
                "id": new_id,
                "name": data["name"],
                "email": data["email"],
                "age": data.get("age", 0)
            }
            
            users_db[new_id] = new_user
            
            ctx.logger.info("http_demo", f"创建用户: {new_user}")
            
            return new_user, 201
            
        except Exception as e:
            return {"error": f"Invalid request data: {str(e)}"}, 400
    
    @http_server.put("/api/users/{user_id}")
    async def update_user_handler(request, user_id):
        """更新用户"""
        user = users_db.get(user_id)
        if not user:
            return {"error": "User not found"}, 404
        
        try:
            data = await request.json()
            
            # 更新用户数据
            user.update({
                "name": data.get("name", user["name"]),
                "email": data.get("email", user["email"]),
                "age": data.get("age", user["age"])
            })
            
            ctx.logger.info("http_demo", f"更新用户: {user}")
            
            return user
            
        except Exception as e:
            return {"error": f"Invalid request data: {str(e)}"}, 400
    
    @http_server.delete("/api/users/{user_id}")
    async def delete_user_handler(request, user_id):
        """删除用户"""
        if user_id not in users_db:
            return {"error": "User not found"}, 404
        
        deleted_user = users_db.pop(user_id)
        
        ctx.logger.info("http_demo", f"删除用户: {deleted_user}")
        
        return {"message": "User deleted successfully"}
    
    # 产品相关路由
    @http_server.get("/api/products")
    async def get_products_handler(request):
        """获取产品列表"""
        category = request.query_params.get("category")
        min_price = request.query_params.get("min_price", type=float)
        max_price = request.query_params.get("max_price", type=float)
        
        # 过滤产品
        filtered_products = list(products_db.values())
        
        if category:
            filtered_products = [p for p in filtered_products if p["category"] == category]
        
        if min_price is not None:
            filtered_products = [p for p in filtered_products if p["price"] >= min_price]
        
        if max_price is not None:
            filtered_products = [p for p in filtered_products if p["price"] <= max_price]
        
        return {"products": filtered_products, "total": len(filtered_products)}
    
    # 文件上传
    @http_server.post("/api/upload")
    async def upload_file_handler(request):
        """文件上传"""
        try:
            # 获取上传的文件
            files = await request.files()
            
            if not files:
                return {"error": "No files uploaded"}, 400
            
            uploaded_files = []
            
            for file in files:
                filename = file.filename
                content = await file.read()
                
                # 验证文件类型
                allowed_extensions = ['.jpg', '.jpeg', '.png', '.pdf', '.txt']
                file_ext = Path(filename).suffix.lower()
                
                if file_ext not in allowed_extensions:
                    return {"error": f"File type {file_ext} not allowed"}, 400
                
                # 验证文件大小 (10MB limit)
                if len(content) > 10 * 1024 * 1024:
                    return {"error": "File too large (max 10MB)"}, 400
                
                # 保存文件
                upload_dir = Path("uploads")
                upload_dir.mkdir(exist_ok=True)
                
                file_path = upload_dir / filename
                
                async with aiofiles.open(file_path, 'wb') as f:
                    await f.write(content)
                
                uploaded_files.append({
                    "filename": filename,
                    "size": len(content),
                    "path": str(file_path)
                })
                
                ctx.logger.info("http_demo", f"上传文件: {filename} ({len(content)} bytes)")
            
            return {
                "message": "Files uploaded successfully",
                "files": uploaded_files
            }, 201
            
        except Exception as e:
            return {"error": f"Upload failed: {str(e)}"}, 500
    
    # 文件下载
    @http_server.get("/api/download/{filename}")
    async def download_file_handler(request, filename):
        """文件下载"""
        try:
            file_path = Path("uploads") / filename
            
            if not file_path.exists():
                return {"error": "File not found"}, 404
            
            # 安全检查：确保文件在uploads目录内
            if not file_path.resolve().is_relative_to(Path("uploads").resolve()):
                return {"error": "Invalid file path"}, 400
            
            return await http_server.file_response(file_path)
            
        except Exception as e:
            return {"error": f"Download failed: {str(e)}"}, 500

async def register_middlewares(ctx, http_server):
    """注册中间件"""
    
    # 认证中间件
    @http_server.middleware
    async def auth_middleware(request, call_next):
        """认证中间件"""
        # 跳过不需要认证的路径
        if request.path in ["/", "/health", "/api/upload"]:
            return await call_next(request)
        
        # 检查认证头
        auth_header = request.headers.get("Authorization")
        
        if not auth_header or not auth_header.startswith("Bearer "):
            return {"error": "Missing or invalid authorization header"}, 401
        
        # 验证令牌（这里简化处理）
        token = auth_header[7:]  # 移除 "Bearer "
        
        if token != "demo-token":
            return {"error": "Invalid token"}, 401
        
        # 将用户信息添加到请求
        request.user = {"id": "1", "username": "demo_user"}
        
        return await call_next(request)
    
    # 日志中间件
    @http_server.middleware
    async def logging_middleware(request, call_next):
        """日志中间件"""
        start_time = datetime.now()
        
        # 调用下一个处理程序
        response = await call_next(request)
        
        # 计算处理时间
        process_time = (datetime.now() - start_time).total_seconds()
        
        # 记录日志
        ctx.logger.info("http_middleware", 
                       f"{request.method} {request.path} - {response.status_code} - {process_time:.3f}s")
        
        # 添加处理时间到响应头
        if hasattr(response, 'headers'):
            response.headers["X-Process-Time"] = str(process_time)
        
        return response
    
    # CORS中间件
    @http_server.middleware
    async def cors_middleware(request, call_next):
        """CORS中间件"""
        response = await call_next(request)
        
        # 添加CORS头
        if hasattr(response, 'headers'):
            response.headers["Access-Control-Allow-Origin"] = "*"
            response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE, OPTIONS"
            response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
        
        return response

async def register_websocket_handlers(ctx, http_server):
    """注册WebSocket处理程序"""
    
    @http_server.websocket("/ws")
    async def websocket_handler(websocket, path):
        """WebSocket处理程序"""
        # 添加连接到集合
        websocket_connections.add(websocket)
        
        try:
            ctx.logger.info("websocket", f"WebSocket连接建立: {websocket.remote_address}")
            
            # 发送欢迎消息
            await websocket.send(json.dumps({
                "type": "welcome",
                "message": "Welcome to DMSC WebSocket!",
                "timestamp": datetime.now().isoformat()
            }))
            
            # 广播新用户加入
            await broadcast_message({
                "type": "user_joined",
                "message": f"User {websocket.remote_address} joined",
                "timestamp": datetime.now().isoformat()
            }, exclude=websocket)
            
            # 处理消息
            async for message in websocket:
                try:
                    data = json.loads(message)
                    
                    # 处理不同类型的消息
                    if data.get("type") == "ping":
                        await websocket.send(json.dumps({
                            "type": "pong",
                            "timestamp": datetime.now().isoformat()
                        }))
                    
                    elif data.get("type") == "message":
                        # 广播消息给所有连接
                        broadcast_data = {
                            "type": "broadcast",
                            "from": str(websocket.remote_address),
                            "message": data.get("content", ""),
                            "timestamp": datetime.now().isoformat()
                        }
                        
                        await broadcast_message(broadcast_data)
                    
                    elif data.get("type") == "stats":
                        # 返回统计信息
                        stats = {
                            "type": "stats",
                            "connections": len(websocket_connections),
                            "uptime": "1h 23m 45s",
                            "timestamp": datetime.now().isoformat()
                        }
                        
                        await websocket.send(json.dumps(stats))
                    
                    else:
                        await websocket.send(json.dumps({
                            "type": "error",
                            "message": "Unknown message type",
                            "timestamp": datetime.now().isoformat()
                        }))
                
                except json.JSONDecodeError:
                    await websocket.send(json.dumps({
                        "type": "error",
                        "message": "Invalid JSON format",
                        "timestamp": datetime.now().isoformat()
                    }))
                
                except Exception as e:
                    ctx.logger.error("websocket", f"处理消息错误: {e}")
                    await websocket.send(json.dumps({
                        "type": "error",
                        "message": f"Error processing message: {str(e)}",
                        "timestamp": datetime.now().isoformat()
                    }))
        
        except websockets.exceptions.ConnectionClosed:
            ctx.logger.info("websocket", f"WebSocket连接关闭: {websocket.remote_address}")
        
        except Exception as e:
            ctx.logger.error("websocket", f"WebSocket错误: {e}")
        
        finally:
            # 从集合中移除连接
            websocket_connections.discard(websocket)
            
            # 广播用户离开
            await broadcast_message({
                "type": "user_left",
                "message": f"User {websocket.remote_address} left",
                "timestamp": datetime.now().isoformat()
            })

async def broadcast_message(data, exclude=None):
    """广播消息给所有WebSocket连接"""
    if websocket_connections:
        message = json.dumps(data)
        
        # 向所有连接发送消息（排除指定的连接）
        tasks = []
        for websocket in websocket_connections:
            if websocket != exclude and not websocket.closed:
                tasks.append(websocket.send(message))
        
        if tasks:
            await asyncio.gather(*tasks, return_exceptions=True)

async def simulate_http_requests(ctx):
    """模拟HTTP请求"""
    await asyncio.sleep(2)  # 等待服务启动
    
    ctx.logger.info("http_demo", "--- 模拟HTTP请求 ---")
    
    # 模拟GET请求
    try:
        response = await ctx.http.get("http://localhost:8080/health")
        ctx.logger.info("http_demo", f"健康检查: {response.status_code} - {response.json()}")
    except Exception as e:
        ctx.logger.error("http_demo", f"健康检查失败: {e}")
    
    # 模拟POST请求
    try:
        new_user = {
            "name": "Demo User",
            "email": "demo@example.com",
            "age": 25
        }
        response = await ctx.http.post(
            "http://localhost:8080/api/users",
            json=new_user,
            headers={"Authorization": "Bearer demo-token"}
        )
        ctx.logger.info("http_demo", f"创建用户: {response.status_code}")
    except Exception as e:
        ctx.logger.error("http_demo", f"创建用户失败: {e}")
    
    # 模拟带查询参数的GET请求
    try:
        response = await ctx.http.get(
            "http://localhost:8080/api/users",
            params={"page": 1, "limit": 5, "search": "Alice"},
            headers={"Authorization": "Bearer demo-token"}
        )
        ctx.logger.info("http_demo", f"查询用户: {response.status_code} - 找到 {len(response.json().get('users', []))} 个用户")
    except Exception as e:
        ctx.logger.error("http_demo", f"查询用户失败: {e}")

if __name__ == "__main__":
    asyncio.run(main())
```

<div align="center">

## 代码解析

</div>

### 1. HTTP服务器配置

- **主机和端口**: 配置服务器监听地址
- **工作进程**: 支持多工作进程
- **CORS配置**: 跨域请求支持
- **速率限制**: 防止API滥用
- **文件上传**: 文件大小和类型限制

### 2. 路由管理

- **RESTful设计**: 遵循RESTful API设计原则
- **路径参数**: 支持URL路径参数
- **查询参数**: 支持查询字符串参数
- **请求方法**: 支持GET、POST、PUT、DELETE等
- **响应格式化**: 统一的JSON响应格式

### 3. 中间件系统

- **认证中间件**: JWT令牌验证
- **日志中间件**: 请求日志和性能监控
- **CORS中间件**: 跨域请求处理
- **错误处理**: 统一的错误响应格式
- **请求处理时间**: 自动计算请求处理时间

### 4. WebSocket支持

- **实时通信**: 支持双向实时通信
- **消息广播**: 向所有连接广播消息
- **连接管理**: 连接生命周期管理
- **心跳机制**: 保持连接活跃
- **消息类型**: 支持不同类型的消息处理

### 5. 文件处理

- **文件上传**: 多文件上传支持
- **文件下载**: 安全的文件下载
- **类型验证**: 文件类型检查
- **大小限制**: 文件大小限制
- **路径安全**: 防止路径遍历攻击

<div align="center">

## 运行步骤

</div>

### 1. 准备环境

```bash
# 创建项目目录
mkdir dms-http-example
cd dms-http-example

# 创建虚拟环境
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate

# 安装依赖
pip install dmsc>=1.0.0 aiofiles>=23.0.0 python-multipart>=0.0.6 websockets>=11.0.0
```

### 2. 创建配置文件

创建`config.yaml`文件，内容如上所示。

### 3. 运行示例

```bash
python main.py
```

### 4. 测试API

使用curl或Postman测试API：

```bash
# 健康检查
curl http://localhost:8080/health

# 获取用户列表
curl -H "Authorization: Bearer demo-token" http://localhost:8080/api/users

# 创建用户
curl -X POST -H "Authorization: Bearer demo-token" -H "Content-Type: application/json" \
  -d '{"name":"Test User","email":"test@example.com","age":30}' \
  http://localhost:8080/api/users

# WebSocket连接
websocat ws://localhost:8080/ws
```

<div align="center">

## 预期结果

</div>

运行示例后，您将看到类似以下输出：

```
[INFO] http_demo: === HTTP服务使用示例开始 ===
[INFO] http_demo: HTTP服务启动在 http://localhost:8080
[INFO] http_demo: 可用端点:
[INFO] http_demo:   GET    / - 根路径
[INFO] http_demo:   GET    /health - 健康检查
[INFO] http_demo:   GET    /api/users - 获取用户列表
[INFO] http_demo:   GET    /api/users/{id} - 获取特定用户
[INFO] http_demo:   POST   /api/users - 创建用户
[INFO] http_demo:   PUT    /api/users/{id} - 更新用户
[INFO] http_demo:   DELETE /api/users/{id} - 删除用户
[INFO] http_demo:   GET    /api/products - 获取产品列表
[INFO] http_demo:   POST   /api/upload - 文件上传
[INFO] http_demo:   GET    /api/download/{filename} - 文件下载
[INFO] http_demo:   WS     /ws - WebSocket连接
[INFO] http_demo: --- 模拟HTTP请求 ---
[INFO] http_middleware: GET /health - 200 - 0.012s
[INFO] http_demo: 健康检查: 200 - {'status': 'healthy', 'timestamp': '2025-12-27T10:00:00.000000', 'uptime': '1h 23m 45s', 'version': '1.0.0'}
[INFO] http_middleware: POST /api/users - 201 - 0.025s
[INFO] http_demo: 创建用户: 201
[INFO] http_middleware: GET /api/users - 200 - 0.018s
[INFO] http_demo: 查询用户: 200 - 找到 4 个用户
```

<div align="center">

## 最佳实践

</div>

1. **RESTful设计**: 遵循RESTful API设计原则
2. **错误处理**: 统一的错误响应格式
3. **认证授权**: 使用JWT或OAuth进行认证
4. **输入验证**: 验证所有输入数据
5. **速率限制**: 防止API滥用和DDoS攻击
6. **CORS配置**: 正确配置跨域请求
7. **日志记录**: 记录重要操作和错误
8. **性能监控**: 监控API响应时间和错误率
9. **版本控制**: 使用API版本控制
10. **文档化**: 提供完整的API文档

<div align="center">

## 相关示例

</div>

- [基础应用](./basic-app.md): 构建简单的DMSC应用
- [认证与授权](./authentication.md): 使用JWT和OAuth进行认证
- [缓存使用](./caching.md): 缓存的基本操作和高级用法
- [数据库操作](./database.md): 数据库连接、查询和事务管理
- [消息队列](./mq.md): 异步消息处理和事件驱动架构
- [可观测性](./observability.md): 分布式追踪、指标收集和监控
- [安全实践](./security.md): 加密、哈希和安全最佳实践
- [存储管理](./storage.md): 文件上传下载和存储管理
- [数据验证](./validation.md): 数据验证、清理和自定义验证器