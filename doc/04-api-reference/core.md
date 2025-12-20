<div align="center">

# Core API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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
| `with_config(path)` | 添加配置文件 | `path: &str` | `DMSCResult<Self>` |
| `with_logging(config)` | 添加日志配置 | `config: DMSCLogConfig` | `DMSCResult<Self>` |
| `with_observability(config)` | 添加可观测性配置 | `config: DMSCObservabilityConfig` | `DMSCResult<Self>` |
| `with_cache(config)` | 添加缓存配置 | `config: DMSCCacheConfig` | `DMSCResult<Self>` |
| `with_queue(config)` | 添加队列配置 | `config: DMSCQueueConfig` | `DMSCResult<Self>` |
| `with_module(module)` | 添加自定义模块 | `module: impl DMSCModule` | `DMSCResult<Self>` |
| `with_async_module(module)` | 添加自定义异步模块 | `module: impl AsyncServiceModule` | `DMSCResult<Self>` |
| `build()` | 构建应用运行时 | 无 | `DMSCResult<DMSCAppRuntime>` |

#### 使用示例

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_observability(DMSCObservabilityConfig::default())?
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

服务上下文，提供对所有模块功能的访问。

#### 方法

| 方法 | 描述 | 返回值 |
|:--------|:-------------|:--------|
| `logger()` | 获取日志记录器 | `&DMSCLogger` |
| `config()` | 获取配置管理器 | `&DMSCConfig` |
| `cache()` | 获取缓存服务 | `&DMSCCacheModule` |
| `queue()` | 获取队列服务 | `&DMSCQueueModule` |
| `fs()` | 获取文件系统服务 | `&DMSCFileSystem` |
| `auth()` | 获取认证服务 | `&DMSCAuthModule` |
| `device()` | 获取设备管理服务 | `&DMSCDeviceControlModule` |
| `gateway()` | 获取网关服务 | `&DMSCGateway` |
| `service_mesh()` | 获取服务网格服务 | `&DMSCServiceMesh` |
| `observability()` | 获取可观测性服务 | `&DMSCObservabilityModule` |

#### 使用示例

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // 访问日志功能
    ctx.logger().info("service", "DMSC service started")?;
    
    // 访问配置功能
    let service_name = ctx.config().get("service.name").unwrap_or("unknown");
    
    // 访问缓存功能
    ctx.cache().set("key", "value", Some(3600)).await?;
    
    Ok(())
}).await
```

### DMSCModule

模块 trait，用于创建自定义同步模块。

#### 方法

| 方法 | 描述 | 默认实现 |
|:--------|:-------------|:--------|
| `name()` | 返回模块名称 | 无，必须实现 |
| `priority()` | 返回模块优先级 | 返回 `50` |
| `initialize(ctx)` | 初始化模块 | 返回 `Ok(())` |
| `start(ctx)` | 启动模块 | 返回 `Ok(())` |
| `stop(ctx)` | 停止模块 | 返回 `Ok(())` |

#### 使用示例

```rust
struct MyCustomModule;

impl DMSCModule for MyCustomModule {
    fn name(&self) -> &str {
        "my_custom_module"
    }
    
    fn initialize(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "Module initialized")?;
        Ok(())
    }
}
```

### AsyncServiceModule

异步模块 trait，用于创建自定义异步模块。

#### 方法

| 方法 | 描述 | 默认实现 |
|:--------|:-------------|:--------|
| `name()` | 返回模块名称 | 无，必须实现 |
| `priority()` | 返回模块优先级 | 返回 `50` |
| `initialize(ctx)` | 异步初始化模块 | 返回 `Ok(())` |
| `start(ctx)` | 异步启动模块 | 返回 `Ok(())` |
| `stop(ctx)` | 异步停止模块 | 返回 `Ok(())` |

#### 使用示例

```rust
struct MyAsyncModule;

#[async_trait::async_trait]
impl AsyncServiceModule for MyAsyncModule {
    fn name(&self) -> &str {
        "my_async_module"
    }
    
    async fn initialize(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "Async module initialized")?;
        Ok(())
    }
}
```

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

1. **使用prelude模块**：通过`use dms::prelude::*`导入常用类型，简化代码
2. **按需配置模块**：只添加应用所需的模块，减少资源消耗
3. **合理使用服务上下文**：通过上下文访问模块功能，避免直接依赖具体实现
4. **实现自定义模块**：根据需要实现自定义模块扩展DMSC功能
5. **正确处理错误**：使用`?`运算符传播错误，或显式处理错误

<div align="center">

## 示例代码

</div>

### 完整应用示例

```rust
use dms::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .with_cache(DMSCCacheConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        
        // 使用缓存
        ctx.cache().set("key", "value", Some(3600)).await?;
        let value = ctx.cache().get("key").await?;
        ctx.logger().info("cache", &format!("Cache value: {:?}", value))?;
        
        Ok(())
    }).await
}
```
<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置
- [cache](./cache.md): 缓存模块，提供多后端缓存抽象，缓存用户会话和权限数据
- [database](./database.md): 数据库模块，提供用户数据持久化和查询功能
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [security](./security.md): 安全模块，提供加密、哈希和验证功能
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据