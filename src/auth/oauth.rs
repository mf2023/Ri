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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthProvider {
    pub id: String,
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub scopes: Vec<String>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
    pub token_type: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSOAuthUserInfo {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar_url: Option<String>,
    pub provider: String,
}

pub struct DMSOAuthManager {
    providers: RwLock<HashMap<String, DMSOAuthProvider>>,
    token_cache: Arc<crate::cache::DMSCache>,
}

impl DMSOAuthManager {
    pub fn _Fnew(cache: Arc<crate::cache::DMSCache>) -> Self {
        Self {
            providers: RwLock::new(HashMap::new()),
            token_cache: cache,
        }
    }

    pub async fn _Fregister_provider(&self, provider: DMSOAuthProvider) -> crate::core::DMSResult<()> {
        let mut providers = self.providers.write().await;
        providers.insert(provider.id.clone(), provider);
        Ok(())
    }

    pub async fn _Fget_provider(&self, provider_id: &str) -> crate::core::DMSResult<Option<DMSOAuthProvider>> {
        let providers = self.providers.read().await;
        Ok(providers.get(provider_id).cloned())
    }

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

    pub async fn _Flist_providers(&self) -> crate::core::DMSResult<Vec<DMSOAuthProvider>> {
        let providers = self.providers.read().await;
        Ok(providers.values().cloned().collect())
    }

    pub async fn _Fdisable_provider(&self, provider_id: &str) -> crate::core::DMSResult<bool> {
        let mut providers = self.providers.write().await;
        
        if let Some(provider) = providers.get_mut(provider_id) {
            provider.enabled = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

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