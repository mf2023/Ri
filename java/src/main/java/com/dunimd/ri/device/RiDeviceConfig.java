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
 * Device configuration for Ri.
 */
public class RiDeviceConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeviceConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Check if CPU discovery is enabled.
     * 
     * @return true if CPU discovery is enabled
     */
    public boolean isCpuDiscoveryEnabled() {
        return isCpuDiscoveryEnabled0(nativePtr);
    }
    
    private native boolean isCpuDiscoveryEnabled0(long ptr);
    
    /**
     * Set CPU discovery enabled.
     * 
     * @param enabled whether CPU discovery is enabled
     */
    public void setCpuDiscoveryEnabled(boolean enabled) {
        setCpuDiscoveryEnabled0(nativePtr, enabled);
    }
    
    private native void setCpuDiscoveryEnabled0(long ptr, boolean enabled);
    
    /**
     * Check if GPU discovery is enabled.
     * 
     * @return true if GPU discovery is enabled
     */
    public boolean isGpuDiscoveryEnabled() {
        return isGpuDiscoveryEnabled0(nativePtr);
    }
    
    private native boolean isGpuDiscoveryEnabled0(long ptr);
    
    /**
     * Set GPU discovery enabled.
     * 
     * @param enabled whether GPU discovery is enabled
     */
    public void setGpuDiscoveryEnabled(boolean enabled) {
        setGpuDiscoveryEnabled0(nativePtr, enabled);
    }
    
    private native void setGpuDiscoveryEnabled0(long ptr, boolean enabled);
    
    /**
     * Get the discovery timeout in seconds.
     * 
     * @return the discovery timeout in seconds
     */
    public long getDiscoveryTimeoutSecs() {
        return getDiscoveryTimeoutSecs0(nativePtr);
    }
    
    private native long getDiscoveryTimeoutSecs0(long ptr);
    
    /**
     * Set the discovery timeout in seconds.
     * 
     * @param timeoutSecs the discovery timeout in seconds
     */
    public void setDiscoveryTimeoutSecs(long timeoutSecs) {
        setDiscoveryTimeoutSecs0(nativePtr, timeoutSecs);
    }
    
    private native void setDiscoveryTimeoutSecs0(long ptr, long timeoutSecs);
    
    /**
     * Get the maximum devices per type.
     * 
     * @return the maximum devices per type
     */
    public int getMaxDevicesPerType() {
        return getMaxDevicesPerType0(nativePtr);
    }
    
    private native int getMaxDevicesPerType0(long ptr);
    
    /**
     * Set the maximum devices per type.
     * 
     * @param max the maximum devices per type
     */
    public void setMaxDevicesPerType(int max) {
        setMaxDevicesPerType0(nativePtr, max);
    }
    
    private native void setMaxDevicesPerType0(long ptr, int max);
    
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
