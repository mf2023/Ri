<div align="center">

# DMSC Python 使用示例

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

通过实际示例学习如何使用DMSC Python的各种功能

</div>

## 目录

- [基础示例](#基础示例)
- [HTTP服务示例](#http服务示例)
- [缓存示例](#缓存示例)
- [文件系统示例](#文件系统示例)
- [认证示例](#认证示例)
- [可观测性示例](#可观测性示例)
- [高级示例](#高级示例)

---

## 基础示例

### 创建第一个DMSC应用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # 创建应用构建器
    app = DMSCAppBuilder()
    
    # 配置日志
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True
    )
    
    # 构建应用
    dms_app = (app
              .with_logging(log_config)
              .build())
    
    # 定义业务逻辑
    async def business_logic(ctx):
        ctx.logger.info("main", "Hello, DMSC Python!")
        return "Application completed successfully"
    
    # 运行应用
    result = await dms_app.run_async(business_logic)
    print(f"Result: {result}")

if __name__ == "__main__":
    asyncio.run(main())
```

### 使用配置管理

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCConfig

async def main():
    # 创建配置
    config = DMSCConfig()
    config.set("app.name", "MyApp")
    config.set("app.version", "1.0.0")
    config.set("database.host", "localhost")
    config.set("database.port", 5432)
    
    # 从文件加载配置
    config.load_file("config.yaml")
    
    # 从环境变量加载配置
    config.load_env("MYAPP_")
    
    # 构建应用
    app = (DMSCAppBuilder()
           .with_config(config)
           .build())
    
    # 使用配置
    async def business_logic(ctx):
        app_name = ctx.config.get("app.name")
        db_host = ctx.config.get("database.host", "localhost")
        db_port = ctx.config.get_int("database.port", 5432)
        
        ctx.logger.info("config", f"App: {app_name}, DB: {db_host}:{db_port}")
        return "Configuration loaded"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 生命周期钩子

```python
import asyncio
from dmsc import DMSCAppBuilder

async def main():
    app = DMSCAppBuilder()
    
    # 注册初始化钩子
    @app.on_init
    async def on_init(ctx):
        ctx.logger.info("lifecycle", "Application initializing...")
        # 执行初始化逻辑
        return True
    
    # 注册启动钩子
    @app.on_start
    async def on_start(ctx):
        ctx.logger.info("lifecycle", "Application starting...")
        # 执行启动逻辑
        return True
    
    # 注册关闭钩子
    @app.on_shutdown
    async def on_shutdown(ctx):
        ctx.logger.info("lifecycle", "Application shutting down...")
        # 执行清理逻辑
        return True
    
    # 构建并运行应用
    dms_app = app.build()
    
    async def business_logic(ctx):
        ctx.logger.info("main", "Business logic executing...")
        return "Success"
    
    await dms_app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## HTTP服务示例

### 创建简单的HTTP服务

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCHTTPConfig

async def main():
    # 配置HTTP
    http_config = DMSCHTTPConfig(
        host="0.0.0.0",
        port=8080,
        workers=2
    )
    
    # 构建应用
    app = (DMSCAppBuilder()
           .with_http(http_config)
           .build())
    
    # 定义路由处理函数
    async def handle_hello(ctx, request):
        return {
            "message": "Hello, World!",
            "method": request.method,
            "path": request.path
        }
    
    async def handle_user(ctx, request, user_id):
        return {
            "user_id": user_id,
            "message": f"Hello, user {user_id}!"
        }
    
    # 获取HTTP客户端
    http_client = ctx.http
    
    # 注册路由
    # 注意：这里假设DMSC Python提供了路由注册机制
    # 实际实现可能有所不同
    
    async def business_logic(ctx):
        ctx.logger.info("http", f"HTTP server starting on port {http_config.port}")
        
        # 模拟HTTP请求
        response = await ctx.http.get("http://httpbin.org/json")
        ctx.logger.info("http", f"Response status: {response.status_code}")
        
        return "HTTP service started"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### HTTP客户端使用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCHTTPConfig

async def main():
    # 配置HTTP客户端
    http_config = DMSCHTTPConfig()
    
    app = (DMSCAppBuilder()
           .with_http(http_config)
           .build())
    
    async def business_logic(ctx):
        # GET请求
        response = await ctx.http.get(
            "https://api.github.com/users/octocat",
            headers={"User-Agent": "DMSC-Python"}
        )
        
        if response.is_success():
            data = response.json()
            ctx.logger.info("http", f"GitHub user: {data.get('login')}")
        
        # POST请求
        post_response = await ctx.http.post(
            "https://httpbin.org/post",
            json={"message": "Hello from DMSC Python"}
        )
        
        if post_response.is_success():
            result = post_response.json()
            ctx.logger.info("http", f"POST response: {result.get('json')}")
        
        return "HTTP requests completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 缓存示例

### 内存缓存使用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCCacheConfig, DMSCCacheBackend

async def main():
    # 配置内存缓存
    cache_config = DMSCCacheConfig.memory_cache(
        ttl=300,  # 5分钟过期
        max_size=100
    )
    
    app = (DMSCAppBuilder()
           .with_cache(cache_config)
           .build())
    
    async def business_logic(ctx):
        # 设置缓存
        await ctx.cache.set("user:123", {"name": "Alice", "age": 30})
        await ctx.cache.set("config:theme", "dark", ttl=3600)  # 1小时过期
        
        # 获取缓存
        user_data = await ctx.cache.get("user:123")
        if user_data:
            ctx.logger.info("cache", f"Cached user: {user_data}")
        
        # 检查缓存是否存在
        exists = await ctx.cache.exists("user:123")
        ctx.logger.info("cache", f"Cache exists: {exists}")
        
        # 获取剩余过期时间
        ttl = await ctx.cache.ttl("user:123")
        ctx.logger.info("cache", f"Cache TTL: {ttl} seconds")
        
        # 删除缓存
        deleted = await ctx.cache.delete("user:123")
        ctx.logger.info("cache", f"Cache deleted: {deleted}")
        
        return "Cache operations completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### Redis缓存使用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCCacheConfig, DMSCCacheBackend

async def main():
    # 配置Redis缓存
    cache_config = DMSCCacheConfig.redis_cache(
        redis_url="redis://localhost:6379/0",
        ttl=3600,  # 1小时过期
        max_connections=10
    )
    
    app = (DMSCAppBuilder()
           .with_cache(cache_config)
           .build())
    
    async def business_logic(ctx):
        # 批量设置缓存
        for i in range(10):
            await ctx.cache.set(f"item:{i}", {"id": i, "name": f"Item {i}"})
        
        # 获取所有匹配的键
        keys = await ctx.cache.keys("item:*")
        ctx.logger.info("cache", f"Found {len(keys)} cached items")
        
        # 批量获取值
        for key in keys[:3]:  # 只获取前3个
            value = await ctx.cache.get(key)
            ctx.logger.info("cache", f"{key}: {value}")
        
        # 清空缓存
        cleared = await ctx.cache.clear()
        ctx.logger.info("cache", f"Cache cleared: {cleared}")
        
        return "Redis cache operations completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 缓存装饰器模式

```python
import asyncio
import hashlib
import json
from dmsc import DMSCAppBuilder, DMSCCacheConfig

async def main():
    cache_config = DMSCCacheConfig.memory_cache(ttl=60)
    
    app = (DMSCAppBuilder()
           .with_cache(cache_config)
           .build())
    
    def cache_result(cache_key: str, ttl: int = None):
        """缓存装饰器"""
        def decorator(func):
            async def wrapper(ctx, *args, **kwargs):
                # 生成缓存键
                key = f"{cache_key}:{hashlib.md5(str(args).encode()).hexdigest()}"
                
                # 尝试从缓存获取
                cached_result = await ctx.cache.get(key)
                if cached_result is not None:
                    ctx.logger.info("cache", f"Cache hit for {key}")
                    return cached_result
                
                # 执行函数
                result = await func(ctx, *args, **kwargs)
                
                # 缓存结果
                await ctx.cache.set(key, result, ttl=ttl)
                ctx.logger.info("cache", f"Cached result for {key}")
                
                return result
            return wrapper
        return decorator
    
    @cache_result("expensive_computation", ttl=120)
    async def expensive_computation(ctx, n: int):
        """模拟耗时计算"""
        ctx.logger.info("computation", f"Computing for n={n}")
        await asyncio.sleep(2)  # 模拟耗时操作
        return {"result": n * n, "computed_at": "now"}
    
    async def business_logic(ctx):
        # 第一次调用，会执行计算
        result1 = await expensive_computation(ctx, 5)
        ctx.logger.info("main", f"First call result: {result1}")
        
        # 第二次调用，会从缓存获取
        result2 = await expensive_computation(ctx, 5)
        ctx.logger.info("main", f"Second call result: {result2}")
        
        return "Cache decorator example completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 文件系统示例

### 文件读写操作

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCFSConfig

async def main():
    fs_config = DMSCFSConfig(
        root_path="./data",  # 设置根路径
        max_file_size=10 * 1024 * 1024,  # 10MB
        allowed_extensions=[".txt", ".json", ".yaml"]
    )
    
    app = (DMSCAppBuilder()
           .with_fs(fs_config)
           .build())
    
    async def business_logic(ctx):
        # 创建目录
        await ctx.fs.create_dir("logs", parents=True)
        
        # 写入文本文件
        await ctx.fs.write_file(
            "config/app.json",
            '{"name": "MyApp", "version": "1.0.0"}'
        )
        
        # 读取文本文件
        content = await ctx.fs.read_file("config/app.json")
        ctx.logger.info("fs", f"File content: {content}")
        
        # 写入二进制文件
        binary_data = b"Binary data example"
        await ctx.fs.write_binary("data/binary.bin", binary_data)
        
        # 读取二进制文件
        read_binary = await ctx.fs.read_binary("data/binary.bin")
        ctx.logger.info("fs", f"Binary data: {read_binary}")
        
        # 检查文件存在
        exists = await ctx.fs.exists("config/app.json")
        ctx.logger.info("fs", f"File exists: {exists}")
        
        # 列出目录内容
        files = await ctx.fs.list_dir("config")
        ctx.logger.info("fs", f"Config files: {files}")
        
        return "File operations completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 文件处理工作流

```python
import asyncio
import json
from dmsc import DMSCAppBuilder, DMSCFSConfig

async def main():
    fs_config = DMSCFSConfig(root_path="./data")
    
    app = (DMSCAppBuilder()
           .with_fs(fs_config)
           .build())
    
    async def process_json_files(ctx, input_dir: str, output_dir: str):
        """处理JSON文件的工作流"""
        
        # 创建输出目录
        await ctx.fs.create_dir(output_dir, parents=True)
        
        # 获取所有JSON文件
        files = await ctx.fs.list_dir(input_dir)
        json_files = [f for f in files if f.endswith('.json')]
        
        ctx.logger.info("workflow", f"Found {len(json_files)} JSON files")
        
        processed_count = 0
        
        for filename in json_files:
            try:
                # 读取文件
                input_path = f"{input_dir}/{filename}"
                content = await ctx.fs.read_file(input_path)
                
                # 解析JSON
                data = json.loads(content)
                
                # 处理数据（示例：添加处理时间戳）
                data["processed_at"] = "2025-12-27T10:00:00Z"
                data["processed_by"] = "DMSC Python"
                
                # 写入处理后的文件
                output_path = f"{output_dir}/{filename}"
                await ctx.fs.write_file(output_path, json.dumps(data, indent=2))
                
                processed_count += 1
                ctx.logger.info("workflow", f"Processed: {filename}")
                
            except Exception as e:
                ctx.logger.error("workflow", f"Failed to process {filename}: {e}")
        
        return processed_count
    
    async def business_logic(ctx):
        # 准备示例数据
        sample_data = {
            "users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}],
            "products": [{"id": 1, "name": "Product A", "price": 99.99}]
        }
        
        await ctx.fs.create_dir("input", parents=True)
        await ctx.fs.write_file("input/users.json", json.dumps(sample_data["users"]))
        await ctx.fs.write_file("input/products.json", json.dumps(sample_data["products"]))
        
        # 处理文件
        count = await process_json_files(ctx, "input", "output")
        ctx.logger.info("main", f"Processed {count} files successfully")
        
        # 验证结果
        output_files = await ctx.fs.list_dir("output")
        ctx.logger.info("main", f"Output files: {output_files}")
        
        return "File processing workflow completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 认证示例

### JWT认证使用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCAuthConfig

async def main():
    auth_config = DMSCAuthConfig(
        jwt_secret="your-secret-key-here",
        jwt_expiry=3600,  # 1小时过期
        jwt_algorithm="HS256",
        enable_permissions=True
    )
    
    app = (DMSCAppBuilder()
           .with_auth(auth_config)
           .build())
    
    async def business_logic(ctx):
        # 模拟用户认证
        user_credentials = {
            "username": "alice",
            "password": "password123"
        }
        
        # 生成令牌（实际应用中需要验证用户名密码）
        token = await ctx.auth.generate_token(
            user_id="user_123",
            claims={
                "username": "alice",
                "email": "alice@example.com",
                "roles": ["user", "admin"]
            }
        )
        
        ctx.logger.info("auth", f"Generated token: {token[:20]}...")
        
        # 验证令牌
        auth_result = await ctx.auth.verify_token(token)
        
        if auth_result.is_success:
            ctx.logger.info("auth", f"Token valid for user: {auth_result.user_id}")
            ctx.logger.info("auth", f"Claims: {auth_result.claims}")
        else:
            ctx.logger.error("auth", "Token validation failed")
        
        # 检查权限
        has_permission = await ctx.auth.check_permission("user_123", "read:data")
        ctx.logger.info("auth", f"Has permission 'read:data': {has_permission}")
        
        # 获取用户角色
        roles = await ctx.auth.get_user_roles("user_123")
        ctx.logger.info("auth", f"User roles: {roles}")
        
        # 刷新令牌
        new_token = await ctx.auth.refresh_token(token)
        ctx.logger.info("auth", f"Refreshed token: {new_token[:20]}...")
        
        # 吊销令牌
        revoked = await ctx.auth.revoke_token(token)
        ctx.logger.info("auth", f"Token revoked: {revoked}")
        
        return "Authentication operations completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 认证装饰器模式

```python
import asyncio
import functools
from dmsc import DMSCAppBuilder, DMSCAuthConfig

async def main():
    auth_config = DMSCAuthConfig(
        jwt_secret="your-secret-key-here",
        jwt_expiry=3600
    )
    
    app = (DMSCAppBuilder()
           .with_auth(auth_config)
           .build())
    
    def require_auth(permission: str = None):
        """认证装饰器"""
        def decorator(func):
            @functools.wraps(func)
            async def wrapper(ctx, *args, **kwargs):
                # 获取请求中的令牌（假设从上下文中获取）
                token = kwargs.get("token") or ctx.config.get("auth.token")
                
                if not token:
                    raise Exception("No authentication token provided")
                
                # 验证令牌
                auth_result = await ctx.auth.verify_token(token)
                
                if not auth_result.is_success:
                    raise Exception(f"Authentication failed: {auth_result.error}")
                
                # 检查权限
                if permission:
                    has_permission = await ctx.auth.check_permission(
                        auth_result.user_id, permission
                    )
                    
                    if not has_permission:
                        raise Exception(f"Permission denied: {permission}")
                
                # 将认证信息添加到上下文
                ctx.auth_info = auth_result
                
                return await func(ctx, *args, **kwargs)
            
            return wrapper
        return decorator
    
    @require_auth(permission="read:users")
    async def get_user_data(ctx, user_id: str):
        """需要认证的用户数据访问"""
        ctx.logger.info("auth", f"User {ctx.auth_info.user_id} accessing data for user {user_id}")
        
        # 模拟用户数据获取
        return {
            "user_id": user_id,
            "name": f"User {user_id}",
            "email": f"user{user_id}@example.com",
            "accessed_by": ctx.auth_info.user_id
        }
    
    async def business_logic(ctx):
        # 生成测试令牌
        token = await ctx.auth.generate_token("admin_user", {"role": "admin"})
        
        try:
            # 调用需要认证的函数
            user_data = await get_user_data(ctx, "123", token=token)
            ctx.logger.info("main", f"User data: {user_data}")
            
        except Exception as e:
            ctx.logger.error("main", f"Authentication error: {e}")
        
        # 测试无权限访问
        user_token = await ctx.auth.generate_token("normal_user", {"role": "user"})
        
        try:
            user_data = await get_user_data(ctx, "456", token=user_token)
            ctx.logger.info("main", f"User data: {user_data}")
            
        except Exception as e:
            ctx.logger.error("main", f"Permission error: {e}")
        
        return "Authentication decorator example completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 可观测性示例

### 指标收集

```python
import asyncio
import random
from dmsc import DMSCAppBuilder, DMSCObservabilityConfig

async def main():
    observability_config = DMSCObservabilityConfig(
        metrics_enabled=True,
        prometheus_enabled=True,
        prometheus_port=9090
    )
    
    app = (DMSCAppBuilder()
           .with_observability(observability_config)
           .build())
    
    async def business_logic(ctx):
        # 创建指标
        request_counter = ctx.metrics.counter(
            "app_requests_total",
            "Total number of requests",
            labels={"method": "GET", "endpoint": "/api/users"}
        )
        
        response_time_histogram = ctx.metrics.histogram(
            "app_response_time_seconds",
            "Response time in seconds",
            buckets=[0.1, 0.5, 1.0, 2.0, 5.0]
        )
        
        active_users_gauge = ctx.metrics.gauge(
            "app_active_users",
            "Number of active users"
        )
        
        # 模拟应用行为
        for i in range(20):
            # 增加请求计数
            request_counter.inc()
            
            # 模拟响应时间
            response_time = random.uniform(0.1, 3.0)
            response_time_histogram.observe(response_time)
            
            # 模拟活跃用户数量
            active_users = random.randint(10, 100)
            active_users_gauge.set(active_users)
            
            ctx.logger.info("metrics", f"Request {i+1}: response_time={response_time:.2f}s, active_users={active_users}")
            
            await asyncio.sleep(0.5)
        
        # 获取指标值
        total_requests = request_counter.get()
        ctx.logger.info("metrics", f"Total requests: {total_requests}")
        
        current_users = active_users_gauge.get()
        ctx.logger.info("metrics", f"Current active users: {current_users}")
        
        return "Metrics collection completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 分布式追踪

```python
import asyncio
import time
from dmsc import DMSCAppBuilder, DMSCObservabilityConfig

async def main():
    observability_config = DMSCObservabilityConfig(
        tracing_enabled=True,
        jaeger_enabled=True,
        jaeger_endpoint="http://localhost:14268/api/traces"
    )
    
    app = (DMSCAppBuilder()
           .with_observability(observability_config)
           .build())
    
    async def database_operation(ctx, operation_name: str, duration: float):
        """模拟数据库操作"""
        ctx.logger.info("database", f"Executing {operation_name}...")
        await asyncio.sleep(duration)
        return {"status": "success", "operation": operation_name}
    
    async def external_api_call(ctx, api_name: str, duration: float):
        """模拟外部API调用"""
        ctx.logger.info("api", f"Calling {api_name}...")
        await asyncio.sleep(duration)
        return {"status": "success", "api": api_name}
    
    async def process_user_request(ctx, user_id: str):
        """处理用户请求（包含多个子操作）"""
        ctx.logger.info("request", f"Processing request for user {user_id}")
        
        # 数据库查询
        user_data = await database_operation(ctx, "get_user", 0.1)
        
        # 外部API调用
        profile_data = await external_api_call(ctx, "user_profile", 0.2)
        
        # 另一个数据库操作
        preferences = await database_operation(ctx, "get_preferences", 0.05)
        
        # 组合结果
        result = {
            "user_id": user_id,
            "user_data": user_data,
            "profile": profile_data,
            "preferences": preferences,
            "processed_at": time.time()
        }
        
        ctx.logger.info("request", f"Request processed for user {user_id}")
        return result
    
    async def business_logic(ctx):
        # 模拟多个用户请求
        user_ids = ["user_123", "user_456", "user_789"]
        
        tasks = []
        for user_id in user_ids:
            task = process_user_request(ctx, user_id)
            tasks.append(task)
        
        # 并发处理请求
        results = await asyncio.gather(*tasks)
        
        ctx.logger.info("main", f"Processed {len(results)} user requests")
        
        for result in results:
            ctx.logger.info("main", f"User {result['user_id']} processed successfully")
        
        return "Distributed tracing example completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 健康检查

```python
import asyncio
import time
from dmsc import DMSCAppBuilder, DMSCObservabilityConfig

async def main():
    observability_config = DMSCObservabilityConfig(
        health_check_enabled=True,
        health_check_path="/health"
    )
    
    app = (DMSCAppBuilder()
           .with_observability(observability_config)
           .build())
    
    class HealthChecker:
        def __init__(self):
            self.start_time = time.time()
            self.checks = {
                "database": self.check_database,
                "cache": self.check_cache,
                "external_api": self.check_external_api
            }
        
        async def check_database(self, ctx):
            """检查数据库连接"""
            try:
                # 模拟数据库检查
                await asyncio.sleep(0.1)
                return {"status": "healthy", "latency_ms": 100}
            except Exception as e:
                return {"status": "unhealthy", "error": str(e)}
        
        async def check_cache(self, ctx):
            """检查缓存连接"""
            try:
                # 模拟缓存检查
                await ctx.cache.set("health_check", "ok", ttl=10)
                value = await ctx.cache.get("health_check")
                if value == "ok":
                    return {"status": "healthy"}
                else:
                    return {"status": "unhealthy", "error": "Cache value mismatch"}
            except Exception as e:
                return {"status": "unhealthy", "error": str(e)}
        
        async def check_external_api(self, ctx):
            """检查外部API"""
            try:
                # 模拟外部API检查
                response = await ctx.http.get("https://httpbin.org/status/200")
                if response.is_success():
                    return {"status": "healthy", "status_code": response.status_code}
                else:
                    return {"status": "unhealthy", "status_code": response.status_code}
            except Exception as e:
                return {"status": "unhealthy", "error": str(e)}
        
        async def get_health_status(self, ctx):
            """获取整体健康状态"""
            overall_status = "healthy"
            checks_result = {}
            
            for check_name, check_func in self.checks.items():
                result = await check_func(ctx)
                checks_result[check_name] = result
                
                if result["status"] != "healthy":
                    overall_status = "unhealthy"
            
            uptime = time.time() - self.start_time
            
            return {
                "status": overall_status,
                "timestamp": time.time(),
                "uptime_seconds": uptime,
                "version": "1.0.0",
                "checks": checks_result
            }
    
    health_checker = HealthChecker()
    
    async def business_logic(ctx):
        # 模拟一些操作
        ctx.logger.info("main", "Application is running...")
        
        # 定期检查健康状态
        for i in range(3):
            await asyncio.sleep(2)
            
            health_status = await health_checker.get_health_status(ctx)
            ctx.logger.info("health", f"Health status: {health_status['status']}")
            
            if health_status["status"] == "healthy":
                ctx.logger.info("health", "All systems operational")
            else:
                ctx.logger.error("health", f"Some systems unhealthy: {health_status['checks']}")
        
        return "Health check example completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

---

## 高级示例

### 组合多个模块

```python
import asyncio
import json
from dmsc import (
    DMSCAppBuilder, DMSCLogConfig, DMSCConfig,
    DMSCCacheConfig, DMSCHTTPConfig, DMSCFSConfig,
    DMSCAuthConfig, DMSCObservabilityConfig
)

async def main():
    # 配置所有模块
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True,
        enable_file=True,
        file_path="logs/app.log"
    )
    
    config = DMSCConfig()
    config.set("app.name", "AdvancedDMSCApp")
    config.set("app.version", "1.0.0")
    
    cache_config = DMSCCacheConfig.memory_cache(ttl=300)
    
    http_config = DMSCHTTPConfig()
    
    fs_config = DMSCFSConfig(root_path="./data")
    
    auth_config = DMSCAuthConfig(
        jwt_secret="secret-key",
        jwt_expiry=3600
    )
    
    observability_config = DMSCObservabilityConfig(
        metrics_enabled=True,
        prometheus_enabled=True,
        prometheus_port=9090
    )
    
    # 构建完整应用
    app = (DMSCAppBuilder()
           .with_logging(log_config)
           .with_config(config)
           .with_cache(cache_config)
           .with_http(http_config)
           .with_fs(fs_config)
           .with_auth(auth_config)
           .with_observability(observability_config)
           .build())
    
    class UserService:
        """用户服务，演示多模块协作"""
        
        def __init__(self, ctx):
            self.ctx = ctx
            self.cache_prefix = "users"
        
        async def get_user(self, user_id: str, use_cache: bool = True):
            """获取用户信息"""
            cache_key = f"{self.cache_prefix}:{user_id}"
            
            # 尝试从缓存获取
            if use_cache:
                cached_user = await self.ctx.cache.get(cache_key)
                if cached_user:
                    self.ctx.logger.info("user_service", f"Cache hit for user {user_id}")
                    self.ctx.metrics.counter("cache_hits_total").inc()
                    return cached_user
            
            # 模拟数据库查询
            self.ctx.logger.info("user_service", f"Fetching user {user_id} from database")
            await asyncio.sleep(0.1)  # 模拟网络延迟
            
            # 模拟外部API调用获取额外信息
            api_response = await self.ctx.http.get(
                f"https://jsonplaceholder.typicode.com/users/{user_id}"
            )
            
            if api_response.is_success():
                user_data = api_response.json()
            else:
                user_data = {
                    "id": user_id,
                    "name": f"User {user_id}",
                    "email": f"user{user_id}@example.com"
                }
            
            # 缓存结果
            if use_cache:
                await self.ctx.cache.set(cache_key, user_data, ttl=300)
                self.ctx.metrics.counter("cache_misses_total").inc()
            
            # 记录指标
            self.ctx.metrics.histogram("user_fetch_duration_seconds").observe(0.1)
            
            return user_data
        
        async def create_user(self, user_data: dict):
            """创建用户"""
            # 验证输入数据
            required_fields = ["name", "email"]
            for field in required_fields:
                if field not in user_data:
                    raise ValueError(f"Missing required field: {field}")
            
            # 生成用户ID
            user_id = str(int(time.time()))
            user_data["id"] = user_id
            
            # 保存到文件（模拟数据库）
            user_file = f"users/{user_id}.json"
            await self.ctx.fs.create_dir("users", parents=True)
            await self.ctx.fs.write_file(user_file, json.dumps(user_data))
            
            # 缓存新用户
            cache_key = f"{self.cache_prefix}:{user_id}"
            await self.ctx.cache.set(cache_key, user_data, ttl=300)
            
            # 记录指标
            self.ctx.metrics.counter("users_created_total").inc()
            
            self.ctx.logger.info("user_service", f"Created user {user_id}")
            
            return user_data
        
        async def authenticate_user(self, username: str, password: str):
            """用户认证"""
            # 模拟用户认证
            if username == "admin" and password == "secret":
                user_data = {
                    "id": "admin_123",
                    "username": username,
                    "role": "admin"
                }
                
                # 生成令牌
                token = await self.ctx.auth.generate_token(
                    user_id=user_data["id"],
                    claims=user_data
                )
                
                self.ctx.metrics.counter("user_logins_total").inc()
                
                return {
                    "success": True,
                    "token": token,
                    "user": user_data
                }
            else:
                self.ctx.metrics.counter("user_login_failures_total").inc()
                return {
                    "success": False,
                    "error": "Invalid credentials"
                }
    
    async def business_logic(ctx):
        # 创建用户服务
        user_service = UserService(ctx)
        
        # 用户认证
        auth_result = await user_service.authenticate_user("admin", "secret")
        if auth_result["success"]:
            ctx.logger.info("main", f"User authenticated: {auth_result['user']['username']}")
            
            # 设置认证令牌到上下文
            ctx.auth_token = auth_result["token"]
        
        # 创建新用户
        new_user = await user_service.create_user({
            "name": "Alice Smith",
            "email": "alice@example.com"
        })
        ctx.logger.info("main", f"Created user: {new_user['name']} (ID: {new_user['id']})")
        
        # 获取用户信息（首次访问，会触发缓存未命中）
        user1 = await user_service.get_user(new_user["id"])
        ctx.logger.info("main", f"Retrieved user: {user1['name']}")
        
        # 再次获取用户信息（应该从缓存获取）
        user2 = await user_service.get_user(new_user["id"])
        ctx.logger.info("main", f"Retrieved user from cache: {user2['name']}")
        
        # 获取多个用户
        user_ids = ["1", "2", "3"]
        users = await asyncio.gather(*[
            user_service.get_user(user_id) for user_id in user_ids
        ])
        
        ctx.logger.info("main", f"Retrieved {len(users)} users")
        
        # 保存操作日志
        log_entry = {
            "timestamp": time.time(),
            "operation": "user_batch_fetch",
            "user_count": len(users),
            "users": [user["name"] for user in users if user]
        }
        
        await ctx.fs.write_file(
            f"logs/operations_{int(time.time())}.json",
            json.dumps(log_entry, indent=2)
        )
        
        # 显示指标
        ctx.logger.info("metrics", "Application metrics collected successfully")
        
        return "Advanced multi-module example completed successfully"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

### 错误处理和重试

```python
import asyncio
import random
from dmsc import DMSCAppBuilder, DMSCError

async def main():
    app = DMSCAppBuilder().build()
    
    def retry_async(max_retries: int = 3, delay: float = 1.0, backoff: float = 2.0):
        """异步重试装饰器"""
        def decorator(func):
            async def wrapper(ctx, *args, **kwargs):
                last_exception = None
                current_delay = delay
                
                for attempt in range(max_retries + 1):
                    try:
                        ctx.logger.info("retry", f"Attempt {attempt + 1} for {func.__name__}")
                        result = await func(ctx, *args, **kwargs)
                        
                        if attempt > 0:
                            ctx.logger.info("retry", f"Success after {attempt + 1} attempts")
                        
                        return result
                        
                    except Exception as e:
                        last_exception = e
                        ctx.logger.warning("retry", f"Attempt {attempt + 1} failed: {e}")
                        
                        if attempt < max_retries:
                            await asyncio.sleep(current_delay)
                            current_delay *= backoff
                        else:
                            ctx.logger.error("retry", f"All {max_retries + 1} attempts failed")
                
                raise last_exception
            
            return wrapper
        return decorator
    
    @retry_async(max_retries=3, delay=0.5)
    async def unreliable_operation(ctx, operation_id: str):
        """模拟不可靠的操作"""
        ctx.logger.info("operation", f"Executing unreliable operation {operation_id}")
        
        # 模拟随机失败
        if random.random() < 0.7:  # 70% 失败率
            error_type = random.choice(["timeout", "connection_error", "server_error"])
            
            if error_type == "timeout":
                raise DMSCError("TIMEOUT", f"Operation {operation_id} timed out")
            elif error_type == "connection_error":
                raise DMSCError("CONNECTION_ERROR", f"Connection failed for {operation_id}")
            else:
                raise DMSCError("SERVER_ERROR", f"Server error for {operation_id}")
        
        # 模拟耗时操作
        await asyncio.sleep(random.uniform(0.1, 0.5))
        
        return {
            "operation_id": operation_id,
            "status": "success",
            "timestamp": time.time()
        }
    
    async def reliable_batch_operation(ctx, operation_count: int):
        """可靠的批量操作"""
        successful_operations = 0
        failed_operations = 0
        
        tasks = []
        for i in range(operation_count):
            task = unreliable_operation(ctx, f"op_{i+1}")
            tasks.append(task)
        
        # 执行所有操作，收集结果
        results = await asyncio.gather(*tasks, return_exceptions=True)
        
        for i, result in enumerate(results):
            if isinstance(result, Exception):
                failed_operations += 1
                ctx.logger.error("batch", f"Operation {i+1} failed: {result}")
            else:
                successful_operations += 1
                ctx.logger.info("batch", f"Operation {i+1} succeeded: {result}")
        
        return {
            "total": operation_count,
            "successful": successful_operations,
            "failed": failed_operations,
            "success_rate": successful_operations / operation_count
        }
    
    async def business_logic(ctx):
        ctx.logger.info("main", "Starting error handling and retry example")
        
        # 单个操作示例
        try:
            result = await unreliable_operation(ctx, "single_op")
            ctx.logger.info("main", f"Single operation succeeded: {result}")
        except DMSCError as e:
            ctx.logger.error("main", f"Single operation failed after retries: {e}")
        
        # 批量操作示例
        batch_result = await reliable_batch_operation(ctx, 10)
        ctx.logger.info("main", f"Batch operation result: {batch_result}")
        
        # 自定义错误处理
        try:
            # 模拟配置错误
            raise DMSCError("CONFIG_ERROR", "Invalid configuration", {
                "config_key": "database.host",
                "expected_type": "string",
                "actual_value": 123
            })
        except DMSCError as e:
            ctx.logger.error("main", f"Configuration error: {e}")
            ctx.logger.error("main", f"Error details: {e.details}")
            
            # 将错误转换为字典格式
            error_dict = e.to_dict()
            ctx.logger.error("main", f"Error as dict: {error_dict}")
        
        return "Error handling and retry example completed"
    
    await app.run_async(business_logic)

if __name__ == "__main__":
    asyncio.run(main())
```

<div align="center">

## 下一步

</div>

- [最佳实践](./06-best-practices.md) - 了解开发最佳实践
- [故障排除](./07-troubleshooting.md) - 常见问题和解决方案