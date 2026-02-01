<div align="center">

# Device Control Usage Example

**Version: 0.1.6**

**Last modified date: 2026-01-30**

This example demonstrates how to use the device module for device discovery, resource scheduling, and resource pool management.

## Prerequisites

</div>

- DMSC Rust SDK
- tokio async runtime
- serde_json for JSON serialization

<div align="center">

## Example Code

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

## Running Steps

</div>

### 1. Create Example Project

```bash
cargo new dms-device-example
cd dms-device-example
```

### 2. Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC.git" }
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### 3. Run Example

```bash
cargo run
```

<div align="center">

## Expected Output

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

## Advanced Features

</div>

### Multiple Resource Allocation

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

### Resource Pool Status Monitoring

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

## Best Practices

</div>

1. **Enable appropriate device discovery**: Enable/disable specific device type discovery based on actual needs
2. **Set appropriate priorities**: Use high priority for critical tasks to ensure resource allocation priority
3. **Set appropriate timeouts**: Set reasonable resource allocation timeouts based on task complexity
4. **Release resources promptly**: Release resources no longer needed immediately after task completion
5. **Monitor resource pools**: Regularly check resource pool status and utilization

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with usage examples summary and quick navigation
- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [basic-app](./basic-app.md): Basic application examples
- [caching](./caching.md): Caching examples, including memory and distributed caching
- [database](./database.md): Database operation examples
- [fs](./fs.md): Filesystem operation examples
- [gateway](./gateway.md): API gateway examples
- [hooks](./hooks.md): Hook system examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [observability](./observability.md): Observability examples
- [protocol](./protocol.md): Protocol module examples
- [service_mesh](./service_mesh.md): Service mesh examples
- [validation](./validation.md): Data validation examples

<div align="center">

## Related Documentation

</div>

- [Device API Reference](../04-api-reference/device.md): Detailed API documentation
- [Core Concepts](../03-core-concepts.md): Learn about DMSC core design principles
- [Best Practices](../06-best-practices.md): More best practice suggestions
