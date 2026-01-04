<div align="center">

# Security API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The security module provides security functionality including authentication, authorization, encryption, input validation, and protection mechanisms.

## Module Overview

</div>

The security module contains the following sub-modules:

- **auth**: Authentication management
- **authorization**: Authorization management
- **crypto**: Encryption and decryption
- **validation**: Input validation
- **rate_limit**: Rate limiting
- **cors**: CORS configuration
- **csrf**: CSRF protection
- **xss**: XSS protection
- **sql_injection**: SQL injection protection

<div align="center">

## Core Components

</div>

### DMSCSecurityManager

The security manager main interface provides unified access to security functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `authenticate(credentials)` | User authentication | `credentials: DMSCCredentials` | `DMSCResult<DMSCUser>` |
| `authorize(user, resource, action)` | Permission check | `user: &DMSCUser`, `resource: &str`, `action: &str` | `DMSCResult<bool>` |
| `encrypt(data, key)` | Data encryption | `data: &[u8]`, `key: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt(data, key)` | Data decryption | `data: &[u8]`, `key: &[u8]` | `DMSCResult<Vec<u8>>` |
| `hash_password(password)` | Password hashing | `password: &str` | `DMSCResult<String>` |
| `verify_password(password, hash)` | Password verification | `password: &str`, `hash: &str` | `DMSCResult<bool>` |
| `validate_input(input, rules)` | Input validation | `input: &str`, `rules: &[DMSCValidationRule]` | `DMSCResult<()>` |
| `rate_limit(key, limit)` | Rate limiting | `key: &str`, `limit: DMSCRateLimit` | `DMSCResult<()>` |

#### Usage Example

```rust
use dms::prelude::*;

// User authentication
let credentials = DMSCCredentials::UsernamePassword {
    username: "john_doe".to_string(),
    password: "secure_password".to_string(),
};

match ctx.security().authenticate(credentials).await? {
    Some(user) => {
        ctx.log().info(format!("User authenticated: {}", user.username));

        // Check permissions
        if ctx.security().authorize(&user, "user:profile", "read").await? {
            ctx.log().info("User has read permission for profile");
        }
    }
    None => {
        ctx.log().warn("Authentication failed");
        return Err(DMSCError::auth("Invalid credentials"));
    }
}

// Data encryption
let sensitive_data = b"confidential information";
let encryption_key = ctx.security().derive_key("my_secret_key", b"salt", 32)?;
let encrypted_data = ctx.security().encrypt(sensitive_data, &encryption_key)?;

// Password hashing
let password = "user_password";
let password_hash = ctx.security().hash_password(password)?;

// Verify password
let is_valid = ctx.security().verify_password(password, &password_hash)?;
```

### DMSCCredentials

Authentication credential enumeration.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `UsernamePassword { username, password }` | Username and password |
| `ApiKey { key }` | API key |
| `JwtToken { token }` | JWT token |
| `OAuth2 { code, provider }` | OAuth2 authorization code |
| `Certificate { cert_data }` | Client certificate |

### DMSCUser

User structure.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | User ID |
| `username` | `String` | Username |
| `email` | `String` | Email |
| `roles` | `Vec<String>` | Role list |
| `permissions` | `Vec<String>` | Permission list |
| `metadata` | `HashMap<String, String>` | Metadata |
| `created_at` | `DateTime<Utc>` | Creation time |
| `last_login` | `Option<DateTime<Utc>>` | Last login time |

<div align="center">

## Authentication Management

</div>

### JWT Authentication

```rust
use dms::prelude::*;

// Generate JWT token
let jwt_config = DMSCJwtConfig {
    secret_key: "my_jwt_secret".to_string(),
    expiration: Duration::from_hours(24),
    issuer: "my-app".to_string(),
    audience: "users".to_string(),
    algorithm: DMSCJwtAlgorithm::HS256,
};

let user = DMSCUser {
    id: "12345".to_string(),
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    roles: vec!["user".to_string(), "admin".to_string()],
    permissions: vec!["read".to_string(), "write".to_string()],
    metadata: HashMap::new(),
    created_at: Utc::now(),
    last_login: Some(Utc::now()),
};

let jwt_token = ctx.security().generate_jwt(&user, &jwt_config)?;
ctx.log().info(format!("Generated JWT token: {}", jwt_token));

// Validate JWT token
match ctx.security().validate_jwt(&jwt_token, &jwt_config)? {
    Some(claims) => {
        ctx.log().info(format!("JWT valid for user: {}", claims.sub));

        // Extract user information from JWT
        let user_from_jwt = ctx.security().user_from_jwt_claims(&claims)?;
        ctx.log().info(format!("User roles: {:?}", user_from_jwt.roles));
    }
    None => {
        ctx.log().warn("Invalid JWT token");
        return Err(DMSCError::auth("Invalid JWT token"));
    }
}
```

### OAuth2 Authentication

```rust
use dms::prelude::*;

// OAuth2 configuration
let oauth2_config = DMSCOAuth2Config {
    client_id: "my_client_id".to_string(),
    client_secret: "my_client_secret".to_string(),
    authorization_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
    token_url: "https://oauth2.googleapis.com/token".to_string(),
    redirect_uri: "http://localhost:8080/oauth2/callback".to_string(),
    scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
    provider: "google".to_string(),
};

// Generate authorization URL
let auth_url = ctx.security().generate_oauth2_auth_url(&oauth2_config)?;
ctx.log().info(format!("OAuth2 authorization URL: {}", auth_url));

// Handle OAuth2 callback
let auth_code = "received_authorization_code";
let oauth2_result = ctx.security().exchange_oauth2_code(auth_code, &oauth2_config).await?;

match oauth2_result {
    DMSCOAuth2Result::Success { access_token, refresh_token, user_info } => {
        ctx.log().info(format!("OAuth2 successful for user: {}", user_info.email));

        // Create or update user
        let user = ctx.security().create_or_update_user_from_oauth2(&user_info).await?;

        // Generate application JWT
        let app_jwt = ctx.security().generate_jwt(&user, &jwt_config)?;
        ctx.log().info(format!("Generated app JWT: {}", app_jwt));
    }
    DMSCOAuth2Result::Error { error, description } => {
        ctx.log().error(format!("OAuth2 error: {} - {}", error, description));
        return Err(DMSCError::auth("OAuth2 authentication failed"));
    }
}
```

### Multi-Factor Authentication (MFA)

```rust
use dms::prelude::*;

// Enable MFA
let user_id = "12345";
let mfa_secret = ctx.security().generate_mfa_secret(user_id)?;

// Generate QR code for setting up authenticator app
let qr_code_data = ctx.security().generate_mfa_qr_code(
    &mfa_secret,
    "MyApp",
    "john@example.com"
)?;

// Verify MFA code
let mfa_code = "123456"; // 6-digit code entered by user
let is_valid = ctx.security().verify_mfa_code(user_id, mfa_code)?;

if is_valid {
    ctx.log().info("MFA verification successful");

    // Enable MFA
    ctx.security().enable_mfa(user_id, &mfa_secret).await?;
} else {
    ctx.log().warn("MFA verification failed");
    return Err(DMSCError::auth("Invalid MFA code"));
}

// Backup codes
let backup_codes = ctx.security().generate_mfa_backup_codes(user_id, 10)?;
ctx.log().info(format!("Generated {} backup codes", backup_codes.len()));
```

<div align="center">

## Authorization Management

</div>

### Role-Based Access Control (RBAC)

```rust
use dms::prelude::*;

// Define roles and permissions
let roles = vec![
    DMSCRole {
        name: "admin".to_string(),
        permissions: vec![
            "user:create".to_string(),
            "user:read".to_string(),
            "user:update".to_string(),
            "user:delete".to_string(),
            "system:manage".to_string(),
        ],
    },
    DMSCRole {
        name: "user".to_string(),
        permissions: vec![
            "user:read:self".to_string(),
            "user:update:self".to_string(),
        ],
    },
    DMSCRole {
        name: "guest".to_string(),
        permissions: vec![
            "user:read:public".to_string(),
        ],
    },
];

// Check role permissions
let user = DMSCUser {
    id: "12345".to_string(),
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    roles: vec!["admin".to_string()],
    permissions: vec![],
    metadata: HashMap::new(),
    created_at: Utc::now(),
    last_login: Some(Utc::now()),
};

if ctx.security().has_role(&user, "admin")? {
    ctx.log().info("User has admin role");

    // Check specific permission
    if ctx.security().has_permission(&user, "user:create")? {
        ctx.log().info("User can create other users");
    }
}

// Check resource permission
let resource = "user:profile:12345";
let action = "read";

if ctx.security().authorize(&user, resource, action).await? {
    ctx.log().info(format!("User authorized to {} {}", action, resource));
} else {
    ctx.log().warn(format!("User not authorized to {} {}", action, resource));
    return Err(DMSCError::auth("Insufficient permissions"));
}
```

### Attribute-Based Access Control (ABAC)

```rust
use dms::prelude::*;

// Define attribute policy
let abac_policy = DMSCAbacPolicy {
    name: "document_access".to_string(),
    rules: vec![
        DMSCAbacRule {
            name: "owner_can_edit".to_string(),
            condition: DMSCAbacCondition::And(vec![
                DMSCAbacCondition::Equals("resource.type", "document"),
                DMSCAbacCondition::Equals("user.id", "resource.owner_id"),
                DMSCAbacCondition::Equals("action", "edit"),
            ]),
            effect: DMSCAbacEffect::Allow,
        },
        DMSCAbacRule {
            name: "team_member_can_read".to_string(),
            condition: DMSCAbacCondition::And(vec![
                DMSCAbacCondition::Equals("resource.type", "document"),
                DMSCAbacCondition::In("user.team_id", "resource.team_ids"),
                DMSCAbacCondition::Equals("action", "read"),
            ]),
            effect: DMSCAbacEffect::Allow,
        },
    ],
};

// Evaluate ABAC policy
let user_attributes = HashMap::from([
    ("id".to_string(), "12345".to_string()),
    ("team_id".to_string(), "team_123".to_string()),
    ("department".to_string(), "engineering".to_string()),
]);

let resource_attributes = HashMap::from([
    ("type".to_string(), "document".to_string()),
    ("owner_id".to_string(), "12345".to_string()),
    ("team_ids".to_string(), "team_123,team_456".to_string()),
    ("classification".to_string(), "internal".to_string()),
]);

let action = "edit";

let decision = ctx.security().evaluate_abac_policy(
    &abac_policy,
    &user_attributes,
    &resource_attributes,
    action
)?;

match decision {
    DMSCAbacDecision::Allow => {
        ctx.log().info("ABAC policy allows access");
    }
    DMSCAbacDecision::Deny => {
        ctx.log().warn("ABAC policy denies access");
        return Err(DMSCError::auth("Access denied by ABAC policy"));
    }
}
```

<div align="center">

## Encryption and Decryption

</div>

### Symmetric Encryption

```rust
use dms::prelude::*;

// AES encryption
let plaintext = b"sensitive data that needs encryption";
let key = ctx.security().generate_symmetric_key(32)?; // 256-bit key
let iv = ctx.security().generate_iv(16)?; // 128-bit IV

let encrypted_data = ctx.security().encrypt_aes256_cbc(plaintext, &key, &iv)?;
ctx.log().info(format!("Encrypted {} bytes", encrypted_data.len()));

// AES decryption
let decrypted_data = ctx.security().decrypt_aes256_cbc(&encrypted_data, &key, &iv)?;
let decrypted_text = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted text: {}", decrypted_text));
```

### Asymmetric Encryption

```rust
use dms::prelude::*;

// Generate RSA key pair
let (private_key, public_key) = ctx.security().generate_rsa_keypair(2048)?;

// Encrypt with public key
let plaintext = b"data to encrypt with public key";
let encrypted_data = ctx.security().encrypt_rsa_public(plaintext, &public_key)?;

// Decrypt with private key
let decrypted_data = ctx.security().decrypt_rsa_private(&encrypted_data, &private_key)?;
let decrypted_text = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted with private key: {}", decrypted_text));

// Digital signature
let data_to_sign = b"important message";
let signature = ctx.security().sign_rsa(data_to_sign, &private_key)?;

// Verify signature
let is_valid = ctx.security().verify_rsa_signature(data_to_sign, &signature, &public_key)?;
if is_valid {
    ctx.log().info("Signature is valid");
} else {
    ctx.log().error("Signature verification failed");
}
```

### Key Derivation

```rust
use dms::prelude::*;

// PBKDF2 key derivation
let password = "user_password";
let salt = ctx.security().generate_salt(32)?;
let derived_key = ctx.security().derive_key_pbkdf2(password, &salt, 10000, 32)?;

ctx.log().info(format!("Derived {}-byte key", derived_key.len()));

// Store salt and derived key for verification
let password_hash = DMSCPasswordHash {
    algorithm: "pbkdf2_sha256".to_string(),
    salt: base64::encode(&salt),
    hash: base64::encode(&derived_key),
    iterations: 10000,
};

// Verify password
let is_valid = ctx.security().verify_password_pbkdf2(
    password,
    &password_hash.salt,
    &password_hash.hash,
    password_hash.iterations
)?;
```

<div align="center">

## Input Validation

</div>

### Validation Rules

```rust
use dms::prelude::*;

// Define validation rules
let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
    DMSCValidationRule::MaxLength(100),
    DMSCValidationRule::Custom(|value| {
        // Custom validation: check if it's a company email
        value.ends_with("@company.com")
    }),
];

let password_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::MinLength(8),
    DMSCValidationRule::MaxLength(128),
    DMSCValidationRule::ContainsUppercase,
    DMSCValidationRule::ContainsLowercase,
    DMSCValidationRule::ContainsDigit,
    DMSCValidationRule::ContainsSpecial,
];

let username_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Alphanumeric,
    DMSCValidationRule::MinLength(3),
    DMSCValidationRule::MaxLength(20),
    DMSCValidationRule::Pattern(r"^[a-zA-Z][a-zA-Z0-9_]*$".to_string()),
];

// Execute validation
let email = "john@company.com";
let password = "SecurePass123!";
let username = "john_doe";

ctx.security().validate_input(email, &email_rules)?;
ctx.security().validate_input(password, &password_rules)?;
ctx.security().validate_input(username, &username_rules)?;

ctx.log().info("All inputs validated successfully");
```

### Data Sanitization

```rust
use dms::prelude::*;

// HTML sanitization
let dirty_html = r#"
    <script>alert('XSS')</script>
    <p>Safe content</p>
    <img src="javascript:alert('XSS')" />
    <a href="javascript:alert('XSS')">Click me</a>
"#;

let clean_html = ctx.security().sanitize_html(dirty_html)?;
ctx.log().info(format!("Cleaned HTML: {}", clean_html));

// SQL injection protection
let user_input = "admin' OR '1'='1";
let safe_input = ctx.security().escape_sql(user_input);
ctx.log().info(format!("Escaped SQL: {}", safe_input));

// Path traversal protection
let file_path = "../../../etc/passwd";
let safe_path = ctx.security().sanitize_path(file_path)?;
ctx.log().info(format!("Sanitized path: {}", safe_path));
```

<div align="center">

## Rate Limiting

</div>

### Token Bucket Algorithm

```rust
use dms::prelude::*;

// Configure rate limiting
let rate_limit_config = DMSCRateLimitConfig {
    algorithm: DMSCRateLimitAlgorithm::TokenBucket,
    capacity: 100,        // Bucket capacity
    refill_rate: 10,      // Tokens refilled per second
    refill_period: Duration::from_secs(1),
};

// Apply rate limiting
let client_ip = "192.168.1.100";
let rate_limit_key = format!("rate_limit:{}", client_ip);

match ctx.security().check_rate_limit(&rate_limit_key, &rate_limit_config)? {
    DMSCRateLimitResult::Allowed => {
        ctx.log().info("Request allowed");
        // Process request
    }
    DMSCRateLimitResult::Denied { retry_after } => {
        ctx.log().warn(format!("Rate limit exceeded, retry after {:?}", retry_after));
        return Err(DMSCError::rate_limit("Too many requests"));
    }
}

// Sliding window rate limiting
let sliding_window_config = DMSCRateLimitConfig {
    algorithm: DMSCRateLimitAlgorithm::SlidingWindow,
    window_size: Duration::from_minutes(1),
    max_requests: 60,  // Maximum 60 requests per minute
};

let api_key = "user_api_key_123";
let api_rate_limit_key = format!("api_rate_limit:{}", api_key);

match ctx.security().check_rate_limit(&api_rate_limit_key, &sliding_window_config)? {
    DMSCRateLimitResult::Allowed => {
        ctx.log().info("API request allowed");
    }
    DMSCRateLimitResult::Denied { retry_after } => {
        ctx.log().warn("API rate limit exceeded");
        return Err(DMSCError::rate_limit("API rate limit exceeded"));
    }
}
```

## CORS Configuration

```rust
use dms::prelude::*;

// Configure CORS
let cors_config = DMSCCorsConfig {
    allowed_origins: vec![
        "https://app.example.com".to_string(),
        "https://admin.example.com".to_string(),
    ],
    allowed_methods: vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
    ],
    allowed_headers: vec![
        "Content-Type".to_string(),
        "Authorization".to_string(),
        "X-Requested-With".to_string(),
    ],
    exposed_headers: vec![
        "X-Total-Count".to_string(),
        "X-Page-Size".to_string(),
    ],
    max_age: Duration::from_hours(24),
    allow_credentials: true,
};

// Check CORS request
let origin = "https://app.example.com";
let method = "POST";
let headers = vec!["Content-Type", "Authorization"];

match ctx.security().check_cors_request(&cors_config, origin, method, &headers)? {
    DMSCCorsResult::Allowed { headers } => {
        ctx.log().info("CORS request allowed");
        // Set response headers
        for (key, value) in headers {
            response.set_header(key, value);
        }
    }
    DMSCCorsResult::Denied => {
        ctx.log().warn("CORS request denied");
        return Err(DMSCError::security("CORS not allowed"));
    }
}
```

## CSRF Protection

```rust
use dms::prelude::*;

// Generate CSRF token
let csrf_token = ctx.security().generate_csrf_token()?;

// Include CSRF token in form
let form_html = format!(r#"
    <form method="POST" action="/update-profile">
        <input type="hidden" name="csrf_token" value="{}" />
        <!-- Other form fields -->
        <button type="submit">Update Profile</button>
    </form>
"#, csrf_token);

// Verify CSRF token
let submitted_token = "submitted_csrf_token";
let session_token = "session_csrf_token";

if !ctx.security().verify_csrf_token(submitted_token, session_token)? {
    ctx.log().warn("CSRF token verification failed");
    return Err(DMSCError::security("Invalid CSRF token"));
}

ctx.log().info("CSRF token verified successfully");
```

<div align="center">

## Security Configuration

</div>

### DMSCSecurityConfig

Security configuration structure.

#### Fields

| Field | Type | Description | Default Value |
|:--------|:-----|:-------------|:-------|
| `jwt_secret` | `String` | JWT secret key | Required |
| `password_hash_algorithm` | `DMSCPasswordHashAlgorithm` | Password hash algorithm | `Argon2id` |
| `session_timeout` | `Duration` | Session timeout | `24h` |
| `max_login_attempts` | `u32` | Maximum login attempts | `5` |
| `lockout_duration` | `Duration` | Account lockout duration | `30m` |
| `mfa_required` | `bool` | Whether MFA is required | `false` |
| `password_policy` | `DMSCPasswordPolicy` | Password policy | Default policy |
| `rate_limit_config` | `DMSCRateLimitConfig` | Rate limit configuration | Default configuration |

#### Configuration Example

```rust
use dms::prelude::*;

let security_config = DMSCSecurityConfig {
    jwt_secret: "my_super_secret_jwt_key_256_bits_long".to_string(),
    password_hash_algorithm: DMSCPasswordHashAlgorithm::Argon2id,
    session_timeout: Duration::from_hours(24),
    max_login_attempts: 5,
    lockout_duration: Duration::from_minutes(30),
    mfa_required: true,
    password_policy: DMSCPasswordPolicy {
        min_length: 8,
        max_length: 128,
        require_uppercase: true,
        require_lowercase: true,
        require_digit: true,
        require_special: true,
        forbid_common_passwords: true,
        max_age: Duration::from_days(90),
    },
    rate_limit_config: DMSCRateLimitConfig {
        login_attempts: DMSCRateLimit {
            window: Duration::from_minutes(15),
            max_requests: 10,
        },
        api_requests: DMSCRateLimit {
            window: Duration::from_minutes(1),
            max_requests: 100,
        },
    },
};
```

<div align="center">

## Error Handling

</div>

### Security Error Codes

| Error Code | Description |
|:--------|:-------------|
| `SECURITY_AUTH_FAILED` | Authentication failed |
| `SECURITY_UNAUTHORIZED` | Unauthorized |
| `SECURITY_FORBIDDEN` | Insufficient permissions |
| `SECURITY_INVALID_TOKEN` | Invalid token |
| `SECURITY_TOKEN_EXPIRED` | Token expired |
| `SECURITY_RATE_LIMITED` | Rate limited |
| `SECURITY_VALIDATION_FAILED` | Validation failed |
| `SECURITY_ENCRYPTION_ERROR` | Encryption error |

### Error Handling Example

```rust
use dms::prelude::*;

match ctx.security().authenticate(credentials).await {
    Ok(user) => {
        ctx.log().info(format!("User authenticated: {}", user.username));
    }
    Err(DMSCError { code, .. }) if code == "SECURITY_AUTH_FAILED" => {
        ctx.log().warn("Authentication failed");

        // Increment failure count
        let fail_count = ctx.security().increment_login_failures(&username).await?;

        if fail_count >= ctx.config().security.max_login_attempts {
            // Lock account
            ctx.security().lock_account(&username, Duration::from_minutes(30)).await?;
            ctx.log().warn(format!("Account locked: {}", username));
        }

        return Err(DMSCError::auth("Authentication failed"));
    }
    Err(e) => {
        ctx.log().error(format!("Authentication error: {}", e));
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Use strong password policies**: Require complex passwords and regular password changes
2. **Enable multi-factor authentication**: Require MFA for sensitive operations
3. **Implement principle of least privilege**: Grant only necessary permissions
4. **Encrypt sensitive data**: Encrypt sensitive data in storage and transmission
5. **Input validation**: Validate and sanitize all user inputs
6. **Rate limiting**: Implement appropriate rate limits for API endpoints
7. **Security auditing**: Log all security-related events
8. **Regular security scans**: Periodically check for security vulnerabilities
9. **Use HTTPS**: Use encrypted connections for all communications
10. **Timely updates**: Keep security libraries and dependencies up to date

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, providing JWT, OAuth2, and RBAC authentication and authorization functionality
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [config](./config.md): Configuration module, managing authentication configuration and key settings
- [cache](./cache.md): Cache module, providing multi-backend cache abstraction, caching user sessions and permission data
- [database](./database.md): Database module, providing user data persistence and query functionality
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [observability](./observability.md): Observability module, monitoring authentication performance and security events
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
