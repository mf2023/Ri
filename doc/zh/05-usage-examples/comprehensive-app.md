<div align="center">

# 综合应用示例

**Version: 0.1.7**

**Last modified date: 2026-02-11**

本示例展示如何构建一个完整的DMSC企业级应用，整合所有核心模块。

## 示例概述

</div>

本示例将创建一个完整的DMSC应用，实现以下功能：

- 应用初始化和配置管理
- JWT认证和授权
- 缓存操作
- 消息队列集成
- 服务网格配置
- 可观测性（指标和追踪）
- 数据库操作
- API网关配置

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- Redis 6.0+（用于缓存和消息队列）
- PostgreSQL 14+（用于数据库）
- 基本的Rust和Python编程知识

<div align="center">

## 项目结构

</div>

```
dmsc-complete-example/
├── Cargo.toml
├── config.yaml
├── src/
│   └── main.rs
└── python/
    └── complete_example.py
```

<div align="center">

## 第一部分：Rust示例

</div>

### 1. 创建项目

```bash
cargo new dmsc-complete-example
cd dmsc-complete-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC", features = ["pyo3"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dmsc-complete-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true
  file_name: "dmsc.log"

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090

cache:
  backend: "memory"
  max_size: 1000
  default_ttl: 300

queue:
  backend: "redis"
  host: "localhost"
  port: 6379
  db: 0

database:
  type: "postgres"
  host: "localhost"
  port: 5432
  database: "dmsc_db"
  max_connections: 10

auth:
  jwt_secret: "your-secret-key-change-in-production"
  token_expiry_hours: 24

gateway:
  port: 8080
  workers: 4
```

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    role: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Order {
    id: u64,
    user_id: u64,
    product: String,
    quantity: u32,
    price: f64,
    status: String,
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    println!("=== DMSC Complete Example ===\n");
    
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        let service_name = ctx.config().config()
            .get_str("service.name").unwrap_or("unknown");
        let service_version = ctx.config().config()
            .get_str("service.version").unwrap_or("unknown");
        
        ctx.logger().info("service", &format!(
            "DMSC service started: {} v{}", service_name, service_version
        ))?;
        
        let cache = ctx.cache();
        let queue = ctx.queue();
        let auth = ctx.auth();
        let mesh = ctx.service_mesh();
        let obs = ctx.observability();
        
        ctx.logger().info("example", "All modules initialized successfully")?;
        
        Ok(())
    }).await
}
```

### 5. 运行示例

```bash
cargo run
```

<div align="center">

## 第二部分：Python示例

</div>

### 1. 安装Python包

```bash
pip install dmsc
```

### 2. 创建Python示例文件

在`python/complete_example.py`中创建以下内容：

```python
#!/usr/bin/env python3

import asyncio
from datetime import datetime, timedelta
from typing import Optional

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCResult,
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTClaims,
    DMSCCacheModule, DMSCCacheConfig,
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager,
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCObservabilityModule, DMSCObservabilityConfig,
    DMSCGateway, DMSCGatewayConfig, DMSCRouter, DMSCRoute,
)


async def demonstrate_application():
    """Demonstrate complete DMSC application."""
    print("=== DMSC Complete Application ===\n")
    
    print("1. Application initialization...")
    builder = DMSCAppBuilder()
    app = builder.with_config("config.yaml").build()
    print("   Application initialized\n")
    
    print("2. Authentication module...")
    auth_config = DMSCAuthConfig()
    auth_config.set_jwt_secret("your-secret-key")
    auth_module = DMSCAuthModule(auth_config)
    print("   Auth module ready\n")
    
    print("3. Cache module...")
    cache_config = DMSCCacheConfig.memory(max_size=1000)
    cache_module = DMSCCacheModule(cache_config)
    cache_module.set("user:1:name", "Alice")
    name = cache_module.get("user:1:name")
    print(f"   Cache test: user:name = {name}\n")
    
    print("4. Message queue module...")
    queue_config = DMSCQueueConfig.redis(host="localhost", port=6379)
    queue_module = DMSCQueueModule(queue_config)
    print("   Queue module ready\n")
    
    print("5. Service mesh module...")
    mesh_config = DMSCServiceMeshConfig()
    service_mesh = DMSCServiceMesh(mesh_config)
    await service_mesh.register_service("api-gateway", "http://api:8080", 100)
    print("   Service mesh ready\n")
    
    print("6. Observability module...")
    obs_config = DMSCObservabilityConfig()
    obs_config.set_metrics_enabled(True)
    obs_module = DMSCObservabilityModule(obs_config)
    print("   Observability ready\n")
    
    print("7. Gateway module...")
    gateway_config = DMSCGatewayConfig()
    gateway_config.set_port(8080)
    router = DMSCRouter()
    route = DMSCRoute(path="/api/health", method="GET", handler="health_handler")
    router.add_route(route)
    gateway = DMSCGateway(gateway_config, router)
    print("   Gateway ready\n")
    
    print("=== Complete Application Demo Finished ===")


async def main():
    try:
        await demonstrate_application()
    except Exception as e:
        print(f"Error: {e}")
        print("Note: Some features require running services")


if __name__ == "__main__":
    asyncio.run(main())
```

### 3. 运行Python示例

```bash
cd python
python complete_example.py
```

<div align="center">

## 模块集成说明

</div>

### 应用初始化流程

```
DMSCAppBuilder
    ↓ with_config()
    ↓ with_logging()
    ↓ with_observability()
    ↓ build()
    ↓
DMSCAppRuntime
    ↓ run()
    ↓
DMSCServiceContext
    (提供对所有模块的访问)
```

### 模块依赖关系

| 模块 | 依赖 | 说明 |
|------|------|------|
| Core | 无 | 所有模块的基础 |
| Log | Core | 日志记录 |
| Config | Core | 配置管理 |
| Auth | Core, Log | 认证授权 |
| Cache | Core, Config | 缓存抽象 |
| Queue | Core, Config | 消息队列 |
| Service Mesh | Core, Gateway | 服务管理 |
| Gateway | Core, Service Mesh | API网关 |
| Observability | Core, Log | 可观测性 |
| Database | Core, Config | 数据库 |

<div align="center">

## 配置文件详解

</div>

### 必需配置项

```yaml
service:
  name: "your-service-name"
  version: "1.0.0"
```

### 日志配置

```yaml
logging:
  level: "info"          # DEBUG, INFO, WARN, ERROR
  format: "json"         # json 或 text
  file_enabled: true     # 是否启用文件日志
  console_enabled: true  # 是否启用控制台日志
  file_name: "app.log"   # 日志文件名
```

### 可观测性配置

```yaml
observability:
  metrics_enabled: true     # 启用指标收集
  tracing_enabled: true     # 启用追踪
  prometheus_port: 9090     # Prometheus指标端口
```

### 缓存配置

```yaml
cache:
  backend: "memory"         # memory, redis
  max_size: 1000           # 最大缓存项数
  default_ttl: 300         # 默认TTL（秒）
```

### 消息队列配置

```yaml
queue:
  backend: "redis"         # redis, rabbitmq, kafka, memory
  host: "localhost"       # Redis主机
  port: 6379              # 端口
  db: 0                   # 数据库编号
```

### 数据库配置

```yaml
database:
  type: "postgres"        # postgres, mysql, sqlite
  host: "localhost"
  port: 5432
  database: "your_db"
  max_connections: 10
```

### 认证配置

```yaml
auth:
  jwt_secret: "your-secret-key"
  token_expiry_hours: 24
```

### 网关配置

```yaml
gateway:
  port: 8080
  workers: 4
```

<div align="center">

## 常见问题

</div>

**Q: 如何添加新的模块？**

A: 实现`DMSCModule` trait并通过`DMSCAppBuilder::with_module()`注册。

**Q: 如何配置多个数据源？**

A: 在配置文件中添加多个数据库配置，使用`DMSCConfigManager`管理。

**Q: 如何实现自定义认证？**

A: 实现`DMSCAuthHandler` trait并注入到`DMSCAuthModule`。

**Q: 如何监控系统性能？**

A: 使用`DMSCObservabilityModule`获取指标，通过Prometheus端点暴露。

<div align="center>

## 完整示例源码

</div>

完整的示例项目请参考：

- Rust示例：`examples/Rust/`
- Python示例：`examples/Python/comprehensive_example.py`
- 文档：`doc/zh/05-usage-examples/`

<div align="center">

## 下一步

</div>

- 查看[认证示例](authentication.md)了解JWT和OAuth配置
- 查看[缓存示例](caching.md)了解缓存策略
- 查看[服务网格示例](service_mesh.md)了解服务发现
- 查看[可观测性示例](observability.md)了解指标和追踪
