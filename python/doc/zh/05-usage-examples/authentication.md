# 认证示例

本示例展示如何使用DMSC Python实现完整的认证系统，包括用户注册、登录、JWT令牌、权限验证等功能。

## 示例概述

我们将创建一个完整的认证系统，包含以下功能：
- 用户注册与密码加密
- JWT令牌生成与验证
- 角色权限管理
- 刷新令牌机制
- 双因子认证（2FA）
- OAuth第三方登录

## 完整代码示例

```python
from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCHTTPConfig, DMSCConfig, DMSCAuthConfig, DMSCError,
    DMSCAuthManager, DMSCAuthMiddleware
)
import asyncio
from datetime import datetime, timedelta
from typing import Dict, List, Optional
import hashlib
import secrets
import base64
from functools import wraps

# 模拟数据库
users_db = {}
sessions = {}
roles_db = {
    "admin": {"permissions": ["read", "write", "delete", "admin"]},
    "user": {"permissions": ["read", "write"]},
    "guest": {"permissions": ["read"]}
}
refresh_tokens = {}

# 用户模型
class User:
    def __init__(self, username: str, email: str, password_hash: str, salt: str):
        self.username = username
        self.email = email
        self.password_hash = password_hash
        self.salt = salt
        self.created_at = datetime.now()
        self.last_login = None
        self.role = "user"
        self.is_active = True
        self.two_factor_enabled = False
        self.two_factor_secret = None
    
    def to_dict(self, include_sensitive: bool = False) -> Dict:
        data = {
            "username": self.username,
            "email": self.email,
            "role": self.role,
            "is_active": self.is_active,
            "created_at": self.created_at.isoformat(),
            "last_login": self.last_login.isoformat() if self.last_login else None,
            "two_factor_enabled": self.two_factor_enabled
        }
        
        if include_sensitive:
            data["password_hash"] = self.password_hash
            data["salt"] = self.salt
            data["two_factor_secret"] = self.two_factor_secret
        
        return data

# 认证服务
class AuthenticationService:
    def __init__(self, context: DMSCServiceContext):
        self.context = context
        self.logger = context.logger
        self.config = context.config
        self.auth_manager = context.auth if hasattr(context, 'auth') else None
    
    def _hash_password(self, password: str, salt: str) -> str:
        """密码哈希"""
        return hashlib.pbkdf2_hmac('sha256', password.encode('utf-8'), salt.encode('utf-8'), 100000).hex()
    
    def _generate_salt(self) -> str:
        """生成盐值"""
        return secrets.token_hex(32)
    
    def _generate_jwt_token(self, username: str, role: str, expires_in: int = 3600) -> str:
        """生成JWT令牌"""
        import json
        import base64
        
        header = {
            "alg": "HS256",
            "typ": "JWT"
        }
        
        payload = {
            "username": username,
            "role": role,
            "exp": int((datetime.now() + timedelta(seconds=expires_in)).timestamp()),
            "iat": int(datetime.now().timestamp())
        }
        
        # 简化的JWT实现（实际应用中应该使用专业的JWT库）
        header_encoded = base64.urlsafe_b64encode(json.dumps(header).encode()).decode().rstrip('=')
        payload_encoded = base64.urlsafe_b64encode(json.dumps(payload).encode()).decode().rstrip('=')
        
        secret = self.config.get("auth.jwt_secret", "default_secret_key")
        signature_input = f"{header_encoded}.{payload_encoded}"
        signature = base64.urlsafe_b64encode(
            hashlib.sha256((signature_input + secret).encode()).digest()
        ).decode().rstrip('=')
        
        return f"{header_encoded}.{payload_encoded}.{signature}"
    
    def _verify_jwt_token(self, token: str) -> Optional[Dict]:
        """验证JWT令牌"""
        try:
            parts = token.split('.')
            if len(parts) != 3:
                return None
            
            import json
            
            # 解码payload
            payload_encoded = parts[1]
            # 添加填充
            payload_encoded += '=' * (4 - len(payload_encoded) % 4)
            payload = json.loads(base64.urlsafe_b64decode(payload_encoded))
            
            # 检查过期时间
            if payload.get("exp", 0) < int(datetime.now().timestamp()):
                return None
            
            # 验证签名（简化版）
            secret = self.config.get("auth.jwt_secret", "default_secret_key")
            signature_input = f"{parts[0]}.{parts[1]}"
            expected_signature = base64.urlsafe_b64encode(
                hashlib.sha256((signature_input + secret).encode()).digest()
            ).decode().rstrip('=')
            
            if parts[2] != expected_signature:
                return None
            
            return payload
            
        except Exception as e:
            self.logger.error("auth_service", "JWT verification failed", error=str(e))
            return None
    
    async def register_user(self, username: str, email: str, password: str, 
                          role: str = "user") -> Dict:
        """注册用户"""
        self.logger.info("auth_service", "Registering new user", 
                       username=username, email=email, role=role)
        
        # 检查用户名是否已存在
        if username in users_db:
            self.logger.warn("auth_service", "Username already exists", username=username)
            raise DMSCError("USER_EXISTS", f"Username {username} already exists")
        
        # 检查邮箱是否已存在
        for user in users_db.values():
            if user.email == email:
                self.logger.warn("auth_service", "Email already exists", email=email)
                raise DMSCError("EMAIL_EXISTS", f"Email {email} already exists")
        
        # 生成盐值并哈希密码
        salt = self._generate_salt()
        password_hash = self._hash_password(password, salt)
        
        # 创建新用户
        user = User(username, email, password_hash, salt)
        user.role = role
        users_db[username] = user
        
        self.logger.info("auth_service", "User registered successfully", 
                       username=username, role=role)
        
        return user.to_dict()
    
    async def login_user(self, username: str, password: str, 
                        two_factor_code: Optional[str] = None) -> Dict:
        """用户登录"""
        self.logger.info("auth_service", "User login attempt", username=username)
        
        # 检查用户是否存在
        if username not in users_db:
            self.logger.warn("auth_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        user = users_db[username]
        
        # 检查用户是否激活
        if not user.is_active:
            self.logger.warn("auth_service", "User account inactive", username=username)
            raise DMSCError("ACCOUNT_INACTIVE", "User account is inactive")
        
        # 验证密码
        password_hash = self._hash_password(password, user.salt)
        if password_hash != user.password_hash:
            self.logger.warn("auth_service", "Invalid password", username=username)
            raise DMSCError("INVALID_PASSWORD", "Invalid password")
        
        # 双因子认证检查
        if user.two_factor_enabled:
            if not two_factor_code:
                self.logger.warn("auth_service", "2FA code required", username=username)
                raise DMSCError("2FA_REQUIRED", "Two-factor authentication code required")
            
            # 这里应该验证2FA代码，简化处理
            if not self._verify_two_factor_code(user, two_factor_code):
                self.logger.warn("auth_service", "Invalid 2FA code", username=username)
                raise DMSCError("INVALID_2FA_CODE", "Invalid two-factor authentication code")
        
        # 更新最后登录时间
        user.last_login = datetime.now()
        
        # 生成访问令牌
        access_token = self._generate_jwt_token(username, user.role, expires_in=3600)
        
        # 生成刷新令牌
        refresh_token = secrets.token_urlsafe(32)
        refresh_tokens[refresh_token] = {
            "username": username,
            "created_at": datetime.now(),
            "expires_at": datetime.now() + timedelta(days=7)
        }
        
        self.logger.info("auth_service", "User logged in successfully", 
                       username=username, role=user.role)
        
        return {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "token_type": "Bearer",
            "expires_in": 3600,
            "user": user.to_dict()
        }
    
    def _verify_two_factor_code(self, user: User, code: str) -> bool:
        """验证双因子认证代码（简化版）"""
        # 这里应该实现真实的2FA验证逻辑
        # 现在只是模拟验证
        return len(code) == 6 and code.isdigit()
    
    async def refresh_token(self, refresh_token: str) -> Dict:
        """刷新访问令牌"""
        self.logger.info("auth_service", "Token refresh attempt")
        
        # 验证刷新令牌
        if refresh_token not in refresh_tokens:
            self.logger.warn("auth_service", "Invalid refresh token")
            raise DMSCError("INVALID_REFRESH_TOKEN", "Invalid refresh token")
        
        token_info = refresh_tokens[refresh_token]
        
        # 检查刷新令牌是否过期
        if token_info["expires_at"] < datetime.now():
            del refresh_tokens[refresh_token]
            self.logger.warn("auth_service", "Refresh token expired")
            raise DMSCError("REFRESH_TOKEN_EXPIRED", "Refresh token has expired")
        
        username = token_info["username"]
        
        # 检查用户是否存在且激活
        if username not in users_db or not users_db[username].is_active:
            del refresh_tokens[refresh_token]
            self.logger.warn("auth_service", "User not found or inactive", username=username)
            raise DMSCError("USER_NOT_FOUND", "User not found or inactive")
        
        user = users_db[username]
        
        # 生成新的访问令牌
        access_token = self._generate_jwt_token(username, user.role, expires_in=3600)
        
        self.logger.info("auth_service", "Token refreshed successfully", username=username)
        
        return {
            "access_token": access_token,
            "token_type": "Bearer",
            "expires_in": 3600,
            "user": user.to_dict()
        }
    
    async def logout_user(self, access_token: str) -> bool:
        """用户登出"""
        self.logger.info("auth_service", "User logout attempt")
        
        # 验证访问令牌
        payload = self._verify_jwt_token(access_token)
        if not payload:
            self.logger.warn("auth_service", "Invalid access token")
            return False
        
        username = payload.get("username")
        
        # 删除相关的刷新令牌
        tokens_to_remove = []
        for token, info in refresh_tokens.items():
            if info["username"] == username:
                tokens_to_remove.append(token)
        
        for token in tokens_to_remove:
            del refresh_tokens[token]
        
        self.logger.info("auth_service", "User logged out successfully", username=username)
        return True
    
    async def enable_two_factor(self, username: str) -> Dict:
        """启用双因子认证"""
        self.logger.info("auth_service", "Enabling 2FA", username=username)
        
        if username not in users_db:
            self.logger.warn("auth_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        user = users_db[username]
        
        # 生成2FA密钥（简化版）
        secret = base64.b32encode(secrets.token_bytes(20)).decode()
        user.two_factor_secret = secret
        user.two_factor_enabled = True
        
        self.logger.info("auth_service", "2FA enabled successfully", username=username)
        
        return {
            "secret": secret,
            "qr_code_url": f"otpauth://totp/DMSC:{username}?secret={secret}&issuer=DMSC"
        }
    
    async def disable_two_factor(self, username: str, two_factor_code: str) -> bool:
        """禁用双因子认证"""
        self.logger.info("auth_service", "Disabling 2FA", username=username)
        
        if username not in users_db:
            self.logger.warn("auth_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        user = users_db[username]
        
        if not user.two_factor_enabled:
            self.logger.warn("auth_service", "2FA not enabled", username=username)
            return True
        
        # 验证2FA代码
        if not self._verify_two_factor_code(user, two_factor_code):
            self.logger.warn("auth_service", "Invalid 2FA code", username=username)
            raise DMSCError("INVALID_2FA_CODE", "Invalid two-factor authentication code")
        
        user.two_factor_enabled = False
        user.two_factor_secret = None
        
        self.logger.info("auth_service", "2FA disabled successfully", username=username)
        return True
    
    async def assign_role(self, username: str, role: str) -> bool:
        """分配角色"""
        self.logger.info("auth_service", "Assigning role", username=username, role=role)
        
        if username not in users_db:
            self.logger.warn("auth_service", "User not found", username=username)
            raise DMSCError("USER_NOT_FOUND", f"User {username} not found")
        
        if role not in roles_db:
            self.logger.warn("auth_service", "Role not found", role=role)
            raise DMSCError("ROLE_NOT_FOUND", f"Role {role} not found")
        
        user = users_db[username]
        old_role = user.role
        user.role = role
        
        self.logger.info("auth_service", "Role assigned successfully", 
                       username=username, old_role=old_role, new_role=role)
        
        return True
    
    async def get_user_permissions(self, username: str) -> List[str]:
        """获取用户权限"""
        if username not in users_db:
            return []
        
        user = users_db[username]
        role_info = roles_db.get(user.role, {})
        return role_info.get("permissions", [])
    
    async def has_permission(self, username: str, permission: str) -> bool:
        """检查用户权限"""
        permissions = await self.get_user_permissions(username)
        return permission in permissions
    
    async def oauth_login(self, provider: str, code: str) -> Dict:
        """OAuth登录（模拟）"""
        self.logger.info("auth_service", "OAuth login attempt", provider=provider)
        
        # 这里应该实现真实的OAuth流程
        # 现在只是模拟OAuth登录
        
        if provider not in ["google", "github", "facebook"]:
            self.logger.warn("auth_service", "Unsupported OAuth provider", provider=provider)
            raise DMSCError("UNSUPPORTED_OAUTH", f"Unsupported OAuth provider: {provider}")
        
        # 模拟OAuth用户数据
        oauth_users = {
            "google": {"username": "google_user", "email": "google@example.com"},
            "github": {"username": "github_user", "email": "github@example.com"},
            "facebook": {"username": "facebook_user", "email": "facebook@example.com"}
        }
        
        user_info = oauth_users.get(provider)
        if not user_info:
            self.logger.warn("auth_service", "OAuth authentication failed", provider=provider)
            raise DMSCError("OAUTH_FAILED", "OAuth authentication failed")
        
        username = user_info["username"]
        email = user_info["email"]
        
        # 如果用户不存在，创建新用户
        if username not in users_db:
            # 生成随机密码（OAuth用户不需要密码）
            random_password = secrets.token_urlsafe(32)
            salt = self._generate_salt()
            password_hash = self._hash_password(random_password, salt)
            
            user = User(username, email, password_hash, salt)
            user.role = "user"
            users_db[username] = user
            
            self.logger.info("auth_service", "New OAuth user created", username=username, provider=provider)
        
        # 生成令牌
        access_token = self._generate_jwt_token(username, "user", expires_in=3600)
        refresh_token = secrets.token_urlsafe(32)
        refresh_tokens[refresh_token] = {
            "username": username,
            "created_at": datetime.now(),
            "expires_at": datetime.now() + timedelta(days=7)
        }
        
        self.logger.info("auth_service", "OAuth login successful", username=username, provider=provider)
        
        return {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "token_type": "Bearer",
            "expires_in": 3600,
            "user": users_db[username].to_dict()
        }

# 认证装饰器
def require_auth(func):
    """要求认证的装饰器"""
    @wraps(func)
    async def wrapper(self, request, *args, **kwargs):
        # 从请求头获取令牌
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return {"error": "Missing or invalid authorization header"}, 401
        
        token = auth_header[7:]  # 移除 "Bearer "
        
        # 验证令牌
        auth_service = request.app.auth_service
        payload = auth_service._verify_jwt_token(token)
        
        if not payload:
            return {"error": "Invalid or expired token"}, 401
        
        # 将用户信息添加到请求
        request.user = payload
        
        return await func(self, request, *args, **kwargs)
    
    return wrapper

def require_permission(permission: str):
    """要求特定权限的装饰器"""
    def decorator(func):
        @wraps(func)
        async def wrapper(self, request, *args, **kwargs):
            if not hasattr(request, 'user'):
                return {"error": "Authentication required"}, 401
            
            username = request.user.get("username")
            auth_service = request.app.auth_service
            
            if not await auth_service.has_permission(username, permission):
                return {"error": f"Permission '{permission}' required"}, 403
            
            return await func(self, request, *args, **kwargs)
        
        return wrapper
    return decorator

# HTTP处理器
class AuthHandler:
    def __init__(self, auth_service: AuthenticationService):
        self.auth_service = auth_service
    
    async def register_handler(self, request):
        """用户注册处理器"""
        try:
            data = await request.json()
            username = data.get("username")
            email = data.get("email")
            password = data.get("password")
            role = data.get("role", "user")
            
            if not all([username, email, password]):
                return {
                    "error": "Missing required fields",
                    "required": ["username", "email", "password"]
                }, 400
            
            # 验证角色
            if role not in ["admin", "user", "guest"]:
                return {
                    "error": "Invalid role",
                    "allowed_roles": ["admin", "user", "guest"]
                }, 400
            
            user_data = await self.auth_service.register_user(username, email, password, role)
            
            return {
                "success": True,
                "message": "User registered successfully",
                "data": user_data
            }, 201
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def login_handler(self, request):
        """用户登录处理器"""
        try:
            data = await request.json()
            username = data.get("username")
            password = data.get("password")
            two_factor_code = data.get("two_factor_code")
            
            if not all([username, password]):
                return {
                    "error": "Missing required fields",
                    "required": ["username", "password"]
                }, 400
            
            auth_result = await self.auth_service.login_user(username, password, two_factor_code)
            
            return {
                "success": True,
                "message": "Login successful",
                "data": auth_result
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 401
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def refresh_token_handler(self, request):
        """令牌刷新处理器"""
        try:
            data = await request.json()
            refresh_token = data.get("refresh_token")
            
            if not refresh_token:
                return {
                    "error": "Missing refresh_token"
                }, 400
            
            auth_result = await self.auth_service.refresh_token(refresh_token)
            
            return {
                "success": True,
                "message": "Token refreshed successfully",
                "data": auth_result
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 401
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def logout_handler(self, request):
        """用户登出处理器"""
        try:
            # 从请求头获取令牌
            auth_header = request.headers.get("Authorization", "")
            if not auth_header.startswith("Bearer "):
                return {
                    "error": "Missing or invalid authorization header"
                }, 401
            
            access_token = auth_header[7:]
            
            success = await self.auth_service.logout_user(access_token)
            
            if success:
                return {
                    "success": True,
                    "message": "Logout successful"
                }, 200
            else:
                return {
                    "error": "Invalid access token"
                }, 401
                
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def enable_two_factor_handler(self, request):
        """启用双因子认证处理器"""
        try:
            # 从请求头获取令牌
            auth_header = request.headers.get("Authorization", "")
            if not auth_header.startswith("Bearer "):
                return {
                    "error": "Missing or invalid authorization header"
                }, 401
            
            access_token = auth_header[7:]
            
            # 验证令牌
            payload = self.auth_service._verify_jwt_token(access_token)
            if not payload:
                return {
                    "error": "Invalid or expired token"
                }, 401
            
            username = payload.get("username")
            
            result = await self.auth_service.enable_two_factor(username)
            
            return {
                "success": True,
                "message": "Two-factor authentication enabled successfully",
                "data": result
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def disable_two_factor_handler(self, request):
        """禁用双因子认证处理器"""
        try:
            # 从请求头获取令牌
            auth_header = request.headers.get("Authorization", "")
            if not auth_header.startswith("Bearer "):
                return {
                    "error": "Missing or invalid authorization header"
                }, 401
            
            access_token = auth_header[7:]
            
            # 验证令牌
            payload = self.auth_service._verify_jwt_token(access_token)
            if not payload:
                return {
                    "error": "Invalid or expired token"
                }, 401
            
            username = payload.get("username")
            
            data = await request.json()
            two_factor_code = data.get("two_factor_code")
            
            if not two_factor_code:
                return {
                    "error": "Missing two_factor_code"
                }, 400
            
            success = await self.auth_service.disable_two_factor(username, two_factor_code)
            
            return {
                "success": True,
                "message": "Two-factor authentication disabled successfully"
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    async def oauth_login_handler(self, request):
        """OAuth登录处理器"""
        try:
            data = await request.json()
            provider = data.get("provider")
            code = data.get("code")
            
            if not all([provider, code]):
                return {
                    "error": "Missing required fields",
                    "required": ["provider", "code"]
                }, 400
            
            auth_result = await self.auth_service.oauth_login(provider, code)
            
            return {
                "success": True,
                "message": "OAuth login successful",
                "data": auth_result
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    @require_auth
    async def get_profile_handler(self, request):
        """获取用户资料处理器"""
        try:
            username = request.user.get("username")
            
            if username not in users_db:
                return {
                    "error": "User not found"
                }, 404
            
            user = users_db[username]
            
            return {
                "success": True,
                "data": user.to_dict()
            }, 200
            
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500
    
    @require_auth
    @require_permission("admin")
    async def assign_role_handler(self, request):
        """分配角色处理器（需要admin权限）"""
        try:
            data = await request.json()
            username = data.get("username")
            role = data.get("role")
            
            if not all([username, role]):
                return {
                    "error": "Missing required fields",
                    "required": ["username", "role"]
                }, 400
            
            success = await self.auth_service.assign_role(username, role)
            
            return {
                "success": True,
                "message": f"Role '{role}' assigned to user '{username}' successfully"
            }, 200
            
        except DMSCError as e:
            return {
                "error": e.message,
                "code": e.code
            }, 400
        except Exception as e:
            return {
                "error": "Internal server error",
                "message": str(e)
            }, 500

# 主应用函数
async def main(context: DMSCServiceContext):
    """主应用函数"""
    logger = context.logger
    logger.info("app", "Starting authentication system API")
    
    # 创建认证服务
    auth_service = AuthenticationService(context)
    auth_handler = AuthHandler(auth_service)
    
    # 将认证服务存储在应用中，供装饰器使用
    context.app.auth_service = auth_service
    
    # 获取HTTP服务器
    http = context.http
    
    # 注册认证相关路由
    http.add_route("POST", "/api/auth/register", auth_handler.register_handler)
    http.add_route("POST", "/api/auth/login", auth_handler.login_handler)
    http.add_route("POST", "/api/auth/refresh", auth_handler.refresh_token_handler)
    http.add_route("POST", "/api/auth/logout", auth_handler.logout_handler)
    http.add_route("POST", "/api/auth/enable-2fa", auth_handler.enable_two_factor_handler)
    http.add_route("POST", "/api/auth/disable-2fa", auth_handler.disable_two_factor_handler)
    http.add_route("POST", "/api/auth/oauth/login", auth_handler.oauth_login_handler)
    http.add_route("GET", "/api/auth/profile", auth_handler.get_profile_handler)
    http.add_route("POST", "/api/auth/assign-role", auth_handler.assign_role_handler)
    
    # 添加健康检查
    async def health_check(request):
        return {
            "status": "healthy",
            "timestamp": datetime.now().isoformat(),
            "service": "authentication-api",
            "version": "1.0.0",
            "features": [
                "user_registration",
                "user_login",
                "jwt_tokens",
                "refresh_tokens",
                "two_factor_auth",
                "oauth_login",
                "role_based_access",
                "permissions"
            ]
        }, 200
    
    http.add_route("GET", "/health", health_check)
    
    # 添加根路径
    async def root_handler(request):
        return {
            "message": "Welcome to DMSC Authentication System API",
            "version": "1.0.0",
            "endpoints": [
                "POST /api/auth/register - 用户注册",
                "POST /api/auth/login - 用户登录",
                "POST /api/auth/refresh - 令牌刷新",
                "POST /api/auth/logout - 用户登出",
                "POST /api/auth/enable-2fa - 启用双因子认证",
                "POST /api/auth/disable-2fa - 禁用双因子认证",
                "POST /api/auth/oauth/login - OAuth登录",
                "GET /api/auth/profile - 获取用户资料（需要认证）",
                "POST /api/auth/assign-role - 分配角色（需要admin权限）",
                "GET /health - 健康检查"
            ],
            "authentication_flows": [
                "Basic Authentication (username/password)",
                "JWT Token-based Authentication",
                "Two-Factor Authentication (2FA)",
                "OAuth Integration (Google, GitHub, Facebook)"
            ]
        }, 200
    
    http.add_route("GET", "/", root_handler)
    
    logger.info("app", "Authentication API routes registered successfully")
    logger.info("app", "Authentication system API is ready")
    
    # 创建一些测试用户
    await create_test_users(auth_service, logger)
    
    # 保持应用运行
    logger.info("app", "Application is running. Press Ctrl+C to stop.")
    try:
        await asyncio.Event().wait()
    except KeyboardInterrupt:
        logger.info("app", "Application stopped by user")

async def create_test_users(auth_service: AuthenticationService, logger):
    """创建测试用户"""
    try:
        # 创建管理员用户
        await auth_service.register_user("admin", "admin@example.com", "admin123", "admin")
        logger.info("app", "Test admin user created")
        
        # 创建普通用户
        await auth_service.register_user("user1", "user1@example.com", "user123", "user")
        logger.info("app", "Test user created")
        
        # 创建访客用户
        await auth_service.register_user("guest", "guest@example.com", "guest123", "guest")
        logger.info("app", "Test guest user created")
        
    except DMSCError as e:
        logger.info("app", "Test users may already exist", error=e.message)

# 创建和运行应用
async def create_and_run_app():
    """创建和运行应用"""
    
    # 创建日志配置
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True,
        enable_file=True,
        file_path="logs/auth-app.log"
    )
    
    # 创建HTTP配置
    http_config = DMSCHTTPConfig(
        host="0.0.0.0",
        port=8081,
        cors_enabled=True,
        cors_origins=["*"]
    )
    
    # 创建认证配置
    auth_config = DMSCAuthConfig(
        auth_type="jwt",
        secret_key="your-secret-key-here-make-it-long-and-random",
        algorithm="HS256",
        token_expiry=3600,
        refresh_token_expiry=86400 * 7,  # 7 days
        enable_refresh_token=True,
        enable_multi_device=True,
        max_devices_per_user=5,
        enable_rate_limit=True,
        rate_limit_attempts=5,
        rate_limit_window=300,
        enable_2fa=True,
        enable_oauth=True,
        enable_permissions=True
    )
    
    # 创建应用配置
    app_config = DMSCConfig()
    app_config.set("auth.jwt_secret", "your-secret-key-here-make-it-long-and-random")
    
    # 创建应用构建器
    app_builder = DMSCAppBuilder()
    
    # 配置应用
    app = (app_builder
           .with_logging(log_config)
           .with_config(app_config)
           .with_http(http_config)
           .with_auth(auth_config)
           .build())
    
    # 运行应用
    await app.run_async(main)

# 运行应用
if __name__ == "__main__":
    asyncio.run(create_and_run_app())
```

## 运行示例

### 1. 安装依赖

```bash
pip install dmsc-python
```

### 2. 运行应用

```bash
python auth_app.py
```

### 3. 测试API

#### 用户注册

```bash
curl -X POST http://localhost:8081/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "email": "test@example.com",
    "password": "SecurePass123!",
    "role": "user"
  }'
```

#### 用户登录

```bash
curl -X POST http://localhost:8081/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "password": "SecurePass123!"
  }'
```

#### 获取用户资料（需要认证）

```bash
curl -X GET http://localhost:8081/api/auth/profile \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

#### 启用双因子认证

```bash
curl -X POST http://localhost:8081/api/auth/enable-2fa \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
```

#### 分配角色（需要admin权限）

```bash
curl -X POST http://localhost:8081/api/auth/assign-role \
  -H "Authorization: Bearer ADMIN_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "username": "test_user",
    "role": "admin"
  }'
```

#### OAuth登录

```bash
curl -X POST http://localhost:8081/api/auth/oauth/login \
  -H "Content-Type: application/json" \
  -d '{
    "provider": "google",
    "code": "oauth_authorization_code"
  }'
```

## 关键特性

### 1. 多层认证
- 基础用户名/密码认证
- JWT令牌认证
- 双因子认证（2FA）
- OAuth第三方登录

### 2. 权限管理
- 基于角色的访问控制（RBAC）
- 细粒度权限验证
- 动态角色分配

### 3. 安全特性
- 密码哈希存储（PBKDF2）
- JWT令牌签名验证
- 刷新令牌机制
- 速率限制
- 会话管理

### 4. 可扩展性
- 模块化设计
- 装饰器模式
- 异步支持
- 配置驱动

## 扩展建议

### 1. 添加数据库支持
```python
# 替换内存存储为数据库存储
app_builder.with_database(database_config)
```

### 2. 添加缓存支持
```python
# 添加缓存模块用于会话和令牌缓存
app_builder.with_cache(cache_config)
```

### 3. 添加邮件服务
```python
# 添加邮件服务用于密码重置和2FA
app_builder.with_email(email_config)
```

### 4. 添加审计日志
```python
# 添加审计日志记录所有认证事件
app_builder.with_audit_logging(audit_config)
```

## 注意事项

1. **安全性**: 示例中的JWT实现是简化的，实际应用中应该使用专业的JWT库
2. **密码策略**: 应该实施更强的密码策略和验证
3. **2FA实现**: 示例中的2FA是简化的，实际应用中应该使用标准的TOTP实现
4. **OAuth集成**: 需要配置真实的OAuth客户端凭据
5. **会话管理**: 应该实施更完善的会话管理和清理机制
6. **错误处理**: 可以根据需要添加更详细的错误处理和日志记录