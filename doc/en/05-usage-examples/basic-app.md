<div align="center">

# Basic Application Example

**Version: 0.1.6**

**Last modified date: 2026-01-24**

This example shows how to build a simple DMSC application, including application configuration, running, and basic functionality usage.

## Example Overview

</div>

This example will create a basic DMSC application that implements the following features:

- Load configuration files
- Enable logging
- Enable observability
- Output a startup log

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge

<div align="center">

## Example Code

</div>

### 1. Create a Project

```bash
cargo new dms-basic-example
cd dms-basic-example
```

### 2. Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root:

```yaml
service:
  name: "dms-basic-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### 4. Write Main Code

Replace the content of `src/main.rs` with the following:

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
        // Get service name and version
        let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
        let service_version = ctx.config().config().get_str("service.version").unwrap_or("unknown");
        
        // Output startup log
        ctx.logger().info(
            "service", 
            &format!("{} v{} started successfully", service_name, service_version)
        )?;
        
        // Output configuration info
        let log_level = ctx.config().config().get_str("logging.level").unwrap_or("info");
        ctx.logger().info(
            "config", 
            &format!("Logging level: {}", log_level)
        )?;
        
        // Wait 3 seconds to simulate business running
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        ctx.logger().info("service", "Service finished successfully")?;
        
        Ok(())
    }).await
}
```
<div align="center">

## Code Explanation

</div>

### 1. Import Dependencies

```rust
use dmsc::prelude::*;
```

This line imports the most commonly used types and traits from DMSC, simplifying code writing. The `prelude` module contains core components needed for building DMSC applications.

### 2. Main Function

```rust
#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Code...
}
```

- `#[tokio::main]`: Converts the main function into an async function and executes it using the Tokio runtime
- `async fn main()`: Defines an asynchronous main function
- `-> DMSCResult<()>`: Returns DMSC result type for error handling

### 3. Build Application

```rust
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_observability(DMSCObservabilityConfig::default())?
    .build()?;
```

- `DMSCAppBuilder::new()`: Creates a new application builder
- `.with_config("config.yaml")?`: Loads the configuration file
- `.with_logging(DMSCLogConfig::default())?`: Enables logging with default configuration
- `.with_observability(DMSCObservabilityConfig::default())?`: Enables observability with default configuration
- `.build()?`: Builds the application runtime

### 4. Run Application

```rust
app.run(|ctx: &DMSCServiceContext| async move {
    // Business logic
}).await
```

- `app.run()`: Starts the application runtime
- `|ctx: &DMSCServiceContext|`: Closure parameter that receives the service context
- `async move`: Async closure that allows using `await` inside the closure

### 5. Business Logic

```rust
let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
let service_version = ctx.config().config().get_str("service.version").unwrap_or("unknown");

ctx.logger().info(
    "service", 
    &format!("{} v{} started successfully", service_name, service_version)
)?;
```

- `ctx.config().config().get_str()`: Gets a value from the configuration
- `ctx.logger().info()`: Records an info-level log
- `?`: Propagates errors

<div align="center">

## Running Steps

</div>

### 1. Build the Project

```bash
cargo build
```

### 2. Run the Project

```bash
cargo run
```

<div align="center">

## Expected Results

</div>

After running the example, you should see output similar to the following:

```json
{
  "timestamp": "2025-12-12T15:30:00Z",
  "level": "info",
  "module": "service",
  "message": "dms-basic-example v1.0.0 started successfully",
  "trace_id": "abc123",
  "span_id": "def456"
}
{
  "timestamp": "2025-12-12T15:30:00Z",
  "level": "info",
  "module": "config",
  "message": "Logging level: info",
  "trace_id": "abc123",
  "span_id": "def456"
}
{
  "timestamp": "2025-12-12T15:30:03Z",
  "level": "info",
  "module": "service",
  "message": "Service finished successfully",
  "trace_id": "abc123",
  "span_id": "def456"
}
```

<div align="center">

## Extending Functionality

</div>

### 1. Implementing Cache Support

**Note**: Cache module needs to be separately configured and enabled.

```rust
// Add cache support when building the application
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .build()?;

// Use cache in business logic
app.run(|ctx: &DMSCServiceContext| async move {
    // Get cache service if enabled
    if let Some(cache) = ctx.cache() {
        // Set cache
        cache.set("key", "value", 3600).await?;
        
        // Get cache
        let value: String = cache.get("key").await?;
        ctx.logger().info("cache", &format!("Cached value: {}", value))?;
    }
    
    Ok(())
}).await
```

### 2. Implementing Queue Support

```rust
// Add queue support when building the application
let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_queue(DMSCQueueConfig::default())?
    .build()?;

// Use queue in business logic
app.run(|ctx: &DMSCServiceContext| async move {
    // Send message to queue
    ctx.queue().publish("task_queue", json!({
        "task_id": "task-123",
        "task_type": "data_processing",
        "priority": 1,
    })).await?;
    
    ctx.logger().info("queue", "Task message sent to queue")?;
    
    Ok(())
}).await
```

### 3. Implementing File System Operations

```rust
// Use file system in business logic
app.run(|ctx: &DMSCServiceContext| async move {
    // Write to file
    ctx.fs().write_file("data/config.json", r#"{"setting": "value"}"#).await?;
    
    // Read from file
    let content = ctx.fs().read_file("data/config.json").await?;
    ctx.logger().info("fs", &format!("File content: {}", content))?;
    
    // Check if file exists
    let exists = ctx.fs().file_exists("data/config.json").await?;
    ctx.logger().info("fs", &format!("File exists: {}", exists))?;
    
    Ok(())
}).await
```

### 4. Implementing Custom Modules

```rust
// Define custom module
struct MyCustomModule {
    name: String,
}

impl MyCustomModule {
    async fn process_data(&self, data: &str) -> DMSCResult<String> {
        // Custom processing logic
        Ok(format!("Processed by {}: {}", self.name, data))
    }
}

// Add custom module when building the application
let custom_module = MyCustomModule {
    name: "MyProcessor".to_string(),
};

let app = DMSCAppBuilder::new()
    .with_config("config.yaml")?
    .with_logging(DMSCLogConfig::default())?
    .with_module(custom_module)?
    .build()?;

// Use custom module in business logic
app.run(|ctx: &DMSCServiceContext| async move {
    // Get custom module
    let processor = ctx.module::<MyCustomModule>()?;
    
    // Use custom functionality
    let result = processor.process_data("sample data").await?;
    ctx.logger().info("custom", &format!("Processing result: {}", result))?;
    
    Ok(())
}).await
```

<div align="center">

## Best Practices

</div>

1. **Start Simple**: Create a basic application first, ensure it runs correctly, then gradually add other modules

2. **Configure Logging Appropriately**: Adjust log levels based on environment, use `debug` for development and `info` or `warn` for production

3. **Use Configuration Files**: Place configuration information in files, avoiding hardcoding, for easier deployment across different environments

4. **Handle Errors Properly**: Use the `?` operator to propagate errors, ensuring errors are correctly handled

5. **Design Modularly**: Encapsulate business logic in functions or modules to keep code clean

6. **Test Driven Development**: Write unit tests and integration tests to ensure code quality

<div align="center">

## Summary

</div>

This example demonstrates how to build a simple DMSC application, including:

- Project creation and dependency addition
- Configuration file writing
- Application building and running
- Basic functionality usage

Through this example, you should have understood the basic structure and usage of DMSC applications. You can further explore other features of DMSC based on this foundation.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [authentication](./authentication.md): Authentication example, learn JWT, OAuth2, and RBAC authentication and authorization
- [caching](./caching.md): Caching example, understand how to use the cache module to improve application performance
- [database](./database.md): Database example, learn database connection and query operations
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [http](./http.md): HTTP service example, build web applications and RESTful APIs
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue example, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability example, monitor application performance and health status
- [security](./security.md): Security example, encryption, hashing, and security best practices
- [storage](./storage.md): Storage example, file upload/download and storage management
- [validation](./validation.md): Validation example, data validation and sanitization operations
