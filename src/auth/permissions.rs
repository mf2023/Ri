//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! Role-based access control (RBAC) implementation for DMSC.
//! 
//! This module provides a comprehensive RBAC system for managing permissions,
//! roles, and user role assignments. It supports:
//! - Fine-grained permission definitions
//! - Role management with inheritance support
//! - User role assignments
//! - Permission checking for users
//! - System roles that cannot be deleted
//! - Wildcard permissions for administrative access
//! 
//! # Design Principles
//! - **Separation of Concerns**: Permissions, roles, and user assignments are managed separately
//! - **Thread Safety**: Uses RwLock for concurrent access to data structures
//! - **Flexibility**: Supports both explicit and wildcard permissions
//! - **Security**: System roles are protected from deletion
//! - **Performance**: Efficient permission checking with hash sets
//! 
//! # Usage Examples
//! ```rust
//! // Create a permission manager
//! let permission_manager = DMSCPermissionManager::new();
//! 
//! // Create a permission
//! let read_device_perm = DMSCPermission {
//!     id: "read:device".to_string(),
//!     name: "Read Device".to_string(),
//!     description: "Allows reading device information".to_string(),
//!     resource: "device".to_string(),
//!     action: "read".to_string(),
//! };
//! permission_manager.create_permission(read_device_perm).await?;
//! 
//! // Create a role
//! let device_admin_role = DMSCRole::new(
//!     "device_admin".to_string(),
//!     "Device Administrator".to_string(),
//!     "Manages devices".to_string(),
//!     vec!["read:device", "write:device"].iter().map(|s| s.to_string()).collect()
//! );
//! permission_manager.create_role(device_admin_role).await?;
//! 
//! // Assign role to user
//! permission_manager.assign_role_to_user("user123".to_string(), "device_admin".to_string()).await?;
//! 
//! // Check if user has permission
//! let has_perm = permission_manager.has_permission("user123", "read:device").await?;
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::core::concurrent::DMSCShardedLock;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Permission definition for fine-grained access control.
///
/// This struct defines a permission with a unique ID, name, description,
/// resource, and action. Permissions follow the "resource:action" convention.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCPermission {
    /// Unique permission identifier following "resource:action" format (e.g., "read:device")
    pub id: String,
    /// Human-readable name for the permission
    pub name: String,
    /// Detailed description explaining what this permission allows
    pub description: String,
    /// Resource being accessed (e.g., "device", "user", "data")
    pub resource: String,
    /// Action being performed (e.g., "read", "write", "delete")
    pub action: String,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCPermission {
    #[new]
    fn py_new(
        id: Option<String>,
        name: String,
        description: String,
        resource: String,
        action: String,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| format!("{}:{}", resource, action)),
            name,
            description,
            resource,
            action,
        }
    }
}

/// Role definition for grouping permissions.
///
/// Roles are collections of permissions that can be assigned to users.
/// System roles cannot be deleted and are created automatically during initialization.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCRole {
    /// Unique role identifier
    pub id: String,
    /// Human-readable name for the role
    pub name: String,
    /// Detailed description explaining the role's purpose and access level
    pub description: String,
    /// Set of permission IDs assigned to this role
    pub permissions: HashSet<String>,
    /// Whether this is a system role that cannot be deleted
    pub is_system: bool,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCRole {
    #[new]
    fn py_new(
        id: Option<String>,
        name: String,
        description: String,
        permissions: Vec<String>,
        is_system: bool,
    ) -> Self {
        Self {
            id: id.unwrap_or_else(|| name.to_lowercase().replace(' ', "_")),
            name,
            description,
            permissions: permissions.into_iter().collect(),
            is_system,
        }
    }
}

impl DMSCRole {
    /// Creates a new role with the specified permissions.
    /// 
    /// # Parameters
    /// - `id`: Unique role identifier
    /// - `name`: Human-readable name
    /// - `description`: Detailed description
    /// - `permissions`: Set of permission IDs
    /// 
    /// # Returns
    /// A new instance of `DMSCRole`
    pub fn new(id: String, name: String, description: String, permissions: HashSet<String>) -> Self {
        Self {
            id,
            name,
            description,
            permissions,
            is_system: false,
        }
    }

    /// Checks if the role has the specified permission.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to check
    /// 
    /// # Returns
    /// `true` if the role has the permission, otherwise `false`
    #[inline]
    pub fn has_permission(&self, permission_id: &str) -> bool {
        self.permissions.contains(permission_id)
    }

    /// Adds a permission to the role.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to add
    #[inline]
    pub fn add_permission(&mut self, permission_id: String) {
        self.permissions.insert(permission_id);
    }

    /// Removes a permission from the role.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to remove
    #[inline]
    pub fn remove_permission(&mut self, permission_id: &str) {
        self.permissions.remove(permission_id);
    }
}

/// Permission manager for handling permissions, roles, and user assignments.
///
/// This struct manages the entire RBAC system, including:
/// - Permission CRUD operations
/// - Role CRUD operations
/// - User role assignments
/// - Permission checking for users
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCPermissionManager {
    permissions: DMSCShardedLock<String, DMSCPermission>,
    roles: DMSCShardedLock<String, DMSCRole>,
    user_roles: DMSCShardedLock<String, HashSet<String>>,
}

impl Default for DMSCPermissionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCPermissionManager {
    /// Creates a new permission manager with default system roles.
    /// 
    /// Initializes the manager with two default system roles:
    /// - `admin`: Has wildcard permission ("*") for all access
    /// - `user`: Has basic user permissions
    /// 
    /// **Performance Note**: This method uses `blocking_write` during initialization
    /// to set up default roles. For production use, consider using `new_async()` or
    /// lazy initialization patterns to avoid blocking the async runtime.
    /// 
    /// # Returns
    /// A new instance of `DMSCPermissionManager`
    pub fn new() -> Self {
        let mut manager = Self {
            permissions: DMSCShardedLock::with_default_shards(),
            roles: DMSCShardedLock::with_default_shards(),
            user_roles: DMSCShardedLock::with_default_shards(),
        };
        
        manager.initialize_default_roles();
        manager
    }

    /// Creates a new permission manager asynchronously with default system roles.
    /// 
    /// This is the preferred method for creating a permission manager in async contexts
    /// as it avoids blocking the runtime during initialization.
    /// 
    /// Initializes the manager with two default system roles:
    /// - `admin`: Has wildcard permission ("*") for all access
    /// - `user`: Has basic user permissions
    /// 
    /// # Returns
    /// A new instance of `DMSCPermissionManager`
    pub async fn new_async() -> Self {
        let manager = Self {
            permissions: DMSCShardedLock::with_default_shards(),
            roles: DMSCShardedLock::with_default_shards(),
            user_roles: DMSCShardedLock::with_default_shards(),
        };
        
        manager.initialize_default_roles_async().await;
        manager
    }

    fn initialize_default_roles(&mut self) {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
        rt.block_on(async {
            self.initialize_default_roles_async().await
        });
    }

    async fn initialize_default_roles_async(&self) {
        let admin_permissions: HashSet<String> = vec![
            "*".to_string(),
        ].into_iter().collect();
        
        let admin_role = DMSCRole {
            id: "admin".to_string(),
            name: "Administrator".to_string(),
            description: "Full system access".to_string(),
            permissions: admin_permissions,
            is_system: true,
        };
        
        self.roles.insert("admin".to_string(), admin_role).await;
        
        let user_permissions: HashSet<String> = vec![
            "read:profile".to_string(),
            "update:profile".to_string(),
            "read:own_data".to_string(),
        ].into_iter().collect();
        
        let user_role = DMSCRole {
            id: "user".to_string(),
            name: "User".to_string(),
            description: "Standard user access".to_string(),
            permissions: user_permissions,
            is_system: true,
        };
        
        self.roles.insert("user".to_string(), user_role).await;
    }

    pub async fn create_permission(&self, permission: DMSCPermission) -> crate::core::DMSCResult<()> {
        self.permissions.insert(permission.id.clone(), permission).await;
        Ok(())
    }

    pub async fn get_permission(&self, permission_id: &str) -> crate::core::DMSCResult<Option<DMSCPermission>> {
        Ok(self.permissions.get(permission_id).await)
    }

    pub async fn create_role(&self, role: DMSCRole) -> crate::core::DMSCResult<()> {
        self.roles.insert(role.id.clone(), role).await;
        Ok(())
    }

    pub async fn get_role(&self, role_id: &str) -> crate::core::DMSCResult<Option<DMSCRole>> {
        Ok(self.roles.get(role_id).await)
    }

    pub async fn assign_role_to_user(&self, user_id: String, role_id: String) -> crate::core::DMSCResult<bool> {
        let role_exists = self.roles.contains_key(&role_id).await;
        if !role_exists {
            return Ok(false);
        }

        let user_role_set = self.user_roles.get(&user_id).await.unwrap_or_default();
        let mut new_set = user_role_set.clone();
        let was_added = new_set.insert(role_id);
        self.user_roles.insert(user_id, new_set).await;
        Ok(was_added)
    }

    pub async fn remove_role_from_user(&self, user_id: &str, role_id: &str) -> crate::core::DMSCResult<bool> {
        let user_role_set = self.user_roles.get(user_id).await;
        
        match user_role_set {
            Some(mut set) => {
                let was_removed = set.remove(role_id);
                if set.is_empty() {
                    self.user_roles.remove(user_id).await;
                } else {
                    self.user_roles.insert(user_id.to_string(), set).await;
                }
                Ok(was_removed)
            }
            None => Ok(false),
        }
    }

    pub async fn get_user_roles(&self, user_id: &str) -> crate::core::DMSCResult<Vec<DMSCRole>> {
        let user_role_set = self.user_roles.get(user_id).await;
        
        match user_role_set {
            Some(role_ids) => {
                let mut result = Vec::with_capacity(role_ids.len());
                for role_id in role_ids {
                    if let Some(role) = self.roles.get(&role_id).await {
                        result.push(role);
                    }
                }
                Ok(result)
            }
            None => Ok(Vec::new()),
        }
    }

    pub async fn has_permission(&self, user_id: &str, permission_id: &str) -> crate::core::DMSCResult<bool> {
        let user_role_set = self.user_roles.get(user_id).await;
        
        if let Some(role_ids) = user_role_set {
            for role_id in role_ids {
                if let Some(role) = self.roles.get(&role_id).await {
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

    pub async fn has_any_permission(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSCResult<bool> {
        for permission in permissions {
            if self.has_permission(user_id, permission).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub async fn has_all_permissions(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSCResult<bool> {
        for permission in permissions {
            if !self.has_permission(user_id, permission).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn get_user_permissions(&self, user_id: &str) -> crate::core::DMSCResult<HashSet<String>> {
        let user_role_set = self.user_roles.get(user_id).await;
        
        let mut permissions = HashSet::new();
        
        if let Some(role_ids) = user_role_set {
            for role_id in role_ids {
                if let Some(role) = self.roles.get(&role_id).await {
                    permissions.extend(role.permissions);
                }
            }
        }
        
        Ok(permissions)
    }

    pub async fn delete_permission(&self, permission_id: &str) -> crate::core::DMSCResult<bool> {
        Ok(self.permissions.remove(permission_id).await.is_some())
    }

    pub async fn delete_role(&self, role_id: &str) -> crate::core::DMSCResult<bool> {
        let role = self.roles.get(role_id).await;
        if let Some(r) = role {
            if r.is_system {
                return Ok(false);
            }
        }

        let was_deleted = self.roles.remove(role_id).await.is_some();
        
        if was_deleted {
            self.user_roles.for_each_mut(|_, role_set| {
                role_set.remove(role_id);
            }).await;
        }
        
        Ok(was_deleted)
    }

    pub async fn list_permissions(&self) -> crate::core::DMSCResult<Vec<DMSCPermission>> {
        Ok(self.permissions.collect_all().await.into_values().collect())
    }

    pub async fn list_roles(&self) -> crate::core::DMSCResult<Vec<DMSCRole>> {
        Ok(self.roles.collect_all().await.into_values().collect())
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for the Permission Manager.
///
/// This module provides Python interface to DMSC RBAC functionality,
/// enabling Python applications to manage permissions, roles, and user assignments.
///
/// ## Supported Operations
///
/// - Permission creation and management
/// - Role creation and management with permission assignments
/// - User role assignments and removal
/// - Permission checking for users
/// - Permission and role listing
///
/// ## Python Usage Example
///
/// ```python
/// from dmsc import DMSCPermission, DMSCRole, DMSCPermissionManager
///
/// # Create permission manager
/// perm_manager = DMSCPermissionManager()
///
/// # Create a permission
/// permission = DMSCPermission(
///     id="read:device",
///     name="Read Device",
///     description="Allows reading device information",
///     resource="device",
///     action="read",
/// )
///
/// # Create a role
/// role = DMSCRole(
///     id="device_admin",
///     name="Device Administrator",
///     description="Manages devices",
///     permissions=["read:device", "write:device"],
///     is_system=False,
/// )
/// # Note: Async operations require Python 3.7+ with asyncio
/// ```
#[pyo3::prelude::pymethods]
impl DMSCPermissionManager {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(Self::new())
    }
    
    #[pyo3(name = "create_permission")]
    fn create_permission_impl(&self, permission: DMSCPermission) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.create_permission(permission).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "create_role")]
    fn create_role_impl(&self, role: DMSCRole) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.create_role(role).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "assign_role_to_user")]
    fn assign_role_to_user_impl(&self, user_id: String, role_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.assign_role_to_user(user_id, role_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "has_permission")]
    fn has_permission_impl(&self, user_id: String, permission_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.has_permission(&user_id, &permission_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_user_roles")]
    fn get_user_roles_impl(&self, user_id: String) -> PyResult<Vec<DMSCRole>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_user_roles(&user_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_user_permissions")]
    fn get_user_permissions_impl(&self, user_id: String) -> PyResult<Vec<String>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_user_permissions(&user_id).await
                .map(|perms| perms.into_iter().collect())
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "remove_role_from_user")]
    fn remove_role_from_user_impl(&self, user_id: String, role_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.remove_role_from_user(&user_id, &role_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "list_roles")]
    fn list_roles_impl(&self) -> PyResult<Vec<DMSCRole>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.list_roles().await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "list_permissions")]
    fn list_permissions_impl(&self) -> PyResult<Vec<DMSCPermission>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.list_permissions().await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "delete_role")]
    fn delete_role_impl(&self, role_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.delete_role(&role_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "delete_permission")]
    fn delete_permission_impl(&self, permission_id: String) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.delete_permission(&permission_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_role")]
    fn get_role_impl(&self, role_id: String) -> PyResult<Option<DMSCRole>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_role(&role_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "get_permission")]
    fn get_permission_impl(&self, permission_id: String) -> PyResult<Option<DMSCPermission>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.get_permission(&permission_id).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "has_any_permission")]
    fn has_any_permission_impl(&self, user_id: String, permissions: Vec<String>) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.has_any_permission(&user_id, &permissions).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
    
    #[pyo3(name = "has_all_permissions")]
    fn has_all_permissions_impl(&self, user_id: String, permissions: Vec<String>) -> PyResult<bool> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        rt.block_on(async {
            self.has_all_permissions(&user_id, &permissions).await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        })
    }
}
