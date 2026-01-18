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
DMSC Authentication Module Example

This example demonstrates how to use the authentication module in DMSC,
including JWT token generation, validation, and session management.

Features Demonstrated:
- JWT token creation and validation
- Session management using session IDs
- Token revocation
- Permission-based access control
"""

import dmsc
from dmsc.auth import DMSCAuthModule, DMSCAuthConfig


def main():
    """
    Main entry point for the authentication module example.
    
    This function demonstrates the complete authentication workflow including:
    - Configuration of authentication settings
    - JWT token generation and validation
    - Session creation and management
    - Permission checking
    
    The example creates a complete authentication scenario showing how
    DMSC handles user authentication, session tracking, and access control.
    """
    print("=== DMSC Authentication Module Example ===\n")
    
    # Configuration Setup: Create authentication configuration with security settings
    # - enabled: Enable/disable authentication module
    # - jwt_secret: Secret key for JWT token signing (should be stored securely)
    # - jwt_expiry_secs: Token validity duration in seconds (1 hour)
    # - session_timeout_secs: Session timeout duration (8 hours)
    # - oauth_providers: List of OAuth providers for social login
    # - enable_api_keys: Enable API key-based authentication
    # - enable_session_auth: Enable session-based authentication
    auth_config = DMSCAuthConfig(
        enabled=True,
        jwt_secret="your-secret-key-here",
        jwt_expiry_secs=3600,
        session_timeout_secs=28800,
        oauth_providers=[],
        enable_api_keys=True,
        enable_session_auth=True,
    )
    
    # Module Initialization: Create authentication module instance with configuration
    # The module provides JWT management and session management capabilities
    auth_module = DMSCAuthModule(auth_config)
    
    # Get JWT manager for token operations (generation and validation)
    jwt_manager = auth_module.jwt_manager()
    
    # Token Operations: Generate and validate JWT tokens
    # Step 1: Generate a JWT token for a user with specific roles and permissions
    # - user_id: Subject identifier (typically username or user ID)
    # - roles: List of role names assigned to the user (e.g., "admin", "user")
    # - permissions: List of permission strings (e.g., "read:data", "write:data")
    print("1. Generating JWT token for user 'user123'...")
    token = jwt_manager.generate_token(
        "user123",
        ["admin", "user"],
        ["read:data", "write:data"],
    )
    print(f"   Token generated: {token[:50]}...\n")
    
    # Step 2: Validate the generated JWT token
    # Validates token signature and expiration, returns claims if valid
    # Claims include: sub (subject/user), exp (expiration), roles, permissions
    print("2. Validating JWT token...")
    claims = jwt_manager.validate_token(token)
    print(f"   Token valid for user: {claims.sub}")
    print(f"   Token expires at: {claims.exp}")
    print(f"   Token roles: {claims.roles}\n")
    
    # Session Management: Create and manage user sessions
    # Sessions provide stateful authentication tracking
    print("3. Creating user session...")
    session_manager = auth_module.session_manager()
    import asyncio
    
    async def create_and_manage_session():
        """
        Async helper function to demonstrate session management operations.
        
        This function handles the complete session lifecycle:
        - Session creation with user and connection details
        - Session retrieval by ID
        - Session expiration checking
        - Session extension for timeout renewal
        
        Args:
            None: Uses closure variables from main() scope
        
        Returns:
            None: Prints session management results directly
        """
        # Create a new session for user with connection metadata
        # Parameters:
        # - user_id: Identifier of the authenticated user
        # - ip_address: Client IP address for security tracking
        # - user_agent: Browser/client user agent string
        session_id = await session_manager.write().create_session(
            "user123",
            "192.168.1.100",
            "Mozilla/5.0",
        )
        print(f"   Session created with ID: {session_id}\n")
        
        # Retrieve session details using session ID
        # Used to validate existing sessions and get user information
        print("4. Retrieving session by ID...")
        session = await session_manager.read().get_session(session_id)
        if session:
            print(f"   Session found for user: {session.user_id}")
            print(f"   Session IP: {session.ip_address}")
            print(f"   Session expires at: {session.expires_at}\n")
            
            # Check if session is still valid (not expired)
            if not session.is_expired():
                print("5. Session is active and not expired\n")
        
        # Extend session timeout to prevent premature logout
        # Resets the session expiration timer
        print("6. Extending session...")
        await session_manager.write().extend_session(session_id)
        print("   Session extended successfully\n")
    
    # Execute the async session management demonstration
    asyncio.run(create_and_manage_session())
    
    # Permission Verification: Check user access rights
    # Demonstrates role-based access control (RBAC) implementation
    print("7. Checking permissions...")
    has_admin = "admin" in claims.roles
    has_superuser = "superuser" in claims.roles
    print(f"   Has 'admin' role: {has_admin}")
    print(f"   Has 'superuser' role: {has_superuser}\n")
    
    print("=== Authentication Example Completed ===")


if __name__ == "__main__":
    main()
