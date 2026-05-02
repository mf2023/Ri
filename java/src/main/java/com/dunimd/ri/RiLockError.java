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

package com.dunimd.ri;

/**
 * Specialized error type for lock-related failures.
 * 
 * This error type provides detailed information about lock acquisition failures,
 * including whether the lock was poisoned or simply contested.
 */
public class RiLockError {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiLockError with a context message.
     * 
     * @param context the context message
     */
    public RiLockError(String context) {
        this.nativePtr = new0(context);
    }
    
    private native long new0(String context);
    
    /**
     * Create a new RiLockError with context and poisoned flag.
     * 
     * @param context the context message
     * @param isPoisoned whether the lock is poisoned
     */
    public RiLockError(String context, boolean isPoisoned) {
        this.nativePtr = newWithPoisoned0(context, isPoisoned);
    }
    
    private native long newWithPoisoned0(String context, boolean isPoisoned);
    
    /**
     * Create a poisoned lock error.
     * 
     * @param context the context message
     * @return a poisoned RiLockError
     */
    public static RiLockError poisoned(String context) {
        long ptr = poisoned0(context);
        return new RiLockError(ptr);
    }
    
    private static native long poisoned0(String context);
    
    /**
     * Get the context message.
     * 
     * @return the context message
     */
    public String getContext() {
        return getContext0(nativePtr);
    }
    
    private native String getContext0(long ptr);
    
    /**
     * Check if the lock is poisoned.
     * 
     * @return true if poisoned
     */
    public boolean isPoisoned() {
        return isPoisoned0(nativePtr);
    }
    
    private native boolean isPoisoned0(long ptr);
    
    /**
     * Get the error message.
     * 
     * @return the error message
     */
    public String getMessage() {
        return getMessage0(nativePtr);
    }
    
    private native String getMessage0(long ptr);
    
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
    
    private RiLockError(long ptr) {
        this.nativePtr = ptr;
    }
}
