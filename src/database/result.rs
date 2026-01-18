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

use crate::database::DMSCDBRow;
use std::sync::Arc;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSCDBResult {
    rows: Arc<Vec<DMSCDBRow>>,
    row_count: usize,
    affected_rows: u64,
    last_insert_id: Option<i64>,
}

impl DMSCDBResult {
    pub fn new() -> Self {
        Self {
            rows: Arc::new(Vec::new()),
            row_count: 0,
            affected_rows: 0,
            last_insert_id: None,
        }
    }

    pub fn with_rows(rows: Vec<DMSCDBRow>) -> Self {
        let row_count = rows.len();
        Self {
            rows: Arc::new(rows),
            row_count,
            affected_rows: row_count as u64,
            last_insert_id: None,
        }
    }

    pub fn with_affected_rows(affected_rows: u64) -> Self {
        Self {
            rows: Arc::new(Vec::new()),
            row_count: 0,
            affected_rows,
            last_insert_id: None,
        }
    }

    pub fn affected_rows(&self) -> u64 {
        self.affected_rows
    }

    pub fn last_insert_id(&self) -> Option<i64> {
        self.last_insert_id
    }

    pub fn set_last_insert_id(&mut self, id: i64) {
        self.last_insert_id = Some(id);
    }

    pub fn is_empty(&self) -> bool {
        self.row_count == 0
    }

    pub fn len(&self) -> usize {
        self.row_count
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn rows(&self) -> &[DMSCDBRow] {
        &self.rows
    }

    pub fn iter(&self) -> impl Iterator<Item = &DMSCDBRow> {
        self.rows.iter()
    }

    pub fn first(&self) -> Option<&DMSCDBRow> {
        self.rows.first()
    }

    pub fn last(&self) -> Option<&DMSCDBRow> {
        self.rows.last()
    }

    pub fn get(&self, index: usize) -> Option<&DMSCDBRow> {
        self.rows.get(index)
    }

    pub fn into_rows(self) -> Vec<DMSCDBRow> {
        Arc::try_unwrap(self.rows).unwrap_or_else(|arc| (*arc).clone())
    }

    pub fn to_vec(&self) -> Vec<DMSCDBRow> {
        self.rows.iter().cloned().collect()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCDBResult {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    fn get_affected_rows(&self) -> u64 {
        self.affected_rows
    }

    fn get_last_insert_id(&self) -> Option<i64> {
        self.last_insert_id
    }

    fn is_empty_result(&self) -> bool {
        self.is_empty()
    }

    fn get_length(&self) -> usize {
        self.len()
    }

    fn get_row_count(&self) -> usize {
        self.row_count()
    }

    fn to_rows(&self) -> Vec<DMSCDBRow> {
        self.to_vec()
    }
}

impl Default for DMSCDBResult {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for DMSCDBResult {
    type Item = DMSCDBRow;
    type IntoIter = std::vec::IntoIter<DMSCDBRow>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_rows().into_iter()
    }
}
