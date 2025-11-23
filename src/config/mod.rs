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

// Configuration module for DMS.
// First-stage implementation: in-memory key-value store with future extension points.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

// Basic configuration storage.
pub struct DMSConfig {
    values: HashMap<String, String>,
}

impl DMSConfig {
    pub fn _Fnew() -> Self {
        DMSConfig { values: HashMap::new() }
    }

    pub fn _Fset(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    pub fn _Fget(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    pub fn _Fget_str(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    pub fn _Fget_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|s| {
            let v = s.trim().to_ascii_lowercase();
            match v.as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            }
        })
    }

    pub fn _Fget_i64(&self, key: &str) -> Option<i64> {
        self.values.get(key).and_then(|s| s.trim().parse::<i64>().ok())
    }

    pub fn _Fget_u64(&self, key: &str) -> Option<u64> {
        self.values.get(key).and_then(|s| s.trim().parse::<u64>().ok())
    }

    pub fn _Fget_f32(&self, key: &str) -> Option<f32> {
        self.values.get(key).and_then(|s| s.trim().parse::<f32>().ok())
    }
}

// Public-facing configuration manager.
pub struct DMSConfigManager {
    config: DMSConfig,
}

impl DMSConfigManager {
    pub fn _Fnew_default() -> Self {
        let mut cfg = DMSConfig::_Fnew();

        // Load defaults from config/dms.json if exists (simple flat key-value JSON object).
        let mut config_path = PathBuf::new();
        if let Ok(cwd) = std::env::current_dir() {
            config_path = cwd.join("config").join("dms.json");
        }

        if !config_path.as_os_str().is_empty() && config_path.exists() {
            if let Ok(text) = fs::read_to_string(&config_path) {
                if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&text) {
                    for (k, v) in map {
                        cfg._Fset(k, v);
                    }
                }
            }
        }

        for (name, value) in std::env::vars() {
            if let Some(rest) = name.strip_prefix("DMS_") {
                let key_parts: Vec<String> = rest
                    .split("__")
                    .map(|part| part.to_ascii_lowercase())
                    .collect();
                let key = key_parts.join(".");
                if !key.is_empty() {
                    cfg._Fset(key, value);
                }
            }
        }

        DMSConfigManager { config: cfg }
    }

    pub fn _Fconfig(&self) -> &DMSConfig {
        &self.config
    }
}
