<div align="center">

# Auth API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The auth module provides authentication and authorization features, supporting JWT, OAuth2, and role-based access control.

## Module Overview

</div>

The auth module contains the following sub-modules:

- **jwt**: JWT token generation and verification
- **oauth**: OAuth2 authorization flow
- **permissions**: Permission management and RBAC
- **session**: Session management

<div align="center">

## Core Components

</div>

### DMSCAuthModule

The authentication module main interface, providing unified authentication service access.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `generate_jwt(payload)` | Generate JWT token | `payload: Dict` | `str` |
| `verify_jwt(token)` | Verify JWT token | `token: str` | `Dict` |
| `generate_jwt_with_refresh(payload)` | Generate JWT and refresh token | `payload: Dict` | `Tuple[str, str]` |
| `refresh_jwt(refresh_token)` | Get new JWT using refresh token | `refresh_token: str` | `str` |
| `check_permission(role, permission)` | Check role permission | `role: str`, `permission: str` | `bool` |
| `oauth_authorization_url(provider, state)` | Generate OAuth authorization URL | `provider: str`, `state: str` | `str` |
| `oauth_exchange_token(provider, code)` | Exchange OAuth token | `provider: str`, `code: str` | `str` |
| `oauth_get_user_info(provider, token)` | Get OAuth user info | `provider: str`, `token: str` | `Dict` |

#### Usage Example

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

# Initialize authentication module
config = DMSCAuthConfig(
    jwt_secret="your-secret-key",
    jwt_issuer="my-app",
    jwt_expires_in=3600
)
auth_module = DMSCAuthModule(config)

# Generate JWT token
payload = {
    "id": 1,
    "username": "test_user",
    "role": "admin"
}
jwt = auth_module.generate_jwt(payload)

# Verify JWT token
decoded = auth_module.verify_jwt(jwt)
print(f"User: {decoded['username']}")

# Generate JWT with refresh token
access_token, refresh_token = auth_module.generate_jwt_with_refresh(payload)

# Use refresh token to get new JWT
new_access_token = auth_module.refresh_jwt(refresh_token)

# Check permissions
can_create = auth_module.check_permission("admin", "create")
print(f"Can create: {can_create}")

# OAuth2 flow
```

### DMSCAuthConfig

Authentication module configuration.

```python
from dmsc import DMSCAuthConfig

config = DMSCAuthConfig(
    jwt_secret="your-jwt-secret-key",
    jwt_issuer="my-application",
    jwt_audience="dmsc-users",
    jwt_algorithm="HS256",
    jwt_expires_in=3600,
    refresh_token_expires_in=86400,
    oauth_providers=["google", "github", "apple"],
    password_hash_algorithm="bcrypt",
    max_login_attempts=5,
    lockout_duration=900
)
```

## JWT Operations

### Generate Token

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

payload = {
    "sub": "user123",
    "name": "John Doe",
    "email": "john@example.com",
    "roles": ["user", "admin"],
    "permissions": ["read", "write", "delete"]
}

token = auth.generate_jwt(payload)
print(f"JWT Token: {token}")
```

### Verify Token

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

token = "your-jwt-token-here"
try:
    decoded = auth.verify_jwt(token)
    print(f"Token valid for user: {decoded['sub']}")
    print(f"Roles: {decoded['roles']}")
except Exception as e:
    print(f"Invalid token: {e}")
```

### Token with Refresh

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

payload = {"sub": "user123", "role": "admin"}

# Generate both access and refresh tokens
access_token, refresh_token = auth.generate_jwt_with_refresh(payload)

# Later, use refresh token to get new access token
new_access_token = auth.refresh_jwt(refresh_token)
```

## OAuth2 Operations

### Google OAuth2

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

# Step 1: Generate authorization URL
auth_url = auth.oauth_authorization_url(
    provider="google",
    state="random-security-state",
    redirect_uri="https://yourapp.com/auth/callback",
    scope="openid email profile"
)
print(f"Redirect user to: {auth_url}")

# Step 2: Exchange code for tokens
# After user authorizes, you get a code
tokens = auth.oauth_exchange_token(
    provider="google",
    code="authorization-code-from-google"
)

# Step 3: Get user info
user_info = auth.oauth_get_user_info(
    provider="google",
    token=tokens["access_token"]
)
print(f"User: {user_info['name']}, Email: {user_info['email']}")
```

## Permission Management

### Role-Based Access Control

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

# Define permissions
permissions = {
    "admin": ["read", "write", "delete", "admin", "users.manage"],
    "editor": ["read", "write"],
    "viewer": ["read"],
    "guest": []
}

# Check permissions
print(auth.check_permission("admin", "write"))  # True
print(auth.check_permission("viewer", "delete"))  # False
```

### Permission Middleware

```python
from dmsc import DMSCAuthMiddleware

# Create auth middleware
auth_middleware = DMSCAuthMiddleware(
    auth_module=auth,
    exclude_paths=["/health", "/metrics"],
    token_header="Authorization",
    token_prefix="Bearer"
)

# Use in HTTP handler
async def protected_handler(request):
    user = await auth_middleware.authenticate(request)
    if user return {"error": "Unauthorized"}
    is None:
        return {"user": user["sub"], "role": user["role"]}
```

## Session Management

```python
from dmsc import DMSCAuthModule, DMSCAuthConfig

auth = DMSCAuthModule(DMSCAuthConfig.default())

# Create session
session = auth.create_session(
    user_id="user123",
    data={"role": "admin"},
    max_idle_time=3600,
    absolute_timeout=86400
)
print(f"Session ID: {session.session_id}")

# Validate session
is_valid = auth.validate_session(session.session_id)
print(f"Session valid: {is_valid}")

# Destroy session
auth.destroy_session(session.session_id)

# Get session data
session_data = auth.get_session_data(session.session_id)
```

## Best Practices

1. **Use HTTPS**: Always use HTTPS in production to protect tokens
2. **Short Token Lifetimes**: Use short-lived access tokens (15-30 minutes)
3. **Secure Refresh Tokens**: Store refresh tokens securely (httpOnly cookies)
4. **Implement Rate Limiting**: Protect auth endpoints from brute force attacks
5. **Log Authentication Events**: Track login attempts and failures
6. **Use Strong Secrets**: Use cryptographically strong secrets for JWT signing
7. **Validate All Inputs**: Always validate OAuth states and tokens
