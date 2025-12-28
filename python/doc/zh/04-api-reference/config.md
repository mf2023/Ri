<div align="center">

# Config API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

config模块提供统一的配置管理功能，支持多种配置源和动态配置更新。

## 模块概述

</div>

config模块包含以下子模块：

- **sources**: 配置源实现（文件、环境变量、数据库等）
- **validators**: 配置验证器
- **transformers**: 配置转换器
- **encryptors**: 配置加密器
- **reloaders**: 配置重载器
- **watchers**: 配置监视器

<div align="center">

## 核心组件

</div>

### DMSCConfig

配置管理器，提供统一的配置接口。

#### 构造函数

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

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key, default=None)` | 获取配置值 | `key: str`, `default: Any` | `Any` |
| `get_string(key, default="")` | 获取字符串配置 | `key: str`, `default: str` | `str` |
| `get_int(key, default=0)` | 获取整数配置 | `key: str`, `default: int` | `int` |
| `get_float(key, default=0.0)` | 获取浮点数配置 | `key: str`, `default: float` | `float` |
| `get_bool(key, default=False)` | 获取布尔配置 | `key: str`, `default: bool` | `bool` |
| `get_list(key, default=None)` | 获取列表配置 | `key: str`, `default: List` | `List` |
| `get_dict(key, default=None)` | 获取字典配置 | `key: str`, `default: Dict` | `Dict` |
| `set(key, value)` | 设置配置值 | `key: str`, `value: Any` | `bool` |
| `set_many(mapping)` | 批量设置配置 | `mapping: Dict[str, Any]` | `bool` |
| `has(key)` | 检查配置是否存在 | `key: str` | `bool` |
| `delete(key)` | 删除配置 | `key: str` | `bool` |
| `clear()` | 清空配置 | `None` | `bool` |
| `keys()` | 获取所有配置键 | `None` | `List[str]` |
| `reload()` | 重载配置 | `None` | `bool` |
| `validate()` | 验证配置 | `None` | `bool` |
| `save()` | 保存配置 | `None` | `bool` |
| `export()` | 导出配置 | `None` | `Dict` |
| `import(data)` | 导入配置 | `data: Dict` | `bool` |
| `add_source(source)` | 添加配置源 | `source: DMSCConfigSource` | `bool` |
| `remove_source(source_id)` | 移除配置源 | `source_id: str` | `bool` |
| `get_sources()` | 获取所有配置源 | `None` | `List[DMSCConfigSource]` |
| `watch(key, callback)` | 监视配置变化 | `key: str`, `callback: Callable` | `bool` |
| `unwatch(key)` | 取消监视 | `key: str` | `bool` |

#### 使用示例

```python
from dmsc import DMSCConfig, DMSCFileSource, DMSCEnvSource

# 创建配置源
file_source = DMSCFileSource("config.yaml")
env_source = DMSCEnvSource(prefix="DMSC_")

# 初始化配置管理器
config = DMSCConfig(
    sources=[file_source, env_source],
    auto_reload=True,
    environment="production"
)

# 获取配置值
database_host = config.get_string("database.host", "localhost")
database_port = config.get_int("database.port", 5432)
database_ssl = config.get_bool("database.ssl", True)

# 获取复杂配置
features = config.get_list("app.features", [])
security_config = config.get_dict("security", {})

# 设置配置值
config.set("app.debug", False)
config.set_many({
    "database.pool_size": 20,
    "cache.ttl": 3600,
    "logging.level": "INFO"
})

# 监视配置变化
config.watch("database.host", lambda old, new: print(f"Database host changed from {old} to {new}"))
```

### DMSCFileSource

文件配置源，从文件加载配置。

#### 构造函数

```python
DMSCFileSource(
    file_path: str,
    format: str = "auto",
    encoding: str = "utf-8",
    watch_file: bool = True,
    required: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `load()` | 加载配置 | `None` | `Dict` |
| `save(data)` | 保存配置 | `data: Dict` | `bool` |
| `validate()` | 验证配置源 | `None` | `bool` |
| `get_info()` | 获取配置源信息 | `None` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCFileSource

# 创建YAML文件配置源
yaml_source = DMSCFileSource("config.yaml", format="yaml")

# 创建JSON文件配置源
json_source = DMSCFileSource("config.json", format="json")

# 创建TOML文件配置源
toml_source = DMSCFileSource("config.toml", format="toml")

# 自动检测格式
auto_source = DMSCFileSource("config.conf", format="auto")

# 加载配置
data = yaml_source.load()
print(f"Loaded config: {data}")
```

### DMSCEnvSource

环境变量配置源，从环境变量加载配置。

#### 构造函数

```python
DMSCEnvSource(
    prefix: str = "",
    separator: str = "_",
    case_sensitive: bool = False,
    required_vars: List[str] = None
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `load()` | 加载配置 | `None` | `Dict` |
| `get_env_var(key)` | 获取环境变量 | `key: str` | `str` |
| `set_env_var(key, value)` | 设置环境变量 | `key: str`, `value: str` | `bool` |
| `validate()` | 验证配置源 | `None` | `bool` |

#### 使用示例

```python
from dmsc import DMSCEnvSource

# 创建环境变量配置源
env_source = DMSCEnvSource(prefix="DMSC_")

# 设置环境变量
import os
os.environ["DMSC_DATABASE_HOST"] = "localhost"
os.environ["DMSC_DATABASE_PORT"] = "5432"
os.environ["DMSC_APP_DEBUG"] = "true"

# 加载配置
data = env_source.load()
print(f"Loaded env config: {data}")

# 获取特定环境变量
db_host = env_source.get_env_var("DATABASE_HOST")
print(f"Database host: {db_host}")
```

### DMSCDatabaseSource

数据库配置源，从数据库加载配置。

#### 构造函数

```python
DMSCDatabaseSource(
    connection_string: str,
    table: str = "config",
    key_column: str = "key",
    value_column: str = "value",
    refresh_interval: int = 300,
    cache_ttl: int = 60
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `load()` | 加载配置 | `None` | `Dict` |
| `save(data)` | 保存配置 | `data: Dict` | `bool` |
| `update(key, value)` | 更新配置 | `key: str`, `value: Any` | `bool` |
| `delete(key)` | 删除配置 | `key: str` | `bool` |
| `validate()` | 验证配置源 | `None` | `bool` |

#### 使用示例

```python
from dmsc import DMSCDatabaseSource

# 创建数据库配置源
db_source = DMSCDatabaseSource(
    connection_string="postgresql://user:pass@localhost/db",
    table="app_config",
    key_column="config_key",
    value_column="config_value"
)

# 加载配置
data = db_source.load()
print(f"Loaded database config: {data}")

# 更新配置
db_source.update("app.debug", "false")
```

### DMSCCachedSource

缓存配置源，为其他配置源提供缓存功能。

#### 构造函数

```python
DMSCCachedSource(
    source: DMSCConfigSource,
    cache_ttl: int = 300,
    cache_key_prefix: str = "config"
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `load()` | 加载配置 | `None` | `Dict` |
| `invalidate_cache()` | 失效缓存 | `None` | `bool` |
| `get_cache_stats()` | 获取缓存统计 | `None` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCCachedSource, DMSCFileSource

# 创建文件配置源
file_source = DMSCFileSource("config.yaml")

# 创建缓存配置源
cached_source = DMSCCachedSource(
    source=file_source,
    cache_ttl=600  # 10分钟缓存
)

# 加载配置（会缓存结果）
data = cached_source.load()
print(f"Loaded cached config: {data}")

# 获取缓存统计
stats = cached_source.get_cache_stats()
print(f"Cache stats: {stats}")
```

### DMSCConfigValidator

配置验证器，验证配置的正确性。

#### 构造函数

```python
DMSCConfigValidator(
    schema: Dict,
    strict: bool = False,
    allow_extra: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate(config)` | 验证配置 | `config: Dict` | `bool` |
| `validate_key(key, value)` | 验证配置键 | `key: str`, `value: Any` | `bool` |
| `get_errors()` | 获取验证错误 | `None` | `List[str]` |
| `get_schema()` | 获取验证模式 | `None` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCConfigValidator

# 定义配置模式
schema = {
    "app": {
        "name": {"type": "string", "required": True},
        "debug": {"type": "boolean", "default": False},
        "port": {"type": "integer", "min": 1, "max": 65535, "default": 8080}
    },
    "database": {
        "host": {"type": "string", "default": "localhost"},
        "port": {"type": "integer", "default": 5432},
        "username": {"type": "string", "required": True},
        "password": {"type": "string", "required": True}
    }
}

# 创建配置验证器
validator = DMSCConfigValidator(schema=schema, strict=True)

# 验证配置
config = {
    "app": {
        "name": "MyApp",
        "debug": True,
        "port": 8080
    },
    "database": {
        "host": "localhost",
        "port": 5432,
        "username": "admin",
        "password": "secret"
    }
}

if validator.validate(config):
    print("Configuration is valid")
else:
    errors = validator.get_errors()
    print(f"Validation errors: {errors}")
```