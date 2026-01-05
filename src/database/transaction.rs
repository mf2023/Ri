//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use crate::core::DMSCResult;
use crate::database::{DMSCDatabase, DMSCDBResult};
use async_trait::async_trait;
use serde::Serialize;

pub enum TransactionLevel {
    ReadUncommitted,
    ReadCommitted,
    RepeatableRead,
    Snapshot,
    Serializable,
}

pub enum DMSCDBTransactionStatus {
    Active,
    Committed,
    RolledBack,
    Failed,
}

#[async_trait]
pub trait DMSCDBTransaction: Send + Sync {
    fn is_active(&self) -> bool;

    async fn commit(&mut self) -> DMSCResult<()>;

    async fn rollback(&mut self) -> DMSCResult<()>;

    async fn savepoint(&mut self, name: &str) -> DMSCResult<()>;

    async fn rollback_to_savepoint(&mut self, name: &str) -> DMSCResult<()>;

    async fn execute(&mut self, sql: &str) -> DMSCResult<u64>;

    async fn execute_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<u64>;

    async fn query(&mut self, sql: &str) -> DMSCResult<DMSCDBResult>;

    async fn query_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<DMSCDBResult>;

    async fn query_one(&mut self, sql: &str) -> DMSCResult<Option<crate::database::DMSCDBRow>>;

    async fn query_one_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<Option<crate::database::DMSCDBRow>>;
}

pub struct DatabaseTransaction<T: DMSCDatabase> {
    db: Arc<T>,
    active: bool,
    level: TransactionLevel,
}

impl<T: DMSCDatabase> DatabaseTransaction<T> {
    pub fn new(db: Arc<T>, level: TransactionLevel) -> Self {
        Self {
            db,
            active: true,
            level,
        }
    }
}

#[async_trait]
impl<T: DMSCDatabase + Clone + Send + Sync> DMSCDBTransaction for DatabaseTransaction<T> {
    fn is_active(&self) -> bool {
        self.active
    }

    async fn commit(&mut self) -> DMSCResult<()> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.execute("COMMIT").await?;
        self.active = false;
        Ok(())
    }

    async fn rollback(&mut self) -> DMSCResult<()> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.execute("ROLLBACK").await?;
        self.active = false;
        Ok(())
    }

    async fn savepoint(&mut self, name: &str) -> DMSCResult<()> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        let sql = format!("SAVEPOINT {}", name);
        self.db.execute(&sql).await?;
        Ok(())
    }

    async fn rollback_to_savepoint(&mut self, name: &str) -> DMSCResult<()> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        let sql = format!("ROLLBACK TO SAVEPOINT {}", name);
        self.db.execute(&sql).await?;
        Ok(())
    }

    async fn execute(&mut self, sql: &str) -> DMSCResult<u64> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.execute(sql).await
    }

    async fn execute_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<u64> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.execute_with_params(sql, params).await
    }

    async fn query(&mut self, sql: &str) -> DMSCResult<DMSCDBResult> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.query(sql).await
    }

    async fn query_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<DMSCDBResult> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.query_with_params(sql, params).await
    }

    async fn query_one(&mut self, sql: &str) -> DMSCResult<Option<crate::database::DMSCDBRow>> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.query_one(sql).await
    }

    async fn query_one_with_params(&mut self, sql: &str, params: &[&dyn Serialize]) -> DMSCResult<Option<crate::database::DMSCDBRow>> {
        if !self.active {
            return Err(crate::core::DMSCError::InvalidState(
                "Transaction is not active".to_string(),
            ));
        }
        self.db.query_one_with_params(sql, params).await
    }
}

impl<T: DMSCDatabase> Drop for DatabaseTransaction<T> {
    fn drop(&mut self) {
        if self.active {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(async {
                    let _ = self.rollback();
                });
            }));
        }
    }
}
