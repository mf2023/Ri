<div align="center">

# FileSystem Usage Example

**Version: 0.1.7**

**Last modified date: 2026-02-11**

This example demonstrates how to use the fs module for file operations, including atomic writes, directory management, and categorized directory organization.

## Prerequisites

</div>

- DMSC Rust SDK
- serde and serde_json for JSON serialization

<div align="center">

## Example Code

</div>

```rust
use dmsc::prelude::*;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct UserConfig {
    name: String,
    email: String,
    theme: String,
    language: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppSettings {
    version: String,
    debug_mode: bool,
    max_connections: u32,
}

fn main() -> DMSCResult<()> {
    println!("=== DMSC FileSystem Example ===\n");
    
    let project_root = PathBuf::from(".");
    let fs = DMSCFileSystem::new_with_root(project_root);
    
    println!("1. Atomic Write Operations");
    println!("---------------------------");
    
    fs.atomic_write_text("example.txt", "Hello, DMSC FileSystem!")?;
    println!("Text file written successfully");
    
    fs.atomic_write_bytes("binary/data.bin", &[0x00, 0x01, 0x02, 0x03, 0x04])?;
    println!("Binary file written successfully");
    
    println!("\n2. Read Operations");
    println!("------------------");
    
    let content = fs.read_text("example.txt")?;
    println!("Text content: {}", content);
    
    println!("\n3. JSON Operations");
    println!("------------------");
    
    let config = UserConfig {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        theme: "dark".to_string(),
        language: "en".to_string(),
    };
    
    fs.write_json("config/user.json", &config)?;
    println!("JSON config written");
    
    let loaded_config: UserConfig = fs.read_json("config/user.json")?;
    println!("Loaded config: {:?}", loaded_config);
    
    println!("\n4. Directory Operations");
    println!("------------------------");
    
    fs.safe_mkdir("data/backups")?;
    fs.safe_mkdir("data/logs")?;
    fs.safe_mkdir("cache/temp")?;
    println!("Directories created successfully");
    
    fs.ensure_parent_dir("logs/2024/01/app.log")?;
    fs.ensure_parent_dir("reports/2024/01/summary.pdf")?;
    println!("Parent directories ensured");
    
    println!("\n5. Categorized Directories");
    println!("---------------------------");
    
    let logs_dir = fs.logs_dir();
    let cache_dir = fs.cache_dir();
    let reports_dir = fs.reports_dir();
    let observability_dir = fs.observability_dir();
    let temp_dir = fs.temp_dir();
    
    println!("Logs directory: {:?}", logs_dir);
    println!("Cache directory: {:?}", cache_dir);
    println!("Reports directory: {:?}", reports_dir);
    println!("Observability directory: {:?}", observability_dir);
    println!("Temp directory: {:?}", temp_dir);
    
    println!("\n6. Category Path Operations");
    println!("----------------------------");
    
    let log_file = fs.ensure_category_path("logs", "app.log")?;
    println!("Log file path: {:?}", log_file);
    
    let cache_file = fs.ensure_category_path("cache", "user_data.json")?;
    println!("Cache file path: {:?}", cache_file);
    
    let normalized = fs.normalize_under_category("logs", "/var/log/../log/old.log")?;
    println!("Normalized path: {:?}", normalized);
    
    println!("\n7. File Copy and Append");
    println!("------------------------");
    
    fs.copy_file("example.txt", "backup/example_backup.txt")?;
    println!("File copied successfully");
    
    fs.append_text("logs/app.log", "[2024-01-15 10:30:00] Application started\n")?;
    fs.append_text("logs/app.log", "[2024-01-15 10:30:05] Configuration loaded\n")?;
    println!("Text appended to log file");
    
    println!("\n8. File Existence Check");
    println!("------------------------");
    
    if fs.exists("example.txt") {
        println!("example.txt exists");
    }
    
    if fs.exists("nonexistent.txt") {
        println!("nonexistent.txt exists");
    } else {
        println!("nonexistent.txt does not exist");
    }
    
    println!("\n9. Application Data Directory");
    println!("------------------------------");
    
    let app_dir = fs.app_dir();
    println!("App data directory: {:?}", app_dir);
    
    let custom_fs = DMSCFileSystem::new_with_roots(
        PathBuf::from("."),
        PathBuf::from("/var/lib/myapp")
    )?;
    
    let custom_app_dir = custom_fs.app_dir();
    println!("Custom app directory: {:?}", custom_app_dir);
    
    println!("\n10. Cleanup Operations");
    println!("-----------------------");
    
    fs.remove_file("example.txt")?;
    println!("example.txt removed");
    
    println!("\n=== Example Completed ===");
    Ok(())
}
```

<div align="center">

## Running Steps

</div>

### 1. Create Example Project

```bash
cargo new dms-fs-example
cd dms-fs-example
```

### 2. Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC.git" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. Run Example

```bash
cargo run
```

<div align="center">

## Expected Output

</div>

```
=== DMSC FileSystem Example ===

1. Atomic Write Operations
---------------------------
Text file written successfully
Binary file written successfully

2. Read Operations
------------------
Text content: Hello, DMSC FileSystem!

3. JSON Operations
------------------
JSON config written
Loaded config: UserConfig { name: "John Doe", email: "john@example.com", theme: "dark", language: "en" }

4. Directory Operations
------------------------
Directories created successfully
Parent directories ensured

5. Categorized Directories
---------------------------
Logs directory: ".dms/logs"
Cache directory: ".dms/cache"
Reports directory: ".dms/reports"
Observability directory: ".dms/observability"
Temp directory: ".dms/tmp"

6. Category Path Operations
----------------------------
Log file path: ".dms/logs/app.log"
Cache file path: ".dms/cache/user_data.json"
Normalized path: ".dms/logs/old.log"

7. File Copy and Append
------------------------
File copied successfully
Text appended to log file

8. File Existence Check
------------------------
example.txt exists
nonexistent.txt does not exist

9. Application Data Directory
------------------------------
App data directory: ".dms"
Custom app directory: "/var/lib/myapp"

10. Cleanup Operations
-----------------------
example.txt removed

=== Example Completed ===
```

<div align="center">

## Advanced Features

</div>

### Batch JSON Operations

```rust
fn batch_json_operations(fs: &DMSCFileSystem) -> DMSCResult<()> {
    let settings = AppSettings {
        version: "0.1.7".to_string(),
        debug_mode: true,
        max_connections: 100,
    };
    
    fs.write_json("config/settings.json", &settings)?;
    
    let loaded: AppSettings = fs.read_json("config/settings.json")?;
    println!("Settings: {:?}", loaded);
    
    Ok(())
}
```

### Recursive Directory Cleanup

```rust
fn cleanup_old_data(fs: &DMSCFileSystem) -> DMSCResult<()> {
    let old_dirs = vec!["cache/old", "logs/backup", "tmp/archive"];
    
    for dir in old_dirs {
        if fs.exists(dir) {
            fs.remove_dir_all(dir)?;
            println!("Cleaned up: {}", dir);
        }
    }
    
    Ok(())
}
```

<div align="center">

## Best Practices

</div>

1. **Use atomic writes**: Use atomic write operations for important data to avoid corruption
2. **Use categorized directories**: Organize files by function for easier management
3. **Use JSON for configuration**: Use JSON format for configuration files for readability
4. **Clean up temporary files promptly**: Regularly clean up the tmp directory
5. **Verify file existence**: Check if files exist before performing operations
6. **Use ensure_parent_dir**: Ensure parent directory exists before writing files

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with usage examples summary and quick navigation
- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [basic-app](./basic-app.md): Basic application examples
- [caching](./caching.md): Caching examples, including memory and distributed caching
- [database](./database.md): Database operation examples
- [device](./device.md): Device control examples
- [gateway](./gateway.md): API gateway examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [hooks](./hooks.md): Hook system examples
- [observability](./observability.md): Observability examples
- [protocol](./protocol.md): Protocol module examples
- [service_mesh](./service_mesh.md): Service mesh examples
- [validation](./validation.md): Data validation examples
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication

<div align="center">

## Related Documentation

</div>

- [FileSystem API Reference](../04-api-reference/fs.md): Detailed API documentation
- [Core Concepts](../03-core-concepts.md): Learn about DMSC core design principles
- [Best Practices](../06-best-practices.md): More best practice suggestions
