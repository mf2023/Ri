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

//! # Authentication Module
//! 
//! This module provides comprehensive authentication and authorization functionality for DMS,
//! offering multiple authentication methods and a robust permission system.
//! 
//! ## Key Components
//! 
//! - **DMSAuthModule**: Main auth module implementing service module traits
//! - **DMSAuthConfig**: Configuration for authentication behavior
//! - **DMSJWTManager**: JWT token management for stateless authentication
//! - **DMSSessionManager**: Session management for stateful authentication
//! - **DMSPermissionManager**: Permission and role management
//! - **DMSOAuthManager**: OAuth provider integration
//! - **DMSJWTClaims**: JWT token claims structure
//! - **DMSJWTValidationOptions**: JWT validation options
//! - **DMSOAuthProvider**: OAuth provider interface
//! - **DMSOAuthToken**: OAuth token structure
//! - **DMSOAuthUserInfo**: OAuth user information
//! - **DMSPermission**: Permission structure
//! - **DMSRole**: Role structure with permissions
//! - **DMSSession**: Session structure
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
//! use dms::auth::{DMSAuthConfig, DMSJWTManager, DMSJWTClaims};
//! use serde_json::json;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create auth configuration
//!     let auth_config = DMSAuthConfig {
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
//!     let auth_module = DMSAuthModule::new(auth_config);
//!     
//!     // Get JWT manager
//!     let jwt_manager = auth_module.jwt_manager();
//!     
//!     // Create JWT claims
//!     let claims = DMSJWTClaims {
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

pub use jwt::DMSJWTManager;
pub use oauth::DMSOAuthManager;
pub use permissions::DMSPermissionManager;
pub use session::DMSSessionManager;

use crate::core::{DMSResult, DMSServiceContext};
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
pub struct DMSAuthConfig {
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

impl Default for DMSAuthConfig {
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

/// Main authentication module for DMS.
/// 
/// This module provides comprehensive authentication and authorization functionality,
/// including JWT management, session management, permission management, and OAuth integration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSAuthModule {
    /// Authentication configuration
    config: DMSAuthConfig,
    /// JWT manager for stateless authentication
    jwt_manager: Arc<DMSJWTManager>,
    /// Session manager for stateful authentication, protected by a RwLock for thread-safe access
    session_manager: Arc<RwLock<DMSSessionManager>>,
    /// Permission manager for role-based access control, protected by a RwLock for thread-safe access
    permission_manager: Arc<RwLock<DMSPermissionManager>>,
    /// OAuth manager for OAuth provider integration, protected by a RwLock for thread-safe access
    oauth_manager: Arc<RwLock<DMSOAuthManager>>,
}

impl DMSAuthModule {
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
    /// A new `DMSAuthModule` instance
    pub fn new(config: DMSAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSJWTManager::new(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSPermissionManager::new()));
        let cache = Arc::new(crate::cache::DMSMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(DMSOAuthManager::new(cache)));

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
    /// A new `DMSAuthModule` instance
    pub async fn new_async(config: DMSAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSJWTManager::new(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSSessionManager::new(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSPermissionManager::new_async().await));
        let cache = Arc::new(crate::cache::DMSMemoryCache::new());
        let oauth_manager = Arc::new(RwLock::new(DMSOAuthManager::new(cache)));

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
    /// An Arc<DMSJWTManager> providing thread-safe access to the JWT manager
    pub fn jwt_manager(&self) -> Arc<DMSJWTManager> {
        self.jwt_manager.clone()
    }

    /// Returns a reference to the session manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSSessionManager>> providing thread-safe access to the session manager
    pub fn session_manager(&self) -> Arc<RwLock<DMSSessionManager>> {
        self.session_manager.clone()
    }

    /// Returns a reference to the permission manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSPermissionManager>> providing thread-safe access to the permission manager
    pub fn permission_manager(&self) -> Arc<RwLock<DMSPermissionManager>> {
        self.permission_manager.clone()
    }

    /// Returns a reference to the OAuth manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSOAuthManager>> providing thread-safe access to the OAuth manager
    pub fn oauth_manager(&self) -> Arc<RwLock<DMSOAuthManager>> {
        self.oauth_manager.clone()
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSAuthModule
#[pyo3::prelude::pymethods]
impl DMSAuthModule {
    #[new]
    fn py_new(config: DMSAuthConfig) -> PyResult<Self> {
        Ok(Self::new(config))
    }
    
    /// Get JWT manager from Python
    fn jwt_manager_py(&self) -> PyResult<DMSJWTManager> {
        // Create a new JWT manager with the same configuration
        Ok(DMSJWTManager::new(self.jwt_manager.get_secret().to_string(), self.jwt_manager.get_token_expiry()))
    }
    
    /// Get session manager from Python
    fn session_manager_py(&self) -> PyResult<DMSSessionManager> {
        // For now, return a new session manager with the same timeout
        // In a real implementation, you'd want to properly clone the state
        Ok(DMSSessionManager::new(self.config.session_timeout_secs))
    }
    
    /// Get permission manager from Python
    fn permission_manager_py(&self) -> PyResult<DMSPermissionManager> {
        // Return a new permission manager
        Ok(DMSPermissionManager::new())
    }
    
    /// Get OAuth manager from Python
    fn oauth_manager_py(&self) -> PyResult<DMSOAuthManager> {
        // Create a new OAuth manager with a memory cache
        let cache = Arc::new(crate::cache::DMSMemoryCache::new());
        Ok(DMSOAuthManager::new(cache))
    }
}

#[async_trait::async_trait]
impl crate::core::DMSModule for DMSAuthModule {
    /// Returns the name of the authentication module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMS.Auth"
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
    /// A `DMSResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        log::info!("Initializing DMS Auth Module");

        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        // Update configuration if provided
        if let Some(auth_config) = cfg.get("auth") {
            self.config = serde_yaml::from_str(auth_config)
                .unwrap_or_else(|_| DMSAuthConfig::default());
        }

        // Initialize JWT manager with new config
        self.jwt_manager = Arc::new(DMSJWTManager::new(self.config.jwt_secret.clone(), self.config.jwt_expiry_secs));

        // Initialize OAuth providers if configured
        if !self.config.oauth_providers.is_empty() {
            for provider_name in &self.config.oauth_providers {
                // Initialize OAuth provider with default configuration
                // In production, this would load provider-specific configuration from secure storage
                let provider_config = crate::auth::oauth::DMSOAuthProvider {
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

        log::info!("DMS Auth Module initialized successfully");
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
    /// A `DMSResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        log::info!("Cleaning up DMS Auth Module");
        
        // Cleanup sessions
        let session_mgr = self.session_manager.write().await;
        session_mgr.cleanup_all().await?;
        
        log::info!("DMS Auth Module cleanup completed");
        Ok(())
    }
}
