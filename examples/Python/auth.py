# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
Ri Auth Module Example

This example demonstrates how to use the Ri authentication module
for JWT, OAuth, and session management.
"""

import asyncio
from ri import (
    RiAuthModule,
    RiAuthConfig,
    RiJWTManager,
    RiJWTClaims,
    RiJWTValidationOptions,
    RiSessionManager,
    RiSession,
    RiPermissionManager,
    RiPermission,
    RiRole,
    RiOAuthManager,
    RiOAuthToken,
    RiOAuthUserInfo,
    RiOAuthProvider,
    RiJWTRevocationList,
    RiRevokedTokenInfo,
)


async def main():
    # Create authentication configuration
    auth_config = RiAuthConfig()
    auth_config.jwt_secret = "your-secret-key-here"
    auth_config.jwt_algorithm = "HS256"
    auth_config.token_expiry_seconds = 3600
    auth_config.refresh_token_expiry_seconds = 86400
    auth_config.enable_oauth = True
    auth_config.enable_session = True

    # Initialize auth module
    auth_module = RiAuthModule(auth_config)

    # Create JWT manager
    jwt_manager = RiJWTManager()

    # Create JWT claims
    claims = RiJWTClaims()
    claims.sub = "user123"
    claims.iss = "ri-auth"
    claims.aud = "ri-api"
    claims.exp = 3600
    claims.iat = 0
    claims.custom_claims = {"role": "admin", "department": "engineering"}

    # Generate JWT token
    print("Generating JWT token...")
    token = jwt_manager.generate_token(claims)
    print(f"Generated token: {token[:50]}...")

    # Validate JWT token
    print("\nValidating JWT token...")
    validation_options = RiJWTValidationOptions()
    validation_options.validate_exp = True
    validation_options.validate_nbf = True
    validation_options.validate_aud = True
    validation_options.expected_audience = "ri-api"
    validation_options.leeway_seconds = 60

    validated_claims = jwt_manager.validate_token(token, validation_options)
    if validated_claims:
        print(f"Token validated successfully!")
        print(f"Subject: {validated_claims.sub}")
        print(f"Custom claims: {validated_claims.custom_claims}")

    # Create permission manager
    perm_manager = RiPermissionManager()

    # Define permissions
    read_perm = RiPermission()
    read_perm.name = "read:users"
    read_perm.description = "Can read user data"

    write_perm = RiPermission()
    write_perm.name = "write:users"
    write_perm.description = "Can modify user data"

    # Define roles
    admin_role = RiRole()
    admin_role.name = "admin"
    admin_role.permissions = ["read:users", "write:users"]

    user_role = RiRole()
    user_role.name = "user"
    user_role.permissions = ["read:users"]

    # Check permissions
    print("\nChecking permissions...")
    has_read = perm_manager.check_permission("user123", "read:users")
    print(f"User has read permission: {has_read}")

    has_write = perm_manager.check_permission("user123", "write:users")
    print(f"User has write permission: {has_write}")

    # Create session manager
    session_manager = RiSessionManager()

    # Create a session
    print("\nCreating session...")
    session = RiSession()
    session.session_id = "sess_123456"
    session.user_id = "user123"
    session.data = {"login_time": "2025-01-01T00:00:00Z", "ip": "192.168.1.1"}
    session.expires_at = 3600

    session_manager.create_session(session)
    print(f"Session created: {session.session_id}")

    # Retrieve session
    retrieved_session = session_manager.get_session("sess_123456")
    if retrieved_session:
        print(f"Retrieved session for user: {retrieved_session.user_id}")

    # Create OAuth manager
    oauth_manager = RiOAuthManager()

    # Configure OAuth providers
    google_provider = RiOAuthProvider()
    google_provider.name = "google"
    google_provider.client_id = "google-client-id"
    google_provider.client_secret = "google-client-secret"
    google_provider.auth_url = "https://accounts.google.com/o/oauth2/auth"
    google_provider.token_url = "https://oauth2.googleapis.com/token"
    google_provider.scopes = ["openid", "email", "profile"]

    github_provider = RiOAuthProvider()
    github_provider.name = "github"
    github_provider.client_id = "github-client-id"
    github_provider.client_secret = "github-client-secret"
    github_provider.auth_url = "https://github.com/login/oauth/authorize"
    github_provider.token_url = "https://github.com/login/oauth/access_token"
    github_provider.scopes = ["user:email", "read:user"]

    # Create OAuth token
    oauth_token = RiOAuthToken()
    oauth_token.access_token = "oauth_access_token_123"
    oauth_token.refresh_token = "oauth_refresh_token_456"
    oauth_token.token_type = "Bearer"
    oauth_token.expires_in = 3600
    oauth_token.scope = "openid email profile"

    print("\nOAuth configuration completed!")

    # Create revocation list for token blacklisting
    revocation_list = RiJWTRevocationList()

    # Revoke a token
    revoked_info = RiRevokedTokenInfo()
    revoked_info.token_id = "token_123"
    revoked_info.revoked_at = 0
    revoked_info.reason = "User logout"

    revocation_list.revoke_token(revoked_info)
    print(f"Token revoked: {revoked_info.token_id}")

    # Check if token is revoked
    is_revoked = revocation_list.is_revoked("token_123")
    print(f"Token is revoked: {is_revoked}")

    print("\nAuthentication operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
