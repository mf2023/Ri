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
use crate::database::{RiDatabase, RiDatabasePool};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tokio::runtime::Runtime;

#[pyclass]
pub struct RiPyORMRepository {
    pool: RiDatabasePool,
    table_name: String,
    rt: Runtime,
}

#[pymethods]
impl RiPyORMRepository {
    #[new]
    #[pyo3(signature = (pool, table_name))]
    pub fn new(pool: RiDatabasePool, table_name: &str) -> PyResult<Self> {
        let rt = Runtime::new().map_err(|e| pyo3::PyErr::from(crate::core::RiError::Other(e.to_string())))?;
        Ok(Self {
            pool,
            table_name: table_name.to_string(),
            rt,
        })
    }

    pub fn get_table_name(&self) -> &str {
        &self.table_name
    }

    pub fn find_all(&self, py: Python) -> PyResult<Py<PyList>> {
        let result = self.rt.block_on(async {
            self.find_all_impl().await
        }).map_err(|e| pyo3::PyErr::from(e))?;
        let list = PyList::empty(py);
        for value in result {
            let dict = PyDict::new(py);
            if let serde_json::Value::Object(map) = value {
                for (k, v) in map {
                    Self::json_to_py(py, k, v, &dict);
                }
            }
            list.append(dict).map_err(|e| pyo3::PyErr::from(e))?;
        }
        Ok(list.into())
    }

    pub fn find_by_id(&self, id: &str, py: Python) -> PyResult<Option<Py<PyDict>>> {
        let result = self.rt.block_on(async {
            self.find_by_id_impl(id).await
        }).map_err(|e| pyo3::PyErr::from(e))?;
        match result {
            Some(value) => {
                let dict = PyDict::new(py);
                if let serde_json::Value::Object(map) = value {
                    for (k, v) in map {
                        Self::json_to_py(py, k, v, &dict);
                    }
                }
                Ok(Some(dict.into()))
            }
            None => Ok(None),
        }
    }

    pub fn count(&self) -> PyResult<u64> {
        self.rt.block_on(async {
            self.count_impl().await
        }).map_err(|e| pyo3::PyErr::from(e))
    }

    pub fn exists(&self, id: &str) -> PyResult<bool> {
        self.rt.block_on(async {
            self.exists_impl(id).await
        }).map_err(|e| pyo3::PyErr::from(e))
    }

    pub fn delete(&self, id: &str) -> PyResult<()> {
        self.rt.block_on(async {
            self.delete_impl(id).await
        }).map_err(|e| pyo3::PyErr::from(e))
    }
}

impl RiPyORMRepository {
    fn json_to_py(py: Python, key: String, value: serde_json::Value, dict: &Bound<PyDict>) {
        match value {
            serde_json::Value::Null => {},
            serde_json::Value::Bool(b) => { let _ = dict.set_item(key, b); },
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    let _ = dict.set_item(key, i);
                } else if let Some(f) = n.as_f64() {
                    let _ = dict.set_item(key, f);
                } else {
                    let _ = dict.set_item(key, n.to_string());
                }
            }
            serde_json::Value::String(s) => { let _ = dict.set_item(key, s); },
            serde_json::Value::Array(arr) => {
                let list = PyList::empty(py);
                for (idx, v) in arr.into_iter().enumerate() {
                    let item = PyDict::new(py);
                    Self::json_to_py(py, idx.to_string(), v, &item);
                    let _ = list.append(item);
                }
                let _ = dict.set_item(key, list);
            }
            serde_json::Value::Object(map) => {
                let nested = PyDict::new(py);
                for (k, v) in map {
                    Self::json_to_py(py, k, v, &nested);
                }
                let _ = dict.set_item(key, nested);
            }
        }
    }

    async fn find_all_impl(&self) -> RiResult<Vec<serde_json::Value>> {
        let db = self.pool.get().await?;
        let sql = format!("SELECT * FROM {}", self.table_name);
        let result = db.query(&sql).await?;
        let mut entities = Vec::with_capacity(4);
        for row in result {
            let json_value = serde_json::to_value(row.to_map())?;
            entities.push(json_value);
        }
        Ok(entities)
    }

    async fn find_by_id_impl(&self, id: &str) -> RiResult<Option<serde_json::Value>> {
        let db = self.pool.get().await?;
        let sql = format!("SELECT * FROM {} WHERE id = ?", self.table_name);
        let result = db.query_with_params(&sql, &[serde_json::json!(id)]).await?;
        if let Some(row) = result.first() {
            let json_value = serde_json::to_value(row.to_map())?;
            Ok(Some(json_value))
        } else {
            Ok(None)
        }
    }

    async fn count_impl(&self) -> RiResult<u64> {
        let db = self.pool.get().await?;
        let sql = format!("SELECT COUNT(*) as total FROM {}", self.table_name);
        if let Some(row) = db.query_one(&sql).await? {
            Ok(row.get::<i64>("total").map(|v| v as u64).unwrap_or(0))
        } else {
            Ok(0)
        }
    }

    async fn exists_impl(&self, id: &str) -> RiResult<bool> {
        let db = self.pool.get().await?;
        let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
        let result = db.query_with_params(&sql, &[serde_json::json!(id)]).await?;
        Ok(!result.is_empty())
    }

    async fn delete_impl(&self, id: &str) -> RiResult<()> {
        let db = self.pool.get().await?;
        let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
        db.execute_with_params(&sql, &[serde_json::json!(id)]).await?;
        Ok(())
    }
}
