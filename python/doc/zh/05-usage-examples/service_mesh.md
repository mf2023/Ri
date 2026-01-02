<div align="center">

# Service Mesh 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC Service Mesh 模块的使用示例。

</div>

## 基础服务网格操作

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

async def basic_service_mesh_example():
    """基础服务网格示例"""
    config = DMSCServiceMeshConfig(
        enable_service_discovery=True,
        enable_health_check=True,
        enable_traffic_management=True
    )
    
    mesh = DMSCServiceMesh(config)
    
    # 注册服务
    await mesh.register_service("user-service", "http://user-service:8080", 100)
    await mesh.register_service("order-service", "http://order-service:8080", 100)
    await mesh.register_service("payment-service", "http://payment-service:8080", 100)
    
    # 发现服务
    endpoints = await mesh.discover_service("user-service")
    print(f"找到 {len(endpoints)} 个 user-service 端点")
    
    for ep in endpoints:
        print(f"  - {ep['endpoint']} (权重: {ep['weight']})")
```

## 服务发现

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

async def service_discovery_example():
    """服务发现示例"""
    config = DMSCServiceMeshConfig()
    mesh = DMSCServiceMesh(config)
    
    # 注册同一服务的多个实例
    await mesh.register_service("api-gateway", "http://api-1:8080", 100)
    await mesh.register_service("api-gateway", "http://api-2:8080", 100)
    await mesh.register_service("api-gateway", "http://api-3:8080", 100)
    
    # 发现健康的实例
    endpoints = await mesh.discover_service("api-gateway")
    print(f"健康实例数: {len(endpoints)}")
    
    # 获取所有注册的服务
    services = await mesh.get_all_services()
    print(f"注册的服务: {services}")
```

## 健康检查

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

async def health_check_example():
    """健康检查示例"""
    config = DMSCServiceMeshConfig(
        enable_health_check=True,
        health_check_interval=30
    )
    
    mesh = DMSCServiceMesh(config)
    
    # 注册服务
    await mesh.register_service("database", "http://db:5432", 100)
    
    # 获取健康状态
    health = await mesh.get_service_health("database")
    print(f"数据库健康状态: {health}")
    
    # 获取所有健康状态
    all_health = await mesh.get_all_health_status()
    for service, status in all_health.items():
        print(f"  {service}: {status}")
```

## 负载均衡服务调用

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

async def service_call_example():
    """负载均衡服务调用示例"""
    config = DMSCServiceMeshConfig()
    mesh = DMSCServiceMesh(config)
    
    # 注册不同权重的服务
    await mesh.register_service("compute", "http://compute-1:8080", 100)
    await mesh.register_service("compute", "http://compute-2:8080", 50)
    await mesh.register_service("compute", "http://compute-3:8080", 50)
    
    # 调用服务 - 自动负载均衡
    for i in range(10):
        response = await mesh.call_service("compute", f"request-{i}".encode())
        print(f"响应 {i}: {response}")
```

## 完整示例

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

class MicroservicesPlatform:
    def __init__(self):
        config = DMSCServiceMeshConfig(
            enable_service_discovery=True,
            enable_health_check=True,
            enable_traffic_management=True,
            health_check_interval=30
        )
        self.mesh = DMSCServiceMesh(config)
    
    async def initialize(self):
        """初始化平台和服务"""
        print("正在初始化微服务平台...")
        
        # 核心服务
        await self.mesh.register_service(
            "user-service",
            "http://user-service:8080",
            100
        )
        
        await self.mesh.register_service(
            "order-service",
            "http://order-service:8080",
            100
        )
        
        await self.mesh.register_service(
            "payment-service",
            "http://payment-service:8080",
            100
        )
        
        # 高可用服务
        await self.mesh.register_service(
            "api-gateway",
            "http://api-gateway-1:8080",
            100
        )
        await self.mesh.register_service(
            "api-gateway",
            "http://api-gateway-2:8080",
            100
        )
        
        print("服务注册成功")
    
    async def process_order(self, user_id: str, order_data: dict) -> dict:
        """通过服务网格处理订单"""
        print(f"正在处理用户 {user_id} 的订单")
        
        # 发现并调用服务
        user_response = await self.mesh.call_service(
            "user-service",
            str(user_id).encode()
        )
        print("用户服务已调用")
        
        order_response = await self.mesh.call_service(
            "order-service",
            str(order_data).encode()
        )
        print("订单服务已调用")
        
        payment_response = await self.mesh.call_service(
            "payment-service",
            str(order_data).encode()
        )
        print("支付服务已调用")
        
        return {
            "user": user_response,
            "order": order_response,
            "payment": payment_response
        }
    
    async def get_platform_health(self) -> dict:
        """获取所有服务的健康状态"""
        services = [
            "user-service",
            "order-service",
            "payment-service",
            "api-gateway"
        ]
        
        health_status = {}
        for service in services:
            try:
                health = await self.mesh.get_service_health(service)
                health_status[service] = health
            except Exception as e:
                health_status[service] = f"错误: {e}"
        
        return health_status
    
    async def shutdown(self):
        """关闭平台"""
        print("正在关闭平台...")
        await self.mesh.shutdown()
        print("平台已关闭")

# 运行示例
async def main():
    platform = MicroservicesPlatform()
    
    await platform.initialize()
    
    # 处理订单
    result = await platform.process_order(
        "user-123",
        {"items": [{"id": 1, "quantity": 2}], "total": 99.99}
    )
    print(f"订单已处理: {result}")
    
    # 检查健康状态
    health = await platform.get_platform_health()
    print(f"平台健康状态: {health}")
    
    await platform.shutdown()

import asyncio
asyncio.run(main())
```
