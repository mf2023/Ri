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
 * Resource request for Ri.
 * 
 * Defines the requirements for resource allocation.
 */
public class RiResourceRequest {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiResourceRequest(String requestId, RiDeviceType deviceType, RiDeviceCapabilities capabilities) {
        this.nativePtr = new0(requestId, deviceType.ordinal(), capabilities.getNativePtr());
    }
    
    RiResourceRequest(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String requestId, int deviceType, long capabilitiesPtr);
    
    /**
     * Get the request ID.
     * 
     * @return the request ID
     */
    public String getRequestId() {
        return getRequestId0(nativePtr);
    }
    
    private native String getRequestId0(long ptr);
    
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
     * Get the required capabilities.
     * 
     * @return the required capabilities
     */
    public RiDeviceCapabilities getRequiredCapabilities() {
        long ptr = getRequiredCapabilities0(nativePtr);
        return new RiDeviceCapabilities(ptr);
    }
    
    private native long getRequiredCapabilities0(long ptr);
    
    /**
     * Get the priority (1-10, higher is more important).
     * 
     * @return the priority
     */
    public int getPriority() {
        return getPriority0(nativePtr);
    }
    
    private native int getPriority0(long ptr);
    
    /**
     * Set the priority (1-10, higher is more important).
     * 
     * @param priority the priority
     */
    public void setPriority(int priority) {
        setPriority0(nativePtr, priority);
    }
    
    private native void setPriority0(long ptr, int priority);
    
    /**
     * Get the timeout in seconds.
     * 
     * @return the timeout in seconds
     */
    public long getTimeoutSecs() {
        return getTimeoutSecs0(nativePtr);
    }
    
    private native long getTimeoutSecs0(long ptr);
    
    /**
     * Set the timeout in seconds.
     * 
     * @param timeoutSecs the timeout in seconds
     */
    public void setTimeoutSecs(long timeoutSecs) {
        setTimeoutSecs0(nativePtr, timeoutSecs);
    }
    
    private native void setTimeoutSecs0(long ptr, long timeoutSecs);
    
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
