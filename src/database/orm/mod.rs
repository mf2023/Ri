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

//! # ORM Module
//!
//! This module provides ORM-like database operations for DMSC.

use crate::core::{DMSCResult, DMSCError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::database::DMSCDatabase;

pub mod repository;

pub use repository::{DMSCORMSimpleRepository, DMSCORMCrudRepository, DMSCORMRepository};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct ColumnDefinition {
    pub name: String,
    pub column_type: String,
    pub is_primary_key: bool,
    pub is_nullable: bool,
    pub default_value: Option<String>,
    pub is_unique: bool,
    pub max_length: Option<usize>,
}

impl Default for ColumnDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            column_type: "TEXT".to_string(),
            is_primary_key: false,
            is_nullable: true,
            default_value: None,
            is_unique: false,
            max_length: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct IndexDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub is_unique: bool,
    pub is_full_text: bool,
}

impl Default for IndexDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            columns: Vec::new(),
            is_unique: false,
            is_full_text: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct ForeignKeyDefinition {
    pub name: String,
    pub column: String,
    pub referenced_table: String,
    pub referenced_column: String,
    pub on_delete: String,
    pub on_update: String,
}

impl Default for ForeignKeyDefinition {
    fn default() -> Self {
        Self {
            name: String::new(),
            column: String::new(),
            referenced_table: String::new(),
            referenced_column: String::new(),
            on_delete: "CASCADE".to_string(),
            on_update: "CASCADE".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableDefinition {
    pub table_name: String,
    pub columns: HashMap<String, ColumnDefinition>,
    pub primary_key: Vec<String>,
    pub indexes: Vec<IndexDefinition>,
    pub foreign_keys: Vec<ForeignKeyDefinition>,
    pub engine: Option<String>,
    pub charset: Option<String>,
}

impl TableDefinition {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            columns: HashMap::new(),
            primary_key: Vec::new(),
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
            engine: None,
            charset: None,
        }
    }

    pub fn add_column(&mut self, column: ColumnDefinition) {
        self.columns.insert(column.name.clone(), column);
    }

    pub fn set_primary_key(&mut self, columns: Vec<String>) {
        self.primary_key = columns;
    }

    pub fn add_index(&mut self, index: IndexDefinition) {
        self.indexes.push(index);
    }

    pub fn add_foreign_key(&mut self, fk: ForeignKeyDefinition) {
        self.foreign_keys.push(fk);
    }

    pub fn get_create_sql(&self) -> String {
        let mut sql = format!("CREATE TABLE IF NOT EXISTS {} (", self.table_name);

        let mut column_defs = Vec::new();

        for (name, col) in &self.columns {
            let mut def = format!("{} {}", name, col.column_type);

            if !col.is_nullable {
                def.push_str(" NOT NULL");
            }

            if col.is_primary_key {
                def.push_str(" PRIMARY KEY");
            }

            if let Some(default) = &col.default_value {
                def.push_str(&format!(" DEFAULT {}", default));
            }

            if col.is_unique {
                def.push_str(" UNIQUE");
            }

            column_defs.push(def);
        }

        if !self.primary_key.is_empty() {
            column_defs.push(format!("PRIMARY KEY ({})", self.primary_key.join(", ")));
        }

        sql.push_str(&column_defs.join(", "));

        for fk in &self.foreign_keys {
            sql.push_str(&format!(
                ", FOREIGN KEY ({}) REFERENCES {}({}) ON DELETE {} ON UPDATE {}",
                fk.column, fk.referenced_table, fk.referenced_column, fk.on_delete, fk.on_update
            ));
        }

        sql.push_str(")");

        if let Some(engine) = &self.engine {
            sql.push_str(&format!(" ENGINE={}", engine));
        }

        if let Some(charset) = &self.charset {
            sql.push_str(&format!(" DEFAULT CHARSET={}", charset));
        }

        sql.push_str(";");

        sql
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Like,
    ILike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct Criteria {
    pub column: String,
    pub operator: ComparisonOperator,
    pub value: serde_json::Value,
}

impl Criteria {
    pub fn new(column: &str, operator: ComparisonOperator, value: serde_json::Value) -> Self {
        Self {
            column: column.to_string(),
            operator,
            value,
        }
    }

    pub fn to_sql(&self) -> (String, Vec<serde_json::Value>) {
        let (op_str, value) = match self.operator {
            ComparisonOperator::Equal => ("=".to_string(), vec![self.value.clone()]),
            ComparisonOperator::NotEqual => ("!=".to_string(), vec![self.value.clone()]),
            ComparisonOperator::GreaterThan => (">".to_string(), vec![self.value.clone()]),
            ComparisonOperator::GreaterThanOrEqual => (">=".to_string(), vec![self.value.clone()]),
            ComparisonOperator::LessThan => ("<".to_string(), vec![self.value.clone()]),
            ComparisonOperator::LessThanOrEqual => ("<=".to_string(), vec![self.value.clone()]),
            ComparisonOperator::Like => ("LIKE".to_string(), vec![self.value.clone()]),
            ComparisonOperator::ILike => ("ILIKE".to_string(), vec![self.value.clone()]),
            ComparisonOperator::In => {
                if let serde_json::Value::Array(arr) = &self.value {
                    let placeholders = (0..arr.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
                    (format!("IN ({})", placeholders), arr.clone())
                } else {
                    ("= ?".to_string(), vec![self.value.clone()])
                }
            }
            ComparisonOperator::NotIn => {
                if let serde_json::Value::Array(arr) = &self.value {
                    let placeholders = (0..arr.len()).map(|_| "?").collect::<Vec<_>>().join(", ");
                    (format!("NOT IN ({})", placeholders), arr.clone())
                } else {
                    ("!= ?".to_string(), vec![self.value.clone()])
                }
            }
            ComparisonOperator::IsNull => ("IS NULL".to_string(), Vec::new()),
            ComparisonOperator::IsNotNull => ("IS NOT NULL".to_string(), Vec::new()),
            ComparisonOperator::Between => {
                if let serde_json::Value::Array(arr) = &self.value {
                    if arr.len() == 2 {
                        ("BETWEEN ? AND ?".to_string(), arr.clone())
                    } else {
                        ("BETWEEN ? AND ?".to_string(), vec![self.value.clone()])
                    }
                } else {
                    ("BETWEEN ? AND ?".to_string(), vec![self.value.clone()])
                }
            }
        };

        let placeholder = if matches!(self.operator, ComparisonOperator::IsNull | ComparisonOperator::IsNotNull) {
            "".to_string()
        } else {
            "?".to_string()
        };

        (format!("{} {} {}", self.column, op_str, placeholder), value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct SortOrder {
    pub column: String,
    pub ascending: bool,
}

impl SortOrder {
    pub fn new(column: &str, ascending: bool) -> Self {
        Self {
            column: column.to_string(),
            ascending,
        }
    }

    pub fn asc(column: &str) -> Self {
        Self::new(column, true)
    }

    pub fn desc(column: &str) -> Self {
        Self::new(column, false)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct Pagination {
    pub page: u64,
    pub page_size: u64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl Pagination {
    pub fn new(page: u64, page_size: u64) -> Self {
        Self { page, page_size }
    }

    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> u64 {
        self.page_size
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct QueryBuilder {
    pub table_name: String,
    pub criteria: Vec<Criteria>,
    pub sort_orders: Vec<SortOrder>,
    pub pagination: Option<Pagination>,
    pub select_columns: Option<Vec<String>>,
    pub group_by_columns: Option<Vec<String>>,
    pub having_criteria: Vec<Criteria>,
    pub distinct: bool,
    pub joins: Vec<JoinClause>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct JoinClause {
    pub join_type: JoinType,
    pub table_name: String,
    pub on_column: String,
    pub referenced_column: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

impl QueryBuilder {
    pub fn new(table_name: &str) -> Self {
        Self {
            table_name: table_name.to_string(),
            criteria: Vec::new(),
            sort_orders: Vec::new(),
            pagination: None,
            select_columns: None,
            group_by_columns: None,
            having_criteria: Vec::new(),
            distinct: false,
            joins: Vec::new(),
        }
    }

    pub fn select(&mut self, columns: Vec<&str>) -> &mut Self {
        self.select_columns = Some(columns.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn where_criteria(&mut self, criteria: Criteria) -> &mut Self {
        self.criteria.push(criteria);
        self
    }

    pub fn and_where(&mut self, criteria: Criteria) -> &mut Self {
        self.where_criteria(criteria)
    }

    pub fn or_where(&mut self, criteria: Criteria) -> &mut Self {
        self.criteria.push(criteria);
        self
    }

    pub fn order_by(&mut self, sort_order: SortOrder) -> &mut Self {
        self.sort_orders.push(sort_order);
        self
    }

    pub fn paginate(&mut self, page: u64, page_size: u64) -> &mut Self {
        self.pagination = Some(Pagination::new(page, page_size));
        self
    }

    pub fn distinct(&mut self) -> &mut Self {
        self.distinct = true;
        self
    }

    pub fn group_by(&mut self, columns: Vec<&str>) -> &mut Self {
        self.group_by_columns = Some(columns.iter().map(|s| s.to_string()).collect());
        self
    }

    pub fn inner_join(&mut self, table_name: &str, on_column: &str, referenced_column: &str) -> &mut Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Inner,
            table_name: table_name.to_string(),
            on_column: on_column.to_string(),
            referenced_column: referenced_column.to_string(),
        });
        self
    }

    pub fn left_join(&mut self, table_name: &str, on_column: &str, referenced_column: &str) -> &mut Self {
        self.joins.push(JoinClause {
            join_type: JoinType::Left,
            table_name: table_name.to_string(),
            on_column: on_column.to_string(),
            referenced_column: referenced_column.to_string(),
        });
        self
    }

    pub fn build(&self) -> (String, Vec<serde_json::Value>) {
        let mut sql = String::new();
        let mut params = Vec::new();

        sql.push_str("SELECT ");

        if self.distinct {
            sql.push_str("DISTINCT ");
        }

        if let Some(columns) = &self.select_columns {
            sql.push_str(&columns.join(", "));
        } else {
            sql.push_str("*");
        }

        sql.push_str(&format!(" FROM {}", self.table_name));

        for join in &self.joins {
            let join_type = match join.join_type {
                JoinType::Inner => "INNER JOIN",
                JoinType::Left => "LEFT JOIN",
                JoinType::Right => "RIGHT JOIN",
                JoinType::Full => "FULL JOIN",
            };
            sql.push_str(&format!(
                " {} {} ON {}.{} = {}.{}",
                join_type, join.table_name, self.table_name, join.on_column, join.table_name, join.referenced_column
            ));
        }

        if !self.criteria.is_empty() {
            sql.push_str(" WHERE 1=1");
            for criteria in &self.criteria {
                sql.push_str(" AND ");
                let (clause, values) = criteria.to_sql();
                sql.push_str(&clause);
                params.extend(values);
            }
        }

        if let Some(group_by) = &self.group_by_columns {
            sql.push_str(&format!(" GROUP BY {}", group_by.join(", ")));
        }

        if !self.having_criteria.is_empty() {
            sql.push_str(" HAVING 1=1");
            for criteria in &self.having_criteria {
                sql.push_str(" AND ");
                let (clause, values) = criteria.to_sql();
                sql.push_str(&clause);
                params.extend(values);
            }
        }

        if !self.sort_orders.is_empty() {
            let orders: Vec<String> = self.sort_orders.iter()
                .map(|o| format!("{} {}", o.column, if o.ascending { "ASC" } else { "DESC" }))
                .collect();
            sql.push_str(&format!(" ORDER BY {}", orders.join(", ")));
        }

        if let Some(pagination) = &self.pagination {
            sql.push_str(&format!(" LIMIT {} OFFSET {}", pagination.limit(), pagination.offset()));
        }

        (sql, params)
    }
}
