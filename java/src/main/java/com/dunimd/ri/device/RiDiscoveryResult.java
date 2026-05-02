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
import java.util.ArrayList;
import java.util.List;

/**
 * Discovery result for Ri.
 * 
 * Contains information about the results of a device discovery scan.
 */
public class RiDiscoveryResult {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiDiscoveryResult(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the discovered devices.
     * 
     * @return the discovered devices
     */
    public List<RiDevice> getDiscoveredDevices() {
        long[] ptrs = getDiscoveredDevices0(nativePtr);
        List<RiDevice> devices = new ArrayList<>();
        for (long ptr : ptrs) {
            devices.add(new RiDevice(ptr));
        }
        return devices;
    }
    
    private native long[] getDiscoveredDevices0(long ptr);
    
    /**
     * Get the updated devices.
     * 
     * @return the updated devices
     */
    public List<RiDevice> getUpdatedDevices() {
        long[] ptrs = getUpdatedDevices0(nativePtr);
        List<RiDevice> devices = new ArrayList<>();
        for (long ptr : ptrs) {
            devices.add(new RiDevice(ptr));
        }
        return devices;
    }
    
    private native long[] getUpdatedDevices0(long ptr);
    
    /**
     * Get the removed device IDs.
     * 
     * @return the removed device IDs
     */
    public List<String> getRemovedDevices() {
        return getRemovedDevices0(nativePtr);
    }
    
    private native List<String> getRemovedDevices0(long ptr);
    
    /**
     * Get the total number of devices.
     * 
     * @return the total number of devices
     */
    public int getTotalDevices() {
        return getTotalDevices0(nativePtr);
    }
    
    private native int getTotalDevices0(long ptr);
    
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
