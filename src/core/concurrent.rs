//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

//! # Sharded Lock Implementation
//!
//! This module provides a sharded lock data structure (`DMSCShardedLock`) that
//! improves concurrent performance by reducing lock contention. Instead of using
//! a single global lock, the data is partitioned into multiple shards, each with
//! its own lock.
//!
//! ## Key Benefits
//!
//! - **Reduced Lock Contention**: Multiple threads can access different shards simultaneously
//! - **Better Scalability**: Performance improves with more shards for high-concurrency scenarios
//! - **Uniform Distribution**: Uses hash-based sharding for even key distribution
//! - **Thread Safety**: All operations are thread-safe using async RwLock
//!
//! ## Design Principles
//!
//! 1. **Sharding Strategy**: Keys are distributed using `hash(key) % shard_count`
//! 2. **Lock Granularity**: Each shard has its own RwLock for fine-grained locking
//! 3. **Default Shard Count**: 16 shards by default, configurable based on workload
//! 4. **Zero-Cost Abstraction**: Sharding adds minimal overhead to operations
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use dmsc::core::concurrent::DMSCShardedLock;
//! use std::collections::HashMap;
//!
//! let sharded_map = DMSCShardedLock::<String, String>::new(16);
//!
//! // Insert a value
//! sharded_map.insert("key1".to_string(), "value1".to_string()).await;
//!
//! // Get a value
//! let value = sharded_map.get("key1").await;
//!
//! // Remove a value
//! sharded_map.remove("key1").await;
//! ```

use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "pyo3")]
use pyo3::pyclass;

const DEFAULT_SHARD_COUNT: usize = 16;

fn calculate_hash<K: Hash + ?Sized>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[inline]
fn get_shard_index<K: Hash + ?Sized>(key: &K, shard_count: usize) -> usize {
    let hash = calculate_hash(key);
    (hash as usize) % shard_count
}

struct Shard<K, V> {
    data: RwLock<HashMap<K, V>>,
}

impl<K, V> Shard<K, V> {
    fn new() -> Self {
        Self {
            data: RwLock::new(HashMap::new()),
        }
    }
}

pub struct DMSCShardedLock<K, V> {
    shards: Vec<Arc<Shard<K, V>>>,
    shard_count: usize,
}

impl<K, V> DMSCShardedLock<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(shard_count: usize) -> Self {
        let actual_shard_count = if shard_count == 0 { DEFAULT_SHARD_COUNT } else { shard_count };
        let shards = (0..actual_shard_count)
            .map(|_| Arc::new(Shard::new()))
            .collect();

        Self {
            shards,
            shard_count: actual_shard_count,
        }
    }

    pub fn with_default_shards() -> Self {
        Self::new(DEFAULT_SHARD_COUNT)
    }

    #[inline]
    fn get_shard(&self, key: &K) -> &Arc<Shard<K, V>> {
        let index = get_shard_index(key, self.shard_count);
        &self.shards[index]
    }

    pub async fn insert(&self, key: K, value: V) -> Option<V> {
        let shard = self.get_shard(&key);
        let mut data = shard.data.write().await;
        data.insert(key, value)
    }

    pub async fn get<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let shard_index = get_shard_index(key, self.shard_count);
        let shard = &self.shards[shard_index];
        let data = shard.data.read().await;
        data.get(key).cloned()
    }

    pub async fn get_mut<F, R, Q>(&self, key: &Q, f: F) -> Option<R>
    where
        F: FnOnce(&mut V) -> R,
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let shard_index = get_shard_index(key, self.shard_count);
        let shard = &self.shards[shard_index];
        let mut data = shard.data.write().await;
        data.get_mut(key).map(f)
    }

    pub async fn remove<Q>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let shard_index = get_shard_index(key, self.shard_count);
        let shard = &self.shards[shard_index];
        let mut data = shard.data.write().await;
        data.remove(key)
    }

    pub async fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let shard_index = get_shard_index(key, self.shard_count);
        let shard = &self.shards[shard_index];
        let data = shard.data.read().await;
        data.contains_key(key)
    }

    pub async fn len(&self) -> usize {
        let mut total = 0;
        for shard in &self.shards {
            let data = shard.data.read().await;
            total += data.len();
        }
        total
    }

    pub async fn is_empty(&self) -> bool {
        self.len().await == 0
    }

    pub async fn clear(&self) {
        for shard in &self.shards {
            let mut data = shard.data.write().await;
            data.clear();
        }
    }

    pub async fn retain<F>(&self, mut f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        for shard in &self.shards {
            let mut data = shard.data.write().await;
            data.retain(|k, v| f(k, v));
        }
    }

    pub async fn for_each<F>(&self, mut f: F)
    where
        F: FnMut(&K, &V),
    {
        for shard in &self.shards {
            let data = shard.data.read().await;
            for (k, v) in data.iter() {
                f(k, v);
            }
        }
    }

    pub async fn for_each_mut<F>(&self, mut f: F)
    where
        F: FnMut(&K, &mut V),
    {
        for shard in &self.shards {
            let mut data = shard.data.write().await;
            for (k, v) in data.iter_mut() {
                f(k, v);
            }
        }
    }

    pub async fn collect_all(&self) -> HashMap<K, V> {
        let mut result = HashMap::new();
        for shard in &self.shards {
            let data = shard.data.read().await;
            for (k, v) in data.iter() {
                result.insert(k.clone(), v.clone());
            }
        }
        result
    }

    pub async fn collect_where<F>(&self, mut predicate: F) -> Vec<V>
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut result = Vec::new();
        for shard in &self.shards {
            let data = shard.data.read().await;
            for (k, v) in data.iter() {
                if predicate(k, v) {
                    result.push(v.clone());
                }
            }
        }
        result
    }

    pub async fn count_where<F>(&self, mut predicate: F) -> usize
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut count = 0;
        for shard in &self.shards {
            let data = shard.data.read().await;
            for (k, v) in data.iter() {
                if predicate(k, v) {
                    count += 1;
                }
            }
        }
        count
    }

    pub async fn remove_where<F>(&self, mut predicate: F) -> usize
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut removed_count = 0;
        for shard in &self.shards {
            let mut data = shard.data.write().await;
            let before_len = data.len();
            data.retain(|k, v| !predicate(k, v));
            removed_count += before_len - data.len();
        }
        removed_count
    }

    pub async fn update<F, R>(&self, key: &K, f: F) -> Option<R>
    where
        F: FnOnce(Option<&mut V>) -> R,
    {
        let shard = self.get_shard(key);
        let mut data = shard.data.write().await;
        Some(f(data.get_mut(key)))
    }

    pub async fn get_or_insert<F>(&self, key: K, default: F) -> V
    where
        F: FnOnce() -> V,
    {
        let shard = self.get_shard(&key);
        let mut data = shard.data.write().await;
        data.entry(key).or_insert_with(default).clone()
    }

    pub async fn get_or_insert_with_key<F>(&self, key: K, default: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        let shard = self.get_shard(&key);
        let mut data = shard.data.write().await;
        data.entry(key.clone()).or_insert_with(|| default(&key)).clone()
    }

    pub fn shard_count(&self) -> usize {
        self.shard_count
    }
}

impl<K, V> Default for DMSCShardedLock<K, V>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::with_default_shards()
    }
}

#[allow(dead_code)]
pub struct DMSCShardedLockReadGuard<'a, K, V> {
    shard_index: usize,
    guard: tokio::sync::RwLockReadGuard<'a, HashMap<K, V>>,
}

#[allow(dead_code)]
pub struct DMSCShardedLockWriteGuard<'a, K, V> {
    shard_index: usize,
    guard: tokio::sync::RwLockWriteGuard<'a, HashMap<K, V>>,
}

#[cfg_attr(feature = "pyo3", pyclass)]
pub struct DMSCShardedLockStats {
    pub shard_count: usize,
    pub total_entries: usize,
    pub shard_distribution: Vec<usize>,
}

impl DMSCShardedLockStats {
    pub fn new(shard_count: usize, total_entries: usize, shard_distribution: Vec<usize>) -> Self {
        Self {
            shard_count,
            total_entries,
            shard_distribution,
        }
    }

    pub fn calc_load_factor(&self) -> f64 {
        if self.shard_count == 0 {
            return 0.0;
        }
        self.total_entries as f64 / self.shard_count as f64
    }

    pub fn calc_distribution_variance(&self) -> f64 {
        if self.shard_count == 0 || self.total_entries == 0 {
            return 0.0;
        }
        let mean = self.total_entries as f64 / self.shard_count as f64;
        let variance: f64 = self.shard_distribution.iter()
            .map(|&count| {
                let diff = count as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / self.shard_count as f64;
        variance
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCShardedLockStats {
    #[getter]
    fn shard_count(&self) -> usize {
        self.shard_count
    }

    #[getter]
    fn total_entries(&self) -> usize {
        self.total_entries
    }

    #[getter]
    fn shard_distribution(&self) -> Vec<usize> {
        self.shard_distribution.clone()
    }

    fn load_factor(&self) -> f64 {
        self.calc_load_factor()
    }

    fn distribution_variance(&self) -> f64 {
        self.calc_distribution_variance()
    }
}
