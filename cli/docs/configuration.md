# Ri CLI Configuration Guide

This document provides detailed documentation for Ri CLI configuration management.

## Table of Contents

- [Configuration Overview](#configuration-overview)
- [Configuration File Format](#configuration-file-format)
- [Configuration Structure](#configuration-structure)
- [Module Configuration](#module-configuration)
- [Environment Variables](#environment-variables)
- [Configuration Validation](#configuration-validation)
- [Best Practices](#best-practices)

---

## Configuration Overview

Ri CLI uses YAML configuration files to manage project settings. Configuration files define:

- Project metadata (name, version, template)
- Build settings (release mode, target, features)
- Runtime settings (log level, workers)
- Module configurations (cache, queue, gateway, auth, etc.)

### Configuration File Locations

| Location | Priority | Description |
|----------|----------|-------------|
| `./ric.yaml` | Highest | Project configuration in current directory |
| `$RI_CONFIG_PATH` | Medium | Custom path via environment variable |
| `~/.config/ric/config.yaml` | Lowest | User-level default configuration |

### Configuration Hierarchy

Configuration values are merged in the following order (later values override earlier):

1. **Code Defaults**: Default values defined in source code
2. **User Configuration**: `~/.config/ric/config.yaml`
3. **Project Configuration**: `./ric.yaml` or `$RI_CONFIG_PATH`
4. **Environment Variables**: Highest priority

---

## Configuration File Format

Ri CLI supports YAML format for configuration files.

### Basic Syntax

```yaml
# Comments start with #
key: value

# Nested structures
section:
  key: value
  nested:
    key: value

# Lists
items:
  - item1
  - item2
  - item3

# Inline lists
items: [item1, item2, item3]

# Multi-line strings
description: |
  This is a multi-line
  description that spans
  multiple lines.
```

### Type Support

| Type | Example |
|------|---------|
| String | `name: "my-project"` |
| Integer | `workers: 4` |
| Float | `sampling_rate: 0.5` |
| Boolean | `enabled: true` |
| List | `features: [default, cache]` |
| Map | `cache: { enabled: true }` |
| Null | `optional: null` |

---

## Configuration Structure

### Complete Configuration Example

```yaml
# =============================================================================
# Project Configuration
# =============================================================================
project:
  name: my-project
  version: 0.1.0
  template: web
  description: A Ri web application
  author: Your Name

# =============================================================================
# Build Configuration
# =============================================================================
build:
  release: false
  target: all
  features:
    - default
  strip: true
  lto: false

# =============================================================================
# Runtime Configuration
# =============================================================================
runtime:
  log_level: info
  workers: 4
  shutdown_timeout_secs: 30

# =============================================================================
# Module Configurations
# =============================================================================

# Cache Module
cache:
  enabled: true
  backend_type: Memory
  default_ttl_secs: 3600
  max_memory_mb: 512
  cleanup_interval_secs: 300
  redis_url: redis://localhost:6379
  redis_pool_size: 10

# Queue Module
queue:
  enabled: true
  backend_type: Memory
  connection_string: memory://localhost
  max_connections: 10
  message_max_size: 1048576
  consumer_timeout_ms: 30000
  producer_timeout_ms: 5000
  retry_policy:
    max_retries: 3
    initial_delay_ms: 100
    max_delay_ms: 5000
    multiplier: 2.0
  dead_letter_config:
    enabled: true
    queue_name: dead-letter

# Gateway Module
gateway:
  listen_address: "0.0.0.0"
  listen_port: 8080
  max_connections: 10000
  request_timeout_seconds: 30
  enable_rate_limiting: true
  enable_circuit_breaker: true
  enable_load_balancing: true
  cors_enabled: true
  cors_origins:
    - "https://example.com"
    - "https://api.example.com"
  cors_methods:
    - GET
    - POST
    - PUT
    - DELETE
    - OPTIONS
  cors_headers:
    - Content-Type
    - Authorization
  enable_logging: true
  log_level: info

# Auth Module
auth:
  enabled: true
  jwt_secret: "${RI_JWT_SECRET}"
  jwt_expiry_secs: 3600
  session_timeout_secs: 86400
  oauth_providers:
    - google
    - github
  enable_api_keys: true
  enable_session_auth: true

# Device Control Module
device:
  discovery_enabled: true
  discovery_interval_secs: 30
  auto_scheduling_enabled: true
  max_concurrent_tasks: 100
  resource_allocation_timeout_secs: 60
  enable_cpu_discovery: true
  enable_gpu_discovery: true
  enable_memory_discovery: true
  enable_storage_discovery: true
  enable_network_discovery: true

# Observability Module
observability:
  tracing_enabled: true
  metrics_enabled: true
  tracing_sampling_rate: 0.1
  tracing_sampling_strategy: rate
  metrics_window_size_secs: 300
  metrics_bucket_size_secs: 10

# =============================================================================
# Logging Configuration
# =============================================================================
logging:
  level: info
  format: json
  console_enabled: true
  file_enabled: false
  file_path: ./logs/app.log
  max_file_size_mb: 100
  max_files: 5
  include_source: true
  include_thread_id: true

# =============================================================================
# Database Configuration (Optional)
# =============================================================================
database:
  enabled: false
  type: postgres
  url: postgresql://user:password@localhost:5432/mydb
  pool_size: 10
  log_queries: true

# =============================================================================
# Health Check Configuration
# =============================================================================
health:
  enabled: true
  path: /health
  detailed: true
  timeout_secs: 5
```

---

## Module Configuration

### Cache Module Configuration

The cache module provides high-performance caching with multiple backend support.

```yaml
cache:
  # Enable/disable caching
  enabled: true

  # Backend type: Memory, Redis, Hybrid
  backend_type: Memory

  # Default TTL for cached items (seconds)
  default_ttl_secs: 3600
  # Range: 1 - 86400 (1 second to 24 hours)

  # Maximum memory usage (megabytes)
  max_memory_mb: 512
  # Range: 1 - 102400 (1 MB to 100 GB)

  # Cleanup interval for expired entries (seconds)
  cleanup_interval_secs: 300
  # Range: 10 - 3600 (10 seconds to 1 hour)

  # Redis configuration (if backend_type is Redis or Hybrid)
  redis_url: redis://localhost:6379
  redis_pool_size: 10
  # Pool size range: 1 - 100
```

**Backend Types**:

| Type | Description | Use Case |
|------|-------------|----------|
| `Memory` | In-memory cache | Development, single-instance apps |
| `Redis` | Redis-backed cache | Production, distributed systems |
| `Hybrid` | Memory + Redis | High-performance with fallback |

### Queue Module Configuration

The queue module provides message queue functionality for async processing.

```yaml
queue:
  # Enable/disable queue
  enabled: true

  # Backend type: Memory, RabbitMQ, Kafka, Redis
  backend_type: Memory

  # Connection string
  connection_string: memory://localhost
  # RabbitMQ: amqp://guest:guest@localhost:5672/
  # Kafka: localhost:9092
  # Redis: redis://localhost:6379

  # Connection settings
  max_connections: 10
  # Range: 1 - 1000

  message_max_size: 1048576
  # Range: 1024 - 104857600 (1 KB to 100 MB)

  consumer_timeout_ms: 30000
  # Range: 1000 - 300000 (1 second to 5 minutes)

  producer_timeout_ms: 5000
  # Range: 100 - 60000 (100 ms to 1 minute)

  # Retry policy
  retry_policy:
    max_retries: 3
    initial_delay_ms: 100
    max_delay_ms: 5000
    multiplier: 2.0

  # Dead letter queue
  dead_letter_config:
    enabled: true
    queue_name: dead-letter
```

**Backend Types**:

| Type | Description | Use Case |
|------|-------------|----------|
| `Memory` | In-memory queue | Development, testing |
| `RabbitMQ` | RabbitMQ broker | Production, reliable messaging |
| `Kafka` | Apache Kafka | High-throughput, streaming |
| `Redis` | Redis streams | Simple queue, existing Redis infra |

### Gateway Module Configuration

The gateway module provides HTTP server and API gateway functionality.

```yaml
gateway:
  # Network settings
  listen_address: "0.0.0.0"
  listen_port: 8080
  # Port range: 1 - 65535 (ports < 1024 require root)

  # Connection settings
  max_connections: 10000
  # Range: 1 - 1000000

  request_timeout_seconds: 30
  # Range: 1 - 3600 (1 second to 1 hour)

  # Feature toggles
  enable_rate_limiting: true
  enable_circuit_breaker: true
  enable_load_balancing: true

  # CORS configuration
  cors_enabled: true
  cors_origins:
    - "https://example.com"
  cors_methods:
    - GET
    - POST
  cors_headers:
    - Content-Type
    - Authorization

  # Logging
  enable_logging: true
  log_level: info
  # Options: trace, debug, info, warn, error
```

### Auth Module Configuration

The auth module provides authentication and authorization functionality.

```yaml
auth:
  # Enable/disable authentication
  enabled: true

  # JWT settings
  jwt_secret: "${RI_JWT_SECRET}"
  # IMPORTANT: Use environment variable in production!
  # Minimum length: 16 characters

  jwt_expiry_secs: 3600
  # Range: 60 - 604800 (1 minute to 7 days)

  session_timeout_secs: 86400
  # Range: 60 - 2592000 (1 minute to 30 days)

  # OAuth providers
  oauth_providers:
    - google
    - github
  # Requires environment variables:
  # RI_OAUTH_GOOGLE_CLIENT_ID, RI_OAUTH_GOOGLE_CLIENT_SECRET
  # RI_OAUTH_GITHUB_CLIENT_ID, RI_OAUTH_GITHUB_CLIENT_SECRET

  # Feature toggles
  enable_api_keys: true
  enable_session_auth: true
```

### Device Control Module Configuration

The device control module provides IoT device management functionality.

```yaml
device:
  # Discovery settings
  discovery_enabled: true
  discovery_interval_secs: 30
  # Range: 5 - 3600 (5 seconds to 1 hour)

  # Scheduling settings
  auto_scheduling_enabled: true
  max_concurrent_tasks: 100
  # Range: 1 - 10000

  resource_allocation_timeout_secs: 60
  # Range: 10 - 600 (10 seconds to 10 minutes)

  # Device type discovery
  enable_cpu_discovery: true
  enable_gpu_discovery: true
  enable_memory_discovery: true
  enable_storage_discovery: true
  enable_network_discovery: true
```

### Observability Module Configuration

The observability module provides metrics and tracing functionality.

```yaml
observability:
  # Tracing settings
  tracing_enabled: true
  tracing_sampling_rate: 0.1
  # Range: 0.0 - 1.0 (0% to 100%)
  # Warning: > 0.5 may impact performance

  tracing_sampling_strategy: rate
  # Options: rate, probability, always, never

  # Metrics settings
  metrics_enabled: true
  metrics_window_size_secs: 300
  # Range: 10 - 3600 (10 seconds to 1 hour)

  metrics_bucket_size_secs: 10
  # Range: 1 - 60 (1 second to 1 minute)
  # Must divide evenly into window_size_secs
```

---

## Environment Variables

### Core Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `RI_CONFIG_PATH` | Custom configuration file path | `/etc/ric/config.yaml` |
| `RI_LOG_LEVEL` | Override log level | `debug` |
| `RUST_LOG` | Rust logging configuration | `ri=debug,info` |
| `CARGO_HOME` | Cargo home directory | `~/.cargo` |
| `RUSTUP_HOME` | Rustup home directory | `~/.rustup` |

### Security Environment Variables

| Variable | Description | Required For |
|----------|-------------|--------------|
| `RI_JWT_SECRET` | JWT signing secret | Auth module |
| `RI_OAUTH_GOOGLE_CLIENT_ID` | Google OAuth client ID | Google OAuth |
| `RI_OAUTH_GOOGLE_CLIENT_SECRET` | Google OAuth secret | Google OAuth |
| `RI_OAUTH_GITHUB_CLIENT_ID` | GitHub OAuth client ID | GitHub OAuth |
| `RI_OAUTH_GITHUB_CLIENT_SECRET` | GitHub OAuth secret | GitHub OAuth |

### Using Environment Variables

**In Configuration**:
```yaml
auth:
  jwt_secret: "${RI_JWT_SECRET}"
```

**Setting Environment Variables**:
```bash
# Linux/macOS
export RI_JWT_SECRET="your-secure-secret-key"

# Windows (PowerShell)
$env:RI_JWT_SECRET = "your-secure-secret-key"

# Docker
docker run -e RI_JWT_SECRET="your-secure-secret-key" my-app

# Kubernetes
env:
  - name: RI_JWT_SECRET
    valueFrom:
      secretKeyRef:
        name: app-secrets
        key: jwt-secret
```

---

## Configuration Validation

### Validation Commands

```bash
# Validate default configuration
ric config validate

# Validate specific file
ric config validate path/to/config.yaml

# Check environment variables
ric config check
```

### Validation Rules

#### Cache Module

| Field | Rule | Error Level |
|-------|------|-------------|
| `backend_type` | Must be Memory, Redis, or Hybrid | Error |
| `default_ttl_secs` | 1 - 86400 | Error |
| `default_ttl_secs` | Should not exceed 86400 | Warning |
| `redis_url` | Required when backend is Redis/Hybrid | Error |
| `redis_url` | Must match pattern `redis://.*` | Error |

#### Queue Module

| Field | Rule | Error Level |
|-------|------|-------------|
| `backend_type` | Must be Memory, RabbitMQ, Kafka, or Redis | Error |
| `connection_string` | Required for non-Memory backends | Error |
| `max_retries` | Must be non-negative | Error |

#### Gateway Module

| Field | Rule | Error Level |
|-------|------|-------------|
| `listen_port` | 1 - 65535 | Error |
| `listen_port` | < 1024 requires root | Warning |
| `log_level` | Must be trace/debug/info/warn/error | Error |
| `cors_origins` | Recommended when CORS enabled | Warning |

#### Auth Module

| Field | Rule | Error Level |
|-------|------|-------------|
| `jwt_secret` | Minimum 16 characters | Warning |
| `jwt_secret` | Cannot be default value | Error |
| `jwt_expiry_secs` | Minimum 60 seconds | Error |
| `jwt_expiry_secs` | > 86400 may pose security risks | Warning |

### Validation Output

**Success**:
```
Validating configuration...
✅ Configuration is valid
```

**Errors**:
```
Validating configuration...
❌ Configuration has errors:

  • cache.default_ttl_secs: must be at least 1 second
    Suggestion: Set default_ttl_secs to at least 1

  • gateway.listen_port: must be between 1 and 65535
    Suggestion: Use a valid port number between 1 and 65535

  • auth.jwt_secret: appears to be a default or weak value
    Suggestion: Use a cryptographically secure random secret
```

**Warnings**:
```
Validating configuration...
⚠️  Configuration has warnings:

  • cache.default_ttl_secs: should not exceed 86400 seconds (24 hours)
    Suggestion: Consider using a smaller TTL value

  • gateway.listen_port: ports below 1024 require root privileges
    Suggestion: Consider using a port above 1024 (e.g., 8080)
```

---

## Best Practices

### Development Configuration

1. **Use Debug Logging**: Enable debug level for visibility
2. **Enable CORS**: Allow localhost origins
3. **Use Memory Backends**: Simplify local development
4. **Disable TLS**: Skip HTTPS for local testing
5. **Short Timeouts**: Faster feedback during development

```yaml
# Development
logging:
  level: debug
  format: pretty

gateway:
  cors_enabled: true
  cors_origins: ["http://localhost:3000"]

cache:
  backend_type: Memory

auth:
  jwt_secret: "dev-secret-key-change-in-production"
```

### Production Configuration

1. **Use Info Logging**: Balance visibility and performance
2. **Restrict CORS**: Limit to trusted domains
3. **Use Redis Backends**: For distributed caching
4. **Enable TLS**: Secure all connections
5. **Use Environment Variables**: For secrets
6. **Longer Timeouts**: For production workloads

```yaml
# Production
logging:
  level: info
  format: json
  file_enabled: true

gateway:
  cors_enabled: true
  cors_origins: ["https://example.com"]

cache:
  backend_type: Redis
  redis_url: "${REDIS_URL}"

auth:
  jwt_secret: "${RI_JWT_SECRET}"
```

### Security Best Practices

1. **Never Commit Secrets**: Use environment variables
2. **Rotate Secrets Regularly**: Change JWT secrets periodically
3. **Use Strong Secrets**: Minimum 32 characters, random
4. **Restrict CORS**: Only allow necessary origins
5. **Enable Rate Limiting**: Protect against abuse
6. **Use HTTPS**: Enable TLS in production

### Performance Best Practices

1. **Tune Worker Count**: Match CPU cores
2. **Configure Connection Pools**: Right-size pools
3. **Set Appropriate TTLs**: Balance freshness and performance
4. **Enable Compression**: Reduce bandwidth
5. **Use Sampling**: For tracing in production

### Configuration Management

1. **Version Control**: Track configuration changes
2. **Environment Parity**: Keep dev/staging/prod similar
3. **Document Changes**: Comment configuration files
4. **Validate Early**: Run validation in CI/CD
5. **Use Templates**: Standardize configurations
