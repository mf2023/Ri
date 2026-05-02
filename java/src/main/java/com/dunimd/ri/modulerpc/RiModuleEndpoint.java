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

package com.dunimd.ri.modulerpc;

import com.dunimd.ri.NativeLoader;
import java.util.List;

/**
 * Module endpoint for Ri.
 * 
 * Endpoint definition for a module's exposed methods.
 */
public class RiModuleEndpoint {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiModuleEndpoint(String moduleName) {
        this.nativePtr = new0(moduleName);
    }
    
    RiModuleEndpoint(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String moduleName);
    
    /**
     * Get the module name.
     * 
     * @return the module name
     */
    public String getModuleName() {
        return getModuleName0(nativePtr);
    }
    
    private native String getModuleName0(long ptr);
    
    /**
     * List the registered methods.
     * 
     * @return the list of method names
     */
    public List<String> listMethods() {
        return listMethods0(nativePtr);
    }
    
    private native List<String> listMethods0(long ptr);
    
    /**
     * Release native resources.
     */
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
