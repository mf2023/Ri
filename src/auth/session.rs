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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSSession {
    pub id: String,
    pub user_id: String,
    pub created_at: u64,
    pub last_accessed: u64,
    pub expires_at: u64,
    pub data: HashMap<String, String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

impl DMSSession {
    pub fn _Fnew(user_id: String, timeout_secs: u64, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            created_at: now,
            last_accessed: now,
            expires_at: now + timeout_secs,
            data: HashMap::new(),
            ip_address,
            user_agent,
        }
    }

    pub fn _Fis_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }

    pub fn _Ftouch(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.last_accessed = now;
    }

    pub fn _Fextend(&mut self, timeout_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at = now + timeout_secs;
    }

    pub fn _Fget_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn _Fset_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn _Fremove_data(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

pub struct DMSSessionManager {
    sessions: RwLock<HashMap<String, DMSSession>>,
    timeout_secs: u64,
    max_sessions_per_user: usize,
}

impl DMSSessionManager {
    pub fn _Fnew(timeout_secs: u64) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            timeout_secs,
            max_sessions_per_user: 5, // Default max 5 sessions per user
        }
    }

    pub async fn _Fcreate_session(&self, user_id: String, ip_address: Option<String>, user_agent: Option<String>) -> crate::core::DMSResult<String> {
        let mut sessions = self.sessions.write().await;
        
        // Check if user has too many sessions
        let user_sessions: Vec<_> = sessions.values()
            .filter(|s| s.user_id == user_id && !s._Fis_expired())
            .collect();
        
        if user_sessions.len() >= self.max_sessions_per_user {
            // Remove oldest session
            if let Some(oldest) = user_sessions.iter().min_by_key(|s| s.created_at) {
                sessions.remove(&oldest.id);
            }
        }

        // Create new session
        let session = DMSSession::_Fnew(user_id, self.timeout_secs, ip_address, user_agent);
        let session_id = session.id.clone();
        sessions.insert(session_id.clone(), session);
        
        Ok(session_id)
    }

    pub async fn _Fget_session(&self, session_id: &str) -> crate::core::DMSResult<Option<DMSSession>> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if session._Fis_expired() {
                sessions.remove(session_id);
                Ok(None)
            } else {
                session._Ftouch();
                Ok(Some(session.clone()))
            }
        } else {
            Ok(None)
        }
    }

    pub async fn _Fupdate_session(&self, session_id: &str, data: HashMap<String, String>) -> crate::core::DMSResult<bool> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if session._Fis_expired() {
                sessions.remove(session_id);
                Ok(false)
            } else {
                for (key, value) in data {
                    session._Fset_data(key, value);
                }
                session._Ftouch();
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub async fn _Fextend_session(&self, session_id: &str) -> crate::core::DMSResult<bool> {
        let mut sessions = self.sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if session._Fis_expired() {
                sessions.remove(session_id);
                Ok(false)
            } else {
                session._Fextend(self.timeout_secs);
                Ok(true)
            }
        } else {
            Ok(false)
        }
    }

    pub async fn _Fdestroy_session(&self, session_id: &str) -> crate::core::DMSResult<bool> {
        let mut sessions = self.sessions.write().await;
        Ok(sessions.remove(session_id).is_some())
    }

    pub async fn _Fdestroy_user_sessions(&self, user_id: &str) -> crate::core::DMSResult<usize> {
        let mut sessions = self.sessions.write().await;
        let mut count = 0;
        
        sessions.retain(|_, session| {
            if session.user_id == user_id {
                count += 1;
                false
            } else {
                true
            }
        });
        
        Ok(count)
    }

    pub async fn _Fget_user_sessions(&self, user_id: &str) -> crate::core::DMSResult<Vec<DMSSession>> {
        let sessions = self.sessions.read().await;
        
        let user_sessions: Vec<DMSSession> = sessions.values()
            .filter(|s| s.user_id == user_id && !s._Fis_expired())
            .cloned()
            .collect();
        
        Ok(user_sessions)
    }

    pub async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        let mut sessions = self.sessions.write().await;
        let mut count = 0;
        
        sessions.retain(|_, session| {
            if session._Fis_expired() {
                count += 1;
                false
            } else {
                true
            }
        });
        
        Ok(count)
    }

    pub async fn _Fcleanup_all(&self) -> crate::core::DMSResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.clear();
        Ok(())
    }

    pub fn _Fget_timeout(&self) -> u64 {
        self.timeout_secs
    }

    pub fn _Fset_timeout(&mut self, timeout_secs: u64) {
        self.timeout_secs = timeout_secs;
    }
}