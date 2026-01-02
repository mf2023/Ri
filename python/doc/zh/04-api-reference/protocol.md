<div align="center">

# Protocol API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

protocol模块提供协议抽象层，支持全局和私有通信协议，实现分布式系统的协议管理、安全特性和集成能力。

</div>

## 模块概述

protocol模块包含以下核心组件：

- **DMSCProtocolManager**: 协议管理器
- **DMSCProtocolConfig**: 协议配置
- **DMSCProtocolType**: 协议类型枚举
- **DMSCProtocolStats**: 协议统计
- **DMSCProtocolStatus**: 协议状态
- **DMSCProtocolHealth**: 协议健康状态
- **DMSCConnectionState**: 连接状态枚举
- **DMSCConnectionStats**: 连接统计
- **DMSCFrame**: 协议帧
- **DMSCFrameType**: 帧类型枚举
- **DMSCFrameHeader**: 帧头
- **DMSCDeviceAuthProtocol**: 设备认证协议
- **DMSCProtocolAdapter**: 协议适配器
- **DMSCSecurityContext**: 安全上下文
- **DMSCThreatLevel**: 威胁等级枚举
- **DMSCDataClassification**: 数据分类枚举
- **DMSCNetworkEnvironment**: 网络环境枚举

<div align="center">

## 核心组件

</div>

### DMSCProtocolManager

协议管理器，用于管理通信协议。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `initialize(config)` | 初始化协议管理器 | `config: DMSCProtocolConfig` | `None` |
| `send_message(target, message)` | 发送消息 | `target: str`, `message: bytes` | `bytes` |
| `send_message_with_protocol(target, message, protocol_type)` | 使用指定协议发送 | `target: str`, `message: bytes`, `protocol_type: DMSCProtocolType` | `bytes` |
| `switch_protocol(protocol_type)` | 切换协议 | `protocol_type: DMSCProtocolType` | `None` |
| `get_current_protocol()` | 获取当前协议 | 无 | `DMSCProtocolType` |
| `get_stats()` | 获取统计信息 | 无 | `DMSCProtocolStats` |
| `shutdown()` | 关闭协议管理器 | 无 | `None` |

#### 使用示例

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

manager = DMSCProtocolManager()

# 初始化
await manager.initialize(DMSCProtocolConfig())

# 发送消息
response = await manager.send_message("device-001", b"Hello")
print(f"响应: {response}")

# 切换协议
await manager.switch_protocol(DMSCProtocolType.PRIVATE)

# 获取统计
stats = await manager.get_stats()
print(f"发送消息数: {stats.total_messages_sent}")
```

### DMSCProtocolConfig

协议配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `default_protocol` | `DMSCProtocolType` | 默认协议类型 | `GLOBAL` |
| `enable_security` | `bool` | 是否启用安全特性 | `true` |
| `enable_state_sync` | `bool` | 是否启用状态同步 | `true` |
| `performance_optimization` | `bool` | 是否启用性能优化 | `true` |
| `connection_timeout` | `int` | 连接超时（秒） | 30 |
| `max_connections_per_protocol` | `int` | 每个协议最大连接数 | 1000 |

#### 使用示例

```python
from dmsc import DMSCProtocolConfig, DMSCProtocolType

config = DMSCProtocolConfig(
    default_protocol=DMSCProtocolType.GLOBAL,
    enable_security=True,
    connection_timeout=60
)
```

### DMSCProtocolType

协议类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `GLOBAL` | 全局通信协议 |
| `PRIVATE` | 私有通信协议 |

### DMSCProtocolStats

协议统计信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `total_messages_sent` | `int` | 发送消息总数 |
| `total_messages_received` | `int` | 接收消息总数 |
| `total_bytes_sent` | `int` | 发送字节数 |
| `total_bytes_received` | `int` | 接收字节数 |
| `average_latency_ms` | `int` | 平均延迟（毫秒） |
| `error_count` | `int` | 错误计数 |
| `success_rate` | `float` | 成功率 |

### DMSCProtocolStatus

协议状态结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `initialized` | `bool` | 是否已初始化 |
| `active` | `bool` | 是否活跃 |
| `active_connections` | `int` | 活跃连接数 |
| `health` | `DMSCProtocolHealth` | 健康状态 |

### DMSCProtocolHealth

协议健康状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `HEALTHY` | 健康 |
| `DEGRADED` | 降级 |
| `UNHEALTHY` | 不健康 |
| `UNKNOWN` | 未知 |

### DMSCConnectionState

连接状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `CONNECTING` | 连接中 |
| `ESTABLISHED` | 已建立 |
| `ACTIVE` | 活跃 |
| `CLOSING` | 关闭中 |
| `CLOSED` | 已关闭 |
| `FAILED` | 失败 |

### DMSCConnectionStats

连接统计信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `connection_id` | `str` | 连接ID |
| `target_device` | `str` | 目标设备 |
| `protocol_type` | `DMSCProtocolType` | 协议类型 |
| `connection_state` | `DMSCConnectionState` | 连接状态 |
| `messages_sent` | `int` | 发送消息数 |
| `messages_received` | `int` | 接收消息数 |
| `bytes_sent` | `int` | 发送字节数 |
| `bytes_received` | `int` | 接收字节数 |

### DMSCFrame

协议帧结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `frame_type` | `DMSCFrameType` | 帧类型 |
| `header` | `DMSCFrameHeader` | 帧头 |
| `payload` | `bytes` | 载荷数据 |
| `checksum` | `int` | 校验和 |

### DMSCFrameType

帧类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `DATA` | 数据帧 |
| `CONTROL` | 控制帧 |
| `ACK` | 确认帧 |
| `NACK` | 否认帧 |
| `HANDSHAKE` | 握手帧 |

### DMSCFrameHeader

协议帧头。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `frame_id` | `int` | 帧ID |
| `source` | `str` | 源地址 |
| `destination` | `str` | 目标地址 |
| `timestamp` | `int` | 时间戳 |
| `flags` | `int` | 标志位 |

### DMSCDeviceAuthProtocol

设备认证协议。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `authenticate(device_id, credentials)` | 认证设备 | `device_id: str`, `credentials: Dict` | `bool` |
| `get_auth_token(device_id)` | 获取认证令牌 | `device_id: str` | `str` |
| `revoke_token(token)` | 撤销令牌 | `token: str` | `None` |

### DMSCProtocolAdapter

协议适配器，用于适配不同协议。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `encode(message)` | 编码消息 | `message: Dict` | `bytes` |
| `decode(data)` | 解码数据 | `data: bytes` | `Dict` |
| `get_supported_protocols()` | 获取支持的协议 | 无 | `List[str]` |

### DMSCSecurityContext

安全上下文。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `encryption_algorithm` | `str` | 加密算法 |
| `key_id` | `str` | 密钥ID |
| `security_level` | `DMSCThreatLevel` | 安全级别 |

### DMSCThreatLevel

威胁等级枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `LOW` | 低威胁 |
| `MEDIUM` | 中威胁 |
| `HIGH` | 高威胁 |
| `CRITICAL` | 严重威胁 |

### DMSCDataClassification

数据分类枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `PUBLIC` | 公开数据 |
| `INTERNAL` | 内部数据 |
| `CONFIDENTIAL` | 机密数据 |
| `RESTRICTED` | 限制数据 |

### DMSCNetworkEnvironment

网络环境枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `LAN` | 局域网 |
| `WAN` | 广域网 |
| `INTERNET` | 互联网 |
| `VPN` | VPN |
| `OFFLINE` | 离线 |

<div align="center">

## 完整使用示例

</div>

```python
from dmsc import (
    DMSCProtocolManager,
    DMSCProtocolConfig,
    DMSCProtocolType,
    DMSCFrame,
    DMSCFrameType
)

async def protocol_example():
    """协议模块完整示例"""
    
    # 初始化协议管理器
    config = DMSCProtocolConfig(
        default_protocol=DMSCProtocolType.GLOBAL,
        enable_security=True,
        connection_timeout=30
    )
    manager = DMSCProtocolManager()
    await manager.initialize(config)
    
    # 发送消息
    response = await manager.send_message("device-001", b"Hello via Global Protocol")
    print(f"响应: {response}")
    
    # 切换到私有协议
    await manager.switch_protocol(DMSCProtocolType.PRIVATE)
    
    # 发送安全消息
    secure_response = await manager.send_message("secure-device", b"Sensitive data")
    print(f"安全响应: {secure_response}")
    
    # 获取当前协议
    current = await manager.get_current_protocol()
    print(f"当前协议: {current}")
    
    # 获取统计信息
    stats = await manager.get_stats()
    print(f"发送消息数: {stats.total_messages_sent}")
    print(f"成功率: {stats.success_rate * 100:.2f}%")
    
    # 关闭
    await manager.shutdown()
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供设备认证支持
- [device](./device.md): 设备模块，提供设备通信
- [service_mesh](./service_mesh.md): 服务网格模块，提供服务发现
- [cache](./cache.md): 缓存模块，提供协议数据缓存
