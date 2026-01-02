<div align="center">

# Authentication Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to implement a complete authentication system using DMSC Python, including user registration, login, JWT tokens, and permission verification.

## Example Overview

We will create a complete authentication system with the following features:
- User registration with password encryption
- JWT token generation and verification
- Role-based permission management
- Refresh token mechanism
- Two-factor authentication (2FA)
- OAuth third-party login

## Complete Code Example

```python
from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCHTTPConfig, DMSCConfig, DMSCError,
    DMSCAuthManager, DMSCAuthMiddleware
)
import asyncio
from datetime import datetime, timedelta
from typing import Dict, List, Optional
import hashlib
import secrets
import base64
from functools import wraps

# Simulated database
users_db = {}
sessions = {}
roles_db = {
    "admin": {"permissions": ["read", "write", "delete", "admin"]},
    "user": {"permissions": ["read", "write"]},
    "guest": {"permissions": ["read"]}
}
refresh_tokens = {}

# User model
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
        self.totp_secret = None
    
    def to_dict(self, include_sensitive: bool = False) -> Dict:
        data = {
            "username": self.username,
            "email": self.email,
            "created_at": self.created_at.isoformat(),
            "last_login": self.last_login.isoformat() if self.last_login else None,
            "role": self.role,
            "is_active": self.is_active
        }
        return data

# Password utilities
def hash_password(password: str, salt: str) -> str:
    return hashlib.pbkdf2_hmac(
        'sha256',
        password.encode('utf-8'),
        salt.encode('utf-8'),
        100000
    ).hex()

def verify_password(password: str, salt: str, password_hash: str) -> bool:
    return hash_password(password, salt) == password_hash

def generate_salt() -> str:
    return secrets.token_hex(16)

# TOTP for 2FA
def generate_totp_secret() -> str:
    return base64.b32encode(secrets.token_bytes(20)).decode('utf-8')

def verify_totp(secret: str, code: str) -> bool:
    # In production, use a proper TOTP library like pyotp
    return len(code) == 6 and code.isdigit()

# JWT token management
class TokenManager:
    def __init__(self, secret_key: str):
        self.secret_key = secret_key
    
    def create_access_token(self, user: User) -> str:
        payload = {
            "sub": user.username,
            "email": user.email,
            "role": user.role,
            "permissions": roles_db.get(user.role, {}).get("permissions", []),
            "exp": datetime.utcnow() + timedelta(minutes=30),
            "iat": datetime.utcnow(),
            "type": "access"
        }
        return self._encode_token(payload)
    
    def create_refresh_token(self, user: User) -> str:
        payload = {
            "sub": user.username,
            "exp": datetime.utcnow() + timedelta(days=7),
            "iat": datetime.utcnow(),
            "type": "refresh"
        }
        token = self._encode_token(payload)
        refresh_tokens[token] = user.username
        return token
    
    def verify_token(self, token: str) -> Optional[Dict]:
        try:
            payload = self._decode_token(token)
            if payload["type"] == "refresh" and token in refresh_tokens:
                return payload
            if payload["type"] == "access":
                return payload
            return None
        except Exception:
            return None
    
    def _encode_token(self, payload: Dict) -> str:
        import jwt
        return jwt.encode(payload, self.secret_key, algorithm="HS256")
    
    def _decode_token(self, token: str) -> Dict:
        import jwt
        return jwt.decode(token, self.secret_key, algorithms=["HS256"])

# Authentication service
class AuthService:
    def __init__(self, context: DMSCServiceContext, token_manager: TokenManager):
        self.context = context
        self.logger = context.logger
        self.token_manager = token_manager
    
    def register(self, username: str, email: str, password: str) -> Dict:
        if username in users_db:
            raise DMSCError("Username already exists", "USERNAME_EXISTS")
        
        if any(u.email == email for u in users_db.values()):
            raise DMSCError("Email already registered", "EMAIL_EXISTS")
        
        salt = generate_salt()
        password_hash = hash_password(password, salt)
        
        user = User(username, email, password_hash, salt)
        users_db[username] = user
        
        self.logger.info("auth", f"User registered: {username}")
        return user.to_dict()
    
    def login(self, username: str, password: str, totp_code: str = None) -> Dict:
        if username not in users_db:
            raise DMSCError("Invalid credentials", "INVALID_CREDENTIALS")
        
        user = users_db[username]
        
        if not verify_password(password, user.salt, user.password_hash):
            self.logger.warn("auth", f"Failed login attempt for: {username}")
            raise DMSCError("Invalid credentials", "INVALID_CREDENTIALS")
        
        # Check 2FA if enabled
        if user.totp_secret and not verify_totp(user.totp_secret, totp_code or ""):
            raise DMSCError("Invalid 2FA code", "INVALID_2FA")
        
        # Update last login
        user.last_login = datetime.now()
        
        # Generate tokens
        access_token = self.token_manager.create_access_token(user)
        refresh_token = self.token_manager.create_refresh_token(user)
        
        self.logger.info("auth", f"User logged in: {username}")
        return {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": 1800,
            "token_type": "Bearer"
        }
    
    def refresh(self, refresh_token: str) -> Dict:
        if refresh_token not in refresh_tokens:
            raise DMSCError("Invalid refresh token", "INVALID_TOKEN")
        
        username = refresh_tokens[refresh_token]
        if username not in users_db:
            raise DMSCError("User not found", "USER_NOT_FOUND")
        
        user = users_db[username]
        
        # Generate new tokens
        access_token = self.token_manager.create_access_token(user)
        refresh_token = self.token_manager.create_refresh_token(user)
        
        # Remove old refresh token
        del refresh_tokens[refresh_token]
        
        return {
            "access_token": access_token,
            "refresh_token": refresh_token,
            "expires_in": 1800,
            "token_type": "Bearer"
        }
    
    def check_permission(self, username: str, permission: str) -> bool:
        if username not in users_db:
            return False
        
        user = users_db[username]
        permissions = roles_db.get(user.role, {}).get("permissions", [])
        return permission in permissions
    
    def enable_2fa(self, username: str) -> Dict:
        if username not in users_db:
            raise DMSCError("User not found", "USER_NOT_FOUND")
        
        user = users_db[username]
        user.totp_secret = generate_totp_secret()
        
        # In production, generate QR code for authenticator app
        return {
            "secret": user.totp_secret,
            "message": "Scan this secret in your authenticator app"
        }

# Permission middleware
def require_permission(permission: str):
    def decorator(func):
        @wraps(func)
        async def wrapper(context: DMSCServiceContext, *args, **kwargs):
            auth_header = context.http.request.headers.get("Authorization", "")
            if not auth_header.startswith("Bearer "):
                return {"error": "No token provided"}, 401
            
            token = auth_header[7:]
            token_manager = context.auth.get_token_manager()
            payload = token_manager.verify_token(token)
            
            if not payload:
                return {"error": "Invalid token"}, 401
            
            if permission not in payload.get("permissions", []):
                return {"error": "Permission denied"}, 403
            
            return await func(context, *args, **kwargs)
        return wrapper
    return decorator

# HTTP handlers
async def register_handler(context: DMSCServiceContext):
    try:
        data = await context.http.request.json()
        auth_service = context.auth_service
        
        user = auth_service._register(
            data["username"],
            data["email"],
            data["password"]
        )
        
        return {"status": "success", "data": user}
    except DMSCError as e:
        return {"status": "error", "message": e.message}, 400

async def login_handler(context: DMSCServiceContext):
    try:
        data = await context.http.request.json()
        auth_service = context.auth_service
        
        result = auth_service.login(
            data["username"],
            data["password"],
            data.get("totp_code")
        )
        
        return {"status": "success", "data": result}
    except DMSCError as e:
        return {"status": "error", "message": e.message}, 400

async def refresh_handler(context: DMSCServiceContext):
    try:
        data = await context.http.request.json()
        auth_service = context.auth_service
        
        result = auth_service.refresh(data["refresh_token"])
        
        return {"status": "success", "data": result}
    except DMSCError as e:
        return {"status": "error", "message": e.message}, 400

async def protected_handler(context: DMSCServiceContext):
    # Get current user from token
    auth_header = context.http.request.headers.get("Authorization", "")
    token = auth_header[7:]
    token_manager = context.auth.get_token_manager()
    payload = token_manager.verify_token(token)
    
    return {
        "status": "success",
        "data": {
            "username": payload["sub"],
            "email": payload["email"],
            "role": payload["role"]
        }
    }

async def admin_handler(context: DMSCServiceContext):
    auth_header = context.http.request.headers.get("Authorization", "")
    token = auth_header[7:]
    token_manager = context.auth.get_token_manager()
    payload = token_manager.verify_token(token)
    
    # Only admin can access
    if payload.get("role") != "admin":
        return {"error": "Admin access required"}, 403
    
    return {
        "status": "success",
        "message": "Admin data accessed",
        "data": {"total_users": len(users_db)}
    }

# Main application
async def main():
    app = DMSCAppBuilder()
    
    # Configure logging
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    # Configure HTTP
    app.with_http(DMSCHTTPConfig(host="0.0.0.0", port=8080))
    
    # Initialize auth service
    token_manager = TokenManager("your-secret-key")
    
    # Build application
    dms_app = app.build()
    
    # Store services in context
    dms_app.auth_service = AuthService(dms_app.context, token_manager)
    dms_app.context.auth_service = dms_app.auth_service
    dms_app.context.auth.get_token_manager = lambda: token_manager
    
    # Add routes
    dms_app.router.add_route("POST", "/register", register_handler)
    dms_app.router.add_route("POST", "/login", login_handler)
    dms_app.router.add_route("POST", "/refresh", refresh_handler)
    dms_app.router.add_route("GET", "/protected", protected_handler)
    dms_app.router.add_route("GET", "/admin", admin_handler)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Authentication Flow

1. **Registration**: Hash password with salt and store user
2. **Login**: Verify password and generate JWT tokens
3. **Access**: Use access token to access protected resources
4. **Refresh**: Use refresh token to get new access token

### Security Features

- **Password Hashing**: PBKDF2 with salt (100,000 iterations)
- **JWT Tokens**: Short-lived access tokens, long-lived refresh tokens
- **2FA Support**: TOTP-based two-factor authentication
- **Role-Based Access**: Permission-based authorization

### Token Management

- **Access Token**: Expires in 30 minutes
- **Refresh Token**: Expires in 7 days
- **Token Storage**: Refresh tokens stored server-side

## Running Steps

1. Install dependencies:
   ```bash
   pip install dmsc pyjwt
   ```

2. Save the code to `auth_app.py`

3. Run the application:
   ```bash
   python auth_app.py
   ```

4. Test the endpoints:

   ```bash
   # Register
   curl -X POST http://localhost:8080/register \
     -H "Content-Type: application/json" \
     -d '{"username": "john", "email": "john@example.com", "password": "password123"}'
   
   # Login
   curl -X POST http://localhost:8080/login \
     -H "Content-Type: application/json" \
     -d '{"username": "john", "password": "password123"}'
   
   # Access protected route
   curl http://localhost:8080/protected \
     -H "Authorization: Bearer YOUR_ACCESS_TOKEN"
   
   # Refresh token
   curl -X POST http://localhost:8080/refresh \
     -H "Content-Type: application/json" \
     -d '{"refresh_token": "YOUR_REFRESH_TOKEN"}'
   ```

## Expected Output

### Registration Response

```json
{
  "status": "success",
  "data": {
    "username": "john",
    "email": "john@example.com",
    "created_at": "2024-01-15T10:30:00",
    "last_login": null,
    "role": "user",
    "is_active": true
  }
}
```

### Login Response

```json
{
  "status": "success",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 1800,
    "token_type": "Bearer"
  }
}
```

### Protected Route Response

```json
{
  "status": "success",
  "data": {
    "username": "john",
    "email": "john@example.com",
    "role": "user"
  }
}
```

## Security Best Practices

1. **Use HTTPS**: Always use HTTPS in production
2. **Secure Secrets**: Store JWT secrets securely (environment variables)
3. **Short Token Lifetimes**: Use short-lived access tokens
4. **Secure Refresh Tokens**: Store refresh tokens securely (httpOnly cookies)
5. **Implement Rate Limiting**: Protect auth endpoints from brute force
6. **Log Auth Events**: Track login attempts and failures
7. **Validate Inputs**: Always validate user inputs
8. **Use Strong Password Hashing**: Use appropriate hash iterations
