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

use std::sync::Arc;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// Cache manager that coordinates different cache backends
pub struct DMSCacheManager {
    backend: Arc<dyn DMSCache + Send + Sync>,
}

impl DMSCacheManager {
    pub fn _Fnew(backend: Arc<dyn DMSCache + Send + Sync>) -> Self {
        Self { backend }
    }
    
    pub async fn _Fget<T: serde::de::DeserializeOwned>(&self, key: &str) -> crate::core::DMSResult<Option<T>> {
        match self.backend._Fget(key).await {
            Some(cached_value) => {
                match cached_value._Fdeserialize::<T>() {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
        }
    }
    
    pub async fn _Fset<T: serde::Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> crate::core::DMSResult<()> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| crate::core::DMSError::Other(format!("Serialization error: {}", e)))?;
        
        let cached_value = CachedValue::_Fnew(serde_json::Value::String(serialized), ttl_seconds.map(|s| std::time::Duration::from_secs(s)));
        self.backend._Fset(key, cached_value).await
    }
    
    pub async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        self.backend._Fdelete(key).await
    }
    
    pub async fn _Fexists(&self, key: &str) -> bool {
        self.backend._Fexists(key).await
    }
    
    pub async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        self.backend._Fclear().await
    }
    
    pub async fn _Fstats(&self) -> CacheStats {
        self.backend._Fstats().await
    }
    
    pub async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        self.backend._Fcleanup_expired().await
    }
    
    pub async fn _Fget_or_set<T, F>(&self, key: &str, ttl_seconds: Option<u64>, factory: F) -> crate::core::DMSResult<T>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone,
        F: FnOnce() -> crate::core::DMSResult<T>,
    {
        // Try to get from cache first
        if let Some(value) = self._Fget::<T>(key).await? {
            return Ok(value);
        }
        
        // If not found, generate the value
        let value = factory()?;
        
        // Store in cache
        self._Fset(key, &value, ttl_seconds).await?;
        
        Ok(value)
    }
}