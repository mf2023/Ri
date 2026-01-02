<div align="center">

# FileSystem API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

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
| `__init__(project_root)` | 创建带项目根的文件系统 | `project_root: str` | `DMSCFileSystem` |
| `new_with_root(project_root)` | 创建带项目根的文件系统 | `project_root: str` | `DMSCFileSystem` |
| `new_with_roots(project_root, app_data_root)` | 创建带根目录的文件系统 | `project_root: str`, `app_data_root: str` | `DMSCFileSystem` |
| `new_auto_root()` | 自动检测项目根目录 | 无 | `DMSCFileSystem` |
| `write_text(path, text)` | 写入文本 | `path: str`, `text: str` | `None` |
| `write_bytes(path, data)` | 写入字节 | `path: str`, `data: bytes` | `None` |
| `atomic_write_text(path, text)` | 原子写入文本 | `path: str`, `text: str` | `None` |
| `atomic_write_bytes(path, data)` | 原子写入字节 | `path: str`, `data: bytes` | `None` |
| `read_text(path)` | 读取文本 | `path: str` | `str` |
| `read_json(path)` | 读取JSON | `path: str` | `Any` |
| `write_json(path, value)` | 写入JSON | `path: str`, `value: Any` | `None` |
| `exists(path)` | 检查路径是否存在 | `path: str` | `bool` |
| `remove_file(path)` | 删除文件 | `path: str` | `None` |
| `remove_dir_all(path)` | 删除目录及其内容 | `path: str` | `None` |
| `copy_file(from_path, to_path)` | 复制文件 | `from_path: str`, `to_path: str` | `None` |
| `append_text(path, text)` | 追加文本 | `path: str`, `text: str` | `None` |
| `mkdir(path)` | 创建目录 | `path: str` | `None` |
| `ensure_parent_dir(path)` | 确保父目录存在 | `path: str` | `str` |
| `app_dir()` | 获取应用数据目录 | 无 | `str` |
| `logs_dir()` | 获取日志目录 | 无 | `str` |
| `cache_dir()` | 获取缓存目录 | 无 | `str` |
| `reports_dir()` | 获取报告目录 | 无 | `str` |
| `observability_dir()` | 获取可观测性目录 | 无 | `str` |
| `temp_dir()` | 获取临时目录 | 无 | `str` |

#### 使用示例

```python
from dmsc import DMSCFileSystem

# 创建带项目根的文件系统
fs = DMSCFileSystem.new_with_root(".")

# 原子写入文本
fs.atomic_write_text("example.txt", "Hello, DMSC!")

# 读取文本
content = fs.read_text("example.txt")
print(f"File content: {content}")

# JSON操作
data = {"key": "value", "number": 42, "nested": {"a": 1, "b": 2}}
fs.write_json("example.json", data)

# 读取JSON
loaded_data = fs.read_json("example.json")
print(f"JSON data: {loaded_data}")

# 获取日志目录
logs_dir = fs.logs_dir()
print(f"Logs directory: {logs_dir}")
```

### 原子写入操作

原子写入使用临时文件确保数据完整性：

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_with_root(".")

fs.atomic_write_text("config/settings.txt", "database_url = localhost\nport = 8080")
fs.atomic_write_bytes("binary/data.bin", bytes([0x00, 0x01, 0x02, 0x03]))

print("Files written atomically")
```

### JSON 操作

```python
from dmsc import DMSCFileSystem
import json

fs = DMSCFileSystem.new_with_root(".")

# 写入JSON
config = {
    "name": "MyApp",
    "version": 1,
    "enabled": True,
    "features": ["auth", "cache", "log"]
}
fs.write_json("config.json", config)

# 读取JSON
loaded_config = fs.read_json("config.json")
print(f"Loaded config: {loaded_config}")
print(f"App name: {loaded_config['name']}")
```

### 目录操作

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# 创建目录
fs.mkdir("data/backups")

# 确保父目录存在
fs.ensure_parent_dir("logs/2024/app.log")

# 检查文件是否存在
if fs.exists("data"):
    print("Directory exists")

# 删除目录
fs.remove_dir_all("tmp/old_data")
```

### 文件操作

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_with_root(".")

# 复制文件
fs.copy_file("original.txt", "backup/original.txt")

# 追加文本
fs.append_text("logs/app.log", "New log entry\n")

# 删除文件
if fs.exists("to_delete.txt"):
    fs.remove_file("to_delete.txt")
```

<div align="center">

## 分类目录

</div>

文件系统支持分类目录组织，便于文件分类管理：

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

logs_dir = fs.logs_dir()
cache_dir = fs.cache_dir()
reports_dir = fs.reports_dir()
observability_dir = fs.observability_dir()
temp_dir = fs.temp_dir()

print(f"Logs: {logs_dir}")
print(f"Cache: {cache_dir}")
print(f"Reports: {reports_dir}")
print(f"Observability: {observability_dir}")
print(f"Temp: {temp_dir}")
```

### 确保分类路径

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

log_file = fs.ensure_category_path("logs", "app.log")
cache_file = fs.ensure_category_path("cache", "user_data.json")

print(f"Log file path: {log_file}")
print(f"Cache file path: {cache_file}")
```

<div align="center>

## 应用数据目录

</div>

应用数据目录用于存储应用私有数据：

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

app_dir = fs.app_dir()
print(f"App data directory: {app_dir}")

# 自定义应用目录
custom_fs = DMSCFileSystem.new_with_roots(
    ".",
    "/var/lib/myapp"
)

custom_app_dir = custom_fs.app_dir()
print(f"Custom app directory: {custom_app_dir}")
```

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
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，使用fs模块存储日志文件
- [cache](./cache.md): 缓存模块，使用fs模块管理缓存文件
- [config](./config.md): 配置模块，使用fs模块读取配置文件
- [observability](./observability.md): 可观测性模块，使用fs模块存储监控数据
