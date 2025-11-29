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

//! OAuth 2.0 authentication implementation for DMS.
//! 
//! This module provides OAuth 2.0 authentication functionality, including support for
//! multiple identity providers, token management, and user information retrieval.
//! It implements the OAuth 2.0 authorization code flow and supports token refresh and revocation.
//! 
//! # Design Principles
//! - **Multi-Provider Support**: Allows registration of multiple OAuth providers
//! - **Thread Safety**: Uses RwLock for concurrent access to provider configuration
//! - **Caching**: Integrates with DMS cache for token storage
//! - **Async Operations**: All network operations are asynchronous
//! - **Extensibility**: Designed to support additional OAuth flows and providers
//! 
//! # Usage Examples
//! ```rust
//! // Create an OAuth manager with a cache
//! let cache = Arc::new(crate::cache::backends::memory_backend::DMSMemoryCache::_Fnew());
//! let oauth_manager = DMSOAuthManager::_Fnew(cache);
//! 
//! // Register a Google OAuth provider
//! let google_provider = DMSOAuthProvider {
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
//! oauth_manager._Fregister_provider(google_provider).await?;
//! 
//! // Get authentication URL for a provider
//! let auth_url = oauth_manager._Fget_auth_url("google", "state123").await?;
//! 
//! // Exchange authorization code for token
//! let token = oauth_manager._Fexchange_code_for_token(
//!     "google",
//!     "auth_code",
//!     "http://localhost:8080/auth/callback"
//! ).await?;
//! 
//! // Get user information
//! if let Some(token) = token {
//!     let user_info = oauth_manager._Fget_user_info("google", &token.access_token).await?;
//! }
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// OAuth provider configuration.
/// 
/// This struct defines the configuration for an OAuth identity provider,
/// including client credentials, endpoints, and scopes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthProvider {
    pub id: String,                  // Unique identifier for the provider
    pub name: String,                // Human-readable name of the provider
    pub client_id: String,           // OAuth client ID
    pub client_secret: String,       // OAuth client secret
    pub auth_url: String,            // Authorization endpoint URL
    pub token_url: String,           // Token endpoint URL
    pub user_info_url: String,       // User information endpoint URL
    pub scopes: Vec<String>,         // Requested OAuth scopes
    pub enabled: bool,               // Whether the provider is enabled
}

/// OAuth token response.
/// 
/// This struct represents the token response from an OAuth provider,
/// including access token, refresh token, and expiration information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthToken {
    pub access_token: String,        // Access token for API requests
    pub refresh_token: Option<String>, // Refresh token for obtaining new access tokens
    pub expires_in: Option<i64>,     // Token expiration time in seconds
    pub token_type: String,          // Token type (usually "Bearer")
    pub scope: Option<String>,       // Granted scopes
}

/// OAuth user information.
/// 
/// This struct represents the user information retrieved from an OAuth provider,
/// including user ID, email, name, and profile information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthUserInfo {
    pub id: String,                  // Unique user ID from the provider
    pub email: String,               // User's email address
    pub name: Option<String>,        // User's full name
    pub avatar_url: Option<String>,  // URL to user's avatar image
    pub provider: String,            // Name of the OAuth provider
}

/// OAuth manager for handling multiple identity providers.
/// 
/// This struct manages OAuth providers, handles token exchange, and retrieves user information.
pub struct DMSOAuthManager {
    providers: RwLock<HashMap<String, DMSOAuthProvider>>, // Registered OAuth providers
    _token_cache: Arc<dyn crate::cache::DMSCache>,        // Cache for storing tokens
}

impl DMSOAuthManager {
    /// Creates a new OAuth manager with the specified cache.
    /// 
    /// # Parameters
    /// - `cache`: Cache implementation for storing tokens
    /// 
    /// # Returns
    /// A new instance of `DMSOAuthManager`
    pub fn _Fnew(cache: Arc<dyn crate::cache::DMSCache>) -> Self {
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
    pub async fn _Fregister_provider(&self, provider: DMSOAuthProvider) -> crate::core::DMSResult<()> {
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
    /// `Some(DMSOAuthProvider)` if the provider exists, otherwise `None`
    pub async fn _Fget_provider(&self, provider_id: &str) -> crate::core::DMSResult<Option<DMSOAuthProvider>> {
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
    pub async fn _Fget_auth_url(&self, provider_id: &str, state: &str) -> crate::core::DMSResult<Option<String>> {
        let providers = self.providers.read().await;
        
        if let Some(provider) = providers.get(provider_id) {
            if !provider.enabled {
                return Ok(None);
            }

            let scope = provider.scopes.join(" ");
            let auth_url = format!(
                "{}?client_id={}&redirect_uri=http://localhost:8080/auth/callback&response_type=code&scope={}&state={}",
                provider.auth_url,
                provider.client_id,
                urlencoding::encode(&scope),
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
    /// `Some(DMSOAuthToken)` if the code exchange was successful, otherwise `None`
    pub async fn _Fexchange_code_for_token(
        &self,
        provider_id: &str,
        code: &str,
        redirect_uri: &str,
    ) -> crate::core::DMSResult<Option<DMSOAuthToken>> {
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
                .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let token_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

                let token = DMSOAuthToken {
                    access_token: token_data["access_token"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSError::ExternalError("Missing access_token".to_string()))?
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

    /// Retrieves user information from an OAuth provider.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `access_token`: Access token for authentication
    /// 
    /// # Returns
    /// `Some(DMSOAuthUserInfo)` if the user information was successfully retrieved, otherwise `None`
    pub async fn _Fget_user_info(
        &self,
        provider_id: &str,
        access_token: &str,
    ) -> crate::core::DMSResult<Option<DMSOAuthUserInfo>> {
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
                .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let user_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

                let user_info = DMSOAuthUserInfo {
                    id: user_data["id"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSError::ExternalError("Missing user id".to_string()))?
                        .to_string(),
                    email: user_data["email"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSError::ExternalError("Missing email".to_string()))?
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

    /// Refreshes an access token using a refresh token.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `refresh_token`: Refresh token for obtaining a new access token
    /// 
    /// # Returns
    /// `Some(DMSOAuthToken)` if the token refresh was successful, otherwise `None`
    pub async fn _Frefresh_token(
        &self,
        provider_id: &str,
        refresh_token: &str,
    ) -> crate::core::DMSResult<Option<DMSOAuthToken>> {
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
                .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

            if response.status().is_success() {
                let token_data: serde_json::Value = response.json().await
                    .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

                let token = DMSOAuthToken {
                    access_token: token_data["access_token"]
                        .as_str()
                        .ok_or_else(|| crate::core::DMSError::ExternalError("Missing access_token".to_string()))?
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

    /// Revokes an access token.
    /// 
    /// # Parameters
    /// - `provider_id`: Unique identifier of the provider
    /// - `access_token`: Access token to revoke
    /// 
    /// # Returns
    /// `true` if the token was successfully revoked, otherwise `false`
    pub async fn _Frevoke_token(
        &self,
        provider_id: &str,
        access_token: &str,
    ) -> crate::core::DMSResult<bool> {
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
                .post(&format!("{}/revoke", provider.token_url))
                .form(&params)
                .send()
                .await
                .map_err(|e| crate::core::DMSError::ExternalError(e.to_string()))?;

            Ok(response.status().is_success())
        } else {
            Ok(false)
        }
    }

    /// Lists all registered OAuth providers.
    /// 
    /// # Returns
    /// A vector of all registered OAuth providers
    pub async fn _Flist_providers(&self) -> crate::core::DMSResult<Vec<DMSOAuthProvider>> {
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
    pub async fn _Fdisable_provider(&self, provider_id: &str) -> crate::core::DMSResult<bool> {
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
    pub async fn _Fenable_provider(&self, provider_id: &str) -> crate::core::DMSResult<bool> {
        let mut providers = self.providers.write().await;
        
        if let Some(provider) = providers.get_mut(provider_id) {
            provider.enabled = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}