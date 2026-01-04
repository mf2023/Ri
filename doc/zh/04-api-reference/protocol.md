<div align="center">

# Protocol API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

protocol模块提供协议抽象层，支持全局协议和私有通信协议，实现加密、HSM、帧处理等核心功能。

## 模块概述

</div>

protocol模块实现分层架构：

- **协议层**：全局协议和私有协议实现
- **安全层**：加密、认证和安全增强
- **适配器层**：协议抽象和统一接口
- **集成层**：跨协议协调和状态管理
- **全局状态层**：分布式状态管理和同步

<div align="center">

## 核心组件

</div>

### DMSCProtocolManager

协议管理器主接口，提供协议初始化、消息发送、协议切换等功能。

**注意**：此类为内部类型，仅在crate内部使用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `initialize(config)` | 初始化管理器 | `config: DMSCProtocolConfig` | `DMSCResult<()>` |
| `send_message(target, message)` | 发送消息 | `target: &str`, `message: &[u8]` | `DMSCResult<Vec<u8>>` |
| `send_message_with_protocol(target, message, protocol_type)` | 使用指定协议发送 | `target: &str`, `message: &[u8]`, `protocol_type: DMSCProtocolType` | `DMSCResult<Vec<u8>>` |
| `get_stats()` | 获取统计信息 | 无 | `DMSCResult<DMSCProtocolStats>` |
| `shutdown()` | 关闭管理器 | 无 | `DMSCResult<()>` |
| `create_control_center(ctx)` | 创建控制中心 | `ctx: DMSCServiceContext` | `DMSCControlCenter` |

#### 使用示例

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

协议类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Global` | 全局通信协议 |
| `Private` | 私有安全协议 |

### DMSCProtocolConfig

协议配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `default_protocol` | `DMSCProtocolType` | 默认协议类型 | `Global` |
| `enable_security` | `bool` | 启用安全功能 | `true` |
| `enable_state_sync` | `bool` | 启用状态同步 | `true` |
| `performance_optimization` | `bool` | 启用性能优化 | `true` |
| `connection_timeout` | `Duration` | 连接超时 | `30s` |
| `max_connections_per_protocol` | `u32` | 每协议最大连接数 | `1000` |
| `protocol_switching_enabled` | `bool` | 启用协议切换 | `true` |

<div align="center">

## 安全功能

</div>

### DMSCCryptoSuite

加密套件接口。

```rust
use dms::protocol::{DMSCCryptoSuite, AES256GCM, ChaCha20Poly1305};

let aes = AES256GCM::new();
let encrypted = aes.encrypt(data, &key, &nonce).await?;
let decrypted = aes.decrypt(&encrypted, &key, &nonce).await?;

let chacha = ChaCha20Poly1305::new();
let encrypted = chacha.encrypt(data, &key, &nonce).await?;
```

### DMSCDeviceAuthProtocol

设备认证协议。

```rust
use dms::protocol::DMSCDeviceAuthProtocol;

let auth = DMSCDeviceAuthProtocol::new();
let device_cert = auth.generate_device_certificate(device_id, public_key)?;
let auth_result = auth.authenticate_device(&device_cert, &signature).await?;
```

### DMSCPostQuantumCrypto

后量子密码学。

```rust
use dms::protocol::DMSCPostQuantumCrypto;

let pq_crypto = DMSCPostQuantumCrypto::new();
let (public_key, secret_key) = pq_crypto.generate_keypair()?;
let ciphertext = pq_crypto.encrypt(&public_key, plaintext)?;
let plaintext = pq_crypto.decrypt(&secret_key, &ciphertext)?;
```

<div align="center">

## 硬件安全模块(HSM)

</div>

### DMSCHSMManager

HSM管理器。

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

HSM配置。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `hsm_type` | `DMSCHSMType` | HSM类型 |
| `key_path` | `String` | 密钥存储路径 |
| `enable_audit_log` | `bool` | 启用审计日志 |

### DMSCHSMType

HSM类型枚举。

| 变体 | 描述 |
|:--------|:-------------|
| `Software` | 软件HSM |
| `Hardware` | 硬件HSM |
| `Cloud` | 云HSM |

<div align="center">

## 帧处理

</div>

### DMSCFrame

协议帧。

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

帧类型枚举。

| 变体 | 描述 |
|:--------|:-------------|
| `Data` | 数据帧 |
| `Control` | 控制帧 |
| `Ack` | 确认帧 |
| `Handshake` | 握手帧 |
| `Heartbeat` | 心跳帧 |

<div align="center>

## 全局状态管理

</div>

### DMSCGlobalStateManager

全局状态管理器。

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

## 错误处理

</div>

### DMSCProtocolStats

协议统计信息。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `total_messages_sent` | `u64` | 发送消息总数 |
| `total_messages_received` | `u64` | 接收消息总数 |
| `total_bytes_sent` | `u64` | 发送字节数 |
| `total_bytes_received` | `u64` | 接收字节数 |
| `average_latency_ms` | `u64` | 平均延迟(毫秒) |
| `error_count` | `u64` | 错误数 |
| `success_rate` | `f32` | 成功率 |

<div align="center>

## 最佳实践

</div>

1. **使用安全协议**：对敏感操作使用Private协议
2. **启用状态同步**：在分布式环境中保持状态一致
3. **合理配置超时**：根据网络条件设置合适的连接超时
4. **监控协议性能**：定期检查协议统计信息
5. **使用HSM保护密钥**：对关键密钥使用硬件安全模块
6. **启用协议切换**：在需要时动态切换协议类型

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录协议事件
- [device](./device.md): 设备模块，使用协议进行设备通信
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [observability](./observability.md): 可观测性模块，监控协议性能
