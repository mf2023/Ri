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

package com.dunimd.ri.gateway;

import com.dunimd.ri.NativeLoader;

/**
 * Token bucket based rate limiter implementation.
 */
public class RiRateLimiter {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRateLimiter() {
        RiRateLimitConfig config = new RiRateLimitConfig();
        this.nativePtr = new0(config.getNativePtr());
    }
    
    public RiRateLimiter(RiRateLimitConfig config) {
        this.nativePtr = new0(config.getNativePtr());
    }
    
    private native long new0(long configPtr);
    
    public boolean checkRequest(String key, int tokens) {
        return checkRateLimit0(nativePtr, key, tokens);
    }
    
    private native boolean checkRateLimit0(long ptr, String key, int tokens);
    
    public RiRateLimitStats getStats(String key) {
        long statsPtr = getStats0(nativePtr, key);
        if (statsPtr == 0) {
            return null;
        }
        return new RiRateLimitStats(statsPtr);
    }
    
    private native long getStats0(long ptr, String key);
    
    public double getRemaining(String key) {
        return getRemaining0(nativePtr, key);
    }
    
    private native double getRemaining0(long ptr, String key);
    
    public void resetBucket(String key) {
        resetBucket0(nativePtr, key);
    }
    
    private native void resetBucket0(long ptr, String key);
    
    public void clearAllBuckets() {
        clearAllBuckets0(nativePtr);
    }
    
    private native void clearAllBuckets0(long ptr);
    
    public RiRateLimitConfig getConfig() {
        long configPtr = getConfig0(nativePtr);
        if (configPtr == 0) {
            return null;
        }
        return new RiRateLimitConfig();
    }
    
    private native long getConfig0(long ptr);
    
    public int bucketCount() {
        return bucketCount0(nativePtr);
    }
    
    private native int bucketCount0(long ptr);
    
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
