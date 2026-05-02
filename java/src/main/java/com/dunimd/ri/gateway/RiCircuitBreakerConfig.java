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
 * Configuration for circuit breaker behavior.
 */
public class RiCircuitBreakerConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiCircuitBreakerConfig() {
        this.nativePtr = new0();
    }
    
    public RiCircuitBreakerConfig(int failureThreshold, int successThreshold, long timeoutSeconds, long monitoringPeriodSeconds) {
        this.nativePtr = newWithValues0(failureThreshold, successThreshold, timeoutSeconds, monitoringPeriodSeconds);
    }
    
    private native long new0();
    
    private native long newWithValues0(int failureThreshold, int successThreshold, long timeoutSeconds, long monitoringPeriodSeconds);
    
    public int getFailureThreshold() {
        return getFailureThreshold0(nativePtr);
    }
    
    private native int getFailureThreshold0(long ptr);
    
    public void setFailureThreshold(int value) {
        setFailureThreshold0(nativePtr, value);
    }
    
    private native void setFailureThreshold0(long ptr, int value);
    
    public int getSuccessThreshold() {
        return getSuccessThreshold0(nativePtr);
    }
    
    private native int getSuccessThreshold0(long ptr);
    
    public void setSuccessThreshold(int value) {
        setSuccessThreshold0(nativePtr, value);
    }
    
    private native void setSuccessThreshold0(long ptr, int value);
    
    public long getTimeoutSeconds() {
        return getTimeoutSeconds0(nativePtr);
    }
    
    private native long getTimeoutSeconds0(long ptr);
    
    public void setTimeoutSeconds(long value) {
        setTimeoutSeconds0(nativePtr, value);
    }
    
    private native void setTimeoutSeconds0(long ptr, long value);
    
    public long getMonitoringPeriodSeconds() {
        return getMonitoringPeriodSeconds0(nativePtr);
    }
    
    private native long getMonitoringPeriodSeconds0(long ptr);
    
    public void setMonitoringPeriodSeconds(long value) {
        setMonitoringPeriodSeconds0(nativePtr, value);
    }
    
    private native void setMonitoringPeriodSeconds0(long ptr, long value);
    
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
