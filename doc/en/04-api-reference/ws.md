<div align="center">

# WebSocket API Reference

**Version: 0.1.5**

**Last modified date: 2026-01-16**

The WebSocket module provides full-duplex communication channels over TCP connections with session management, message handling, and Python bindings support.

## Module Overview

</div>

The WebSocket module includes the following sub-modules:

- **handler**: WebSocket connection handlers with event callbacks
- **session**: Session management and state tracking
- **message**: Message encoding and decoding
- **broadcast**: Message broadcasting to multiple clients
- **heartbeat**: Connection keep-alive and health monitoring

<div align="center">

## Core Components

</div>

### DMSCWSPythonHandler

Python-compatible WebSocket handler with event callback support.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `on_connect` | `Option<PyCallable>` | Connection established callback |
| `on_disconnect` | `Option<PyCallable>` | Connection closed callback |
| `on_message` | `Option<PyCallable>` | Message received callback |
| `on_error` | `Option<PyCallable>` | Error occurred callback |
| `on_ping` | `Option<PyCallable>` | Ping received callback |
| `on_pong` | `Option<PyCallable>` | Pong received callback |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(on_connect, on_disconnect, on_message, on_error)` | Create handler | See fields | `Self` |
| `set_on_connect(&mut self, callback)` | Set connect callback | `callback: PyCallable` | `()` |
| `set_on_disconnect(&mut self, callback)` | Set disconnect callback | `callback: PyCallable` | `()` |
| `set_on_message(&mut self, callback)` | Set message callback | `callback: PyCallable` | `()` |
| `set_on_error(&mut self, callback)` | Set error callback | `callback: PyCallable` | `()` |

#### Python Usage Example

```python
from dmsc.ws import DMSCWSPythonHandler

def on_connect(session_id: str, remote_addr: str):
    print(f"Client connected: {session_id} from {remote_addr}")

def on_disconnect(session_id: str):
    print(f"Client disconnected: {session_id}")

def on_message(session_id: str, data: bytes) -> bytes:
    print(f"Received from {session_id}: {data.decode()}")
    return b"Echo: " + data

def on_error(session_id: str, error: str):
    print(f"Error for {session_id}: {error}")

handler = DMSCWSPythonHandler(
    on_connect=on_connect,
    on_disconnect=on_disconnect,
    on_message=on_message,
    on_error=on_error
)
```

### DMSCWSSessionManagerPy

Python-compatible session manager for WebSocket connections.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `max_connections` | `u32` | Maximum concurrent connections | `10000` |
| `session_timeout` | `Duration` | Session timeout duration | `300s` |
| `heartbeat_interval` | `Duration` | Heartbeat check interval | `30s` |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(max_connections)` | Create session manager | `max_connections: u32` | `Self` |
| `get_session(session_id)` | Get session by ID | `session_id: &str` | `DMSCResult<Option<DMSCWSSession>>` |
| `get_all_sessions()` | Get all active sessions | None | `DMSCResult<Vec<DMSCWSSession>>` |
| `get_session_count()` | Get active session count | None | `DMSCResult<u32>` |
| `broadcast(message, exclude)` | Broadcast message | `message: &[u8]`, `exclude: Option<&str>` | `DMSCResult<u64>` |
| `send_to(session_id, message)` | Send to specific session | `session_id: &str`, `message: &[u8]` | `DMSCResult<()>` |
| `close_session(session_id)` | Close a session | `session_id: &str` | `DMSCResult<()>` |
| `close_all_sessions()` | Close all sessions | None | `DMSCResult<()>` |

#### Python Usage Example

```python
from dmsc.ws import DMSCWSSessionManagerPy

# Create session manager
manager = DMSCWSSessionManagerPy(max_connections=1000)

# Get all active sessions
sessions = manager.get_all_sessions()
print(f"Active sessions: {len(sessions)}")

# Get session count
count = manager.get_session_count()
print(f"Session count: {count}")

# Send message to specific session
manager.send_to("session-123", b"Hello, client!")

# Broadcast to all clients
manager.broadcast(b"Server announcement: Maintenance in 10 minutes")

# Broadcast excluding specific session
manager.broadcast(b"Hello everyone except session-123", exclude="session-123")

# Close specific session
manager.close_session("session-123")

# Close all sessions
manager.close_all_sessions()
```

### DMSCWSSession

WebSocket session representing a client connection.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Unique session identifier |
| `remote_addr` | `String` | Remote client address |
| `local_addr` | `String` | Local server address |
| `connected_at` | `DateTime<Utc>` | Connection timestamp |
| `last_active_at` | `DateTime<Utc>` | Last activity timestamp |
| `is_connected` | `bool` | Connection status |
| `metadata` | `HashMap<String, String>` | Session metadata |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `id(&self)` | Get session ID | None | `&str` |
| `remote_addr(&self)` | Get remote address | None | `&str` |
| `is_connected(&self)` | Check connection status | None | `bool` |
| `send(&self, message)` | Send message | `message: &[u8]` | `DMSCResult<()>` |
| `close(&self)` | Close connection | None | `DMSCResult<()>` |
| `set_metadata(&mut self, key, value)` | Set metadata | `key: &str`, `value: &str` | `()` |
| `get_metadata(&self, key)` | Get metadata | `key: &str` | `Option<&str>` |

#### Usage Example

```rust
use dmsc::prelude::*;

let session = DMSCWSSession::new(
    "session-123".to_string(),
    "192.168.1.100:54321".to_string(),
    "0.0.0.0:8080".to_string(),
)?;

// Check connection status
if session.is_connected() {
    // Send message
    session.send(b"Welcome!")?;
    
    // Set metadata
    session.set_metadata("username", "alice");
    session.set_metadata("room", "general");
}

// Get metadata
if let Some(username) = session.get_metadata("username") {
    println!("User: {}", username);
}

// Close session
session.close()?;
```

<div align="center>

## WebSocket Server

</div>

### DMSCWebSocketServer

WebSocket server for accepting and managing connections.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create server | `config: DMSCWebSocketConfig` | `DMSCResult<Self>` |
| `start(&self)` | Start the server | None | `DMSCResult<()>` |
| `stop(&self)` | Stop the server | None | `DMSCResult<()>` |
| `is_running(&self)` | Check if running | None | `bool` |
| `set_handler(&mut self, handler)` | Set message handler | `handler: impl DMSCWebSocketHandler` | `()` |
| `set_session_manager(&mut self, manager)` | Set session manager | `manager: impl DMSCWebSocketSessionManager` | `()` |
| `broadcast(&self, message)` | Broadcast to all | `message: &[u8]` | `DMSCResult<u64>` |
| `get_connections(&self)` | Get all connections | None | `DMSCResult<Vec<DMSCWSSession>>` |

### DMSCWebSocketConfig

WebSocket server configuration.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | Bind host | `"0.0.0.0"` |
| `port` | `u16` | Bind port | `8080` |
| `max_connections` | `u32` | Max concurrent connections | `10000` |
| `connection_timeout` | `Duration` | Connection timeout | `60s` |
| `heartbeat_interval` | `Duration` | Heartbeat interval | `30s` |
| `max_message_size` | `u64` | Max message size (bytes) | `65536` |
| `ping_interval` | `Duration` | Ping interval | `25s` |
| `ping_timeout` | `Duration` | Ping timeout | `10s` |
| `tls_enabled` | `bool` | Enable WSS | `false` |
| `tls_cert_path` | `Option<String>` | TLS certificate path | `None` |
| `tls_key_path` | `Option<String>` | TLS key path | `None` |

#### Usage Example

```rust
use dmsc::prelude::*;

let config = DMSCWebSocketConfig {
    host: "0.0.0.0".to_string(),
    port: 8080,
    max_connections: 10000,
    connection_timeout: Duration::from_secs(60),
    heartbeat_interval: Duration::from_secs(30),
    max_message_size: 65536,
    ping_interval: Duration::from_secs(25),
    ping_timeout: Duration::from_secs(10),
    tls_enabled: false,
    tls_cert_path: None,
    tls_key_path: None,
};

let mut server = DMSCWebSocketServer::new(config)?;
server.set_handler(my_handler);
server.start()?;

println!("WebSocket server started on {}:{}", config.host, config.port);

// Server runs until stopped
server.stop()?;
```

<div align="center

## Message Handling

</div>

### DMSCWebSocketMessage

Message types for WebSocket communication.

#### Message Types

| Type | Description |
|:--------|:-------------|
| `Text` | Text message (UTF-8 encoded) |
| `Binary` | Binary data |
| `Close` | Connection close frame |
| `Ping` | Ping frame (keep-alive) |
| `Pong` | Pong frame (keep-alive response) |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new_text(data)` | Create text message | `data: String` | `Self` |
| `new_binary(data)` | Create binary message | `data: Vec<u8>` | `Self` |
| `is_text(&self)` | Check if text | None | `bool` |
| `is_binary(&self)` | Check if binary | None | `bool` |
| `is_close(&self)` | Check if close | None | `bool` |
| `is_ping(&self)` | Check if ping | None | `bool` |
| `is_pong(&self)` | Check if pong | None | `bool` |
| `into_text(self)` | Convert to text | None | `DMSCResult<String>` |
| `into_binary(self)` | Convert to binary | None | `DMSCResult<Vec<u8>>` |

#### Usage Example

```rust
use dmsc::prelude::*;

let text_msg = DMSCWebSocketMessage::new_text("Hello, WebSocket!".to_string());
assert!(text_msg.is_text());

let binary_msg = DMSCWebSocketMessage::new_binary(vec![0x01, 0x02, 0x03]);
assert!(binary_msg.is_binary());

// Handle incoming message
match message {
    DMSCWebSocketMessage::Text(text) => {
        println!("Received text: {}", text);
    }
    DMSCWebSocketMessage::Binary(data) => {
        println!("Received binary: {:?}", data);
    }
    DMSCWebSocketMessage::Close(code, reason) => {
        println!("Client closed: {} - {}", code, reason);
    }
    DMSCWebSocketMessage::Ping(data) => {
        println!("Received ping: {:?}", data);
    }
    DMSCWebSocketMessage::Pong(data) => {
        println!("Received pong: {:?}", data);
    }
}
```

<div align="center

## Broadcasting

</div>

### DMSCWebSocketBroadcaster

Message broadcasting to multiple sessions.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create broadcaster | None | `Self` |
| `broadcast(&self, message)` | Broadcast to all | `message: &[u8]` | `DMSCResult<u64>` |
| `broadcast_to(&self, message, sessions)` | Broadcast to specific | `message: &[u8]`, `sessions: &[String]` | `DMSCResult<u64>` |
| `broadcast_excluding(&self, message, exclude)` | Broadcast excluding | `message: &[u8]`, `exclude: &str` | `DMSCResult<u64>` |
| `add_session(&mut self, session)` | Add session | `session: DMSCWSSession` | `()` |
| `remove_session(&mut self, session_id)` | Remove session | `session_id: &str` | `()` |
| `get_session_count(&self)` | Get session count | None | `usize` |

#### Usage Example

```rust
use dmsc::prelude::*;

let mut broadcaster = DMSCWebSocketBroadcaster::new();

// Add sessions
broadcaster.add_session(session1.clone());
broadcaster.add_session(session2.clone());
broadcaster.add_session(session3.clone());

// Broadcast to all
let count = broadcaster.broadcast(b"Hello, everyone!")?;
println!("Message sent to {} clients", count);

// Broadcast to specific sessions
let targets = vec!["session1".to_string(), "session2".to_string()];
broadcaster.broadcast_to(b"Hello, specific users!", &targets)?;

// Broadcast excluding one session
broadcaster.broadcast_excluding(b"Hello everyone except session1", "session1")?;

// Remove session
broadcaster.remove_session("session1");
```

<div align="center

## Heartbeat and Health Checks

</div>

### DMSCWebSocketHeartbeat

Connection keep-alive mechanism.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(interval, timeout)` | Create heartbeat | `interval: Duration`, `timeout: Duration` | `Self` |
| `start(&self, session_id, sender)` | Start heartbeat | `session_id: &str`, `sender: impl Send + Clone` | `()` |
| `stop(&self, session_id)` | Stop heartbeat | `session_id: &str` | `()` |
| `check_alive(&self, session_id)` | Check if alive | `session_id: &str` | `DMSCResult<bool>` |
| `get_last_pong(&self, session_id)` | Get last pong time | `session_id: &str` | `DMSCResult<Option<DateTime<Utc>>>` |

#### Usage Example

```rust
use dmsc::prelude::*;

let heartbeat = DMSCWebSocketHeartbeat::new(
    Duration::from_secs(30),
    Duration::from_secs(10),
);

// Start heartbeat for session
heartbeat.start("session-123", sender.clone());

// Check if session is alive
let is_alive = heartbeat.check_alive("session-123")?;
println!("Session is alive: {}", is_alive);

// Get last pong time
if let Some(last_pong) = heartbeat.get_last_pong("session-123")? {
    println!("Last pong: {:?}", last_pong);
}

// Stop heartbeat
heartbeat.stop("session-123");
```

<div align="center

## Error Handling

</div>

### DMSCWebSocketError

WebSocket-specific errors.

| Error Code | Description |
|:--------|:-------------|
| `WS_CONNECTION_ERROR` | Connection error |
| `WS_MESSAGE_TOO_LARGE` | Message exceeds max size |
| `WS_SESSION_NOT_FOUND` | Session not found |
| `WS_SESSION_CLOSED` | Session already closed |
| `WS_SEND_ERROR` | Failed to send message |
| `WS_TIMEOUT` | Operation timeout |
| `WS_TLS_ERROR` | TLS/certificate error |
| `WS_PROTOCOL_ERROR` | WebSocket protocol error |

#### Usage Example

```rust
use dmsc::prelude::*;

match session.send(message) {
    Ok(_) => {
        println!("Message sent successfully");
    }
    Err(DMSCWebSocketError::SessionClosed) => {
        println!("Session already closed, cleaning up");
    }
    Err(DMSCWebSocketError::SendError(e)) => {
        println!("Failed to send: {}", e);
    }
    Err(e) => {
        println!("WebSocket error: {}", e);
    }
}
```

<div align="center

## Python Support

</div>

The WebSocket module provides full Python bindings through PyO3:

```python
from dmsc.ws import (
    DMSCWSPythonHandler,
    DMSCWSSessionManagerPy,
    DMSCWebSocketConfig
)

# Create handler with callbacks
def on_connect(session_id, remote_addr):
    print(f"Connected: {session_id} from {remote_addr}")

def on_disconnect(session_id):
    print(f"Disconnected: {session_id}")

def on_message(session_id, data):
    print(f"Message from {session_id}: {data.decode()}")
    return b"Echo: " + data

def on_error(session_id, error):
    print(f"Error for {session_id}: {error}")

handler = DMSCWSPythonHandler(
    on_connect=on_connect,
    on_disconnect=on_disconnect,
    on_message=on_message,
    on_error=on_error
)

# Create session manager
manager = DMSCWSSessionManagerPy(max_connections=1000)

# Get session info
sessions = manager.get_all_sessions()
count = manager.get_session_count()

# Broadcast to all clients
manager.broadcast(b"Server announcement")

# Send to specific client
manager.send_to("session-123", b"Hello!")
```

<div align="center

## Best Practices

</div>

1. **Implement heartbeat**: Use heartbeat mechanism to detect dead connections
2. **Set message limits**: Configure max message size to prevent memory issues
3. **Handle disconnections**: Properly handle client disconnections and cleanup
4. **Use session metadata**: Store user info in session metadata for identification
5. **Broadcast efficiently**: Use broadcaster for multi-client messaging
6. **Implement reconnection**: Handle client reconnection scenarios
7. **Monitor connections**: Track connection counts and health metrics
8. **Enable TLS in production**: Use WSS for secure connections in production

<div align="center

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [grpc](./grpc.md): gRPC module providing RPC functionality
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [mq](./mq.md): Message queue module providing message queue support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
