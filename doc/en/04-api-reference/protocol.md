<div align="center">

# Protocol API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The protocol module provides a protocol abstraction layer, supporting global protocols and private communication protocols, implementing encryption, HSM, frame processing, and other core features.

## Module Overview

</div>

The protocol module implements a layered architecture:

- **Protocol Layer**: Global protocol and private protocol implementations
- **Security Layer**: Encryption, authentication, and security enhancement
- **Adapter Layer**: Protocol abstraction and unified interfaces
- **Integration Layer**: Cross-protocol coordination and state management
- **Global State Layer**: Distributed state management and synchronization

<div align="center">

## Core Components

</div>

### DMSCProtocolManager

The main protocol manager interface, providing protocol initialization, message sending, and protocol switching capabilities.

**Note**: This class is an internal type, only available within the crate.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `initialize(config)` | Initialize manager | `config: DMSCProtocolConfig` | `DMSCResult<()>` |
| `send_message(target, message)` | Send message | `target: &str`, `message: &[u8]` | `DMSCResult<Vec<u8>>` |
| `send_message_with_protocol(target, message, protocol_type)` | Send with specific protocol | `target: &str`, `message: &[u8]`, `protocol_type: DMSCProtocolType` | `DMSCResult<Vec<u8>>` |
| `get_stats()` | Get statistics | None | `DMSCResult<DMSCProtocolStats>` |
| `shutdown()` | Shutdown manager | None | `DMSCResult<()>` |
| `create_control_center(ctx)` | Create control center | `ctx: DMSCServiceContext` | `DMSCControlCenter` |

#### Usage Example

```rust
use dms::protocol::{DMSCProtocolManager, DMSCProtocolType, DMSCProtocolConfig};
use dms::core::DMSCServiceContext;

async fn example() -> DMSCResult<()> {
    let mut ctx = DMSCServiceContext::new();
    let mut manager = DMSCProtocolManager::new(ctx);
    
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
    
    let response = manager.send_message("target-device", b"Hello DMSC").await?;
    
    let stats = manager.get_stats().await?;
    println!("Messages sent: {}", stats.total_messages_sent);
    println!("Messages received: {}", stats.total_messages_received);
    
    manager.shutdown().await?;
    
    Ok(())
}
```

### DMSCProtocolType

Protocol type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Global` | Global communication protocol |
| `Private` | Private secure protocol |

### DMSCProtocolConfig

Protocol configuration.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `default_protocol` | `DMSCProtocolType` | Default protocol type | `Global` |
| `enable_security` | `bool` | Enable security features | `true` |
| `enable_state_sync` | `bool` | Enable state synchronization | `true` |
| `performance_optimization` | `bool` | Enable performance optimization | `true` |
| `connection_timeout` | `Duration` | Connection timeout | `30s` |
| `max_connections_per_protocol` | `u32` | Max connections per protocol | `1000` |
| `protocol_switching_enabled` | `bool` | Enable protocol switching | `true` |

<div align="center">

## Security Features

</div>

### DMSCCryptoSuite

Crypto suite interface.

```rust
use dms::protocol::{DMSCCryptoSuite, AES256GCM, ChaCha20Poly1305};

let aes = AES256GCM::new();
let encrypted = aes.encrypt(data, &key, &nonce).await?;
let decrypted = aes.decrypt(&encrypted, &key, &nonce).await?;

let chacha = ChaCha20Poly1305::new();
let encrypted = chacha.encrypt(data, &key, &nonce).await?;
```

### DMSCDeviceAuthProtocol

Device authentication protocol.

```rust
use dms::protocol::DMSCDeviceAuthProtocol;

let auth = DMSCDeviceAuthProtocol::new();
let device_cert = auth.generate_device_certificate(device_id, public_key)?;
let auth_result = auth.authenticate_device(&device_cert, &signature).await?;
```

### DMSCPostQuantumCrypto

Post-quantum cryptography.

```rust
use dms::protocol::DMSCPostQuantumCrypto;

let pq_crypto = DMSCPostQuantumCrypto::new();
let (public_key, secret_key) = pq_crypto.generate_keypair()?;
let ciphertext = pq_crypto.encrypt(&public_key, plaintext)?;
let plaintext = pq_crypto.decrypt(&secret_key, &ciphertext)?;
```

<div align="center">

## Hardware Security Module (HSM)

</div>

### DMSCHSMManager

HSM manager.

```rust
use dms::protocol::{DMSCHSMManager, DMSCHSMType, DMSCHSMConfig};

let hsm_config = DMSCHSMConfig {
    hsm_type: DMSCHSMType::Software,
    key_path: "/etc/dms/keys".to_string(),
    enable_audit_log: true,
};

let mut hsm = DMSCHSMManager::new(hsm_config)?;
hsm.initialize().await?;

let key_info = hsm.generate_key(DMSCKeyType::Symmetric, 256)?;
let encrypted_key = hsm.encrypt_key(key_id, data).await?;
let decrypted_key = hsm.decrypt_key(key_id, &encrypted_key).await?;

let stats = hsm.get_statistics()?;
println!("Keys generated: {}", stats.keys_generated);
println!("Operations performed: {}", stats.operations_performed);
```

### DMSCHSMConfig

HSM configuration.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `hsm_type` | `DMSCHSMType` | HSM type |
| `key_path` | `String` | Key storage path |
| `enable_audit_log` | `bool` | Enable audit log |

### DMSCHSMType

HSM type enum.

| Variant | Description |
|:--------|:-------------|
| `Software` | Software HSM |
| `Hardware` | Hardware HSM |
| `Cloud` | Cloud HSM |

<div align="center>

## Frame Processing

</div>

### DMSCFrame

Protocol frame.

```rust
use dms::protocol::{DMSCFrame, DMSCFrameType, DMSCFrameBuilder};

let frame = DMSCFrameBuilder::new()
    .with_frame_type(DMSCFrameType::Data)
    .with_payload(data)
    .with_sequence(1)
    .with_flags(flags)
    .build()?;

let bytes = frame.to_bytes()?;
let parsed_frame = DMSCFrame::parse(&bytes)?;
```

### DMSCFrameType

Frame type enum.

| Variant | Description |
|:--------|:-------------|
| `Data` | Data frame |
| `Control` | Control frame |
| `Ack` | Acknowledgment frame |
| `Handshake` | Handshake frame |
| `Heartbeat` | Heartbeat frame |

<div align="center">

## Global State Management

</div>

### DMSCGlobalStateManager

Global state manager.

```rust
use dms::protocol::DMSCGlobalStateManager;

let state_manager = DMSCGlobalStateManager::new();
state_manager.initialize().await?;

let update = DMSCStateUpdate {
    category: DMSCStateCategory::Device,
    key: "device:001".to_string(),
    value: serde_json::json!({"status": "online"}),
    timestamp: chrono::Utc::now(),
};

state_manager.publish_update(update).await?;

let state = state_manager.get_state(DMSCStateCategory::Device, "device:001").await?;
```

<div align="center>

## Error Handling

</div>

### DMSCProtocolStats

Protocol statistics.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `total_messages_sent` | `u64` | Total messages sent |
| `total_messages_received` | `u64` | Total messages received |
| `total_bytes_sent` | `u64` | Total bytes sent |
| `total_bytes_received` | `u64` | Total bytes received |
| `average_latency_ms` | `u64` | Average latency (ms) |
| `error_count` | `u64` | Error count |
| `success_rate` | `f32` | Success rate |

<div align="center>

## Best Practices

</div>

1. **Use secure protocols**: Use Private protocol for sensitive operations
2. **Enable state synchronization**: Maintain state consistency in distributed environments
3. **Configure timeouts appropriately**: Set appropriate connection timeouts based on network conditions
4. **Monitor protocol performance**: Regularly check protocol statistics
5. **Use HSM for key protection**: Use hardware security modules for critical keys
6. **Enable protocol switching**: Dynamically switch protocol types when needed

<div align="center>

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [core](./core.md): Core module providing error handling and service context
- [log](./log.md): Logging module for protocol events
- [device](./device.md): Device module using protocols for device communication
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [observability](./observability.md): Observability module for protocol performance monitoring
