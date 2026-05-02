// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package com.dunimd.ri.auth;

import com.dunimd.ri.NativeLoader;
import java.util.List;
import java.util.ArrayList;

/**
 * Permission manager for Ri.
 * 
 * Manages permissions, roles, and user assignments for RBAC.
 */
public class RiPermissionManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiPermissionManager() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public void createPermission(RiPermission permission) {
        createPermission0(nativePtr, permission.getNativePtr());
    }
    
    private native void createPermission0(long ptr, long permissionPtr);
    
    public RiPermission getPermission(String permissionId) {
        long permPtr = getPermission0(nativePtr, permissionId);
        if (permPtr == 0) {
            return null;
        }
        return new RiPermission(permPtr);
    }
    
    private native long getPermission0(long ptr, String permissionId);
    
    public void createRole(RiRole role) {
        createRole0(nativePtr, role.getNativePtr());
    }
    
    private native void createRole0(long ptr, long rolePtr);
    
    public RiRole getRole(String roleId) {
        long rolePtr = getRole0(nativePtr, roleId);
        if (rolePtr == 0) {
            return null;
        }
        return new RiRole(rolePtr);
    }
    
    private native long getRole0(long ptr, String roleId);
    
    public boolean assignRoleToUser(String userId, String roleId) {
        return assignRoleToUser0(nativePtr, userId, roleId);
    }
    
    private native boolean assignRoleToUser0(long ptr, String userId, String roleId);
    
    public boolean removeRoleFromUser(String userId, String roleId) {
        return removeRoleFromUser0(nativePtr, userId, roleId);
    }
    
    private native boolean removeRoleFromUser0(long ptr, String userId, String roleId);
    
    public List<RiRole> getUserRoles(String userId) {
        long[] rolePtrs = getUserRoles0(nativePtr, userId);
        List<RiRole> roles = new ArrayList<>();
        for (long ptr : rolePtrs) {
            if (ptr != 0) {
                roles.add(new RiRole(ptr));
            }
        }
        return roles;
    }
    
    private native long[] getUserRoles0(long ptr, String userId);
    
    public boolean hasPermission(String userId, String permissionId) {
        return hasPermission0(nativePtr, userId, permissionId);
    }
    
    private native boolean hasPermission0(long ptr, String userId, String permissionId);
    
    public List<String> getUserPermissions(String userId) {
        String[] perms = getUserPermissions0(nativePtr, userId);
        return java.util.Arrays.asList(perms);
    }
    
    private native String[] getUserPermissions0(long ptr, String userId);
    
    public boolean deletePermission(String permissionId) {
        return deletePermission0(nativePtr, permissionId);
    }
    
    private native boolean deletePermission0(long ptr, String permissionId);
    
    public boolean deleteRole(String roleId) {
        return deleteRole0(nativePtr, roleId);
    }
    
    private native boolean deleteRole0(long ptr, String roleId);
    
    public List<RiPermission> listPermissions() {
        long[] permPtrs = listPermissions0(nativePtr);
        List<RiPermission> perms = new ArrayList<>();
        for (long ptr : permPtrs) {
            if (ptr != 0) {
                perms.add(new RiPermission(ptr));
            }
        }
        return perms;
    }
    
    private native long[] listPermissions0(long ptr);
    
    public List<RiRole> listRoles() {
        long[] rolePtrs = listRoles0(nativePtr);
        List<RiRole> roles = new ArrayList<>();
        for (long ptr : rolePtrs) {
            if (ptr != 0) {
                roles.add(new RiRole(ptr));
            }
        }
        return roles;
    }
    
    private native long[] listRoles0(long ptr);
    
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    @Override
    protected void finalize() {
        close();
    }
}
