<div align="center">

# Log API参考

**Version: 0.1.7**

**Last modified date: 2026-02-11**

log模块提供结构化日志记录与多后端支持，支持日志级别、格式化、采样等功能。

## 模块概述

</div>

log模块包含以下子模块：

- **core**: 日志核心接口和类型定义
- **formatters**: 日志格式化器
- **backends**: 日志后端实现
- **sampling**: 日志采样机制
- **analytics**: 日志分析功能

<div align="center">

## 核心组件

</div>

### DMSCLogger

日志记录器主接口，提供统一的日志记录功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `trace(message)` | 记录跟踪日志 | `message: impl Display` | `()` |
| `debug(message)` | 记录调试日志 | `message: impl Display` | `()` |
| `info(message)` | 记录信息日志 | `message: impl Display` | `()` |
| `warn(message)` | 记录警告日志 | `message: impl Display` | `()` |
| `error(message)` | 记录错误日志 | `message: impl Display` | `()` |
| `fatal(message)` | 记录致命日志 | `message: impl Display` | `()` |
| `log(level, message)` | 记录指定级别日志 | `level: DMSCLogLevel`, `message: impl Display` | `()` |
| `with_field(key, value)` | 添加字段到日志上下文 | `key: &str`, `value: impl Serialize` | `DMSCLogger` |
| `with_fields(fields)` | 添加多个字段 | `fields: impl Serialize` | `DMSCLogger` |
| `with_span(name)` | 创建日志跨度 | `name: &str` | `LogSpan` |
| `flush()` | 刷新日志缓冲区 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 基本日志记录
ctx.log().info("Application started");
ctx.log().warn("Configuration file not found, using defaults");
ctx.log().error("Database connection failed");

// 带字段的日志记录
ctx.log()
    .with_field("user_id", 12345)
    .with_field("action", "login")
    .info("User login successful");

// 结构化日志记录
let user_data = serde_json::json!({
    "id": 12345,
    "name": "John Doe",
    "email": "john@example.com"
});

ctx.log()
    .with_fields(user_data)
    .info("User profile updated");
```

### DMSCLogLevel

日志级别枚举类型。

#### 变体

| 变体 | 描述 | 数值 |
|:--------|:-------------|:-----|
| `Trace` | 最详细的调试信息 | 0 |
| `Debug` | 调试信息 | 1 |
| `Info` | 一般信息 | 2 |
| `Warn` | 警告信息 | 3 |
| `Error` | 错误信息 | 4 |
| `Fatal` | 致命错误 | 5 |

#### 颜色方块

每个日志级别都有对应的颜色方块标识：

| 级别 | 颜色方块 | 说明 |
|:--------|:-------------|:-----|
| `Debug` | 🟦 | 蓝色方块，用于调试信息 |
| `Info` | 🟩 | 绿色方块，用于一般信息 |
| `Warn` | 🟨 | 黄色方块，用于警告信息 |
| `Error` | 🟥 | 红色方块，用于错误信息 |

### 日志格式

#### 文本格式（带颜色方块）

```
🟩 | 2024-01-15 10:30:45.123 | INFO  | service | event=service_start | 应用程序已启动 | port=8080
🟨 | 2024-01-15 10:30:45.456 | WARN  | cache   | event=cache_miss    | 缓存未命中
🟥 | 2024-01-15 10:30:45.789 | ERROR | db      | event=conn_failed   | 连接超时 | retry=3
```

格式说明：
- `🟩` - 颜色方块（根据日志级别变化）
- `2024-01-15 10:30:45.123` - 时间戳
- `INFO` - 日志级别
- `service` - 目标模块
- `event=service_start` - 事件名称
- `应用程序已启动` - 日志消息
- `port=8080` - 上下文字段

#### 文本格式（不带颜色方块）

当 `color_blocks` 设置为 `false` 时：

```
2024-01-15 10:30:45.123 | INFO  | service | event=service_start | 应用程序已启动 | port=8080
2024-01-15 10:30:45.456 | WARN  | cache   | event=cache_miss    | 缓存未命中
```

#### JSON格式

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "target": "service",
  "message": "应用程序已启动",
  "event": "service_start",
  "port": "8080"
}
```

### DMSCLogConfig

日志配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `level` | `DMSCLogLevel` | 日志级别 | `Info` |
| `console_enabled` | `bool` | 启用控制台输出 | `true` |
| `file_enabled` | `bool` | 启用文件输出 | `false` |
| `sampling_default` | `f32` | 默认采样率 (0.0-1.0) | `1.0` |
| `file_name` | `String` | 日志文件名 | `"dms.log"` |
| `json_format` | `bool` | 使用JSON格式 | `false` |
| `rotate_when` | `String` | 何时轮转 ("size" 或 "none") | `"size"` |
| `color_blocks` | `bool` | 在日志输出中使用颜色方块 | `true` |

#### 配置示例

```rust
use dmsc::log::DMSCLogConfig;

let log_config = DMSCLogConfig {
    level: DMSCLogLevel::Info,
    console_enabled: true,
    file_enabled: true,
    sampling_default: 1.0,
    file_name: "myapp.log".to_string(),
    json_format: false,
    rotate_when: "size".to_string(),
    color_blocks: true,          // 在日志输出中启用颜色方块
    ..Default::default()
};
```

<div align="center">

## 日志过滤

</div>

### 级别过滤

```rust
use dmsc::prelude::*;

// 设置全局日志级别
ctx.log().set_level(DMSCLogLevel::Warn);
```

<div align="center">

## 错误处理

</div>

### 日志错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `LOG_FILE_NOT_FOUND` | 日志文件未找到 |
| `LOG_FILE_PERMISSION_DENIED` | 日志文件权限不足 |
| `LOG_FORMAT_ERROR` | 日志格式错误 |
| `LOG_BACKEND_ERROR` | 日志后端错误 |

### 错误处理示例

```rust
use dmsc::prelude::*;

match ctx.log().flush() {
    Ok(_) => {
        // 日志刷新成功
    }
    Err(DMSCError { code, .. }) if code == "LOG_FILE_PERMISSION_DENIED" => {
        // 文件权限错误，回退到标准输出
        ctx.log().set_backend(DMSCLogBackend::Stdout);
        ctx.log().warn("Falling back to stdout logging due to file permission error");
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

1. **使用结构化日志**: 使用字段而不是字符串拼接
2. **适当的日志级别**: 根据重要性选择合适的日志级别
3. **避免记录敏感信息**: 不要记录密码、密钥等敏感信息
4. **定期轮转日志**: 使用日志轮转避免磁盘空间耗尽

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
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
