<div align="center">

# Validation API参考

**Version: 0.1.4**

**Last modified date: 2026-01-15**

validation模块提供数据验证与清理功能，支持多种验证规则和自定义验证器。

## 模块概述

</div>

validation模块包含以下子模块：

- **rules**: 验证规则定义
- **validators**: 验证器实现
- **sanitizers**: 数据清理器
- **format**: 格式验证
- **constraints**: 约束条件
- **custom**: 自定义验证

<div align="center">

## 核心组件

</div>

### DMSCValidationManager

验证管理器主接口，提供统一的验证功能访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `validate(data, rules)` | 数据验证 | `data: &Value`, `rules: &[DMSCValidationRule]` | `DMSCResult<DMSCValidationResult>` |
| `validate_field(field, value, rules)` | 字段验证 | `field: &str`, `value: &Value`, `rules: &[DMSCValidationRule]` | `DMSCResult<DMSCValidationResult>` |
| `sanitize(data, sanitizers)` | 数据清理 | `data: Value`, `sanitizers: &[DMSCSanitizer]` | `DMSCResult<Value>` |
| `sanitize_field(field, value, sanitizers)` | 字段清理 | `field: &str`, `value: Value`, `sanitizers: &[DMSCSanitizer]` | `DMSCResult<Value>` |
| `validate_schema(data, schema)` | 模式验证 | `data: &Value`, `schema: &DMSCSchema` | `DMSCResult<DMSCValidationResult>` |
| `register_validator(name, validator)` | 注册验证器 | `name: &str`, `validator: impl DMSCValidator` | `DMSCResult<()>` |
| `register_sanitizer(name, sanitizer)` | 注册清理器 | `name: &str`, `sanitizer: impl DMSCSanitizer` | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;
use serde_json::json;

// 简单数据验证
let data = json!({
    "email": "john@example.com",
    "age": 25,
    "username": "john_doe"
});

let rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
    DMSCValidationRule::MinLength(5),
    DMSCValidationRule::MaxLength(100),
];

let result = ctx.validation().validate_field("email", &data["email"], &rules)?;
if result.is_valid {
    ctx.log().info("Email validation passed");
} else {
    for error in result.errors {
        ctx.log().error(format!("Validation error: {}", error.message));
    }
}

// 数据清理
let dirty_data = json!({
    "username": "  John Doe  ",
    "email": "JOHN@EXAMPLE.COM",
    "bio": "<script>alert('xss')</script>Hello World!"
});

let sanitizers = vec![
    DMSCSanitizer::Trim,
    DMSCSanitizer::ToLowercase,
    DMSCSanitizer::RemoveHtml,
];

let clean_data = ctx.validation().sanitize(dirty_data, &sanitizers)?;
ctx.log().info(format!("Cleaned data: {}", clean_data));
```

### DMSCValidationRule

验证规则枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Required` | 必填字段 |
| `Optional` | 可选字段 |
| `Email` | 邮箱格式 |
| `Url` | URL格式 |
| `Phone` | 电话号码 |
| `Numeric` | 数字格式 |
| `Alpha` | 字母格式 |
| `Alphanumeric` | 字母数字格式 |
| `MinLength(usize)` | 最小长度 |
| `MaxLength(usize)` | 最大长度 |
| `LengthRange(usize, usize)` | 长度范围 |
| `Min(i64)` | 最小值 |
| `Max(i64)` | 最大值 |
| `Range(i64, i64)` | 数值范围 |
| `Pattern(String)` | 正则表达式 |
| `In(Vec<String>)` | 枚举值 |
| `NotIn(Vec<String>)` | 排除值 |
| `Custom(String)` | 自定义验证 |

### DMSCValidationResult

验证结果结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `is_valid` | `bool` | 是否有效 |
| `errors` | `Vec<DMSCValidationError>` | 错误列表 |
| `warnings` | `Vec<DMSCValidationWarning>` | 警告列表 |
| `field_results` | `HashMap<String, DMSCValidationResult>` | 字段验证结果 |

<div align="center">

## 数据验证

</div>

### 字段验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 验证用户注册数据
let user_data = json!({
    "username": "john_doe",
    "email": "john@example.com",
    "password": "SecurePass123!",
    "age": 25,
    "phone": "+1-555-123-4567",
    "website": "https://john.example.com",
    "bio": "Software developer passionate about Rust"
});

// 用户名验证
let username_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Alphanumeric,
    DMSCValidationRule::MinLength(3),
    DMSCValidationRule::MaxLength(20),
    DMSCValidationRule::Pattern(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_string()),
];

let username_result = ctx.validation().validate_field("username", &user_data["username"], &username_rules)?;
if !username_result.is_valid {
    return Err(DMSCError::validation(format!("Username validation failed: {:?}", username_result.errors)));
}

// 邮箱验证
let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
    DMSCValidationRule::MaxLength(100),
];

let email_result = ctx.validation().validate_field("email", &user_data["email"], &email_rules)?;
if !email_result.is_valid {
    return Err(DMSCError::validation(format!("Email validation failed: {:?}", email_result.errors)));
}

// 密码验证
let password_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(8),
    DMSCValidationRule::MaxLength(128),
    DMSCValidationRule::Pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$".to_string()),
];

let password_result = ctx.validation().validate_field("password", &user_data["password"], &password_rules)?;
if !password_result.is_valid {
    return Err(DMSCError::validation(format!("Password validation failed: {:?}", password_result.errors)));
}

// 年龄验证
let age_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Numeric,
    DMSCValidationRule::Range(18, 120),
];

let age_result = ctx.validation().validate_field("age", &user_data["age"], &age_rules)?;
if !age_result.is_valid {
    return Err(DMSCError::validation(format!("Age validation failed: {:?}", age_result.errors)));
}

ctx.log().info("All field validations passed");
```

### 复杂数据验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 验证订单数据
let order_data = json!({
    "order_id": "ORD-2024-001234",
    "customer": {
        "name": "John Doe",
        "email": "john@example.com",
        "phone": "+1-555-123-4567",
        "address": {
            "street": "123 Main St",
            "city": "New York",
            "state": "NY",
            "zip": "10001",
            "country": "USA"
        }
    },
    "items": [
        {
            "product_id": "PROD-001",
            "name": "Rust Programming Book",
            "quantity": 2,
            "price": 49.99,
            "currency": "USD"
        },
        {
            "product_id": "PROD-002",
            "name": "Rust T-Shirt",
            "quantity": 1,
            "price": 29.99,
            "currency": "USD"
        }
    ],
    "total_amount": 129.97,
    "currency": "USD",
    "payment_method": "credit_card",
    "shipping_method": "express",
    "order_date": "2024-01-15T10:30:00Z"
});

// 定义验证模式
let order_schema = DMSCSchema {
    fields: vec![
        DMSCSchemaField {
            name: "order_id".to_string(),
            field_type: DMSCSchemaFieldType::String,
            rules: vec![
                DMSCValidationRule::Required,
                DMSCValidationRule::Pattern(r"^ORD-\d{4}-\d{6}$".to_string()),
            ],
            ..Default::default()
        },
        DMSCSchemaField {
            name: "customer".to_string(),
            field_type: DMSCSchemaFieldType::Object,
            nested_schema: Some(Box::new(DMSCSchema {
                fields: vec![
                    DMSCSchemaField {
                        name: "name".to_string(),
                        field_type: DMSCSchemaFieldType::String,
                        rules: vec![
                            DMSCValidationRule::Required,
                            DMSCValidationRule::MinLength(2),
                            DMSCValidationRule::MaxLength(100),
                        ],
                        ..Default::default()
                    },
                    DMSCSchemaField {
                        name: "email".to_string(),
                        field_type: DMSCSchemaFieldType::String,
                        rules: vec![
                            DMSCValidationRule::Required,
                            DMSCValidationRule::Email,
                        ],
                        ..Default::default()
                    },
                    DMSCSchemaField {
                        name: "phone".to_string(),
                        field_type: DMSCSchemaFieldType::String,
                        rules: vec![
                            DMSCValidationRule::Phone,
                        ],
                        ..Default::default()
                    },
                ],
                ..Default::default()
            })),
            ..Default::default()
        },
        DMSCSchemaField {
            name: "items".to_string(),
            field_type: DMSCSchemaFieldType::Array,
            rules: vec![
                DMSCValidationRule::Required,
                DMSCValidationRule::Min(1),
            ],
            array_item_schema: Some(Box::new(DMSCSchemaField {
                field_type: DMSCSchemaFieldType::Object,
                rules: vec![DMSCValidationRule::Required],
                ..Default::default()
            })),
            ..Default::default()
        },
        DMSCSchemaField {
            name: "total_amount".to_string(),
            field_type: DMSCSchemaFieldType::Number,
            rules: vec![
                DMSCValidationRule::Required,
                DMSCValidationRule::Min(0),
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
};

// 执行模式验证
let result = ctx.validation().validate_schema(&order_data, &order_schema)?;
if !result.is_valid {
    return Err(DMSCError::validation(format!("Order validation failed: {:?}", result.errors)));
}

ctx.log().info("Order data validation passed");
```

<div align="center">

## 数据清理

</div>

### 基本清理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 清理用户输入数据
let dirty_data = json!({
    "username": "  John Doe  ",
    "email": "JOHN@EXAMPLE.COM",
    "phone": "+1 (555) 123-4567",
    "bio": "<script>alert('xss')</script><p>Hello <b>World</b>!</p>",
    "website": "https://EXAMPLE.COM/path/",
    "tags": ["  rust  ", "  programming  ", "  web  "]
});

// 应用清理器
let sanitizers = vec![
    DMSCSanitizer::Trim,
    DMSCSanitizer::ToLowercase,
    DMSCSanitizer::RemoveHtml,
    DMSCSanitizer::NormalizeWhitespace,
];

let clean_data = ctx.validation().sanitize(dirty_data, &sanitizers)?;
ctx.log().info(format!("Cleaned data: {}", clean_data));

// 字段特定清理
let email_sanitizers = vec![
    DMSCSanitizer::Trim,
    DMSCSanitizer::ToLowercase,
];

let cleaned_email = ctx.validation().sanitize_field(
    "email",
    dirty_data["email"].clone(),
    &email_sanitizers
)?;
ctx.log().info(format!("Cleaned email: {}", cleaned_email));
```

### 高级清理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 清理HTML内容
let html_content = r#"
    <div class="content">
        <script>alert('XSS Attack!');</script>
        <p>Hello <strong>World</strong>!</p>
        <img src="javascript:alert('XSS')" alt="XSS">
        <a href="javascript:alert('XSS')">Click me</a>
        <iframe src="https://evil.com"></iframe>
        <form action="https://evil.com/submit">
            <input type="text" name="steal_data">
        </form>
    </div>
"#;

let safe_html = ctx.validation().sanitize_html(html_content, &DMSCHtmlSanitizerConfig {
    allowed_tags: vec!["p".to_string(), "strong".to_string(), "em".to_string(), "br".to_string()],
    allowed_attributes: HashMap::new(),
    allowed_protocols: vec!["http".to_string(), "https".to_string()],
    remove_script: true,
    remove_iframe: true,
    remove_form: true,
    remove_javascript: true,
})?;

ctx.log().info(format!("Sanitized HTML: {}", safe_html));

// 清理SQL输入
let user_input = "admin' OR '1'='1' --";
let safe_sql = ctx.validation().sanitize_sql(user_input);
ctx.log().info(format!("Sanitized SQL: {}", safe_sql));

// 清理文件路径
let file_path = "../../../etc/passwd";
let safe_path = ctx.validation().sanitize_path(file_path)?;
ctx.log().info(format!("Sanitized path: {}", safe_path));

// 清理URL
let dirty_url = "https://example.com/path/../../../../etc/passwd";
let safe_url = ctx.validation().sanitize_url(dirty_url)?;
ctx.log().info(format!("Sanitized URL: {}", safe_url));
```

<div align="center">

## 自定义验证器

</div>

### 创建自定义验证器

```rust
use dmsc::prelude::*;
use serde_json::Value;

// 创建用户名验证器
struct UsernameValidator {
    reserved_names: Vec<String>,
}

impl DMSCValidator for UsernameValidator {
    fn validate(&self, field_name: &str, value: &Value, _params: &[String]) -> DMSCResult<DMSCValidationResult> {
        if let Some(username) = value.as_str() {
            // 检查长度
            if username.len() < 3 {
                return Ok(DMSCValidationResult {
                    is_valid: false,
                    errors: vec![DMSCValidationError {
                        field: field_name.to_string(),
                        message: "Username must be at least 3 characters long".to_string(),
                        code: "USERNAME_TOO_SHORT".to_string(),
                    }],
                    warnings: vec![],
                    field_results: HashMap::new(),
                });
            }
            
            // 检查是否以字母开头
            if !username.chars().next().unwrap().is_alphabetic() {
                return Ok(DMSCValidationResult {
                    is_valid: false,
                    errors: vec![DMSCValidationError {
                        field: field_name.to_string(),
                        message: "Username must start with a letter".to_string(),
                        code: "USERNAME_INVALID_START".to_string(),
                    }],
                    warnings: vec![],
                    field_results: HashMap::new(),
                });
            }
            
            // 检查保留名称
            if self.reserved_names.contains(&username.to_lowercase()) {
                return Ok(DMSCValidationResult {
                    is_valid: false,
                    errors: vec![DMSCValidationError {
                        field: field_name.to_string(),
                        message: "Username is reserved".to_string(),
                        code: "USERNAME_RESERVED".to_string(),
                    }],
                    warnings: vec![],
                    field_results: HashMap::new(),
                });
            }
            
            // 检查是否包含禁止词汇
            let forbidden_words = vec!["admin", "root", "system", "moderator"];
            for word in &forbidden_words {
                if username.to_lowercase().contains(word) {
                    return Ok(DMSCValidationResult {
                        is_valid: false,
                        errors: vec![DMSCValidationError {
                            field: field_name.to_string(),
                            message: format!("Username cannot contain '{}'", word),
                            code: "USERNAME_FORBIDDEN_WORD".to_string(),
                        }],
                        warnings: vec![],
                        field_results: HashMap::new(),
                    });
                }
            }
            
            Ok(DMSCValidationResult {
                is_valid: true,
                errors: vec![],
                warnings: vec![],
                field_results: HashMap::new(),
            })
        } else {
            Ok(DMSCValidationResult {
                is_valid: false,
                errors: vec![DMSCValidationError {
                    field: field_name.to_string(),
                    message: "Username must be a string".to_string(),
                    code: "USERNAME_INVALID_TYPE".to_string(),
                }],
                warnings: vec![],
                field_results: HashMap::new(),
            })
        }
    }
    
    fn name(&self) -> &str {
        "username_validator"
    }
}

// 创建强密码验证器
struct StrongPasswordValidator {
    min_strength: f64,
}

impl DMSCValidator for StrongPasswordValidator {
    fn validate(&self, field_name: &str, value: &Value, _params: &[String]) -> DMSCResult<DMSCValidationResult> {
        if let Some(password) = value.as_str() {
            let strength = calculate_password_strength(password);
            
            if strength < self.min_strength {
                return Ok(DMSCValidationResult {
                    is_valid: false,
                    errors: vec![DMSCValidationError {
                        field: field_name.to_string(),
                        message: format!("Password strength {:.1} is too low. Minimum required: {:.1}", strength, self.min_strength),
                        code: "PASSWORD_TOO_WEAK".to_string(),
                    }],
                    warnings: vec![],
                    field_results: HashMap::new(),
                });
            }
            
            Ok(DMSCValidationResult {
                is_valid: true,
                errors: vec![],
                warnings: vec![DMSCValidationWarning {
                    field: field_name.to_string(),
                    message: format!("Password strength: {:.1}/10.0", strength),
                    code: "PASSWORD_STRENGTH".to_string(),
                }],
                field_results: HashMap::new(),
            })
        } else {
            Ok(DMSCValidationResult {
                is_valid: false,
                errors: vec![DMSCValidationError {
                    field: field_name.to_string(),
                    message: "Password must be a string".to_string(),
                    code: "PASSWORD_INVALID_TYPE".to_string(),
                }],
                warnings: vec![],
                field_results: HashMap::new(),
            })
        }
    }
    
    fn name(&self) -> &str {
        "strong_password_validator"
    }
}

fn calculate_password_strength(password: &str) -> f64 {
    let mut strength = 0.0;
    
    // 长度评分
    strength += (password.len() as f64 * 0.3).min(3.0);
    
    // 包含小写字母
    if password.chars().any(|c| c.is_lowercase()) {
        strength += 1.0;
    }
    
    // 包含大写字母
    if password.chars().any(|c| c.is_uppercase()) {
        strength += 1.0;
    }
    
    // 包含数字
    if password.chars().any(|c| c.is_numeric()) {
        strength += 1.0;
    }
    
    // 包含特殊字符
    if password.chars().any(|c| !c.is_alphanumeric()) {
        strength += 1.0;
    }
    
    // 多样性评分
    let unique_chars: std::collections::HashSet<_> = password.chars().collect();
    strength += (unique_chars.len() as f64 * 0.1).min(2.0);
    
    strength.min(10.0)
}

// 注册自定义验证器
ctx.validation().register_validator("username", UsernameValidator {
    reserved_names: vec![
        "admin".to_string(),
        "root".to_string(),
        "system".to_string(),
        "moderator".to_string(),
        "guest".to_string(),
    ],
})?;

ctx.validation().register_validator("strong_password", StrongPasswordValidator {
    min_strength: 7.0,
})?;

// 使用自定义验证器
let username_rules = vec![
    DMSCValidationRule::Custom("username".to_string()),
];

let password_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(8),
    DMSCValidationRule::Custom("strong_password".to_string()),
];

let username_result = ctx.validation().validate_field("username", &json!("admin_user"), &username_rules)?;
let password_result = ctx.validation().validate_field("password", &json!("weak"), &password_rules)?;
```

<div align="center">

## 条件验证

</div>

### 条件规则

```rust
use dmsc::prelude::*;
use serde_json::json;

// 条件验证：如果用户选择公司注册，则需要公司信息
let registration_data = json!({
    "account_type": "company",
    "personal_info": {
        "name": "John Doe",
        "email": "john@example.com"
    },
    "company_info": {
        "company_name": "Tech Corp",
        "tax_id": "12-3456789",
        "business_license": "BL123456"
    }
});

// 定义条件验证规则
let conditional_rules = vec![
    DMSCConditionalValidationRule {
        field: "company_info".to_string(),
        condition: DMSCValidationCondition::Equals("account_type", "company"),
        rules: vec![
            DMSCValidationRule::Required,
        ],
        nested_rules: Some(vec![
            DMSCSchemaField {
                name: "company_name".to_string(),
                field_type: DMSCSchemaFieldType::String,
                rules: vec![
                    DMSCValidationRule::Required,
                    DMSCValidationRule::MinLength(2),
                    DMSCValidationRule::MaxLength(100),
                ],
                ..Default::default()
            },
            DMSCSchemaField {
                name: "tax_id".to_string(),
                field_type: DMSCSchemaFieldType::String,
                rules: vec![
                    DMSCValidationRule::Required,
                    DMSCValidationRule::Pattern(r"^\d{2}-\d{7}$".to_string()),
                ],
                ..Default::default()
            },
        ]),
        ..Default::default()
    },
];

// 执行条件验证
let result = ctx.validation().validate_conditional(&registration_data, &conditional_rules)?;
if !result.is_valid {
    return Err(DMSCError::validation(format!("Conditional validation failed: {:?}", result.errors)));
}

// 动态验证规则
let dynamic_rules = vec![
    DMSCDynamicValidationRule {
        field: "shipping_address".to_string(),
        condition_field: "requires_shipping".to_string(),
        condition_value: json!(true),
        rules: vec![
            DMSCValidationRule::Required,
        ],
        ..Default::default()
    },
];

let order_data = json!({
    "requires_shipping": true,
    "shipping_address": {
        "street": "123 Main St",
        "city": "New York",
        "state": "NY",
        "zip": "10001"
    }
});

let dynamic_result = ctx.validation().validate_dynamic(&order_data, &dynamic_rules)?;
ctx.log().info(format!("Dynamic validation result: {:?}", dynamic_result));
```

<div align="center">

## 异步验证

</div>

### 外部服务验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建异步邮箱验证器
struct AsyncEmailValidator {
    api_endpoint: String,
    api_key: String,
}

impl AsyncEmailValidator {
    async fn validate_email(&self, email: &str) -> DMSCResult<bool> {
        // 调用外部邮箱验证服务
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/validate", self.api_endpoint))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .query(&[("email", email)])
            .send()
            .await?;
        
        if response.status().is_success() {
            let validation_result: serde_json::Value = response.json().await?;
            Ok(validation_result["valid"].as_bool().unwrap_or(false))
        } else {
            Ok(false) // 如果服务不可用，假设邮箱有效
        }
    }
}

// 使用异步验证
let email_validator = AsyncEmailValidator {
    api_endpoint: "https://api.emailvalidator.com".to_string(),
    api_key: "your_api_key".to_string(),
};

let email = "john@example.com";
let is_valid = email_validator.validate_email(email).await?;

if !is_valid {
    return Err(DMSCError::validation(format!("Email validation failed for: {}", email)));
}

ctx.log().info(format!("Email validation passed for: {}", email));
```

<div align="center">

## 验证配置

</div>

### DMSCValidationConfig

验证配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `strict_mode` | `bool` | 严格模式 | `false` |
| `stop_on_first_error` | `bool` | 第一个错误时停止 | `false` |
| `include_warnings` | `bool` | 包含警告 | `true` |
| `max_errors_per_field` | `usize` | 每字段最大错误数 | `5` |
| `max_nested_depth` | `usize` | 最大嵌套深度 | `10` |
| `async_timeout` | `Duration` | 异步验证超时 | `30s` |
| `cache_results` | `bool` | 缓存验证结果 | `true` |
| `cache_ttl` | `Duration` | 缓存过期时间 | `5m` |

#### 配置示例

```rust
use dmsc::prelude::*;

let validation_config = DMSCValidationConfig {
    strict_mode: true,
    stop_on_first_error: false,
    include_warnings: true,
    max_errors_per_field: 3,
    max_nested_depth: 5,
    async_timeout: Duration::from_secs(30),
    cache_results: true,
    cache_ttl: Duration::from_minutes(5),
};

ctx.validation().set_config(validation_config)?;
```

<div align="center">

## 错误处理

</div>

### 验证错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `VALIDATION_FAILED` | 验证失败 |
| `VALIDATION_REQUIRED` | 必填字段缺失 |
| `VALIDATION_FORMAT` | 格式错误 |
| `VALIDATION_LENGTH` | 长度错误 |
| `VALIDATION_RANGE` | 范围错误 |
| `VALIDATION_PATTERN` | 模式不匹配 |
| `VALIDATION_TYPE` | 类型错误 |
| `VALIDATION_CUSTOM` | 自定义验证失败 |

### 错误处理示例

```rust
use dmsc::prelude::*;
use serde_json::json;

match ctx.validation().validate(&user_data, &validation_rules) {
    Ok(result) => {
        if result.is_valid {
            ctx.log().info("Validation passed");
            // 继续处理数据
        } else {
            ctx.log().warn(format!("Validation failed with {} errors", result.errors.len()));
            
            // 处理验证错误
            for error in &result.errors {
                ctx.log().error(format!("Field '{}': {}", error.field, error.message));
            }
            
            // 返回用户友好的错误信息
            let user_errors: Vec<String> = result.errors.iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();
            
            return Err(DMSCError::validation(format!("Validation errors: {}", user_errors.join(", "))));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Validation error: {}", e));
        return Err(e);
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **早期验证**: 在数据进入系统前进行验证
2. **分层验证**: 在客户端、API层和业务层都进行验证
3. **验证与清理结合**: 先清理数据，再进行验证
4. **自定义验证器**: 为业务特定需求创建自定义验证器
5. **异步验证**: 对外部服务验证使用异步方式
6. **验证缓存**: 缓存验证结果以提高性能
7. **详细错误信息**: 提供清晰、用户友好的错误信息
8. **验证配置**: 根据环境调整验证严格程度
9. **测试验证规则**: 为验证规则编写测试用例
10. **文档化验证规则**: 记录所有验证规则和自定义验证器

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
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持