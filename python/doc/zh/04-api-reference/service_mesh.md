<div align="center">

# Service Mesh API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

service_mesh模块提供服务发现、健康检查、流量管理、负载均衡和熔断能力。

</div>

## 模块概述

service_mesh模块包含以下核心组件：

- **DMSCServiceMesh**: 服务网格主接口
- **DMSCServiceMeshConfig**: 服务网格配置
- **DMSCServiceDiscovery**: 服务发现组件
- **DMSCServiceInstance**: 服务实例
- **DMSCServiceStatus**: 服务状态枚举
- **DMSCServiceEndpoint**: 服务端点
- **DMSCServiceHealthStatus**: 服务健康状态枚举
- **DMSCHealthChecker**: 健康检查组件
- **DMSCHealthSummary**: 健康检查汇总
- **DMSCHealthStatus**: 健康状态枚举
- **DMSCTrafficManager**: 流量管理组件
- **DMSCTrafficRoute**: 流量路由
- **DMSCMatchCriteria**: 匹配条件
- **DMSCRouteAction**: 路由动作枚举
- **DMSCWeightedDestination**: 权重目标

<div align="center">

## 核心组件

</div>

### DMSCServiceMesh

服务网格主接口，用于管理分布式系统中的服务。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `register_service(service_name, endpoint, weight)` | 注册服务 | `service_name: str`, `endpoint: str`, `weight: int` | `None` |
| `discover_service(service_name)` | 发现健康服务实例 | `service_name: str` | `List[DMSCServiceInstance]` |
| `call_service(service_name, data)` | 调用服务（负载均衡） | `service_name: str`, `data: bytes` | `bytes` |
| `get_service_health(service_name)` | 获取服务健康状态 | `service_name: str` | `dict` |
| `shutdown()` | 关闭服务网格 | 无 | `None` |

#### 使用示例

```python
from dmsc import DMSCServiceMesh, DMSCServiceMeshConfig

config = DMSCServiceMeshConfig()
mesh = DMSCServiceMesh(config)

# 注册服务
await mesh.register_service("user-service", "http://user-service:8080", 100)
await mesh.register_service("order-service", "http://order-service:8080", 100)

# 发现服务
endpoints = await mesh.discover_service("user-service")
print(f"发现 {len(endpoints)} 个 user-service 端点")

# 调用服务
response = await mesh.call_service("user-service", b"get_users")
print(f"响应: {response}")

# 获取健康状态
health = await mesh.get_service_health("user-service")
print(f"服务健康状态: {health}")

await mesh.shutdown()
```

### DMSCServiceMeshConfig

服务网格配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `enable_service_discovery` | `bool` | 是否启用服务发现 | `true` |
| `enable_health_check` | `bool` | 是否启用健康检查 | `true` |
| `enable_traffic_management` | `bool` | 是否启用流量管理 | `true` |
| `health_check_interval` | `int` | 健康检查间隔（秒） | 30 |

#### 使用示例

```python
from dmsc import DMSCServiceMeshConfig

config = DMSCServiceMeshConfig(
    enable_service_discovery=True,
    enable_health_check=True,
    enable_traffic_management=True,
    health_check_interval=30
)
```

### DMSCServiceDiscovery

服务发现组件。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `discover_service(service_name)` | 发现服务实例 | `service_name: str` | `List[dict]` |
| `register_service(service_name, endpoint, weight)` | 注册服务实例 | `service_name: str`, `endpoint: str`, `weight: int` | `bool` |
| `unregister_service(service_name, endpoint)` | 注销服务实例 | `service_name: str`, `endpoint: str` | `bool` |
| `get_all_services()` | 获取所有服务名 | 无 | `List[str]` |

### DMSCServiceInstance

服务实例表示。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `service_name` | `str` | 服务名称 |
| `endpoint` | `str` | 服务端点URL |
| `weight` | `int` | 负载均衡权重 |
| `status` | `DMSCServiceStatus` | 服务状态 |
| `health_score` | `float` | 健康评分 |

### DMSCServiceStatus

服务状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `HEALTHY` | 健康 |
| `UNHEALTHY` | 不健康 |
| `UNKNOWN` | 未知 |
| `PENDING` | 待定 |

### DMSCServiceEndpoint

服务端点信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `url` | `str` | 端点URL |
| `port` | `int` | 端口号 |
| `protocol` | `str` | 协议类型 |
| `metadata` | `Dict` | 元数据 |

### DMSCServiceHealthStatus

服务健康状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `HEALTHY` | 健康 |
| `DEGRADED` | 降级 |
| `CRITICAL` | 严重 |
| `UNKNOWN` | 未知 |

### DMSCHealthChecker

健康检查组件。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_health_check(service_name, endpoint)` | 启动健康检查 | `service_name: str`, `endpoint: str` | `None` |
| `stop_health_check(service_name, endpoint)` | 停止健康检查 | `service_name: str`, `endpoint: str` | `None` |
| `get_health_status(service_name, endpoint)` | 获取健康状态 | `service_name: str`, `endpoint: str` | `dict` |
| `get_all_health_status()` | 获取所有服务健康状态 | 无 | `dict` |

### DMSCHealthSummary

健康检查汇总。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `total_services` | `int` | 服务总数 |
| `healthy_count` | `int` | 健康数量 |
| `unhealthy_count` | `int` | 不健康数量 |
| `last_check_time` | `int` | 最后检查时间 |

### DMSCHealthStatus

健康状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `HEALTHY` | 健康 |
| `UNHEALTHY` | 不健康 |
| `TIMEOUT` | 超时 |
| `ERROR` | 错误 |

### DMSCTrafficManager

流量管理组件。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_route(route)` | 添加流量路由 | `route: DMSCTrafficRoute` | `None` |
| `remove_route(route_id)` | 移除流量路由 | `route_id: str` | `bool` |
| `route_request(endpoint, data)` | 路由请求 | `endpoint: str`, `data: bytes` | `bytes` |
| `get_traffic_stats()` | 获取流量统计 | 无 | `dict` |

### DMSCTrafficRoute

流量路由配置。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `route_id` | `str` | 路由ID |
| `match_criteria` | `DMSCMatchCriteria` | 匹配条件 |
| `action` | `DMSCRouteAction` | 路由动作 |
| `destinations` | `List[DMSCWeightedDestination]` | 目标列表 |

### DMSCMatchCriteria

流量匹配条件。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `source_ip` | `str` | 源IP |
| `path_prefix` | `str` | 路径前缀 |
| `headers` | `Dict` | 请求头 |
| `methods` | `List[str]` | HTTP方法 |

### DMSCRouteAction

路由动作枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `ROUTE` | 路由到目标 |
| `REDIRECT` | 重定向 |
| `FAULT` | 故障注入 |
| `METRIC` | 收集指标 |

### DMSCWeightedDestination

权重目标配置。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `endpoint` | `str` | 目标端点 |
| `weight` | `int` | 权重值 |
| `labels` | `Dict` | 标签 |

<div align="center>

## 完整使用示例

</div>

```python
from dmsc import (
    DMSCServiceMesh,
    DMSCServiceMeshConfig,
    DMSCServiceDiscovery,
    DMSCHealthChecker,
    DMSCTrafficManager
)

async def service_mesh_complete_example():
    """服务网格完整示例"""
    
    # 初始化配置
    config = DMSCServiceMeshConfig(
        enable_service_discovery=True,
        enable_health_check=True,
        enable_traffic_management=True,
        health_check_interval=30
    )
    
    # 初始化服务网格
    mesh = DMSCServiceMesh(config)
    
    # 使用服务发现
    discovery = DMSCServiceDiscovery()
    await discovery.register_service("user-service", "http://user-service:8080", 100)
    await discovery.register_service("order-service", "http://order-service:8080", 100)
    
    services = await discovery.get_all_services()
    print(f"注册的服务: {services}")
    
    # 使用健康检查
    health_checker = DMSCHealthChecker()
    await health_checker.start_health_check("user-service", "http://user-service:8080")
    
    all_health = await health_checker.get_all_health_status()
    print(f"所有服务健康状态: {all_health}")
    
    # 使用流量管理
    traffic_manager = DMSCTrafficManager()
    
    # 发现并调用服务
    endpoints = await mesh.discover_service("user-service")
    print(f"发现 {len(endpoints)} 个 user-service 端点")
    
    response = await mesh.call_service("user-service", b"get_users")
    print(f"响应: {response}")
    
    # 获取服务健康状态
    health = await mesh.get_service_health("user-service")
    print(f"服务健康状态: {health}")
    
    # 关闭
    await health_checker.stop_health_check("user-service", "http://user-service:8080")
    await mesh.shutdown()
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [device](./device.md): 设备模块，提供设备服务
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [cache](./cache.md): 缓存模块，提供服务发现缓存
