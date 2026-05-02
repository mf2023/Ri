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
 * Metrics for monitoring circuit breaker performance.
 */
public class RiCircuitBreakerMetrics {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiCircuitBreakerMetrics(String state, long failureCount, long successCount, long consecutiveFailures, long consecutiveSuccesses) {
        this.nativePtr = new0(state, failureCount, successCount, consecutiveFailures, consecutiveSuccesses);
    }
    
    private RiCircuitBreakerMetrics(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String state, long failureCount, long successCount, long consecutiveFailures, long consecutiveSuccesses);
    
    public String getState() {
        return getState0(nativePtr);
    }
    
    private native String getState0(long ptr);
    
    public long getFailureCount() {
        return getFailureCount0(nativePtr);
    }
    
    private native long getFailureCount0(long ptr);
    
    public long getSuccessCount() {
        return getSuccessCount0(nativePtr);
    }
    
    private native long getSuccessCount0(long ptr);
    
    public long getConsecutiveFailures() {
        return getConsecutiveFailures0(nativePtr);
    }
    
    private native long getConsecutiveFailures0(long ptr);
    
    public long getConsecutiveSuccesses() {
        return getConsecutiveSuccesses0(nativePtr);
    }
    
    private native long getConsecutiveSuccesses0(long ptr);
    
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
