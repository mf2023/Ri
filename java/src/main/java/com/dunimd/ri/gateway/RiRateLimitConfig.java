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
 * Configuration for rate limiting behavior.
 */
public class RiRateLimitConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRateLimitConfig() {
        this.nativePtr = new0();
    }
    
    public RiRateLimitConfig(int requestsPerSecond, int burstSize, long windowSeconds) {
        this.nativePtr = newWithValues0(requestsPerSecond, burstSize, windowSeconds);
    }
    
    private native long new0();
    
    private native long newWithValues0(int requestsPerSecond, int burstSize, long windowSeconds);
    
    public int getRequestsPerSecond() {
        return getRequestsPerSecond0(nativePtr);
    }
    
    private native int getRequestsPerSecond0(long ptr);
    
    public void setRequestsPerSecond(int value) {
        setRequestsPerSecond0(nativePtr, value);
    }
    
    private native void setRequestsPerSecond0(long ptr, int value);
    
    public int getBurstSize() {
        return getBurstSize0(nativePtr);
    }
    
    private native int getBurstSize0(long ptr);
    
    public void setBurstSize(int value) {
        setBurstSize0(nativePtr, value);
    }
    
    private native void setBurstSize0(long ptr, int value);
    
    public long getWindowSeconds() {
        return getWindowSeconds0(nativePtr);
    }
    
    private native long getWindowSeconds0(long ptr);
    
    public void setWindowSeconds(long value) {
        setWindowSeconds0(nativePtr, value);
    }
    
    private native void setWindowSeconds0(long ptr, long value);
    
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
