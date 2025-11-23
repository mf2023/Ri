//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTClaims {
    pub sub: String, // Subject (user ID)
    pub exp: u64,      // Expiration time
    pub iat: u64,      // Issued at
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct JWTValidationOptions {
    pub validate_exp: bool,
    pub validate_iat: bool,
    pub required_roles: Vec<String>,
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

pub struct DMSJWTManager {
    secret: String,
    expiry_secs: u64,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl DMSJWTManager {
    pub fn _Fnew(secret: String, expiry_secs: u64) -> Self {
        let encoding_key = EncodingKey::from_secret(secret.as_bytes());
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        
        Self {
            secret,
            expiry_secs,
            encoding_key,
            decoding_key,
        }
    }

    pub fn _Fgenerate_token(&self, user_id: &str, roles: Vec<String>, permissions: Vec<String>) -> crate::core::DMSResult<String> {
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
            .map_err(|e| crate::core::DMSError::Other(format!("JWT encoding error: {}", e)))
    }

    pub fn _Fvalidate_token(&self, token: &str) -> crate::core::DMSResult<JWTClaims> {
        let validation = Validation::default();
        
        decode::<JWTClaims>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|e| crate::core::DMSError::Other(format!("JWT validation error: {}", e)))
    }

    pub fn _Fvalidate_token_with_options(&self, token: &str, options: JWTValidationOptions) -> crate::core::DMSResult<JWTClaims> {
        let claims = self._Fvalidate_token(token)?;

        if options.validate_exp {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if claims.exp < now {
                return Err(crate::core::DMSError::Other("Token has expired".to_string()));
            }
        }

        if options.validate_iat {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if claims.iat > now {
                return Err(crate::core::DMSError::Other("Token issued in future".to_string()));
            }
        }

        // Check required roles
        for required_role in &options.required_roles {
            if !claims.roles.contains(required_role) {
                return Err(crate::core::DMSError::Other(format!("Missing required role: {}", required_role)));
            }
        }

        // Check required permissions
        for required_permission in &options.required_permissions {
            if !claims.permissions.contains(required_permission) {
                return Err(crate::core::DMSError::Other(format!("Missing required permission: {}", required_permission)));
            }
        }

        Ok(claims)
    }

    pub fn _Frefresh_token(&self, token: &str) -> crate::core::DMSResult<String> {
        let claims = self._Fvalidate_token(token)?;
        
        // Generate new token with same user info
        self._Fgenerate_token(&claims.sub, claims.roles, claims.permissions)
    }

    pub fn _Fget_token_expiry(&self) -> u64 {
        self.expiry_secs
    }

    pub fn _Fget_secret(&self) -> &str {
        &self.secret
    }
}