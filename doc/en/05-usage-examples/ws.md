<div align="center">

# WebSocket Usage Examples

**Version: 0.1.7**

**Last modified date: 2026-01-16**

This example demonstrates how to use DMSC's WebSocket module for real-time bidirectional communication with session management, message handling, broadcasting, and Python bindings support.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- WebSocket server with connection management
- Python handler support for dynamic message processing
- Session management and state tracking
- Message broadcasting to multiple clients
- Heartbeat mechanism for connection health
- Connection timeout and cleanup

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of WebSocket concepts
- Python 3.8+ (for Python bindings examples)

<div align="center>

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-websocket-example
cd dms-websocket-example
```

### 2. Add Dependencies

Add the following dependencies to the `Cargo.toml` file:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pyo3 = { version = "0.20", features = ["extension-module"] }
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

```yaml
service:
  name: "dms-websocket-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

websocket:
  host: "0.0.0.0"
  port: 8080
  max_connections: 10000
  connection_timeout: 60
  heartbeat_interval: 30
  max_message_size: 65536
  ping_interval: 25
  ping_timeout: 10
  tls_enabled: false
```

### 4. Write Main Code

Replace the `src/main.rs` file with the following content:

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_websocket(DMSCWebSocketConfig::default())?
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC WebSocket Example started")?;
        
        let server = ctx.websocket_server();
        let session_manager = ctx.websocket_session_manager();
        
        let broadcast_manager = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
        let broadcast_manager_clone = broadcast_manager.clone();
        
        struct ChatMessageHandler {
            broadcast_manager: Arc<Mutex<HashMap<String, Vec<String>>>>,
        }
        
        impl DMSCWebSocketHandler for ChatMessageHandler {
            fn on                &self,
_connect(
                session: &DMSCWSSession,
            ) -> DMSCResult<()> {
                let mut manager = self.broadcast_manager.lock().unwrap();
                manager.insert(session.id().to_string(), Vec::new());
                
                println!("Client connected: {} from {}", session.id(), session.remote_addr());
                Ok(())
            }
            
            fn on_disconnect(
                &self,
                session: &DMSCWSSession,
            ) -> DMSCResult<()> {
                let mut manager = self.broadcast_manager.lock().unwrap();
                manager.remove(session.id());
                
                println!("Client disconnected: {}", session.id());
                Ok(())
            }
            
            fn on_message(
                &self,
                session: &DMSCWSSession,
                message: &DMSCWebSocketMessage,
            ) -> DMSCResult<Option<DMSCWebSocketMessage>> {
                if let DMSCWebSocketMessage::Text(text) = message {
                    println!("Received from {}: {}", session.id(), text);
                    
                    let response = format!("Echo: {}", text);
                    Some(DMSCWebSocketMessage::new_text(response))
                } else {
                    Ok(None)
                }
            }
            
            fn on_error(
                &self,
                session: &DMSCWSError,
            ) -> DMSCResult<()> {
                println!("WebSocket error: {}", session);
                Ok(())
            }
        }
        
        let handler = ChatMessageHandler {
            broadcast_manager: broadcast_manager_clone,
        };
        
        server.set_handler(handler);
        server.start()?;
        
        ctx.logger().info("websocket", &format!("WebSocket server started on port {}", 8080))?;
        
        Ok(())
    }).await
}
```

### 5. Python Handler Example

Create a `python_handler.py` file for Python-based WebSocket handlers:

```python
import sys
sys.path.insert(0, 'target/release')

from dmsc.ws import DMSCWSPythonHandler, DMSCWSSessionManagerPy

class ChatPythonHandler:
    def __init__(self):
        self.sessions = {}
        
    def on_connect(self, session_id: str, remote_addr: str):
        self.sessions[session_id] = {
            'remote_addr': remote_addr,
            'connected_at': '2024-01-15T10:00:00Z'
        }
        print(f"Client connected: {session_id} from {remote_addr}")
        
    def on_disconnect(self, session_id: str):
        if session_id in self.sessions:
            del self.sessions[session_id]
        print(f"Client disconnected: {session_id}")
        
    def on_message(self, session_id: str, data: bytes) -> bytes:
        message = data.decode('utf-8')
        print(f"Received from {session_id}: {message}")
        
        response = f"Echo: {message}"
        return response.encode('utf-8')
        
    def on_error(self, session_id: str, error: str):
        print(f"Error for {session_id}: {error}")
        
    def create_handler(self):
        return DMSCWSPythonHandler(
            on_connect=self.on_connect,
            on_disconnect=self.on_disconnect,
            on_message=self.on_message,
            on_error=self.on_error
        )

if __name__ == "__main__":
    handler = ChatPythonHandler()
    ws_handler = handler.create_handler()
    
    manager = DMSCWSSessionManagerPy(max_connections=1000)
    
    print("WebSocket Python handler initialized")
    print(f"Active sessions: {len(handler.sessions)}")
    
    import time
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("Shutting down Python handler")
```

### 6. Create HTML Client

Create an `index.html` file for browser-based WebSocket client:

```html
<!DOCTYPE html>
<html>
<head>
    <title>DMSC WebSocket Client</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        #status { padding: 10px; margin: 10px 0; border-radius: 5px; }
        #messages { 
            border: 1px solid #ccc; 
            height: 300px; 
            overflow-y: scroll; 
            padding: 10px;
            margin: 10px 0;
        }
        #message { width: 70%; padding: 10px; }
        #send { width: 25%; padding: 10px; }
        .connected { background-color: #d4edda; color: #155724; }
        .disconnected { background-color: #f8d7da; color: #721c24; }
        .sent { color: #0066cc; }
        .received { color: #009933; }
    </style>
</head>
<body>
    <h1>DMSC WebSocket Client</h1>
    <div id="status" class="disconnected">Disconnected</div>
    <div id="messages"></div>
    <input type="text" id="message" placeholder="Enter message...">
    <button id="send" onclick="sendMessage()">Send</button>
    
    <script>
        let ws;
        
        function connect() {
            ws = new WebSocket('ws://localhost:8080/ws');
            
            ws.onopen = function() {
                document.getElementById('status').textContent = 'Connected';
                document.getElementById('status').className = 'connected';
                addMessage('System', 'Connected to server', 'received');
            };
            
            ws.onclose = function() {
                document.getElementById('status').textContent = 'Disconnected';
                document.getElementById('status').className = 'disconnected';
                addMessage('System', 'Disconnected from server', 'received');
            };
            
            ws.onmessage = function(event) {
                addMessage('Server', event.data, 'received');
            };
            
            ws.onerror = function(error) {
                addMessage('Error', 'WebSocket error occurred', 'received');
            };
        }
        
        function sendMessage() {
            const message = document.getElementById('message').value;
            if (message && ws.readyState === WebSocket.OPEN) {
                ws.send(message);
                addMessage('You', message, 'sent');
                document.getElementById('message').value = '';
            }
        }
        
        function addMessage(sender, text, type) {
            const messages = document.getElementById('messages');
            const div = document.createElement('div');
            div.textContent = `[${new Date().toLocaleTimeString()}] ${sender}: ${text}`;
            div.className = type;
            messages.appendChild(div);
            messages.scrollTop = messages.scrollHeight;
        }
        
        document.getElementById('message').addEventListener('keypress', function(e) {
            if (e.key === 'Enter') sendMessage();
        });
        
        connect();
    </script>
</body>
</html>
```

<div align="center">

## Code Analysis

</div>

### WebSocket Server Setup

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
    ..Default::default()
};

let mut server = DMSCWebSocketServer::new(config)?;
server.start()?;
```

### Session Management

```rust
use dmsc::prelude::*;

let session_manager = ctx.websocket_session_manager();

// Get all active sessions
let sessions = session_manager.get_all_sessions()?;
println!("Active sessions: {}", sessions.len());

// Get session count
let count = session_manager.get_session_count()?;
println!("Total connections: {}", count);

// Send to specific session
session_manager.send_to("session-123", b"Hello, client!")?;

// Broadcast to all clients
session_manager.broadcast(b"Server announcement")?;

// Broadcast excluding specific session
session_manager.broadcast(b"Hello everyone", exclude="session-123")?;

// Close specific session
session_manager.close_session("session-123")?;

// Close all sessions
session_manager.close_all_sessions()?;
```

### Python Handler Setup

```python
from dmsc.ws import DMSCWSPythonHandler

def on_connect(session_id, remote_addr):
    print(f"Client connected: {session_id} from {remote_addr}")

def on_disconnect(session_id):
    print(f"Client disconnected: {session_id}")

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
```

### Message Broadcasting

```rust
use dmsc::prelude::*;

let mut broadcaster = DMSCWebSocketBroadcaster::new();

// Add sessions to broadcaster
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
broadcaster.broadcast_excluding(b"Hello except session1", "session1")?;
```

### Heartbeat Configuration

```rust
use dmsc::prelude::*;

let heartbeat = DMSCWebSocketHeartbeat::new(
    Duration::from_secs(30),  // ping interval
    Duration::from_secs(10),  // timeout
);

// Start heartbeat for session
heartbeat.start("session-123", sender.clone());

// Check if session is alive
let is_alive = heartbeat.check_alive("session-123")?;
println!("Session alive: {}", is_alive);

// Get last pong time
if let Some(last_pong) = heartbeat.get_last_pong("session-123")? {
    println!("Last pong: {:?}", last_pong);
}

// Stop heartbeat
heartbeat.stop("session-123");
```

<div align="center>

## Running Steps

</div>

### 1. Build Project

```bash
cargo build --release
```

### 2. Run Project

```bash
cargo run
```

### 3. Test with Browser Client

```bash
# Open index.html in a web browser
# The browser will connect to ws://localhost:8080/ws
```

### 4. Test with Python Client

```python
import websocket
import threading

def on_message(ws, message):
    print(f"Received: {message}")

def on_error(ws, error):
    print(f"Error: {error}")

def on_close(ws, close_status_code, close_msg):
    print("Connection closed")

def on_open(ws):
    print("Connection established")
    ws.send("Hello, WebSocket!")

if __name__ == "__main__":
    ws = websocket.WebSocketApp(
        "ws://localhost:8080/ws",
        on_open=on_open,
        on_message=on_message,
        on_error=on_error,
        on_close=on_close
    )
    
    ws.run_forever()
```

<div align="center>

## Expected Results

</div>

After running the example, you should see output similar to the following:

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC WebSocket Example started","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"websocket","message":"WebSocket server started on port 8080","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"websocket","message":"Client connected: abc123 from 192.168.1.100:54321","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:02Z","level":"info","module":"websocket","message":"Received from abc123: Hello, WebSocket!","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:02Z","level":"info","module":"websocket","message":"Response sent to abc123: Echo: Hello, WebSocket!","trace_id":"abc123","span_id":"def456"}
```

<div align="center>

## Extended Features

</div>

### 1. Chat Room Implementation

```rust
use dmsc::prelude::*;
use std::collections::HashMap;

struct ChatRoom {
    rooms: HashMap<String, HashSet<String>>,
    user_rooms: HashMap<String, String>,
}

impl ChatRoom {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            user_rooms: HashMap::new(),
        }
    }
    
    fn join_room(&mut self, session_id: &str, room_name: &str) {
        self.rooms
            .entry(room_name.to_string())
            .or_insert_with(HashSet::new)
            .insert(session_id.to_string());
        self.user_rooms.insert(session_id.to_string(), room_name.to_string());
    }
    
    fn leave_room(&mut self, session_id: &str) {
        if let Some(room_name) = self.user_rooms.remove(session_id) {
            if let Some(room) = self.rooms.get_mut(&room_name) {
                room.remove(session_id);
                if room.is_empty() {
                    self.rooms.remove(&room_name);
                }
            }
        }
    }
    
    fn broadcast_to_room(&self, room_name: &str, message: &[u8], exclude: Option<&str>) -> DMSCResult<u64> {
        if let Some(room) = self.rooms.get(room_name) {
            let mut count = 0;
            for session_id in room {
                if exclude.map(|e| e == session_id).unwrap_or(false) {
                    continue;
                }
                // Send message to session
                count += 1;
            }
            Ok(count)
        } else {
            Ok(0)
        }
    }
}
```

### 2. Authentication Middleware

```rust
use dmsc::prelude::*;

struct AuthMiddleware;

impl DMSCWebSocketHandler for AuthMiddleware {
    fn on_connect(
        &self,
        session: &DMSCWSSession,
    ) -> DMSCResult<()> {
        // Extract auth token from query params or headers
        let token = session.get_metadata("auth_token");
        
        match validate_token(&token) {
            Ok(user_id) => {
                session.set_metadata("user_id", &user_id);
                Ok(())
            }
            Err(_) => Err(DMSCError::unauthorized("Invalid token"))
        }
    }
    
    fn on_message(
        &self,
        session: &DMSCWSSession,
        message: &DMSCWebSocketMessage,
    ) -> DMSCResult<Option<DMSCWebSocketMessage>> {
        // Check if user is authenticated
        if session.get_metadata("user_id").is_none() {
            return Err(DMSCError::unauthorized("Not authenticated"));
        }
        
        // Continue to next handler
        Ok(None)
    }
}

fn validate_token(token: &Option<String>) -> DMSCResult<String> {
    if let Some(t) = token {
        if t.starts_with("user_") {
            Ok(t.clone())
        } else {
            Err(DMSCError::unauthorized("Invalid token format"))
        }
    } else {
        Err(DMSCError::unauthorized("Missing token"))
    }
}
```

### 3. Rate Limiting

```rust
use dmsc::prelude::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

struct RateLimiter {
    limits: HashMap<String, (u64, Instant)>,
    max_requests: u64,
    window: Duration,
}

impl RateLimiter {
    fn new(max_requests: u64, window: Duration) -> Self {
        Self {
            limits: HashMap::new(),
            max_requests,
            window,
        }
    }
    
    fn allow(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        
        if let Some((count, start)) = self.limits.get(client_id) {
            if now.duration_since(start) > self.window {
                // Window expired, reset
                self.limits.insert(client_id.to_string(), (1, now));
                return true;
            }
            
            if *count >= self.max_requests {
                return false;
            }
            
            // Increment count
            self.limits.insert(client_id.to_string(), (*count + 1, *start));
            true
        } else {
            // First request
            self.limits.insert(client_id.to_string(), (1, now));
            true
        }
    }
}
```

### 4. TLS/WSS Support

```rust
use dmsc::prelude::*;

let config = DMSCWebSocketConfig {
    host: "0.0.0.0".to_string(),
    port: 8443,
    tls_enabled: true,
    tls_cert_path: Some("/path/to/server.crt".to_string()),
    tls_key_path: Some("/path/to/server.key".to_string()),
    ..Default::default()
};
```

<div align="center>

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
9. **Use compression**: Enable per-message deflate for large payloads
10. **Validate messages**: Validate incoming message format and size

<div align="center

## Python Integration

</div>

### Python Session Management

```python
from dmsc.ws import DMSCWSSessionManagerPy

# Create session manager
manager = DMSCWSSessionManagerPy(max_connections=1000)

# Get all active sessions
sessions = manager.get_all_sessions()
for session in sessions:
    print(f"Session: {session.id} from {session.remote_addr}")

# Get session count
count = manager.get_session_count()
print(f"Total connections: {count}")

# Broadcast to all clients
manager.broadcast(b"Server announcement: Maintenance in 10 minutes")

# Send to specific client
manager.send_to("session-123", b"Hello, client!")

# Close specific session
manager.close_session("session-123")

# Close all sessions
manager.close_all_sessions()
```

### Python Client Implementation

```python
import asyncio
import websockets

async def client():
    uri = "ws://localhost:8080/ws"
    
    async with websockets.connect(uri) as websocket:
        print("Connected to server")
        
        # Send message
        await websocket.send("Hello, WebSocket!")
        print("Sent: Hello, WebSocket!")
        
        # Receive response
        response = await websocket.recv()
        print(f"Received: {response}")
        
        # Keep connection alive
        try:
            while True:
                message = await asyncio.wait_for(websocket.recv(), timeout=30)
                print(f"Received: {message}")
        except asyncio.TimeoutError:
            print("No message received for 30 seconds")

if __name__ == "__main__":
    asyncio.run(client())
```

<div align="center

## Summary

</div>

This example demonstrates how to use the DMSC WebSocket module for:

- Creating WebSocket servers with connection management
- Implementing Python-based handlers with PyO3 bindings
- Session management and state tracking
- Message broadcasting to multiple clients
- Heartbeat mechanism for connection health
- Connection timeout and cleanup
- Chat room implementation
- Authentication middleware
- Rate limiting
- TLS/WSS support

Through this example, you should have mastered the core functions and usage methods of the DMSC WebSocket module. You can build more complex real-time applications based on this foundation.

<div align="center

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation for all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [database](./database.md): Database examples, learn database connection and query operations
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [protocol](./protocol.md): Protocol examples, implement custom communication protocols
- [service_mesh](./service_mesh.md): Service mesh examples, implement inter-service communication
