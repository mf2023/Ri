<div align="center">

# 核心概念

**Version: 1.0.0**

**Last modified date: 2025-12-12**

本章深入介绍DMSC的核心设计理念和关键组件，帮助您更好地理解和使用DMSC框架。

## 1. 模块化架构

DMSC采用高度模块化的设计，将不同功能划分为独立的模块，支持按需组合和扩展。

</div>

### 1.1 设计原则

- **单一职责**：每个模块专注于一个特定领域的功能
- **松耦合**：模块间通过明确定义的接口通信，减少依赖
- **高内聚**：相关功能集中在同一模块内
- **可扩展性**：支持通过自定义模块扩展功能
- **可测试性**：模块可以独立测试

### 1.2 模块依赖关系

模块间存在一定的依赖关系，DMSC框架会自动处理模块的加载顺序：

```
core → log → config → hooks → observability → [其他模块]
```

- **core**：最基础的模块，提供运行时和核心功能
- **log**：依赖core，提供日志功能
- **config**：依赖core，提供配置管理
- **hooks**：依赖core，提供生命周期钩子
- **observability**：依赖core、log和hooks，提供可观测性功能
- 其他模块：根据需要依赖上述基础模块

### 1.3 模块组合

您可以根据应用需求，选择性地组合所需的模块：

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_cache(DMSCCacheConfig::default())?
    // 只包含需要的模块
    .build()?;
```

<div align="center">

## 2. 服务上下文

`DMSCServiceContext`是DMSC应用的核心，提供对所有模块功能的访问。

</div>

### 2.1 设计理念

服务上下文采用依赖注入模式，将所有模块的功能集中在一个统一的接口中，便于：

- **统一访问**：通过一个对象访问所有模块功能
- **依赖解耦**：业务代码不直接依赖具体模块实现
- **测试友好**：便于在测试中替换具体实现
- **扩展性**：新模块可以无缝集成到上下文中

### 2.2 核心功能访问

通过服务上下文，您可以访问各种模块的功能：

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // 访问日志功能
    ctx.logger().info("service", "DMSC service started")?;
    
    // 访问配置功能
    let service_name = ctx.config().get("service.name").unwrap_or("unknown");
    
    // 访问缓存功能
    ctx.cache().set("key", "value", Some(3600)).await?;
    
    // 访问文件系统功能
    ctx.fs().write_file("data.txt", "content").await?;
    
    Ok(())
}).await
```

### 2.3 上下文生命周期

服务上下文的生命周期与应用程序的生命周期一致：

1. **创建**：在`DMSCAppBuilder::build()`时创建
2. **使用**：在应用运行期间通过闭包传递给业务逻辑
3. **销毁**：在应用程序退出时自动销毁

<div align="center">

## 3. 模块系统

DMSC的模块系统允许您扩展框架功能，实现自定义模块。

</div>

### 3.1 模块类型

DMSC支持两种类型的模块：

- **同步模块**：实现`DMSCModule` trait，在主线程中执行
- **异步模块**：实现`AsyncServiceModule` trait，在异步上下文中执行

### 3.2 自定义模块示例

创建一个自定义同步模块：

```rust
use dms::core::module::DMSCModule;
use dms::core::context::DMSCServiceContext;
use dms::core::error::DMSCResult;

pub struct MyCustomModule {
    // 模块配置
}

impl DMSCModule for MyCustomModule {
    fn name(&self) -> &str {
        "my_custom_module"
    }
    
    fn initialize(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 模块初始化逻辑
        ctx.logger().info(self.name(), "My custom module initialized")?;
        Ok(())
    }
    
    fn start(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 模块启动逻辑
        Ok(())
    }
    
    fn stop(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 模块停止逻辑
        Ok(())
    }
}
```

注册自定义模块：

```rust
let app = DMSCAppBuilder::new()
    // 其他配置
    .with_module(MyCustomModule { /* 配置 */ })?
    .build()?;
```

### 3.3 模块优先级

您可以为模块设置优先级，控制模块的加载顺序：

```rust
impl DMSCModule for MyCustomModule {
    // 其他方法
    
    fn priority(&self) -> u32 {
        100 // 数值越大，优先级越高
    }
}
```

<div align="center">

## 4. 生命周期管理

DMSC应用和模块都有明确的生命周期，确保资源的正确初始化和释放。

</div>

### 4.1 应用生命周期

1. **构建**：通过`DMSCAppBuilder`配置和构建应用
2. **初始化**：初始化所有注册的模块
3. **启动**：启动所有模块
4. **运行**：执行用户提供的业务逻辑
5. **停止**：停止所有模块
6. **清理**：释放资源

### 4.2 模块生命周期

每个模块都经历以下生命周期阶段：

1. **初始化**：调用`initialize()`方法，进行模块初始化
2. **启动**：调用`start()`方法，启动模块服务
3. **运行**：模块处于运行状态，处理请求
4. **停止**：调用`stop()`方法，停止模块服务

### 4.3 生命周期钩子

DMSC提供生命周期钩子，允许您在特定阶段执行自定义逻辑：

```rust
use dms::hooks::{DMSCHookBus, DMSCHookKind};

// 在应用构建时注册钩子
let app = DMSCAppBuilder::new()
    // 其他配置
    .build()?;

// 获取钩子总线
let hook_bus = app.hook_bus();

// 注册启动前钩子
hook_bus.register(DMSCHookKind::BeforeStart, |ctx| {
    ctx.logger().info("hooks", "Before start hook executed")?;
    Ok(())
});
```

支持的钩子类型：

- `BeforeInitialize`：初始化前
- `AfterInitialize`：初始化后
- `BeforeStart`：启动前
- `AfterStart`：启动后
- `BeforeStop`：停止前
- `AfterStop`：停止后

<div align="center">

## 5. 错误处理机制

DMSC采用统一的错误处理机制，确保错误信息的一致性和完整性。

</div>

### 5.1 错误类型

DMSC使用`DMSCError`类型表示所有错误，它包含：

- **错误代码**：唯一标识错误类型
- **错误消息**：详细的错误描述
- **错误上下文**：可选的上下文信息
- **源代码位置**：错误发生的文件和行号
- **内部错误**：可选的嵌套错误

### 5.2 结果类型

DMSC定义了`DMSCResult`类型别名，简化错误处理：

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

### 5.3 错误传播

在异步代码中，DMSC错误可以通过`?`运算符自动传播：

```rust
async fn my_function(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    let value = ctx.cache().get("key").await?; // 错误自动传播
    Ok(())
}
```

### 5.4 自定义错误

您可以创建自定义错误，并转换为`DMSCError`：

```rust
use dms::core::error::{DMSCError, DMSCResult};

fn my_custom_error() -> DMSCResult<()> {
    Err(DMSCError::new("CUSTOM_ERROR", "This is a custom error"))
}
```

<div align="center">

## 6. 异步设计

DMSC采用异步优先的设计，充分利用现代硬件的并发能力。

</div>

### 6.1 异步运行时

DMSC基于Tokio异步运行时，支持高并发和非阻塞I/O操作。

### 6.2 异步模块

您可以创建异步模块，处理异步操作：

```rust
use dms::core::module::AsyncServiceModule;
use dms::core::context::DMSCServiceContext;
use dms::core::error::DMSCResult;

pub struct MyAsyncModule {
    // 模块配置
}

#[async_trait::async_trait]
impl AsyncServiceModule for MyAsyncModule {
    fn name(&self) -> &str {
        "my_async_module"
    }
    
    async fn initialize(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 异步初始化逻辑
        Ok(())
    }
    
    async fn start(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 异步启动逻辑
        Ok(())
    }
    
    async fn stop(&mut self, ctx: &DMSCServiceContext) -> DMSCResult<()> {
        // 异步停止逻辑
        Ok(())
    }
}
```

### 6.3 异步API

大多数DMSC API都是异步的，使用`async/await`语法：

```rust
// 异步缓存操作
ctx.cache().set("key", "value", Some(3600)).await?;
let value = ctx.cache().get("key").await?;

// 异步文件操作
ctx.fs().write_file("data.txt", "content").await?;
let content = ctx.fs().read_file("data.txt").await?;
```

<div align="center">

## 7. 可观测性设计

DMSC内置了完整的可观测性支持，帮助您监控和调试应用。

</div>

### 7.1 分布式追踪

DMSC实现了W3C Trace Context标准，支持跨服务的分布式追踪：

```rust
use dms::observability::traced;

#[traced(name = "user_service")]
async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    // 自动记录追踪信息
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

### 7.2 指标收集

DMSC内置Prometheus指标收集，支持多种指标类型：

- **Counter**：单调递增的计数器
- **Gauge**：可增可减的仪表盘
- **Histogram**：分布直方图
- **Summary**：分位数统计

### 7.3 日志集成

DMSC的日志系统自动包含追踪上下文，便于关联日志和追踪信息：

```json
{
  "timestamp": "2025-12-12T15:30:00Z",
  "level": "info",
  "module": "service",
  "message": "DMSC service started",
  "trace_id": "abc123",
  "span_id": "def456"
}
```

<div align="center">

## 8. 配置管理

DMSC支持多源配置管理，允许您从不同来源加载配置。

</div>

### 8.1 配置源优先级

DMSC按照以下优先级加载配置（从高到低）：

1. **运行时参数**：通过代码设置的配置
2. **环境变量**：以`DMSC_`为前缀的环境变量
3. **配置文件**：YAML、TOML或JSON格式的配置文件
4. **默认值**：模块提供的默认配置

### 8.2 配置热重载

DMSC支持配置热重载，无需重启应用即可更新配置：

```yaml
config:
  watch_enabled: true
  watch_interval: 30s
```

### 8.3 配置访问

您可以通过服务上下文访问配置：

```rust
// 获取字符串配置
let service_name = ctx.config().get("service.name").unwrap_or("unknown");

// 获取整数配置
let port = ctx.config().get("service.port").unwrap_or(8080);

// 获取布尔配置
let enabled = ctx.config().get("feature.enabled").unwrap_or(false);
```

<div align="center">

## 9. 安全性设计

DMSC内置了多种安全机制，保护应用程序的安全。

</div>

### 9.1 安全文件系统

DMSC提供安全的文件系统操作，防止路径遍历和其他安全问题：

```rust
// 安全的文件写入，防止路径遍历
ctx.fs().write_file("data.txt", "content").await?;

// 安全的文件读取
let content = ctx.fs().read_file("data.txt").await?;
```

### 9.2 认证与授权

DMSC的auth模块提供完整的认证和授权机制：

- **JWT认证**：支持JSON Web Token
- **OAuth2**：支持OAuth2.0协议
- **权限管理**：基于角色的访问控制
- **API密钥**：支持API密钥认证

### 9.3 安全日志

DMSC的日志系统自动过滤敏感信息，防止泄露机密数据：

```rust
// 敏感信息会被自动过滤
ctx.logger().info("auth", &format!("User authenticated: {}", user_id))?;
```

<div align="center">

## 10. 性能优化

DMSC采用多种性能优化技术，确保高性能和低资源消耗。

</div>

### 10.1 零拷贝设计

对于I/O密集型操作，DMSC采用零拷贝设计，减少内存拷贝开销。

### 10.2 连接池

DMSC为数据库、Redis等资源提供连接池，减少连接建立和销毁的开销。

### 10.3 异步I/O

DMSC充分利用异步I/O，减少线程上下文切换开销，提高并发处理能力。

### 10.4 内存管理

DMSC采用高效的内存管理策略，减少内存分配和垃圾回收开销。

<div align="center">

## 总结

</div>

DMSC的核心设计理念是：

- **模块化**：高度模块化的架构，支持按需组合
- **异步优先**：充分利用现代硬件的并发能力
- **可观测性**：内置完整的监控和追踪支持
- **安全性**：内置多种安全机制
- **易用性**：提供简洁的API和良好的文档
- **扩展性**：支持通过自定义模块扩展功能

理解这些核心概念，将帮助您更好地设计和开发基于DMSC的应用程序。

<div align="center">

## 下一步

</div> 

- [API 参考](./04-api-reference/README.md)：详细的模块 API 文档
- [使用示例](./05-usage-examples/README.md)：各种场景下的使用示例
- [最佳实践](./06-best-practices.md)：开发 DMSC 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：常见问题和解决方案
- [术语表](./08-glossary.md)：核心术语解释