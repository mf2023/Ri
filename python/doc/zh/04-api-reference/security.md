<div align="center">

# Security API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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

<div align="center">

#### 方法表

</div>

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

```python
from dmsc.security import DMSCSecurityManager, DMSCCredentials

# 创建安全管理器
security_manager = DMSCSecurityManager()

# 用户认证
credentials = DMSCCredentials(
    username="user@example.com",
    password="secure_password"
)
result = security_manager.authenticate(credentials)

if result.is_success():
    user = result.data
    print(f"Authentication successful for user: {user.username}")
else:
    print(f"Authentication failed: {result.error}")

# 权限检查
user = result.data
auth_result = security_manager.authorize(user, "documents", "read")
if auth_result.is_success() and auth_result.data:
    print("User has read permission on documents")

# 数据加密
sensitive_data = b"confidential information"
encryption_key = b"32_byte_encryption_key_1234567890"
encrypted_result = security_manager.encrypt(sensitive_data, encryption_key)

if encrypted_result.is_success():
    encrypted_data = encrypted_result.data
    print(f"Data encrypted successfully: {len(encrypted_data)} bytes")

# 密码哈希
password = "user_password"
hash_result = security_manager.hash_password(password)

if hash_result.is_success():
    password_hash = hash_result.data
    print(f"Password hashed: {password_hash[:20]}...")

# 密码验证
verify_result = security_manager.verify_password(password, password_hash)
if verify_result.is_success() and verify_result.data:
    print("Password verification successful")

# 输入验证
user_input = "user@example.com"
validation_rules = [
    DMSCValidationRule.EMAIL,
    DMSCValidationRule.REQUIRED
]
validation_result = security_manager.validate_input(user_input, validation_rules)

if validation_result.is_success():
    print("Input validation passed")

# 速率限制
rate_limit_config = DMSCRateLimit(
    max_requests=100,
    window_seconds=60
)
rate_result = security_manager.rate_limit("user_ip_123", rate_limit_config)

if rate_result.is_success():
    print("Rate limit check passed")
```

### DMSCCredentials

认证凭据类，封装用户认证信息。

#### 构造函数

```python
DMSCCredentials(
    username: str = "",
    password: str = "",
    api_key: str = "",
    token: str = "",
    certificate_path: str = "",
    metadata: Dict[str, Any] = None
)
```

#### 属性

| 属性 | 类型 | 描述 |
|:--------|:--------|:--------|
| `username` | `str` | 用户名 |
| `password` | `str` | 密码 |
| `api_key` | `str` | API密钥 |
| `token` | `str` | 认证令牌 |
| `certificate_path` | `str` | 证书路径 |
| `metadata` | `Dict[str, Any]` | 附加元数据 |

### DMSCUser

用户实体类，表示已认证的用户。

#### 构造函数

```python
DMSCUser(
    id: str = "",
    username: str = "",
    email: str = "",
    roles: List[str] = None,
    permissions: List[str] = None,
    metadata: Dict[str, Any] = None,
    created_at: datetime = None,
    last_login: datetime = None,
    is_active: bool = True,
    is_verified: bool = False
)
```

#### 属性

| 属性 | 类型 | 描述 |
|:--------|:--------|:--------|
| `id` | `str` | 用户ID |
| `username` | `str` | 用户名 |
| `email` | `str` | 邮箱地址 |
| `roles` | `List[str]` | 用户角色 |
| `permissions` | `List[str]` | 用户权限 |
| `metadata` | `Dict[str, Any]` | 附加元数据 |
| `created_at` | `datetime` | 创建时间 |
| `last_login` | `datetime` | 最后登录时间 |
| `is_active` | `bool` | 是否激活 |
| `is_verified` | `bool` | 是否已验证 |

### DMSCValidationRule

验证规则枚举，定义输入验证规则。

#### 取值

```python
from dmsc.security import DMSCValidationRule

class DMSCValidationRule:
    REQUIRED = "required"          # 必填验证
    EMAIL = "email"                # 邮箱格式验证
    URL = "url"                    # URL格式验证
    PHONE = "phone"                # 电话号码验证
    INTEGER = "integer"            # 整数验证
    FLOAT = "float"                # 浮点数验证
    BOOLEAN = "boolean"            # 布尔值验证
    MIN_LENGTH = "min_length"      # 最小长度验证
    MAX_LENGTH = "max_length"      # 最大长度验证
    PATTERN = "pattern"            # 正则表达式验证
    RANGE = "range"                # 范围验证
    ENUM = "enum"                  # 枚举验证
    CUSTOM = "custom"              # 自定义验证
```

### DMSCRateLimit

速率限制配置类，定义速率限制规则。

#### 构造函数

```python
DMSCRateLimit(
    max_requests: int = 100,
    window_seconds: int = 60,
    burst_size: int = 10,
    key_prefix: str = "rate_limit",
    skip_successful_requests: bool = False,
    skip_failed_requests: bool = False,
    enable_headers: bool = True,
    error_message: str = "Rate limit exceeded"
)
```

#### 属性

| 属性 | 类型 | 描述 |
|:--------|:--------|:--------|
| `max_requests` | `int` | 最大请求数 |
| `window_seconds` | `int` | 时间窗口（秒） |
| `burst_size` | `int` | 突发大小 |
| `key_prefix` | `str` | 键前缀 |
| `skip_successful_requests` | `bool` | 是否跳过成功请求 |
| `skip_failed_requests` | `bool` | 是否跳过失败请求 |
| `enable_headers` | `bool` | 是否启用响应头 |
| `error_message` | `str` | 错误消息 |

### DMSCResult

结果封装类，用于返回操作结果。

#### 构造函数

```python
DMSCResult(
    success: bool = True,
    data: Any = None,
    error: str = "",
    code: int = 0
)
```

#### 属性

| 属性 | 类型 | 描述 |
|:--------|:--------|:--------|
| `success` | `bool` | 是否成功 |
| `data` | `Any` | 返回数据 |
| `error` | `str` | 错误信息 |
| `code` | `int` | 错误代码 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `is_success()` | 检查是否成功 | 无 | `bool` |
| `is_error()` | 检查是否错误 | 无 | `bool` |
| `get_data()` | 获取数据 | 无 | `Any` |
| `get_error()` | 获取错误信息 | 无 | `str` |

<div align="center">

## 使用示例

</div>

### 完整认证流程

```python
from dmsc.security import (
    DMSCSecurityManager, 
    DMSCCredentials, 
    DMSCUser,
    DMSCValidationRule
)

async def authenticate_user(username: str, password: str):
    """完整的用户认证流程"""
    
    # 创建安全管理器
    security_manager = DMSCSecurityManager()
    
    # 验证输入
    validation_rules = [
        DMSCValidationRule.REQUIRED,
        DMSCValidationRule.MIN_LENGTH
    ]
    
    username_result = security_manager.validate_input(username, validation_rules)
    password_result = security_manager.validate_input(password, validation_rules)
    
    if not username_result.is_success() or not password_result.is_success():
        return DMSCResult(success=False, error="Invalid input")
    
    # 创建认证凭据
    credentials = DMSCCredentials(
        username=username,
        password=password
    )
    
    # 执行认证
    auth_result = security_manager.authenticate(credentials)
    
    if auth_result.is_success():
        user = auth_result.data
        print(f"User {user.username} authenticated successfully")
        
        # 检查权限
        perm_result = security_manager.authorize(user, "documents", "read")
        if perm_result.is_success() and perm_result.data:
            print(f"User has read permission on documents")
        
        return DMSCResult(success=True, data=user)
    else:
        print(f"Authentication failed: {auth_result.error}")
        return DMSCResult(success=False, error="Authentication failed")

# 使用示例
result = await authenticate_user("user@example.com", "secure_password")
```

### 数据加密和解密

```python
from dmsc.security import DMSCSecurityManager
import os

def secure_data_transmission():
    """安全数据传输示例"""
    
    security_manager = DMSCSecurityManager()
    
    # 生成加密密钥
    encryption_key = os.urandom(32)
    
    # 敏感数据
    sensitive_data = {
        "credit_card": "1234-5678-9012-3456",
        "ssn": "123-45-6789",
        "password": "user_secret_password"
    }
    
    # 加密数据
    import json
    data_bytes = json.dumps(sensitive_data).encode('utf-8')
    
    encrypt_result = security_manager.encrypt(data_bytes, encryption_key)
    
    if encrypt_result.is_success():
        encrypted_data = encrypt_result.data
        print(f"Data encrypted: {len(encrypted_data)} bytes")
        
        # 传输加密数据...
        
        # 解密数据
        decrypt_result = security_manager.decrypt(encrypted_data, encryption_key)
        
        if decrypt_result.is_success():
            decrypted_bytes = decrypt_result.data
            decrypted_data = json.loads(decrypted_bytes.decode('utf-8'))
            print(f"Data decrypted successfully")
            return decrypted_data
    
    return None

# 使用示例
original_data = secure_data_transmission()
```

### 速率限制实现

```python
from dmsc.security import DMSCSecurityManager, DMSCRateLimit
import asyncio

class RateLimitedAPI:
    def __init__(self):
        self.security_manager = DMSCSecurityManager()
        
    async def handle_request(self, client_ip: str, endpoint: str):
        """处理API请求并实施速率限制"""
        
        # 根据端点设置不同的速率限制
        if endpoint == "/api/login":
            rate_limit = DMSCRateLimit(
                max_requests=5,
                window_seconds=300,  # 5分钟
                burst_size=2,
                error_message="Too many login attempts"
            )
        elif endpoint == "/api/data":
            rate_limit = DMSCRateLimit(
                max_requests=100,
                window_seconds=60,   # 1分钟
                burst_size=10,
                error_message="Rate limit exceeded"
            )
        else:
            rate_limit = DMSCRateLimit(
                max_requests=1000,
                window_seconds=60,   # 1分钟
                burst_size=50,
                error_message="Rate limit exceeded"
            )
        
        # 创建速率限制键
        rate_limit_key = f"{client_ip}:{endpoint}"
        
        # 检查速率限制
        rate_result = self.security_manager.rate_limit(rate_limit_key, rate_limit)
        
        if rate_result.is_success():
            # 处理请求
            print(f"Processing request from {client_ip} to {endpoint}")
            return {"status": "success", "data": "request_data"}
        else:
            # 速率限制超限
            print(f"Rate limit exceeded for {client_ip} on {endpoint}")
            return {"status": "error", "message": rate_result.error}

# 使用示例
api = RateLimitedAPI()
result = await api.handle_request("192.168.1.100", "/api/data")
```

<div align="center">

## 最佳实践

</div>

### 1. 密码安全

```python
from dmsc.security import DMSCSecurityManager

def secure_password_handling():
    """安全密码处理示例"""
    
    security_manager = DMSCSecurityManager()
    
    # 用户注册时哈希密码
    password = "user_password_123"
    hash_result = security_manager.hash_password(password)
    
    if hash_result.is_success():
        password_hash = hash_result.data
        # 存储密码哈希到数据库
        store_user_password_hash(password_hash)
        
        # 验证密码（登录时）
        verify_result = security_manager.verify_password(password, password_hash)
        
        if verify_result.is_success() and verify_result.data:
            print("Password verification successful")
        else:
            print("Password verification failed")

# 使用示例
secure_password_handling()
```

### 2. 输入验证

```python
from dmsc.security import DMSCSecurityManager, DMSCValidationRule

def validate_user_input():
    """用户输入验证示例"""
    
    security_manager = DMSCSecurityManager()
    
    # 验证邮箱地址
    email = "user@example.com"
    email_rules = [
        DMSCValidationRule.REQUIRED,
        DMSCValidationRule.EMAIL,
        DMSCValidationRule.MAX_LENGTH
    ]
    
    email_result = security_manager.validate_input(email, email_rules)
    
    if email_result.is_success():
        print(f"Email {email} is valid")
    else:
        print(f"Email validation failed: {email_result.error}")
    
    # 验证URL
    website = "https://example.com"
    url_rules = [
        DMSCValidationRule.URL,
        DMSCValidationRule.REQUIRED
    ]
    
    url_result = security_manager.validate_input(website, url_rules)
    
    if url_result.is_success():
        print(f"URL {website} is valid")
    else:
        print(f"URL validation failed: {url_result.error}")

# 使用示例
validate_user_input()
```

### 3. 安全头部设置

```python
from dmsc.security import DMSCSecurityManager

def set_security_headers():
    """设置安全HTTP头部"""
    
    security_manager = DMSCSecurityManager()
    
    # 配置安全头部
    security_headers = {
        "X-Content-Type-Options": "nosniff",
        "X-Frame-Options": "DENY",
        "X-XSS-Protection": "1; mode=block",
        "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
        "Content-Security-Policy": "default-src 'self'",
        "Referrer-Policy": "strict-origin-when-cross-origin"
    }
    
    # 应用安全头部到响应
    for header, value in security_headers.items():
        # 在实际应用中，这里会设置HTTP响应头
        print(f"Setting header: {header} = {value}")

# 使用示例
set_security_headers()
```

### 4. 错误处理

```python
from dmsc.security import DMSCSecurityManager, DMSCResult

def secure_error_handling():
    """安全错误处理示例"""
    
    security_manager = DMSCSecurityManager()
    
    try:
        # 执行认证操作
        credentials = DMSCCredentials(
            username="user@example.com",
            password="wrong_password"
        )
        
        result = security_manager.authenticate(credentials)
        
        if not result.is_success():
            # 不要泄露具体的错误信息
            print("Authentication failed")
            # 记录详细错误到安全日志
            log_security_event("auth_failed", username=credentials.username)
            
            return DMSCResult(success=False, error="Invalid credentials")
        
        return result
        
    except Exception as e:
        # 捕获异常并安全处理
        print("Security error occurred")
        log_security_event("security_error", error=str(e))
        
        # 返回通用错误信息，不泄露内部细节
        return DMSCResult(success=False, error="Security error")

# 使用示例
result = secure_error_handling()
```