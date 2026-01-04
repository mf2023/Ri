<div align="center">

# Config API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

config模块提供多源配置管理与热重载功能，支持文件、环境变量等多种配置来源。

## 模块概述

</div>

config模块包含以下子模块：

- **core**: 配置核心接口和类型定义
- **sources**: 配置源实现（文件、环境变量等）
- **validators**: 配置验证器
- **reload**: 配置热重载机制

<div align="center">

## 核心组件

</div>

### DMSCConfig

配置管理器主接口，提供统一的配置访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key)` | 获取配置值 | `key: &str` | `Option<String>` |
| `get_typed<T>(key)` | 获取类型安全的配置值 | `key: &str` | `DMSCResult<T>` |
| `get_or_default(key, default)` | 获取配置值或默认值 | `key: &str`, `default: T` | `T` |
| `set(key, value)` | 设置配置值 | `key: &str`, `value: impl Serialize` | `DMSCResult<()>` |
| `has(key)` | 检查配置是否存在 | `key: &str` | `bool` |
| `keys()` | 获取所有配置键 | 无 | `Vec<String>` |
| `reload()` | 重新加载配置 | 无 | `DMSCResult<()>` |
| `watch(key, callback)` | 监听配置变化 | `key: &str`, `callback: impl Fn(&str)` | `DMSCResult<()>` |
| `validate()` | 验证配置完整性 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dms::prelude::*;

// 获取字符串配置
let service_name = ctx.config().get("service.name").unwrap_or("default");

// 获取类型安全的配置
let port: u16 = ctx.config().get_typed("service.port")?;
let max_connections: usize = ctx.config().get_typed("database.max_connections")?;

// 获取配置或默认值
let timeout = ctx.config().get_or_default("service.timeout", 30);

// 检查配置是否存在
if ctx.config().has("feature.new_feature") {
    // 启用新功能
}

// 获取所有配置键
let keys = ctx.config().keys();
for key in keys {
    println!("Config key: {}", key);
}
```

### DMSCConfigSource

配置源枚举类型。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `File(path)` | 文件配置源 |
| `Env(prefix)` | 环境变量配置源 |
| `Http(url)` | HTTP配置源 |
| `Database(connection)` | 数据库配置源 |
| `Custom(name, data)` | 自定义配置源 |

### DMSCConfigBuilder

配置构建器，用于构建配置管理器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的配置构建器 | 无 | `Self` |
| `add_source(source)` | 添加配置源 | `source: DMSCConfigSource` | `Self` |
| `set_default(key, value)` | 设置默认值 | `key: &str`, `value: impl Serialize` | `Self` |
| `add_validator(validator)` | 添加验证器 | `validator: impl ConfigValidator` | `Self` |
| `enable_hot_reload()` | 启用热重载 | 无 | `Self` |
| `set_reload_interval(seconds)` | 设置重载间隔 | `seconds: u64` | `Self` |
| `build()` | 构建配置管理器 | 无 | `DMSCResult<DMSCConfig>` |

#### 使用示例

```rust
use dms::prelude::*;

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))
    .set_default("service.port", 8080)
    .set_default("service.host", "localhost")
    .enable_hot_reload()
    .set_reload_interval(60)
    .build()?;
```

## 配置源

### 文件配置

#### YAML文件

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"
  port: 8080
  host: "0.0.0.0"

database:
  url: "postgres://localhost/mydb"
  max_connections: 100
  timeout: 30

logging:
  level: "info"
  format: "json"
```

#### JSON文件

```json
{
  "service": {
    "name": "my-service",
    "version": "1.0.0",
    "port": 8080,
    "host": "0.0.0.0"
  },
  "database": {
    "url": "postgres://localhost/mydb",
    "max_connections": 100,
    "timeout": 30
  }
}
```

#### TOML文件

```toml
[service]
name = "my-service"
version = "1.0.0"
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://localhost/mydb"
max_connections = 100
timeout = 30
```

### 环境变量配置

```bash
# 基本环境变量
export SERVICE_NAME=my-service
export SERVICE_PORT=8080
export DATABASE_URL=postgres://localhost/mydb

# 带前缀的环境变量
export DMSC_SERVICE_NAME=my-service
export DMSC_SERVICE_PORT=8080
export DMSC_DATABASE_URL=postgres://localhost/mydb
```

### 配置优先级

配置源的优先级从高到低：

1. **环境变量** (最高优先级)
2. **配置文件** (中等优先级)
3. **默认值** (最低优先级)

```rust
let config = DMSCConfigBuilder::new()
    .set_default("service.port", 3000)                    // 默认值
    .add_source(DMSCConfigSource::File("config.yaml".to_string())) // 配置文件
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))        // 环境变量
    .build()?;

// 优先级：环境变量 > 配置文件 > 默认值
```
<div align="center">

## 类型安全的配置访问

</div>

### 基本类型

```rust
// 字符串
let name: String = ctx.config().get_typed("service.name")?;

// 数字类型
let port: u16 = ctx.config().get_typed("service.port")?;
let max_connections: usize = ctx.config().get_typed("database.max_connections")?;
let timeout: f64 = ctx.config().get_typed("service.timeout")?;

// 布尔类型
let debug_mode: bool = ctx.config().get_typed("service.debug")?;
let enable_feature: bool = ctx.config().get_typed("feature.enabled")?;
```

### 复杂类型

```rust
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: usize,
    timeout: u64,
}

// 获取结构体配置
let db_config: DatabaseConfig = ctx.config().get_typed("database")?;

// 获取数组配置
let allowed_hosts: Vec<String> = ctx.config().get_typed("security.allowed_hosts")?;

// 获取映射配置
let feature_flags: HashMap<String, bool> = ctx.config().get_typed("features")?;
```

### 可选类型

```rust
// 获取可选配置
let port: Option<u16> = ctx.config().get_typed("service.port").ok();
let timeout: Option<u64> = ctx.config().get_typed("service.timeout").ok();

// 使用unwrap_or提供默认值
let port = ctx.config().get_typed("service.port").unwrap_or(8080);
let timeout = ctx.config().get_typed("service.timeout").unwrap_or(30);
```
<div align="center">

## 配置验证

</div>  

### 内置验证器

```rust
use dms::prelude::*;

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_validator(RequiredValidator::new(vec![
        "service.name",
        "service.port",
        "database.url",
    ]))
    .add_validator(RangeValidator::new("service.port", 1, 65535))
    .add_validator(RegexValidator::new("service.host", r"^[a-zA-Z0-9.-]+$"))
    .build()?;
```

### 自定义验证器

```rust
use dms::prelude::*;

struct CustomValidator;

impl ConfigValidator for CustomValidator {
    fn validate(&self, config: &DMSCConfig) -> DMSCResult<()> {
        let port: u16 = config.get_typed("service.port")?;
        let host: String = config.get_typed("service.host")?;
        
        if port < 1024 && host != "localhost" {
            return Err(DMSCError::new("INVALID_CONFIG", "Privileged ports require localhost"));
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "custom_validator"
    }
}

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_validator(CustomValidator)
    .build()?;
```

<div align="center">

## 热重载

</div>  

### 启用热重载

```rust
let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .enable_hot_reload()
    .set_reload_interval(60) // 每60秒检查一次
    .build()?;

// 手动触发重载
config.reload().await?;
```

### 监听配置变化

```rust
// 监听特定配置变化
config.watch("service.port", |new_value| {
    println!("Port changed to: {}", new_value);
    // 重启服务或更新配置
}).await?;

// 监听所有配置变化
config.watch_all(|changes| {
    for (key, old_value, new_value) in changes {
        println!("Config {} changed from {:?} to {:?}", key, old_value, new_value);
    }
}).await?;
```

<div align="center">

## 配置模板

</div>  

### 环境特定配置

```rust
// config.dev.yaml
service:
  name: "my-service-dev"
  port: 3000
  debug: true

// config.prod.yaml  
service:
  name: "my-service"
  port: 8080
  debug: false
```

### 配置继承

```yaml
# base.yaml
service:
  name: "my-service"
  version: "1.0.0"

# config.yaml
import: "base.yaml"
service:
  port: 8080  # 覆盖基础配置
```
<div align="center">

## 配置加密

</div>  

### 敏感信息加密

```rust
// 加密配置值
let encrypted_value = encrypt_config_value("secret-password", &encryption_key)?;
config.set("database.password", encrypted_value)?;

// 解密配置值
let decrypted_value = decrypt_config_value(&encrypted_value, &encryption_key)?;
```

### 使用密钥管理服务

```rust
// 从AWS Secrets Manager获取配置
let secret_config = get_secret_from_aws("my-service/config").await?;
config.merge(secret_config)?;

// 从HashiCorp Vault获取配置
let vault_config = get_secret_from_vault("secret/my-service").await?;
config.merge(vault_config)?;
```

<div align="center">

## 配置调试

</div>      

### 配置信息

```rust
// 获取配置信息
let info = config.get_info()?;
println!("Config sources: {:?}", info.sources);
println!("Last reload: {:?}", info.last_reload);
println!("Total keys: {}", info.total_keys);

// 导出配置
let exported = config.export()?;
println!("Current config: {}", exported);
```

### 配置差异

```rust
// 比较配置差异
let diff = config.compare_with_file("new-config.yaml")?;
for change in diff {
    match change.change_type {
        ConfigChangeType::Added => println!("Added: {}", change.key),
        ConfigChangeType::Modified => println!("Modified: {} ({} -> {})", change.key, change.old_value, change.new_value),
        ConfigChangeType::Removed => println!("Removed: {}", change.key),
    }
}
```

<div align="center">

## 错误处理

</div>  

### 配置错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `CONFIG_FILE_NOT_FOUND` | 配置文件未找到 |
| `CONFIG_PARSE_ERROR` | 配置解析错误 |
| `CONFIG_VALIDATION_FAILED` | 配置验证失败 |
| `CONFIG_TYPE_ERROR` | 配置类型错误 |
| `CONFIG_SOURCE_ERROR` | 配置源错误 |

### 错误处理示例

```rust
match ctx.config().get_typed::<u16>("service.port") {
    Ok(port) => {
        // 配置正确
        println!("Service port: {}", port);
    }
    Err(DMSCError { code, .. }) if code == "CONFIG_TYPE_ERROR" => {
        // 类型错误，使用默认值
        let port: u16 = 8080;
        println!("Using default port: {}", port);
    }
    Err(e) => {
        // 其他错误
        return Err(e);
    }
}
```

<div align="center">

## 最佳实践

</div>  

1. **使用类型安全的配置访问**: 避免手动类型转换
2. **提供合理的默认值**: 确保应用在缺少配置时能正常运行
3. **验证配置完整性**: 在应用启动时验证所有必需的配置
4. **使用环境变量覆盖**: 在不同环境中使用环境变量覆盖配置文件
5. **启用热重载**: 对于需要动态调整的配置启用热重载
6. **加密敏感信息**: 对密码、密钥等敏感信息进行加密
7. **使用配置模板**: 为不同环境创建配置模板
8. **记录配置变化**: 监听和记录配置变化，便于审计
9. **注意配置修改的安全时机**:
   - **启动阶段**: 所有配置都可以安全修改
   - **运行阶段**: 只有标记为"可动态修改"的配置才能安全修改
   - **敏感模块**: 网关、认证、服务网格等核心模块的配置修改需要特别谨慎
   - **重启要求**: 某些配置修改后需要重启服务才能生效

<div align="center">

## 配置修改的安全时机

</div>  

### 可安全动态修改的配置

以下类型的配置通常可以安全地在运行时修改：

- **日志级别**: 可以动态调整日志输出级别
- **监控配置**: 可以动态调整监控采样率和告警阈值
- **超时设置**: 可以动态调整请求超时时间
- **限流配置**: 可以动态调整速率限制
- **缓存配置**: 可以动态调整缓存大小和TTL
- **功能开关**: 可以动态启用或禁用功能

### 需要谨慎修改的配置

以下类型的配置修改需要特别谨慎，可能影响系统稳定性：

- **认证配置**: 可能导致用户无法登录或权限失效
- **数据库配置**: 可能导致数据库连接中断
- **网络配置**: 可能导致服务间通信中断
- **安全配置**: 可能导致安全漏洞
- **核心组件配置**: 可能导致系统崩溃

### 修改配置的最佳时机

1. **应用启动前**: 修改所有需要重启才能生效的配置
2. **低峰期**: 在系统负载较低时修改配置
3. **逐步修改**: 对于关键配置，先在非生产环境测试，然后逐步推广到生产环境
4. **监控修改**: 修改配置后密切监控系统指标
5. **回滚机制**: 准备好配置回滚方案，以便在出现问题时快速恢复

### 配置修改的影响范围

| 配置类型 | 影响范围 | 是否需要重启 |
|:--------|:-------------|:--------|
| 日志级别 | 全局 | 否 |
| 监控配置 | 全局 | 否 |
| 超时设置 | 局部 | 否 |
| 限流配置 | 局部 | 否 |
| 缓存配置 | 局部 | 否 |
| 功能开关 | 局部/全局 | 否 |
| 认证配置 | 全局 | 是 |
| 数据库配置 | 全局 | 是 |
| 网络配置 | 全局 | 是 |
| 安全配置 | 全局 | 是 |
| 核心组件配置 | 全局 | 是 |

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [cache](./cache.md): 缓存模块，提供多后端缓存抽象，缓存用户会话和权限数据
- [database](./database.md): 数据库模块，提供用户数据持久化和查询功能
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [security](./security.md): 安全模块，提供加密、哈希和验证功能
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据