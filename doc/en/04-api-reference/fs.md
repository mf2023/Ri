<div align="center">

# FileSystem API Reference

**Version: 0.1.4**

**Last modified date: 2026-01-15**

The fs module provides secure file system operations, supporting atomic writes, directory management, and categorized directory organization.

## Module Overview

</div>

The fs module is DMSC's file system abstraction layer, providing the following core features:

- **Atomic Writes**: Use temporary files to ensure write operation atomicity
- **Directory Management**: Automatically create necessary directory structures
- **Categorized Directories**: Support for logs, cache, reports, observability, and other categories
- **JSON Support**: Built-in JSON serialization and deserialization
- **Safe Operations**: All file operations are processed safely

<div align="center">

## Core Components

</div>

### DMSCFileSystem

The main file system interface, providing unified file operation functionality.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new_with_root(project_root)` | Create file system with project root | `project_root: PathBuf` | `Self` |
| `new_with_roots(project_root, app_data_root)` | Create file system with roots | `project_root: PathBuf`, `app_data_root: PathBuf` | `Self` |
| `new_auto_root()` | Auto-detect project root | None | `DMSCResult<Self>` |
| `atomic_write_text(path, text)` | Atomically write text | `path: P`, `text: &str` | `DMSCResult<()>` |
| `atomic_write_bytes(path, data)` | Atomically write bytes | `path: P`, `data: &[u8]` | `DMSCResult<()>` |
| `read_text(path)` | Read text | `path: P` | `DMSCResult<String>` |
| `read_json(path)` | Read JSON | `path: P` | `DMSCResult<T>` |
| `write_json(path, value)` | Write JSON | `path: P`, `value: &T` | `DMSCResult<()>` |
| `exists(path)` | Check if path exists | `path: P` | `bool` |
| `remove_file(path)` | Remove file | `path: P` | `DMSCResult<()>` |
| `remove_dir_all(path)` | Remove directory and contents | `path: P` | `DMSCResult<()>` |
| `copy_file(from, to)` | Copy file | `from: P`, `to: Q` | `DMSCResult<()>` |
| `append_text(path, text)` | Append text | `path: P`, `text: &str` | `DMSCResult<()>` |
| `safe_mkdir(path)` | Safely create directory | `path: P` | `DMSCResult<PathBuf>` |
| `ensure_parent_dir(path)` | Ensure parent directory exists | `path: P` | `DMSCResult<PathBuf>` |
| `app_dir()` | Get application data directory | None | `PathBuf` |
| `logs_dir()` | Get logs directory | None | `PathBuf` |
| `cache_dir()` | Get cache directory | None | `PathBuf` |
| `reports_dir()` | Get reports directory | None | `PathBuf` |
| `observability_dir()` | Get observability directory | None | `PathBuf` |
| `temp_dir()` | Get temporary directory | None | `PathBuf` |

#### Usage Example

```rust
use dmsc::prelude::*;
use std::path::PathBuf;

fn example() -> DMSCResult<()> {
    let project_root = PathBuf::from(".");
    let fs = DMSCFileSystem::new_with_root(project_root);
    
    fs.atomic_write_text("example.txt", "Hello, DMSC!")?;
    
    let content = fs.read_text("example.txt")?;
    println!("File content: {}", content);
    
    let data = json!({"key": "value"});
    fs.write_json("example.json", &data)?;
    
    let read_data: serde_json::Value = fs.read_json("example.json")?;
    println!("JSON data: {:?}", read_data);
    
    let logs_dir = fs.logs_dir();
    println!("Logs directory: {:?}", logs_dir);
    
    Ok(())
}
```

### Atomic Write Operations

Atomic writes use temporary files to ensure data integrity:

```rust
use dmsc::prelude::*;
use std::path::PathBuf;

let fs = DMSCFileSystem::new_with_root(PathBuf::from("."))?;

fs.atomic_write_text("config/settings.txt", "database_url = localhost\nport = 8080")?;
fs.atomic_write_bytes("binary/data.bin", &[0x00, 0x01, 0x02, 0x03])?;

println!("Files written atomically");
```

### JSON Operations

```rust
use dmsc::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: u32,
    enabled: bool,
}

let config = Config {
    name: "MyApp".to_string(),
    version: 1,
    enabled: true,
};

fs.write_json("config.json", &config)?;

let loaded_config: Config = fs.read_json("config.json")?;
println!("Loaded config: {:?}", loaded_config);
```

### Directory Operations

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

fs.safe_mkdir("data/backups")?;
fs.ensure_parent_dir("logs/2024/app.log")?;

if fs.exists("data") {
    println!("Directory exists");
}

fs.remove_dir_all("tmp/old_data")?;
```

### File Operations

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

fs.copy_file("original.txt", "backup/original.txt")?;
fs.append_text("logs/app.log", "New log entry\n")?;

if fs.exists("to_delete.txt") {
    fs.remove_file("to_delete.txt")?;
}
```

<div align="center">

## Categorized Directories

</div>

The file system supports categorized directory organization for easier file management:

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let logs_dir = fs.logs_dir();
let cache_dir = fs.cache_dir();
let reports_dir = fs.reports_dir();
let observability_dir = fs.observability_dir();
let temp_dir = fs.temp_dir();

println!("Logs: {:?}", logs_dir);
println!("Cache: {:?}", cache_dir);
println!("Reports: {:?}", reports_dir);
println!("Observability: {:?}", observability_dir);
println!("Temp: {:?}", temp_dir);
```

### Ensure Category Path

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let log_file = fs.ensure_category_path("logs", "app.log")?;
let cache_file = fs.ensure_category_path("cache", "user_data.json")?;

println!("Log file path: {:?}", log_file);
println!("Cache file path: {:?}", cache_file);
```

### Normalize Path

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let normalized = fs.normalize_under_category("logs", "/var/log/../log/old.log")?;
println!("Normalized path: {:?}", normalized);
```

<div align="center">

## Application Data Directory

</div>

The application data directory is used to store application-private data:

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let app_dir = fs.app_dir();
println!("App data directory: {:?}", app_dir);

let custom_fs = DMSCFileSystem::new_with_roots(
    PathBuf::from("."),
    PathBuf::from("/var/lib/myapp")
)?;

let custom_app_dir = custom_fs.app_dir();
println!("Custom app directory: {:?}", custom_app_dir);
```

<div align="center">

## Configuration Options

</div>

File system configuration is primarily controlled through constructor parameters:

| Parameter | Type | Description |
|:--------|:-----|:-------------|
| `project_root` | `PathBuf` | Project root directory |
| `app_data_root` | `PathBuf` | Application data directory (default: .dms under project root) |

<div align="center">

## Best Practices

</div>

1. **Use atomic writes**: Use atomic write operations for important data to avoid corruption
2. **Use categorized directories**: Organize files by function for easier management and cleanup
3. **Clean up temporary files promptly**: Use temp_dir for temporary files and clean up regularly
4. **Use JSON for configuration**: Use JSON format for configuration files for readability and editing
5. **Verify file existence**: Check if files exist before performing file operations
6. **Use ensure_parent_dir**: Ensure parent directory exists before writing files

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
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
