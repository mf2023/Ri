<div align="center">

# DMSC Python 核心概念

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

深入理解DMSC Python的设计哲学、服务上下文、模块系统和生命周期管理

</div>

## 1. 模块化架构

DMSC Python采用高度模块化的设计，将不同功能划分为独立的模块，支持按需组合和扩展。

### 1.1 设计原则

- **单一职责**: 每个模块专注于一个特定领域的功能
- **松耦合**: 模块间通过明确定义的接口通信，减少依赖
- **高内聚**: 相关功能集中在同一模块内
- **可扩展性**: 支持通过自定义模块扩展功能
- **可测试性**: 模块可以独立测试

### 1.2 模块依赖关系

模块间存在复杂的依赖关系，DMSC Python框架通过以下机制自动处理模块的加载顺序：

1. **依赖声明**: 每个模块可以通过`dependencies()`方法声明它依赖的其他模块
2. **优先级排序**: 模块可以通过`priority()`方法设置优先级，数值越大优先级越高
3. **自动排序**: 框架根据模块的依赖关系和优先级自动排序，确保依赖的模块先加载
4. **关键模块标记**: 模块可以通过`is_critical()`方法标记是否为关键模块，关键模块失败会导致整个系统失败

### 1.3 核心模块结构

- **core**: 最基础的模块，提供运行时、错误处理、服务上下文和模块系统
  - 包含：错误处理、上下文管理、生命周期、应用构建器等
- **log**: 依赖core，提供结构化日志功能
- **config**: 依赖core，提供配置管理功能
- **cache**: 依赖core，提供缓存抽象功能
- **http**: 依赖core，提供Web服务功能
- **fs**: 依赖core，提供安全的文件系统访问
- **其他模块**: 根据需要依赖上述基础模块

### 1.4 模块组合

您可以根据应用需求，选择性地组合所需的模块：

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCCacheConfig

# 创建应用构建器
app = DMSCAppBuilder()

# 添加日志模块
app.with_logging(DMSCLogConfig.default())

# 添加配置模块
app.with_config("config.yaml")

# 添加缓存模块
app.with_cache(DMSCCacheConfig.memory_cache())

# 添加HTTP模块
app.with_http(host="0.0.0.0", port=8080)

# 构建应用
application = app.build()
```

## 2. 服务上下文

`DMSCServiceContext`是DMSC Python应用的核心，提供对所有模块功能的访问。

### 2.1 设计理念

服务上下文采用依赖注入模式，将所有模块的功能集中在一个统一的接口中，便于：

- **统一访问**: 通过一个对象访问所有模块功能
- **依赖解耦**: 业务代码不直接依赖具体模块实现
- **测试友好**: 便于在测试中替换具体实现
- **扩展性**: 新模块可以无缝集成到上下文中

### 2.2 核心功能访问

通过服务上下文，您可以访问各种模块的功能：

```python
from dmsc import DMSCAppBuilder

async def business_logic(ctx):
    """业务逻辑函数，接收服务上下文"""
    
    # 访问日志功能
    ctx.logger.info("business", "开始处理业务逻辑")
    
    # 访问配置功能
    app_name = ctx.config.get("app.name", "Unknown App")
    debug_mode = ctx.config.get("app.debug", False)
    
    # 访问缓存功能（如果已启用）
    if hasattr(ctx, 'cache'):
        await ctx.cache.set("user:123", user_data, ttl=3600)
        cached_data = await ctx.cache.get("user:123")
    
    # 访问HTTP功能（如果已启用）
    if hasattr(ctx, 'http'):
        response = await ctx.http.get("https://api.example.com/data")
        
    # 访问文件系统功能（如果已启用）
    if hasattr(ctx, 'fs'):
        content = await ctx.fs.read_file("data/config.json")
        await ctx.fs.write_file("output/result.json", result_data)
    
    ctx.logger.info("business", "业务逻辑处理完成")
    return {"status": "success", "app": app_name}

# 构建并运行应用
app = DMSCAppBuilder()
app.with_logging()
app.with_config("config.yaml")
app.with_cache()
app.with_http()
app.with_fs()

application = app.build()
await application.run_async(business_logic)
```

### 2.3 上下文生命周期

服务上下文的生命周期与应用程序的生命周期一致：

1. **创建**: 在`DMSCAppBuilder.build()`时创建
2. **使用**: 在应用运行期间通过参数传递给业务逻辑
3. **销毁**: 在应用程序退出时自动销毁

### 2.4 上下文扩展

您可以扩展服务上下文，添加自定义功能：

```python
from dmsc import DMSCServiceContext

class CustomServiceContext(DMSCServiceContext):
    """自定义服务上下文"""
    
    def __init__(self, base_context):
        super().__init__(base_context)
        self.custom_data = {}
    
    def set_custom_data(self, key, value):
        """设置自定义数据"""
        self.custom_data[key] = value
    
    def get_custom_data(self, key, default=None):
        """获取自定义数据"""
        return self.custom_data.get(key, default)

# 使用自定义上下文
async def custom_business_logic(ctx):
    """使用自定义上下文的业务逻辑"""
    
    # 设置自定义数据
    ctx.set_custom_data("request_id", "req-12345")
    ctx.set_custom_data("user_id", "user-67890")
    
    # 获取自定义数据
    request_id = ctx.get_custom_data("request_id")
    user_id = ctx.get_custom_data("user_id")
    
    ctx.logger.info("custom", f"处理请求 {request_id} 用户 {user_id}")
    
    return {"request_id": request_id, "user_id": user_id}
```

## 3. 模块系统

DMSC Python的模块系统允许您扩展框架功能，实现自定义模块。

### 3.1 模块类型

DMSC Python提供了灵活的模块系统，支持多种模块类型：

- **Python模块**: 使用Python编写的模块，通过PyO3与Rust核心交互
- **混合模块**: 结合Python的易用性和Rust的性能
- **原生模块**: 完全基于Rust核心，通过Python绑定暴露功能

### 3.2 内置模块

#### 日志模块 (log)

提供结构化日志功能：

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

# 配置日志
log_config = DMSCLogConfig(
    level="INFO",
    format="json",
    enable_console=True,
    enable_file=True,
    file_path="logs/app.log"
)

# 添加到应用
app = DMSCAppBuilder()
app.with_logging(log_config)

# 使用日志
async def business_logic(ctx):
    ctx.logger.debug("module", "调试信息")
    ctx.logger.info("module", "普通信息")
    ctx.logger.warn("module", "警告信息")
    ctx.logger.error("module", "错误信息")
```

#### 配置模块 (config)

提供配置管理功能：

```python
from dmsc import DMSCAppBuilder, DMSCConfig

# 创建配置
config = DMSCConfig()
config.set("app.name", "My App")
config.set("app.version", "1.0.0")
config.set("database.host", "localhost")
config.set("database.port", 5432)

# 从文件加载
config.load_file("config.yaml")
config.load_file("config.json")

# 从环境变量加载
config.load_env("MYAPP_")

# 添加到应用
app = DMSCAppBuilder()
app.with_config(config)

# 使用配置
async def business_logic(ctx):
    app_name = ctx.config.get("app.name")
    db_host = ctx.config.get("database.host", "localhost")
    db_port = ctx.config.get("database.port", 5432)
```

#### 缓存模块 (cache)

提供缓存抽象功能：

```python
from dmsc import DMSCAppBuilder, DMSCCacheConfig, DMSCCacheBackend

# 内存缓存
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.MEMORY,
        max_size=1000,
        ttl=3600
    )
)

# Redis缓存
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.REDIS,
        redis_url="redis://localhost:6379/0",
        max_connections=10,
        ttl=7200
    )
)

# 混合缓存
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.HYBRID,
        memory_max_size=100,
        redis_url="redis://localhost:6379/0",
        ttl=3600
    )
)

# 使用缓存
async def business_logic(ctx):
    # 设置缓存
    await ctx.cache.set("user:123", user_data, ttl=3600)
    
    # 获取缓存
    cached_data = await ctx.cache.get("user:123")
    
    # 删除缓存
    await ctx.cache.delete("user:123")
    
    # 检查存在
    exists = await ctx.cache.exists("user:123")
    
    # 清空缓存
    await ctx.cache.clear()
```

#### HTTP模块 (http)

提供Web服务功能：

```python
from dmsc import DMSCAppBuilder, DMSCHTTPConfig

# 配置HTTP服务器
app.with_http(
    host="0.0.0.0",
    port=8080,
    workers=4,
    max_requests=1000
)

# 定义路由和处理函数
async def user_handler(ctx, request):
    """用户处理器"""
    user_id = request.path_params.get("id")
    
    # 记录请求
    ctx.logger.info("http", f"处理用户请求: {user_id}")
    
    # 获取用户数据
    user_data = await ctx.cache.get(f"user:{user_id}")
    
    if not user_data:
        return {"error": "用户不存在"}, 404
    
    return user_data, 200

async def health_handler(ctx, request):
    """健康检查处理器"""
    return {"status": "healthy"}, 200

# 注册路由
app.with_route("GET", "/users/{id}", user_handler)
app.with_route("GET", "/health", health_handler)
```

### 3.3 自定义模块

创建自定义Python模块：

```python
from dmsc import DMSCModule, DMSCServiceContext
from abc import ABC, abstractmethod

class CustomModule(DMSCModule):
    """自定义模块基类"""
    
    def __init__(self, name):
        self.name = name
        self.initialized = False
        self.config = {}
    
    def get_name(self):
        return self.name
    
    def get_priority(self):
        return 0  # 默认优先级
    
    def get_dependencies(self):
        return []  # 无依赖
    
    def is_critical(self):
        return False  # 非关键模块
    
    async def initialize(self, ctx: DMSCServiceContext):
        """初始化模块"""
        ctx.logger.info(self.name, f"初始化模块: {self.name}")
        self.initialized = True
    
    async def start(self, ctx: DMSCServiceContext):
        """启动模块"""
        ctx.logger.info(self.name, f"启动模块: {self.name}")
    
    async def shutdown(self, ctx: DMSCServiceContext):
        """关闭模块"""
        ctx.logger.info(self.name, f"关闭模块: {self.name}")
        self.initialized = False

class DatabaseModule(CustomModule):
    """数据库模块示例"""
    
    def __init__(self):
        super().__init__("database")
        self.connection = None
    
    def get_dependencies(self):
        return ["log", "config"]  # 依赖日志和配置模块
    
    def is_critical(self):
        return True  # 关键模块
    
    async def initialize(self, ctx: DMSCServiceContext):
        """初始化数据库连接"""
        await super().initialize(ctx)
        
        # 获取数据库配置
        db_host = ctx.config.get("database.host", "localhost")
        db_port = ctx.config.get("database.port", 5432)
        db_name = ctx.config.get("database.name", "myapp")
        
        ctx.logger.info(self.name, f"连接到数据库: {db_host}:{db_port}/{db_name}")
        
        # 这里应该建立实际的数据库连接
        # self.connection = await create_database_connection(...)
    
    async def execute_query(self, ctx: DMSCServiceContext, query: str):
        """执行数据库查询"""
        if not self.initialized:
            raise RuntimeError("数据库模块未初始化")
        
        ctx.logger.debug(self.name, f"执行查询: {query}")
        
        # 这里应该执行实际的数据库查询
        # return await self.connection.execute(query)
        return {"result": "query_result"}

# 使用自定义模块
async def business_logic(ctx):
    """使用数据库模块的业务逻辑"""
    
    # 执行数据库查询
    result = await ctx.database.execute_query(ctx, "SELECT * FROM users")
    
    ctx.logger.info("business", f"查询结果: {result}")
    
    return result

# 注册自定义模块
app = DMSCAppBuilder()
app.with_module(DatabaseModule())
app.with_logging()
app.with_config("config.yaml")

application = app.build()
await application.run_async(business_logic)
```

## 4. 生命周期管理

DMSC Python提供了完整的生命周期管理机制，确保应用的正确启动和关闭。

### 4.1 生命周期状态

应用的生命周期包含以下状态：

1. **初始化前 (BeforeInit)**: 应用构建阶段
2. **初始化 (Init)**: 模块初始化阶段
3. **初始化后 (AfterInit)**: 初始化完成后的处理
4. **启动前 (BeforeStart)**: 启动前的准备工作
5. **启动 (Start)**: 应用正式启动
6. **启动后 (AfterStart)**: 启动完成后的处理
7. **运行中 (Running)**: 应用正常运行
8. **关闭前 (BeforeShutdown)**: 关闭前的清理工作
9. **关闭 (Shutdown)**: 应用关闭
10. **关闭后 (AfterShutdown)**: 关闭完成后的处理

### 4.2 生命周期钩子

您可以在生命周期的各个阶段添加自定义逻辑：

```python
from dmsc import DMSCAppBuilder, DMSCLifecycleHook

app = DMSCAppBuilder()

# 添加生命周期钩子
@app.on_init
def on_app_init(ctx):
    """应用初始化钩子"""
    ctx.logger.info("lifecycle", "应用正在初始化...")

@app.on_start
def on_app_start(ctx):
    """应用启动钩子"""
    ctx.logger.info("lifecycle", "应用已启动，开始处理请求")

@app.on_shutdown
def on_app_shutdown(ctx):
    """应用关闭钩子"""
    ctx.logger.info("lifecycle", "应用正在关闭，清理资源...")

# 构建和运行应用
application = app.build()
await application.run_async(business_logic)
```

### 4.3 优雅关闭

DMSC Python支持优雅关闭，确保在应用关闭时正确处理：

```python
import signal
import asyncio
from dmsc import DMSCAppBuilder

class GracefulShutdown:
    """优雅关闭管理器"""
    
    def __init__(self):
        self.shutdown_event = asyncio.Event()
    
    def setup_signal_handlers(self):
        """设置信号处理器"""
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)
    
    def _signal_handler(self, signum, frame):
        """信号处理器"""
        print(f"接收到信号 {signum}，开始优雅关闭...")
        self.shutdown_event.set()
    
    async def wait_for_shutdown(self):
        """等待关闭信号"""
        await self.shutdown_event.wait()

# 使用优雅关闭
async def main():
    """主函数"""
    
    # 创建优雅关闭管理器
    shutdown_manager = GracefulShutdown()
    shutdown_manager.setup_signal_handlers()
    
    # 创建应用
    app = DMSCAppBuilder()
    app.with_logging()
    app.with_config("config.yaml")
    
    # 添加关闭钩子
    @app.on_shutdown
    def on_shutdown(ctx):
        ctx.logger.info("shutdown", "正在关闭应用...")
        # 执行清理工作
        # await cleanup_resources(ctx)
        ctx.logger.info("shutdown", "应用已关闭")
    
    application = app.build()
    
    # 运行应用
    try:
        # 启动应用
        await application.start_async()
        
        # 等待关闭信号
        await shutdown_manager.wait_for_shutdown()
        
    finally:
        # 优雅关闭
        await application.shutdown_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## 5. 配置管理最佳实践

### 5.1 分层配置

```python
from dmsc import DMSCConfig

class HierarchicalConfig:
    """分层配置管理"""
    
    def __init__(self):
        self.config = DMSCConfig()
        self._load_config()
    
    def _load_config(self):
        """加载分层配置"""
        
        # 第1层：默认配置
        self._load_defaults()
        
        # 第2层：环境配置
        self._load_environment_config()
        
        # 第3层：用户配置
        self._load_user_config()
        
        # 第4层：运行时配置
        self._load_runtime_config()
    
    def _load_defaults(self):
        """加载默认配置"""
        defaults = {
            # 应用配置
            "app.name": "DMSC Application",
            "app.version": "1.0.0",
            "app.debug": False,
            
            # 服务器配置
            "server.host": "0.0.0.0",
            "server.port": 8080,
            "server.workers": 4,
            
            # 数据库配置
            "database.url": "sqlite:///app.db",
            "database.pool_size": 10,
            "database.timeout": 30,
            
            # 缓存配置
            "cache.backend": "memory",
            "cache.ttl": 3600,
            "cache.max_size": 1000,
            
            # 日志配置
            "logging.level": "INFO",
            "logging.format": "json",
            "logging.file_path": "logs/app.log"
        }
        
        for key, value in defaults.items():
            self.config.set(key, value)
    
    def _load_environment_config(self):
        """加载环境相关配置"""
        import os
        
        # 根据环境加载不同配置
        env = os.getenv("APP_ENV", "development")
        
        if env == "production":
            self.config.set("app.debug", False)
            self.config.set("logging.level", "WARN")
            self.config.set("server.workers", 8)
        elif env == "testing":
            self.config.set("app.debug", True)
            self.config.set("logging.level", "DEBUG")
            self.config.set("database.url", "sqlite:///:memory:")
        else:  # development
            self.config.set("app.debug", True)
            self.config.set("logging.level", "DEBUG")
    
    def _load_user_config(self):
        """加载用户配置"""
        import os
        
        # 尝试加载用户配置文件
        config_files = [
            "config/app.yaml",
            "config/app.json",
            "~/.myapp/config.yaml",
            "/etc/myapp/config.yaml"
        ]
        
        for file_path in config_files:
            expanded_path = os.path.expanduser(file_path)
            if os.path.exists(expanded_path):
                self.config.load_file(expanded_path)
                break
    
    def _load_runtime_config(self):
        """加载运行时配置"""
        # 从环境变量加载（最高优先级）
        self.config.load_env("MYAPP_")
    
    # 便捷访问器
    @property
    def app_name(self):
        return self.config.get("app.name")
    
    @property
    def debug_mode(self):
        return self.config.get("app.debug", False)
    
    @property
    def server_config(self):
        return {
            "host": self.config.get("server.host", "0.0.0.0"),
            "port": self.config.get("server.port", 8080),
            "workers": self.config.get("server.workers", 4)
        }
    
    @property
    def database_config(self):
        return {
            "url": self.config.get("database.url"),
            "pool_size": self.config.get("database.pool_size", 10),
            "timeout": self.config.get("database.timeout", 30)
        }
```

## 6. 错误处理策略

### 6.1 错误分类

```python
from dmsc import DMSCError
import enum

class ErrorCategory(enum.Enum):
    """错误分类"""
    VALIDATION = "validation"
    BUSINESS = "business"
    INFRASTRUCTURE = "infrastructure"
    NETWORK = "network"
    DATABASE = "database"
    CACHE = "cache"
    EXTERNAL = "external"
    UNKNOWN = "unknown"

class AppError(DMSCError):
    """应用错误基类"""
    
    def __init__(self, category: ErrorCategory, code: str, message: str, 
                 details: dict = None, cause: Exception = None):
        super().__init__(code, message)
        self.category = category
        self.details = details or {}
        self.cause = cause
    
    def to_dict(self):
        """转换为字典格式"""
        return {
            "category": self.category.value,
            "code": self.code,
            "message": self.message,
            "details": self.details,
            "cause": str(self.cause) if self.cause else None
        }

class ValidationError(AppError):
    """验证错误"""
    
    def __init__(self, field: str, message: str, value=None):
        super().__init__(
            ErrorCategory.VALIDATION,
            "VALIDATION_ERROR",
            message,
            {"field": field, "value": value}
        )
        self.field = field
        self.value = value

class BusinessError(AppError):
    """业务逻辑错误"""
    
    def __init__(self, code: str, message: str, context: dict = None):
        super().__init__(
            ErrorCategory.BUSINESS,
            code,
            message,
            context or {}
        )

class InfrastructureError(AppError):
    """基础设施错误"""
    
    def __init__(self, component: str, message: str, cause: Exception = None):
        super().__init__(
            ErrorCategory.INFRASTRUCTURE,
            "INFRASTRUCTURE_ERROR",
            f"{component}错误: {message}",
            {"component": component},
            cause
        )
```

### 6.2 错误处理策略

```python
from dmsc import DMSCServiceContext
import asyncio
from typing import Optional, Any

class ErrorHandler:
    """错误处理器"""
    
    def __init__(self):
        self.error_handlers = {}
        self.fallback_handler = self._default_fallback_handler
    
    def register_handler(self, error_category: ErrorCategory, handler):
        """注册错误处理器"""
        self.error_handlers[error_category] = handler
    
    def set_fallback_handler(self, handler):
        """设置回退处理器"""
        self.fallback_handler = handler
    
    async def handle_error(self, ctx: DMSCServiceContext, error: Exception) -> Optional[Any]:
        """处理错误"""
        
        # 记录错误
        await self._log_error(ctx, error)
        
        # 分类错误
        app_error = self._categorize_error(error)
        
        # 查找处理器
        handler = self.error_handlers.get(app_error.category)
        
        if handler:
            try:
                # 使用特定处理器处理
                return await handler(ctx, app_error)
            except Exception as handler_error:
                # 处理器失败，使用回退处理器
                ctx.logger.error("error_handler", f"错误处理器失败: {handler_error}")
                return await self.fallback_handler(ctx, app_error)
        else:
            # 使用回退处理器
            return await self.fallback_handler(ctx, app_error)
    
    async def _log_error(self, ctx: DMSCServiceContext, error: Exception):
        """记录错误"""
        if isinstance(error, AppError):
            ctx.logger.error(
                "app_error",
                f"应用错误 [{error.category.value}]: {error.message}",
                extra=error.to_dict()
            )
        else:
            ctx.logger.error(
                "unexpected_error",
                f"意外错误: {error}",
                extra={"error_type": type(error).__name__, "error": str(error)}
            )
    
    def _categorize_error(self, error: Exception) -> AppError:
        """分类错误"""
        
        if isinstance(error, AppError):
            return error
        
        # 根据错误类型分类
        if isinstance(error, ValueError):
            return ValidationError("unknown", str(error))
        elif isinstance(error, ConnectionError):
            return InfrastructureError("network", str(error), error)
        elif isinstance(error, TimeoutError):
            return InfrastructureError("timeout", str(error), error)
        else:
            return AppError(
                ErrorCategory.UNKNOWN,
                "UNKNOWN_ERROR",
                str(error),
                cause=error
            )
    
    async def _default_fallback_handler(self, ctx: DMSCServiceContext, error: AppError):
        """默认回退处理器"""
        
        ctx.logger.warn("error_handler", f"使用默认回退处理器处理错误: {error.code}")
        
        # 返回默认错误响应
        return {
            "error": True,
            "category": error.category.value,
            "code": error.code,
            "message": error.message,
            "details": error.details
        }

# 使用错误处理器
async def business_logic_with_error_handling(ctx):
    """带错误处理的业务逻辑"""
    
    error_handler = ErrorHandler()
    
    # 注册特定错误处理器
    async def handle_validation_error(ctx, error: ValidationError):
        return {
            "error": True,
            "type": "validation",
            "field": error.field,
            "message": error.message
        }
    
    async def handle_business_error(ctx, error: BusinessError):
        return {
            "error": True,
            "type": "business",
            "code": error.code,
            "message": error.message
        }
    
    error_handler.register_handler(ErrorCategory.VALIDATION, handle_validation_error)
    error_handler.register_handler(ErrorCategory.BUSINESS, handle_business_error)
    
    try:
        # 执行业务逻辑
        result = await risky_business_operation(ctx)
        return result
        
    except Exception as e:
        # 处理错误
        return await error_handler.handle_error(ctx, e)

async def risky_business_operation(ctx):
    """可能失败的操作"""
    import random
    
    # 模拟各种错误
    error_type = random.choice([
        "validation", "business", "infrastructure", "none"
    ])
    
    if error_type == "validation":
        raise ValidationError("email", "无效的邮箱地址", "invalid-email")
    elif error_type == "business":
        raise BusinessError("INSUFFICIENT_BALANCE", "余额不足", {"balance": 100, "required": 200})
    elif error_type == "infrastructure":
        raise InfrastructureError("database", "数据库连接失败", ConnectionError("连接超时"))
    
    return {"status": "success", "data": "operation_result"}
```

<div align="center">

## 下一步

</div>

- [API参考](./04-api-reference/README.md) - 查看详细的API文档
- [使用示例](./05-usage-examples/README.md) - 学习更多实际应用示例
- [最佳实践](./06-best-practices.md) - 了解开发最佳实践