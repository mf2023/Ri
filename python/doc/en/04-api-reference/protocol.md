<div align="center">

# Protocol API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The protocol module provides a protocol abstraction layer, supporting global and private communication protocols, implementing protocol management, security features, and integration capabilities for distributed systems.

## Module Overview

</div>

The protocol module contains the following core components:

- **DMSCProtocolManager**: Protocol manager
- **DMSCProtocolConfig**: Protocol configuration
- **DMSCProtocolType**: Protocol type enumeration
- **DMSCProtocolStats**: Protocol statistics
- **DMSCProtocolStatus**: Protocol status
- **DMSCProtocolHealth**: Protocol health status
- **DMSCConnectionState**: Connection state enumeration
- **DMSCConnectionStats**: Connection statistics
- **DMSCFrame**: Protocol frame
- **DMSCFrameType**: Frame type enumeration
- **DMSCFrameHeader**: Frame header
- **DMSCDeviceAuthProtocol**: Device authentication protocol
- **DMSCProtocolAdapter**: Protocol adapter
- **DMSCSecurityContext**: Security context
- **DMSCThreatLevel**: Threat level enumeration
- **DMSCDataClassification**: Data classification enumeration
- **DMSCNetworkEnvironment**: Network environment enumeration

<div align="center">

## Core Components

</div>

### DMSCProtocolManager

Protocol manager, used for managing communication protocols.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `initialize(config)` | Initialize protocol manager | `config: DMSCProtocolConfig` | `None` |
| `send_message(target, message)` | Send message | `target: str`, `message: bytes` | `bytes` |
| `send_message_async(target, message)` | Send message asynchronously | `target: str`, `message: bytes` | `bytes` |
| `switch_protocol(protocol_type)` | Switch protocol type | `protocol_type: DMSCProtocolType` | `None` |
| `get_current_protocol()` | Get current protocol | None | `DMSCProtocolType` |
| `get_stats()` | Get protocol statistics | None | `DMSCProtocolStats` |
| `get_status()` | Get protocol status | None | `DMSCProtocolStatus` |
| `shutdown()` | Shutdown protocol manager | None | `None` |

#### Usage Example

```python
from dmsc import (
    DMSCProtocolManager, DMSCProtocolConfig,
    DMSCProtocolType
)

# Initialize protocol manager
config = DMSCProtocolConfig(
    default_protocol=DMSCProtocolType.GLOBAL,
    enable_security=True,
    enable_state_sync=True
)
protocol_manager = DMSCProtocolManager()
await protocol_manager.initialize(config)

# Send message using default protocol
response = await protocol_manager.send_message(
    target="device-001",
    message=b"Hello, DMSC Protocol!"
)
print(f"Response: {response}")

# Switch to private protocol
await protocol_manager.switch_protocol(DMSCProtocolType.PRIVATE)

# Send secure message
secure_response = await protocol_manager.send_message(
    target="secure-device",
    message=b"Sensitive data"
)
```

### DMSCProtocolConfig

Protocol configuration.

```python
from dmsc import (
    DMSCProtocolConfig, DMSCProtocolType,
    DMSCConnectionState, DMSCDataClassification
)

config = DMSCProtocolConfig(
    default_protocol=DMSCProtocolType.GLOBAL,
    enable_security=True,
    enable_state_sync=True,
    performance_optimization=True,
    connection_timeout=30,
    max_connections_per_protocol=1000,
    protocol_switching_enabled=True,
    auto_reconnect=True,
    heartbeat_interval=30,
    heartbeat_timeout=10,
    max_retry_attempts=3,
    retry_delay_ms=1000,
    encryption_algorithm="aes256",
    compression_algorithm="lz4"
)
```

### DMSCProtocolType

Protocol type enumeration.

```python
from dmsc import DMSCProtocolType

# Global protocol - standard communication
global_protocol = DMSCProtocolType.GLOBAL

# Private protocol - enhanced security for sensitive operations
private_protocol = DMSCProtocolType.PRIVATE

# Custom protocol - user-defined protocol
custom_protocol = DMSCProtocolType.CUSTOM
```

## Protocol Frames

### DMSCFrame

Protocol frame representation.

```python
from dmsc import DMSCFrame, DMSCFrameType, DMSCFrameHeader

# Create data frame
header = DMSCFrameHeader(
    frame_type=DMSCFrameType.DATA,
    sequence_number=1,
    flags={"compressed": False, "encrypted": True}
)
frame = DMSCFrame(
    header=header,
    payload=b"Hello, DMSC!",
    checksum=0xABCDEF
)

# Send frame
await protocol_manager.send_frame("device-001", frame)

# Receive frame
received_frame = await protocol_manager.receive_frame("device-001")
print(f"Frame type: {received_frame.header.frame_type}")
print(f"Payload: {received_frame.payload}")
```

### Frame Types

```python
from dmsc import DMSCFrameType

# Data frame - standard data transmission
data_frame = DMSCFrameType.DATA

# Control frame - control commands
control_frame = DMSCFrameType.CONTROL

# Acknowledgment frame - acknowledgment of receipt
ack_frame = DMSCFrameType.ACK

# Heartbeat frame - connection keep-alive
heartbeat_frame = DMSCFrameType.HEARTBEAT

# Error frame - error reporting
error_frame = DMSCFrameType.ERROR

# Handshake frame - connection establishment
handshake_frame = DMSCFrameType.HANDSHAKE

# Disconnect frame - connection termination
disconnect_frame = DMSCFrameType.DISCONNECT
```

## Connection Management

### Connection States

```python
from dmsc import DMSCConnectionState

# Connection states
disconnected = DMSCConnectionState.DISCONNECTED
connecting = DMSCConnectionState.CONNECTING
connected = DMSCConnectionState.CONNECTED
reconnecting = DMSCConnectionState.RECONNECTING
disconnecting = DMSCConnectionState.DISCONNECTING
error = DMSCConnectionState.ERROR
```

### Connection Statistics

```python
from dmsc import DMSCProtocolManager

protocol_manager = DMSCProtocolManager()
await protocol_manager.initialize()

# Get connection statistics
stats = protocol_manager.get_connection_stats()
print(f"Total connections: {stats.total_connections}")
print(f"Active connections: {stats.active_connections}")
print(f"Failed connections: {stats.failed_connections}")
print(f"Bytes sent: {stats.bytes_sent}")
print(f"Bytes received: {stats.bytes_received}")
print(f"Messages sent: {stats.messages_sent}")
print(f"Messages received: {stats.messages_received}")
```

## Security Features

### Encryption

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

# Configure encryption
config = DMSCProtocolConfig(
    default_protocol=DMSCProtocolType.GLOBAL,
    enable_security=True,
    encryption_algorithm="aes256-gcm",
    key_exchange_algorithm="ecdh",
    certificate_verification=True
)
protocol_manager = DMSCProtocolManager()
await protocol_manager.initialize(config)

# All messages are now encrypted
response = await protocol_manager.send_message("device-001", b"Sensitive data")
```

### Authentication

```python
from dmsc import DMSCDeviceAuthProtocol

# Device authentication
auth_protocol = DMSCDeviceAuthProtocol()

# Authenticate device
auth_result = await auth_protocol.authenticate(
    device_id="device-001",
    credentials={"api_key": "your-api-key"},
    timeout=30
)

if auth_result.success:
    print(f"Device authenticated: {auth_result.device_id}")
    print(f"Certificate: {auth_result.certificate}")
else:
    print(f"Authentication failed: {auth_result.error}")
```

## Protocol Switching

```python
from dmsc import DMSCProtocolManager, DMSCProtocolType

protocol_manager = DMSCProtocolManager()
await protocol_manager.initialize()

# Start with global protocol
current = protocol_manager.get_current_protocol()
print(f"Current protocol: {current}")  # GLOBAL

# Switch to private protocol for sensitive operations
await protocol_manager.switch_protocol(DMSCProtocolType.PRIVATE)
current = protocol_manager.get_current_protocol()
print(f"Switched to: {current}")  # PRIVATE

# Perform sensitive operations
secure_response = await protocol_manager.send_message(
    "secure-server",
    b"Confidential command"
)

# Switch back to global protocol
await protocol_manager.switch_protocol(DMSCProtocolType.GLOBAL)
```

## Performance Monitoring

```python
from dmsc import DMSCProtocolManager

protocol_manager = DMSCProtocolManager()
await protocol_manager.initialize()

# Get protocol statistics
stats = protocol_manager.get_stats()

print(f"Total messages sent: {stats.total_messages_sent}")
print(f"Total messages received: {stats.total_messages_received}")
print(f"Bytes sent: {stats.total_bytes_sent}")
print(f"Bytes received: {stats.total_bytes_received}")
print(f"Average latency: {stats.average_latency_ms}ms")
print(f"Error count: {stats.error_count}")
print(f"Success rate: {stats.success_rate * 100:.2f}%")
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
