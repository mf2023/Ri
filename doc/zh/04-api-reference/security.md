<div align="center">

# Security API参考

**Version: 0.1.6**

**Last modified date: 2026-01-30**

security模块提供加密、解密和加密功能。

## 模块概述

</div>

security模块包含以下组件：

- **encryption**: AES-256-GCM 加密/解密
- **hmac**: HMAC 签名和验证
- **key management**: 密钥生成和管理

<div align="center">

## 核心组件

</div>

### DMSCSecurityManager

安全管理器提供对加密功能的统一访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `encrypt(plaintext)` | 使用AES-256-GCM加密数据 | `plaintext: &str` | `String` |
| `decrypt(encrypted)` | 解密数据 | `encrypted: &str` | `Option<String>` |
| `hmac_sign(data)` | 使用HMAC签名数据 | `data: &str` | `String` |
| `hmac_verify(data, signature)` | 验证HMAC签名 | `data: &str`, `signature: &str` | `bool` |
| `generate_encryption_key()` | 生成加密密钥 | 无 | `String` |
| `generate_hmac_key()` | 生成HMAC密钥 | 无 | `String` |

**注意**: 如需JWT认证，请使用auth模块中的`DMSCJWTManager`。

#### 使用示例

```rust
use dmsc::prelude::*;
use dmsc::auth::DMSCSecurityManager;

// 数据加密
let manager = DMSCSecurityManager;
let plaintext = "confidential information";
let encrypted = manager.encrypt(plaintext);
ctx.logger().info("security", &format!("Encrypted: {}", encrypted))?;

// 数据解密
if let Some(decrypted) = manager.decrypt(&encrypted) {
    ctx.logger().info("security", &format!("Decrypted: {}", decrypted))?;
}

// HMAC签名
let data = "important message";
let signature = manager.hmac_sign(data);
ctx.logger().info("security", &format!("Signature: {}", signature))?;

// HMAC验证
let is_valid = manager.hmac_verify(data, &signature);
ctx.logger().info("security", &format!("HMAC valid: {}", is_valid))?;

// 生成密钥
let enc_key = manager.generate_encryption_key();
let hmac_key = manager.generate_hmac_key();
```

### 相关模块

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
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信

