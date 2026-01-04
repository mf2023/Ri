<div align="center">

# DMSC (Dunimd Middleware Service) - DMSC Library for Python

English | [简体中文](README.zh.md)

[Help Documentation](../doc/en/index.md)

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://crates.io/crates/dmsc" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMSC-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework with Python bindings. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
DMSC adopts a highly modular architecture with 12 core modules, enabling on-demand composition and seamless extension:

<div align="center">

| Module | Description |
|:--------|:-------------|
| **auth** | Authentication & authorization (JWT, OAuth, permissions) |
| **cache** | Multi-backend cache abstraction (Memory, Redis, Hybrid) |
| **config** | Multi-source configuration management with hot reload |
| **core** | Runtime, error handling, and service context |
| **device** | Device control, discovery, and intelligent scheduling |
| **fs** | Secure file system operations and management |
| **gateway** | API gateway with load balancing, rate limiting, and circuit breaking |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) |
| **log** | Structured logging with tracing context integration |
| **observability** | Metrics, tracing, and Grafana integration |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| **service_mesh** | Service discovery, health checking, and traffic management |

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
- **Python**: 3.8+
- **pip**: Latest version
- **Platforms**: Linux, macOS, Windows

### Quick Setup

Install DMSC Python package:

```bash
pip install dmsc
```

Or add to your `requirements.txt`:

```
dmsc==0.1.3
```

<h2 align="center">⚡ Quick Start</h2>

### Core API Usage

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCObservabilityConfig

# Build service runtime
app = DMSCAppBuilder() \\
    .with_config("config.yaml") \\
    .with_logging(DMSCLogConfig.default()) \\
    .with_observability(DMSCObservabilityConfig.default()) \\
    .build()

# Run business logic
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

### Authentication Example

```python
from dmsc import DMSCAuthModule, DMSCJWTManager

# Create JWT manager
jwt_manager = DMSCJWTManager()
token = jwt_manager.generate_token({"user_id": 123})

# Verify token
payload = jwt_manager.verify_token(token)
```

### Queue Management Example

```python
from dmsc import DMSCQueueManager, DMSCQueueConfig

# Create queue manager
queue_config = DMSCQueueConfig()
queue_manager = DMSCQueueManager(queue_config)

# Send message
queue_manager.send_message("my_queue", {"data": "hello"})

# Receive message
message = queue_manager.receive_message("my_queue")
```

### Service Mesh Example

```python
from dmsc import DMSCServiceMesh, DMSCServiceDiscovery

# Create service mesh
service_mesh = DMSCServiceMesh()
service_discovery = DMSCServiceDiscovery()

# Register service
service_discovery.register_service("user-service", "localhost:8080")

# Discover service
service_info = service_discovery.discover_service("user-service")
```

### Cache Management Example

```python
from dmsc import DMSCCacheManager, DMSCCacheConfig

# Create cache manager
cache_config = DMSCCacheConfig()
cache_manager = DMSCCacheManager(cache_config)

# Set cache value
cache_manager.set("user:123", {"name": "John"}, ttl=3600)

# Get cache value
user_data = cache_manager.get("user:123")
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

### Configuration Sources

DMSC supports multiple configuration sources in order of priority (highest to lowest):
1. Runtime parameters
2. Environment variables (prefixed with `DMSC_`)
3. Configuration files (YAML, TOML, JSON)
4. Default values

<h2 align="center">🧪 Development & Testing</h2>

### Running Tests

```bash
# Install development dependencies
pip install -e .

# Run Python tests
python -m pytest tests/

# Run specific test module
python -m pytest tests/test_auth.py
```

<h2 align="center">❓ Frequently Asked Questions</h2>

**Q: How to add a new module?**
A: Use the existing module interfaces and register via DMSCAppBuilder.

**Q: How to configure logging level?**
A: Set `logging.level` in the configuration file, supporting DEBUG/INFO/WARN/ERROR levels.

**Q: How to enable metrics export?**
A: Set `observability.metrics_enabled: true` and configure `prometheus_port` in the configuration file.

**Q: How to extend configuration sources?**
A: Implement custom configuration loaders and register with DMSC configuration system.

**Q: How to handle asynchronous tasks?**
A: The framework handles async operations internally, use the provided async interfaces.

**Q: What Python versions are supported?**
A: Python 3.7 and above are supported.

**Q: Is the Rust backend included?**
A: Yes, the package includes the compiled Rust backend with Python bindings.

<h2 align="center">🌏 Community & Citation</h2>

- Welcome to submit Issues and PRs!
- Gitee: https://gitee.com/dunimd/dmsc.git

<div align="center">

## 📄 License & Open Source Agreements

### 🏛️ Project License

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

This project uses **Apache License 2.0** open source agreement, see [LICENSE](LICENSE) file.

### 📋 Dependencies License

<div align="center">

| 📦 Package | 📜 License |
|:-----------|:-----------|
| setuptools | MIT |
| setuptools-rust | MIT |
| wheel | MIT |
| pytest | MIT |
| pytest-asyncio | Apache 2.0 |
| black | MIT |
| isort | MIT |
| mypy | MIT |
| pyo3 | Apache 2.0 |

</div>  

</div>