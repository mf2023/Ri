<div align="center">

# Gateway API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

gateway模块提供API网关功能，包括路由、负载均衡、限流和熔断能力。

</div>

## 模块概述

gateway模块包含以下核心组件：

- **DMSCGateway**: API网关主接口
- **DMSCGatewayConfig**: 网关配置
- **DMSCRoute**: 路由定义
- **DMSCRouter**: 路由管理器
- **DMSCGatewayRequest**: 网关请求
- **DMSCGatewayResponse**: 网关响应

<div align="center">

## 核心组件

</div>

### DMSCGateway

API网关主接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start()` | 启动网关 | 无 | `None` |
| `stop()` | 停止网关 | 无 | `None` |
| `add_route(route)` | 添加路由 | `route: DMSCRoute` | `None` |
| `remove_route(path)` | 移除路由 | `path: str` | `None` |
| `get_statistics()` | 获取统计信息 | 无 | `dict` |

#### 使用示例

```python
from dmsc import DMSCGateway, DMSCGatewayConfig, DMSCRoute

config = DMSCGatewayConfig(
    host="0.0.0.0",
    port=8080,
    workers=4
)
gateway = DMSCGateway(config)

# 添加路由
route = DMSCRoute(
    path="/api/users",
    backend="user-service",
    methods=["GET", "POST"]
)
await gateway.add_route(route)

# 启动网关
await gateway.start()

# 获取统计
stats = await gateway.get_statistics()
print(f"总请求数: {stats['total_requests']}")

# 停止网关
await gateway.stop()
```

### DMSCGatewayConfig

网关配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `host` | `str` | 网关监听地址 | `"0.0.0.0"` |
| `port` | `int` | 网关监听端口 | `8080` |
| `workers` | `int` | 工作进程数 | `4` |
| `max_connections` | `int` | 最大连接数 | `10000` |
| `request_timeout` | `int` | 请求超时（秒） | `30` |
| `enable_rate_limit` | `bool` | 是否启用限流 | `true` |
| `rate_limit_requests` | `int` | 窗口期最大请求数 | `1000` |
| `rate_limit_window` | `int` | 限流窗口（秒） | `60` |

#### 使用示例

```python
from dmsc import DMSCGatewayConfig

config = DMSCGatewayConfig(
    host="0.0.0.0",
    port=8080,
    workers=4,
    max_connections=10000,
    request_timeout=30,
    enable_rate_limit=True,
    rate_limit_requests=1000,
    rate_limit_window=60
)
```

### DMSCRoute

路由定义结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `path` | `str` | 路由路径模式 |
| `backend` | `str` | 后端服务名称 |
| `methods` | `List[str]` | 允许的HTTP方法 |
| `middleware` | `List[str]` | 中间件列表 |
| `timeout` | `int` | 请求超时时间 |

#### 使用示例

```python
from dmsc import DMSCRoute

route = DMSCRoute(
    path="/api/users",
    backend="user-service",
    methods=["GET", "POST"],
    middleware=["auth", "logging"],
    timeout=30
)
```

### DMSCRouter

路由管理器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_route(route)` | 添加路由 | `route: DMSCRoute` | `None` |
| `remove_route(path)` | 移除路由 | `path: str` | `bool` |
| `get_route(path)` | 获取路由 | `path: str` | `DMSCRoute` |
| `list_routes()` | 列出所有路由 | 无 | `List[DMSCRoute]` |

#### 使用示例

```python
from dmsc import DMSCRouter, DMSCRoute

router = DMSCRouter()

# 添加路由
route = DMSCRoute(path="/api/orders", backend="order-service")
router.add_route(route)

# 获取路由
route = router.get_route("/api/orders")

# 列出所有路由
all_routes = router.list_routes()
print(f"总路由数: {len(all_routes)}")

# 移除路由
router.remove_route("/api/orders")
```

### DMSCGatewayRequest

网关请求结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `method` | `str` | HTTP方法 |
| `path` | `str` | 请求路径 |
| `headers` | `Dict` | 请求头 |
| `body` | `bytes` | 请求体 |
| `query_params` | `Dict` | 查询参数 |
| `client_ip` | `str` | 客户端IP |

### DMSCGatewayResponse

网关响应结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `status_code` | `int` | 状态码 |
| `headers` | `Dict` | 响应头 |
| `body` | `bytes` | 响应体 |
| `latency_ms` | `int` | 延迟（毫秒） |

<div align="center">

## 完整使用示例

</div>

```python
from dmsc import (
    DMSCGateway,
    DMSCGatewayConfig,
    DMSCRoute,
    DMSCRouter,
    DMSCGatewayRequest,
    DMSCGatewayResponse
)

async def gateway_complete_example():
    """网关完整示例"""
    
    # 创建配置
    config = DMSCGatewayConfig(
        host="0.0.0.0",
        port=8080,
        workers=4,
        max_connections=10000,
        request_timeout=30,
        enable_rate_limit=True,
        rate_limit_requests=1000,
        rate_limit_window=60
    )
    
    # 创建网关
    gateway = DMSCGateway(config)
    
    # 使用路由管理器
    router = DMSCRouter()
    
    # 定义路由
    user_route = DMSCRoute(
        path="/api/users",
        backend="user-service",
        methods=["GET", "POST", "PUT", "DELETE"]
    )
    
    order_route = DMSCRoute(
        path="/api/orders",
        backend="order-service",
        methods=["GET", "POST"]
    )
    
    product_route = DMSCRoute(
        path="/api/products",
        backend="product-service",
        methods=["GET"]
    )
    
    # 添加路由
    router.add_route(user_route)
    router.add_route(order_route)
    router.add_route(product_route)
    
    # 将路由添加到网关
    await gateway.add_route(user_route)
    await gateway.add_route(order_route)
    await gateway.add_route(product_route)
    
    # 列出所有路由
    all_routes = router.list_routes()
    print(f"已添加 {len(all_routes)} 条路由:")
    for route in all_routes:
        print(f"  - {route.path} -> {route.backend}")
    
    # 创建请求
    request = DMSCGatewayRequest(
        method="GET",
        path="/api/users",
        headers={"Authorization": "Bearer token"},
        body=b"",
        query_params={"page": "1"},
        client_ip="192.168.1.100"
    )
    
    # 创建响应
    response = DMSCGatewayResponse(
        status_code=200,
        headers={"Content-Type": "application/json"},
        body=b'{"users": []}',
        latency_ms=50
    )
    
    # 启动网关
    print("启动网关...")
    await gateway.start()
    
    # 获取统计信息
    stats = await gateway.get_statistics()
    print(f"网关统计: {stats}")
    
    # 停止网关
    print("停止网关...")
    await gateway.stop()
    
    return {"status": "gateway_example_completed"}
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [service_mesh](./service_mesh.md): 服务网格模块，提供服务发现
- [protocol](./protocol.md): 协议模块，提供通信协议
- [cache](./cache.md): 缓存模块，提供请求缓存
- [auth](./auth.md): 认证模块，提供认证中间件
