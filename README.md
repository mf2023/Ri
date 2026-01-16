<div align="center">

# DMSCC (Dunimd Middleware Service)

English | [简体中文](README.zh.md)

[Help Documentation](doc/en/index.md)

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://github.com/mf2023/DMSC" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMSC-181717?style=flat-square&logo=github"/>
</a>
<a href="https://crates.io/crates/dmsc" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMSC-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework that unifies backend infrastructure. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
DMSC adopts a highly modular architecture with 17 core modules, enabling on-demand composition and seamless extension:

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
| **grpc** | gRPC server and client support with Python bindings |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) |
| **log** | Structured logging with tracing context integration |
| **observability** | Metrics, tracing, and Grafana integration |
| **orm** | Type-safe ORM with repository pattern, query builder, and Python bindings |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| **service_mesh** | Service discovery, health checking, and traffic management |
| **validation** | Input validation and data sanitization utilities |
| **ws** | WebSocket server support with Python bindings |

</div>

### 🚀 Key Features

#### 🔍 Distributed Tracing
- W3C Trace Context standard implementation
- Full-chain TraceID/SpanID propagation
- Baggage data transmission for business context
- Multi-language compatibility (Java, Go, Python)

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
- **Platforms**: Linux, macOS, Windows

### Quick Setup

Add DMSC to your project's `Cargo.toml`:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
```

Or use cargo add:

```bash
cargo add DMSC --git https://github.com/mf2023/DMSC
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
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
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
use dmsc::observability::*;

#[tracing::instrument(name = "user_service", skip(ctx))]
async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

或者使用手动 span：

```rust
use dmsc::prelude::*;

async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    let _span = ctx.tracer().start_span("get_user")?;
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
- Gitee: https://github.com/mf2023/DMSC.git


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

</div>

</div>