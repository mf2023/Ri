<div align="center">

# Config API Reference

**Version: 0.1.7**

**Last modified date: 2026-02-11**

The config module provides multi-source configuration management and hot reload functionality, supporting multiple configuration sources such as files and environment variables.

## Module Overview

</div>

The config module includes the following sub-modules:

- **core**: Configuration core interfaces and type definitions
- **sources**: Configuration source implementations (files, environment variables, etc.)
- **validators**: Configuration validators
- **reload**: Configuration hot reload mechanism

<div align="center">

## Core Components

</div>

### DMSCConfig

Configuration manager main interface, providing unified configuration access.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new config | None | `DMSCConfig` |
| `get(key)` | Get configuration value | `key: &str` | `String` or `None` |
| `get_f64(key)` | Get f64 value | `key: &str` | `f64` |
| `get_usize(key)` | Get usize value | `key: &str` | `usize` |
| `set(key, value)` | Set configuration value | `key: &str`, `value: &str` | None |
| `contains(key)` | Check if key exists | `key: &str` | `bool` |
| `keys()` | Get all keys | None | `Vec<String>` |
| `values()` | Get all values | None | `Vec<String>` |
| `len()` | Get config count | None | `usize` |
| `is_empty()` | Check if empty | None | `bool` |
| `merge(other)` | Merge configs | `other: &DMSCConfig` | None |
| `clear()` | Clear config | None | None |

#### Usage Example

```rust
use dmsc::config::DMSCConfig;

// Create config
let config = DMSCConfig::new();

// Set config
config.set("service.port", "8080");
config.set("database.url", "postgres://localhost/mydb");

// Get config
let port = config.get("service.port");
let url = config.get("database.url");

// Check config
if config.contains("service.host") {
    let host = config.get("service.host");
}

// Get all keys
let keys = config.keys();
for key in &keys {
    println!("Config key: {}", key);
}

// Merge configs
let other = DMSCConfig::new();
other.set("additional", "value");
config.merge(&other);
```

### DMSCConfigManager

Configuration manager with multi-source support.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new config manager | None | `DMSCConfigManager` |
| `add_file_source(path)` | Add file source | `path: &str` | None |
| `add_environment_source()` | Add environment source | None | None |
| `get(key)` | Get config value | `key: &str` | `String` or `None` |

### DMSCConfigSource

Configuration source enum type.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `File(path)` | File configuration source |
| `Env(prefix)` | Environment variable configuration source |
| `Http(url)` | HTTP configuration source |
| `Database(connection)` | Database configuration source |
| `Custom(name, data)` | Custom configuration source |

### DMSCConfigBuilder

Configuration builder, used to build configuration manager.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new configuration builder | None | `Self` |
| `add_source(source)` | Add configuration source | `source: DMSCConfigSource` | `Self` |
| `set_default(key, value)` | Set default value | `key: &str`, `value: impl Serialize` | `Self` |
| `add_validator(validator)` | Add validator | `validator: impl ConfigValidator` | `Self` |
| `enable_hot_reload()` | Enable hot reload | None | `Self` |
| `set_reload_interval(seconds)` | Set reload interval | `seconds: u64` | `Self` |
| `build()` | Build configuration manager | None | `DMSCResult<DMSCConfig>` |

#### Usage Example

```rust
use dmsc::prelude::*;

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))
    .set_default("service.port", 8080)
    .set_default("service.host", "localhost")
    .enable_hot_reload()
    .set_reload_interval(60)
    .build()?;
```

## Configuration Sources

### File Configuration

#### YAML File

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"
  port: 8080
  host: "0.0.0.0"

database:
  url: "postgres://localhost/mydb"
  max_connections: 100
  timeout: 30

logging:
  level: "info"
  file_format: "json"
```

#### JSON File

```json
{
  "service": {
    "name": "my-service",
    "version": "1.0.0",
    "port": 8080,
    "host": "0.0.0.0"
  },
  "database": {
    "url": "postgres://localhost/mydb",
    "max_connections": 100,
    "timeout": 30
  }
}
```

#### TOML File

```toml
[service]
name = "my-service"
version = "1.0.0"
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://localhost/mydb"
max_connections = 100
timeout = 30
```

### Environment Variable Configuration

```bash
# Basic environment variables
export SERVICE_NAME=my-service
export SERVICE_PORT=8080
export DATABASE_URL=postgres://localhost/mydb

# Environment variables with prefix
export DMSC_SERVICE_NAME=my-service
export DMSC_SERVICE_PORT=8080
export DMSC_DATABASE_URL=postgres://localhost/mydb
```

### Configuration Priority

Configuration source priority from high to low:

1. **Environment Variables** (highest priority)
2. **Configuration Files** (medium priority)
3. **Default Values** (lowest priority)

```rust
let config = DMSCConfigBuilder::new()
    .set_default("service.port", 3000)                    // Default value
    .add_source(DMSCConfigSource::File("config.yaml".to_string())) // Configuration file
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))        // Environment variable
    .build()?;

// Priority: Environment variables > Configuration files > Default values
```
<div align="center">

## Type-Safe Configuration Access

</div>

### Basic Types

```rust
// String
let name: String = ctx.config().get_typed("service.name")?;

// Numeric types
let port: u16 = ctx.config().get_typed("service.port")?;
let max_connections: usize = ctx.config().get_typed("database.max_connections")?;
let timeout: f64 = ctx.config().get_typed("service.timeout")?;

// Boolean types
let debug_mode: bool = ctx.config().get_typed("service.debug")?;
let enable_feature: bool = ctx.config().get_typed("feature.enabled")?;
```

### Complex Types

```rust
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: usize,
    timeout: u64,
}

// Get struct configuration
let db_config: DatabaseConfig = ctx.config().get_typed("database")?;

// Get array configuration
let allowed_hosts: Vec<String> = ctx.config().get_typed("security.allowed_hosts")?;

// Get map configuration
let feature_flags: HashMap<String, bool> = ctx.config().get_typed("features")?;
```

### Optional Types

```rust
// Get optional configuration
let port: Option<u16> = ctx.config().get_typed("service.port").ok();
let timeout: Option<u64> = ctx.config().get_typed("service.timeout").ok();

// Use unwrap_or to provide default value
let port = ctx.config().get_typed("service.port").unwrap_or(8080);
let timeout = ctx.config().get_typed("service.timeout").unwrap_or(30);
```
<div align="center">

## Configuration Validation

</div>  

### Built-in Validators

```rust
use dmsc::prelude::*;

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_validator(RequiredValidator::new(vec![
        "service.name",
        "service.port",
        "database.url",
    ]))
    .add_validator(RangeValidator::new("service.port", 1, 65535))
    .add_validator(RegexValidator::new("service.host", r"^[a-zA-Z0-9.-]+$"))
    .build()?;
```

### Custom Validators

```rust
use dmsc::prelude::*;

struct CustomValidator;

impl ConfigValidator for CustomValidator {
    fn validate(&self, config: &DMSCConfig) -> DMSCResult<()> {
        let port: u16 = config.get_typed("service.port")?;
        let host: String = config.get_typed("service.host")?;
        
        if port < 1024 && host != "localhost" {
            return Err(DMSCError::new("INVALID_CONFIG", "Privileged ports require localhost"));
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "custom_validator"
    }
}

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_validator(CustomValidator)
    .build()?;
```

<div align="center">

## Hot Reload

</div>  

### Enable Hot Reload

```rust
let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .enable_hot_reload()
    .set_reload_interval(60) // Check every 60 seconds
    .build()?;

// Manually trigger reload
config.reload().await?;
```

### Watch Configuration Changes

```rust
// Watch specific configuration changes
config.watch("service.port", |new_value| {
    println!("Port changed to: {}", new_value);
    // Restart service or update configuration
}).await?;

// Watch all configuration changes
config.watch_all(|changes| {
    for (key, old_value, new_value) in changes {
        println!("Config {} changed from {:?} to {:?}", key, old_value, new_value);
    }
}).await?;
```

<div align="center">

## Configuration Templates

</div>  

### Environment-Specific Configuration

```rust
// config.dev.yaml
service:
  name: "my-service-dev"
  port: 3000
  debug: true

// config.prod.yaml  
service:
  name: "my-service"
  port: 8080
  debug: false
```

### Configuration Inheritance

```yaml
# base.yaml
service:
  name: "my-service"
  version: "1.0.0"

# config.yaml
import: "base.yaml"
service:
  port: 8080  # Override base configuration
```
<div align="center">

## Configuration Encryption

</div>  

### Sensitive Information Encryption

```rust
// Encrypt configuration value
let encrypted_value = encrypt_config_value("secret-password", &encryption_key)?;
config.set("database.password", encrypted_value)?;

// Decrypt configuration value
let decrypted_value = decrypt_config_value(&encrypted_value, &encryption_key)?;
```

### Using Key Management Services

```rust
// Get configuration from AWS Secrets Manager
let secret_config = get_secret_from_aws("my-service/config").await?;
config.merge(secret_config)?;

// Get configuration from HashiCorp Vault
let vault_config = get_secret_from_vault("secret/my-service").await?;
config.merge(vault_config)?;
```

<div align="center">

## Configuration Debugging

</div>      

### Configuration Information

```rust
// Get configuration information
let info = config.get_info()?;
println!("Config sources: {:?}", info.sources);
println!("Last reload: {:?}", info.last_reload);
println!("Total keys: {}", info.total_keys);

// Export configuration
let exported = config.export()?;
println!("Current config: {}", exported);
```

### Configuration Diff

```rust
// Compare configuration differences
let diff = config.compare_with_file("new-config.yaml")?;
for change in diff {
    match change.change_type {
        ConfigChangeType::Added => println!("Added: {}", change.key),
        ConfigChangeType::Modified => println!("Modified: {} ({} -> {})", change.key, change.old_value, change.new_value),
        ConfigChangeType::Removed => println!("Removed: {}", change.key),
    }
}
```

<div align="center">

## Error Handling

</div>  

### Configuration Error Codes

| Error Code | Description |
|:--------|:-------------|
| `CONFIG_FILE_NOT_FOUND` | Configuration file not found |
| `CONFIG_PARSE_ERROR` | Configuration parsing error |
| `CONFIG_VALIDATION_FAILED` | Configuration validation failed |
| `CONFIG_TYPE_ERROR` | Configuration type error |
| `CONFIG_SOURCE_ERROR` | Configuration source error |

### Error Handling Example

```rust
match ctx.config().get_typed::<u16>("service.port") {
    Ok(port) => {
        // Configuration is correct
        println!("Service port: {}", port);
    }
    Err(DMSCError { code, .. }) if code == "CONFIG_TYPE_ERROR" => {
        // Type error, use default value
        let port: u16 = 8080;
        println!("Using default port: {}", port);
    }
    Err(e) => {
        // Other errors
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>  

1. **Use type-safe configuration access**: Avoid manual type conversion
2. **Provide reasonable default values**: Ensure application can run normally when configuration is missing
3. **Validate configuration integrity**: Validate all required configurations at application startup
4. **Use environment variable overrides**: Use environment variables to override configuration files in different environments
5. **Enable hot reload**: Enable hot reload for configurations that need dynamic adjustment
6. **Encrypt sensitive information**: Encrypt passwords, keys, and other sensitive information
7. **Use configuration templates**: Create configuration templates for different environments
8. **Log configuration changes**: Monitor and log configuration changes for auditing
9. **Pay attention to safe timing for configuration modifications**:
   - **Startup phase**: All configurations can be safely modified
   - **Runtime phase**: Only configurations marked as "dynamically modifiable" can be safely modified
   - **Sensitive modules**: Configuration modifications for core modules like gateway, authentication, and service mesh require special caution
   - **Restart requirements**: Some configuration modifications require service restart to take effect

<div align="center">

## Safe Timing for Configuration Modifications

</div>  

### Configurations That Can Be Safely Dynamically Modified

The following types of configurations can usually be safely modified at runtime:

- **Log levels**: Can dynamically adjust log output levels
- **Monitoring configuration**: Can dynamically adjust monitoring sampling rates and alert thresholds
- **Timeout settings**: Can dynamically adjust request timeout times
- **Rate limiting configuration**: Can dynamically adjust rate limits
- **Cache configuration**: Can dynamically adjust cache size and TTL
- **Feature flags**: Can dynamically enable or disable features

### Configurations That Require Cautious Modification

The following types of configuration modifications require special caution and may affect system stability:

- **Authentication configuration**: May cause users to be unable to log in or permissions to become invalid
- **Database configuration**: May cause database connection interruption
- **Network configuration**: May cause inter-service communication interruption
- **Security configuration**: May cause security vulnerabilities
- **Core component configuration**: May cause system crash

### Best Timing for Configuration Modifications

1. **Before application startup**: Modify all configurations that require restart to take effect
2. **Off-peak hours**: Modify configurations when system load is low
3. **Gradual modification**: For critical configurations, test in non-production environment first, then gradually promote to production environment
4. **Monitor modifications**: Closely monitor system metrics after modifying configurations
5. **Rollback mechanism**: Prepare configuration rollback plan to quickly restore in case of problems

### Impact Scope of Configuration Modifications

| Configuration Type | Impact Scope | Requires Restart |
|:--------|:-------------|:--------|
| Log levels | Global | No |
| Monitoring configuration | Global | No |
| Timeout settings | Local | No |
| Rate limiting configuration | Local | No |
| Cache configuration | Local | No |
| Feature flags | Local/Global | No |
| Authentication configuration | Global | Yes |
| Database configuration | Global | Yes |
| Network configuration | Global | Yes |
| Security configuration | Global | Yes |
| Core component configuration | Global | Yes |

<div align="center">

## Configuration File Reference

</div>

This section provides detailed documentation for all configuration structures used in DMSC applications.

### Complete Configuration File Example

```yaml
# config.yaml - Complete DMSC Configuration File

# =============================================================================
# Authentication Configuration
# =============================================================================
auth:
  enabled: true                          # Enable authentication
  jwt_secret: "your-secret-key"          # JWT secret (use env var in production)
  jwt_expiry_secs: 3600                  # JWT token expiry (1 hour)
  session_timeout_secs: 86400            # Session timeout (24 hours)
  oauth_providers: []                    # OAuth providers: ["google", "github"]
  enable_api_keys: true                  # Enable API key authentication
  enable_session_auth: true              # Enable session authentication
  oauth_cache_backend_type: "Memory"     # Cache backend: Memory, Redis, Hybrid
  oauth_cache_redis_url: "redis://127.0.0.1:6379"

# =============================================================================
# Cache Configuration
# =============================================================================
cache:
  enabled: true                          # Enable caching
  default_ttl_secs: 3600                 # Default TTL (1 hour)
  max_memory_mb: 512                     # Max memory usage (MB)
  cleanup_interval_secs: 300             # Cleanup interval (5 minutes)
  backend_type: "Memory"                 # Backend: Memory, Redis, Hybrid
  redis_url: "redis://127.0.0.1:6379"    # Redis connection URL
  redis_pool_size: 10                    # Redis connection pool size

# =============================================================================
# Logging Configuration
# =============================================================================
logging:
  level: "INFO"                          # Log level: DEBUG, INFO, WARN, ERROR
  console_enabled: true                  # Enable console output
  file_enabled: true                     # Enable file output
  sampling_default: 1.0                  # Sampling rate (0.0-1.0)
  file_name: "app.log"                   # Log file name
  json_format: false                     # Use JSON format
  rotate_when: "size"                    # Rotation: size, none
  max_bytes: 10485760                    # Max file size (10MB)
  color_blocks: true                     # Use color blocks

# =============================================================================
# Gateway Configuration
# =============================================================================
gateway:
  listen_address: "0.0.0.0"              # Listen address
  listen_port: 8080                      # Listen port
  max_connections: 1000                  # Max concurrent connections
  request_timeout_seconds: 30            # Request timeout
  enable_rate_limiting: true             # Enable rate limiting
  enable_circuit_breaker: true           # Enable circuit breaker
  enable_load_balancing: true            # Enable load balancing
  cors_enabled: true                     # Enable CORS
  cors_origins: ["*"]                    # Allowed origins
  cors_methods: ["GET", "POST", "PUT", "DELETE"]
  cors_headers: ["Content-Type", "Authorization"]
  enable_logging: true                   # Enable request logging
  log_level: "INFO"                      # Gateway log level

# =============================================================================
# Database Configuration
# =============================================================================
database:
  database_type: "Postgres"              # Database: Postgres, MySQL, SQLite
  host: "localhost"                      # Database host
  port: 5432                             # Database port
  database: "mydb"                       # Database name
  username: "user"                       # Database username
  password: "password"                   # Database password
  max_connections: 10                    # Max connections
  min_idle_connections: 1                # Min idle connections
  connection_timeout_secs: 30            # Connection timeout
  idle_timeout_secs: 600                 # Idle timeout (10 minutes)
  max_lifetime_secs: 1800                # Max connection lifetime (30 minutes)
  ssl_mode: "Prefer"                     # SSL mode: Disable, Prefer, Require

# =============================================================================
# Queue Configuration
# =============================================================================
queue:
  enabled: true                          # Enable queue
  backend_type: "Memory"                 # Backend: Memory, RabbitMQ, Kafka, Redis
  connection_string: "memory://localhost"
  max_connections: 10                    # Max connections
  message_max_size: 1048576              # Max message size (1MB)
  consumer_timeout_ms: 30000             # Consumer timeout (30s)
  producer_timeout_ms: 5000              # Producer timeout (5s)
  retry_policy:
    max_retries: 3                       # Max retry attempts
    initial_delay_ms: 100                # Initial delay
    max_delay_ms: 5000                   # Max delay
    multiplier: 2.0                      # Delay multiplier
  dead_letter_config:
    enabled: true                        # Enable dead letter queue
    queue_name: "dead_letter"            # DLQ name
    max_retention_secs: 86400            # Retention (24 hours)

# =============================================================================
# Observability Configuration
# =============================================================================
observability:
  tracing_enabled: true                  # Enable distributed tracing
  metrics_enabled: true                  # Enable metrics collection
  tracing_sampling_rate: 0.1             # Sampling rate (10%)
  tracing_sampling_strategy: "rate"      # Strategy: rate, probabilistic
  metrics_window_size_secs: 300          # Metrics window (5 minutes)
  metrics_bucket_size_secs: 10           # Bucket size (10 seconds)
```

### DMSCAuthConfig

Authentication configuration for JWT, OAuth, and session management.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | Enable authentication |
| `jwt_secret` | `String` | Auto-generated | Secret key for JWT tokens |
| `jwt_expiry_secs` | `u64` | `3600` | JWT token expiry in seconds |
| `session_timeout_secs` | `u64` | `86400` | Session timeout in seconds |
| `oauth_providers` | `Vec<String>` | `[]` | List of OAuth providers |
| `enable_api_keys` | `bool` | `true` | Enable API key authentication |
| `enable_session_auth` | `bool` | `true` | Enable session authentication |
| `oauth_cache_backend_type` | `DMSCCacheBackendType` | `Memory` | OAuth token cache backend |
| `oauth_cache_redis_url` | `String` | `"redis://127.0.0.1:6379"` | Redis URL for OAuth cache |

**Environment Variables:**
- `DMSC_JWT_SECRET`: Override JWT secret (recommended for production)

### DMSCCacheConfig

Cache system configuration for memory and Redis backends.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | Enable caching |
| `default_ttl_secs` | `u64` | `3600` | Default TTL in seconds |
| `max_memory_mb` | `u64` | `512` | Maximum memory in MB |
| `cleanup_interval_secs` | `u64` | `300` | Cleanup interval in seconds |
| `backend_type` | `DMSCCacheBackendType` | `Memory` | Cache backend type |
| `redis_url` | `String` | `"redis://127.0.0.1:6379"` | Redis connection URL |
| `redis_pool_size` | `usize` | `10` | Redis connection pool size |

**DMSCCacheBackendType Values:**
- `Memory`: In-memory cache (fast, non-persistent)
- `Redis`: Redis cache (persistent, distributed)
- `Hybrid`: Memory + Redis (performance and persistence)

### DMSCLogConfig

Logging configuration for console and file output.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `level` | `DMSCLogLevel` | `INFO` | Minimum log level |
| `console_enabled` | `bool` | `true` | Enable console output |
| `file_enabled` | `bool` | `true` | Enable file output |
| `sampling_default` | `f32` | `1.0` | Default sampling rate (0.0-1.0) |
| `file_name` | `String` | `"app.log"` | Log file name |
| `json_format` | `bool` | `false` | Use JSON format |
| `rotate_when` | `String` | `"size"` | Rotation trigger: "size" or "none" |
| `max_bytes` | `u64` | `10485760` | Max file size before rotation |
| `color_blocks` | `bool` | `true` | Use color blocks in output |

**DMSCLogLevel Values:**
- `DEBUG`: Debug-level messages
- `INFO`: Informational messages
- `WARN`: Warning messages
- `ERROR`: Error messages

### DMSCGatewayConfig

API Gateway configuration for HTTP routing and CORS.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `listen_address` | `String` | `"0.0.0.0"` | Listen address |
| `listen_port` | `u16` | `8080` | Listen port |
| `max_connections` | `usize` | `1000` | Max concurrent connections |
| `request_timeout_seconds` | `u64` | `30` | Request timeout in seconds |
| `enable_rate_limiting` | `bool` | `true` | Enable rate limiting |
| `enable_circuit_breaker` | `bool` | `true` | Enable circuit breaker |
| `enable_load_balancing` | `bool` | `true` | Enable load balancing |
| `cors_enabled` | `bool` | `true` | Enable CORS |
| `cors_origins` | `Vec<String>` | `["*"]` | Allowed CORS origins |
| `cors_methods` | `Vec<String>` | `["GET", "POST", ...]` | Allowed CORS methods |
| `cors_headers` | `Vec<String>` | `["Content-Type", ...]` | Allowed CORS headers |
| `enable_logging` | `bool` | `true` | Enable request logging |
| `log_level` | `String` | `"INFO"` | Gateway log level |

### DMSCDatabaseConfig

Database connection configuration for SQL databases.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `database_type` | `DatabaseType` | `Postgres` | Database backend type |
| `host` | `String` | `"localhost"` | Database host |
| `port` | `u16` | `5432` | Database port |
| `database` | `String` | Required | Database name |
| `username` | `String` | Required | Database username |
| `password` | `String` | Required | Database password |
| `max_connections` | `u32` | `10` | Max connection pool size |
| `min_idle_connections` | `u32` | `1` | Min idle connections |
| `connection_timeout_secs` | `u64` | `30` | Connection timeout |
| `idle_timeout_secs` | `u64` | `600` | Idle connection timeout |
| `max_lifetime_secs` | `u64` | `1800` | Max connection lifetime |
| `ssl_mode` | `SslMode` | `Prefer` | SSL/TLS mode |

**DatabaseType Values:**
- `Postgres`: PostgreSQL database
- `MySQL`: MySQL database
- `SQLite`: SQLite database (file-based)

**SslMode Values:**
- `Disable`: No SSL
- `Prefer`: SSL preferred but not required
- `Require`: SSL required

### DMSCQueueConfig

Message queue configuration for async processing.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | Enable queue system |
| `backend_type` | `DMSCQueueBackendType` | `Memory` | Queue backend type |
| `connection_string` | `String` | `"memory://localhost"` | Backend connection string |
| `max_connections` | `u32` | `10` | Max connections |
| `message_max_size` | `usize` | `1048576` | Max message size (1MB) |
| `consumer_timeout_ms` | `u64` | `30000` | Consumer timeout (30s) |
| `producer_timeout_ms` | `u64` | `5000` | Producer timeout (5s) |
| `retry_policy` | `DMSCRetryPolicy` | See below | Retry configuration |
| `dead_letter_config` | `Option<DMSCDeadLetterConfig>` | `None` | Dead letter queue config |

**DMSCQueueBackendType Values:**
- `Memory`: In-memory queue (development/testing)
- `RabbitMQ`: RabbitMQ backend
- `Kafka`: Apache Kafka backend
- `Redis`: Redis-based queue

### DMSCObservabilityConfig

Observability configuration for tracing and metrics.

| Field | Type | Default | Description |
|:------|:-----|:--------|:------------|
| `tracing_enabled` | `bool` | `true` | Enable distributed tracing |
| `metrics_enabled` | `bool` | `true` | Enable metrics collection |
| `tracing_sampling_rate` | `f64` | `0.1` | Sampling rate (0.0-1.0) |
| `tracing_sampling_strategy` | `String` | `"rate"` | Sampling strategy |
| `metrics_window_size_secs` | `u64` | `300` | Metrics window (5 min) |
| `metrics_bucket_size_secs` | `u64` | `10` | Bucket size (10 sec) |

### Configuration File Locations

DMSC uses the following directory structure:

```
project_root/
├── config.yaml              # Main configuration file
├── .dms/                    # Application data directory
│   ├── logs/                # Log files
│   │   └── app.log
│   ├── cache/               # Cache files
│   ├── reports/             # Generated reports
│   ├── observability/       # Tracing/metrics data
│   └── tmp/                 # Temporary files
```

### Loading Configuration

**Rust:**
```rust
use dmsc::prelude::*;

let app = DMSCAppBuilder::new()
    .with_config("config.yaml")
    .with_logging(DMSCLogConfig::default())
    .build()?;

// Access configuration
let port: u16 = app.context().config().get_typed("gateway.listen_port")?;
```

**Python:**
```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

app = (DMSCAppBuilder()
    .with_config("config.yaml")
    .with_logging(DMSCLogConfig())
    .build())
```

### Environment Variable Overrides

Configuration values can be overridden using environment variables:

```bash
# Override using DMSC_ prefix
export DMSC_AUTH_JWT_SECRET="production-secret"
export DMSC_CACHE_BACKEND_TYPE="Redis"
export DMSC_DATABASE_HOST="prod-db.example.com"
export DMSC_GATEWAY_LISTEN_PORT="443"
```

Environment variable naming convention: `DMSC_<SECTION>_<KEY>` (uppercase, underscores)

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [queue](./queue.md): Message queue module providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
