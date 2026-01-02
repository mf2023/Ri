<div align="center">

# Device Control 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC Device Control 模块的使用示例。

</div>

## 基础设备操作

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceControlConfig, DMSCDeviceType

async def basic_device_example():
    """基础设备控制示例"""
    config = DMSCDeviceControlConfig(
        max_devices=100,
        discovery_interval=60,
        enable_auto_discovery=True
    )
    
    module = DMSCDeviceControlModule(config)
    
    # 发现可用设备
    devices = await module.discover_devices()
    print(f"发现 {len(devices)} 个设备")
    
    # 获取所有已注册设备
    all_devices = await module.get_all_devices()
    
    for device in all_devices:
        print(f"设备: {device.device_id}, 类型: {device.device_type}")
```

## 设备池管理

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceType

async def device_pool_example():
    """设备池管理示例"""
    module = DMSCDeviceControlModule()
    
    # 获取设备池
    pool = await module.get_device_pool()
    
    # 获取设备
    devices = await pool.acquire(DMSCDeviceType.SENSOR, count=2)
    
    for device in devices:
        print(f"获取设备: {device.device_id}")
        # 使用设备进行操作
        await device.connect()
    
    # 释放设备回池
    await pool.release(devices)
```

## 设备调度

```python
from dmsc import DMSCDeviceControlModule

async def device_scheduling_example():
    """设备调度示例"""
    module = DMSCDeviceControlModule()
    
    scheduler = await module.get_scheduler()
    
    # 调度任务
    task = {
        "type": "data_collection",
        "priority": "high",
        "data": {"sensor_id": "sensor-001"}
    }
    
    device = await scheduler.schedule(task)
    print(f"任务分配给设备: {device.device_id}")
    
    # 批量调度任务
    tasks = [
        {"type": "data_collection", "data": {"sensor_id": "sensor-001"}},
        {"type": "data_collection", "data": {"sensor_id": "sensor-002"}},
        {"type": "data_collection", "data": {"sensor_id": "sensor-003"}},
    ]
    
    devices = await scheduler.schedule_batch(tasks)
    print(f"批量任务分配给 {len(devices)} 个设备")
```

## 完整示例

```python
from dmsc import DMSCDeviceControlModule, DMSCDeviceControlConfig

async def complete_device_example():
    """完整设备控制示例"""
    # 初始化模块
    config = DMSCDeviceControlConfig(
        max_devices=50,
        discovery_interval=30,
        enable_auto_discovery=True
    )
    
    module = DMSCDeviceControlModule(config)
    
    # 发现并注册设备
    discovered = await module.discover_devices()
    
    # 获取设备池进行资源管理
    pool = await module.get_device_pool()
    
    # 获取调度器进行智能任务分配
    scheduler = await module.get_scheduler()
    
    # 调度任务
    task = {"command": "collect_data", "interval": 60}
    device = await scheduler.schedule(task)
    
    # 使用设备
    await device.connect()
    response = await device.send_command("status")
    print(f"设备状态: {response}")
    
    # 清理
    await device.disconnect()
    await module.shutdown()
```
