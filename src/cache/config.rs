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

use serde::{Serialize, Deserialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCacheConfig {
    pub enabled: bool,
    pub default_ttl_secs: u64,
    pub max_memory_mb: u64,
    pub cleanup_interval_secs: u64,
    pub backend_type: CacheBackendType,
    pub redis_url: String,
    pub redis_pool_size: usize,
}

impl Default for DMSCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl_secs: 3600, // 1 hour
            max_memory_mb: 512,
            cleanup_interval_secs: 300, // 5 minutes
            backend_type: CacheBackendType::Memory,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            redis_pool_size: 10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CacheBackendType {
    Memory,
    Redis,
    Hybrid, // Memory + Redis
}

impl CacheBackendType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "redis" => CacheBackendType::Redis,
            "hybrid" => CacheBackendType::Hybrid,
            _ => CacheBackendType::Memory,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    pub ttl: Option<Duration>,
    pub refresh_on_access: bool,
    pub max_size: Option<usize>,
}

impl Default for CachePolicy {
    fn default() -> Self {
        Self {
            ttl: Some(Duration::from_secs(3600)),
            refresh_on_access: false,
            max_size: None,
        }
    }
}