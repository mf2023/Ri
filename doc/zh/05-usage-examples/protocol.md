<div align="center">

# 协议模块使用指南

**Version: 0.1.6**

**Last modified date: 2026-02-01**

本文档提供 DMSC 协议模块的完整使用示例，演示如何利用协议抽象、安全特性和状态管理功能。

## 目录

</div>

1. [协议管理器基础](#协议管理器基础)
2. [协议类型](#协议类型)
3. [消息发送](#消息发送)
4. [协议切换](#协议切换)
5. [安全配置](#安全配置)
6. [状态管理](#状态管理)
7. [HSM 集成](#hsm-集成)
8. [帧处理](#帧处理)
9. [完整示例](#完整示例)

---

## 协议管理器基础

协议管理器是 DMSC 中管理通信协议的核心组件。它为不同协议类型提供统一接口，并处理协议切换、安全和状态同步。

### 创建协议管理器

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn create_protocol_manager() -> DMSCResult<()> {
    let manager = DMSCProtocolManager::new();
    Ok(())
}
```

### 使用自定义配置初始化

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn initialize_with_config() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    
    let config = DMSCProtocolConfig {
        default_protocol: DMSCProtocolType::Global,
        enable_security: true,
        enable_state_sync: true,
        performance_optimization: true,
        connection_timeout: std::time::Duration::from_secs(30),
        max_connections_per_protocol: 1000,
        protocol_switching_enabled: true,
    };
    
    manager.initialize(config).await?;
    
    Ok(())
}
```

### 默认配置

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn use_default_config() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    Ok(())
}
```

---

## 协议类型

DMSC 支持两种协议类型以满足不同的通信需求。

### 全局协议

全局协议是适用于一般场景的标准通信协议：

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn use_global_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let response = manager.send_message("target-device", b"Hello via Global Protocol").await?;
    
    Ok(())
}
```

### 私有协议

私有协议为敏感操作提供增强的安全性：

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn use_private_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    manager.switch_protocol(DMSCProtocolType::Private).await?;
    
    let response = manager.send_message("secure-device", b"Sensitive data").await?;
    
    Ok(())
}
```

### 协议类型参考

| 协议类型 | 用途 | 安全级别 |
|-----------|-------------|----------------|
| `Global` | 标准通信 | 标准 |
| `Private` | 敏感操作 | 增强 |

---

## 消息发送

### 基本消息发送

```rust
use dmsc::protocol::{DMSCProtocolManager};
use dmsc::prelude::*;

async fn send_basic_message() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let message = b"Hello, DMSC Protocol!";
    let response = manager.send_message("device-001", message).await?;
    
    println!("Response: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

### 使用指定协议发送

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn send_with_specific_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let response = manager.send_message_with_protocol(
        "device-001",
        b"Global message",
        DMSCProtocolType::Global,
    ).await?;
    
    let secure_response = manager.send_message_with_protocol(
        "secure-device",
        b"Private message",
        DMSCProtocolType::Private,
    ).await?;
    
    Ok(())
}
```

### 二进制数据传输

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn send_binary_data() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xff, 0xfe];
    
    let response = manager.send_message("file-server", &binary_data).await?;
    
    println!("Received {} bytes", response.len());
    
    Ok(())
}
```

### JSON数据传输

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn send_json_data() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let json_payload = r#"{
        "command": "get_status",
        "device_id": "sensor-001",
        "timestamp": 1699999999
    }"#;
    
    let response = manager.send_message(
        "control-center",
        json_payload.as_bytes(),
    ).await?;
    
    let response_str = String::from_utf8_lossy(&response);
    println!("Response: {}", response_str);
    
    Ok(())
}
```

---

## 协议切换

### 运行时切换协议

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn runtime_protocol_switching() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let current = manager.get_current_protocol().await;
    println!("Current protocol: {:?}", current);
    
    manager.switch_protocol(DMSCProtocolType::Private).await?;
    let new_current = manager.get_current_protocol().await;
    println!("Switched to: {:?}", new_current);
    
    let secure_response = manager.send_message("secure-server", b"Confidential data").await?;
    
    manager.switch_protocol(DMSCProtocolType::Global).await?;
    
    Ok(())
}
```

### 检查当前协议

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn check_current_protocol() -> DMSCResult<()> {
    let manager = DMSCProtocolManager::new();
    
    let current = manager.get_current_protocol().await;
    
    match current {
        DMSCProtocolType::Global => println!("Using Global Protocol"),
        DMSCProtocolType::Private => println!("Using Private Protocol"),
    }
    
    Ok(())
}
```

---

## 安全配置

### 启用安全功能

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn configure_security() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    
    let config = DMSCProtocolConfig {
        default_protocol: DMSCProtocolType::Global,
        enable_security: true,
        enable_state_sync: true,
        performance_optimization: true,
        connection_timeout: std::time::Duration::from_secs(30),
        max_connections_per_protocol: 1000,
        protocol_switching_enabled: true,
    };
    
    manager.initialize(config).await?;
    
    let response = manager.send_message("secure-device", b"Sensitive data").await?;
    
    Ok(())
}
```

### 安全级别参考

| 级别 | 功能 |
|-------|----------|
| `None` | 无安全 |
| `Basic` | 仅加密 |
| `Standard` | 加密+认证 |
| `High` | 增强加密+多因素认证 |
| `Maximum` | 抗量子+设备认证 |

---

## 状态管理

### 获取协议统计

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn get_protocol_stats() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    manager.send_message("device-001", b"Test message").await?;
    
    let stats = manager.get_stats().await?;
    
    println!("Messages sent: {}", stats.total_messages_sent);
    println!("Messages received: {}", stats.total_messages_received);
    println!("Bytes sent: {}", stats.total_bytes_sent);
    println!("Bytes received: {}", stats.total_bytes_received);
    println!("Average latency: {}ms", stats.average_latency_ms);
    println!("Success rate: {:.2}%", stats.success_rate * 100.0);
    
    Ok(())
}
```

### 协议状态

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolStatus};
use dmsc::prelude::*;

async fn check_protocol_status() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    let status = manager.get_status().await?;
    
    println!("Protocol initialized: {}", status.initialized);
    println!("Protocol active: {}", status.active);
    println!("Active connections: {}", status.active_connections);
    println!("Protocol health: {:?}", status.health);
    
    Ok(())
}
```

---

## HSM集成

### HSM管理器设置

```rust
use dmsc::protocol::{
    DMSCHSMManager,
    DMSCHSMType,
    DMSCHSMConfig,
    DMSCHSMStatistics,
};
use dmsc::prelude::*;

async fn setup_hsm() -> DMSCResult<()> {
    let config = DMSCHSMConfig {
        hsm_type: DMSCHSMType::Software,
        max_keys: 100,
        enable_audit_log: true,
        key_cache_size: 50,
    };
    
    let mut hsm_manager = DMSCHSMManager::new(config)?;
    hsm_manager.initialize().await?;
    
    Ok(())
}
```

### 密钥管理

```rust
use dmsc::protocol::{DMSCHSMManager, DMSCHSMConfig, DMSCKeyType};
use dmsc::prelude::*;

async fn manage_keys() -> DMSCResult<()> {
    let config = DMSCHSMConfig::default();
    let mut hsm_manager = DMSCHSMManager::new(config)?;
    hsm_manager.initialize().await?;
    
    let key_info = hsm_manager.generate_key(DMSCKeyType::Aes256).await?;
    println!("Generated key: {}", key_info.key_id);
    
    let key = hsm_manager.get_key_info(&key_info.key_id).await?;
    println!("Key type: {:?}", key.key_type);
    
    Ok(())
}
```

---

## 帧处理

### 帧类型参考

| 帧类型 | 描述 |
|------------|-------------|
| `Data` | 标准数据帧 |
| `Control` | 控制消息帧 |
| `Ack` | 确认帧 |
| `Heartbeat` | 心跳帧 |
| `Error` | 错误报告帧 |

### 创建帧

```rust
use dmsc::protocol::{DMSCFrame, DMSCFrameHeader, DMSCFrameType};
use dmsc::prelude::*;

fn create_frames() -> DMSCResult<()> {
    let data_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Data),
        b"Hello, DMSC!".to_vec(),
    );
    
    let control_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Control),
        vec![],
    );
    
    let heartbeat_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Heartbeat),
        vec![],
    );
    
    Ok(())
}
```

---

## 完整示例

以下示例演示了协议模块的完整集成：

```rust
use dmsc::protocol::{
    DMSCProtocolManager,
    DMSCProtocolConfig,
    DMSCProtocolType,
    DMSCHSMManager,
    DMSCHSMConfig,
    DMSCKeyType,
};
use dmsc::prelude::*;

struct ProtocolApplication {
    manager: DMSCProtocolManager,
    hsm_manager: DMSCHSMManager,
}

impl ProtocolApplication {
    async fn new() -> DMSCResult<Self> {
        let mut manager = DMSCProtocolManager::new();
        
        let config = DMSCProtocolConfig {
            default_protocol: DMSCProtocolType::Global,
            enable_security: true,
            enable_state_sync: true,
            performance_optimization: true,
            connection_timeout: std::time::Duration::from_secs(30),
            max_connections_per_protocol: 1000,
            protocol_switching_enabled: true,
        };
        
        manager.initialize(config).await?;
        
        let hsm_config = DMSCHSMConfig {
            hsm_type: DMSCHSMType::Software,
            max_keys: 100,
            enable_audit_log: true,
            key_cache_size: 50,
        };
        
        let mut hsm_manager = DMSCHSMManager::new(hsm_config)?;
        hsm_manager.initialize().await?;
        
        Ok(Self {
            manager,
            hsm_manager,
        })
    }
    
    async fn send_secure_command(&self, device: &str, command: &str) -> DMSCResult<Vec<u8>> {
        // 确保使用私有协议以保证安全性
        self.manager.switch_protocol(DMSCProtocolType::Private).await?;
        
        // 发送命令
        let response = self.manager.send_message(device, command.as_bytes()).await?;
        
        // 切换回全局协议
        self.manager.switch_protocol(DMSCProtocolType::Global).await?;
        
        Ok(response)
    }
    
    async fn get_statistics(&self) -> DMSCResult<()> {
        let stats = self.manager.get_stats().await?;
        
        println!("=== Protocol Statistics ===");
        println!("Messages sent: {}", stats.total_messages_sent);
        println!("Messages received: {}", stats.total_messages_received);
        println!("Bytes sent: {}", stats.total_bytes_sent);
        println!("Bytes received: {}", stats.total_bytes_received);
        println!("Average latency: {}ms", stats.average_latency_ms);
        println!("Error count: {}", stats.error_count);
        println!("Success rate: {:.2}%", stats.success_rate * 100.0);
        
        Ok(())
    }
    
    async fn manage_keys(&self) -> DMSCResult<()> {
        // 生成新的加密密钥
        let key_info = self.hsm_manager.generate_key(DMSCKeyType::Aes256).await?;
        println!("Generated new key: {}", key_info.key_id);
        
        Ok(())
    }
    
    async fn shutdown(&mut self) -> DMSCResult<()> {
        self.manager.shutdown().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let mut app = ProtocolApplication::new().await?;
    
    // 使用不同协议发送命令
    println!("Sending global protocol message...");
    let _global_response = app.manager.send_message(
        "monitor-001",
        b"Get system status",
    ).await?;
    
    println!("Sending private protocol message...");
    let secure_response = app.send_secure_command(
        "secure-gateway",
        "Execute critical operation",
    ).await?;
    
    println!("Secure response: {}", String::from_utf8_lossy(&secure_response));
    
    // 获取统计信息
    app.get_statistics().await?;
    
    // 管理加密密钥
    app.manage_keys().await?;
    
    // 优雅关闭
    app.shutdown().await?;
    
    Ok(())
}
```

### 预期输出

```
Sending global protocol message...
Sending private protocol message...
Secure response: Operation completed successfully
=== Protocol Statistics ===
Messages sent: 2
Messages received: 2
Bytes sent: 64
Bytes received: 128
Average latency: 15ms
Error count: 0
Success rate: 100.00%
Generated new key: key-a1b2c3d4
```

<div align="center">

## 相关模块

</div>

- [README](./README.md)：使用示例总览，提供快速导航
- [authentication](./authentication.md)：认证示例，包括JWT、OAuth2和多因素认证
- [basic-app](./basic-app.md)：基础应用示例
- [caching](./caching.md)：缓存示例，包括内存缓存和分布式缓存
- [database](./database.md)：数据库操作示例
- [device](./device.md)：设备控制示例
- [fs](./fs.md)：文件系统操作示例
- [gateway](./gateway.md)：API网关示例
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [hooks](./hooks.md)：钩子系统示例
- [observability](./observability.md)：可观测性示例
- [service_mesh](./service_mesh.md)：服务网格示例
- [validation](./validation.md)：数据验证示例
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信
