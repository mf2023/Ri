<div align="center">

# Core API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-30**

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
| `with_python_module(module)` | Add Python module (requires pyo3 feature) | `module: DMSCPythonModuleAdapter` | `Self` |
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

#### Enum Variants

| Variant | Description | Parameters |
|:--------|:-------------|:--------|
| `Io(String)` | I/O operation failed | Error message |
| `Serde(String)` | Serialization/deserialization error | Error message |
| `Config(String)` | Configuration error | Error message |
| `Hook(String)` | Hook execution error | Error message |
| `Prometheus(String)` | Prometheus metrics error | Error message |
| `ServiceMesh(String)` | Service mesh error | Error message |
| `InvalidState(String)` | Invalid state error | Error message |
| `InvalidInput(String)` | Invalid input error | Error message |
| `SecurityViolation(String)` | Security violation error | Error message |
| `DeviceNotFound { device_id }` | Device not found | Device ID |
| `DeviceAllocationFailed { device_id, reason }` | Device allocation failed | Device ID and reason |
| `AllocationNotFound { allocation_id }` | Allocation not found | Allocation ID |
| `ModuleNotFound { module_name }` | Module not found | Module name |
| `ModuleInitFailed { module_name, reason }` | Module initialization failed | Module name and reason |
| `ModuleStartFailed { module_name, reason }` | Module start failed | Module name and reason |
| `ModuleShutdownFailed { module_name, reason }` | Module shutdown failed | Module name and reason |
| `CircularDependency { modules }` | Circular dependency detected | List of involved modules |
| `MissingDependency { module_name, dependency }` | Missing dependency | Module name and dependency |
| `Other(String)` | Other error | Error message |
| `ExternalError(String)` | External error | Error message |
| `PoolError(String)` | Connection pool error | Error message |
| `DeviceError(String)` | Device error | Error message |
| `RedisError(String)` | Redis error | Error message |
| `HttpClientError(String)` | HTTP client error | Error message |
| `TomlError(String)` | TOML parsing error | Error message |
| `YamlError(String)` | YAML parsing error | Error message |
| `Queue(String)` | Queue error | Error message |
| `FrameError(String)` | Frame error | Error message |
| `Database(String)` | Database error | Error message |

#### Usage Example

```rust
// Create error using enum variant
Err(DMSCError::Config("Invalid port number".to_string()))

// I/O error auto-conversion
let file = std::fs::File::open("config.yaml")?; // Auto converts to DMSCError::Io

// Device-related error
Err(DMSCError::DeviceNotFound { device_id: "gpu-001".to_string() })
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

### DMSCLockError

Safe lock error type, specifically for concurrent lock operation error handling.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(context)` | Create a new lock error | `context: &str` | `DMSCLockError` |
| `poisoned(context)` | Create a poisoned lock error | `context: &str` | `DMSCLockError` |
| `is_poisoned()` | Check if it's a poisoned lock error | None | `bool` |
| `context()` | Get error context | None | `&str` |

#### Usage Example

```rust
match lock.read_safe("my data") {
    Ok(data) => println!("Data: {}", data),
    Err(e) if e.is_poisoned() => {
        log::error!("Lock poisoned: {}", e.context());
    }
    Err(e) => {
        log::error!("Lock error: {}", e.context());
    }
}
```

### DMSCLockResult

Lock operation result type alias.

> **Note**: This type is a Rust-only type alias and is not available in Python. In Python, you can use `Result[T, DMSCLockError]` directly instead.

```rust
type DMSCLockResult<T> = Result<T, DMSCLockError>;
```

#### Usage Example

```rust
fn safe_read_data(lock: &RwLock<String>, context: &str) -> DMSCLockResult<String> {
    let data = lock.read_safe(context)?;
    Ok(data.clone())
}
```

### RwLockExtensions

Extension trait providing safe lock acquisition for standard library `RwLock`.

> **Note**: This trait is Rust-only and is not available in Python. Python users can use `RwLock.read()` and `RwLock.write()` methods directly.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `read_safe(context)` | Safely acquire read lock | `context: &str` | `DMSCLockResult<RwLockReadGuard<T>>` |
| `write_safe(context)` | Safely acquire write lock | `context: &str` | `DMSCLockResult<RwLockWriteGuard<T>>` |
| `try_read_safe(context)` | Try to acquire read lock (non-blocking) | `context: &str` | `DMSCLockResult<Option<RwLockReadGuard<T>>>` |
| `try_write_safe(context)` | Try to acquire write lock (non-blocking) | `context: &str` | `DMSCLockResult<Option<RwLockWriteGuard<T>>>` |

#### Usage Example

```rust
use dmsc::core::lock::RwLockExtensions;

let lock = RwLock::new(42);

fn read_value(lock: &RwLock<i32>) -> DMSCLockResult<i32> {
    let value = lock.read_safe("reading counter")?;
    Ok(*value)
}

fn write_value(lock: &RwLock<i32>, new_value: i32) -> DMSCLockResult<()> {
    let mut value = lock.write_safe("writing counter")?;
    *value = new_value;
    Ok(())
}
```

### MutexExtensions

Extension trait providing safe lock acquisition for standard library `Mutex`.

> **Note**: This trait is Rust-only and is not available in Python. Python users can use `Mutex.lock()` method directly.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `lock_safe(context)` | Safely acquire mutex lock | `context: &str` | `DMSCLockResult<MutexGuard<T>>` |
| `try_lock_safe(context)` | Try to acquire mutex lock (non-blocking) | `context: &str` | `DMSCLockResult<Option<MutexGuard<T>>>` |

#### Usage Example

```rust
use dmsc::core::lock::MutexExtensions;

let mutex = Mutex::new(Vec::new());

fn push_item(mutex: &Mutex<Vec<String>>, item: String) -> DMSCLockResult<()> {
    let mut items = mutex.lock_safe("pushing item")?;
    items.push(item);
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
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication