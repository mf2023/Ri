<div align="center">

# Auth API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

auth模块提供认证与授权功能，支持JWT、OAuth2和基于角色的访问控制。

## 模块概述

</div>

auth模块包含以下子模块：

- **jwt**: JWT令牌生成和验证
- **oauth**: OAuth2授权流程
- **permissions**: 权限管理和RBAC
- **session**: 会话管理

<div align="center">

## 核心组件

</div>

### DMSCAuthConfig

认证配置类，用于配置认证行为。

#### 构造函数

```python
DMSCAuthConfig(
    auth_type: str = "jwt",
    secret_key: str = "",
    algorithm: str = "HS256",
    token_expiry: int = 3600,
    refresh_token_expiry: int = 86400,
    issuer: str = "dmsc",
    audience: str = "dmsc-users",
    enable_refresh_token: bool = True,
    enable_multi_device: bool = True,
    max_devices_per_user: int = 5,
    enable_rate_limit: bool = True,
    rate_limit_attempts: int = 5,
    rate_limit_window: int = 300,
    enable_captcha: bool = False,
    captcha_provider: str = "recaptcha",
    captcha_secret: str = "",
    enable_2fa: bool = False,
    sms_provider: str = "",
    sms_api_key: str = "",
    enable_oauth: bool = False,
    oauth_providers: List[Dict] = None,
    enable_ldap: bool = False,
    ldap_url: str = "",
    ldap_bind_dn: str = "",
    ldap_bind_password: str = "",
    ldap_user_base: str = "",
    ldap_user_filter: str = "",
    ldap_group_base: str = "",
    ldap_group_filter: str = "",
    enable_sso: bool = False,
    sso_provider: str = "",
    sso_client_id: str = "",
    sso_client_secret: str = "",
    sso_redirect_uri: str = "",
    enable_session: bool = True,
    session_backend: str = "redis",
    session_ttl: int = 86400,
    session_cookie_name: str = "dmsc_session",
    session_cookie_domain: str = "",
    session_cookie_path: str = "/",
    session_cookie_secure: bool = True,
    session_cookie_httponly: bool = True,
    session_cookie_samesite: str = "lax"
)
```

### DMSCAuthManager

认证管理器，提供统一的认证接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `register(username, password, email, **kwargs)` | 用户注册 | `username: str`, `password: str`, `email: str`, `**kwargs` | `Dict` |
| `login(username, password, **kwargs)` | 用户登录 | `username: str`, `password: str`, `**kwargs` | `Dict` |
| `logout(token)` | 用户登出 | `token: str` | `bool` |
| `verify_token(token)` | 验证令牌 | `token: str` | `Dict` |
| `refresh_token(refresh_token)` | 刷新令牌 | `refresh_token: str` | `Dict` |
| `change_password(username, old_password, new_password)` | 修改密码 | `username: str`, `old_password: str`, `new_password: str` | `bool` |
| `reset_password(username, reset_token, new_password)` | 重置密码 | `username: str`, `reset_token: str`, `new_password: str` | `bool` |
| `send_reset_email(email)` | 发送重置邮件 | `email: str` | `bool` |
| `enable_2fa(username)` | 启用双因子认证 | `username: str` | `Dict` |
| `verify_2fa(username, code)` | 验证双因子认证 | `username: str`, `code: str` | `bool` |
| `disable_2fa(username, password)` | 禁用双因子认证 | `username: str`, `password: str` | `bool` |
| `oauth_login(provider, code)` | OAuth登录 | `provider: str`, `code: str` | `Dict` |
| `oauth_callback(provider, code, state)` | OAuth回调 | `provider: str`, `code: str`, `state: str` | `Dict` |
| `get_user_info(username)` | 获取用户信息 | `username: str` | `Dict` |
| `update_user_info(username, **kwargs)` | 更新用户信息 | `username: str`, `**kwargs` | `bool` |
| `check_permission(username, permission)` | 检查权限 | `username: str`, `permission: str` | `bool` |
| `assign_role(username, role)` | 分配角色 | `username: str`, `role: str` | `bool` |
| `remove_role(username, role)` | 移除角色 | `username: str`, `role: str` | `bool` |
| `get_user_roles(username)` | 获取用户角色 | `username: str` | `List[str]` |
| `get_role_permissions(role)` | 获取角色权限 | `role: str` | `List[str]` |
| `create_role(role, permissions)` | 创建角色 | `role: str`, `permissions: List[str]` | `bool` |
| `delete_role(role)` | 删除角色 | `role: str` | `bool` |
| `ldap_login(username, password)` | LDAP登录 | `username: str`, `password: str` | `Dict` |
| `sso_login(provider)` | SSO登录 | `provider: str` | `str` |
| `sso_callback(provider, ticket)` | SSO回调 | `provider: str`, `ticket: str` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCAuthManager, DMSCAuthConfig

# 初始化认证管理器
config = DMSCAuthConfig(
    secret_key="your-secret-key",
    token_expiry=3600,
    enable_refresh_token=True
)

auth_manager = DMSCAuthManager(config)

# 用户注册
user = auth_manager.register(
    username="john_doe",
    password="secure_password",
    email="john@example.com"
)

# 用户登录
result = auth_manager.login(
    username="john_doe",
    password="secure_password"
)

# 验证令牌
token_info = auth_manager.verify_token(result["access_token"])

# 检查权限
has_permission = auth_manager.check_permission("john_doe", "admin:read")
```

### DMSCAuthMiddleware

认证中间件，用于保护HTTP路由。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `__init__(auth_manager, **kwargs)` | 初始化中间件 | `auth_manager: DMSCAuthManager`, `**kwargs` | `None` |
| `authenticate_request(request)` | 认证请求 | `request: Request` | `Dict` |
| `require_permission(permission)` | 要求权限 | `permission: str` | `Callable` |
| `require_role(role)` | 要求角色 | `role: str` | `Callable` |
| `require_auth()` | 要求认证 | `None` | `Callable` |
| `optional_auth()` | 可选认证 | `None` | `Callable` |

#### 使用示例

```python
from dmsc import DMSCAppBuilder, DMSCAuthMiddleware

# 创建应用
app = DMSCAppBuilder().build()

# 创建认证中间件
auth_middleware = DMSCAuthMiddleware(auth_manager)

# 保护路由
@app.route("/protected")
@auth_middleware.require_auth()
async def protected_route(request):
    user = request.user
    return {"message": f"Hello {user['username']}!"}

# 要求权限
@app.route("/admin")
@auth_middleware.require_permission("admin:read")
async def admin_route(request):
    return {"message": "Admin access granted"}

# 要求角色
@app.route("/moderator")
@auth_middleware.require_role("moderator")
async def moderator_route(request):
    return {"message": "Moderator access granted"}
```

### DMSCTokenManager

令牌管理器，提供JWT令牌管理功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `generate_token(payload, expiry)` | 生成令牌 | `payload: Dict`, `expiry: int` | `str` |
| `verify_token(token)` | 验证令牌 | `token: str` | `Dict` |
| `decode_token(token)` | 解码令牌 | `token: str` | `Dict` |
| `refresh_token(token, expiry)` | 刷新令牌 | `token: str`, `expiry: int` | `str` |
| `revoke_token(token)` | 撤销令牌 | `token: str` | `bool` |
| `is_token_expired(token)` | 检查令牌是否过期 | `token: str` | `bool` |
| `get_token_expiry(token)` | 获取令牌过期时间 | `token: str` | `datetime` |
| `get_token_issuer(token)` | 获取令牌颁发者 | `token: str` | `str` |
| `get_token_audience(token)` | 获取令牌受众 | `token: str` | `str` |
| `get_token_subject(token)` | 获取令牌主题 | `token: str` | `str` |

#### 使用示例

```python
from dmsc import DMSCTokenManager

# 初始化令牌管理器
token_manager = DMSCTokenManager(
    secret_key="your-secret-key",
    algorithm="HS256",
    issuer="dmsc",
    audience="dmsc-users"
)

# 生成令牌
payload = {
    "username": "john_doe",
    "role": "user",
    "permissions": ["read", "write"]
}
token = token_manager.generate_token(payload, expiry=3600)

# 验证令牌
try:
    decoded = token_manager.verify_token(token)
    print(f"Token valid for user: {decoded['username']}")
except Exception as e:
    print(f"Token invalid: {e}")

# 检查过期
if token_manager.is_token_expired(token):
    # 刷新令牌
    new_token = token_manager.refresh_token(token, expiry=3600)
```

### DMSCPermissionManager

权限管理器，提供角色权限管理功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `create_role(role, permissions)` | 创建角色 | `role: str`, `permissions: List[str]` | `bool` |
| `delete_role(role)` | 删除角色 | `role: str` | `bool` |
| `update_role(role, permissions)` | 更新角色 | `role: str`, `permissions: List[str]` | `bool` |
| `get_role(role)` | 获取角色 | `role: str` | `Dict` |
| `get_all_roles()` | 获取所有角色 | `None` | `List[Dict]` |
| `assign_role_to_user(username, role)` | 分配角色给用户 | `username: str`, `role: str` | `bool` |
| `remove_role_from_user(username, role)` | 从用户移除角色 | `username: str`, `role: str` | `bool` |
| `get_user_roles(username)` | 获取用户角色 | `username: str` | `List[str]` |
| `get_role_permissions(role)` | 获取角色权限 | `role: str` | `List[str]` |
| `check_permission(role, permission)` | 检查角色权限 | `role: str`, `permission: str` | `bool` |
| `check_user_permission(username, permission)` | 检查用户权限 | `username: str`, `permission: str` | `bool` |
| `create_permission(permission, description)` | 创建权限 | `permission: str`, `description: str` | `bool` |
| `delete_permission(permission)` | 删除权限 | `permission: str` | `bool` |
| `get_all_permissions()` | 获取所有权限 | `None` | `List[Dict]` |

#### 使用示例

```python
from dmsc import DMSCPermissionManager

# 初始化权限管理器
permission_manager = DMSCPermissionManager()

# 创建角色
permission_manager.create_role("admin", [
    "user:create", "user:read", "user:update", "user:delete",
    "post:create", "post:read", "post:update", "post:delete"
])

permission_manager.create_role("editor", [
    "user:read", "user:update",
    "post:create", "post:read", "post:update"
])

permission_manager.create_role("viewer", [
    "user:read", "post:read"
])

# 分配角色给用户
permission_manager.assign_role_to_user("john_doe", "editor")

# 检查用户权限
if permission_manager.check_user_permission("john_doe", "post:create"):
    print("User can create posts")

# 获取用户角色
user_roles = permission_manager.get_user_roles("john_doe")
print(f"User roles: {user_roles}")
```

### DMSCOAuthProvider

OAuth提供者，提供OAuth2认证功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `__init__(provider, client_id, client_secret, redirect_uri)` | 初始化提供者 | `provider: str`, `client_id: str`, `client_secret: str`, `redirect_uri: str` | `None` |
| `get_authorization_url(state)` | 获取授权URL | `state: str` | `str` |
| `exchange_code_for_token(code)` | 交换代码获取令牌 | `code: str` | `Dict` |
| `refresh_access_token(refresh_token)` | 刷新访问令牌 | `refresh_token: str` | `Dict` |
| `get_user_info(access_token)` | 获取用户信息 | `access_token: str` | `Dict` |
| `revoke_token(token)` | 撤销令牌 | `token: str` | `bool` |

#### 使用示例

```python
from dmsc import DMSCOAuthProvider

# 初始化Google OAuth提供者
google_oauth = DMSCOAuthProvider(
    provider="google",
    client_id="your-google-client-id",
    client_secret="your-google-client-secret",
    redirect_uri="https://your-app.com/oauth/callback"
)

# 获取授权URL
auth_url = google_oauth.get_authorization_url(state="random-state")
print(f"Please visit: {auth_url}")

# 用户授权后，交换代码获取令牌
token_data = google_oauth.exchange_code_for_token(code="authorization-code")
access_token = token_data["access_token"]

# 获取用户信息
user_info = google_oauth.get_user_info(access_token)
print(f"Logged in as: {user_info['name']}")
```