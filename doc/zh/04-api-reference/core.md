<div align="center">

# Core API参考

**Version: 0.1.6**

**Last modified date: 2026-01-24**

core模块是DMSC的基础，提供运行时、错误处理、服务上下文和模块系统等核心功能。

## 模块概述

</div>

core模块包含以下子模块：

- **error**: 错误处理机制
- **context**: 服务上下文
- **module**: 模块系统
- **runtime**: 应用运行时
- **app_builder**: 应用构建器
- **app_runtime**: 应用运行时管理

<div align="center">

## 核心组件

</div>

### DMSCAppBuilder

应用构建器，用于配置和构建DMSC应用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的应用构建器 | 无 | `DMSCAppBuilder` |
| `with_config(path)` | 添加配置文件 | `path: impl Into<String>` | `DMSCResult<Self>` |
| `with_logging(config)` | 设置日志配置 | `config: DMSCLogConfig` | `DMSCResult<Self>` |
| `with_observability(config)` | 设置可观测性配置 | `config: DMSCObservabilityConfig` | `DMSCResult<Self>` |
| `with_module(module)` | 添加同步模块 | `module: Box<dyn ServiceModule>` | `Self` |
| `with_async_module(module)` | 添加异步模块 | `module: Box<dyn AsyncServiceModule>` | `Self` |
| `with_python_module(module)` | 添加Python模块（需要pyo3特性） | `module: DMSCPythonModuleAdapter` | `Self` |
| `with_dms_module(module)` | 添加DMSC模块 | `module: Box<dyn DMSCModule>` | `Self` |
| `with_modules(modules)` | 添加多个同步模块 | `modules: Vec<Box<dyn ServiceModule>>` | `Self` |
| `with_async_modules(modules)` | 添加多个异步模块 | `modules: Vec<Box<dyn AsyncServiceModule>>` | `Self` |
| `with_dms_modules(modules)` | 添加多个DMSC模块 | `modules: Vec<Box<dyn DMSCModule>>` | `Self` |
| `build()` | 构建应用运行时 | 无 | `DMSCResult<DMSCAppRuntime>` |

#### 使用示例

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_observability(DMSCObservabilityConfig::default())?
    .with_dms_module(Box::new(MyCustomModule::new()))
    .build()?;
```

### DMSCAppRuntime

应用运行时，用于管理DMSC应用的生命周期。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `run<F>(f)` | 运行应用，执行提供的业务逻辑 | `f: F`，其中 `F: Fn(&DMSCServiceContext) -> Fut` | `DMSCResult<()>` |
| `hook_bus()` | 获取钩子总线 | 无 | `&DMSCHookBus` |
| `stop()` | 停止应用 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    ctx.logger().info("service", "DMSC service started")?;
    Ok(())
}).await
```

### DMSCServiceContext

服务上下文，提供对核心功能的访问。

#### 方法

| 方法 | 描述 | 返回值 |
|:--------|:-------------|:--------|
| `fs()` | 获取文件系统访问器 | `&DMSCFileSystem` |
| `logger()` | 获取结构化日志记录器 | `&DMSCLogger` |
| `config()` | 获取配置管理器（共享所有权） | `Arc<DMSCConfigManager>` |
| `hooks()` | 获取钩子总线（共享所有权） | `Arc<DMSCHookBus>` |
| `hooks_mut()` | 获取可变钩子总线（仅在独享所有权时可用） | `&mut DMSCHookBus` |
| `config_mut()` | 获取可变配置管理器（仅在独享所有权时可用） | `&mut DMSCConfigManager` |
| `fs_mut()` | 获取可变文件系统访问器 | `&mut DMSCFileSystem` |
| `logger_mut()` | 获取可变日志记录器（仅在独享所有权时可用） | `&mut DMSCLogger` |
| `metrics_registry()` | 获取指标注册表（如果可用） | `Option<Arc<DMSCMetricsRegistry>>` |

#### 使用示例

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // 访问日志功能
    ctx.logger().info("service", "DMSC service started")?;
    
    // 访问配置功能
    let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
    
    // 访问文件系统功能
    ctx.fs().write_file("data.txt", "content").await?;
    
    Ok(())
}).await
```

### DMSCModule

异步模块 trait，用于创建自定义异步模块（推荐使用）。

#### 方法

| 方法 | 描述 | 参数 | 返回值 | 默认实现 |
|:--------|:-------------|:--------|:--------|:--------|
| `name()` | 返回模块名称 | 无 | `&str` | 无，必须实现 |
| `is_critical()` | 指示模块是否关键 | 无 | `bool` | 返回 `true` |
| `priority()` | 返回模块优先级 | 无 | `i32` | 返回 `0` |
| `dependencies()` | 返回模块依赖列表 | 无 | `Vec<&str>` | 返回空列表 |
| `init(ctx)` | 初始化模块 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `before_start(ctx)` | 启动前准备 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `start(ctx)` | 启动模块服务 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `after_start(ctx)` | 启动后操作 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `before_shutdown(ctx)` | 关闭前准备 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `shutdown(ctx)` | 关闭模块服务 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |
| `after_shutdown(ctx)` | 关闭后清理 | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | 返回 `Ok(())` |

#### 使用示例

```rust
struct MyCustomModule;

#[async_trait::async_trait]
impl DMSCModule for MyCustomModule {
    fn name(&self) -> &str {
        "my_custom_module"
    }
    
    async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "Module started")?;
        Ok(())
    }
}
```

### AsyncServiceModule

**注意**：这是一个内部 trait，不对外公开。用户应使用 `DMSCModule` trait 创建自定义模块。

异步模块 trait，用于框架内部的异步模块。

#### 方法

| 方法 | 描述 | 返回值 | 默认实现 |
|:--------|:-------------|:--------|:--------|
| `name()` | 返回模块名称 | `&str` | 无，必须实现 |
| `is_critical()` | 指示模块是否关键 | `bool` | 返回 `true` |
| `priority()` | 返回模块优先级 | `i32` | 返回 `0` |
| `dependencies()` | 返回模块依赖列表 | `Vec<&str>` | 返回空列表 |
| `init(ctx)` | 异步初始化模块 | `DMSCResult<()>` | 返回 `Ok(())` |
| `before_start(ctx)` | 异步启动前准备 | `DMSCResult<()>` | 返回 `Ok(())` |
| `start(ctx)` | 异步启动模块服务 | `DMSCResult<()>` | 返回 `Ok(())` |
| `after_start(ctx)` | 异步启动后操作 | `DMSCResult<()>` | 返回 `Ok(())` |
| `before_shutdown(ctx)` | 异步关闭前准备 | `DMSCResult<()>` | 返回 `Ok(())` |
| `shutdown(ctx)` | 异步关闭模块服务 | `DMSCResult<()>` | 返回 `Ok(())` |
| `after_shutdown(ctx)` | 异步关闭后清理 | `DMSCResult<()>` | 返回 `Ok(())` |

### DMSCError

DMSC的统一错误类型。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(code, message)` | 创建新的错误 | `code: &str`, `message: &str` | `DMSCError` |
| `with_context(context)` | 添加错误上下文 | `context: impl Into<String>` | `Self` |
| `with_source(source)` | 添加内部错误 | `source: impl std::error::Error + Send + Sync + 'static` | `Self` |
| `code()` | 获取错误代码 | 无 | `&str` |
| `message()` | 获取错误消息 | 无 | `&str` |
| `context()` | 获取错误上下文 | 无 | `Option<&str>` |

#### 使用示例

```rust
Err(DMSCError::new("INVALID_CONFIG", "Invalid configuration")
    .with_context("service.port must be a positive integer"))
```

### DMSCResult

结果类型别名，简化错误处理。

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

#### 使用示例

```rust
async fn my_function() -> DMSCResult<()> {
    // 业务逻辑
    Ok(())
}
```

### DMSCLockError

安全锁错误类型，专用于并发锁操作错误处理。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(context)` | 创建新的锁错误 | `context: &str` | `DMSCLockError` |
| `poisoned(context)` | 创建毒锁错误 | `context: &str` | `DMSCLockError` |
| `is_poisoned()` | 检查是否为毒锁错误 | 无 | `bool` |
| `context()` | 获取错误上下文 | 无 | `&str` |

#### 使用示例

```rust
match lock.read_safe("my data") {
    Ok(data) => println!("Data: {}", data),
    Err(e) if e.is_poisoned() => {
        log::error!("Lock poisoned: {}", e.context());
    }
    Err(e) => {
        log::error!("Lock error: {}", e.context());
    }
}
```

### DMSCLockResult

锁操作结果类型别名。

> **注意**：此类型为 Rust 专用类型别名，在 Python 中不可用。Python 中可直接使用 `Result[T, DMSCLockError]` 替代。

```rust
type DMSCLockResult<T> = Result<T, DMSCLockError>;
```

#### 使用示例

```rust
fn safe_read_data(lock: &RwLock<String>, context: &str) -> DMSCLockResult<String> {
    let data = lock.read_safe(context)?;
    Ok(data.clone())
}
```

### RwLockExtensions

为标准库 `RwLock` 提供安全锁获取扩展 trait。

> **注意**：此 trait 为 Rust 专用，在 Python 中不可用。Python 用户可直接使用 `RwLock.read()` 和 `RwLock.write()` 方法。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `read_safe(context)` | 安全获取读锁 | `context: &str` | `DMSCLockResult<RwLockReadGuard<T>>` |
| `write_safe(context)` | 安全获取写锁 | `context: &str` | `DMSCLockResult<RwLockWriteGuard<T>>` |
| `try_read_safe(context)` | 尝试获取读锁（非阻塞） | `context: &str` | `DMSCLockResult<Option<RwLockReadGuard<T>>>` |
| `try_write_safe(context)` | 尝试获取写锁（非阻塞） | `context: &str` | `DMSCLockResult<Option<RwLockWriteGuard<T>>>` |

#### 使用示例

```rust
use dmsc::core::lock::RwLockExtensions;

let lock = RwLock::new(42);

fn read_value(lock: &RwLock<i32>) -> DMSCLockResult<i32> {
    let value = lock.read_safe("reading counter")?;
    Ok(*value)
}

fn write_value(lock: &RwLock<i32>, new_value: i32) -> DMSCLockResult<()> {
    let mut value = lock.write_safe("writing counter")?;
    *value = new_value;
    Ok(())
}
```

### MutexExtensions

为标准库 `Mutex` 提供安全锁获取扩展 trait。

> **注意**：此 trait 为 Rust 专用，在 Python 中不可用。Python 用户可直接使用 `Mutex.lock()` 方法。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `lock_safe(context)` | 安全获取互斥锁 | `context: &str` | `DMSCLockResult<MutexGuard<T>>` |
| `try_lock_safe(context)` | 尝试获取互斥锁（非阻塞） | `context: &str` | `DMSCLockResult<Option<MutexGuard<T>>>` |

#### 使用示例

```rust
use dmsc::core::lock::MutexExtensions;

let mutex = Mutex::new(Vec::new());

fn push_item(mutex: &Mutex<Vec<String>>, item: String) -> DMSCLockResult<()> {
    let mut items = mutex.lock_safe("pushing item")?;
    items.push(item);
    Ok(())
}
```

<div align="center">

## 错误码

</div>

core模块定义了以下错误码：

| 错误码 | 描述 |
|:--------|:-------------|
| `INITIALIZATION_FAILED` | 初始化失败 |
| `START_FAILED` | 启动失败 |
| `STOP_FAILED` | 停止失败 |
| `INVALID_CONFIG` | 无效配置 |
| `MODULE_NOT_FOUND` | 模块未找到 |
| `MODULE_ALREADY_REGISTERED` | 模块已注册 |

<div align="center">

## 最佳实践

</div>

1. **使用prelude模块**：通过`use dmsc::prelude::*`导入常用类型，简化代码
2. **按需配置模块**：只添加应用所需的模块，减少资源消耗
3. **合理使用服务上下文**：通过上下文访问模块功能，避免直接依赖具体实现
4. **实现自定义模块**：根据需要实现自定义模块扩展DMSC功能
5. **正确处理错误**：使用`?`运算符传播错误，或显式处理错误

<div align="center">

## 示例代码

</div>

### 完整应用示例

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        
        // 访问配置
        let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
        ctx.logger().info("service", &format!("Service name: {}", service_name))?;
        
        Ok(())
    }).await
}
```
<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
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