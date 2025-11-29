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

//! Session management implementation for DMS.
//! 
//! This module provides session management functionality, including:
//! - Session creation and validation
//! - Session expiration tracking
//! - User session management
//! - Session data storage
//! - Expired session cleanup
//! 
//! # Design Principles
//! - **Security**: Session IDs are generated using UUID v4 for uniqueness
//! - **Performance**: Efficient session lookup with hash maps
//! - **Flexibility**: Supports custom session timeout and data storage
//! - **Scalability**: Limits sessions per user to prevent resource exhaustion
//! - **Convenience**: Automatically cleans up expired sessions
//! - **Thread Safety**: Uses RwLock for concurrent access to session storage
//! 
//! # Usage Examples
//! ```rust
//! // Create a session manager with 30-minute timeout
//! let session_manager = DMSSessionManager::_Fnew(1800);
//! 
//! // Create a new session for a user
//! let session_id = session_manager._Fcreate_session(
//!     "user123".to_string(),
//!     Some("192.168.1.1".to_string()),
//!     Some("Mozilla/5.0".to_string())
//! ).await?;
//! 
//! // Get session data
//! let session = session_manager._Fget_session(&session_id).await?;
//! 
//! // Update session data
//! let mut data = HashMap::new();
//! data.insert("theme".to_string(), "dark".to_string());
//! session_manager._Fupdate_session(&session_id, data).await?;
//! 
//! // Destroy a session
//! session_manager._Fdestroy_session(&session_id).await?;
//! 
//! // Cleanup expired sessions
//! let cleaned_count = session_manager._Fcleanup_expired().await?;
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use uuid::Uuid;

/// Session structure for tracking user sessions.
/// 
/// This struct represents a user session with metadata, expiration tracking,
/// and custom data storage. Sessions are uniquely identified by UUIDs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSSession {
    pub id: String,               // Unique session ID (UUID v4)
    pub user_id: String,          // ID of the user associated with the session
    pub created_at: u64,          // Session creation time (UNIX timestamp)
    pub last_accessed: u64,       // Last time the session was accessed (UNIX timestamp)
    pub expires_at: u64,          // Session expiration time (UNIX timestamp)
    pub data: HashMap<String, String>, // Custom session data
    pub ip_address: Option<String>, // Client IP address
    pub user_agent: Option<String>, // Client user agent
}

impl DMSSession {
    /// Creates a new session for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user to create the session for
    /// - `timeout_secs`: Session timeout in seconds
    /// - `ip_address`: Optional client IP address
    /// - `user_agent`: Optional client user agent
    /// 
    /// # Returns
    /// A new instance of `DMSSession`
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

    /// Checks if the session has expired.
    /// 
    /// # Returns
    /// `true` if the session has expired, otherwise `false`
    pub fn _Fis_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        now > self.expires_at
    }

    /// Updates the last accessed time of the session.
    /// 
    /// This method is called when a session is accessed to update its
    /// last accessed timestamp, which can be used for session activity tracking.
    pub fn _Ftouch(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.last_accessed = now;
    }

    /// Extends the session expiration time.
    /// 
    /// # Parameters
    /// - `timeout_secs`: New timeout in seconds from the current time
    pub fn _Fextend(&mut self, timeout_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.expires_at = now + timeout_secs;
    }

    /// Gets a value from the session data.
    /// 
    /// # Parameters
    /// - `key`: Key to look up in the session data
    /// 
    /// # Returns
    /// `Some(&String)` if the key exists, otherwise `None`
    pub fn _Fget_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Sets a value in the session data.
    /// 
    /// # Parameters
    /// - `key`: Key to set in the session data
    /// - `value`: Value to associate with the key
    pub fn _Fset_data(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    /// Removes a value from the session data.
    /// 
    /// # Parameters
    /// - `key`: Key to remove from the session data
    /// 
    /// # Returns
    /// `Some(String)` if the key existed and was removed, otherwise `None`
    pub fn _Fremove_data(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

/// Session manager for handling user sessions.
/// 
/// This struct manages session creation, validation, and cleanup. It limits
/// the number of sessions per user and automatically cleans up expired sessions.
pub struct DMSSessionManager {
    sessions: RwLock<HashMap<String, DMSSession>>, // Session ID -> Session
    timeout_secs: u64,                          // Default session timeout in seconds
    max_sessions_per_user: usize,               // Maximum number of sessions per user
}

impl DMSSessionManager {
    /// Creates a new session manager with the specified timeout.
    /// 
    /// # Parameters
    /// - `timeout_secs`: Default session timeout in seconds
    /// 
    /// # Returns
    /// A new instance of `DMSSessionManager`
    /// 
    /// # Notes
    /// - Default maximum sessions per user is 5
    pub fn _Fnew(timeout_secs: u64) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            timeout_secs,
            max_sessions_per_user: 5, // Default max 5 sessions per user
        }
    }

    /// Creates a new session for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user to create the session for
    /// - `ip_address`: Optional client IP address
    /// - `user_agent`: Optional client user agent
    /// 
    /// # Returns
    /// The ID of the newly created session
    /// 
    /// # Notes
    /// - If the user has reached the maximum number of sessions, the oldest session is removed
    pub async fn _Fcreate_session(&self, user_id: String, ip_address: Option<String>, user_agent: Option<String>) -> crate::core::DMSResult<String> {
        let mut sessions = self.sessions.write().await;
        
        // Check if user has too many sessions
        let user_sessions: Vec<String> = sessions.values()
            .filter(|s| s.user_id == user_id && !s._Fis_expired())
            .map(|s| s.id.clone())
            .collect();
        
        if user_sessions.len() >= self.max_sessions_per_user {
            // Remove oldest session
            if let Some(oldest_id) = user_sessions.iter().min() {
                sessions.remove(oldest_id);
            }
        }

        // Create new session
        let session = DMSSession::_Fnew(user_id, self.timeout_secs, ip_address, user_agent);
        let session_id = session.id.clone();
        sessions.insert(session_id.clone(), session);
        
        Ok(session_id)
    }

    /// Gets a session by ID.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to retrieve
    /// 
    /// # Returns
    /// `Some(DMSSession)` if the session exists and is not expired, otherwise `None`
    /// 
    /// # Notes
    /// - Expired sessions are automatically removed and return `None`
    /// - The session's last accessed time is updated when retrieved
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

    /// Updates a session's data.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to update
    /// - `data`: HashMap of key-value pairs to update in the session
    /// 
    /// # Returns
    /// `true` if the session was updated successfully, `false` if the session doesn't exist or is expired
    /// 
    /// # Notes
    /// - The session's last accessed time is updated when modified
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

    /// Extends a session's expiration time.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to extend
    /// 
    /// # Returns
    /// `true` if the session was extended successfully, `false` if the session doesn't exist or is expired
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

    /// Destroys a session by ID.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to destroy
    /// 
    /// # Returns
    /// `true` if the session was destroyed successfully, `false` if the session doesn't exist
    pub async fn _Fdestroy_session(&self, session_id: &str) -> crate::core::DMSResult<bool> {
        let mut sessions = self.sessions.write().await;
        Ok(sessions.remove(session_id).is_some())
    }

    /// Destroys all sessions for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user whose sessions to destroy
    /// 
    /// # Returns
    /// The number of sessions destroyed
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

    /// Gets all active sessions for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user whose sessions to retrieve
    /// 
    /// # Returns
    /// A vector of active sessions for the user
    pub async fn _Fget_user_sessions(&self, user_id: &str) -> crate::core::DMSResult<Vec<DMSSession>> {
        let sessions = self.sessions.read().await;
        
        let user_sessions: Vec<DMSSession> = sessions.values()
            .filter(|s| s.user_id == user_id && !s._Fis_expired())
            .cloned()
            .collect();
        
        Ok(user_sessions)
    }

    /// Cleans up all expired sessions.
    /// 
    /// # Returns
    /// The number of expired sessions cleaned up
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

    /// Cleans up all sessions.
    /// 
    /// This method removes all sessions, regardless of their expiration status.
    pub async fn _Fcleanup_all(&self) -> crate::core::DMSResult<()> {
        let mut sessions = self.sessions.write().await;
        sessions.clear();
        Ok(())
    }

    /// Gets the default session timeout.
    /// 
    /// # Returns
    /// The default session timeout in seconds
    pub fn _Fget_timeout(&self) -> u64 {
        self.timeout_secs
    }

    /// Sets the default session timeout.
    /// 
    /// # Parameters
    /// - `timeout_secs`: New default session timeout in seconds
    pub fn _Fset_timeout(&mut self, timeout_secs: u64) {
        self.timeout_secs = timeout_secs;
    }
}