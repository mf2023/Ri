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

//! # Authentication Module
//! 
//! This module provides comprehensive authentication and authorization functionality for Ri,
//! offering multiple authentication methods and a robust permission system.
//! 
//! ## Key Components
//! 
//! - **RiAuthModule**: Main auth module implementing service module traits
//! - **RiAuthConfig**: Configuration for authentication behavior
//! - **RiJWTManager**: JWT token management for stateless authentication
//! - **RiSessionManager**: Session management for stateful authentication
//! - **RiPermissionManager**: Permission and role management
//! - **RiOAuthManager**: OAuth provider integration
//! - **RiJWTClaims**: JWT token claims structure
//! - **RiJWTValidationOptions**: JWT validation options
//! - **RiOAuthProvider**: OAuth provider interface
//! - **RiOAuthToken**: OAuth token structure
//! - **RiOAuthUserInfo**: OAuth user information
//! - **RiPermission**: Permission structure
//! - **RiRole**: Role structure with permissions
//! - **RiSession**: Session structure
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Authentication Methods**: Supports JWT, sessions, OAuth, and API keys
//! 2. **Configurable**: Highly configurable authentication behavior
//! 3. **Async Support**: Full async/await compatibility for session and OAuth operations
//! 4. **Role-Based Access Control**: Comprehensive permission system with roles
//! 5. **Stateless and Stateful Options**: Supports both stateless (JWT) and stateful (session) authentication
//! 6. **Service Module Integration**: Implements service module traits for seamless integration
//! 7. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 8. **Non-critical**: Auth failures should not break the entire application
//! 9. **Extensible**: Easy to add new authentication methods and OAuth providers
//! 10. **Secure by Default**: Sensible default configurations for security
//! 
//! ## Usage
//! 
//! ```rust,ignore
//! use ri::prelude::*;
//! use ri::auth::{RiAuthConfig, RiJWTManager, RiJWTClaims};
//! use serde_json::json;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create auth configuration
//!     let auth_config = RiAuthConfig {
//!         enabled: true,
//!         jwt_secret: "secure-secret-key".to_string(),
//!         jwt_expiry_secs: 3600,
//!         session_timeout_secs: 86400,
//!         oauth_providers: vec![],
//!         enable_api_keys: true,
//!         enable_session_auth: true,
//!     };
//!     
//!     // Create auth module
//!     let auth_module = RiAuthModule::new(auth_config);
//!     
//!     // Get JWT manager
//!     let jwt_manager = auth_module.jwt_manager();
//!     
//!     // Create JWT claims
//!     let claims = RiJWTClaims {
//!         sub: "user-123".to_string(),
//!         email: "user@example.com".to_string(),
//!         roles: vec!["user".to_string()],
//!         permissions: vec!["read:data".to_string()],
//!         extra: json!({ "custom": "value" }),
//!     };
//!     
//!     // Generate JWT token
//!     let token = jwt_manager.generate_token(claims)?;
//!     println!("Generated JWT token: {}", token);
//!     
//!     // Validate JWT token
//!     let validated_claims = jwt_manager.validate_token(&token)?;
//!     println!("Validated claims: {:?}", validated_claims);
//!     
//!     // Get session manager
//!     let session_manager = auth_module.session_manager();
//!     
//!     // Create a session
//!     let session = session_manager.write().await.create_session("user-123").await?;
//!     println!("Created session: {}", session.id);
//!     
//!     Ok(())
//! }
//! ```

mod jwt;
mod oauth;
mod permissions;
mod session;
mod security;
mod revocation;

pub use jwt::{RiJWTManager, RiJWTClaims, RiJWTValidationOptions};
pub use oauth::{RiOAuthManager, RiOAuthToken, RiOAuthUserInfo, RiOAuthProvider};
pub use permissions::{RiPermissionManager, RiPermission, RiRole};
pub use session::{RiSessionManager, RiSession};
pub use security::RiSecurityManager;
pub use revocation::{RiJWTRevocationList, RiRevokedTokenInfo};

use crate::core::{RiResult, RiError, RiServiceContext};
use rand::RngCore;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
#[cfg(feature = "pyo3")]
use tokio::runtime::Handle;

const DEFAULT_JWT_SECRET_ENV: &str = "Ri_JWT_SECRET";
const FALLBACK_SECRET_LENGTH: usize = 64;

#[derive(Debug, Clone)]
struct LoginAttempt {
    count: u32,
    first_attempt: u64,
    locked_until: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RiRateLimiter {
    attempts: Arc<RwLock<HashMap<String, LoginAttempt>>>,
    max_attempts: u32,
    lockout_secs: u64,
    window_secs: u64,
}

impl RiRateLimiter {
    pub fn new(max_attempts: u32, lockout_secs: u64, window_secs: u64) -> Self {
        Self {
            attempts: Arc::new(RwLock::new(HashMap::new())),
            max_attempts,
            lockout_secs,
            window_secs,
        }
    }

    pub async fn check_and_record(&self, identifier: &str) -> Result<(), u64> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut attempts = self.attempts.write().await;

        if let Some(attempt) = attempts.get_mut(identifier) {
            if let Some(locked_until) = attempt.locked_until {
                if now < locked_until {
                    return Err(locked_until - now);
                }
                attempts.remove(identifier);
                return Ok(());
            }

            if now - attempt.first_attempt > self.window_secs {
                attempts.remove(identifier);
                return Ok(());
            }

            attempt.count += 1;
            if attempt.count >= self.max_attempts {
                attempt.locked_until = Some(now + self.lockout_secs);
                return Err(self.lockout_secs);
            }
        } else {
            attempts.insert(identifier.to_string(), LoginAttempt {
                count: 1,
                first_attempt: now,
                locked_until: None,
            });
        }

        Ok(())
    }

    pub async fn reset(&self, identifier: &str) {
        self.attempts.write().await.remove(identifier);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RiAuditEvent {
    pub timestamp: String,
    pub event_type: String,
    pub user_identifier: Option<String>,
    pub ip_address: Option<String>,
    pub action: String,
    pub resource: Option<String>,
    pub success: bool,
    pub details: Option<String>,
}

impl RiAuditEvent {
    pub fn new(event_type: &str, action: &str) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: event_type.to_string(),
            user_identifier: None,
            ip_address: None,
            action: action.to_string(),
            resource: None,
            success: true,
            details: None,
        }
    }

    pub fn with_user(mut self, user: &str) -> Self {
        self.user_identifier = Some(user.to_string());
        self
    }

    pub fn with_ip(mut self, ip: &str) -> Self {
        self.ip_address = Some(ip.to_string());
        self
    }

    pub fn with_resource(mut self, resource: &str) -> Self {
        self.resource = Some(resource.to_string());
        self
    }

    pub fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }

    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }
}

#[derive(Debug, Clone)]
pub struct RiAuditLogger {
    events: Arc<RwLock<Vec<RiAuditEvent>>>,
    max_events: usize,
}

impl RiAuditLogger {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::with_capacity(max_events))),
            max_events,
        }
    }

    pub async fn log(&self, event: RiAuditEvent) {
        let mut events = self.events.write().await;
        if events.len() >= self.max_events {
            events.remove(0);
        }
        events.push(event);
    }

    pub async fn get_events(&self) -> Vec<RiAuditEvent> {
        self.events.read().await.clone()
    }
}

fn load_jwt_secret_from_env() -> String {
    env::var(DEFAULT_JWT_SECRET_ENV).unwrap_or_else(|_| {
        let mut secret = vec![0u8; FALLBACK_SECRET_LENGTH];
        rand::thread_rng().fill_bytes(&mut secret);
        hex::encode(secret)
    })
}

fn load_oauth_env_var(provider_name: &str, suffix: &str) -> Result<String, RiError> {
    let env_var = format!("Ri_OAUTH_{}_{}", provider_name.to_uppercase(), suffix);
    env::var(&env_var).map_err(|_| {
        RiError::Config(format!(
            "OAuth {} is not set for provider '{}'. Please set the environment variable {}",
            suffix.to_lowercase(),
            provider_name,
            env_var
        ))
    })
}

fn get_oauth_url(provider_name: &str, endpoint: &str) -> String {
    match load_oauth_env_var(provider_name, endpoint) {
        Ok(url) if !url.is_empty() => url,
        _ => format!("https://{}.com/oauth/{}", provider_name, endpoint)
    }
}

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Configuration for the authentication module.
/// 
/// This struct defines the configuration options for authentication behavior, including
/// JWT settings, session settings, OAuth providers, and enabled authentication methods.
/// 
/// ## Security
/// 
/// The JWT secret is loaded from the `Ri_JWT_SECRET` environment variable. If not set,
/// a cryptographically secure random secret is generated automatically.
/// 
/// **Important**: For production environments, always set the `Ri_JWT_SECRET` environment
/// variable to a strong, unique value. Do not rely on auto-generated secrets in production.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct RiAuthConfig {
    /// Whether authentication is enabled
    pub enabled: bool,
    /// Secret key for JWT token generation and validation
    pub jwt_secret: String,
    /// JWT token expiry time in seconds
    pub jwt_expiry_secs: u64,
    /// Session timeout in seconds
    pub session_timeout_secs: u64,
    /// List of OAuth providers to enable
    pub oauth_providers: Vec<String>,
    /// Whether API key authentication is enabled
    pub enable_api_keys: bool,
    /// Whether session authentication is enabled
    pub enable_session_auth: bool,
    /// OAuth token cache backend type (Memory or Redis)
    #[cfg(feature = "cache")]
    pub oauth_cache_backend_type: crate::cache::RiCacheBackendType,
    /// Redis URL for OAuth token cache (used when backend is Redis)
    #[cfg(feature = "cache")]
    pub oauth_cache_redis_url: String,
    /// Rate limiting: max login attempts per IP before lockout
    pub rate_limit_max_login_attempts: u32,
    /// Rate limiting: lockout duration in seconds after max attempts exceeded
    pub rate_limit_lockout_secs: u64,
    /// Rate limiting: window size in seconds for tracking attempts
    pub rate_limit_window_secs: u64,
}

impl Default for RiAuthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            jwt_secret: load_jwt_secret_from_env(),
            jwt_expiry_secs: 3600,
            session_timeout_secs: 86400,
            oauth_providers: vec![],
            enable_api_keys: true,
            enable_session_auth: true,
            #[cfg(feature = "cache")]
            oauth_cache_backend_type: crate::cache::RiCacheBackendType::Memory,
            #[cfg(feature = "cache")]
            oauth_cache_redis_url: "redis://127.0.0.1:6379".to_string(),
            rate_limit_max_login_attempts: 5,
            rate_limit_lockout_secs: 300,
            rate_limit_window_secs: 900,
        }
    }
}

/// Main authentication module for Ri.
///
/// This module provides comprehensive authentication and authorization functionality,
/// including JWT management, session management, permission management, and OAuth integration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiAuthModule {
    /// Authentication configuration
    config: RiAuthConfig,
    /// JWT manager for stateless authentication
    jwt_manager: Arc<RiJWTManager>,
    /// Session manager for stateful authentication, protected by a RwLock for thread-safe access
    session_manager: Arc<RwLock<RiSessionManager>>,
    /// Permission manager for role-based access control, protected by a RwLock for thread-safe access
    permission_manager: Arc<RwLock<RiPermissionManager>>,
    /// OAuth manager for OAuth provider integration, protected by a RwLock for thread-safe access
    oauth_manager: Arc<RwLock<RiOAuthManager>>,
    /// JWT token revocation list for token invalidation
    revocation_list: Arc<RiJWTRevocationList>,
    /// Rate limiter for login attempts
    rate_limiter: RiRateLimiter,
    /// Audit logger for security events
    audit_logger: RiAuditLogger,
}

impl RiAuthModule {
    /// Creates a new authentication module with the given configuration.
    /// 
    /// **Performance Note**: This method creates a permission manager using the synchronous
    /// `new()` method which uses `blocking_write` during initialization. For async contexts,
    /// consider using `new_async()` to avoid blocking the runtime.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The authentication configuration to use
    /// 
    /// # Returns
    /// 
    /// A `RiResult` containing the new `RiAuthModule` instance
    /// 
    /// # Errors
    /// 
    /// Returns an error if Redis cache creation fails when Redis backend is configured
    pub async fn new(config: RiAuthConfig) -> crate::core::error::RiResult<Self> {
        let jwt_manager = Arc::new(RiJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(RiSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(RiPermissionManager::new()));
        
        #[cfg(feature = "cache")]
        let cache: Arc<dyn crate::cache::RiCache> = match config.oauth_cache_backend_type {
            crate::cache::RiCacheBackendType::Memory => {
                Arc::new(crate::cache::RiMemoryCache::new())
            }
            crate::cache::RiCacheBackendType::Redis => {
                let cache = crate::cache::RiRedisCache::new(&config.oauth_cache_redis_url).await
                    .map_err(|e| crate::core::error::RiError::RedisError(format!("Failed to create Redis cache for OAuth: {}", e)))?;
                Arc::new(cache)
            }
            _ => Arc::new(crate::cache::RiMemoryCache::new()),
        };
        
        #[cfg(not(feature = "cache"))]
        let cache = Arc::new(crate::cache::RiMemoryCache::new());
        
        let oauth_manager = Arc::new(RwLock::new(RiOAuthManager::new(cache)));
        let revocation_list = Arc::new(RiJWTRevocationList::new());
        let rate_limiter = RiRateLimiter::new(
            config.rate_limit_max_login_attempts,
            config.rate_limit_lockout_secs,
            config.rate_limit_window_secs,
        );
        let audit_logger = RiAuditLogger::new(10000);

        Ok(Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
            rate_limiter,
            audit_logger,
        })
    }

    /// Creates a new authentication module with the given configuration (synchronous version).
    /// 
    /// This is a synchronous wrapper for use in the builder pattern.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The authentication configuration to use
    /// 
    /// # Returns
    /// 
    /// A new `RiAuthModule` instance
    pub fn with_config(config: RiAuthConfig) -> Self {
        let jwt_manager = Arc::new(RiJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(RiSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(RiPermissionManager::new()));
        let cache = Arc::new(crate::cache::RiMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(RiOAuthManager::new(cache)));
        let revocation_list = Arc::new(RiJWTRevocationList::new());
        let rate_limiter = RiRateLimiter::new(
            config.rate_limit_max_login_attempts,
            config.rate_limit_lockout_secs,
            config.rate_limit_window_secs,
        );
        let audit_logger = RiAuditLogger::new(10000);

        Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
            rate_limiter,
            audit_logger,
        }
    }

    /// Creates a new authentication module with the given configuration asynchronously.
    /// 
    /// This method is preferred for async contexts as it avoids blocking the runtime
    /// during permission manager initialization by using the async `new_async()` method.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The authentication configuration to use
    /// 
    /// # Returns
    /// 
    /// A `RiResult` containing the new `RiAuthModule` instance
    /// 
    /// # Errors
    /// 
    /// Returns an error if Redis cache creation fails when Redis backend is configured
    pub async fn new_async(config: RiAuthConfig) -> crate::core::error::RiResult<Self> {
        let jwt_manager = Arc::new(RiJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(RiSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(RiPermissionManager::new_async().await));
        
        #[cfg(feature = "cache")]
        let cache: Arc<dyn crate::cache::RiCache> = match config.oauth_cache_backend_type {
            crate::cache::RiCacheBackendType::Memory => {
                Arc::new(crate::cache::RiMemoryCache::new())
            }
            crate::cache::RiCacheBackendType::Redis => {
                let cache = crate::cache::RiRedisCache::new(&config.oauth_cache_redis_url).await
                    .map_err(|e| crate::core::error::RiError::RedisError(format!("Failed to create Redis cache: {}", e)))?;
                Arc::new(cache)
            }
            _ => Arc::new(crate::cache::RiMemoryCache::new()),
        };
        
        #[cfg(not(feature = "cache"))]
        let cache = Arc::new(crate::cache::RiMemoryCache::new());
        
        let oauth_manager = Arc::new(RwLock::new(RiOAuthManager::new(cache)));
        let revocation_list = Arc::new(RiJWTRevocationList::new());
        let rate_limiter = RiRateLimiter::new(
            config.rate_limit_max_login_attempts,
            config.rate_limit_lockout_secs,
            config.rate_limit_window_secs,
        );
        let audit_logger = RiAuditLogger::new(10000);

        Ok(Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
            rate_limiter,
            audit_logger,
        })
    }

    /// Returns a reference to the JWT revocation list.
    /// 
    /// # Returns
    /// 
    /// An Arc<RiJWTRevocationList> providing thread-safe access to the token revocation list
    pub fn revocation_list(&self) -> Arc<RiJWTRevocationList> {
        self.revocation_list.clone()
    }

    /// Check if a login attempt from the given identifier (IP address or username) should be allowed.
    ///
    /// This method implements rate limiting for login attempts. If the rate limit is exceeded,
    /// the method returns an error with the remaining lockout time in seconds.
    ///
    /// # Parameters
    ///
    /// - `identifier`: The identifier to check (typically IP address or username)
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the attempt is allowed
    /// - `Err(lockout_remaining_secs)` if the identifier is locked out
    pub async fn check_rate_limit(&self, identifier: &str) -> Result<(), u64> {
        self.rate_limiter.check_and_record(identifier).await
    }

    /// Reset the rate limit for a given identifier (e.g., after successful login).
    ///
    /// # Parameters
    ///
    /// - `identifier`: The identifier to reset
    pub async fn reset_rate_limit(&self, identifier: &str) {
        self.rate_limiter.reset(identifier).await;
    }

    /// Log an audit event for security tracking.
    ///
    /// # Parameters
    ///
    /// - `event`: The audit event to log
    pub async fn log_audit_event(&self, event: RiAuditEvent) {
        self.audit_logger.log(event).await;
    }

    /// Log a login attempt event.
    ///
    /// # Parameters
    ///
    /// - `username`: The username that attempted to log in
    /// - `ip_address`: The IP address of the login attempt
    /// - `success`: Whether the login was successful
    /// - `reason`: Optional reason for failure or success
    pub async fn log_login_attempt(&self, username: &str, ip_address: Option<&str>, success: bool, reason: Option<&str>) {
        let mut event = RiAuditEvent::new("LOGIN_ATTEMPT", if success { "login_success" } else { "login_failure" });
        event = event.with_user(username);
        if let Some(ip) = ip_address {
            event = event.with_ip(ip);
        }
        if let Some(r) = reason {
            event = event.with_details(r);
        }
        event = event.with_success(success);
        self.audit_logger.log(event).await;
    }

    /// Get recent audit events.
    ///
    /// # Returns
    ///
    /// A vector of recent audit events
    pub async fn get_audit_events(&self) -> Vec<RiAuditEvent> {
        self.audit_logger.get_events().await
    }

    /// Returns a reference to the JWT manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RiJWTManager> providing thread-safe access to the JWT manager
    pub fn jwt_manager(&self) -> Arc<RiJWTManager> {
        self.jwt_manager.clone()
    }

    /// Returns a reference to the session manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<RiSessionManager>> providing thread-safe access to the session manager
    pub fn session_manager(&self) -> Arc<RwLock<RiSessionManager>> {
        self.session_manager.clone()
    }

    /// Returns a reference to the permission manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<RiPermissionManager>> providing thread-safe access to the permission manager
    pub fn permission_manager(&self) -> Arc<RwLock<RiPermissionManager>> {
        self.permission_manager.clone()
    }

    /// Returns a reference to the OAuth manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<RiOAuthManager>> providing thread-safe access to the OAuth manager
    pub fn oauth_manager(&self) -> Arc<RwLock<RiOAuthManager>> {
        self.oauth_manager.clone()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiAuthConfig {
    /// Creates a new authentication configuration with the specified parameters.
    ///
    /// All parameters have sensible defaults, making it easy to create a basic configuration.
    /// The JWT secret is automatically loaded from the `Ri_JWT_SECRET` environment variable
    /// if not provided.
    ///
    /// # Parameters
    ///
    /// - `enabled`: Whether authentication is enabled (default: true)
    /// - `jwt_secret`: Secret key for JWT tokens (default: loaded from Ri_JWT_SECRET env var)
    /// - `jwt_expiry_secs`: JWT token expiry time in seconds (default: 3600)
    /// - `session_timeout_secs`: Session timeout in seconds (default: 86400)
    /// - `oauth_providers`: List of OAuth providers to enable (default: empty)
    /// - `enable_api_keys`: Whether API key authentication is enabled (default: true)
    /// - `enable_session_auth`: Whether session authentication is enabled (default: true)
    /// - `oauth_cache_backend_type`: Cache backend type - "Memory" or "Redis" (default: "Memory")
    /// - `oauth_cache_redis_url`: Redis URL for OAuth cache (default: "redis://127.0.0.1:6379")
    ///
    /// # Returns
    ///
    /// A new `RiAuthConfig` instance
    ///
    /// # Example
    ///
    /// ```python
    /// from ri import RiAuthConfig
    ///
    /// # Create with defaults
    /// config = RiAuthConfig()
    ///
    /// # Create with custom settings
    /// config = RiAuthConfig(
    ///     enabled=True,
    ///     jwt_secret="my-secret-key",
    ///     jwt_expiry_secs=7200,
    ///     oauth_providers=["google", "github"]
    /// )
    /// ```
    #[new]
    #[pyo3(signature = (
        enabled = true,
        jwt_secret = "",
        jwt_expiry_secs = 3600,
        session_timeout_secs = 86400,
        oauth_providers = vec![],
        enable_api_keys = true,
        enable_session_auth = true,
        oauth_cache_backend_type = None,
        oauth_cache_redis_url = "redis://127.0.0.1:6379"
    ))]
    fn py_new(
        enabled: bool,
        jwt_secret: &str,
        jwt_expiry_secs: u64,
        session_timeout_secs: u64,
        oauth_providers: Vec<String>,
        enable_api_keys: bool,
        enable_session_auth: bool,
        oauth_cache_backend_type: Option<String>,
        oauth_cache_redis_url: &str,
    ) -> Self {
        let secret = if jwt_secret.is_empty() {
            load_jwt_secret_from_env()
        } else {
            jwt_secret.to_string()
        };
        
        #[cfg(feature = "cache")]
        {
            let backend_type = match oauth_cache_backend_type.as_deref() {
                Some("Redis") => crate::cache::RiCacheBackendType::Redis,
                _ => crate::cache::RiCacheBackendType::Memory,
            };
            
            Self {
                enabled,
                jwt_secret: secret,
                jwt_expiry_secs,
                session_timeout_secs,
                oauth_providers,
                enable_api_keys,
                enable_session_auth,
                oauth_cache_backend_type: backend_type,
                oauth_cache_redis_url: oauth_cache_redis_url.to_string(),
            }
        }
        
        #[cfg(not(feature = "cache"))]
        {
            let _ = oauth_cache_backend_type;
            let _ = oauth_cache_redis_url;
            
            Self {
                enabled,
                jwt_secret: secret,
                jwt_expiry_secs,
                session_timeout_secs,
                oauth_providers,
                enable_api_keys,
                enable_session_auth,
            }
        }
    }

    /// Creates a new authentication configuration with default values.
    ///
    /// This is a convenience method that creates a configuration with all default settings.
    /// The JWT secret is loaded from the `Ri_JWT_SECRET` environment variable.
    ///
    /// # Returns
    ///
    /// A new `RiAuthConfig` instance with default values
    ///
    /// # Example
    ///
    /// ```python
    /// from ri import RiAuthConfig
    ///
    /// config = RiAuthConfig.default()
    /// ```
    #[staticmethod]
    fn default() -> Self {
        <Self as Default>::default()
    }

    /// Creates a new authentication configuration from environment variables.
    ///
    /// This method loads the JWT secret from the `Ri_JWT_SECRET` environment variable
    /// and uses default values for all other settings.
    ///
    /// # Returns
    ///
    /// A new `RiAuthConfig` instance with values from environment
    ///
    /// # Example
    ///
    /// ```python
    /// import os
    /// from ri import RiAuthConfig
    ///
    /// os.environ["Ri_JWT_SECRET"] = "my-secret-key"
    /// config = RiAuthConfig.from_env()
    /// ```
    #[staticmethod]
    fn from_env() -> Self {
        Self {
            jwt_secret: load_jwt_secret_from_env(),
            ..Self::default()
        }
    }

    /// Returns whether authentication is enabled.
    #[getter]
    fn get_enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the JWT secret key.
    #[getter]
    fn get_jwt_secret(&self) -> String {
        self.jwt_secret.clone()
    }

    /// Returns the JWT token expiry time in seconds.
    #[getter]
    fn get_jwt_expiry_secs(&self) -> u64 {
        self.jwt_expiry_secs
    }

    /// Returns the session timeout in seconds.
    #[getter]
    fn get_session_timeout_secs(&self) -> u64 {
        self.session_timeout_secs
    }

    /// Returns the list of OAuth providers.
    #[getter]
    fn get_oauth_providers(&self) -> Vec<String> {
        self.oauth_providers.clone()
    }

    /// Returns whether API key authentication is enabled.
    #[getter]
    fn get_enable_api_keys(&self) -> bool {
        self.enable_api_keys
    }

    /// Returns whether session authentication is enabled.
    #[getter]
    fn get_enable_session_auth(&self) -> bool {
        self.enable_session_auth
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for the Ri Authentication Module.
///
/// This module provides Python interface to Ri authentication functionality,
/// enabling Python applications to leverage Ri's authentication capabilities.
///
/// ## Supported Operations
///
/// - JWT token generation and validation
/// - Session management for stateful authentication
/// - Permission and role management for RBAC
/// - OAuth provider integration
///
/// ## Python Usage Example
///
/// ```python
/// from ri import RiAuthConfig, RiJWTManager
///
/// # Create auth configuration
/// config = RiAuthConfig(
///     enabled=True,
///     jwt_secret="secure-secret-key",
///     jwt_expiry_secs=3600,
///     session_timeout_secs=86400,
///     oauth_providers=["google", "github"],
///     enable_api_keys=True,
///     enable_session_auth=True,
/// )
///
/// # Create auth module
/// auth_module = RiAuthModule(config)
///
/// # Get JWT manager and generate token
/// jwt_manager = auth_module.jwt_manager()
/// token = jwt_manager.generate_token("user123", ["user"], ["read:data"])
/// ```
#[pyo3::prelude::pymethods]
impl RiAuthModule {
    #[new]
    fn py_new(config: RiAuthConfig) -> PyResult<Self> {
        let rt = Handle::current();
        rt.block_on(async {
            Self::new(config).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }

    #[getter]
    fn get_config(&self) -> RiAuthConfig {
        self.config.clone()
    }

    #[getter]
    fn get_jwt_expiry_secs(&self) -> u64 {
        self.jwt_manager.get_token_expiry()
    }

    #[getter]
    fn get_session_timeout_secs(&self) -> u64 {
        self.config.session_timeout_secs
    }

    #[getter]
    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    #[getter]
    fn is_api_keys_enabled(&self) -> bool {
        self.config.enable_api_keys
    }

    #[getter]
    fn is_session_auth_enabled(&self) -> bool {
        self.config.enable_session_auth
    }

    #[getter]
    fn get_oauth_providers(&self) -> Vec<String> {
        self.config.oauth_providers.clone()
    }

    fn validate_jwt_token(&self, token: &str) -> bool {
        self.jwt_manager.validate_token(token).is_ok()
    }

    fn generate_test_token(&self, subject: &str, roles: Vec<String>, permissions: Vec<String>) -> PyResult<String> {
        self.jwt_manager.generate_token(subject, roles, permissions)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

impl crate::core::ServiceModule for RiAuthModule {
    fn name(&self) -> &str {
        "Ri.Auth"
    }

    fn is_critical(&self) -> bool {
        false
    }

    fn priority(&self) -> i32 {
        20
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }

    fn init(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }

    fn start(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }

    fn shutdown(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::core::RiModule for RiAuthModule {
    /// Returns the name of the authentication module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "Ri.Auth"
    }

    /// Indicates whether the authentication module is critical.
    /// 
    /// The authentication module is non-critical, meaning that if it fails to initialize or operate,
    /// it should not break the entire application. This allows the core functionality to continue
    /// even if authentication features are unavailable.
    /// 
    /// # Returns
    /// 
    /// `false` since authentication is non-critical
    fn is_critical(&self) -> bool {
        false // Auth failures should not break the application
    }

    /// Initializes the authentication module asynchronously.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Updates the module configuration if provided
    /// 3. Reinitializes the JWT manager with the new configuration
    /// 4. Initializes OAuth providers if configured
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
        log::info!("Initializing Ri Auth Module");

        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        // Update configuration if provided
        if let Some(auth_config) = cfg.get("auth") {
            self.config = serde_yaml::from_str(auth_config)
                .unwrap_or_else(|_| RiAuthConfig::default());
        }

        // Initialize JWT manager with new config
        self.jwt_manager = Arc::new(RiJWTManager::create(self.config.jwt_secret.clone(), self.config.jwt_expiry_secs));

        // Initialize OAuth providers if configured
        if !self.config.oauth_providers.is_empty() {
            for provider_name in &self.config.oauth_providers {
                let client_id = load_oauth_env_var(provider_name, "CLIENT_ID")?;
                let client_secret = load_oauth_env_var(provider_name, "CLIENT_SECRET")?;

                let provider_config = crate::auth::oauth::RiOAuthProvider {
                    id: provider_name.clone(),
                    name: provider_name.clone(),
                    client_id,
                    client_secret,
                    auth_url: get_oauth_url(provider_name, "authorize"),
                    token_url: get_oauth_url(provider_name, "token"),
                    user_info_url: get_oauth_url(provider_name, "userinfo"),
                    scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
                    enabled: true,
                    redirect_uri: None,
                    allowed_redirect_uris: vec![],
                };
                
                let oauth_mgr = self.oauth_manager.write().await;
                oauth_mgr.register_provider(provider_config).await?;
                log::info!("OAuth provider registered: {provider_name}");
            }
        }

        log::info!("Ri Auth Module initialized successfully");
        Ok(())
    }

    /// Performs asynchronous cleanup after the application has shut down.
    /// 
    /// This method cleans up all sessions managed by the session manager.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut RiServiceContext) -> RiResult<()> {
        log::info!("Cleaning up Ri Auth Module");
        
        // Cleanup sessions
        let session_mgr = self.session_manager.write().await;
        session_mgr.cleanup_all().await?;
        
        log::info!("Ri Auth Module cleanup completed");
        Ok(())
    }
}
