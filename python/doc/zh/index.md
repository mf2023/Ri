<div align="center">

# DMSC Python 文档

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

## 文档导航

### 1. 介绍与入门

- [**介绍**](./01-introduction.md) - DMSC Python绑定的概述和核心特性
- [**快速开始**](./02-getting-started.md) - 安装、配置和运行第一个DMSC Python应用

### 2. 核心概念

- [**核心概念**](./03-core-concepts.md) - 深入理解DMSC Python的设计哲学、服务上下文、模块系统和生命周期管理

### 3. API 参考

- [**API 参考**](./04-api-reference/README.md) - 详细的模块API文档，包括核心、认证、缓存、配置等模块

### 4. 使用示例

- [**使用示例**](./05-usage-examples/README.md) - 各种功能的使用示例，包括基础应用、认证与授权、缓存使用、可观测性等

### 5. 最佳实践

- [**最佳实践**](./06-best-practices.md) - 构建高效、可靠、安全的DMSC Python应用的最佳实践

### 6. 故障排除

- [**故障排除**](./07-troubleshooting.md) - 常见问题和解决方案，帮助您快速定位和解决问题

### 7. 术语表

- [**术语表**](./08-glossary.md) - DMSC Python文档中使用的技术术语和概念定义

<div align="center">

## 什么是 DMSC Python？

</div>

**DMSC Python** — DMSC (Dunimd Middleware Service) 的官方Python绑定，为Python开发者提供高性能、企业级的微服务开发框架。它继承了Rust核心的所有优势，同时提供Python友好的API接口。

### 核心特性

- **🚀 高性能**: 基于Rust核心，提供接近原生的性能
- **🐍 Python友好**: 符合Python编程习惯的API设计
- **🔧 完整功能**: 支持DMSC所有12个核心模块
- **📦 易于安装**: 通过PyPI简单安装
- **🔍 类型提示**: 完整的类型注解支持
- **⚡ 异步支持**: 原生支持async/await模式

### 支持的模块

| 模块 | Python支持 | 描述 |
|------|------------|------|
| **core** | ✅ | 运行时、错误处理和服务上下文 |
| **auth** | ✅ | 认证与授权（JWT、OAuth、权限） |
| **cache** | ✅ | 多后端缓存抽象 |
| **config** | ✅ | 多源配置管理 |
| **log** | ✅ | 结构化日志 |
| **observability** | ✅ | 指标、追踪和监控 |
| **http** | ✅ | Web服务和RESTful API |
| **fs** | ✅ | 安全的文件系统操作 |
| **device** | 🚧 | 设备管理（开发中） |
| **gateway** | 🚧 | API网关（开发中） |
| **queue** | 🚧 | 分布式队列（开发中） |
| **service_mesh** | 🚧 | 服务网格（开发中） |

### 快速示例

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # 创建应用构建器
    app = DMSCAppBuilder()
    
    # 配置日志
    app.with_logging(DMSCLogConfig.default())
    
    # 构建应用
    dms_app = app.build()
    
    # 运行应用
    await dms_app.run_async(my_service_logic)

async def my_service_logic(ctx):
    # 使用服务上下文
    ctx.logger.info("demo", "Hello from DMSC Python!")
    
    # 访问配置
    config_value = ctx.config.get("my.key", "default")
    
    # 使用缓存
    await ctx.cache.set("key", "value", ttl=3600)
    
    return {"status": "success"}

if __name__ == "__main__":
    asyncio.run(main())
```

### 安装方式

```bash
# 从PyPI安装
pip install dmsc

# 或者使用poetry
poetry add dmsc

# 或者使用pipenv
pipenv install dmsc
```

### 系统要求

- **Python版本**: 3.8+
- **操作系统**: Windows, Linux, macOS
- **架构支持**: x86_64, ARM64

<div align="center">

## 下一步

</div>

- [介绍](./01-introduction.md) - 深入了解DMSC Python的核心架构和设计理念
- [快速开始](./02-getting-started.md) - 开始您的第一个DMSC Python项目
- [核心概念](./03-core-concepts.md) - 掌握DMSC Python的核心概念和最佳实践