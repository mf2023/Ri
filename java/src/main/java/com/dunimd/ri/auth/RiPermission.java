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

/**
 * Permission for Ri.
 * 
 * Defines a permission with resource and action.
 */
public class RiPermission {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiPermission(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiPermission(String id, String name, String description, String resource, String action) {
        this.nativePtr = new0(id, name, description, resource, action);
    }
    
    private native long new0(String id, String name, String description, String resource, String action);
    
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
    
    public String getResource() {
        return getResource0(nativePtr);
    }
    
    private native String getResource0(long ptr);
    
    public String getAction() {
        return getAction0(nativePtr);
    }
    
    private native String getAction0(long ptr);
    
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
