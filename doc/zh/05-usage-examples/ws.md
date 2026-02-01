<div align="center">

# WebSocket 使用示例

**Version: 0.1.6**

**Last modified date: 2026-01-16**

本示例演示如何使用 DMSC 的 WebSocket 模块实现实时双向通信，包含会话管理、消息处理、广播功能，以及 Python 绑定支持。

## 示例概述

</div>

本示例将创建一个 DMSC 应用程序，实现以下功能：

- 带连接管理的 WebSocket 服务器
- 支持 Python 处理器的动态消息处理
- 会话管理和状态跟踪
- 向多个客户端广播消息
- 用于连接健康的保活机制
- 连接超时和清理

<div align="center>

## 前置条件

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的 Rust 编程知识
- 理解 WebSocket 概念
- Python 3.8+（用于 Python 绑定示例）

<div align="center>

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-websocket-example
cd dms-websocket-example
```

### 2. 添加依赖

在 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pyo3 = { version = "0.20", features = ["extension-module"] }
```

### 3. 创建配置文件

在项目根目录创建 `config.yaml` 文件：

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

### 4. 编写主代码

将 `src/main.rs` 文件替换为以下内容：

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
        ctx.logger().info("service", "DMSC WebSocket 示例已启动")?;
        
        let server = ctx.websocket_server();
        let session_manager = ctx.websocket_session_manager();
        
        let broadcast_manager = Arc::new(Mutex::new(HashMap::<String, Vec<String>>::new()));
        let broadcast_manager_clone = broadcast_manager.clone();
        
        struct ChatMessageHandler {
            broadcast_manager: Arc<Mutex<HashMap<String, Vec<String>>>>,
        }
        
        impl DMSCWebSocketHandler for ChatMessageHandler {
            fn on_connect(
                &self,
                session: &DMSCWSSession,
            ) -> DMSCResult<()> {
                let mut manager = self.broadcast_manager.lock().unwrap();
                manager.insert(session.id().to_string(), Vec::new());
                
                println!("客户端连接: {} 来自 {}", session.id(), session.remote_addr());
                Ok(())
            }
            
            fn on_disconnect(
                &self,
                session: &DMSCWSSession,
            ) -> DMSCResult<()> {
                let mut manager = self.broadcast_manager.lock().unwrap();
                manager.remove(session.id());
                
                println!("客户端断开: {}", session.id());
                Ok(())
            }
            
            fn on_message(
                &self,
                session: &DMSCWSSession,
                message: &DMSCWebSocketMessage,
            ) -> DMSCResult<Option<DMSCWebSocketMessage>> {
                if let DMSCWebSocketMessage::Text(text) = message {
                    println!("收到来自 {}: {}", session.id(), text);
                    
                    let response = format!("回显: {}", text);
                    Some(DMSCWebSocketMessage::new_text(response))
                } else {
                    Ok(None)
                }
            }
            
            fn on_error(
                &self,
                session: &DMSCWebSocketError,
            ) -> DMSCResult<()> {
                println!("WebSocket 错误: {}", session);
                Ok(())
            }
        }
        
        let handler = ChatMessageHandler {
            broadcast_manager: broadcast_manager_clone,
        };
        
        server.set_handler(handler);
        server.start()?;
        
        ctx.logger().info("websocket", &format!("WebSocket 服务器已启动，端口 {}", 8080))?;
        
        Ok(())
    }).await
}
```

### 5. Python 处理器示例

创建 `python_handler.py` 文件用于基于 Python 的 WebSocket 处理器：

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
        print(f"客户端连接: {session_id} 来自 {remote_addr}")
        
    def on_disconnect(self, session_id: str):
        if session_id in self.sessions:
            del self.sessions[session_id]
        print(f"客户端断开: {session_id}")
        
    def on_message(self, session_id: str, data: bytes) -> bytes:
        message = data.decode('utf-8')
        print(f"收到来自 {session_id}: {message}")
        
        response = f"回显: {message}"
        return response.encode('utf-8')
        
    def on_error(self, session_id: str, error: str):
        print(f"{session_id} 错误: {error}")
        
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
    
    print("WebSocket Python 处理器已初始化")
    print(f"活动会话: {len(handler.sessions)}")
    
    import time
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("正在关闭 Python 处理器")
```

### 6. 创建 HTML 客户端

创建 `index.html` 文件用于基于浏览器的 WebSocket 客户端：

```html
<!DOCTYPE html>
<html>
<head>
    <title>DMSC WebSocket 客户端</title>
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
    <h1>DMSC WebSocket 客户端</h1>
    <div id="status" class="disconnected">已断开</div>
    <div id="messages"></div>
    <input type="text" id="message" placeholder="输入消息...">
    <button id="send" onclick="sendMessage()">发送</button>
    
    <script>
        let ws;
        
        function connect() {
            ws = new WebSocket('ws://localhost:8080/ws');
            
            ws.onopen = function() {
                document.getElementById('status').textContent = '已连接';
                document.getElementById('status').className = 'connected';
                addMessage('系统', '已连接到服务器', 'received');
            };
            
            ws.onclose = function() {
                document.getElementById('status').textContent = '已断开';
                document.getElementById('status').className = 'disconnected';
                addMessage('系统', '已从服务器断开', 'received');
            };
            
            ws.onmessage = function(event) {
                addMessage('服务器', event.data, 'received');
            };
            
            ws.onerror = function(error) {
                addMessage('错误', '发生 WebSocket 错误', 'received');
            };
        }
        
        function sendMessage() {
            const message = document.getElementById('message').value;
            if (message && ws.readyState === WebSocket.OPEN) {
                ws.send(message);
                addMessage('你', message, 'sent');
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

<div align="center

## 代码分析

</div>

### WebSocket 服务器设置

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

### 会话管理

```rust
use dmsc::prelude::*;

let session_manager = ctx.websocket_session_manager();

// 获取所有活动会话
let sessions = session_manager.get_all_sessions()?;
println!("活动会话: {}", sessions.len());

// 获取会话数
let count = session_manager.get_session_count()?;
println!("总连接数: {}", count);

// 发送到特定会话
session_manager.send_to("session-123", b"你好, 客户端!")?;

// 广播到所有客户端
session_manager.broadcast(b"服务器公告")?;

// 排除特定会话广播
session_manager.broadcast(b"大家好", exclude="session-123")?;

// 关闭特定会话
session_manager.close_session("session-123")?;

// 关闭所有会话
session_manager.close_all_sessions()?;
```

### Python 处理器设置

```python
from dmsc.ws import DMSCWSPythonHandler

def on_connect(session_id, remote_addr):
    print(f"客户端连接: {session_id} 来自 {remote_addr}")

def on_disconnect(session_id):
    print(f"客户端断开: {session_id}")

def on_message(session_id, data):
    print(f"来自 {session_id} 的消息: {data.decode()}")
    return b"回显: " + data

def on_error(session_id, error):
    print(f"{session_id} 错误: {error}")

handler = DMSCWSPythonHandler(
    on_connect=on_connect,
    on_disconnect=on_disconnect,
    on_message=on_message,
    on_error=on_error
)
```

### 消息广播

```rust
use dmsc::prelude::*;

let mut broadcaster = DMSCWebSocketBroadcaster::new();

// 将会话添加到广播器
broadcaster.add_session(session1.clone());
broadcaster.add_session(session2.clone());
broadcaster.add_session(session3.clone());

// 广播到所有
let count = broadcaster.broadcast(b"大家好!")?;
println!("消息已发送给 {} 个客户端", count);

// 广播到特定会话
let targets = vec!["session1".to_string(), "session2".to_string()];
broadcaster.broadcast_to(b"你好, 指定用户!", &targets)?;

// 排除某个会话广播
broadcaster.broadcast_excluding(b"大家好除了 session1", "session1")?;
```

### 心跳配置

```rust
use dmsc::prelude::*;

let heartbeat = DMSCWebSocketHeartbeat::new(
    Duration::from_secs(30),  // ping 间隔
    Duration::from_secs(10),  // 超时
);

// 为会话启动心跳
heartbeat.start("session-123", sender.clone());

// 检查会话是否存活
let is_alive = heartbeat.check_alive("session-123")?;
println!("会话存活: {}", is_alive);

// 获取最后 Pong 时间
if let Some(last_pong) = heartbeat.get_last_pong("session-123")? {
    println!("最后 Pong: {:?}", last_pong);
}

// 停止心跳
heartbeat.stop("session-123");
```

<div align="center

## 运行步骤

</div>

### 1. 构建项目

```bash
cargo build --release
```

### 2. 运行项目

```bash
cargo run
```

### 3. 使用浏览器客户端测试

```bash
# 在浏览器中打开 index.html
# 浏览器将连接到 ws://localhost:8080/ws
```

### 4. 使用 Python 客户端测试

```python
import websocket
import threading

def on_message(ws, message):
    print(f"收到: {message}")

def on_error(ws, error):
    print(f"错误: {error}")

def on_close(ws, close_status_code, close_msg):
    print("连接已关闭")

def on_open(ws):
    print("连接已建立")
    ws.send("你好, WebSocket!")

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

<div align="center

## 预期结果

</div>

运行示例后，您应该看到类似以下输出：

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC WebSocket 示例已启动","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"websocket","message":"WebSocket 服务器已启动，端口 8080","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"websocket","message":"客户端连接: abc123 来自 192.168.1.100:54321","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:02Z","level":"info","module":"websocket","message":"收到来自 abc123: 你好, WebSocket!","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:02Z","level":"info","module":"websocket","message":"响应已发送给 abc123: 回显: 你好, WebSocket!","trace_id":"abc123","span_id":"def456"}
```

<div align="center

## 扩展功能

</div>

### 1. 聊天室实现

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
                // 发送消息到会话
                count += 1;
            }
            Ok(count)
        } else {
            Ok(0)
        }
    }
}
```

### 2. 认证中间件

```rust
use dmsc::prelude::*;

struct AuthMiddleware;

impl DMSCWebSocketHandler for AuthMiddleware {
    fn on_connect(
        &self,
        session: &DMSCWSSession,
    ) -> DMSCResult<()> {
        // 从查询参数或头中提取认证令牌
        let token = session.get_metadata("auth_token");
        
        match validate_token(&token) {
            Ok(user_id) => {
                session.set_metadata("user_id", &user_id);
                Ok(())
            }
            Err(_) => Err(DMSCError::unauthorized("无效令牌"))
        }
    }
    
    fn on_message(
        &self,
        session: &DMSCWSSession,
        message: &DMSCWebSocketMessage,
    ) -> DMSCResult<Option<DMSCWebSocketMessage>> {
        // 检查用户是否已认证
        if session.get_metadata("user_id").is_none() {
            return Err(DMSCError::unauthorized("未认证"));
        }
        
        // 继续到下一个处理器
        Ok(None)
    }
}

fn validate_token(token: &Option<String>) -> DMSCResult<String> {
    if let Some(t) = token {
        if t.starts_with("user_") {
            Ok(t.clone())
        } else {
            Err(DMSCError::unauthorized("无效令牌格式"))
        }
    } else {
        Err(DMSCError::unauthorized("缺少令牌"))
    }
}
```

### 3. 速率限制

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
                // 窗口过期，重置
                self.limits.insert(client_id.to_string(), (1, now));
                return true;
            }
            
            if *count >= self.max_requests {
                return false;
            }
            
            // 增加计数
            self.limits.insert(client_id.to_string(), (*count + 1, *start));
            true
        } else {
            // 第一次请求
            self.limits.insert(client_id.to_string(), (1, now));
            true
        }
    }
}
```

### 4. TLS/WSS 支持

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

<div align="center

## 最佳实践

</div>

1. **实现心跳**：使用心跳机制检测断开的连接
2. **设置消息限制**：配置最大消息大小以防止内存问题
3. **处理断开连接**：正确处理客户端断开连接和清理
4. **使用会话元数据**：在会话元数据中存储用户信息以便识别
5. **高效广播**：使用广播器进行多客户端消息发送
6. **实现重连**：处理客户端重连场景
7. **监控连接**：跟踪连接数和健康指标
8. **在生产环境启用 TLS**：生产环境使用 WSS 确保安全连接
9. **使用压缩**：对大型负载启用每消息 deflate
10. **验证消息**：验证传入消息的格式和大小

<div align="center

## Python 集成

</div>

### Python 会话管理

```python
from dmsc.ws import DMSCWSSessionManagerPy

# 创建会话管理器
manager = DMSCWSSessionManagerPy(max_connections=1000)

# 获取所有活动会话
sessions = manager.get_all_sessions()
for session in sessions:
    print(f"会话: {session.id} 来自 {session.remote_addr}")

# 获取会话数
count = manager.get_session_count()
print(f"总连接数: {count}")

# 广播到所有客户端
manager.broadcast(b"服务器公告: 10分钟后维护")

# 发送到特定客户端
manager.send_to("session-123", b"你好, 客户端!")

# 关闭特定会话
manager.close_session("session-123")

# 关闭所有会话
manager.close_all_sessions()
```

### Python 客户端实现

```python
import asyncio
import websockets

async def client():
    uri = "ws://localhost:8080/ws"
    
    async with websockets.connect(uri) as websocket:
        print("已连接到服务器")
        
        # 发送消息
        await websocket.send("你好, WebSocket!")
        print("发送: 你好, WebSocket!")
        
        # 接收响应
        response = await websocket.recv()
        print(f"收到: {response}")
        
        # 保持连接活跃
        try:
            while True:
                message = await asyncio.wait_for(websocket.recv(), timeout=30)
                print(f"收到: {message}")
        except asyncio.TimeoutError:
            print("30秒内未收到消息")

if __name__ == "__main__":
    asyncio.run(client())
```

<div align="center

## 总结

</div>

本示例演示了如何使用 DMSC WebSocket 模块：

- 创建带连接管理的 WebSocket 服务器
- 使用 PyO3 绑定实现基于 Python 的处理器
- 会话管理和状态跟踪
- 向多个客户端广播消息
- 用于连接健康的保活机制
- 连接超时和清理
- 聊天室实现
- 认证中间件
- 速率限制
- TLS/WSS 支持

通过本示例，您应该掌握了 DMSC WebSocket 模块的核心功能和使用方法。您可以在此基础上构建更复杂的实时应用程序。

<div align="center

## 相关模块

</div>

- [README](./README.md)：使用示例概述，提供所有使用示例的快速导航
- [authentication](./authentication.md)：认证示例，学习 JWT、OAuth2 和 RBAC 认证授权
- [basic-app](./basic-app.md)：基础应用程序示例，学习如何创建和运行您的第一个 DMSC 应用程序
- [database](./database.md)：数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [protocol](./protocol.md)：协议示例，实现自定义通信协议
- [service_mesh](./service_mesh.md)：服务网格示例，实现服务间通信
