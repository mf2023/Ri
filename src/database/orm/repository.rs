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

//! # ORM Repository
//!
//! This module provides repository implementations for ORM operations.

use super::*;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait]
pub trait DMSCORMRepository<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync>: Send + Sync {
    fn table_name(&self) -> &'static str;
    
    async fn find_all(&self, db: &dyn DMSCDatabase) -> DMSCResult<Vec<E>>;
    async fn find_by_id(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<Option<E>>;
    async fn find_one(&self, db: &dyn DMSCDatabase, criteria: &Criteria) -> DMSCResult<Option<E>>;
    async fn find_many(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<Vec<E>>;
    async fn find_paginated(&self, db: &dyn DMSCDatabase, pagination: Pagination, criteria: Vec<Criteria>) -> DMSCResult<(Vec<E>, u64)>;
    async fn count(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<u64>;
    
    async fn save(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<E>;
    async fn save_many(&self, db: &dyn DMSCDatabase, entities: &[E]) -> DMSCResult<Vec<E>>;
    async fn update(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<E>;
    async fn delete(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<()>;
    async fn delete_by_id(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<()>;
    async fn delete_many(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<u64>;
    
    async fn exists(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<bool>;
    async fn exists_by(&self, db: &dyn DMSCDatabase, criteria: &Criteria) -> DMSCResult<bool>;
    
    async fn batch_insert(&self, db: &dyn DMSCDatabase, entities: &[E], batch_size: usize) -> DMSCResult<Vec<E>>;
    async fn upsert(&self, db: &dyn DMSCDatabase, entity: &E, conflict_columns: &[&str]) -> DMSCResult<E>;
}

#[async_trait]
pub trait DMSCORMCrudRepository<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync>: Send + Sync {
    fn table_name(&self) -> &'static str;
    
    async fn find_all(&self) -> DMSCResult<Vec<E>>;
    async fn find_by_id(&self, id: &str) -> DMSCResult<Option<E>>;
    async fn save(&self, entity: &E) -> DMSCResult<E>;
    async fn delete(&self, entity: &E) -> DMSCResult<()>;
}

#[derive(Debug, Clone)]
pub struct DMSCORMSimpleRepository<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync> {
    table_name: &'static str,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync> DMSCORMSimpleRepository<E> {
    pub fn new(table_name: &'static str) -> Self {
        Self {
            table_name,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync> DMSCORMRepository<E> for DMSCORMSimpleRepository<E> {
    fn table_name(&self) -> &'static str {
        self.table_name
    }

    async fn find_all(&self, db: &dyn DMSCDatabase) -> DMSCResult<Vec<E>> {
        let sql = format!("SELECT * FROM {}", self.table_name);
        let result = db.query(&sql).await?;
        
        let mut entities = Vec::new();
        for row in result {
            let json_value = serde_json::to_value(row.to_map())?;
            let entity: E = serde_json::from_value(json_value)?;
            entities.push(entity);
        }
        
        Ok(entities)
    }

    async fn find_by_id(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<Option<E>> {
        let sql = format!("SELECT * FROM {} WHERE id = ?", self.table_name);
        
        let result = db.query_with_params(&sql, &[serde_json::json!(id)]).await?;
        
        if let Some(row) = result.first() {
            let json_value = serde_json::to_value(row.to_map())?;
            let entity: E = serde_json::from_value(json_value)?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    async fn find_one(&self, db: &dyn DMSCDatabase, criteria: &Criteria) -> DMSCResult<Option<E>> {
        let mut query = QueryBuilder::new(self.table_name);
        query.where_criteria(criteria.clone());
        
        let (sql, params) = query.build();
        let result = db.query_with_params(&sql, &params).await?;
        
        if let Some(row) = result.first() {
            let json_value = serde_json::to_value(row.to_map())?;
            let entity: E = serde_json::from_value(json_value)?;
            Ok(Some(entity))
        } else {
            Ok(None)
        }
    }

    async fn find_many(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<Vec<E>> {
        let mut query = QueryBuilder::new(self.table_name);
        
        for criteria in criteria {
            query.and_where(criteria);
        }
        
        let (sql, params) = query.build();
        let result = db.query_with_params(&sql, &params).await?;
        
        let mut entities = Vec::new();
        for row in result {
            let json_value = serde_json::to_value(row.to_map())?;
            let entity: E = serde_json::from_value(json_value)?;
            entities.push(entity);
        }
        
        Ok(entities)
    }

    async fn find_paginated(&self, db: &dyn DMSCDatabase, pagination: Pagination, criteria: Vec<Criteria>) -> DMSCResult<(Vec<E>, u64)> {
        let count_sql = format!("SELECT COUNT(*) as total FROM {}", self.table_name);
        
        let total: u64 = if criteria.is_empty() {
            if let Some(row) = db.query_one(&count_sql).await? {
                row.get::<i64>("total").map(|v| v as u64).unwrap_or(0)
            } else {
                0
            }
        } else {
            let mut count_query = QueryBuilder::new(self.table_name);
            for c in &criteria {
                count_query.and_where(c.clone());
            }
            let (sql, params) = count_query.build();
            let count_sql = format!("SELECT COUNT(*) as total FROM ({}) as subquery", sql);
            
            let result = db.query_with_params(&count_sql, &params).await?;
            let row = result.first().ok_or_else(|| DMSCError::Other("Query returned no rows".to_string()))?;
            row.get_i64("total").map(|v| v as u64).unwrap_or(0)
        };
        
        let mut data_query = QueryBuilder::new(self.table_name);
        for c in criteria {
            data_query.and_where(c);
        }
        data_query.paginate(pagination.page, pagination.page_size);
        
        let (sql, params) = data_query.build();
        let result = db.query_with_params(&sql, &params).await?;
        
        let mut entities = Vec::new();
        for row in result {
            let json_value = serde_json::to_value(row.to_map())?;
            let entity: E = serde_json::from_value(json_value)?;
            entities.push(entity);
        }
        
        Ok((entities, total))
    }

    async fn count(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<u64> {
        let mut query = QueryBuilder::new(self.table_name);
        for c in criteria {
            query.and_where(c);
        }
        let (sql, _) = query.build();
        let count_sql = sql.replace("*", "COUNT(*) as total");
        
        if let Some(row) = db.query_one(&count_sql).await? {
            row.get::<i64>("total").map(|v| v as u64).ok_or_else(|| DMSCError::Other("Failed to get count".to_string()))
        } else {
            Ok(0)
        }
    }

    async fn save(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<E> {
        let json_value = serde_json::to_value(entity)?;
        let values: HashMap<String, serde_json::Value> = serde_json::from_value(json_value)?;
        
        let columns: Vec<&str> = values.keys().map(|s| s.as_str()).collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name,
            columns.join(", "),
            placeholders.join(", ")
        );
        
        let params: Vec<serde_json::Value> = columns.iter()
            .map(|&col| values.get(col).cloned().unwrap_or(serde_json::Value::Null))
            .collect();
        
        db.execute_with_params(&sql, &params).await?;
        
        Ok(entity.clone())
    }

    async fn save_many(&self, db: &dyn DMSCDatabase, entities: &[E]) -> DMSCResult<Vec<E>> {
        let mut saved = Vec::with_capacity(entities.len());
        
        for entity in entities {
            saved.push(self.save(db, entity).await?);
        }
        
        Ok(saved)
    }

    async fn update(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<E> {
        let json_value = serde_json::to_value(entity)?;
        let values: HashMap<String, serde_json::Value> = serde_json::from_value(json_value)?;
        
        let updates: Vec<String> = values.keys()
            .filter(|&col| col != "id")
            .map(|col| format!("{} = ?", col))
            .collect();
        
        if updates.is_empty() {
            return Ok(entity.clone());
        }
        
        let sql = format!(
            "UPDATE {} SET {} WHERE id = ?",
            self.table_name,
            updates.join(", ")
        );
        
        let mut params: Vec<serde_json::Value> = values.iter()
            .filter(|(col, _)| *col != "id")
            .map(|(_, v)| v.clone())
            .collect();
        
        if let Some(id) = values.get("id") {
            params.push(id.clone());
        }
        
        db.execute_with_params(&sql, &params).await?;
        
        Ok(entity.clone())
    }

    async fn delete(&self, db: &dyn DMSCDatabase, entity: &E) -> DMSCResult<()> {
        let json_value = serde_json::to_value(entity)?;
        let values: HashMap<String, serde_json::Value> = serde_json::from_value(json_value)?;
        
        if let Some(id) = values.get("id") {
            self.delete_by_id(db, &id.to_string()).await
        } else {
            Err(DMSCError::Other("Entity has no id field".to_string()))
        }
    }

    async fn delete_by_id(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<()> {
        let sql = format!("DELETE FROM {} WHERE id = ?", self.table_name);
        
        db.execute_with_params(&sql, &[serde_json::json!(id)]).await?;
        Ok(())
    }

    async fn delete_many(&self, db: &dyn DMSCDatabase, criteria: Vec<Criteria>) -> DMSCResult<u64> {
        if criteria.is_empty() {
            return Err(DMSCError::Other("Criteria required for delete_many operation".to_string()));
        }
        
        let mut query = QueryBuilder::new(self.table_name);
        for c in criteria {
            query.and_where(c);
        }
        
        let (sql, params) = query.build();
        let delete_sql = format!("DELETE FROM {}", sql.split("FROM").nth(1).unwrap_or(&sql));
        
        db.execute_with_params(&delete_sql, &params).await.map_err(|e| e.into())
    }
    
    async fn batch_insert(&self, db: &dyn DMSCDatabase, entities: &[E], batch_size: usize) -> DMSCResult<Vec<E>> {
        let mut inserted = Vec::with_capacity(entities.len());
        
        for chunk in entities.chunks(batch_size) {
            let json_values: Vec<serde_json::Value> = chunk.iter()
                .map(|e| serde_json::to_value(e))
                .collect::<Result<_, _>>()?;
            
            let mut all_columns: std::collections::HashSet<&str> = std::collections::HashSet::new();
            for json_value in &json_values {
                if let serde_json::Value::Object(map) = json_value {
                    for key in map.keys() {
                        all_columns.insert(key);
                    }
                }
            }
            
            let columns: Vec<&str> = all_columns.iter().copied().collect();
            let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();
            
            let sql = format!(
                "INSERT INTO {} ({}) VALUES ({})",
                self.table_name,
                columns.join(", "),
                placeholders.join(", ")
            );
            
            for json_value in chunk {
                let json_val = serde_json::to_value(json_value)?;
                let values: HashMap<String, serde_json::Value> = serde_json::from_value(json_val)?;
                
                let params: Vec<serde_json::Value> = columns.iter()
                    .map(|&col| values.get(col).cloned().unwrap_or(serde_json::Value::Null))
                    .collect();
                
                db.execute_with_params(&sql, &params).await?;
                inserted.push(json_value.clone());
            }
        }
        
        Ok(inserted)
    }
    
    async fn upsert(&self, db: &dyn DMSCDatabase, entity: &E, conflict_columns: &[&str]) -> DMSCResult<E> {
        let json_value = serde_json::to_value(entity)?;
        let values: HashMap<String, serde_json::Value> = serde_json::from_value(json_value)?;
        
        let columns: Vec<&str> = values.keys().map(|s| s.as_str()).collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();
        
        let update_parts: Vec<String> = columns.iter()
            .filter(|&&col| !conflict_columns.contains(&col))
            .map(|col| format!("{} = EXCLUDED.{}", col, col))
            .collect();
        
        let conflict_cols = conflict_columns.join(", ");
        let update_set = if update_parts.is_empty() {
            String::new()
        } else {
            format!("ON CONFLICT ({}) DO UPDATE SET {}", conflict_cols, update_parts.join(", "))
        };
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({}) {}",
            self.table_name,
            columns.join(", "),
            placeholders.join(", "),
            update_set
        );
        
        let params: Vec<serde_json::Value> = columns.iter()
            .map(|&col| values.get(col).cloned().unwrap_or(serde_json::Value::Null))
            .collect();
        
        db.execute_with_params(&sql, &params).await?;
        
        Ok(entity.clone())
    }

    async fn exists(&self, db: &dyn DMSCDatabase, id: &str) -> DMSCResult<bool> {
        let sql = format!("SELECT 1 FROM {} WHERE id = ? LIMIT 1", self.table_name);
        
        let result = db.query_with_params(&sql, &[serde_json::json!(id)]).await?;
        Ok(!result.is_empty())
    }

    async fn exists_by(&self, db: &dyn DMSCDatabase, criteria: &Criteria) -> DMSCResult<bool> {
        let mut query = QueryBuilder::new(self.table_name);
        query.select(vec!["1"]);
        query.where_criteria(criteria.clone());
        
        let (sql, params) = query.build();
        let sql = format!("{} LIMIT 1", sql);
        
        let result = db.query_with_params(&sql, &params).await?;
        Ok(!result.is_empty())
    }
}

impl<E: for<'de> serde::Deserialize<'de> + serde::Serialize + Clone + Send + Sync> DMSCORMSimpleRepository<E> {
    pub fn default() -> Self {
        Self::new("unknown")
    }
}
