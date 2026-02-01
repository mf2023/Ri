<div align="center">

# API参考

**Version: 0.1.6**

**Last modified date: 2026-01-30**

本目录包含DMSC各个核心模块的详细API文档。

## 模块列表

</div>

| 模块 | 描述 | 文件 |
|:--------|:-------------|:--------|
| **auth** | 认证与授权（JWT、OAuth、权限） | [auth.md](./auth.md) |
| **cache** | 多后端缓存抽象（内存、Redis、混合） | [cache.md](./cache.md) |
| **config** | 多源配置管理与热重载 | [config.md](./config.md) |
| **core** | 运行时、错误处理和服务上下文 | [core.md](./core.md) |
| **database** | 数据库访问层，带 ORM 支持 | [database.md](./database.md) |
| **device** | 设备控制、发现和智能调度 | [device.md](./device.md) |
| **fs** | 安全的文件系统操作和管理 | [fs.md](./fs.md) |
| **gateway** | API 网关，支持 HTTP 服务、路由、中间件、负载均衡、限流和熔断 | [gateway.md](./gateway.md) |
| **grpc** | 高性能 RPC，带服务注册和 Python 绑定 | [grpc.md](./grpc.md) |
| **hooks** | 生命周期事件钩子（启动、关闭等） | [hooks.md](./hooks.md) |
| **log** | 结构化日志与追踪上下文集成 | [log.md](./log.md) |
| **observability** | 指标、追踪和 Grafana 集成 | [observability.md](./observability.md) |
| **protocol** | 通信协议支持，包含加密、后量子密码和 HSM | [protocol.md](./protocol.md) |
| **queue** | 消息队列抽象（Kafka、RabbitMQ、Redis、内存） | [queue.md](./queue.md) |
| **service_mesh** | 服务发现、健康检查和流量管理 | [service_mesh.md](./service_mesh.md) |
| **validation** | 数据验证和清理 | [validation.md](./validation.md) |
| **ws** | WebSocket 实时通信，带 Python 绑定 | [ws.md](./ws.md) |

<div align="center">

## 使用指南

</div>

每个API文档包含以下内容：

1. **模块概述**：模块的主要功能和用途
2. **核心组件**：模块中的关键类型和结构体
3. **API 参考**：详细的方法和函数说明
4. **使用示例**：代码示例展示模块的使用方式
5. **最佳实践**：使用该模块的建议和最佳实践

<div align="center">

## 命名约定

</div>

DMSC API遵循以下命名约定：

- **类名**：采用PascalCase，如`DMSCAppBuilder`
- **方法名**：采用snake_case，如`with_config`
- **常量**：采用SCREAMING_SNAKE_CASE，如`DEFAULT_PORT`
- **类型别名**：采用PascalCase，如`DMSCResult`

<div align="center">

## 错误处理

</div>

所有DMSC API方法都返回`DMSCResult<T>`类型，其中`T`是方法的返回值类型。`DMSCResult`是`Result<T, DMSCError>`的别名。

```rust
type DMSCResult<T> = Result<T, DMSCError>;
```

<div align="center">

## 异步API

</div>

大多数DMSC API都是异步的，使用`async/await`语法。异步方法在调用时需要使用`.await`关键字。

```rust
// 异步方法调用
ctx.cache().set("key", "value", Some(3600)).await?;
```

<div align="center">

## 类型安全

</div>

DMSC API设计注重类型安全，使用强类型确保编译时检查。

```rust
// 类型安全的配置访问
let port: u16 = ctx.config().config().get("service.port").unwrap_or(8080);
```

<div align="center">

## 下一步

</div>

选择您感兴趣的模块，查看其详细的API文档。

- [使用示例](./05-usage-examples/)：各种场景下的使用示例
- [最佳实践](./06-best-practices.md)：开发 DMSC 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：解决常见问题和故障
- [术语解释](./08-glossary.md)：理解 DMSC 中使用的专业术语