<div align="center">

# Core API参考

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

core模块是DMSC Python的基础，提供应用构建器、服务上下文、错误处理等核心功能。

</div>

## 模块概述

core模块包含以下核心组件：

- **DMSCAppBuilder**: 应用构建器
- **DMSCApplication**: DMSC应用实例
- **DMSCServiceContext**: 服务上下文
- **DMSCError**: 错误处理
- **DMSCModule**: 模块系统

<div align="center">

## 核心组件

</div>

### DMSCAppBuilder

应用构建器，用于配置和构建DMSC应用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `__init__()` | 创建新的应用构建器 | 无 | `DMSCAppBuilder` |
| `with_logging(config)` | 添加日志模块 | `config: DMSCLogConfig` | `DMSCAppBuilder` |
| `with_config(config)` | 添加配置模块 | `config: Union[str, DMSCConfig]` | `DMSCAppBuilder` |
| `with_cache(config)` | 添加缓存模块 | `config: DMSCCacheConfig` | `DMSCAppBuilder` |
| `with_http(config)` | 添加HTTP模块 | `config: DMSCHTTPConfig` | `DMSCAppBuilder` |
| `with_fs(config)` | 添加文件系统模块 | `config: Optional[DMSCFSConfig]` | `DMSCAppBuilder` |
| `with_auth(config)` | 添加认证模块 | `config: DMSCAuthConfig` | `DMSCAppBuilder` |
| `with_observability(config)` | 添加可观测性模块 | `config: DMSCObservabilityConfig` | `DMSCAppBuilder` |
| `with_module(module)` | 添加自定义模块 | `module: DMSCModule` | `DMSCAppBuilder` |
| `on_init(func)` | 注册初始化钩子 | `func: Callable` | `DMSCAppBuilder` |
| `on_start(func)` | 注册启动钩子 | `func: Callable` | `DMSCAppBuilder` |
| `on_shutdown(func)` | 注册关闭钩子 | `func: Callable` | `DMSCAppBuilder` |
| `build()` | 构建DMSC应用 | 无 | `DMSCApplication` |

#### 使用示例

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCConfig

# 创建应用构建器
builder = DMSCAppBuilder()

# 配置日志
log_config = DMSCLogConfig(level="INFO")
builder.with_logging(log_config)

# 配置应用
config = DMSCConfig()
config.set("app.name", "MyApp")
builder.with_config(config)

# 构建应用
app = builder.build()
```

### DMSCApplication

DMSC应用实例，代表构建完成的应用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `run_async(func)` | 异步运行应用 | `func: Callable[[DMSCServiceContext], Any]` | `Any` |
| `start_async()` | 异步启动应用 | 无 | `None` |
| `shutdown_async()` | 异步关闭应用 | 无 | `None` |
| `get_context()` | 获取服务上下文 | 无 | `DMSCServiceContext` |

#### 使用示例

```python
import asyncio
from dmsc import DMSCAppBuilder

async def main():
    # 构建应用
    app = DMSCAppBuilder().build()
    
    # 定义业务逻辑
    async def business_logic(ctx):
        ctx.logger.info("app", "Application is running")
        return "Success"
    
    # 运行应用
    result = await app.run_async(business_logic)
    print(f"Result: {result}")

asyncio.run(main())
```

### DMSCServiceContext

服务上下文，提供对所有模块功能的访问。

#### 属性

| 属性 | 描述 | 类型 |
|:--------|:-------------|:--------|
| `logger` | 日志器 | `DMSCLogger` |
| `config` | 配置管理器 | `DMSCConfig` |
| `cache` | 缓存管理器（如果已启用） | `DMSCCache` |
| `http` | HTTP客户端（如果已启用） | `DMSCHTTPClient` |
| `fs` | 文件系统管理器（如果已启用） | `DMSCFS` |
| `auth` | 认证管理器（如果已启用） | `DMSCAuth` |
| `metrics` | 指标管理器（如果已启用） | `DMSCMetrics` |

#### 使用示例

```python
async def business_logic(ctx):
    # 使用日志器
    ctx.logger.info("service", "Service started")
    
    # 使用配置
    app_name = ctx.config.get("app.name", "DefaultApp")
    
    # 使用缓存（如果已启用）
    if hasattr(ctx, 'cache'):
        await ctx.cache.set("key", "value")
        value = await ctx.cache.get("key")
    
    # 使用HTTP客户端（如果已启用）
    if hasattr(ctx, 'http'):
        response = await ctx.http.get("https://api.example.com/data")
    
    return "Business logic completed"
```

### DMSCError

DMSC错误类型，所有DMSC异常的基类。

#### 构造函数

```python
DMSCError(code: str, message: str, details: dict = None)
```

#### 参数

| 参数 | 描述 | 类型 | 默认值 |
|:--------|:-------------|:--------|:--------|
| `code` | 错误代码 | `str` | 必填 |
| `message` | 错误消息 | `str` | 必填 |
| `details` | 错误详情 | `dict` | `None` |

#### 属性

| 属性 | 描述 | 类型 |
|:--------|:-------------|:--------|
| `code` | 错误代码 | `str` |
| `message` | 错误消息 | `str` |
| `details` | 错误详情 | `dict` |

#### 方法

| 方法 | 描述 | 返回值 |
|:--------|:-------------|:--------|
| `to_dict()` | 转换为字典格式 | `dict` |

#### 使用示例

```python
from dmsc import DMSCError

# 创建错误
try:
    raise DMSCError(
        code="INVALID_CONFIG",
        message="Invalid configuration value",
        details={"field": "port", "value": -1, "expected": "positive integer"}
    )
except DMSCError as e:
    print(f"Error {e.code}: {e.message}")
    print(f"Details: {e.details}")
    
    # 转换为字典
    error_dict = e.to_dict()
    print(f"Error dict: {error_dict}")
```

### DMSCModule

模块基类，用于创建自定义模块。

#### 方法

| 方法 | 描述 | 参数 | 返回值 | 默认实现 |
|:--------|:-------------|:--------|:--------|:--------|
| `name()` | 返回模块名称 | 无 | `str` | 必须实现 |
| `is_critical()` | 指示模块是否关键 | 无 | `bool` | `True` |
| `priority()` | 返回模块优先级 | 无 | `int` | `0` |
| `dependencies()` | 返回模块依赖列表 | 无 | `List[str]` | `[]` |
| `init(ctx)` | 初始化模块 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `before_start(ctx)` | 启动前准备 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `start(ctx)` | 启动模块服务 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `after_start(ctx)` | 启动后操作 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `before_shutdown(ctx)` | 关闭前准备 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `shutdown(ctx)` | 关闭模块服务 | `ctx: DMSCServiceContext` | `None` | 空实现 |
| `after_shutdown(ctx)` | 关闭后清理 | `ctx: DMSCServiceContext` | `None` | 空实现 |

#### 使用示例

```python
from dmsc import DMSCModule

class MyCustomModule(DMSCModule):
    def name(self) -> str:
        return "my_custom_module"
    
    def is_critical(self) -> bool:
        return True
    
    def priority(self) -> int:
        return 10
    
    def dependencies(self) -> List[str]:
        return ["log", "config"]
    
    async def start(self, ctx: DMSCServiceContext):
        ctx.logger.info(self.name(), "My custom module started")
        # 模块启动逻辑
    
    async def shutdown(self, ctx: DMSCServiceContext):
        ctx.logger.info(self.name(), "My custom module stopped")
        # 模块关闭逻辑

# 使用自定义模块
from dmsc import DMSCAppBuilder

app = (DMSCAppBuilder()
       .with_module(MyCustomModule())
       .build())
```

## 错误处理

### 常见错误代码

| 错误代码 | 描述 | 场景 |
|:--------|:-------------|:--------|
| `INVALID_CONFIG` | 无效配置 | 配置值格式错误或超出范围 |
| `MODULE_NOT_FOUND` | 模块未找到 | 请求的模块不存在 |
| `MODULE_INIT_FAILED` | 模块初始化失败 | 模块初始化过程中出错 |
| `CIRCULAR_DEPENDENCY` | 循环依赖 | 模块间存在循环依赖 |
| `SERVICE_UNAVAILABLE` | 服务不可用 | 依赖的服务不可用 |
| `PERMISSION_DENIED` | 权限拒绝 | 没有执行操作的权限 |
| `RESOURCE_NOT_FOUND` | 资源未找到 | 请求的资源不存在 |
| `VALIDATION_FAILED` | 验证失败 | 输入数据验证失败 |

### 错误处理最佳实践

```python
from dmsc import DMSCError
import asyncio

async def safe_operation(ctx):
    try:
        # 可能失败的操作
        result = await risky_operation()
        return result
    
    except DMSCError as e:
        # 记录错误
        ctx.logger.error("operation", f"DMSC error occurred: {e.code} - {e.message}")
        
        # 根据错误类型采取不同措施
        if e.code == "RESOURCE_NOT_FOUND":
            # 资源不存在，返回默认值
            return get_default_value()
        elif e.code == "PERMISSION_DENIED":
            # 权限问题，向上传播
            raise
        else:
            # 其他错误，重试或降级
            return await fallback_operation()
    
    except Exception as e:
        # 非DMSC错误，包装为DMSCError
        raise DMSCError(
            code="UNKNOWN_ERROR",
            message="Unknown error occurred",
            details={"original_error": str(e)}
        )
```

## 生命周期管理

### 应用生命周期

```python
import asyncio
from dmsc import DMSCAppBuilder

async def main():
    # 创建构建器
    builder = DMSCAppBuilder()
    
    # 注册生命周期钩子
    @builder.on_init
    async def on_init(ctx):
        print("1. Application initializing...")
        # 执行初始化逻辑
        return True
    
    @builder.on_start
    async def on_start(ctx):
        print("2. Application starting...")
        # 执行启动逻辑
        return True
    
    @builder.on_shutdown
    async def on_shutdown(ctx):
        print("3. Application shutting down...")
        # 执行清理逻辑
        return True
    
    # 构建应用
    app = builder.build()
    
    # 定义业务逻辑
    async def business_logic(ctx):
        print("4. Business logic executing...")
        return "Success"
    
    # 运行应用
    result = await app.run_async(business_logic)
    print(f"5. Result: {result}")

# 运行结果：
# 1. Application initializing...
# 2. Application starting...
# 4. Business logic executing...
# 5. Result: Success
# 3. Application shutting down...

asyncio.run(main())
```

## 依赖注入

### 基本使用

```python
from dmsc import DMSCModule, provider

class DatabaseService:
    def __init__(self, connection_string: str):
        self.connection_string = connection_string

class UserService:
    def __init__(self, database: DatabaseService):
        self.database = database

class DatabaseModule(DMSCModule):
    @provider
    def provide_database(self, ctx) -> DatabaseService:
        connection_string = ctx.config.get("database.url", "sqlite://default.db")
        return DatabaseService(connection_string)

class UserModule(DMSCModule):
    @provider
    def provide_user_service(self, database: DatabaseService) -> UserService:
        return UserService(database)

# 使用依赖注入
app = (DMSCAppBuilder()
       .with_module(DatabaseModule())
       .with_module(UserModule())
       .build())

async def business_logic(ctx):
    # 获取注入的服务
    user_service = ctx.get_service(UserService)
    # 使用服务...
    return "Dependency injection working"
```

## 性能优化

### 异步并发

```python
import asyncio
from dmsc import DMSCAppBuilder

async def concurrent_operations(ctx):
    # 并发执行多个操作
    tasks = [
        fetch_user_data(ctx, 1),
        fetch_user_data(ctx, 2),
        fetch_user_data(ctx, 3),
    ]
    
    # 等待所有任务完成
    results = await asyncio.gather(*tasks)
    
    return results

async def fetch_user_data(ctx, user_id):
    # 模拟异步操作
    await asyncio.sleep(0.1)
    return {"user_id": user_id, "name": f"User {user_id}"}
```

### 连接池

```python
from dmsc import DMSCModule

class DatabaseModule(DMSCModule):
    def __init__(self):
        self.connection_pool = None
    
    async def start(self, ctx: DMSCServiceContext):
        # 初始化连接池
        pool_size = ctx.config.get_int("database.pool_size", 10)
        self.connection_pool = await create_connection_pool(pool_size)
        
        ctx.logger.info(self.name(), f"Database connection pool created with size {pool_size}")
    
    async def shutdown(self, ctx: DMSCServiceContext):
        # 关闭连接池
        if self.connection_pool:
            await self.connection_pool.close()
            ctx.logger.info(self.name(), "Database connection pool closed")
```

## 调试和监控

### 调试模式

```python
from dmsc import DMSCConfig

# 启用调试模式
config = DMSCConfig()
config.set("debug", True)
config.set("log.level", "DEBUG")

# 在代码中使用调试信息
async def debug_operation(ctx):
    if ctx.config.get_bool("debug", False):
        ctx.logger.debug("debug", "Debug information...")
        # 额外的调试逻辑
        print(f"Context: {ctx}")
```

### 性能监控

```python
import time
from dmsc import DMSCModule

class PerformanceMonitor(DMSCModule):
    def name(self):
        return "performance_monitor"
    
    async def monitor_operation(self, ctx, operation_name: str, func):
        start_time = time.time()
        
        try:
            result = await func()
            return result
        finally:
            duration = time.time() - start_time
            
            # 记录性能指标
            ctx.logger.info(
                self.name(),
                f"Operation {operation_name} completed in {duration:.3f}s"
            )
            
            # 如果启用了指标收集，记录指标
            if hasattr(ctx, 'metrics'):
                await ctx.metrics.record_timing(operation_name, duration)
```