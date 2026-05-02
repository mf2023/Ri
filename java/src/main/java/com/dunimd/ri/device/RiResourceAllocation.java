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
 * Resource allocation for Ri.
 * 
 * Contains information about a successful resource allocation.
 */
public class RiResourceAllocation {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiResourceAllocation(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the allocation ID.
     * 
     * @return the allocation ID
     */
    public String getAllocationId() {
        return getAllocationId0(nativePtr);
    }
    
    private native String getAllocationId0(long ptr);
    
    /**
     * Get the device ID.
     * 
     * @return the device ID
     */
    public String getDeviceId() {
        return getDeviceId0(nativePtr);
    }
    
    private native String getDeviceId0(long ptr);
    
    /**
     * Get the device name.
     * 
     * @return the device name
     */
    public String getDeviceName() {
        return getDeviceName0(nativePtr);
    }
    
    private native String getDeviceName0(long ptr);
    
    /**
     * Get the allocated at timestamp.
     * 
     * @return the allocated at timestamp in ISO 8601 format
     */
    public String getAllocatedAt() {
        return getAllocatedAt0(nativePtr);
    }
    
    private native String getAllocatedAt0(long ptr);
    
    /**
     * Get the expires at timestamp.
     * 
     * @return the expires at timestamp in ISO 8601 format
     */
    public String getExpiresAt() {
        return getExpiresAt0(nativePtr);
    }
    
    private native String getExpiresAt0(long ptr);
    
    /**
     * Check if the allocation is expired.
     * 
     * @return true if the allocation is expired
     */
    public boolean isExpired() {
        return isExpired0(nativePtr);
    }
    
    private native boolean isExpired0(long ptr);
    
    /**
     * Get the remaining time in seconds.
     * 
     * @return the remaining time in seconds
     */
    public long getRemainingTimeSecs() {
        return getRemainingTimeSecs0(nativePtr);
    }
    
    private native long getRemainingTimeSecs0(long ptr);
    
    /**
     * Get the original resource request.
     * 
     * @return the original resource request
     */
    public RiResourceRequest getRequest() {
        long ptr = getRequest0(nativePtr);
        return new RiResourceRequest(ptr);
    }
    
    private native long getRequest0(long ptr);
    
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
