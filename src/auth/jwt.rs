//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

#![allow(non_snake_case)]

//! # JWT Authentication Module
//!
//! This module provides JSON Web Token (JWT) based authentication functionality
//! for the Ri framework. It includes JWT token generation, validation, and
//! claims management.
//!
//! ## JSON Web Tokens
//!
//! JWT is an open standard (RFC 7519) for securely transmitting information
//! between parties as a JSON object. This module implements JWT-based stateless
//! authentication, which is suitable for distributed systems and microservices
//! architectures where session persistence is challenging.
//!
//! ## Key Components
//!
//! - **RiJWTClaims**: Standard JWT claims including subject, expiration, issued at,
//!   roles, and permissions
//! - **RiJWTValidationOptions**: Configuration options for token validation
//! - **RiJWTManager**: Core manager for token generation and validation
//!
//! ## Token Structure
//!
//! A JWT consists of three parts separated by dots:
//!
//! 1. **Header**: Contains token type (JWT) and signing algorithm (HS256)
//! 2. **Payload**: Contains the claims (subject, expiration, roles, permissions)
//! 3. **Signature**: Verifies the token's integrity
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use ri::auth::jwt::RiJWTManager;
//!
//! fn authenticate_user() {
//!     let manager = RiJWTManager::create(
//!         "your-secret-key".to_string(),
//!         3600  // 1 hour expiry
//!     );
//!
//!     // Generate token
//!     let token = manager.generate_token(
//!         "user123",
//!         vec!["admin".to_string()],
//!         vec!["read".to_string(), "write".to_string()]
//!     );
//!
//!     // Validate token
//!     let claims = manager.validate_token(&token);
//!     println!("User: {}", claims.sub);
//!     println!("Roles: {:?}", claims.roles);
//! }
//! ```
//!
//! ## Security Considerations
//!
//! - **Secret Key**: Keep the secret key secure and never expose it in client code
//! - **Expiration**: Always set appropriate expiration times for tokens
//! - **HTTPS**: Transmit tokens only over HTTPS connections
//! - **Token Storage**: Store tokens securely on the client side
//!
//! ## Claims Reference
//!
//! - **sub (Subject)**: The user identifier or principal
//! - **exp (Expiration)**: Token expiration time in Unix timestamp
//! - **iat (Issued At)**: Token creation time in Unix timestamp
//! - **roles**: List of role identifiers assigned to the user
//! - **permissions**: List of permission identifiers granted to the subject

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::Zeroize;

use crate::core::error::RiError;

/// Represents the claims payload in a JWT token.
///
/// This structure contains all the standard and custom claims for a Ri JWT.
/// It follows the JWT standard specification with additional custom claims
/// for role-based access control (RBAC).
///
/// ## Standard Claims
///
/// - **sub**: Subject claim identifying the principal (user ID)
/// - **exp**: Expiration time claim (Unix timestamp)
/// - **iat**: Issued at claim (Unix timestamp)
///
/// ## Custom Claims
///
/// - **roles**: Role-based access control roles assigned to the subject
/// - **permissions**: Specific permissions granted to the subject
///
/// ## Serialization
///
/// This struct uses serde with custom field names to ensure compatibility
/// with standard JWT libraries across different programming languages.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RiJWTClaims {
    /// Subject claim - identifies the principal (user ID)
    #[serde(rename = "sub")]
    pub sub: String,

    /// Expiration time claim - Unix timestamp when the token expires
    #[serde(rename = "exp")]
    pub exp: u64,

    /// Issued at claim - Unix timestamp when the token was created
    #[serde(rename = "iat")]
    pub iat: u64,

    /// Custom claim - list of role identifiers for RBAC
    #[serde(rename = "roles")]
    pub roles: Vec<String>,

    /// Custom claim - list of permission identifiers for fine-grained access control
    #[serde(rename = "permissions")]
    pub permissions: Vec<String>,
}

/// Configuration options for JWT token validation.
///
/// This structure provides configurable validation parameters that control
/// how tokens are validated during the authentication process. Default values
/// are provided for all options, making the struct suitable for common use cases.
///
/// ## Validation Options
///
/// - **validate_exp**: Verify the expiration claim is valid (not expired)
/// - **validate_iat**: Verify the issued-at claim is valid (not issued in future)
/// - **required_roles**: Minimum roles required for token to be valid
/// - **required_permissions**: Minimum permissions required for token to be valid
///
/// ## Usage
///
/// ```rust,ignore
/// use ri::auth::jwt::RiJWTValidationOptions;
///
/// let options = RiJWTValidationOptions {
///     validate_exp: true,
///     validate_iat: true,
///     required_roles: vec!["user".to_string()],
///     required_permissions: vec!["read".to_string()],
/// };
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RiJWTValidationOptions {
    /// Whether to validate the expiration time claim
    pub validate_exp: bool,

    /// Whether to validate the issued-at time claim
    pub validate_iat: bool,

    /// Minimum roles required for the token to be valid
    pub required_roles: Vec<String>,

    /// Minimum permissions required for the token to be valid
    pub required_permissions: Vec<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiJWTValidationOptions {
    #[new]
    fn py_new(
        validate_exp: bool,
        validate_iat: bool,
        required_roles: Vec<String>,
        required_permissions: Vec<String>,
    ) -> Self {
        Self {
            validate_exp,
            validate_iat,
            required_roles,
            required_permissions,
        }
    }
}

impl Default for RiJWTValidationOptions {
    fn default() -> Self {
        Self {
            validate_exp: true,
            validate_iat: true,
            required_roles: vec![],
            required_permissions: vec![],
        }
    }
}

/// Core JWT management structure.
///
/// The `RiJWTManager` handles all JWT-related operations including token
/// generation, validation, and secret key management. It uses the HS256
/// (HMAC SHA-256) algorithm for signing tokens.
///
/// ## Thread Safety
///
/// This structure is designed to be shared across threads when wrapped in
/// an Arc. All methods are stateless regarding the token content and only
/// read the configuration (secret and expiry).
///
/// ## Algorithm
///
/// Uses HMAC-SHA256 (HS256) for token signing. This symmetric algorithm
/// uses the same secret key for both signing and verification.
///
/// ## Security
///
/// The secret key is protected with zeroize to ensure it is securely cleared
/// from memory when the manager is dropped, preventing memory dump attacks.
///
/// ## Performance
///
/// Token generation and validation are designed to be fast operations.
/// The encoding/decoding operations are primarily CPU-bound due to the
/// HMAC computation.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiJWTManager {
    /// The secret key used for signing and verifying tokens
    /// This field is securely zeroized on drop
    secret: String,

    /// Default expiry time in seconds for generated tokens
    expiry_secs: u64,

    /// Pre-computed encoding key for faster token generation
    encoding_key: EncodingKey,

    /// Pre-computed decoding key for faster token validation
    decoding_key: DecodingKey,
}

impl Drop for RiJWTManager {
    fn drop(&mut self) {
        // Securely zeroize the secret key to prevent memory dump attacks
        self.secret.zeroize();
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiJWTManager {
    /// Creates a new JWT manager with the specified secret and expiry time.
    ///
    /// This constructor is used for Python bindings and creates a JWT manager
    /// that can generate and validate tokens with the given configuration.
    ///
    /// # Parameters
    ///
    /// - `secret`: The secret key used for signing and verifying JWT tokens
    /// - `expiry_secs`: The default expiry time in seconds for generated tokens
    ///
    /// # Returns
    ///
    /// A new instance of `RiJWTManager`
    #[new]
    pub fn new(secret: String, expiry_secs: u64) -> Self {
        let secret_bytes = secret.as_bytes().to_vec();
        Self {
            secret,
            expiry_secs,
            encoding_key: EncodingKey::from_secret(&secret_bytes),
            decoding_key: DecodingKey::from_secret(&secret_bytes),
        }
    }

    /// Generates a new JWT token for the specified user with roles and permissions.
    ///
    /// # Parameters
    ///
    /// - `user_id`: The unique identifier of the user (subject claim)
    /// - `roles`: A list of role identifiers assigned to the user
    /// - `permissions`: A list of permission identifiers granted to the user
    ///
    /// # Returns
    ///
    /// The encoded JWT token string
    pub fn py_generate_token(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> pyo3::prelude::PyResult<String> {
        self.generate_token(user_id, roles, permissions).map_err(crate::auth::security::ri_error_to_py_err)
    }

    /// Validates a JWT token and returns the decoded claims.
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to validate
    ///
    /// # Returns
    ///
    /// The decoded RiJWTClaims if validation succeeds
    pub fn py_validate_token(&self, token: &str) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::PyAny>> {
        use pyo3::prelude::*;
        
        Python::with_gil(|py| {
            self.validate_token(token)
                .map_err(crate::auth::security::ri_error_to_py_err)
                .map(|claims| {
                    Py::new(py, claims).unwrap().into_py(py)
                })
        })
    }

    /// Returns the default token expiry time in seconds.
    pub fn py_get_token_expiry(&self) -> u64 {
        self.expiry_secs
    }

    /// Generates a new JWT token for the specified user with roles and permissions.
    ///
    /// This is an alias for `py_generate_token` providing a more Pythonic API.
    ///
    /// # Parameters
    ///
    /// - `user_id`: The unique identifier of the user (subject claim)
    /// - `roles`: A list of role identifiers assigned to the user
    /// - `permissions`: A list of permission identifiers granted to the user
    ///
    /// # Returns
    ///
    /// The encoded JWT token string
    #[pyo3(name = "generate_token")]
    pub fn generate_token_py(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> pyo3::prelude::PyResult<String> {
        self.py_generate_token(user_id, roles, permissions)
    }

    /// Validates a JWT token and returns the decoded claims.
    ///
    /// This is an alias for `py_validate_token` providing a more Pythonic API.
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to validate
    ///
    /// # Returns
    ///
    /// The decoded RiJWTClaims if validation succeeds
    #[pyo3(name = "validate_token")]
    pub fn validate_token_py(&self, token: &str) -> pyo3::prelude::PyResult<pyo3::Py<pyo3::PyAny>> {
        self.py_validate_token(token)
    }

    /// Returns the default token expiry time in seconds.
    ///
    /// This is an alias for `py_get_token_expiry` providing a more Pythonic API.
    #[pyo3(name = "get_token_expiry")]
    pub fn get_token_expiry_py(&self) -> u64 {
        self.py_get_token_expiry()
    }
}

impl RiJWTManager {
    /// Creates a new JWT manager with the specified secret and expiry time.
    ///
    /// This is the primary constructor for creating a JWT manager. It initializes
    /// the manager with a secret key and default token expiry time. The secret key
    /// is used for both signing new tokens and validating existing ones.
    ///
    /// ## Performance
    ///
    /// This constructor pre-computes the encoding and decoding keys for optimal
    /// performance during token generation and validation operations.
    ///
    /// # Parameters
    ///
    /// - `secret`: The secret key used for signing and verifying JWT tokens
    /// - `expiry_secs`: The default expiry time in seconds for generated tokens
    ///
    /// # Returns
    ///
    /// A new instance of `RiJWTManager`
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::jwt::RiJWTManager;
    ///
    /// let manager = RiJWTManager::create(
    ///     "your-secret-key".to_string(),
    ///     3600  // 1 hour expiry
    /// );
    /// ```
    pub fn create(secret: String, expiry_secs: u64) -> Self {
        let secret_bytes = secret.as_bytes().to_vec();
        Self {
            secret,
            expiry_secs,
            encoding_key: EncodingKey::from_secret(&secret_bytes),
            decoding_key: DecodingKey::from_secret(&secret_bytes),
        }
    }

    /// Generates a new JWT token for the specified user with roles and permissions.
    ///
    /// This method creates a signed JWT token containing the user's subject identifier,
    /// assigned roles, and permissions. The token is signed using HMAC-SHA256 algorithm.
    ///
    /// ## Token Claims
    ///
    /// The generated token includes the following claims:
    /// - `sub`: The user identifier
    /// - `exp`: Expiration time (current time + expiry_secs)
    /// - `iat`: Issued at time (current time)
    /// - `roles`: List of role identifiers
    /// - `permissions`: List of permission identifiers
    ///
    /// # Parameters
    ///
    /// - `user_id`: The unique identifier of the user (subject claim)
    /// - `roles`: A vector of role identifiers assigned to the user
    /// - `permissions`: A vector of permission identifiers granted to the user
    ///
    /// # Returns
    ///
    /// A Result containing the encoded JWT token string, or a RiError if encoding fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::jwt::RiJWTManager;
    ///
    /// let manager = RiJWTManager::create("secret".to_string(), 3600);
    ///
    /// let token = manager.generate_token(
    ///     "user123",
    ///     vec!["admin".to_string()],
    ///     vec!["read:data".to_string(), "write:data".to_string()]
    /// );
    ///
    /// match token {
    ///     Ok(t) => println!("Generated token: {}", t),
    ///     Err(e) => println!("Failed to generate token: {:?}", e),
    /// }
    /// ```
    pub fn generate_token(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> Result<String, RiError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| RiError::Other(format!("System time error: {}", e)))?
            .as_secs();

        let claims = RiJWTClaims {
            sub: user_id.to_string(),
            exp: now + self.expiry_secs,
            iat: now,
            roles,
            permissions,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| RiError::Other(format!("JWT encoding failed: {}", e)))
    }

    /// Validates a JWT token and returns the decoded claims.
    ///
    /// This method verifies the token's signature and decodes the claims payload.
    /// It validates the token structure and signature using the configured secret key.
    ///
    /// ## Validation Performed
    ///
    /// - Verifies the token signature using HMAC-SHA256
    /// - Validates the token structure (header, payload, signature)
    /// - Checks token expiration if validation is enabled
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to validate
    ///
    /// # Returns
    ///
    /// A Result containing the decoded RiJWTClaims if validation succeeds,
    /// or a RiError if validation fails (invalid signature, expired token, etc.)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::jwt::RiJWTManager;
    ///
    /// let manager = RiJWTManager::create("secret".to_string(), 3600);
    ///
    /// // First generate a token
    /// let token = manager.generate_token("user123", vec![], vec![]).unwrap();
    ///
    /// // Then validate it
    /// let claims = manager.validate_token(&token);
    ///
    /// match claims {
    ///     Ok(c) => println!("User: {}, Roles: {:?}", c.sub, c.roles),
    ///     Err(e) => println!("Invalid token: {:?}", e),
    /// }
    /// ```
    pub fn validate_token(&self, token: &str) -> Result<RiJWTClaims, RiError> {
        let mut validation = Validation::default();
        validation.set_required_spec_claims(&["exp"]);
        validation.algorithms = vec![jsonwebtoken::Algorithm::HS256];
        
        decode::<RiJWTClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| {
                // Security: Log detailed error internally, return generic message
                log::warn!("[Ri.JWT] Token validation failed: {}", e);
                RiError::Other("Invalid token".to_string())
            })
            .map(|token_data| token_data.claims)
    }

    /// Returns the default token expiry time in seconds.
    ///
    /// This method returns the configured default expiry time that is used
    /// when generating new tokens.
    ///
    /// # Returns
    ///
    /// The default token expiry time in seconds
    pub fn get_token_expiry(&self) -> u64 {
        self.expiry_secs
    }

    /// Returns a fingerprint of the secret key for audit purposes.
    ///
    /// This method returns a SHA-256 hash of the secret key prefix,
    /// useful for identifying which key was used for signing without
    /// exposing the actual secret.
    ///
    /// # Returns
    ///
    /// A hexadecimal string representing the key fingerprint
    pub fn get_key_fingerprint(&self) -> String {
        use sha2::{Sha256, Digest};
        let prefix = if self.secret.len() > 8 {
            &self.secret[..8]
        } else {
            &self.secret
        };
        let mut hasher = Sha256::new();
        hasher.update(prefix.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
