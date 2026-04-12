# Ri CLI Template Guide

This document provides detailed documentation for Ri CLI project templates.

## Table of Contents

- [Overview](#overview)
- [Available Templates](#available-templates)
  - [Minimal Template](#minimal-template)
  - [Web Template](#web-template)
  - [API Template](#api-template)
  - [Worker Template](#worker-template)
  - [Microservice Template](#microservice-template)
- [Template Structure](#template-structure)
- [Template Variables](#template-variables)
- [Custom Templates](#custom-templates)
- [Best Practices](#best-practices)

---

## Overview

Ri CLI provides project templates that generate complete, production-ready project structures. Each template is designed for a specific use case and includes:

- Pre-configured `Cargo.toml` with appropriate dependencies
- Main entry point with module initialization
- Configuration files for different environments
- README with project-specific documentation
- `.gitignore` for Rust projects

---

## Available Templates

### Minimal Template

The minimal template provides the simplest Ri application structure, perfect for learning or simple applications.

**Use Cases**:
- Learning Ri framework basics
- Simple command-line tools
- Prototyping new features
- Minimal overhead applications

**Features**:
- RiAppBuilder for application initialization
- RiLogger for structured logging
- Minimal dependency footprint
- Clean, simple structure

**Create Command**:
```bash
ric new my-minimal --template minimal
```

**Generated Structure**:
```
my-minimal/
├── Cargo.toml
├── src/
│   └── main.rs
├── .gitignore
└── README.md
```

**Dependencies**:
```toml
[dependencies]
ri = { version = "0.1.9" }
tokio = { version = "1", features = ["full"] }
```

**Main Entry Point**:
```rust
use ri::{RiAppBuilder, RiLogger, RiLogConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = RiAppBuilder::new()
        .name("my-minimal")
        .version("0.1.0");

    let log_config = RiLogConfig::default();
    let logger = RiLogger::new(log_config);

    logger.info("Application started");

    // Your application logic here

    tokio::signal::ctrl_c().await?;
    Ok(())
}
```

---

### Web Template

The web template creates a full-featured web application with HTTP server, caching, and authentication.

**Use Cases**:
- Web applications with user interfaces
- Full-stack web services
- Applications requiring authentication
- Projects needing caching

**Features**:
- RiGateway for HTTP routing and middleware
- RiCacheModule for response caching
- RiAuthModule for authentication
- CORS support for cross-origin requests
- TLS/HTTPS support
- Development and production configurations

**Create Command**:
```bash
ric new my-web --template web
```

**Generated Structure**:
```
my-web/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/
│   ├── development.yaml
│   └── production.yaml
├── .gitignore
└── README.md
```

**Dependencies**:
```toml
[dependencies]
ri = { version = "0.1.9" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
log = "0.4"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

**Configuration Files**:

Development (`config/development.yaml`):
```yaml
[server]
bind_address = "127.0.0.1:8080"
workers = 0
debug = true
cors_enabled = true

[cache]
enabled = true
backend = "memory"
default_ttl_secs = 300

[auth]
enabled = true
jwt_secret = "dev-secret-key-change-in-production"
token_expiry_secs = 3600

[logging]
level = "debug"
format = "pretty"
console_enabled = true
```

Production (`config/production.yaml`):
```yaml
[server]
bind_address = "0.0.0.0:8080"
workers = 4
debug = false
cors_enabled = true
cors_origins = ["https://example.com"]

[cache]
enabled = true
backend = "redis"
default_ttl_secs = 3600

[auth]
enabled = true
jwt_secret = "${RI_JWT_SECRET}"
token_expiry_secs = 3600

[logging]
level = "info"
format = "json"
console_enabled = true
file_enabled = true
```

**Key Components**:

1. **Server Configuration**:
```rust
pub struct ServerConfig {
    pub bind_address: String,
    pub workers: usize,
    pub tls_enabled: bool,
    pub request_timeout_secs: u64,
    pub max_body_size: usize,
}
```

2. **Application State**:
```rust
pub struct AppState {
    pub cache: RiCacheModule,
    pub auth: RiAuthModule,
    pub logger: RiLogger,
    pub config: ServerConfig,
}
```

3. **API Response**:
```rust
pub struct ApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}
```

---

### API Template

The API template creates a RESTful API service with validation and structured responses.

**Use Cases**:
- RESTful API backends
- Microservice APIs
- Public API services
- API-first applications

**Features**:
- RiGateway for API routing
- RiValidationModule for request validation
- Standard API response types
- Health check endpoints
- OpenAPI documentation support
- Request/response logging

**Create Command**:
```bash
ric new my-api --template api
```

**Generated Structure**:
```
my-api/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/
│   ├── development.yaml
│   └── production.yaml
├── .gitignore
└── README.md
```

**Key Components**:

1. **Validation Configuration**:
```rust
pub struct ValidationConfig {
    pub enabled: bool,
    pub max_body_size: usize,
    pub strict_mode: bool,
    pub validate_headers: bool,
}
```

2. **Request/Response Types**:
```rust
pub struct ExampleRequest {
    pub name: String,
    pub value: i32,
}

pub struct ExampleResponse {
    pub id: String,
    pub name: String,
    pub value: i32,
    pub processed_at: String,
}
```

3. **API Endpoints**:
```rust
// Health check
pub async fn health_check() -> ApiResponse<HealthStatus>

// Root endpoint
pub async fn root() -> ApiResponse<String>

// Example endpoint
pub async fn example_endpoint(
    request: ExampleRequest,
    state: &AppState,
) -> ApiResponse<ExampleResponse>
```

---

### Worker Template

The worker template creates a background job processing service with queue management.

**Use Cases**:
- Background job processing
- Task queues
- Event-driven processing
- Device control services
- Scheduled tasks

**Features**:
- RiQueueModule for task processing
- RiDeviceControlModule for device management
- Configurable worker pools
- Dead letter queue support
- Task retry mechanisms
- Worker statistics

**Create Command**:
```bash
ric new my-worker --template worker
```

**Generated Structure**:
```
my-worker/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/
│   ├── development.yaml
│   └── production.yaml
├── .gitignore
└── README.md
```

**Key Components**:

1. **Worker Configuration**:
```rust
pub struct WorkerConfig {
    pub consumer_id: String,
    pub worker_count: usize,
    pub task_timeout_secs: u64,
    pub max_retries: u32,
    pub enable_priority: bool,
}
```

2. **Queue Configuration**:
```rust
pub struct QueueConfig {
    pub backend: String,        // "rabbitmq", "redis", "kafka", "memory"
    pub url: String,
    pub queue_name: String,
    pub dead_letter_enabled: bool,
    pub dead_letter_queue: String,
}
```

3. **Task Types**:
```rust
pub struct Task {
    pub id: String,
    pub task_type: String,
    pub payload: serde_json::Value,
    pub priority: i32,
    pub created_at: String,
    pub retry_count: u32,
}

pub struct TaskResult {
    pub task_id: String,
    pub status: String,         // "success", "failed", "retry"
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
    pub completed_at: String,
    pub duration_ms: u64,
}
```

4. **Worker Statistics**:
```rust
pub struct WorkerStats {
    pub tasks_processed: u64,
    pub tasks_succeeded: u64,
    pub tasks_failed: u64,
    pub tasks_retried: u64,
}
```

---

### Microservice Template

The microservice template creates a distributed service with service mesh and observability.

**Use Cases**:
- Distributed microservices
- Cloud-native applications
- Service mesh deployments
- Production-grade services

**Features**:
- RiServiceMesh for service discovery
- RiObservabilityModule for metrics and tracing
- Distributed tracing support
- Health check server
- Load balancing
- Circuit breaker

**Create Command**:
```bash
ric new my-service --template microservice
```

**Generated Structure**:
```
my-service/
├── Cargo.toml
├── src/
│   └── main.rs
├── config/
│   ├── development.yaml
│   └── production.yaml
├── .gitignore
└── README.md
```

**Key Components**:

1. **Service Configuration**:
```rust
pub struct ServiceConfig {
    pub service_name: String,
    pub service_version: String,
    pub instance_id: String,
    pub bind_address: String,
    pub workers: usize,
    pub tls_enabled: bool,
}
```

2. **Service Mesh Configuration**:
```rust
pub struct MeshConfig {
    pub enabled: bool,
    pub registry_url: String,
    pub namespace: String,
    pub load_balancing: bool,
    pub lb_strategy: String,        // "round_robin", "random", "least_connections"
    pub circuit_breaker: bool,
    pub cb_failure_threshold: u32,
    pub cb_timeout_secs: u64,
    pub service_discovery: bool,
    pub health_check_interval_secs: u64,
}
```

3. **Observability Configuration**:
```rust
pub struct ObservabilityConfig {
    pub metrics_enabled: bool,
    pub metrics_path: String,
    pub tracing_enabled: bool,
    pub tracing_endpoint: String,
    pub sampling_rate: f32,
    pub logging_enabled: bool,
    pub log_level: String,
    pub log_format: String,
}
```

4. **Service Response with Tracing**:
```rust
pub struct ServiceResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub error: Option<String>,
    pub trace_id: String,
    pub timestamp: String,
    pub instance_id: String,
}
```

---

## Template Structure

Each template follows a consistent structure:

### File Organization

```
template-name/
├── Cargo.toml.tmpl         # Package configuration template
├── src/
│   └── main.rs.tmpl        # Main entry point template
├── config/                 # Configuration files (optional)
│   ├── development.yaml.tmpl
│   └── production.yaml.tmpl
├── .gitignore.tmpl         # Git ignore rules
└── README.md.tmpl          # Project documentation
```

### Template Processing

1. **Variable Substitution**: Template variables are replaced with actual values
2. **File Generation**: Templates are rendered and written to the project directory
3. **Directory Creation**: Required directories are created automatically
4. **Git Initialization**: A git repository is initialized with initial commit

---

## Template Variables

The following variables are available in all templates:

| Variable | Description | Example |
|----------|-------------|---------|
| `{{ project_name }}` | Project name | `my-project` |
| `{{ project_version }}` | Initial version | `0.1.0` |
| `{{ author }}` | Author name from git config | `John Doe` |
| `{{ ri_version }}` | Ri framework version | `0.1.9` |
| `{{ date }}` | Current date | `2025-04-12` |

### Template-Specific Variables

**Web Template**:
| Variable | Description | Default |
|----------|-------------|---------|
| `{{ port }}` | Server port | `8080` |
| `{{ workers }}` | Worker threads | `4` |
| `{{ enable_tls }}` | TLS enabled | `false` |
| `{{ enable_cors }}` | CORS enabled | `true` |

**API Template**:
| Variable | Description | Default |
|----------|-------------|---------|
| `{{ api_version }}` | API version | `v1` |
| `{{ enable_docs }}` | API docs enabled | `true` |
| `{{ enable_auth }}` | Auth enabled | `false` |

**Worker Template**:
| Variable | Description | Default |
|----------|-------------|---------|
| `{{ queue_type }}` | Queue backend | `memory` |
| `{{ max_workers }}` | Max workers | `4` |
| `{{ enable_persistence }}` | Persistence enabled | `false` |

**Microservice Template**:
| Variable | Description | Default |
|----------|-------------|---------|
| `{{ grpc_port }}` | gRPC port | `50051` |
| `{{ enable_reflection }}` | gRPC reflection | `true` |
| `{{ enable_health_check }}` | Health check | `true` |

---

## Custom Templates

### Creating Custom Templates

You can create custom templates for your organization or specific use cases.

1. **Create Template Directory**:
```bash
mkdir -p templates/my-template
```

2. **Create Template Files**:
```
templates/my-template/
├── Cargo.toml.tmpl
├── src/
│   └── main.rs.tmpl
├── .gitignore.tmpl
└── README.md.tmpl
```

3. **Use Template Variables**:
```toml
# Cargo.toml.tmpl
[package]
name = "{{ project_name }}"
version = "{{ project_version }}"
authors = ["{{ author }}"]
edition = "2021"

[dependencies]
ri = { version = "{{ ri_version }}" }
```

4. **Register Template** (requires code modification):
Add the template to the template engine in `src/templates/mod.rs`.

### Template Best Practices

1. **Use Meaningful Defaults**: Provide sensible default values
2. **Document Variables**: Comment on what each variable does
3. **Include README**: Provide project-specific documentation
4. **Add Configuration**: Include development and production configs
5. **Test Templates**: Verify generated projects compile and run

---

## Best Practices

### Choosing a Template

| Use Case | Recommended Template |
|----------|---------------------|
| Learning Ri | Minimal |
| Simple CLI tool | Minimal |
| Web application | Web |
| RESTful API | API |
| Background jobs | Worker |
| Distributed service | Microservice |
| Event-driven service | Worker |
| Cloud-native service | Microservice |

### Template Customization

After creating a project, customize it for your needs:

1. **Update Dependencies**: Add/remove dependencies in `Cargo.toml`
2. **Configure Modules**: Adjust module settings in config files
3. **Add Routes**: Define your API endpoints
4. **Implement Handlers**: Write your business logic
5. **Add Tests**: Create test modules for your code

### Configuration Management

1. **Environment-Specific Configs**: Use different configs for dev/staging/prod
2. **Environment Variables**: Use env vars for sensitive data
3. **Config Validation**: Always validate configuration on startup
4. **Default Values**: Provide sensible defaults for optional settings

### Security Considerations

1. **Change Default Secrets**: Never use default JWT secrets in production
2. **Use Environment Variables**: Store secrets in environment variables
3. **Enable TLS**: Use HTTPS in production environments
4. **Restrict CORS**: Limit CORS origins to trusted domains
5. **Validate Input**: Always validate incoming requests
