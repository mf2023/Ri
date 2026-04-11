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

//! # JWT Token Revocation Module
//!
//! This module provides JWT token revocation functionality, including:
//! - Token blacklist management
//! - User-based token revocation
//! - Revoked token validation
//!
//! ## Security Considerations
//!
//! - Revoked tokens are stored in-memory and will be lost on restart
//! - For production use, integrate with Redis or database-backed storage
//! - Consider implementing token versioning for per-user revocation

use dashmap::DashSet;
use std::time::{SystemTime, UNIX_EPOCH};
use std::vec::Vec;
use uuid::Uuid;

/// Information about a revoked JWT token.
///
/// This struct stores metadata about a token that has been revoked,
/// including when it was revoked, when it expires, and the reason for revocation.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
#[derive(Debug, Clone)]
pub struct RiRevokedTokenInfo {
    /// Unique identifier for the revocation record
    pub token_id: String,
    /// User ID associated with the revoked token
    pub user_id: String,
    /// Unix timestamp when the token was revoked
    pub revoked_at: u64,
    /// Unix timestamp when the token expires (may differ from original token expiry)
    pub expires_at: u64,
    /// Optional reason for revocation (e.g., "user_logout", "security_breach")
    pub reason: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiRevokedTokenInfo {
    #[new]
    fn py_new(
        token_id: String,
        user_id: String,
        revoked_at: u64,
        expires_at: u64,
        reason: Option<String>,
    ) -> Self {
        Self {
            token_id,
            user_id,
            revoked_at,
            expires_at,
            reason,
        }
    }
}

/// JWT token revocation list for managing invalidated tokens.
///
/// This struct provides functionality to revoke JWT tokens and check if tokens
/// have been revoked. It uses concurrent data structures for thread-safe access.
///
/// ## Usage
///
/// The revocation list can be used to implement token invalidation scenarios:
/// - User logout (revoke specific token)
/// - Password change (revoke all user tokens)
/// - Security incidents (bulk revocation)
///
/// ## Storage
///
/// By default, revoked tokens are stored in-memory. For production use,
/// consider integrating with Redis or a database-backed storage solution
/// to persist revocations across application restarts.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiJWTRevocationList {
    /// Set of revoked token strings for O(1) lookup
    revoked_tokens: DashSet<String>,
    /// Map of token string to RiRevokedTokenInfo for metadata storage
    token_info: DashMap<String, RiRevokedTokenInfo>,
    /// Maximum number of revoked tokens to store
    max_tokens: usize,
}

use dashmap::DashMap;

/// Default maximum number of revoked tokens to store in the list.
const DEFAULT_MAX_REVOKED_TOKENS: usize = 10000;

impl RiJWTRevocationList {
    /// Creates a new JWT revocation list with default capacity.
    ///
    /// This constructor initializes an empty revocation list with the default
    /// maximum capacity for storing revoked tokens.
    ///
    /// # Returns
    ///
    /// A new instance of `RiJWTRevocationList`
    pub fn new() -> Self {
        Self {
            revoked_tokens: DashSet::new(),
            token_info: DashMap::new(),
            max_tokens: DEFAULT_MAX_REVOKED_TOKENS,
        }
    }

    /// Creates a new JWT revocation list with specified capacity.
    ///
    /// This constructor allows specifying the maximum number of revoked tokens
    /// that can be stored. When the capacity is exceeded, oldest revoked tokens
    /// are automatically removed.
    ///
    /// # Parameters
    ///
    /// - `capacity`: Maximum number of revoked tokens to store
    ///
    /// # Returns
    ///
    /// A new instance of `RiJWTRevocationList` with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            revoked_tokens: DashSet::with_capacity(capacity),
            token_info: DashMap::with_capacity(capacity),
            max_tokens: capacity,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiJWTRevocationList {
    #[new]
    fn py_new() -> Self {
        Self {
            revoked_tokens: DashSet::new(),
            token_info: DashMap::new(),
            max_tokens: DEFAULT_MAX_REVOKED_TOKENS,
        }
    }

    #[pyo3(name = "get_revoked_count")]
    fn get_revoked_count_impl(&self) -> usize {
        self.revoked_tokens.len()
    }

    #[pyo3(name = "is_revoked")]
    fn is_revoked_impl(&self, token: &str) -> bool {
        self.revoked_tokens.contains(token)
    }

    #[pyo3(name = "revoke_token")]
    fn revoke_token_impl(&self, token: String, user_id: String, reason: Option<String>, ttl_secs: u64) {
        self.revoke_token(&token, &user_id, reason, ttl_secs);
    }

    #[pyo3(name = "revoke_by_user")]
    fn revoke_by_user_impl(&self, user_id: &str) -> bool {
        let mut removed = false;
        
        let to_remove: Vec<String> = self.token_info
            .iter()
            .filter(|x| x.user_id == user_id)
            .map(|x| x.token_id.clone())
            .collect();

        for token in to_remove {
            self.revoked_tokens.remove(&token);
            self.token_info.remove(&token);
            removed = true;
        }
        removed
    }

    #[pyo3(name = "cleanup")]
    fn cleanup_impl(&self) -> usize {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expired: Vec<String> = self.token_info
            .iter()
            .filter(|x| x.expires_at <= now)
            .map(|x| x.token_id.clone())
            .collect();

        for token in &expired {
            self.revoked_tokens.remove(token);
            self.token_info.remove(token);
        }
        expired.len()
    }
}

impl RiJWTRevocationList {
     /// Revokes a specific JWT token.
     ///
     /// This method adds a token to the revocation list with associated metadata.
     /// The token will be considered invalid for the specified time-to-live duration.
     ///
     /// ## Automatic Cleanup
     ///
     /// After revocation, expired tokens are automatically cleaned up if the
     /// revocation list exceeds its maximum capacity.
     ///
     /// # Parameters
     ///
     /// - `token`: The JWT token string to revoke
     /// - `user_id`: The user ID associated with the token
     /// - `reason`: Optional reason for revocation
     /// - `ttl_secs`: Time-to-live in seconds for this revocation record
     pub fn revoke_token(
         &self,
         token: &str,
         user_id: &str,
         reason: Option<String>,
         ttl_secs: u64,
     ) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        self.revoked_tokens.insert(token.to_string());

        let info = RiRevokedTokenInfo {
            token_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            revoked_at: now,
            expires_at: now + ttl_secs,
            reason,
        };

        self.token_info.insert(token.to_string(), info);

        self.cleanup_expired();
    }

    /// Revokes all tokens for a specific user.
    ///
    /// This method finds all tokens associated with the given user ID and
    /// marks them as revoked. Useful for implementing "logout everywhere"
    /// functionality or revoking tokens after a security incident.
    ///
    /// # Parameters
    ///
    /// - `user_id`: The user ID whose tokens should be revoked
    /// - `reason`: Optional reason for the mass revocation
    ///
    /// # Returns
    ///
    /// The number of tokens that were revoked
    pub fn revoke_all_user_tokens(&self, user_id: &str, reason: Option<String>) -> usize {
        let mut count = 0;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        for entry in self.token_info.iter() {
            let info = entry.value();
            if info.user_id == user_id {
                self.revoked_tokens.insert(entry.key().clone());

                let updated_info = RiRevokedTokenInfo {
                    token_id: info.token_id.clone(),
                    user_id: info.user_id.clone(),
                    revoked_at: info.revoked_at,
                    expires_at: now + 86400,
                    reason: reason.clone(),
                };

                self.token_info.insert(entry.key().clone(), updated_info);
                count += 1;
            }
        }

        count
    }

    /// Checks if a token has been revoked.
    ///
    /// This method performs an O(1) lookup to determine if a token exists
    /// in the revocation list. If found, it also checks if the revocation
    /// record has expired and removes it if so.
    ///
    /// ## Expiration Handling
    ///
    /// If the token is found but its revocation record has expired,
    /// the token is automatically removed and treated as not revoked.
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to check
    ///
    /// # Returns
    ///
    /// `true` if the token is revoked and valid, `false` otherwise
    pub fn is_revoked(&self, token: &str) -> bool {
        if self.revoked_tokens.contains(token) {
            if let Some(info) = self.token_info.get(token) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_or(0, |d| d.as_secs());

                if now > info.expires_at {
                    self.remove_revoked_token(token);
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// Retrieves revocation information for a specific token.
    ///
    /// This method returns the metadata associated with a revoked token,
    /// including when it was revoked and the reason (if provided).
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to look up
    ///
    /// # Returns
    ///
    /// `Some(RiRevokedTokenInfo)` if the token is revoked, `None` otherwise
    pub fn get_revocation_info(&self, token: &str) -> Option<RiRevokedTokenInfo> {
        self.token_info.get(token).map(|i| i.clone())
    }

    /// Removes a single revoked token from the list.
    ///
    /// This is an internal method used for cleanup operations.
    ///
    /// # Parameters
    ///
    /// - `token`: The JWT token string to remove
    fn remove_revoked_token(&self, token: &str) {
        self.revoked_tokens.remove(token);
        self.token_info.remove(token);
    }

    /// Removes all expired revocation records from the list.
    ///
    /// This internal method is called after token revocation to clean up
    /// expired entries and enforce the maximum capacity limit.
    ///
    /// ## Cleanup Criteria
    ///
    /// - Removes all revocation records where the expiry time has passed
    /// - If capacity is exceeded, removes oldest entries first
    fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        let tokens_to_remove: Vec<String> = self
            .token_info
            .iter()
            .filter(|entry| entry.expires_at <= now)
            .map(|entry| entry.key().clone())
            .collect();

        for token in tokens_to_remove {
            self.remove_revoked_token(&token);
        }

        while self.revoked_tokens.len() > self.max_tokens {
            if let Some(entry) = self.token_info.iter().next() {
                self.remove_revoked_token(entry.key());
            } else {
                break;
            }
        }
    }

    /// Returns the current count of revoked tokens.
    ///
    /// This method provides the number of tokens currently in the revocation list.
    ///
    /// # Returns
    ///
    /// The number of revoked tokens stored
    pub fn get_revoked_count(&self) -> usize {
        self.revoked_tokens.len()
    }

    /// Clears all revoked tokens from the list.
    ///
    /// This method removes all entries from both the revoked tokens set
    /// and the token info map. Use with caution as it cannot be undone.
    pub fn clear(&self) {
        self.revoked_tokens.clear();
        self.token_info.clear();
    }
}

impl Default for RiJWTRevocationList {
    fn default() -> Self {
        Self::new()
    }
}
