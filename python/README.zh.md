<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">适用于 Python 的 DMSC 库</span>
</h1>

[English](README.md) | 简体中文

[帮助文档](../doc/zh/index.md) | [更新日志](CHANGELOG.md) | [安全](../SECURITY.md) | [贡献](../CONTRIBUTING.md) | [行为准则](../CODE_OF_CONDUCT.md)

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

**DMSC (Dunimd Middleware Service)** — 一个高性能的 Rust 中间件框架，带有 Python 绑定。专为企业级规模构建，具有模块化架构、内置可观测性和分布式系统支持。

</div>

<h2 align="center">🏗️ 核心架构</h2>

### 📐 模块化设计
DMSC 采用高度模块化的架构，拥有 18 个核心模块，支持按需组合和无缝扩展：

<div align="center">

| 模块 | 描述 | Python 支持 |
|:--------|:------------|:------------|
| **auth** | 认证与授权（JWT、OAuth、权限） | ✅ 完整 |
| **cache** | 多后端缓存抽象（内存、Redis、混合） | ✅ 完整 |
| **config** | 多源配置管理与热重载 | ✅ 完整 |
| **core** | 运行时、错误处理和服务上下文 | ✅ 完整 |
| **database** | 数据库抽象层，支持 PostgreSQL、MySQL、SQLite | ✅ 完整 |
| **device** | 设备控制、发现和智能调度 | ✅ 完整 |
| **fs** | 安全的文件系统操作和管理 | ✅ 完整 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 | ✅ 完整 |
| **grpc** | gRPC 服务器和客户端支持 | ✅ 完整（服务注册表+处理器） |
| **hooks** | 生命周期事件钩子（启动、关闭等） | ✅ 完整 |
| **log** | 结构化日志与追踪上下文集成 | ✅ 完整 |
| **module_rpc** | 模块间 RPC 通信，支持分布式方法调用 | ✅ 完整 |
| **observability** | 指标、追踪和 Grafana 集成 | ✅ 完整 |
| **orm** | 类型安全的 ORM，带有仓储模式和查询构建器 | ✅ 完整（类型+QueryBuilder） |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） | ✅ 完整 |
| **service_mesh** | 服务发现、健康检查和流量管理 | ✅ 完整 |
| **validation** | 数据验证和清理工具 | ✅ 完整 |
| **ws** | WebSocket 服务器支持 | ✅ 完整（处理器+会话管理器） |

> **注意**：在 Python 中使用 gRPC/WebSocket 服务器，请使用原生 Python 库如 `grpcio` 和 `websockets`。DMSC Rust API 为高性能场景提供高级功能。

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
- **Python**: 3.8 及以上版本
- **pip**: 最新版本
- **平台**: Linux、macOS、Windows

### 构建依赖（从源码构建时）

某些功能在从源码构建时需要额外的系统依赖：

| 依赖 | 用于 | 安装 |
|:-----------|:-------------|:-------------|
| **protoc** | etcd 功能 (Protocol Buffers) | [Protocol Buffers](https://protobuf.dev/downloads/) |
| **CMake + C++ 编译器** | kafka 功能 (rdkafka) | 见下方说明 |

#### 安装 protoc

**Windows:**
```powershell
# 使用 chocolatey
choco install protoc

# 或从 GitHub releases 下载
# https://github.com/protocolbuffers/protobuf/releases
```

**macOS:**
```bash
brew install protobuf
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y protobuf-compiler
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install -y protobuf-compiler
```

#### 安装 CMake 和 C++ 编译器（用于 Kafka 支持）

**Windows:**
```powershell
# CMake 通常随 Visual Studio 安装
# 或从官网下载: https://cmake.org/download/

# 使用 chocolatey
choco install cmake
```

**macOS:**
```bash
# CMake 和 C++ 编译器 (Xcode Command Line Tools)
xcode-select --install

# 或使用 Homebrew
brew install cmake
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install -y cmake build-essential
```

**Linux (CentOS/RHEL):**
```bash
sudo yum install -y cmake gcc-c++ make
```

### 快速设置

安装 DMSC Python 包：

```bash
pip install dmsc
```

或者添加到您的 `requirements.txt`：

```
dmsc==0.1.7
```

<h2 align="center">⚡ 快速开始</h2>

### 核心 API 使用

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # 构建服务运行时（支持链式调用）
    runtime = (DMSCAppBuilder()
        .with_config("config.yaml")
        .with_logging(DMSCLogConfig())
        .build())
    
    # 运行业务逻辑
    await runtime.run()

asyncio.run(main())
```

### 认证示例

```python
from dmsc import DMSCJWTManager

# 创建 JWT 管理器（需要密钥和过期时间）
jwt_manager = DMSCJWTManager("your-secret-key", 3600)

# 生成令牌（需要用户 ID、角色、权限）
token = jwt_manager.generate_token("user123", ["admin"], ["read", "write"])

# 验证令牌
payload = jwt_manager.validate_token(token)
print(f"用户 ID: {payload.sub}")
print(f"角色: {payload.roles}")
print(f"权限: {payload.permissions}")
```

### 队列管理示例

```python
from dmsc import DMSCQueueManager

# 创建队列管理器（默认使用内存后端）
queue_manager = DMSCQueueManager()

# 创建队列
queue_manager.create_queue("my_queue")

# 列出所有队列
queues = queue_manager.list_queues()
print(f"队列: {queues}")

# 获取指定队列（在 Rust 中进行进一步操作）
queue = queue_manager.get_queue("my_queue")

# 删除队列
queue_manager.delete_queue("my_queue")
```

**注意**：有关高级队列操作（push、pop 等），请直接使用 Rust API 或扩展 Python 绑定。

### 服务网格示例

```python
from dmsc.service_mesh import DMSCServiceMesh, DMSCServiceMeshConfig

# 创建服务网格配置
config = DMSCServiceMeshConfig()

# 创建服务网格
service_mesh = DMSCServiceMesh(config)

# 注册服务
service_mesh.register_service("user-service", "http://localhost:8080", 100)

# 发现服务
instances = service_mesh.discover_service("user-service")
for instance in instances:
    print(f"服务: {instance.service_name}, 主机: {instance.host}, 端口: {instance.port}")
```

### 缓存管理示例

```python
from dmsc import DMSCCacheManager

# 创建缓存管理器（默认使用内存后端）
cache_manager = DMSCCacheManager()

# 设置缓存值
cache_manager.set("user:123", '{"name": "John"}', ttl=3600)

# 获取缓存值
user_data = cache_manager.get("user:123")

# 检查键是否存在
if cache_manager.exists("user:123"):
    cache_manager.delete("user:123")
```

### gRPC 服务示例

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# 创建 gRPC 服务注册表
registry = DMSCGrpcServiceRegistryPy()

# 定义服务处理器
def my_handler(method: str, data: bytes) -> bytes:
    print(f"收到请求: {method}")
    return b"来自 Python 处理器的响应"

# 注册服务
registry.register("my-service", my_handler)

# 列出已注册的服务
services = registry.list_services()
print(f"服务: {services}")
```

### WebSocket 处理器示例

```python
from dmsc.ws import DMSCWSPythonHandler, DMSCWSSessionManagerPy

# 创建带回调的 WebSocket 处理器
handler = DMSCWSPythonHandler(
    on_connect=lambda session_id, remote_addr: print(f"连接: {session_id}"),
    on_disconnect=lambda session_id: print(f"断开: {session_id}"),
    on_message=lambda session_id, data: b"回显: " + data,
    on_error=lambda session_id, error: print(f"错误: {error}")
)

# 创建会话管理器
manager = DMSCWSSessionManagerPy(max_connections=1000)

# 获取会话数量
count = manager.get_session_count()
print(f"活动会话: {count}")
```

### ORM 使用示例（Rust API）

```rust
use dmsc::database::{DMSCORMSimpleRepository, Criteria, Pagination, ComparisonOperator};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct User {
    id: String,
    name: String,
    email: String,
}

// 创建仓储（仅 Rust）
let repo = DMSCORMSimpleRepository::<User>::new("users");

// 查找所有用户
let users = repo.find_all(&db).await?;

// 带条件的查询
let criteria = Criteria::new("name", ComparisonOperator::Like, serde_json::json!("%John%"));
let users = repo.find_many(&db, vec![criteria]).await?;

// 分页查询
let pagination = Pagination::new(1, 20);
let (users, total) = repo.find_paginated(&db, pagination, vec![]).await?;
```

> **注意**：ORM 模块在 Rust 中提供类型安全的数据库操作。在 Python 中，请直接使用 SQLAlchemy 或其他 ORM 库。

<h2 align="center">🔧 配置</h2>

### 配置示例

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"

logging:
  level: "info"
  file_format: "json"
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

DMSC 支持多种配置源，按优先级排序（从低到高）：
1. 配置文件（YAML、TOML、JSON）
2. 自定义配置（通过代码设置）
3. 环境变量（以 `DMSC_` 为前缀）

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
A: 支持 Python 3.8 及以上版本。

**Q: Rust 后端是否包含在内？**
A: 是的，该包包含了编译后的 Rust 后端和 Python 绑定。

<h2 align="center">🌏 社区与引用</h2>

- 欢迎提交 Issues 和 PRs！
- Gitee: https://gitee.com/dunimd/dmsc.git
- Github: https://github.com/mf2023/DMSC.git

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