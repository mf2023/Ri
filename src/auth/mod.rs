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
//! ```rust
//! use dms::prelude::*;
//! use dms::auth::{DMSCAuthConfig, DMSCJWTManager, DMSCJWTClaims};
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

pub use jwt::DMSCJWTManager;
pub use oauth::DMSCOAuthManager;
pub use permissions::DMSCPermissionManager;
pub use session::DMSCSessionManager;

use crate::core::{DMSCResult, DMSCServiceContext};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Configuration for the authentication module.
/// 
/// This struct defines the configuration options for authentication behavior, including
/// JWT settings, session settings, OAuth providers, and enabled authentication methods.
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
}

impl Default for DMSCAuthConfig {
    /// Returns the default configuration for authentication.
    /// 
    /// Default values:
    /// - enabled: true
    /// - jwt_secret: "default-secret-change-in-production"
    /// - jwt_expiry_secs: 3600 (1 hour)
    /// - session_timeout_secs: 86400 (24 hours)
    /// - oauth_providers: empty vector
    /// - enable_api_keys: true
    /// - enable_session_auth: true
    fn default() -> Self {
        Self {
            enabled: true,
            jwt_secret: "default-secret-change-in-production".to_string(),
            jwt_expiry_secs: 3600,
            session_timeout_secs: 86400,
            oauth_providers: vec![],
            enable_api_keys: true,
            enable_session_auth: true,
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
    /// A new `DMSCAuthModule` instance
    pub fn new(config: DMSCAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSCJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSCSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSCPermissionManager::new()));
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(DMSCOAuthManager::new(cache)));

        Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
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
    /// A new `DMSCAuthModule` instance
    pub async fn new_async(config: DMSCAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSCJWTManager::create(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSCSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSCPermissionManager::new_async().await));
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(DMSCOAuthManager::new(cache)));

        Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
        }
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
/// from dms import DMSCAuthConfig, DMSCJWTManager
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
        Ok(Self::new(config))
    }
    
    #[pyo3(name = "jwt_manager")]
    fn jwt_manager_impl(&self) -> PyResult<DMSCJWTManager> {
        Ok(DMSCJWTManager::create(self.jwt_manager.get_secret().to_string(), self.jwt_manager.get_token_expiry()))
    }
    
    #[pyo3(name = "session_manager")]
    fn session_manager_impl(&self) -> PyResult<DMSCSessionManager> {
        Ok(DMSCSessionManager::new(self.config.session_timeout_secs))
    }
    
    #[pyo3(name = "permission_manager")]
    fn permission_manager_impl(&self) -> PyResult<DMSCPermissionManager> {
        Ok(DMSCPermissionManager::new())
    }
    
    #[pyo3(name = "oauth_manager")]
    fn oauth_manager_impl(&self) -> PyResult<DMSCOAuthManager> {
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        Ok(DMSCOAuthManager::new(cache))
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
                // Initialize OAuth provider with default configuration
                // In production, this would load provider-specific configuration from secure storage
                let provider_config = crate::auth::oauth::DMSCOAuthProvider {
                    id: provider_name.clone(),
                    name: provider_name.clone(),
                    client_id: format!("{provider_name}_client_id"),
                    client_secret: format!("{provider_name}_client_secret"),
                    auth_url: format!("https://{provider_name}.com/oauth/authorize"),
                    token_url: format!("https://{provider_name}.com/oauth/token"),
                    user_info_url: format!("https://{provider_name}.com/oauth/userinfo"),
                    scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
                    enabled: true,
                };
                
                // Register the OAuth provider
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
