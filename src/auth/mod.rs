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

pub mod jwt;
pub mod oauth;
pub mod permissions;
pub mod session;

pub use jwt::{DMSJWTManager, JWTClaims, JWTValidationOptions};
pub use oauth::{DMSOAuthManager, DMSOAuthProvider, DMSOAuthToken, DMSOAuthUserInfo};
pub use permissions::{DMSPermission, DMSPermissionManager, DMSRole};
pub use session::{DMSSession, DMSSessionManager};

use crate::core::{DMSResult, DMSServiceContext};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct DMSAuthConfig {
    pub enabled: bool,
    pub jwt_secret: String,
    pub jwt_expiry_secs: u64,
    pub session_timeout_secs: u64,
    pub oauth_providers: Vec<String>,
    pub enable_api_keys: bool,
    pub enable_session_auth: bool,
}

impl Default for DMSAuthConfig {
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

pub struct DMSAuthModule {
    config: DMSAuthConfig,
    jwt_manager: Arc<DMSJWTManager>,
    session_manager: Arc<RwLock<DMSSessionManager>>,
    permission_manager: Arc<RwLock<DMSPermissionManager>>,
    oauth_manager: Arc<RwLock<DMSOAuthManager>>,
}

impl DMSAuthModule {
    pub fn _Fnew(config: DMSAuthConfig) -> Self {
        let jwt_manager = Arc::new(DMSJWTManager::_Fnew(config.jwt_secret.clone(), config.jwt_expiry_secs));
        let session_manager = Arc::new(RwLock::new(DMSSessionManager::_Fnew(config.session_timeout_secs)));
        let permission_manager = Arc::new(RwLock::new(DMSPermissionManager::_Fnew()));
        let oauth_manager = Arc::new(RwLock::new(DMSOAuthManager::_Fnew()));

        Self {
            config,
            jwt_manager,
            session_manager,
            permission_manager,
            oauth_manager,
        }
    }

    pub fn _Fjwt_manager(&self) -> Arc<DMSJWTManager> {
        self.jwt_manager.clone()
    }

    pub fn _Fsession_manager(&self) -> Arc<RwLock<DMSSessionManager>> {
        self.session_manager.clone()
    }

    pub fn _Fpermission_manager(&self) -> Arc<RwLock<DMSPermissionManager>> {
        self.permission_manager.clone()
    }

    pub fn _Foauth_manager(&self) -> Arc<RwLock<DMSOAuthManager>> {
        self.oauth_manager.clone()
    }
}

#[async_trait::async_trait]
impl crate::core::_CAsyncServiceModule for DMSAuthModule {
    fn _Fname(&self) -> &str {
        "DMS.Auth"
    }

    fn _Fis_critical(&self) -> bool {
        false // Auth failures should not break the application
    }

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
            let mut oauth_mgr = self.oauth_manager.write().await;
            for provider_name in &self.config.oauth_providers {
                oauth_mgr._Fadd_provider(provider_name.as_str()).await?;
            }
        }

        println!("DMS Auth Module initialized successfully");
        Ok(())
    }

    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Cleaning up DMS Auth Module");
        
        // Cleanup sessions
        let mut session_mgr = self.session_manager.write().await;
        session_mgr._Fcleanup_all().await?;
        
        println!("DMS Auth Module cleanup completed");
        Ok(())
    }
}

impl crate::core::_CServiceModule for DMSAuthModule {
    fn _Fname(&self) -> &str {
        "DMS.Auth"
    }

    fn _Fis_critical(&self) -> bool {
        false
    }

    fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Async initialization is handled in _Finit
        Ok(())
    }

    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Async cleanup is handled in _Fafter_shutdown
        Ok(())
    }
}