<div align="center">

# Log API Reference

**Version: 0.1.8**

**Last modified date: 2026-02-15**

The log module provides structured logging with multi-backend support, supporting log levels, formatting, sampling, and log analysis features.

## Module Overview

</div>

The log module contains the following sub-modules:

- **core**: Log core interfaces and type definitions
- **formatters**: Log formatters
- **backends**: Log backend implementations
- **sampling**: Log sampling mechanisms
- **analytics**: Log analysis features

<div align="center">

## Core Components

</div>

### DMSCLogger

The main logger interface providing unified logging functionality.

#### Constructor

```python
DMSCLogger(config: DMSCLogConfig, fs: DMSCFileSystem)
```

Creates a new logger instance.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `debug(target, message)` | Record debug log | `target: &str`, `message: &str` | `PyResult<()>` |
| `info(target, message)` | Record info log | `target: &str`, `message: &str` | `PyResult<()>` |
| `warn(target, message)` | Record warning log | `target: &str`, `message: &str` | `PyResult<()>` |
| `error(target, message)` | Record error log | `target: &str`, `message: &str` | `PyResult<()>` |

**Parameter Description:**
- `target`: Log target identifier, usually a module or component name
- `message`: Log message content

#### Python Usage Example

```python
from dmsc import DMSCLogger, DMSCLogConfig, DMSCFileSystem

# Create log configuration
config = DMSCLogConfig.default()

# Create file system
fs = DMSCFileSystem()

# Create logger
logger = DMSCLogger(config, fs)

# Record logs
logger.debug("my_module", "Debug message")
logger.info("my_module", "Application started")
logger.warn("my_module", "Configuration file not found, using defaults")
logger.error("my_module", "Database connection failed")
```

#### Rust Usage Example

```rust
use dmsc::log::{DMSCLogger, DMSCLogConfig};
use dmsc::fs::DMSCFileSystem;

// Create log configuration
let config = DMSCLogConfig::default();

// Create file system
let fs = DMSCFileSystem::new();

// Create logger
let logger = DMSCLogger::new(&config, fs);

// Record logs
logger.debug("my_module", "Debug message")?;
logger.info("my_module", "Application started")?;
logger.warn("my_module", "Configuration file not found")?;
logger.error("my_module", "Database connection failed")?;
```

### DMSCLogLevel

Log level enumeration type.

#### Variants

| Variant | Description | Value |
|:--------|:-------------|:-----|
| `Debug` | Debug information | 0 |
| `Info` | General information | 1 |
| `Warn` | Warning information | 2 |
| `Error` | Error information | 3 |

#### Color Blocks

Each log level has a corresponding color block indicator:

| Level | Color Block | Description |
|:--------|:-------------|:-----|
| `Debug` | 🟦 | Blue block, for debug information |
| `Info` | 🟩 | Green block, for general information |
| `Warn` | 🟨 | Yellow block, for warning information |
| `Error` | 🟥 | Red block, for error information |

### Log Format

#### Text Format (with color blocks)

```
🟩 | 2024-01-15 10:30:45.123 | INFO  | service | Application started | port=8080
🟨 | 2024-01-15 10:30:45.456 | WARN  | cache   | Key not found
🟥 | 2024-01-15 10:30:45.789 | ERROR | db      | Connection timeout | retry=3
```

Format Description:
- `🟩` - Color block (varies by log level)
- `2024-01-15 10:30:45.123` - Timestamp
- `INFO` - Log level
- `service` - Target module
- `Application started` - Log message

#### Text Format (without color blocks)

When `color_blocks` is set to `false`:

```
2024-01-15 10:30:45.123 | INFO  | service | Application started | port=8080
2024-01-15 10:30:45.456 | WARN  | cache   | Key not found
```

#### JSON Format

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "target": "service",
  "message": "Application started"
}
```

### DMSCLogConfig

Log configuration structure.

#### Methods

| Method | Description | Return Value |
|:--------|:-------------|:--------|
| `default()` | Create default configuration | `DMSCLogConfig` |
| `from_config(config)` | Create from config object | `DMSCLogConfig` |
| `from_env()` | Create from environment variables | `DMSCLogConfig` |

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `level` | `DMSCLogLevel` | Log level | `Info` |
| `console_enabled` | `bool` | Enable console output | `true` |
| `file_enabled` | `bool` | Enable file output | `false` |
| `sampling_default` | `f32` | Default sampling rate (0.0-1.0) | `1.0` |
| `file_name` | `String` | Log file name | `"dms.log"` |
| `json_format` | `bool` | Use JSON format | `false` |
| `rotate_when` | `String` | When to rotate ("size" or "none") | `"size"` |
| `color_blocks` | `bool` | Use color blocks in log output | `true` |

#### Configuration Example

```python
from dmsc import DMSCLogConfig, DMSCLogLevel

# Use default configuration
config = DMSCLogConfig.default()

# Create from config file
config = DMSCLogConfig.from_config(dmsc_config)
```

```rust
use dmsc::log::DMSCLogConfig;
use dmsc::log::DMSCLogLevel;

// Use default configuration
let config = DMSCLogConfig::default();

// Create from config object
let config = DMSCLogConfig::from_config(&dmsc_config);

// Create from environment variables
let config = DMSCLogConfig::from_env();
```

<div align="center">

## Error Handling

</div>

### Log Error Codes

| Error Code | Description |
|:--------|:-------------|
| `LOG_FILE_NOT_FOUND` | Log file not found |
| `LOG_FILE_PERMISSION_DENIED` | Log file permission denied |
| `LOG_FORMAT_ERROR` | Log format error |
| `LOG_BACKEND_ERROR` | Log backend error |

<div align="center">

## Best Practices

</div>

1. **Use Appropriate Log Levels**: Choose appropriate log levels based on importance
2. **Avoid Logging Sensitive Information**: Do not log passwords, keys, or other sensitive information
3. **Use Meaningful Targets**: Use module or component names as targets for easier log filtering
4. **Regular Log Rotation**: Use log rotation to avoid disk space exhaustion

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
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
