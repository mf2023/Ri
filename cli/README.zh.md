<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 8px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../assets/svg/ri.svg" width="36" height="36" alt="Ri">Ri CLI (ric)</span>
</h1>

[English](README.md) | 简体中文

<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>
<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>

<a href="https://github.com/mf2023/Ri" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-Ri-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://gitcode.com/dunimd/ri.git" target="_blank">
    <img alt="GitCode" src="https://img.shields.io/badge/GitCode-Ri-FF6B35?style=flat-square&logo=git"/>
</a>

<img alt="Version" src="https://img.shields.io/badge/version-0.1.0-green?style=flat-square"/>
<img alt="Rust" src="https://img.shields.io/badge/rust-1.65%2B-orange?style=flat-square"/>
<img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square"/>

**Ri CLI (ric)** — 强大的命令行界面工具，用于管理 Ri 框架项目。提供项目创建、构建、运行和配置管理的完整命令。

</div>

<h2 align="center">✨ 功能特性</h2>

<div align="center">

| 功能 | 描述 |
|:--------|:-------------|
| **项目脚手架** | 使用多种模板创建新的 Ri 项目（web、api、worker、microservice、minimal） |
| **构建管理** | 为不同目标构建项目（Python、Java、C、原生） |
| **配置管理** | 初始化、验证和管理项目配置 |
| **连接测试** | 测试 Redis、PostgreSQL、MySQL 和 Kafka 的连接性 |
| **环境诊断** | 全面的环境检查，支持自动修复功能 |
| **代码生成** | 生成模块、中间件和配置结构 |
| **彩色输出** | 丰富的终端输出，带有进度指示器 |
| **交互式提示** | 项目创建时的用户友好提示 |

</div>

<h2 align="center">🛠️ 安装</h2>

### 从源码安装（推荐）

```bash
# 克隆仓库
git clone https://github.com/mf2023/Ri.git
cd Ri/cli

# 以发布模式构建
cargo build --release

# 二进制文件将位于：
# ./target/release/ric

# 可选：添加到 PATH
cp ./target/release/ric /usr/local/bin/
```

### 使用 Cargo 安装

```bash
# 直接从仓库安装
cargo install --git https://github.com/mf2023/Ri --bin ric
```

### 二进制文件下载

预构建的二进制文件适用于主要平台：

- **Linux**: `ric-linux-x86_64`
- **macOS**: `ric-darwin-x86_64`
- **Windows**: `ric-windows-x86_64.exe`

从 [Releases](https://github.com/mf2023/Ri/releases) 页面下载。

<h2 align="center">⚡ 快速开始</h2>

### 创建新项目

```bash
# 创建最小项目（默认）
ric new my-project

# 创建 Web 应用
ric new my-web-app --template web

# 创建 API 服务
ric new my-api --template api

# 创建 Worker 服务
ric new my-worker --template worker

# 创建微服务
ric new my-service --template microservice
```

### 构建和运行

```bash
# 进入项目目录
cd my-project

# 构建项目
ric build

# 以发布模式构建
ric build --release

# 运行项目
ric run

# 以发布模式运行
ric run --release
```

### 配置管理

```bash
# 初始化配置文件
ric config init

# 显示当前配置
ric config show

# 验证配置
ric config validate

# 设置配置值
ric config set runtime.workers 8

# 获取配置值
ric config get project.name
```

<h2 align="center">📋 命令参考</h2>

### 项目管理

| 命令 | 描述 |
|:--------|:-------------|
| `ric new <name>` | 创建新的 Ri 项目 |
| `ric build` | 构建项目 |
| `ric run` | 运行项目 |
| `ric check` | 检查项目错误 |
| `ric clean` | 清理构建产物 |
| `ric info` | 显示项目信息 |

### 配置

| 命令 | 描述 |
|:--------|:-------------|
| `ric config init` | 初始化配置文件 |
| `ric config show` | 显示当前配置 |
| `ric config validate` | 验证配置文件 |
| `ric config check` | 检查环境变量 |
| `ric config set <key> <value>` | 设置配置值 |
| `ric config get <key>` | 获取配置值 |

### 连接测试

| 命令 | 描述 |
|:--------|:-------------|
| `ric test redis <url>` | 测试 Redis 连接 |
| `ric test postgres <url>` | 测试 PostgreSQL 连接 |
| `ric test mysql <url>` | 测试 MySQL 连接 |
| `ric test kafka <url>` | 测试 Kafka 连接 |

### 代码生成

| 命令 | 描述 |
|:--------|:-------------|
| `ric generate module <type> <name>` | 生成新模块 |
| `ric generate middleware <name>` | 生成中间件模板 |
| `ric generate config <file>` | 从配置生成 Rust 结构体 |

### 诊断

| 命令 | 描述 |
|:--------|:-------------|
| `ric doctor` | 运行环境诊断 |
| `ric doctor --verbose` | 详细诊断 |
| `ric doctor --fix` | 自动修复检测到的问题 |
| `ric version` | 显示版本信息 |

<h2 align="center">📁 模板</h2>

Ri CLI 为不同的使用场景提供五种项目模板：

### Minimal（最小）

最简单的模板，仅包含应用程序构建器和日志记录器。非常适合简单应用程序或学习 Ri 基础。

```bash
ric new my-minimal --template minimal
```

**特性**：
- RiAppBuilder 用于应用程序初始化
- RiLogger 用于结构化日志
- 最小依赖

### Web（Web 应用）

功能完整的 Web 应用模板，包含 HTTP 服务器、缓存和认证。

```bash
ric new my-web --template web
```

**特性**：
- RiGateway 用于 HTTP 路由
- RiCacheModule 用于响应缓存
- RiAuthModule 用于认证
- CORS 和 TLS 支持
- 开发和生产配置

### API（API 服务）

RESTful API 服务模板，支持验证和 OpenAPI 文档。

```bash
ric new my-api --template api
```

**特性**：
- RiGateway 用于 API 路由
- RiValidationModule 用于请求验证
- 标准 API 响应类型
- 健康检查端点

### Worker（工作服务）

后台作业处理服务，具有队列管理和设备控制功能。

```bash
ric new my-worker --template worker
```

**特性**：
- RiQueueModule 用于任务处理
- RiDeviceControlModule 用于设备管理
- 可配置的工作池
- 死信队列支持

### Microservice（微服务）

分布式微服务模板，具有服务网格和可观测性。

```bash
ric new my-service --template microservice
```

**特性**：
- RiServiceMesh 用于服务发现
- RiObservabilityModule 用于指标和追踪
- 分布式追踪支持
- 健康检查服务器

<h2 align="center">🔧 配置</h2>

Ri CLI 使用 YAML 配置文件（`ric.yaml`）进行项目设置。

### 配置结构

```yaml
# 项目元数据
project:
  name: my-project
  version: 0.1.0
  template: web

# 构建设置
build:
  release: false
  target: all
  features:
    - default

# 运行时设置
runtime:
  log_level: info
  workers: 4

# 模块配置
cache:
  enabled: true
  backend_type: Memory
  default_ttl_secs: 3600

gateway:
  listen_address: "0.0.0.0"
  listen_port: 8080
  cors_enabled: true
```

### 配置文件位置

1. **项目配置**：`./ric.yaml`（当前目录）
2. **环境覆盖**：`RI_CONFIG_PATH` 环境变量
3. **用户配置**：`~/.config/ric/config.yaml`

### 环境变量

| 变量 | 描述 |
|:---------|:-------------|
| `RI_CONFIG_PATH` | 自定义配置文件路径 |
| `RI_LOG_LEVEL` | 覆盖日志级别 |
| `RUST_LOG` | Rust 日志配置 |
| `CARGO_HOME` | Cargo 主目录 |
| `RUSTUP_HOME` | Rustup 主目录 |

<h2 align="center">❓ 故障排除</h2>

### 常见问题

#### 项目创建失败

**错误**：`Project directory already exists`

**解决方案**：选择不同的项目名称或删除现有目录。

```bash
# 删除现有目录
rm -rf my-project
ric new my-project
```

#### 构建失败

**错误**：`Could not find Ri in registry`

**解决方案**：确保从 Ri 仓库构建或本地有 Ri 可用。

```bash
# 从源码构建
cd Ri
cargo build --release
```

#### 配置验证失败

**错误**：`Invalid configuration: missing required field`

**解决方案**：使用 `ric config validate` 识别问题并修复。

```bash
ric config validate
# 按照建议修复错误
```

#### 连接测试失败

**错误**：`Connection refused`

**解决方案**：确保服务正在运行且可访问。

```bash
# 检查 Redis 是否运行
redis-cli ping

# 使用正确的 URL 测试
ric test redis redis://localhost:6379
```

### 诊断工具

运行诊断以识别和修复常见问题：

```bash
# 基本诊断
ric doctor

# 详细诊断
ric doctor --verbose

# 自动修复问题
ric doctor --fix
```

### 获取帮助

1. **CLI 帮助**：使用 `--help` 标志获取特定命令的帮助
   ```bash
   ric --help
   ric new --help
   ric config --help
   ```

2. **文档**：查看 [docs](docs/) 目录获取详细指南

3. **问题**：在 [GitHub Issues](https://github.com/mf2023/Ri/issues) 报告错误

<h2 align="center">🤝 贡献</h2>

我们欢迎贡献！请遵循以下指南：

### 开发设置

```bash
# 克隆仓库
git clone https://github.com/mf2023/Ri.git
cd Ri/cli

# 安装开发依赖
cargo build

# 运行测试
cargo test

# 运行 clippy
cargo clippy

# 格式化代码
cargo fmt
```

### 贡献指南

1. **Fork 仓库**并创建功能分支
2. **编写测试**用于新功能
3. **遵循 Rust 约定**并运行 `cargo fmt`
4. **更新文档**用于更改的功能
5. **提交拉取请求**并提供清晰的描述

### 代码风格

- 遵循标准 Rust 约定
- 使用 `cargo fmt` 进行格式化
- 运行 `cargo clippy` 进行代码检查
- 为公共 API 添加文档注释

<div align="center">

## 📄 许可证与开源协议

### 🏛️ 项目许可证

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

本项目使用 **Apache License 2.0** 开源协议，详见 [LICENSE](LICENSE) 文件。

### 📋 依赖包开源协议

本项目使用的开源包及其协议信息：

### 依赖许可证

<div align="center">

| 📦 包名 | 📜 许可证 | 📦 包名 | 📜 许可证 |
|:-----------|:-----------|:-----------|:-----------|
| clap | MIT/Apache-2.0 | serde | MIT/Apache-2.0 |
| serde_yaml | MIT/Apache-2.0 | serde_json | MIT/Apache-2.0 |
| tera | MIT | anyhow | MIT/Apache-2.0 |
| thiserror | MIT/Apache-2.0 | colored | MIT |
| indicatif | MIT | dialoguer | MIT |
| tokio | MIT | chrono | MIT/Apache-2.0 |
| uuid | MIT/Apache-2.0 | regex | MIT/Apache-2.0 |
| walkdir | MIT/Apache-2.0 | toml | MIT/Apache-2.0 |
| async-trait | MIT/Apache-2.0 | ri | Apache-2.0 |
| redis | MIT | tokio-postgres | MIT/Apache-2.0 |
| mysql_async | MIT | rdkafka | BSD-2-Clause |

</div>

</div>