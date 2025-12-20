//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!     http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! JWT (JSON Web Token) authentication implementation for DMSC.
//! 
//! This module provides JWT token generation, validation, and management functionality
//! for the DMSC authentication system. It supports custom claims, role-based access control,
//! and permission validation. The implementation uses HS256 algorithm for token signing
//! and verification.
//! 
//! # Design Principles
//! - **Security First**: Uses secure encoding/decoding keys and proper token validation
//! - **Flexibility**: Supports custom validation options and claims
//! - **Performance**: Caches encoding/decoding keys for efficient token operations
//! - **Extensibility**: Designed to support additional JWT algorithms and claim types
//! 
//! # Usage Examples
//! ```rust
//! // Create a JWT manager with a secret key and 1-hour expiry
//! let jwt_manager = DMSCJWTManager::new("secret_key".to_string(), 3600);
//! 
//! // Generate a token for a user with roles and permissions
//! let token = jwt_manager.generate_token(
//!     "user123",
//!     vec!["admin".to_string()],
//!     vec!["read", "write"].iter().map(|s| s.to_string()).collect()
//! )?;
//! 
//! // Validate the token
//! let claims = jwt_manager.validate_token(&token)?;
//! 
//! // Validate with custom options
//! let options = JWTValidationOptions {
//!     required_roles: vec!["admin"].iter().map(|s| s.to_string()).collect(),
//!     ..Default::default()
//! };
//! let claims = jwt_manager.validate_token_with_options(&token, options)?;
//! ```

#![allow(non_snake_case)]

#[cfg(feature = "auth")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};


#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// JWT claims structure containing user information and permissions.
/// 
/// This struct defines the standard claims for JWT tokens used in DMSC,
/// including subject, expiration time, issued time, roles, and permissions.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    pub sub: String, // Subject (user ID)
    pub exp: u64,      // Expiration time (UNIX timestamp)
    pub iat: u64,      // Issued at (UNIX timestamp)
    pub roles: Vec<String>,      // User roles for role-based access control
    pub permissions: Vec<String>, // User permissions for fine-grained access control
}

/// Options for validating JWT tokens.
/// 
/// This struct allows customization of JWT validation behavior, including
/// expiration validation, issued time validation, and required roles/permissions.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct JWTValidationOptions {
    pub validate_exp: bool,              // Whether to validate token expiration
    pub validate_iat: bool,              // Whether to validate token issued time
    pub required_roles: Vec<String>,     // Roles required for token validity
    pub required_permissions: Vec<String>, // Permissions required for token validity
}

impl Default for JWTValidationOptions {
    /// Creates default validation options with strict validation settings.
    /// 
    /// Default behavior:
    /// - Validate expiration time
    /// - Validate issued time
    /// - No required roles
    /// - No required permissions
    fn default() -> Self {
        Self {
            validate_exp: true,
            validate_iat: true,
            required_roles: vec![],
            required_permissions: vec![],
        }
    }
}

/// JWT manager for generating and validating tokens.
/// 
/// This struct manages JWT token operations, including generation, validation,
/// and refreshing. It uses HS256 algorithm for token signing and verification.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCJWTManager {
    secret: String,           // Secret key for token signing
    expiry_secs: u64,         // Default token expiry time in seconds
    #[cfg(feature = "auth")]
    encoding_key: EncodingKey, // Cached encoding key for performance
    #[cfg(feature = "auth")]
    decoding_key: DecodingKey, // Cached decoding key for performance
}

impl DMSCJWTManager {
    /// Creates a new JWT manager with the specified secret and expiry time.
    /// 
    /// # Parameters
    /// - `secret`: Secret key used for token signing and verification
    /// - `expiry_secs`: Default token expiration time in seconds
    /// 
    /// # Returns
    /// A new instance of `DMSCJWTManager`
    pub fn new(secret: String, expiry_secs: u64) -> Self {
        Self {
            secret,
            expiry_secs,
            #[cfg(feature = "auth")]
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            #[cfg(feature = "auth")]
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    /// Generates a new JWT token for a user.
    /// 
    /// # Parameters
    /// - `user_id`: Unique identifier for the user
    /// - `roles`: List of roles assigned to the user
    /// - `permissions`: List of permissions assigned to the user
    /// 
    /// # Returns
    /// A signed JWT token string if successful, otherwise an error
    #[cfg(feature = "auth")]
    pub fn generate_token(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> crate::core::DMSCResult<String> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let claims = JWTClaims {
            sub: user_id.to_string(),
            exp: now + self.expiry_secs,
            iat: now,
            roles,
            permissions,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| crate::core::DMSCError::Other(format!("JWT encoding error: {e}")))
    }
    
    #[cfg(not(feature = "auth"))]
    pub fn generate_token(&self, _user_id: &str, _roles: Vec<String>, _permissions: Vec<String>) -> crate::core::DMSCResult<String> {
        Err(crate::core::DMSCError::Other("JWT support is disabled. Enable the 'auth' feature to use JWT functionality.".to_string()))
    }

    /// Validates a JWT token with default validation settings.
    /// 
    /// # Parameters
    /// - `token`: JWT token string to validate
    /// 
    /// # Returns
    /// The decoded claims if the token is valid, otherwise an error
    #[cfg(feature = "auth")]
    pub fn validate_token(&self, token: &str) -> crate::core::DMSCResult<JWTClaims> {
        let validation = Validation::default();
        
        decode::<JWTClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| crate::core::DMSCError::Other(format!("JWT validation error: {e}")))
    }
    
    #[cfg(not(feature = "auth"))]
    pub fn validate_token(&self, _token: &str) -> crate::core::DMSCResult<JWTClaims> {
        Err(crate::core::DMSCError::Other("JWT support is disabled. Enable the 'auth' feature to use JWT functionality.".to_string()))
    }

    /// Validates a JWT token with custom validation options.
    /// 
    /// # Parameters
    /// - `token`: JWT token string to validate
    /// - `options`: Custom validation options
    /// 
    /// # Returns
    /// The decoded claims if the token is valid, otherwise an error
    #[cfg(feature = "auth")]
    pub fn validate_token_with_options(&self, token: &str, options: JWTValidationOptions) -> crate::core::DMSCResult<JWTClaims> {
        let claims = self.validate_token(token)?;

        if options.validate_exp {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if claims.exp < now {
                return Err(crate::core::DMSCError::Other("Token has expired".to_string()));
            }
        }

        if options.validate_iat {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if claims.iat > now {
                return Err(crate::core::DMSCError::Other("Token issued in future".to_string()));
            }
        }

        // Check required roles
        for required_role in &options.required_roles {
            if !claims.roles.contains(required_role) {
                return Err(crate::core::DMSCError::Other(format!("Missing required role: {required_role}")));
            }
        }

        // Check required permissions
        for required_permission in &options.required_permissions {
            if !claims.permissions.contains(required_permission) {
                return Err(crate::core::DMSCError::Other(format!("Missing required permission: {required_permission}")));
            }
        }

        Ok(claims)
    }
    
    #[cfg(not(feature = "auth"))]
    pub fn validate_token_with_options(&self, _token: &str, _options: JWTValidationOptions) -> crate::core::DMSCResult<JWTClaims> {
        Err(crate::core::DMSCError::Other("JWT support is disabled. Enable the 'auth' feature to use JWT functionality.".to_string()))
    }

    /// Refreshes an existing JWT token with a new expiration time.
    /// 
    /// # Parameters
    /// - `token`: Existing JWT token to refresh
    /// 
    /// # Returns
    /// A new JWT token with the same claims but updated expiration time
    #[cfg(feature = "auth")]
    pub fn refresh_token(&self, token: &str) -> crate::core::DMSCResult<String> {
        let claims = self.validate_token(token)?;
        
        // Generate new token with same user info
        self.generate_token(&claims.sub, claims.roles, claims.permissions)
    }
    
    #[cfg(not(feature = "auth"))]
    pub fn refresh_token(&self, _token: &str) -> crate::core::DMSCResult<String> {
        Err(crate::core::DMSCError::Other("JWT support is disabled. Enable the 'auth' feature to use JWT functionality.".to_string()))
    }

    /// Gets the default token expiry time in seconds.
    /// 
    /// # Returns
    /// The default token expiry time in seconds
    pub fn get_token_expiry(&self) -> u64 {
        self.expiry_secs
    }

    /// Gets the secret key used for token signing.
    /// 
    /// # Returns
    /// A reference to the secret key string
    pub fn get_secret(&self) -> &str {
        &self.secret
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCJWTManager
#[pyo3::prelude::pymethods]
impl DMSCJWTManager {
    #[new]
    fn py_new(secret: String, expiry_secs: u64) -> PyResult<Self> {
        Ok(Self::new(secret, expiry_secs))
    }
    
    /// Generate a JWT token from Python
    fn generate_token_py(&self, user_id: String, roles: Vec<String>, permissions: Vec<String>) -> PyResult<String> {
        match self.generate_token(&user_id, roles, permissions) {
            Ok(token) => Ok(token),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to generate token: {e}"))),
        }
    }
    
    /// Validate a JWT token from Python
    fn validate_token_py(&self, token: String) -> PyResult<JWTClaims> {
        match self.validate_token(&token) {
            Ok(claims) => Ok(claims),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to validate token: {e}"))),
        }
    }
    
    /// Get token expiry time from Python
    fn get_token_expiry_py(&self) -> u64 {
        self.get_token_expiry()
    }
}
