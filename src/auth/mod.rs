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

//! # Authentication Module
//! 
//! This module provides comprehensive authentication and authorization functionality for DMSC,
//! offering multiple authentication methods and a robust permission system.
//! 
//! ## Key Components
//! 
//! - **DMSCAuthModule**: Main auth module implementing service module traits
//! - **DMSCAuthConfig**: Configuration for authentication behavior
//! - **DMSCJWTManager**: JWT token management for stateless authentication
//! - **DMSCSessionManager**: Session management for stateful authentication
//! - **DMSCPermissionManager**: Permission and role management
//! - **DMSCOAuthManager**: OAuth provider integration
//! - **DMSCJWTClaims**: JWT token claims structure
//! - **DMSCJWTValidationOptions**: JWT validation options
//! - **DMSCOAuthProvider**: OAuth provider interface
//! - **DMSCOAuthToken**: OAuth token structure
//! - **DMSCOAuthUserInfo**: OAuth user information
//! - **DMSCPermission**: Permission structure
//! - **DMSCRole**: Role structure with permissions
//! - **DMSCSession**: Session structure
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
//! use dmsc::prelude::*;
//! use dmsc::auth::{DMSCAuthConfig, DMSCJWTManager, DMSCJWTClaims};
//! use serde_json::json;
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create auth configuration
//!     let auth_config = DMSCAuthConfig {
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
//!     let auth_module = DMSCAuthModule::new(auth_config);
//!     
//!     // Get JWT manager
//!     let jwt_manager = auth_module.jwt_manager();
//!     
//!     // Create JWT claims
//!     let claims = DMSCJWTClaims {
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

pub use jwt::{DMSCJWTManager, DMSCJWTClaims, DMSCJWTValidationOptions};
pub use oauth::{DMSCOAuthManager, DMSCOAuthToken, DMSCOAuthUserInfo, DMSCOAuthProvider};
pub use permissions::{DMSCPermissionManager, DMSCPermission, DMSCRole};
pub use session::{DMSCSessionManager, DMSCSession};
pub use security::DMSCSecurityManager;
pub use revocation::{DMSCJWTRevocationList, DMSCRevokedTokenInfo};

use crate::core::{DMSCResult, DMSCError, DMSCServiceContext};
use rand::RngCore;
use serde::Deserialize;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
#[cfg(feature = "pyo3")]
use tokio::runtime::Handle;

const DEFAULT_JWT_SECRET_ENV: &str = "DMSC_JWT_SECRET";
const FALLBACK_SECRET_LENGTH: usize = 64;

fn load_jwt_secret_from_env() -> String {
    env::var(DEFAULT_JWT_SECRET_ENV).unwrap_or_else(|_| {
        let mut secret = vec![0u8; FALLBACK_SECRET_LENGTH];
        rand::thread_rng().fill_bytes(&mut secret);
        hex::encode(secret)
    })
}

fn load_oauth_env_var(provider_name: &str, suffix: &str) -> Result<String, DMSCError> {
    let env_var = format!("DMSC_OAUTH_{}_{}", provider_name.to_uppercase(), suffix);
    env::var(&env_var).map_err(|_| {
        DMSCError::Config(format!(
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
/// The JWT secret is loaded from the `DMSC_JWT_SECRET` environment variable. If not set,
/// a cryptographically secure random secret is generated automatically.
/// 
/// **Important**: For production environments, always set the `DMSC_JWT_SECRET` environment
/// variable to a strong, unique value. Do not rely on auto-generated secrets in production.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
#[derive(Deserialize)]
pub struct DMSCAuthConfig {
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
    pub oauth_cache_backend_type: crate::cache::DMSCCacheBackendType,
    /// Redis URL for OAuth token cache (used when backend is Redis)
    #[cfg(feature = "cache")]
    pub oauth_cache_redis_url: String,
}

impl Default for DMSCAuthConfig {
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
            oauth_cache_backend_type: crate::cache::DMSCCacheBackendType::Memory,
            #[cfg(feature = "cache")]
            oauth_cache_redis_url: "redis://127.0.0.1:6379".to_string(),
        }
    }
}

/// Main authentication module for DMSC.
/// 
/// This module provides comprehensive authentication and authorization functionality,
/// including JWT management, session management, permission management, and OAuth integration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCAuthModule {
    /// Authentication configuration
    config: DMSCAuthConfig,
    /// JWT manager for stateless authentication
    jwt_manager: Arc<DMSCJWTManager>,
    /// Session manager for stateful authentication, protected by a RwLock for thread-safe access
    session_manager: Arc<RwLock<DMSCSessionManager>>,
    /// Permission manager for role-based access control, protected by a RwLock for thread-safe access
    permission_manager: Arc<RwLock<DMSCPermissionManager>>,
    /// OAuth manager for OAuth provider integration, protected by a RwLock for thread-safe access
    oauth_manager: Arc<RwLock<DMSCOAuthManager>>,
    /// JWT token revocation list for token invalidation
    revocation_list: Arc<DMSCJWTRevocationList>,
}

impl DMSCAuthModule {
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
    /// A `DMSCResult` containing the new `DMSCAuthModule` instance
    /// 
    /// # Errors
    /// 
    /// Returns an error if Redis cache creation fails when Redis backend is configured
    pub async fn new(config: DMSCAuthConfig) -> crate::core::error::DMSCResult<Self> {
        let jwt_manager = Arc::new(DMSCJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSCSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSCPermissionManager::new()));
        
        #[cfg(feature = "cache")]
        let cache: Arc<dyn crate::cache::DMSCCache> = match config.oauth_cache_backend_type {
            crate::cache::DMSCCacheBackendType::Memory => {
                Arc::new(crate::cache::DMSCMemoryCache::new())
            }
            crate::cache::DMSCCacheBackendType::Redis => {
                let cache = crate::cache::DMSCRedisCache::new(&config.oauth_cache_redis_url).await
                    .map_err(|e| crate::core::error::DMSCError::RedisError(format!("Failed to create Redis cache for OAuth: {}", e)))?;
                Arc::new(cache)
            }
            _ => Arc::new(crate::cache::DMSCMemoryCache::new()),
        };
        
        #[cfg(not(feature = "cache"))]
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        
        let oauth_manager = Arc::new(RwLock::new(DMSCOAuthManager::new(cache)));
        let revocation_list = Arc::new(DMSCJWTRevocationList::new());

        Ok(Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
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
    /// A new `DMSCAuthModule` instance
    pub fn with_config(config: DMSCAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSCJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSCSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSCPermissionManager::new()));
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(DMSCOAuthManager::new(cache)));
        let revocation_list = Arc::new(DMSCJWTRevocationList::new());

        Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
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
    /// A `DMSCResult` containing the new `DMSCAuthModule` instance
    /// 
    /// # Errors
    /// 
    /// Returns an error if Redis cache creation fails when Redis backend is configured
    pub async fn new_async(config: DMSCAuthConfig) -> crate::core::error::DMSCResult<Self> {
        let jwt_manager = Arc::new(DMSCJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSCSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSCPermissionManager::new_async().await));
        
        #[cfg(feature = "cache")]
        let cache: Arc<dyn crate::cache::DMSCCache> = match config.oauth_cache_backend_type {
            crate::cache::DMSCCacheBackendType::Memory => {
                Arc::new(crate::cache::DMSCMemoryCache::new())
            }
            crate::cache::DMSCCacheBackendType::Redis => {
                let cache = crate::cache::DMSCRedisCache::new(&config.oauth_cache_redis_url).await
                    .map_err(|e| crate::core::error::DMSCError::RedisError(format!("Failed to create Redis cache: {}", e)))?;
                Arc::new(cache)
            }
            _ => Arc::new(crate::cache::DMSCMemoryCache::new()),
        };
        
        #[cfg(not(feature = "cache"))]
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        
        let oauth_manager = Arc::new(RwLock::new(DMSCOAuthManager::new(cache)));
        let revocation_list = Arc::new(DMSCJWTRevocationList::new());

        Ok(Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
            revocation_list,
        })
    }

    /// Returns a reference to the JWT revocation list.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSCJWTRevocationList> providing thread-safe access to the token revocation list
    pub fn revocation_list(&self) -> Arc<DMSCJWTRevocationList> {
        self.revocation_list.clone()
    }

    /// Returns a reference to the JWT manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSCJWTManager> providing thread-safe access to the JWT manager
    pub fn jwt_manager(&self) -> Arc<DMSCJWTManager> {
        self.jwt_manager.clone()
    }

    /// Returns a reference to the session manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSCSessionManager>> providing thread-safe access to the session manager
    pub fn session_manager(&self) -> Arc<RwLock<DMSCSessionManager>> {
        self.session_manager.clone()
    }

    /// Returns a reference to the permission manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSCPermissionManager>> providing thread-safe access to the permission manager
    pub fn permission_manager(&self) -> Arc<RwLock<DMSCPermissionManager>> {
        self.permission_manager.clone()
    }

    /// Returns a reference to the OAuth manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSCOAuthManager>> providing thread-safe access to the OAuth manager
    pub fn oauth_manager(&self) -> Arc<RwLock<DMSCOAuthManager>> {
        self.oauth_manager.clone()
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for the DMSC Authentication Module.
///
/// This module provides Python interface to DMSC authentication functionality,
/// enabling Python applications to leverage DMSC's authentication capabilities.
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
/// from dmsc import DMSCAuthConfig, DMSCJWTManager
///
/// # Create auth configuration
/// config = DMSCAuthConfig(
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
/// auth_module = DMSCAuthModule(config)
///
/// # Get JWT manager and generate token
/// jwt_manager = auth_module.jwt_manager()
/// token = jwt_manager.generate_token("user123", ["user"], ["read:data"])
/// ```
#[pyo3::prelude::pymethods]
impl DMSCAuthModule {
    #[new]
    fn py_new(config: DMSCAuthConfig) -> PyResult<Self> {
        let rt = Handle::current();
        rt.block_on(async {
            Self::new(config).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }

    #[getter]
    fn get_config(&self) -> DMSCAuthConfig {
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

impl crate::core::ServiceModule for DMSCAuthModule {
    fn name(&self) -> &str {
        "DMSC.Auth"
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

    fn init(&mut self, _ctx: &mut crate::core::DMSCServiceContext) -> crate::core::DMSCResult<()> {
        Ok(())
    }

    fn start(&mut self, _ctx: &mut crate::core::DMSCServiceContext) -> crate::core::DMSCResult<()> {
        Ok(())
    }

    fn shutdown(&mut self, _ctx: &mut crate::core::DMSCServiceContext) -> crate::core::DMSCResult<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::core::DMSCModule for DMSCAuthModule {
    /// Returns the name of the authentication module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.Auth"
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        log::info!("Initializing DMSC Auth Module");

        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        // Update configuration if provided
        if let Some(auth_config) = cfg.get("auth") {
            self.config = serde_yaml::from_str(auth_config)
                .unwrap_or_else(|_| DMSCAuthConfig::default());
        }

        // Initialize JWT manager with new config
        self.jwt_manager = Arc::new(DMSCJWTManager::create(self.config.jwt_secret.clone(), self.config.jwt_expiry_secs));

        // Initialize OAuth providers if configured
        if !self.config.oauth_providers.is_empty() {
            for provider_name in &self.config.oauth_providers {
                let client_id = load_oauth_env_var(provider_name, "CLIENT_ID")?;
                let client_secret = load_oauth_env_var(provider_name, "CLIENT_SECRET")?;

                let provider_config = crate::auth::oauth::DMSCOAuthProvider {
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
                };
                
                let oauth_mgr = self.oauth_manager.write().await;
                oauth_mgr.register_provider(provider_config).await?;
                log::info!("OAuth provider registered: {provider_name}");
            }
        }

        log::info!("DMSC Auth Module initialized successfully");
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        log::info!("Cleaning up DMSC Auth Module");
        
        // Cleanup sessions
        let session_mgr = self.session_manager.write().await;
        session_mgr.cleanup_all().await?;
        
        log::info!("DMSC Auth Module cleanup completed");
        Ok(())
    }
}
