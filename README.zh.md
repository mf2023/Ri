<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 8px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="assets/svg/dmsc.svg" width="36" height="36" alt="DMSC">Dunimd Middleware Service</span>
</h1>

[English](README.md) | 简体中文

[帮助文档](https://mf2023.github.io/DMSC/dmsc/) | [更新日志](CHANGELOG.md) | [安全](SECURITY.md) | [贡献](CONTRIBUTING.md) | [行为准则](CODE_OF_CONDUCT.md)


<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>
<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>


<a href="https://github.com/mf2023/DMSC" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMSC-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://gitcode.com/dunimd/dmsc.git" target="_blank">
    <img alt="GitCode" src="https://img.shields.io/badge/GitCode-DMSC-FF6B35?style=flat-square&logo=git"/>
</a>
<a href="https://huggingface.co/dunimd" target="_blank">
    <img alt="Hugging Face" src="https://img.shields.io/badge/Hugging%20Face-Dunimd-FFD21E?style=flat-square&logo=huggingface"/>
</a>
<a href="https://modelscope.cn/organization/dunimd" target="_blank">
    <img alt="ModelScope" src="https://img.shields.io/badge/ModelScope-Dunimd-1E6CFF?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTcuMDA2IDBDMy4xNDIgMCAwIDMuMTQyIDAgNy4wMDZTMy4xNDIgMTQuMDEyIDcuMDA2IDE0LjAxMkMxMC44NyAxNC4wMTIgMTQuMDEyIDEwLjg3IDE0LjAxMiA3LjAwNkMxNC4wMTIgMy4xNDIgMTAuODcgMCA3LjAwNiAwWiIgZmlsbD0iIzFFNkNGRiIvPgo8L3N2Zz4K"/>
</a>


<a href="https://crates.io/crates/dmsc" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMSC-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>
<a href="https://docs.rs/dmsc/latest/dmsc/c/index.html" target="_blank">
    <img alt="C/C++" src="https://img.shields.io/badge/C%2FC%2B%2B-DMSC-00599C?style=flat-square&logo=c"/>
</a>
<a href="https://search.maven.org/artifact/com.dunimd/dmsc" target="_blank">
    <img alt="Maven Central" src="https://img.shields.io/badge/Maven-DMSC-007396?style=flat-square&logo=apachemaven"/>
</a>

**DMSC (Dunimd Middleware Service)** — 一个高性能的 Rust 中间件框架，统一后端基础设施。专为企业级规模构建，具有模块化架构、内置可观测性和分布式系统支持。

</div>

<h2 align="center">🏗️ 核心架构</h2>

### 📐 模块化设计
DMSC 采用高度模块化的架构，拥有 18 个核心模块，支持按需组合和无缝扩展：

<div align="center">

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **database** | 数据库抽象层，支持 PostgreSQL、MySQL、SQLite |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **grpc** | gRPC 服务器和客户端支持，带 Python 绑定（需要 `grpc` 特性） |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **module_rpc** | 模块间 RPC 通信，支持分布式方法调用 |
| **observability** | 指标、追踪和 Grafana 集成 |
| **database.orm** | 类型安全的 ORM，带有仓储模式、查询构建器和 Python 绑定 |
| **protocol** | 协议抽象层，支持多协议（需要 `pyo3` 特性） |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |
| **validation** | 数据验证和清理工具 |
| **ws** | WebSocket 服务器支持，带 Python 绑定（需要 `websocket` 特性） |
| **c** | C/C++ FFI 绑定，用于跨语言集成（需要 `c` 特性） |
| **java** | Java JNI 绑定，用于 Java 应用集成（需要 `java` 特性） |

</div>

> **注意**：部分模块需要特定特性标志：
> - `grpc`: gRPC 支持（`--features grpc`）
> - `websocket`: WebSocket 支持（`--features websocket`）
> - `protocol`: 协议抽象层（`--features protocol` 或 `full`）
> - `c`: C/C++ FFI 绑定（`--features c`）
> - `java`: Java JNI 绑定（`--features java`）

### 🚀 核心特性

#### 🔍 分布式追踪
- W3C 追踪上下文标准实现
- 全链路 TraceID/SpanID 传播
- 业务上下文的数据传输
- 多语言兼容性（Java、Go、Python）
- 通过 `#[tracing::instrument]` 属性自动创建 span

#### 📊 企业级可观测性
- 原生 Prometheus 指标导出
- Counter、Gauge、Histogram、Summary 指标类型
- 开箱即用的 Grafana 仪表板集成
- 实时性能统计与分位数计算
- 全栈指标（CPU、内存、I/O、网络）

#### 🤖 智能设备管理
- 自动发现和注册
- 高效的资源池管理
- 基于策略的调度与优先级支持
- 动态负载均衡
- 完整的设备生命周期管理

#### 📝 结构化日志
- 支持 JSON 和文本格式
- 可配置的采样率
- 智能日志轮转
- 自动包含追踪上下文
- DEBUG/INFO/WARN/ERROR 日志级别

#### ⚙️ 灵活配置
- 多源加载（文件、环境变量、运行时）
- 热配置更新
- 模块化架构支持按需组合
- 基于插件的扩展机制

#### 📁 安全文件系统
- 统一项目根目录管理
- 原子文件操作
- 分类目录结构
- JSON 数据持久化
- 安全路径处理

<h2 align="center">🛠️ 安装与环境</h2>

### 前置要求
- **Rust**: 1.65+ (2021 版本)
- **Cargo**: 1.65+
- **平台**: Linux (x64, arm64)、macOS (x64, arm64)、Windows (x64, arm64)、Android (arm64-v8a, armeabi-v7a, x86_64)

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

### 快速设置

将 DMSC 添加到您项目的 `Cargo.toml`：

```toml
[dependencies]
dmsc = "0.1.8"
```

或者使用 cargo add：

```bash
cargo add dmsc
```

<h2 align="center">⚡ 快速开始</h2>

### 核心 API 使用

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())
        .with_observability(DMSCObservabilityConfig::default())
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        // 您的业务代码在这里
        Ok(())
    }).await
}
```

### 可观测性示例

```rust
use dmsc::prelude::*;
use dmsc::observability::{DMSCTracer, DMSCSpanKind, DMSCSpanStatus};

#[tracing::instrument(name = "user_service", skip(ctx))]
async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

或者直接使用 DMSCTracer：

```rust
use dmsc::prelude::*;
use dmsc::observability::DMSCTracer;

async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    let tracer = DMSCTracer::new(1.0);
    let _span = tracer.span("get_user")
        .with_attribute("user_id", user_id.to_string())
        .start();
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

<h2 align="center">🔧 配置</h2>

### 配置示例

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"

logging:
  level: "info"
  file_format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090

resource:
  providers: ["cpu", "gpu", "memory"]
  scheduling_policy: "priority_based"
```

### 配置源

DMSC 支持多种配置源，按优先级排序（从低到高）：
1. 配置文件（YAML、TOML、JSON）
2. 自定义配置（通过代码设置）
3. 环境变量（以 `DMSC_` 为前缀）

<h2 align="center">🧪 开发与测试</h2>

### 运行测试

#### 多语言测试

DMSC 为所有支持的语言提供全面的测试：

- **Rust**：使用 `cargo test` 的核心库测试
- **Python**：使用 `pytest` 的 Python 绑定测试
- **Java**：使用标准 Java 测试运行器的 JNI 绑定测试
- **C/C++**：使用原生编译器的 C API 测试

#### 测试覆盖

测试验证：
- ✅ 所有语言的构建器模式行为
- ✅ 方法链式调用返回适当的实例（语言特定）
- ✅ 运行时创建和生命周期管理
- ✅ 错误处理和边界情况
- ✅ 跨语言 API 一致性

#### 运行 Rust 测试

```bash
# 运行所有 Rust 测试
cargo test

# 运行特定测试模块
cargo test --lib app_builder
cargo test --lib app_runtime

# 带详细输出运行
cargo test -- --nocapture

# 使用所有特性运行
cargo test --all-features
```

#### 运行 Python 测试

```bash
# 以开发模式安装 Python 包
cd python
pip install -e .

# 运行所有 Python 测试
python -m pytest tests/Python/ -v

# 运行特定测试类
python -m pytest tests/Python/test_core.py::TestDMSCAppBuilderWrapper -v
python -m pytest tests/Python/test_core.py::TestDMSCAppRuntimeWrapper -v
```

#### 运行 Java 测试

```bash
# 构建 JNI 库
cargo build --release --no-default-features --features java

# 编译并运行 Java 测试
cd java
javac -d build/classes/java/test -cp build/classes/java/main \
  src/test/java/TestAll.java src/test/java/TestAppBuilder.java

java -cp build/classes/java/test:build/classes/java/main \
  -Djava.library.path=../target/release TestAll
```

#### 跨语言 API 行为

| 语言 | 构建器模式 | 方法链式调用 | 原因 |
|------|-----------|-------------|------|
| **Rust** | 返回 `Self` | 消费原对象 | 原生构建器模式 |
| **Python** | 返回 `self` | 同一个实例 | PyO3 的 Pythonic 包装器 |
| **Java** | 返回新实例 | 不可变构建器 | Java 最佳实践 |
| **C** | 返回新指针 | 内存管理 | C 语言习惯 |

<h2 align="center">❓ 常见问题</h2>

**Q: 如何添加新模块？**
A: 实现 `DMSCModule` trait 并通过 `DMSCAppBuilder::with_module` 注册。

**Q: 如何配置日志级别？**
A: 在配置文件中设置 `logging.level`，支持 DEBUG/INFO/WARN/ERROR 级别。

**Q: 如何启用指标导出？**
A: 在配置文件中设置 `observability.metrics_enabled: true` 并配置 `prometheus_port`。

**Q: 如何扩展配置源？**
A: 实现自定义配置加载器并用 `DMSCConfigManager` 注册。

**Q: 如何处理异步任务？**
A: 使用 `DMSCAppBuilder::with_async_module` 添加异步模块，框架自动处理异步生命周期。

<h2 align="center">🌏 社区与引用</h2>

- 欢迎提交 Issues 和 PRs！
- Github: https://github.com/mf2023/DMSC.git
- Gitee: https://gitee.com/dunimd/dmsc.git
- GitCode: https://gitcode.com/dunimd/dmsc.git


<div align="center">

## 📄 许可证与开源协议

### 🏛️ 项目许可证

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

本项目使用 **Apache License 2.0** 开源协议，详见 [LICENSE](LICENSE) 文件。

### 📋 依赖包开源协议

本项目使用的开源包及其协议信息：

### 依赖许可证

<div align="center">

| 📦 包 | 📜 许可证 | 📦 包 | 📜 许可证 |
|:------|:---------|:------|:---------|
| serde | MIT/Apache-2.0 | serde_json | MIT/Apache-2.0 |
| serde_yaml | MIT/Apache-2.0 | tokio | MIT |
| futures | MIT/Apache-2.0 | futures-util | MIT/Apache-2.0 |
| http | MIT/Apache-2.0 | hyper | MIT |
| prometheus | MIT/Apache-2.0 | redis | MIT |
| lapin | MIT/Apache-2.0 | rdkafka | MIT |
| yaml-rust | MIT/Apache-2.0 | toml | MIT/Apache-2.0 |
| etcd-client | MIT | sysinfo | MIT |
| async-trait | MIT/Apache-2.0 | dashmap | MIT |
| chrono | MIT | uuid | MIT/Apache-2.0 |
| rand | MIT/Apache-2.0 | notify | CC0-1.0 |
| jsonwebtoken | MIT | reqwest | MIT/Apache-2.0 |
| urlencoding | MIT | parking_lot | MIT/Apache-2.0 |
| log | MIT/Apache-2.0 | tracing | MIT |
| pyo3 | MIT/Apache-2.0 | jni | MIT/Apache-2.0 |
| safer-ffi | MIT | tempfile | MIT/Apache-2.0 |
| thiserror | MIT/Apache-2.0 | hex | MIT/Apache-2.0 |
| base64 | MIT/Apache-2.0 | regex | MIT/Apache-2.0 |
| url | MIT/Apache-2.0 | aes-gcm | MIT/Apache-2.0 |
| ring | ISC | lazy_static | MIT/Apache-2.0 |
| libloading | ISC | zeroize | MIT/Apache-2.0 |
| zeroize_derive | MIT/Apache-2.0 | secrecy | MIT |
| erased-serde | MIT | data-encoding | MIT |
| crc32fast | MIT/Apache-2.0 | generic-array | MIT |
| bincode | MIT | typenum | MIT/Apache-2.0 |
| html-escape | MIT | rustls | MIT/Apache-2.0 |
| rustls-pemfile | MIT/Apache-2.0 | webpki | ISC |
| rustls-native-certs | MIT/Apache-2.0 | tokio-rustls | MIT/Apache-2.0 |
| bytes | MIT | tonic | MIT |
| prost | MIT/Apache-2.0 | prost-types | MIT/Apache-2.0 |
| tokio-stream | MIT | tower | MIT |
| async-stream | MIT | tokio-tungstenite | MIT |
| tungstenite | MIT | num-bigint | MIT/Apache-2.0 |
| oqs | MIT/Apache-2.0 | sm-crypto | MIT |
| openssl-sys | Apache-2.0 | tokio-postgres | MIT/Apache-2.0 |
| rusqlite | MIT | sqlx | MIT/Apache-2.0 |
| criterion | MIT/Apache-2.0 | | |

</div>

</div>