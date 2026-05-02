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
 * Configuration for dead letter queue functionality.
 */
public class RiDeadLetterConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeadLetterConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    RiDeadLetterConfig(long ptr) {
        this.nativePtr = ptr;
    }
    
    long getNativePtr() {
        return nativePtr;
    }
    
    public RiDeadLetterConfig setEnabled(boolean enabled) {
        setEnabled0(nativePtr, enabled);
        return this;
    }
    
    private native void setEnabled0(long ptr, boolean enabled);
    
    public boolean isEnabled() {
        return isEnabled0(nativePtr);
    }
    
    private native boolean isEnabled0(long ptr);
    
    public RiDeadLetterConfig setMaxRetryCount(int maxRetryCount) {
        setMaxRetryCount0(nativePtr, maxRetryCount);
        return this;
    }
    
    private native void setMaxRetryCount0(long ptr, int maxRetryCount);
    
    public int getMaxRetryCount() {
        return getMaxRetryCount0(nativePtr);
    }
    
    private native int getMaxRetryCount0(long ptr);
    
    public RiDeadLetterConfig setDeadLetterQueueName(String name) {
        setDeadLetterQueueName0(nativePtr, name);
        return this;
    }
    
    private native void setDeadLetterQueueName0(long ptr, String name);
    
    public String getDeadLetterQueueName() {
        return getDeadLetterQueueName0(nativePtr);
    }
    
    private native String getDeadLetterQueueName0(long ptr);
    
    public RiDeadLetterConfig setTtlHours(int ttlHours) {
        setTtlHours0(nativePtr, ttlHours);
        return this;
    }
    
    private native void setTtlHours0(long ptr, int ttlHours);
    
    public int getTtlHours() {
        return getTtlHours0(nativePtr);
    }
    
    private native int getTtlHours0(long ptr);
    
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
