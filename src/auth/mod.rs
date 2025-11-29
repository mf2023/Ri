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
//! - **JWTClaims**: JWT token claims structure
//! - **JWTValidationOptions**: JWT validation options
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
//! use dms::auth::{DMSAuthConfig, DMSJWTManager, JWTClaims};
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
//!     let auth_module = DMSAuthModule::_Fnew(auth_config);
//!     
//!     // Get JWT manager
//!     let jwt_manager = auth_module._Fjwt_manager();
//!     
//!     // Create JWT claims
//!     let claims = JWTClaims {
//!         sub: "user-123".to_string(),
//!         email: "user@example.com".to_string(),
//!         roles: vec!["user".to_string()],
//!         permissions: vec!["read:data".to_string()],
//!         extra: json!({ "custom": "value" }),
//!     };
//!     
//!     // Generate JWT token
//!     let token = jwt_manager._Fgenerate_token(claims)?;
//!     println!("Generated JWT token: {}", token);
//!     
//!     // Validate JWT token
//!     let validated_claims = jwt_manager._Fvalidate_token(&token)?;
//!     println!("Validated claims: {:?}", validated_claims);
//!     
//!     // Get session manager
//!     let session_manager = auth_module._Fsession_manager();
//!     
//!     // Create a session
//!     let session = session_manager.write().await._Fcreate_session("user-123").await?;
//!     println!("Created session: {}", session.id);
//!     
//!     Ok(())
//! }
//! ```

pub mod jwt;
pub mod oauth;
pub mod permissions;
pub mod session;

pub use jwt::{DMSJWTManager, JWTClaims, JWTValidationOptions};
pub use oauth::{DMSOAuthManager, DMSOAuthProvider, DMSOAuthToken, DMSOAuthUserInfo};
pub use permissions::{DMSPermission, DMSPermissionManager, DMSRole};
pub use session::{DMSSession, DMSSessionManager};

use crate::core::{DMSResult, DMSServiceContext};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for the authentication module.
/// 
/// This struct defines the configuration options for authentication behavior, including
/// JWT settings, session settings, OAuth providers, and enabled authentication methods.
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
    /// # Parameters
    /// 
    /// - `config`: The authentication configuration to use
    /// 
    /// # Returns
    /// 
    /// A new `DMSAuthModule` instance
    pub fn _Fnew(config: DMSAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSJWTManager::_Fnew(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSSessionManager::_Fnew(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSPermissionManager::_Fnew()));
        let cache = Arc::new(crate::cache::backends::memory_backend::DMSMemoryCache::_Fnew());
        let oauth_manager = Arc::new(RwLock::new(DMSOAuthManager::_Fnew(cache)));

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
    pub fn _Fjwt_manager(&self) -> Arc<DMSJWTManager> {
        self.jwt_manager.clone()
    }

    /// Returns a reference to the session manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSSessionManager>> providing thread-safe access to the session manager
    pub fn _Fsession_manager(&self) -> Arc<RwLock<DMSSessionManager>> {
        self.session_manager.clone()
    }

    /// Returns a reference to the permission manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSPermissionManager>> providing thread-safe access to the permission manager
    pub fn _Fpermission_manager(&self) -> Arc<RwLock<DMSPermissionManager>> {
        self.permission_manager.clone()
    }

    /// Returns a reference to the OAuth manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSOAuthManager>> providing thread-safe access to the OAuth manager
    pub fn _Foauth_manager(&self) -> Arc<RwLock<DMSOAuthManager>> {
        self.oauth_manager.clone()
    }
}

#[async_trait::async_trait]
impl crate::core::_CAsyncServiceModule for DMSAuthModule {
    /// Returns the name of the authentication module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
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
    fn _Fis_critical(&self) -> bool {
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
    async fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Initializing DMS Auth Module");

        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        // Update configuration if provided
        if let Some(auth_config) = cfg._Fget("auth") {
            self.config = serde_json::from_str(auth_config)
                .unwrap_or_else(|_| DMSAuthConfig::default());
        }

        // Initialize JWT manager with new config
        self.jwt_manager = Arc::new(DMSJWTManager::_Fnew(self.config.jwt_secret.clone(), self.config.jwt_expiry_secs));

        // Initialize OAuth providers if configured
        if !self.config.oauth_providers.is_empty() {
            for provider_name in &self.config.oauth_providers {
                // Note: This is a placeholder - actual provider registration would require full provider details
                // For now, we'll skip registration since we don't have the complete provider configuration
                println!("Skipping OAuth provider registration for: {}", provider_name);
            }
        }

        println!("DMS Auth Module initialized successfully");
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
    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Cleaning up DMS Auth Module");
        
        // Cleanup sessions
        let session_mgr = self.session_manager.write().await;
        session_mgr._Fcleanup_all().await?;
        
        println!("DMS Auth Module cleanup completed");
        Ok(())
    }
}

impl crate::core::_CServiceModule for DMSAuthModule {
    /// Returns the name of the authentication module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
        "DMS.Auth"
    }

    /// Indicates whether the authentication module is critical.
    /// 
    /// # Returns
    /// 
    /// `false` since authentication is non-critical
    fn _Fis_critical(&self) -> bool {
        false
    }

    /// Initializes the authentication module synchronously.
    /// 
    /// This method is a no-op for the authentication module, as all actual initialization is handled
    /// in the async `_Finit` method.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success
    fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Async initialization is handled in _Finit
        Ok(())
    }

    /// Performs synchronous cleanup after the application has shut down.
    /// 
    /// This method is a no-op for the authentication module, as all actual cleanup is handled
    /// in the async `_Fafter_shutdown` method.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success
    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Async cleanup is handled in _Fafter_shutdown
        Ok(())
    }
}