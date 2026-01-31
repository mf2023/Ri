<div align="center">

# DMSC Introduction

**Version: 0.1.6**

**Last modified date: 2026-01-30**

## Project Overview

**DMSC (Dunimd Middleware Service)** is a high-performance Rust middleware framework designed for unifying backend infrastructure. It adopts a modular architecture, providing various features required for enterprise-scale applications, including built-in observability and distributed system support.

## Core Features

</div>

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

<div align="center">

## Technology Stack

| Technology | Purpose |
|------------|---------|
| Rust | Main development language, providing high performance and memory safety |
| Tokio | Async runtime, supporting high concurrency |
| Serde | Serialization/deserialization library |
| Prometheus | Metrics collection and monitoring |
| W3C Trace Context | Distributed tracing standard |
| YAML/TOML/JSON | Configuration file format support |

## Modular Design

DMSC adopts a highly modular architecture with 12 core modules, enabling on-demand composition and seamless extension:

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

### 🐍 Python Module Support

</div>

DMSC provides complete Python bindings, allowing you to use all core features through Python:

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

**Python SDK Features:**
- Native Python interfaces with seamless Rust core invocation
- Support for sync and async service modules
- Version: **0.1.6** (requires Python 3.8+)
- Distributed via [PyPI](https://pypi.org/project/dmsc/)

**Quick Start:**
```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

# Build service runtime
app = DMSCAppBuilder() \
    .with_config("config.yaml") \
    .with_logging(DMSCLogConfig()) \
    .build()

# Run business logic
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

For more Python usage examples, please check the [Python README](https://github.com/mf2023/DMSC/blob/master/python/README.md).

<div align="center">

## Application Scenarios

</div>

DMSC is suitable for various enterprise-level backend application scenarios, including:

- **Microservice Architecture**: As middleware for inter-service communication and coordination
- **API Gateway**: Providing a unified API entry with rate limiting, circuit breaking, etc.
- **Distributed Systems**: Simplifying the development and management of distributed systems
- **Real-time Data Processing**: Supporting high-concurrency data processing and stream processing
- **Enterprise Applications**: Providing reliable infrastructure support


<div align="center">

## Community & Support

</div>

- **GitHub/Gitee**: [https://github.com/mf2023/DMSC](https://github.com/mf2023/DMSC)
- **Issues**: Submit questions and suggestions
- **Pull Requests**: Contributions are welcome

<div align="center">

## 📄 License & Open Source Agreements

### 🏛️ Project License

<p align="center">
  <a href="../LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

This project uses the **Apache License 2.0** open source agreement, see [LICENSE](../LICENSE) file.

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

</div>

### 📋 Python Dependencies License

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

## Next Steps

</div>

- [Getting Started](./02-getting-started.md): Installation and creating your first DMSC application
- [Core Concepts](./03-core-concepts.md): In-depth understanding of DMSC's design philosophy and core components
- [API Reference](./04-api-reference/README.md): Detailed module API documentation
- [Usage Examples](./05-usage-examples/README.md): Usage examples for various scenarios
- [Best Practices](./06-best-practices.md): Best practices for developing DMSC applications
- [Troubleshooting](./07-troubleshooting.md): Common issues and solutions
- [Glossary](./08-glossary.md): Core terminology explanation