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

package com.dunimd.ri.queue;

import com.dunimd.ri.NativeLoader;

/**
 * Configuration for message retry behavior.
 */
public class RiRetryPolicy {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRetryPolicy() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    RiRetryPolicy(long ptr) {
        this.nativePtr = ptr;
    }
    
    long getNativePtr() {
        return nativePtr;
    }
    
    public RiRetryPolicy setMaxRetries(int maxRetries) {
        setMaxRetries0(nativePtr, maxRetries);
        return this;
    }
    
    private native void setMaxRetries0(long ptr, int maxRetries);
    
    public int getMaxRetries() {
        return getMaxRetries0(nativePtr);
    }
    
    private native int getMaxRetries0(long ptr);
    
    public RiRetryPolicy setInitialDelayMs(long initialDelayMs) {
        setInitialDelayMs0(nativePtr, initialDelayMs);
        return this;
    }
    
    private native void setInitialDelayMs0(long ptr, long initialDelayMs);
    
    public long getInitialDelayMs() {
        return getInitialDelayMs0(nativePtr);
    }
    
    private native long getInitialDelayMs0(long ptr);
    
    public RiRetryPolicy setMaxDelayMs(long maxDelayMs) {
        setMaxDelayMs0(nativePtr, maxDelayMs);
        return this;
    }
    
    private native void setMaxDelayMs0(long ptr, long maxDelayMs);
    
    public long getMaxDelayMs() {
        return getMaxDelayMs0(nativePtr);
    }
    
    private native long getMaxDelayMs0(long ptr);
    
    public RiRetryPolicy setBackoffMultiplier(double backoffMultiplier) {
        setBackoffMultiplier0(nativePtr, backoffMultiplier);
        return this;
    }
    
    private native void setBackoffMultiplier0(long ptr, double backoffMultiplier);
    
    public double getBackoffMultiplier() {
        return getBackoffMultiplier0(nativePtr);
    }
    
    private native double getBackoffMultiplier0(long ptr);
    
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
