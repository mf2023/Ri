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
 * Method call for Ri.
 * 
 * Represents an RPC method call request.
 */
public class RiMethodCall {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiMethodCall(String methodName, byte[] params) {
        this.nativePtr = new0(methodName, params);
    }
    
    RiMethodCall(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String methodName, byte[] params);
    
    /**
     * Get the method name.
     * 
     * @return the method name
     */
    public String getMethodName() {
        return getMethodName0(nativePtr);
    }
    
    private native String getMethodName0(long ptr);
    
    /**
     * Get the parameters.
     * 
     * @return the parameters
     */
    public byte[] getParams() {
        return getParams0(nativePtr);
    }
    
    private native byte[] getParams0(long ptr);
    
    /**
     * Get the timeout in milliseconds.
     * 
     * @return the timeout in milliseconds
     */
    public long getTimeoutMs() {
        return getTimeoutMs0(nativePtr);
    }
    
    private native long getTimeoutMs0(long ptr);
    
    /**
     * Set the timeout in milliseconds.
     * 
     * @param timeoutMs the timeout in milliseconds
     */
    public void setTimeoutMs(long timeoutMs) {
        setTimeoutMs0(nativePtr, timeoutMs);
    }
    
    private native void setTimeoutMs0(long ptr, long timeoutMs);
    
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
