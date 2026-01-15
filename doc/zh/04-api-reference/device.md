<div align="center">

# Device API参考

**Version: 0.1.4**

**Last modified date: 2026-01-15**

device模块提供智能设备控制功能，包括设备发现、资源调度和资源池管理。

## 模块概述

</div>

device模块包含以下子模块：

- **core**: 设备核心接口和类型定义
- **controller**: 设备控制器
- **scheduler**: 设备调度器
- **pool**: 资源池管理
- **discovery_scheduler**: 设备发现与调度引擎

<div align="center">

## 核心组件

</div>

### DMSCDeviceControlModule

设备控制模块主接口，提供统一的设备管理功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建设备控制模块 | 无 | `Self` |
| `with_config(config)` | 配置模块 | `config: DMSCDeviceControlConfig` | `Self` |
| `discover_devices()` | 发现设备 | 无 | `DMSCResult<DMSCDiscoveryResult>` |
| `allocate_resource(request)` | 分配资源 | `request: DMSCResourceRequest` | `DMSCResult<Option<DMSCResourceAllocation>>` |
| `release_resource(allocation_id)` | 释放资源 | `allocation_id: &str` | `DMSCResult<()>` |
| `get_device_status()` | 获取设备状态 | 无 | `DMSCResult<Vec<DMSCDevice>>` |
| `get_resource_pool_status()` | 获取资源池状态 | 无 | `HashMap<String, DMSCResourcePoolStatus>` |

#### 使用示例

```rust
use dmsc::prelude::*;
use dmsc::device::{DMSCDeviceControlConfig, DMSCResourceRequest, DMSCDeviceType, DMSCDeviceCapabilities};

async fn example() -> DMSCResult<()> {
    let device_config = DMSCDeviceControlConfig {
        discovery_enabled: true,
        discovery_interval_secs: 30,
        auto_scheduling_enabled: true,
        max_concurrent_tasks: 100,
        resource_allocation_timeout_secs: 60,
        enable_cpu_discovery: true,
        enable_gpu_discovery: true,
        enable_memory_discovery: true,
        enable_storage_discovery: true,
        enable_network_discovery: true,
    };
    
    let device_module = DMSCDeviceControlModule::new()
        .with_config(device_config);
    
    let discovery_result = device_module.discover_devices().await?;
    println!("Discovered {} devices, total devices: {}", 
             discovery_result.discovered_devices.len(), 
             discovery_result.total_devices);
    
    let resource_request = DMSCResourceRequest {
        request_id: "request-123".to_string(),
        device_type: DMSCDeviceType::Compute,
        required_capabilities: DMSCDeviceCapabilities {
            cpu_cores: Some(4),
            memory_gb: Some(8.0),
            storage_gb: Some(100.0),
            gpu_enabled: Some(true),
            network_speed_mbps: Some(1000.0),
            extra: Default::default(),
        },
        priority: 5,
        timeout_secs: 30,
        sla_class: None,
        resource_weights: None,
        affinity: None,
        anti_affinity: None,
    };
    
    if let Some(allocation) = device_module.allocate_resource(resource_request).await? {
        println!("Allocated device: {} (ID: {})", 
                 allocation.device_name, 
                 allocation.device_id);
        
        device_module.release_resource(&allocation.allocation_id).await?;
    }
    
    Ok(())
}
```

### DMSCDevice

设备结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 设备ID |
| `name` | `String` | 设备名称 |
| `device_type` | `DMSCDeviceType` | 设备类型 |
| `status` | `DMSCDeviceStatus` | 设备状态 |
| `capabilities` | `DMSCDeviceCapabilities` | 设备能力 |
| `metadata` | `HashMap<String, String>` | 元数据 |

### DMSCDeviceType

设备类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `CPU` | CPU设备 |
| `GPU` | GPU设备 |
| `Memory` | 内存设备 |
| `Storage` | 存储设备 |
| `Network` | 网络设备 |
| `Compute` | 计算设备 |
| `Sensor` | 传感器设备 |
| `Actuator` | 执行器设备 |

### DMSCDeviceStatus

设备状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Available` | 可用 |
| `Busy` | 忙碌 |
| `Offline` | 离线 |
| `Error` | 错误 |

### DMSCDeviceCapabilities

设备能力结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `cpu_cores` | `Option<u32>` | CPU核心数 |
| `memory_gb` | `Option<f64>` | 内存大小(GB) |
| `storage_gb` | `Option<f64>` | 存储大小(GB) |
| `gpu_enabled` | `Option<bool>` | 是否支持GPU |
| `network_speed_mbps` | `Option<f64>` | 网络速度(Mbps) |
| `extra` | `HashMap<String, Value>` | 扩展属性 |

<div align="center">

## 资源调度

</div>

### DMSCResourceRequest

资源请求结构体。

```rust
use dmsc::device::{DMSCResourceRequest, DMSCDeviceType, DMSCDeviceCapabilities};

let request = DMSCResourceRequest {
    request_id: "req-001".to_string(),
    device_type: DMSCDeviceType::GPU,
    required_capabilities: DMSCDeviceCapabilities {
        cpu_cores: Some(8),
        memory_gb: Some(32.0),
        storage_gb: Some(500.0),
        gpu_enabled: Some(true),
        network_speed_mbps: Some(10000.0),
        extra: HashMap::new(),
    },
    priority: 8,
    timeout_secs: 120,
    sla_class: Some(DMSCRequestSlaClass::High),
    resource_weights: Some(DMSCResourceWeights {
        compute_weight: 1.0,
        memory_weight: 1.0,
        storage_weight: 0.5,
        bandwidth_weight: 0.8,
    }),
    affinity: None,
    anti_affinity: None,
};
```

### DMSCResourceAllocation

资源分配结果。

```rust
if let Some(allocation) = device_module.allocate_resource(request).await? {
    println!("Allocation ID: {}", allocation.allocation_id);
    println!("Device ID: {}", allocation.device_id);
    println!("Device Name: {}", allocation.device_name);
    println!("Allocated At: {}", allocation.allocated_at);
    println!("Expires At: {}", allocation.expires_at);
    
    if chrono::Utc::now() > allocation.expires_at {
        println!("Allocation has expired");
    }
}
```

### DMSCRequestSlaClass

SLA 优先级类别。

| 变体 | 描述 |
|:--------|:-------------|
| `Critical` | 关键任务 |
| `High` | 高优先级 |
| `Medium` | 普通优先级 |
| `Low` | 低优先级 |

<div align="center">

## 资源池管理

</div>

### DMSCResourcePool

资源池接口。

```rust
use dmsc::device::DMSCResourcePool;

let pool = DMSCResourcePool::new("gpu-pool".to_string(), 10);

let status = pool.get_status();
println!("Total capacity: {}", status.total_capacity);
println!("Available capacity: {}", status.available_capacity);
println!("Utilization rate: {:.2}%", status.utilization_rate * 100.0);
```

### DMSCResourcePoolStatus

资源池状态。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `total_capacity` | `usize` | 总容量 |
| `available_capacity` | `usize` | 可用容量 |
| `allocated_capacity` | `usize` | 已分配容量 |
| `pending_requests` | `usize` | 等待请求数 |
| `utilization_rate` | `f64` | 利用率 |

<div align="center">

## 设备发现

</div>

### DMSCDiscoveryResult

设备发现结果。

```rust
let discovery_result = device_module.discover_devices().await?;

println!("Newly discovered: {} devices", discovery_result.discovered_devices.len());
println!("Updated: {} devices", discovery_result.updated_devices.len());
println!("Removed: {} devices", discovery_result.removed_devices.len());
println!("Total devices: {}", discovery_result.total_devices);

for device in &discovery_result.discovered_devices {
    println!("Device: {} ({:?})", device.name, device.device_type);
}
```

<div align="center">

## 配置选项

</div>

### DMSCDeviceControlConfig

设备控制配置。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `discovery_enabled` | `bool` | 是否启用发现 | `true` |
| `discovery_interval_secs` | `u64` | 发现间隔(秒) | `30` |
| `auto_scheduling_enabled` | `bool` | 是否自动调度 | `true` |
| `max_concurrent_tasks` | `usize` | 最大并发任务数 | `100` |
| `resource_allocation_timeout_secs` | `u64` | 分配超时(秒) | `60` |
| `enable_cpu_discovery` | `bool` | 发现CPU设备 | `true` |
| `enable_gpu_discovery` | `bool` | 发现GPU设备 | `true` |
| `enable_memory_discovery` | `bool` | 发现内存设备 | `true` |
| `enable_storage_discovery` | `bool` | 发现存储设备 | `true` |
| `enable_network_discovery` | `bool` | 发现网络设备 | `true` |

<div align="center">

## 最佳实践

</div>

1. **启用设备发现**: 根据实际需求启用/禁用特定类型设备发现
2. **合理设置超时**: 根据设备类型和任务复杂度设置合适的超时时间
3. **使用SLA分类**: 对关键任务使用高优先级SLA确保资源优先分配
4. **监控资源池**: 定期检查资源池状态和利用率
5. **及时释放资源**: 任务完成后及时释放不再需要的资源
6. **配置亲和性**: 根据需求配置资源亲和性和反亲和性规则

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
