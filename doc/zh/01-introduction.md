<div align="center">

# DMSC 介绍

**Version: 0.1.7**

**Last modified date: 2026-02-17**

## 项目概述

**DMSC (Dunimd Middleware Service)** 是一个高性能的 Rust 中间件框架，专为统一后端基础设施而设计。它采用模块化架构，提供了企业级规模所需的各种功能，包括内置可观测性和分布式系统支持。

## 核心特性

</div>

#### 🔍 分布式追踪
- W3C 追踪上下文标准实现
- 全链路 TraceID/SpanID 传播
- 业务上下文的数据传输
- 多语言兼容性（Java、Go、Python）

#### 📊 企业级可观测性
- 原生 Prometheus 指标导出
- Counter、Gauge、Histogram、Summary 指标类型
- 开箱即用的 Grafana 仪表板集成
- 实时性能统计与分位数计算
- 全栈指标（CPU、内存、I/O、网络）

#### 🤖 智能设备管理
- 自动发现和注册
- 高效的资源池管理
- 基于策略的调度与优先级支持
- 动态负载均衡
- 完整的设备生命周期管理

#### 📝 结构化日志
- 支持 JSON 和文本格式
- 可配置的采样率
- 智能日志轮转
- 自动包含追踪上下文
- DEBUG/INFO/WARN/ERROR 日志级别

#### ⚙️ 灵活配置
- 多源加载（文件、环境变量、运行时）
- 热配置更新
- 模块化架构支持按需组合
- 基于插件的扩展机制

#### 📁 安全文件系统
- 统一项目根目录管理
- 原子文件操作
- 分类目录结构
- JSON 数据持久化
- 安全路径处理

<div align="center">

## 技术栈

| 技术 | 用途 |
|------|------|
| Rust | 主要开发语言，提供高性能和内存安全 |
| Tokio | 异步运行时，支持高并发 |
| Serde | 序列化/反序列化库 |
| Prometheus | 指标收集和监控 |
| W3C Trace Context | 分布式追踪标准 |
| YAML/TOML/JSON | 配置文件格式支持 |

## 模块化设计

DMSC 采用高度模块化的架构，拥有 18 个核心模块，支持按需组合和无缝扩展：

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **database** | 数据库抽象层（ORM、连接池、事务） |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **grpc** | gRPC 服务支持（客户端、服务端、拦截器） |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **module_rpc** | 模块间 RPC 通信（服务注册、调用、负载均衡） |
| **observability** | 指标、追踪和 Grafana 集成 |
| **protocol** | 协议支持（国密 SM2/SM3/SM4、后量子密码） |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |
| **validation** | 数据验证（字段校验、规则引擎） |
| **ws** | WebSocket 支持（客户端、服务端） |

### 🐍 Python 模块支持

</div>

DMSC 提供完整的 Python 绑定，支持通过 Python 使用所有核心功能：

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **database** | 数据库抽象层（ORM、连接池、事务） |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **grpc** | gRPC 服务支持（客户端、服务端、拦截器） |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **module_rpc** | 模块间 RPC 通信（服务注册、调用、负载均衡） |
| **observability** | 指标、追踪和 Grafana 集成 |
| **protocol** | 协议支持（国密 SM2/SM3/SM4、后量子密码） |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |
| **validation** | 数据验证（字段校验、规则引擎） |
| **ws** | WebSocket 支持（客户端、服务端） |

**Python SDK 特性：**
- 原生 Python 接口，无缝调用 Rust 核心
- 支持同步和异步服务模块
- 版本：**0.1.7**（需 Python 3.8+）
- 通过 [PyPI](https://pypi.org/project/dmsc/) 分发

**快速开始：**
```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

# 构建服务运行时
app = DMSCAppBuilder() \
    .with_config("config.yaml") \
    .with_logging(DMSCLogConfig()) \
    .build()

# 运行业务逻辑
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

更多 Python 使用示例，请查看 [Python README](https://github.com/mf2023/DMSC/blob/master/python/README.zh.md)。

<div align="center">

## 应用场景

</div>

DMSC 适用于各种企业级后端应用场景，包括：

- **微服务架构**：作为服务间通信和协调的中间件
- **API 网关**：提供统一的 API 入口，支持限流、熔断等功能
- **分布式系统**：简化分布式系统的开发和管理
- **实时数据处理**：支持高并发数据处理和流处理
- **企业级应用**：提供可靠的基础设施支持


<div align="center">

## 社区与支持

</div>

- **GitHub/Gitee**：[https://github.com/mf2023/DMSC](https://github.com/mf2023/DMSC)
- **Issues**：提交问题和建议
- **Pull Requests**：欢迎贡献代码

<div align="center">

## 📄 许可证与开源协议

### 🏛️ 项目许可证

<p align="center">
  <a href="../LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

本项目使用 **Apache License 2.0** 开源协议，详见 [LICENSE](../LICENSE) 文件。

### 📋 依赖包开源协议

本项目使用的开源包及其协议信息：

### 依赖许可证

<div align="center">

| 📦 包 | 📜 许可证 |
|:-----------|:-----------|
| serde | Apache 2.0 |
| serde_json | MIT |
| serde_yaml | MIT |
| tokio | MIT |
| prometheus | Apache 2.0 |
| redis | MIT |
| hyper | MIT |
| lapin | Apache 2.0 |
| futures | MIT |
| yaml-rust | MIT |
| toml | MIT |
| etcd-client | MIT |
| sysinfo | MIT |
| async-trait | MIT |
| dashmap | MIT |
| chrono | MIT |
| uuid | Apache 2.0 |
| rand | MIT |
| notify | MIT |
| jsonwebtoken | MIT |
| reqwest | MIT |
| urlencoding | MIT |
| parking_lot | MIT |
| log | MIT |
| pyo3 | Apache 2.0 |
| tempfile | MIT |

</div>

### 📋 Python 依赖许可证

| 📦 包 | 📜 许可证 |
|:-----------|:-----------|
| setuptools | MIT |
| setuptools-rust | MIT |
| wheel | MIT |
| pytest | MIT |
| pytest-asyncio | Apache 2.0 |
| black | MIT |
| isort | MIT |
| mypy | MIT |
| pyo3 | Apache 2.0 |

## 下一步

</div>

- [快速开始](./02-getting-started.md)：安装和创建第一个 DMSC 应用
- [核心概念](./03-core-concepts.md)：深入理解 DMSC 的设计理念和核心组件
- [API 参考](./04-api-reference/README.md)：详细的模块 API 文档
- [使用示例](./05-usage-examples/README.md)：各种场景下的使用示例
- [最佳实践](./06-best-practices.md)：开发 DMSC 应用的最佳实践
- [故障排除](./07-troubleshooting.md)：常见问题和解决方案
- [术语表](./08-glossary.md)：核心术语解释
