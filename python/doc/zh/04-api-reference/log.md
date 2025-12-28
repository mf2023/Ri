<div align="center">

# 日志 API参考

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

日志模块提供结构化日志功能，支持多种输出格式和目的地。

## 模块概述

</div>

日志模块包含以下子模块：

- **config**: 日志配置管理
- **formatters**: 日志格式器
- **handlers**: 日志处理器
- **filters**: 日志过滤器
- **tracers**: 日志追踪器

<div align="center">

## 核心组件

</div>

### DMSCLogConfig

日志配置类，用于配置日志行为。

#### 构造函数

```python
DMSCLogConfig(
    level: str = "INFO",
    format: str = "json",
    enable_console: bool = True,
    enable_file: bool = False,
    file_path: str = "logs/app.log",
    max_file_size: str = "100MB",
    max_files: int = 10,
    compress: bool = True,
    enable_trace: bool = True,
    sampling_rate: float = 1.0
)
```

#### 参数说明

| 参数 | 描述 | 类型 | 默认值 |
|:--------|:-------------|:--------|:--------|
| `level` | 日志级别 | `str` | `"INFO"` |
| `format` | 日志格式 ("json" 或 "text") | `str` | `"json"` |
| `enable_console` | 是否输出到控制台 | `bool` | `True` |
| `enable_file` | 是否输出到文件 | `bool` | `False` |
| `file_path` | 日志文件路径 | `str` | `"logs/app.log"` |
| `max_file_size` | 单个日志文件最大大小 | `str` | `"100MB"` |
| `max_files` | 保留的日志文件数量 | `int` | `10` |
| `compress` | 是否压缩旧日志文件 | `bool` | `True` |
| `enable_trace` | 是否启用追踪上下文 | `bool` | `True` |
| `sampling_rate` | 日志采样率 (0.0-1.0) | `float` | `1.0` |

<div align="center">

#### 方法表

</div>

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `default()` | 获取默认配置 | 无 | `DMSCLogConfig` |
| `development()` | 获取开发环境配置 | 无 | `DMSCLogConfig` |
| `production()` | 获取生产环境配置 | 无 | `DMSCLogConfig` |

#### 使用示例

```python
from dmsc import DMSCLogConfig

# 基本配置
config = DMSCLogConfig(
    level="INFO",
    format="json",
    enable_console=True
)

# 文件日志配置
file_config = DMSCLogConfig(
    level="DEBUG",
    format="text",
    enable_console=True,
    enable_file=True,
    file_path="logs/myapp.log",
    max_file_size="50MB",
    max_files=5,
    compress=True
)

# 使用预设配置
dev_config = DMSCLogConfig.development()
prod_config = DMSCLogConfig.production()
```

### DMSCLogger

日志器接口，提供结构化日志记录功能。

<div align="center">

#### 方法表

</div>

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `debug(module, message, **kwargs)` | 记录调试日志 | `module: str`, `message: str`, `**kwargs` | `None` |
| `info(module, message, **kwargs)` | 记录信息日志 | `module: str`, `message: str`, `**kwargs` | `None` |
| `warn(module, message, **kwargs)` | 记录警告日志 | `module: str`, `message: str`, `**kwargs` | `None` |
| `error(module, message, **kwargs)` | 记录错误日志 | `module: str`, `message: str`, `**kwargs` | `None` |
| `log(level, module, message, **kwargs)` | 记录日志（通用方法） | `level: str`, `module: str`, `message: str`, `**kwargs` | `None` |

#### 使用示例

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # 配置日志
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True
    )
    
    # 构建应用
    app = DMSCAppBuilder().with_logging(log_config).build()
    
    async def business_logic(ctx):
        # 使用日志器
        ctx.logger.debug("my_module", "Debug information", extra_data="value")
        ctx.logger.info("my_module", "Application started", version="1.0.0")
        ctx.logger.warn("my_module", "Low memory warning", memory_usage="85%")
        ctx.logger.error("my_module", "Database connection failed", error="timeout")
        
        # 使用通用日志方法
        ctx.logger.log("INFO", "my_module", "Custom log message", custom_field="data")
        
        return "Logging completed"
    
    await app.run_async(business_logic)

asyncio.run(main())
```

### DMSCLogLevel

日志级别枚举。

#### 取值

```python
from dmsc import DMSCLogLevel

class DMSCLogLevel:
    DEBUG = "DEBUG"    # 调试信息
    INFO = "INFO"      # 一般信息
    WARN = "WARN"      # 警告信息
    ERROR = "ERROR"    # 错误信息
```

### DMSCLogFormat

日志格式枚举。

#### 取值

```python
from dmsc import DMSCLogFormat

class DMSCLogFormat:
    JSON = "json"      # JSON格式
    TEXT = "text"      # 文本格式
```

<div align="center">

## 高级功能

</div>

### 结构化日志

```python
# JSON格式日志示例
{
    "timestamp": "2025-12-27T10:30:00.123Z",
    "level": "INFO",
    "module": "auth",
    "message": "User login successful",
    "user_id": "12345",
    "ip_address": "192.168.1.1",
    "session_id": "abc-def-ghi",
    "trace_id": "xyz-123-456"
}
```

### 日志采样

```python
from dmsc import DMSCLogConfig

# 配置日志采样（只记录10%的日志）
config = DMSCLogConfig(
    level="INFO",
    sampling_rate=0.1  # 10%采样率
)
```

### 日志追踪

```python
async def traced_operation(ctx):
    # 生成追踪ID
    trace_id = generate_trace_id()
    
    # 在日志中包含追踪信息
    ctx.logger.info(
        "operation",
        "Starting operation",
        trace_id=trace_id,
        operation_name="data_processing"
    )
    
    try:
        # 执行操作
        result = await process_data()
        
        ctx.logger.info(
            "operation",
            "Operation completed successfully",
            trace_id=trace_id,
            duration_ms=150
        )
        
        return result
    
    except Exception as e:
        ctx.logger.error(
            "operation",
            "Operation failed",
            trace_id=trace_id,
            error=str(e),
            error_type=type(e).__name__
        )
        raise
```

### 日志轮转

```python
from dmsc import DMSCLogConfig

# 配置日志轮转
config = DMSCLogConfig(
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="100MB",  # 单个文件最大100MB
    max_files=10,           # 保留10个文件
    compress=True           # 压缩旧文件
)
```

### 环境特定配置

```python
import os
from dmsc import DMSCLogConfig

def get_log_config():
    """根据环境获取日志配置"""
    env = os.getenv("ENVIRONMENT", "development")
    
    if env == "production":
        return DMSCLogConfig(
            level="WARN",
            format="json",
            enable_console=False,
            enable_file=True,
            file_path="/var/log/myapp/app.log",
            max_file_size="500MB",
            max_files=30,
            compress=True
        )
    elif env == "testing":
        return DMSCLogConfig(
            level="DEBUG",
            format="text",
            enable_console=True,
            enable_file=True,
            file_path="logs/test.log"
        )
    else:  # development
        return DMSCLogConfig.development()
```

<div align="center">

## 日志最佳实践

</div>

### 1. 使用结构化日志

```python
# ❌ 不好的做法
ctx.logger.info("auth", f"User {user_id} logged in from {ip_address}")

# ✅ 好的做法
ctx.logger.info(
    "auth",
    "User login successful",
    user_id=user_id,
    ip_address=ip_address,
    login_method="password",
    user_agent=request.headers.get("User-Agent")
)
```

### 2. 适当的日志级别

```python
# DEBUG: 调试信息，只在开发环境启用
ctx.logger.debug("database", "SQL query executed", query="SELECT * FROM users", duration_ms=45)

# INFO: 重要业务事件
ctx.logger.info("payment", "Payment processed successfully", order_id="12345", amount=99.99)

# WARN: 警告但不影响系统运行
ctx.logger.warn("cache", "Cache miss detected", key="user:123", fallback="database")

# ERROR: 错误事件，需要关注
ctx.logger.error("database", "Connection failed", error="timeout", retry_count=3)
```

### 3. 包含上下文信息

```python
async def process_request(ctx, request_id: str, user_id: str):
    # 在请求开始时记录上下文
    ctx.logger.info(
        "request",
        "Processing request",
        request_id=request_id,
        user_id=user_id,
        endpoint=request.path,
        method=request.method
    )
    
    try:
        # 处理请求
        result = await handle_request(request)
        
        # 记录成功结果
        ctx.logger.info(
            "request",
            "Request completed successfully",
            request_id=request_id,
            duration_ms=150,
            status_code=200
        )
        
        return result
    
    except Exception as e:
        # 记录错误信息
        ctx.logger.error(
            "request",
            "Request failed",
            request_id=request_id,
            error=str(e),
            error_type=type(e).__name__,
            status_code=500
        )
        raise
```

### 4. 敏感信息处理

```python
# ❌ 不要记录敏感信息
ctx.logger.info("auth", "User login", password=user_password)  # 危险！

# ✅ 安全地记录认证事件
ctx.logger.info(
    "auth",
    "User login attempt",
    user_id=user_id,
    ip_address=ip_address,
    login_method="password",
    success=True
)
```

<div align="center">

## 性能考虑

</div>

### 异步日志记录

```python
# DMSC日志器是异步的，不会阻塞业务逻辑
async def business_operation(ctx):
    # 日志记录不会阻塞后续操作
    ctx.logger.info("operation", "Starting operation")
    
    # 立即执行后续逻辑
    result = await perform_operation()
    
    # 日志记录是异步的
    ctx.logger.info("operation", "Operation completed")
    
    return result
```

### 日志采样对性能的影响

```python
from dmsc import DMSCLogConfig

# 高流量环境下使用采样减少日志量
config = DMSCLogConfig(
    level="INFO",
    sampling_rate=0.01  # 只记录1%的日志，大幅提升性能
)
```

<div align="center">

## 故障排除

</div>

### 日志不输出

```python
# 检查日志级别
config = DMSCLogConfig(level="DEBUG")  # 确保级别足够低

# 检查输出配置
config.enable_console = True  # 确保启用了控制台输出
config.enable_file = True   # 如果使用文件日志

# 验证日志器
print(f"Logger level: {ctx.logger.level}")
print(f"Console enabled: {config.enable_console}")
print(f"File enabled: {config.enable_file}")
```

### 日志文件过大

```python
from dmsc import DMSCLogConfig

# 配置日志轮转防止文件过大
config = DMSCLogConfig(
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="50MB",  # 限制单个文件大小
    max_files=5,           # 限制文件数量
    compress=True,         # 压缩旧文件
    sampling_rate=0.1      # 采样减少日志量
)
```