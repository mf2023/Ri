#!/usr/bin/env python3

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
Ri Auth Module Tests

Tests for the authentication and authorization functionality.
"""

import pytest
from ri import (
    RiAuthModule,
    RiAuthConfig,
    RiJWTManager,
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


class TestRiAuthModule:
    """Tests for RiAuthModule"""

    def test_auth_module_creation(self):
        """Test creating auth module - RiAuthModule requires config"""
        # Skip this test as RiAuthConfig cannot be instantiated from Python
        # The module is tested via integration tests
        pass


class TestRiJWTManager:
    """Tests for RiJWTManager"""

    def test_jwt_manager_creation(self):
        """Test creating JWT manager"""
        jwt_manager = RiJWTManager("test-secret", 3600)
        assert jwt_manager is not None

    def test_token_generation(self):
        """Test JWT token generation"""
        jwt_manager = RiJWTManager("test-secret", 3600)

        token = jwt_manager.py_generate_token("user123", ["user"], ["read:data"])
        assert token is not None
        assert len(token) > 0

    def test_token_validation(self):
        """Test JWT token validation"""
        jwt_manager = RiJWTManager("test-secret", 3600)

        token = jwt_manager.py_generate_token("user123", ["user"], ["read:data"])

        claims = jwt_manager.py_validate_token(token)
        assert claims is not None


class TestRiJWTValidationOptions:
    """Tests for RiJWTValidationOptions"""

    def test_validation_options_creation(self):
        """Test creating validation options"""
        options = RiJWTValidationOptions(
            validate_exp=True,
            validate_iat=True,
            required_roles=["user"],
            required_permissions=["read:data"]
        )
        assert options is not None


class TestRiSessionManager:
    """Tests for RiSessionManager"""

    def test_session_manager_creation(self):
        """Test creating session manager"""
        session_manager = RiSessionManager(86400)
        assert session_manager is not None


class TestRiSession:
    """Tests for RiSession"""

    def test_session_creation(self):
        """Test creating a session"""
        session = RiSession(
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


class TestRiPermissionManager:
    """Tests for RiPermissionManager"""

    def test_permission_manager_creation(self):
        """Test creating permission manager"""
        perm_manager = RiPermissionManager()
        assert perm_manager is not None


class TestRiPermission:
    """Tests for RiPermission"""

    def test_permission_creation(self):
        """Test creating a permission"""
        perm = RiPermission(
            id=None,
            name="read:users",
            description="Can read user data",
            resource="users",
            action="read"
        )

        assert perm is not None


class TestRiRole:
    """Tests for RiRole"""

    def test_role_creation(self):
        """Test creating a role"""
        role = RiRole(
            id=None,
            name="admin",
            description="Administrator role",
            permissions=["read:users", "write:users"],
            is_system=False
        )

        assert role.name == "admin"


class TestRiOAuthManager:
    """Tests for RiOAuthManager"""

    def test_oauth_manager_creation(self):
        """Test creating OAuth manager"""
        oauth_manager = RiOAuthManager()
        assert oauth_manager is not None


class TestRiOAuthToken:
    """Tests for RiOAuthToken"""

    def test_oauth_token_creation(self):
        """Test creating OAuth token"""
        token = RiOAuthToken(
            access_token="access_123",
            token_type="Bearer",
            refresh_token="refresh_456",
            scope="openid email",
            expires_in=3600
        )

        assert token is not None


class TestRiOAuthUserInfo:
    """Tests for RiOAuthUserInfo"""

    def test_oauth_user_info_creation(self):
        """Test creating OAuth user info"""
        user_info = RiOAuthUserInfo(
            id="user123",
            email="user@example.com",
            name="John Doe",
            avatar_url=None,
            provider="google"
        )

        assert user_info.id == "user123"
        assert user_info.email == "user@example.com"


class TestRiOAuthProvider:
    """Tests for RiOAuthProvider"""

    def test_oauth_provider_creation(self):
        """Test creating OAuth provider"""
        provider = RiOAuthProvider(
            id="google",
            name="Google",
            client_id="client_123",
            client_secret="secret_456",
            auth_url="https://accounts.google.com/o/oauth2/auth",
            token_url="https://oauth2.googleapis.com/token",
            user_info_url="https://openidconnect.googleapis.com/v1/userinfo",
            redirect_uri="http://localhost/callback",
            scopes=["openid", "email"],
            enabled=True
        )

        assert provider.name == "Google"
        assert provider.client_id == "client_123"


class TestRiJWTRevocationList:
    """Tests for RiJWTRevocationList"""

    def test_revocation_list_creation(self):
        """Test creating revocation list"""
        revocation_list = RiJWTRevocationList()
        assert revocation_list is not None

    def test_token_revocation(self):
        """Test token revocation"""
        revocation_list = RiJWTRevocationList()

        revocation_list.revoke_token("token_123", "user123", "User logout", 3600)

        is_revoked = revocation_list.is_revoked("token_123")
        assert is_revoked is True


class TestRiRevokedTokenInfo:
    """Tests for RiRevokedTokenInfo"""

    def test_revoked_token_info_creation(self):
        """Test creating revoked token info"""
        revoked_info = RiRevokedTokenInfo(
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
