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
 * Cached value wrapper with TTL and LRU support.
 */
public class RiCachedValue {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiCachedValue(String value, Long ttlSecs) {
        this.nativePtr = new0(value, ttlSecs != null ? ttlSecs : -1);
    }
    
    private native long new0(String value, long ttlSecs);
    
    RiCachedValue(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getValue() {
        return getValue0(nativePtr);
    }
    
    private native String getValue0(long ptr);
    
    public Long getExpiresAt() {
        long expiresAt = getExpiresAt0(nativePtr);
        return expiresAt >= 0 ? expiresAt : null;
    }
    
    private native long getExpiresAt0(long ptr);
    
    public Long getLastAccessed() {
        long lastAccessed = getLastAccessed0(nativePtr);
        return lastAccessed >= 0 ? lastAccessed : null;
    }
    
    private native long getLastAccessed0(long ptr);
    
    public boolean isExpired() {
        return isExpired0(nativePtr);
    }
    
    private native boolean isExpired0(long ptr);
    
    public void touch() {
        touch0(nativePtr);
    }
    
    private native void touch0(long ptr);
    
    public boolean isStale(long maxIdleSecs) {
        return isStale0(nativePtr, maxIdleSecs);
    }
    
    private native boolean isStale0(long ptr, long maxIdleSecs);
    
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
