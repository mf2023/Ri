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

//! Role-based access control (RBAC) implementation for DMS.
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
//! let permission_manager = DMSPermissionManager::_Fnew();
//! 
//! // Create a permission
//! let read_device_perm = DMSPermission {
//!     id: "read:device".to_string(),
//!     name: "Read Device".to_string(),
//!     description: "Allows reading device information".to_string(),
//!     resource: "device".to_string(),
//!     action: "read".to_string(),
//! };
//! permission_manager._Fcreate_permission(read_device_perm).await?;
//! 
//! // Create a role
//! let device_admin_role = DMSRole::_Fnew(
//!     "device_admin".to_string(),
//!     "Device Administrator".to_string(),
//!     "Manages devices".to_string(),
//!     vec!["read:device", "write:device"].iter().map(|s| s.to_string()).collect()
//! );
//! permission_manager._Fcreate_role(device_admin_role).await?;
//! 
//! // Assign role to user
//! permission_manager._Fassign_role_to_user("user123".to_string(), "device_admin".to_string()).await?;
//! 
//! // Check if user has permission
//! let has_perm = permission_manager._Fhas_permission("user123", "read:device").await?;
//! ```

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

/// Permission definition for fine-grained access control.
/// 
/// This struct defines a permission with a unique ID, name, description,
/// resource, and action. Permissions follow the "resource:action" convention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSPermission {
    pub id: String,          // Unique permission identifier (e.g., "read:device")
    pub name: String,        // Human-readable name
    pub description: String, // Detailed description of what the permission allows
    pub resource: String,    // Resource being accessed (e.g., "device")
    pub action: String,      // Action being performed (e.g., "read", "write")
}

/// Role definition for grouping permissions.
/// 
/// Roles are collections of permissions that can be assigned to users.
/// System roles cannot be deleted and are created automatically.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSRole {
    pub id: String,                // Unique role identifier
    pub name: String,              // Human-readable name
    pub description: String,       // Detailed description of the role's purpose
    pub permissions: HashSet<String>, // Set of permission IDs assigned to this role
    pub is_system: bool,           // Whether this is a system role that cannot be deleted
}

impl DMSRole {
    /// Creates a new role with the specified permissions.
    /// 
    /// # Parameters
    /// - `id`: Unique role identifier
    /// - `name`: Human-readable name
    /// - `description`: Detailed description
    /// - `permissions`: Set of permission IDs
    /// 
    /// # Returns
    /// A new instance of `DMSRole`
    pub fn _Fnew(id: String, name: String, description: String, permissions: HashSet<String>) -> Self {
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
    pub fn _Fhas_permission(&self, permission_id: &str) -> bool {
        self.permissions.contains(permission_id)
    }

    /// Adds a permission to the role.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to add
    pub fn _Fadd_permission(&mut self, permission_id: String) {
        self.permissions.insert(permission_id);
    }

    /// Removes a permission from the role.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to remove
    pub fn _Fremove_permission(&mut self, permission_id: &str) {
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
pub struct DMSPermissionManager {
    permissions: RwLock<HashMap<String, DMSPermission>>, // Permission ID -> Permission
    roles: RwLock<HashMap<String, DMSRole>>,           // Role ID -> Role
    user_roles: RwLock<HashMap<String, HashSet<String>>>, // User ID -> Role IDs
}

impl DMSPermissionManager {
    /// Creates a new permission manager with default system roles.
    /// 
    /// Initializes the manager with two default system roles:
    /// - `admin`: Has wildcard permission ("*") for all access
    /// - `user`: Has basic user permissions
    /// 
    /// # Returns
    /// A new instance of `DMSPermissionManager`
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

    /// Initializes default system roles.
    /// 
    /// This method is called during construction to create the default admin
    /// and user roles. It uses `blocking_write` because it's called from a
    /// non-async context.
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

    /// Creates a new permission.
    /// 
    /// # Parameters
    /// - `permission`: Permission to create
    /// 
    /// # Returns
    /// `Ok(())` if the permission was successfully created
    pub async fn _Fcreate_permission(&self, permission: DMSPermission) -> crate::core::DMSResult<()> {
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
    /// `Some(DMSPermission)` if the permission exists, otherwise `None`
    pub async fn _Fget_permission(&self, permission_id: &str) -> crate::core::DMSResult<Option<DMSPermission>> {
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
    pub async fn _Fcreate_role(&self, role: DMSRole) -> crate::core::DMSResult<()> {
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
    /// `Some(DMSRole)` if the role exists, otherwise `None`
    pub async fn _Fget_role(&self, role_id: &str) -> crate::core::DMSResult<Option<DMSRole>> {
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

    /// Removes a role from a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to remove the role from
    /// - `role_id`: Role ID to remove
    /// 
    /// # Returns
    /// `true` if the role was successfully removed, `false` if the user didn't have the role
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

    /// Gets all roles assigned to a user.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to retrieve roles for
    /// 
    /// # Returns
    /// A vector of `DMSRole` objects assigned to the user
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

    /// Checks if a user has any of the specified permissions.
    /// 
    /// # Parameters
    /// - `user_id`: User ID to check
    /// - `permissions`: List of permission IDs to check for
    /// 
    /// # Returns
    /// `true` if the user has at least one of the permissions, otherwise `false`
    pub async fn _Fhas_any_permission(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSResult<bool> {
        for permission in permissions {
            if self._Fhas_permission(user_id, permission).await? {
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
    pub async fn _Fhas_all_permissions(&self, user_id: &str, permissions: &[String]) -> crate::core::DMSResult<bool> {
        for permission in permissions {
            if !self._Fhas_permission(user_id, permission).await? {
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

    /// Deletes a permission.
    /// 
    /// # Parameters
    /// - `permission_id`: Permission ID to delete
    /// 
    /// # Returns
    /// `true` if the permission was successfully deleted, otherwise `false`
    pub async fn _Fdelete_permission(&self, permission_id: &str) -> crate::core::DMSResult<bool> {
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

    /// Lists all permissions.
    /// 
    /// # Returns
    /// A vector of all registered permissions
    pub async fn _Flist_permissions(&self) -> crate::core::DMSResult<Vec<DMSPermission>> {
        let permissions = self.permissions.read().await;
        Ok(permissions.values().cloned().collect())
    }

    /// Lists all roles.
    /// 
    /// # Returns
    /// A vector of all registered roles
    pub async fn _Flist_roles(&self) -> crate::core::DMSResult<Vec<DMSRole>> {
        let roles = self.roles.read().await;
        Ok(roles.values().cloned().collect())
    }
}