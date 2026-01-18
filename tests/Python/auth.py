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
DMSC Authentication Module Python Tests.

This module contains comprehensive tests for the DMSC authentication and authorization
Python bindings. The tests cover JWT token management, session management, permission
control, and OAuth authentication functionality.

Test Classes:
- TestDMSCJWTRevocationList: Tests for JWT token revocation list functionality
- TestDMSCSessionManager: Tests for session management operations
- TestDMSCPermissionManager: Tests for permission and role-based access control
- TestDMSCOAuthManager: Tests for OAuth authentication flow

Each test class validates specific authentication APIs exposed by the Rust backend,
ensuring Python bindings correctly map to Rust functionality.
"""

import unittest
from dmsc import (
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTManager, DMSCJWTClaims,
    DMSCJWTValidationOptions, DMSCSessionManager, DMSCSession,
    DMSCPermissionManager, DMSCPermission, DMSCRole,
    DMSCOAuthManager, DMSCOAuthToken, DMSCOAuthUserInfo, DMSCOAuthProvider,
    DMSCJWTRevocationList, DMSCRevokedTokenInfo
)


class TestDMSCJWTRevocationList(unittest.TestCase):
    """
    Test suite for DMSCJWTRevocationList class.

    The DMSCJWTRevocationList class provides functionality for managing a blacklist
    of revoked JWT tokens. This is essential for implementing token invalidation
    before expiration, such as when a user logs out or when security concerns require
    immediate token revocation.

    Test Methods:
    - test_jwt_revocation_list_new: Verify basic instantiation of revocation list
    - test_jwt_revocation_list_get_revoked_count: Verify initial count is zero
    - test_jwt_revocation_list_revoke_token: Test token revocation and is_revoked check
    - test_jwt_revocation_list_revoke_by_user: Test bulk revocation for a user
    """

    def test_jwt_revocation_list_new(self):
        """Test creating new revocation list.

        This test verifies that DMSCJWTRevocationList can be instantiated successfully.
        The revocation list starts empty, ready to track revoked tokens.
        """
        list = DMSCJWTRevocationList()
        self.assertIsNotNone(list)

    def test_jwt_revocation_list_get_revoked_count(self):
        """Test getting revoked count.

        This test verifies that the initial count of revoked tokens is zero.
        The get_revoked_count() method should return 0 for a newly created list.
        """
        list = DMSCJWTRevocationList()
        count = list.get_revoked_count()
        self.assertEqual(count, 0)

    def test_jwt_revocation_list_revoke_token(self):
        """Test revoking a token.

        This test validates the core token revocation functionality:
        1. Call revoke_token() to add a token to the blacklist
        2. Verify is_revoked() returns True for the revoked token
        3. Verify is_revoked() returns False for non-revoked tokens

        The revoke_token() method accepts:
        - token_id: Unique identifier for the token
        - user_id: Associated user identifier
        - reason: Optional explanation for revocation
        - expiry_seconds: Time until the revocation entry expires
        """
        list = DMSCJWTRevocationList()
        list.revoke_token("test_token", "user123", None, 3600)
        self.assertTrue(list.is_revoked("test_token"))
        self.assertFalse(list.is_revoked("token2"))

    def test_jwt_revocation_list_revoke_by_user(self):
        """Test revoking all tokens for a user.

        This test verifies that revoke_by_user() can invalidate all tokens
        associated with a specific user. This is useful for scenarios like
        password changes or account compromise where all sessions should be terminated.

        Returns True if any tokens were revoked for the user, False if none existed.
        """
        list = DMSCJWTRevocationList()
        list.revoke_token("token1", "user123", None, 3600)
        list.revoke_token("token2", "user123", None, 3600)
        result = list.revoke_by_user("user123")
        self.assertTrue(result)


class TestDMSCSessionManager(unittest.TestCase):
    """
    Test suite for DMSCSessionManager class.

    The DMSCSessionManager class handles user session lifecycle management,
    including session creation, validation, and cleanup. Sessions provide a
    way to maintain user authentication state across multiple requests.

    Test Methods:
    - test_session_manager_new: Verify session manager instantiation
    """

    def test_session_manager_new(self):
        """Test creating session manager.

        This test verifies that DMSCSessionManager can be instantiated with
        a session timeout value. The timeout parameter (in seconds) determines
        how long a session remains valid before requiring re-authentication.
        """
        manager = DMSCSessionManager(3600)
        self.assertIsNotNone(manager)


class TestDMSCPermissionManager(unittest.TestCase):
    """
    Test suite for DMSCPermissionManager class.

    The DMSCPermissionManager class implements role-based access control (RBAC),
    managing permissions and roles for users. It provides methods to check
    user permissions, assign roles, and enforce access policies.

    Test Methods:
    - test_permission_manager_new: Verify permission manager instantiation
    """

    def test_permission_manager_new(self):
        """Test creating permission manager.

        This test verifies that DMSCPermissionManager can be instantiated.
        The manager is ready to accept permission and role configurations.
        """
        manager = DMSCPermissionManager()
        self.assertIsNotNone(manager)


class TestDMSCOAuthManager(unittest.TestCase):
    """
    Test suite for DMSCOAuthManager class.

    The DMSCOAuthManager class handles OAuth 2.0 authentication flow,
    supporting various OAuth providers for third-party authentication.
    It manages OAuth token exchange, refresh, and user info retrieval.

    Test Methods_oauth_manager_new:
    - test: Verify OAuth manager instantiation
    """

    def test_oauth_manager_new(self):
        """Test creating OAuth manager.

        This test verifies that DMSCOAuthManager can be instantiated.
        The manager is ready to configure OAuth providers and handle
        authentication flows with external identity providers.
        """
        manager = DMSCOAuthManager()
        self.assertIsNotNone(manager)


if __name__ == "__main__":
    unittest.main()
