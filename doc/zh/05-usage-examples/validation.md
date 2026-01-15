# 验证使用示例

validation模块提供数据验证、数据清理、自定义验证器、条件验证、异步验证和验证配置功能的使用示例。

## 基本数据验证

### 简单字段验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 初始化验证管理器
let validation_config = DMSCValidationConfig {
    strict_mode: true,
    stop_on_first_error: false,
    enable_type_coercion: true,
    custom_messages: {
        let mut messages = std::collections::HashMap::new();
        messages.insert("required".to_string(), "This field is required".to_string());
        messages.insert("email".to_string(), "Please enter a valid email address".to_string());
        messages.insert("min_length".to_string(), "This field must be at least {min} characters".to_string());
        messages
    },
    locale: "en".to_string(),
    timezone: "UTC".to_string(),
};

ctx.validation().init_validation(validation_config).await?;

// 用户注册数据验证
let user_data = json!({
    "username": "john_doe",
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!",
    "age": 25,
    "phone": "+1234567890",
    "website": "https://johndoe.com",
    "birth_date": "1998-01-15",
    "agree_to_terms": true,
});

// 定义验证规则
let validation_rules = vec![
    DMSCValidationRule {
        field: "username".to_string(),
        rule_type: DMSCValidationType::Pattern("^[a-zA-Z0-9_]{3,20}$".to_string()),
        required: true,
        message: "Username must be 3-20 characters, alphanumeric and underscore only".to_string(),
    },
    DMSCValidationRule {
        field: "email".to_string(),
        rule_type: DMSCValidationType::Email,
        required: true,
        message: "Valid email address is required".to_string(),
    },
    DMSCValidationRule {
        field: "password".to_string(),
        rule_type: DMSCValidationType::Custom(Box::new(|value| {
            let password = value.as_str().unwrap_or_default();
            
            if password.len() < 8 {
                return Err("Password must be at least 8 characters long".to_string());
            }
            
            if !password.chars().any(|c| c.is_uppercase()) {
                return Err("Password must contain at least one uppercase letter".to_string());
            }
            
            if !password.chars().any(|c| c.is_lowercase()) {
                return Err("Password must contain at least one lowercase letter".to_string());
            }
            
            if !password.chars().any(|c| c.is_numeric()) {
                return Err("Password must contain at least one number".to_string());
            }
            
            if !password.chars().any(|c| "!@#$%^&*".contains(c)) {
                return Err("Password must contain at least one special character".to_string());
            }
            
            Ok(())
        })),
        required: true,
        message: "Password does not meet security requirements".to_string(),
    },
    DMSCValidationRule {
        field: "age".to_string(),
        rule_type: DMSCValidationType::Range(18, 100),
        required: true,
        message: "Age must be between 18 and 100".to_string(),
    },
    DMSCValidationRule {
        field: "phone".to_string(),
        rule_type: DMSCValidationType::Pattern(r"^\+?[1-9]\d{1,14}$".to_string()),
        required: false,
        message: "Valid phone number is required".to_string(),
    },
    DMSCValidationRule {
        field: "website".to_string(),
        rule_type: DMSCValidationType::Url,
        required: false,
        message: "Valid URL is required".to_string(),
    },
    DMSCValidationRule {
        field: "birth_date".to_string(),
        rule_type: DMSCValidationType::Date,
        required: true,
        message: "Valid birth date is required".to_string(),
    },
    DMSCValidationRule {
        field: "agree_to_terms".to_string(),
        rule_type: DMSCValidationType::Boolean,
        required: true,
        message: "You must agree to the terms and conditions".to_string(),
    },
];

// 执行验证
match ctx.validation().validate_data(&user_data, &validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("User data validation successful");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
        
        // 继续处理用户注册
        // ...
    }
    Err(validation_errors) => {
        ctx.log().error("User data validation failed");
        for error in &validation_errors {
            ctx.log().error(format!("Validation error: {}", error));
        }
        
        // 返回验证错误给前端
        return Err(DMSCError::validation(format!("Validation failed: {:?}", validation_errors)));
    }
}
```

### 复杂数据验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 产品数据验证
let product_data = json!({
    "name": "Wireless Headphones",
    "description": "High-quality wireless headphones with noise cancellation",
    "price": 199.99,
    "currency": "USD",
    "sku": "WH-2024-001",
    "categories": ["Electronics", "Audio", "Headphones"],
    "inventory": {
        "quantity": 150,
        "warehouse": "US-WH-01",
        "reserved": 25
    },
    "dimensions": {
        "length": 7.5,
        "width": 6.2,
        "height": 3.1,
        "weight": 0.85,
        "unit": "inches"
    },
    "shipping": {
        "free_shipping": true,
        "shipping_weight": 1.2,
        "shipping_class": "standard",
        "restrictions": ["US", "CA", "EU"]
    },
    "warranty": {
        "duration": 24,
        "type": "manufacturer",
        "coverage": "worldwide"
    }
});

// 复杂验证规则
let complex_rules = vec![
    DMSCValidationRule {
        field: "name".to_string(),
        rule_type: DMSCValidationType::Length(5, 100),
        required: true,
        message: "Product name must be 5-100 characters".to_string(),
    },
    DMSCValidationRule {
        field: "description".to_string(),
        rule_type: DMSCValidationType::Length(20, 2000),
        required: true,
        message: "Description must be 20-2000 characters".to_string(),
    },
    DMSCValidationRule {
        field: "price".to_string(),
        rule_type: DMSCValidationType::Range(0.01, 10000.0),
        required: true,
        message: "Price must be between $0.01 and $10,000".to_string(),
    },
    DMSCValidationRule {
        field: "currency".to_string(),
        rule_type: DMSCValidationType::In(vec!["USD".to_string(), "EUR".to_string(), "GBP".to_string(), "JPY".to_string()]),
        required: true,
        message: "Currency must be USD, EUR, GBP, or JPY".to_string(),
    },
    DMSCValidationRule {
        field: "sku".to_string(),
        rule_type: DMSCValidationType::Pattern(r"^[A-Z]{2}-\d{4}-\d{3}$".to_string()),
        required: true,
        message: "SKU must follow format: XX-YYYY-ZZZ".to_string(),
    },
    DMSCValidationRule {
        field: "categories".to_string(),
        rule_type: DMSCValidationType::ArrayLength(1, 5),
        required: true,
        message: "Product must have 1-5 categories".to_string(),
    },
    DMSCValidationRule {
        field: "inventory.quantity".to_string(),
        rule_type: DMSCValidationType::Range(0, 10000),
        required: true,
        message: "Inventory quantity must be 0-10,000".to_string(),
    },
    DMSCValidationRule {
        field: "dimensions.weight".to_string(),
        rule_type: DMSCValidationType::Range(0.01, 100.0),
        required: true,
        message: "Weight must be 0.01-100.0".to_string(),
    },
    DMSCValidationRule {
        field: "shipping.shipping_class".to_string(),
        rule_type: DMSCValidationType::In(vec!["standard".to_string(), "express".to_string(), "overnight".to_string()]),
        required: true,
        message: "Shipping class must be standard, express, or overnight".to_string(),
    },
];

// 执行复杂验证
match ctx.validation().validate_complex_data(&product_data, &complex_rules).await {
    Ok(validated_product) => {
        ctx.log().info("Product data validation successful");
        
        // 进一步处理产品数据
        // ...
    }
    Err(errors) => {
        ctx.log().error("Product validation failed");
        // 处理验证错误
        return Err(DMSCError::validation(format!("Product validation failed: {:?}", errors)));
    }
}
```

## 数据清理

### 基本数据清理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 用户输入数据清理
let dirty_data = json!({
    "username": "  john_doe  ",
    "email": "JOHN.DOE@EXAMPLE.COM",
    "phone": "+1 (234) 567-8900",
    "website": "https://johndoe.com/",
    "bio": "<script>alert('XSS')</script>Hello, I'm John! <b>Welcome</b> to my profile.",
    "tags": ["  developer  ", "  designer  ", "  photographer  "],
    "skills": ["  Rust  ", "  JavaScript  ", "  Python  "],
});

// 定义清理规则
let sanitization_rules = vec![
    DMSCSanitizationRule {
        field: "username".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::ToLowercase,
            DMSCSanitizationOperation::RemoveSpecialCharsExcept(vec!['_', '-']),
        ],
    },
    DMSCSanitizationRule {
        field: "email".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::ToLowercase,
        ],
    },
    DMSCSanitizationRule {
        field: "phone".to_string(),
        operations: vec![
            DMSCSanitizationOperation::RemoveSpecialCharsExcept(vec!['+', '-']),
            DMSCSanitizationOperation::NormalizePhoneNumber,
        ],
    },
    DMSCSanitizationRule {
        field: "website".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::NormalizeUrl,
        ],
    },
    DMSCSanitizationRule {
        field: "bio".to_string(),
        operations: vec![
            DMSCSanitizationOperation::StripHtmlTags,
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::NormalizeWhitespace,
        ],
    },
    DMSCSanitizationRule {
        field: "tags".to_string(),
        operations: vec![
            DMSCSanitizationOperation::TrimEach,
            DMSCSanitizationOperation::ToLowercaseEach,
            DMSCSanitizationOperation::RemoveDuplicates,
        ],
    },
    DMSCSanitizationRule {
        field: "skills".to_string(),
        operations: vec![
            DMSCSanitizationOperation::TrimEach,
            DMSCSanitizationOperation::NormalizeWhitespace,
        ],
    },
];

// 执行数据清理
let sanitized_data = ctx.validation()
    .sanitize_data(&dirty_data, &sanitization_rules)
    .await?;

ctx.log().info(format!("Original data: {:?}", dirty_data));
ctx.log().info(format!("Sanitized data: {:?}", sanitized_data));

// 验证清理后的数据
let is_valid = ctx.validation()
    .validate_sanitized_data(&sanitized_data, &validation_rules)
    .await?;

ctx.log().info(format!("Sanitized data is valid: {}", is_valid));
```

### 高级数据清理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 富文本内容清理
let rich_content = json!({
    "title": "  Product <b>Review</b>  ",
    "content": r#"
        <script>alert('XSS')</script>
        <p>This is a <strong>great</strong> product!</p>
        <img src="javascript:alert('XSS')" onerror="alert('XSS')">
        <a href="javascript:alert('XSS')">Click here</a>
        <div onclick="maliciousFunction()">Bad div</div>
        <iframe src="javascript:alert('XSS')"></iframe>
        <p>Check out <a href="https://example.com" target="_blank">this link</a></p>
    "#,
    "tags": ["<script>", "electronics", "review", "<b>test</b>"],
    "metadata": {
        "author": "  john_doe  ",
        "source": "https://trusted-site.com",
        "rating": 5,
        "verified": true
    }
});

// 高级清理规则
let advanced_rules = vec![
    DMSCSanitizationRule {
        field: "title".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::StripDangerousHtmlTags,
            DMSCSanitizationOperation::NormalizeWhitespace,
        ],
    },
    DMSCSanitizationRule {
        field: "content".to_string(),
        operations: vec![
            DMSCSanitizationOperation::StripScriptTags,
            DMSCSanitizationOperation::StripEventHandlers,
            DMSCSanitizationOperation::StripDangerousAttributes,
            DMSCSanitizationOperation::NormalizeUrls,
            DMSCSanitizationOperation::AddNoFollowToExternalLinks,
            DMSCSanitizationOperation::NormalizeWhitespace,
        ],
    },
    DMSCSanitizationRule {
        field: "tags".to_string(),
        operations: vec![
            DMSCSanitizationOperation::StripHtmlTagsEach,
            DMSCSanitizationOperation::TrimEach,
            DMSCSanitizationOperation::ToLowercaseEach,
            DMSCSanitizationOperation::RemoveDuplicates,
            DMSCSanitizationOperation::LimitLengthEach(50),
        ],
    },
    DMSCSanitizationRule {
        field: "metadata.author".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::ToLowercase,
            DMSCSanitizationOperation::AlphanumericAndSpacesOnly,
        ],
    },
    DMSCSanitizationRule {
        field: "metadata.source".to_string(),
        operations: vec![
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::NormalizeUrl,
            DMSCSanitizationOperation::ValidateUrl,
        ],
    },
];

// 执行高级清理
let sanitized_content = ctx.validation()
    .sanitize_data_advanced(&rich_content, &advanced_rules)
    .await?;

ctx.log().info(format!("Sanitized rich content: {:?}", sanitized_content));

// SQL注入防护清理
let sql_input = json!({
    "search_query": "electronics'; DROP TABLE products; --",
    "category": "computers' OR '1'='1",
    "price_range": "0 AND 1=1",
    "sort_by": "name; DELETE FROM users",
    "filter": json!({
        "brand": "Samsung'; TRUNCATE TABLE orders; --",
        "min_price": "100",
        "max_price": "500"
    })
});

let sql_sanitization_rules = vec![
    DMSCSanitizationRule {
        field: "search_query".to_string(),
        operations: vec![
            DMSCSanitizationOperation::EscapeSql,
            DMSCSanitizationOperation::RemoveSqlKeywords,
            DMSCSanitizationOperation::Trim,
            DMSCSanitizationOperation::LimitLength(100),
        ],
    },
    DMSCSanitizationRule {
        field: "category".to_string(),
        operations: vec![
            DMSCSanitizationOperation::EscapeSql,
            DMSCSanitizationOperation::AlphanumericAndSpacesOnly,
            DMSCSanitizationOperation::Trim,
        ],
    },
    DMSCSanitizationRule {
        field: "price_range".to_string(),
        operations: vec![
            DMSCSanitizationOperation::EscapeSql,
            DMSCSanitizationOperation::NumericCharactersOnly,
        ],
    },
    DMSCSanitizationRule {
        field: "sort_by".to_string(),
        operations: vec![
            DMSCSanitizationOperation::EscapeSql,
            DMSCSanitizationOperation::RemoveSqlKeywords,
            DMSCSanitizationOperation::AlphanumericAndUnderscoreOnly,
        ],
    },
];

let sanitized_sql_input = ctx.validation()
    .sanitize_data(&sql_input, &sql_sanitization_rules)
    .await?;

ctx.log().info(format!("SQL input sanitized: {:?}", sanitized_sql_input));
```

## 自定义验证器

### 用户名验证器

```rust
use dmsc::prelude::*;
use std::collections::HashSet;

// 创建自定义用户名验证器
struct UsernameValidator {
    reserved_usernames: HashSet<String>,
    min_length: usize,
    max_length: usize,
}

impl UsernameValidator {
    fn new() -> Self {
        let mut reserved = HashSet::new();
        reserved.insert("admin".to_string());
        reserved.insert("administrator".to_string());
        reserved.insert("root".to_string());
        reserved.insert("user".to_string());
        reserved.insert("test".to_string());
        reserved.insert("guest".to_string());
        
        Self {
            reserved_usernames: reserved,
            min_length: 3,
            max_length: 20,
        }
    }
}

impl DMSCValidator for UsernameValidator {
    fn validate(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        let username = value.as_str().ok_or("Username must be a string")?;
        
        // 检查长度
        if username.len() < self.min_length {
            return Err(format!("Username must be at least {} characters long", self.min_length));
        }
        
        if username.len() > self.max_length {
            return Err(format!("Username must be no more than {} characters long", self.max_length));
        }
        
        // 检查字符集
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Username can only contain letters, numbers, underscore, and hyphen".to_string());
        }
        
        // 检查是否以数字开头
        if username.chars().next().unwrap_or_default().is_numeric() {
            return Err("Username cannot start with a number".to_string());
        }
        
        // 检查保留词
        let lower_username = username.to_lowercase();
        if self.reserved_usernames.contains(&lower_username) {
            return Err("This username is reserved and cannot be used".to_string());
        }
        
        // 检查连续字符
        if self.has_consecutive_chars(username, 3) {
            return Err("Username cannot have 3 or more consecutive identical characters".to_string());
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "username_validator"
    }
}

impl UsernameValidator {
    fn has_consecutive_chars(&self, s: &str, max_consecutive: usize) -> bool {
        let mut consecutive_count = 1;
        let chars: Vec<char> = s.chars().collect();
        
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                consecutive_count += 1;
                if consecutive_count >= max_consecutive {
                    return true;
                }
            } else {
                consecutive_count = 1;
            }
        }
        
        false
    }
}

// 注册自定义验证器
let username_validator = UsernameValidator::new();
ctx.validation().register_custom_validator(Box::new(username_validator)).await?;

// 使用自定义验证器
let user_data = json!({
    "username": "john_doe_123",
    "email": "john@example.com",
});

let username_rule = DMSCValidationRule {
    field: "username".to_string(),
    rule_type: DMSCValidationType::CustomValidator("username_validator".to_string()),
    required: true,
    message: "Username validation failed".to_string(),
};

match ctx.validation().validate_data(&user_data, &[username_rule]).await {
    Ok(_) => {
        ctx.log().info("Username validation successful");
    }
    Err(errors) => {
        ctx.log().error(format!("Username validation failed: {:?}", errors));
    }
}
```

### 强密码验证器

```rust
use dmsc::prelude::*;
use std::collections::HashSet;

// 创建强密码验证器
struct StrongPasswordValidator {
    common_passwords: HashSet<String>,
    min_length: usize,
    require_uppercase: bool,
    require_lowercase: bool,
    require_numbers: bool,
    require_special_chars: bool,
    max_consecutive_chars: usize,
}

impl StrongPasswordValidator {
    fn new() -> Self {
        let mut common_passwords = HashSet::new();
        common_passwords.insert("password".to_string());
        common_passwords.insert("123456".to_string());
        common_passwords.insert("12345678".to_string());
        common_passwords.insert("qwerty".to_string());
        common_passwords.insert("abc123".to_string());
        common_passwords.insert("password123".to_string());
        common_passwords.insert("admin".to_string());
        common_passwords.insert("letmein".to_string());
        
        Self {
            common_passwords,
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            max_consecutive_chars: 3,
        }
    }
    
    fn calculate_entropy(&self, password: &str) -> f64 {
        let charset_size = self.get_charset_size(password);
        (password.len() as f64) * (charset_size as f64).log2()
    }
    
    fn get_charset_size(&self, password: &str) -> usize {
        let mut charset_size = 0;
        
        if password.chars().any(|c| c.is_lowercase()) {
            charset_size += 26;
        }
        if password.chars().any(|c| c.is_uppercase()) {
            charset_size += 26;
        }
        if password.chars().any(|c| c.is_numeric()) {
            charset_size += 10;
        }
        if password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            charset_size += 32;
        }
        
        charset_size
    }
}

impl DMSCValidator for StrongPasswordValidator {
    fn validate(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        let password = value.as_str().ok_or("Password must be a string")?;
        
        // 检查长度
        if password.len() < self.min_length {
            return Err(format!("Password must be at least {} characters long", self.min_length));
        }
        
        // 检查是否包含常见密码
        let lower_password = password.to_lowercase();
        if self.common_passwords.contains(&lower_password) {
            return Err("Password is too common, please choose a more unique password".to_string());
        }
        
        // 检查字符类型
        if self.require_uppercase && !password.chars().any(|c| c.is_uppercase()) {
            return Err("Password must contain at least one uppercase letter".to_string());
        }
        
        if self.require_lowercase && !password.chars().any(|c| c.is_lowercase()) {
            return Err("Password must contain at least one lowercase letter".to_string());
        }
        
        if self.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err("Password must contain at least one number".to_string());
        }
        
        if self.require_special_chars && !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            return Err("Password must contain at least one special character".to_string());
        }
        
        // 检查连续字符
        if self.has_consecutive_chars(password, self.max_consecutive_chars) {
            return Err(format!("Password cannot have {} or more consecutive identical characters", self.max_consecutive_chars));
        }
        
        // 检查熵值
        let entropy = self.calculate_entropy(password);
        if entropy < 50.0 {
            return Err(format!("Password is too predictable (entropy: {:.1} bits, minimum: 50 bits)", entropy));
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "strong_password_validator"
    }
}

impl StrongPasswordValidator {
    fn has_consecutive_chars(&self, s: &str, max_consecutive: usize) -> bool {
        let mut consecutive_count = 1;
        let chars: Vec<char> = s.chars().collect();
        
        for i in 1..chars.len() {
            if chars[i] == chars[i - 1] {
                consecutive_count += 1;
                if consecutive_count >= max_consecutive {
                    return true;
                }
            } else {
                consecutive_count = 1;
            }
        }
        
        false
    }
}

// 注册强密码验证器
let password_validator = StrongPasswordValidator::new();
ctx.validation().register_custom_validator(Box::new(password_validator)).await?;

// 使用强密码验证器
let password_data = json!({
    "password": "MyS3cur3P@ssw0rd!2024",
});

let password_rule = DMSCValidationRule {
    field: "password".to_string(),
    rule_type: DMSCValidationType::CustomValidator("strong_password_validator".to_string()),
    required: true,
    message: "Password is not strong enough".to_string(),
};

match ctx.validation().validate_data(&password_data, &[password_rule]).await {
    Ok(_) => {
        ctx.log().info("Password validation successful");
    }
    Err(errors) => {
        ctx.log().error(format!("Password validation failed: {:?}", errors));
    }
}
```

## 条件验证

### 动态验证规则

```rust
use dmsc::prelude::*;
use serde_json::json;

// 订单验证 - 根据订单类型应用不同规则
let order_data = json!({
    "order_type": "international", // 或 "domestic"
    "customer_id": "cust_123",
    "items": [
        {
            "product_id": "prod_456",
            "quantity": 2,
            "price": 99.99
        }
    ],
    "shipping_address": {
        "country": "Canada",
        "postal_code": "M5V 3A8",
        "province": "Ontario"
    },
    "billing_address": {
        "country": "Canada",
        "postal_code": "M5V 3A8"
    },
    "payment_method": "credit_card",
    "currency": "CAD",
    "total_amount": 199.98
});

// 根据订单类型获取验证规则
fn get_validation_rules(order_type: &str) -> Vec<DMSCValidationRule> {
    let mut rules = vec![
        DMSCValidationRule {
            field: "customer_id".to_string(),
            rule_type: DMSCValidationType::Pattern(r"^cust_\d+$".to_string()),
            required: true,
            message: "Invalid customer ID format".to_string(),
        },
        DMSCValidationRule {
            field: "items".to_string(),
            rule_type: DMSCValidationType::ArrayLength(1, 10),
            required: true,
            message: "Order must contain 1-10 items".to_string(),
        },
        DMSCValidationRule {
            field: "total_amount".to_string(),
            rule_type: DMSCValidationType::Range(0.01, 10000.0),
            required: true,
            message: "Total amount must be between $0.01 and $10,000".to_string(),
        },
    ];
    
    match order_type {
        "international" => {
            rules.push(DMSCValidationRule {
                field: "shipping_address.country".to_string(),
                rule_type: DMSCValidationType::NotIn(vec!["US".to_string(), "USA".to_string(), "United States".to_string()]),
                required: true,
                message: "International orders must ship outside the US".to_string(),
            });
            
            rules.push(DMSCValidationRule {
                field: "total_amount".to_string(),
                rule_type: DMSCValidationType::Range(50.0, 5000.0),
                required: true,
                message: "International orders must be between $50 and $5,000".to_string(),
            });
        }
        "domestic" => {
            rules.push(DMSCValidationRule {
                field: "shipping_address.country".to_string(),
                rule_type: DMSCValidationType::In(vec!["US".to_string(), "USA".to_string(), "United States".to_string()]),
                required: true,
                message: "Domestic orders must ship within the US".to_string(),
            });
            
            rules.push(DMSCValidationRule {
                field: "total_amount".to_string(),
                rule_type: DMSCValidationType::Range(10.0, 10000.0),
                required: true,
                message: "Domestic orders must be between $10 and $10,000".to_string(),
            });
        }
        _ => {}
    }
    
    rules
}

// 获取条件验证规则
let order_type = order_data["order_type"].as_str().unwrap_or("");
let conditional_rules = get_validation_rules(order_type);

// 执行条件验证
match ctx.validation().validate_data(&order_data, &conditional_rules).await {
    Ok(validated_order) => {
        ctx.log().info("Order validation successful");
        ctx.log().info(format!("Validated order data: {:?}", validated_order));
    }
    Err(validation_errors) => {
        ctx.log().error("Order validation failed");
        for error in &validation_errors {
            ctx.log().error(format!("Validation error: {}", error));
        }
    }
}
```

### 依赖字段验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 表单验证 - 字段间依赖关系
let form_data = json!({
    "contact_method": "email", // 或 "phone", "mail"
    "email": "john@example.com",
    "phone": "+1234567890",
    "address": {
        "street": "123 Main St",
        "city": "New York",
        "state": "NY",
        "zip_code": "10001"
    },
    "preferred_contact_time": "morning", // 或 "afternoon", "evening"
    "contact_restriction": "weekdays_only", // 或 "anytime", "weekends_only"
});

// 依赖字段验证器
struct ContactMethodValidator;

impl DMSCValidator for ContactMethodValidator {
    fn validate(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        let contact_method = value["contact_method"].as_str().unwrap_or("");
        
        match contact_method {
            "email" => {
                if value.get("email").is_none() || value["email"].as_str().unwrap_or_default().is_empty() {
                    return Err("Email is required when contact method is email".to_string());
                }
                
                // 验证邮箱格式
                let email = value["email"].as_str().unwrap();
                if !email.contains('@') || !email.contains('.') {
                    return Err("Invalid email format".to_string());
                }
            }
            "phone" => {
                if value.get("phone").is_none() || value["phone"].as_str().unwrap_or_default().is_empty() {
                    return Err("Phone number is required when contact method is phone".to_string());
                }
                
                // 验证电话号码格式
                let phone = value["phone"].as_str().unwrap();
                let phone_regex = regex::Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
                if !phone_regex.is_match(phone) {
                    return Err("Invalid phone number format".to_string());
                }
            }
            "mail" => {
                if value.get("address").is_none() {
                    return Err("Address is required when contact method is mail".to_string());
                }
                
                // 验证地址字段
                let address = &value["address"];
                let required_fields = ["street", "city", "state", "zip_code"];
                for field in &required_fields {
                    if address.get(field).is_none() || address[field].as_str().unwrap_or_default().is_empty() {
                        return Err(format!("{} is required in address", field));
                    }
                }
            }
            _ => {
                return Err("Invalid contact method".to_string());
            }
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "contact_method_validator"
    }
}

// 注册依赖字段验证器
ctx.validation().register_custom_validator(Box::new(ContactMethodValidator)).await?;

// 使用依赖字段验证
let dependency_rule = DMSCValidationRule {
    field: "contact_method".to_string(),
    rule_type: DMSCValidationType::CustomValidator("contact_method_validator".to_string()),
    required: true,
    message: "Contact method validation failed".to_string(),
};

match ctx.validation().validate_data(&form_data, &[dependency_rule]).await {
    Ok(_) => {
        ctx.log().info("Contact form validation successful");
    }
    Err(errors) => {
        ctx.log().error(format!("Contact form validation failed: {:?}", errors));
    }
}
```

## 异步验证

### 数据库验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 用户注册数据
let registration_data = json!({
    "username": "john_doe_2024",
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!",
    "referral_code": "REF123456",
});

// 异步验证器 - 检查用户名和邮箱的唯一性
struct UniqueUserValidator {
    database_connection: String,
}

impl UniqueUserValidator {
    fn new() -> Self {
        Self {
            database_connection: "user_database".to_string(),
        }
    }
    
    async fn check_username_unique(&self, username: &str) -> Result<bool, String> {
        // 模拟数据库查询
        let existing_usernames = vec!["admin", "root", "existing_user"];
        Ok(!existing_usernames.contains(&username.to_lowercase().as_str()))
    }
    
    async fn check_email_unique(&self, email: &str) -> Result<bool, String> {
        // 模拟数据库查询
        let existing_emails = vec!["admin@example.com", "existing@example.com"];
        Ok(!existing_emails.contains(&email.to_lowercase().as_str()))
    }
    
    async fn check_referral_code_valid(&self, code: &str) -> Result<bool, String> {
        // 模拟验证推荐码
        let valid_codes = vec!["REF123456", "REF789012", "REF345678"];
        Ok(valid_codes.contains(&code.to_uppercase().as_str()))
    }
}

#[async_trait::async_trait]
impl DMSCAsyncValidator for UniqueUserValidator {
    async fn validate_async(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        match field_name {
            "username" => {
                let username = value.as_str().ok_or("Username must be a string")?;
                let is_unique = self.check_username_unique(username).await?;
                
                if !is_unique {
                    return Err("Username is already taken".to_string());
                }
            }
            "email" => {
                let email = value.as_str().ok_or("Email must be a string")?;
                let is_unique = self.check_email_unique(email).await?;
                
                if !is_unique {
                    return Err("Email is already registered".to_string());
                }
            }
            "referral_code" => {
                let code = value.as_str().ok_or("Referral code must be a string")?;
                let is_valid = self.check_referral_code_valid(code).await?;
                
                if !is_valid {
                    return Err("Invalid referral code".to_string());
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "unique_user_validator"
    }
}

// 注册异步验证器
let unique_validator = UniqueUserValidator::new();
ctx.validation().register_async_validator(Box::new(unique_validator)).await?;

// 定义异步验证规则
let async_validation_rules = vec![
    DMSCValidationRule {
        field: "username".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("unique_user_validator".to_string()),
        required: true,
        message: "Username validation failed".to_string(),
    },
    DMSCValidationRule {
        field: "email".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("unique_user_validator".to_string()),
        required: true,
        message: "Email validation failed".to_string(),
    },
    DMSCValidationRule {
        field: "referral_code".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("unique_user_validator".to_string()),
        required: false,
        message: "Referral code validation failed".to_string(),
    },
];

// 执行异步验证
match ctx.validation().validate_data_async(&registration_data, &async_validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("User registration validation successful");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
        
        // 继续用户注册流程
        // ...
    }
    Err(validation_errors) => {
        ctx.log().error("User registration validation failed");
        for error in &validation_errors {
            ctx.log().error(format!("Async validation error: {}", error));
        }
    }
}
```

### API验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// API端点验证 - 检查外部API的可用性
let api_data = json!({
    "webhook_url": "https://api.example.com/webhook",
    "api_key": "sk_test_1234567890abcdef",
    "endpoint": "https://api.payment-provider.com/v1/charges",
    "timeout": 30,
});

// API端点验证器
struct ApiEndpointValidator {
    client: reqwest::Client,
}

impl ApiEndpointValidator {
    fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
    
    async fn validate_webhook_url(&self, url: &str) -> Result<bool, String> {
        // 验证Webhook URL格式和可访问性
        match self.client.head(url).send().await {
            Ok(response) => {
                if response.status().is_success() || response.status() == 404 {
                    Ok(true)
                } else {
                    Err(format!("Webhook URL returned status: {}", response.status()))
                }
            }
            Err(e) => {
                Err(format!("Failed to connect to webhook URL: {}", e))
            }
        }
    }
    
    async fn validate_api_key_format(&self, api_key: &str) -> Result<bool, String> {
        // 验证API密钥格式
        let api_key_pattern = regex::Regex::new(r"^sk_(test|live)_[a-zA-Z0-9]{24}$").unwrap();
        
        if api_key_pattern.is_match(api_key) {
            Ok(true)
        } else {
            Err("API key must follow format: sk_(test|live)_[24 alphanumeric characters]".to_string())
        }
    }
    
    async fn validate_payment_endpoint(&self, endpoint: &str) -> Result<bool, String> {
        // 验证支付端点
        if !endpoint.starts_with("https://") {
            return Err("Payment endpoint must use HTTPS".to_string());
        }
        
        if !endpoint.contains("payment-provider") {
            return Err("Invalid payment provider endpoint".to_string());
        }
        
        // 检查端点可用性
        match self.client.head(endpoint).send().await {
            Ok(response) => {
                if response.status().is_success() || response.status() == 401 {
                    Ok(true)
                } else {
                    Err(format!("Payment endpoint returned status: {}", response.status()))
                }
            }
            Err(e) => {
                Err(format!("Failed to connect to payment endpoint: {}", e))
            }
        }
    }
}

#[async_trait::async_trait]
impl DMSCAsyncValidator for ApiEndpointValidator {
    async fn validate_async(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        match field_name {
            "webhook_url" => {
                let url = value.as_str().ok_or("Webhook URL must be a string")?;
                let is_valid = self.validate_webhook_url(url).await?;
                
                if !is_valid {
                    return Err("Invalid webhook URL".to_string());
                }
            }
            "api_key" => {
                let api_key = value.as_str().ok_or("API key must be a string")?;
                let is_valid = self.validate_api_key_format(api_key).await?;
                
                if !is_valid {
                    return Err("Invalid API key format".to_string());
                }
            }
            "endpoint" => {
                let endpoint = value.as_str().ok_or("Endpoint must be a string")?;
                let is_valid = self.validate_payment_endpoint(endpoint).await?;
                
                if !is_valid {
                    return Err("Invalid payment endpoint".to_string());
                }
            }
            _ => {}
        }
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "api_endpoint_validator"
    }
}

// 注册API端点验证器
let api_validator = ApiEndpointValidator::new();
ctx.validation().register_async_validator(Box::new(api_validator)).await?;

// 定义API验证规则
let api_validation_rules = vec![
    DMSCValidationRule {
        field: "webhook_url".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("api_endpoint_validator".to_string()),
        required: true,
        message: "Webhook URL validation failed".to_string(),
    },
    DMSCValidationRule {
        field: "api_key".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("api_endpoint_validator".to_string()),
        required: true,
        message: "API key validation failed".to_string(),
    },
    DMSCValidationRule {
        field: "endpoint".to_string(),
        rule_type: DMSCValidationType::AsyncCustomValidator("api_endpoint_validator".to_string()),
        required: true,
        message: "Payment endpoint validation failed".to_string(),
    },
];

// 执行API验证
match ctx.validation().validate_data_async(&api_data, &api_validation_rules).await {
    Ok(validated_api) => {
        ctx.log().info("API configuration validation successful");
        ctx.log().info(format!("Validated API config: {:?}", validated_api));
    }
    Err(validation_errors) => {
        ctx.log().error("API configuration validation failed");
        for error in &validation_errors {
            ctx.log().error(format!("API validation error: {}", error));
        }
    }
}
```

## 验证配置

### 验证规则配置

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置验证规则模板
let validation_templates = vec![
    DMSCValidationTemplate {
        name: "user_registration".to_string(),
        description: "Validation rules for user registration".to_string(),
        rules: vec![
            DMSCValidationRule {
                field: "username".to_string(),
                rule_type: DMSCValidationType::Pattern("^[a-zA-Z0-9_]{3,20}$".to_string()),
                required: true,
                message: "Username must be 3-20 characters, alphanumeric and underscore only".to_string(),
            },
            DMSCValidationRule {
                field: "email".to_string(),
                rule_type: DMSCValidationType::Email,
                required: true,
                message: "Valid email address is required".to_string(),
            },
            DMSCValidationRule {
                field: "password".to_string(),
                rule_type: DMSCValidationType::CustomValidator("strong_password_validator".to_string()),
                required: true,
                message: "Password does not meet security requirements".to_string(),
            },
        ],
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("category".to_string(), "user_management".to_string());
            meta.insert("priority".to_string(), "high".to_string());
            meta
        },
    },
    DMSCValidationTemplate {
        name: "product_creation".to_string(),
        description: "Validation rules for product creation".to_string(),
        rules: vec![
            DMSCValidationRule {
                field: "name".to_string(),
                rule_type: DMSCValidationType::Length(5, 100),
                required: true,
                message: "Product name must be 5-100 characters".to_string(),
            },
            DMSCValidationRule {
                field: "price".to_string(),
                rule_type: DMSCValidationType::Range(0.01, 10000.0),
                required: true,
                message: "Price must be between $0.01 and $10,000".to_string(),
            },
            DMSCValidationRule {
                field: "sku".to_string(),
                rule_type: DMSCValidationType::Pattern(r"^[A-Z]{2}-\d{4}-\d{3}$".to_string()),
                required: true,
                message: "SKU must follow format: XX-YYYY-ZZZ".to_string(),
            },
        ],
        metadata: {
            let mut meta = std::collections::HashMap::new();
            meta.insert("category".to_string(), "product_management".to_string());
            meta.insert("priority".to_string(), "medium".to_string());
            meta
        },
    },
];

// 注册验证模板
for template in validation_templates {
    ctx.validation().register_validation_template(template).await?;
}

ctx.log().info("Validation templates registered");

// 使用验证模板
let user_data = json!({
    "username": "john_doe",
    "email": "john@example.com",
    "password": "SecureP@ssw0rd!",
});

// 应用用户注册验证模板
match ctx.validation().apply_validation_template("user_registration", &user_data).await {
    Ok(validated_data) => {
        ctx.log().info("User registration validation successful using template");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
    }
    Err(validation_errors) => {
        ctx.log().error("User registration validation failed using template");
        for error in &validation_errors {
            ctx.log().error(format!("Template validation error: {}", error));
        }
    }
}

// 动态验证配置
let dynamic_config = DMSCDynamicValidationConfig {
    enable_caching: true,
    cache_ttl: Duration::from_minutes(30),
    enable_rule_optimization: true,
    parallel_validation: true,
    max_validation_threads: 4,
    error_message_format: DMSCValidationErrorFormat::Detailed,
    enable_field_suggestions: true,
    strict_mode_threshold: 0.8,
};

ctx.validation().configure_dynamic_validation(dynamic_config).await?;
ctx.log().info("Dynamic validation configured");
```

## 错误处理

### 验证错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 验证错误处理示例
let user_data = json!({
    "username": "jo", // 太短
    "email": "invalid-email", // 无效邮箱
    "password": "weak", // 太弱
    "age": 15, // 太小
});

let validation_rules = vec![
    DMSCValidationRule {
        field: "username".to_string(),
        rule_type: DMSCValidationType::Length(3, 20),
        required: true,
        message: "Username must be 3-20 characters".to_string(),
    },
    DMSCValidationRule {
        field: "email".to_string(),
        rule_type: DMSCValidationType::Email,
        required: true,
        message: "Valid email address is required".to_string(),
    },
    DMSCValidationRule {
        field: "password".to_string(),
        rule_type: DMSCValidationType::CustomValidator("strong_password_validator".to_string()),
        required: true,
        message: "Password is not strong enough".to_string(),
    },
    DMSCValidationRule {
        field: "age".to_string(),
        rule_type: DMSCValidationType::Range(18, 100),
        required: true,
        message: "Age must be between 18 and 100".to_string(),
    },
];

match ctx.validation().validate_data(&user_data, &validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("Validation successful");
    }
    Err(validation_errors) => {
        ctx.log().error("Validation failed");
        
        // 分类处理验证错误
        for error in &validation_errors {
            match error.error_type.as_str() {
                "length" => {
                    ctx.log().warn(format!("Length validation failed for field '{}': {}", error.field, error.message));
                }
                "email" => {
                    ctx.log().warn(format!("Email validation failed for field '{}': {}", error.field, error.message));
                }
                "password_strength" => {
                    ctx.log().warn(format!("Password validation failed for field '{}': {}", error.field, error.message));
                }
                "range" => {
                    ctx.log().warn(format!("Range validation failed for field '{}': {}", error.field, error.message));
                }
                _ => {
                    ctx.log().error(format!("Unknown validation error for field '{}': {}", error.field, error.message));
                }
            }
        }
        
        // 生成用户友好的错误消息
        let user_friendly_errors: Vec<String> = validation_errors.iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect();
        
        ctx.log().info(format!("User-friendly errors: {:?}", user_friendly_errors));
        
        // 返回格式化错误
        return Err(DMSCError::validation(format!("Validation failed: {:?}", user_friendly_errors)));
    }
}
```

## 最佳实践

1. **验证顺序**: 先验证必填字段，再验证格式和逻辑
2. **错误消息**: 提供清晰、具体的错误消息
3. **性能优化**: 缓存验证规则和结果
4. **安全考虑**: 防止验证过程中的信息泄露
5. **异步验证**: 对数据库和API验证使用异步方式
6. **验证模板**: 使用验证模板提高代码复用性
7. **渐进式验证**: 支持部分验证和逐步验证
8. **验证日志**: 记录验证过程和结果
9. **验证测试**: 编写验证规则的单元测试
10. **验证文档**: 维护验证规则的文档说明
11. **输入清理**: 始终清理用户输入数据
12. **正则优化**: 优化正则表达式性能
13. **依赖验证**: 处理字段间的依赖关系
14. **条件验证**: 根据上下文动态调整验证规则
15. **验证缓存**: 缓存验证结果提升性能

## Python验证示例

### 基本数据验证

```python
import dmsc
from dmsc import DMSCPyValidationConfig, DMSCValidationRule, DMSCValidationType

config = DMSCPyValidationConfig(
    strict_mode=True,
    stop_on_first_error=False,
    enable_type_coercion=True,
    locale="en",
    timezone="UTC"
)

ctx.validation().init_validation(config)

user_data = {
    "username": "john_doe",
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!",
    "age": 25,
}

validation_rules = [
    DMSCValidationRule(
        field="username",
        rule_type=DMSCValidationType.Pattern("^[a-zA-Z0-9_]{3,20}$"),
        required=True,
        message="Username must be 3-20 characters"
    ),
    DMSCValidationRule(
        field="email",
        rule_type=DMSCValidationType.Email,
        required=True,
        message="Valid email is required"
    ),
    DMSCValidationRule(
        field="age",
        rule_type=DMSCValidationType.Range(18, 100),
        required=True,
        message="Age must be between 18 and 100"
    ),
]

result = ctx.validation().validate_data(user_data, validation_rules)
if result.is_success():
    ctx.log().info("Validation successful")
else:
    ctx.log().error(f"Validation failed: {result.errors}")
```

### 自定义验证器

```python
from dmsc import DMSCPyValidator, DMSCValidationRule, DMSCValidationType

class StrongPasswordValidator(DMSCPyValidator):
    def __init__(self):
        self.common_passwords = {"password", "123456", "qwerty", "admin"}
        self.min_length = 8
    
    def validate(self, value, field_name):
        password = value if isinstance(value, str) else str(value)
        
        if len(password) < self.min_length:
            return "Password must be at least 8 characters"
        
        if password.lower() in self.common_passwords:
            return "Password is too common"
        
        if not any(c.isupper() for c in password):
            return "Password must contain uppercase letter"
        
        if not any(c.islower() for c in password):
            return "Password must contain lowercase letter"
        
        if not any(c.isdigit() for c in password):
            return "Password must contain number"
        
        if not any(c in "!@#$%^&*" for c in password):
            return "Password must contain special character"
        
        return None
    
    @property
    def name(self):
        return "strong_password_validator"

password_validator = StrongPasswordValidator()
ctx.validation().register_custom_validator(password_validator)

password_rule = DMSCValidationRule(
    field="password",
    rule_type=DMSCValidationType.CustomValidator("strong_password_validator"),
    required=True,
    message="Password validation failed"
)

result = ctx.validation().validate_data({"password": "MyS3cur3P@ss!"}, [password_rule])
```

### 数据清理

```python
from dmsc import DMSCSanitizationRule, DMSCSanitizationOperation

dirty_data = {
    "username": "  john_doe  ",
    "email": "JOHN.DOE@EXAMPLE.COM",
    "bio": "<script>alert('XSS')</script>Hello!"
}

sanitization_rules = [
    DMSCSanitizationRule(
        field="username",
        operations=[
            DMSCSanitizationOperation.Trim,
            DMSCSanitizationOperation.ToLowercase
        ]
    ),
    DMSCSanitizationRule(
        field="email",
        operations=[
            DMSCSanitizationOperation.Trim,
            DMSCSanitizationOperation.ToLowercase
        ]
    ),
    DMSCSanitizationRule(
        field="bio",
        operations=[
            DMSCSanitizationOperation.StripHtmlTags,
            DMSCSanitizationOperation.Trim
        ]
    ),
]

sanitized_data = ctx.validation().sanitize_data(dirty_data, sanitization_rules)
ctx.log().info(f"Sanitized data: {sanitized_data}")
```

### 异步验证

```python
from dmsc import DMSCPyAsyncValidator, DMSCValidationRule, DMSCValidationType

class UniqueUserValidator(DMSCPyAsyncValidator):
    def __init__(self):
        self.existing_users = {"admin", "root", "user"}
        self.existing_emails = {"admin@example.com"}
    
    async def validate_async(self, value, field_name):
        if field_name == "username":
            username = value if isinstance(value, str) else str(value)
            if username.lower() in self.existing_users:
                return "Username is already taken"
        elif field_name == "email":
            email = value if isinstance(value, str) else str(value)
            if email.lower() in self.existing_emails:
                return "Email is already registered"
        return None
    
    @property
    def name(self):
        return "unique_user_validator"

unique_validator = UniqueUserValidator()
ctx.validation().register_async_validator(unique_validator)

async_rules = [
    DMSCValidationRule(
        field="username",
        rule_type=DMSCValidationType.AsyncCustomValidator("unique_user_validator"),
        required=True,
        message="Username validation failed"
    ),
    DMSCValidationRule(
        field="email",
        rule_type=DMSCValidationType.AsyncCustomValidator("unique_user_validator"),
        required=True,
        message="Email validation failed"
    ),
]

result = ctx.validation().validate_data_async(
    {"username": "john_doe", "email": "john@example.com"},
    async_rules
)
```

<div align="center">

## 运行步骤

</div>

1. **环境准备**: 确保已安装Rust环境和DMSC框架（或Python 3.8+和dmsc包）
2. **依赖配置**: 在Cargo.toml中添加验证相关依赖（或pip install dmsc）
3. **初始化验证器**: 创建验证管理器并配置验证规则
4. **数据验证**: 使用validate_data执行数据验证
5. **错误处理**: 处理验证失败的情况并返回友好错误消息
6. **清理优化**: 对验证通过的数据进行清理和优化

<div align="center">

## 预期结果

</div>

运行验证示例后，您将看到：

```
[2024-01-15 10:30:45] INFO: User data validation successful
[2024-01-15 10:30:45] INFO: Validated data: {"username": "john_doe", "email": "john.doe@example.com", "password": "SecureP@ssw0rd!", "age": 25}
[2024-01-15 10:30:45] INFO: Product data validation successful
[2024-01-15 10:30:45] INFO: Sanitized data: {"username": "john_doe", "email": "john.doe@example.com", "bio": "Hello, I'm John! Welcome to my profile."}
[2024-01-15 10:30:45] INFO: Custom username validation successful
[2024-01-15 10:30:45] INFO: Strong password validation successful
[2024-01-15 10:30:45] INFO: Contact form validation successful
[2024-01-15 10:30:45] INFO: User registration validation successful
[2024-01-15 10:30:45] INFO: API configuration validation successful
[2024-01-15 10:30:45] INFO: Validation templates registered
```

<div align="center">

## 扩展功能

</div>

### 智能验证引擎

```rust
use dmsc::prelude::*;
use serde_json::json;

// 智能验证引擎配置
let smart_config = DMSCSmartValidationConfig {
    enable_ml_suggestions: true,
    confidence_threshold: 0.85,
    learning_mode: true,
    pattern_recognition: true,
    anomaly_detection: true,
    auto_rule_generation: true,
};

ctx.validation().configure_smart_validation(smart_config).await?;

// 使用智能验证
let user_input = json!({
    "email": "user@exampl.com", // 可能是拼写错误
    "phone": "+1234567890",
    "postal_code": "12345-6789",
});

let smart_result = ctx.validation()
    .validate_with_ml_suggestions(&user_input)
    .await?;

if let Some(suggestions) = smart_result.suggestions {
    for suggestion in suggestions {
        ctx.log().info(format!(
            "ML suggestion for field '{}': {} (confidence: {:.2}%)",
            suggestion.field, suggestion.suggestion, suggestion.confidence * 100.0
        ));
    }
}
```

### 实时验证监控

```rust
use dmsc::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

// 实时验证监控器
struct ValidationMonitor {
    metrics: Arc<RwLock<DMSCValidationMetrics>>,
}

impl ValidationMonitor {
    fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(DMSCValidationMetrics::default())),
        }
    }
    
    async fn record_validation(&self, result: &DMSCValidationResult) {
        let mut metrics = self.metrics.write().await;
        
        metrics.total_validations += 1;
        
        if result.is_success() {
            metrics.successful_validations += 1;
        } else {
            metrics.failed_validations += 1;
            
            for error in &result.errors {
                *metrics.error_types.entry(error.error_type.clone()).or_insert(0) += 1;
                *metrics.field_errors.entry(error.field.clone()).or_insert(0) += 1;
            }
        }
        
        // 计算验证时间
        metrics.total_validation_time += result.validation_time;
        metrics.average_validation_time = metrics.total_validation_time / metrics.total_validations as f64;
    }
    
    async fn get_metrics(&self) -> DMSCValidationMetrics {
        self.metrics.read().await.clone()
    }
}

// 注册监控器
let monitor = ValidationMonitor::new();
ctx.validation().set_validation_monitor(Arc::new(monitor)).await?;

// 获取验证指标
let metrics = ctx.validation().get_validation_metrics().await?;
ctx.log().info(format!("Validation metrics: {:?}", metrics));
```

### 分布式验证协调

```rust
use dmsc::prelude::*;
use serde_json::json;

// 分布式验证配置
let distributed_config = DMSCDistributedValidationConfig {
    node_count: 5,
    replication_factor: 3,
    consistency_level: DMSCConsistencyLevel::Quorum,
    load_balancing: DMSCLoadBalancing::RoundRobin,
    health_check_interval: Duration::from_secs(30),
    failover_timeout: Duration::from_secs(10),
};

ctx.validation().configure_distributed_validation(distributed_config).await?;

// 执行分布式验证
let large_dataset = json!({
    "users": (0..1000).map(|i| json!({
        "id": i,
        "username": format!("user_{}", i),
        "email": format!("user{}@example.com", i),
        "age": 18 + (i % 80),
    })).collect::<Vec<_>>(),
});

let distributed_result = ctx.validation()
    .validate_distributed(&large_dataset, "user_batch_validation")
    .await?;

ctx.log().info(format!(
    "Distributed validation completed: {} successful, {} failed, {} nodes participated",
    distributed_result.successful_count,
    distributed_result.failed_count,
    distributed_result.participating_nodes
));
```

<div align="center">

## 总结

</div>

验证模块为DMSC框架提供了全面的数据验证解决方案，支持从简单字段验证到复杂业务逻辑验证的各种场景。通过智能验证引擎、实时监控和分布式协调等高级特性，验证模块能够处理大规模数据验证需求，确保数据的完整性和安全性。

### 核心功能

- **基本数据验证**: 支持字符串、数字、日期、邮箱等常见数据类型的验证
- **复杂数据验证**: 处理嵌套对象、数组和复杂数据结构的验证
- **数据清理**: 提供数据清理和标准化功能，防止恶意输入
- **自定义验证器**: 支持创建业务特定的验证规则
- **条件验证**: 根据上下文动态调整验证规则
- **异步验证**: 支持数据库和外部API的异步验证
- **验证配置**: 提供灵活的验证规则和模板配置

### 高级特性

- **智能验证引擎**: 基于机器学习的验证建议和异常检测
- **实时监控**: 验证性能指标和错误统计
- **分布式验证**: 支持大规模数据集的分布式验证
- **验证缓存**: 缓存验证结果提升性能
- **依赖验证**: 处理字段间的复杂依赖关系
- **渐进式验证**: 支持部分验证和逐步验证

### 最佳实践

- **验证顺序**: 先验证必填字段，再验证格式和逻辑
- **错误消息**: 提供清晰、具体的错误消息，帮助用户理解问题
- **性能优化**: 使用缓存和异步验证提升验证性能
- **安全考虑**: 防止验证过程中的信息泄露和注入攻击
- **验证模板**: 使用验证模板提高代码复用性和维护性
- **监控分析**: 持续监控验证指标，优化验证规则

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践

