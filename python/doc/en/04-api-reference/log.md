<div align="center">

# Log API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The log module provides structured logging functionality, supporting multiple output formats and destinations.

## Module Overview

</div>

The log module contains the following sub-modules:

- **config**: Log configuration management
- **formatters**: Log formatters
- **handlers**: Log handlers
- **filters**: Log filters
- **tracers**: Log tracers

<div align="center">

## Core Components

</div>

### DMSCLogConfig

Log configuration class, used to configure log behavior.

#### Constructor

```python
DMSCLogConfig(
    level: str = "INFO",
    format: str = "json",
    enable_console: bool = True,
    enable_file: bool = False,
    file_path: str = "logs/app.log",
    max_file_size: str = "100MB",
    max_files: int = 10,
    compress: bool = True,
    enable_trace: bool = True,
    sampling_rate: float = 1.0
)
```

#### Parameter Description

| Parameter | Description | Type | Default |
|:--------|:-------------|:--------|:--------|
| `level` | Log level | `str` | `"INFO"` |
| `format` | Log format ("json" or "text") | `str` | `"json"` |
| `enable_console` | Output to console | `bool` | `True` |
| `enable_file` | Output to file | `bool` | `False` |
| `file_path` | Log file path | `str` | `"logs/app.log"` |
| `max_file_size` | Max size per log file | `str` | `"100MB"` |
| `max_files` | Number of log files to keep | `int` | `10` |
| `compress` | Compress old log files | `bool` | `True` |
| `enable_trace` | Enable trace context | `bool` | `True` |
| `sampling_rate` | Log sampling rate | `float` | `1.0` |

### DMSCLogger

Structured logger class.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `debug(category, message, **kwargs)` | Log debug message | `category: str`, `message: str`, `**kwargs` | `None` |
| `info(category, message, **kwargs)` | Log info message | `category: str`, `message: str`, `**kwargs` | `None` |
| `warn(category, message, **kwargs)` | Log warning message | `category: str`, `message: str`, `**kwargs` | `None` |
| `error(category, message, **kwargs)` | Log error message | `category: str`, `message: str`, `**kwargs` | `None` |
| `critical(category, message, **kwargs)` | Log critical message | `category: str`, `message: str`, `**kwargs` | `None` |

#### Usage Example

```python
from dmsc import DMSCLogConfig

# Configure logging
config = DMSCLogConfig(
    level="DEBUG",
    format="json",
    enable_console=True,
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="50MB",
    max_files=5
)

# Create logger
logger = DMSCLogger(config)

# Log messages
logger.info("app", "Application started")
logger.info("app", "User logged in", user_id=123, username="john")
logger.warn("auth", "Failed login attempt", ip="192.168.1.1", attempts=3)
logger.error("database", "Connection failed", error="timeout", host="db.example.com")
```

## Log Levels

```python
from dmsc import DMSCLogger, DMSCLogConfig

config = DMSCLogConfig(level="DEBUG")
logger = DMSCLogger(config)

# DEBUG - Detailed information for debugging
logger.debug("debug", "Variable values", x=1, y=2, z=3)

# INFO - General information about application progress
logger.info("info", "Server started", port=8080, workers=4)

# WARN - Warning situations that may need attention
logger.warn("warn", "High memory usage", usage="85%", threshold="80%")

# ERROR - Error situations that don't stop the application
logger.error("error", "Request failed", status_code=500, url="/api/users")

# CRITICAL - Critical situations that may stop the application
logger.critical("critical", "Database connection lost", retry_count=5)
```

## Structured Logging

### JSON Format

```python
from dmsc import DMSCLogger, DMSCLogConfig

config = DMSCLogConfig(format="json")
logger = DMSCLogger(config)

# JSON log output:
# {"level": "INFO", "category": "order", "message": "Order created", "order_id": "12345", ...}
logger.info("order", "Order created", order_id="12345", total=99.99, items=3)
```

### Text Format

```python
from dmsc import DMSCLogger, DMSCLogConfig

config = DMSCLogConfig(format="text", enable_trace=False)
logger = DMSCLogger(config)

# Text log output:
# [2024-01-15 10:30:45] [INFO] [order] Order created - order_id=12345, total=99.99
logger.info("order", "Order created", order_id="12345", total=99.99)
```

## Log Handlers

### Console Handler

```python
from dmsc import DMSCLogConfig

config = DMSCLogConfig(
    level="INFO",
    enable_console=True,
    enable_file=False
)
```

### File Handler

```python
from dmsc import DMSCLogConfig

config = DMSCLogConfig(
    level="DEBUG",
    enable_console=False,
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="100MB",
    max_files=10,
    compress=True
)
```

### Rotating File Handler

```python
from dmsc import DMSCLogConfig

# Use rotating files for better log management
config = DMSCLogConfig(
    level="INFO",
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="50MB",
    max_files=5,
    compress=True
)
```

## Log Filtering

### By Level

```python
from dmsc import DMSCLogger, DMSCLogConfig, DMSCBatchLogFilter

config = DMSCLogConfig(level="INFO")

# Only show INFO and above
logger = DMSCLogger(config)
```

### By Category

```python
from dmsc import DMSCLogger, DMSCLogConfig, DMSCCategoryLogFilter

config = DMSCLogConfig(level="DEBUG")

# Filter logs by category
filter_config = {
    "include": ["app", "auth", "database"],
    "exclude": ["debug", "trace"]
}
```

## Log Sampling

```python
from dmsc import DMSCLogConfig

# Sample high-volume logs
config = DMSCLogConfig(
    level="DEBUG",
    sampling_rate=0.1  # Log only 10% of DEBUG messages
)

logger = DMSCLogger(config)
```

## Integration with Tracing

```python
from dmsc import DMSCLogger, DMSCLogConfig

config = DMSCLogConfig(
    level="INFO",
    enable_trace=True
)

logger = DMSCLogger(config)

# Trace ID is automatically included in logs
# {"level": "INFO", "trace_id": "abc123", "span_id": "def456", ...}
logger.info("request", "Request received", path="/api/users", method="GET")
```

## Best Practices

1. **Use Structured Logging**: Always use JSON format for easy parsing
2. **Include Context**: Add relevant context to every log message
3. **Use Categories**: Organize logs by category (e.g., "auth", "database", "api")
4. **Set Appropriate Levels**: Use DEBUG for development, INFO for production
5. **Don't Log Sensitive Data**: Never log passwords, tokens, or personal information
6. **Log Exceptions with Context**: Always include context when logging exceptions
7. **Use Sampling for High-Volume Logs**: Reduce log volume with sampling
8. **Implement Log Rotation**: Prevent disk space issues with log rotation
9. **Monitor Log Volume**: Track log volume to detect anomalies
10. **Correlate Logs with Traces**: Use trace_id to link logs across services
