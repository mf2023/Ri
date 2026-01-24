<div align="center">

# Usage Examples

**Version: 0.1.5**

**Last modified date: 2026-01-24**

This directory contains usage examples for each core feature of DMSC, helping you quickly understand and use the DMSC framework.

## Example List

</div>

| Example | Description | File |
|:--------|:-------------|:--------|
| **Basic Application** | Building a simple DMSC application | [basic-app.md](./basic-app.md) |
| **Authentication and Authorization** | Using JWT and OAuth for authentication | [authentication.md](./authentication.md) |
| **Cache Usage** | Basic operations and advanced usage of cache | [caching.md](./caching.md) |
| **Database Operations** | Database connection, querying, and transaction management | [database.md](./database.md) |
| **gRPC Service** | High-performance RPC with service registry and Python bindings | [grpc.md](./grpc.md) |
| **HTTP Service** | Building web applications and RESTful APIs | [http.md](./http.md) |
| **Message Queue** | Asynchronous message processing and event-driven architecture | [mq.md](./mq.md) |
| **Observability** | Distributed tracing, metrics collection, and monitoring | [observability.md](./observability.md) |
| **Security Practices** | Encryption, hashing, and security best practices | [security.md](./security.md) |
| **Storage Management** | File upload/download and storage management | [storage.md](./storage.md) |
| **Data Validation** | Data validation, cleaning, and custom validators | [validation.md](./validation.md) |
| **WebSocket** | Real-time bidirectional communication with Python bindings | [ws.md](./ws.md) |

<div align="center">

## Usage Guide

</div>

Each example document contains the following sections:

1. **Example Overview**: Purpose and functionality of the example
2. **Prerequisites**: Environment and dependencies required to run the example
3. **Example Code**: Complete example code
4. **Code Analysis**: Detailed explanation of the example code
5. **Running Steps**: How to run the example
6. **Expected Results**: Expected output after running the example

<div align="center">

## Example Structure

</div>

All examples follow this structure:

```rust
// 1. Import necessary dependencies
use dmsc::prelude::*;

// 2. Main function
#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 3. Build application
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        // Other configurations
        .build()?;
    
    // 4. Run application
    app.run(|ctx| async move {
        // 5. Business logic
        Ok(())
    }).await
}
```

<div align="center">

## Running Examples

</div>

### 1. Clone the DMSC Repository

```bash
git clone https://github.com/mf2023/DMSC.git
cd dmsc
```

### 2. Create an Example Project

```bash
cargo new dms-example
cd dms-example
```

### 3. Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
```

### 4. Write Example Code

Copy the example code to the `src/main.rs` file.

### 5. Run the Example

```bash
cargo run
```

<div align="center">

## Customizing Examples

</div>

You can modify the example code as needed to explore various features of DMSC:

1. **Add or remove modules**: Adjust application configuration based on requirements
2. **Modify business logic**: Implement your own business functionality
3. **Adjust configuration**: Modify configuration files or runtime parameters
4. **Add custom modules**: Extend DMSC functionality

## Best Practices

1. **Start with simple examples**: Run basic examples first, then try complex features
2. **Understand code logic**: Carefully read code analysis to understand the role of each component
3. **Gradually expand**: Gradually add new features to the basic example
4. **Check API documentation**: Refer to corresponding API documentation when encountering questions
5. **Test and debug**: Use DMSC's observability features for testing and debugging

<div align="center">

## Next Steps

</div>

Select the example you're interested in, run and modify it according to the instructions to deeply understand DMSC's features and usage.

- [Best Practices](./06-best-practices.md): Best practices for developing DMSC applications
- [Troubleshooting](./07-troubleshooting.md): Common issues and solutions
- [Glossary](./08-glossary.md): Core terminology explanations
