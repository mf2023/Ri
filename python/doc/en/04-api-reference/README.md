<div align="center">

# API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This directory contains detailed API documentation for DMSC Python core modules.

## Module List

</div>

| Module | Description | File |
|:--------|:-------------|:--------|
| **auth** | Authentication and authorization (JWT, OAuth, permissions) | [auth.md](./auth.md) |
| **cache** | Multi-backend cache abstraction (memory, Redis, hybrid) | [cache.md](./cache.md) |
| **config** | Multi-source configuration management and hot reload | [config.md](./config.md) |
| **core** | Runtime, error handling, and service context | [core.md](./core.md) |
| **device** | Device control, discovery, and intelligent scheduling | [device.md](./device.md) |
| **fs** | Secure file system operations and management | [fs.md](./fs.md) |
| **gateway** | API gateway with load balancing, rate limiting, and circuit breaker | [gateway.md](./gateway.md) |
| **hooks** | Lifecycle event hooks (startup, shutdown, etc.) | [hooks.md](./hooks.md) |
| **log** | Structured logging with trace context integration | [log.md](./log.md) |
| **mq** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, memory) | [mq.md](./mq.md) |
| **observability** | Metrics, tracing, and Grafana integration | [observability.md](./observability.md) |
| **protocol** | Protocol abstraction layer with global and private communication protocols | [protocol.md](./protocol.md) |
| **service_mesh** | Service discovery, health checks, and traffic management | [service_mesh.md](./service_mesh.md) |

<div align="center">

## Usage Guide

</div>

Each API documentation includes:

1. **Module Overview**: Main functionality and purpose of the module
2. **Core Components**: Key types and classes in the module
3. **API Reference**: Detailed method and function descriptions
4. **Usage Examples**: Code examples demonstrating module usage
5. **Best Practices**: Recommendations and best practices for using the module

<div align="center">

## Naming Conventions

</div>

All DMSC Python APIs follow these naming conventions:

- **Classes**: PascalCase (e.g., `DMSCAppBuilder`, `DMSCServiceContext`)
- **Functions/Methods**: snake_case (e.g., `get_user`, `create_session`)
- **Constants**: UPPER_SNAKE_CASE (e.g., `DEFAULT_TIMEOUT`, `MAX_CONNECTIONS`)
- **Configuration Keys**: dot.notation (e.g., `app.name`, `database.url`)

<div align="center">

## Error Handling

</div>

DMSC Python uses a Result-based error handling pattern:

```python
from dmsc import DMSCResult, DMSCError

async def safe_operation() -> DMSCResult[str]:
    try:
        result = await risky_operation()
        return Ok(result)
    except DMSCError as e:
        return Err(e)
```

<div align="center">

## Type Hints

</div>

All APIs include complete type hints for better IDE support and type checking:

```python
from dmsc import DMSCServiceContext
from typing import Optional, Dict, Any

async def get_user(
    ctx: DMSCServiceContext,
    user_id: str
) -> Optional[Dict[str, Any]]:
    return await ctx.auth.get_user(user_id)
```
