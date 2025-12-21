<div align="center">

# Getting Started with DMSC

**Version: 1.0.0**

**Last modified date: 2025-12-12**

This guide will walk you through the process of installing DMSC, creating your first DMSC application, and running it.

## Prerequisites

</div>

Before you begin, ensure you have the following installed:

- **Rust**: 1.65+ (2021 Edition)
- **Cargo**: 1.65+
- **Platforms**: Linux, macOS, Windows

<details>
<summary>Installing Rust</summary>

If you don't have Rust installed, follow these steps:

1. **Linux/macOS**:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Windows**:
   Download and run the [Rust installer](https://www.rust-lang.org/tools/install).

3. **Verify installation**:
   ```bash
   rustc --version
   cargo --version
   ```
</details>

<details>
<summary>Installing Required Dependencies</summary>

DMSC has some system dependencies that may need to be installed:

- **Linux**: `libssl-dev`, `pkg-config`, `build-essential`
  ```bash
  # Debian/Ubuntu
  sudo apt-get update
  sudo apt-get install libssl-dev pkg-config build-essential
  ```

- **macOS**: Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

- **Windows**: Microsoft Visual C++ Build Tools
  ```
  Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
  ```
</details>

<div align="center">

## Creating Your First DMSC Application

</div>

### 1. Create a New Rust Project

```bash
cargo new dms-first-app
cd dms-first-app
```

### 2. Add DMSC Dependency

Edit the `Cargo.toml` file to add DMSC as a dependency:

```toml
[package]
name = "dms-first-app"
version = "0.1.0"
edition = "2021"

[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
```

### 3. Create a Configuration File

Create a `config.yaml` file in the project root:

```yaml
service:
  name: "dms-first-app"
  version: "0.1.0"

logging:
  level: "info"
  format: "json"
  console_enabled: true
  file_enabled: false

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### 4. Write the Main Application Code

Replace the content of `src/main.rs` with the following:

```rust
use dms::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build the DMSC application
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // Run the application with custom business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        // Access service context
        let service_name = ctx.config().config().get_str("service.name").unwrap_or("unknown");
        let service_version = ctx.config().config().get_str("service.version").unwrap_or("unknown");
        
        // Log application startup
        ctx.logger().info(
            "service", 
            &format!("{} v{} started successfully", service_name, service_version)
        )?;
        
        // Simulate some business logic
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        ctx.logger().info("service", "Business logic completed")?;
        
        Ok(())
    }).await
}
```

<details>
<summary>Code Explanation</summary>

1. **Import DMSC Prelude**: Imports commonly used DMSC types and traits.
2. **Main Function**: Async main function using Tokio runtime.
3. **Application Builder**: Creates and configures a DMSC application.
   - `with_config("config.yaml")`: Loads configuration from file.
   - `with_logging(DMSCLogConfig::default())`: Enables logging.
   - `with_observability(DMSCObservabilityConfig::default())`: Enables observability.
4. **Application Runtime**: Runs the application with custom business logic.
5. **Service Context**: Accesses configuration, logging, and other services.
6. **Business Logic**: Simple example with logging and sleep.
</details>

<div align="center">

## Running Your Application

</div>

### 1. Build the Application

```bash
cargo build
```

### 2. Run the Application

```bash
cargo run
```

### 3. Expected Output

```json
{"timestamp":"2025-12-12T10:00:00Z","level":"info","module":"service","message":"dms-first-app v0.1.0 started successfully"}
{"timestamp":"2025-12-12T10:00:05Z","level":"info","module":"service","message":"Business logic completed"}
```

<details>
<summary>Monitoring Metrics</summary>

With observability enabled, you can access metrics at `http://localhost:9090/metrics`.

```bash
# Test metrics endpoint
curl http://localhost:9090/metrics
```
</details>

<details>
<summary>Running with Different Configuration</summary>

You can override configuration with environment variables:

```bash
# Set log level to debug
DMSC_LOGGING_LEVEL=debug cargo run
```
</details>

<details>
<summary>Running in Release Mode</summary>

For production, run in release mode for better performance:

```bash
cargo run --release
```
</details>

<div align="center">

## Next Steps

</div>

Now that you've created your first DMSC application, you can:

1. **Explore Core Concepts**: Learn about DMSC's architecture and design philosophy
   - [Core Concepts](./03-core-concepts.md)

2. **Study API Reference**: Learn about the available APIs and services
   - [API Reference](./04-api-reference/README.md)

3. **View Usage Examples**: See practical examples for various scenarios
   - [Usage Examples](./05-usage-examples/README.md)

4. **Learn Best Practices**: Follow recommended practices for developing DMSC applications
   - [Best Practices](./06-best-practices.md)

<details>
<summary>Common Issues and Solutions</summary>

**Issue**: Build failure due to missing system dependencies.
**Solution**: Install the required system dependencies as mentioned in prerequisites.

**Issue**: Configuration file not found.
**Solution**: Ensure the config file is in the correct location or provide the full path.

**Issue**: Permission denied when accessing files.
**Solution**: Check file permissions and ensure the application has access.

**Issue**: Port already in use for metrics.
**Solution**: Change the `prometheus_port` in the config file.
</details>

<details>
<summary>Useful Cargo Commands</summary>

```bash
# Check for errors without building
cargo check

# Build with verbose output
cargo build -v

# Run with backtrace for debugging
RUST_BACKTRACE=1 cargo run

# Run tests
cargo test

# Generate documentation
cargo doc --open
```
</details>

<div align="center">

## Resources

</div>

- [GitHub/Gitee Repository](https://gitee.com/dunimd/dmsc)
- [API Reference](./04-api-reference/README.md)
- [Usage Examples](./05-usage-examples/README.md)
- [Best Practices](./06-best-practices.md)
- [Troubleshooting](./07-troubleshooting.md)

Congratulations! You've successfully created and run your first DMSC application. Now you can explore more features and build more complex applications using DMSC's modular architecture.