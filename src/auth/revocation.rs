//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

const DEFAULT_MAX_REVOKED_TOKENS: usize = 10000;

#[derive(Debug, Clone)]
pub struct RevokedTokenInfo {
    pub token_id: String,
    pub user_id: String,
    pub revoked_at: u64,
    pub expires_at: u64,
    pub reason: Option<String>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct JWTRevocationList {
    revoked_tokens: DashSet<String>,
    token_info: DashMap<String, RevokedTokenInfo>,
    max_tokens: usize,
}

use dashmap::DashMap;

impl JWTRevocationList {
    pub fn new() -> Self {
        Self {
            revoked_tokens: DashSet::new(),
            token_info: DashMap::new(),
            max_tokens: DEFAULT_MAX_REVOKED_TOKENS,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            revoked_tokens: DashSet::with_capacity(capacity),
            token_info: DashMap::with_capacity(capacity),
            max_tokens: capacity,
        }
    }

    pub fn revoke_token(
        &self,
        token: &str,
        user_id: &str,
        reason: Option<String>,
        ttl_secs: u64,
    ) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        self.revoked_tokens.insert(token.to_string());

        let info = RevokedTokenInfo {
            token_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            revoked_at: now,
            expires_at: now + ttl_secs,
            reason,
        };

        self.token_info.insert(token.to_string(), info);

        self.cleanup_expired();
    }

    pub fn revoke_all_user_tokens(&self, user_id: &str, reason: Option<String>) -> usize {
        let mut count = 0;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        for entry in self.token_info.iter() {
            let info = entry.value();
            if info.user_id == user_id {
                self.revoked_tokens.insert(entry.key().clone());

                let updated_info = RevokedTokenInfo {
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

    pub fn is_revoked(&self, token: &str) -> bool {
        if self.revoked_tokens.contains(token) {
            if let Some(info) = self.token_info.get(token) {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if now > info.expires_at {
                    self.remove_revoked_token(token);
                    return false;
                }
            }
            return true;
        }
        false
    }

    pub fn get_revocation_info(&self, token: &str) -> Option<RevokedTokenInfo> {
        self.token_info.get(token).map(|i| i.clone())
    }

    fn remove_revoked_token(&self, token: &str) {
        self.revoked_tokens.remove(token);
        self.token_info.remove(token);
    }

    fn cleanup_expired(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

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

    pub fn get_revoked_count(&self) -> usize {
        self.revoked_tokens.len()
    }

    pub fn clear(&self) {
        self.revoked_tokens.clear();
        self.token_info.clear();
    }
}

impl Default for JWTRevocationList {
    fn default() -> Self {
        Self::new()
    }
}
