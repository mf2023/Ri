# Security Usage Examples

The security module provides usage examples for authentication management, authorization management, encryption and decryption, input validation, rate limiting, CORS configuration, and CSRF protection.

## Authentication Management

### JWT Authentication

```rust
use dms::prelude::*;
use serde_json::json;

// JWT configuration
let jwt_config = DMSCJwtConfig {
    secret_key: "your-secret-key-here".to_string(),
    algorithm: DMSCJwtAlgorithm::HS256,
    issuer: "dms-service".to_string(),
    audience: vec!["web-app".to_string(), "mobile-app".to_string()],
    expiration: Duration::from_hours(24),
    refresh_expiration: Duration::from_days(30),
    not_before: Duration::from_minutes(0),
    subject: "user-auth".to_string(),
    claims: {
        let mut claims = std::collections::HashMap::new();
        claims.insert("service".to_string(), "dms".to_string());
        claims.insert("version".to_string(), "1.0".to_string());
        claims
    },
};

// Initialize JWT authentication
ctx.security().init_jwt(jwt_config).await?;

// Generate JWT token
let user = DMSCUser {
    id: "user123".to_string(),
    username: "john_doe".to_string(),
    email: "john@example.com".to_string(),
    roles: vec!["user".to_string(), "admin".to_string()],
    permissions: vec!["read".to_string(), "write".to_string(), "delete".to_string()],
    metadata: {
        let mut meta = std::collections::HashMap::new();
        meta.insert("department".to_string(), "engineering".to_string());
        meta.insert("team".to_string(), "backend".to_string());
        meta.insert("join_date".to_string(), "2023-01-15".to_string());
        meta
    },
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    last_login: Some(chrono::Utc::now()),
    is_active: true,
    is_verified: true,
};

let token = ctx.security().generate_jwt(&user).await?;
ctx.log().info(format!("JWT token generated: {}...", &token[..20]));

// Verify JWT token
match ctx.security().verify_jwt(&token).await {
    Ok(claims) => {
        ctx.log().info(format!("JWT verification successful for user: {}", claims.sub));
        
        // Extract user information from claims
        let user_id = claims.sub;
        let roles = claims.roles.unwrap_or_default();
        let permissions = claims.permissions.unwrap_or_default();
        
        ctx.log().info(format!("User {} has roles: {:?}, permissions: {:?}", 
            user_id, roles, permissions));
    }
    Err(e) => {
        ctx.log().error(format!("JWT verification failed: {}", e));
        return Err(e);
    }
}

// Refresh JWT token
let refresh_token = ctx.security().generate_refresh_token(&user).await?;
let new_token = ctx.security().refresh_jwt(&refresh_token).await?;
ctx.log().info("JWT token refreshed successfully");

// Revoke JWT token
ctx.security().revoke_jwt(&token).await?;
ctx.log().info("JWT token revoked");
```

### OAuth2 Authentication

```rust
use dms::prelude::*;
use serde_json::json;

// OAuth2 configuration
let oauth2_config = DMSCOAuth2Config {
    client_id: "your-client-id".to_string(),
    client_secret: "your-client-secret".to_string(),
    authorization_endpoint: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
    token_endpoint: "https://oauth2.googleapis.com/token".to_string(),
    user_info_endpoint: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
    redirect_uri: "https://your-app.com/auth/callback".to_string(),
    scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
    response_type: "code".to_string(),
    grant_type: "authorization_code".to_string(),
    additional_params: {
        let mut params = std::collections::HashMap::new();
        params.insert("access_type".to_string(), "offline".to_string());
        params.insert("prompt".to_string(), "consent".to_string());
        params
    },
};

// Initialize OAuth2
ctx.security().init_oauth2(oauth2_config).await?;

// Generate authorization URL
let auth_url = ctx.security().generate_authorization_url().await?;
ctx.log().info(format!("Authorization URL: {}", auth_url));

// Handle OAuth2 callback
let auth_code = "authorization_code_from_callback";
let token_response = ctx.security().exchange_code_for_token(auth_code).await?;

ctx.log().info(format!("Access token: {}...", &token_response.access_token[..20]));
ctx.log().info(format!("Token type: {}", token_response.token_type));
ctx.log().info(format!("Expires in: {} seconds", token_response.expires_in));

// Get user information
let user_info = ctx.security().get_user_info(&token_response.access_token).await?;
ctx.log().info(format!("User info: {:?}", user_info));

// Refresh access token
let new_token_response = ctx.security().refresh_access_token(&token_response.refresh_token.unwrap()).await?;
ctx.log().info("Access token refreshed");

// Revoke access token
ctx.security().revoke_access_token(&token_response.access_token).await?;
ctx.log().info("Access token revoked");
```

### Multi-Factor Authentication (MFA)

```rust
use dms::prelude::*;
use serde_json::json;

// Initialize MFA
ctx.security().init_mfa().await?;

// Generate MFA secret
let user_id = "user123";
let mfa_secret = ctx.security().generate_mfa_secret(user_id).await?;
ctx.log().info(format!("MFA secret generated for user: {}", user_id));

// Generate QR code (for Google Authenticator, etc.)
let qr_code = ctx.security().generate_mfa_qr_code(
    user_id,
    "DMSC Service",
    &mfa_secret,
).await?;
ctx.log().info(format!("MFA QR code generated: {}...", &qr_code[..50]));

// Verify MFA code
let user_code = "123456"; // 6-digit code entered by user
let backup_codes = vec!["backup-code-1".to_string(), "backup-code-2".to_string()];

match ctx.security().verify_mfa_code(user_id, user_code).await {
    Ok(true) => {
        ctx.log().info("MFA verification successful");
        
        // Generate authentication token
        let auth_token = ctx.security().generate_mfa_token(user_id, &backup_codes).await?;
        ctx.log().info("MFA authentication token generated");
    }
    Ok(false) => {
        ctx.log().warn("MFA verification failed");
        
        // Check if backup codes can be used
        match ctx.security().verify_mfa_backup_code(user_id, user_code).await? {
            true => {
                ctx.log().info("MFA backup code verification successful");
            }
            false => {
                ctx.log().error("All MFA verification methods failed");
                return Err(DMSCError::authentication("MFA verification failed".to_string()));
            }
        }
    }
    Err(e) => {
        ctx.log().error(format!("MFA verification error: {}", e));
        return Err(e);
    }
}

// Disable MFA
ctx.security().disable_mfa(user_id).await?;
ctx.log().info(format!("MFA disabled for user: {}", user_id));

// Re-enable MFA
ctx.security().enable_mfa(user_id).await?;
ctx.log().info(format!("MFA re-enabled for user: {}", user_id));
```

## Authorization Management

### RBAC Role-Based Access Control

```rust
use dms::prelude::*;
use serde_json::json;

// Create roles
let admin_role = DMSCRole {
    id: "role_admin".to_string(),
    name: "Administrator".to_string(),
    description: "Full system administrator".to_string(),
    permissions: vec![
        "users.create".to_string(),
        "users.read".to_string(),
        "users.update".to_string(),
        "users.delete".to_string(),
        "roles.manage".to_string(),
        "system.configure".to_string(),
        "logs.access".to_string(),
        "backup.manage".to_string(),
    ],
    metadata: {
        let mut meta = std::collections::HashMap::new();
        meta.insert("level".to_string(), "admin".to_string());
        meta.insert("scope".to_string(), "global".to_string());
        meta
    },
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    is_active: true,
};

let user_role = DMSCRole {
    id: "role_user".to_string(),
    name: "User".to_string(),
    description: "Regular user".to_string(),
    permissions: vec![
        "profile.read".to_string(),
        "profile.update".to_string(),
        "orders.create".to_string(),
        "orders.read".to_string(),
        "products.read".to_string(),
    ],
    metadata: {
        let mut meta = std::collections::HashMap::new();
        meta.insert("level".to_string(), "user".to_string());
        meta.insert("scope".to_string(), "personal".to_string());
        meta
    },
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    is_active: true,
};

let moderator_role = DMSCRole {
    id: "role_moderator".to_string(),
    name: "Moderator".to_string(),
    description: "Content moderator".to_string(),
    permissions: vec![
        "content.read".to_string(),
        "content.moderate".to_string(),
        "users.read".to_string(),
        "reports.access".to_string(),
    ],
    metadata: {
        let mut meta = std::collections::HashMap::new();
        meta.insert("level".to_string(), "moderator".to_string());
        meta.insert("scope".to_string(), "content".to_string());
        meta
    },
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    is_active: true,
};

// Create roles
ctx.security().create_role(admin_role).await?;
ctx.security().create_role(user_role).await?;
ctx.security().create_role(moderator_role).await?;

// Assign role to user
let user_id = "user123";
ctx.security().assign_role(user_id, "role_admin").await?;
ctx.log().info(format!("Admin role assigned to user: {}", user_id));

// Check user permission
let has_permission = ctx.security().check_permission(user_id, "users.create").await?;
ctx.log().info(format!("User {} has permission 'users.create': {}", user_id, has_permission));

// Get user roles
let user_roles = ctx.security().get_user_roles(user_id).await?;
ctx.log().info(format!("User {} has roles: {:?}", user_id, user_roles));

// Get user permissions
let user_permissions = ctx.security().get_user_permissions(user_id).await?;
ctx.log().info(format!("User {} has permissions: {:?}", user_id, user_permissions));

// Revoke role
ctx.security().revoke_role(user_id, "role_admin").await?;
ctx.log().info(format!("Admin role revoked from user: {}", user_id));

// Update role permissions
let updated_permissions = vec![
    "users.read".to_string(),
    "users.update".to_string(),
    "profile.read".to_string(),
    "profile.update".to_string(),
];

ctx.security().update_role_permissions("role_user", updated_permissions).await?;
ctx.log().info("User role permissions updated");
```

### ABAC Attribute-Based Access Control

```rust
use dms::prelude::*;
use serde_json::json;

// Define ABAC policy
let abac_policy = DMSCABACPolicy {
    id: "policy_document_access".to_string(),
    name: "Document Access Policy".to_string(),
    description: "Controls access to documents based on user attributes".to_string(),
    rules: vec![
        DMSCABACRule {
            id: "rule_owner_access".to_string(),
            name: "Owner Access".to_string(),
            description: "Document owners have full access".to_string(),
            subject_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("role".to_string(), vec!["owner".to_string()]);
                attrs
            },
            resource_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("type".to_string(), vec!["document".to_string()]);
                attrs
            },
            action: "full_access".to_string(),
            condition: "subject.id == resource.owner_id".to_string(),
            effect: DMSCABACEffect::Allow,
            priority: 1,
        },
        DMSCABACRule {
            id: "rule_department_access".to_string(),
            name: "Department Access".to_string(),
            description: "Users can access documents in their department".to_string(),
            subject_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("department".to_string(), vec!["engineering".to_string(), "sales".to_string()]);
                attrs
            },
            resource_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("department".to_string(), vec!["engineering".to_string(), "sales".to_string()]);
                attrs
            },
            action: "read".to_string(),
            condition: "subject.department == resource.department".to_string(),
            effect: DMSCABACEffect::Allow,
            priority: 2,
        },
        DMSCABACRule {
            id: "rule_time_based_access".to_string(),
            name: "Time-Based Access".to_string(),
            description: "Access restricted during business hours".to_string(),
            subject_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("role".to_string(), vec!["employee".to_string()]);
                attrs
            },
            resource_attributes: {
                let mut attrs = std::collections::HashMap::new();
                attrs.insert("sensitivity".to_string(), vec!["confidential".to_string()]);
                attrs
            },
            action: "read".to_string(),
            condition: "current_time >= '09:00' && current_time <= '17:00'".to_string(),
            effect: DMSCABACEffect::Allow,
            priority: 3,
        },
    ],
    created_at: chrono::Utc::now(),
    updated_at: chrono::Utc::now(),
    is_active: true,
};

// Create ABAC policy
ctx.security().create_abac_policy(abac_policy).await?;

// Evaluate ABAC access request
let access_request = DMSCABACRequest {
    subject: {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "user123".to_string());
        attrs.insert("role".to_string(), "employee".to_string());
        attrs.insert("department".to_string(), "engineering".to_string());
        attrs.insert("clearance_level".to_string(), "3".to_string());
        attrs
    },
    resource: {
        let mut attrs = std::collections::HashMap::new();
        attrs.insert("id".to_string(), "doc456".to_string());
        attrs.insert("type".to_string(), "document".to_string());
        attrs.insert("department".to_string(), "engineering".to_string());
        attrs.insert("owner_id".to_string(), "user123".to_string());
        attrs.insert("sensitivity".to_string(), "confidential".to_string());
        attrs
    },
    action: "read".to_string(),
    context: {
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("current_time".to_string(), "14:30".to_string());
        ctx.insert("location".to_string(), "office".to_string());
        ctx
    },
};

let access_decision = ctx.security().evaluate_abac_policy(&access_request).await?;
ctx.log().info(format!("ABAC access decision: {:?}", access_decision));

if access_decision.effect == DMSCABACEffect::Allow {
    ctx.log().info("Access granted");
} else {
    ctx.log().warn("Access denied");
}
```

## Encryption and Decryption

### Symmetric Encryption

```rust
use dms::prelude::*;
use serde_json::json;

// Configure symmetric encryption
let symmetric_config = DMSCSymmetricEncryptionConfig {
    algorithm: DMSCSymmetricAlgorithm::AES256GCM,
    key: "this-is-a-32-byte-secret-key!".to_string(),
    iv: Some("16-byte-iv-here".to_string()),
    mode: DMSCEncryptionMode::GCM,
    padding: DMSCPaddingScheme::PKCS7,
};

ctx.security().init_symmetric_encryption(symmetric_config).await?;

// Encrypt data
let sensitive_data = "This is sensitive information that needs to be encrypted";
let encrypted_data = ctx.security().encrypt_symmetric(sensitive_data).await?;
ctx.log().info(format!("Encrypted data: {}", encrypted_data));

// Decrypt data
let decrypted_data = ctx.security().decrypt_symmetric(&encrypted_data).await?;
ctx.log().info(format!("Decrypted data: {}", decrypted_data));

// Batch encryption
let data_batch = vec![
    "sensitive data 1".to_string(),
    "sensitive data 2".to_string(),
    "sensitive data 3".to_string(),
];

let encrypted_batch = ctx.security().encrypt_symmetric_batch(data_batch).await?;
ctx.log().info(format!("Batch encrypted {} items", encrypted_batch.len()));

// Batch decryption
let decrypted_batch = ctx.security().decrypt_symmetric_batch(encrypted_batch).await?;
ctx.log().info(format!("Batch decrypted {} items", decrypted_batch.len()));
```

### Asymmetric Encryption

```rust
use dms::prelude::*;
use serde_json::json;

// Generate key pair
let key_pair = ctx.security().generate_asymmetric_key_pair(DMSCAsymmetricAlgorithm::RSA2048).await?;
ctx.log().info("Asymmetric key pair generated");

// Save key pair
ctx.security().save_key_pair(
    &key_pair,
    "keys/service_key".to_string(),
    "secure-password".to_string(),
).await?;

// Load key pair
let loaded_key_pair = ctx.security().load_key_pair(
    "keys/service_key".to_string(),
    "secure-password".to_string(),
).await?;

// Encrypt with public key
let public_key = &loaded_key_pair.public_key;
let data_to_encrypt = "This data will be encrypted with public key";
let encrypted_with_public = ctx.security().encrypt_with_public_key(data_to_encrypt, public_key).await?;

// Decrypt with private key
let private_key = &loaded_key_pair.private_key;
let decrypted_with_private = ctx.security().decrypt_with_private_key(&encrypted_with_public, private_key).await?;
ctx.log().info(format!("Decrypted data: {}", decrypted_with_private));

// Digital signature
let data_to_sign = "This data will be digitally signed";
let signature = ctx.security().sign_data(data_to_sign, private_key).await?;
ctx.log().info(format!("Digital signature: {}", signature));

// Verify signature
let is_valid = ctx.security().verify_signature(data_to_sign, &signature, public_key).await?;
ctx.log().info(format!("Signature verification: {}", is_valid));
```

### Key Derivation

```rust
use dms::prelude::*;
use serde_json::json;

// Derive key from password
let password = "user-strong-password";
let salt = "random-salt-here";
let key_derivation_config = DMSCKeyDerivationConfig {
    algorithm: DMSCKeyDerivationAlgorithm::PBKDF2,
    iterations: 100000,
    key_length: 32,
    salt: salt.to_string(),
};

let derived_key = ctx.security().derive_key_from_password(password, &key_derivation_config).await?;
ctx.log().info(format!("Derived key: {}", derived_key));

// Encrypt with derived key
let encryption_config = DMSCSymmetricEncryptionConfig {
    algorithm: DMSCSymmetricAlgorithm::AES256GCM,
    key: derived_key.clone(),
    iv: None,
    mode: DMSCEncryptionMode::GCM,
    padding: DMSCPaddingScheme::PKCS7,
};

ctx.security().init_symmetric_encryption(encryption_config).await?;

let user_data = "User sensitive data encrypted with derived key";
let encrypted_user_data = ctx.security().encrypt_symmetric(user_data).await?;
ctx.log().info("User data encrypted with derived key");

// Verify password
let is_password_valid = ctx.security().verify_password_against_key(
    password,
    &derived_key,
    &key_derivation_config,
).await?;

ctx.log().info(format!("Password validation: {}", is_password_valid));
```

## Input Validation

### Data Validation

```rust
use dms::prelude::*;
use serde_json::json;

// Validate user input
let user_input = json!({
    "email": "user@example.com",
    "username": "john_doe",
    "password": "StrongP@ssw0rd!",
    "age": 25,
    "phone": "+1234567890",
    "website": "https://example.com",
});

// Define validation rules
let validation_rules = vec![
    DMSCValidationRule {
        field: "email".to_string(),
        rule_type: DMSCValidationType::Email,
        required: true,
        message: "Valid email address is required".to_string(),
    },
    DMSCValidationRule {
        field: "username".to_string(),
        rule_type: DMSCValidationType::Pattern("^[a-zA-Z0-9_]{3,20}$".to_string()),
        required: true,
        message: "Username must be 3-20 characters, alphanumeric and underscore only".to_string(),
    },
    DMSCValidationRule {
        field: "password".to_string(),
        rule_type: DMSCValidationType::Custom(Box::new(|value| {
            let password = value.as_str().unwrap_or_default();
            
            // Check password strength
            if password.len() < 8 {
                return Err("Password must be at least 8 characters long".to_string());
            }
            
            if !password.chars().any(|c| c.is_uppercase()) {
                return Err("Password must contain at least one uppercase letter".to_string());
            }
            
            if !password.chars().any(|c| c.is_lowercase()) {
                return Err("Password must contain at least one lowercase letter".to_string());
            }
            
            if !password.chars().any(|c| c.is_ascii_digit()) {
                return Err("Password must contain at least one digit".to_string());
            }
            
            if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
                return Err("Password must contain at least one special character".to_string());
            }
            
            Ok(())
        })),
        required: true,
        message: "Password must meet strength requirements".to_string(),
    },
    DMSCValidationRule {
        field: "age".to_string(),
        rule_type: DMSCValidationType::Range(18, 120),
        required: true,
        message: "Age must be between 18 and 120".to_string(),
    },
    DMSCValidationRule {
        field: "phone".to_string(),
        rule_type: DMSCValidationType::Pattern("^\\+?[1-9]\\d{1,14}$".to_string()),
        required: false,
        message: "Invalid phone number format".to_string(),
    },
    DMSCValidationRule {
        field: "website".to_string(),
        rule_type: DMSCValidationType::URL,
        required: false,
        message: "Invalid URL format".to_string(),
    },
];

// Perform validation
let validation_result = ctx.security().validate_input(&user_input, &validation_rules).await?;

if validation_result.is_valid {
    ctx.log().info("All validation passed");
} else {
    ctx.log().error("Validation failed:");
    for error in validation_result.errors {
        ctx.log().error(format!("  - {}: {}", error.field, error.message));
    }
    return Err(DMSCError::validation("Input validation failed".to_string()));
}
```

### Input Sanitization

```rust
use dms::prelude::*;
use serde_json::json;

// Sanitize user input
let unsafe_input = "<script>alert('XSS')</script> & <img src=x onerror=alert(1)>";

// Remove HTML tags
let sanitized_html = ctx.security().sanitize_html(unsafe_input).await?;
ctx.log().info(format!("HTML sanitized: {}", sanitized_html));

// Remove SQL injection patterns
let sql_input = "1' OR '1'='1' --";
let sanitized_sql = ctx.security().sanitize_sql(sql_input).await?;
ctx.log().info(format!("SQL sanitized: {}", sanitized_sql));

// Remove path traversal patterns
let path_input = "../../../etc/passwd";
let sanitized_path = ctx.security().sanitize_path(path_input).await?;
ctx.log().info(format!("Path sanitized: {}", sanitized_path));

// Remove XSS patterns
let xss_input = "<script>alert('XSS')</script><img src=x onerror=alert(1)>";
let sanitized_xss = ctx.security().sanitize_xss(xss_input).await?;
ctx.log().info(format!("XSS sanitized: {}", sanitized_xss));

// Batch sanitization
let unsafe_batch = vec![
    "<script>alert(1)</script>",
    "1' OR '1'='1'",
    "../../../etc/passwd",
    "<img src=x onerror=alert(1)>",
];

let sanitized_batch = ctx.security().sanitize_batch(unsafe_batch).await?;
ctx.log().info(format!("Batch sanitized {} items", sanitized_batch.len()));
```

## Rate Limiting

### Token Bucket Algorithm

```rust
use dms::prelude::*;

// Configure token bucket rate limiter
let rate_limiter_config = DMSCRateLimiterConfig {
    algorithm: DMSCRateLimitAlgorithm::TokenBucket,
    capacity: 100,
    refill_rate: 10,
    window_size: Duration::from_seconds(60),
    burst_size: 20,
};

ctx.security().init_rate_limiter(rate_limiter_config).await?;

// Check rate limit
let client_id = "client123";
let request_count = 5;

match ctx.security().check_rate_limit(client_id, request_count).await {
    Ok(allowed) => {
        if allowed {
            ctx.log().info("Request allowed");
        } else {
            ctx.log().warn("Rate limit exceeded");
            return Err(DMSCError::rate_limit("Too many requests".to_string()));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Rate limit check error: {}", e));
        return Err(e);
    }
}

// Get rate limit status
let rate_limit_status = ctx.security().get_rate_limit_status(client_id).await?;
ctx.log().info(format!("Rate limit status: {:?}", rate_limit_status));
```

### Sliding Window Algorithm

```rust
use dms::prelude::*;

// Configure sliding window rate limiter
let rate_limiter_config = DMSCRateLimiterConfig {
    algorithm: DMSCRateLimitAlgorithm::SlidingWindow,
    capacity: 1000,
    refill_rate: 100,
    window_size: Duration::from_seconds(60),
    burst_size: 50,
};

ctx.security().init_rate_limiter(rate_limiter_config).await?;

// Check rate limit for multiple requests
let client_id = "client456";
let requests = vec!["req1", "req2", "req3", "req4", "req5"];

for request in requests {
    match ctx.security().check_rate_limit(client_id, 1).await {
        Ok(allowed) => {
            if allowed {
                ctx.log().info(format!("Request {} allowed", request));
            } else {
                ctx.log().warn(format!("Request {} blocked - rate limit exceeded", request));
            }
        }
        Err(e) => {
            ctx.log().error(format!("Rate limit check error: {}", e));
        }
    }
}
```

### IP-Based Rate Limiting

```rust
use dms::prelude::*;

// Configure IP-based rate limiter
let ip_rate_limiter_config = DMSCRateLimiterConfig {
    algorithm: DMSCRateLimitAlgorithm::TokenBucket,
    capacity: 50,
    refill_rate: 5,
    window_size: Duration::from_seconds(60),
    burst_size: 10,
};

ctx.security().init_ip_rate_limiter(ip_rate_limiter_config).await?;

// Check rate limit by IP
let client_ip = "192.168.1.100";

match ctx.security().check_ip_rate_limit(client_ip, 1).await {
    Ok(allowed) => {
        if allowed {
            ctx.log().info(format!("Request from {} allowed", client_ip));
        } else {
            ctx.log().warn(format!("Request from {} blocked - rate limit exceeded", client_ip));
            return Err(DMSCError::rate_limit("IP rate limit exceeded".to_string()));
        }
    }
    Err(e) => {
        ctx.log().error(format!("IP rate limit check error: {}", e));
        return Err(e);
    }
}
```

## CORS Configuration

### Basic CORS Configuration

```rust
use dms::prelude::*;

// Configure CORS
let cors_config = DMSCCorsConfig {
    allowed_origins: vec![
        "https://example.com".to_string(),
        "https://www.example.com".to_string(),
    ],
    allowed_methods: vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
        "OPTIONS".to_string(),
    ],
    allowed_headers: vec![
        "Content-Type".to_string(),
        "Authorization".to_string(),
        "X-Requested-With".to_string(),
    ],
    exposed_headers: vec![
        "X-Custom-Header".to_string(),
        "X-Total-Count".to_string(),
    ],
    allow_credentials: true,
    max_age: Duration::from_hours(1),
    preflight_continue: false,
    options_success_status: 204,
};

ctx.security().configure_cors(cors_config).await?;
ctx.log().info("CORS configuration completed");
```

### Dynamic CORS Configuration

```rust
use dms::prelude::*;

// Configure dynamic CORS based on request
let cors_config = DMSCCorsConfig {
    allowed_origins: vec!["*".to_string()],
    allowed_methods: vec!["GET".to_string(), "POST".to_string()],
    allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
    exposed_headers: vec![],
    allow_credentials: false,
    max_age: Duration::from_hours(1),
    preflight_continue: false,
    options_success_status: 204,
};

ctx.security().configure_cors(cors_config).await?;

// Handle CORS preflight request
let origin = "https://example.com";
let method = "POST";
let headers = vec!["Content-Type".to_string(), "Authorization".to_string()];

let cors_headers = ctx.security().get_cors_headers(origin, method, &headers).await?;
ctx.log().info(format!("CORS headers: {:?}", cors_headers));
```

## CSRF Protection

### CSRF Token Generation

```rust
use dms::prelude::*;

// Initialize CSRF protection
ctx.security().init_csrf_protection().await?;

// Generate CSRF token for user session
let user_id = "user123";
let csrf_token = ctx.security().generate_csrf_token(user_id).await?;
ctx.log().info(format!("CSRF token generated for user: {}", user_id));

// Validate CSRF token
let submitted_token = "submitted_token_from_form";

match ctx.security().validate_csrf_token(user_id, &submitted_token).await {
    Ok(true) => {
        ctx.log().info("CSRF token validation successful");
    }
    Ok(false) => {
        ctx.log().error("CSRF token validation failed");
        return Err(DMSCError::security("Invalid CSRF token".to_string()));
    }
    Err(e) => {
        ctx.log().error(format!("CSRF token validation error: {}", e));
        return Err(e);
    }
}
```

### CSRF Token Rotation

```rust
use dms::prelude::*;

// Rotate CSRF token
let user_id = "user123";
let old_token = "old_csrf_token";

let new_token = ctx.security().rotate_csrf_token(user_id, old_token).await?;
ctx.log().info(format!("CSRF token rotated for user: {}", user_id));

// Invalidate CSRF token
ctx.security().invalidate_csrf_token(user_id).await?;
ctx.log().info(format!("CSRF token invalidated for user: {}", user_id));
```

## Security Best Practices

### Secure Password Storage

```rust
use dms::prelude::*;

// Hash password securely
let password = "user_password";
let hashed_password = ctx.security().hash_password(password).await?;
ctx.log().info("Password hashed successfully");

// Verify password
let input_password = "user_password";
let is_valid = ctx.security().verify_password(input_password, &hashed_password).await?;

if is_valid {
    ctx.log().info("Password verification successful");
} else {
    ctx.log().error("Password verification failed");
}
```

### Secure Session Management

```rust
use dms::prelude::*;

// Create secure session
let user_id = "user123";
let session_data = json!({
    "user_id": user_id,
    "login_time": chrono::Utc::now(),
    "ip_address": "192.168.1.100",
});

let session_id = ctx.security().create_session(user_id, &session_data).await?;
ctx.log().info(format!("Session created: {}", session_id));

// Validate session
match ctx.security().validate_session(&session_id).await {
    Ok(is_valid) => {
        if is_valid {
            ctx.log().info("Session is valid");
        } else {
            ctx.log().warn("Session is invalid or expired");
        }
    }
    Err(e) => {
        ctx.log().error(format!("Session validation error: {}", e));
    }
}

// Destroy session
ctx.security().destroy_session(&session_id).await?;
ctx.log().info("Session destroyed");
```

## See Also

- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [validation](./validation.md): Validation examples, data validation and sanitization operations
