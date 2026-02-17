<div align="center">

# C/C++ API Reference

**Version: 0.1.7**

**Last modified date: 2026-02-17**

The C/C++ API module provides comprehensive C language bindings for the DMSC framework, enabling C and C++ applications to leverage DMSC's capabilities.

## Module Overview

</div>

The C API module is organized into functional submodules, each providing bindings for a specific DMSC capability:

- **core**: Application initialization, configuration management, and lifecycle control
- **auth**: Authentication and authorization services including JWT token handling
- **cache**: In-memory caching with configurable eviction policies
- **database**: Database connection pooling and query execution support
- **device**: Device abstraction layer for managing computational resources
- **fs**: Cross-platform file system operations
- **gateway**: HTTP API gateway with request routing and middleware chains
- **grpc**: gRPC server and client support for RPC communication
- **hooks**: Event hook system for extensibility through callback registration
- **log**: Structured logging infrastructure with multiple output destinations
- **module_rpc**: Inter-module RPC communication within the DMSC framework
- **observability**: Metrics collection, tracing integration, and health check endpoints
- **protocol**: Protocol handling for various wire formats and serialization schemes
- **queue**: Message queue operations for asynchronous task processing
- **service_mesh**: Service mesh integration for distributed systems
- **validation**: Data validation with schema definitions and custom validation rules
- **ws**: WebSocket protocol support for full-duplex communication

<div align="center">

## Global Functions

</div>

### Initialization and Cleanup

| Function | Description | Parameters | Return Value |
|:---------|:------------|:-----------|:-------------|
| `dmsc_init()` | Initialize DMSC library | None | `int` (0 for success) |
| `dmsc_cleanup()` | Cleanup DMSC library resources | None | `void` |
| `dmsc_version()` | Get DMSC version string | None | `char*` (must be freed) |
| `dmsc_string_free(char* s)` | Free a string returned by DMSC | `s`: String to free | `void` |

#### Usage Example

```c
#include <stdio.h>
#include "dmsc.h"

int main() {
    // Initialize DMSC
    int result = dmsc_init();
    if (result != 0) {
        fprintf(stderr, "DMSC initialization failed\n");
        return 1;
    }
    
    // Get version
    char* version = dmsc_version();
    printf("DMSC Version: %s\n", version);
    dmsc_string_free(version);
    
    // Cleanup
    dmsc_cleanup();
    return 0;
}
```

<div align="center">

## Core Module

</div>

### Types

| Type | Description |
|:-----|:------------|
| `DMSCApplication` | Application instance handle |
| `DMSCAppConfig` | Application configuration handle |
| `DMSCServiceContext` | Service context handle |

### Functions

| Function | Description | Parameters | Return Value |
|:---------|:------------|:-----------|:-------------|
| `dmsc_app_config_new()` | Create new application configuration | None | `DMSCAppConfig*` |
| `dmsc_app_config_free(DMSCAppConfig* config)` | Free application configuration | `config`: Configuration handle | `void` |
| `dmsc_app_config_set_name(DMSCAppConfig* config, const char* name)` | Set application name | `config`, `name` | `int` |
| `dmsc_app_config_set_environment(DMSCAppConfig* config, const char* env)` | Set environment | `config`, `env` | `int` |
| `dmsc_application_new(DMSCAppConfig* config)` | Create application instance | `config` | `DMSCApplication*` |
| `dmsc_application_free(DMSCApplication* app)` | Free application instance | `app` | `void` |
| `dmsc_application_start(DMSCApplication* app)` | Start application | `app` | `int` |
| `dmsc_application_stop(DMSCApplication* app)` | Stop application | `app` | `int` |
| `dmsc_application_get_context(DMSCApplication* app)` | Get service context | `app` | `DMSCServiceContext*` |

<div align="center">

## Log Module

</div>

### Types

| Type | Description |
|:-----|:------------|
| `DMSCLogger` | Logger instance handle |
| `DMSCLogConfig` | Logger configuration handle |
| `DMSCLogLevel` | Log level enum |

### Log Levels

| Level | Value | Description |
|:------|:------|:------------|
| `DMSC_LOG_TRACE` | 0 | Trace level |
| `DMSC_LOG_DEBUG` | 1 | Debug level |
| `DMSC_LOG_INFO` | 2 | Info level |
| `DMSC_LOG_WARN` | 3 | Warning level |
| `DMSC_LOG_ERROR` | 4 | Error level |
| `DMSC_LOG_FATAL` | 5 | Fatal level |

### Functions

| Function | Description | Parameters | Return Value |
|:---------|:------------|:-----------|:-------------|
| `dmsc_log_config_new()` | Create log configuration | None | `DMSCLogConfig*` |
| `dmsc_log_config_free(DMSCLogConfig* config)` | Free log configuration | `config` | `void` |
| `dmsc_log_config_set_level(DMSCLogConfig* config, DMSCLogLevel level)` | Set log level | `config`, `level` | `int` |
| `dmsc_logger_new(DMSCLogConfig* config)` | Create logger | `config` | `DMSCLogger*` |
| `dmsc_logger_free(DMSCLogger* logger)` | Free logger | `logger` | `void` |
| `dmsc_logger_log(DMSCLogger* logger, DMSCLogLevel level, const char* message)` | Log message | `logger`, `level`, `message` | `int` |
| `dmsc_logger_trace(DMSCLogger* logger, const char* message)` | Log trace | `logger`, `message` | `int` |
| `dmsc_logger_debug(DMSCLogger* logger, const char* message)` | Log debug | `logger`, `message` | `int` |
| `dmsc_logger_info(DMSCLogger* logger, const char* message)` | Log info | `logger`, `message` | `int` |
| `dmsc_logger_warn(DMSCLogger* logger, const char* message)` | Log warning | `logger`, `message` | `int` |
| `dmsc_logger_error(DMSCLogger* logger, const char* message)` | Log error | `logger`, `message` | `int` |
| `dmsc_logger_fatal(DMSCLogger* logger, const char* message)` | Log fatal | `logger`, `message` | `int` |

#### Usage Example

```c
DMSCLogConfig* log_config = dmsc_log_config_new();
dmsc_log_config_set_level(log_config, DMSC_LOG_INFO);

DMSCLogger* logger = dmsc_logger_new(log_config);
dmsc_log_config_free(log_config);

dmsc_logger_info(logger, "Application started");
dmsc_logger_warn(logger, "This is a warning");
dmsc_logger_error(logger, "This is an error");

dmsc_logger_free(logger);
```

<div align="center">

## Cache Module

</div>

### Types

| Type | Description |
|:-----|:------------|
| `DMSCCache` | Cache instance handle |
| `DMSCCacheConfig` | Cache configuration handle |

### Functions

| Function | Description | Parameters | Return Value |
|:---------|:------------|:-----------|:-------------|
| `dmsc_cache_config_new()` | Create cache configuration | None | `DMSCCacheConfig*` |
| `dmsc_cache_config_free(DMSCCacheConfig* config)` | Free cache configuration | `config` | `void` |
| `dmsc_cache_config_set_capacity(DMSCCacheConfig* config, size_t capacity)` | Set cache capacity | `config`, `capacity` | `int` |
| `dmsc_cache_config_set_ttl(DMSCCacheConfig* config, uint64_t ttl_seconds)` | Set default TTL | `config`, `ttl_seconds` | `int` |
| `dmsc_cache_new(DMSCCacheConfig* config)` | Create cache instance | `config` | `DMSCCache*` |
| `dmsc_cache_free(DMSCCache* cache)` | Free cache instance | `cache` | `void` |
| `dmsc_cache_set(DMSCCache* cache, const char* key, const void* value, size_t value_len, uint64_t ttl_seconds)` | Set cache value | `cache`, `key`, `value`, `value_len`, `ttl_seconds` | `int` |
| `dmsc_cache_get(DMSCCache* cache, const char* key, void** value, size_t* value_len)` | Get cache value | `cache`, `key`, `value`, `value_len` | `int` |
| `dmsc_cache_delete(DMSCCache* cache, const char* key)` | Delete cache entry | `cache`, `key` | `int` |
| `dmsc_cache_exists(DMSCCache* cache, const char* key)` | Check if key exists | `cache`, `key` | `int` (1 if exists) |
| `dmsc_cache_clear(DMSCCache* cache)` | Clear all cache entries | `cache` | `int` |

<div align="center">

## Error Handling

</div>

### Error Codes

| Code | Value | Description |
|:-----|:------|:------------|
| `DMSC_OK` | 0 | Success |
| `DMSC_ERROR_GENERAL` | -1 | General error |
| `DMSC_ERROR_INVALID_ARG` | -2 | Invalid argument |
| `DMSC_ERROR_ALLOCATION` | -3 | Memory allocation failure |
| `DMSC_ERROR_NOT_FOUND` | -4 | Resource not found |
| `DMSC_ERROR_PERMISSION` | -5 | Permission denied |
| `DMSC_ERROR_TIMEOUT` | -6 | Timeout |
| `DMSC_ERROR_NETWORK` | -7 | Network error |

<div align="center">

## Memory Management

</div>

The C API uses manual memory management following C conventions:

- **Object Creation**: Constructor functions return newly allocated objects. All objects must be freed using the corresponding destructor function.
- **String Handling**: String-returning functions allocate C strings that must be freed using `dmsc_string_free()`. Do not use standard `free()` on these strings.
- **NULL Safety**: All functions handle NULL pointers gracefully, returning error codes or NULL outputs rather than causing undefined behavior.

<div align="center">

## Thread Safety

</div>

The DMSC C API is designed for thread-safe concurrent access:

- **Object-Level Safety**: Individual objects are safe for concurrent use from multiple threads unless documented otherwise.
- **Global State**: Global initialization is thread-safe; subsequent calls to `dmsc_init()` from multiple threads are handled correctly.
- **Resource Sharing**: Objects can be shared across threads following the same patterns as the underlying Rust implementation.

<div align="center">

## Feature Flags

</div>

The DMSC C API supports feature flags for conditional compilation:

| Feature | Description |
|:--------|:------------|
| `c` | Enable C/C++ FFI bindings (required) |
| `gateway` | Enable API gateway features |
| `grpc` | Enable gRPC server and client support |
| `observability` | Enable metrics and tracing |
| `service-mesh` | Enable service mesh integration |
| `websocket` | Enable WebSocket support |

<div align="center">

## Build Integration

</div>

To use the DMSC C API in a C/C++ project:

1. **Compilation**: Include the generated C headers and link against the DMSC shared or static library.
2. **Header Files**: Include the main DMSC header which provides access to all submodule interfaces.
3. **Linking**: Link against the DMSC library using appropriate linker flags for your build system.

### Example CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.10)
project(my_dmsc_app)

find_library(DMSC_LIB dmsc PATHS /path/to/dmsc/lib)

add_executable(my_app main.c)
target_include_directories(my_app PRIVATE /path/to/dmsc/include)
target_link_libraries(my_app ${DMSC_LIB})
```

### Example Build Command

```bash
# Compile
gcc -c main.c -I/path/to/dmsc/include

# Link
gcc main.o -L/path/to/dmsc/lib -ldmsc -o my_app
```
