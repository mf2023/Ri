<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">DMSC Library for Python</span>
</h1>

English | [简体中文](README.zh.md)

[Help Documentation](https://mf2023.github.io/dmsc/dmsc/) | [Changelog](CHANGELOG.md) | [Security](../SECURITY.md) | [Contributing](../CONTRIBUTING.md) | [Code of Conduct](../CODE_OF_CONDUCT.md)

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


<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework with Python bindings. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
DMSC adopts a highly modular architecture with 18 core modules, enabling on-demand composition and seamless extension:

<div align="center">

| Module | Description | Python Support |
|:--------|:------------|:---------------|
| **auth** | Authentication & authorization (JWT, OAuth, permissions) | ✅ Full |
| **cache** | Multi-backend cache abstraction (Memory, Redis, Hybrid) | ✅ Full |
| **config** | Multi-source configuration management with hot reload | ✅ Full |
| **core** | Runtime, error handling, and service context | ✅ Full |
| **database** | Database abstraction with PostgreSQL, MySQL, SQLite support | ✅ Full |
| **device** | Device control, discovery, and intelligent scheduling | ✅ Full |
| **fs** | Secure file system operations and management | ✅ Full |
| **gateway** | API gateway with load balancing, rate limiting, and circuit breaking | ✅ Full |
| **grpc** | gRPC server and client support | ✅ Full (service registry + handler) |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) | ✅ Full |
| **log** | Structured logging with tracing context integration | ✅ Full |
| **module_rpc** | Inter-module RPC communication for distributed method calls | ✅ Full |
| **observability** | Metrics, tracing, and Grafana integration | ✅ Full |
| **orm** | Type-safe ORM with repository pattern and query builder | ✅ Full (types + QueryBuilder) |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) | ✅ Full |
| **service_mesh** | Service discovery, health checking, and traffic management | ✅ Full |
| **validation** | Input validation and data sanitization utilities | ✅ Full |
| **ws** | WebSocket server support | ✅ Full (handler + session manager) |

> **Note**: For gRPC/WebSocket servers in Python, use native Python libraries like `grpcio` and `websockets`. The DMSC Rust API provides advanced features for high-performance scenarios.

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
- **Python**: 3.8+ (Windows ARM64 requires 3.11+)
- **pip**: Latest version
- **Platforms**: Linux, macOS, Windows

### Build Dependencies (for building from source)

Some features require additional system dependencies when building from source:

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

Install DMSC Python package:

```bash
pip install dmsc
```

Or add to your `requirements.txt`:

```
dmsc==0.1.8
```

<h2 align="center">⚡ Quick Start</h2>

### Core API Usage

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # Build service runtime (supports method chaining)
    runtime = (DMSCAppBuilder()
        .with_config("config.yaml")
        .with_logging(DMSCLogConfig())
        .build())
    
    # Run business logic
    await runtime.run()

asyncio.run(main())
```

### Authentication Example

```python
from dmsc import DMSCJWTManager

# Create JWT manager with secret and expiry (in seconds)
jwt_manager = DMSCJWTManager("your-secret-key", 3600)

# Generate token with user ID, roles, and permissions
token = jwt_manager.generate_token("user123", ["admin"], ["read", "write"])

# Verify/validate token
payload = jwt_manager.validate_token(token)
print(f"User ID: {payload.sub}")
print(f"Roles: {payload.roles}")
print(f"Permissions: {payload.permissions}")
```

### Queue Management Example

```python
from dmsc import DMSCQueueManager

# Create queue manager (uses in-memory backend by default)
queue_manager = DMSCQueueManager()

# Create a queue
queue_manager.create_queue("my_queue")

# List all queues
queues = queue_manager.list_queues()
print(f"Queues: {queues}")

# Get a specific queue (for further operations in Rust)
queue = queue_manager.get_queue("my_queue")

# Delete a queue
queue_manager.delete_queue("my_queue")
```

**Note**: For advanced queue operations (push, pop, etc.), use the Rust API directly or extend the Python bindings.

### Service Mesh Example

```python
from dmsc.service_mesh import DMSCServiceMesh, DMSCServiceMeshConfig

# Create service mesh config
config = DMSCServiceMeshConfig()

# Create service mesh
service_mesh = DMSCServiceMesh(config)

# Register service
service_mesh.register_service("user-service", "http://localhost:8080", 100)

# Discover service
instances = service_mesh.discover_service("user-service")
for instance in instances:
    print(f"Service: {instance.service_name}, Host: {instance.host}, Port: {instance.port}")
```

### Cache Management Example

```python
from dmsc import DMSCCacheManager

# Create cache manager with in-memory backend (default)
cache_manager = DMSCCacheManager()

# Set cache value
cache_manager.set("user:123", '{"name": "John"}', ttl=3600)

# Get cache value
user_data = cache_manager.get("user:123")

# Check if key exists
if cache_manager.exists("user:123"):
    cache_manager.delete("user:123")
```

### gRPC Service Example

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# Create gRPC service registry
registry = DMSCGrpcServiceRegistryPy()

# Define service handler
def my_handler(method: str, data: bytes) -> bytes:
    print(f"Received request: {method}")
    return b"Response from Python handler"

# Register service
registry.register("my-service", my_handler)

# List registered services
services = registry.list_services()
print(f"Services: {services}")
```

### WebSocket Handler Example

```python
from dmsc.ws import DMSCWSPythonHandler, DMSCWSSessionManagerPy

# Create WebSocket handler with callbacks
handler = DMSCWSPythonHandler(
    on_connect=lambda session_id, remote_addr: print(f"Connected: {session_id}"),
    on_disconnect=lambda session_id: print(f"Disconnected: {session_id}"),
    on_message=lambda session_id, data: b"Echo: " + data,
    on_error=lambda session_id, error: print(f"Error: {error}")
)

# Create session manager
manager = DMSCWSSessionManagerPy(max_connections=1000)

# Get session count
count = manager.get_session_count()
print(f"Active sessions: {count}")
```

### ORM Usage Example (Rust API)

```rust
use dmsc::database::{DMSCORMSimpleRepository, Criteria, Pagination, ComparisonOperator};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

// Create repository (Rust only)
let repo = DMSCORMSimpleRepository::<User>::new("users");

// Find all users
let users = repo.find_all(&db).await?;

// Query with criteria
let criteria = Criteria::new("name", ComparisonOperator::Like, serde_json::json!("%John%"));
let users = repo.find_many(&db, vec![criteria]).await?;

// Paginated query
let pagination = Pagination::new(1, 20);
let (users, total) = repo.find_paginated(&db, pagination, vec![]).await?;
```

> **Note**: The ORM module provides type-safe database operations in Rust. For Python, use SQLAlchemy or other ORM libraries directly.

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
A: Python 3.8 and above are supported. Note: Windows ARM64 requires Python 3.11+.

**Q: Is the Rust backend included?**
A: Yes, the package includes the compiled Rust backend with Python bindings.

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