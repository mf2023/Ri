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
 * Basic circuit breaker implementation.
 */
public class RiCircuitBreaker {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiCircuitBreaker() {
        RiCircuitBreakerConfig config = new RiCircuitBreakerConfig();
        this.nativePtr = new0(config.getNativePtr());
    }
    
    public RiCircuitBreaker(RiCircuitBreakerConfig config) {
        this.nativePtr = new0(config.getNativePtr());
    }
    
    private native long new0(long configPtr);
    
    public boolean allowRequest() {
        return allowRequest0(nativePtr);
    }
    
    private native boolean allowRequest0(long ptr);
    
    public void recordSuccess() {
        recordSuccess0(nativePtr);
    }
    
    private native void recordSuccess0(long ptr);
    
    public void recordFailure() {
        recordFailure0(nativePtr);
    }
    
    private native void recordFailure0(long ptr);
    
    public RiCircuitBreakerState getState() {
        int state = getState0(nativePtr);
        switch (state) {
            case 0:
                return RiCircuitBreakerState.CLOSED;
            case 1:
                return RiCircuitBreakerState.OPEN;
            case 2:
                return RiCircuitBreakerState.HALF_OPEN;
            default:
                return RiCircuitBreakerState.CLOSED;
        }
    }
    
    private native int getState0(long ptr);
    
    public RiCircuitBreakerMetrics getStats() {
        long statsPtr = getStats0(nativePtr);
        if (statsPtr == 0) {
            return null;
        }
        return new RiCircuitBreakerMetrics(statsPtr);
    }
    
    private native long getStats0(long ptr);
    
    public RiCircuitBreakerConfig getConfig() {
        long configPtr = getConfig0(nativePtr);
        if (configPtr == 0) {
            return null;
        }
        return new RiCircuitBreakerConfig();
    }
    
    private native long getConfig0(long ptr);
    
    public void reset() {
        reset0(nativePtr);
    }
    
    private native void reset0(long ptr);
    
    public void forceOpen() {
        forceOpen0(nativePtr);
    }
    
    private native void forceOpen0(long ptr);
    
    public void forceClose() {
        forceClose0(nativePtr);
    }
    
    private native void forceClose0(long ptr);
    
    public double getFailureRate() {
        return getFailureRate0(nativePtr);
    }
    
    private native double getFailureRate0(long ptr);
    
    public double getSuccessRate() {
        return getSuccessRate0(nativePtr);
    }
    
    private native double getSuccessRate0(long ptr);
    
    public long getTotalRequests() {
        return getTotalRequests0(nativePtr);
    }
    
    private native long getTotalRequests0(long ptr);
    
    public boolean isOpen() {
        return isOpen0(nativePtr);
    }
    
    private native boolean isOpen0(long ptr);
    
    public boolean isClosed() {
        return isClosed0(nativePtr);
    }
    
    private native boolean isClosed0(long ptr);
    
    public boolean isHalfOpen() {
        return isHalfOpen0(nativePtr);
    }
    
    private native boolean isHalfOpen0(long ptr);
    
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
