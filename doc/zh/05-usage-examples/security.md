# 安全使用示例

security模块提供认证管理、授权管理、加密解密、输入验证、速率限制、CORS配置和CSRF保护功能的使用示例。

## 认证管理

### JWT认证

```rust
use dmsc::prelude::*;
use serde_json::json;

// JWT配置
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

// 初始化JWT认证
ctx.security().init_jwt(jwt_config).await?;

// 生成JWT令牌
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

// 验证JWT令牌
match ctx.security().verify_jwt(&token).await {
    Ok(claims) => {
        ctx.log().info(format!("JWT verification successful for user: {}", claims.sub));
        
        // 从声明中提取用户信息
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

// 刷新JWT令牌
let refresh_token = ctx.security().generate_refresh_token(&user).await?;
let new_token = ctx.security().refresh_jwt(&refresh_token).await?;
ctx.log().info("JWT token refreshed successfully");

// 撤销JWT令牌
ctx.security().revoke_jwt(&token).await?;
ctx.log().info("JWT token revoked");
```

### OAuth2认证

```rust
use dmsc::prelude::*;
use serde_json::json;

// OAuth2配置
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

// 初始化OAuth2
ctx.security().init_oauth2(oauth2_config).await?;

// 生成授权URL
let auth_url = ctx.security().generate_authorization_url().await?;
ctx.log().info(format!("Authorization URL: {}", auth_url));

// 处理OAuth2回调
let auth_code = "authorization_code_from_callback";
let token_response = ctx.security().exchange_code_for_token(auth_code).await?;

ctx.log().info(format!("Access token: {}...", &token_response.access_token[..20]));
ctx.log().info(format!("Token type: {}", token_response.token_type));
ctx.log().info(format!("Expires in: {} seconds", token_response.expires_in));

// 获取用户信息
let user_info = ctx.security().get_user_info(&token_response.access_token).await?;
ctx.log().info(format!("User info: {:?}", user_info));

// 刷新访问令牌
let new_token_response = ctx.security().refresh_access_token(&token_response.refresh_token.unwrap()).await?;
ctx.log().info("Access token refreshed");

// 撤销访问令牌
ctx.security().revoke_access_token(&token_response.access_token).await?;
ctx.log().info("Access token revoked");
```

### 多因素认证(MFA)

```rust
use dmsc::prelude::*;
use serde_json::json;

// 初始化MFA
ctx.security().init_mfa().await?;

// 生成MFA密钥
let user_id = "user123";
let mfa_secret = ctx.security().generate_mfa_secret(user_id).await?;
ctx.log().info(format!("MFA secret generated for user: {}", user_id));

// 生成QR码（用于Google Authenticator等）
let qr_code = ctx.security().generate_mfa_qr_code(
    user_id,
    "DMSC Service",
    &mfa_secret,
).await?;
ctx.log().info(format!("MFA QR code generated: {}...", &qr_code[..50]));

// 验证MFA代码
let user_code = "123456"; // 用户输入的6位代码
let backup_codes = vec!["backup-code-1".to_string(), "backup-code-2".to_string()];

match ctx.security().verify_mfa_code(user_id, user_code).await {
    Ok(true) => {
        ctx.log().info("MFA verification successful");
        
        // 生成认证令牌
        let auth_token = ctx.security().generate_mfa_token(user_id, &backup_codes).await?;
        ctx.log().info("MFA authentication token generated");
    }
    Ok(false) => {
        ctx.log().warn("MFA verification failed");
        
        // 检查是否可以使用备份代码
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

// 禁用MFA
ctx.security().disable_mfa(user_id).await?;
ctx.log().info(format!("MFA disabled for user: {}", user_id));

// 重新启用MFA
ctx.security().enable_mfa(user_id).await?;
ctx.log().info(format!("MFA re-enabled for user: {}", user_id));
```

## 授权管理

### RBAC基于角色的访问控制

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建角色
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

// 创建角色
ctx.security().create_role(admin_role).await?;
ctx.security().create_role(user_role).await?;
ctx.security().create_role(moderator_role).await?;

// 分配角色给用户
let user_id = "user123";
ctx.security().assign_role(user_id, "role_admin").await?;
ctx.log().info(format!("Admin role assigned to user: {}", user_id));

// 检查用户权限
let has_permission = ctx.security().check_permission(user_id, "users.create").await?;
ctx.log().info(format!("User {} has permission 'users.create': {}", user_id, has_permission));

// 获取用户角色
let user_roles = ctx.security().get_user_roles(user_id).await?;
ctx.log().info(format!("User {} has roles: {:?}", user_id, user_roles));

// 获取用户权限
let user_permissions = ctx.security().get_user_permissions(user_id).await?;
ctx.log().info(format!("User {} has permissions: {:?}", user_id, user_permissions));

// 撤销角色
ctx.security().revoke_role(user_id, "role_admin").await?;
ctx.log().info(format!("Admin role revoked from user: {}", user_id));

// 更新角色权限
let updated_permissions = vec![
    "users.read".to_string(),
    "users.update".to_string(),
    "profile.read".to_string(),
    "profile.update".to_string(),
];

ctx.security().update_role_permissions("role_user", updated_permissions).await?;
ctx.log().info("User role permissions updated");
```

### ABAC基于属性的访问控制

```rust
use dmsc::prelude::*;
use serde_json::json;

// 定义ABAC策略
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

// 创建ABAC策略
ctx.security().create_abac_policy(abac_policy).await?;

// 评估ABAC访问请求
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

## 加密解密

### 对称加密

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置对称加密
let symmetric_config = DMSCSymmetricEncryptionConfig {
    algorithm: DMSCSymmetricAlgorithm::AES256GCM,
    key: "this-is-a-32-byte-secret-key!".to_string(),
    iv: Some("16-byte-iv-here".to_string()),
    mode: DMSCEncryptionMode::GCM,
    padding: DMSCPaddingScheme::PKCS7,
};

ctx.security().init_symmetric_encryption(symmetric_config).await?;

// 加密数据
let sensitive_data = "This is sensitive information that needs to be encrypted";
let encrypted_data = ctx.security().encrypt_symmetric(sensitive_data).await?;
ctx.log().info(format!("Encrypted data: {}", encrypted_data));

// 解密数据
let decrypted_data = ctx.security().decrypt_symmetric(&encrypted_data).await?;
ctx.log().info(format!("Decrypted data: {}", decrypted_data));

// 批量加密
let data_batch = vec![
    "sensitive data 1".to_string(),
    "sensitive data 2".to_string(),
    "sensitive data 3".to_string(),
];

let encrypted_batch = ctx.security().encrypt_symmetric_batch(data_batch).await?;
ctx.log().info(format!("Batch encrypted {} items", encrypted_batch.len()));

// 批量解密
let decrypted_batch = ctx.security().decrypt_symmetric_batch(encrypted_batch).await?;
ctx.log().info(format!("Batch decrypted {} items", decrypted_batch.len()));
```

### 非对称加密

```rust
use dmsc::prelude::*;
use serde_json::json;

// 生成密钥对
let key_pair = ctx.security().generate_asymmetric_key_pair(DMSCAsymmetricAlgorithm::RSA2048).await?;
ctx.log().info("Asymmetric key pair generated");

// 保存密钥对
ctx.security().save_key_pair(
    &key_pair,
    "keys/service_key".to_string(),
    "secure-password".to_string(),
).await?;

// 加载密钥对
let loaded_key_pair = ctx.security().load_key_pair(
    "keys/service_key".to_string(),
    "secure-password".to_string(),
).await?;

// 使用公钥加密
let public_key = &loaded_key_pair.public_key;
let data_to_encrypt = "This data will be encrypted with public key";
let encrypted_with_public = ctx.security().encrypt_with_public_key(data_to_encrypt, public_key).await?;

// 使用私钥解密
let private_key = &loaded_key_pair.private_key;
let decrypted_with_private = ctx.security().decrypt_with_private_key(&encrypted_with_public, private_key).await?;
ctx.log().info(format!("Decrypted data: {}", decrypted_with_private));

// 数字签名
let data_to_sign = "This data will be digitally signed";
let signature = ctx.security().sign_data(data_to_sign, private_key).await?;
ctx.log().info(format!("Digital signature: {}", signature));

// 验证签名
let is_valid = ctx.security().verify_signature(data_to_sign, &signature, public_key).await?;
ctx.log().info(format!("Signature verification: {}", is_valid));
```

### 密钥派生

```rust
use dmsc::prelude::*;
use serde_json::json;

// 从密码派生密钥
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

// 使用派生密钥进行加密
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

// 验证密码
let is_password_valid = ctx.security().verify_password_against_key(
    password,
    &derived_key,
    &key_derivation_config,
).await?;

ctx.log().info(format!("Password validation: {}", is_password_valid));
```

## 输入验证

### 数据验证

```rust
use dmsc::prelude::*;
use serde_json::json;

// 验证用户输入
let user_input = json!({
    "email": "user@example.com",
    "username": "john_doe",
    "password": "StrongP@ssw0rd!",
    "age": 25,
    "phone": "+1234567890",
    "website": "https://example.com",
});

// 定义验证规则
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
            
            // 检查密码强度
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
];

// 执行验证
match ctx.security().validate_input(&user_input, &validation_rules).await {
    Ok(validated_data) => {
        ctx.log().info("Input validation successful");
        ctx.log().info(format!("Validated data: {:?}", validated_data));
    }
    Err(validation_errors) => {
        ctx.log().error("Input validation failed");
        for error in validation_errors {
            ctx.log().error(format!("Validation error: {}", error));
        }
        return Err(DMSCError::validation("Input validation failed".to_string()));
    }
}

// 清理输入数据
let sanitized_data = ctx.security().sanitize_input(&user_input).await?;
ctx.log().info(format!("Sanitized data: {:?}", sanitized_data));
```

### SQL注入防护

```rust
use dmsc::prelude::*;
use serde_json::json;

// 用户输入的搜索查询
let user_query = "john'; DROP TABLE users; --";

// 验证和清理SQL输入
let sanitized_query = ctx.security().sanitize_sql_input(user_query).await?;
ctx.log().info(format!("Sanitized SQL query: {}", sanitized_query));

// 使用参数化查询
let user_id = 123;
let search_term = "john";

// 安全的参数化查询
let users = ctx.database()
    .query(
        "SELECT id, username, email FROM users WHERE username LIKE $1 AND id != $2",
        vec![format!("%{search_term}%").into(), user_id.into()]
    )
    .await?;

ctx.log().info(format!("Found {} users matching search term", users.len()));

// 验证动态表名和列名
let table_name = "users";
let column_name = "username";

// 验证表名和列名是否安全
let is_safe_table = ctx.security().is_safe_identifier(table_name).await?;
let is_safe_column = ctx.security().is_safe_identifier(column_name).await?;

if is_safe_table && is_safe_column {
    let query = format!("SELECT {} FROM {} WHERE id = $1", column_name, table_name);
    let result = ctx.database()
        .query_one(&query, vec![user_id.into()])
        .await?;
    
    ctx.log().info("Dynamic query executed safely");
} else {
    return Err(DMSCError::validation("Unsafe table or column name".to_string()));
}
```

### XSS防护

```rust
use dmsc::prelude::*;
use serde_json::json;

// 用户输入的HTML内容
let user_html = r#"
    <script>alert('XSS')</script>
    <div onclick="maliciousFunction()">Click me</div>
    <img src="x" onerror="alert('XSS')">
    <a href="javascript:alert('XSS')">Link</a>
    <iframe src="javascript:alert('XSS')"></iframe>
"#;

// 清理HTML内容
let sanitized_html = ctx.security().sanitize_html(user_html).await?;
ctx.log().info(format!("Sanitized HTML: {}", sanitized_html));

// 验证和清理富文本内容
let rich_text_content = r#"
    <p>This is <strong>bold</strong> text with <em>italic</em> style.</p>
    <ul>
        <li>Item 1</li>
        <li>Item 2</li>
    </ul>
    <script>alert('This should be removed')</script>
"#;

let allowed_tags = vec!["p", "strong", "em", "ul", "li", "h1", "h2", "h3"];
let allowed_attributes = vec!["class", "id"];

let cleaned_rich_text = ctx.security().sanitize_html_with_policy(
    rich_text_content,
    &allowed_tags,
    &allowed_attributes,
).await?;

ctx.log().info(format!("Cleaned rich text: {}", cleaned_rich_text));

// 清理URL参数
let url_params = json!({
    "search": "<script>alert('XSS')</script>",
    "category": "electronics' OR '1'='1",
    "page": "1<script>",
});

let sanitized_params = ctx.security().sanitize_url_parameters(&url_params).await?;
ctx.log().info(format!("Sanitized URL parameters: {:?}", sanitized_params));
```

## 速率限制

### API速率限制

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置速率限制
let rate_limit_config = DMSCRateLimitConfig {
    enabled: true,
    window_duration: Duration::from_minutes(1),
    max_requests: 100,
    burst_size: 10,
    key_prefix: "api_rate_limit".to_string(),
    skip_successful_requests: false,
    skip_failed_requests: false,
    key_generator: Box::new(|request| {
        // 基于IP地址和用户ID生成速率限制键
        format!("{}:{}", request.ip_address, request.user_id.unwrap_or_default())
    }),
};

ctx.security().init_rate_limiting(rate_limit_config).await?;

// 检查速率限制
let client_info = DMSCClientInfo {
    ip_address: "192.168.1.100".to_string(),
    user_agent: "Mozilla/5.0".to_string(),
    user_id: Some("user123".to_string()),
    session_id: Some("session456".to_string()),
};

match ctx.security().check_rate_limit(&client_info, "api_endpoint").await {
    Ok(rate_limit_info) => {
        ctx.log().info(format!(
            "Rate limit check: {} requests remaining, resets at {}",
            rate_limit_info.remaining,
            rate_limit_info.reset_time
        ));
        
        if rate_limit_info.is_limited {
            ctx.log().warn("Rate limit exceeded");
            return Err(DMSCError::rate_limit_exceeded("Too many requests".to_string()));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Rate limit check failed: {}", e));
        return Err(e);
    }
}

// 基于端点的速率限制
let endpoint_limits = vec![
    ("/api/auth/login", 5, Duration::from_minutes(5)),
    ("/api/password/reset", 3, Duration::from_hours(1)),
    ("/api/upload", 10, Duration::from_minutes(10)),
    ("/api/export", 2, Duration::from_hours(1)),
];

for (endpoint, max_requests, window) in endpoint_limits {
    let endpoint_config = DMSCRateLimitConfig {
        enabled: true,
        window_duration: window,
        max_requests,
        burst_size: max_requests / 2,
        key_prefix: format!("endpoint_{}", endpoint.replace('/', "_")),
        skip_successful_requests: false,
        skip_failed_requests: true, // 失败请求不计入速率限制
        key_generator: Box::new(move |request| {
            format!("{}:{}", endpoint, request.ip_address)
        }),
    };
    
    ctx.security().set_endpoint_rate_limit(endpoint, endpoint_config).await?;
}

// 动态调整速率限制
ctx.security().update_rate_limit(
    &client_info,
    "api_endpoint",
    50, // 新的请求限制
    Duration::from_minutes(1),
).await?;

// 重置用户速率限制
ctx.security().reset_rate_limit(&client_info, "api_endpoint").await?;
ctx.log().info("Rate limit reset for user");
```

### 分布式速率限制

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置分布式速率限制
let distributed_config = DMSCDistributedRateLimitConfig {
    enabled: true,
    sync_interval: Duration::from_secs(10),
    sync_method: DMSCSyncMethod::Redis,
    consistency_level: DMSCConsistencyLevel::Eventual,
    partition_count: 10,
    replication_factor: 3,
};

ctx.security().init_distributed_rate_limiting(distributed_config).await?;

// 在集群环境中检查速率限制
let cluster_client_info = DMSCClientInfo {
    ip_address: "192.168.1.100".to_string(),
    user_agent: "Mozilla/5.0".to_string(),
    user_id: Some("user123".to_string()),
    session_id: Some("session456".to_string()),
};

// 分布式速率限制检查
match ctx.security().check_distributed_rate_limit(&cluster_client_info, "global_api").await {
    Ok(distributed_info) => {
        ctx.log().info(format!(
            "Distributed rate limit: {} requests remaining across {} nodes",
            distributed_info.remaining,
            distributed_info.node_count
        ));
        
        if distributed_info.is_limited {
            ctx.log().warn("Distributed rate limit exceeded");
            return Err(DMSCError::rate_limit_exceeded("Global rate limit exceeded".to_string()));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Distributed rate limit check failed: {}", e));
        return Err(e);
    }
}

// 滑动窗口速率限制
let sliding_window_config = DMSCSlidingWindowConfig {
    window_size: Duration::from_minutes(5),
    bucket_count: 10,
    sync_interval: Duration::from_secs(30),
};

ctx.security().init_sliding_window_rate_limiting(sliding_window_config).await?;

// 检查滑动窗口速率限制
let sliding_window_info = ctx.security().check_sliding_window_rate_limit(
    &cluster_client_info,
    "api_service",
).await?;

ctx.log().info(format!(
    "Sliding window rate limit: {:.2} requests per second average",
    sliding_window_info.requests_per_second
));
```

## CORS配置

### 跨域资源共享

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置CORS
let cors_config = DMSCCORSConfig {
    allowed_origins: vec![
        "https://app.example.com".to_string(),
        "https://admin.example.com".to_string(),
        "http://localhost:3000".to_string(),
    ],
    allowed_methods: vec![
        "GET".to_string(),
        "POST".to_string(),
        "PUT".to_string(),
        "DELETE".to_string(),
        "OPTIONS".to_string(),
        "PATCH".to_string(),
    ],
    allowed_headers: vec![
        "Content-Type".to_string(),
        "Authorization".to_string(),
        "X-Requested-With".to_string(),
        "X-API-Key".to_string(),
        "X-CSRF-Token".to_string(),
    ],
    exposed_headers: vec![
        "X-Total-Count".to_string(),
        "X-Page-Size".to_string(),
        "X-Rate-Limit-Remaining".to_string(),
    ],
    max_age: Duration::from_hours(24),
    allow_credentials: true,
    enable_preflight: true,
    strict_transport_security: true,
    content_security_policy: Some("default-src 'self'; script-src 'self' 'unsafe-inline'".to_string()),
};

ctx.security().init_cors(cors_config).await?;

// 处理CORS预检请求
let preflight_request = DMSCCORSRequest {
    origin: "https://app.example.com".to_string(),
    method: "POST".to_string(),
    headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
};

match ctx.security().handle_cors_preflight(&preflight_request).await {
    Ok(cors_response) => {
        ctx.log().info("CORS preflight request approved");
        ctx.log().info(format!("Allowed methods: {:?}", cors_response.allowed_methods));
        ctx.log().info(format!("Allowed headers: {:?}", cors_response.allowed_headers));
        
        // 设置CORS响应头
        for (header, value) in cors_response.headers {
            ctx.http().response().header(&header, &value);
        }
    }
    Err(e) => {
        ctx.log().error(format!("CORS preflight request denied: {}", e));
        return Err(e);
    }
}

// 动态CORS配置
let dynamic_cors_config = DMSCDynamicCORSConfig {
    enable_origin_validation: true,
    allowed_origin_patterns: vec![
        r"^https://.*\.example\.com$".to_string(),
        r"^http://localhost:\d+$".to_string(),
    ],
    enable_method_validation: true,
    enable_header_validation: true,
    cache_duration: Duration::from_hours(1),
};

ctx.security().init_dynamic_cors(dynamic_cors_config).await?;

// 验证动态来源
let origin = "https://new-app.example.com";
let is_allowed = ctx.security().validate_cors_origin(origin).await?;
ctx.log().info(format!("Origin {} is allowed: {}", origin, is_allowed));
```

## CSRF保护

### 跨站请求伪造防护

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置CSRF保护
let csrf_config = DMSCCSRFConfig {
    enabled: true,
    token_length: 32,
    token_expiration: Duration::from_hours(24),
    cookie_name: "csrf_token".to_string(),
    header_name: "X-CSRF-Token".to_string(),
    form_field_name: "csrf_token".to_string(),
    secure_cookie: true,
    http_only_cookie: true,
    same_site_cookie: DMSCSameSite::Strict,
    double_submit_cookie: true,
    rotate_tokens: true,
    exempt_paths: vec!["/api/health".to_string(), "/api/public".to_string()],
};

ctx.security().init_csrf_protection(csrf_config).await?;

// 生成CSRF令牌
let session_id = "session123";
let csrf_token = ctx.security().generate_csrf_token(session_id).await?;
ctx.log().info(format!("CSRF token generated: {}...", &csrf_token[..10]));

// 设置CSRF Cookie
ctx.security().set_csrf_cookie(&csrf_token, session_id).await?;

// 验证CSRF令牌
let request_csrf_token = "token_from_request";
let is_valid = ctx.security().verify_csrf_token(request_csrf_token, session_id).await?;

if is_valid {
    ctx.log().info("CSRF token verification successful");
} else {
    ctx.log().error("CSRF token verification failed");
    return Err(DMSCError::security("Invalid CSRF token".to_string()));
}

// 双重提交Cookie验证
let cookie_token = "token_from_cookie";
let header_token = "token_from_header";
let is_double_submit_valid = ctx.security().verify_double_submit_cookie(cookie_token, header_token).await?;

ctx.log().info(format!("Double submit cookie validation: {}", is_double_submit_valid));

// 旋转CSRF令牌
let new_csrf_token = ctx.security().rotate_csrf_token(session_id).await?;
ctx.log().info("CSRF token rotated");

// 清理过期令牌
ctx.security().cleanup_expired_csrf_tokens().await?;
ctx.log().info("Expired CSRF tokens cleaned up");
```

## 安全配置

### 安全策略配置

```rust
use dmsc::prelude::*;
use serde_json::json;

// 综合安全配置
let security_config = DMSCSecurityConfig {
    encryption: DMSCEncryptionConfig {
        default_algorithm: DMSCEncryptionAlgorithm::AES256GCM,
        key_rotation_interval: Duration::from_days(90),
        secure_random_source: DMSCSecureRandom::System,
    },
    authentication: DMSCAuthenticationConfig {
        session_timeout: Duration::from_hours(24),
        max_failed_attempts: 5,
        lockout_duration: Duration::from_minutes(30),
        require_mfa: true,
        password_policy: DMSCPasswordPolicy {
            min_length: 12,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            prevent_common_passwords: true,
            password_history_size: 5,
            max_age: Duration::from_days(90),
        },
    },
    authorization: DMSCAuthorizationConfig {
        default_strategy: DMSCAuthorizationStrategy::RBAC,
        enable_abac: true,
        enable_dynamic_policies: true,
        policy_cache_duration: Duration::from_minutes(15),
    },
    audit: DMSCAuditConfig {
        enabled: true,
        log_level: DMSCAuditLevel::Info,
        retention_period: Duration::from_days(365),
        include_request_body: true,
        include_response_body: false,
        sensitive_fields: vec!["password".to_string(), "credit_card".to_string(), "ssn".to_string()],
    },
    compliance: DMSCComplianceConfig {
        enable_gdpr: true,
        enable_hipaa: false,
        enable_pcidss: false,
        enable_soc2: true,
        data_classification: DMSCDataClassification::Confidential,
        retention_policies: vec![
            DMSCRetentionPolicy {
                data_type: "user_data".to_string(),
                retention_period: Duration::from_days(2555), // 7 years
                anonymize_after: Duration::from_days(1095), // 3 years
            },
            DMSCRetentionPolicy {
                data_type: "audit_logs".to_string(),
                retention_period: Duration::from_days(365), // 1 year
                anonymize_after: Duration::from_days(180), // 6 months
            },
        ],
    },
};

ctx.security().init_security(security_config).await?;

// 应用安全策略
ctx.security().apply_security_policy("default_policy").await?;
ctx.log().info("Security policy applied");

// 获取安全配置状态
let security_status = ctx.security().get_security_status().await?;
ctx.log().info(format!("Security status: {:?}", security_status));
```

## 错误处理

### 安全错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 安全错误处理示例
match ctx.security().authenticate_user("user123", "password").await {
    Ok(auth_result) => {
        ctx.log().info("User authentication successful");
    }
    Err(DMSCError::AuthenticationFailed(e)) => {
        ctx.log().warn(format!("Authentication failed: {}", e));
        
        // 记录失败尝试
        ctx.security().record_failed_attempt("user123").await?;
        
        // 检查是否需要锁定账户
        let failed_attempts = ctx.security().get_failed_attempts("user123").await?;
        if failed_attempts >= 5 {
            ctx.security().lock_account("user123", Duration::from_minutes(30)).await?;
            ctx.log().warn("Account locked due to too many failed attempts");
        }
        
        return Err(DMSCError::authentication("Invalid credentials".to_string()));
    }
    Err(DMSCError::AccountLocked(e)) => {
        ctx.log().warn(format!("Account is locked: {}", e));
        return Err(DMSCError::authentication("Account is temporarily locked".to_string()));
    }
    Err(DMSCError::MFARequired(e)) => {
        ctx.log().info("MFA verification required");
        
        // 引导用户进行MFA验证
        let mfa_challenge = ctx.security().generate_mfa_challenge("user123").await?;
        return Err(DMSCError::mfa_required("MFA verification required".to_string()));
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected authentication error: {}", e));
        return Err(e);
    }
}

// 安全事件响应
let security_event = DMSCSecurityEvent {
    event_type: "authentication_failure".to_string(),
    severity: DMSCSeverity::High,
    source_ip: "192.168.1.100".to_string(),
    user_id: Some("user123".to_string()),
    description: "Multiple authentication failures detected".to_string(),
    metadata: {
        let mut meta = std::collections::HashMap::new();
        meta.insert("failed_attempts".to_string(), "5".to_string());
        meta.insert("time_window".to_string(), "5_minutes".to_string());
        meta
    },
    timestamp: chrono::Utc::now(),
};

ctx.security().handle_security_event(security_event).await?;
ctx.log().info("Security event handled");
```

## 最佳实践

1. **密钥管理**: 使用安全的密钥存储和管理机制
2. **定期轮换**: 定期轮换加密密钥和认证令牌
3. **最小权限**: 实施最小权限原则，只授予必要的权限
4. **多层防护**: 使用多层安全防护措施
5. **监控审计**: 实施全面的安全监控和审计
6. **错误处理**: 妥善处理安全错误，避免信息泄露
7. **定期评估**: 定期评估和更新安全策略
8. **合规性**: 确保符合相关安全合规要求
9. **备份恢复**: 实施安全备份和恢复机制
10. **安全培训**: 定期进行安全培训和意识提升
11. **输入验证**: 对所有用户输入进行严格验证和清理
12. **使用HTTPS**: 始终使用HTTPS加密数据传输
13. **安全头部**: 配置适当的安全HTTP头部
14. **依赖管理**: 定期更新和修补安全依赖
15. **渗透测试**: 定期进行安全渗透测试

<div align="center">

## 运行步骤

</div>

1. **环境准备**: 确保已安装Rust开发环境
2. **创建项目**: 使用 `cargo new security-example` 创建新项目
3. **添加依赖**: 在 `Cargo.toml` 中添加 dms 依赖
4. **创建配置**: 复制上述配置代码到 `src/config.rs`
5. **运行示例**: 执行 `cargo run` 启动应用

<div align="center">

## 预期结果

</div>

运行成功后，您将看到以下输出：

```
[2024-01-01 12:00:00] INFO: DMSC Security Example starting...
[2024-01-01 12:00:00] INFO: JWT authentication initialized
[2024-01-01 12:00:00] INFO: OAuth2 authentication configured
[2024-01-01 12:00:00] INFO: MFA support enabled
[2024-01-01 12:00:00] INFO: RBAC roles created: admin, user, moderator
[2024-01-01 12:00:00] INFO: ABAC policies configured
[2024-01-01 12:00:00] INFO: Symmetric encryption initialized
[2024-01-01 12:00:00] INFO: Asymmetric key pair generated
[2024-01-01 12:00:00] INFO: Input validation rules configured
[2024-01-01 12:00:00] INFO: Rate limiting enabled
[2024-01-01 12:00:00] INFO: CORS configuration applied
[2024-01-01 12:00:00] INFO: CSRF protection enabled
[2024-01-01 12:00:00] INFO: Security policy applied
[2024-01-01 12:00:00] INFO: Application running with full security features!
```

<div align="center">

## 扩展功能

</div>

### 高级加密功能

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置高级加密方案
let advanced_encryption = DMAdvancedEncryptionConfig {
    master_key_rotation: Duration::from_days(30),
    key_hierarchy: DMKeyHierarchy {
        master_key_id: "master-key-2024".to_string(),
        data_encryption_keys: vec![
            DMDataEncryptionKey {
                key_id: "dek-001".to_string(),
                algorithm: DMSCEncryptionAlgorithm::AES256GCM,
                rotation_schedule: Duration::from_days(7),
                backup_enabled: true,
            },
            DMDataEncryptionKey {
                key_id: "dek-002".to_string(),
                algorithm: DMSCEncryptionAlgorithm::ChaCha20Poly1305,
                rotation_schedule: Duration::from_days(14),
                backup_enabled: true,
            },
        ],
    },
    hardware_security_module: Some(DMHSMConfig {
        provider: "AWS-CloudHSM".to_string(),
        endpoint: "https://cloudhsm.us-east-1.amazonaws.com".to_string(),
        key_store: "dms-key-store".to_string(),
    }),
    quantum_safe: DMQuantumSafeConfig {
        enabled: true,
        algorithm: "CRYSTALS-KYBER".to_string(),
        key_size: 1024,
        hybrid_mode: true,
    },
};

ctx.security().configure_advanced_encryption(advanced_encryption).await?;

// 实施信封加密
let sensitive_data = "Highly sensitive business information";
let envelope_encrypted = ctx.security().envelope_encrypt(sensitive_data).await?;
ctx.log().info(format!(

<div align="center">

## 总结

</div>

本示例展示了DMSC框架全面的安全功能，帮助您构建安全、可靠且符合合规要求的应用程序。通过认证管理、授权控制、加密解密、输入验证、速率限制和CSRF保护等多重安全机制，您可以有效保护应用和数据安全。

### 核心功能

1. **JWT认证**: 支持HS256/RS256算法，完整的令牌生命周期管理
2. **OAuth2认证**: 集成第三方认证服务，支持多种授权流程
3. **多因素认证**: TOTP支持，备份代码机制，增强账户安全
4. **RBAC授权**: 基于角色的访问控制，灵活的权限管理
5. **ABAC授权**: 基于属性的访问控制，支持复杂业务场景
6. **对称加密**: AES256-GCM加密，支持批量处理
7. **非对称加密**: RSA密钥对管理，数字签名验证
8. **输入验证**: 多维度数据验证，防止注入攻击
9. **速率限制**: 分布式限流，滑动窗口算法
10. **CSRF保护**: 双重提交Cookie，令牌旋转机制

### 高级特性

1. **密钥派生**: PBKDF2/Argon2算法，安全密码存储
2. **动态CORS**: 灵活配置跨域策略，支持正则表达式
3. **安全审计**: 全面的安全事件记录和分析
4. **合规支持**: GDPR、HIPAA、PCI DSS、SOC2合规
5. **错误处理**: 完善的安全错误处理机制
6. **配置管理**: 集中化安全配置管理

### 最佳实践

- 使用强密码策略和定期密码轮换
- 实施最小权限原则和零信任架构
- 对所有用户输入进行严格验证和清理
- 使用HTTPS加密所有数据传输
- 配置适当的安全HTTP头部
- 定期更新和修补安全依赖
- 实施全面的安全监控和审计
- 定期进行安全渗透测试
- 建立完善的密钥管理制度
- 制定和测试安全事件响应计划

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况

- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信