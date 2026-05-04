//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! Session management implementation for Ri.
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
//! let session_manager = RiSessionManager::new(1800);
//! 
//! // Create a new session for a user
//! let session_id = session_manager.create_session(
//!     "user123".to_string(),
//!     Some("192.168.1.1".to_string()),
//!     Some("Mozilla/5.0".to_string())
//! ).await?;
//! 
//! // Get session data
//! let session = session_manager.get_session(&session_id).await?;
//! 
//! // Update session data
//! let mut data = FxHashMap::default();
//! data.insert("theme".to_string(), "dark".to_string());
//! session_manager.update_session(&session_id, data).await?;
//! 
//! // Destroy a session
//! session_manager.destroy_session(&session_id).await?;
//! 
//! // Cleanup expired sessions
//! let cleaned_count = session_manager.cleanup_expired().await?;
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap as FxHashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use crate::core::concurrent::RiShardedLock;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Maximum number of data entries in a session
const MAX_SESSION_DATA_ENTRIES: usize = 100;

/// Maximum length of a session data key
const MAX_SESSION_DATA_KEY_LENGTH: usize = 256;

/// Maximum length of a session data value
const MAX_SESSION_DATA_VALUE_LENGTH: usize = 4096;

/// Maximum length of user agent string
#[allow(dead_code)]
const MAX_USER_AGENT_LENGTH: usize = 1024;

/// Session structure for tracking user sessions.
///
/// This struct represents a user session with metadata, expiration tracking,
/// and custom data storage. Sessions are uniquely identified by UUIDs.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiSession {
    /// Unique session identifier generated using UUID v4
    pub id: String,
    /// User ID associated with this session
    pub user_id: String,
    /// Session creation time as Unix timestamp
    pub created_at: u64,
    /// Last access time as Unix timestamp (updated on each access)
    pub last_accessed: u64,
    /// Session expiration time as Unix timestamp
    pub expires_at: u64,
    /// Custom key-value data associated with the session
    pub data: FxHashMap<String, String>,
    /// Client IP address from which the session was created
    pub ip_address: Option<String>,
    /// Client user agent string from which the session was created
    pub user_agent: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiSession {
    #[new]
    fn py_new(
        id: Option<String>,
        user_id: String,
        created_at: Option<u64>,
        last_accessed: Option<u64>,
        expires_at: Option<u64>,
        data: Option<FxHashMap<String, String>>,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        
        Self {
            id: id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            user_id,
            created_at: created_at.unwrap_or(now),
            last_accessed: last_accessed.unwrap_or(now),
            expires_at: expires_at.unwrap_or(now + 28800),
            data: data.unwrap_or_default(),
            ip_address,
            user_agent,
        }
    }
}

impl RiSession {
    /// Creates a new session for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user to create the session for
    /// - `timeout_secs`: Session timeout in seconds
    /// - `ip_address`: Optional client IP address
    /// - `user_agent`: Optional client user agent
    /// 
    /// # Returns
    /// A new instance of `RiSession`
    pub fn new(user_id: String, timeout_secs: u64, ip_address: Option<String>, user_agent: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());

        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            created_at: now,
            last_accessed: now,
            expires_at: now + timeout_secs,
            data: FxHashMap::default(),
            ip_address,
            user_agent,
        }
    }

    /// Checks if the session has expired.
    /// 
    /// # Returns
    /// `true` if the session has expired, otherwise `false`
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        now > self.expires_at
    }

    /// Updates the last accessed time of the session.
    /// 
    /// This method is called when a session is accessed to update its
    /// last accessed timestamp, which can be used for session activity tracking.
    pub fn touch(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        self.last_accessed = now;
    }

    /// Extends the session expiration time.
    /// 
    /// # Parameters
    /// - `timeout_secs`: New timeout in seconds from the current time
    pub fn extend(&mut self, timeout_secs: u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs());
        self.expires_at = now + timeout_secs;
    }

    /// Gets a value from the session data.
    /// 
    /// # Parameters
    /// - `key`: Key to look up in the session data
    /// 
    /// # Returns
    /// `Some(&String)` if the key exists, otherwise `None`
    pub fn get_data(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Sets a value in the session data.
    /// 
    /// # Security
    /// 
    /// This method validates:
    /// - Maximum number of data entries (100)
    /// - Maximum key length (256 characters)
    /// - Maximum value length (4096 characters)
    /// 
    /// # Parameters
    /// - `key`: Key to set in the session data
    /// - `value`: Value to associate with the key
    /// 
    /// # Returns
    /// `true` if the value was set, `false` if limits exceeded
    pub fn set_data(&mut self, key: String, value: String) -> bool {
        // Security: Check key length
        if key.len() > MAX_SESSION_DATA_KEY_LENGTH {
            log::warn!(
                "[Ri.Session] Data key too long: {} chars (max {})",
                key.len(), MAX_SESSION_DATA_KEY_LENGTH
            );
            return false;
        }
        
        // Security: Check value length
        let safe_value = if value.len() > MAX_SESSION_DATA_VALUE_LENGTH {
            log::warn!(
                "[Ri.Session] Data value truncated: {} chars (max {})",
                value.len(), MAX_SESSION_DATA_VALUE_LENGTH
            );
            value[..MAX_SESSION_DATA_VALUE_LENGTH].to_string()
        } else {
            value
        };
        
        // Security: Check maximum entries (only for new keys)
        if !self.data.contains_key(&key) && self.data.len() >= MAX_SESSION_DATA_ENTRIES {
            log::warn!(
                "[Ri.Session] Maximum data entries reached: {} (max {})",
                self.data.len(), MAX_SESSION_DATA_ENTRIES
            );
            return false;
        }
        
        self.data.insert(key, safe_value);
        true
    }

    /// Removes a value from the session data.
    /// 
    /// # Parameters
    /// - `key`: Key to remove from the session data
    /// 
    /// # Returns
    /// `Some(String)` if the key existed and was removed, otherwise `None`
    pub fn remove_data(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
}

/// Session manager for handling user sessions.
///
/// This struct manages session creation, validation, and cleanup. It limits
/// the number of sessions per user and automatically cleans up expired sessions.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiSessionManager {
    sessions: RiShardedLock<String, RiSession>,
    timeout_secs: u64,
    max_sessions_per_user: usize,
}

impl RiSessionManager {
    /// Creates a new session manager with the specified timeout.
    ///
    /// # Parameters
    ///
    /// - `timeout_secs`: Default session timeout in seconds
    ///
    /// # Returns
    ///
    /// A new instance of `RiSessionManager`
    ///
    /// # Notes
    ///
    /// Default maximum sessions per user is 5
    pub fn new(timeout_secs: u64) -> Self {
        Self {
            sessions: RiShardedLock::with_default_shards(),
            timeout_secs,
            max_sessions_per_user: 5,
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
    /// - Session ID is generated using UUID v4 for security
    pub async fn create_session(&self, user_id: String, ip_address: Option<String>, user_agent: Option<String>) -> crate::core::RiResult<String> {
        let user_sessions: Vec<(String, u64)> = self.sessions.collect_where(|_, s| s.user_id == user_id && !s.is_expired()).await
            .into_iter()
            .map(|s| (s.id.clone(), s.created_at))
            .collect();
        
        if user_sessions.len() >= self.max_sessions_per_user {
            let mut sessions_with_time = user_sessions;
            sessions_with_time.sort_by_key(|(_, t)| *t);
            
            if let Some((oldest_id, _)) = sessions_with_time.first() {
                self.sessions.remove(oldest_id).await;
            }
        }

        let session = RiSession::new(user_id, self.timeout_secs, ip_address, user_agent);
        let session_id = session.id.clone();
        self.sessions.insert(session_id.clone(), session).await;
        
        Ok(session_id)
    }

    /// Validates session security by checking IP and User-Agent.
    ///
    /// # Security
    ///
    /// This method helps prevent session hijacking by verifying that
    /// the client's IP address and User-Agent match the original values.
    ///
    /// # Parameters
    /// - `session_id`: ID of the session to validate
    /// - `current_ip`: Current client IP address
    /// - `current_user_agent`: Current client User-Agent
    ///
    /// # Returns
    /// `true` if the session is valid, `false` if there's a mismatch
    pub async fn validate_session_security(
        &self,
        session_id: &str,
        current_ip: Option<&str>,
        current_user_agent: Option<&str>,
    ) -> crate::core::RiResult<bool> {
        let session = self.sessions.get(session_id).await;
        
        match session {
            Some(s) => {
                if s.is_expired() {
                    self.sessions.remove(session_id).await;
                    return Ok(false);
                }

                // Check IP address match (if both are set)
                if let (Some(stored_ip), Some(current)) = (&s.ip_address, current_ip) {
                    if stored_ip != current {
                        log::warn!(
                            "[Ri.Session] IP address mismatch for session {}: expected {}, got {}",
                            session_id, stored_ip, current
                        );
                        return Ok(false);
                    }
                }

                // Check User-Agent match (if both are set)
                if let (Some(stored_ua), Some(current)) = (&s.user_agent, current_user_agent) {
                    if stored_ua != current {
                        log::warn!(
                            "[Ri.Session] User-Agent mismatch for session {}: expected {}, got {}",
                            session_id, stored_ua, current
                        );
                        return Ok(false);
                    }
                }

                Ok(true)
            }
            None => Ok(false),
        }
    }

    /// Regenerates a session ID to prevent session fixation attacks.
    ///
    /// # Security
    ///
    /// This method should be called after successful authentication to
    /// prevent session fixation attacks. It creates a new session ID
    /// while preserving the session data.
    ///
    /// # Parameters
    /// - `old_session_id`: The current session ID
    ///
    /// # Returns
    /// The new session ID, or None if the old session doesn't exist
    pub async fn regenerate_session_id(&self, old_session_id: &str) -> crate::core::RiResult<Option<String>> {
        let old_session = self.sessions.get(old_session_id).await;
        
        match old_session {
            Some(mut s) => {
                if s.is_expired() {
                    self.sessions.remove(old_session_id).await;
                    return Ok(None);
                }

                // Remove old session
                self.sessions.remove(old_session_id).await;

                // Generate new session ID
                let new_session_id = Uuid::new_v4().to_string();
                
                // Update session with new ID
                s.id = new_session_id.clone();
                s.touch();
                
                // Insert with new ID
                self.sessions.insert(new_session_id.clone(), s).await;

                log::info!(
                    "[Ri.Session] Regenerated session ID: {} -> {}",
                    old_session_id, new_session_id
                );

                Ok(Some(new_session_id))
            }
            None => Ok(None),
        }
    }

    /// Gets a session by ID.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to retrieve
    /// 
    /// # Returns
    /// `Some(RiSession)` if the session exists and is not expired, otherwise `None`
    /// 
    /// # Notes
    /// - Expired sessions are automatically removed and return `None`
    /// - The session's last accessed time is updated when retrieved
    pub async fn get_session(&self, session_id: &str) -> crate::core::RiResult<Option<RiSession>> {
        let session = self.sessions.get(session_id).await;
        
        match session {
            Some(mut s) => {
                if s.is_expired() {
                    self.sessions.remove(session_id).await;
                    Ok(None)
                } else {
                    s.touch();
                    self.sessions.insert(session_id.to_string(), s.clone()).await;
                    Ok(Some(s))
                }
            }
            None => Ok(None),
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
    pub async fn update_session(&self, session_id: &str, data: FxHashMap<String, String>) -> crate::core::RiResult<bool> {
        let session = self.sessions.get(session_id).await;
        
        match session {
            Some(mut s) => {
                if s.is_expired() {
                    self.sessions.remove(session_id).await;
                    Ok(false)
                } else {
                    for (key, value) in data {
                        s.set_data(key, value);
                    }
                    s.touch();
                    self.sessions.insert(session_id.to_string(), s).await;
                    Ok(true)
                }
            }
            None => Ok(false),
        }
    }

    /// Extends a session's expiration time.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to extend
    /// 
    /// # Returns
    /// `true` if the session was extended successfully, `false` if the session doesn't exist or is expired
    pub async fn extend_session(&self, session_id: &str) -> crate::core::RiResult<bool> {
        let session = self.sessions.get(session_id).await;
        
        match session {
            Some(mut s) => {
                if s.is_expired() {
                    self.sessions.remove(session_id).await;
                    Ok(false)
                } else {
                    s.extend(self.timeout_secs);
                    self.sessions.insert(session_id.to_string(), s).await;
                    Ok(true)
                }
            }
            None => Ok(false),
        }
    }

    /// Destroys a session by ID.
    /// 
    /// # Parameters
    /// - `session_id`: ID of the session to destroy
    /// 
    /// # Returns
    /// `true` if the session was destroyed successfully, `false` if the session doesn't exist
    pub async fn destroy_session(&self, session_id: &str) -> crate::core::RiResult<bool> {
        Ok(self.sessions.remove(session_id).await.is_some())
    }

    /// Destroys all sessions for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user whose sessions to destroy
    /// 
    /// # Returns
    /// The number of sessions destroyed
    pub async fn destroy_user_sessions(&self, user_id: &str) -> crate::core::RiResult<usize> {
        let count = self.sessions.remove_where(|_, s| s.user_id == user_id).await;
        Ok(count)
    }

    /// Gets all active sessions for a user.
    /// 
    /// # Parameters
    /// - `user_id`: ID of the user whose sessions to retrieve
    /// 
    /// # Returns
    /// A vector of active sessions for the user
    pub async fn get_user_sessions(&self, user_id: &str) -> crate::core::RiResult<Vec<RiSession>> {
        let user_sessions = self.sessions.collect_where(|_, s| s.user_id == user_id && !s.is_expired()).await;
        Ok(user_sessions)
    }

    /// Cleans up all expired sessions.
    /// 
    /// # Returns
    /// The number of expired sessions cleaned up
    pub async fn cleanup_expired(&self) -> crate::core::RiResult<usize> {
        let count = self.sessions.remove_where(|_, s| s.is_expired()).await;
        Ok(count)
    }

    /// Cleans up all sessions.
    /// 
    /// This method removes all sessions, regardless of their expiration status.
    pub async fn cleanup_all(&self) -> crate::core::RiResult<()> {
        self.sessions.clear().await;
        Ok(())
    }

    /// Gets the default session timeout.
    /// 
    /// # Returns
    /// The default session timeout in seconds
    pub fn get_timeout(&self) -> u64 {
        self.timeout_secs
    }

    /// Sets the default session timeout.
    /// 
    /// # Parameters
    /// - `timeout_secs`: New default session timeout in seconds
    pub fn set_timeout(&mut self, timeout_secs: u64) {
        self.timeout_secs = timeout_secs;
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for the Session Manager.
///
/// This module provides Python interface to Ri session management functionality,
/// enabling Python applications to manage user sessions with expiration and data storage.
///
/// ## Supported Operations
///
/// - Session creation with user ID, IP address, and user agent tracking
/// - Session retrieval and validation
/// - Session data storage with key-value pairs
/// - Session expiration management
/// - Session cleanup for expired sessions
///
/// ## Python Usage Example
///
/// ```python
/// from ri import RiSessionManager
///
/// # Create session manager with 30-minute timeout
/// session_manager = RiSessionManager(1800)
///
/// # Create a new session
/// session_id = session_manager.create_session(
///     "user123",
///     "192.168.1.1",
///     "Mozilla/5.0"
/// )
///
/// # Get session data
/// session = session_manager.get_session(session_id)
/// if session:
///     print(f"Session created at: {session.created_at}")
///     print(f"Session expires at: {session.expires_at}")
///
/// # Update session data
/// session_manager.update_session(session_id, {"theme": "dark"})
///
/// # Extend session
/// session_manager.extend_session(session_id)
///
/// # Destroy session when done
/// session_manager.destroy_session(session_id)
/// ```
///
/// ## Limitations
///
/// The current Python bindings do not support async session operations.
/// For async scenarios, use the Rust API directly or implement async wrappers
/// using Python's asyncio library.
#[pyo3::prelude::pymethods]
impl RiSessionManager {
    #[new]
    fn py_new(timeout_secs: u64) -> PyResult<Self> {
        Ok(Self::new(timeout_secs))
    }
    
    #[pyo3(name = "create_session")]
    fn create_session_impl(&self, user_id: String, ip_address: Option<String>, user_agent: Option<String>) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.create_session(user_id, ip_address, user_agent).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_session")]
    fn get_session_impl(&self, session_id: String) -> PyResult<Option<RiSession>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_session(&session_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "update_session")]
    fn update_session_impl(&self, session_id: String, data: FxHashMap<String, String>) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.update_session(&session_id, data).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "destroy_session")]
    fn destroy_session_impl(&self, session_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.destroy_session(&session_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "extend_session")]
    fn extend_session_impl(&self, session_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.extend_session(&session_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "cleanup_expired")]
    fn cleanup_expired_impl(&self) -> PyResult<usize> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.cleanup_expired().await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_timeout")]
    fn get_timeout_impl(&self) -> u64 {
        self.get_timeout()
    }
}
