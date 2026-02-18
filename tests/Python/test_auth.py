#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
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
DMSC Auth Module Tests

Tests for the authentication and authorization functionality.
"""

import pytest
from dmsc import (
    DMSCAuthModule,
    DMSCAuthConfig,
    DMSCJWTManager,
    DMSCJWTClaims,
    DMSCJWTValidationOptions,
    DMSCSessionManager,
    DMSCSession,
    DMSCPermissionManager,
    DMSCPermission,
    DMSCRole,
    DMSCOAuthManager,
    DMSCOAuthToken,
    DMSCOAuthUserInfo,
    DMSCOAuthProvider,
    DMSCJWTRevocationList,
    DMSCRevokedTokenInfo,
)


class TestDMSCAuthModule:
    """Tests for DMSCAuthModule"""

    def test_auth_module_creation(self):
        """Test creating auth module"""
        config = DMSCAuthConfig()

        auth_module = DMSCAuthModule(config)
        assert auth_module is not None
        assert auth_module.is_enabled is True


class TestDMSCJWTManager:
    """Tests for DMSCJWTManager"""

    def test_jwt_manager_creation(self):
        """Test creating JWT manager"""
        jwt_manager = DMSCJWTManager("test-secret", 3600)
        assert jwt_manager is not None

    def test_token_generation(self):
        """Test JWT token generation"""
        jwt_manager = DMSCJWTManager("test-secret", 3600)

        token = jwt_manager.py_generate_token("user123", ["user"], ["read:data"])
        assert token is not None
        assert len(token) > 0

    def test_token_validation(self):
        """Test JWT token validation"""
        jwt_manager = DMSCJWTManager("test-secret", 3600)

        token = jwt_manager.py_generate_token("user123", ["user"], ["read:data"])

        claims = jwt_manager.py_validate_token(token)
        assert claims is not None
        assert claims.subject == "user123"


class TestDMSCJWTValidationOptions:
    """Tests for DMSCJWTValidationOptions"""

    def test_validation_options_creation(self):
        """Test creating validation options"""
        options = DMSCJWTValidationOptions(
            validate_exp=True,
            validate_iat=True,
            required_roles=["user"],
            required_permissions=["read:data"]
        )
        assert options is not None


class TestDMSCSessionManager:
    """Tests for DMSCSessionManager"""

    def test_session_manager_creation(self):
        """Test creating session manager"""
        session_manager = DMSCSessionManager(86400)
        assert session_manager is not None


class TestDMSCSession:
    """Tests for DMSCSession"""

    def test_session_creation(self):
        """Test creating a session"""
        session = DMSCSession(
            id=None,
            user_id="user123",
            created_at=None,
            last_accessed=None,
            expires_at=None,
            data=None,
            ip_address=None,
            user_agent=None
        )

        assert session.user_id == "user123"


class TestDMSCPermissionManager:
    """Tests for DMSCPermissionManager"""

    def test_permission_manager_creation(self):
        """Test creating permission manager"""
        perm_manager = DMSCPermissionManager()
        assert perm_manager is not None


class TestDMSCPermission:
    """Tests for DMSCPermission"""

    def test_permission_creation(self):
        """Test creating a permission"""
        perm = DMSCPermission(
            id=None,
            name="read:users",
            description="Can read user data",
            resource="users",
            action="read"
        )

        assert perm.name == "read:users"
        assert perm.resource == "users"


class TestDMSCRole:
    """Tests for DMSCRole"""

    def test_role_creation(self):
        """Test creating a role"""
        role = DMSCRole(
            id=None,
            name="admin",
            description="Administrator role",
            permissions=["read:users", "write:users"],
            is_system=False
        )

        assert role.name == "admin"


class TestDMSCOAuthManager:
    """Tests for DMSCOAuthManager"""

    def test_oauth_manager_creation(self):
        """Test creating OAuth manager"""
        oauth_manager = DMSCOAuthManager()
        assert oauth_manager is not None


class TestDMSCOAuthToken:
    """Tests for DMSCOAuthToken"""

    def test_oauth_token_creation(self):
        """Test creating OAuth token"""
        token = DMSCOAuthToken(
            access_token="access_123",
            token_type="Bearer",
            refresh_token="refresh_456",
            scope="openid email",
            expires_in=3600
        )

        assert token.access_token == "access_123"
        assert token.token_type == "Bearer"


class TestDMSCOAuthUserInfo:
    """Tests for DMSCOAuthUserInfo"""

    def test_oauth_user_info_creation(self):
        """Test creating OAuth user info"""
        user_info = DMSCOAuthUserInfo(
            id="user123",
            email="user@example.com",
            name="John Doe",
            avatar_url=None,
            provider="google"
        )

        assert user_info.id == "user123"
        assert user_info.email == "user@example.com"


class TestDMSCOAuthProvider:
    """Tests for DMSCOAuthProvider"""

    def test_oauth_provider_creation(self):
        """Test creating OAuth provider"""
        provider = DMSCOAuthProvider(
            id="google",
            name="Google",
            client_id="client_123",
            client_secret="secret_456",
            auth_url="https://accounts.google.com/o/oauth2/auth",
            token_url="https://oauth2.googleapis.com/token",
            user_info_url="https://openidconnect.googleapis.com/v1/userinfo",
            scopes=["openid", "email"],
            enabled=True
        )

        assert provider.name == "Google"
        assert provider.client_id == "client_123"


class TestDMSCJWTRevocationList:
    """Tests for DMSCJWTRevocationList"""

    def test_revocation_list_creation(self):
        """Test creating revocation list"""
        revocation_list = DMSCJWTRevocationList()
        assert revocation_list is not None

    def test_token_revocation(self):
        """Test token revocation"""
        revocation_list = DMSCJWTRevocationList()

        revocation_list.revoke_token("token_123", "user123", "User logout", 3600)

        is_revoked = revocation_list.is_revoked("token_123")
        assert is_revoked is True


class TestDMSCRevokedTokenInfo:
    """Tests for DMSCRevokedTokenInfo"""

    def test_revoked_token_info_creation(self):
        """Test creating revoked token info"""
        revoked_info = DMSCRevokedTokenInfo(
            token_id="token_123",
            user_id="user123",
            revoked_at=0,
            expires_at=3600,
            reason="User logout"
        )

        assert revoked_info.token_id == "token_123"
        assert revoked_info.reason == "User logout"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
