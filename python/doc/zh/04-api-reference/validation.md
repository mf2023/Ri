# 验证模块

验证模块提供全面的数据验证功能，包括输入验证、数据类型验证、业务规则验证等。

## 概述

验证模块包含以下核心组件：

- **DMSCValidationConfig**: 验证配置管理
- **DMSCValidationManager**: 验证管理器
- **DMSCValidator**: 基础验证器
- **DMSCSchema**: 数据模式定义
- **DMSCField**: 字段定义
- **DMSCValidationRule**: 验证规则

## 核心类

### DMSCValidationConfig

验证配置类，用于配置验证行为。

#### 构造函数

```python
DMSCValidationConfig(
    strict_mode: bool = False,
    allow_unknown_fields: bool = True,
    validate_on_assignment: bool = True,
    validate_on_load: bool = True,
    validate_on_save: bool = True,
    stop_on_first_error: bool = False,
    error_message_language: str = "en",
    enable_custom_messages: bool = True,
    enable_field_normalization: bool = True,
    enable_type_coercion: bool = True,
    coercion_rules: Dict = None,
    enable_caching: bool = True,
    cache_ttl: int = 3600,
    enable_async_validation: bool = True,
    max_async_workers: int = 4,
    timeout: int = 30,
    enable_remote_validation: bool = False,
    remote_validation_endpoints: List[str] = None,
    enable_profiling: bool = False,
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    enable_logging: bool = True,
    log_level: str = "INFO",
    log_invalid_data: bool = True,
    log_validation_errors: bool = True,
    enable_sanitization: bool = True,
    sanitization_rules: Dict = None,
    enable_whitelist: bool = False,
    field_whitelist: List[str] = None,
    enable_blacklist: bool = False,
    field_blacklist: List[str] = None,
    enable_rate_limiting: bool = True,
    rate_limit_requests: int = 1000,
    rate_limit_window: int = 3600,
    enable_security_checks: bool = True,
    security_rules: Dict = None,
    enable_performance_monitoring: bool = True,
    performance_threshold: int = 100,  # milliseconds
    enable_schema_validation: bool = True,
    schema_validation_level: str = "strict",
    enable_cross_field_validation: bool = True,
    enable_conditional_validation: bool = True,
    enable_custom_validators: bool = True,
    custom_validators_path: str = "",
    enable_validation_chains: bool = True,
    max_validation_chain_length: int = 10,
    enable_result_caching: bool = True,
    result_cache_ttl: int = 1800,
    enable_error_recovery: bool = True,
    error_recovery_strategies: Dict = None,
    enable_validation_history: bool = False,
    validation_history_retention: int = 30  # days
)
```

### DMSCValidationManager

验证管理器，提供统一的验证接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate(data, schema, **kwargs)` | 验证数据 | `data: Any`, `schema: DMSCSchema`, `**kwargs` | `DMSCValidationResult` |
| `validate_field(value, field, **kwargs)` | 验证字段 | `value: Any`, `field: DMSCField`, `**kwargs` | `DMSCValidationResult` |
| `validate_schema(schema, **kwargs)` | 验证模式 | `schema: DMSCSchema`, `**kwargs` | `DMSCValidationResult` |
| `add_validator(name, validator)` | 添加验证器 | `name: str`, `validator: Callable` | `bool` |
| `remove_validator(name)` | 移除验证器 | `name: str` | `bool` |
| `get_validator(name)` | 获取验证器 | `name: str` | `Optional[Callable]` |
| `list_validators()` | 列出验证器 | 无 | `List[str]` |
| `add_rule(name, rule)` | 添加规则 | `name: str`, `rule: DMSCValidationRule` | `bool` |
| `remove_rule(name)` | 移除规则 | `name: str` | `bool` |
| `get_rule(name)` | 获取规则 | `name: str` | `Optional[DMSCValidationRule]` |
| `list_rules()` | 列出规则 | 无 | `List[str]` |
| `validate_async(data, schema, **kwargs)` | 异步验证 | `data: Any`, `schema: DMSCSchema`, `**kwargs` | `Awaitable[DMSCValidationResult]` |
| `batch_validate(data_list, schema, **kwargs)` | 批量验证 | `data_list: List[Any]`, `schema: DMSCSchema`, `**kwargs` | `List[DMSCValidationResult]` |
| `create_schema(name, fields, **kwargs)` | 创建模式 | `name: str`, `fields: List[DMSCField]`, `**kwargs` | `DMSCSchema` |
| `load_schema(schema_dict)` | 加载模式 | `schema_dict: Dict` | `DMSCSchema` |
| `dump_schema(schema)` | 导出模式 | `schema: DMSCSchema` | `Dict` |
| `validate_with_context(data, schema, context, **kwargs)` | 上下文验证 | `data: Any`, `schema: DMSCSchema`, `context: Dict`, `**kwargs` | `DMSCValidationResult` |
| `sanitize(data, schema, **kwargs)` | 清理数据 | `data: Any`, `schema: DMSCSchema`, `**kwargs` | `Any` |
| `normalize(data, schema, **kwargs)` | 标准化数据 | `data: Any`, `schema: DMSCSchema`, `**kwargs` | `Any` |
| `get_validation_history(**kwargs)` | 获取验证历史 | `**kwargs` | `List[Dict]` |
| `clear_validation_history()` | 清除验证历史 | 无 | `bool` |
| `get_validation_stats(**kwargs)` | 获取验证统计 | `**kwargs` | `Dict` |
| `export_validation_rules(format="json")` | 导出验证规则 | `format: str` | `str` |
| `import_validation_rules(rules_data, format="json")` | 导入验证规则 | `rules_data: str`, `format: str` | `bool` |

### DMSCSchema

数据模式类，定义数据结构。

#### 构造函数

```python
DMSCSchema(
    name: str = "",
    description: str = "",
    fields: List[DMSCField] = None,
    validators: List[Callable] = None,
    allow_unknown: bool = True,
    strict: bool = False,
    validate_all: bool = True,
    additional_properties: bool = True,
    required: List[str] = None,
    dependencies: Dict = None,
    metadata: Dict = None
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_field(field)` | 添加字段 | `field: DMSCField` | `bool` |
| `remove_field(name)` | 移除字段 | `name: str` | `bool` |
| `get_field(name)` | 获取字段 | `name: str` | `Optional[DMSCField]` |
| `list_fields()` | 列出字段 | 无 | `List[DMSCField]` |
| `validate(data, **kwargs)` | 验证数据 | `data: Any`, `**kwargs` | `DMSCValidationResult` |
| `to_dict()` | 转换为字典 | 无 | `Dict` |
| `from_dict(data)` | 从字典创建 | `data: Dict` | `bool` |

### DMSCField

字段定义类，定义单个字段的验证规则。

#### 构造函数

```python
DMSCField(
    name: str = "",
    field_type: str = "string",
    required: bool = False,
    default: Any = None,
    validators: List[Callable] = None,
    description: str = "",
    nullable: bool = True,
    empty: bool = True,
    allowed: List[Any] = None,
    forbidden: List[Any] = None,
    min_length: int = None,
    max_length: int = None,
    length: int = None,
    min_value: Union[int, float] = None,
    max_value: Union[int, float] = None,
    regex: str = None,
    coerce: bool = False,
    coerce_type: str = None,
    normalize: bool = True,
    sanitize: bool = True,
    metadata: Dict = None,
    dependencies: List[str] = None,
    exclude: bool = False,
    readonly: bool = False,
    writeonly: bool = False,
    load_only: bool = False,
    dump_only: bool = False,
    error_messages: Dict = None,
    validation_mode: str = "default"
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate(value, **kwargs)` | 验证值 | `value: Any`, `**kwargs` | `DMSCValidationResult` |
| `add_validator(validator)` | 添加验证器 | `validator: Callable` | `bool` |
| `remove_validator(validator)` | 移除验证器 | `validator: Callable` | `bool` |
| `to_dict()` | 转换为字典 | 无 | `Dict` |

### DMSCValidationRule

验证规则类，定义验证逻辑。

#### 构造函数

```python
DMSCValidationRule(
    name: str = "",
    validator: Callable = None,
    description: str = "",
    error_message: str = "",
    severity: str = "error",
    condition: Callable = None,
    metadata: Dict = None,
    async_validator: Callable = None,
    timeout: int = 30,
    retry_attempts: int = 3,
    retry_delay: float = 1.0,
    cache_result: bool = False,
    cache_ttl: int = 3600,
    enable_profiling: bool = False,
    enable_metrics: bool = True,
    enable_tracing: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate(value, **kwargs)` | 执行验证 | `value: Any`, `**kwargs` | `DMSCValidationResult` |
| `validate_async(value, **kwargs)` | 异步验证 | `value: Any`, `**kwargs` | `Awaitable[DMSCValidationResult]` |
| `should_apply(context)` | 检查是否应用 | `context: Dict` | `bool` |

### DMSCValidationResult

验证结果类，包含验证结果信息。

#### 属性

| 属性 | 类型 | 描述 |
|:--------|:--------|:--------|
| `is_valid` | `bool` | 是否验证通过 |
| `errors` | `List[DMSCValidationError]` | 错误列表 |
| `warnings` | `List[DMSCValidationWarning]` | 警告列表 |
| `data` | `Any` | 验证后的数据 |
| `metadata` | `Dict` | 元数据 |
| `validation_time` | `float` | 验证耗时 |
| `timestamp` | `float` | 时间戳 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_error(error)` | 添加错误 | `error: DMSCValidationError` | `bool` |
| `add_warning(warning)` | 添加警告 | `warning: DMSCValidationWarning` | `bool` |
| `to_dict()` | 转换为字典 | 无 | `Dict` |
| `to_json()` | 转换为JSON | 无 | `str` |

## 使用示例

### 基本验证

```python
from dmsc import DMSCValidationManager, DMSCSchema, DMSCField

# 创建验证管理器
validator = DMSCValidationManager()

# 创建模式
schema = DMSCSchema(name="UserSchema")
schema.add_field(DMSCField("name", field_type="string", required=True, min_length=2, max_length=50))
schema.add_field(DMSCField("email", field_type="string", required=True, regex=r"^[\w\.-]+@[\w\.-]+\.\w+$"))
schema.add_field(DMSCField("age", field_type="integer", min_value=18, max_value=120))

# 验证数据
data = {
    "name": "John Doe",
    "email": "john@example.com",
    "age": 25
}

result = validator.validate(data, schema)
print(f"Valid: {result.is_valid}")
print(f"Errors: {result.errors}")
```

### 自定义验证器

```python
# 定义自定义验证器
def validate_password_strength(value):
    if len(value) < 8:
        return False, "Password must be at least 8 characters long"
    if not any(c.isupper() for c in value):
        return False, "Password must contain at least one uppercase letter"
    if not any(c.isdigit() for c in value):
        return False, "Password must contain at least one digit"
    return True, ""

# 添加自定义验证器
validator.add_validator("password_strength", validate_password_strength)

# 创建密码字段
password_field = DMSCField(
    "password",
    field_type="string",
    required=True,
    validators=["password_strength"]
)

# 验证密码
result = validator.validate_field("MySecure123", password_field)
print(f"Valid: {result.is_valid}")
print(f"Errors: {result.errors}")
```

### 异步验证

```python
import asyncio

# 定义异步验证器
async def validate_username_availability(value):
    # 模拟异步检查
    await asyncio.sleep(0.1)
    # 假设 "admin" 是已存在的用户名
    if value.lower() == "admin":
        return False, "Username already exists"
    return True, ""

# 创建用户名字段
username_field = DMSCField(
    "username",
    field_type="string",
    required=True,
    min_length=3,
    max_length=20
)

# 异步验证
async def validate_username():
    result = await validator.validate_field_async("admin", username_field)
    print(f"Valid: {result.is_valid}")
    print(f"Errors: {result.errors}")

# 运行异步验证
asyncio.run(validate_username())
```

### 嵌套验证

```python
# 创建地址模式
address_schema = DMSCSchema(name="AddressSchema")
address_schema.add_field(DMSCField("street", field_type="string", required=True))
address_schema.add_field(DMSCField("city", field_type="string", required=True))
address_schema.add_field(DMSCField("zipcode", field_type="string", regex=r"^\d{5}$"))

# 创建用户模式（包含地址）
user_schema = DMSCSchema(name="UserWithAddressSchema")
user_schema.add_field(DMSCField("name", field_type="string", required=True))
user_schema.add_field(DMSCField("email", field_type="string", required=True))
user_schema.add_field(DMSCField("address", field_type="dict", schema=address_schema))

# 验证嵌套数据
data = {
    "name": "John Doe",
    "email": "john@example.com",
    "address": {
        "street": "123 Main St",
        "city": "New York",
        "zipcode": "10001"
    }
}

result = validator.validate(data, user_schema)
print(f"Valid: {result.is_valid}")
```

### 条件验证

```python
# 创建条件验证规则
def validate_credit_card(value, context):
    payment_method = context.get("payment_method", "")
    if payment_method == "credit_card" and not value:
        return False, "Credit card number is required for credit card payment"
    return True, ""

# 创建支付模式
payment_schema = DMSCSchema(name="PaymentSchema")
payment_schema.add_field(DMSCField("payment_method", field_type="string", required=True, allowed=["credit_card", "paypal", "bank_transfer"]))
payment_schema.add_field(DMSCField("credit_card_number", field_type="string", validators=[validate_credit_card]))

# 验证数据
context = {"payment_method": "credit_card"}
result = validator.validate_with_context({"credit_card_number": ""}, payment_schema, context)
print(f"Valid: {result.is_valid}")
print(f"Errors: {result.errors}")
```

### 批量验证

```python
# 创建用户数据列表
users = [
    {"name": "John Doe", "email": "john@example.com", "age": 25},
    {"name": "Jane Smith", "email": "jane@example.com", "age": 30},
    {"name": "", "email": "invalid-email", "age": 15}  # 无效数据
]

# 批量验证
results = validator.batch_validate(users, user_schema)
for i, result in enumerate(results):
    print(f"User {i+1}: Valid={result.is_valid}, Errors={result.errors}")
```

### 数据清理和标准化

```python
# 创建包含清理规则的模式
schema = DMSCSchema(name="CleanDataSchema")
schema.add_field(DMSCField("email", field_type="string", normalize=True, sanitize=True))
schema.add_field(DMSCField("phone", field_type="string", normalize=True))

# 原始数据
data = {
    "email": "  JOHN@EXAMPLE.COM  ",
    "phone": "(123) 456-7890"
}

# 清理数据
cleaned_data = validator.sanitize(data, schema)
print(f"Cleaned data: {cleaned_data}")

# 标准化数据
normalized_data = validator.normalize(data, schema)
print(f"Normalized data: {normalized_data}")
```

## 最佳实践

1. **模式定义**: 明确定义数据模式，包含所有必要的字段和验证规则
2. **错误处理**: 提供清晰、有用的错误消息
3. **性能优化**: 使用缓存和异步验证提高性能
4. **安全考虑**: 验证所有用户输入，防止注入攻击
5. **类型安全**: 使用适当的类型验证和强制转换
6. **验证顺序**: 按照从简单到复杂的顺序进行验证
7. **上下文验证**: 使用上下文信息进行条件验证
8. **测试覆盖**: 为验证逻辑编写充分的测试用例

## 注意事项

- 验证规则应该清晰明确，避免歧义
- 考虑验证性能，避免过于复杂的验证逻辑
- 提供详细的错误信息，帮助用户理解验证失败原因
- 定期更新验证规则以适应业务变化
- 监控验证性能，及时发现性能瓶颈
- 实施适当的错误恢复机制