<div align="center">

# ⚠️ Compliance Notice

**According to relevant laws and regulations of various countries (including but not limited to China's "Interim Measures for the Management of Generative AI Services", the EU's "AI Act", the US "AI Risk Management Framework", Japan's "AI Principles", etc.), developers or users must bear their own compliance responsibilities. Failure to fulfill relevant obligations may result in service suspension, regulatory penalties, or legal liability.**

---

# DMS (Dunimd Middleware Service)

[English](README.md) | [简体中文](README.zh.md)

<a href="https://github.com/dunimd/dms" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMS-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://crates.io/crates/DMS" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMS-000000?style=flat-square&logo=rust"/>
</a>

An enterprise-grade Rust service framework that provides unified infrastructure support for all Dunimd team projects. DMS transforms the originally scattered Python utils toolkit into a modern, enterprise-level Rust service framework, similar to GMS/HMS positioning, providing unified infrastructure capabilities for all backend services.

</div>

<h2 align="center">🚀 Core Architecture</h2>

### 🌐 Distributed Tracing System
Implements W3C Trace Context standard with full-chain TraceID/SpanID propagation, Baggage data transmission for business context information, standardized context carrier mechanisms, and multi-language compatibility for integration with Java, Go, Python and other heterogeneous systems.

### 📊 Enterprise Observability Platform
Native Prometheus metrics export supporting Counter, Gauge, Histogram, Summary types, out-of-the-box Grafana dashboard integration, high-performance sliding window algorithms for real-time data collection (DMSSlidingWindow), precise quantile calculations for performance statistics (DMSQuantileCalculator), and comprehensive multi-dimensional monitoring for CPU, memory, I/O, network and other full-stack metrics.

### 🤖 Intelligent Device Control & Scheduling
Intelligent device auto-discovery and registration, efficient resource pool management for allocation and recycling, policy-based intelligent scheduling algorithms with priority support, dynamic device load balancing, and complete lifecycle management with device state monitoring and maintenance.

### 📝 Enterprise Logging System
Structured log output supporting both JSON and text formats, configurable sampling rates to avoid performance impact, intelligent log rotation based on file size, automatic inclusion of tracing context information, and multi-level support for DEBUG/INFO/WARN/ERROR log levels.

### ⚙️ Configuration Management & Extensibility
Multi-source configuration loading from files, environment variables, and runtime parameters, hot configuration capabilities for runtime dynamic updates, modular architecture with 7 core modules for on-demand composition, lifecycle hooks for Startup, Shutdown and other critical events, and plugin-based extensions supporting custom modules and extension points.

### 📁 File System & Data Management
Unified project root directory management for file system namespace, atomic file operations ensuring data consistency, categorized directory management separating logs, cache, reports, observability, and temporary directories, JSON data persistence for complex data structure serialization, and secure directory creation preventing path traversal and permission issues.

### 🔧 Modular Architecture

```
DMS Framework
├── dms-core          # Core runtime and error handling
├── dms-config        # Unified configuration management
├── dms-log           # Enterprise logging system
├── dms-observability # Observability platform
├── dms-hooks         # Lifecycle hooks
├── dms-cache         # Cache abstraction layer
├── dms-fs            # File system encapsulation
├── dms-resource      # Resource management (generalized devices)
└── dms-extension-api # Extension capability definitions
```

### 🎯 Core Components

- **ServiceRuntime**: Unified service runtime
- **ServiceContext**: Service context integrating all infrastructure
- **AppBuilder**: Declarative service builder
- **DMSModule**: Modular extension interface

---

<h2 align="center">⚡ Quick Start</h2>

### **1. Add Dependency**

```toml
[dependencies]
DMS = { git = "https://github.com/dunimd/dms" }
```

### **2. Create Service**

```rust
use DMS::prelude::*;

#[tokio::main]
async fn main() -> DMSResult<()> {
    // Build service runtime
    let app = DMSAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(LoggingConfig::default())?
        .with_observability(ObservabilityConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSServiceContext| async move {
        ctx.logger().info("service", "DMS service started")?;
        // Your business code
        Ok(())
    }).await
}
```

### **3. Use Observability**

```rust
use DMS::observability::*;

#[traced(name = "user_service")]
async fn get_user(ctx: &DMSServiceContext, user_id: u64) -> DMSResult<User> {
    // Automatically record tracing information and metrics
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

---

<h2 align="center">📈 Performance Metrics</h2>

| Metric | Value |
|--------|-------|
| **Compilation Time** | Release build < 15 seconds |
| **Memory Usage** | Base runtime < 10MB |
| **Zero-Cost Abstraction** | Compile-time optimization, no runtime overhead |
| **Thread Safety** | Rust ownership system guarantees memory safety |
| **Build Status** | Zero warnings, zero errors |

---

<h2 align="center">🔧 Configuration Example</h2>

```yaml
# config.yaml
service:
  name: "my-service"
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

resource:
  providers: ["cpu", "gpu", "memory"]
  scheduling_policy: "priority_based"
```

---

<h2 align="center">🧪 Development Status</h2>

| Module | Status | Description |
|--------|--------|-------------|
| **Core Modules** | ✅ Complete | All 7 core modules finished |
| **Extension Mechanism** | ✅ Supported | Plugin-based architecture ready |
| **Example Projects** | ✅ Provided | Working examples available |
| **Documentation** | ✅ Complete | Full API documentation |
| **Testing** | ✅ Coverage | Unit test coverage |
| **CI/CD** | ✅ Automated | Automated build pipeline |

---

<h2 align="center">📚 Module Documentation</h2>

- [Core Module](src/core/) - Runtime and error handling
- [Configuration Management](src/config/) - Unified configuration interface
- [Logging System](src/log/) - Structured logging
- [Observability](src/observability/) - Metrics and tracing
- [Device Control](src/device/) - Resource scheduling and management
- [File System](src/fs/) - Secure file operations
- [Lifecycle Hooks](src/hooks/) - Event system

---

<h2 align="center">🤝 Contributing Guidelines</h2>

1. Fork the project
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add some amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Create Pull Request

---

<h2 align="center">📄 License</h2>

This project adopts [MIT License](LICENSE) - see [LICENSE](LICENSE) file for details

---

<h2 align="center">🏆 Achievements</h2>

- **Zero Warnings Zero Errors**: Verified through `cargo check --quiet`
- **Enterprise Quality**: Production environment deployment ready
- **Modular Design**: Supports on-demand composition and extension
- **Performance Optimization**: Release build optimization complete

---

<div align="center">

**DMS** - *Providing solid technical foundation for every Dunimd service* 🦀✨

</div>