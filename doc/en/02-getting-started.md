<div align="center">

# Getting Started

**Version: 0.1.4**

**Last modified date: 2026-01-15**

This guide will help you get started with DMSC, from installation to creating your first application.

## Prerequisites

</div>

Before you begin, ensure your environment meets the following requirements:

- **Rust**: 1.65+ (2021 Edition)
- **Cargo**: 1.65+ (Rust package manager)
- **Platforms**: Linux, macOS, or Windows

You can check your Rust and Cargo versions with the following commands:

```bash
rustc --version
cargo --version
```

If you don't have Rust installed yet, you can install it via [rustup](https://rustup.rs/):

```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows
# Visit https://rustup.rs/ and download the installer
```

### Python Environment Requirements

If you use the Python SDK, ensure your environment meets the following requirements:

- **Python**: 3.8+
- **pip**: Latest version
- **Platforms**: Linux, macOS, or Windows

You can check your Python version with the following commands:

```bash
python --version
# or
python3 --version
```

If you don't have Python 3.8+ installed, please visit [Python Official Website](https://www.python.org/downloads/) to download and install.

<div align="center">

## Installing DMSC

</div>

### Using DMSC in a New Project

Create a new Rust project:

```bash
cargo new my-dms-app
cd my-dms-app
```

Add DMSC to your project's `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
```

Or use the `cargo add` command:

```bash
cargo add dms --git https://github.com/mf2023/DMSC
cargo add tokio --features full
```

### Using DMSC in an Existing Project

Simply add DMSC to your existing project's `Cargo.toml` file:

```toml
[dependencies]
# Other dependencies
dms = { git = "https://github.com/mf2023/DMSC" }
```

### Using Python SDK

The simplest way to install the Python SDK is via pip:

```bash
pip install dmsc
```

Or add to your `requirements.txt`:

```
dmsc==0.1.4
```

Verify the installation:

```python
import dmsc
print(f"DMSC Python SDK Version: {dmsc.__version__}")
```

<div align="center">

## Your First DMSC Application

</div>

Now, let's create a simple DMSC application.

### Basic Application Structure

Open the `src/main.rs` file and replace it with the following content:

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build the service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        // Your business code here
        Ok(())
    }).await
}
```

### Configuration File

Create a `config.yaml` file in the project root directory:

```yaml
# config.yaml
service:
  name: "my-dms-app"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### Running the Application

Use Cargo to run the application:

```bash
cargo run
```

You should see output similar to the following:

```
2025-12-12T15:30:00Z INFO service: DMSC service started
```

### Your First Python Application

Creating an application with Python SDK is equally simple:

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

# Build the service runtime
app = DMSCAppBuilder() \
    .with_config("config.yaml") \
    .with_logging(DMSCLogConfig.default()) \
    .build()

# Run business logic
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

### Configuration File

Create a `config.yaml` file in the project root directory (shared by Rust and Python):

```yaml
# config.yaml
service:
  name: "my-dms-app"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

### Running Python Application

```bash
python main.py
```

You should see output similar to the following:

```
2025-12-12T15:30:00Z INFO service: DMSC service started
```

<div align="center">

## Application Structure Breakdown

</div>  

Let's break down this simple DMSC application:

1. **Import DMSC Components**:
   ```rust
   use dmsc::prelude::*;
   ```
   This line imports the most commonly used types and traits from DMSC, simplifying code writing.

2. **Create Application Builder**:
   ```rust
   let app = DMSCAppBuilder::new()
   ```
   Use the builder pattern to create a DMSC application instance.

3. **Configure Application**:
   ```rust
   .with_config("config.yaml")?
   .with_logging(DMSCLogConfig::default())?
   .with_observability(DMSCObservabilityConfig::default())?
   ```
   - Add configuration file support
   - Enable logging functionality
   - Enable observability (metrics and tracing)

4. **Build Application**:
   ```rust
   .build()?
   ```
   Build the final application instance.

5. **Run Application**:
   ```rust
   app.run(|ctx: &DMSCServiceContext| async move {
       ctx.logger().info("service", "DMSC service started")?;
       Ok(())
   }).await
   ```
   - Use the `run` method to start the application
   - Pass a closure that receives a `DMSCServiceContext` instance
   - Write business logic inside the closure

<div align="center">

## Adding More Features

</div>  

### Adding Cache Support

Modify `Cargo.toml` to add Redis dependency (if you need to use Redis cache):

```toml
[dependencies]
# Other dependencies
redis = "0.23"
```

Modify the application code to add cache support:

```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .with_cache(DMSCCacheConfig::default())? // Add cache support
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        
        // Use cache
        let cache = ctx.cache();
        cache.set("key", "value", Some(3600)).await?;
        let value = cache.get("key").await?;
        ctx.logger().info("cache", &format!("Cache value: {:?}", value))?;
        
        Ok(())
    }).await
}
```

<div align="center">

## Running Tests

</div>  

DMSC provides a comprehensive test suite. You can run these tests to verify your installation:

```bash
# Clone the DMSC repository
git clone https://github.com/mf2023/DMSC.git
cd dms

# Run all tests
cargo test

# Run specific test module
cargo test cache

# Run with verbose output
cargo test -- --nocapture
```
<div align="center">

## FAQ

</div>

### Q: How to configure log level?
A: Set `logging.level` in the configuration file, supporting DEBUG/INFO/WARN/ERROR levels.

### Q: How to enable metrics export?
A: Set `observability.metrics_enabled: true` in the configuration file and configure `prometheus_port`.

### Q: How to extend DMSC?
A: Implement the `DMSCModule` trait and register it through `DMSCAppBuilder::with_module`.

### Q: How to handle async tasks?
A: Use `DMSCAppBuilder::with_async_module` to add async modules, the framework automatically handles async lifecycle.

<div align="center">

## Troubleshooting

</div>  

- **Compilation errors**: Ensure Rust version meets requirements, check dependency version compatibility.
- **Runtime errors**: Check configuration file path and content, view log output for detailed information.
- **Dependency conflicts**: Use the `cargo tree` command to view the dependency tree and resolve version conflicts.

<div align="center">

## Next Steps

</div>  

- [Core Concepts](./03-core-concepts.md): In-depth understanding of DMSC's design philosophy and core components
- [API Reference](./04-api-reference/README.md): Detailed module API documentation
- [Usage Examples](./05-usage-examples/README.md): Usage examples for various scenarios
- [Best Practices](./06-best-practices.md): Best practices for developing DMSC applications
- [Troubleshooting](./07-troubleshooting.md): Common issues and solutions
- [Glossary](./08-glossary.md): Core terminology explanation
