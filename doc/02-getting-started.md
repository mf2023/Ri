<div align="center">

# 快速开始

**Version: 1.0.0**

**Last modified date: 2025-12-12**

本指南将帮助您快速上手DMS，从安装到创建第一个应用。

## 前置要求

</div>

在开始之前，确保您的环境满足以下要求：

- **Rust**: 1.65+ (2021 版本)
- **Cargo**: 1.65+ (Rust 包管理器)
- **平台**: Linux、macOS 或 Windows

您可以使用以下命令检查 Rust 和 Cargo 版本：

```bash
rustc --version
cargo --version
```

如果您还没有安装 Rust，可以通过 [rustup](https://rustup.rs/) 安装：

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# 访问 https://rustup.rs/ 并下载安装程序
```

<div align="center">

## 安装 DMS

</div>

### 在新项目中使用 DMS

创建一个新的 Rust 项目：

```bash
cargo new my-dms-app
cd my-dms-app
```

将 DMS 添加到您项目的 `Cargo.toml` 文件中：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dms" }
tokio = { version = "1.0", features = ["full"] }
```

或者使用 `cargo add` 命令：

```bash
cargo add dms --git https://gitee.com/dunimd/dms
cargo add tokio --features full
```

### 在现有项目中使用 DMS

直接将 DMS 添加到现有项目的 `Cargo.toml` 文件中：

```toml
[dependencies]
# 其他依赖
dms = { git = "https://gitee.com/dunimd/dms" }
```

<div align="center">

## 第一个 DMS 应用

</div>

现在，让我们创建一个简单的 DMS 应用程序。

### 基础应用结构

打开 `src/main.rs` 文件，替换为以下内容：

```rust
use dms::prelude::*;

#[tokio::main]
async fn main() -> DMSResult<()> {
    // 构建服务运行时
    let app = DMSAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSLogConfig::default())?
        .with_observability(DMSObservabilityConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSServiceContext| async move {
        ctx.logger().info("service", "DMS service started")?;
        // 您的业务代码在这里
        Ok(())
    }).await
}
```

### 配置文件

在项目根目录创建 `config.yaml` 文件：

```yaml
# config.yaml
service:
  name: "my-dms-app"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### 运行应用

使用 Cargo 运行应用：

```bash
cargo run
```

您应该会看到类似以下的输出：

```
2025-12-12T15:30:00Z INFO service: DMS service started
```
<div align="center">

## 应用结构解析

</div>  

让我们解析一下这个简单的 DMS 应用：

1. **导入 DMS 组件**：
   ```rust
   use dms::prelude::*;
   ```
   这行代码导入了 DMS 中最常用的类型和特性，简化了代码编写。

2. **创建应用构建器**：
   ```rust
   let app = DMSAppBuilder::new()
   ```
   使用构建器模式创建 DMS 应用实例。

3. **配置应用**：
   ```rust
   .with_config("config.yaml")?
   .with_logging(DMSLogConfig::default())?
   .with_observability(DMSObservabilityConfig::default())?
   ```
   - 添加配置文件支持
   - 启用日志功能
   - 启用可观测性（指标和追踪）

4. **构建应用**：
   ```rust
   .build()?
   ```
   构建最终的应用实例。

5. **运行应用**：
   ```rust
   app.run(|ctx: &DMSServiceContext| async move {
       ctx.logger().info("service", "DMS service started")?;
       Ok(())
   }).await
   ```
   - 使用 `run` 方法启动应用
   - 传入一个闭包，该闭包接收 `DMSServiceContext` 实例
   - 在闭包中编写业务逻辑

<div align="center">

## 添加更多功能

</div>  

### 添加缓存支持

修改 `Cargo.toml` 添加 Redis 依赖（如果需要使用 Redis 缓存）：

```toml
[dependencies]
# 其他依赖
redis = "0.23"
```

修改应用代码，添加缓存支持：

```rust
use dms::prelude::*;

#[tokio::main]
async fn main() -> DMSResult<()> {
    let app = DMSAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSLogConfig::default())?
        .with_observability(DMSObservabilityConfig::default())?
        .with_cache(DMSCacheConfig::default())? // 添加缓存支持
        .build()?;
    
    app.run(|ctx: &DMSServiceContext| async move {
        ctx.logger().info("service", "DMS service started")?;
        
        // 使用缓存
        let cache = ctx.cache();
        cache.set("key", "value", Some(3600)).await?;
        let value = cache.get("key").await?;
        ctx.logger().info("cache", &format!("Cache value: {:?}", value))?;
        
        Ok(())
    }).await
}
```

<div align="center">

## 运行测试

</div>  

DMS 提供了完整的测试套件，您可以运行这些测试来验证安装：

```bash
# 克隆 DMS 仓库
git clone https://gitee.com/dunimd/dms.git
cd dms

# 运行所有测试
cargo test

# 运行特定测试模块
cargo test cache

# 带详细输出运行
cargo test -- --nocapture
```
<div align="center">

## 常见问题

</div>

### Q: 如何配置日志级别？
A: 在配置文件中设置 `logging.level`，支持 DEBUG/INFO/WARN/ERROR 级别。

### Q: 如何启用指标导出？
A: 在配置文件中设置 `observability.metrics_enabled: true` 并配置 `prometheus_port`。

### Q: 如何扩展 DMS？
A: 实现 `DMSModule` trait 并通过 `DMSAppBuilder::with_module` 注册。

### Q: 如何处理异步任务？
A: 使用 `DMSAppBuilder::with_async_module` 添加异步模块，框架自动处理异步生命周期。

<div align="center">

## 故障排除

</div>  

- **编译错误**：确保 Rust 版本符合要求，检查依赖版本兼容性。
- **运行时错误**：检查配置文件路径和内容，查看日志输出获取详细信息。
- **依赖冲突**：使用 `cargo tree` 命令查看依赖树，解决版本冲突。

<div align="center">

## 下一步

</div>  

- [核心概念](./03-core-concepts.md)：深入理解 DMS 的设计理念和核心组件
- [API 参考](./04-api-reference/README.md)：详细的模块 API 文档
- [使用示例](./05-usage-examples/README.md)：各种场景下的使用示例
- [最佳实践](./06-best-practices.md)：开发 DMS 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：常见问题和解决方案
- [术语表](./08-glossary.md)：核心术语解释