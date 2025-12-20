<div align="center">

# DMSC (Dunimd Middleware Service) - Python 绑定

[English](README.md) | 简体中文

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://crates.io/crates/dmsc" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMSC-000000?style=flat-square&logo=rust"/>
</a>
<a href="https://pypi.org/project/dmsc/" target="_blank">
    <img alt="PyPI" src="https://img.shields.io/badge/PyPI-DMSC-3775A9?style=flat-square&logo=pypi"/>
</a>

**DMSC (Dunimd Middleware Service)** — 一个高性能的 Rust 中间件框架，带有 Python 绑定。专为企业级规模构建，具有模块化架构、内置可观测性和分布式系统支持。

</div>

<h2 align="center">🏗️ 核心架构</h2>

### 📐 模块化设计
DMSC 采用高度模块化的架构，拥有 12 个核心模块，支持按需组合和无缝扩展：

<div align="center">

| 模块 | 描述 |
|:--------|:-------------|
| **auth** | 认证与授权（JWT、OAuth、权限） |
| **cache** | 多后端缓存抽象（内存、Redis、混合） |
| **config** | 多源配置管理与热重载 |
| **core** | 运行时、错误处理和服务上下文 |
| **device** | 设备控制、发现和智能调度 |
| **fs** | 安全的文件系统操作和管理 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 |
| **hooks** | 生命周期事件钩子（启动、关闭等） |
| **log** | 结构化日志与追踪上下文集成 |
| **observability** | 指标、追踪和 Grafana 集成 |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） |
| **service_mesh** | 服务发现、健康检查和流量管理 |

</div>

### 🚀 核心特性

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

<h2 align="center">🛠️ 安装与环境</h2>

### 前置要求
- **Python**: 3.7+
- **pip**: 最新版本
- **平台**: Linux、macOS、Windows

### 快速设置

安装 DMSC Python 包：

```bash
pip install dmsc
```

或者添加到您的 `requirements.txt`：

```
dmsc==0.1.3
```

<h2 align="center">⚡ 快速开始</h2>

### 核心 API 使用

```python
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCObservabilityConfig

# 构建服务运行时
app = DMSCAppBuilder() \\
    .with_config("config.yaml") \\
    .with_logging(DMSCLogConfig.default()) \\
    .with_observability(DMSCObservabilityConfig.default()) \\
    .build()

# 运行业务逻辑
app.run(lambda ctx: ctx.logger().info("service", "DMSC service started"))
```

### 认证示例

```python
from dmsc import DMSCAuthModule, DMSCJWTManager

# 创建 JWT 管理器
jwt_manager = DMSCJWTManager()
token = jwt_manager.generate_token({"user_id": 123})

# 验证令牌
payload = jwt_manager.verify_token(token)
```

### 队列管理示例

```python
from dmsc import DMSCQueueManager, DMSCQueueConfig

# 创建队列管理器
queue_config = DMSCQueueConfig()
queue_manager = DMSCQueueManager(queue_config)

# 发送消息
queue_manager.send_message("my_queue", {"data": "hello"})

# 接收消息
message = queue_manager.receive_message("my_queue")
```

### 服务网格示例

```python
from dmsc import DMSCServiceMesh, DMSCServiceDiscovery

# 创建服务网格
service_mesh = DMSCServiceMesh()
service_discovery = DMSCServiceDiscovery()

# 注册服务
service_discovery.register_service("user-service", "localhost:8080")

# 发现服务
service_info = service_discovery.discover_service("user-service")
```

### 缓存管理示例

```python
from dmsc import DMSCCacheManager, DMSCCacheConfig

# 创建缓存管理器
cache_config = DMSCCacheConfig()
cache_manager = DMSCCacheManager(cache_config)

# 设置缓存值
cache_manager.set("user:123", {"name": "John"}, ttl=3600)

# 获取缓存值
user_data = cache_manager.get("user:123")
```

<h2 align="center">🔧 配置</h2>

### 配置示例

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090

resource:
  providers: ["cpu", "gpu", "memory"]
  scheduling_policy: "priority_based"
```

### 配置源

DMSC 支持多种配置源，按优先级排序（从高到低）：
1. 运行时参数
2. 环境变量（以 `DMSC_` 为前缀）
3. 配置文件（YAML、TOML、JSON）
4. 默认值

<h2 align="center">🧪 开发与测试</h2>

### 运行测试

```bash
# 安装开发依赖
pip install -e .

# 运行 Python 测试
python -m pytest tests/

# 运行特定测试模块
python -m pytest tests/test_auth.py
```

<h2 align="center">❓ 常见问题</h2>

**Q: 如何添加新模块？**
A: 使用现有的模块接口并通过 DMSCAppBuilder 注册。

**Q: 如何配置日志级别？**
A: 在配置文件中设置 `logging.level`，支持 DEBUG/INFO/WARN/ERROR 级别。

**Q: 如何启用指标导出？**
A: 在配置文件中设置 `observability.metrics_enabled: true` 并配置 `prometheus_port`。

**Q: 如何扩展配置源？**
A: 实现自定义配置加载器并用 DMSC 配置系统注册。

**Q: 如何处理异步任务？**
A: 框架内部处理异步操作，使用提供的异步接口即可。

**Q: 支持哪些 Python 版本？**
A: 支持 Python 3.7 及以上版本。

**Q: Rust 后端是否包含在内？**
A: 是的，该包包含了编译后的 Rust 后端和 Python 绑定。

<h2 align="center">🌏 社区与引用</h2>

- 欢迎提交 Issues 和 PRs！
- Gitee: https://gitee.com/dunimd/dmsc.git

<div align="center">

## 📄 许可证与开源协议

### 🏛️ 项目许可证

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

本项目使用 **Apache License 2.0** 开源协议，详见 [LICENSE](LICENSE) 文件。

### 📋 依赖包许可证

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