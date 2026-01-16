<div align="center">

# Security API Reference

**Version: 0.1.4**

**Last modified date: 2026-01-15**

The security module provides encryption, decryption, and cryptographic functionality.

## Module Overview

</div>

The security module contains the following components:

- **encryption**: AES-256-GCM encryption/decryption
- **hmac**: HMAC signing and verification
- **key management**: Encryption key generation and management

<div align="center">

## Core Components

</div>

### DMSCSecurityManager

The security manager provides unified access to cryptographic functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `encrypt(plaintext)` | Encrypt data with AES-256-GCM | `plaintext: &str` | `String` |
| `decrypt(encrypted)` | Decrypt data | `encrypted: &str` | `Option<String>` |
| `hmac_sign(data)` | Sign data with HMAC | `data: &str` | `String` |
| `hmac_verify(data, signature)` | Verify HMAC signature | `data: &str`, `signature: &str` | `bool` |
| `generate_encryption_key()` | Generate encryption key | None | `String` |
| `generate_hmac_key()` | Generate HMAC key | None | `String` |

**Note**: For JWT authentication, use `DMSCJWTManager` in the auth module.

#### Usage Example

```rust
use dmsc::prelude::*;
use dmsc::auth::DMSCSecurityManager;

// Data encryption
let manager = DMSCSecurityManager;
let plaintext = "confidential information";
let encrypted = manager.encrypt(plaintext);
ctx.logger().info("security", &format!("Encrypted: {}", encrypted))?;

// Data decryption
if let Some(decrypted) = manager.decrypt(&encrypted) {
    ctx.logger().info("security", &format!("Decrypted: {}", decrypted))?;
}

// HMAC signing
let data = "important message";
let signature = manager.hmac_sign(data);
ctx.logger().info("security", &format!("Signature: {}", signature))?;

// HMAC verification
let is_valid = manager.hmac_verify(data, &signature);
ctx.logger().info("security", &format!("HMAC valid: {}", is_valid))?;

// Generate keys
let enc_key = manager.generate_encryption_key();
let hmac_key = manager.generate_hmac_key();
```

### Related Modules

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
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication

