<div align="center">

# Security API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

security模块提供安全功能，包括认证、授权、加密、输入验证与防护机制。

## 模块概述

</div>

security模块包含以下子模块：

- **auth**: 认证管理
- **authorization**: 授权管理  
- **crypto**: 加密解密
- **validation**: 输入验证
- **rate_limit**: 速率限制
- **cors**: CORS配置
- **csrf**: CSRF保护
- **xss**: XSS防护
- **sql_injection**: SQL注入防护

<div align="center">

## 核心组件

</div>  

### DMSCSecurityManager

安全管理器主接口，提供统一的安全功能访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `authenticate(credentials)` | 用户认证 | `credentials: DMSCCredentials` | `DMSCResult<DMSCUser>` |
| `authorize(user, resource, action)` | 权限检查 | `user: &DMSCUser`, `resource: &str`, `action: &str` | `DMSCResult<bool>` |
| `encrypt(data, key)` | 数据加密 | `data: &[u8]`, `key: &[u8]` | `DMSCResult<Vec<u8>>` |
| `decrypt(data, key)` | 数据解密 | `data: &[u8]`, `key: &[u8]` | `DMSCResult<Vec<u8>>` |
| `hash_password(password)` | 密码哈希 | `password: &str` | `DMSCResult<String>` |
| `verify_password(password, hash)` | 密码验证 | `password: &str`, `hash: &str` | `DMSCResult<bool>` |
| `validate_input(input, rules)` | 输入验证 | `input: &str`, `rules: &[DMSCValidationRule]` | `DMSCResult<()>` |
| `rate_limit(key, limit)` | 速率限制 | `key: &str`, `limit: DMSCRateLimit` | `DMSCResult<()>` |

#### 使用示例

```rust
use dms::prelude::*;

// 用户认证
let credentials = DMSCCredentials::UsernamePassword {
    username: "john_doe".to_string(),
    password: "secure_password".to_string(),
};

match ctx.security().authenticate(credentials).await? {
    Some(user) => {
        ctx.log().info(format!("User authenticated: {}", user.username));
        
        // 检查权限
        if ctx.security().authorize(&user, "user:profile", "read").await? {
            ctx.log().info("User has read permission for profile");
        }
    }
    None => {
        ctx.log().warn("Authentication failed");
        return Err(DMSCError::auth("Invalid credentials"));
    }
}

// 数据加密
let sensitive_data = b"confidential information";
let encryption_key = ctx.security().derive_key("my_secret_key", b"salt", 32)?;
let encrypted_data = ctx.security().encrypt(sensitive_data, &encryption_key)?;

// 密码哈希
let password = "user_password";
let password_hash = ctx.security().hash_password(password)?;

// 验证密码
let is_valid = ctx.security().verify_password(password, &password_hash)?;
```

### DMSCCredentials

认证凭据枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `UsernamePassword { username, password }` | 用户名密码 |
| `ApiKey { key }` | API密钥 |
| `JwtToken { token }` | JWT令牌 |
| `OAuth2 { code, provider }` | OAuth2授权码 |
| `Certificate { cert_data }` | 客户端证书 |

### DMSCUser

用户结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 用户ID |
| `username` | `String` | 用户名 |
| `email` | `String` | 邮箱 |
| `roles` | `Vec<String>` | 角色列表 |
| `permissions` | `Vec<String>` | 权限列表 |
| `metadata` | `HashMap<String, String>` | 元数据 |
| `created_at` | `DateTime<Utc>` | 创建时间 |
| `last_login` | `Option<DateTime<Utc>>` | 最后登录时间 |

<div align="center">

## 认证管理

</div>  

### JWT认证

```rust
use dms::prelude::*;

// 生成JWT令牌
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

// 验证JWT令牌
match ctx.security().validate_jwt(&jwt_token, &jwt_config)? {
    Some(claims) => {
        ctx.log().info(format!("JWT valid for user: {}", claims.sub));
        
        // 从JWT提取用户信息
        let user_from_jwt = ctx.security().user_from_jwt_claims(&claims)?;
        ctx.log().info(format!("User roles: {:?}", user_from_jwt.roles));
    }
    None => {
        ctx.log().warn("Invalid JWT token");
        return Err(DMSCError::auth("Invalid JWT token"));
    }
}
```

### OAuth2认证

```rust
use dms::prelude::*;

// OAuth2配置
let oauth2_config = DMSCOAuth2Config {
    client_id: "my_client_id".to_string(),
    client_secret: "my_client_secret".to_string(),
    authorization_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
    token_url: "https://oauth2.googleapis.com/token".to_string(),
    redirect_uri: "http://localhost:8080/oauth2/callback".to_string(),
    scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
    provider: "google".to_string(),
};

// 生成授权URL
let auth_url = ctx.security().generate_oauth2_auth_url(&oauth2_config)?;
ctx.log().info(format!("OAuth2 authorization URL: {}", auth_url));

// 处理OAuth2回调
let auth_code = "received_authorization_code";
let oauth2_result = ctx.security().exchange_oauth2_code(auth_code, &oauth2_config).await?;

match oauth2_result {
    DMSCOAuth2Result::Success { access_token, refresh_token, user_info } => {
        ctx.log().info(format!("OAuth2 successful for user: {}", user_info.email));
        
        // 创建或更新用户
        let user = ctx.security().create_or_update_user_from_oauth2(&user_info).await?;
        
        // 生成应用JWT
        let app_jwt = ctx.security().generate_jwt(&user, &jwt_config)?;
        ctx.log().info(format!("Generated app JWT: {}", app_jwt));
    }
    DMSCOAuth2Result::Error { error, description } => {
        ctx.log().error(format!("OAuth2 error: {} - {}", error, description));
        return Err(DMSCError::auth("OAuth2 authentication failed"));
    }
}
```

### 多因素认证(MFA)

```rust
use dms::prelude::*;

// 启用MFA
let user_id = "12345";
let mfa_secret = ctx.security().generate_mfa_secret(user_id)?;

// 生成QR码用于设置认证器应用
let qr_code_data = ctx.security().generate_mfa_qr_code(
    &mfa_secret,
    "MyApp",
    "john@example.com"
)?;

// 验证MFA代码
let mfa_code = "123456"; // 用户输入的6位代码
let is_valid = ctx.security().verify_mfa_code(user_id, mfa_code)?;

if is_valid {
    ctx.log().info("MFA verification successful");
    
    // 启用MFA
    ctx.security().enable_mfa(user_id, &mfa_secret).await?;
} else {
    ctx.log().warn("MFA verification failed");
    return Err(DMSCError::auth("Invalid MFA code"));
}

// 备份代码
let backup_codes = ctx.security().generate_mfa_backup_codes(user_id, 10)?;
ctx.log().info(format!("Generated {} backup codes", backup_codes.len()));
```

<div align="center">

## 授权管理

</div>  

### 基于角色的访问控制(RBAC)

```rust
use dms::prelude::*;

// 定义角色和权限
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

// 检查角色权限
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
    
    // 检查具体权限
    if ctx.security().has_permission(&user, "user:create")? {
        ctx.log().info("User can create other users");
    }
}

// 检查资源权限
let resource = "user:profile:12345";
let action = "read";

if ctx.security().authorize(&user, resource, action).await? {
    ctx.log().info(format!("User authorized to {} {}", action, resource));
} else {
    ctx.log().warn(format!("User not authorized to {} {}", action, resource));
    return Err(DMSCError::auth("Insufficient permissions"));
}
```

### 基于属性的访问控制(ABAC)

```rust
use dms::prelude::*;

// 定义属性策略
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

// 评估ABAC策略
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

## 加密解密

</div>

### 对称加密

```rust
use dms::prelude::*;

// AES加密
let plaintext = b"sensitive data that needs encryption";
let key = ctx.security().generate_symmetric_key(32)?; // 256-bit key
let iv = ctx.security().generate_iv(16)?; // 128-bit IV

let encrypted_data = ctx.security().encrypt_aes256_cbc(plaintext, &key, &iv)?;
ctx.log().info(format!("Encrypted {} bytes", encrypted_data.len()));

// AES解密
let decrypted_data = ctx.security().decrypt_aes256_cbc(&encrypted_data, &key, &iv)?;
let decrypted_text = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted text: {}", decrypted_text));
```

### 非对称加密

```rust
use dms::prelude::*;

// 生成RSA密钥对
let (private_key, public_key) = ctx.security().generate_rsa_keypair(2048)?;

// 使用公钥加密
let plaintext = b"data to encrypt with public key";
let encrypted_data = ctx.security().encrypt_rsa_public(plaintext, &public_key)?;

// 使用私钥解密
let decrypted_data = ctx.security().decrypt_rsa_private(&encrypted_data, &private_key)?;
let decrypted_text = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted with private key: {}", decrypted_text));

// 数字签名
let data_to_sign = b"important message";
let signature = ctx.security().sign_rsa(data_to_sign, &private_key)?;

// 验证签名
let is_valid = ctx.security().verify_rsa_signature(data_to_sign, &signature, &public_key)?;
if is_valid {
    ctx.log().info("Signature is valid");
} else {
    ctx.log().error("Signature verification failed");
}
```

### 密钥派生

```rust
use dms::prelude::*;

// PBKDF2密钥派生
let password = "user_password";
let salt = ctx.security().generate_salt(32)?;
let derived_key = ctx.security().derive_key_pbkdf2(password, &salt, 10000, 32)?;

ctx.log().info(format!("Derived {}-byte key", derived_key.len()));

// 存储盐和派生密钥用于验证
let password_hash = DMSCPasswordHash {
    algorithm: "pbkdf2_sha256".to_string(),
    salt: base64::encode(&salt),
    hash: base64::encode(&derived_key),
    iterations: 10000,
};

// 验证密码
let is_valid = ctx.security().verify_password_pbkdf2(
    password,
    &password_hash.salt,
    &password_hash.hash,
    password_hash.iterations
)?;
```

<div align="center">

## 输入验证

</div>

### 验证规则

```rust
use dms::prelude::*;

// 定义验证规则
let email_rules = vec![
    DMSCValidationRule::Required,
    DMSCValidationRule::Email,
    DMSCValidationRule::MaxLength(100),
    DMSCValidationRule::Custom(|value| {
        // 自定义验证：检查是否是公司邮箱
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

// 执行验证
let email = "john@company.com";
let password = "SecurePass123!";
let username = "john_doe";

ctx.security().validate_input(email, &email_rules)?;
ctx.security().validate_input(password, &password_rules)?;
ctx.security().validate_input(username, &username_rules)?;

ctx.log().info("All inputs validated successfully");
```

### 数据清理

```rust
use dms::prelude::*;

// HTML清理
let dirty_html = r#"
    <script>alert('XSS')</script>
    <p>Safe content</p>
    <img src="javascript:alert('XSS')" />
    <a href="javascript:alert('XSS')">Click me</a>
"#;

let clean_html = ctx.security().sanitize_html(dirty_html)?;
ctx.log().info(format!("Cleaned HTML: {}", clean_html));

// SQL注入防护
let user_input = "admin' OR '1'='1";
let safe_input = ctx.security().escape_sql(user_input);
ctx.log().info(format!("Escaped SQL: {}", safe_input));

// 路径遍历防护
let file_path = "../../../etc/passwd";
let safe_path = ctx.security().sanitize_path(file_path)?;
ctx.log().info(format!("Sanitized path: {}", safe_path));
```

<div align="center">

## 速率限制

</div>

### 令牌桶算法

```rust
use dms::prelude::*;

// 配置速率限制
let rate_limit_config = DMSCRateLimitConfig {
    algorithm: DMSCRateLimitAlgorithm::TokenBucket,
    capacity: 100,        // 桶容量
    refill_rate: 10,      // 每秒补充令牌数
    refill_period: Duration::from_secs(1),
};

// 应用速率限制
let client_ip = "192.168.1.100";
let rate_limit_key = format!("rate_limit:{}", client_ip);

match ctx.security().check_rate_limit(&rate_limit_key, &rate_limit_config)? {
    DMSCRateLimitResult::Allowed => {
        ctx.log().info("Request allowed");
        // 处理请求
    }
    DMSCRateLimitResult::Denied { retry_after } => {
        ctx.log().warn(format!("Rate limit exceeded, retry after {:?}", retry_after));
        return Err(DMSCError::rate_limit("Too many requests"));
    }
}

// 滑动窗口速率限制
let sliding_window_config = DMSCRateLimitConfig {
    algorithm: DMSCRateLimitAlgorithm::SlidingWindow,
    window_size: Duration::from_minutes(1),
    max_requests: 60,  // 每分钟最多60个请求
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

## CORS配置

```rust
use dms::prelude::*;

// 配置CORS
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

// 检查CORS请求
let origin = "https://app.example.com";
let method = "POST";
let headers = vec!["Content-Type", "Authorization"];

match ctx.security().check_cors_request(&cors_config, origin, method, &headers)? {
    DMSCCorsResult::Allowed { headers } => {
        ctx.log().info("CORS request allowed");
        // 设置响应头
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

## CSRF保护

```rust
use dms::prelude::*;

// 生成CSRF令牌
let csrf_token = ctx.security().generate_csrf_token()?;

// 在表单中包含CSRF令牌
let form_html = format!(r#"
    <form method="POST" action="/update-profile">
        <input type="hidden" name="csrf_token" value="{}" />
        <!-- 其他表单字段 -->
        <button type="submit">Update Profile</button>
    </form>
"#, csrf_token);

// 验证CSRF令牌
let submitted_token = "submitted_csrf_token";
let session_token = "session_csrf_token";

if !ctx.security().verify_csrf_token(submitted_token, session_token)? {
    ctx.log().warn("CSRF token verification failed");
    return Err(DMSCError::security("Invalid CSRF token"));
}

ctx.log().info("CSRF token verified successfully");
```

<div align="center">

## 安全配置

</div>

### DMSCSecurityConfig

安全配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `jwt_secret` | `String` | JWT密钥 | 必填 |
| `password_hash_algorithm` | `DMSCPasswordHashAlgorithm` | 密码哈希算法 | `Argon2id` |
| `session_timeout` | `Duration` | 会话超时时间 | `24h` |
| `max_login_attempts` | `u32` | 最大登录尝试次数 | `5` |
| `lockout_duration` | `Duration` | 账户锁定时间 | `30m` |
| `mfa_required` | `bool` | 是否需要MFA | `false` |
| `password_policy` | `DMSCPasswordPolicy` | 密码策略 | 默认策略 |
| `rate_limit_config` | `DMSCRateLimitConfig` | 速率限制配置 | 默认配置 |

#### 配置示例

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

## 错误处理

</div>

### 安全错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `SECURITY_AUTH_FAILED` | 认证失败 |
| `SECURITY_UNAUTHORIZED` | 未授权 |
| `SECURITY_FORBIDDEN` | 权限不足 |
| `SECURITY_INVALID_TOKEN` | 无效令牌 |
| `SECURITY_TOKEN_EXPIRED` | 令牌过期 |
| `SECURITY_RATE_LIMITED` | 速率限制 |
| `SECURITY_VALIDATION_FAILED` | 验证失败 |
| `SECURITY_ENCRYPTION_ERROR` | 加密错误 |

### 错误处理示例

```rust
use dms::prelude::*;

match ctx.security().authenticate(credentials).await {
    Ok(user) => {
        ctx.log().info(format!("User authenticated: {}", user.username));
    }
    Err(DMSCError { code, .. }) if code == "SECURITY_AUTH_FAILED" => {
        ctx.log().warn("Authentication failed");
        
        // 增加失败计数
        let fail_count = ctx.security().increment_login_failures(&username).await?;
        
        if fail_count >= ctx.config().security.max_login_attempts {
            // 锁定账户
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

## 最佳实践

</div>

1. **使用强密码策略**: 要求复杂密码并定期更换
2. **启用多因素认证**: 对敏感操作要求MFA
3. **实施最小权限原则**: 只授予必要的权限
4. **加密敏感数据**: 对存储和传输的敏感数据进行加密
5. **输入验证**: 对所有用户输入进行验证和清理
6. **速率限制**: 对API端点实施适当的速率限制
7. **安全审计**: 记录所有安全相关事件
8. **定期安全扫描**: 定期检查安全漏洞
9. **使用HTTPS**: 所有通信使用加密连接
10. **及时更新**: 保持安全库和依赖项的最新版本

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置
- [cache](./cache.md): 缓存模块，提供多后端缓存抽象，缓存用户会话和权限数据
- [database](./database.md): 数据库模块，提供用户数据持久化和查询功能
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据