<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 8px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="assets/svg/dmsc.svg" width="36" height="36" alt="DMSC">Dunimd Middleware Service</span>
</h1>

English | [简体中文](README.zh.md)

[Help Documentation](https://mf2023.github.io/dmsc/dmsc/) | [Changelog](CHANGELOG.md) | [Security](SECURITY.md) | [Contributing](CONTRIBUTING.md) | [Code of Conduct](CODE_OF_CONDUCT.md)

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>

<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://github.com/mf2023/DMSC" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMSC-181717?style=flat-square&logo=github"/>
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
    <img alt="C/C++" src="https://img.shields.io/badge/C%2FC%2B%2B-API-00599C?style=flat-square&logo=c"/>
</a>
<a href="https://search.maven.org/artifact/com.dunimd/dmsc" target="_blank">
    <img alt="Maven Central" src="https://img.shields.io/badge/Maven-DMSC-007396?style=flat-square&logo=apachemaven"/>
</a>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework that unifies backend infrastructure. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
DMSC adopts a highly modular architecture with 18 core modules, enabling on-demand composition and seamless extension:

<div align="center">

| Module | Description |
|:--------|:-------------|
| **auth** | Authentication & authorization (JWT, OAuth, permissions) |
| **cache** | Multi-backend cache abstraction (Memory, Redis, Hybrid) |
| **config** | Multi-source configuration management with hot reload |
| **core** | Runtime, error handling, and service context |
| **database** | Database abstraction with PostgreSQL, MySQL, SQLite support |
| **device** | Device control, discovery, and intelligent scheduling |
| **fs** | Secure file system operations and management |
| **gateway** | API gateway with load balancing, rate limiting, and circuit breaking |
| **grpc** | gRPC server and client support with Python bindings (requires `grpc` feature) |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) |
| **log** | Structured logging with tracing context integration |
| **module_rpc** | Inter-module RPC communication for distributed method calls |
| **observability** | Metrics, tracing, and Grafana integration |
| **database.orm** | Type-safe ORM with repository pattern, query builder, and Python bindings |
| **protocol** | Protocol abstraction layer for multi-protocol support (requires `pyo3` feature) |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| **service_mesh** | Service discovery, health checking, and traffic management |
| **validation** | Input validation and data sanitization utilities |
| **ws** | WebSocket server support with Python bindings (requires `websocket` feature) |
| **c** | C/C++ FFI bindings for cross-language integration (requires `c` feature) |
| **java** | Java JNI bindings for Java application integration (requires `java` feature) |

</div>

> **Note**: Some modules require specific feature flags:
> - `grpc`: gRPC support (`--features grpc`)
> - `websocket`: WebSocket support (`--features websocket`)
> - `protocol`: Protocol abstraction layer (`--features protocol` or `full`)
> - `c`: C/C++ FFI bindings (`--features c`)
> - `java`: Java JNI bindings (`--features java`)

### 🚀 Key Features

#### 🔍 Distributed Tracing
- W3C Trace Context standard implementation
- Full-chain TraceID/SpanID propagation
- Baggage data transmission for business context
- Multi-language compatibility (Java, Go, Python)
- Automatic span creation via `#[tracing::instrument]` attribute

#### 📊 Enterprise Observability
- Native Prometheus metrics export
- Counter, Gauge, Histogram, Summary metric types
- Out-of-the-box Grafana dashboard integration
- Real-time performance statistics with quantile calculation
- Full-stack metrics (CPU, memory, I/O, network)

#### 🤖 Intelligent Device Management
- Auto-discovery and registration
- Efficient resource pool management
- Policy-based scheduling with priority support
- Dynamic load balancing
- Complete device lifecycle management

#### 📝 Structured Logging
- JSON and text format support
- Configurable sampling rates
- Intelligent log rotation
- Automatic tracing context inclusion
- DEBUG/INFO/WARN/ERROR log levels

#### ⚙️ Flexible Configuration
- Multi-source loading (files, environment variables, runtime)
- Hot configuration updates
- Modular architecture for on-demand composition
- Plugin-based extension mechanism

#### 📁 Secure File System
- Unified project root directory management
- Atomic file operations
- Categorized directory structure
- JSON data persistence
- Secure path handling

<h2 align="center">🛠️ Installation & Environment</h2>

### Prerequisites
- **Rust**: 1.65+ (2021 Edition)
- **Cargo**: 1.65+
- **Platforms**: Linux (x64, arm64), macOS (x64, arm64), Windows (x64, arm64), Android (arm64-v8a, armeabi-v7a, x86_64)

### Build Dependencies

Some features require additional system dependencies:

| Dependency | Required For | Installation |
|:-----------|:-------------|:-------------|
| **protoc** | etcd feature (Protocol Buffers) | [Protocol Buffers](https://protobuf.dev/downloads/) |
| **CMake + C++ compiler** | kafka feature (rdkafka) | See instructions below |

#### Installing protoc

**Windows:**
```powershell
# Using chocolatey
choco install protoc

# Or download from GitHub releases
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

#### Installing CMake and C++ compiler (for Kafka support)

**Windows:**
```powershell
# CMake is usually installed with Visual Studio
# Or download from: https://cmake.org/download/

# Using chocolatey
choco install cmake
```

**macOS:**
```bash
# CMake and C++ compiler (Xcode Command Line Tools)
xcode-select --install

# Or using Homebrew
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

### Quick Setup

Add DMSC to your project's `Cargo.toml`:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
```

Or use cargo add:

```bash
cargo add dmsc --git https://github.com/mf2023/DMSC
```

<h2 align="center">⚡ Quick Start</h2>

### Core API Usage

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())
        .with_observability(DMSCObservabilityConfig::default())
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        // Your business code here
        Ok(())
    }).await
}
```

### Observability Example

```rust
use dmsc::prelude::*;
use dmsc::observability::{DMSCTracer, DMSCSpanKind, DMSCSpanStatus};

#[tracing::instrument(name = "user_service", skip(ctx))]
async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

Or using DMSCTracer directly:

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

<h2 align="center">🔧 Configuration</h2>

### Configuration Example

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

### Configuration Sources

DMSC supports multiple configuration sources in order of priority (lowest to highest):
1. Configuration files (YAML, TOML, JSON)
2. Custom configuration via code
3. Environment variables (prefixed with `DMSC_`)

<h2 align="center">🧪 Development & Testing</h2>

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test cache

# Run with verbose output
cargo test -- --nocapture
```

<h2 align="center">❓ Frequently Asked Questions</h2>

**Q: How to add a new module?**
A: Implement the `DMSCModule` trait and register it via `DMSCAppBuilder::with_module`.

**Q: How to configure logging level?**
A: Set `logging.level` in the configuration file, supporting DEBUG/INFO/WARN/ERROR levels.

**Q: How to enable metrics export?**
A: Set `observability.metrics_enabled: true` and configure `prometheus_port` in the configuration file.

**Q: How to extend configuration sources?**
A: Implement a custom configuration loader and register it with `DMSCConfigManager`.

**Q: How to handle asynchronous tasks?**
A: Use `DMSCAppBuilder::with_async_module` to add async modules, the framework handles async lifecycle automatically.

<h2 align="center">🌏 Community & Citation</h2>

- Welcome to submit Issues and PRs!
- Gitee: https://gitee.com/dunimd/dmsc.git
- Github: https://github.com/mf2023/DMSC.git


<div align="center">

## 📄 License & Open Source Agreements

### 🏛️ Project License

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

This project uses **Apache License 2.0** open source agreement, see [LICENSE](LICENSE) file.

### 📋 Dependency Package Open Source Agreements

Open source packages and their agreement information used by this project:

### Dependencies License

<div align="center">

| 📦 Package | 📜 License |
|:-----------|:-----------|
| serde | Apache 2.0 |
| serde_json | MIT |
| serde_yaml | MIT |
| tokio | MIT |
| prometheus | Apache 2.0 |
| redis | MIT |
| hyper | MIT |
| lapin | Apache 2.0 |
| futures | MIT |
| yaml-rust | MIT |
| toml | MIT |
| etcd-client | MIT |
| sysinfo | MIT |
| async-trait | MIT |
| dashmap | MIT |
| chrono | MIT |
| uuid | Apache 2.0 |
| rand | MIT |
| notify | MIT |
| jsonwebtoken | MIT |
| reqwest | MIT |
| urlencoding | MIT |
| parking_lot | MIT |
| log | MIT |
| pyo3 | Apache 2.0 |
| jni | Apache 2.0 |
| tempfile | MIT |
| tracing | MIT |
| thiserror | MIT |
| hex | MIT |
| base64 | MIT |
| regex | MIT |
| url | Apache 2.0 |
| aes-gcm | Apache 2.0 |
| ring | Apache 2.0 |
| lazy_static | MIT |
| libloading | MIT |
| zeroize | MIT/Apache-2.0 |
| secrecy | MIT |
| data-encoding | MIT |
| crc32fast | MIT |
| generic-array | MIT |
| bincode | MIT |
| typenum | MIT |
| html-escape | MIT |
| rustls | Apache 2.0/MIT |
| rustls-pemfile | Apache 2.0/MIT |
| webpki | ISC |
| rustls-native-certs | Apache 2.0/MIT |
| bytes | Apache 2.0 |
| tonic | MIT |
| prost | Apache 2.0 |
| tokio-stream | MIT |
| tower | MIT |
| async-stream | MIT |
| tokio-tungstenite | MIT |
| tungstenite | MPL-2.0 |
| num-bigint | MIT/Apache-2.0 |
| oqs | MIT/Apache-2.0 |

</div>

</div>