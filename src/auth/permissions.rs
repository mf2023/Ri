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

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSPermission {
    pub id: String,
    pub name: String,
    pub description: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSRole {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: HashSet<String>,
    pub is_system: bool,
}

impl DMSRole {
    pub fn _Fnew(id: String, name: String, description: String, permissions: HashSet<String>) -> Self {
        Self {
            id,
            name,
            description,
            permissions,
            is_system: false,
        }
    }

    pub fn _Fhas_permission(&self, permission_id: &str) -> bool {
        self.permissions.contains(permission_id)
    }

    pub fn _Fadd_permission(&mut self, permission_id: String) {
        self.permissions.insert(permission_id);
    }

    pub fn _Fremove_permission(&mut self, permission_id: &str) {
        self.permissions.remove(permission_id);
    }
}

pub struct DMSPermissionManager {
    permissions: RwLock<HashMap<String, DMSPermission>>,
    roles: RwLock<HashMap<String, DMSRole>>,
    user_roles: RwLock<HashMap<String, HashSet<String>>>, // user_id -> role_ids
}

impl DMSPermissionManager {
    pub fn _Fnew() -> Self {
        let mut manager = Self {
            permissions: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            user_roles: RwLock::new(HashMap::new()),
        };
        
        // Initialize with default system roles
        manager._Finitialize_default_roles();
        manager
    }

    fn _Finitialize_default_roles(&mut self) {
        // This would be called in blocking context, so we use blocking_write
        let mut roles = self.roles.blocking_write();
        
        // Admin role - all permissions
        let admin_permissions: HashSet<String> = vec![
            "*".to_string(), // Wildcard permission
        ].into_iter().collect();
        
        let admin_role = DMSRole {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            description: "Full system access".to_string(),
            permissions: admin_permissions,
            is_system: true,
        };
        
        roles.insert("admin".to_string(), admin_role);
        
        // User role - basic permissions
        let user_permissions: HashSet<String> = vec![
            "read:profile".to_string(),
            "update:profile".to_string(),
            "read:own_data".to_string(),
        ].into_iter().collect();
        
        let user_role = DMSRole {
            id: "user".to_string(),
            name: "User".to_string(),
            description: "Standard user access".to_string(),
            permissions: user_permissions,
            is_system: true,
        };
        
        roles.insert("user".to_string(), user_role);
    }

    pub async fn _Fcreate_permission(&self, permission: DMSPermission) -> crate::core::DMSResult<()> {
        let mut permissions = self.permissions.write().await;
        permissions.insert(permission.id.clone(), permission);
        Ok(())
    }

    pub async fn _Fget_permission(&self, permission_id: &str) -> crate::core::DMSResult<Option<DMSPermission>> {
        let permissions = self.permissions.read().await;
        Ok(permissions.get(permission_id).cloned())
    }

    pub async fn _Fcreate_role(&self, role: DMSRole) -> crate::core::DMSResult<()> {
        let mut roles = self.roles.write().await;
        roles.insert(role.id.clone(), role);
        Ok(())
    }

    pub async fn _Fget_role(&self, role_id: &str) -> crate::core::DMSResult<Option<DMSRole>> {
        let roles = self.roles.read().await;
        Ok(roles.get(role_id).cloned())
    }

    pub async fn _Fassign_role_to_user(&self, user_id: String, role_id: String) -> crate::core::DMSResult<bool> {
        // Check if role exists
        let roles = self.roles.read().await;
        if !roles.contains_key(&role_id) {
            return Ok(false);
        }
        drop(roles);

        let mut user_roles = self.user_roles.write().await;
        let user_role_set = user_roles.entry(user_id).or_insert_with(HashSet::new);
        let was_added = user_role_set.insert(role_id);
        Ok(was_added)
    }

    pub async fn _Fremove_role_from_user(&self, user_id: &str, role_id: &str) -> crate::core::DMSResult<bool> {
        let mut user_roles = self.user_roles.write().await;
        
        if let Some(user_role_set) = user_roles.get_mut(user_id) {
            let was_removed = user_role_set.remove(role_id);
            if user_role_set.is_empty() {
                user_roles.remove(user_id);
            }
            Ok(was_removed)
        } else {
            Ok(false)
        }
    }

    pub async fn _Fget_user_roles(&self, user_id: &str) -> crate::core::DMSResult<Vec<DMSRole>> {
        let user_roles = self.user_roles.read().await;
        let roles = self.roles.read().await;
        
        let mut result = Vec::new();
        
        if let Some(user_role_ids) = user_roles.get(user_id) {
            for role_id in user_role_ids {
                if let Some(role) = roles.get(role_id) {
                    result.push(role.clone());
                }
            }
        }
        
        Ok(result)
    }

    pub async fn _Fhas_permission(&self, user_id: &str, permission_id: &str) -> crate::core::DMSResult<bool> {
        let user_roles = self.user_roles.read().await;
        let roles = self.roles.read().await;
        
        if let Some(user_role_ids) = user_roles.get(user_id) {
            for role_id in user_role_ids {
                if let Some(role) = roles.get(role_id) {
                    // Check for wildcard permission
                    if role.permissions.contains("*") {
                        return Ok(true);
                    }
                    
                    if role.permissions.contains(permission_id) {
                        return Ok(true);
                    }
                }
            }
        }
        
        Ok(false)
    }

    pub async fn _Fhas_any_permission(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSResult<bool> {
        for permission in permissions {
            if self._Fhas_permission(user_id, permission).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn _Fhas_all_permissions(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSResult<bool> {
        for permission in permissions {
            if !self._Fhas_permission(user_id, permission).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn _Fget_user_permissions(&self, user_id: &str) -> crate::core::DMSResult<HashSet<String>> {
        let user_roles = self.user_roles.read().await;
        let roles = self.roles.read().await;
        
        let mut permissions = HashSet::new();
        
        if let Some(user_role_ids) = user_roles.get(user_id) {
            for role_id in user_role_ids {
                if let Some(role) = roles.get(role_id) {
                    permissions.extend(role.permissions.clone());
                }
            }
        }
        
        Ok(permissions)
    }

    pub async fn _Fdelete_permission(&self, permission_id: &str) -> crate::core::DMSResult<bool> {
        let mut permissions = self.permissions.write().await;
        Ok(permissions.remove(permission_id).is_some())
    }

    pub async fn _Fdelete_role(&self, role_id: &str) -> crate::core::DMSResult<bool> {
        // Don't delete system roles
        let roles = self.roles.read().await;
        if let Some(role) = roles.get(role_id) {
            if role.is_system {
                return Ok(false);
            }
        }
        drop(roles);

        let mut roles = self.roles.write().await;
        let was_deleted = roles.remove(role_id).is_some();
        
        if was_deleted {
            // Remove role from all users
            let mut user_roles = self.user_roles.write().await;
            for user_role_set in user_roles.values_mut() {
                user_role_set.remove(role_id);
            }
        }
        
        Ok(was_deleted)
    }

    pub async fn _Flist_permissions(&self) -> crate::core::DMSResult<Vec<DMSPermission>> {
        let permissions = self.permissions.read().await;
        Ok(permissions.values().cloned().collect())
    }

    pub async fn _Flist_roles(&self) -> crate::core::DMSResult<Vec<DMSRole>> {
        let roles = self.roles.read().await;
        Ok(roles.values().cloned().collect())
    }
}