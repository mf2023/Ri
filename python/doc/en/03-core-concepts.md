<div align="center">

# Core Concepts

**Version: 0.0.3**

**Last modified date: 2026-01-01**

Deep understanding of DMSC Python design philosophy, service context, module system, and lifecycle management

</div>

## Service Context

The service context is the core concept in DMSC Python, providing access to all framework features.

### Using Service Context

```python
async def my_service(ctx):
    # Access logger
    ctx.logger.info("service", "Processing request")
    
    # Access configuration
    db_url = ctx.config.get("database.url")
    
    # Use cache
    cached = await ctx.cache.get("key")
    
    # Access auth module
    user = await ctx.auth.get_current_user()
    
    return {"status": "ok"}
```

## Module System

DMSC Python provides a modular architecture with 12 core modules:

### Core Modules

| Module | Purpose |
|--------|---------|
| **core** | Runtime management and error handling |
| **auth** | Authentication and authorization |
| **cache** | Multi-backend caching |
| **config** | Configuration management |
| **log** | Structured logging |
| **observability** | Metrics and tracing |

### Extension Modules

| Module | Purpose |
|--------|---------|
| **fs** | File system operations |
| **device** | Device management |
| **gateway** | API gateway features |
| **mq** | Message queue |
| **service_mesh** | Service mesh capabilities |
| **protocol** | Protocol abstraction |
| **hooks** | Lifecycle event hooks |

## Lifecycle Management

### Application Lifecycle

DMSC Python applications go through several lifecycle stages:

1. **Initialization** - Configure and build the application
2. **Startup** - Initialize all modules
3. **Running** - Handle requests and events
4. **Shutdown** - Gracefully shutdown all modules

### Lifecycle Hooks

```python
from dmsc import DMSCHooks

async def on_startup(ctx):
    ctx.logger.info("lifecycle", "Application starting")

async def on_shutdown(ctx):
    ctx.logger.info("lifecycle", "Application shutting down")

app = DMSCAppBuilder()
app.with_hooks(
    on_startup=on_startup,
    on_shutdown=on_shutdown
)
```

## Error Handling

DMSC Python provides structured error handling:

```python
from dmsc import DMSCError, DMSCResult

async def my_operation() -> DMSCResult[str]:
    try:
        result = await risky_operation()
        return Ok(result)
    except DMSCError as e:
        return Err(e)
```

## Best Practices

### Use Async/Aawait

DMSC Python is designed for async programming:

```python
# Good
async def fetch_data():
    return await cache.get("key")

# Avoid
def fetch_data():
    return cache.get("key")  # This blocks!
```

### Handle Errors Gracefully

```python
async def robust_service(ctx):
    try:
        result = await ctx.cache.get("key")
        return result
    except Exception as e:
        ctx.logger.error("service", f"Error: {e}")
        return None
```

### Use Type Hints

DMSC Python supports full type hints:

```python
from typing import Optional
from dmsc import DMSCServiceContext

async def get_user(ctx: DMSCServiceContext, user_id: str) -> Optional[dict]:
    return await ctx.auth.get_user(user_id)
```
