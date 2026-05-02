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
 * Device controller for Ri.
 * 
 * Manages devices and their resources.
 */
public class RiDeviceController {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeviceController() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get all devices.
     * 
     * @return all devices
     */
    public List<RiDevice> getAllDevices() {
        long[] ptrs = getAllDevices0(nativePtr);
        List<RiDevice> devices = new ArrayList<>();
        for (long ptr : ptrs) {
            devices.add(new RiDevice(ptr));
        }
        return devices;
    }
    
    private native long[] getAllDevices0(long ptr);
    
    /**
     * Get a device by ID.
     * 
     * @param deviceId the device ID
     * @return the device, or null if not found
     */
    public RiDevice getDevice(String deviceId) {
        long ptr = getDevice0(nativePtr, deviceId);
        if (ptr == 0) {
            return null;
        }
        return new RiDevice(ptr);
    }
    
    private native long getDevice0(long ptr, String deviceId);
    
    /**
     * Add a device.
     * 
     * @param device the device to add
     */
    public void addDevice(RiDevice device) {
        addDevice0(nativePtr, device.getNativePtr());
    }
    
    private native void addDevice0(long ptr, long devicePtr);
    
    /**
     * Remove a device.
     * 
     * @param deviceId the device ID
     * @return true if the device was removed
     */
    public boolean removeDevice(String deviceId) {
        return removeDevice0(nativePtr, deviceId);
    }
    
    private native boolean removeDevice0(long ptr, String deviceId);
    
    /**
     * Get the device count.
     * 
     * @return the device count
     */
    public int getDeviceCount() {
        return getDeviceCount0(nativePtr);
    }
    
    private native int getDeviceCount0(long ptr);
    
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
