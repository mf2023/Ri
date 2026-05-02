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
import java.util.HashMap;
import java.util.Map;

/**
 * Device capabilities for Ri.
 * 
 * Represents the capabilities of a device for resource allocation.
 */
public class RiDeviceCapabilities {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeviceCapabilities() {
        this.nativePtr = new0();
    }
    
    RiDeviceCapabilities(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Get the number of compute units.
     * 
     * @return the compute units, or -1 if not set
     */
    public int getComputeUnits() {
        return getComputeUnits0(nativePtr);
    }
    
    private native int getComputeUnits0(long ptr);
    
    /**
     * Set the number of compute units.
     * 
     * @param units the number of compute units
     */
    public void setComputeUnits(int units) {
        setComputeUnits0(nativePtr, units);
    }
    
    private native void setComputeUnits0(long ptr, int units);
    
    /**
     * Get the memory capacity in GB.
     * 
     * @return the memory in GB, or -1.0 if not set
     */
    public double getMemoryGb() {
        return getMemoryGb0(nativePtr);
    }
    
    private native double getMemoryGb0(long ptr);
    
    /**
     * Set the memory capacity in GB.
     * 
     * @param memoryGb the memory in GB
     */
    public void setMemoryGb(double memoryGb) {
        setMemoryGb0(nativePtr, memoryGb);
    }
    
    private native void setMemoryGb0(long ptr, double memoryGb);
    
    /**
     * Get the storage capacity in GB.
     * 
     * @return the storage in GB, or -1.0 if not set
     */
    public double getStorageGb() {
        return getStorageGb0(nativePtr);
    }
    
    private native double getStorageGb0(long ptr);
    
    /**
     * Set the storage capacity in GB.
     * 
     * @param storageGb the storage in GB
     */
    public void setStorageGb(double storageGb) {
        setStorageGb0(nativePtr, storageGb);
    }
    
    private native void setStorageGb0(long ptr, double storageGb);
    
    /**
     * Get the bandwidth in Gbps.
     * 
     * @return the bandwidth in Gbps, or -1.0 if not set
     */
    public double getBandwidthGbps() {
        return getBandwidthGbps0(nativePtr);
    }
    
    private native double getBandwidthGbps0(long ptr);
    
    /**
     * Set the bandwidth in Gbps.
     * 
     * @param bandwidthGbps the bandwidth in Gbps
     */
    public void setBandwidthGbps(double bandwidthGbps) {
        setBandwidthGbps0(nativePtr, bandwidthGbps);
    }
    
    private native void setBandwidthGbps0(long ptr, double bandwidthGbps);
    
    /**
     * Check if the device meets the required capabilities.
     * 
     * @param requirements the required capabilities
     * @return true if the device meets the requirements
     */
    public boolean meetsRequirements(RiDeviceCapabilities requirements) {
        return meetsRequirements0(nativePtr, requirements.nativePtr);
    }
    
    private native boolean meetsRequirements0(long ptr, long requirementsPtr);
    
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
