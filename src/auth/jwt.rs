//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
//! for the DMSC framework. It includes JWT token generation, validation, and
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
//! - **JWTClaims**: Standard JWT claims including subject, expiration, issued at,
//!   roles, and permissions
//! - **JWTValidationOptions**: Configuration options for token validation
//! - **DMSCJWTManager**: Core manager for token generation and validation
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
//! ```rust
//! use dmsc::auth::jwt::DMSCJWTManager;
//!
//! fn authenticate_user() {
//!     let manager = DMSCJWTManager::create(
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

#[cfg(feature = "pyo3")]
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::error::DMSCError;

/// Represents the claims payload in a JWT token.
///
/// This structure contains all the standard and custom claims for a DMSC JWT.
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
pub struct JWTClaims {
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
/// ```rust
/// use dmsc::auth::jwt::JWTValidationOptions;
///
/// let options = JWTValidationOptions {
///     validate_exp: true,
///     validate_iat: true,
///     required_roles: vec!["user".to_string()],
///     required_permissions: vec!["read".to_string()],
/// };
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct JWTValidationOptions {
    /// Whether to validate the expiration time claim
    pub validate_exp: bool,

    /// Whether to validate the issued-at time claim
    pub validate_iat: bool,

    /// Minimum roles required for the token to be valid
    pub required_roles: Vec<String>,

    /// Minimum permissions required for the token to be valid
    pub required_permissions: Vec<String>,
}

impl Default for JWTValidationOptions {
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
/// The `DMSCJWTManager` handles all JWT-related operations including token
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
/// ## Performance
///
/// Token generation and validation are designed to be fast operations.
/// The encoding/decoding operations are primarily CPU-bound due to the
/// HMAC computation.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCJWTManager {
    /// The secret key used for signing and verifying tokens
    secret: String,

    /// Default expiry time in seconds for generated tokens
    expiry_secs: u64,

    /// Pre-computed encoding key for faster token generation
    encoding_key: EncodingKey,

    /// Pre-computed decoding key for faster token validation
    decoding_key: DecodingKey,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCJWTManager {
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
}

impl DMSCJWTManager {
    pub fn create(secret: String, expiry_secs: u64) -> Self {
        let secret_bytes = secret.as_bytes().to_vec();
        Self {
            secret,
            expiry_secs,
            encoding_key: EncodingKey::from_secret(&secret_bytes),
            decoding_key: DecodingKey::from_secret(&secret_bytes),
        }
    }

    pub fn generate_token(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> Result<String, DMSCError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DMSCError::Other(format!("System time error: {}", e)))?
            .as_secs();

        let claims = JWTClaims {
            sub: user_id.to_string(),
            exp: now + self.expiry_secs,
            iat: now,
            roles,
            permissions,
        };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| DMSCError::Other(format!("JWT encoding failed: {}", e)))
    }

    pub fn validate_token(&self, token: &str) -> Result<JWTClaims, DMSCError> {
        let validation = Validation::default();
        decode::<JWTClaims>(token, &self.decoding_key, &validation)
            .map_err(|e| DMSCError::Other(format!("JWT decoding failed: {}", e)))
            .map(|token_data| token_data.claims)
    }

    pub fn get_token_expiry(&self) -> u64 {
        self.expiry_secs
    }

    pub fn get_secret(&self) -> &str {
        &self.secret
    }
}
