<div align="center">

# 使用示例

**Version: 0.1.6**

**Last modified date: 2026-02-01**

本目录包含DMSC各个核心功能的使用示例，帮助您快速理解和使用DMSC框架。

## 示例列表

</div>

| 示例 | 描述 | 文件 |
|:--------|:-------------|:--------|
| **基础应用** | 构建简单的DMSC应用 | [basic-app.md](./basic-app.md) |
| **认证与授权** | 使用JWT和OAuth进行认证 | [authentication.md](./authentication.md) |
| **缓存使用** | 缓存的基本操作和高级用法 | [caching.md](./caching.md) |
| **数据库操作** | 数据库连接、查询和事务管理 | [database.md](./database.md) |
| **设备管理** | 设备控制、发现和智能调度 | [device.md](./device.md) |
| **文件系统** | 安全的文件操作和管理 | [fs.md](./fs.md) |
| **网关** | API网关，支持HTTP服务、路由、中间件、负载均衡、限流和熔断 | [gateway.md](./gateway.md) |
| **gRPC服务** | 高性能 RPC，带服务注册和 Python 绑定 | [grpc.md](./grpc.md) |
| **生命周期钩子** | 启动和关闭的生命周期事件钩子 | [hooks.md](./hooks.md) |
| **消息队列** | 异步消息处理和事件驱动架构 | [queue.md](./queue.md) |
| **可观测性** | 分布式追踪、指标收集和监控 | [observability.md](./observability.md) |
| **协议** | 通信协议、加密和后量子密码学 | [protocol.md](./protocol.md) |
| **服务网格** | 服务发现、健康检查和流量管理 | [service_mesh.md](./service_mesh.md) |
| **数据验证** | 数据验证、清理和自定义验证器 | [validation.md](./validation.md) |
| **WebSocket** | 实时双向通信，带 Python 绑定 | [ws.md](./ws.md) |

<div align="center">

## 使用指南

</div>

每个示例文档包含以下内容：

1. **示例概述**：示例的目的和功能
2. **前置要求**：运行示例所需的环境和依赖
3. **示例代码**：完整的示例代码
4. **代码解析**：对示例代码的详细解释
5. **运行步骤**：如何运行示例
6. **预期结果**：运行示例后预期的输出

<div align="center">

## 示例结构

</div>

所有示例都遵循以下结构：

```rust
// 1. 导入必要的依赖
use dmsc::prelude::*;

// 2. 主函数
#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 3. 构建应用
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        // 其他配置
        .build()?;
    
    // 4. 运行应用
    app.run(|ctx| async move {
        // 5. 业务逻辑
        Ok(())
    }).await
}
```

<div align="center">

## 运行示例

</div>

### 1. 克隆DMSC仓库

```bash
git clone https://github.com/mf2023/DMSC.git
cd dmsc
```

### 2. 创建示例项目

```bash
cargo new dms-example
cd dms-example
```

### 3. 添加依赖

在`Cargo.toml`中添加：

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
```

### 4. 编写示例代码

将示例代码复制到`src/main.rs`文件中。

### 5. 运行示例

```bash
cargo run
```

<div align="center">

## 自定义示例

</div>

您可以根据需要修改示例代码，探索DMSC的各种功能：

1. **添加或移除模块**：根据需求调整应用配置
2. **修改业务逻辑**：实现自己的业务功能
3. **调整配置**：修改配置文件或运行时参数
4. **添加自定义模块**：扩展DMSC功能

## 最佳实践

1. **从简单示例开始**：先运行基础示例，再尝试复杂功能
2. **理解代码逻辑**：仔细阅读代码解析，理解每个组件的作用
3. **逐步扩展**：在基础示例上逐步添加新功能
4. **查看API文档**：遇到疑问时，参考对应的API文档
5. **测试和调试**：使用DMSC的可观测性功能进行测试和调试

<div align="center">

## 下一步

</div>

选择您感兴趣的示例，按照说明运行和修改，深入了解DMSC的功能和用法。

- [最佳实践](./06-best-practices.md)：开发 DMSC 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：常见问题和解决方案
- [术语表](./08-glossary.md)：核心术语解释
