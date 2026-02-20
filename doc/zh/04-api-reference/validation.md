<div align="center">

# Validation API参考

**Version: 0.1.8**

**Last modified date: 2026-02-20**

validation模块提供数据验证与清理功能，支持多种验证规则和自定义验证器。

## 模块概述

</div>

validation模块包含以下子模块：

- **rules**: 验证规则定义
- **validators**: 验证器实现
- **sanitizers**: 数据清理器

<div align="center">

## 核心组件

</div>

### DMSCValidationModule

验证模块，提供统一的验证功能访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate_email(value)` | 验证邮箱 | `value: String` | `DMSCValidationResult` |
| `validate_username(value)` | 验证用户名 | `value: String` | `DMSCValidationResult` |
| `validate_password(value)` | 验证密码 | `value: String` | `DMSCValidationResult` |
| `validate_url(value)` | 验证URL | `value: String` | `DMSCValidationResult` |
| `validate_ip(value)` | 验证IP地址 | `value: String` | `DMSCValidationResult` |
| `validate_not_empty(field, value)` | 验证非空 | `field: String`, `value: String` | `DMSCValidationResult` |
| `validate_length(field, value, min, max)` | 验证长度 | `field: String`, `value: String`, `min: usize`, `max: usize` | `DMSCValidationResult` |

#### 使用示例

```rust
use dmsc::validation::DMSCValidationModule;

let result = DMSCValidationModule::validate_email("user@example.com".to_string());
if result.is_valid {
    println!("Email is valid");
} else {
    println!("Email is invalid: {:?}", result.errors);
}
```

### DMSCValidationSeverity

验证严重性枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Error` | 错误 |
| `Warning` | 警告 |
| `Info` | 信息 |
| `Critical` | 严重 |

### DMSCValidationError

验证错误结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `field` | `String` | 字段名称 |
| `message` | `String` | 错误消息 |
| `code` | `String` | 错误代码 |
| `severity` | `DMSCValidationSeverity` | 严重级别 |
| `value` | `Option<Value>` | 可选值 |

### DMSCValidationResult

验证结果结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `is_valid` | `bool` | 是否有效 |
| `errors` | `Vec<DMSCValidationError>` | 错误列表 |
| `warnings` | `Vec<DMSCValidationError>` | 警告列表 |

<div align="center">

## 验证构建器

</div>

### DMSCValidatorBuilder

验证器构建器，用于构建复杂的验证规则。

#### 方法

| 方法 | 描述 |
|:--------|:-------------|
| `new(field_name)` | 创建新的验证器 |
| `not_empty()` | 验证非空 |
| `min_length(n)` | 最小长度 |
| `max_length(n)` | 最大长度 |
| `exact_length(n)` | 精确长度 |
| `is_email()` | 验证邮箱格式 |
| `is_url()` | 验证URL格式 |
| `is_ip()` | 验证IP地址格式 |
| `is_uuid()` | 验证UUID格式 |
| `is_base64()` | 验证Base64格式 |
| `min_value(n)` | 最小值 |
| `max_value(n)` | 最大值 |
| `range(min, max)` | 值范围 |
| `matches_regex(pattern)` | 匹配正则表达式 |
| `alphanumeric()` | 验证字母数字 |
| `alphabetic()` | 验证字母 |
| `numeric()` | 验证数字 |
| `lowercase()` | 验证小写 |
| `uppercase()` | 验证大写 |
| `contains(substring)` | 包含子字符串 |
| `starts_with(prefix)` | 以指定前缀开头 |
| `ends_with(suffix)` | 以指定后缀结尾 |
| `is_in(values)` | 值在列表中 |
| `not_in(values)` | 值不在列表中 |
| `build()` | 构建验证器 |

#### 使用示例

```rust
use dmsc::validation::DMSCValidatorBuilder;

let validator = DMSCValidatorBuilder::new("email")
    .not_empty()
    .max_length(255)
    .is_email()
    .build();

let result = validator.validate_value(Some("user@example.com"));
```

### DMSCValidationRunner

验证运行器，用于执行验证。

#### 方法

| 方法 | 描述 |
|:--------|:-------------|
| `validate(value)` | 验证字符串值 |
| `validate_optional(value)` | 验证可选字符串 |

<div align="center">

## 数据清理

</div>

### DMSCSanitizer

数据清理器。

#### 方法

| 方法 | 描述 |
|:--------|:-------------|
| `new()` | 创建新的清理器 |
| `with_config(config)` | 使用配置创建 |
| `sanitize(input)` | 清理输入字符串 |
| `sanitize_email(input)` | 清理邮箱 |
| `sanitize_filename(input)` | 清理文件名 |

### DMSCSanitizationConfig

清理配置。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `trim_whitespace` | `bool` | 去除空白 | `true` |
| `lowercase` | `bool` | 转为小写 | `false` |
| `uppercase` | `bool` | 转为大写 | `false` |
| `remove_extra_spaces` | `bool` | 移除多余空格 | `false` |
| `remove_html_tags` | `bool` | 移除HTML标签 | `false` |
| `escape_special_chars` | `bool` | 转义特殊字符 | `false` |

<div align="center">

## 模式验证

</div>

### DMSCSchemaValidator

模式验证器，用于验证数据结构。

```rust
use dmsc::validation::DMSCSchemaValidator;

let schema = r#"{"type": "string", "minLength": 5}"#;
let validator = DMSCSchemaValidator::new(schema.to_string());

let result = validator.validate_json(r#""hello""#.to_string());
```

<div align="center">

## 最佳实践

</div>

1. **使用验证器构建器**: 使用 `DMSCValidatorBuilder` 构建复杂验证规则
2. **提供清晰的错误消息**: 验证失败时返回有意义的错误消息
3. **验证输入数据**: 始终验证用户输入数据
4. **清理敏感数据**: 使用清理器处理敏感信息

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
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
