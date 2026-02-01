<div align="center">

# Comprehensive Application Example

**Version: 0.1.6**

**Last modified date: 2026-02-01**

This example shows how to build a complete DMSC enterprise application integrating all core modules.

## Example Overview

</div>

This example will create a complete DMSC application that implements the following features:

- Application initialization and configuration management
- JWT authentication and authorization
- Cache operations
- Message queue integration
- Service mesh configuration
- Observability (metrics and tracing)
- Database operations
- API gateway configuration

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Redis 6.0+ (for cache and message queue)
- PostgreSQL 14+ (for database)
- Basic Rust and Python programming knowledge

<div align="center">

## Project Structure

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

## Part 1: Rust Example

</div>

### 1. Create a Project

```bash
cargo new dmsc-complete-example
cd dmsc-complete-example
```

### 2. Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC", features = ["pyo3"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root:

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

### 4. Write Main Code

Replace the content of `src/main.rs` with the following:

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

### 5. Run the Example

```bash
cargo run
```

<div align="center">

## Part 2: Python Example

</div>

### 1. Install Python Package

```bash
pip install dmsc
```

### 2. Create Python Example File

Create the following content in `python/complete_example.py`:

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

### 3. Run the Python Example

```bash
cd python
python complete_example.py
```

<div align="center">

## Module Integration Guide

</div>

### Application Initialization Flow

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
    (provides access to all modules)
```

### Module Dependencies

| Module | Dependencies | Description |
|--------|--------------|-------------|
| Core | None | Base for all modules |
| Log | Core | Logging |
| Config | Core | Configuration management |
| Auth | Core, Log | Authentication and authorization |
| Cache | Core, Config | Cache abstraction |
| Queue | Core, Config | Message queue |
| Service Mesh | Core, Gateway | Service management |
| Gateway | Core, Service Mesh | API gateway |
| Observability | Core, Log | Observability |
| Database | Core, Config | Database |

<div align="center">

## Configuration File Details

</div>

### Required Configuration

```yaml
service:
  name: "your-service-name"
  version: "1.0.0"
```

### Logging Configuration

```yaml
logging:
  level: "info"          # DEBUG, INFO, WARN, ERROR
  format: "json"         # json or text
  file_enabled: true     # Enable file logging
  console_enabled: true  # Enable console logging
  file_name: "app.log"   # Log filename
```

### Observability Configuration

```yaml
observability:
  metrics_enabled: true     # Enable metrics collection
  tracing_enabled: true     # Enable tracing
  prometheus_port: 9090     # Prometheus metrics port
```

### Cache Configuration

```yaml
cache:
  backend: "memory"         # memory, redis
  max_size: 1000           # Maximum cache items
  default_ttl: 300         # Default TTL (seconds)
```

### Message Queue Configuration

```yaml
queue:
  backend: "redis"         # redis, rabbitmq, kafka, memory
  host: "localhost"       # Redis host
  port: 6379              # Port
  db: 0                   # Database number
```

### Database Configuration

```yaml
database:
  type: "postgres"        # postgres, mysql, sqlite
  host: "localhost"
  port: 5432
  database: "your_db"
  max_connections: 10
```

### Authentication Configuration

```yaml
auth:
  jwt_secret: "your-secret-key"
  token_expiry_hours: 24
```

### Gateway Configuration

```yaml
gateway:
  port: 8080
  workers: 4
```

<div align="center">

## FAQ

</div>

**Q: How to add a new module?**

A: Implement the `DMSCModule` trait and register it via `DMSCAppBuilder::with_module()`.

**Q: How to configure multiple data sources?**

A: Add multiple database configurations in the config file and use `DMSCConfigManager` to manage them.

**Q: How to implement custom authentication?**

A: Implement the `DMSCAuthHandler` trait and inject it into `DMSCAuthModule`.

**Q: How to monitor system performance?**

A: Use `DMSCObservabilityModule` to get metrics and expose them via the Prometheus endpoint.

<div align="center">

## Complete Example Source Code

</div>

For the complete example project, please refer to:

- Rust examples: `examples/Rust/`
- Python examples: `examples/Python/comprehensive_example.py`
- Documentation: `doc/en/05-usage-examples/`

<div align="center">

## Next Steps

</div>

- See [Authentication Example](authentication.md) for JWT and OAuth configuration
- See [Caching Example](caching.md) for cache policies
- See [Service Mesh Example](service_mesh.md) for service discovery
- See [Observability Example](observability.md) for metrics and tracing
