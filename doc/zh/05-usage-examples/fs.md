<div align="center">

# 文件系统使用示例

**Version: 0.1.6**

**Last modified date: 2026-02-01**

本示例展示如何使用 fs 模块进行文件操作，包括原子写入、目录管理和分类目录组织。

## 前置要求

</div>

- DMSC Rust SDK
- serde 和 serde_json 用于 JSON 序列化

<div align="center">

## 示例代码

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

## 运行步骤

</div>

### 1. 创建示例项目

```bash
cargo new dms-fs-example
cd dms-fs-example
```

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC.git" }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. 运行示例

```bash
cargo run
```

<div align="center>

## 预期输出

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

## 高级功能

</div>

### 批量JSON读写

```rust
fn batch_json_operations(fs: &DMSCFileSystem) -> DMSCResult<()> {
    let settings = AppSettings {
        version: "0.1.6".to_string(),
        debug_mode: true,
        max_connections: 100,
    };
    
    fs.write_json("config/settings.json", &settings)?;
    
    let loaded: AppSettings = fs.read_json("config/settings.json")?;
    println!("Settings: {:?}", loaded);
    
    Ok(())
}
```

### 递归目录清理

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

## 最佳实践

</div>

1. **使用原子写入**：对重要数据使用原子写入，避免数据损坏
2. **使用分类目录**：按功能分类组织文件，便于管理
3. **使用JSON进行配置**：使用JSON格式存储配置，便于阅读
4. **及时清理临时文件**：定期清理tmp目录
5. **验证文件存在性**：操作前检查文件是否存在
6. **使用ensure_parent_dir**：写入前确保父目录存在

<div align="center">

## 相关模块

</div>

- [README](./README.md)：使用示例总览，提供快速导航
- [authentication](./authentication.md)：认证示例，包括JWT、OAuth2和多因素认证
- [basic-app](./basic-app.md)：基础应用示例
- [caching](./caching.md)：缓存示例，包括内存缓存和分布式缓存
- [database](./database.md)：数据库操作示例
- [device](./device.md)：设备控制示例
- [gateway](./gateway.md)：API网关示例
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [hooks](./hooks.md)：钩子系统示例
- [observability](./observability.md)：可观测性示例
- [protocol](./protocol.md)：协议模块示例
- [service_mesh](./service_mesh.md)：服务网格示例
- [validation](./validation.md)：数据验证示例
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信

<div align="center">

## 相关文档

</div>

- [文件系统API参考](../04-api-reference/fs.md)：详细的API文档
- [核心概念](../03-core-concepts.md)：了解DMSC核心设计理念
- [最佳实践](../06-best-practices.md)：更多最佳实践建议
