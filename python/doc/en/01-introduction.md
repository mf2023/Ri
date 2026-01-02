<div align="center">

# DMSC Python Introduction

**Version: 0.0.3**

**Last modified date: 2026-01-01**

Overview and core features of DMSC Python bindings

</div>

## Project Overview

**DMSC Python** is the official Python binding for DMSC (Dunimd Middleware Service), providing Python developers with an enterprise-grade microservice development framework. It combines the high performance of the Rust core with Python's ease of use, enabling you to build high-performance distributed systems using familiar Python syntax.

## Core Features

</div>

#### 🔍 Distributed Tracing
- W3C trace context standard implementation
- Full链路 TraceID/SpanID propagation
- Automatic context propagation
- Seamless integration with Python async frameworks

#### 📊 Enterprise Observability
- Native Prometheus metrics export
- Counter, Gauge, Histogram, Summary metric types
- Out-of-the-box Grafana dashboard integration
- Real-time performance statistics and percentile calculation
- Full-stack metrics monitoring (CPU, memory, I/O, network)

#### 🚀 High-Performance Python Bindings
- Zero-cost Python-Rust interoperability based on PyO3
- Async support, perfectly compatible with asyncio
- Type-safe Python API
- Memory-efficient serialization

#### 📝 Python-Friendly Structured Logging
- Supports JSON and text formats
- Configurable sampling rate
- Intelligent log rotation
- Automatic trace context inclusion
- DEBUG/INFO/WARN/ERROR log levels

#### ⚙️ Flexible Configuration Management
- Multi-source loading (files, environment variables, runtime)
- Hot configuration updates
- Python dictionary-style configuration access
- Type-safe configuration parsing

#### 🔒 Secure File System
- Unified project root directory management
- Atomic file operations
- Categorized directory structure
- JSON data persistence

#### 🌐 Web Service Support
- Built-in HTTP server
- RESTful API development
- WebSocket support
- Middleware mechanism
- Request/response interceptors

## Architecture Advantages

### 🏗️ Modular Design
DMSC Python adopts a highly modular architecture with 12 core modules, supporting on-demand combination and seamless extension:

| Module | Python Support | Description |
|:-------|:---------------|:------------|
| **core** | ✅ | Runtime, error handling, and service context |
| **auth** | ✅ | Authentication and authorization (JWT, OAuth, permissions) |
| **cache** | ✅ | Multi-backend cache abstraction (memory, Redis, hybrid) |
| **config** | ✅ | Multi-source configuration management and hot reload |
| **log** | ✅ | Structured logging with trace context integration |
| **observability** | ✅ | Metrics, tracing, and Grafana integration |
| **http** | ✅ | Web services and RESTful API development |
| **fs** | ✅ | Secure file system operations and management |
| **device** | 🚧 | Device control, discovery, and intelligent scheduling |
| **gateway** | 🚧 | API gateway with load balancing, rate limiting, and circuit breaker |
| **queue** | 🚧 | Distributed queue abstraction (Kafka, RabbitMQ, Redis) |
| **service_mesh** | 🚧 | Service discovery, health checks, and traffic management |

### 🔧 Unified Technology Stack
- **Single Language**: Python development, Rust performance
- **Async-First**: Native async/await support
- **Type Safety**: Complete type hints
- **Zero-Copy**: Efficient data transfer

### 📦 Easy Integration
- **pip Install**: Standard Python package management
- **Virtual Environment**: Perfect support for venv/poetry/pipenv
- **Containerization**: Ready for Docker and Kubernetes
- **CI/CD**: Support for mainstream platforms like GitHub Actions

## Performance

### ⚡ Benchmark Results
