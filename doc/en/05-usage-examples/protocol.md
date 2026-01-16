# Protocol Module Usage Guide

This document provides comprehensive usage examples for the DMSC Protocol Module, demonstrating how to leverage protocol abstraction, security features, and state management capabilities.

## Table of Contents

1. [Protocol Manager Basics](#protocol-manager-basics)
2. [Protocol Types](#protocol-types)
3. [Message Sending](#message-sending)
4. [Protocol Switching](#protocol-switching)
5. [Security Configuration](#security-configuration)
6. [State Management](#state-management)
7. [HSM Integration](#hsm-integration)
8. [Frame Processing](#frame-processing)
9. [Complete Example](#complete-example)

---

## Protocol Manager Basics

The protocol manager serves as the central component for managing communication protocols in DMSC. It provides a unified interface for different protocol types and handles protocol switching, security, and state synchronization.

### Creating a Protocol Manager

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn create_protocol_manager() -> DMSCResult<()> {
    let manager = DMSCProtocolManager::new();
    Ok(())
}
```

### Initializing with Custom Configuration

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

### Default Configuration

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType};

async fn use_default_config() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    
    // Use default configuration
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    Ok(())
}
```

---

## Protocol Types

DMSC supports two protocol types for different communication needs.

### Global Protocol

The global protocol is the standard communication protocol for general use cases:

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn use_global_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Send message using global protocol
    let response = manager.send_message("target-device", b"Hello via Global Protocol").await?;
    
    Ok(())
}
```

### Private Protocol

The private protocol provides enhanced security for sensitive operations:

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn use_private_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Switch to private protocol for sensitive data
    manager.switch_protocol(DMSCProtocolType::Private).await?;
    
    // Send secure message
    let response = manager.send_message("secure-device", b"Sensitive data").await?;
    
    Ok(())
}
```

### Protocol Type Reference

| Protocol Type | Use Case | Security Level |
|---------------|----------|----------------|
| `Global` | Standard communication | Standard |
| `Private` | Sensitive operations | Enhanced |

---

## Message Sending

### Basic Message Sending

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn send_basic_message() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Send a simple text message
    let message = b"Hello, DMSC Protocol!";
    let response = manager.send_message("device-001", message).await?;
    
    println!("Response: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

### Sending with Specific Protocol

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn send_with_specific_protocol() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Send message using global protocol explicitly
    let response = manager.send_message_with_protocol(
        "device-001",
        b"Global message",
        DMSCProtocolType::Global,
    ).await?;
    
    // Send message using private protocol explicitly
    let secure_response = manager.send_message_with_protocol(
        "secure-device",
        b"Private message",
        DMSCProtocolType::Private,
    ).await?;
    
    Ok(())
}
```

### Binary Data Transfer

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn send_binary_data() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Prepare binary data (e.g., file content)
    let binary_data = vec![0x00, 0x01, 0x02, 0x03, 0xff, 0xfe];
    
    // Send binary data
    let response = manager.send_message("file-server", &binary_data).await?;
    
    // Process binary response
    println!("Received {} bytes", response.len());
    
    Ok(())
}
```

### JSON Data Transfer

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn send_json_data() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Create JSON payload
    let json_payload = r#"{
        "command": "get_status",
        "device_id": "sensor-001",
        "timestamp": 1699999999
    }"#;
    
    // Send JSON data
    let response = manager.send_message(
        "control-center",
        json_payload.as_bytes(),
    ).await?;
    
    // Parse JSON response
    let response_str = String::from_utf8_lossy(&response);
    println!("Response: {}", response_str);
    
    Ok(())
}
```

---

## Protocol Switching

### Switching at Runtime

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn runtime_protocol_switching() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Start with global protocol
    let current = manager.get_current_protocol().await;
    println!("Current protocol: {:?}", current);
    
    // Switch to private protocol for sensitive operations
    manager.switch_protocol(DMSCProtocolType::Private).await?;
    let new_current = manager.get_current_protocol().await;
    println!("Switched to: {:?}", new_current);
    
    // Perform secure operations
    let secure_response = manager.send_message("secure-server", b"Confidential data").await?;
    
    // Switch back to global protocol
    manager.switch_protocol(DMSCProtocolType::Global).await?;
    
    Ok(())
}
```

### Checking Current Protocol

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType};
use dmsc::prelude::*;

async fn check_current_protocol() -> DMSCResult<()> {
    let manager = DMSCProtocolManager::new();
    
    // Get current protocol type
    let current = manager.get_current_protocol().await;
    
    match current {
        DMSCProtocolType::Global => println!("Using Global Protocol"),
        DMSCProtocolType::Private => println!("Using Private Protocol"),
    }
    
    Ok(())
}
```

---

## Security Configuration

### Enabling Security Features

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
    
    // All messages will now be encrypted and authenticated
    let response = manager.send_message("secure-device", b"Sensitive data").await?;
    
    Ok(())
}
```

### Security Level Reference

| Level | Features |
|-------|----------|
| `None` | No security |
| `Basic` | Encryption only |
| `Standard` | Encryption + authentication |
| `High` | Enhanced encryption + multi-factor auth |
| `Maximum` | Quantum-resistant + device authentication |

---

## State Management

### Getting Protocol Statistics

```rust
use dmsc::protocol::DMSCProtocolManager;
use dmsc::prelude::*;

async fn get_protocol_stats() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Perform some operations
    manager.send_message("device-001", b"Test message").await?;
    
    // Get statistics
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

### Protocol Status

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolStatus};
use dmsc::prelude::*;

async fn check_protocol_status() -> DMSCResult<()> {
    let mut manager = DMSCProtocolManager::new();
    manager.initialize(DMSCProtocolConfig::default()).await?;
    
    // Check protocol status
    let status = manager.get_status().await?;
    
    println!("Protocol initialized: {}", status.initialized);
    println!("Protocol active: {}", status.active);
    println!("Active connections: {}", status.active_connections);
    println!("Protocol health: {:?}", status.health);
    
    Ok(())
}
```

---

## HSM Integration

### HSM Manager Setup

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

### Key Management

```rust
use dmsc::protocol::{DMSCHSMManager, DMSCHSMConfig, DMSCKeyType};
use dmsc::prelude::*;

async fn manage_keys() -> DMSCResult<()> {
    let config = DMSCHSMConfig::default();
    let mut hsm_manager = DMSCHSMManager::new(config)?;
    hsm_manager.initialize().await?;
    
    // Generate a new AES key
    let key_info = hsm_manager.generate_key(DMSCKeyType::Aes256).await?;
    println!("Generated key: {}", key_info.key_id);
    
    // Get key information
    let key = hsm_manager.get_key_info(&key_info.key_id).await?;
    println!("Key type: {:?}", key.key_type);
    
    Ok(())
}
```

---

## Frame Processing

### Frame Types Reference

| Frame Type | Description |
|------------|-------------|
| `Data` | Standard data frame |
| `Control` | Control message frame |
| `Ack` | Acknowledgment frame |
| `Heartbeat` | Heartbeat frame |
| `Error` | Error report frame |

### Creating Frames

```rust
use dmsc::protocol::{DMSCFrame, DMSCFrameHeader, DMSCFrameType};
use dmsc::prelude::*;

fn create_frames() -> DMSCResult<()> {
    // Create a data frame
    let data_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Data),
        b"Hello, DMSC!".to_vec(),
    );
    
    // Create a control frame
    let control_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Control),
        vec![],
    );
    
    // Create a heartbeat frame
    let heartbeat_frame = DMSCFrame::new(
        DMSCFrameHeader::new(DMSCFrameType::Heartbeat),
        vec![],
    );
    
    Ok(())
}
```

---

## Complete Example

The following example demonstrates a complete integration of the protocol module:

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
        // Ensure we're using private protocol for security
        self.manager.switch_protocol(DMSCProtocolType::Private).await?;
        
        // Send the command
        let response = self.manager.send_message(device, command.as_bytes()).await?;
        
        // Switch back to global protocol
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
        // Generate a new encryption key
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
    
    // Send commands using different protocols
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
    
    // Get statistics
    app.get_statistics().await?;
    
    // Manage cryptographic keys
    app.manage_keys().await?;
    
    // Shutdown gracefully
    app.shutdown().await?;
    
    Ok(())
}
```

### Expected Output

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
- [hooks](./hooks.md): Hook system examples
- [http](./http.md): HTTP server and client examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples
- [observability](./observability.md): Observability examples
- [security](./security.md): Security and encryption examples
- [service_mesh](./service_mesh.md): Service mesh examples
- [storage](./storage.md): Cloud storage examples
- [validation](./validation.md): Data validation examples
