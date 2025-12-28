<div align="center">

# 安全功能使用示例

**版本：1.0.0**

**最后修改日期：2025-12-12**

本示例展示如何使用DMSC的security模块进行加密解密、签名验证、权限控制、输入验证和安全配置等。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 数据加密和解密（对称加密和非对称加密）
- 数字签名和验证
- JWT令牌生成和验证
- 输入验证和过滤
- 敏感信息脱敏
- 安全随机数生成
- 密码哈希和验证
- API安全中间件

## 环境准备

### 依赖安装

```bash
pip install dmsc[crypto,jwt,validation]
```

### 配置文件

```yaml
# config.yaml
security:
  encryption:
    default_algorithm: AES-256-GCM
    key_derivation: PBKDF2
    key_size: 32
    salt_size: 16
    iv_size: 12
  
  jwt:
    algorithm: RS256
    access_token_expiry: 3600  # 1小时
    refresh_token_expiry: 604800  # 7天
    issuer: dmsc-demo
    audience: dmsc-users
  
  password:
    hashing_algorithm: bcrypt
    salt_rounds: 12
    min_length: 8
    require_uppercase: true
    require_lowercase: true
    require_numbers: true
    require_special_chars: true
  
  input_validation:
    max_string_length: 1000
    allowed_file_extensions: [".jpg", ".png", ".pdf", ".txt"]
    max_file_size: 10485760  # 10MB
    sql_injection_patterns: ["SELECT.*FROM", "DROP.*TABLE", "INSERT.*INTO"]
    xss_patterns: ["<script", "javascript:", "onerror="]
  
  cors:
    allowed_origins: ["https://example.com", "https://app.example.com"]
    allowed_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    allowed_headers: ["Content-Type", "Authorization", "X-Request-ID"]
    max_age: 86400
  
  rate_limiting:
    default_limit: 100
    window_size: 3600  # 1小时
    key_prefix: "rate_limit"
    storage: redis
```

## 完整代码

```python
import asyncio
import json
import time
import random
import base64
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any
from dataclasses import dataclass, asdict
from dmsc import DMSC, DMSCContext
from dmsc.security import (
    EncryptionManager, JWTManager, PasswordManager,
    InputValidator, SecurityMiddleware, RateLimiter
)


@dataclass
class UserCredentials:
    """用户凭据"""
    username: str
    email: str
    password: str
    full_name: Optional[str] = None


@dataclass
class EncryptedData:
    """加密数据结构"""
    ciphertext: str
    iv: str
    tag: str
    algorithm: str
    timestamp: datetime


@dataclass
class ApiRequest:
    """API请求数据"""
    endpoint: str
    method: str
    headers: Dict[str, str]
    body: Optional[Dict[str, Any]] = None
    query_params: Optional[Dict[str, str]] = None


class EncryptionExample:
    """加密解密示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.encryption = EncryptionManager()
        self.logger = ctx.logger.getChild("encryption_example")
    
    async def symmetric_encryption_demo(self):
        """对称加密示例"""
        self.logger.info("=== 对称加密示例 ===")
        
        # 生成密钥
        key = await self.encryption.generate_key(algorithm="AES-256-GCM")
        self.logger.info("generated_symmetric_key", key_length=len(key))
        
        # 敏感数据
        sensitive_data = {
            "credit_card": "1234-5678-9012-3456",
            "ssn": "123-45-6789",
            "api_key": "secret-api-key-12345"
        }
        
        # 加密数据
        plaintext = json.dumps(sensitive_data)
        encrypted = await self.encryption.encrypt(plaintext, key)
        
        self.logger.info("encrypted_data", 
                         algorithm=encrypted.algorithm,
                         size=len(encrypted.ciphertext))
        
        # 解密数据
        decrypted = await self.encryption.decrypt(encrypted, key)
        decrypted_data = json.loads(decrypted)
        
        self.logger.info("decrypted_data_success", original=sensitive_data == decrypted_data)
        
        # 批量加密
        data_list = [
            {"user_id": "user1", "token": "token123"},
            {"user_id": "user2", "token": "token456"},
            {"user_id": "user3", "token": "token789"}
        ]
        
        encrypted_batch = []
        for data in data_list:
            encrypted_item = await self.encryption.encrypt(
                json.dumps(data), key
            )
            encrypted_batch.append(encrypted_item)
        
        self.logger.info("batch_encrypted", count=len(encrypted_batch))
        
        # 批量解密
        decrypted_batch = []
        for encrypted_item in encrypted_batch:
            decrypted_item = await self.encryption.decrypt(encrypted_item, key)
            decrypted_batch.append(json.loads(decrypted_item))
        
        self.logger.info("batch_decrypted_success", count=len(decrypted_batch))
        
        return key, encrypted_batch
    
    async def asymmetric_encryption_demo(self):
        """非对称加密示例"""
        self.logger.info("=== 非对称加密示例 ===")
        
        # 生成密钥对
        private_key, public_key = await self.encryption.generate_key_pair(algorithm="RSA-2048")
        self.logger.info("generated_key_pair", private_key_size=len(private_key), public_key_size=len(public_key))
        
        # 要加密的数据
        message = "This is a secret message that needs to be encrypted"
        
        # 使用公钥加密
        encrypted_message = await self.encryption.encrypt_with_public_key(message, public_key)
        self.logger.info("encrypted_with_public_key", size=len(encrypted_message))
        
        # 使用私钥解密
        decrypted_message = await self.encryption.decrypt_with_private_key(encrypted_message, private_key)
        self.logger.info("decrypted_with_private_key", success=message == decrypted_message)
        
        # 数字签名
        signature = await self.encryption.sign(message, private_key)
        self.logger.info("message_signed", signature_size=len(signature))
        
        # 验证签名
        is_valid = await self.encryption.verify_signature(message, signature, public_key)
        self.logger.info("signature_verified", valid=is_valid)
        
        # 篡改消息后的签名验证
        tampered_message = message + " (tampered)"
        is_tampered_valid = await self.encryption.verify_signature(tampered_message, signature, public_key)
        self.logger.info("tampered_signature_check", valid=is_tampered_valid)
        
        return private_key, public_key
    
    async def key_derivation_demo(self):
        """密钥派生示例"""
        self.logger.info("=== 密钥派生示例 ===")
        
        # 用户密码
        password = "MySecurePassword123!"
        salt = await self.encryption.generate_salt()
        
        # 派生密钥
        derived_key = await self.encryption.derive_key(password, salt, iterations=100000)
        self.logger.info("derived_key_from_password", key_length=len(derived_key), salt_length=len(salt))
        
        # 使用派生密钥加密
        data = "Important data that needs encryption"
        encrypted = await self.encryption.encrypt(data, derived_key)
        
        # 使用相同密码和盐重新派生密钥进行解密
        same_key = await self.encryption.derive_key(password, salt, iterations=100000)
        decrypted = await self.encryption.decrypt(encrypted, same_key)
        
        self.logger.info("decryption_with_derived_key", success=data == decrypted)
        
        return derived_key, salt


class JWTExample:
    """JWT令牌示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.jwt = JWTManager()
        self.logger = ctx.logger.getChild("jwt_example")
    
    async def jwt_generation_demo(self):
        """JWT生成示例"""
        self.logger.info("=== JWT生成示例 ===")
        
        # 生成密钥对用于JWT签名
        private_key, public_key = await self.jwt.generate_key_pair(algorithm="RS256")
        
        # 用户声明
        claims = {
            "sub": "user123",
            "username": "john_doe",
            "email": "john@example.com",
            "roles": ["user", "admin"],
            "permissions": ["read", "write"],
            "iat": datetime.utcnow(),
            "exp": datetime.utcnow() + timedelta(hours=1),
            "iss": "dmsc-demo",
            "aud": "dmsc-users"
        }
        
        # 生成访问令牌
        access_token = await self.jwt.generate_token(claims, private_key)
        self.logger.info("generated_access_token", token_length=len(access_token))
        
        # 生成刷新令牌
        refresh_claims = {
            "sub": claims["sub"],
            "type": "refresh",
            "iat": datetime.utcnow(),
            "exp": datetime.utcnow() + timedelta(days=7),
            "iss": "dmsc-demo"
        }
        
        refresh_token = await self.jwt.generate_token(refresh_claims, private_key)
        self.logger.info("generated_refresh_token", token_length=len(refresh_token))
        
        # 验证令牌
        try:
            decoded_claims = await self.jwt.verify_token(access_token, public_key)
            self.logger.info("token_verified_successfully", user_id=decoded_claims.get("sub"))
        except Exception as e:
            self.logger.error("token_verification_failed", error=str(e))
        
        return access_token, refresh_token, private_key, public_key
    
    async def jwt_refresh_demo(self):
        """JWT刷新示例"""
        self.logger.info("=== JWT刷新示例 ===")
        
        # 生成密钥对
        private_key, public_key = await self.jwt.generate_key_pair(algorithm="RS256")
        
        # 原始令牌声明
        original_claims = {
            "sub": "user456",
            "username": "jane_doe",
            "iat": datetime.utcnow(),
            "exp": datetime.utcnow() + timedelta(minutes=30),  # 30分钟后过期
            "iss": "dmsc-demo"
        }
        
        # 生成原始令牌
        original_token = await self.jwt.generate_token(original_claims, private_key)
        
        # 模拟等待，检查令牌是否即将过期
        await asyncio.sleep(1)
        
        # 检查令牌是否需要刷新
        needs_refresh = await self.jwt.is_token_expiring_soon(original_token, threshold_minutes=5)
        self.logger.info("token_refresh_check", needs_refresh=needs_refresh)
        
        if needs_refresh:
            # 生成新令牌
            new_claims = original_claims.copy()
            new_claims["iat"] = datetime.utcnow()
            new_claims["exp"] = datetime.utcnow() + timedelta(hours=1)
            new_claims["refresh_count"] = original_claims.get("refresh_count", 0) + 1
            
            new_token = await self.jwt.generate_token(new_claims, private_key)
            self.logger.info("token_refreshed_successfully", new_token_length=len(new_token))
            
            return new_token
        
        return original_token
    
    async def jwt_validation_demo(self):
        """JWT验证示例"""
        self.logger.info("=== JWT验证示例 ===")
        
        private_key, public_key = await self.jwt.generate_key_pair(algorithm="RS256")
        
        # 生成测试令牌
        test_claims = {
            "sub": "test_user",
            "username": "tester",
            "roles": ["user"],
            "iat": datetime.utcnow(),
            "exp": datetime.utcnow() + timedelta(hours=1),
            "iss": "dmsc-demo"
        }
        
        token = await self.jwt.generate_token(test_claims, private_key)
        
        # 验证令牌结构
        is_valid = await self.jwt.validate_token_structure(token)
        self.logger.info("token_structure_valid", valid=is_valid)
        
        # 验证签名
        signature_valid = await self.jwt.verify_signature(token, public_key)
        self.logger.info("signature_valid", valid=signature_valid)
        
        # 验证过期时间
        try:
            decoded = await self.jwt.verify_token(token, public_key)
            self.logger.info("token_not_expired", exp=decoded.get("exp"))
        except Exception as e:
            self.logger.error("token_expired_or_invalid", error=str(e))
        
        # 验证发行者
        issuer_valid = await self.jwt.verify_issuer(token, "dmsc-demo", public_key)
        self.logger.info("issuer_valid", valid=issuer_valid)
        
        # 验证受众
        audience_valid = await self.jwt.verify_audience(token, ["dmsc-users"], public_key)
        self.logger.info("audience_valid", valid=audience_valid)
        
        # 验证角色权限
        has_role = await self.jwt.has_role(token, "user", public_key)
        has_admin_role = await self.jwt.has_role(token, "admin", public_key)
        self.logger.info("role_check", has_user_role=has_role, has_admin_role=has_admin_role)
        
        return token


class PasswordExample:
    """密码处理示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.password_manager = PasswordManager()
        self.logger = ctx.logger.getChild("password_example")
    
    async def password_hashing_demo(self):
        """密码哈希示例"""
        self.logger.info("=== 密码哈希示例 ===")
        
        # 用户密码
        passwords = [
            "MySecurePass123!",
            "AnotherStrong#Pass456",
            "WeakPass789",  # 这个会被验证失败
            "Complex$Pass123!@#"
        ]
        
        for password in passwords:
            self.logger.info("processing_password", password_length=len(password))
            
            # 验证密码强度
            is_valid, validation_errors = await self.password_manager.validate_password_strength(password)
            
            if is_valid:
                # 哈希密码
                hashed_password = await self.password_manager.hash_password(password)
                self.logger.info("password_hashed", hash_length=len(hashed_password))
                
                # 验证密码
                is_match = await self.password_manager.verify_password(password, hashed_password)
                self.logger.info("password_verification", success=is_match)
                
                # 验证错误密码
                wrong_password = password + "_wrong"
                is_wrong_match = await self.password_manager.verify_password(wrong_password, hashed_password)
                self.logger.info("wrong_password_check", success=not is_wrong_match)
                
            else:
                self.logger.warning("password_validation_failed", errors=validation_errors)
    
    async def password_policy_demo(self):
        """密码策略示例"""
        self.logger.info("=== 密码策略示例 ===")
        
        # 自定义密码策略
        custom_policy = {
            "min_length": 10,
            "max_length": 128,
            "require_uppercase": True,
            "require_lowercase": True,
            "require_numbers": True,
            "require_special_chars": True,
            "min_special_chars": 2,
            "forbidden_patterns": ["password", "123456", "qwerty"],
            "max_consecutive_chars": 3
        }
        
        test_passwords = [
            "Short1!",  # 太短
            "nouppercase123!",  # 没有大写字母
            "NOLOWERCASE123!",  # 没有小写字母
            "NoSpecialChars123",  # 没有特殊字符
            "ValidPass123!@#",  # 有效密码
            "password123!",  # 包含禁止模式
            "AAAbbb123!",  # 连续字符过多
        ]
        
        for password in test_passwords:
            is_valid, errors = await self.password_manager.validate_against_policy(password, custom_policy)
            self.logger.info("password_policy_check", 
                           password=password, 
                           valid=is_valid, 
                           errors=errors if not is_valid else None)
    
    async def secure_random_demo(self):
        """安全随机数生成示例"""
        self.logger.info("=== 安全随机数生成示例 ===")
        
        # 生成安全随机字节
        random_bytes = await self.password_manager.generate_secure_random(32)
        self.logger.info("generated_secure_bytes", length=len(random_bytes))
        
        # 生成安全令牌
        token = await self.password_manager.generate_secure_token(32)
        self.logger.info("generated_secure_token", token=token[:8] + "..." if len(token) > 8 else token)
        
        # 生成安全验证码
        verification_code = await self.password_manager.generate_verification_code(length=6, digits_only=True)
        self.logger.info("generated_verification_code", code=verification_code)
        
        # 生成安全密码
        secure_password = await self.password_manager.generate_secure_password(length=16)
        self.logger.info("generated_secure_password", password=secure_password[:8] + "...")
        
        # 生成UUID
        uuid4 = await self.password_manager.generate_uuid4()
        self.logger.info("generated_uuid4", uuid=uuid4)


class InputValidationExample:
    """输入验证示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.validator = InputValidator()
        self.logger = ctx.logger.getChild("validation_example")
    
    async def basic_validation_demo(self):
        """基础验证示例"""
        self.logger.info("=== 基础验证示例 ===")
        
        # 测试数据
        test_data = {
            "username": "john_doe123",
            "email": "john@example.com",
            "age": 25,
            "phone": "+1-555-123-4567",
            "website": "https://example.com",
            "credit_card": "1234-5678-9012-3456",
            "ssn": "123-45-6789"
        }
        
        # 验证规则
        validation_rules = {
            "username": {"type": "string", "min_length": 3, "max_length": 20, "pattern": r"^[a-zA-Z0-9_]+$"},
            "email": {"type": "email"},
            "age": {"type": "integer", "min": 18, "max": 120},
            "phone": {"type": "phone", "format": "international"},
            "website": {"type": "url"},
            "credit_card": {"type": "credit_card", "format": "visa"},
            "ssn": {"type": "ssn", "format": "us"}
        }
        
        # 执行验证
        is_valid, validation_results = await self.validator.validate_data(test_data, validation_rules)
        
        self.logger.info("validation_results", 
                        valid=is_valid, 
                        results={k: v for k, v in validation_results.items() if not v["valid"]})
        
        return validation_results
    
    async def sql_injection_demo(self):
        """SQL注入检测示例"""
        self.logger.info("=== SQL注入检测示例 ===")
        
        # 测试输入
        test_inputs = [
            "john@example.com",  # 正常输入
            "john@example.com' OR '1'='1",  # SQL注入
            "john@example.com'; DROP TABLE users; --",  # 恶意SQL
            "john@example.com UNION SELECT * FROM passwords",  # UNION注入
            "john@example.com AND 1=1",  # 布尔注入
        ]
        
        for user_input in test_inputs:
            is_safe, threats = await self.validator.check_sql_injection(user_input)
            self.logger.info("sql_injection_check", 
                            input=user_input[:30] + "..." if len(user_input) > 30 else user_input,
                            safe=is_safe, 
                            threats=threats if threats else None)
    
    async def xss_detection_demo(self):
        """XSS攻击检测示例"""
        self.logger.info("=== XSS攻击检测示例 ===")
        
        # 测试输入
        test_inputs = [
            "Hello World",  # 正常输入
            "<script>alert('XSS')</script>",  # 脚本注入
            "<img src='x' onerror='alert(1)'>",  # 事件处理器注入
            "javascript:alert('XSS')",  # JavaScript协议
            "<iframe src='malicious.com'></iframe>",  # iframe注入
            "<svg onload='alert(1)'></svg>",  # SVG注入
        ]
        
        for user_input in test_inputs:
            is_safe, threats = await self.validator.check_xss(user_input)
            self.logger.info("xss_check", 
                            input=user_input[:40] + "..." if len(user_input) > 40 else user_input,
                            safe=is_safe, 
                            threats=threats if threats else None)
    
    async def file_validation_demo(self):
        """文件验证示例"""
        self.logger.info("=== 文件验证示例 ===")
        
        # 测试文件信息
        test_files = [
            {"filename": "document.pdf", "size": 1024 * 1024, "content_type": "application/pdf"},
            {"filename": "image.jpg", "size": 500 * 1024, "content_type": "image/jpeg"},
            {"filename": "script.exe", "size": 2048 * 1024, "content_type": "application/x-msdownload"},
            {"filename": "malicious.php", "size": 10 * 1024, "content_type": "text/php"},
            {"filename": "huge_file.zip", "size": 50 * 1024 * 1024, "content_type": "application/zip"},
        ]
        
        # 验证规则
        file_rules = {
            "allowed_extensions": [".pdf", ".jpg", ".jpeg", ".png", ".txt"],
            "max_size": 10 * 1024 * 1024,  # 10MB
            "allowed_content_types": ["application/pdf", "image/jpeg", "image/png", "text/plain"]
        }
        
        for file_info in test_files:
            is_valid, errors = await self.validator.validate_file_upload(file_info, file_rules)
            self.logger.info("file_validation", 
                            filename=file_info["filename"],
                            valid=is_valid, 
                            errors=errors if errors else None)


class SecurityMiddlewareExample:
    """安全中间件示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.middleware = SecurityMiddleware()
        self.logger = ctx.logger.getChild("middleware_example")
    
    async def cors_demo(self):
        """CORS配置示例"""
        self.logger.info("=== CORS配置示例 ===")
        
        # CORS配置
        cors_config = {
            "allowed_origins": ["https://example.com", "https://app.example.com"],
            "allowed_methods": ["GET", "POST", "PUT", "DELETE", "OPTIONS"],
            "allowed_headers": ["Content-Type", "Authorization", "X-Request-ID"],
            "exposed_headers": ["X-Total-Count", "X-Page-Size"],
            "max_age": 86400,
            "allow_credentials": True
        }
        
        # 测试请求
        test_requests = [
            {
                "origin": "https://example.com",
                "method": "GET",
                "headers": {"Content-Type": "application/json"}
            },
            {
                "origin": "https://malicious.com",  # 不允许的来源
                "method": "POST",
                "headers": {"Content-Type": "application/json"}
            },
            {
                "origin": "https://app.example.com",
                "method": "PUT",
                "headers": {"Authorization": "Bearer token123"}
            }
        ]
        
        for request_data in test_requests:
            is_allowed, cors_headers = await self.middleware.check_cors_policy(request_data, cors_config)
            self.logger.info("cors_check", 
                            origin=request_data["origin"],
                            method=request_data["method"],
                            allowed=is_allowed,
                            headers=cors_headers if cors_headers else None)
    
    async def rate_limiting_demo(self):
        """速率限制示例"""
        self.logger.info("=== 速率限制示例 ===")
        
        # 创建速率限制器
        rate_limiter = RateLimiter(
            storage="redis",
            default_limit=10,  # 每小时10次请求
            window_size=3600
        )
        
        # 测试客户端
        test_clients = [
            {"client_id": "client_1", "endpoint": "/api/users"},
            {"client_id": "client_2", "endpoint": "/api/products"},
            {"client_id": "client_1", "endpoint": "/api/users"},  # 同一个客户端
        ]
        
        for i, client_data in enumerate(test_clients * 5):  # 每个客户端请求5次
            client_id = client_data["client_id"]
            endpoint = client_data["endpoint"]
            
            # 检查速率限制
            allowed, remaining, reset_time = await rate_limiter.check_limit(client_id, endpoint)
            
            self.logger.info("rate_limit_check",
                           attempt=i + 1,
                           client_id=client_id,
                           endpoint=endpoint,
                           allowed=allowed,
                           remaining=remaining,
                           reset_time=reset_time)
            
            if allowed:
                # 消耗一次请求
                await rate_limiter.consume(client_id, endpoint)
            
            await asyncio.sleep(0.1)
    
    async def security_headers_demo(self):
        """安全头部示例"""
        self.logger.info("=== 安全头部示例 ===")
        
        # 安全头部配置
        security_headers = {
            "X-Content-Type-Options": "nosniff",
            "X-Frame-Options": "DENY",
            "X-XSS-Protection": "1; mode=block",
            "Strict-Transport-Security": "max-age=31536000; includeSubDomains",
            "Content-Security-Policy": "default-src 'self'; script-src 'self' 'unsafe-inline'",
            "Referrer-Policy": "strict-origin-when-cross-origin",
            "Permissions-Policy": "geolocation=(), microphone=(), camera=()"
        }
        
        # 生成安全头部
        headers = await self.middleware.generate_security_headers(security_headers)
        
        self.logger.info("security_headers_generated", headers=headers)
        
        return headers


class DataMaskingExample:
    """数据脱敏示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.logger = ctx.logger.getChild("masking_example")
    
    async def sensitive_data_masking_demo(self):
        """敏感数据脱敏示例"""
        self.logger.info("=== 敏感数据脱敏示例 ===")
        
        # 测试数据
        sensitive_data = {
            "user_id": "user_12345",
            "username": "john_doe",
            "email": "john.doe@example.com",
            "phone": "+1-555-123-4567",
            "credit_card": "1234-5678-9012-3456",
            "ssn": "123-45-6789",
            "bank_account": "9876543210",
            "api_key": "sk-1234567890abcdef",
            "password": "MyPassword123!",
            "address": "123 Main St, City, State 12345"
        }
        
        # 脱敏规则
        masking_rules = {
            "email": {"type": "email", "show_first": 2, "show_last": 2},
            "phone": {"type": "phone", "show_first": 3, "show_last": 2},
            "credit_card": {"type": "credit_card", "show_first": 4, "show_last": 4},
            "ssn": {"type": "ssn", "show_last": 4},
            "bank_account": {"type": "bank_account", "show_last": 4},
            "api_key": {"type": "api_key", "show_first": 7, "show_last": 4},
            "password": {"type": "password", "mask_all": True},
            "address": {"type": "address", "show_first": 10}
        }
        
        # 执行脱敏
        masked_data = {}
        for field, value in sensitive_data.items():
            if field in masking_rules:
                rule = masking_rules[field]
                masked_value = await self.mask_field(value, rule)
                masked_data[field] = masked_value
            else:
                masked_data[field] = value
        
        self.logger.info("data_masking_completed", 
                        original_fields=list(sensitive_data.keys()),
                        masked_fields=list(masked_data.keys()))
        
        # 显示对比
        for field in masking_rules.keys():
            if field in sensitive_data:
                self.logger.info("masking_result",
                               field=field,
                               original=sensitive_data[field],
                               masked=masked_data[field])
        
        return masked_data
    
    async def mask_field(self, value: str, rule: Dict[str, Any]) -> str:
        """脱敏单个字段"""
        if rule.get("mask_all", False):
            return "*" * len(value)
        
        mask_type = rule.get("type", "custom")
        show_first = rule.get("show_first", 0)
        show_last = rule.get("show_last", 0)
        
        if len(value) <= show_first + show_last:
            return "*" * len(value)
        
        masked_middle = "*" * (len(value) - show_first - show_last)
        
        if show_first > 0 and show_last > 0:
            return value[:show_first] + masked_middle + value[-show_last:]
        elif show_first > 0:
            return value[:show_first] + masked_middle
        elif show_last > 0:
            return masked_middle + value[-show_last:]
        else:
            return "*" * len(value)


async def main():
    """主函数"""
    # 创建DMSC应用
    dmsc = DMSC()
    
    # 启动应用
    async with dmsc.run() as ctx:
        # 加密解密示例
        encryption = EncryptionExample(ctx)
        await encryption.symmetric_encryption_demo()
        await encryption.asymmetric_encryption_demo()
        await encryption.key_derivation_demo()
        
        # JWT示例
        jwt_example = JWTExample(ctx)
        await jwt_example.jwt_generation_demo()
        await jwt_example.jwt_refresh_demo()
        await jwt_example.jwt_validation_demo()
        
        # 密码示例
        password_example = PasswordExample(ctx)
        await password_example.password_hashing_demo()
        await password_example.password_policy_demo()
        await password_example.secure_random_demo()
        
        # 输入验证示例
        validation_example = InputValidationExample(ctx)
        await validation_example.basic_validation_demo()
        await validation_example.sql_injection_demo()
        await validation_example.xss_detection_demo()
        await validation_example.file_validation_demo()
        
        # 安全中间件示例
        middleware_example = SecurityMiddlewareExample(ctx)
        await middleware_example.cors_demo()
        await middleware_example.rate_limiting_demo()
        await middleware_example.security_headers_demo()
        
        # 数据脱敏示例
        masking_example = DataMaskingExample(ctx)
        await masking_example.sensitive_data_masking_demo()
        
        ctx.logger.info("security_demo", "所有安全示例执行完成！")


if __name__ == "__main__":
    # 运行应用
    asyncio.run(main())
```

## 运行示例

### 基本运行

```bash
python security_example.py
```

### 使用配置文件

```bash
python security_example.py --config config.yaml
```

### 后台运行

```bash
# 作为后台服务运行
nohup python security_example.py > security_demo.log 2>&1 &

# 查看日志
tail -f security_demo.log
```

## 关键特性

### 1. 加密解密
- 支持AES、RSA等多种算法
- 对称加密和非对称加密
- 密钥派生和管理
- 批量加密处理

### 2. JWT认证
- JWT令牌生成和验证
- 支持多种签名算法
- 令牌刷新机制
- 角色权限验证

### 3. 密码安全
- 安全哈希算法
- 密码强度验证
- 安全随机数生成
- 密码策略配置

### 4. 输入验证
- SQL注入检测
- XSS攻击防护
- 文件上传验证
- 数据格式验证

### 5. 安全中间件
- CORS策略管理
- 速率限制
- 安全头部生成
- 请求过滤

### 6. 数据保护
- 敏感数据脱敏
- 数据掩码处理
- 隐私信息保护
- 审计日志记录

## 最佳实践

### 1. 加密策略
- 使用强加密算法
- 安全密钥管理
- 定期密钥轮换
- 加密传输和存储

### 2. 认证授权
- 实施最小权限原则
- 使用JWT进行状态管理
- 定期令牌刷新
- 权限验证检查

### 3. 输入处理
- 验证所有用户输入
- 使用参数化查询
- 实施内容安全策略
- 文件上传限制

### 4. 安全配置
- 启用安全头部
- 配置CORS策略
- 实施速率限制
- 监控安全事件

## 故障排除

### 常见问题

1. **加密失败**
   - 检查密钥格式和长度
   - 验证加密算法支持
   - 确认数据编码格式

2. **JWT验证失败**
   - 检查密钥对匹配
   - 验证令牌过期时间
   - 确认发行者和受众

3. **密码验证失败**
   - 检查哈希算法一致性
   - 验证密码策略配置
   - 确认编码格式

4. **输入验证误报**
   - 调整验证规则
   - 检查正则表达式
   - 测试边界情况

### 调试技巧

```python
# 启用安全调试日志
import logging
logging.getLogger("dmsc.security").setLevel(logging.DEBUG)

# 检查加密配置
encryption_config = await ctx.security.get_encryption_config()
print(f"Default algorithm: {encryption_config['default_algorithm']}")

# 验证JWT配置
jwt_config = await ctx.security.get_jwt_config()
print(f"JWT algorithm: {jwt_config['algorithm']}")

# 测试密码策略
policy_result = await ctx.security.validate_password_policy("test_password")
print(f"Policy validation: {policy_result}")
```

## 相关链接

- [安全API参考](../04-api-reference/security.md)
- [认证使用示例](authentication.md)
- [配置管理使用示例](config.md)
- [DMSC官方文档](https://dmsc.org/docs)