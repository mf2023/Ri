<div align="center">

# Device API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The device module provides smart device control functionality, including device discovery, resource scheduling, and resource pool management.

## Module Overview

</div>

The device module contains the following sub-modules:

- **core**: Device core interfaces and type definitions
- **controller**: Device controller
- **scheduler**: Device scheduler
- **pool**: Resource pool management
- **discovery_scheduler**: Device discovery and scheduling engine

<div align="center">

## Core Components

</div>

### DMSCDeviceControlModule

The main device control module interface, providing unified device management functionality.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create device control module | None | `Self` |
| `with_config(config)` | Configure module | `config: DMSCDeviceControlConfig` | `Self` |
| `discover_devices()` | Discover devices | None | `DMSCResult<DMSCDiscoveryResult>` |
| `allocate_resource(request)` | Allocate resource | `request: DMSCResourceRequest` | `DMSCResult<Option<DMSCResourceAllocation>>` |
| `release_resource(allocation_id)` | Release resource | `allocation_id: &str` | `DMSCResult<()>` |
| `get_device_status()` | Get device status | None | `DMSCResult<Vec<DMSCDevice>>` |
| `get_resource_pool_status()` | Get resource pool status | None | `HashMap<String, DMSCResourcePoolStatus>` |

#### Usage Example

```rust
use dms::prelude::*;
use dms::device::{DMSCDeviceControlConfig, DMSCResourceRequest, DMSCDeviceType, DMSCDeviceCapabilities};

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

Device struct.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Device ID |
| `name` | `String` | Device name |
| `device_type` | `DMSCDeviceType` | Device type |
| `status` | `DMSCDeviceStatus` | Device status |
| `capabilities` | `DMSCDeviceCapabilities` | Device capabilities |
| `metadata` | `HashMap<String, String>` | Metadata |

### DMSCDeviceType

Device type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `CPU` | CPU device |
| `GPU` | GPU device |
| `Memory` | Memory device |
| `Storage` | Storage device |
| `Network` | Network device |
| `Compute` | Compute device |
| `Sensor` | Sensor device |
| `Actuator` | Actuator device |

### DMSCDeviceStatus

Device status enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Available` | Available |
| `Busy` | Busy |
| `Offline` | Offline |
| `Error` | Error |

### DMSCDeviceCapabilities

Device capabilities struct.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `cpu_cores` | `Option<u32>` | CPU cores |
| `memory_gb` | `Option<f64>` | Memory size (GB) |
| `storage_gb` | `Option<f64>` | Storage size (GB) |
| `gpu_enabled` | `Option<bool>` | GPU support |
| `network_speed_mbps` | `Option<f64>` | Network speed (Mbps) |
| `extra` | `HashMap<String, Value>` | Extended properties |

<div align="center">

## Resource Scheduling

</div>

### DMSCResourceRequest

Resource request struct.

```rust
use dms::device::{DMSCResourceRequest, DMSCDeviceType, DMSCDeviceCapabilities};

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

Resource allocation result.

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

SLA priority class.

| Variant | Description |
|:--------|:-------------|
| `Critical` | Mission critical |
| `High` | High priority |
| `Medium` | Normal priority |
| `Low` | Low priority |

<div align="center">

## Resource Pool Management

</div>

### DMSCResourcePool

Resource pool interface.

```rust
use dms::device::DMSCResourcePool;

let pool = DMSCResourcePool::new("gpu-pool".to_string(), 10);

let status = pool.get_status();
println!("Total capacity: {}", status.total_capacity);
println!("Available capacity: {}", status.available_capacity);
println!("Utilization rate: {:.2}%", status.utilization_rate * 100.0);
```

### DMSCResourcePoolStatus

Resource pool status.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `total_capacity` | `usize` | Total capacity |
| `available_capacity` | `usize` | Available capacity |
| `allocated_capacity` | `usize` | Allocated capacity |
| `pending_requests` | `usize` | Pending requests |
| `utilization_rate` | `f64` | Utilization rate |

<div align="center">

## Device Discovery

</div>

### DMSCDiscoveryResult

Device discovery result.

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

## Configuration Options

</div>

### DMSCDeviceControlConfig

Device control configuration.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `discovery_enabled` | `bool` | Enable discovery | `true` |
| `discovery_interval_secs` | `u64` | Discovery interval (seconds) | `30` |
| `auto_scheduling_enabled` | `bool` | Enable auto scheduling | `true` |
| `max_concurrent_tasks` | `usize` | Max concurrent tasks | `100` |
| `resource_allocation_timeout_secs` | `u64` | Allocation timeout (seconds) | `60` |
| `enable_cpu_discovery` | `bool` | Discover CPU devices | `true` |
| `enable_gpu_discovery` | `bool` | Discover GPU devices | `true` |
| `enable_memory_discovery` | `bool` | Discover memory devices | `true` |
| `enable_storage_discovery` | `bool` | Discover storage devices | `true` |
| `enable_network_discovery` | `bool` | Discover network devices | `true` |

<div align="center">

## Best Practices

</div>

1. **Enable device discovery**: Enable/disable specific device type discovery based on actual needs
2. **Set appropriate timeouts**: Set appropriate timeout values based on device type and task complexity
3. **Use SLA classification**: Use high priority SLA for critical tasks to ensure resource allocation priority
4. **Monitor resource pools**: Regularly check resource pool status and utilization
5. **Release resources promptly**: Release resources no longer needed after task completion
6. **Configure affinity**: Configure resource affinity and anti-affinity rules as needed

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [core](./core.md): Core module providing error handling and service context
- [log](./log.md): Logging module for device events
- [config](./config.md): Configuration module for device settings
- [cache](./cache.md): Caching module for device state
- [observability](./observability.md): Observability module for device performance monitoring
- [service_mesh](./service_mesh.md): Service mesh module for device service discovery
