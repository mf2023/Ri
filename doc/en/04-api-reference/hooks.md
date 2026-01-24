<div align="center">

# Hooks API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-24**

The hooks module provides lifecycle event hook system, supporting custom logic execution at critical moments like application startup and shutdown.

## Module Overview

</div>

The hooks module, based on the event bus pattern, provides the following features:

- **Lifecycle hooks**: Support for application startup, shutdown, and other lifecycle events
- **Module phase hooks**: Support for module initialization, startup, shutdown, and other phases
- **Event-driven architecture**: Loose coupling component communication mechanism
- **Flexible event handling**: Support for synchronous and asynchronous handling

<div align="center">

## Core Components

</div>

### DMSCHookBus

Hook event bus, managing hook registration and triggering.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create hook bus | None | `Self` |
| `register(kind, id, handler)` | Register hook handler | `kind: DMSCHookKind`, `id: DMSCHookId`, `handler: F` | `()` |
| `emit(kind, ctx)` | Trigger hook event | `kind: &DMSCHookKind`, `ctx: &DMSCServiceContext` | `DMSCResult<()>` |
| `emit_with(kind, ctx, module, phase)` | Trigger hook with module info | `kind: &DMSCHookKind`, `ctx: &DMSCServiceContext`, `module: Option<&str>`, `phase: Option<DMSCModulePhase>` | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookHandler};

fn example() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "example.startup".to_string(),
        |ctx, event| {
            println!("Application starting up!");
            println!("Hook kind: {:?}", event.kind);
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "example.shutdown".to_string(),
        |ctx, event| {
            println!("Application shutting down!");
            Ok(())
        }
    );
    
    let ctx = DMSCServiceContext::new();
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    Ok(())
}
```

### DMSCHookKind

Hook type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Startup` | When application starts |
| `Shutdown` | When application shuts down |
| `BeforeModulesInit` | Before modules are initialized |
| `AfterModulesInit` | After modules are initialized |
| `BeforeModulesStart` | Before modules start |
| `AfterModulesStart` | After modules start |
| `BeforeModulesShutdown` | Before modules shut down |
| `AfterModulesShutdown` | After modules shut down |

### DMSCModulePhase

Module lifecycle phase enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Init` | Synchronous initialization phase |
| `BeforeStart` | Synchronous before-start phase |
| `Start` | Synchronous start phase |
| `AfterStart` | Synchronous after-start phase |
| `BeforeShutdown` | Synchronous before-shutdown phase |
| `Shutdown` | Synchronous shutdown phase |
| `AfterShutdown` | Synchronous after-shutdown phase |
| `AsyncInit` | Asynchronous initialization phase |
| `AsyncBeforeStart` | Asynchronous before-start phase |
| `AsyncStart` | Asynchronous start phase |
| `AsyncAfterStart` | Asynchronous after-start phase |
| `AsyncBeforeShutdown` | Asynchronous before-shutdown phase |
| `AsyncShutdown` | Asynchronous shutdown phase |
| `AsyncAfterShutdown` | Asynchronous after-shutdown phase |

### DMSCHookEvent

Hook event struct.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `kind` | `DMSCHookKind` | Hook type |
| `module` | `Option<String>` | Associated module name |
| `phase` | `Option<DMSCModulePhase>` | Module phase |

<div align="center">

## Hook Registration

</div>

### Basic Hook Registration

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookEvent};
use dmsc::core::DMSCServiceContext;

let mut hook_bus = DMSCHookBus::new();

hook_bus._register(
    DMSCHookKind::Startup,
    "my_module.startup".to_string(),
    |ctx: &DMSCServiceContext, event: &DMSCHookEvent| {
        println!("My module is starting up");
        Ok(())
    }
);
```

### Module Lifecycle Hooks

```rust
hook_bus._register(
    DMSCHookKind::BeforeModulesInit,
    "my_module.before_init".to_string(),
    |ctx, event| {
        println!("Before module initialization");
        Ok(())
    }
);

hook_bus._register(
    DMSCHookKind::AfterModulesShutdown,
    "my_module.after_shutdown".to_string(),
    |ctx, event| {
        println!("After module shutdown");
        Ok(())
    }
);
```

### Hooks with Module Information

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase};

hook_bus._emit_with(
    &DMSCHookKind::BeforeModulesInit,
    &ctx,
    Some("auth_module"),
    Some(DMSCModulePhase::Init)
)?;
```

<div align="center">

## Practical Use Cases

</div>

### Application Startup Initialization

```rust
use dmsc::prelude::*;
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};

fn setup_startup_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::Startup,
        "db.initialize".to_string(),
        |ctx, _event| {
            println!("Initializing database connection...");
            // ctx.database().connect()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "cache.warmup".to_string(),
        |ctx, _event| {
            println!("Warming up cache...");
            // ctx.cache().warm_up()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "metrics.start".to_string(),
        |ctx, _event| {
            println!("Starting metrics collection...");
            // ctx.metrics().start()?;
            Ok(())
        }
    );
}
```

### Resource Cleanup

```rust
fn setup_shutdown_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "cache.flush".to_string(),
        |ctx, _event| {
            println!("Flushing cache to persistent storage...");
            // ctx.cache().flush()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "db.disconnect".to_string(),
        |ctx, _event| {
            println!("Disconnecting from database...");
            // ctx.database().disconnect()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "metrics.stop".to_string(),
        |ctx, _event| {
            println!("Stopping metrics collection...");
            // ctx.metrics().stop()?;
            Ok(())
        }
    );
}
```

### Module Dependency Management

```rust
fn setup_dependency_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::BeforeModulesInit,
        "dependencies.check".to_string(),
        |ctx, event| {
            println!("Checking module dependencies...");
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "dependencies.ready".to_string(),
        |ctx, event| {
            println!("All dependencies are ready");
            Ok(())
        }
    );
}
```

<div align="center">

## Error Handling

</div>

Errors during hook execution propagate upwards:

```rust
use dmsc::core::DMSCError;

hook_bus._register(
    DMSCHookKind::Startup,
    "critical.startup".to_string(),
    |ctx, event| {
        match ctx.database().connect() {
            Ok(_) => Ok(()),
            Err(e) => Err(DMSCError::Other("Database connection failed".to_string()))
        }
    }
);
```

<div align="center">

## Best Practices

</div>

1. **Keep hooks simple**: Hook functions should execute quickly and avoid long blocking
2. **Handle errors**: Properly handle errors in hooks to avoid affecting application startup
3. **Use meaningful IDs**: Use descriptive IDs for hooks to facilitate debugging
4. **Avoid circular dependencies**: Be aware of dependencies between hooks to avoid cycles
5. **Log operations**: Log key operations in hooks for easier troubleshooting
6. **Register as needed**: Only register necessary hooks to avoid unnecessary overhead

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [http](./http.md): HTTP module providing HTTP server and client functionality
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
