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
import java.util.Set;
import java.util.HashSet;
import java.util.List;
import java.util.ArrayList;

/**
 * Role for Ri.
 * 
 * Defines a role with permissions for RBAC.
 */
public class RiRole {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRole(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiRole(String id, String name, String description, List<String> permissions, boolean isSystem) {
        this.nativePtr = new0(id, name, description, permissions.toArray(new String[0]), isSystem);
    }
    
    private native long new0(String id, String name, String description, String[] permissions, boolean isSystem);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    public String getDescription() {
        return getDescription0(nativePtr);
    }
    
    private native String getDescription0(long ptr);
    
    public List<String> getPermissions() {
        String[] perms = getPermissions0(nativePtr);
        return java.util.Arrays.asList(perms);
    }
    
    private native String[] getPermissions0(long ptr);
    
    public boolean isSystem() {
        return isSystem0(nativePtr);
    }
    
    private native boolean isSystem0(long ptr);
    
    public boolean hasPermission(String permissionId) {
        return hasPermission0(nativePtr, permissionId);
    }
    
    private native boolean hasPermission0(long ptr, String permissionId);
    
    public void addPermission(String permissionId) {
        addPermission0(nativePtr, permissionId);
    }
    
    private native void addPermission0(long ptr, String permissionId);
    
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
