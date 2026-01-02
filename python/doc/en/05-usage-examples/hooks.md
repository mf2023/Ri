<div align="center">

# Hooks System Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's hooks module for event-driven architecture, lifecycle management, and component communication.

## Example Overview

This example creates a DMSC Python application with the following features:

- Event bus for component communication
- Lifecycle hooks (startup, shutdown, etc.)
- Module initialization hooks
- Custom event types
- Event prioritization
- Async event handling

## Prerequisites

- Python 3.8+
- Understanding of event-driven architecture
- Knowledge of lifecycle management concepts

## Complete Code Example

```python
import asyncio
from datetime import datetime
from typing import Dict, List, Callable, Optional, Any
from enum import Enum
from dataclasses import dataclass
from functools import wraps

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCHookBus, DMSCHookKind, DMSCHookEvent, DMSCHookHandler,
    DMSCModulePhase, DMSCConfig, DMSCError
)

# Event priority
class EventPriority(Enum):
    LOW = 0
    NORMAL = 100
    HIGH = 200
    CRITICAL = 300

# Custom event types
class AppEventType(Enum):
    USER_LOGIN = "user_login"
    USER_LOGOUT = "user_logout"
    ORDER_CREATED = "order_created"
    ORDER_SHIPPED = "order_shipped"
    NOTIFICATION_SENT = "notification_sent"
    DATA_SYNC = "data_sync"

# Event data class
@dataclass
class AppEvent:
    event_id: str
    event_type: AppEventType
    timestamp: datetime
    data: Dict[str, Any]
    source: str
    priority: EventPriority
    handled: bool = False

# Hook handler result
@dataclass
class HookResult:
    success: bool
    message: str
    data: Optional[Dict] = None

# Event bus service
class EventBusService:
    def __init__(self, hook_bus: DMSCHookBus, context: DMSCServiceContext):
        self.hook_bus = hook_bus
        self.context = context
        self.logger = context.logger
        self.event_handlers: Dict[str, List[Dict]] = {}
        self.event_history: List[AppEvent] = []
        self.event_stats: Dict[str, int] = {}
    
    def register_handler(
        self,
        event_type: AppEventType,
        handler: Callable,
        priority: EventPriority = EventPriority.NORMAL,
        async_mode: bool = True
    ) -> str:
        """Register an event handler"""
        handler_id = f"handler_{datetime.now().timestamp()}"
        
        handler_info = {
            "id": handler_id,
            "handler": handler,
            "priority": priority,
            "async_mode": async_mode
        }
        
        if event_type.value not in self.event_handlers:
            self.event_handlers[event_type.value] = []
        
        self.event_handlers[event_type.value].append(handler_info)
        self.event_handlers[event_type.value].sort(key=lambda x: x["priority"].value, reverse=True)
        
        self.logger.info("hooks", f"Handler registered for {event_type.value}: {handler_id}")
        return handler_id
    
    def unregister_handler(self, handler_id: str) -> bool:
        """Unregister an event handler"""
        for event_type, handlers in self.event_handlers.items():
            for handler in handlers:
                if handler["id"] == handler_id:
                    handlers.remove(handler)
                    self.logger.info("hooks", f"Handler unregistered: {handler_id}")
                    return True
        return False
    
    async def emit_event(self, event: AppEvent) -> List[HookResult]:
        """Emit an event to all registered handlers"""
        results = []
        event_type = event.event_type.value
        
        self.event_history.append(event)
        self.event_stats[event_type] = self.event_stats.get(event_type, 0) + 1
        
        if event_type in self.event_handlers:
            for handler_info in self.event_handlers[event_type]:
                try:
                    handler = handler_info["handler"]
                    
                    if handler_info["async_mode"]:
                        if asyncio.iscoroutinefunction(handler):
                            result = await handler(event)
                        else:
                            result = handler(event)
                    else:
                        result = handler(event)
                    
                    results.append(HookResult(
                        success=True,
                        message="Handler executed successfully",
                        data=result if isinstance(result, dict) else None
                    ))
                    
                    event.handled = True
                
                except Exception as e:
                    self.logger.error("hooks", f"Handler error: {e}")
                    results.append(HookResult(
                        success=False,
                        message=str(e)
                    ))
        
        self.logger.info("hooks", f"Event {event.event_type.value} emitted, {len(results)} handlers executed")
        return results
    
    def create_event(
        self,
        event_type: AppEventType,
        data: Dict[str, Any],
        source: str,
        priority: EventPriority = EventPriority.NORMAL
    ) -> AppEvent:
        """Create a new event"""
        return AppEvent(
            event_id=f"evt_{datetime.now().timestamp()}",
            event_type=event_type,
            timestamp=datetime.now(),
            data=data,
            source=source,
            priority=priority
        )
    
    def get_event_history(self, event_type: Optional[AppEventType] = None) -> List[AppEvent]:
        """Get event history, optionally filtered by type"""
        if event_type:
            return [e for e in self.event_history if e.event_type == event_type]
        return list(self.event_history)
    
    def get_event_stats(self) -> Dict[str, int]:
        """Get event statistics"""
        return dict(self.event_stats)

# Module lifecycle manager
class LifecycleManager:
    def __init__(self, event_bus: EventBusService, context: DMSCServiceContext):
        self.event_bus = event_bus
        self.context = context
        self.logger = context.logger
        self.modules: Dict[str, Dict] = {}
        self.module_order: List[str] = []
    
    def register_module(self, module_name: str, dependencies: List[str] = None):
        """Register a module with its dependencies"""
        self.modules[module_name] = {
            "name": module_name,
            "dependencies": dependencies or [],
            "initialized": False,
            "started": False,
            "stopped": False
        }
        
        # Calculate initialization order
        self._calculate_init_order()
        
        self.logger.info("lifecycle", f"Module registered: {module_name}")
    
    def _calculate_init_order(self):
        """Calculate module initialization order based on dependencies"""
        self.module_order = []
        remaining = set(self.modules.keys())
        
        while remaining:
            # Find modules with all dependencies satisfied
            ready = [
                name for name in remaining
                if all(dep in self.module_order or dep not in self.modules
                       for dep in self.modules[name]["dependencies"])
            ]
            
            if not ready:
                raise DMSCError("Circular dependency detected", "CIRCULAR_DEPENDENCY")
            
            self.module_order.extend(ready)
            remaining -= set(ready)
    
    async def initialize_all(self):
        """Initialize all modules in order"""
        for module_name in self.module_order:
            module = self.modules[module_name]
            
            # Emit BEFORE_INIT event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "before_init"},
                    "lifecycle",
                    EventPriority.HIGH
                )
            )
            
            # Initialize module (simulated)
            await asyncio.sleep(0.1)
            module["initialized"] = True
            
            # Emit AFTER_INIT event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "after_init"},
                    "lifecycle",
                    EventPriority.HIGH
                )
            )
            
            self.logger.info("lifecycle", f"Module initialized: {module_name}")
    
    async def start_all(self):
        """Start all modules in order"""
        for module_name in self.module_order:
            module = self.modules[module_name]
            
            # Emit BEFORE_START event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "before_start"},
                    "lifecycle",
                    EventPriority.CRITICAL
                )
            )
            
            # Start module (simulated)
            await asyncio.sleep(0.1)
            module["started"] = True
            
            # Emit AFTER_START event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "after_start"},
                    "lifecycle",
                    EventPriority.CRITICAL
                )
            )
            
            self.logger.info("lifecycle", f"Module started: {module_name}")
    
    async def shutdown_all(self):
        """Shutdown all modules in reverse order"""
        for module_name in reversed(self.module_order):
            module = self.modules[module_name]
            
            # Emit BEFORE_SHUTDOWN event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "before_shutdown"},
                    "lifecycle",
                    EventPriority.CRITICAL
                )
            )
            
            # Shutdown module (simulated)
            await asyncio.sleep(0.1)
            module["stopped"] = True
            
            # Emit AFTER_SHUTDOWN event
            await self.event_bus.emit_event(
                self.event_bus.create_event(
                    AppEventType.DATA_SYNC,
                    {"module": module_name, "phase": "after_shutdown"},
                    "lifecycle",
                    EventPriority.CRITICAL
                )
            )
            
            self.logger.info("lifecycle", f"Module shutdown: {module_name}")
    
    def get_module_status(self) -> Dict[str, Dict]:
        """Get status of all modules"""
        return dict(self.modules)

# Event handlers
async def handle_user_login(event: AppEvent) -> Dict:
    """Handle user login event"""
    user_id = event.data.get("user_id")
    print(f"User logged in: {user_id}")
    return {"user_id": user_id, "action": "login_recorded"}

async def handle_user_logout(event: AppEvent) -> Dict:
    """Handle user logout event"""
    user_id = event.data.get("user_id")
    print(f"User logged out: {user_id}")
    return {"user_id": user_id, "action": "cleanup_performed"}

async def handle_order_created(event: AppEvent) -> Dict:
    """Handle order created event"""
    order_id = event.data.get("order_id")
    print(f"Order created: {order_id}")
    return {"order_id": order_id, "action": "inventory_reserved"}

# Request handlers
async def handle_emit_event(context: DMSCServiceContext):
    """Emit a custom event"""
    data = await context.http.request.json()
    
    event_type_str = data.get("event_type")
    event_data = data.get("data", {})
    source = data.get("source", "api")
    priority_str = data.get("priority", "normal")
    
    try:
        event_type = AppEventType(event_type_str)
        priority = EventPriority(priority_str)
        
        event_bus = context.event_bus
        event = event_bus.create_event(event_type, event_data, source, priority)
        results = await event_bus.emit_event(event)
        
        return {
            "status": "success",
            "data": {
                "event_id": event.event_id,
                "handlers_executed": len(results),
                "results": [
                    {"success": r.success, "message": r.message}
                    for r in results
                ]
            }
        }
    except ValueError:
        return {"status": "error", "message": "Invalid event type"}, 400

async def handle_register_handler(context: DMSCServiceContext):
    """Register an event handler"""
    data = await context.http.request.json()
    
    event_type_str = data.get("event_type")
    handler_type = data.get("handler_type")
    
    try:
        event_type = AppEventType(event_type_str)
        
        # Select handler based on type
        if handler_type == "login":
            handler = handle_user_login
        elif handler_type == "logout":
            handler = handle_user_logout
        elif handler_type == "order":
            handler = handle_order_created
        else:
            return {"status": "error", "message": "Unknown handler type"}, 400
        
        event_bus = context.event_bus
        priority = EventPriority(data.get("priority", "normal"))
        handler_id = event_bus.register_handler(event_type, handler, priority)
        
        return {"status": "success", "data": {"handler_id": handler_id}}
    except ValueError:
        return {"status": "error", "message": "Invalid event type"}, 400

async def handle_get_event_history(context: DMSCServiceContext):
    """Get event history"""
    data = await context.http.request.json()
    event_type_str = data.get("event_type")
    
    event_bus = context.event_bus
    
    if event_type_str:
        try:
            event_type = AppEventType(event_type_str)
            history = event_bus.get_event_history(event_type)
        except ValueError:
            return {"status": "error", "message": "Invalid event type"}, 400
    else:
        history = event_bus.get_event_history()
    
    return {
        "status": "success",
        "data": {
            "count": len(history),
            "events": [
                {
                    "id": e.event_id,
                    "type": e.event_type.value,
                    "timestamp": e.timestamp.isoformat(),
                    "source": e.source,
                    "handled": e.handled
                }
                for e in history[-50:]
            ]
        }
    }

async def handle_get_event_stats(context: DMSCServiceContext):
    """Get event statistics"""
    event_bus = context.event_bus
    stats = event_bus.get_event_stats()
    
    return {"status": "success", "data": stats}

async def handle_get_modules(context: DMSCServiceContext):
    """Get module status"""
    lifecycle = context.lifecycle_manager
    modules = lifecycle.get_module_status()
    
    return {
        "status": "success",
        "data": {
            "modules": modules,
            "init_order": lifecycle.module_order
        }
    }

async def handle_initialize_modules(context: DMSCServiceContext):
    """Initialize all modules"""
    lifecycle = context.lifecycle_manager
    await lifecycle.initialize_all()
    
    return {"status": "success", "message": "All modules initialized"}

async def handle_start_modules(context: DMSCServiceContext):
    """Start all modules"""
    lifecycle = context.lifecycle_manager
    await lifecycle.start_all()
    
    return {"status": "success", "message": "All modules started"}

async def handle_shutdown_modules(context: DMSCServiceContext):
    """Shutdown all modules"""
    lifecycle = context.lifecycle_manager
    await lifecycle.shutdown_all()
    
    return {"status": "success", "message": "All modules shutdown"}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize hook bus
    hook_bus = DMSCHookBus()
    
    # Initialize services
    event_bus = EventBusService(hook_bus, dms_app.context)
    lifecycle = LifecycleManager(event_bus, dms_app.context)
    
    # Store in context
    dms_app.context.hook_bus = hook_bus
    dms_app.context.event_bus = event_bus
    dms_app.context.lifecycle_manager = lifecycle
    
    # Register modules
    lifecycle.register_module("database", [])
    lifecycle.register_module("cache", ["database"])
    lifecycle.register_module("auth", ["database"])
    lifecycle.register_module("api", ["database", "cache", "auth"])
    
    # Register default handlers
    event_bus.register_handler(AppEventType.USER_LOGIN, handle_user_login)
    event_bus.register_handler(AppEventType.USER_LOGOUT, handle_user_logout)
    event_bus.register_handler(AppEventType.ORDER_CREATED, handle_order_created)
    
    # Add routes
    dms_app.router.add_route("POST", "/events/emit", handle_emit_event)
    dms_app.router.add_route("POST", "/handlers/register", handle_register_handler)
    dms_app.router.add_route("POST", "/events/history", handle_get_event_history)
    dms_app.router.add_route("GET", "/events/stats", handle_get_event_stats)
    dms_app.router.add_route("GET", "/modules", handle_get_modules)
    dms_app.router.add_route("POST", "/modules/initialize", handle_initialize_modules)
    dms_app.router.add_route("POST", "/modules/start", handle_start_modules)
    dms_app.router.add_route("POST", "/modules/shutdown", handle_shutdown_modules)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Event Bus Architecture

1. **Event Types**: Custom event types for different business events
2. **Event Handlers**: Register handlers for specific event types
3. **Event Priority**: Execute handlers in priority order
4. **Async Handling**: Support both sync and async handlers
5. **Event History**: Track all events for debugging

### Module Lifecycle

1. **Dependency Management**: Handle module dependencies automatically
2. **Initialization Order**: Calculate correct initialization order
3. **Lifecycle Events**: Emit events at each lifecycle phase
4. **Graceful Shutdown**: Proper cleanup in reverse order

## Running Steps

1. Save the code to `hooks_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python hooks_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Emit an event
   curl -X POST http://localhost:8080/events/emit \
     -H "Content-Type: application/json" \
     -d '{"event_type": "user_login", "data": {"user_id": "123"}, "source": "api"}'
   
   # Register a handler
   curl -X POST http://localhost:8080/handlers/register \
     -H "Content-Type: application/json" \
     -d '{"event_type": "order_created", "handler_type": "order"}'
   
   # Get event history
   curl -X POST http://localhost:8080/events/history \
     -H "Content-Type: application/json" \
     -d '{"event_type": "user_login"}'
   
   # Get event stats
   curl http://localhost:8080/events/stats
   
   # Get module status
   curl http://localhost:8080/modules
   
   # Initialize modules
   curl -X POST http://localhost:8080/modules/initialize
   
   # Start modules
   curl -X POST http://localhost:8080/modules/start
   
   # Shutdown modules
   curl -X POST http://localhost:8080/modules/shutdown
   ```

## Expected Output

### Emit Event Response

```json
{
  "status": "success",
  "data": {
    "event_id": "evt_1705313400.123",
    "handlers_executed": 1,
    "results": [
      {"success": true, "message": "Handler executed successfully"}
    ]
  }
}
```

### Event Stats Response

```json
{
  "status": "success",
  "data": {
    "user_login": 15,
    "user_logout": 8,
    "order_created": 25
  }
}
```

### Module Status Response

```json
{
  "status": "success",
  "data": {
    "modules": {
      "database": {"initialized": true, "started": true, "stopped": false},
      "cache": {"initialized": true, "started": true, "stopped": false},
      "auth": {"initialized": true, "started": true, "stopped": false},
      "api": {"initialized": true, "started": true, "stopped": false}
    },
    "init_order": ["database", "cache", "auth", "api"]
  }
}
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
