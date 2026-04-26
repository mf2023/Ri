//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

use crate::core::RiResult;
use crate::database::{RiDBResult, RiDBRow};
use async_trait::async_trait;
use serde::Serialize;
use std::sync::Arc;

#[async_trait]
pub trait RiDBStatement: Send + Sync {
    fn sql(&self) -> &str;
    fn param_count(&self) -> usize;
    async fn execute(&self, params: &[&dyn Serialize]) -> RiResult<RiDBResult>;
    async fn execute_row(&self, params: &[&dyn Serialize]) -> RiResult<Option<RiDBRow>>;
    async fn query(&self, params: &[&dyn Serialize]) -> RiResult<RiDBResult>;
    async fn query_one(&self, params: &[&dyn Serialize]) -> RiResult<Option<RiDBRow>>;
}

pub struct PreparedStatement {
    sql: String,
    params: Vec<String>,
    cached_result: Option<RiDBResult>,
}

impl PreparedStatement {
    pub fn new(sql: &str) -> Self {
        let params = extract_params(sql);
        Self {
            sql: sql.to_string(),
            params,
            cached_result: None,
        }
    }

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn param_count(&self) -> usize {
        self.params.len()
    }

    pub fn params(&self) -> &[String] {
        &self.params
    }
}

fn extract_params(sql: &str) -> Vec<String> {
    let mut params = Vec::with_capacity(8);
    let mut chars = sql.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            let mut param = String::from("$");
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    if let Some(digit) = chars.next() {
                        param.push(digit);
                    }
                } else {
                    break;
                }
            }
            if param != "$" {
                params.push(param);
            }
        } else if c == '?' {
            params.push("?".to_string());
        }
    }
    params
}

pub struct StatementCache {
    cache: Arc<dashmap::DashMap<String, Arc<dyn RiDBStatement>>>,
    max_size: usize,
    hit_count: std::sync::atomic::AtomicUsize,
    miss_count: std::sync::atomic::AtomicUsize,
}

impl StatementCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            max_size,
            hit_count: AtomicUsize::new(0),
            miss_count: AtomicUsize::new(0),
        }
    }

    pub fn get(&self, sql: &str) -> Option<Arc<dyn RiDBStatement>> {
        if let Some(entry) = self.cache.get(sql) {
            self.hit_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Some(entry.value().clone())
        } else {
            self.miss_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            None
        }
    }

    pub fn insert(&self, sql: &str, statement: Arc<dyn RiDBStatement>) {
        if self.cache.len() >= self.max_size {
            if let Some(entry) = self.cache.iter().next() {
                self.cache.remove(entry.key());
            }
        }
        self.cache.insert(sql.to_string(), statement);
    }

    pub fn stats(&self) -> (usize, usize, usize, f64) {
        let hits = self.hit_count.load(std::sync::atomic::Ordering::SeqCst);
        let misses = self.miss_count.load(std::sync::atomic::Ordering::SeqCst);
        let total = hits + misses;
        let ratio = if total > 0 { hits as f64 / total as f64 } else { 0.0 };
        (hits, misses, self.cache.len(), ratio)
    }

    pub fn clear(&self) {
        self.cache.clear();
    }
}

impl Default for StatementCache {
    fn default() -> Self {
        Self::new(100)
    }
}
