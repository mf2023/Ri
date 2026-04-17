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

package com.dunimd.ri.cache;

import com.dunimd.ri.NativeLoader;

/**
 * Cache configuration for Ri.
 */
public class RiCacheConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiCacheConfig with default values.
     */
    public RiCacheConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get the native pointer.
     * 
     * @return the native pointer
     */
    long getNativePtr() {
        return nativePtr;
    }
    
    /**
     * Set whether caching is enabled.
     * 
     * @param enabled true to enable caching
     * @return this config instance
     */
    public RiCacheConfig setEnabled(boolean enabled) {
        setEnabled0(nativePtr, enabled);
        return this;
    }
    
    private native void setEnabled0(long ptr, boolean enabled);
    
    /**
     * Set the default TTL in seconds.
     * 
     * @param ttlSecs the default TTL in seconds
     * @return this config instance
     */
    public RiCacheConfig setDefaultTtlSecs(long ttlSecs) {
        setDefaultTtlSecs0(nativePtr, ttlSecs);
        return this;
    }
    
    private native void setDefaultTtlSecs0(long ptr, long ttlSecs);
    
    /**
     * Set the backend type.
     * 
     * @param backendType the backend type
     * @return this config instance
     */
    public RiCacheConfig setBackendType(RiCacheBackendType backendType) {
        setBackendType0(nativePtr, backendType.ordinal());
        return this;
    }
    
    private native void setBackendType0(long ptr, int backendType);
    
    /**
     * Set the Redis URL.
     * 
     * @param url the Redis URL
     * @return this config instance
     */
    public RiCacheConfig setRedisUrl(String url) {
        setRedisUrl0(nativePtr, url);
        return this;
    }
    
    private native void setRedisUrl0(long ptr, String url);
    
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
