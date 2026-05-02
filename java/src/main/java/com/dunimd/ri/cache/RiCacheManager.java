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
 * Cache manager that coordinates different cache backends with consistency support.
 */
public class RiCacheManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiCacheManager() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    RiCacheManager(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String get(String key) {
        return get0(nativePtr, key);
    }
    
    private native String get0(long ptr, String key);
    
    public void set(String key, String value, Long ttlSecs) {
        set0(nativePtr, key, value, ttlSecs != null ? ttlSecs : -1);
    }
    
    private native void set0(long ptr, String key, String value, long ttlSecs);
    
    public boolean delete(String key) {
        return delete0(nativePtr, key);
    }
    
    private native boolean delete0(long ptr, String key);
    
    public boolean exists(String key) {
        return exists0(nativePtr, key);
    }
    
    private native boolean exists0(long ptr, String key);
    
    public void clear() {
        clear0(nativePtr);
    }
    
    private native void clear0(long ptr);
    
    public RiCacheStats stats() {
        long statsPtr = stats0(nativePtr);
        return new RiCacheStats(statsPtr);
    }
    
    private native long stats0(long ptr);
    
    public long cleanupExpired() {
        return cleanupExpired0(nativePtr);
    }
    
    private native long cleanupExpired0(long ptr);
    
    public void invalidatePattern(String pattern) {
        invalidatePattern0(nativePtr, pattern);
    }
    
    private native void invalidatePattern0(long ptr, String pattern);
    
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
