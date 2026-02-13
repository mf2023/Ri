<div align="center">

# Module RPC API参考

**Version: 0.1.7**

**Last modified date: 2026-02-13**

module_rpc模块提供模块间RPC（远程过程调用）通信能力，支持同步和异步方法调用。

## 模块概述

</div>

module_rpc模块包含以下核心组件：

- **DMSCModuleRPC**: RPC协调器，管理端点和方法调用
- **DMSCModuleClient**: RPC客户端，用于调用其他模块的方法
- **DMSCModuleEndpoint**: 模块端点定义，暴露模块方法
- **DMSCMethodCall**: RPC方法调用请求
- **DMSCMethodResponse**: RPC方法调用响应

<div align="center">

## 核心组件

</div>

### DMSCModuleRPC

RPC协调器，负责管理所有注册的模块端点和路由方法调用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建RPC协调器 | 无 | `Self` |
| `with_default_timeout(timeout)` | 设置默认超时 | `timeout: Duration` | `Self` |
| `register_endpoint(endpoint)` | 注册模块端点 | `endpoint: DMSCModuleEndpoint` | `()` |
| `unregister_endpoint(module_name)` | 注销模块端点 | `module_name: &str` | `()` |
| `get_endpoint(module_name)` | 获取模块端点 | `module_name: &str` | `Option<Arc<DMSCModuleEndpoint>>` |
| `call_method(module_name, method_name, params, timeout_ms)` | 调用方法 | `module_name: &str`, `method_name: &str`, `params: Vec<u8>`, `timeout_ms: Option<u64>` | `DMSCMethodResponse` |
| `list_registered_modules()` | 列出已注册模块 | 无 | `Vec<String>` |

#### 使用示例

```rust
use dmsc::prelude::*;
use std::sync::Arc;

async fn example() -> DMSCResult<()> {
    // 创建RPC协调器
    let rpc = DMSCModuleRPC::new();

    // 创建并注册模块端点
    let endpoint = DMSCModuleEndpoint::new("user_service");
    endpoint.register_method("get_user", |_params| {
        Ok(vec![b"user_data"])
    });

    rpc.register_endpoint(endpoint).await;

    // 调用方法
    let response = rpc.call_method(
        "user_service",
        "get_user",
        vec![],
        None
    ).await;

    if response.is_success() {
        println!("Response: {:?}", response.data);
    }

    Ok(())
}
```

### DMSCModuleClient

RPC客户端，提供便捷的方法调用接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(rpc)` | 创建RPC客户端 | `rpc: Arc<DMSCModuleRPC>` | `Self` |
| `call(module_name, method_name, params)` | 调用方法（使用默认超时） | `module_name: &str`, `method_name: &str`, `params: Vec<u8>` | `DMSCMethodResponse` |
| `call_with_timeout(module_name, method_name, params, timeout_ms)` | 调用方法（指定超时） | `module_name: &str`, `method_name: &str`, `params: Vec<u8>`, `timeout_ms: u64` | `DMSCMethodResponse` |

#### 使用示例

```rust
use dmsc::prelude::*;
use std::sync::Arc;

async fn client_example() -> DMSCResult<()> {
    let rpc = Arc::new(DMSCModuleRPC::new());
    
    // 创建客户端
    let client = DMSCModuleClient::new(rpc);

    // 调用方法
    let response = client.call(
        "user_service",
        "get_user",
        vec![]
    ).await;

    // 使用自定义超时调用
    let response = client.call_with_timeout(
        "user_service",
        "get_user",
        vec![],
        3000  // 3秒超时
    ).await;

    Ok(())
}
```

### DMSCModuleEndpoint

模块端点，定义模块暴露的方法。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(module_name)` | 创建模块端点 | `module_name: &str` | `Self` |
| `module_name()` | 获取模块名称 | 无 | `&str` |
| `register_method(name, handler)` | 注册同步方法 | `name: &str`, `handler: Fn(Vec<u8>) -> DMSCResult<Vec<u8>>` | `&Self` |
| `register_method_async(name, handler)` | 注册异步方法 | `name: &str`, `handler: Fn(Vec<u8>) -> DMSCResult<Vec<u8>>` | `&Self` |
| `get_method(name)` | 获取方法 | `name: &str` | `Option<DMSCMethodRegistration>` |
| `list_methods()` | 列出所有方法 | 无 | `Vec<String>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建端点
let endpoint = DMSCModuleEndpoint::new("order_service");

// 注册方法
endpoint
    .register_method("create_order", |params| {
        // 处理创建订单逻辑
        Ok(vec![b"order_created"])
    })
    .register_method("cancel_order", |params| {
        // 处理取消订单逻辑
        Ok(vec![b"order_cancelled"])
    });

// 列出所有方法
let methods = endpoint.list_methods().await;
println!("Available methods: {:?}", methods);
```

### DMSCMethodCall

RPC方法调用请求结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `method_name` | `String` | 方法名称 |
| `params` | `Vec<u8>` | 方法参数（序列化后的字节） |
| `timeout_ms` | `u64` | 超时时间（毫秒），默认5000 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(method_name, params)` | 创建方法调用 | `method_name: String`, `params: Vec<u8>` | `Self` |
| `with_timeout_ms(timeout_ms)` | 设置超时时间 | `timeout_ms: u64` | `Self` |

### DMSCMethodResponse

RPC方法调用响应结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `success` | `bool` | 调用是否成功 |
| `data` | `Vec<u8>` | 返回数据（序列化后的字节） |
| `error` | `String` | 错误信息 |
| `is_timeout` | `bool` | 是否超时 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建空响应 | 无 | `Self` |
| `success_data(data)` | 创建成功响应 | `data: Vec<u8>` | `Self` |
| `error_msg(msg)` | 创建错误响应 | `msg: String` | `Self` |
| `timeout()` | 创建超时响应 | 无 | `Self` |
| `is_success()` | 检查是否成功 | 无 | `bool` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建成功响应
let response = DMSCMethodResponse::success_data(vec![1, 2, 3]);
assert!(response.is_success());

// 创建错误响应
let response = DMSCMethodResponse::error_msg("Invalid parameter".to_string());
assert!(!response.is_success());
assert_eq!(response.error, "Invalid parameter");

// 创建超时响应
let response = DMSCMethodResponse::timeout();
assert!(response.is_timeout);
```

<div align="center">

## 设计原则

</div>

1. **类型安全**: 所有RPC调用都是类型安全的，支持正确的序列化
2. **异步支持**: 同时支持同步和异步RPC调用
3. **超时控制**: 所有RPC调用都支持可配置的超时
4. **错误处理**: 完善的错误处理，包含特定的错误类型
5. **线程安全**: 所有组件都使用Arc和RwLock保证线程安全
6. **模块隔离**: 每个模块都有独立的方法命名空间

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [queue](./queue.md): 消息队列模块，提供消息队列支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
