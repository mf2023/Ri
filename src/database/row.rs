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

use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap as FxHashMap;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct RiDBRow {
    pub columns: Vec<String>,
    pub values: Vec<Option<serde_json::Value>>,
}

impl RiDBRow {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.columns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.columns.is_empty()
    }

    pub fn column_names(&self) -> &[String] {
        &self.columns
    }

    pub fn has_column(&self, name: &str) -> bool {
        self.columns.iter().any(|c| c.eq_ignore_ascii_case(name))
    }

    pub fn index_of(&self, name: &str) -> Option<usize> {
        self.columns.iter().position(|c| c.eq_ignore_ascii_case(name))
    }

    pub fn get<T: for<'de> Deserialize<'de> + Send + Sync>(&self, name: &str) -> Option<T> {
        let idx = self.index_of(name)?;
        let value = self.values[idx].as_ref()?;
        serde_json::from_value(value.clone()).ok()
    }

    pub fn get_opt<T: for<'de> Deserialize<'de> + Send + Sync>(&self, name: &str) -> Option<Option<T>> {
        let idx = self.index_of(name)?;
        if self.values[idx].is_none() {
            return Some(None);
        }
        Some(self.get(name))
    }

    pub fn get_by_index<T: for<'de> Deserialize<'de> + Send + Sync>(&self, index: usize) -> Option<T> {
        if index >= self.values.len() {
            return None;
        }
        let value = self.values[index].as_ref()?;
        serde_json::from_value(value.clone()).ok()
    }

    pub fn get_raw(&self, name: &str) -> Option<&serde_json::Value> {
        let idx = self.index_of(name)?;
        self.values[idx].as_ref()
    }

    pub fn get_raw_by_index(&self, index: usize) -> Option<&serde_json::Value> {
        if index >= self.values.len() {
            return None;
        }
        self.values[index].as_ref()
    }

    pub fn try_get<T: for<'de> Deserialize<'de> + Send + Sync>(&self, name: &str) -> Result<T, crate::core::RiError> {
        self.get(name).ok_or_else(|| crate::core::RiError::InvalidInput(format!("Column '{} not found or type mismatch", name)))
    }

    pub fn get_i32(&self, name: &str) -> Option<i32> {
        self.get::<i32>(name)
    }

    pub fn get_i64(&self, name: &str) -> Option<i64> {
        self.get::<i64>(name)
    }

    pub fn get_f64(&self, name: &str) -> Option<f64> {
        self.get::<f64>(name)
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.get::<bool>(name)
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        self.get::<String>(name)
    }

    pub fn get_bytes(&self, name: &str) -> Option<Vec<u8>> {
        self.get::<Vec<u8>>(name)
    }

    pub fn is_null(&self, name: &str) -> bool {
        let idx = match self.index_of(name) {
            Some(i) => i,
            None => return false,
        };
        self.values[idx].is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &str, Option<&serde_json::Value>)> {
        self.columns.iter().enumerate().map(|(i, col)| {
            (i, col.as_str(), self.values[i].as_ref())
        })
    }

    pub fn to_map(&self) -> FxHashMap<String, Option<serde_json::Value>> {
        let mut map = FxHashMap::default();
        for (i, col) in self.columns.iter().enumerate() {
            map.insert(col.clone(), self.values[i].clone());
        }
        map
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiDBRow {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    fn get_length(&self) -> usize {
        self.columns.len()
    }

    fn is_empty_row(&self) -> bool {
        self.is_empty()
    }

    fn check_has_column(&self, name: &str) -> bool {
        self.has_column(name)
    }

    fn get_string_value(&self, name: &str) -> Option<String> {
        self.get_string(name)
    }

    fn get_column_names(&self) -> Vec<String> {
        self.columns.clone()
    }

    fn to_dict(&self) -> FxHashMap<String, Option<String>> {
        let mut map = FxHashMap::default();
        for (i, col) in self.columns.iter().enumerate() {
            let value = self.values[i].as_ref().map(|v| v.to_string());
            map.insert(col.clone(), value);
        }
        map
    }
}

#[allow(dead_code)]
pub struct RiRowBuilder {
    row: RiDBRow,
}

#[allow(dead_code)]
impl RiRowBuilder {
    pub fn new() -> Self {
        Self { row: RiDBRow::new() }
    }

    pub fn add_column(mut self, name: &str) -> Self {
        self.row.columns.push(name.to_string());
        self.row.values.push(None);
        self
    }

    pub fn add_null(mut self, name: &str) -> Self {
        self.row.columns.push(name.to_string());
        self.row.values.push(None);
        self
    }

    pub fn add_value<T: Serialize + Send + Sync>(mut self, name: &str, value: T) -> Self {
        if let Some(idx) = self.row.index_of(name) {
            let json = serde_json::to_value(value).unwrap_or_default();
            self.row.values[idx] = Some(json);
        } else {
            let json = serde_json::to_value(value).unwrap_or_default();
            self.row.columns.push(name.to_string());
            self.row.values.push(Some(json));
        }
        self
    }

    pub fn build(self) -> RiDBRow {
        self.row
    }
}

#[allow(dead_code)]
impl Default for RiRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
