<div align="center">

# Device API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

device模块提供设备管理功能，包括设备发现、调度、资源池管理和智能任务分配。

</div>

## 模块概述

device模块包含以下核心组件：

- **DMSCDeviceControlModule**: 设备控制模块主接口
- **DMSCDeviceControlConfig**: 设备控制模块配置
- **DMSCDevice**: 设备表示
- **DMSCDeviceType**: 设备类型枚举
- **DMSCDeviceStatus**: 设备状态枚举
- **DMSCDeviceCapabilities**: 设备能力
- **DMSCDeviceHealthMetrics**: 设备健康指标
- **DMSCDeviceController**: 设备控制器
- **DMSCDeviceConfig**: 设备配置
- **NetworkDeviceInfo**: 网络设备信息
- **DMSCDiscoveryResult**: 设备发现结果
- **DMSCResourceRequest**: 资源请求
- **DMSCResourceAllocation**: 资源分配
- **DMSCRequestSlaClass**: 请求SLA等级
- **DMSCAffinityRules**: 亲和性规则

<div align="center">

## 核心组件

</div>

### DMSCDeviceControlModule

设备控制模块主接口，提供统一的设备管理服务访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `discover_devices()` | 发现可用设备 | 无 | `List[DMSCDevice]` |
| `get_device(device_id)` | 根据ID获取设备 | `device_id: str` | `DMSCDevice` |
| `get_all_devices()` | 获取所有已注册设备 | 无 | `List[DMSCDevice]` |
| `register_device(device)` | 注册新设备 | `device: DMSCDevice` | `bool` |
| `unregister_device(device_id)` | 注销设备 | `device_id: str` | `bool` |

#### 使用示例

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceControlConfig

config = DMSCDeviceControlConfig()
module = DMSCDeviceControlModule(config)

# 发现设备
devices = await module.discover_devices()
print(f"发现 {len(devices)} 个设备")

# 获取所有设备
all_devices = await module.get_all_devices()

# 注册设备
device = DMSCDevice(device_id="sensor-001", device_type="sensor")
await module.register_device(device)

# 获取设备
device = await module.get_device("sensor-001")
```

### DMSCDeviceControlConfig

设备控制模块配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `max_devices` | `int` | 最大设备数量 | 100 |
| `discovery_interval` | `int` | 设备发现间隔（秒） | 60 |
| `enable_auto_discovery` | `bool` | 是否启用自动发现 | `true` |
| `default_timeout` | `int` | 默认超时时间（秒） | 30 |

#### 使用示例

```python
from dmsc import DMSCDeviceControlConfig

config = DMSCDeviceControlConfig(
    max_devices=200,
    discovery_interval=30,
    enable_auto_discovery=True,
    default_timeout=60
)
```

### DMSCDevice

设备表示，包含设备的基本信息和状态。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `device_id` | `str` | 设备唯一标识 |
| `device_type` | `DMSCDeviceType` | 设备类型 |
| `status` | `DMSCDeviceStatus` | 设备状态 |
| `metadata` | `Dict` | 设备元数据 |

#### 使用示例

```python
from dmsc import DMSCDevice, DMSCDeviceType, DMSCDeviceStatus

device = DMSCDevice(
    device_id="sensor-001",
    device_type=DMSCDeviceType.SENSOR,
    status=DMSCDeviceStatus.ONLINE,
    metadata={"location": "room-101", "firmware": "1.2.3"}
)

print(f"设备ID: {device.device_id}")
print(f"设备类型: {device.device_type}")
print(f"设备状态: {device.status}")
```

### DMSCDeviceType

设备类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `SENSOR` | 传感器设备 |
| `ACTUATOR` | 执行器设备 |
| `CONTROLLER` | 控制器设备 |
| `GATEWAY` | 网关设备 |
| `STORAGE` | 存储设备 |
| `COMPUTE` | 计算设备 |
| `NETWORK` | 网络设备 |
| `CAMERA` | 摄像头设备 |
| `DISPLAY` | 显示设备 |
| `OTHER` | 其他设备 |

### DMSCDeviceStatus

设备状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `ONLINE` | 在线 |
| `OFFLINE` | 离线 |
| `BUSY` | 忙碌 |
| `MAINTENANCE` | 维护中 |
| `ERROR` | 错误状态 |

### DMSCDeviceCapabilities

设备能力定义。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `supported_protocols` | `List[str]` | 支持的协议列表 |
| `max_connections` | `int` | 最大连接数 |
| `features` | `List[str]` | 功能特性列表 |
| `properties` | `Dict` | 设备属性 |

### DMSCDeviceHealthMetrics

设备健康指标。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `cpu_usage` | `float` | CPU使用率 |
| `memory_usage` | `float` | 内存使用率 |
| `temperature` | `float` | 温度 |
| `uptime` | `int` | 运行时间（秒） |
| `error_count` | `int` | 错误计数 |

### DMSCDeviceController

设备控制器，用于管理设备连接和命令执行。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `connect()` | 连接设备 | 无 | `bool` |
| `disconnect()` | 断开连接 | 无 | `bool` |
| `send_command(command, data)` | 发送命令 | `command: str`, `data: Dict` | `Dict` |
| `get_status()` | 获取状态 | 无 | `DMSCDeviceStatus` |

#### 使用示例

```python
from dmsc import DMSCDeviceController

controller = DMSCDeviceController(device_id="sensor-001")

# 连接设备
if await controller.connect():
    print("设备连接成功")

# 发送命令
response = await controller.send_command("get_data", {"interval": 5})
print(f"响应: {response}")

# 断开连接
await controller.disconnect()
```

### DMSCDeviceConfig

设备配置结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `device_id` | `str` | 设备ID |
| `name` | `str` | 设备名称 |
| `device_type` | `DMSCDeviceType` | 设备类型 |
| `connection_config` | `Dict` | 连接配置 |

### NetworkDeviceInfo

网络设备信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `ip_address` | `str` | IP地址 |
| `mac_address` | `str` | MAC地址 |
| `port` | `int` | 端口号 |
| `protocol` | `str` | 通信协议 |

### DMSCDiscoveryResult

设备发现结果。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `device_id` | `str` | 发现的设备ID |
| `device_type` | `DMSCDeviceType` | 设备类型 |
| `address` | `str` | 设备地址 |
| `metadata` | `Dict` | 设备元数据 |

### DMSCResourceRequest

资源请求定义。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `resource_type` | `str` | 资源类型 |
| `quantity` | `int` | 数量 |
| `priority` | `int` | 优先级 |
| `constraints` | `Dict` | 约束条件 |

### DMSCResourceAllocation

资源分配结果。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `request_id` | `str` | 请求ID |
| `allocated_resources` | `List[str]` | 分配的资源ID列表 |
| `status` | `str` | 分配状态 |
| `timestamp` | `int` | 时间戳 |

### DMSCRequestSlaClass

请求SLA等级枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `CRITICAL` | 关键任务 |
| `HIGH` | 高优先级 |
| `MEDIUM` | 中优先级 |
| `LOW` | 低优先级 |
| `BEST_EFFORT` | 尽力而为 |

### DMSCAffinityRules

亲和性规则定义。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `rule_id` | `str` | 规则ID |
| `source_device` | `str` | 源设备类型 |
| `target_device` | `str` | 目标设备类型 |
| `policy` | `str` | 亲和策略 |

<div align="center">

## 完整使用示例

</div>

```python
from dmsc import (
    DMSCDeviceControlModule,
    DMSCDeviceControlConfig,
    DMSCDevice,
    DMSCDeviceType,
    DMSCDeviceStatus,
    DMSCDeviceController
)

async def device_management_example():
    """设备管理完整示例"""
    
    # 初始化配置和模块
    config = DMSCDeviceControlConfig(
        max_devices=100,
        discovery_interval=60,
        enable_auto_discovery=True,
        default_timeout=30
    )
    
    module = DMSCDeviceControlModule(config)
    
    # 发现设备
    print("开始发现设备...")
    discovered = await module.discover_devices()
    print(f"发现 {len(discovered)} 个设备")
    
    # 创建设备
    sensor = DMSCDevice(
        device_id="sensor-001",
        device_type=DMSCDeviceType.SENSOR,
        status=DMSCDeviceStatus.ONLINE,
        metadata={"location": "room-101", "firmware": "1.2.3"}
    )
    
    # 注册设备
    success = await module.register_device(sensor)
    print(f"设备注册: {success}")
    
    # 使用设备控制器
    controller = DMSCDeviceController(device_id="sensor-001")
    await controller.connect()
    
    # 发送命令
    result = await controller.send_command("get_status", {})
    print(f"设备状态: {result}")
    
    await controller.disconnect()
    
    # 获取所有设备
    all_devices = await module.get_all_devices()
    print(f"总共 {len(all_devices)} 个设备")
    
    # 注销设备
    await module.unregister_device("sensor-001")
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [service_mesh](./service_mesh.md): 服务网格模块，提供服务发现和负载均衡
- [cache](./cache.md): 缓存模块，提供设备状态缓存
- [observability](./observability.md): 可观测性模块，监控设备性能指标
