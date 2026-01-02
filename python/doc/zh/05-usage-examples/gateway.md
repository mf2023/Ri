<div align="center">

# Gateway 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC Gateway 模块的使用示例。

</div>

## 基础网关设置

```python
from dmsc import DMSCGateway, DMSCGatewayConfig, DMSCRoute

async def basic_gateway_example():
    """基础网关设置示例"""
    config = DMSCGatewayConfig(
        host="0.0.0.0",
        port=8080,
        workers=4
    )
    
    gateway = DMSCGateway(config)
    
    # 添加路由
    routes = [
        DMSCRoute(path="/api/users", backend="user-service", methods=["GET", "POST"]),
        DMSCRoute(path="/api/orders", backend="order-service", methods=["GET"]),
        DMSCRoute(path="/api/products", backend="product-service", methods=["GET"]),
    ]
    
    for route in routes:
        await gateway.add_route(route)
    
    # 启动网关
    await gateway.start()
    print("网关已在端口 8080 启动")
    
    # 获取统计
    stats = await gateway.get_statistics()
    print(f"总请求数: {stats['total_requests']}")
```

## 路由管理

```python
from dmsc import DMSCGateway, DMSCRoute

async def route_management_example():
    """路由管理示例"""
    gateway = DMSCGateway()
    
    # 添加路由
    route = DMSCRoute(
        path="/api/v1/users",
        backend="user-service",
        methods=["GET", "POST", "PUT", "DELETE"],
        timeout=30
    )
    await gateway.add_route(route)
    
    # 列出所有路由
    routes = gateway.router.list_routes()
    for r in routes:
        print(f"路由: {r.path} -> {r.backend}")
    
    # 移除路由
    await gateway.remove_route("/api/v1/legacy")
```

## 熔断器使用

```python
from dmsc import DMSCCircuitBreaker

async def circuit_breaker_example():
    """熔断器示例"""
    circuit_breaker = DMSCCircuitBreaker()
    
    # 检查是否允许请求
    if await circuit_breaker.allow_request():
        try:
            # 发起请求
            result = await make_external_call()
            await circuit_breaker.record_success()
        except Exception as e:
            await circuit_breaker.record_failure()
    else:
        print("熔断器已打开，请求被拒绝")
    
    # 检查熔断器状态
    state = await circuit_breaker.get_state()
    print(f"熔断器状态: {state}")
```

## 负载均衡器使用

```python
from dmsc import DMSCLoadBalancer, DMSCLoadBalancerStrategy

async def load_balancer_example():
    """负载均衡器示例"""
    load_balancer = DMSCLoadBalancer()
    
    # 添加后端服务器
    servers = [
        "http://api-server-1:8080",
        "http://api-server-2:8080",
        "http://api-server-3:8080",
    ]
    
    for server in servers:
        await load_balancer.add_server(server, weight=100)
    
    # 使用轮询选择服务器
    server = await load_balancer.select_server(DMSCLoadBalancerStrategy.ROUND_ROBIN)
    print(f"选择的服务器: {server}")
    
    # 获取健康的服务器
    healthy = await load_balancer.get_healthy_servers()
    print(f"健康服务器: {healthy}")
```

## 完整示例

```python
from dmsc import DMSCGateway, DMSCGatewayConfig, DMSCRoute

async def complete_gateway_example():
    """完整网关示例"""
    # 配置网关
    config = DMSCGatewayConfig(
        host="0.0.0.0",
        port=8080,
        workers=8,
        enable_rate_limit=True,
        rate_limit_requests=1000,
        rate_limit_window=60
    )
    
    gateway = DMSCGateway(config)
    
    # 添加路由
    await gateway.add_route(DMSCRoute(
        path="/api/users/{id}",
        backend="user-service"
    ))
    await gateway.add_route(DMSCRoute(
        path="/api/orders",
        backend="order-service"
    ))
    
    # 启动网关
    await gateway.start()
    print("网关正在运行")
    
    # 监控统计
    stats = await gateway.get_statistics()
    print(f"请求数: {stats['total_requests']}")
    print(f"错误数: {stats['total_errors']}")
    
    # 关闭
    await gateway.stop()
```
