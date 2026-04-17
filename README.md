> **⚠️ IMPORTANT NOTICE**
>
> **DMSC → RI** starting from version **0.1.9** | CLI tools included | [📖 Read Migration Guide](./ANNOUNCEMENT.md)
>
> All `DMSC*` types → `RI*` | Package: `dmsc` → `ri`

---

<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 8px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="assets/svg/ri.svg" width="36" height="36" alt="Ri">Ri</span>
</h1>

English | [简体中文](README.zh.md)

[Help Documentation](https://mf2023.github.io/Ri/ri/) | [Changelog](CHANGELOG.md) | [Security](SECURITY.md) | [Contributing](CONTRIBUTING.md) | [Code of Conduct](CODE_OF_CONDUCT.md)

<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>
<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>


<a href="https://github.com/mf2023/Ri" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-Ri-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://gitcode.com/dunimd/ri.git" target="_blank">
    <img alt="GitCode" src="https://img.shields.io/badge/GitCode-Ri-FF6B35?style=flat-square&logo=git"/>
</a>
<a href="https://huggingface.co/dunimd" target="_blank">
    <img alt="Hugging Face" src="https://img.shields.io/badge/Hugging%20Face-Dunimd-FFD21E?style=flat-square&logo=huggingface"/>
</a>
<a href="https://modelscope.cn/organization/dunimd" target="_blank">
    <img alt="ModelScope" src="https://img.shields.io/badge/ModelScope-Dunimd-1E6CFF?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTcuMDA2IDBDMy4xNDIgMCAwIDMuMTQyIDAgNy4wMDZTMy4xNDIgMTQuMDEyIDcuMDA2IDE0LjAxMkMxMC44NyAxNC4wMTIgMTQuMDEyIDEwLjg3IDE0LjAxMiA3LjAwNkMxNC4wMTIgMy4xNDIgMTAuODcgMCA3LjAwNiAwWiIgZmlsbD0iIzFFNkNGRiIvPgo8L3N2Zz4K"/>
</a>


<a href="https://crates.io/crates/ri" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-Ri-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/ri/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-Ri-3775A9?style=flat-square&logo=pypi"/>
</a>
<a href="https://docs.rs/ri/latest/ri/c/index.html" target="_blank">
    <img alt="C/C++" src="https://img.shields.io/badge/C%2FC%2B%2B-Ri-00599C?style=flat-square&logo=c"/>
</a>
<a href="https://search.maven.org/artifact/com.dunimd/ri" target="_blank">
    <img alt="Maven Central" src="https://img.shields.io/badge/Maven-Ri-007396?style=flat-square&logo=apachemaven"/>
</a>

**Ri (Ri)** — A high-performance Rust middleware framework that unifies backend infrastructure. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
Ri adopts a highly modular architecture with 18 core modules plus optional extension modules, enabling on-demand composition and seamless extension:

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

Add Ri to your project's `Cargo.toml`:

```toml
[dependencies]
ri = "0.1.9"
```

Or use cargo add:

```bash
cargo add ri
```

<h2 align="center">⚡ Quick Start</h2>

### Core API Usage

```rust
use ri::prelude::*;

#[tokio::main]
async fn main() -> RiResult<()> {
    // Build service runtime
    let app = RiAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(RiLogConfig::default())
        .with_observability(RiObservabilityConfig::default())
        .build()?;
    
    // Run business logic
    app.run(|ctx: &RiServiceContext| async move {
        ctx.logger().info("service", "Ri service started")?;
        // Your business code here
        Ok(())
    }).await
}
```

### Observability Example

```rust
use ri::prelude::*;
use ri::observability::{RiTracer, RiSpanKind, RiSpanStatus};

#[tracing::instrument(name = "user_service", skip(ctx))]
async fn get_user(ctx: &RiServiceContext, user_id: u64) -> RiResult<User> {
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

Or using RiTracer directly:

```rust
use ri::prelude::*;
use ri::observability::RiTracer;

async fn get_user(ctx: &RiServiceContext, user_id: u64) -> RiResult<User> {
    let tracer = RiTracer::new(1.0);
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

Ri supports multiple configuration sources in order of priority (lowest to highest):
1. Configuration files (YAML, TOML, JSON)
2. Custom configuration via code
3. Environment variables (prefixed with `Ri_`)

<h2 align="center">🧪 Development & Testing</h2>

### Running Tests

#### Multi-Language Testing

Ri provides comprehensive testing across all supported languages:

- **Rust**: Core library tests with `cargo test`
- **Python**: Python binding tests with `pytest`
- **Java**: JNI binding tests with standard Java test runner
- **C/C++**: C API tests with native compilers

#### Test Coverage

The tests verify:
- ✅ Builder pattern behavior in all languages
- ✅ Method chaining returns appropriate instances (language-specific)
- ✅ Runtime creation and lifecycle management
- ✅ Error handling and edge cases
- ✅ Cross-language API consistency

#### Running Rust Tests

```bash
# Run all Rust tests
cargo test

# Run specific test modules
cargo test --lib app_builder
cargo test --lib app_runtime

# Run with verbose output
cargo test -- --nocapture

# Run with all features
cargo test --all-features
```

#### Running Python Tests

```bash
# Install Python package in development mode
cd python
pip install -e .

# Run all Python tests
python -m pytest tests/Python/ -v

# Run specific test classes
python -m pytest tests/Python/test_core.py::TestRiAppBuilderWrapper -v
python -m pytest tests/Python/test_core.py::TestRiAppRuntimeWrapper -v
```

#### Running Java Tests

```bash
# Build JNI library
cargo build --release --no-default-features --features java

# Compile and run Java tests
cd java
javac -d build/classes/java/test -cp build/classes/java/main \
  src/test/java/TestAll.java src/test/java/TestAppBuilder.java

java -cp build/classes/java/test:build/classes/java/main \
  -Djava.library.path=../target/release TestAll
```

#### API Behavior Across Languages

| Language | Builder Pattern | Method Chaining | Reason |
|----------|----------------|-----------------|--------|
| **Rust** | Returns `Self` | Consumes original | Native builder pattern |
| **Python** | Returns `self` | Same instance | Pythonic wrapper for PyO3 |
| **Java** | Returns new instance | Immutable builder | Java best practice |
| **C** | Returns new pointer | Memory management | C idioms |

<h2 align="center">❓ Frequently Asked Questions</h2>

**Q: How to add a new module?**
A: Implement the `RiModule` trait and register it via `RiAppBuilder::with_module`.

**Q: How to configure logging level?**
A: Set `logging.level` in the configuration file, supporting DEBUG/INFO/WARN/ERROR levels.

**Q: How to enable metrics export?**
A: Set `observability.metrics_enabled: true` and configure `prometheus_port` in the configuration file.

**Q: How to extend configuration sources?**
A: Implement a custom configuration loader and register it with `RiConfigManager`.

**Q: How to handle asynchronous tasks?**
A: Use `RiAppBuilder::with_async_module` to add async modules, the framework handles async lifecycle automatically.

<h2 align="center">🌏 Community & Citation</h2>

- Welcome to submit Issues and PRs!
- Github: https://github.com/mf2023/Ri.git
- Gitee: https://gitee.com/dunimd/ri.git
- GitCode: https://gitcode.com/dunimd/ri.git


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

| 📦 Package | 📜 License | 📦 Package | 📜 License |
|:-----------|:-----------|:-----------|:-----------|
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