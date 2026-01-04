<div align="center">

# 基础应用示例

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本示例展示如何构建一个简单的DMSC应用，包括应用配置、运行和基本功能使用。

## 示例概述

</div>

本示例将创建一个基本的DMSC应用，实现以下功能：

- 加载配置文件
- 启用日志记录
- 启用可观测性
- 输出一条启动日志

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-basic-example
cd dms-basic-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-basic-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dms::prelude::*;

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
        // 获取服务名称和版本
        let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
        let service_version = ctx.config().config().get_str("service.version").unwrap_or("unknown");
        
        // 输出启动日志
        ctx.logger().info(
            "service", 
            &format!("{} v{} started successfully", service_name, service_version)
        )?;
        
        // 输出配置信息
        let log_level = ctx.config().config().get_str("logging.level").unwrap_or("info");
        ctx.logger().info(
            "config", 
            &format!("Logging level: {}", log_level)
        )?
        
        // 等待3秒，模拟业务运行
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        ctx.logger().info("service", "Service finished successfully")?;
        
        Ok(())
    }).await
}
```
<div align="center">

## 代码解析

</div>

### 1. 导入依赖

```rust
use dms::prelude::*;
```

这行代码导入了DMSC中最常用的类型和特性，简化了代码编写。`prelude`模块包含了构建DMSC应用所需的核心组件。

### 2. 主函数

```rust
#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 代码...
}
```

- `#[tokio::main]`：将主函数转换为异步函数，并使用Tokio运行时执行
- `async fn main()`：定义异步主函数
- `-> DMSCResult<()>`：返回DMSC结果类型，用于错误处理

### 3. 构建应用

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_observability(DMSCObservabilityConfig::default())?
    .build()?;
```

- `DMSCAppBuilder::new()`：创建新的应用构建器
- `.with_config("config.yaml")?`：加载配置文件
- `.with_logging(DMSCLogConfig::default())?`：启用日志记录，使用默认配置
- `.with_observability(DMSCObservabilityConfig::default())?`：启用可观测性，使用默认配置
- `.build()?`：构建应用运行时

### 4. 运行应用

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // 业务逻辑
}).await
```

- `app.run()`：启动应用运行时
- `|ctx: &DMSCServiceContext|`：闭包参数，接收服务上下文
- `async move`：异步闭包，允许在闭包内使用`await`

### 5. 业务逻辑

```rust
let service_name = ctx.config().get("service.name").unwrap_or("unknown");
let service_version = ctx.config().get("service.version").unwrap_or("unknown");

ctx.logger().info(
    "service", 
    &format!("{} v{} started successfully", service_name, service_version)
)?;
```

- `ctx.config().get()`：从配置中获取值
- `ctx.logger().info()`：记录一条信息日志
- `?`：传播错误

<div align="center">

## 运行步骤

</div>

### 1. 构建项目

```bash
cargo build
```

### 2. 运行项目

```bash
cargo run
```

<div align="center">

## 预期结果

</div>

运行示例后，您应该会看到类似以下的输出：

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"dms-basic-example v1.0.0 started successfully","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"config","message":"Logging level: info","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"Service finished successfully","trace_id":"abc123","span_id":"def456"}
```

<div align="center">

## 扩展功能

</div>

### 1. 实现缓存支持

```rust
// 在应用构建时添加缓存支持
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_cache(DMSCCacheConfig::default())?
    .build()?;

// 在业务逻辑中使用缓存
app.run(|ctx: &DMSCServiceContext| async move {
    // 设置缓存
    ctx.cache().set("key", "value", 3600).await?;
    
    // 获取缓存
    let value: String = ctx.cache().get("key").await?;
    ctx.logger().info("cache", &format!("Cached value: {}", value))?;
    
    Ok(())
}).await
```

### 2. 实现队列支持

```rust
// 在应用构建时添加队列支持
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_queue(DMSCQueueConfig::default())?
    .build()?;

// 在业务逻辑中使用队列
app.run(|ctx: &DMSCServiceContext| async move {
    // 发送消息到队列
    ctx.queue().publish("task_queue", json!({
        "task_id": "task-123",
        "task_type": "data_processing",
        "priority": 1,
    })).await?;
    
    ctx.logger().info("queue", "Task message sent to queue")?;
    
    Ok(())
}).await
```

### 3. 实现文件系统操作

```rust
// 在业务逻辑中使用文件系统
app.run(|ctx: &DMSCServiceContext| async move {
    // 写入文件
    ctx.fs().write_file("data/config.json", r#"{"setting": "value"}"#).await?;
    
    // 读取文件
    let content = ctx.fs().read_file("data/config.json").await?;
    ctx.logger().info("fs", &format!("File content: {}", content))?;
    
    // 检查文件是否存在
    let exists = ctx.fs().file_exists("data/config.json").await?;
    ctx.logger().info("fs", &format!("File exists: {}", exists))?;
    
    Ok(())
}).await
```

### 4. 实现自定义模块

```rust
// 定义自定义模块
struct MyCustomModule {
    name: String,
}

impl MyCustomModule {
    async fn process_data(&self, data: &str) -> DMSCResult<String> {
        // 自定义处理逻辑
        Ok(format!("Processed by {}: {}", self.name, data))
    }
}

// 在应用构建时添加自定义模块
let custom_module = MyCustomModule {
    name: "MyProcessor".to_string(),
};

let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_module(custom_module)?
    .build()?;

// 在业务逻辑中使用自定义模块
app.run(|ctx: &DMSCServiceContext| async move {
    // 获取自定义模块
    let processor = ctx.module::<MyCustomModule>()?;
    
    // 使用自定义功能
    let result = processor.process_data("sample data").await?;
    ctx.logger().info("custom", &format!("Processing result: {}", result))?;
    
    Ok(())
}).await
```

<div align="center">

## 最佳实践

</div>

1. **从简单开始**：先创建基础应用，确保能正常运行，再逐步添加其他模块

2. **合理配置日志**：根据环境调整日志级别，开发环境使用`debug`，生产环境使用`info`或`warn`

3. **使用配置文件**：将配置信息放在配置文件中，避免硬编码，便于不同环境的部署

4. **错误处理**：使用`?`操作符传播错误，确保错误能被正确处理

5. **模块化设计**：将业务逻辑封装在函数或模块中，保持代码整洁

6. **测试驱动**：编写单元测试和集成测试，确保代码质量

<div align="center">

## 总结

</div>

本示例展示了如何构建一个简单的DMSC应用，包括：

- 项目创建和依赖添加
- 配置文件编写
- 应用构建和运行
- 基本功能使用

通过本示例，您应该已经了解了DMSC应用的基本结构和使用方式。您可以在此基础上进一步探索DMSC的其他功能。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
