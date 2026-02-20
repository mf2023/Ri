<div align="center">

# Validation API Reference

**Version: 0.1.8**

**Last modified date: 2026-02-20**

The validation module provides data validation and sanitization functionality, supporting multiple validation rules and custom validators.

## Module Overview

</div>

The validation module contains the following submodules:

- **rules**: Validation rule definitions
- **validators**: Validator implementations
- **sanitizers**: Data sanitizers

<div align="center">

## Core Components

</div>

### DMSCValidationModule

Validation module providing unified access to validation functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `validate_email(value)` | Validate email | `value: String` | `DMSCValidationResult` |
| `validate_username(value)` | Validate username | `value: String` | `DMSCValidationResult` |
| `validate_password(value)` | Validate password | `value: String` | `DMSCValidationResult` |
| `validate_url(value)` | Validate URL | `value: String` | `DMSCValidationResult` |
| `validate_ip(value)` | Validate IP address | `value: String` | `DMSCValidationResult` |
| `validate_not_empty(field, value)` | Validate not empty | `field: String`, `value: String` | `DMSCValidationResult` |
| `validate_length(field, value, min, max)` | Validate length | `field: String`, `value: String`, `min: usize`, `max: usize` | `DMSCValidationResult` |

#### Usage Example

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

Validation severity enumeration.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Error` | Error |
| `Warning` | Warning |
| `Info` | Information |
| `Critical` | Critical |

### DMSCValidationError

Validation error structure.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `field` | `String` | Field name |
| `message` | `String` | Error message |
| `code` | `String` | Error code |
| `severity` | `DMSCValidationSeverity` | Severity level |
| `value` | `Option<Value>` | Optional value |

### DMSCValidationResult

Validation result structure.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `is_valid` | `bool` | Whether valid |
| `errors` | `Vec<DMSCValidationError>` | Error list |
| `warnings` | `Vec<DMSCValidationError>` | Warning list |

<div align="center">

## Validator Builder

</div>

### DMSCValidatorBuilder

Validator builder for constructing complex validation rules.

#### Methods

| Method | Description |
|:--------|:-------------|
| `new(field_name)` | Create new validator |
| `not_empty()` | Validate not empty |
| `min_length(n)` | Minimum length |
| `max_length(n)` | Maximum length |
| `exact_length(n)` | Exact length |
| `is_email()` | Validate email format |
| `is_url()` | Validate URL format |
| `is_ip()` | Validate IP address format |
| `is_uuid()` | Validate UUID format |
| `is_base64()` | Validate Base64 format |
| `min_value(n)` | Minimum value |
| `max_value(n)` | Maximum value |
| `range(min, max)` | Value range |
| `matches_regex(pattern)` | Match regex pattern |
| `alphanumeric()` | Validate alphanumeric |
| `alphabetic()` | Validate alphabetic |
| `numeric()` | Validate numeric |
| `lowercase()` | Validate lowercase |
| `uppercase()` | Validate uppercase |
| `contains(substring)` | Contains substring |
| `starts_with(prefix)` | Starts with prefix |
| `ends_with(suffix)` | Ends with suffix |
| `is_in(values)` | Value in list |
| `not_in(values)` | Value not in list |
| `build()` | Build validator |

#### Usage Example

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

Validation runner for executing validation.

#### Methods

| Method | Description |
|:--------|:-------------|
| `validate(value)` | Validate string value |
| `validate_optional(value)` | Validate optional string |

<div align="center">

## Data Sanitization

</div>

### DMSCSanitizer

Data sanitizer.

#### Methods

| Method | Description |
|:--------|:-------------|
| `new()` | Create new sanitizer |
| `with_config(config)` | Create with config |
| `sanitize(input)` | Sanitize input string |
| `sanitize_email(input)` | Sanitize email |
| `sanitize_filename(input)` | Sanitize filename |

### DMSCSanitizationConfig

Sanitization configuration.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `trim_whitespace` | `bool` | Trim whitespace | `true` |
| `lowercase` | `bool` | Convert to lowercase | `false` |
| `uppercase` | `bool` | Convert to uppercase | `false` |
| `remove_extra_spaces` | `bool` | Remove extra spaces | `false` |
| `remove_html_tags` | `bool` | Remove HTML tags | `false` |
| `escape_special_chars` | `bool` | Escape special chars | `false` |

<div align="center">

## Schema Validation

</div>

### DMSCSchemaValidator

Schema validator for validating data structures.

```rust
use dmsc::validation::DMSCSchemaValidator;

let schema = r#"{"type": "string", "minLength": 5}"#;
let validator = DMSCSchemaValidator::new(schema.to_string());

let result = validator.validate_json(r#""hello""#.to_string());
```

<div align="center">

## Best Practices

</div>

1. **Use Validator Builder**: Use `DMSCValidatorBuilder` to build complex validation rules
2. **Provide Clear Error Messages**: Return meaningful error messages on validation failure
3. **Validate Input Data**: Always validate user input data
4. **Sanitize Sensitive Data**: Use sanitizers to handle sensitive information

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, handling user authentication and authorization
- [cache](./cache.md): Cache module, providing memory cache and distributed cache support
- [config](./config.md): Configuration module, managing application configuration
- [core](./core.md): Core module, providing error handling and service context
- [database](./database.md): Database module, providing database operation support
- [device](./device.md): Device module, using protocols for device communication
- [fs](./fs.md): File system module, providing file operation functionality
- [gateway](./gateway.md): Gateway module, providing API gateway functionality
- [grpc](./grpc.md): gRPC module, with service registration and Python bindings
- [hooks](./hooks.md): Hooks module, providing lifecycle hook support
- [log](./log.md): Logging module, recording protocol events
- [observability](./observability.md): Observability module, monitoring protocol performance
- [protocol](./protocol.md): Protocol module, providing communication protocol support
- [service_mesh](./service_mesh.md): Service mesh module, using protocols for inter-service communication
- [ws](./ws.md): WebSocket module, with Python bindings for real-time communication
