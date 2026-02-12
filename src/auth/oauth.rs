//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! OAuth 2.0 authentication implementation for DMSC.
//! 
//! This module provides OAuth 2.0 authentication functionality, including support for
//! multiple identity providers, token management, and user information retrieval.
//! It implements the OAuth 2.0 authorization code flow and supports token refresh and revocation.
//! 
//! # Design Principles
//! - **Multi-Provider Support**: Allows registration of multiple OAuth providers
//! - **Thread Safety**: Uses RwLock for concurrent access to provider configuration
//! - **Caching**: Integrates with DMSC cache for token storage
//! - **Async Operations**: All network operations are asynchronous
//! - **Extensibility**: Designed to support additional OAuth flows and providers
//! 
//! # Usage Examples
//! ```rust
//! // Create an OAuth manager with a cache
//! let cache = Arc::new(crate::cache::backends::memory_backend::DMSCMemoryCache::new());
//! let oauth_manager = DMSCOAuthManager::new(cache);
//! 
//! // Register a Google OAuth provider
//! let google_provider = DMSCOAuthProvider {
//!     id: "google".to_string(),
//!     name: "Google".to_string(),
//!     client_id: "client_id".to_string(),
//!     client_secret: "client_secret".to_string(),
//!     auth_url: "https://accounts.google.com/o/oauth2/auth".to_string(),
//!     token_url: "https://oauth2.googleapis.com/token".to_string(),
//!     user_info_url: "https://www.googleapis.com/oauth2/v3/userinfo".to_string(),
//!     scopes: vec!["openid", "email", "profile"].iter().map(|s| s.to_string()).collect(),
//!     enabled: true,
//! };
//! oauth_manager.register_provider(google_provider).await?;
//! 
//! // Get authentication URL for a provider
//! let auth_url = oauth_manager.get_auth_url("google", "state123").await?;
//! 
//! // Exchange authorization code for token
//! let token = oauth_manager.exchange_code_for_token(
//!     "google",
//!     "auth_code",
//!     "http://localhost:8080/auth/callback"
//! ).await?;
//! 
//! // Get user information
//! if let Some(token) = token {
//!     let user_info = oauth_manager.get_user_info("google", &token.access_token).await?;
//! }
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::runtime::Runtime;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

#[cfg(feature = "auth")]
extern crate urlencoding;

/// OAuth provider configuration.
///
/// This struct defines the configuration for an OAuth identity provider,
/// including client credentials, endpoints, scopes, and redirect URI.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCOAuthProvider {
    /// Unique identifier for the OAuth provider
    pub id: String,
    /// Human-readable name of the provider (e.g., "Google", "GitHub")
    pub name: String,
    /// OAuth client ID issued by the provider
    pub client_id: String,
    /// OAuth client secret issued by the provider
    pub client_secret: String,
    /// Authorization endpoint URL for initiating OAuth flow
    pub auth_url: String,
    /// Token endpoint URL for exchanging authorization codes
    pub token_url: String,
    /// User information endpoint URL for retrieving user details
    pub user_info_url: String,
    /// Requested OAuth scopes (e.g., "openid", "email", "profile")
    pub scopes: Vec<String>,
    /// Whether the provider is enabled for authentication
    pub enabled: bool,
    /// Redirect URI for OAuth callback (defaults to "http://localhost:8080/auth/callback" if not set)
    pub redirect_uri: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCOAuthProvider {
    #[new]
    fn py_new(
        id: String,
        name: String,
        client_id: String,
        client_secret: String,
        auth_url: String,
        token_url: String,
        user_info_url: String,
        scopes: Vec<String>,
        enabled: bool,
        redirect_uri: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            client_id,
            client_secret,
            auth_url,
            token_url,
            user_info_url,
            scopes,
            enabled,
            redirect_uri,
        }
    }
}

/// OAuth token response.
///
/// This struct represents the token response from an OAuth provider,
/// including access token, refresh token, and expiration information.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCOAuthToken {
    /// Access token for making authenticated API requests
    pub access_token: String,
    /// Refresh token for obtaining new access tokens when expired
    pub refresh_token: Option<String>,
    /// Token expiration time in seconds from issuance
    pub expires_in: Option<i64>,
    /// Token type (typically "Bearer")
    pub token_type: String,
    /// Granted scopes (may differ from requested scopes)
    pub scope: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCOAuthToken {
    #[new]
    fn py_new(
        access_token: String,
        token_type: String,
        refresh_token: Option<String>,
        scope: Option<String>,
        expires_in: Option<i64>,
    ) -> Self {
        Self {
            access_token,
            token_type,
            refresh_token,
            scope,
            expires_in,
        }
    }
}

/// OAuth user information.
///
/// This struct represents the user information retrieved from an OAuth provider,
/// including user ID, email, name, and profile information.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCOAuthUserInfo {
    /// Unique user identifier from the OAuth provider
    pub id: String,
    /// User's email address from the provider
    pub email: String,
    /// User's full name from the provider
    pub name: Option<String>,
    /// URL to user's avatar profile image
    pub avatar_url: Option<String>,
    /// Name of the OAuth provider that authenticated the user
    pub provider: String,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCOAuthUserInfo {
    #[new]
    fn py_new(
        id: String,
        email: String,
        name: Option<String>,
        avatar_url: Option<String>,
        provider: String,
    ) -> Self {
        Self {
            id,
            email,
            name,
            avatar_url,
            provider,
        }
    }
}

/// OAuth manager for handling multiple identity providers.
///
/// This struct manages OAuth providers, handles token exchange, and retrieves user information.
/// It supports concurrent access through RwLock and integrates with the DMSC cache system
/// for token storage.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCOAuthManager {
    /// Hash map of registered OAuth providers indexed by provider ID
    providers: RwLock<HashMap<String, DMSCOAuthProvider>>,
    /// Cache implementation for storing OAuth tokens
    _token_cache: Arc<dyn crate::cache::DMSCCache>,
}

impl DMSCOAuthManager {
    /// Creates a new OAuth manager with the specified cache.
    /// 
    /// # Parameters
    /// - `cache`: Cache implementation for storing tokens
    /// 
    /// # Returns
    /// A new instance of `DMSCOAuthManager`
    pub fn new(cache: Arc<dyn crate::cache::DMSCCache>) -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            _token_cache: cache,
        }
    }

    /// Registers a new OAuth provider.
    /// 
    /// # Parameters
    /// - `provider`: OAuth provider configuration
    /// 
    /// # Returns
    /// `Ok(())` if the provider was successfully registered
    pub async fn register_provider(&self, provider: DMSCOAuthProvider) -> crate::core::DMSCResult<()> {
        let mut providers = self.providers.write().await;
        providers.insert(provider.id.clone(), provider);
        Ok(())
    }

    /// Gets an OAuth provider by ID.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// 
    /// # Returns
    /// `Some(DMSCOAuthProvider)` if the provider exists, otherwise `None`
    pub async fn get_provider(&self, provider_id: &str) -> crate::core::DMSCResult<Option<DMSCOAuthProvider>> {
        let providers = self.providers.read().await;
        Ok(providers.get(provider_id).cloned())
    }

    /// Gets the authentication URL for a provider.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `state`: State parameter for CSRF protection
    /// 
    /// # Returns
    /// `Some(String)` containing the authentication URL if the provider is enabled, otherwise `None`
    pub async fn get_auth_url(&self, provider_id: &str, state: &str) -> crate::core::DMSCResult<Option<String>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(None);
            }

            let scope = provider.scopes.join(" ");
            let encoded_scope = scope.clone();
            let redirect_uri = provider.redirect_uri.as_deref()
                .unwrap_or("http://localhost:8080/auth/callback");
            let auth_url = format!(
                "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
                provider.auth_url,
                provider.client_id,
                urlencoding::encode(redirect_uri),
                encoded_scope,
                state
            );
            
            Ok(Some(auth_url))
        } else {
            Ok(None)
        }
    }

    /// Exchanges an authorization code for an access token.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `code`: Authorization code from the provider
    /// - `redirect_uri`: Redirect URI used in the authentication request
    /// 
    /// # Returns
    /// `Some(DMSCOAuthToken)` if the code exchange was successful, otherwise `None`
    #[cfg(feature = "http_client")]
    pub async fn exchange_code_for_token(
        &self,
        provider_id: &str,
        code: &str,
        redirect_uri: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthToken>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(None);
            }

            let client = reqwest::Client::new();
            let params = [
                ("grant_type", "authorization_code"),
                ("code", code),
                ("redirect_uri", redirect_uri),
                ("client_id", &provider.client_id),
                ("client_secret", &provider.client_secret),
            ];

            let response = client
                .post(&provider.token_url)
                .form(&params)
                .send()
                .await
                .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let token_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

                let token = DMSCOAuthToken {
                    access_token: token_data["access_token"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSCError::ExternalError("Missing access_token".to_string()))?
                        .to_string(),
                    refresh_token: token_data["refresh_token"].as_str().map(String::from),
                    expires_in: token_data["expires_in"].as_i64(),
                    token_type: token_data["token_type"]
                        .as_str()
                        .unwrap_or("Bearer")
                        .to_string(),
                    scope: token_data["scope"].as_str().map(String::from),
                };

                Ok(Some(token))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    #[cfg(not(feature = "http_client"))]
    /// Exchanges an authorization code for an access token.
    ///
    /// This method requires the `http_client` feature to be enabled.
    /// Without this feature, calling this method returns an error.
    ///
    /// # Parameters
    ///
    /// - `_provider_id`: Unique identifier of the provider (not used when feature is disabled)
    /// - `_code`: Authorization code from the provider (not used when feature is disabled)
    /// - `_redirect_uri`: Redirect URI used in the authentication request (not used when feature is disabled)
    ///
    /// # Returns
    ///
    /// A Result containing an error indicating the http_client feature is not enabled
    pub async fn exchange_code_for_token(
        &self,
        _provider_id: &str,
        _code: &str,
        _redirect_uri: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthToken>> {
        Err(crate::core::DMSCError::Other("HTTP client is not enabled. Enable the 'http_client' feature to use OAuth functionality.".to_string()))
    }

    /// Retrieves user information from an OAuth provider.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `access_token`: Access token for authentication
    /// 
    /// # Returns
    /// `Some(DMSCOAuthUserInfo)` if the user information was successfully retrieved, otherwise `None`
    #[cfg(feature = "http_client")]
    pub async fn get_user_info(
        &self,
        provider_id: &str,
        access_token: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthUserInfo>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(None);
            }

            let client = reqwest::Client::new();
            let response = client
                .get(&provider.user_info_url)
                .bearer_auth(access_token)
                .send()
                .await
                .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let user_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

                let user_info = DMSCOAuthUserInfo {
                    id: user_data["id"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSCError::ExternalError("Missing user id".to_string()))?
                        .to_string(),
                    email: user_data["email"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSCError::ExternalError("Missing email".to_string()))?
                        .to_string(),
                    name: user_data["name"].as_str().map(String::from),
                    avatar_url: user_data["avatar_url"].as_str().map(String::from),
                    provider: provider_id.to_string(),
                };

                Ok(Some(user_info))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    #[cfg(not(feature = "http_client"))]
    /// Retrieves user information from an OAuth provider.
    ///
    /// This method requires the `http_client` feature to be enabled.
    /// Without this feature, calling this method returns an error.
    ///
    /// # Parameters
    ///
    /// - `_provider_id`: Unique identifier of the provider (not used when feature is disabled)
    /// - `_access_token`: Access token for authentication (not used when feature is disabled)
    ///
    /// # Returns
    ///
    /// A Result containing an error indicating the http_client feature is not enabled
    pub async fn get_user_info(
        &self,
        _provider_id: &str,
        _access_token: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthUserInfo>> {
        Err(crate::core::DMSCError::Other("HTTP client is not enabled. Enable the 'http_client' feature to use OAuth functionality.".to_string()))
    }

    /// Refreshes an access token using a refresh token.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `refresh_token`: Refresh token for obtaining a new access token
    /// 
    /// # Returns
    /// `Some(DMSCOAuthToken)` if the token refresh was successful, otherwise `None`
    #[cfg(feature = "http_client")]
    pub async fn refresh_token(
        &self,
        provider_id: &str,
        refresh_token: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthToken>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(None);
            }

            let client = reqwest::Client::new();
            let params = [
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &provider.client_id),
                ("client_secret", &provider.client_secret),
            ];

            let response = client
                .post(&provider.token_url)
                .form(&params)
                .send()
                .await
                .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let token_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

                let token = DMSCOAuthToken {
                    access_token: token_data["access_token"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSCError::ExternalError("Missing access_token".to_string()))?
                        .to_string(),
                    refresh_token: token_data["refresh_token"].as_str().map(String::from),
                    expires_in: token_data["expires_in"].as_i64(),
                    token_type: token_data["token_type"]
                        .as_str()
                        .unwrap_or("Bearer")
                        .to_string(),
                    scope: token_data["scope"].as_str().map(String::from),
                };

                Ok(Some(token))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    #[cfg(not(feature = "http_client"))]
    /// Refreshes an access token using a refresh token.
    ///
    /// This method requires the `http_client` feature to be enabled.
    /// Without this feature, calling this method returns an error.
    ///
    /// # Parameters
    ///
    /// - `_provider_id`: Unique identifier of the provider (not used when feature is disabled)
    /// - `_refresh_token`: Refresh token for obtaining a new access token (not used when feature is disabled)
    ///
    /// # Returns
    ///
    /// A Result containing an error indicating the http_client feature is not enabled
    pub async fn refresh_token(
        &self,
        _provider_id: &str,
        _refresh_token: &str,
    ) -> crate::core::DMSCResult<Option<DMSCOAuthToken>> {
        Err(crate::core::DMSCError::Other("HTTP client is not enabled. Enable the 'http_client' feature to use OAuth functionality.".to_string()))
    }

    /// Revokes an access token.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `access_token`: Access token to revoke
    /// 
    /// # Returns
    /// `true` if the token was successfully revoked, otherwise `false`
    #[cfg(feature = "http_client")]
    pub async fn revoke_token(
        &self,
        provider_id: &str,
        access_token: &str,
    ) -> crate::core::DMSCResult<bool> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(false);
            }

            let client = reqwest::Client::new();
            let params = [
                ("token", access_token),
                ("client_id", &provider.client_id),
                ("client_secret", &provider.client_secret),
            ];

            let response = client
                .post(format!("{}/revoke", provider.token_url))
                .form(&params)
                .send()
                .await
                .map_err(|e| crate::core::DMSCError::ExternalError(e.to_string()))?;

            Ok(response.status().is_success())
        } else {
            Ok(false)
        }
    }
    
    #[cfg(not(feature = "http_client"))]
    /// Revokes an access token.
    ///
    /// This method requires the `http_client` feature to be enabled.
    /// Without this feature, calling this method returns an error.
    ///
    /// # Parameters
    ///
    /// - `_provider_id`: Unique identifier of the provider (not used when feature is disabled)
    /// - `_access_token`: Access token to revoke (not used when feature is disabled)
    ///
    /// # Returns
    ///
    /// A Result containing an error indicating the http_client feature is not enabled
    pub async fn revoke_token(
        &self,
        _provider_id: &str,
        _access_token: &str,
    ) -> crate::core::DMSCResult<bool> {
        Err(crate::core::DMSCError::Other("HTTP client is not enabled. Enable the 'http_client' feature to use OAuth functionality.".to_string()))
    }

    /// Lists all registered OAuth providers.
    /// 
    /// # Returns
    /// A vector of all registered OAuth providers
    pub async fn list_providers(&self) -> crate::core::DMSCResult<Vec<DMSCOAuthProvider>> {
        let providers = self.providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    /// Disables an OAuth provider.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// 
    /// # Returns
    /// `true` if the provider was successfully disabled, otherwise `false`
    pub async fn disable_provider(&self, provider_id: &str) -> crate::core::DMSCResult<bool> {
        let mut providers = self.providers.write().await;
        
        if let Some(provider) = providers.get_mut(provider_id) {
            provider.enabled = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Enables an OAuth provider.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// 
    /// # Returns
    /// `true` if the provider was successfully enabled, otherwise `false`
    pub async fn enable_provider(&self, provider_id: &str) -> crate::core::DMSCResult<bool> {
        let mut providers = self.providers.write().await;
        
        if let Some(provider) = providers.get_mut(provider_id) {
            provider.enabled = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for the OAuth Manager.
///
/// This module provides Python interface to DMSC OAuth functionality,
/// enabling Python applications to integrate with OAuth identity providers.
///
/// ## Supported Operations
///
/// - Provider registration and management
/// - Authentication URL generation for OAuth flows
/// - Token exchange with authorization codes
/// - User information retrieval from OAuth providers
/// - Token refresh and revocation
///
/// ## Python Usage Example
///
/// ```python
/// from dmsc import DMSCOAuthProvider, DMSCOAuthManager
///
/// # Create OAuth manager
/// oauth_manager = DMSCOAuthManager()
///
/// # Register a provider
/// provider = DMSCOAuthProvider(
///     id="google",
///     name="Google",
///     client_id="your_client_id",
///     client_secret="your_client_secret",
///     auth_url="https://accounts.google.com/o/oauth2/auth",
///     token_url="https://oauth2.googleapis.com/token",
///     user_info_url="https://www.googleapis.com/oauth2/v3/userinfo",
///     scopes=["openid", "email", "profile"],
///     enabled=True,
/// )
/// # Note: Async operations require Python 3.7+ with asyncio
/// ```
#[pyo3::prelude::pymethods]
impl DMSCOAuthManager {
    #[new]
    fn py_new() -> PyResult<Self> {
        let cache = Arc::new(crate::cache::DMSCMemoryCache::new());
        Ok(Self::new(cache))
    }
    
    #[pyo3(name = "register_provider")]
    fn register_provider_impl(&self, provider: DMSCOAuthProvider) -> PyResult<bool> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.register_provider(provider).await?;
            Ok(true)
        })
    }
    
    #[pyo3(name = "get_provider")]
    fn get_provider_impl(&self, provider_id: String) -> PyResult<Option<DMSCOAuthProvider>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_provider(&provider_id).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_auth_url")]
    fn get_auth_url_impl(&self, provider_id: String, state: String) -> PyResult<Option<String>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_auth_url(&provider_id, &state).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "exchange_code_for_token")]
    fn exchange_code_for_token_impl(&self, provider_id: String, code: String, redirect_uri: String) -> PyResult<Option<DMSCOAuthToken>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.exchange_code_for_token(&provider_id, &code, &redirect_uri).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_user_info")]
    fn get_user_info_impl(&self, provider_id: String, access_token: String) -> PyResult<Option<DMSCOAuthUserInfo>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_user_info(&provider_id, &access_token).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "refresh_token")]
    fn refresh_token_impl(&self, provider_id: String, refresh_token: String) -> PyResult<Option<DMSCOAuthToken>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.refresh_token(&provider_id, &refresh_token).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "revoke_token")]
    fn revoke_token_impl(&self, provider_id: String, access_token: String) -> PyResult<bool> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.revoke_token(&provider_id, &access_token).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "list_providers")]
    fn list_providers_impl(&self) -> PyResult<Vec<DMSCOAuthProvider>> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.list_providers().await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "disable_provider")]
    fn disable_provider_impl(&self, provider_id: String) -> PyResult<bool> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.disable_provider(&provider_id).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "enable_provider")]
    fn enable_provider_impl(&self, provider_id: String) -> PyResult<bool> {
        let rt = Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.enable_provider(&provider_id).await.map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
}
