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
 * Sliding window rate limiter for fine-grained control.
 */
public class RiSlidingWindowRateLimiter {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSlidingWindowRateLimiter(int maxRequests, long windowSeconds) {
        this.nativePtr = new0(maxRequests, windowSeconds);
    }
    
    private native long new0(int maxRequests, long windowSeconds);
    
    public boolean allowRequest() {
        return allowRequest0(nativePtr);
    }
    
    private native boolean allowRequest0(long ptr);
    
    public int getCurrentCount() {
        return getCurrentCount0(nativePtr);
    }
    
    private native int getCurrentCount0(long ptr);
    
    public void reset() {
        reset0(nativePtr);
    }
    
    private native void reset0(long ptr);
    
    public int getMaxRequests() {
        return getMaxRequests0(nativePtr);
    }
    
    private native int getMaxRequests0(long ptr);
    
    public long getWindowSeconds() {
        return getWindowSeconds0(nativePtr);
    }
    
    private native long getWindowSeconds0(long ptr);
    
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
