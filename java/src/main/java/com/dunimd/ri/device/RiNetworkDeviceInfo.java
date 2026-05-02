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
 * Network device information for Ri.
 * 
 * Contains information about a network device for remote device discovery.
 */
public class RiNetworkDeviceInfo {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiNetworkDeviceInfo(String id, String deviceType, String source) {
        this.nativePtr = new0(id, deviceType, source);
    }
    
    RiNetworkDeviceInfo(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String id, String deviceType, String source);
    
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
     * Get the device type.
     * 
     * @return the device type
     */
    public String getDeviceType() {
        return getDeviceType0(nativePtr);
    }
    
    private native String getDeviceType0(long ptr);
    
    /**
     * Get the source system identifier.
     * 
     * @return the source system identifier
     */
    public String getSource() {
        return getSource0(nativePtr);
    }
    
    private native String getSource0(long ptr);
    
    /**
     * Get the compute units.
     * 
     * @return the compute units, or -1 if not set
     */
    public int getComputeUnits() {
        return getComputeUnits0(nativePtr);
    }
    
    private native int getComputeUnits0(long ptr);
    
    /**
     * Set the compute units.
     * 
     * @param units the compute units
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
