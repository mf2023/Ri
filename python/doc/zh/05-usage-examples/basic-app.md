# 基础应用示例

本示例展示如何使用DMSC Python构建一个基础的Web应用，包含基本的配置、日志、HTTP服务等功能。

## 示例概述

我们将创建一个简单的用户管理API，包含以下功能：
- 用户注册
- 用户登录
- 获取用户信息
- 用户列表

## 完整代码示例

```python
from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCHTTPConfig, DMSCConfig, DMSCError
)
import asyncio
from datetime import datetime
from typing import Dict, List, Optional

# 模拟用户数据库
users_db = {}
sessions = {}

# 用户数据模型
class User:
    def __init__(self, username: str, email: str, password: str):
        self.username = username
        self.email = email
        self.password = password  # 实际应用中应该加密存储
        self.created_at = datetime.now()
        self.last_login = None
    
    def to_dict(self) -> Dict:
        return {
            "username": self.username,
            "email": self.email,
            "created_at": self.created_at.isoformat(),
            "last_login": self.last_login.isoformat() if self.last_login else None
        }

# 用户服务
class UserService:
    def __init__(self, context: DMSCServiceContext):
        self.context = context
        self.logger = context.logger
    
    async def register_user(self, username: str, email: str, password: str) -> Dict:
        """注册用户"""
        self.logger.info("user_service", "Registering new user", 
                       username=username, email=email)
        
        # 检查用户名是否已存在
        if username in users_db:
            self.logger.warn("user_service", "Username already exists", username=username)
            raise DMSCError("USER_EXISTS", f"Username {username} already exists")
        
        # 检查邮箱是否已存在
        for user in users_db.values():
            if user.email == email:
                self.logger.warn("user_service", "Email already exists", email=email)
                raise DMSCError("EMAIL_EXISTS", f"Email {email} already exists")
        
        # 创建新用户
        user = User(username, email, password)
        users_db[username] = user
        
        self.logger.info("user_service", "User registered successfully", 
                       username=username, user_count=len(users_db))
        
        return user.to_dict()
    
    async def login_user(self, username: str, password: str) -> str:
        """用户登录"""
        self.logger.info("user_service", "User login attempt", username=username)
        
        # 检查用户是否存在
        if username not in users_db:
            self.logger.warn("user_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        user = users_db[username]
        
        # 验证密码
        if user.password != password:
            self.logger.warn("user_service", "Invalid password", username=username)
            raise DMSCError("INVALID_PASSWORD", "Invalid password")
        
        # 更新最后登录时间
        user.last_login = datetime.now()
        
        # 创建会话（简化版）
        session_id = f"session_{username}_{datetime.now().timestamp()}"
        sessions[session_id] = username
        
        self.logger.info("user_service", "User logged in successfully", 
                       username=username, session_id=session_id)
        
        return session_id
    
    async def get_user_info(self, username: str) -> Dict:
        """获取用户信息"""
        self.logger.info("user_service", "Getting user info", username=username)
        
        if username not in users_db:
            self.logger.warn("user_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        user = users_db[username]
        return user.to_dict()
    
    async def list_users(self, limit: Optional[int] = None) -> List[Dict]:
        """获取用户列表"""
        self.logger.info("user_service", "Listing users", limit=limit)
        
        users = list(users_db.values())
        if limit:
            users = users[:limit]
        
        user_list = [user.to_dict() for user in users]
        
        self.logger.info("user_service", "Users listed", 
                       count=len(user_list), total=len(users_db))
        
        return user_list

# HTTP处理器
class UserHandler:
    def __init__(self, user_service: UserService):
        self.user_service = user_service
    
    async def register_handler(self, request):
        """注册处理器"""
        try:
            data = await request.json()
            username = data.get("username")
            email = data.get("email")
            password = data.get("password")
            
            if not all([username, email, password]):
                return {
                    "error": "Missing required fields",
                    "required": ["username", "email", "password"]
                }, 400
            
            user_data = await self.user_service.register_user(username, email, password)
            
            return {
                "success": True,
                "message": "User registered successfully",
                "data": user_data
            }, 201
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def login_handler(self, request):
        """登录处理器"""
        try:
            data = await request.json()
            username = data.get("username")
            password = data.get("password")
            
            if not all([username, password]):
                return {
                    "error": "Missing required fields",
                    "required": ["username", "password"]
                }, 400
            
            session_id = await self.user_service.login_user(username, password)
            
            return {
                "success": True,
                "message": "Login successful",
                "data": {
                    "session_id": session_id,
                    "username": username
                }
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 401
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def get_user_handler(self, request):
        """获取用户信息处理器"""
        try:
            username = request.path_params.get("username")
            
            if not username:
                return {
                    "error": "Username parameter is required"
                }, 400
            
            user_data = await self.user_service.get_user_info(username)
            
            return {
                "success": True,
                "data": user_data
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 404
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def list_users_handler(self, request):
        """用户列表处理器"""
        try:
            limit = request.query_params.get("limit")
            if limit:
                try:
                    limit = int(limit)
                except ValueError:
                    return {
                        "error": "Invalid limit parameter"
                    }, 400
            
            users = await self.user_service.list_users(limit)
            
            return {
                "success": True,
                "data": users,
                "count": len(users)
            }, 200
            
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500

# 主应用函数
async def main(context: DMSCServiceContext):
    """主应用函数"""
    logger = context.logger
    logger.info("app", "Starting basic user management API")
    
    # 创建用户服务
    user_service = UserService(context)
    user_handler = UserHandler(user_service)
    
    # 获取HTTP服务器
    http = context.http
    
    # 注册路由
    http.add_route("POST", "/api/users/register", user_handler.register_handler)
    http.add_route("POST", "/api/users/login", user_handler.login_handler)
    http.add_route("GET", "/api/users/{username}", user_handler.get_user_handler)
    http.add_route("GET", "/api/users", user_handler.list_users_handler)
    
    # 添加健康检查
    async def health_check(request):
        return {
            "status": "healthy",
            "timestamp": datetime.now().isoformat(),
            "service": "user-management-api",
            "version": "1.0.0"
        }, 200
    
    http.add_route("GET", "/health", health_check)
    
    # 添加根路径
    async def root_handler(request):
        return {
            "message": "Welcome to DMSC Basic User Management API",
            "version": "1.0.0",
            "endpoints": [
                "POST /api/users/register - 用户注册",
                "POST /api/users/login - 用户登录",
                "GET /api/users/{username} - 获取用户信息",
                "GET /api/users - 获取用户列表",
                "GET /health - 健康检查"
            ]
        }, 200
    
    http.add_route("GET", "/", root_handler)
    
    logger.info("app", "API routes registered successfully")
    logger.info("app", "Basic user management API is ready")
    
    # 保持应用运行
    logger.info("app", "Application is running. Press Ctrl+C to stop.")
    try:
        await asyncio.Event().wait()
    except KeyboardInterrupt:
        logger.info("app", "Application stopped by user")

# 创建和运行应用
async def create_and_run_app():
    """创建和运行应用"""
    
    # 创建日志配置
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True,
        enable_file=True,
        file_path="logs/basic-app.log"
    )
    
    # 创建HTTP配置
    http_config = DMSCHTTPConfig(
        host="0.0.0.0",
        port=8080,
        cors_enabled=True,
        cors_origins=["*"]
    )
    
    # 创建应用构建器
    app_builder = DMSCAppBuilder()
    
    # 配置应用
    app = (app_builder
           .with_logging(log_config)
           .with_config(DMSCConfig())
           .with_http(http_config)
           .build())
    
    # 运行应用
    await app.run_async(main)

# 运行应用
if __name__ == "__main__":
    asyncio.run(create_and_run_app())
```

## 运行示例

### 1. 安装依赖

```bash
pip install dmsc-python
```

### 2. 运行应用

```bash
python basic_app.py
```

### 3. 测试API

#### 用户注册

```bash
curl -X POST http://localhost:8080/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "email": "john@example.com",
    "password": "secure_password123"
  }'
```

响应示例：
```json
{
  "success": true,
  "message": "User registered successfully",
  "data": {
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2025-12-27T10:30:00",
    "last_login": null
  }
}
```

#### 用户登录

```bash
curl -X POST http://localhost:8080/api/users/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "john_doe",
    "password": "secure_password123"
  }'
```

响应示例：
```json
{
  "success": true,
  "message": "Login successful",
  "data": {
    "session_id": "session_john_doe_1735306200.123456",
    "username": "john_doe"
  }
}
```

#### 获取用户信息

```bash
curl http://localhost:8080/api/users/john_doe
```

响应示例：
```json
{
  "success": true,
  "data": {
    "username": "john_doe",
    "email": "john@example.com",
    "created_at": "2025-12-27T10:30:00",
    "last_login": "2025-12-27T10:35:00"
  }
}
```

#### 获取用户列表

```bash
curl "http://localhost:8080/api/users?limit=10"
```

响应示例：
```json
{
  "success": true,
  "data": [
    {
      "username": "john_doe",
      "email": "john@example.com",
      "created_at": "2025-12-27T10:30:00",
      "last_login": "2025-12-27T10:35:00"
    }
  ],
  "count": 1
}
```

#### 健康检查

```bash
curl http://localhost:8080/health
```

响应示例：
```json
{
  "status": "healthy",
  "timestamp": "2025-12-27T10:40:00",
  "service": "user-management-api",
  "version": "1.0.0"
}
```

## 关键特性

### 1. 模块化设计
- 使用DMSCAppBuilder构建应用
- 按需添加日志、HTTP、配置等模块
- 支持链式配置

### 2. 结构化日志
- JSON格式日志输出
- 包含上下文信息
- 支持文件和控制台输出

### 3. 错误处理
- 统一的DMSCError异常处理
- 详细的错误信息和状态码
- 适当的HTTP响应

### 4. 服务上下文
- 通过DMSCServiceContext访问所有模块
- 依赖注入模式
- 统一的模块访问接口

### 5. 异步支持
- 完全异步的API设计
- 支持async/await语法
- 高性能的并发处理

## 扩展建议

### 1. 添加数据库支持
```python
# 可以添加数据库模块
app_builder.with_database(database_config)
```

### 2. 添加认证支持
```python
# 可以添加认证模块
app_builder.with_auth(auth_config)
```

### 3. 添加缓存支持
```python
# 可以添加缓存模块
app_builder.with_cache(cache_config)
```

### 4. 添加监控支持
```python
# 可以添加可观测性模块
app_builder.with_observability(observability_config)
```

## 注意事项

1. **安全性**: 示例中的密码存储是明文的，实际应用中应该使用加密存储
2. **会话管理**: 示例中的会话管理是简化的，实际应用中应该使用更安全的会话机制
3. **数据验证**: 示例中的数据验证是基础的，实际应用中应该添加更严格的验证
4. **错误处理**: 可以根据需要添加更详细的错误处理和日志记录
5. **性能优化**: 对于高并发场景，可以添加连接池、缓存等优化措施