<div align="center">

# FileSystem API参考

**Version: 0.1.6**

**Last modified date: 2026-01-30**

fs模块提供安全的文件系统操作功能，支持原子写入、目录管理和分类目录组织。

## 模块概述

</div>

fs模块是DMSC的文件系统抽象层，提供以下核心功能：

- **原子写入**：使用临时文件确保写入操作的原子性
- **目录管理**：自动创建必要的目录结构
- **分类目录**：支持日志、缓存、报告、可观测性等分类目录
- **JSON支持**：内置JSON序列化与反序列化支持
- **安全操作**：所有文件操作都经过安全处理

<div align="center">

## 核心组件

</div>

### DMSCFileSystem

文件系统主接口，提供统一的文件操作功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new_with_root(project_root)` | 创建带项目根的文件系统 | `project_root: PathBuf` | `Self` |
| `new_with_roots(project_root, app_data_root)` | 创建带根目录的文件系统 | `project_root: PathBuf`, `app_data_root: PathBuf` | `Self` |
| `new_auto_root()` | 自动检测项目根目录 | 无 | `DMSCResult<Self>` |
| `atomic_write_text(path, text)` | 原子写入文本 | `path: P`, `text: &str` | `DMSCResult<()>` |
| `atomic_write_bytes(path, data)` | 原子写入字节 | `path: P`, `data: &[u8]` | `DMSCResult<()>` |
| `read_text(path)` | 读取文本 | `path: P` | `DMSCResult<String>` |
| `read_json(path)` | 读取JSON | `path: P` | `DMSCResult<T>` |
| `write_json(path, value)` | 写入JSON | `path: P`, `value: &T` | `DMSCResult<()>` |
| `exists(path)` | 检查路径是否存在 | `path: P` | `bool` |
| `remove_file(path)` | 删除文件 | `path: P` | `DMSCResult<()>` |
| `remove_dir_all(path)` | 删除目录及其内容 | `path: P` | `DMSCResult<()>` |
| `copy_file(from, to)` | 复制文件 | `from: P`, `to: Q` | `DMSCResult<()>` |
| `append_text(path, text)` | 追加文本 | `path: P`, `text: &str` | `DMSCResult<()>` |
| `safe_mkdir(path)` | 安全创建目录 | `path: P` | `DMSCResult<PathBuf>` |
| `ensure_parent_dir(path)` | 确保父目录存在 | `path: P` | `DMSCResult<PathBuf>` |
| `app_dir()` | 获取应用数据目录 | 无 | `PathBuf` |
| `logs_dir()` | 获取日志目录 | 无 | `PathBuf` |
| `cache_dir()` | 获取缓存目录 | 无 | `PathBuf` |
| `reports_dir()` | 获取报告目录 | 无 | `PathBuf` |
| `observability_dir()` | 获取可观测性目录 | 无 | `PathBuf` |
| `temp_dir()` | 获取临时目录 | 无 | `PathBuf` |

#### 使用示例

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

### 原子写入操作

原子写入使用临时文件确保数据完整性：

```rust
use dmsc::prelude::*;
use std::path::PathBuf;

let fs = DMSCFileSystem::new_with_root(PathBuf::from("."))?;

fs.atomic_write_text("config/settings.txt", "database_url = localhost\nport = 8080")?;
fs.atomic_write_bytes("binary/data.bin", &[0x00, 0x01, 0x02, 0x03])?;

println!("Files written atomically");
```

### JSON 操作

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

### 目录操作

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

### 文件操作

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

## 分类目录

</div>

文件系统支持分类目录组织，便于文件分类管理：

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

### 确保分类路径

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let log_file = fs.ensure_category_path("logs", "app.log")?;
let cache_file = fs.ensure_category_path("cache", "user_data.json")?;

println!("Log file path: {:?}", log_file);
println!("Cache file path: {:?}", cache_file);
```

### 标准化路径

```rust
use dmsc::prelude::*;

let fs = DMSCFileSystem::new_auto_root()?;

let normalized = fs.normalize_under_category("logs", "/var/log/../log/old.log")?;
println!("Normalized path: {:?}", normalized);
```

<div align="center">

## 应用数据目录

</div>

应用数据目录用于存储应用私有数据：

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

## 配置选项

</div>

文件系统的配置主要通过构造函数参数控制：

| 参数 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `project_root` | `PathBuf` | 项目根目录 |
| `app_data_root` | `PathBuf` | 应用数据目录（默认在项目根目录下创建.dms） |

<div align="center">

## 最佳实践

</div>

1. **使用原子写入**：对重要数据使用原子写入操作，避免数据损坏
2. **使用分类目录**：按功能分类组织文件，便于管理和清理
3. **及时清理临时文件**：使用temp_dir存储临时文件，定期清理
4. **使用JSON进行配置**：使用JSON格式存储配置，便于阅读和编辑
5. **验证文件存在性**：在进行文件操作前检查文件是否存在
6. **使用ensure_parent_dir**：写入文件前确保父目录存在

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
