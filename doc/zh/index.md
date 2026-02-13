<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../../assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">帮助文档</span>
</h1>

[English](../en/index.md) | 简体中文

**Version: 0.1.7**

**最后更新日期: 2026-02-13**

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>

<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://github.com/mf2023/DMSC" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMSC-181717?style=flat-square&logo=github"/>
</a>
<a href="https://huggingface.co/dunimd" target="_blank">
    <img alt="Hugging Face" src="https://img.shields.io/badge/Hugging%20Face-Dunimd-FFD21E?style=flat-square&logo=huggingface"/>
</a>
<a href="https://modelscope.cn/organization/dunimd" target="_blank">
    <img alt="ModelScope" src="https://img.shields.io/badge/ModelScope-Dunimd-1E6CFF?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTcuMDA2IDBDMy4xNDIgMCAwIDMuMTQyIDAgNy4wMDZTMy4xNDIgMTQuMDEyIDcuMDA2IDE0LjAxMkMxMC44NyAxNC4wMTIgMTQuMDEyIDEwLjg3IDE0LjAxMiA3LjAwNkMxNC4wMTIgMy4xNDIgMTAuODcgMCA3LjAwNiAwWiIgZmlsbD0iIzFFNkNGRiIvPgo8L3N2Zz4K"/>
</a>


<a href="https://crates.io/crates/dmsc" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMSC-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>

欢迎使用 DMSC (Dunimd Middleware Service) 帮助文档！本文档将帮助您理解和使用 DMSC 框架，构建高性能、可靠、安全的后端应用。

</div>

<div align="center">

## 文档导航

</div>

### 1. 入门指南

- [**介绍**](./01-introduction.md) - 了解 DMSC 的核心特性、模块化设计和应用场景
- [**快速开始**](./02-getting-started.md) - 从安装到运行第一个 DMSC 应用的完整指南

### 2. 核心概念

- [**核心概念**](./03-core-concepts.md) - 深入理解 DMSC 的设计理念、服务上下文、模块系统和生命周期管理

### 3. API 参考

- [**API 参考**](./04-api-reference/README.md) - 详细的模块 API 文档，包括 core、auth、cache、config 等模块

### 4. 使用示例

- [**使用示例**](./05-usage-examples/README.md) - 各种功能的使用示例，包括基础应用、认证与授权、缓存使用、可观测性等

### 5. 最佳实践

- [**最佳实践**](./06-best-practices.md) - 构建高效、可靠、安全的 DMSC 应用的最佳实践

### 6. 故障排除

- [**故障排除**](./07-troubleshooting.md) - 常见问题和解决方案，帮助您快速定位和解决问题

### 7. 术语表

- [**术语表**](./08-glossary.md) - DMSC 文档中使用的技术术语和概念定义

<div align="center">

## 什么是 DMSC？

</div>

**DMSC (Dunimd Middleware Service)** — 一个高性能的 Rust 中间件框架，统一后端基础设施。专为企业级规模构建，具有模块化架构、内置可观测性和分布式系统支持。

### 核心特性

- **分布式追踪**：W3C 追踪上下文标准实现，全链路 TraceID/SpanID 传播
- **企业级可观测性**：原生 Prometheus 指标导出，开箱即用的 Grafana 集成
- **智能设备管理**：自动发现和注册，高效的资源池管理
- **结构化日志**：支持 JSON 和文本格式，自动包含追踪上下文
- **灵活配置**：多源加载，热配置更新
- **安全文件系统**：统一项目根目录管理，原子文件操作

### 模块化设计

DMSC 采用高度模块化的架构，拥有 18 个核心模块，支持按需组合和无缝扩展：

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **database** | 数据库抽象层，支持 PostgreSQL、MySQL、SQLite |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **observability** | 指标、追踪和 Grafana 集成 |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |
| **validation** | 数据验证和清理工具 |
| **protocol** | 协议抽象层，支持多协议 |
| **module_rpc** | 模块间 RPC 通信，支持分布式方法调用 |

### 🐍 Python 模块支持

DMSC 提供完整的 Python 绑定，支持通过 Python 使用所有核心功能：

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **database** | 数据库抽象层，支持 PostgreSQL、MySQL、SQLite |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **observability** | 指标、追踪和 Grafana 集成 |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |
| **validation** | 数据验证和清理工具 |
| **protocol** | 协议抽象层，支持多协议 |
| **module_rpc** | 模块间 RPC 通信，支持分布式方法调用 |

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
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started") or None)
```

更多 Python 使用示例，请查看 [Python README](https://github.com/mf2023/DMSC/blob/master/python/README.zh.md)。

<div align="center">

## 开始使用

</div>

如果您是第一次使用 DMSC，建议从 [快速开始](./02-getting-started.md) 开始，了解如何安装和运行第一个 DMSC 应用。

如果您已经熟悉 DMSC 的基本概念，可以查看 [API 参考](./04-api-reference/README.md) 了解详细的模块 API，或查看 [使用示例](./05-usage-examples/README.md) 学习如何使用各种功能。

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

### Rust 依赖许可证

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

<div align="center">

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

</div>

</div>
