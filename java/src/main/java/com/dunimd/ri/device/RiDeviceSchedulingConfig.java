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

package com.dunimd.ri.device;

import com.dunimd.ri.NativeLoader;

/**
 * Device scheduling configuration for Ri.
 */
public class RiDeviceSchedulingConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeviceSchedulingConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Check if discovery is enabled.
     * 
     * @return true if discovery is enabled
     */
    public boolean isDiscoveryEnabled() {
        return isDiscoveryEnabled0(nativePtr);
    }
    
    private native boolean isDiscoveryEnabled0(long ptr);
    
    /**
     * Set discovery enabled.
     * 
     * @param enabled whether discovery is enabled
     */
    public void setDiscoveryEnabled(boolean enabled) {
        setDiscoveryEnabled0(nativePtr, enabled);
    }
    
    private native void setDiscoveryEnabled0(long ptr, boolean enabled);
    
    /**
     * Get the discovery interval in seconds.
     * 
     * @return the discovery interval in seconds
     */
    public long getDiscoveryIntervalSecs() {
        return getDiscoveryIntervalSecs0(nativePtr);
    }
    
    private native long getDiscoveryIntervalSecs0(long ptr);
    
    /**
     * Set the discovery interval in seconds.
     * 
     * @param intervalSecs the discovery interval in seconds
     */
    public void setDiscoveryIntervalSecs(long intervalSecs) {
        setDiscoveryIntervalSecs0(nativePtr, intervalSecs);
    }
    
    private native void setDiscoveryIntervalSecs0(long ptr, long intervalSecs);
    
    /**
     * Check if auto scheduling is enabled.
     * 
     * @return true if auto scheduling is enabled
     */
    public boolean isAutoSchedulingEnabled() {
        return isAutoSchedulingEnabled0(nativePtr);
    }
    
    private native boolean isAutoSchedulingEnabled0(long ptr);
    
    /**
     * Set auto scheduling enabled.
     * 
     * @param enabled whether auto scheduling is enabled
     */
    public void setAutoSchedulingEnabled(boolean enabled) {
        setAutoSchedulingEnabled0(nativePtr, enabled);
    }
    
    private native void setAutoSchedulingEnabled0(long ptr, boolean enabled);
    
    /**
     * Get the maximum concurrent tasks.
     * 
     * @return the maximum concurrent tasks
     */
    public int getMaxConcurrentTasks() {
        return getMaxConcurrentTasks0(nativePtr);
    }
    
    private native int getMaxConcurrentTasks0(long ptr);
    
    /**
     * Set the maximum concurrent tasks.
     * 
     * @param max the maximum concurrent tasks
     */
    public void setMaxConcurrentTasks(int max) {
        setMaxConcurrentTasks0(nativePtr, max);
    }
    
    private native void setMaxConcurrentTasks0(long ptr, int max);
    
    /**
     * Get the resource allocation timeout in seconds.
     * 
     * @return the resource allocation timeout in seconds
     */
    public long getResourceAllocationTimeoutSecs() {
        return getResourceAllocationTimeoutSecs0(nativePtr);
    }
    
    private native long getResourceAllocationTimeoutSecs0(long ptr);
    
    /**
     * Set the resource allocation timeout in seconds.
     * 
     * @param timeoutSecs the resource allocation timeout in seconds
     */
    public void setResourceAllocationTimeoutSecs(long timeoutSecs) {
        setResourceAllocationTimeoutSecs0(nativePtr, timeoutSecs);
    }
    
    private native void setResourceAllocationTimeoutSecs0(long ptr, long timeoutSecs);
    
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
