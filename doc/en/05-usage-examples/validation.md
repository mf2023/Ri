# Validation Usage Examples

The validation module provides usage examples for data validation, data sanitization, custom validators, conditional validation, asynchronous validation, and validation configuration features.

## Basic Data Validation

### Simple Field Validation

```rust
use dms::prelude::*;
use serde_json::json;

// Initialize validation manager
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

// User registration data validation
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

// Define validation rules
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

// Execute validation
match ctx.validation().validate_data(&user_data, &validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("User data validation successful");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
        
        // Continue processing user registration
        // ...
    }
    Err(validation_errors) => {
        ctx.log().error("User data validation failed");
        for error in &validation_errors {
            ctx.log().error(format!("Validation error: {}", error));
        }
        
        // Return validation errors to frontend
        return Err(DMSCError::validation(format!("Validation failed: {:?}", validation_errors)));
    }
}
```

### Complex Data Validation

```rust
use dms::prelude::*;
use serde_json::json;

// Product data validation
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

// Complex validation rules
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

// Execute complex validation
match ctx.validation().validate_complex_data(&product_data, &complex_rules).await {
    Ok(validated_product) => {
        ctx.log().info("Product data validation successful");
        
        // Further process product data
        // ...
    }
    Err(errors) => {
        ctx.log().error("Product validation failed");
        // Handle validation errors
        return Err(DMSCError::validation(format!("Product validation failed: {:?}", errors)));
    }
}
```

## Data Sanitization

### Basic Data Sanitization

```rust
use dms::prelude::*;
use serde_json::json;

// User input data sanitization
let dirty_data = json!({
    "username": "  john_doe  ",
    "email": "JOHN.DOE@EXAMPLE.COM",
    "phone": "+1 (234) 567-8900",
    "website": "https://johndoe.com/",
    "bio": "<script>alert('XSS')</script>Hello, I'm John! <b>Welcome</b> to my profile.",
    "tags": ["  developer  ", "  designer  ", "  photographer  "],
    "skills": ["  Rust  ", "  JavaScript  ", "  Python  "],
});

// Define sanitization rules
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

// Execute data sanitization
let sanitized_data = ctx.validation()
    .sanitize_data(&dirty_data, &sanitization_rules)
    .await?;

ctx.log().info(format!("Original data: {:?}", dirty_data));
ctx.log().info(format!("Sanitized data: {:?}", sanitized_data));

// Validate sanitized data
let is_valid = ctx.validation()
    .validate_sanitized_data(&sanitized_data, &validation_rules)
    .await?;

ctx.log().info(format!("Sanitized data is valid: {}", is_valid));
```

### Advanced Data Sanitization

```rust
use dms::prelude::*;
use serde_json::json;

// Rich text content sanitization
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

// Advanced sanitization rules
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

// Execute advanced sanitization
let sanitized_content = ctx.validation()
    .sanitize_data_advanced(&rich_content, &advanced_rules)
    .await?;

ctx.log().info(format!("Sanitized rich content: {:?}", sanitized_content));

// SQL injection protection sanitization
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

## Custom Validators

### Username Validator

```rust
use dms::prelude::*;
use std::collections::HashSet;

// Create custom username validator
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
        
        // Check length
        if username.len() < self.min_length {
            return Err(format!("Username must be at least {} characters long", self.min_length));
        }
        
        if username.len() > self.max_length {
            return Err(format!("Username must be no more than {} characters long", self.max_length));
        }
        
        // Check character set
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
            return Err("Username can only contain letters, numbers, underscore, and hyphen".to_string());
        }
        
        // Check if starts with number
        if username.chars().next().unwrap_or_default().is_numeric() {
            return Err("Username cannot start with a number".to_string());
        }
        
        // Check reserved words
        let lower_username = username.to_lowercase();
        if self.reserved_usernames.contains(&lower_username) {
            return Err("This username is reserved and cannot be used".to_string());
        }
        
        // Check consecutive characters
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

// Register custom validator
let username_validator = UsernameValidator::new();
ctx.validation().register_custom_validator(Box::new(username_validator)).await?;

// Use custom validator
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

### Strong Password Validator

```rust
use dms::prelude::*;
use std::collections::HashSet;

// Create strong password validator
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
        
        // Check length
        if password.len() < self.min_length {
            return Err(format!("Password must be at least {} characters long", self.min_length));
        }
        
        // Check for common passwords
        let lower_password = password.to_lowercase();
        if self.common_passwords.contains(&lower_password) {
            return Err("Password is too common, please choose a more unique password".to_string());
        }
        
        // Check character types
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
        
        // Check consecutive characters
        if self.has_consecutive_chars(password, self.max_consecutive_chars) {
            return Err(format!("Password cannot have {} or more consecutive identical characters", self.max_consecutive_chars));
        }
        
        // Check entropy
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

// Register strong password validator
let password_validator = StrongPasswordValidator::new();
ctx.validation().register_custom_validator(Box::new(password_validator)).await?;

// Use strong password validator
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

## Conditional Validation

### Dynamic Validation Rules

```rust
use dms::prelude::*;
use serde_json::json;

// Order validation - apply different rules based on order type
let order_data = json!({
    "order_type": "international", // or "domestic"
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

// Get validation rules based on order type
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

// Get conditional validation rules
let order_type = order_data["order_type"].as_str().unwrap_or("");
let conditional_rules = get_validation_rules(order_type);

// Execute conditional validation
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

### Dependent Field Validation

```rust
use dms::prelude::*;
use serde_json::json;

// Form validation - inter-field dependencies
let form_data = json!({
    "contact_method": "email", // or "phone", "mail"
    "email": "john@example.com",
    "phone": "+1234567890",
    "address": {
        "street": "123 Main St",
        "city": "New York",
        "state": "NY",
        "zip_code": "10001"
    },
    "preferred_contact_time": "morning", // or "afternoon", "evening"
    "contact_restriction": "weekdays_only", // or "anytime", "weekends_only"
});

// Dependent field validator
struct ContactMethodValidator;

impl DMSCValidator for ContactMethodValidator {
    fn validate(&self, value: &serde_json::Value, field_name: &str) -> Result<(), String> {
        let contact_method = value["contact_method"].as_str().unwrap_or("");
        
        match contact_method {
            "email" => {
                if value.get("email").is_none() || value["email"].as_str().unwrap_or_default().is_empty() {
                    return Err("Email is required when contact method is email".to_string());
                }
                
                // Validate email format
                let email = value["email"].as_str().unwrap();
                if !email.contains('@') || !email.contains('.') {
                    return Err("Invalid email format".to_string());
                }
            }
            "phone" => {
                if value.get("phone").is_none() || value["phone"].as_str().unwrap_or_default().is_empty() {
                    return Err("Phone number is required when contact method is phone".to_string());
                }
                
                // Validate phone format
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
                
                // Validate address fields
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

// Register dependent field validator
ctx.validation().register_custom_validator(Box::new(ContactMethodValidator)).await?;

// Use dependent field validation
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

## Asynchronous Validation

### Database Validation

```rust
use dms::prelude::*;
use serde_json::json;

// User registration data
let registration_data = json!({
    "username": "john_doe_2024",
    "email": "john.doe@example.com",
    "password": "SecureP@ssw0rd!",
    "referral_code": "REF123456",
});

// Async validator - check username and email uniqueness
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
        // Simulate database query
        let existing_usernames = vec!["admin", "root", "existing_user"];
        Ok(!existing_usernames.contains(&username.to_lowercase().as_str()))
    }
    
    async fn check_email_unique(&self, email: &str) -> Result<bool, String> {
        // Simulate database query
        let existing_emails = vec!["admin@example.com", "existing@example.com"];
        Ok(!existing_emails.contains(&email.to_lowercase().as_str()))
    }
    
    async fn check_referral_code_valid(&self, code: &str) -> Result<bool, String> {
        // Simulate referral code validation
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

// Register async validator
let unique_validator = UniqueUserValidator::new();
ctx.validation().register_async_validator(Box::new(unique_validator)).await?;

// Define async validation rules
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

// Execute async validation
match ctx.validation().validate_data_async(&registration_data, &async_validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("User registration validation successful");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
        
        // Continue user registration process
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

### API Validation

```rust
use dms::prelude::*;
use serde_json::json;

// API endpoint validation - check external API availability
let api_data = json!({
    "webhook_url": "https://api.example.com/webhook",
    "api_key": "sk_test_1234567890abcdef",
    "endpoint": "https://api.payment-provider.com/v1/charges",
    "timeout": 30,
});

// API endpoint validator
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
        // Validate Webhook URL format and accessibility
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
        // Validate API key format
        let api_key_pattern = regex::Regex::new(r"^sk_(test|live)_[a-zA-Z0-9]{24}$").unwrap();
        
        if api_key_pattern.is_match(api_key) {
            Ok(true)
        } else {
            Err("API key must follow format: sk_(test|live)_[24 alphanumeric characters]".to_string())
        }
    }
    
    async fn validate_payment_endpoint(&self, endpoint: &str) -> Result<bool, String> {
        // Validate payment endpoint
        if !endpoint.starts_with("https://") {
            return Err("Payment endpoint must use HTTPS".to_string());
        }
        
        if !endpoint.contains("payment-provider") {
            return Err("Invalid payment provider endpoint".to_string());
        }
        
        // Check endpoint availability
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

// Register API endpoint validator
let api_validator = ApiEndpointValidator::new();
ctx.validation().register_async_validator(Box::new(api_validator)).await?;

// Define API validation rules
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

// Execute API validation
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

## Validation Configuration

### Validation Rules Configuration

```rust
use dms::prelude::*;
use serde_json::json;

// Configure validation rule templates
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

// Register validation templates
for template in validation_templates {
    ctx.validation().register_validation_template(template).await?;
}

ctx.log().info("Validation templates registered");

// Use validation templates
let user_data = json!({
    "username": "john_doe",
    "email": "john@example.com",
    "password": "SecureP@ssw0rd!",
});

// Apply user registration validation template
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

// Dynamic validation configuration
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

## Error Handling

### Validation Error Handling

```rust
use dms::prelude::*;
use serde_json::json;

// Validation error handling example
let user_data = json!({
    "username": "jo", // too short
    "email": "invalid-email", // invalid email
    "password": "weak", // too weak
    "age": 15, // too young
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
        
        // Categorize validation errors
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
        
        // Generate user-friendly error messages
        let user_friendly_errors: Vec<String> = validation_errors.iter()
            .map(|e| format!("{}: {}", e.field, e.message))
            .collect();
        
        ctx.log().info(format!("User-friendly errors: {:?}", user_friendly_errors));
        
        // Return formatted errors
        return Err(DMSCError::validation(format!("Validation failed: {:?}", user_friendly_errors)));
    }
}
```

## Best Practices

1. **Validation Order**: Validate required fields first, then format and logic
2. **Error Messages**: Provide clear, specific error messages
3. **Performance Optimization**: Cache validation rules and results
4. **Security Considerations**: Prevent information leakage during validation
5. **Asynchronous Validation**: Use asynchronous methods for database and API validation
6. **Validation Templates**: Use validation templates to improve code reusability
7. **Progressive Validation**: Support partial validation and step-by-step validation
8. **Validation Logging**: Record validation processes and results
9. **Validation Testing**: Write unit tests for validation rules
10. **Validation Documentation**: Maintain documentation for validation rules
11. **Input Sanitization**: Always sanitize user input data
12. **Regex Optimization**: Optimize regular expression performance
13. **Dependency Validation**: Handle inter-field dependencies
14. **Conditional Validation**: Dynamically adjust validation rules based on context
15. **Validation Caching**: Cache validation results to improve performance

<div align="center">

## Running Steps

</div>

1. **Environment Preparation**: Ensure Rust environment and DMSC framework are installed
2. **Dependency Configuration**: Add validation-related dependencies in Cargo.toml
3. **Initialize Validator**: Create validation manager and configure validation rules
4. **Data Validation**: Use validate_data to perform data validation
5. **Error Handling**: Handle validation failures and return user-friendly error messages
6. **Sanitization Optimization**: Clean and optimize validated data

<div align="center">

## Expected Results

</div>

After running the validation examples, you will see:

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

## Extended Features

</div>

### Smart Validation Engine

```rust
use dms::prelude::*;
use serde_json::json;

// Smart validation engine configuration
let smart_config = DMSCSmartValidationConfig {
    enable_ml_suggestions: true,
    confidence_threshold: 0.85,
    learning_mode: true,
    pattern_recognition: true,
    anomaly_detection: true,
    auto_rule_generation: true,
};

ctx.validation().configure_smart_validation(smart_config).await?;

// Use smart validation
let user_input = json!({
    "email": "user@exampl.com", // possible typo
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

### Real-time Validation Monitoring

```rust
use dms::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

// Real-time validation monitor
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
        
        // Calculate validation time
        metrics.total_validation_time += result.validation_time;
        metrics.average_validation_time = metrics.total_validation_time / metrics.total_validations as f64;
    }
    
    async fn get_metrics(&self) -> DMSCValidationMetrics {
        self.metrics.read().await.clone()
    }
}

// Register monitor
let monitor = ValidationMonitor::new();
ctx.validation().set_validation_monitor(Arc::new(monitor)).await?;

// Get validation metrics
let metrics = ctx.validation().get_validation_metrics().await?;
ctx.log().info(format!("Validation metrics: {:?}", metrics));
```

### Distributed Validation Coordination

```rust
use dms::prelude::*;
use serde_json::json;

// Distributed validation configuration
let distributed_config = DMSCDistributedValidationConfig {
    node_count: 5,
    replication_factor: 3,
    consistency_level: DMSCConsistencyLevel::Quorum,
    load_balancing: DMSCLoadBalancing::RoundRobin,
    health_check_interval: Duration::from_secs(30),
    failover_timeout: Duration::from_secs(10),
};

ctx.validation().configure_distributed_validation(distributed_config).await?;

// Execute distributed validation
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

## Summary

</div>

The validation module provides a comprehensive data validation solution for the DMSC framework, supporting various scenarios from simple field validation to complex business logic validation. Through advanced features such as intelligent validation engines, real-time monitoring, and distributed coordination, the validation module can handle large-scale data validation requirements, ensuring data integrity and security.

### Core Features

- **Basic Data Validation**: Supports validation of common data types such as strings, numbers, dates, emails, etc.
- **Complex Data Validation**: Handles validation of nested objects, arrays, and complex data structures
- **Data Sanitization**: Provides data cleaning and standardization functions to prevent malicious input
- **Custom Validators**: Supports creation of business-specific validation rules
- **Conditional Validation**: Dynamically adjusts validation rules based on context
- **Asynchronous Validation**: Supports asynchronous validation for databases and external APIs
- **Validation Configuration**: Provides flexible validation rules and template configuration

### Advanced Features

- **Smart Validation Engine**: Machine learning-based validation suggestions and anomaly detection
- **Real-time Monitoring**: Validation performance metrics and error statistics
- **Distributed Validation**: Supports distributed validation for large-scale datasets
- **Validation Caching**: Caches validation results to improve performance
- **Dependency Validation**: Handles complex dependencies between fields
- **Progressive Validation**: Supports partial validation and step-by-step validation

### Best Practices

- **Validation Order**: Validate required fields first, then format and logic
- **Error Messages**: Provide clear, specific error messages to help users understand issues
- **Performance Optimization**: Use caching and asynchronous validation to improve validation performance
- **Security Considerations**: Prevent information leakage and injection attacks during validation
- **Validation Templates**: Use validation templates to improve code reusability and maintainability
- **Monitoring Analysis**: Continuously monitor validation metrics and optimize validation rules

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application examples, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, learn how to use caching modules to improve application performance
- [database](./database.md): Database examples, learn database connections and query operations
- [http](./http.md): HTTP service examples, build web applications and RESTful APIs
- [mq](./mq.md): Message queue examples, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing and security best practices

