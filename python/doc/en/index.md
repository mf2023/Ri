<div align="center">

# DMSC Python Documentation

**Version: 0.0.3**

**Last modified date: 2026-01-01**

## Documentation Navigation

### 1. Introduction & Getting Started

- [**Introduction**](./01-introduction.md) - Overview and core features of DMSC Python bindings
- [**Getting Started**](./02-getting-started.md) - Install, configure, and run your first DMSC Python application

### 2. Core Concepts

- [**Core Concepts**](./03-core-concepts.md) - Deep understanding of DMSC Python design philosophy, service context, module system, and lifecycle management

### 3. API Reference

- [**API Reference**](./04-api-reference/README.md) - Detailed module API documentation including core, authentication, cache, config, and more

### 4. Usage Examples

- [**Usage Examples**](./05-usage-examples/README.md) - Usage examples for various features including basic applications, authentication & authorization, caching, observability, and more

### 5. Best Practices

- [**Best Practices**](./06-best-practices.md) - Best practices for building efficient, reliable, and secure DMSC Python applications

### 6. Troubleshooting

- [**Troubleshooting**](./07-troubleshooting.md) - Common issues and solutions to help you quickly locate and resolve problems

### 7. Glossary

- [**Glossary**](./08-glossary.md) - Technical terms and concept definitions used in DMSC Python documentation

<div align="center">

## What is DMSC Python?

</div>

**DMSC Python** — The official Python bindings for DMSC (Dunimd Middleware Service), providing Python developers with a high-performance, enterprise-grade microservice development framework. It inherits all the advantages of the Rust core while offering Python-friendly API interfaces.

### Core Features

- **🚀 High Performance**: Based on Rust core, providing near-native performance
- **🐍 Python-Friendly**: API design that follows Python programming conventions
- **🔧 Complete Functionality**: Supports all 12 core modules of DMSC
- **📦 Easy Installation**: Simple installation via PyPI
- **🔍 Type Hints**: Complete type annotation support
- **⚡ Async Support**: Native async/await support

### Supported Modules

| Module | Python Support | Description |
|--------|----------------|-------------|
| **core** | ✅ | Runtime, error handling, and service context |
| **auth** | ✅ | Authentication and authorization (JWT, OAuth, permissions) |
| **cache** | ✅ | Multi-backend cache abstraction |
| **config** | ✅ | Multi-source configuration management |
| **log** | ✅ | Structured logging |
| **observability** | ✅ | Metrics, tracing, and monitoring |
| **fs** | ✅ | Secure file system operations |
| **device** | ✅ | Device management, discovery, and scheduling |
| **gateway** | ✅ | API gateway with load balancing, rate limiting, and circuit breaker |
| **queue** | ✅ | Distributed queue abstraction |
| **service_mesh** | ✅ | Service discovery, health checks, and traffic management |
| **protocol** | ✅ | Protocol abstraction layer with encryption and communication |
| **hooks** | ✅ | Lifecycle event hooks |

### Quick Example

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # Create application builder
    app = DMSCAppBuilder()
    
    # Configure logging
    app.with_logging(DMSCLogConfig.default())
    
    # Build application
    dms_app = app.build()
    
    # Run application
    await dms_app.run_async(my_service_logic)

async def my_service_logic(ctx):
    # Use service context
    ctx.logger.info("demo", "Hello from DMSC Python!")
    
    # Access configuration
    config_value = ctx.config.get("my.key", "default")
    
    # Use cache
    await ctx.cache.set("key", "value", ttl=3600)
    
    return {"status": "success"}

if __name__ == "__main__":
    asyncio.run(main())
```

### Installation

```bash
# Install from PyPI
pip install dmsc

# Or using poetry
poetry add dmsc

# Or using pipenv
pipenv install dmsc
```

### System Requirements

- **Python Version**: 3.8+
- **Operating System**: Windows, Linux, macOS
- **Architecture Support**: x86_64, ARM64

<div align="center">

## Next Steps

</div>

- [Introduction](./01-introduction.md) - Learn more about DMSC Python's core architecture and design philosophy
- [Getting Started](./02-getting-started.md) - Start your first DMSC Python project
- [Core Concepts](./03-core-concepts.md) - Master DMSC Python's core concepts and best practices
