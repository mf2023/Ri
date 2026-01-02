<div align="center">

# Device Control Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's device control module for device management, discovery, resource pooling, and intelligent scheduling.

## Example Overview

This example creates a DMSC Python application with the following features:

- Device discovery and registration
- Device pool management
- Intelligent device scheduling
- Resource allocation and management
- Device health monitoring
- Task distribution across devices

## Prerequisites

- Python 3.8+
- Network connectivity for device discovery
- Understanding of IoT device concepts
- (Optional) Physical devices for real testing

## Complete Code Example

```python
import asyncio
from datetime import datetime
from typing import Dict, List, Optional
from enum import Enum
from dataclasses import dataclass

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCDeviceControlModule, DMSCDeviceControlConfig,
    DMSCDevice, DMSCDeviceType, DMSCDeviceStatus,
    DMSCResourceRequest, DMSCRequestSlaClass,
    DMSCScheduler, DMSCTask, DMSCTaskPriority,
    DMSCConfig, DMSCError
)

# Task status enumeration
class TaskStatus(Enum):
    PENDING = "pending"
    RUNNING = "running"
    COMPLETED = "completed"
    FAILED = "failed"
    CANCELLED = "cancelled"

# Task data class
@dataclass
class DeviceTask:
    task_id: str
    device_id: str
    task_type: str
    data: dict
    status: TaskStatus
    created_at: datetime
    started_at: Optional[datetime]
    completed_at: Optional[datetime]
    result: Optional[dict]
    error: Optional[str]

# Device manager
class DeviceManager:
    def __init__(self, module: DMSCDeviceControlModule, context: DMSCServiceContext):
        self.module = module
        self.context = context
        self.logger = context.logger
        self.device_pool = {}
        self.task_queue: List[DeviceTask] = []
        self.active_tasks: Dict[str, DeviceTask] = {}
    
    async def discover_devices(self) -> List[DMSCDevice]:
        """Discover available devices on the network"""
        self.logger.info("device", "Starting device discovery...")
        
        devices = await self.module.discover_devices()
        
        for device in devices:
            self.logger.info("device", f"Discovered device: {device.id} ({device.device_type})")
            if device.id not in self.device_pool:
                self.device_pool[device.id] = device
        
        return devices
    
    async def register_device(self, device: DMSCDevice) -> bool:
        """Register a new device manually"""
        if device.id in self.device_pool:
            self.logger.warn("device", f"Device {device.id} already registered")
            return False
        
        self.device_pool[device.id] = device
        self.logger.info("device", f"Registered device: {device.id}")
        return True
    
    async def get_all_devices(self) -> List[DMSCDevice]:
        """Get all registered devices"""
        return list(self.device_pool.values())
    
    async def get_devices_by_type(self, device_type: DMSCDeviceType) -> List[DMSCDevice]:
        """Get devices filtered by type"""
        return [
            device for device in self.device_pool.values()
            if device.device_type == device_type
        ]
    
    async def get_available_devices(self, count: int, device_type: DMSCDeviceType) -> List[DMSCDevice]:
        """Acquire available devices from pool"""
        available = [
            device for device in self.device_pool.values()
            if device.device_type == device_type and device.status == DMSCDeviceStatus.ONLINE
        ]
        
        if len(available) < count:
            raise DMSCError(
                f"Not enough devices: requested {count}, available {len(available)}",
                "INSUFFICIENT_DEVICES"
            )
        
        selected = available[:count]
        for device in selected:
            device.status = DMSCDeviceStatus.BUSY
        
        self.logger.info("device", f"Acquired {count} {device_type} devices")
        return selected
    
    async def release_devices(self, devices: List[DMSCDevice]):
        """Release devices back to pool"""
        for device in devices:
            device.status = DMSCDeviceStatus.ONLINE
        
        self.logger.info("device", f"Released {len(devices)} devices")
    
    async def get_device_health(self, device_id: str) -> dict:
        """Get device health metrics"""
        if device_id not in self.device_pool:
            raise DMSCError(f"Device {device_id} not found", "DEVICE_NOT_FOUND")
        
        health = await self.module.get_device_health(device_id)
        return {
            "device_id": device_id,
            "cpu_usage": health.cpu_usage,
            "memory_usage": health.memory_usage,
            "temperature": health.temperature,
            "uptime": health.uptime,
            "is_healthy": health.is_healthy,
            "warnings": health.warnings
        }

# Task scheduler
class TaskScheduler:
    def __init__(self, device_manager: DeviceManager, context: DMSCServiceContext):
        self.device_manager = device_manager
        self.context = context
        self.logger = context.logger
        self.pending_tasks: List[DeviceTask] = []
        self.running_tasks: Dict[str, DeviceTask] = {}
        self.completed_tasks: List[DeviceTask] = []
    
    async def create_task(
        self,
        task_type: str,
        data: dict,
        priority: DMSCTaskPriority = DMSCTaskPriority.NORMAL,
        required_device_type: DMSCDeviceType = DMSCDeviceType.GENERIC,
        sla_class: DMSCRequestSlaClass = DMSCRequestSlaClass.NORMAL
    ) -> DeviceTask:
        """Create a new task"""
        task_id = f"task_{datetime.now().timestamp()}"
        
        task = DeviceTask(
            task_id=task_id,
            device_id="",  # Will be assigned when scheduled
            task_type=task_type,
            data=data,
            status=TaskStatus.PENDING,
            created_at=datetime.now(),
            started_at=None,
            completed_at=None,
            result=None,
            error=None
        )
        
        self.pending_tasks.append(task)
        self.logger.info("scheduler", f"Created task: {task_id} ({task_type})")
        
        # Try to schedule immediately
        await self.schedule_tasks()
        
        return task
    
    async def schedule_tasks(self):
        """Schedule pending tasks to available devices"""
        for task in self.pending_tasks[:]:
            # Find available device
            device_type = self._get_device_type_for_task(task.task_type)
            
            try:
                devices = await self.device_manager.get_available_devices(
                    count=1,
                    device_type=device_type
                )
                device = devices[0]
                
                # Assign device and start task
                task.device_id = device.id
                task.status = TaskStatus.RUNNING
                task.started_at = datetime.now()
                
                self.pending_tasks.remove(task)
                self.running_tasks[task.task_id] = task
                
                # Execute task
                asyncio.create_task(self._execute_task(task, device))
                
            except DMSCError as e:
                self.logger.warn("scheduler", f"Cannot schedule task {task.task_id}: {e}")
    
    async def _execute_task(self, task: DeviceTask, device: DMSCDevice):
        """Execute a task on a device"""
        try:
            self.logger.info("scheduler", f"Executing task {task.task_id} on device {device.id}")
            
            # Simulate task execution
            await asyncio.sleep(2)
            
            # Generate result
            task.result = {
                "status": "success",
                "device_id": device.id,
                "data": task.data,
                "executed_at": datetime.now().isoformat()
            }
            task.status = TaskStatus.COMPLETED
            
        except Exception as e:
            task.error = str(e)
            task.status = TaskStatus.FAILED
            self.logger.error("scheduler", f"Task {task.task_id} failed: {e}")
        
        finally:
            task.completed_at = datetime.now()
            
            # Release device
            device.status = DMSCDeviceStatus.ONLINE
            
            # Move to completed
            if task.task_id in self.running_tasks:
                del self.running_tasks[task.task_id]
            self.completed_tasks.append(task)
            
            # Schedule more tasks
            await self.schedule_tasks()
    
    def _get_device_type_for_task(self, task_type: str) -> DMSCDeviceType:
        """Map task type to device type"""
        task_device_map = {
            "data_collection": DMSCDeviceType.SENSOR,
            "video_processing": DMSCDeviceType.CAMERA,
            "actuation": DMSCDeviceType.ACTUATOR,
            "display": DMSCDeviceType.DISPLAY,
            "storage": DMSCDeviceType.STORAGE,
            "gateway": DMSCDeviceType.GATEWAY
        }
        return task_device_map.get(task_type, DMSCDeviceType.GENERIC)
    
    async def cancel_task(self, task_id: str) -> bool:
        """Cancel a pending task"""
        for task in self.pending_tasks:
            if task.task_id == task_id:
                task.status = TaskStatus.CANCELLED
                task.completed_at = datetime.now()
                self.pending_tasks.remove(task)
                self.logger.info("scheduler", f"Cancelled task: {task_id}")
                return True
        return False
    
    def get_task_status(self, task_id: str) -> Optional[DeviceTask]:
        """Get task status"""
        for task in self.pending_tasks + list(self.running_tasks.values()) + self.completed_tasks:
            if task.task_id == task_id:
                return task
        return None
    
    def get_all_tasks(self) -> Dict[str, List[DeviceTask]]:
        """Get all tasks by status"""
        return {
            "pending": self.pending_tasks,
            "running": list(self.running_tasks.values()),
            "completed": self.completed_tasks
        }

# Request handlers
async def handle_discover_devices(context: DMSCServiceContext):
    """Discover and register devices"""
    device_manager = context.device_manager
    devices = await device_manager.discover_devices()
    
    return {
        "status": "success",
        "data": {
            "discovered": len(devices),
            "total_registered": len(device_manager.device_pool),
            "devices": [{"id": d.id, "type": d.device_type.value} for d in devices]
        }
    }

async def handle_get_devices(context: DMSCServiceContext):
    """Get all registered devices"""
    device_manager = context.device_manager
    devices = await device_manager.get_all_devices()
    
    return {
        "status": "success",
        "data": {
            "count": len(devices),
            "devices": [
                {
                    "id": d.id,
                    "type": d.device_type.value,
                    "status": d.status.value,
                    "location": getattr(d, "location", "unknown")
                }
                for d in devices
            ]
        }
    }

async def handle_get_device_health(context: DMSCServiceContext):
    """Get device health metrics"""
    data = await context.http.request.json()
    device_id = data.get("device_id")
    
    if not device_id:
        return {"status": "error", "message": "device_id required"}, 400
    
    device_manager = context.device_manager
    health = await device_manager.get_device_health(device_id)
    
    return {"status": "success", "data": health}

async def handle_create_task(context: DMSCServiceContext):
    """Create a new task"""
    data = await context.http.request.json()
    
    task_type = data.get("task_type", "data_collection")
    data_content = data.get("data", {})
    priority = DMSCTaskPriority(data.get("priority", "normal"))
    device_type = DMSCDeviceType(data.get("device_type", "generic"))
    
    scheduler = context.task_scheduler
    task = await scheduler.create_task(
        task_type=task_type,
        data=data_content,
        priority=priority,
        required_device_type=device_type
    )
    
    return {
        "status": "success",
        "data": {
            "task_id": task.task_id,
            "status": task.status.value,
            "created_at": task.created_at.isoformat()
        }
    }

async def handle_get_task_status(context: DMSCServiceContext):
    """Get task status"""
    data = await context.http.request.json()
    task_id = data.get("task_id")
    
    if not task_id:
        return {"status": "error", "message": "task_id required"}, 400
    
    scheduler = context.task_scheduler
    task = scheduler.get_task_status(task_id)
    
    if task:
        return {
            "status": "success",
            "data": {
                "task_id": task.task_id,
                "device_id": task.device_id,
                "type": task.task_type,
                "status": task.status.value,
                "created_at": task.created_at.isoformat(),
                "result": task.result,
                "error": task.error
            }
        }
    else:
        return {"status": "error", "message": "Task not found"}, 404

async def handle_get_tasks(context: DMSCServiceContext):
    """Get all tasks"""
    scheduler = context.task_scheduler
    all_tasks = scheduler.get_all_tasks()
    
    return {
        "status": "success",
        "data": {
            "pending": len(all_tasks["pending"]),
            "running": len(all_tasks["running"]),
            "completed": len(all_tasks["completed"]),
            "tasks": {
                "pending": [{"id": t.task_id, "type": t.task_type} for t in all_tasks["pending"]],
                "running": [{"id": t.task_id, "device": t.device_id, "type": t.task_type} for t in all_tasks["running"]],
                "completed": [{"id": t.task_id, "type": t.task_type} for t in all_tasks["completed"][-10:]]
            }
        }
    }

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize device control module
    device_config = DMSCDeviceControlConfig(
        enable_discovery=True,
        discovery_interval=60,
        health_check_interval=30
    )
    device_module = DMSCDeviceControlModule(device_config)
    
    # Initialize managers
    device_manager = DeviceManager(device_module, dms_app.context)
    task_scheduler = TaskScheduler(device_manager, dms_app.context)
    
    # Store in context
    dms_app.context.device_manager = device_manager
    dms_app.context.task_scheduler = task_scheduler
    
    # Add routes
    dms_app.router.add_route("POST", "/devices/discover", handle_discover_devices)
    dms_app.router.add_route("GET", "/devices", handle_get_devices)
    dms_app.router.add_route("POST", "/devices/health", handle_get_device_health)
    dms_app.router.add_route("POST", "/tasks", handle_create_task)
    dms_app.router.add_route("POST", "/tasks/status", handle_get_task_status)
    dms_app.router.add_route("GET", "/tasks", handle_get_tasks)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Device Management Architecture

1. **Device Discovery**: Automatic network discovery of IoT devices
2. **Device Pool**: Resource pooling for efficient device utilization
3. **Task Scheduling**: Intelligent task distribution based on device type and availability
4. **Health Monitoring**: Continuous device health tracking

### Key Components

- **DMSCDeviceControlModule**: Main interface for device operations
- **DMSCDevice**: Device representation with type, status, and capabilities
- **Device Pool**: Efficient resource allocation and release
- **Task Scheduler**: Priority-based task distribution

## Running Steps

1. Save the code to `device_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python device_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Discover devices
   curl -X POST http://localhost:8080/devices/discover
   
   # Get all devices
   curl http://localhost:8080/devices
   
   # Get device health
   curl -X POST http://localhost:8080/devices/health \
     -H "Content-Type: application/json" \
     -d '{"device_id": "sensor-001"}'
   
   # Create a task
   curl -X POST http://localhost:8080/tasks \
     -H "Content-Type: application/json" \
     -d '{"task_type": "data_collection", "data": {"sensor_id": "sensor-001"}, "priority": "high"}'
   
   # Get task status
   curl -X POST http://localhost:8080/tasks/status \
     -H "Content-Type: application/json" \
     -d '{"task_id": "task_1705313400.123"}'
   
   # Get all tasks
   curl http://localhost:8080/tasks
   ```

## Expected Output

### Discover Devices Response

```json
{
  "status": "success",
  "data": {
    "discovered": 5,
    "total_registered": 5,
    "devices": [
      {"id": "sensor-001", "type": "sensor"},
      {"id": "camera-001", "type": "camera"},
      {"id": "actuator-001", "type": "actuator"}
    ]
  }
}
```

### Create Task Response

```json
{
  "status": "success",
  "data": {
    "task_id": "task_1705313400.123",
    "status": "running",
    "created_at": "2024-01-15T10:30:00"
  }
}
```

## Best Practices

1. **Regular Health Checks**: Implement regular health checks for all devices
2. **Device Grouping**: Group devices by location, type, or capability
3. **Failover Planning**: Plan failover for critical devices
4. **Capacity Planning**: Monitor device utilization for capacity planning
5. **Secure Communication**: Use secure communication with devices
6. **Implement Timeouts**: Always implement timeouts for device operations
7. **Monitor Discovery**: Monitor device discovery for new devices
8. **Handle Disconnections**: Handle device disconnections gracefully
9. **Logging**: Log all device management operations
10. **Document Device Types**: Document supported device types and capabilities
