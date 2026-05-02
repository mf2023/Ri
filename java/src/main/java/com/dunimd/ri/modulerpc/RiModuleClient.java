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

/**
 * Module client for Ri.
 * 
 * Provides client for making RPC calls to other modules.
 */
public class RiModuleClient {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiModuleClient(RiModuleRPC rpc) {
        this.nativePtr = new0(rpc.getNativePtr());
    }
    
    private native long new0(long rpcPtr);
    
    /**
     * Call a method on a module.
     * 
     * @param moduleName the module name
     * @param methodName the method name
     * @param params the parameters
     * @return the method response
     */
    public RiMethodResponse call(String moduleName, String methodName, byte[] params) {
        long ptr = call0(nativePtr, moduleName, methodName, params);
        return new RiMethodResponse(ptr);
    }
    
    private native long call0(long ptr, String moduleName, String methodName, byte[] params);
    
    /**
     * Call a method on a module with timeout.
     * 
     * @param moduleName the module name
     * @param methodName the method name
     * @param params the parameters
     * @param timeoutMs the timeout in milliseconds
     * @return the method response
     */
    public RiMethodResponse callWithTimeout(String moduleName, String methodName, byte[] params, long timeoutMs) {
        long ptr = callWithTimeout0(nativePtr, moduleName, methodName, params, timeoutMs);
        return new RiMethodResponse(ptr);
    }
    
    private native long callWithTimeout0(long ptr, String moduleName, String methodName, byte[] params, long timeoutMs);
    
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
    
    @Override
    protected void finalize() {
        close();
    }
}
