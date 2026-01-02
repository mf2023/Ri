<div align="center">

# Config API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The config module provides unified configuration management functionality, supporting multiple configuration sources and dynamic configuration updates.

## Module Overview

</div>

The config module contains the following sub-modules:

- **sources**: Configuration source implementations (files, environment variables, database, etc.)
- **validators**: Configuration validators
- **transformers**: Configuration transformers
- **encryptors**: Configuration encryptors
- **reloaders**: Configuration reloaders
- **watchers**: Configuration watchers

<div align="center">

## Core Components

</div>

### DMSCConfig

Configuration manager, providing unified configuration interface.

#### Constructor

```python
DMSCConfig(
    sources: List[DMSCConfigSource] = None,
    auto_reload: bool = True,
    reload_interval: int = 60,
    enable_encryption: bool = False,
    encryption_key: str = "",
    enable_validation: bool = True,
    enable_caching: bool = True,
    cache_ttl: int = 300,
    environment: str = "development",
    fallback_to_default: bool = True,
    strict_mode: bool = False
)
```

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `get(key, default=None)` | Get configuration value | `key: str`, `default: Any` | `Any` |
| `get_string(key, default="")` | Get string configuration | `key: str`, `default: str` | `str` |
| `get_int(key, default=0)` | Get integer configuration | `key: str`, `default: int` | `int` |
| `get_float(key, default=0.0)` | Get float configuration | `key: str`, `default: float` | `float` |
| `get_bool(key, default=False)` | Get boolean configuration | `key: str`, `default: bool` | `bool` |

#### Usage Example

```python
from dmsc import DMSCConfig

# Create configuration manager
config = DMSCConfig()

# Set configuration values
config.set("app.name", "MyDMSCApp")
config.set("app.host", "0.0.0.0")
config.set("app.port", 8080)
config.set("database.url", "postgresql://localhost/mydb")
config.set("cache.enabled", True)

# Get configuration values
app_name = config.get("app.name")
host = config.get("app.host")
port = config.get("app.port", 8080)

# Get with default value
debug = config.get("app.debug", False)
```

### DMSCConfigSource

Configuration source base class.

#### Built-in Sources

```python
from dmsc import (
    DMSCConfig,
    DMSCFileConfigSource,
    DMSCEnvConfigSource,
    DMSCArgsConfigSource
)

# File source (YAML)
file_source = DMSCFileConfigSource(
    path="config.yaml",
    format="yaml",
    optional=True
)

# Environment variable source
env_source = DMSCEnvConfigSource(
    prefix="MYAPP_",
    separator="__"
)

# Command line arguments source
args_source = DMSCArgsConfigSource(
    prefix="--"
)

# Combine sources
config = DMSCConfig(
    sources=[file_source, env_source, args_source],
    auto_reload=True
)
```

## Configuration File Formats

### YAML Configuration

```yaml
# config.yaml
app:
  name: MyDMSCApp
  host: "0.0.0.0"
  port: 8080
  debug: true
  workers: 4

database:
  url: "postgresql://localhost:5432/mydb"
  pool_size: 10
  timeout: 30

redis:
  host: "localhost"
  port: 6379
  db: 0

logging:
  level: "INFO"
  format: "json"

cache:
  enabled: true
  ttl: 3600
  max_size: 10000
```

### JSON Configuration

```json
{
  "app": {
    "name": "MyDMSCApp",
    "host": "0.0.0.0",
    "port": 8080
  },
  "database": {
    "url": "postgresql://localhost/mydb",
    "pool_size": 10
  }
}
```

## Dynamic Configuration Updates

### Hot Reload

```python
from dmsc import DMSCConfig, DMSCFileConfigSource

# Create config with auto-reload
config = DMSCConfig(
    sources=[DMSCFileConfigSource("config.yaml")],
    auto_reload=True,
    reload_interval=30
)

# Configuration changes are automatically detected
# No need to restart the application
```

### Manual Reload

```python
# Manually trigger configuration reload
config.reload()

# Get configuration version
version = config.get_version()
print(f"Config version: {version}")
```

## Configuration Validation

### Built-in Validators

```python
from dmsc import DMSCConfig, DMSCConfigValidator

config = DMSCConfig()

# Add validators
config.add_validator(
    DMSCConfigValidator("app.port")
    .is_required()
    .is_integer()
    .in_range(1, 65535)
)

config.add_validator(
    DMSCConfigValidator("database.url")
    .is_required()
    .is_url()
)

config.add_validator(
    DMSCConfigValidator("app.debug")
    .is_boolean()
)

# Validate all configurations
errors = config.validate()
if errors:
    for error in errors:
        print(f"Validation error: {error}")
```

### Custom Validators

```python
from dmsc import DMSCConfigValidator

class CustomValidator(DMSCConfigValidator):
    def validate(self, value):
        if value == "forbidden":
            return "Value cannot be 'forbidden'"
        return None

config.add_validator(
    CustomValidator("app.name")
    .custom_rule(lambda v: len(v) > 3 if isinstance(v, str) else True)
)
```

## Environment-Specific Configuration

```python
from dmsc import DMSCConfig

# Development configuration
config = DMSCConfig(
    environment="development"
)
config.set("debug", True)
config.set("log_level", "DEBUG")

# Production configuration
if config.get_environment() == "production":
    config.set("debug", False)
    config.set("log_level", "INFO")
    config.set("workers", 8)

# Override with environment variables
import os
if os.environ.get("DEBUG_MODE"):
    config.set("debug", os.environ["DEBUG_MODE"] == "true")
```

## Configuration Encryption

```python
from dmsc import DMSCConfig

# Create config with encryption
config = DMSCConfig(
    enable_encryption=True,
    encryption_key="your-encryption-key"
)

# Set sensitive values (automatically encrypted)
config.set("database.password", "secret-password")
config.set("api密钥", "sensitive-api-key")

# Get decrypted values
password = config.get("database.password")  # Automatically decrypted
```

## Best Practices

1. **Use Hierarchical Keys**: Use dot notation for organization (e.g., `database.pool.size`)
2. **Provide Defaults**: Always provide sensible default values
3. **Validate Early**: Validate configuration at application startup
4. **Use Environment Variables**: Use env vars for deployment-specific settings
5. **Secure Sensitive Data**: Encrypt passwords and API keys
6. **Document Configuration**: Add comments to configuration files
7. **Version Control**: Keep configuration files in version control (without secrets)
8. **Test Configuration**: Write tests for configuration validation
