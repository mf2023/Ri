<div align="center">

# Device API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The device module provides device management functionality, including device discovery, scheduling, resource pooling, and intelligent task allocation.

</div>

## Module Overview

The device module contains the following core components:

- **DMSCDeviceControlModule**: Device control module main interface
- **DMSCDeviceControlConfig**: Device control module configuration
- **DMSCDevice**: Device representation
- **DMSCDeviceType**: Device type enumeration
- **DMSCDeviceStatus**: Device status enumeration
- **DMSCDeviceCapabilities**: Device capabilities
- **DMSCDeviceHealthMetrics**: Device health metrics
- **DMSCDeviceController**: Device controller
- **DMSCDeviceConfig**: Device configuration
- **NetworkDeviceInfo**: Network device information
- **DMSCDiscoveryResult**: Device discovery result
- **DMSCResourceRequest**: Resource request
- **DMSCResourceAllocation**: Resource allocation
- **DMSCRequestSlaClass**: Request SLA class
- **DMSCAffinityRules**: Affinity rules

<div align="center">

## Core Components

</div>

### DMSCDeviceControlModule

Device control module main interface, providing unified device management service access.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `discover_devices()` | Discover available devices | None | `List[DMSCDevice]` |
| `get_device(device_id)` | Get device by ID | `device_id: str` | `DMSCDevice` |
| `get_all_devices()` | Get all registered devices | None | `List[DMSCDevice]` |
| `register_device(device)` | Register new device | `device: DMSCDevice` | `bool` |

#### Usage Example

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceControlConfig

# Initialize device control module
config = DMSCDeviceControlConfig(
    enable_discovery=True,
    discovery_interval=60,
    health_check_interval=30
)
device_module = DMSCDeviceControlModule(config)

# Discover available devices
devices = device_module.discover_devices()
print(f"Found {len(devices)} devices")

# Get specific device
device = device_module.get_device("device-001")
print(f"Device: {device.name}, Status: {device.status}")

# Get all registered devices
all_devices = device_module.get_all_devices()
```

### DMSCDevice

Device representation.

```python
from dmsc import DMSCDevice, DMSCDeviceType, DMSCDeviceStatus

# Create device instance
device = DMSCDevice(
    id="sensor-001",
    name="Temperature Sensor",
    device_type=DMSCDeviceType.SENSOR,
    status=DMSCDeviceStatus.ONLINE,
    capabilities=["temperature", "humidity"],
    location="Building A, Floor 1",
    metadata={"manufacturer": "Acme", "model": "T100"}
)

# Access device properties
print(f"Device ID: {device.id}")
print(f"Device Name: {device.name}")
print(f"Device Type: {device.device_type}")
print(f"Status: {device.status}")
```

### Device Types

```python
from dmsc import DMSCDeviceType

# Available device types
sensor = DMSCDeviceType.SENSOR        # Environmental sensors
controller = DMSCDeviceType.CONTROLLER  # Control devices
gateway = DMSCDeviceType.GATEWAY      # Network gateways
actuator = DMSCDeviceType.ACTUATOR    # Actuator devices
camera = DMSCDeviceType.CAMERA        # Camera devices
display = DMSCDeviceType.DISPLAY      # Display devices
printer = DMSCDeviceType.PRINTER      # Printing devices
storage = DMSCDeviceType.STORAGE      # Storage devices
custom = DMSCDeviceType.CUSTOM        # Custom device types
```

### Device Status

```python
from dmsc import DMSCDeviceStatus

# Device status values
online = DMSCDeviceStatus.ONLINE      # Device is online and operational
offline = DMSCDeviceStatus.OFFLINE    # Device is offline
maintenance = DMSCDeviceStatus.MAINTENANCE  # Device under maintenance
error = DMSCDeviceStatus.ERROR        # Device has errors
unknown = DMSCDeviceStatus.UNKNOWN    # Device status unknown
```

## Device Discovery

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceControlConfig

device_module = DMSCDeviceControlModule(
    DMSCDeviceControlConfig(
        enable_discovery=True,
        discovery_interval=60,
        discovery_timeout=30,
        network_ranges=["192.168.1.0/24", "10.0.0.0/16"]
    )
)

# Manual device discovery
devices = device_module.discover_devices()

# Auto-discovery is running in background
# New devices are automatically registered
```

## Device Health Monitoring

```python
from dmsc import DMSCDeviceControlModule

device_module = DMSCDeviceControlModule()

# Get device health metrics
health = device_module.get_device_health("sensor-001")
print(f"CPU Usage: {health.cpu_usage}%")
print(f"Memory Usage: {health.memory_usage}%")
print(f"Temperature: {health.temperature}C")
print(f"Uptime: {health.uptime} seconds")

# Check if device is healthy
if health.is_healthy():
    print("Device is healthy")
else:
    print(f"Device has issues: {health.warnings}")
```

## Resource Management

```python
from dmsc import (
    DMSCDeviceControlModule, DMSCDevice,
    DMSCResourceRequest, DMSCRequestSlaClass,
    DMSCResourceAllocation
)

device_module = DMSCDeviceControlModule()

# Request resources
request = DMSCResourceRequest(
    task_type="inference",
    required_capabilities=["gpu", "memory>8gb"],
    sla_class=DMSCRequestSlaClass.HIGH,
    preferred_locations=["gpu-cluster"],
    max_latency_ms=100
)

# Allocate resources
allocation = device_module.allocate_resources(request)
print(f"Allocated device: {allocation.device_id}")
print(f"Allocated resources: {allocation.resources}")
```

## Best Practices

1. **Regular Health Checks**: Implement regular health checks for all devices
2. **Device Grouping**: Group devices by location, type, or capability
3. **Failover Planning**: Plan failover for critical devices
4. **Capacity Planning**: Monitor device utilization for capacity planning
5. **Secure Communication**: Use secure communication with devices
6. **Document Device Types**: Document supported device types and capabilities
7. **Implement Timeouts**: Always implement timeouts for device operations
8. **Monitor Discovery**: Monitor device discovery for new devices
9. **Handle Disconnections**: Handle device disconnections gracefully
10. **Logging**: Log all device management operations
