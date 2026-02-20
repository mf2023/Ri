// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

package com.dunimd.dmsc.cache;

import com.dunimd.dmsc.NativeLoader;

/**
 * Cache module for DMSC.
 * 
 * Provides multi-backend caching with support for Memory, Redis, and Hybrid backends.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * DMSCCacheConfig config = new DMSCCacheConfig()
 *     .setEnabled(true)
 *     .setDefaultTtlSecs(3600)
 *     .setBackendType(DMSCCacheBackendType.Memory);
 * 
 * DMSCCacheModule cache = new DMSCCacheModule(config);
 * cache.set("key", "value", 3600);
 * String value = cache.get("key");
 * }</pre>
 */
public class DMSCCacheModule {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new DMSCCacheModule with the given configuration.
     * 
     * @param config the cache configuration
     */
    public DMSCCacheModule(DMSCCacheConfig config) {
        this.nativePtr = new0(config.getNativePtr());
    }
    
    private native long new0(long configPtr);
    
    /**
     * Set a key-value pair in the cache.
     * 
     * @param key the cache key
     * @param value the cache value
     * @param ttlSecs the time-to-live in seconds
     */
    public void set(String key, String value, long ttlSecs) {
        set0(nativePtr, key, value, ttlSecs);
    }
    
    private native void set0(long ptr, String key, String value, long ttlSecs);
    
    /**
     * Get a value from the cache.
     * 
     * @param key the cache key
     * @return the cached value, or null if not found
     */
    public String get(String key) {
        return get0(nativePtr, key);
    }
    
    private native String get0(long ptr, String key);
    
    /**
     * Delete a key from the cache.
     * 
     * @param key the cache key
     */
    public void delete(String key) {
        delete0(nativePtr, key);
    }
    
    private native void delete0(long ptr, String key);
    
    /**
     * Check if a key exists in the cache.
     * 
     * @param key the cache key
     * @return true if the key exists
     */
    public boolean exists(String key) {
        return exists0(nativePtr, key);
    }
    
    private native boolean exists0(long ptr, String key);
    
    /**
     * Get cache statistics.
     * 
     * @return the cache statistics
     */
    public DMSCCacheStats getStats() {
        long statsPtr = getStats0(nativePtr);
        return new DMSCCacheStats(statsPtr);
    }
    
    private native long getStats0(long ptr);
    
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
