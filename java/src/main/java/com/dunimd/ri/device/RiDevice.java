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
 * Smart device representation for Ri.
 * 
 * Represents a smart device in the Ri system, including its status, capabilities,
 * health metrics, and lifecycle information.
 */
public class RiDevice {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDevice(String name, RiDeviceType deviceType) {
        this.nativePtr = new0(name, deviceType.ordinal());
    }
    
    RiDevice(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String name, int deviceType);
    
    /**
     * Get the device ID.
     * 
     * @return the device ID
     */
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    /**
     * Get the device name.
     * 
     * @return the device name
     */
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    /**
     * Get the device type.
     * 
     * @return the device type
     */
    public RiDeviceType getDeviceType() {
        int ordinal = getDeviceType0(nativePtr);
        return RiDeviceType.values()[ordinal];
    }
    
    private native int getDeviceType0(long ptr);
    
    /**
     * Get the device status.
     * 
     * @return the device status
     */
    public RiDeviceStatus getStatus() {
        int ordinal = getStatus0(nativePtr);
        return RiDeviceStatus.values()[ordinal];
    }
    
    private native int getStatus0(long ptr);
    
    /**
     * Set the device status.
     * 
     * @param status the device status
     */
    public void setStatus(RiDeviceStatus status) {
        setStatus0(nativePtr, status.ordinal());
    }
    
    private native void setStatus0(long ptr, int status);
    
    /**
     * Get the device capabilities.
     * 
     * @return the device capabilities
     */
    public RiDeviceCapabilities getCapabilities() {
        long ptr = getCapabilities0(nativePtr);
        return new RiDeviceCapabilities(ptr);
    }
    
    private native long getCapabilities0(long ptr);
    
    /**
     * Set the device capabilities.
     * 
     * @param capabilities the device capabilities
     */
    public void setCapabilities(RiDeviceCapabilities capabilities) {
        setCapabilities0(nativePtr, capabilities.getNativePtr());
    }
    
    private native void setCapabilities0(long ptr, long capabilitiesPtr);
    
    /**
     * Get the device health metrics.
     * 
     * @return the device health metrics
     */
    public RiDeviceHealthMetrics getHealthMetrics() {
        long ptr = getHealthMetrics0(nativePtr);
        return new RiDeviceHealthMetrics(ptr);
    }
    
    private native long getHealthMetrics0(long ptr);
    
    /**
     * Check if the device is available.
     * 
     * @return true if the device is available
     */
    public boolean isAvailable() {
        return isAvailable0(nativePtr);
    }
    
    private native boolean isAvailable0(long ptr);
    
    /**
     * Check if the device is allocated.
     * 
     * @return true if the device is allocated
     */
    public boolean isAllocated() {
        return isAllocated0(nativePtr);
    }
    
    private native boolean isAllocated0(long ptr);
    
    /**
     * Get the health score (0-100).
     * 
     * @return the health score
     */
    public int getHealthScore() {
        return getHealthScore0(nativePtr);
    }
    
    private native int getHealthScore0(long ptr);
    
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
    
    long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
