<div align="center">

# Core API Reference

**Version: 0.1.4**

**Last modified date: 2026-01-15**

The core module is the foundation of DMSC, providing runtime, error handling, service context, and module system functionality.

## Module Overview

</div>

The core module contains the following submodules:

- **error**: Error handling mechanism
- **context**: Service context
- **module**: Module system
- **runtime**: Application runtime
- **app_builder**: Application builder
- **app_runtime**: Application runtime management

<div align="center">

## Core Components

</div>

### DMSCAppBuilder

Application builder for configuring and building DMSC applications.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create a new application builder | None | `DMSCAppBuilder` |
| `with_config(path)` | Add configuration file | `path: impl Into<String>` | `DMSCResult<Self>` |
| `with_logging(config)` | Set logging configuration | `config: DMSCLogConfig` | `DMSCResult<Self>` |
| `with_observability(config)` | Set observability configuration | `config: DMSCObservabilityConfig` | `DMSCResult<Self>` |
| `with_module(module)` | Add internal synchronous module | `module: Box<dyn ServiceModule>` | `Self` |
| `with_async_module(module)` | Add internal asynchronous module | `module: Box<dyn AsyncServiceModule>` | `Self` |
| `with_dms_module(module)` | Add custom asynchronous module | `module: Box<dyn DMSCModule>` | `Self` |
| `with_modules(modules)` | Add multiple internal synchronous modules | `modules: Vec<Box<dyn ServiceModule>>` | `Self` |
| `with_async_modules(modules)` | Add multiple internal asynchronous modules | `modules: Vec<Box<dyn AsyncServiceModule>>` | `Self` |
| `with_dms_modules(modules)` | Add multiple custom asynchronous modules | `modules: Vec<Box<dyn DMSCModule>>` | `Self` |
| `build()` | Build application runtime | None | `DMSCResult<DMSCAppRuntime>` |

#### Usage Example

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_observability(DMSCObservabilityConfig::default())?
    .with_dms_module(Box::new(MyCustomModule::new()))
    .build()?;
```

### DMSCAppRuntime

Application runtime for managing DMSC application lifecycle.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `run<F>(f)` | Run the application, executing the provided business logic | `f: F` where `F: Fn(&DMSCServiceContext) -> Fut` | `DMSCResult<()>` |
| `hook_bus()` | Get the hook bus | None | `&DMSCHookBus` |
| `stop()` | Stop the application | None | `DMSCResult<()>` |

#### Usage Example

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    ctx.logger().info("service", "DMSC service started")?;
    Ok(())
}).await
```

### DMSCServiceContext

Service context that provides access to core functionalities.

#### Methods

| Method | Description | Return Value |
|:--------|:-------------|:--------|
| `fs()` | Get file system accessor | `&DMSCFileSystem` |
| `logger()` | Get structured logger | `&DMSCLogger` |
| `config()` | Get configuration manager (shared ownership) | `Arc<DMSCConfigManager>` |
| `hooks()` | Get hook bus (shared ownership) | `Arc<DMSCHookBus>` |
| `hooks_mut()` | Get mutable hook bus (only available when having exclusive ownership) | `&mut DMSCHookBus` |
| `config_mut()` | Get mutable configuration manager (only available when having exclusive ownership) | `&mut DMSCConfigManager` |
| `fs_mut()` | Get mutable file system accessor | `&mut DMSCFileSystem` |
| `logger_mut()` | Get mutable structured logger (only available when having exclusive ownership) | `&mut DMSCLogger` |
| `metrics_registry()` | Get metrics registry if available | `Option<Arc<DMSCMetricsRegistry>>` |

#### Usage Example

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // Access logging functionality
    ctx.logger().info("service", "DMSC service started")?;
    
    // Access configuration functionality
    let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
    
    // Access file system functionality
    ctx.fs().write_file("data.txt", "content").await?;
    
    Ok(())
}).await
```

### DMSCModule

Async module trait for creating custom async modules (recommended).

#### Methods

| Method | Description | Parameters | Return Value | Default Implementation |
|:--------|:-------------|:--------|:--------|:--------|
| `name()` | Return module name | None | `&str` | None, must be implemented |
| `is_critical()` | Indicate if the module is critical | None | `bool` | Returns `true` |
| `priority()` | Return module priority | None | `i32` | Returns `0` |
| `dependencies()` | Return module dependency list | None | `Vec<&str>` | Returns empty list |
| `init(ctx)` | Initialize the module | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `before_start(ctx)` | Prepare for module startup | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `start(ctx)` | Start module service | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `after_start(ctx)` | Perform post-startup operations | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `before_shutdown(ctx)` | Prepare for shutdown | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `shutdown(ctx)` | Stop module service | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |
| `after_shutdown(ctx)` | Cleanup resources after shutdown | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` | Returns `Ok(())` |

#### Usage Example

```rust
struct MyCustomModule;

#[async_trait::async_trait]
impl DMSCModule for MyCustomModule {
    fn name(&self) -> &str {
        "my_custom_module"
    }
    
    async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "Module started")?;
        Ok(())
    }
}
```

### AsyncServiceModule

**Note**: This is an internal trait and not exposed to external users. Users should use the `DMSCModule` trait to create custom modules.

Async module trait for internal async modules in DMSC.

#### Methods

| Method | Description | Return Value | Default Implementation |
|:--------|:-------------|:--------|:--------|
| `name()` | Return module name | `&str` | None, must be implemented |
| `is_critical()` | Indicate if the module is critical | `bool` | Returns `true` |
| `priority()` | Return module priority | `i32` | Returns `0` |
| `dependencies()` | Return module dependency list | `Vec<&str>` | Returns empty list |
| `init(ctx)` | Async initialize module | `DMSCResult<()>` | Returns `Ok(())` |
| `before_start(ctx)` | Async prepare for startup | `DMSCResult<()>` | Returns `Ok(())` |
| `start(ctx)` | Async start module service | `DMSCResult<()>` | Returns `Ok(())` |
| `after_start(ctx)` | Async perform post-startup operations | `DMSCResult<()>` | Returns `Ok(())` |
| `before_shutdown(ctx)` | Async prepare for shutdown | `DMSCResult<()>` | Returns `Ok(())` |
| `shutdown(ctx)` | Async stop module service | `DMSCResult<()>` | Returns `Ok(())` |
| `after_shutdown(ctx)` | Async cleanup after shutdown | `DMSCResult<()>` | Returns `Ok(())` |

### DMSCError

Unified error type for DMSC.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(code, message)` | Create a new error | `code: &str`, `message: &str` | `DMSCError` |
| `with_context(context)` | Add error context | `context: impl Into<String>` | `Self` |
| `with_source(source)` | Add inner error | `source: impl std::error::Error + Send + Sync + 'static` | `Self` |
| `code()` | Get error code | None | `&str` |
| `message()` | Get error message | None | `&str` |
| `context()` | Get error context | None | `Option<&str>` |

#### Usage Example

```rust
Err(DMSCError::new("INVALID_CONFIG", "Invalid configuration")
    .with_context("service.port must be a positive integer"))
```

### DMSCResult

Result type alias to simplify error handling.

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

#### Usage Example

```rust
async fn my_function() -> DMSCResult<()> {
    // Business logic
    Ok(())
}
```
<div align="center">

## Error Codes

</div>

The core module defines the following error codes:

| Error Code | Description |
|:--------|:-------------|
| `INITIALIZATION_FAILED` | Initialization failed |
| `START_FAILED` | Startup failed |
| `STOP_FAILED` | Shutdown failed |
| `INVALID_CONFIG` | Invalid configuration |
| `MODULE_NOT_FOUND` | Module not found |
| `MODULE_ALREADY_REGISTERED` | Module already registered |

<div align="center">

## Best Practices

</div>

1. **Use the prelude module**: Import common types through `use dmsc::prelude::*` to simplify code
2. **Configure modules on demand**: Only add modules needed by the application to reduce resource consumption
3. **Use service context appropriately**: Access module functionality through the context to avoid direct dependencies on specific implementations
4. **Implement custom modules**: Extend DMSC functionality by implementing custom modules as needed
5. **Handle errors correctly**: Use the `?` operator to propagate errors, or handle them explicitly

<div align="center">

## Example Code

</div>

### Complete Application Example

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        
        // Access configuration
        let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
        ctx.logger().info("service", &format!("Service name: {}", service_name))?;
        
        Ok(())
    }).await
}
```
<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions