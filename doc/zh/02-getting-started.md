<div align="center">

# 快速开始

**Version: 0.1.7**

**Last modified date: 2026-02-17**

本指南将帮助您快速上手DMSC，从安装到创建第一个应用。

## 前置要求

</div>

在开始之前，确保您的环境满足以下要求：

- **Rust**: 1.65+ (2021 版本)
- **Cargo**: 1.65+ (Rust 包管理器)
- **平台**: Linux、macOS 或 Windows

### 构建依赖

某些功能需要额外的系统依赖：

| 依赖 | 用于 | 安装 |
|:-----------|:-------------|:-------------|
| **protoc** | etcd 功能 (Protocol Buffers) | [Protocol Buffers](https://protobuf.dev/downloads/) |
| **CMake + C++ 编译器** | kafka 功能 (rdkafka) | 见下方说明 |

#### 安装 protoc

**Windows:**
```powershell
# 使用 chocolatey
choco install protoc

# 或从 GitHub releases 下载
# https://github.com/protocolbuffers/protobuf/releases
```

**macOS:**
```bash
brew install protobuf
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install -y protobuf-compiler
```

#### 安装 CMake 和 C++ 编译器（用于 Kafka 支持）

**Windows:**
```powershell
# CMake 通常随 Visual Studio 安装
# 或从官网下载: https://cmake.org/download/

# 使用 chocolatey
choco install cmake
```

**macOS:**
```bash
# CMake 和 C++ 编译器 (Xcode Command Line Tools)
xcode-select --install

# 或使用 Homebrew
brew install cmake
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y cmake build-essential
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install -y cmake gcc-c++ make
```

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

### Python 环境要求

如果您使用 Python SDK，需满足以下要求：

- **Python**: 3.8+
- **pip**: 最新版本
- **平台**: Linux、macOS 或 Windows

您可以使用以下命令检查 Python 版本：

```bash
python --version
# 或
python3 --version
```

如果您还没有安装 Python 3.8+，请访问 [Python 官网](https://www.python.org/downloads/) 下载安装。

<div align="center">

## 安装 DMSC

</div>

### 在新项目中使用 DMSC

创建一个新的 Rust 项目：

```bash
cargo new my-dms-app
cd my-dms-app
```

将 DMSC 添加到您项目的 `Cargo.toml` 文件中：

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
```

或者使用 `cargo add` 命令：

```bash
cargo add dmsc --git https://github.com/mf2023/DMSC
cargo add tokio --features full
```

### 在现有项目中使用 DMSC

直接将 DMSC 添加到现有项目的 `Cargo.toml` 文件中：

```toml
[dependencies]
# 其他依赖
dmsc = { git = "https://github.com/mf2023/DMSC" }
```

### 使用 Python SDK

安装 Python SDK 最简单的方式是通过 pip：

```bash
pip install dmsc
```

或者添加到 `requirements.txt`：

```
dmsc==0.1.7
```

验证安装：

```python
import dmsc
print(f"DMSC Python SDK 版本: {dmsc.__version__}")
```

<div align="center">

## 第一个 DMSC 应用

</div>

现在，让我们创建一个简单的 DMSC 应用程序。

### 基础应用结构

打开 `src/main.rs` 文件，替换为以下内容：

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
2025-12-12T15:30:00Z INFO service: DMSC service started
```

### 第一个 Python 应用

使用 Python SDK 创建应用同样简单：

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

# 构建服务运行时
app = DMSCAppBuilder() \
    .with_config("config.yaml") \
    .with_logging(DMSCLogConfig()) \
    .build()

# 运行业务逻辑
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

### 配置文件

在项目根目录创建 `config.yaml` 文件（Rust 和 Python 共用）：

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

### 运行 Python 应用

```bash
python main.py
```

您应该会看到类似以下的输出：

```
2025-12-12T15:30:00Z INFO service: DMSC service started
```

<div align="center">

## 应用结构解析

</div>  

让我们解析一下这个简单的 DMSC 应用：

1. **导入 DMSC 组件**：
   ```rust
   use dmsc::prelude::*;
   ```
   这行代码导入了 DMSC 中最常用的类型和特性，简化了代码编写。

2. **创建应用构建器**：
   ```rust
   let app = DMSCAppBuilder::new()
   ```
   使用构建器模式创建 DMSC 应用实例。

3. **配置应用**：
   ```rust
   .with_config("config.yaml")?
   .with_logging(DMSCLogConfig::default())?
   .with_observability(DMSCObservabilityConfig::default())?
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
   app.run(|ctx: &DMSCServiceContext| async move {
       ctx.logger().info("service", "DMSC service started")?;
       Ok(())
   }).await
   ```
   - 使用 `run` 方法启动应用
   - 传入一个闭包，该闭包接收 `DMSCServiceContext` 实例
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
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .with_cache(DMSCCacheConfig::default())? // 添加缓存支持
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        
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

DMSC 提供了完整的测试套件，您可以运行这些测试来验证安装：

```bash
# 克隆 DMSC 仓库
git clone https://github.com/mf2023/DMSC.git
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

### Q: 如何扩展 DMSC？
A: 实现 `DMSCModule` trait 并通过 `DMSCAppBuilder::with_module` 注册。

### Q: 如何处理异步任务？
A: 使用 `DMSCAppBuilder::with_async_module` 添加异步模块，框架自动处理异步生命周期。

<div align="center">

## 故障排除

</div>  

- **编译错误**：确保 Rust 版本符合要求，检查依赖版本兼容性。
- **运行时错误**：检查配置文件路径和内容，查看日志输出获取详细信息。
- **依赖冲突**：使用 `cargo tree` 命令查看依赖树，解决版本冲突。

<div align="center">

## 下一步

</div>  

- [核心概念](./03-core-concepts.md)：深入理解 DMSC 的设计理念和核心组件
- [API 参考](./04-api-reference/README.md)：详细的模块 API 文档
- [使用示例](./05-usage-examples/README.md)：各种场景下的使用示例
- [最佳实践](./06-best-practices.md)：开发 DMSC 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：常见问题和解决方案
- [术语表](./08-glossary.md)：核心术语解释