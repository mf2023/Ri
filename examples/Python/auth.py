#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
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
DMSC Authentication Module Example.

This module demonstrates the DMSC authentication and authorization system.

The DMSC authentication module provides comprehensive authentication and
authorization functionality including:

- JWT (JSON Web Token) based authentication
- OAuth2 integration with multiple providers
- Session management with secure token handling
- Permission-based access control (RBAC)

Available Types:
- DMSCAuthModule: Main authentication module for service integration
- DMSCAuthConfig: Configuration for authentication module
- DMSCJWTManager: JWT token creation and validation
- DMSCOAuthManager: OAuth2 provider integration
- DMSCPermissionManager: Role-based permission management
- DMSCSessionManager: Session lifecycle management

Note: Some types require proper configuration and initialization through
the DMSC application framework. Direct instantiation may require specific
parameters or use of factory methods.

Usage:
    python auth.py
"""

from dmsc import (
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTManager,
    DMSCOAuthManager, DMSCPermissionManager, DMSCSessionManager
)


def main():
    """Main entry point for authentication module demonstration."""
    print("=== DMSC Authentication Module Example ===")
    print()
    
    print("Available authentication types:")
    auth_types = [
        ("DMSCAuthModule", "Main authentication module for service integration"),
        ("DMSCAuthConfig", "Configuration for authentication module"),
        ("DMSCJWTManager", "JWT token creation and validation"),
        ("DMSCOAuthManager", "OAuth2 provider integration"),
        ("DMSCPermissionManager", "Role-based permission management"),
        ("DMSCSessionManager", "Session lifecycle management")
    ]
    
    for type_name, description in auth_types:
        print(f"  - {type_name}: {description}")
    
    print()
    print("Authentication types imported successfully!")
    print()
    print("Note: Full functionality requires integration with DMSC application.")
    print("See Rust documentation for detailed usage patterns.")


if __name__ == "__main__":
    main()
