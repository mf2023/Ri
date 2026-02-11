<div align="center">

# Protocol API Reference

**Version: 0.1.7**

**Last modified date: 2026-01-18**

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
| `send_message(target, message)` | Send message with detailed response | `target: &str`, `message: &[u8]` | `Vec<u8>` (JSON response) |
| `send_message_with_flags(target, message, flags)` | Send message with custom flags | `target: &str`, `message: &[u8]`, `flags: DMSCMessageFlags` | `Vec<u8>` (JSON response) |
| `get_stats()` | Get statistics | None | `DMSCProtocolStats` |
| `close_connection(connection_id)` | Close connection | `connection_id: &str` | `bool` |
| `get_connection_info(connection_id)` | Get connection info | `connection_id: &str` | `Option<DMSCConnectionInfo>` |

#### Response Format

The `send_message` method returns a JSON response with the following structure:

```json
{
    "success": true,
    "sequence_number": 123,
    "target_id": "target-device",
    "response_data": {
        "status": "delivered",
        "target": "target-device",
        "source": "protocol_manager",
        "sequence": 123,
        "timestamp": 1705588800000,
        "frame_type": "Data",
        "payload_size": 11,
        "protocol": "Global",
        "delivery": {
            "delivered_at": 1705588800000,
            "hops": 1,
            "route": ["protocol_manager", "target-device"]
        }
    },
    "timestamp": 1705588800000
}
```

#### Usage Example

```rust
use dmsc::protocol::{DMSCProtocolManager, DMSCProtocolType, DMSCProtocolConfig};
use serde_json::Value;

async fn example() -> DMSCResult<()> {
    let manager = DMSCProtocolManager::new();
    
    let config = DMSCProtocolConfig {
        default_protocol: DMSCProtocolType::Global,
        enable_security: true,
        security_level: DMSCSecurityLevel::Standard,
        enable_state_sync: true,
        performance_optimization: true,
    };
    
    manager.initialize(config)?;
    
    let response = manager.send_message("target-device", b"Hello DMSC");
    let response_json: Value = serde_json::from_slice(&response)?;
    
    println!("Status: {}", response_json["response_data"]["status"]);
    println!("Sequence: {}", response_json["response_data"]["sequence"]);
    println!("Frame Type: {}", response_json["response_data"]["frame_type"]);
    
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
| `security_level` | `DMSCSecurityLevel` | Security level | `Standard` |
| `enable_state_sync` | `bool` | Enable state synchronization | `true` |
| `performance_optimization` | `bool` | Enable performance optimization | `true` |

### DMSCSecurityLevel

Security level enum.

| Variant | Description |
|:--------|:-------------|
| `None` | No security |
| `Standard` | Standard security |
| `High` | High security |
| `Military` | Military-grade security |

<div align="center">

## Security Features

</div>

### DMSCCryptoSuite

Crypto suite interface.

```rust
use dmsc::protocol::{DMSCCryptoSuite, AES256GCM, ChaCha20Poly1305};

let aes = AES256GCM::new();
let encrypted = aes.encrypt(data, &key, &nonce).await?;
let decrypted = aes.decrypt(&encrypted, &key, &nonce).await?;

let chacha = ChaCha20Poly1305::new();
let encrypted = chacha.encrypt(data, &key, &nonce).await?;
```

### DMSCDeviceAuthProtocol

Device authentication protocol.

```rust
use dmsc::protocol::DMSCDeviceAuthProtocol;

let auth = DMSCDeviceAuthProtocol::new();
let device_cert = auth.generate_device_certificate(device_id, public_key)?;
let auth_result = auth.authenticate_device(&device_cert, &signature).await?;
```

### DMSCPostQuantumCrypto

Post-quantum cryptography.

```rust
use dmsc::protocol::DMSCPostQuantumCrypto;

let pq_crypto = DMSCPostQuantumCrypto::new();
let (public_key, secret_key) = pq_crypto.generate_keypair()?;
let ciphertext = pq_crypto.encrypt(&public_key, plaintext)?;
let plaintext = pq_crypto.decrypt(&secret_key, &ciphertext)?;
```

<div align="center">

## National Cryptography (Guomi)

</div>

### DMSCGuomi

National cryptography algorithm suite (SM2/SM3/SM4).

```rust
use dmsc::protocol::guomi::{DMSCGuomi, SM2Signer, SM3, SM4};

// SM2 signature
let signer = SM2Signer::new()?;
let (sm2_public, sm2_private) = signer.keygen()?;
let signature = signer.sign(&sm2_private, &message)?;

// SM3 hash - returns DMSCResult<[u8; 32]>
let sm3 = SM3::new();
let sm3_hash = sm3.hash(&data)?;

// SM4 encryption
let sm4 = SM4::new();
let sm4_key = [0u8; 16]; // 16-byte key
let encrypted = sm4.encrypt_ecb(&sm4_key, &data)?;
```

### SM2Signer

SM2 elliptic curve digital signature algorithm.

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create signer | None | `DMSCResult<Self>` |
| `keygen()` | Generate key pair | None | `DMSCResult<(Vec<u8>, Vec<u8>)>` |
| `sign(secret_key, message)` | Sign message | `secret_key: &[u8]`, `message: &[u8]` | `DMSCResult<Vec<u8>>` |
| `verify(public_key, message, signature)` | Verify signature | `public_key: &[u8]`, `message: &[u8]`, `signature: &[u8]` | `DMSCResult<bool>` |

### SM3

SM3 cryptographic hash algorithm.

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create hasher | None | `Self` |
| `hash(data)` | Compute hash | `data: &[u8]` | `DMSCResult<[u8; 32]>` |

### SM4

SM4 block cipher algorithm.

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create cipher | None | `Self` |
| `encrypt_ecb(key, plaintext)` | ECB mode encryption | `key: &[u8; 16]`, `plaintext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt_ecb(key, ciphertext)` | ECB mode decryption | `key: &[u8; 16]`, `ciphertext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `encrypt_cbc(key, iv, plaintext)` | CBC mode encryption | `key: &[u8; 16]`, `iv: &[u8; 16]`, `plaintext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt_cbc(key, iv, ciphertext)` | CBC mode decryption | `key: &[u8; 16]`, `iv: &[u8; 16]`, `ciphertext: &[u8]` | `DMSCResult<Vec<u8>>` |

<div align="center">

## Hardware Security Module (HSM)

</div>

### DMSCHSMManager

HSM manager.

```rust
use dmsc::protocol::{DMSCHSMManager, DMSCHSMType, DMSCHSMConfig};

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

<div align="center">

## Frame Processing

</div>

### DMSCFrame

Protocol frame.

```rust
use dmsc::protocol::{DMSCFrame, DMSCFrameType, DMSCFrameBuilder};

let frame = DMSCFrameBuilder::new()
    .with_frame_type(DMSCFrameType::Data)
    .with_payload(data)
    .with_sequence(1)
    .with_flags(flags)
    .build()?;

let bytes = frame.to_bytes()?;
let parsed_frame = DMSCFrame::parse(&bytes)?;
```

### DMSCFrameBuilder

Frame builder for constructing protocol frames.

```rust
use dmsc::protocol::DMSCFrameBuilder;

let mut builder = DMSCFrameBuilder::new();
let control_frame = builder.build_control_frame(vec![0x01, 0x02, 0x03]).ok()?;
let data_frame = builder.build_data_frame(b"Hello".to_vec()).ok()?;
let auth_frame = builder.build_auth_frame(vec![0xFF]).ok()?;
let keepalive_frame = builder.build_keepalive_frame().ok()?;
```

### DMSCFrameParser

Frame parser for parsing protocol frames from byte streams.

```rust
use dmsc::protocol::frames::DMSCFrameParser;

let mut parser = DMSCFrameParser::new();
parser.add_data(received_bytes);

if let Some(frame) = parser.parse_frame() {
    println!("Received frame: {:?}", frame.header.frame_type);
}
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
use dmsc::protocol::DMSCGlobalStateManager;

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

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [queue](./queue.md): Message queue module providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
