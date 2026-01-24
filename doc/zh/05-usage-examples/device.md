<div align="center">

# 设备控制使用示例

**Version: 0.1.6**

**Last modified date: 2026-01-24**

本示例展示如何使用 device 模块进行设备发现、资源调度和资源池管理。

## 前置要求

</div>

- DMSC Rust SDK
- tokio 异步运行时
- serde_json 用于 JSON 序列化

<div align="center">

## 示例代码

</div>

```rust
use dmsc::prelude::*;
use dmsc::device::{DMSCDeviceControlModule, DMSCDeviceControlConfig, DMSCDeviceType, DMSCDeviceCapabilities, DMSCResourceRequest};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    println!("=== DMSC Device Control Example ===\n");
    
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
    
    println!("1. Device Discovery");
    println!("--------------------");
    
    let discovery_result = device_module.discover_devices().await?;
    println!("Discovered {} new devices", discovery_result.discovered_devices.len());
    println!("Updated {} devices", discovery_result.updated_devices.len());
    println!("Removed {} devices", discovery_result.removed_devices.len());
    println!("Total devices: {}\n", discovery_result.total_devices);
    
    for device in &discovery_result.discovered_devices {
        println!("  - {} ({:?}) - Status: {:?}", 
                 device.name, device.device_type, device.status);
    }
    
    println!("\n2. Resource Allocation");
    println!("-----------------------");
    
    let resource_request = DMSCResourceRequest {
        request_id: "gpu-compute-001".to_string(),
        device_type: DMSCDeviceType::GPU,
        required_capabilities: DMSCDeviceCapabilities {
            cpu_cores: Some(8),
            memory_gb: Some(32.0),
            storage_gb: Some(500.0),
            gpu_enabled: Some(true),
            network_speed_mbps: Some(10000.0),
            extra: std::collections::HashMap::new(),
        },
        priority: 8,
        timeout_secs: 300,
        sla_class: Some(dmsc::device::DMSCRequestSlaClass::High),
        resource_weights: Some(dmsc::device::DMSCResourceWeights {
            compute_weight: 1.0,
            memory_weight: 0.8,
            storage_weight: 0.5,
            bandwidth_weight: 0.6,
        }),
        affinity: None,
        anti_affinity: None,
    };
    
    match device_module.allocate_resource(resource_request).await? {
        Some(allocation) => {
            println!("Resource allocated successfully!");
            println!("  Allocation ID: {}", allocation.allocation_id);
            println!("  Device ID: {}", allocation.device_id);
            println!("  Device Name: {}", allocation.device_name);
            println!("  Allocated At: {}", allocation.allocated_at);
            println!("  Expires At: {}\n", allocation.expires_at);
            
            println!("3. Using Allocated Resource");
            println!("----------------------------");
            println!("Simulating GPU computation task...\n");
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            println!("Task completed!\n");
            
            println!("4. Resource Release");
            println!("-------------------");
            device_module.release_resource(&allocation.allocation_id).await?;
            println!("Resource released successfully!");
        }
        None => {
            println!("Failed to allocate resource");
        }
    }
    
    println!("\n5. Get All Device Status");
    println!("------------------------");
    
    let devices = device_module.get_device_status().await?;
    println!("Total managed devices: {}", devices.len());
    
    for device in &devices {
        println!("  - {} ({:?}) - {:?}", 
                 device.name, device.device_type, device.status);
    }
    
    println!("\n=== Example Completed ===");
    Ok(())
}
```

<div align="center">

## 运行步骤

</div>

### 1. 创建示例项目

```bash
cargo new dms-device-example
cd dms-device-example
```

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC.git" }
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### 3. 运行示例

```bash
cargo run
```

<div align="center">

## 预期输出

</div>

```
=== DMSC Device Control Example ===

1. Device Discovery
--------------------
Discovered 5 new devices
Updated 0 devices
Removed 0 devices
Total devices: 5

  - CPU-0 (Cpu) - Status: Available
  - GPU-0 (Gpu) - Status: Available
  - Memory-0 (Memory) - Status: Available
  - Storage-0 (Storage) - Status: Available
  - Network-0 (Network) - Status: Available

2. Resource Allocation
-----------------------
Resource allocated successfully!
  Allocation ID: abc-123-def
  Device ID: gpu-001
  Device Name: GPU-0
  Allocated At: 2024-01-15T10:30:00Z
  Expires At: 2024-01-15T10:35:00Z

3. Using Allocated Resource
----------------------------
Simulating GPU computation task...

Task completed!

4. Resource Release
-------------------
Resource released successfully!

5. Get All Device Status
------------------------
Total managed devices: 5
  - CPU-0 (Cpu) - Available
  - GPU-0 (Gpu) - Available
  - Memory-0 (Memory) - Available
  - Storage-0 (Storage) - Available
  - Network-0 (Network) - Available

=== Example Completed ===
```

<div align="center">

## 高级功能

</div>

### 批量资源分配

```rust
async fn allocate_multiple_resources(device_module: &DMSCDeviceControlModule) -> DMSCResult<()> {
    let requests = vec![
        DMSCResourceRequest {
            request_id: "req-1".to_string(),
            device_type: DMSCDeviceType::CPU,
            required_capabilities: DMSCDeviceCapabilities::default(),
            priority: 5,
            timeout_secs: 60,
            ..Default::default()
        },
        DMSCResourceRequest {
            request_id: "req-2".to_string(),
            device_type: DMSCDeviceType::Memory,
            required_capabilities: DMSCDeviceCapabilities::default(),
            priority: 5,
            timeout_secs: 60,
            ..Default::default()
        },
    ];
    
    for request in requests {
        if let Some(allocation) = device_module.allocate_resource(request).await? {
            println!("Allocated: {}", allocation.device_name);
        }
    }
    
    Ok(())
}
```

### 资源池状态监控

```rust
fn monitor_resource_pools(device_module: &DMSCDeviceControlModule) {
    let pool_status = device_module.get_resource_pool_status();
    
    for (pool_name, status) in pool_status {
        println!("Pool: {}", pool_name);
        println!("  Total: {}", status.total_capacity);
        println!("  Available: {}", status.available_capacity);
        println!("  Utilization: {:.1}%", status.utilization_rate * 100.0);
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **启用适当的设备发现**：根据实际需求启用/禁用特定类型设备发现
2. **合理设置优先级**：对关键任务使用高优先级确保资源优先分配
3. **设置合适的超时**：根据任务复杂度设置合理的资源分配超时
4. **及时释放资源**：任务完成后立即释放不再需要的资源
5. **监控资源池**：定期检查资源池状态和利用率

<div align="center">

## 相关模块

</div>

- [README](./README.md)：使用示例总览，提供快速导航
- [authentication](./authentication.md)：认证示例，包括JWT、OAuth2和多因素认证
- [basic-app](./basic-app.md)：基础应用示例
- [caching](./caching.md)：缓存示例，包括内存缓存和分布式缓存
- [database](./database.md)：数据库操作示例
- [fs](./fs.md)：文件系统操作示例
- [gateway](./gateway.md)：API网关示例
- [hooks](./hooks.md)：钩子系统示例
- [http](./http.md)：HTTP服务器和客户端示例
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信
- [mq](./mq.md)：消息队列示例
- [observability](./observability.md)：可观测性示例
- [protocol](./protocol.md)：协议模块示例
- [security](./security.md)：安全和加密示例
- [service_mesh](./service_mesh.md)：服务网格示例
- [storage](./storage.md)：云存储示例
- [validation](./validation.md)：数据验证示例

<div align="center">

## 相关文档

</div>

- [设备API参考](../04-api-reference/device.md)：详细的API文档
- [核心概念](../03-core-concepts.md)：了解DMSC核心设计理念
- [最佳实践](../06-best-practices.md)：更多最佳实践建议
