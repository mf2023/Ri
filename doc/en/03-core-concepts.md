<div align="center">

# Core Concepts

**Version: 0.1.6**

**Last modified date: 2026-02-01**

This chapter provides an in-depth introduction to DMSC's core design principles and key components, helping you better understand and use the DMSC framework.

## 1. Modular Architecture

DMSC adopts a highly modular design, dividing different functionalities into independent modules, supporting on-demand composition and extension.

</div>

### 1.1 Design Principles

- **Single Responsibility**: Each module focuses on a specific domain of functionality
- **Loose Coupling**: Modules communicate through clearly defined interfaces, reducing dependencies
- **High Cohesion**: Related functionalities are concentrated within the same module
- **Extensibility**: Supports extending functionality through custom modules
- **Testability**: Modules can be tested independently

### 1.2 Module Dependency Relationships

Modules have complex dependency relationships, and the DMSC framework automatically handles module loading order through the following mechanisms:

1. **Dependency Declaration**: Each module can declare its dependencies through the `dependencies()` method
2. **Priority Sorting**: Modules can set priority through the `priority()` method, higher values mean higher priority
3. **Automatic Sorting**: The framework automatically sorts modules based on their dependencies and priorities, ensuring dependent modules are loaded first
4. **Critical Module Marking**: Modules can be marked as critical through the `is_critical()` method, failure of critical modules will cause the entire system to fail

### 1.3 Core Module Structure

- **core**: The most fundamental module, providing runtime, error handling, service context, and module system
  - Includes: error, context, module, runtime, app_builder, app_runtime, lifecycle, etc.
- **log**: Depends on core, provides structured logging functionality
- **config**: Depends on core, provides configuration management functionality
- **hooks**: Depends on core, provides lifecycle hook functionality
- **observability**: Depends on core, provides metrics, tracing, and monitoring functionality
- **fs**: Depends on core, provides secure file system access
- **Other modules**: Depend on the above basic modules as needed

### 1.5 Python Module Support

DMSC provides complete Python bindings, allowing you to use all core features through Python. Python modules have the same functionality as Rust modules:

| Python Module | Corresponding Rust Module | Description |
|:-------------|:------------------------|:------------|
| `dmsc.auth` | auth | Authentication & authorization (JWT, OAuth, permissions) |
| `dmsc.cache` | cache | Multi-backend cache abstraction (Memory, Redis, Hybrid) |
| `dmsc.config` | config | Multi-source configuration management with hot reload |
| `dmsc.core` | core | Runtime, error handling, and service context |
| `dmsc.device` | device | Device control, discovery, and intelligent scheduling |
| `dmsc.fs` | fs | Secure file system operations and management |
| `dmsc.gateway` | gateway | API gateway with load balancing, rate limiting, and circuit breaking |
| `dmsc.hooks` | hooks | Lifecycle event hooks (Startup, Shutdown, etc.) |
| `dmsc.log` | log | Structured logging with tracing context integration |
| `dmsc.observability` | observability | Metrics, tracing, and Grafana integration |
| `dmsc.queue` | queue | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) |
| `dmsc.service_mesh` | service_mesh | Service discovery, health checking, and traffic management |

**Python Module Features:**
- Native Python interfaces with seamless Rust core invocation
- Support for sync and async service modules
- Version: **0.1.6** (requires Python 3.8+)

### 1.6 Module Composition

You can selectively compose the modules you need based on your application requirements:

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    // Add custom module
    .with_dms_module(Box::new(MyCustomModule::new()))?
    .build()?;
```

<div align="center">

## 2. Service Context

`DMSCServiceContext` is the core of DMSC applications, providing access to all module functionalities.

</div>

### 2.1 Design Philosophy

The service context adopts a dependency injection pattern, centralizing all module functionalities in a unified interface, facilitating:

- **Unified Access**: Access all module functionalities through a single object
- **Dependency Decoupling**: Business code does not directly depend on specific module implementations
- **Test Friendliness**: Easy to replace specific implementations in tests
- **Extensibility**: New modules can be seamlessly integrated into the context

### 2.2 Core Functionality Access

Through the service context, you can access various module functionalities:

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // Access logging functionality
    ctx.logger().info("service", "DMSC service started")?;
    
    // Access configuration functionality
    let service_name = ctx.config().get("service.name").unwrap_or("unknown");
    
    // Access cache functionality (through module system)
    if let Ok(cache) = ctx.module::<DMSCCacheModule>().await {
        let cache_manager = cache.cache_manager();
        cache_manager.set("key", "value", Some(3600)).await?;
    }
    
    // Access file system functionality
    ctx.fs().write_file("data.txt", "content").await?;
    
    Ok(())
}).await
```

### 2.3 Context Lifecycle

The service context's lifecycle is consistent with the application's lifecycle:

1. **Creation**: Created during `DMSCAppBuilder::build()`
2. **Usage**: Passed to business logic through closures during application runtime
3. **Destruction**: Automatically destroyed when the application exits

<div align="center">

## 3. Module System

DMSC's module system allows you to extend framework functionality by implementing custom modules.

</div>

### 3.1 Module Types

DMSC provides a flexible module system, supporting multiple module types:

- **Public Async Module**: Implements the `DMSCModule` trait, executes in an async context, this is the recommended type for custom modules
- **Internal Sync Module**: Implements the `ServiceModule` trait, executes in the main thread, mainly used for internal framework implementations
- **Internal Async Module**: Implements the `AsyncServiceModule` trait, executes in an async context, mainly used for internal framework implementations

### 3.2 Custom Module Example

Create a custom async module (recommended):

```rust
use dmsc::core::module::DMSCModule;
use dmsc::core::context::DMSCServiceContext;
use dmsc::core::error::DMSCResult;
use async_trait::async_trait;

pub struct MyCustomModule {
    // Module configuration
}

#[async_trait]
impl DMSCModule for MyCustomModule {
    fn name(&self) -> &str {
        "my_custom_module"
    }
    
    // Set module priority (default: 0, higher values mean higher priority)
    fn priority(&self) -> i32 {
        100
    }
    
    // Declare dependencies on other modules (default: empty list)
    fn dependencies(&self) -> Vec<&str> {
        vec!["log", "config"]
    }
    
    // Initialize module, set up resources
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "My custom module initialized")?;
        Ok(())
    }
    
    // Start module service, begin handling requests
    async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "My custom module started")?;
        Ok(())
    }
    
    // Stop module service, release core resources
    async fn shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        ctx.logger().info(self.name(), "My custom module stopped")?;
        Ok(())
    }
}
```

Register custom module:

```rust
let app = DMSCAppBuilder::new()
    // Other configurations
    .with_dms_module(Box::new(MyCustomModule { /* configuration */ }))?
    .build()?;
```

### 3.3 Module Priority

You can set a priority for modules to control their loading order:

```rust
impl DMSCModule for MyCustomModule {
    // Other methods
    
    fn priority(&self) -> i32 {
        100 // Higher value means higher priority
    }
}
```

### 3.4 Python Custom Modules

DMSC Python SDK supports creating custom sync and async modules:

```python
from dmsc import DMSCPythonServiceModule, DMSCPythonAsyncServiceModule, DMSCServiceContext

class MyPyModule(DMSCPythonServiceModule):
    """Sync service module"""
    def name(self) -> str:
        return "my_python_module"
    
    def init(self, ctx: DMSCServiceContext) -> None:
        """Initialize module, return None for success"""
        ctx.logger().info("my_python_module", "Python module initialized")
    
    def start(self, ctx: DMSCServiceContext) -> None:
        """Start module, return None for success"""
        ctx.logger().info("my_python_module", "Python module started")
    
    def shutdown(self, ctx: DMSCServiceContext) -> None:
        """Shutdown module, return None for success"""
        ctx.logger().info("my_python_module", "Python module stopped")

# Async module example
class MyAsyncPyModule(DMSCPythonAsyncServiceModule):
    """Async service module"""
    async def init(self, ctx: DMSCServiceContext) -> None:
        """Async initialize module, return None for success"""
        ctx.logger().info("my_async_python_module", "Async Python module initialized")
    
    async def start(self, ctx: DMSCServiceContext) -> None:
        """Async start module, return None for success"""
        ctx.logger().info("my_async_python_module", "Async Python module started")
    
    async def shutdown(self, ctx: DMSCServiceContext) -> None:
        """Async shutdown module, return None for success"""
        ctx.logger().info("my_async_python_module", "Async Python module stopped")
```

Using custom modules in Python application:

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

app = DMSCAppBuilder() \
    .with_config("config.yaml") \
    .with_logging(DMSCLogConfig()) \
    .build()

app.run(lambda ctx: ctx.logger().info("service", "Python service started") or None)
```

<div align="center">

## 4. Lifecycle Management

DMSC applications and modules have clear lifecycles, ensuring proper initialization and resource release.

</div>

### 4.1 Application Lifecycle

1. **Build**: Configure and build the application through `DMSCAppBuilder`
2. **Initialize**: Initialize all registered modules
3. **Start**: Start all modules
4. **Run**: Execute user-provided business logic
5. **Stop**: Stop all modules
6. **Cleanup**: Release resources

### 4.2 Module Lifecycle

Each module goes through the following complete lifecycle stages:

1. **Initialization (init)**: Set up module resources, initialize internal state
2. **Before Start**: Prepare for module startup, perform pre-start checks
3. **Start**: Start module service, begin handling requests
4. **After Start**: Perform post-startup operations, such as registering services, sending notifications
5. **Running**: Module is in running state, handling requests and events
6. **Before Shutdown**: Prepare for shutdown, stop accepting new requests
7. **Shutdown**: Stop module service, release core resources
8. **After Shutdown**: Clean up all resources, such as closing connections, releasing memory

### 4.3 Lifecycle Hooks

DMSC provides lifecycle hooks, allowing you to execute custom logic at specific stages:

```rust
use dmsc::prelude::*;

// Register hooks when building the application
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .build()?;

// Get hook bus
let hooks = app.hooks();

// Register before start hook
hooks.register(DMSCHookKind::BeforeStart, |ctx| async move {
    ctx.logger().info("hooks", "Before start hook executed")?;
    Ok(())
});
```

Supported hook types:

- `BeforeInit`: Before initialization
- `AfterInit`: After initialization
- `BeforeStart`: Before startup
- `AfterStart`: After startup
- `BeforeShutdown`: Before shutdown
- `AfterShutdown`: After shutdown

<div align="center">

## 5. Error Handling Mechanism

DMSC adopts a unified error handling mechanism, ensuring consistency and completeness of error information.

</div>

### 5.1 Error Types

DMSC uses the `DMSCError` type to represent all errors, which includes:

- **Error Code**: Unique identifier for the error type
- **Error Message**: Detailed error description
- **Error Context**: Optional context information
- **Source Code Location**: File and line number where the error occurred
- **Internal Error**: Optional nested error

### 5.2 Result Type

DMSC defines the `DMSCResult` type alias to simplify error handling:

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

### 5.3 Error Propagation

In async code, DMSC errors can be automatically propagated through the `?` operator:

```rust
async fn my_function(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    // Access cache through module system
    if let Ok(cache) = ctx.module::<DMSCCacheModule>().await {
        let cache_manager = cache.cache_manager();
        let value = cache_manager.get("key").await?;
    }
    Ok(())
}
```

### 5.4 Custom Errors

You can create custom errors and convert them to `DMSCError`:

```rust
use dmsc::core::error::{DMSCError, DMSCResult};

fn my_custom_error() -> DMSCResult<()> {
    Err(DMSCError::new("CUSTOM_ERROR", "This is a custom error"))
}
```

<div align="center">

## 6. Async Design

DMSC adopts an async-first design, fully utilizing the concurrency capabilities of modern hardware.

</div>

### 6.1 Async Runtime

DMSC is based on the Tokio async runtime, supporting high concurrency and non-blocking I/O operations.

### 6.2 Async Modules

For custom modules, you should use the `DMSCModule` trait, which is async and public:

```rust
use dmsc::core::module::DMSCModule;
use dmsc::core::context::DMSCServiceContext;
use dmsc::core::error::DMSCResult;

pub struct MyAsyncModule {
    // Module configuration
}

#[async_trait::async_trait]
impl DMSCModule for MyAsyncModule {
    fn name(&self) -> &str {
        "my_async_module"
    }
    
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        // Async initialization logic
        Ok(())
    }
    
    async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        // Async start logic
        Ok(())
    }
    
    async fn shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        // Async stop logic
        Ok(())
    }
}
```

### 6.3 Async APIs

Most DMSC APIs are async, using `async/await` syntax:

```rust
// Async file operations
ctx.fs().write_file("data.txt", "content").await?;
let content = ctx.fs().read_file("data.txt").await?;
```

<div align="center">

## 7. Observability Design

DMSC has built-in complete observability support, helping you monitor and debug applications.

</div>

### 7.1 Distributed Tracing

DMSC implements the W3C Trace Context standard, supporting cross-service distributed tracing:

```rust
use dmsc::observability::traced;

#[traced(name = "user_service")]
async fn get_user(ctx: &DMSCServiceContext, user_id: u64) -> DMSCResult<User> {
    // Automatically records tracing information
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

### 7.2 Metrics Collection

DMSC has built-in Prometheus metrics collection, supporting multiple metrics types:

- **Counter**: Monotonically increasing counter
- **Gauge**: Increments and decrements
- **Histogram**: Distribution histogram
- **Summary**: Quantile statistics

### 7.3 Log Integration

DMSC's logging system automatically includes trace context, facilitating correlation between logs and traces:

```json
{
  "timestamp": "2025-12-12T15:30:00Z",
  "level": "info",
  "module": "service",
  "message": "DMSC service started",
  "trace_id": "abc123",
  "span_id": "def456"
}
```

<div align="center">

## 8. Configuration Management

DMSC supports multi-source configuration management, allowing you to load configurations from different sources.

</div>

### 8.1 Configuration Source Priority

DMSC loads configurations in the following priority order (from lowest to highest):

1. **Configuration Files**: YAML, TOML, or JSON format configuration files (lowest priority)
2. **Custom Configuration**: Configurations set through code
3. **Environment Variables**: Environment variables prefixed with `DMSC_` (highest priority)

### 8.2 Configuration Hot Reload

DMSC supports configuration hot reload, allowing you to update configurations without restarting the application:

```yaml
config:
  watch_enabled: true
  watch_interval: 30s
```

### 8.3 Configuration Access

You can access configurations through the service context:

```rust
// Get string configuration
let service_name = ctx.config().get("service.name").unwrap_or("unknown");

// Get integer configuration
let port = ctx.config().get("service.port").unwrap_or(8080);

// Get boolean configuration
let enabled = ctx.config().get("feature.enabled").unwrap_or(false);
```

<div align="center">

## 9. Security Design

DMSC has built-in multiple security mechanisms to protect applications.

</div>

### 9.1 Secure File System

DMSC provides secure file system operations, preventing path traversal and other security issues:

```rust
// Secure file writing, preventing path traversal
ctx.fs().write_file("data.txt", "content").await?;

// Secure file reading
let content = ctx.fs().read_file("data.txt").await?;
```

### 9.2 Authentication and Authorization

DMSC's auth module provides complete authentication and authorization mechanisms:

- **JWT Authentication**: Supports JSON Web Token
- **OAuth2**: Supports OAuth2.0 protocol
- **Permission Management**: Role-based access control
- **API Keys**: Supports API key authentication

### 9.3 Secure Logging

DMSC's logging system automatically filters sensitive information, preventing leakage of confidential data:

```rust
// Sensitive information will be automatically filtered
ctx.logger().info("auth", &format!("User authenticated: {}", user_id))?;
```

<div align="center">

## 10. Performance Optimization

DMSC adopts multiple performance optimization techniques, ensuring high performance and low resource consumption.

</div>

### 10.1 Zero-Copy Design

For I/O-intensive operations, DMSC adopts a zero-copy design, reducing memory copy overhead.

### 10.2 Connection Pooling

DMSC provides connection pooling for databases, Redis, and other resources, reducing connection establishment and destruction overhead.

### 10.3 Async I/O

DMSC fully utilizes async I/O, reducing thread context switching overhead and improving concurrent processing capabilities.

### 10.4 Memory Management

DMSC adopts efficient memory management strategies, reducing memory allocation and garbage collection overhead.

<div align="center">

## Summary

</div>

DMSC's core design principles are:

- **Modularity**: Highly modular architecture, supporting on-demand composition
- **Async First**: Fully utilize modern hardware's concurrency capabilities
- **Observability**: Built-in complete monitoring and tracing support
- **Security**: Built-in multiple security mechanisms
- **Usability**: Provides simple APIs and good documentation
- **Extensibility**: Supports extending functionality through custom modules

Understanding these core concepts will help you better design and develop DMSC-based applications.

<div align="center">

## Next Steps

</div> 

- [API Reference](./04-api-reference/README.md): Detailed module API documentation
- [Usage Examples](./05-usage-examples/README.md): Usage examples for various scenarios
- [Best Practices](./06-best-practices.md): Best practices for developing DMSC applications
- [Troubleshooting](./07-troubleshooting.md): Common issues and solutions
- [Glossary](./08-glossary.md): Core terminology explanation