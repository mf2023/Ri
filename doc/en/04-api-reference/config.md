<div align="center">

# Config API Reference

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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
| `get(key)` | Get configuration value | `key: &str` | `Option<String>` |
| `get_typed<T>(key)` | Get type-safe configuration value | `key: &str` | `DMSCResult<T>` |
| `get_or_default(key, default)` | Get configuration value or default | `key: &str`, `default: T` | `T` |
| `set(key, value)` | Set configuration value | `key: &str`, `value: impl Serialize` | `DMSCResult<()>` |
| `has(key)` | Check if configuration exists | `key: &str` | `bool` |
| `keys()` | Get all configuration keys | None | `Vec<String>` |
| `reload()` | Reload configuration | None | `DMSCResult<()>` |
| `watch(key, callback)` | Watch configuration changes | `key: &str`, `callback: impl Fn(&str)` | `DMSCResult<()>` |
| `validate()` | Validate configuration integrity | None | `DMSCResult<()>` |

#### Usage Example

```rust
use dms::prelude::*;

// Get string configuration
let service_name = ctx.config().get("service.name").unwrap_or("default");

// Get type-safe configuration
let port: u16 = ctx.config().get_typed("service.port")?;
let max_connections: usize = ctx.config().get_typed("database.max_connections")?;

// Get configuration or default value
let timeout = ctx.config().get_or_default("service.timeout", 30);

// Check if configuration exists
if ctx.config().has("feature.new_feature") {
    // Enable new feature
}

// Get all configuration keys
let keys = ctx.config().keys();
for key in keys {
    println!("Config key: {}", key);
}
```

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
use dms::prelude::*;

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
  format: "json"
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
use dms::prelude::*;

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
use dms::prelude::*;

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

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, providing JWT, OAuth2, and RBAC authentication and authorization functionality
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [cache](./cache.md): Cache module, providing multi-backend cache abstraction, caching user sessions and permission data
- [database](./database.md): Database module, providing user data persistence and query functionality
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [observability](./observability.md): Observability module, monitoring authentication performance and security events
- [security](./security.md): Security module, providing encryption, hashing, and validation functionality
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
