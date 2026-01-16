<div align="center">

# WebSocket API 参考

**Version: 0.1.4**

**Last modified date: 2026-01-16**

WebSocket 模块通过 TCP 连接提供全双工通信通道，支持会话管理、消息处理，以及 Python 绑定。

## 模块概述

</div>

WebSocket 模块包含以下子模块：

- **handler**：带事件回调的 WebSocket 连接处理器
- **session**：会话管理和状态跟踪
- **message**：消息编码和解码
- **broadcast**：向多个客户端广播消息
- **heartbeat**：连接保活和健康监控

<div align="center">

## 核心组件

</div>

### DMSCWSPythonHandler

兼容 Python 的 WebSocket 处理器，带事件回调支持。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `on_connect` | `Option<PyCallable>` | 连接建立回调 |
| `on_disconnect` | `Option<PyCallable>` | 连接关闭回调 |
| `on_message` | `Option<PyCallable>` | 消息接收回调 |
| `on_error` | `Option<PyCallable>` | 错误发生回调 |
| `on_ping` | `Option<PyCallable>` | 收到 Ping 回调 |
| `on_pong` | `Option<PyCallable>` | 收到 Pong 回调 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(on_connect, on_disconnect, on_message, on_error)` | 创建处理器 | 详见字段 | `Self` |
| `set_on_connect(&mut self, callback)` | 设置连接回调 | `callback: PyCallable` | `()` |
| `set_on_disconnect(&mut self, callback)` | 设置断开回调 | `callback: PyCallable` | `()` |
| `set_on_message(&mut self, callback)` | 设置消息回调 | `callback: PyCallable` | `()` |
| `set_on_error(&mut self, callback)` | 设置错误回调 | `callback: PyCallable` | `()` |

#### Python 使用示例

```python
from dmsc.ws import DMSCWSPythonHandler

def on_connect(session_id: str, remote_addr: str):
    print(f"客户端连接: {session_id} 来自 {remote_addr}")

def on_disconnect(session_id: str):
    print(f"客户端断开: {session_id}")

def on_message(session_id: str, data: bytes) -> bytes:
    print(f"收到来自 {session_id}: {data.decode()}")
    return b"回显: " + data

def on_error(session_id: str, error: str):
    print(f"{session_id} 错误: {error}")

handler = DMSCWSPythonHandler(
    on_connect=on_connect,
    on_disconnect=on_disconnect,
    on_message=on_message,
    on_error=on_error
)
```

### DMSCWSSessionManagerPy

兼容 Python 的 WebSocket 会话管理器。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `max_connections` | `u32` | 最大并发连接数 | `10000` |
| `session_timeout` | `Duration` | 会话超时时间 | `300s` |
| `heartbeat_interval` | `Duration` | 心跳检查间隔 | `30s` |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(max_connections)` | 创建会话管理器 | `max_connections: u32` | `Self` |
| `get_session(session_id)` | 按 ID 获取会话 | `session_id: &str` | `DMSCResult<Option<DMSCWSSession>>` |
| `get_all_sessions()` | 获取所有活动会话 | 无 | `DMSCResult<Vec<DMSCWSSession>>` |
| `get_session_count()` | 获取活动会话数 | 无 | `DMSCResult<u32>` |
| `broadcast(message, exclude)` | 广播消息 | `message: &[u8]`, `exclude: Option<&str>` | `DMSCResult<u64>` |
| `send_to(session_id, message)` | 发送到特定会话 | `session_id: &str`, `message: &[u8]` | `DMSCResult<()>` |
| `close_session(session_id)` | 关闭会话 | `session_id: &str` | `DMSCResult<()>` |
| `close_all_sessions()` | 关闭所有会话 | 无 | `DMSCResult<()>` |

#### Python 使用示例

```python
from dmsc.ws import DMSCWSSessionManagerPy

# 创建会话管理器
manager = DMSCWSSessionManagerPy(max_connections=1000)

# 获取所有活动会话
sessions = manager.get_all_sessions()
print(f"活动会话: {len(sessions)}")

# 获取会话数
count = manager.get_session_count()
print(f"会话数: {count}")

# 发送消息到特定会话
manager.send_to("session-123", b"你好, 客户端!")

# 广播到所有客户端
manager.broadcast(b"服务器公告: 10分钟后维护")

# 排除特定会话广播
manager.broadcast(b"大家好，除了 session-123", exclude="session-123")

# 关闭特定会话
manager.close_session("session-123")

# 关闭所有会话
manager.close_all_sessions()
```

### DMSCWSSession

WebSocket 会话，表示客户端连接。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 唯一会话标识符 |
| `remote_addr` | `String` | 远程客户端地址 |
| `local_addr` | `String` | 本地服务器地址 |
| `connected_at` | `DateTime<Utc>` | 连接时间戳 |
| `last_active_at` | `DateTime<Utc>` | 最后活动时间戳 |
| `is_connected` | `bool` | 连接状态 |
| `metadata` | `HashMap<String, String>` | 会话元数据 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `id(&self)` | 获取会话 ID | 无 | `&str` |
| `remote_addr(&self)` | 获取远程地址 | 无 | `&str` |
| `is_connected(&self)` | 检查连接状态 | 无 | `bool` |
| `send(&self, message)` | 发送消息 | `message: &[u8]` | `DMSCResult<()>` |
| `close(&self)` | 关闭连接 | 无 | `DMSCResult<()>` |
| `set_metadata(&mut self, key, value)` | 设置元数据 | `key: &str`, `value: &str` | `()` |
| `get_metadata(&self, key)` | 获取元数据 | `key: &str` | `Option<&str>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let session = DMSCWSSession::new(
    "session-123".to_string(),
    "192.168.1.100:54321".to_string(),
    "0.0.0.0:8080".to_string(),
)?;

// 检查连接状态
if session.is_connected() {
    // 发送消息
    session.send(b"欢迎!")?;
    
    // 设置元数据
    session.set_metadata("username", "alice");
    session.set_metadata("room", "general");
}

// 获取元数据
if let Some(username) = session.get_metadata("username") {
    println!("用户: {}", username);
}

// 关闭会话
session.close()?;
```

<div align="center

## WebSocket 服务器

</div>

### DMSCWebSocketServer

用于接受和管理连接的 WebSocket 服务器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建服务器 | `config: DMSCWebSocketConfig` | `DMSCResult<Self>` |
| `start(&self)` | 启动服务器 | 无 | `DMSCResult<()>` |
| `stop(&self)` | 停止服务器 | 无 | `DMSCResult<()>` |
| `is_running(&self)` | 检查是否运行 | 无 | `bool` |
| `set_handler(&mut self, handler)` | 设置消息处理器 | `handler: impl DMSCWebSocketHandler` | `()` |
| `set_session_manager(&mut self, manager)` | 设置会话管理器 | `manager: impl DMSCWebSocketSessionManager` | `()` |
| `broadcast(&self, message)` | 广播到所有 | `message: &[u8]` | `DMSCResult<u64>` |
| `get_connections(&self)` | 获取所有连接 | 无 | `DMSCResult<Vec<DMSCWSSession>>` |

### DMSCWebSocketConfig

WebSocket 服务器配置。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | 绑定主机 | `"0.0.0.0"` |
| `port` | `u16` | 绑定端口 | `8080` |
| `max_connections` | `u32` | 最大并发连接数 | `10000` |
| `connection_timeout` | `Duration` | 连接超时 | `60s` |
| `heartbeat_interval` | `Duration` | 心跳间隔 | `30s` |
| `max_message_size` | `u64` | 最大消息大小（字节） | `65536` |
| `ping_interval` | `Duration` | Ping 间隔 | `25s` |
| `ping_timeout` | `Duration` | Ping 超时 | `10s` |
| `tls_enabled` | `bool` | 启用 WSS | `false` |
| `tls_cert_path` | `Option<String>` | TLS 证书路径 | `None` |
| `tls_key_path` | `Option<String>` | TLS 密钥路径 | `None` |

#### 使用示例

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

println!("WebSocket 服务器已启动于 {}:{}", config.host, config.port);

// 服务器运行直到停止
server.stop()?;
```

<div align="center

## 消息处理

</div>

### DMSCWebSocketMessage

WebSocket 通信的消息类型。

#### 消息类型

| 类型 | 描述 |
|:--------|:-------------|
| `Text` | 文本消息（UTF-8 编码） |
| `Binary` | 二进制数据 |
| `Close` | 连接关闭帧 |
| `Ping` | Ping 帧（保活） |
| `Pong` | Pong 帧（保活响应） |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new_text(data)` | 创建文本消息 | `data: String` | `Self` |
| `new_binary(data)` | 创建二进制消息 | `data: Vec<u8>` | `Self` |
| `is_text(&self)` | 检查是否为文本 | 无 | `bool` |
| `is_binary(&self)` | 检查是否为二进制 | 无 | `bool` |
| `is_close(&self)` | 检查是否为关闭 | 无 | `bool` |
| `is_ping(&self)` | 检查是否为 Ping | 无 | `bool` |
| `is_pong(&self)` | 检查是否为 Pong | 无 | `bool` |
| `into_text(self)` | 转换为文本 | 无 | `DMSCResult<String>` |
| `into_binary(self)` | 转换为二进制 | 无 | `DMSCResult<Vec<u8>>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let text_msg = DMSCWebSocketMessage::new_text("你好, WebSocket!".to_string());
assert!(text_msg.is_text());

let binary_msg = DMSCWebSocketMessage::new_binary(vec![0x01, 0x02, 0x03]);
assert!(binary_msg.is_binary());

// 处理接收到的消息
match message {
    DMSCWebSocketMessage::Text(text) => {
        println!("收到文本: {}", text);
    }
    DMSCWebSocketMessage::Binary(data) => {
        println!("收到二进制: {:?}", data);
    }
    DMSCWebSocketMessage::Close(code, reason) => {
        println!("客户端关闭: {} - {}", code, reason);
    }
    DMSCWebSocketMessage::Ping(data) => {
        println!("收到 Ping: {:?}", data);
    }
    DMSCWebSocketMessage::Pong(data) => {
        println!("收到 Pong: {:?}", data);
    }
}
```

<div align="center

## 广播

</div>

### DMSCWebSocketBroadcaster

向多个会话广播消息。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建广播器 | 无 | `Self` |
| `broadcast(&self, message)` | 广播到所有 | `message: &[u8]` | `DMSCResult<u64>` |
| `broadcast_to(&self, message, sessions)` | 广播到特定会话 | `message: &[u8]`, `sessions: &[String]` | `DMSCResult<u64>` |
| `broadcast_excluding(&self, message, exclude)` | 排除广播 | `message: &[u8]`, `exclude: &str` | `DMSCResult<u64>` |
| `add_session(&mut self, session)` | 添加会话 | `session: DMSCWSSession` | `()` |
| `remove_session(&mut self, session_id)` | 移除会话 | `session_id: &str` | `()` |
| `get_session_count(&self)` | 获取会话数 | 无 | `usize` |

#### 使用示例

```rust
use dmsc::prelude::*;

let mut broadcaster = DMSCWebSocketBroadcaster::new();

// 添加会话
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
broadcaster.broadcast_excluding(b"大家好，除了 session1", "session1")?;

// 移除会话
broadcaster.remove_session("session1");
```

<div align="center

## 心跳和健康检查

</div>

### DMSCWebSocketHeartbeat

连接保活机制。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(interval, timeout)` | 创建心跳 | `interval: Duration`, `timeout: Duration` | `Self` |
| `start(&self, session_id, sender)` | 启动心跳 | `session_id: &str`, `sender: impl Send + Clone` | `()` |
| `stop(&self, session_id)` | 停止心跳 | `session_id: &str` | `()` |
| `check_alive(&self, session_id)` | 检查存活 | `session_id: &str` | `DMSCResult<bool>` |
| `get_last_pong(&self, session_id)` | 获取最后 Pong 时间 | `session_id: &str` | `DMSCResult<Option<DateTime<Utc>>>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let heartbeat = DMSCWebSocketHeartbeat::new(
    Duration::from_secs(30),
    Duration::from_secs(10),
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

## 错误处理

</div>

### DMSCWebSocketError

WebSocket 特定错误。

| 错误码 | 描述 |
|:--------|:-------------|
| `WS_CONNECTION_ERROR` | 连接错误 |
| `WS_MESSAGE_TOO_LARGE` | 消息超出最大大小 |
| `WS_SESSION_NOT_FOUND` | 会话未找到 |
| `WS_SESSION_CLOSED` | 会话已关闭 |
| `WS_SEND_ERROR` | 发送消息失败 |
| `WS_TIMEOUT` | 操作超时 |
| `WS_TLS_ERROR` | TLS/证书错误 |
| `WS_PROTOCOL_ERROR` | WebSocket 协议错误 |

#### 使用示例

```rust
use dmsc::prelude::*;

match session.send(message) {
    Ok(_) => {
        println!("消息发送成功");
    }
    Err(DMSCWebSocketError::SessionClosed) => {
        println!("会话已关闭，清理中");
    }
    Err(DMSCWebSocketError::SendError(e)) => {
        println!("发送失败: {}", e);
    }
    Err(e) => {
        println!("WebSocket 错误: {}", e);
    }
}
```

<div align="center

## Python 支持

</div>

WebSocket 模块通过 PyO3 提供完整的 Python 绑定：

```python
from dmsc.ws import (
    DMSCWSPythonHandler,
    DMSCWSSessionManagerPy,
    DMSCWebSocketConfig
)

# 创建带回调的处理器
def on_connect(session_id, remote_addr):
    print(f"已连接: {session_id} 来自 {remote_addr}")

def on_disconnect(session_id):
    print(f"已断开: {session_id}")

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

# 创建会话管理器
manager = DMSCWSSessionManagerPy(max_connections=1000)

# 获取会话信息
sessions = manager.get_all_sessions()
count = manager.get_session_count()

# 广播到所有客户端
manager.broadcast(b"服务器公告")

# 发送到特定客户端
manager.send_to("session-123", b"你好!")
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

<div align="center

## 相关模块

</div>

- [README](./README.md)：模块概览，提供 API 参考文档总览和快速导航
- [auth](./auth.md)：认证模块，处理用户认证和授权
- [cache](./cache.md)：缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md)：配置模块，管理应用程序配置
- [core](./core.md)：核心模块，提供错误处理和服务上下文
- [grpc](./grpc.md)：gRPC 模块，提供 RPC 功能
- [http](./http.md)：HTTP 模块，提供 HTTP 服务器和客户端功能
- [mq](./mq.md)：消息队列模块，提供消息队列支持
- [protocol](./protocol.md)：协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md)：服务网格模块，使用协议进行服务间通信
