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

package com.dunimd.ri;

/**
 * Configuration for health checks.
 */
public class RiHealthCheckConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiHealthCheckConfig with default values.
     */
    public RiHealthCheckConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Create a new RiHealthCheckConfig with specified values.
     * 
     * @param checkInterval the interval between health checks in seconds
     * @param timeout the timeout for individual health checks in seconds
     * @param failureThreshold the number of consecutive failures before marking as unhealthy
     * @param successThreshold the number of consecutive successes before marking as healthy
     * @param enabled whether the health check is enabled
     */
    public RiHealthCheckConfig(long checkInterval, long timeout, int failureThreshold, int successThreshold, boolean enabled) {
        this.nativePtr = newWithValues0(checkInterval, timeout, failureThreshold, successThreshold, enabled);
    }
    
    private native long newWithValues0(long checkInterval, long timeout, int failureThreshold, int successThreshold, boolean enabled);
    
    /**
     * Get the check interval in seconds.
     * 
     * @return the check interval
     */
    public long getCheckInterval() {
        return getCheckInterval0(nativePtr);
    }
    
    private native long getCheckInterval0(long ptr);
    
    /**
     * Set the check interval in seconds.
     * 
     * @param value the check interval
     */
    public void setCheckInterval(long value) {
        setCheckInterval0(nativePtr, value);
    }
    
    private native void setCheckInterval0(long ptr, long value);
    
    /**
     * Get the timeout in seconds.
     * 
     * @return the timeout
     */
    public long getTimeout() {
        return getTimeout0(nativePtr);
    }
    
    private native long getTimeout0(long ptr);
    
    /**
     * Set the timeout in seconds.
     * 
     * @param value the timeout
     */
    public void setTimeout(long value) {
        setTimeout0(nativePtr, value);
    }
    
    private native void setTimeout0(long ptr, long value);
    
    /**
     * Get the failure threshold.
     * 
     * @return the failure threshold
     */
    public int getFailureThreshold() {
        return getFailureThreshold0(nativePtr);
    }
    
    private native int getFailureThreshold0(long ptr);
    
    /**
     * Get the success threshold.
     * 
     * @return the success threshold
     */
    public int getSuccessThreshold() {
        return getSuccessThreshold0(nativePtr);
    }
    
    private native int getSuccessThreshold0(long ptr);
    
    /**
     * Check if the health check is enabled.
     * 
     * @return true if enabled
     */
    public boolean isEnabled() {
        return isEnabled0(nativePtr);
    }
    
    private native boolean isEnabled0(long ptr);
    
    /**
     * Get the native pointer.
     * 
     * @return the native pointer
     */
    public long getNativePtr() {
        return nativePtr;
    }
    
    /**
     * Release native resources.
     */
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
