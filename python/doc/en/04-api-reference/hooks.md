<div align="center">

# Hooks API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The hooks module provides an event bus system, allowing components to communicate during various lifecycle events.

</div>

## Module Overview

The hooks module contains the following core components:

- **DMSCHookBus**: Hook event bus for registering and triggering hooks
- **DMSCHookEvent**: Hook event representation
- **DMSCHookKind**: Hook type enumeration
- **DMSCModulePhase**: Module lifecycle phase enumeration

<div align="center">

## Core Components

</div>

### DMSCHookKind

Hook type enumeration, defining different types of hooks.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `STARTUP` | Startup hook |
| `SHUTDOWN` | Shutdown hook |
| `BEFORE_MODULES_INIT` | Before module initialization |
| `AFTER_MODULES_INIT` | After module initialization |
| `BEFORE_MODULES_START` | Before module startup |
| `AFTER_MODULES_START` | After module startup |
| `BEFORE_MODULES_SHUTDOWN` | Before module shutdown |
| `AFTER_MODULES_SHUTDOWN` | After module shutdown |

### DMSCModulePhase

Module lifecycle phase enumeration.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `INIT` | Initialization phase |
| `BEFORE_START` | Pre-start phase |
| `START` | Startup phase |
| `AFTER_START` | Post-start phase |
| `BEFORE_SHUTDOWN` | Pre-shutdown phase |
| `SHUTDOWN` | Shutdown phase |
| `AFTER_SHUTDOWN` | Post-shutdown phase |
| `ASYNC_INIT` | Asynchronous initialization |

### DMSCHookBus

Hook event bus for registering and triggering hooks.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `register(kind, handler, priority)` | Register a hook handler | `kind: DMSCHookKind`, `handler: Callable`, `priority: int` | `str` |
| `unregister(handler_id)` | Unregister a hook handler | `handler_id: str` | `bool` |
| `trigger(event)` | Trigger a hook event | `event: DMSCHookEvent` | `None` |
| `trigger_sync(event)` | Trigger hook event synchronously | `event: DMSCHookEvent` | `List[Result]` |
| `trigger_async(event)` | Trigger hook event asynchronously | `event: DMSCHookEvent` | `List[Result]` |
| `get_registered(kind)` | Get all handlers for a hook type | `kind: DMSCHookKind` | `List[Handler]` |

#### Usage Example

```python
from dmsc import (
    DMSCHookBus, DMSCHookKind, DMSCHookEvent,
    DMSCModulePhase, DMSCServiceContext
)

# Create hook bus
hook_bus = DMSCHookBus()

# Define hook handler
async def on_startup_handler(ctx: DMSCServiceContext, event: DMSCHookEvent):
    ctx.logger.info("hooks", "Application starting up")
    # Perform startup operations
    return {"status": "success"}

async def on_shutdown_handler(ctx: DMSCServiceContext, event: DMSCHookEvent):
    ctx.logger.info("hooks", "Application shutting down")
    # Perform cleanup operations
    return {"status": "success"}

# Register handlers
handler_id = hook_bus._register(
    DMSCHookKind.STARTUP,
    on_startup_handler,
    priority=100
)

hook_bus._register(
    DMSCHookKind.SHUTDOWN,
    on_shutdown_handler,
    priority=100
)
```

### DMSCHookEvent

Hook event representation.

#### Properties

| Property | Type | Description |
|:---------|:-----|:------------|
| `kind` | `DMSCHookKind` | Type of hook |
| `phase` | `DMSCModulePhase` | Current module phase |
| `timestamp` | `datetime` | Event timestamp |
| `data` | `Dict` | Event data |
| `source` | `str` | Event source module |

#### Usage Example

```python
from dmsc import DMSCHookEvent, DMSCHookKind, DMSCModulePhase
from datetime import datetime

# Create hook event
event = DMSCHookEvent(
    kind=DMSCHookKind.STARTUP,
    phase=DMSCModulePhase.START,
    timestamp=datetime.now(),
    data={"app_name": "MyApp"},
    source="core"
)

# Access event properties
print(f"Hook type: {event.kind}")
print(f"Phase: {event.phase}")
print(f"Timestamp: {event.timestamp}")
print(f"Data: {event.data}")
```

## Lifecycle Hooks

### Application Lifecycle

```python
from dmsc import DMSCAppBuilder, DMSCHookKind

async def on_init(ctx, event):
    ctx.logger.info("lifecycle", "Initializing application")
    return {"phase": "init"}

async def on_startup(ctx, event):
    ctx.logger.info("lifecycle", "Starting application")
    # Initialize services
    await ctx.cache.connect()
    await ctx.db.connect()
    return {"phase": "startup"}

async def on_ready(ctx, event):
    ctx.logger.info("lifecycle", "Application ready")
    return {"phase": "ready"}

async def on_shutdown(ctx, event):
    ctx.logger.info("lifecycle", "Shutting down application")
    # Cleanup services
    await ctx.cache.disconnect()
    await ctx.db.disconnect()
    return {"phase": "shutdown"}

app = DMSCAppBuilder()
app.on_init(on_init)
app.on_startup(on_startup)
app.on_ready(on_ready)
app.on_shutdown(on_shutdown)
```

### Module Lifecycle

```python
from dmsc import DMSCHookKind, DMSCModulePhase

# Module initialization hooks
async def before_module_init(ctx, event):
    ctx.logger.info("modules", f"Initializing module: {event.data.get('module_name')}")

async def after_module_init(ctx, event):
    ctx.logger.info("modules", f"Module initialized: {event.data.get('module_name')}")

# Module startup hooks
async def before_module_start(ctx, event):
    ctx.logger.info("modules", f"Starting module: {event.data.get('module_name')}")

async def after_module_start(ctx, event):
    ctx.logger.info("modules", f"Module started: {event.data.get('module_name')}")

# Module shutdown hooks
async def before_module_shutdown(ctx, event):
    ctx.logger.info("modules", f"Shutting down module: {event.data.get('module_name')}")

async def after_module_shutdown(ctx, event):
    ctx.logger.info("modules", f"Module shut down: {event.data.get('module_name')}")
```

## Custom Hooks

### Define Custom Hook Kind

```python
from dmsc import DMSCHookKind

# Extend hook kinds with custom types
class CustomHookKind(DMSCHookKind):
    USER_LOGIN = "user_login"
    USER_LOGOUT = "user_logout"
    ORDER_CREATED = "order_created"
    ORDER_COMPLETED = "order_completed"
```

### Trigger Custom Hooks

```python
from dmsc import DMSCHookBus, DMSCHookEvent, DMSCHookKind

hook_bus = DMSCHookBus()

# Register custom hook handler
async def on_user_login(ctx, event):
    ctx.logger.info("auth", f"User logged in: {event.data.get('user_id')}")
    return {"logged_in": True}

hook_bus._register(
    CustomHookKind.USER_LOGIN,
    on_user_login,
    priority=100
)

# Trigger custom hook
login_event = DMSCHookEvent(
    kind=CustomHookKind.USER_LOGIN,
    phase=DMSCModulePhase.START,
    timestamp=datetime.now(),
    data={"user_id": 123, "username": "john"},
    source="auth"
)

await hook_bus.trigger_async(login_event)
```

## Event-Driven Architecture

### Publish-Subscribe Pattern

```python
from dmsc import DMSCHookBus, DMSCHookEvent

# Create event bus
event_bus = DMSCHookBus()

# Publisher
async def publish_event(bus: DMSCHookBus, event_type: str, data: dict):
    event = DMSCHookEvent(
        kind=event_type,
        phase=DMSCModulePhase.START,
        timestamp=datetime.now(),
        data=data,
        source="publisher"
    )
    await bus.trigger_async(event)

# Subscriber
async def handle_event(ctx, event: DMSCHookEvent):
    ctx.logger.info("subscriber", f"Received event: {event.kind}")
    return {"processed": True}

# Subscribe to events
event_bus._register("custom.event", handle_event)

# Publish event
await publish_event(event_bus, "custom.event", {"message": "Hello subscribers"})
```

### Event Priority

```python
from dmsc import DMSCHookBus, DMSCHookKind

hook_bus = DMSCHookBus()

# Higher priority handlers run first
async def high_priority_handler(ctx, event):
    ctx.logger.info("priority", "High priority handler")
    return {"priority": 100}

async def low_priority_handler(ctx, event):
    ctx.logger.info("priority", "Low priority handler")
    return {"priority": 1}

# Register with different priorities
hook_bus._register(DMSCHookKind.STARTUP, low_priority_handler, priority=1)
hook_bus._register(DMSCHookKind.STARTUP, high_priority_handler, priority=100)
```

## Best Practices

1. **Keep Handlers Light**: Hook handlers should be fast and non-blocking
2. **Handle Errors Gracefully**: Always catch exceptions in hook handlers
3. **Use Priorities Wisely**: Use priorities to control execution order
4. **Avoid Circular Dependencies**: Don't trigger hooks from within hook handlers
5. **Document Hooks**: Document all custom hooks and their purposes
6. **Test Hooks**: Write tests for hook handlers
7. **Use Context**: Pass relevant context through hook events
8. **Cleanup Properly**: Always unregister handlers when no longer needed
9. **Use Async for I/O**: Use async handlers for I/O operations
10. **Monitor Performance**: Track hook execution time in production
