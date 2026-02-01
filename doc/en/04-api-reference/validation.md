<div align="center">

# Validation API Reference

**Version: 0.1.6**

**Last modified date: 2026-02-01**

The validation module provides data validation and sanitization functionality, supporting multiple validation rules and custom validators.

## Module Overview

</div>

The validation module contains the following submodules:

- **rules**: Validation rule definitions
- **validators**: Validator implementations
- **sanitizers**: Data sanitizers
- **format**: Format validation
- **constraints**: Constraint conditions
- **custom**: Custom validation

<div align="center">

## Core Components

</div>

### DMSCValidationManager

Main interface for the validation manager, providing unified access to validation functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `validate(data, rules)` | Data validation | `data: &Value`, `rules: &[DMSCValidationRule]` | `DMSCResult<DMSCValidationResult>` |
| `validate_field(field, value, rules)` | Field validation | `field: &str`, `value: &Value`, `rules: &[DMSCValidationRule]` | `DMSCResult<DMSCValidationResult>` |
| `sanitize(data, sanitizers)` | Data sanitization | `data: Value`, `sanitizers: &[DMSCSanitizer]` | `DMSCResult<Value>` |
| `sanitize_field(field, value, sanitizers)` | Field sanitization | `field: &str`, `value: Value`, `sanitizers: &[DMSCSanitizer]` | `DMSCResult<Value>` |
| `validate_schema(data, schema)` | Schema validation | `data: &Value`, `schema: &DMSCSchema` | `DMSCResult<DMSCValidationResult>` |
| `register_validator(name, validator)` | Register validator | `name: &str`, `validator: impl DMSCValidator` | `DMSCResult<()>` |
| `register_sanitizer(name, sanitizer)` | Register sanitizer | `name: &str`, `sanitizer: impl DMSCSanitizer` | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use serde_json::json;

// Simple data validation
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

// Data sanitization
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

Validation rule enumeration.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Required` | Required field |
| `Optional` | Optional field |
| `Email` | Email format |
| `Url` | URL format |
| `Phone` | Phone number |
| `Numeric` | Numeric format |
| `Alpha` | Alphabetic format |
| `Alphanumeric` | Alphanumeric format |
| `MinLength(usize)` | Minimum length |
| `MaxLength(usize)` | Maximum length |
| `LengthRange(usize, usize)` | Length range |
| `Min(i64)` | Minimum value |
| `Max(i64)` | Maximum value |
| `Range(i64, i64)` | Value range |
| `Pattern(String)` | Regular expression |
| `In(Vec<String>)` | Enumerated values |
| `NotIn(Vec<String>)` | Excluded values |
| `Custom(String)` | Custom validation |

### DMSCValidationResult

Validation result structure.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `is_valid` | `bool` | Whether valid |
| `errors` | `Vec<DMSCValidationError>` | Error list |
| `warnings` | `Vec<DMSCValidationWarning>` | Warning list |
| `field_results` | `HashMap<String, DMSCValidationResult>` | Field validation results |

<div align="center">

## Data Validation

</div>

### Field Validation

```rust
use dmsc::prelude::*;
use serde_json::json;

// Validate user registration data
let user_data = json!({
    "username": "john_doe",
    "email": "john@example.com",
    "password": "SecurePass123!",
    "age": 25,
    "phone": "+1-555-123-4567",
    "website": "https://john.example.com",
    "bio": "Software developer passionate about Rust"
});

// Username validation
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

// Email validation
let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
    DMSCValidationRule::MaxLength(100),
];

let email_result = ctx.validation().validate_field("email", &user_data["email"], &email_rules)?;
if !email_result.is_valid {
    return Err(DMSCError::validation(format!("Email validation failed: {:?}", email_result.errors)));
}

// Password validation
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

// Age validation
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

### Complex Data Validation

```rust
use dmsc::prelude::*;
use serde_json::json;

// Validate order data
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

// Define validation schema
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

// Execute schema validation
let result = ctx.validation().validate_schema(&order_data, &order_schema)?;
if !result.is_valid {
    return Err(DMSCError::validation(format!("Order validation failed: {:?}", result.errors)));
}

ctx.log().info("Order data validation passed");
```

<div align="center">

## Data Sanitization

</div>

### Basic Sanitization

```rust
use dmsc::prelude::*;
use serde_json::json;

// Sanitize user input data
let dirty_data = json!({
    "username": "  John Doe  ",
    "email": "JOHN@EXAMPLE.COM",
    "phone": "+1 (555) 123-4567",
    "bio": "<script>alert('xss')</script><p>Hello <b>World</b>!</p>",
    "website": "https://EXAMPLE.COM/path/",
    "tags": ["  rust  ", "  programming  ", "  web  "]
});

// Apply sanitizers
let sanitizers = vec![
    DMSCSanitizer::Trim,
    DMSCSanitizer::ToLowercase,
    DMSCSanitizer::RemoveHtml,
    DMSCSanitizer::NormalizeWhitespace,
];

let clean_data = ctx.validation().sanitize(dirty_data, &sanitizers)?;
ctx.log().info(format!("Cleaned data: {}", clean_data));

// Field-specific sanitization
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

### Advanced Sanitization

```rust
use dmsc::prelude::*;
use serde_json::json;

// Sanitize HTML content
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

// Sanitize SQL input
let user_input = "admin' OR '1'='1' --";
let safe_sql = ctx.validation().sanitize_sql(user_input);
ctx.log().info(format!("Sanitized SQL: {}", safe_sql));

// Sanitize file path
let file_path = "../../../etc/passwd";
let safe_path = ctx.validation().sanitize_path(file_path)?;
ctx.log().info(format!("Sanitized path: {}", safe_path));

// Sanitize URL
let dirty_url = "https://example.com/path/../../../../etc/passwd";
let safe_url = ctx.validation().sanitize_url(dirty_url)?;
ctx.log().info(format!("Sanitized URL: {}", safe_url));
```

<div align="center">

## Custom Validators

</div>

### Creating Custom Validators

```rust
use dmsc::prelude::*;
use serde_json::Value;

// Create username validator
struct UsernameValidator {
    reserved_names: Vec<String>,
}

impl DMSCValidator for UsernameValidator {
    fn validate(&self, field_name: &str, value: &Value, _params: &[String]) -> DMSCResult<DMSCValidationResult> {
        if let Some(username) = value.as_str() {
            // Check length
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
            
            // Check if starts with a letter
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
            
            // Check reserved names
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
            
            // Check for forbidden words
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

// Create strong password validator
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
    
    // Length scoring
    strength += (password.len() as f64 * 0.3).min(3.0);
    
    // Contains lowercase letters
    if password.chars().any(|c| c.is_lowercase()) {
        strength += 1.0;
    }
    
    // Contains uppercase letters
    if password.chars().any(|c| c.is_uppercase()) {
        strength += 1.0;
    }
    
    // Contains numbers
    if password.chars().any(|c| c.is_numeric()) {
        strength += 1.0;
    }
    
    // Contains special characters
    if password.chars().any(|c| !c.is_alphanumeric()) {
        strength += 1.0;
    }
    
    // Diversity scoring
    let unique_chars: std::collections::HashSet<_> = password.chars().collect();
    strength += (unique_chars.len() as f64 * 0.1).min(2.0);
    
    strength.min(10.0)
}

// Register custom validators
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

// Use custom validators
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

## Conditional Validation

</div>

### Conditional Rules

```rust
use dmsc::prelude::*;
use serde_json::json;

// Conditional validation: If user chooses company registration, company information is required
let registration_data = json!({
    "account_type": "company",
    "personal_info": {
        "name": "John Doe",
        "email": "john@example.com"
    },
    "company_info": {
        "company_name": "Tech Corp",
        "tax_id": "123456789",
        "address": "456 Business Ave"
    }
});

// Define conditional validation rules
let account_type = registration_data["account_type"].as_str().unwrap();

let mut all_errors = vec![];

// Validate personal information (always required)
let personal_info_rules = vec![
    DMSCValidationRule::Required,
];

let name_result = ctx.validation().validate_field(
    "personal_info.name",
    &registration_data["personal_info"]["name"],
    &vec![DMSCValidationRule::Required, DMSCValidationRule::MinLength(2)]
)?;

if !name_result.is_valid {
    all_errors.extend(name_result.errors);
}

let email_result = ctx.validation().validate_field(
    "personal_info.email",
    &registration_data["personal_info"]["email"],
    &vec![DMSCValidationRule::Required, DMSCValidationRule::Email]
)?;

if !email_result.is_valid {
    all_errors.extend(email_result.errors);
}

// Conditional validation: Company information required for company accounts
if account_type == "company" {
    let company_name_rules = vec![
        DMSCValidationRule::Required,
        DMSCValidationRule::MinLength(2),
    ];
    
    let company_name_result = ctx.validation().validate_field(
        "company_info.company_name",
        &registration_data["company_info"]["company_name"],
        &company_name_rules
    )?;
    
    if !company_name_result.is_valid {
        all_errors.extend(company_name_result.errors);
    }
    
    let tax_id_rules = vec![
        DMSCValidationRule::Required,
        DMSCValidationRule::Pattern(r"^\d{9}$".to_string()),
    ];
    
    let tax_id_result = ctx.validation().validate_field(
        "company_info.tax_id",
        &registration_data["company_info"]["tax_id"],
        &tax_id_rules
    )?;
    
    if !tax_id_result.is_valid {
        all_errors.extend(tax_id_result.errors);
    }
}

if !all_errors.is_empty() {
    return Err(DMSCError::validation(format!("Validation failed: {:?}", all_errors)));
}

ctx.log().info("Registration data validation passed");
```

### Multi-field Validation

```rust
use dmsc::prelude::*;
use serde_json::json;

// Multi-field validation: Password and confirm password must match
let password_data = json!({
    "password": "SecurePass123!",
    "confirm_password": "SecurePass123!",
    "old_password": "OldPass123!"
});

// Validate password format
let password_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(8),
    DMSCValidationRule::Pattern(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$".to_string()),
];

let password_result = ctx.validation().validate_field(
    "password",
    &password_data["password"],
    &password_rules
)?;

if !password_result.is_valid {
    return Err(DMSCError::validation(format!("Password validation failed: {:?}", password_result.errors)));
}

// Validate confirm password format
let confirm_password_result = ctx.validation().validate_field(
    "confirm_password",
    &password_data["confirm_password"],
    &password_rules
)?;

if !confirm_password_result.is_valid {
    return Err(DMSCError::validation(format!("Confirm password validation failed: {:?}", confirm_password_result.errors)));
}

// Check if passwords match
if password_data["password"] != password_data["confirm_password"] {
    return Err(DMSCError::validation("Passwords do not match".to_string()));
}

// Check if new password is different from old password
if password_data["password"] == password_data["old_password"] {
    return Err(DMSCError::validation("New password must be different from old password".to_string()));
}

ctx.log().info("Password change validation passed");
```

<div align="center">

## Error Handling

</div>

### Validation Errors

```rust
use dmsc::prelude::*;
use serde_json::json;

// Handle validation errors
let user_data = json!({
    "username": "ab",
    "email": "invalid-email",
    "age": 15
});

let username_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(3),
];

let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
];

let age_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Numeric,
    DMSCValidationRule::Range(18, 120),
];

let username_result = ctx.validation().validate_field("username", &user_data["username"], &username_rules)?;
let email_result = ctx.validation().validate_field("email", &user_data["email"], &email_rules)?;
let age_result = ctx.validation().validate_field("age", &user_data["age"], &age_rules)?;

// Collect all errors
let mut all_errors = vec![];

if !username_result.is_valid {
    all_errors.extend(username_result.errors);
}

if !email_result.is_valid {
    all_errors.extend(email_result.errors);
}

if !age_result.is_valid {
    all_errors.extend(age_result.errors);
}

// Format error messages
if !all_errors.is_empty() {
    let error_messages: Vec<String> = all_errors
        .iter()
        .map(|e| format!("{}: {} ({})", e.field, e.message, e.code))
        .collect();
    
    ctx.log().error(format!("Validation errors: {}", error_messages.join("; ")));
    
    return Err(DMSCError::validation(format!("Validation failed with {} error(s)", all_errors.len())));
}

ctx.log().info("All validations passed");
```

### Custom Error Messages

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create custom error messages
struct CustomErrorMessageValidator {
    custom_messages: HashMap<String, String>,
}

impl DMSCValidator for CustomErrorMessageValidator {
    fn validate(&self, field_name: &str, value: &Value, _params: &[String]) -> DMSCResult<DMSCValidationResult> {
        if let Some(input) = value.as_str() {
            if input.is_empty() {
                let message = self.custom_messages
                    .get("required")
                    .cloned()
                    .unwrap_or_else(|| format!("{} is required", field_name));
                
                return Ok(DMSCValidationResult {
                    is_valid: false,
                    errors: vec![DMSCValidationError {
                        field: field_name.to_string(),
                        message,
                        code: "FIELD_REQUIRED".to_string(),
                    }],
                    warnings: vec![],
                    field_results: HashMap::new(),
                });
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
                    message: format!("{} must be a string", field_name),
                    code: "INVALID_TYPE".to_string(),
                }],
                warnings: vec![],
                field_results: HashMap::new(),
            })
        }
    }
    
    fn name(&self) -> &str {
        "custom_error_message_validator"
    }
}

// Register custom validator with error messages
let mut custom_messages = HashMap::new();
custom_messages.insert("required".to_string(), "Please provide a value for this field".to_string());

ctx.validation().register_validator("custom_message", CustomErrorMessageValidator {
    custom_messages,
})?;

// Use custom validator
let result = ctx.validation().validate_field(
    "username",
    &json!(""),
    &vec![DMSCValidationRule::Custom("custom_message".to_string())]
)?;

if !result.is_valid {
    for error in result.errors {
        ctx.log().error(format!("{}: {}", error.field, error.message));
    }
}
```

<div align="center">

## Performance Optimization

</div>

### Caching Validation Results

```rust
use dmsc::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Validation result cache
type ValidationCache = Arc<RwLock<HashMap<String, DMSCValidationResult>>>;

async fn validate_with_cache(
    ctx: &DMSCServiceContext,
    cache: &ValidationCache,
    field: &str,
    value: &serde_json::Value,
    rules: &[DMSCValidationRule],
) -> DMSCResult<DMSCValidationResult> {
    // Generate cache key
    let cache_key = format!("{}:{}", field, value);
    
    // Check cache
    {
        let cache_read = cache.read().await;
        if let Some(cached_result) = cache_read.get(&cache_key) {
            return Ok(cached_result.clone());
        }
    }
    
    // Perform validation
    let result = ctx.validation().validate_field(field, value, rules)?;
    
    // Cache result
    {
        let mut cache_write = cache.write().await;
        cache_write.insert(cache_key, result.clone());
    }
    
    Ok(result)
}
```

### Batch Validation

```rust
use dmsc::prelude::*;
use serde_json::json;

// Batch validation for multiple records
let users_data = vec![
    json!({"username": "user1", "email": "user1@example.com", "age": 25}),
    json!({"username": "user2", "email": "user2@example.com", "age": 30}),
    json!({"username": "user3", "email": "user3@example.com", "age": 35}),
];

let username_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(3),
    DMSCValidationRule::MaxLength(20),
];

let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
];

let age_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Numeric,
    DMSCValidationRule::Range(18, 120),
];

let mut validation_results = Vec::new();

for (index, user_data) in users_data.iter().enumerate() {
    let mut user_errors = vec![];
    
    let username_result = ctx.validation().validate_field(
        "username",
        &user_data["username"],
        &username_rules
    )?;
    
    if !username_result.is_valid {
        user_errors.extend(username_result.errors);
    }
    
    let email_result = ctx.validation().validate_field(
        "email",
        &user_data["email"],
        &email_rules
    )?;
    
    if !email_result.is_valid {
        user_errors.extend(email_result.errors);
    }
    
    let age_result = ctx.validation().validate_field(
        "age",
        &user_data["age"],
        &age_rules
    )?;
    
    if !age_result.is_valid {
        user_errors.extend(age_result.errors);
    }
    
    validation_results.push((index, user_errors));
}

// Process validation results
for (index, errors) in validation_results {
    if errors.is_empty() {
        ctx.log().info(format!("User {} validation passed", index));
    } else {
        ctx.log().error(format!("User {} validation failed: {:?}", index, errors));
    }
}
```

<div align="center">

## Security Best Practices

</div>

### Input Sanitization

```rust
use dmsc::prelude::*;
use serde_json::json;

// Always sanitize user input before processing
let user_input = json!({
    "username": "<script>alert('xss')</script>admin",
    "email": "admin@example.com",
    "bio": "Hello <img src=x onerror=alert('xss')>World",
    "comment": "Nice post! <a href='javascript:alert(1)'>Click</a>"
});

// Apply comprehensive sanitization
let sanitizers = vec![
    DMSCSanitizer::Trim,
    DMSCSanitizer::RemoveHtml,
    DMSCSanitizer::NormalizeWhitespace,
    DMSCSanitizer::EscapeSpecialChars,
];

let sanitized_input = ctx.validation().sanitize(user_input, &sanitizers)?;

// Additional security: Validate after sanitization
let username_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Alphanumeric,
    DMSCValidationRule::MinLength(3),
];

let username_result = ctx.validation().validate_field(
    "username",
    &sanitized_input["username"],
    &username_rules
)?;

if !username_result.is_valid {
    return Err(DMSCError::validation("Invalid username after sanitization".to_string()));
}

ctx.log().info(format!("Sanitized and validated input: {}", sanitized_input));
```

### SQL Injection Prevention

```rust
use dmsc::prelude::*;

// Sanitize SQL input to prevent injection attacks
let user_search = "admin' OR '1'='1' --";

let safe_search = ctx.validation().sanitize_sql(user_search);

// Use parameterized queries instead of string concatenation
let query = "SELECT * FROM users WHERE username = ?";

// The sanitized input can now be safely used in parameterized queries
ctx.log().info(format!("Safe search term: {}", safe_search));
```

### Path Traversal Prevention

```rust
use dmsc::prelude::*;

// Sanitize file paths to prevent directory traversal attacks
let user_path = "../../../etc/passwd";

let safe_path = ctx.validation().sanitize_path(user_path)?;

// The sanitized path will be relative to a safe base directory
ctx.log().info(format!("Safe path: {}", safe_path));

// Additional validation: Ensure path is within allowed directory
let allowed_base = "/var/www/uploads";

if !safe_path.starts_with(allowed_base) {
    return Err(DMSCError::validation("Path traversal attempt detected".to_string()));
}
```

<div align="center">

## Testing and Debugging

</div>

### Validation Testing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Test validation with various inputs
#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        let valid_emails = vec![
            "test@example.com",
            "user.name@example.co.uk",
            "user+tag@example.com",
        ];
        
        let invalid_emails = vec![
            "invalid",
            "@example.com",
            "user@",
            "user@.com",
        ];
        
        for email in valid_emails {
            let result = ctx.validation().validate_field(
                "email",
                &json!(email),
                &vec![DMSCValidationRule::Required, DMSCValidationRule::Email]
            ).unwrap();
            
            assert!(result.is_valid, "Valid email {} failed validation", email);
        }
        
        for email in invalid_emails {
            let result = ctx.validation().validate_field(
                "email",
                &json!(email),
                &vec![DMSCValidationRule::Required, DMSCValidationRule::Email]
            ).unwrap();
            
            assert!(!result.is_valid, "Invalid email {} passed validation", email);
        }
    }
    
    #[test]
    fn test_password_strength() {
        let weak_passwords = vec!["password", "123456", "abc"];
        let strong_passwords = vec!["SecurePass123!", "MyP@ssw0rd", "Str0ng!P@ss"];
        
        for password in weak_passwords {
            let result = ctx.validation().validate_field(
                "password",
                &json!(password),
                &vec![DMSCValidationRule::Required, DMSCValidationRule::MinLength(8)]
            ).unwrap();
            
            assert!(!result.is_valid || password.len() >= 8, "Weak password {} passed validation", password);
        }
        
        for password in strong_passwords {
            let result = ctx.validation().validate_field(
                "password",
                &json!(password),
                &vec![DMSCValidationRule::Required, DMSCValidationRule::MinLength(8)]
            ).unwrap();
            
            assert!(result.is_valid, "Strong password {} failed validation", password);
        }
    }
}
```

### Debugging Validation

```rust
use dmsc::prelude::*;
use serde_json::json;

// Enable detailed validation logging for debugging
let debug_data = json!({
    "username": "test_user",
    "email": "test@example.com",
    "age": 25
});

let rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(3),
];

let result = ctx.validation().validate_field("username", &debug_data["username"], &rules)?;

// Log detailed validation results
ctx.log().info(format!("Validation result: {:?}", result));

if !result.is_valid {
    ctx.log().error("Validation failed:");
    for error in &result.errors {
        ctx.log().error(format!("  Field: {}", error.field));
        ctx.log().error(format!("  Message: {}", error.message));
        ctx.log().error(format!("  Code: {}", error.code));
    }
    
    for warning in &result.warnings {
        ctx.log().warn(format!("  Warning: {}", warning.message));
    }
} else {
    ctx.log().info("Validation passed successfully");
}
```
