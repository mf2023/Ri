<div align="center">

# API Reference

**Version: 0.1.7**

**Last modified date: 2026-02-13**

This directory contains detailed API documentation for each core module of DMSC.

## Module List

</div>

| Module | Description | File |
|:--------|:-------------|:--------|
| **auth** | Authentication and authorization (JWT, OAuth, permissions) | [auth.md](./auth.md) |
| **cache** | Multi-backend cache abstraction (Memory, Redis, Hybrid) | [cache.md](./cache.md) |
| **config** | Multi-source configuration management with hot reload | [config.md](./config.md) |
| **core** | Runtime, error handling, and service context | [core.md](./core.md) |
| **database** | Database access layer with ORM support | [database.md](./database.md) |
| **device** | Device control, discovery, and intelligent scheduling | [device.md](./device.md) |
| **fs** | Secure file system operations and management | [fs.md](./fs.md) |
| **gateway** | API gateway with HTTP server, routing, middleware, load balancing, rate limiting, and circuit breaking | [gateway.md](./gateway.md) |
| **grpc** | High-performance RPC with service registry and Python bindings | [grpc.md](./grpc.md) |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) | [hooks.md](./hooks.md) |
| **log** | Structured logging with tracing context integration | [log.md](./log.md) |
| **module_rpc** | Inter-module RPC communication (service registration, invocation, load balancing) | [module_rpc.md](./module_rpc.md) |
| **observability** | Metrics, tracing, and Grafana integration | [observability.md](./observability.md) |
| **protocol** | Communication protocol support with encryption, post-quantum crypto, and HSM | [protocol.md](./protocol.md) |
| **queue** | Message queue abstraction (Kafka, RabbitMQ, Redis, Memory) | [queue.md](./queue.md) |
| **service_mesh** | Service discovery, health checking, and traffic management | [service_mesh.md](./service_mesh.md) |
| **validation** | Data validation and sanitization | [validation.md](./validation.md) |
| **ws** | WebSocket real-time communication with Python bindings | [ws.md](./ws.md) |

<div align="center">

## Usage Guide

</div>

Each API document contains the following sections:

1. **Module Overview**: Main functionality and purpose of the module
2. **Core Components**: Key types and structs in the module
3. **API Reference**: Detailed method and function descriptions
4. **Usage Examples**: Code examples showing how to use the module
5. **Best Practices**: Recommendations and best practices for using the module

<div align="center">

## Naming Conventions

</div>

DMSC API follows these naming conventions:

- **Class Names**: Use PascalCase, e.g., `DMSCAppBuilder`
- **Method Names**: Use snake_case, e.g., `with_config`
- **Constants**: Use SCREAMING_SNAKE_CASE, e.g., `DEFAULT_PORT`
- **Type Aliases**: Use PascalCase, e.g., `DMSCResult`

<div align="center">

## Error Handling

</div>

All DMSC API methods return `DMSCResult<T>` type, where `T` is the method's return value type. `DMSCResult` is an alias for `Result<T, DMSCError>`.

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

<div align="center">

## Async APIs

</div>

Most DMSC APIs are asynchronous, using `async/await` syntax. Async methods require the `.await` keyword when called.

```rust
// Async method call
ctx.cache().set("key", "value", Some(3600)).await?;
```

<div align="center">

## Type Safety

</div>

DMSC API design emphasizes type safety, using strong typing to ensure compile-time checks.

```rust
// Type-safe configuration access
let port: u16 = ctx.config().config().get("service.port").unwrap_or(8080);
```

<div align="center">

## Next Steps

</div>

Select the module you're interested in to view its detailed API documentation.

- [Usage Examples](./05-usage-examples/README.md): Usage examples for various scenarios
- [Best Practices](./06-best-practices.md): Best practices for developing DMSC applications
- [Troubleshooting](./07-troubleshooting.md): Resolve common issues and faults
- [Glossary](./08-glossary.md): Understand the terminology used in DMSC