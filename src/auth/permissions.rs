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
use tokio::sync::RwLock;

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
    /// Hash map of permissions indexed by permission ID
    permissions: RwLock<HashMap<String, DMSCPermission>>,
    /// Hash map of roles indexed by role ID
    roles: RwLock<HashMap<String, DMSCRole>>,
    /// Hash map of user role assignments indexed by user ID
    user_roles: RwLock<HashMap<String, HashSet<String>>>,
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
            permissions: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            user_roles: RwLock::new(HashMap::new()),
        };
        
        // Initialize with default system roles
        // Note: This uses blocking_write which may block the async runtime
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
            permissions: RwLock::new(HashMap::new()),
            roles: RwLock::new(HashMap::new()),
            user_roles: RwLock::new(HashMap::new()),
        };
        
        // Initialize with default system roles asynchronously
        manager.initialize_default_roles_async().await;
        manager
    }

    /// Initializes default system roles.
    /// 
    /// This method is called during construction to create the default admin
    /// and user roles. It uses `blocking_write` because it's called from a
    /// non-async context.
    /// 
    /// **Performance Note**: This method is called during initialization and uses
    /// `blocking_write` to avoid async complexity in the constructor. In production
    /// scenarios, consider using an async factory pattern or lazy initialization.
    fn initialize_default_roles(&mut self) {
        // This would be called in blocking context, so we use blocking_write
        // Note: In production, consider using an async factory pattern to avoid blocking
        let mut roles = self.roles.blocking_write();
        
        // Admin role - all permissions
        let admin_permissions: HashSet<String> = vec![
            "*".to_string(), // Wildcard permission
        ].into_iter().collect();
        
        let admin_role = DMSCRole {
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
        
        let user_role = DMSCRole {
            id: "user".to_string(),
            name: "User".to_string(),
            description: "Standard user access".to_string(),
            permissions: user_permissions,
            is_system: true,
        };
        
        roles.insert("user".to_string(), user_role);
    }

    /// Initializes default system roles asynchronously.
    /// 
    /// This method is the async version of `initialize_default_roles` that avoids
    /// using `blocking_write`. It should be used when creating the permission manager
    /// in async contexts.
    /// 
    /// **Performance**: This method uses async write locks and is preferred for
    /// async initialization scenarios.
    async fn initialize_default_roles_async(&self) {
        let mut roles = self.roles.write().await;
        
        // Admin role - all permissions
        let admin_permissions: HashSet<String> = vec![
            "*".to_string(), // Wildcard permission
        ].into_iter().collect();
        
        let admin_role = DMSCRole {
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
        
        let user_role = DMSCRole {
            id: "user".to_string(),
            name: "User".to_string(),
            description: "Standard user access".to_string(),
            permissions: user_permissions,
            is_system: true,
        };
        
        roles.insert("user".to_string(), user_role);
    }

    /// Creates a new permission.
    /// 
    /// # Parameters
    /// - `permission`: Permission to create
    /// 
    /// # Returns
    /// `Ok(())` if the permission was successfully created
    pub async fn create_permission(&self, permission: DMSCPermission) -> crate::core::DMSCResult<()> {
        let mut permissions = self.permissions.write().await;
        permissions.insert(permission.id.clone(), permission);
        Ok(())
    }

    /// Gets a permission by ID.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to retrieve
    /// 
    /// # Returns
    /// `Some(DMSCPermission)` if the permission exists, otherwise `None`
    pub async fn get_permission(&self, permission_id: &str) -> crate::core::DMSCResult<Option<DMSCPermission>> {
        let permissions = self.permissions.read().await;
        Ok(permissions.get(permission_id).cloned())
    }

    /// Creates a new role.
    /// 
    /// # Parameters
    /// - `role`: Role to create
    /// 
    /// # Returns
    /// `Ok(())` if the role was successfully created
    pub async fn create_role(&self, role: DMSCRole) -> crate::core::DMSCResult<()> {
        let mut roles = self.roles.write().await;
        roles.insert(role.id.clone(), role);
        Ok(())
    }

    /// Gets a role by ID.
    /// 
    /// # Parameters
    /// - `role_id`: Role ID to retrieve
    /// 
    /// # Returns
    /// `Some(DMSCRole)` if the role exists, otherwise `None`
    pub async fn get_role(&self, role_id: &str) -> crate::core::DMSCResult<Option<DMSCRole>> {
        let roles = self.roles.read().await;
        Ok(roles.get(role_id).cloned())
    }

    /// Assigns a role to a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to assign the role to
    /// - `role_id`: Role ID to assign
    /// 
    /// # Returns
    /// `true` if the role was successfully assigned, `false` if the role doesn't exist
    pub async fn assign_role_to_user(&self, user_id: String, role_id: String) -> crate::core::DMSCResult<bool> {
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

    /// Removes a role from a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to remove the role from
    /// - `role_id`: Role ID to remove
    /// 
    /// # Returns
    /// `true` if the role was successfully removed, `false` if the user didn't have the role
    pub async fn remove_role_from_user(&self, user_id: &str, role_id: &str) -> crate::core::DMSCResult<bool> {
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

    /// Gets all roles assigned to a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to retrieve roles for
    /// 
    /// # Returns
    /// A vector of `DMSCRole` objects assigned to the user
    pub async fn get_user_roles(&self, user_id: &str) -> crate::core::DMSCResult<Vec<DMSCRole>> {
        let user_roles = self.user_roles.read().await;
        let roles = self.roles.read().await;
        
        let mut result = if let Some(user_role_ids) = user_roles.get(user_id) {
            Vec::with_capacity(user_role_ids.len())
        } else {
            Vec::new()
        };
        
        if let Some(user_role_ids) = user_roles.get(user_id) {
            for role_id in user_role_ids {
                if let Some(role) = roles.get(role_id) {
                    result.push(role.clone());
                }
            }
        }
        
        Ok(result)
    }

    /// Checks if a user has a specific permission.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to check
    /// - `permission_id`: Permission ID to check for
    /// 
    /// # Returns
    /// `true` if the user has the permission, otherwise `false`
    /// 
    /// # Notes
    /// - Users with the wildcard permission ("*") have all permissions
    /// - Permission checking is done by examining all roles assigned to the user
    pub async fn has_permission(&self, user_id: &str, permission_id: &str) -> crate::core::DMSCResult<bool> {
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

    /// Checks if a user has any of the specified permissions.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to check
    /// - `permissions`: List of permission IDs to check for
    /// 
    /// # Returns
    /// `true` if the user has at least one of the permissions, otherwise `false`
    pub async fn has_any_permission(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSCResult<bool> {
        for permission in permissions {
            if self.has_permission(user_id, permission).await? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Checks if a user has all of the specified permissions.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to check
    /// - `permissions`: List of permission IDs to check for
    /// 
    /// # Returns
    /// `true` if the user has all of the permissions, otherwise `false`
    pub async fn has_all_permissions(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSCResult<bool> {
        for permission in permissions {
            if !self.has_permission(user_id, permission).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Gets all permissions assigned to a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to retrieve permissions for
    /// 
    /// # Returns
    /// A set of permission IDs assigned to the user
    pub async fn get_user_permissions(&self, user_id: &str) -> crate::core::DMSCResult<HashSet<String>> {
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

    /// Deletes a permission.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to delete
    /// 
    /// # Returns
    /// `true` if the permission was successfully deleted, otherwise `false`
    pub async fn delete_permission(&self, permission_id: &str) -> crate::core::DMSCResult<bool> {
        let mut permissions = self.permissions.write().await;
        Ok(permissions.remove(permission_id).is_some())
    }

    /// Deletes a role.
    /// 
    /// # Parameters
    /// - `role_id`: Role ID to delete
    /// 
    /// # Returns
    /// `true` if the role was successfully deleted, otherwise `false`
    /// 
    /// # Notes
    /// - System roles cannot be deleted
    /// - If the role is deleted, it is removed from all users
    pub async fn delete_role(&self, role_id: &str) -> crate::core::DMSCResult<bool> {
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

    /// Lists all permissions.
    /// 
    /// # Returns
    /// A vector of all registered permissions
    pub async fn list_permissions(&self) -> crate::core::DMSCResult<Vec<DMSCPermission>> {
        let permissions = self.permissions.read().await;
        Ok(permissions.values().cloned().collect())
    }

    /// Lists all roles.
    /// 
    /// # Returns
    /// A vector of all registered roles
    pub async fn list_roles(&self) -> crate::core::DMSCResult<Vec<DMSCRole>> {
        let roles = self.roles.read().await;
        Ok(roles.values().cloned().collect())
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
