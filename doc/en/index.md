<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../../assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">Documentation</span>
</h1>

<p align="center">
  English | <a href="../zh/index.md">简体中文</a>
</p>

<p align="center">
  <strong>Version: 0.1.7</strong> | <strong>Last modified date: 2026-02-17</strong>
</p>

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

Welcome to the DMSC (Dunimd Middleware Service) documentation! This documentation will help you understand and use the DMSC framework to build high-performance, reliable, and secure backend applications.

</div>

<div align="center">

## Documentation Navigation

</div>

### 1. Getting Started

- [**Introduction**](./01-introduction.md) - Learn about DMSC's core features, modular design, and application scenarios
- [**Getting Started**](./02-getting-started.md) - Complete guide from installation to running your first DMSC application

### 2. Core Concepts

- [**Core Concepts**](./03-core-concepts.md) - Deep understanding of DMSC's design philosophy, service context, module system, and lifecycle management

### 3. API Reference

- [**API Reference**](./04-api-reference/README.md) - Detailed module API documentation, including core, auth, cache, config, and other modules

### 4. Usage Examples

- [**Usage Examples**](./05-usage-examples/README.md) - Usage examples for various features, including basic applications, authentication and authorization, cache usage, observability, etc.

### 5. Best Practices

- [**Best Practices**](./06-best-practices.md) - Best practices for building efficient, reliable, and secure DMSC applications

### 6. Troubleshooting

- [**Troubleshooting**](./07-troubleshooting.md) - Common issues and solutions to help you quickly locate and resolve problems

### 7. Glossary

- [**Glossary**](./08-glossary.md) - Technical terms and concept definitions used in DMSC documentation

<div align="center">

## What is DMSC?

</div>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework that unifies backend infrastructure. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

### Core Features

- **Distributed Tracing**: W3C Trace Context standard implementation, full-chain TraceID/SpanID propagation
- **Enterprise Observability**: Native Prometheus metrics export, out-of-the-box Grafana integration
- **Intelligent Device Management**: Auto-discovery and registration, efficient resource pool management
- **Structured Logging**: JSON and text format support, automatic tracing context inclusion
- **Flexible Configuration**: Multi-source loading, hot configuration updates
- **Secure File System**: Unified project root directory management, atomic file operations

### Modular Design

DMSC adopts a highly modular architecture with 18 core modules, enabling on-demand composition and seamless extension:

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
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) |
| **log** | Structured logging with tracing context integration |
| **observability** | Metrics, tracing, and Grafana integration |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| **service_mesh** | Service discovery, health checking, and traffic management |
| **validation** | Input validation and data sanitization utilities |
| **protocol** | Protocol abstraction layer for multi-protocol support |
| **module_rpc** | Inter-module RPC communication for distributed method calls |

### 🐍 Python Module Support

DMSC provides complete Python bindings, allowing you to use all core features through Python:

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
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) |
| **log** | Structured logging with tracing context integration |
| **observability** | Metrics, tracing, and Grafana integration |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| **service_mesh** | Service discovery, health checking, and traffic management |
| **validation** | Input validation and data sanitization utilities |
| **protocol** | Protocol abstraction layer for multi-protocol support |
| **module_rpc** | Inter-module RPC communication for distributed method calls |

**Python SDK Features:**
- Native Python interfaces with seamless Rust core invocation
- Support for sync and async service modules
- Version: **0.1.7** (requires Python 3.8+)
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

## Getting Started

</div>

If you're using DMSC for the first time, we recommend starting with [Getting Started](./02-getting-started.md) to learn how to install and run your first DMSC application.

If you're already familiar with DMSC's basic concepts, you can check the [API Reference](./04-api-reference/README.md) for detailed module APIs, or view [Usage Examples](./05-usage-examples/README.md) to learn how to use various features.

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

</div>