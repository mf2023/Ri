<div align="center">

# Protocol Layer Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's protocol module for protocol abstraction, message sending, protocol switching, and secure communication.

## Example Overview

This example creates a DMSC Python application with the following features:

- Protocol manager initialization and configuration
- Global and private protocol usage
- Message sending with different protocols
- Protocol switching at runtime
- Protocol statistics and monitoring
- Connection state management

## Prerequisites

- Python 3.8+
- Understanding of protocol concepts
- Knowledge of secure communication patterns

## Complete Code Example

```python
import asyncio
import json
from datetime import datetime
from typing import Dict, List, Optional, Any, Tuple
from enum import Enum
from dataclasses import dataclass
from collections import deque

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType,
    DMSCFrame, DMSCFrameType, DMSCFrameHeader,
    DMSCConnectionState, DMSCProtocolStats,
    DMSCProtocolStatus, DMSCProtocolHealth,
    DMSCConfig, DMSCError
)

# Message priority
class MessagePriority(Enum):
    LOW = 0
    NORMAL = 100
    HIGH = 200
    URGENT = 300

# Message data class
@dataclass
class ProtocolMessage:
    message_id: str
    target: str
    payload: Dict[str, Any]
    protocol_type: DMSCProtocolType
    priority: MessagePriority
    created_at: datetime
    sent_at: Optional[datetime]
    received_at: Optional[datetime]
    status: str
    response: Optional[Dict]

# Protocol service
class ProtocolService:
    def __init__(self, protocol_manager: DMSCProtocolManager, context: DMSCServiceContext):
        self.protocol_manager = protocol_manager
        self.context = context
        self.logger = context.logger
        self.message_history: List[ProtocolMessage] = []
        self.pending_messages: Dict[str, ProtocolMessage] = {}
        self.connection_states: Dict[str, DMSCConnectionState] = {}
        self.protocol_stats: DMSCProtocolStats = None
    
    async def initialize(self, config: DMSCProtocolConfig = None):
        """Initialize the protocol manager"""
        if config is None:
            config = DMSCProtocolConfig.default()
        
        await self.protocol_manager.initialize(config)
        self.logger.info("protocol", "Protocol manager initialized")
    
    async def send_message(
        self,
        target: str,
        payload: Dict[str, Any],
        protocol_type: DMSCProtocolType = None,
        priority: MessagePriority = MessagePriority.NORMAL
    ) -> ProtocolMessage:
        """Send a message using the protocol"""
        message_id = f"msg_{datetime.now().timestamp()}"
        
        if protocol_type is None:
            protocol_type = await self.protocol_manager.get_current_protocol()
        
        message = ProtocolMessage(
            message_id=message_id,
            target=target,
            payload=payload,
            protocol_type=protocol_type,
            priority=priority,
            created_at=datetime.now(),
            sent_at=None,
            received_at=None,
            status="pending",
            response=None
        )
        
        # Track message
        self.message_history.append(message)
        self.pending_messages[message_id] = message
        
        # Send message
        try:
            response = await self.protocol_manager.send_message(
                target,
                json.dumps(payload).encode()
            )
            
            message.sent_at = datetime.now()
            message.status = "sent"
            
            # Parse response
            if response:
                try:
                    message.response = json.loads(response.decode())
                    message.received_at = datetime.now()
                    message.status = "completed"
                except:
                    message.response = {"raw": response.decode()}
                    message.received_at = datetime.now()
                    message.status = "completed"
            
            self.logger.info("protocol", f"Message sent: {message_id} to {target}")
            
        except Exception as e:
            message.status = "failed"
            self.logger.error("protocol", f"Failed to send message: {e}")
            raise
        
        return message
    
    async def switch_protocol(self, protocol_type: DMSCProtocolType):
        """Switch the current protocol"""
        await self.protocol_manager.switch_protocol(protocol_type)
        current = await self.protocol_manager.get_current_protocol()
        self.logger.info("protocol", f"Protocol switched to: {current.name}")
    
    async def get_current_protocol(self) -> DMSCProtocolType:
        """Get the current protocol type"""
        return await self.protocol_manager.get_current_protocol()
    
    async def send_with_global_protocol(
        self,
        target: str,
        payload: Dict[str, Any]
    ) -> ProtocolMessage:
        """Send a message using the global protocol"""
        return await self.send_message(target, payload, DMSCProtocolType.GLOBAL)
    
    async def send_with_private_protocol(
        self,
        target: str,
        payload: Dict[str, Any]
    ) -> ProtocolMessage:
        """Send a message using the private protocol for sensitive operations"""
        return await self.send_message(target, payload, DMSCProtocolType.PRIVATE)
    
    async def get_protocol_stats(self) -> DMSCProtocolStats:
        """Get protocol statistics"""
        stats = await self.protocol_manager.get_stats()
        self.protocol_stats = stats
        return stats
    
    async def get_protocol_status(self) -> DMSCProtocolStatus:
        """Get protocol status"""
        status = await self.protocol_manager.get_status()
        return status
    
    async def get_connection_state(self, target: str) -> DMSCConnectionState:
        """Get connection state for a target"""
        # In a real implementation, this would check the actual connection
        return self.connection_states.get(target, DMSCConnectionState.DISCONNECTED)
    
    async def set_connection_state(self, target: str, state: DMSCConnectionState):
        """Set connection state for a target"""
        self.connection_states[target] = state
    
    def get_message_history(
        self,
        target: Optional[str] = None,
        status: Optional[str] = None
    ) -> List[ProtocolMessage]:
        """Get message history, optionally filtered"""
        messages = self.message_history
        
        if target:
            messages = [m for m in messages if m.target == target]
        
        if status:
            messages = [m for m in messages if m.status == status]
        
        return sorted(messages, key=lambda x: x.created_at, reverse=True)
    
    def get_message_stats(self) -> Dict:
        """Get message statistics"""
        total = len(self.message_history)
        completed = len([m for m in self.message_history if m.status == "completed"])
        failed = len([m for m in self.message_history if m.status == "failed"])
        pending = len([m for m in self.message_history if m.status == "pending"])
        
        return {
            "total_messages": total,
            "completed": completed,
            "failed": failed,
            "pending": pending,
            "success_rate": (completed / total * 100) if total > 0 else 0
        }

# Request handlers
async def handle_initialize(context: DMSCServiceContext):
    """Initialize the protocol manager"""
    data = await context.http.request.json()
    
    protocol_type_str = data.get("protocol_type", "global")
    enable_security = data.get("enable_security", True)
    enable_state_sync = data.get("enable_state_sync", True)
    
    try:
        protocol_type = DMSCProtocolType(protocol_type_str)
    except ValueError:
        protocol_type = DMSCProtocolType.GLOBAL
    
    config = DMSCProtocolConfig(
        default_protocol=protocol_type,
        enable_security=enable_security,
        enable_state_sync=enable_state_sync
    )
    
    protocol_service = context.protocol_service
    await protocol_service.initialize(config)
    
    return {
        "status": "success",
        "message": "Protocol manager initialized",
        "data": {
            "default_protocol": protocol_type.value,
            "security_enabled": enable_security
        }
    }

async def handle_send_message(context: DMSCServiceContext):
    """Send a message"""
    data = await context.http.request.json()
    
    target = data.get("target")
    payload = data.get("payload", {})
    protocol_type_str = data.get("protocol_type")
    priority_str = data.get("priority", "normal")
    
    if not target:
        return {"status": "error", "message": "target required"}, 400
    
    try:
        priority = MessagePriority(priority_str)
    except ValueError:
        priority = MessagePriority.NORMAL
    
    try:
        protocol_type = DMSCProtocolType(protocol_type_str) if protocol_type_str else None
    except ValueError:
        protocol_type = None
    
    protocol_service = context.protocol_service
    message = await protocol_service.send_message(
        target=target,
        payload=payload,
        protocol_type=protocol_type,
        priority=priority
    )
    
    return {
        "status": "success",
        "data": {
            "message_id": message.message_id,
            "target": message.target,
            "protocol_type": message.protocol_type.value,
            "status": message.status,
            "created_at": message.created_at.isoformat()
        }
    }

async def handle_send_global(context: DMSCServiceContext):
    """Send a message using global protocol"""
    data = await context.http.request.json()
    
    target = data.get("target")
    payload = data.get("payload", {})
    
    if not target:
        return {"status": "error", "message": "target required"}, 400
    
    protocol_service = context.protocol_service
    message = await protocol_service.send_with_global_protocol(target, payload)
    
    return {
        "status": "success",
        "data": {
            "message_id": message.message_id,
            "status": message.status
        }
    }

async def handle_send_private(context: DMSCServiceContext):
    """Send a message using private protocol"""
    data = await context.http.request.json()
    
    target = data.get("target")
    payload = data.get("payload", {})
    
    if not target:
        return {"status": "error", "message": "target required"}, 400
    
    protocol_service = context.protocol_service
    message = await protocol_service.send_with_private_protocol(target, payload)
    
    return {
        "status": "success",
        "data": {
            "message_id": message.message_id,
            "status": message.status
        }
    }

async def handle_switch_protocol(context: DMSCServiceContext):
    """Switch the current protocol"""
    data = await context.http.request.json()
    
    protocol_type_str = data.get("protocol_type")
    
    if not protocol_type_str:
        return {"status": "error", "message": "protocol_type required"}, 400
    
    try:
        protocol_type = DMSCProtocolType(protocol_type_str)
    except ValueError:
        return {"status": "error", "message": "Invalid protocol type"}, 400
    
    protocol_service = context.protocol_service
    await protocol_service.switch_protocol(protocol_type)
    current = await protocol_service.get_current_protocol()
    
    return {
        "status": "success",
        "data": {
            "current_protocol": current.value
        }
    }

async def handle_get_current_protocol(context: DMSCServiceContext):
    """Get the current protocol"""
    protocol_service = context.protocol_service
    current = await protocol_service.get_current_protocol()
    
    return {
        "status": "success",
        "data": {
            "protocol": current.value
        }
    }

async def handle_get_stats(context: DMSCServiceContext):
    """Get protocol statistics"""
    protocol_service = context.protocol_service
    stats = await protocol_service.get_protocol_stats()
    
    return {
        "status": "success",
        "data": {
            "total_messages_sent": stats.total_messages_sent,
            "total_messages_received": stats.total_messages_received,
            "total_bytes_sent": stats.total_bytes_sent,
            "total_bytes_received": stats.total_bytes_received,
            "average_latency_ms": stats.average_latency_ms,
            "error_count": stats.error_count,
            "success_rate": stats.success_rate * 100
        }
    }

async def handle_get_status(context: DMSCServiceContext):
    """Get protocol status"""
    protocol_service = context.protocol_service
    status = await protocol_service.get_protocol_status()
    
    return {
        "status": "success",
        "data": {
            "initialized": status.initialized,
            "active": status.active,
            "active_connections": status.active_connections,
            "health": str(status.health)
        }
    }

async def handle_get_message_history(context: DMSCServiceContext):
    """Get message history"""
    data = await context.http.request.json()
    
    target = data.get("target")
    status = data.get("status")
    
    protocol_service = context.protocol_service
    messages = protocol_service.get_message_history(target, status)
    
    return {
        "status": "success",
        "data": {
            "count": len(messages),
            "messages": [
                {
                    "message_id": m.message_id,
                    "target": m.target,
                    "protocol_type": m.protocol_type.value,
                    "status": m.status,
                    "created_at": m.created_at.isoformat()
                }
                for m in messages[:100]
            ]
        }
    }

async def handle_get_message_stats(context: DMSCServiceContext):
    """Get message statistics"""
    protocol_service = context.protocol_service
    stats = protocol_service.get_message_stats()
    
    return {"status": "success", "data": stats}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize protocol manager
    protocol_manager = DMSCProtocolManager()
    
    # Initialize protocol service
    protocol_service = ProtocolService(protocol_manager, dms_app.context)
    dms_app.context.protocol_service = protocol_service
    
    # Add routes
    dms_app.router.add_route("POST", "/protocol/initialize", handle_initialize)
    dms_app.router.add_route("POST", "/protocol/send", handle_send_message)
    dms_app.router.add_route("POST", "/protocol/send/global", handle_send_global)
    dms_app.router.add_route("POST", "/protocol/send/private", handle_send_private)
    dms_app.router.add_route("POST", "/protocol/switch", handle_switch_protocol)
    dms_app.router.add_route("GET", "/protocol/current", handle_get_current_protocol)
    dms_app.router.add_route("GET", "/protocol/stats", handle_get_stats)
    dms_app.router.add_route("GET", "/protocol/status", handle_get_status)
    dms_app.router.add_route("POST", "/protocol/messages", handle_get_message_history)
    dms_app.router.add_route("GET", "/protocol/messages/stats", handle_get_message_stats)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Protocol Service Architecture

1. **Protocol Manager**: Initialize and configure the protocol system
2. **Message Sending**: Send messages with different protocols and priorities
3. **Protocol Switching**: Switch between global and private protocols
4. **Statistics**: Track message counts, latency, and success rates
5. **Connection Management**: Monitor connection states

### Key Components

- **DMSCProtocolManager**: Main protocol interface
- **DMSCProtocolConfig**: Protocol configuration
- **DMSCProtocolType**: Protocol type enumeration (GLOBAL, PRIVATE)
- **DMSCProtocolStats**: Protocol statistics
- **DMSCConnectionState**: Connection state enumeration

## Running Steps

1. Save the code to `protocol_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python protocol_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Initialize protocol manager
   curl -X POST http://localhost:8080/protocol/initialize \
     -H "Content-Type: application/json" \
     -d '{"protocol_type": "global", "enable_security": true}'
   
   # Send a message
   curl -X POST http://localhost:8080/protocol/send \
     -H "Content-Type: application/json" \
     -d '{"target": "device-001", "payload": {"command": "get_status"}, "priority": "high"}'
   
   # Send using global protocol
   curl -X POST http://localhost:8080/protocol/send/global \
     -H "Content-Type: application/json" \
     -d '{"target": "monitor-001", "payload": {"type": "status_check"}}'
   
   # Send using private protocol (for sensitive operations)
   curl -X POST http://localhost:8080/protocol/send/private \
     -H "Content-Type: application/json" \
     -d '{"target": "secure-gateway", "payload": {"command": "execute_critical"}}'
   
   # Switch protocol
   curl -X POST http://localhost:8080/protocol/switch \
     -H "Content-Type: application/json" \
     -d '{"protocol_type": "private"}'
   
   # Get current protocol
   curl http://localhost:8080/protocol/current
   
   # Get protocol stats
   curl http://localhost:8080/protocol/stats
   
   # Get protocol status
   curl http://localhost:8080/protocol/status
   
   # Get message history
   curl -X POST http://localhost:8080/protocol/messages \
     -H "Content-Type: application/json" \
     -d '{"status": "completed"}'
   
   # Get message stats
   curl http://localhost:8080/protocol/messages/stats
   ```

## Expected Output

### Send Message Response

```json
{
  "status": "success",
  "data": {
    "message_id": "msg_1705313400.123",
    "target": "device-001",
    "protocol_type": "global",
    "status": "completed",
    "created_at": "2024-01-15T10:30:00"
  }
}
```

### Protocol Stats Response

```json
{
  "status": "success",
  "data": {
    "total_messages_sent": 150,
    "total_messages_received": 145,
    "total_bytes_sent": 10240,
    "total_bytes_received": 20480,
    "average_latency_ms": 15.5,
    "error_count": 5,
    "success_rate": 96.67
  }
}
```

### Message Stats Response

```json
{
  "status": "success",
  "data": {
    "total_messages": 150,
    "completed": 145,
    "failed": 5,
    "pending": 0,
    "success_rate": 96.67
  }
}
```

## Best Practices

1. **Use Private Protocol for Sensitive Data**: Always use private protocol for sensitive operations
2. **Configure Timeouts**: Set appropriate timeouts for all operations
3. **Monitor Performance**: Track protocol statistics in production
4. **Handle Reconnections**: Implement automatic reconnection logic
5. **Use Heartbeats**: Implement heartbeats to detect connection failures
6. **Validate Certificates**: Always validate certificates in production
7. **Use Compression**: Compress large messages to reduce bandwidth
8. **Implement Rate Limiting**: Prevent protocol abuse with rate limiting
9. **Log Protocol Events**: Log all protocol events for debugging
10. **Test Protocol Switching**: Test protocol switching behavior thoroughly
