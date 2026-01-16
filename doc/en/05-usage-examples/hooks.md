# Hooks System Usage Guide

This document provides comprehensive usage examples for the DMSC Hooks System, demonstrating how to leverage the event-driven architecture for module lifecycle management.

## Table of Contents

1. [Basic Hook Bus Operations](#basic-hook-bus-operations)
2. [Registering Hook Handlers](#registering-hook-handlers)
3. [Emitting Hook Events](#emitting-hook-events)
4. [Lifecycle Event Handling](#lifecycle-event-handling)
5. [Advanced Usage](#advanced-usage)
6. [Complete Example](#complete-example)

---

## Basic Hook Bus Operations

The hook bus serves as the central event management system for DMSC. It allows components to register handlers for specific lifecycle events and emit events when those events occur.

### Creating a Hook Bus

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};

// Create a new hook bus instance
let hook_bus = DMSCHookBus::new();
```

The hook bus is created empty with no registered handlers. You can immediately start registering handlers for various hook kinds.

### Hook Kinds Reference

DMSC supports the following hook kinds for application lifecycle management:

| Hook Kind | Description |
|-----------|-------------|
| `Startup` | Emitted when the application starts up |
| `Shutdown` | Emitted when the application shuts down |
| `BeforeModulesInit` | Emitted before modules are initialized |
| `AfterModulesInit` | Emitted after modules are initialized |
| `BeforeModulesStart` | Emitted before modules are started |
| `AfterModulesStart` | Emitted after modules are started |
| `BeforeModulesShutdown` | Emitted before modules are shut down |
| `AfterModulesShutdown` | Emitted after modules are shut down |

---

## Registering Hook Handlers

Hook handlers are closures that execute when a specific hook event is emitted. Each handler must conform to the `DMSCHookHandler` type signature.

### Simple Handler Registration

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_basic_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register a startup handler
    hook_bus._register(
        DMSCHookKind::Startup,
        "logger.startup".to_string(),
        |_ctx, _event| {
            println!("Application startup detected");
            Ok(())
        },
    );
    
    // Register a shutdown handler
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "logger.shutdown".to_string(),
        |_ctx, _event| {
            println!("Application shutdown detected");
            Ok(())
        },
    );
    
    Ok(())
}
```

### Handler with Contextual Information

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookEvent};
use dmsc::prelude::*;

fn register_contextual_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register a handler that logs event details
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "logger.module_init".to_string(),
        |ctx, event| {
            println!("Module initialized: {:?}", event.module);
            println!("Service context: {:?}", ctx);
            Ok(())
        },
    );
    
    Ok(())
}
```

### Module-Specific Handlers

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_module_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register a handler for a specific module
    hook_bus._register(
        DMSCHookKind::BeforeModulesStart,
        "cache.warmup".to_string(),
        |_ctx, event| {
            if let Some(module_name) = &event.module {
                if module_name == "cache" {
                    println!("Warming up cache module");
                    // Perform cache warmup operations
                }
            }
            Ok(())
        },
    );
    
    Ok(())
}
```

---

## Emitting Hook Events

The hook bus provides two methods for emitting events: `emit()` for simple events and `emit_with()` for events with additional context.

### Basic Event Emission

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn emit_basic_event() -> DMSCResult<()> {
    let hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new();
    
    // Emit a startup event
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    // Emit a shutdown event
    hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
    
    Ok(())
}
```

### Event Emission with Context

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase};
use dmsc::prelude::*;

fn emit_contextual_event() -> DMSCResult<()> {
    let hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new();
    
    // Emit an event with module and phase information
    hook_bus._emit_with(
        &DMSCHookKind::BeforeModulesStart,
        &ctx,
        Some("database"),
        Some(DMSCModulePhase::BeforeStart),
    )?;
    
    Ok(())
}
```

### Module Phase Reference

DMSC supports both synchronous and asynchronous module phases:

**Synchronous Phases:**
- `Init` - Synchronous initialization phase
- `BeforeStart` - Phase before starting
- `Start` - Start phase
- `AfterStart` - Phase after starting
- `BeforeShutdown` - Phase before shutting down
- `Shutdown` - Shutdown phase
- `AfterShutdown` - Phase after shutting down

**Asynchronous Phases:**
- `AsyncInit` - Asynchronous initialization phase
- `AsyncBeforeStart` - Asynchronous phase before starting
- `AsyncStart` - Asynchronous start phase
- `AsyncAfterStart` - Asynchronous phase after starting
- `AsyncBeforeShutdown` - Asynchronous phase before shutting down
- `AsyncShutdown` - Asynchronous shutdown phase
- `AsyncAfterShutdown` - Asynchronous phase after shutting down

---

## Lifecycle Event Handling

This section demonstrates comprehensive lifecycle event handling patterns.

### Application Startup Sequence

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

async fn handle_startup_sequence() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register startup handlers
    hook_bus._register(
        DMSCHookKind::Startup,
        "config.load".to_string(),
        |_ctx, _event| {
            println!("Loading configuration");
            Ok(())
    
    hook_bus        },
    );
._register(
        DMSCHookKind::BeforeModulesInit,
        "resource.allocate".to_string(),
        |_ctx, _event| {
            println!("Allocating system resources");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "service.register".to_string(),
        |_ctx, _event| {
            println!("Registering services");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesStart,
        "health.check".to_string(),
        |_ctx, _event| {
            println!("Performing initial health check");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // Emit lifecycle events in sequence
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    hook_bus.emit(&DMSCHookKind::BeforeModulesInit, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesInit, &ctx)?;
    hook_bus.emit(&DMSCHookKind::BeforeModulesStart, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesStart, &ctx)?;
    
    Ok(())
}
```

### Application Shutdown Sequence

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

async fn handle_shutdown_sequence() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register shutdown handlers
    hook_bus._register(
        DMSCHookKind::BeforeModulesShutdown,
        "cache.flush".to_string(),
        |_ctx, _event| {
            println!("Flushing cache to disk");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::BeforeModulesShutdown,
        "connection.close".to_string(),
        |_ctx, _event| {
            println!("Closing active connections");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesShutdown,
        "resource.release".to_string(),
        |_ctx, _event| {
            println!("Releasing system resources");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "logger.close".to_string(),
        |_ctx, _event| {
            println!("Closing logger");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // Emit shutdown events in sequence
    hook_bus.emit(&DMSCHookKind::BeforeModulesShutdown, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesShutdown, &ctx)?;
    hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
    
    Ok(())
}
```

---

## Advanced Usage

### Error Handling in Hooks

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_error_handling_hooks() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register handler that can return errors
    hook_bus._register(
        DMSCHookKind::Startup,
        "critical.startup".to_string(),
        |_ctx, _event| {
            // Simulate potential failure
            let startup_success = true;
            if !startup_success {
                return Err(DMSCError::HookError(
                    "Critical startup failed".to_string()
                ));
            }
            Ok(())
        },
    );
    
    // Register error recovery handler
    hook_bus._register(
        DMSCHookKind::Startup,
        "error.recovery".to_string(),
        |ctx, event| {
            // Check if previous handler failed
            println!("Recovery handler executed for: {:?}", event.kind);
            Ok(())
        },
    );
    
    Ok(())
}
```

### Conditional Handler Execution

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_conditional_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Register a handler that only executes under certain conditions
    hook_bus._register(
        DMSCHookKind::AfterModulesStart,
        "monitoring.enable".to_string(),
        |_ctx, event| {
            // Only enable monitoring for specific modules
            if let Some(module) = &event.module {
                if module == "api-server" || module == "websocket-server" {
                    println!("Enabling monitoring for: {}", module);
                }
            }
            Ok(())
        },
    );
    
    Ok(())
}
```

### Multiple Handlers for Same Hook

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_multiple_handlers() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // Multiple handlers for the same hook kind execute in registration order
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.1".to_string(),
        |_ctx, _event| {
            println!("Handler 1 executed");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.2".to_string(),
        |_ctx, _event| {
            println!("Handler 2 executed");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.3".to_string(),
        |_ctx, _event| {
            println!("Handler 3 executed");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // All three handlers will be executed
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    Ok(())
}
```

---

## Complete Example

The following example demonstrates a complete integration of the hooks system:

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase, DMSCHookEvent};
use dmsc::prelude::*;

struct Application {
    hook_bus: DMSCHookBus,
    is_running: bool,
}

impl Application {
    fn new() -> Self {
        let mut hook_bus = DMSCHookBus::new();
        
        // Initialize application lifecycle handlers
        Self::register_lifecycle_handlers(&mut hook_bus);
        Self::register_module_handlers(&mut hook_bus);
        Self::register_monitoring_handlers(&mut hook_bus);
        
        Self {
            hook_bus,
            is_running: false,
        }
    }
    
    fn register_lifecycle_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::Startup,
            "app.config".to_string(),
            |_ctx, _event| {
                println!("[STARTUP] Loading application configuration");
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::Shutdown,
            "app.cleanup".to_string(),
            |_ctx, _event| {
                println!("[SHUTDOWN] Performing final cleanup");
                Ok(())
            },
        );
    }
    
    fn register_module_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::BeforeModulesInit,
            "module.validate".to_string(),
            |_ctx, event| {
                if let Some(module) = &event.module {
                    println!("[MODULE] Validating module: {}", module);
                }
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::AfterModulesInit,
            "module.register".to_string(),
            |_ctx, event| {
                if let Some(module) = &event.module {
                    println!("[MODULE] Module registered: {}", module);
                }
                Ok(())
            },
        );
    }
    
    fn register_monitoring_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::AfterModulesStart,
            "monitor.start".to_string(),
            |_ctx, _event| {
                println!("[MONITOR] Starting performance monitoring");
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::BeforeModulesShutdown,
            "monitor.stop".to_string(),
            |_ctx, _event| {
                println!("[MONITOR] Stopping performance monitoring");
                Ok(())
            },
        );
    }
    
    async fn start(&mut self) -> DMSCResult<()> {
        let ctx = DMSCServiceContext::new();
        
        println!("Starting application lifecycle");
        
        self.hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
        
        let modules = vec!["config", "cache", "database", "api"];
        for module in &modules {
            self.hook_bus._emit_with(
                &DMSCHookKind::BeforeModulesInit,
                &ctx,
                Some(module),
                Some(DMSCModulePhase::Init),
            )?;
            
            self.hook_bus._emit_with(
                &DMSCHookKind::AfterModulesInit,
                &ctx,
                Some(module),
                Some(DMSCModulePhase::AfterStart),
            )?;
        }
        
        self.hook_bus.emit(&DMSCHookKind::AfterModulesStart, &ctx)?;
        
        self.is_running = true;
        println!("Application started successfully");
        
        Ok(())
    }
    
    async fn stop(&mut self) -> DMSCResult<()> {
        if !self.is_running {
            return Ok(());
        }
        
        let ctx = DMSCServiceContext::new();
        
        println!("Stopping application lifecycle");
        
        self.hook_bus.emit(&DMSCHookKind::BeforeModulesShutdown, &ctx)?;
        self.hook_bus.emit(&DMSCHookKind::AfterModulesShutdown, &ctx)?;
        self.hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
        
        self.is_running = false;
        println!("Application stopped successfully");
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let mut app = Application::new();
    
    app.start().await?;
    
    // Simulate application running
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    app.stop().await?;
    
    Ok(())
}
```

### Expected Output

```
Starting application lifecycle
[STARTUP] Loading application configuration
[MODULE] Validating module: config
[MODULE] Module registered: config
[MODULE] Validating module: cache
[MODULE] Module registered: cache
[MODULE] Validating module: database
[MODULE] Module registered: database
[MODULE] Validating module: api
[MODULE] Module registered: api
[MONITOR] Starting performance monitoring
Application started successfully
Stopping application lifecycle
[MONITOR] Stopping performance monitoring
[SHUTDOWN] Performing final cleanup
Application stopped successfully
```

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with usage examples summary and quick navigation
- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [basic-app](./basic-app.md): Basic application examples
- [caching](./caching.md): Caching examples, including memory and distributed caching
- [database](./database.md): Database operation examples
- [device](./device.md): Device control examples
- [fs](./fs.md): Filesystem operation examples
- [gateway](./gateway.md): API gateway examples
- [http](./http.md): HTTP server and client examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples
- [observability](./observability.md): Observability examples
- [protocol](./protocol.md): Protocol module examples
- [security](./security.md): Security and encryption examples
- [service_mesh](./service_mesh.md): Service mesh examples
- [storage](./storage.md): Cloud storage examples
- [validation](./validation.md): Data validation examples
