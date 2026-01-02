<div align="center">

# FileSystem API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The fs module provides secure file system operation functionality, supporting atomic writes, directory management, and categorized directory organization.

## Module Overview

</div>

The fs module is DMSC's file system abstraction layer, providing the following core features:

- **Atomic Writes**: Use temporary files to ensure write operation atomicity
- **Directory Management**: Automatically create necessary directory structures
- **Categorized Directories**: Support for logs, cache, reports, observability, and other categorized directories
- **JSON Support**: Built-in JSON serialization and deserialization support
- **Secure Operations**: All file operations are securely handled

<div align="center">

## Core Components

</div>

### DMSCFileSystem

File system main interface, providing unified file operation functionality.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `__init__(project_root)` | Create file system with project root | `project_root: str` | `DMSCFileSystem` |
| `new_with_root(project_root)` | Create file system with project root | `project_root: str` | `DMSCFileSystem` |
| `new_with_roots(project_root, app_data_root)` | Create file system with roots | `project_root: str`, `app_data_root: str` | `DMSCFileSystem` |
| `new_auto_root()` | Auto-detect project root | None | `DMSCFileSystem` |
| `write_text(path, text)` | Write text | `path: str`, `text: str` | `None` |
| `write_bytes(path, data)` | Write bytes | `path: str`, `data: bytes` | `None` |
| `atomic_write_text(path, text)` | Atomic write text | `path: str`, `text: str` | `None` |
| `atomic_write_bytes(path, data)` | Atomic write bytes | `path: str`, `data: bytes` | `None` |
| `read_text(path)` | Read text | `path: str` | `str` |
| `read_json(path)` | Read JSON | `path: str` | `Any` |
| `write_json(path, value)` | Write JSON | `path: str`, `value: Any` | `None` |
| `exists(path)` | Check if path exists | `path: str` | `bool` |
| `remove_file(path)` | Remove file | `path: str` | `None` |
| `remove_dir_all(path)` | Remove directory and contents | `path: str` | `None` |

#### Usage Example

```python
from dmsc import DMSCFileSystem

# Create file system with project root
fs = DMSCFileSystem.new_auto_root()

# Write text file
fs.write_text("logs/app.txt", "Hello, DMSC!")

# Read text file
content = fs.read_text("logs/app.txt")
print(f"Content: {content}")

# Write JSON file
data = {"name": "MyApp", "version": "1.0.0"}
fs.write_json("config/app.json", data)

# Read JSON file
config = fs.read_json("config/app.json")
print(f"Config: {config}")
```

## Atomic Writes

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# Atomic write ensures data integrity
# If write fails, original file remains unchanged
fs.atomic_write_text(
    "data/important.txt",
    "Critical data that must not be corrupted"
)

# For binary data
binary_data = b"\x00\x01\x02\x03\xff\xfe\xfd"
fs.atomic_write_bytes("data/binary.bin", binary_data)
```

## Categorized Directories

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# DMSC provides categorized directories
logs_dir = fs.get_logs_dir()
cache_dir = fs.get_cache_dir()
reports_dir = fs.get_reports_dir()
observability_dir = fs.get_observability_dir()
data_dir = fs.get_data_dir()
temp_dir = fs.get_temp_dir()

# Write to categorized directories
fs.write_text(f"{logs_dir}/app.log", "Application started")
fs.write_json(f"{cache_dir}/session.json", {"session_id": "abc123"})
```

## File Operations

### Check File Existence

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# Check if file exists
if fs.exists("config/app.json"):
    config = fs.read_json("config/app.json")
    print(f"Config loaded: {config}")
else:
    print("Config file not found")
```

### Remove Files and Directories

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# Remove single file
fs.remove_file("temp/old_file.txt")

# Remove directory and all contents
fs.remove_dir_all("temp/cache")
```

### Directory Structure

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# Get project structure
structure = fs.get_project_structure()
print("Project structure:")
for path in structure:
    print(f"  - {path}")

# Get application data directory
app_data = fs.get_app_data_dir()
print(f"App data directory: {app_data}")
```

## JSON Operations

```python
from dmsc import DMSCFileSystem

fs = DMSCFileSystem.new_auto_root()

# Write complex JSON data
complex_data = {
    "app": "MyDMSCApp",
    "version": "1.0.0",
    "config": {
        "debug": True,
        "port": 8080,
        "features": ["auth", "cache", "logging"]
    },
    "metadata": {
        "created": "2024-01-15",
        "author": "Developer"
    }
}

# Write and read JSON
fs.write_json("config/complex.json", complex_data)
loaded_data = fs.read_json("config/complex.json")

# Verify data integrity
assert loaded_data == complex_data
print("JSON data written and read successfully")
```

## Best Practices

1. **Use Atomic Writes**: Always use atomic writes for important data
2. **Use Categorized Directories**: Use categorized directories for organization
3. **Handle Errors**: Always handle file operation errors gracefully
4. **Check Existence**: Check if files exist before reading
5. **Use JSON for Configuration**: Use JSON for structured configuration
6. **Clean Up Temp Files**: Clean up temporary files regularly
7. **Use Paths Correctly**: Use proper path handling across platforms
8. **Monitor Disk Space**: Monitor disk space usage in production
9. **Secure Sensitive Files**: Secure sensitive files with proper permissions
10. **Backup Important Data**: Backup important data regularly
