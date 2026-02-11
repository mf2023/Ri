<div align="center">

# Protocol API参考

**Version: 0.1.7**

**Last modified date: 2026-01-18**

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
| `send_message(target, message)` | 发送消息（返回详细响应） | `target: &str`, `message: &[u8]` | `Vec<u8>` (JSON响应) |
| `send_message_with_flags(target, message, flags)` | 使用自定义标志发送消息 | `target: &str`, `message: &[u8]`, `flags: DMSCMessageFlags` | `Vec<u8>` (JSON响应) |
| `get_stats()` | 获取统计信息 | 无 | `DMSCProtocolStats` |
| `close_connection(connection_id)` | 关闭连接 | `connection_id: &str` | `bool` |
| `get_connection_info(connection_id)` | 获取连接信息 | `connection_id: &str` | `Option<DMSCConnectionInfo>` |

#### 响应格式

`send_message`方法返回包含详细信息的JSON响应：

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

#### 使用示例

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
| `security_level` | `DMSCSecurityLevel` | 安全级别 | `Standard` |
| `enable_state_sync` | `bool` | 启用状态同步 | `true` |
| `performance_optimization` | `bool` | 启用性能优化 | `true` |

### DMSCSecurityLevel

安全级别枚举。

| 变体 | 描述 |
|:--------|:-------------|
| `None` | 无安全 |
| `Standard` | 标准安全 |
| `High` | 高安全 |
| `Military` | 军事级安全 |

<div align="center">

## 安全功能

</div>

### DMSCCryptoSuite

加密套件接口。

```rust
use dmsc::protocol::{DMSCCryptoSuite, AES256GCM, ChaCha20Poly1305};

let aes = AES256GCM::new();
let encrypted = aes.encrypt(data, &key, &nonce).await?;
let decrypted = aes.decrypt(&encrypted, &key, &nonce).await?;

let chacha = ChaCha20Poly1305::new();
let encrypted = chacha.encrypt(data, &key, &nonce).await?;
```

### DMSCDeviceAuthProtocol

设备认证协议。

```rust
use dmsc::protocol::DMSCDeviceAuthProtocol;

let auth = DMSCDeviceAuthProtocol::new();
let device_cert = auth.generate_device_certificate(device_id, public_key)?;
let auth_result = auth.authenticate_device(&device_cert, &signature).await?;
```

### DMSCPostQuantumCrypto

后量子密码学。

```rust
use dmsc::protocol::DMSCPostQuantumCrypto;

let pq_crypto = DMSCPostQuantumCrypto::new();
let (public_key, secret_key) = pq_crypto.generate_keypair()?;
let ciphertext = pq_crypto.encrypt(&public_key, plaintext)?;
let plaintext = pq_crypto.decrypt(&secret_key, &ciphertext)?;
```

<div align="center">

## 国密算法

</div>

### DMSCGuomi

国密算法套件（SM2/SM3/SM4）。

```rust
use dmsc::protocol::guomi::{DMSCGuomi, SM2Signer, SM3, SM4};

// SM2 签名
let signer = SM2Signer::new()?;
let (sm2_public, sm2_private) = signer.keygen()?;
let signature = signer.sign(&sm2_private, &message)?;

// SM3 哈希 - 返回 DMSCResult<[u8; 32]>
let sm3 = SM3::new();
let sm3_hash = sm3.hash(&data)?;

// SM4 加密
let sm4 = SM4::new();
let sm4_key = [0u8; 16]; // 16字节密钥
let encrypted = sm4.encrypt_ecb(&sm4_key, &data)?;
```

### SM2Signer

SM2 椭圆曲线数字签名算法。

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建签名器 | 无 | `DMSCResult<Self>` |
| `keygen()` | 生成密钥对 | 无 | `DMSCResult<(Vec<u8>, Vec<u8>)>` |
| `sign(secret_key, message)` | 签名消息 | `secret_key: &[u8]`, `message: &[u8]` | `DMSCResult<Vec<u8>>` |
| `verify(public_key, message, signature)` | 验证签名 | `public_key: &[u8]`, `message: &[u8]`, `signature: &[u8]` | `DMSCResult<bool>` |

### SM3

SM3 密码学哈希算法。

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建哈希器 | 无 | `Self` |
| `hash(data)` | 计算哈希 | `data: &[u8]` | `DMSCResult<[u8; 32]>` |

### SM4

SM4 分组密码算法。

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建密码器 | 无 | `Self` |
| `encrypt_ecb(key, plaintext)` | ECB 模式加密 | `key: &[u8; 16]`, `plaintext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt_ecb(key, ciphertext)` | ECB 模式解密 | `key: &[u8; 16]`, `ciphertext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `encrypt_cbc(key, iv, plaintext)` | CBC 模式加密 | `key: &[u8; 16]`, `iv: &[u8; 16]`, `plaintext: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt_cbc(key, iv, ciphertext)` | CBC 模式解密 | `key: &[u8; 16]`, `iv: &[u8; 16]`, `ciphertext: &[u8]` | `DMSCResult<Vec<u8>>` |

<div align="center">

## 硬件安全模块 (HSM)

</div>

### DMSCHSMManager

HSM管理器。

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
println!("生成的密钥数: {}", stats.keys_generated);
println!("执行的操作数: {}", stats.operations_performed);
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

帧构建器，用于构建协议帧。

```rust
use dmsc::protocol::DMSCFrameBuilder;

let mut builder = DMSCFrameBuilder::new();
let control_frame = builder.build_control_frame(vec![0x01, 0x02, 0x03]).ok()?;
let data_frame = builder.build_data_frame(b"Hello".to_vec()).ok()?;
let auth_frame = builder.build_auth_frame(vec![0xFF]).ok()?;
let keepalive_frame = builder.build_keepalive_frame().ok()?;
```

### DMSCFrameParser

帧解析器，用于从字节流中解析协议帧。

```rust
use dmsc::protocol::frames::DMSCFrameParser;

let mut parser = DMSCFrameParser::new();
parser.add_data(received_bytes);

if let Some(frame) = parser.parse_frame() {
    println!("Received frame: {:?}", frame.header.frame_type);
}
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

## 错误处理

</div>

### DMSCProtocolStats

协议统计信息。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `total_messages_sent` | `u64` | 发送消息总数 |
| `total_messages_received` | `u64` | 接收消息总数 |
| `total_bytes_sent` | `u64` | 发送字节总数 |
| `total_bytes_received` | `u64` | 接收字节总数 |
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

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [queue](./queue.md): 消息队列模块，提供消息队列支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
