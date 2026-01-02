<div align="center">

# Core API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The core module is the foundation of DMSC Python, providing application builder, service context, error handling, and other core features.

</div>

## Module Overview

The core module contains the following core components:

- **DMSCAppBuilder**: Application builder
- **DMSCAppRuntime**: DMSC application runtime
- **DMSCServiceContext**: Service context
- **DMSCError**: Error handling
- **DMSCModule**: Module system

<div align="center">

## Core Components

</div>

### DMSCAppBuilder

Application builder, used to configure and build DMSC applications.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `__init__()` | Create new application builder | None | `DMSCAppBuilder` |
| `with_logging(config)` | Add logging module | `config: DMSCLogConfig` | `DMSCAppBuilder` |
| `with_config(config)` | Add configuration module | `config: Union[str, DMSCConfig]` | `DMSCAppBuilder` |
| `with_cache(config)` | Add cache module | `config: DMSCCacheConfig` | `DMSCAppBuilder` |
| `with_fs(config)` | Add file system module | `config: None` | `DMSCAppBuilder` |
| `with_auth(config)` | Add authentication module | `config: DMSCAuthConfig` | `DMSCAppBuilder` |
| `with_observability(config)` | Add observability module | `config: DMSCObservabilityConfig` | `DMSCAppBuilder` |
| `with_module(module)` | Add custom module | `module: DMSCModule` | `DMSCAppBuilder` |
| `on_init(func)` | Register initialization hook | `func: Callable` | `DMSCAppBuilder` |
| `on_start(func)` | Register startup hook | `func: Callable` | `DMSCAppBuilder` |
| `on_shutdown(func)` | Register shutdown hook | `func: Callable` | `DMSCAppBuilder` |
| `build()` | Build DMSC application | None | `DMSCAppRuntime` |

#### Usage Example

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCConfig

# Create application builder
builder = DMSCAppBuilder()

# Configure logging
log_config = DMSCLogConfig(level="INFO")
builder.with_logging(log_config)

# Configure application
config = DMSCConfig()
config.set("app.name", "MyApp")
builder.with_config(config)

# Build application
app = builder.build()
```

### DMSCAppRuntime

DMSC application runtime instance, represents the built application.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `run_async(func)` | Run application asynchronously | `func: Callable[[DMSCServiceContext], Any]` | `Any` |
| `start_async()` | Start application asynchronously | None | `None` |
| `stop_async()` | Stop application asynchronously | None | `None` |
| `create_context()` | Create service context | None | `DMSCServiceContext` |
| `is_running()` | Check if application is running | None | `bool` |
| `await_shutdown()` | Await application shutdown | None | `None` |

#### Usage Example

```python
from dmsc import DMSCAppBuilder

async def my_service(ctx):
    ctx.logger.info("service", "Service started")
    return {"status": "running"}

# Build and run
app = DMSCAppBuilder().build()
result = app.run_async(my_service)
print(f"Result: {result}")
```

### DMSCServiceContext

Service context provides access to all DMSC modules and services.

#### Properties

| Property | Type | Description |
|:---------|:-----|:------------|
| `logger` | `DMSCLogger` | Logging service |
| `config` | `DMSCConfig` | Configuration service |
| `cache` | `DMSCCacheModule` | Cache service |
| `auth` | `DMSCAuthModule` | Authentication service |
| `fs` | `DMSCFSModule` | File system service |
| `observability` | `DMSCObservabilityModule` | Observability service |
| `time` | `DMSCTime` | Time service |
| `http` | `DMSCHTTPService` | HTTP service |
| `db` | `DMSCDatabase` | Database service |

#### Usage Example

```python
from dmsc import DMSCAppBuilder

async def my_service(ctx):
    # Use logger
    ctx.logger.info("service", "Processing request")
    
    # Access configuration
    app_name = ctx.config.get("app.name")
    
    # Use cache
    cached = await ctx.cache.get("key")
    
    # Use authentication
    user = await ctx.auth.get_current_user()
    
    return {
        "app": app_name,
        "user": user,
        "cached": cached
    }
```

### DMSCError

DMSC error type for structured error handling.

#### Properties

| Property | Type | Description |
|:---------|:-----|:------------|
| `code` | `str` | Error code |
| `message` | `str` | Error message |
| `details` | `Dict` | Additional error details |
| `cause` | `Exception` | Original exception |

#### Usage Example

```python
from dmsc import DMSCError, DMSCResult, Ok, Err

async def risky_operation() -> DMSCResult[str]:
    try:
        result = await might_fail()
        return Ok(result)
    except DMSCError as e:
        return Err(e)

# Handle result
result = await risky_operation()
if result.is_ok():
    print(f"Success: {result.value}")
else:
    print(f"Error: {result.error.message}")
```

## Module System

### DMSCModule

Base class for all DMSC modules.

```python
from dmsc import DMSCModule, DMSCServiceContext

class MyCustomModule(DMSCModule):
    name = "my_module"
    version = "1.0.0"
    
    async def initialize(self, ctx: DMSCServiceContext) -> None:
        """Initialize the module."""
        pass
    
    async def start(self, ctx: DMSCServiceContext) -> None:
        """Start the module."""
        pass
    
    async def stop(self, ctx: DMSCServiceContext) -> None:
        """Stop the module."""
        pass
    
    async def shutdown(self, ctx: DMSCServiceContext) -> None:
        """Shutdown the module."""
        pass
```

## Lifecycle Hooks

### Register Hooks

```python
from dmsc import DMSCAppBuilder

async def on_init(ctx):
    ctx.logger.info("lifecycle", "Application initializing")

async def on_start(ctx):
    ctx.logger.info("lifecycle", "Application starting")
    # Perform startup operations
    await ctx.cache.connect()

async def on_shutdown(ctx):
    ctx.logger.info("lifecycle", "Application shutting down")
    # Perform cleanup
    await ctx.cache.disconnect()

app = DMSCAppBuilder()
app.on_init(on_init)
app.on_start(on_start)
app.on_shutdown(on_shutdown)
app.build()
```

## Complete Example

```python
from dmsc import (
    DMSCAppBuilder, DMSCLogConfig, DMSCConfig,
    DMSCCacheConfig, DMSCAuthConfig, DMSCServiceContext
)
import asyncio

async def main():
    # Create application
    app = (
        DMSCAppBuilder()
        .with_logging(DMSCLogConfig(level="INFO"))
        .with_config(DMSCConfig.from_file("config.yaml"))
        .with_cache(DMSCCacheConfig(backend="memory"))
        .with_auth(DMSCAuthConfig(jwt_secret="secret"))
        .on_start(lambda ctx: ctx.logger.info("app", "Started"))
        .on_shutdown(lambda ctx: ctx.logger.info("app", "Shutdown"))
        .build()
    )
    
    # Run application
    await app.run_async(service_logic)

async def service_logic(ctx: DMSCServiceContext):
    ctx.logger.info("app", "Service running")
    
    # Access all services
    config = ctx.config.get("app.name")
    cached = await ctx.cache.get("key")
    user = await ctx.auth.get_current_user()
    
    return {"status": "ok", "config": config}

if __name__ == "__main__":
    asyncio.run(main())
```
