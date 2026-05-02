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

package com.dunimd.ri.protocol;

import com.dunimd.ri.NativeLoader;

/**
 * Connection information for Ri.
 */
public class RiConnectionInfo {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiConnectionInfo(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the connection ID.
     * 
     * @return the connection ID
     */
    public String getConnectionId() {
        return getConnectionId0(nativePtr);
    }
    
    private native String getConnectionId0(long ptr);
    
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
     * Get the address.
     * 
     * @return the address
     */
    public String getAddress() {
        return getAddress0(nativePtr);
    }
    
    private native String getAddress0(long ptr);
    
    /**
     * Get the protocol type.
     * 
     * @return the protocol type
     */
    public RiProtocolType getProtocolType() {
        int ordinal = getProtocolType0(nativePtr);
        return RiProtocolType.values()[ordinal];
    }
    
    private native int getProtocolType0(long ptr);
    
    /**
     * Get the connection state.
     * 
     * @return the connection state
     */
    public RiConnectionState getState() {
        int ordinal = getState0(nativePtr);
        return RiConnectionState.values()[ordinal];
    }
    
    private native int getState0(long ptr);
    
    /**
     * Get the security level.
     * 
     * @return the security level
     */
    public RiSecurityLevel getSecurityLevel() {
        int ordinal = getSecurityLevel0(nativePtr);
        return RiSecurityLevel.values()[ordinal];
    }
    
    private native int getSecurityLevel0(long ptr);
    
    /**
     * Get the connected at timestamp.
     * 
     * @return the connected at timestamp
     */
    public long getConnectedAt() {
        return getConnectedAt0(nativePtr);
    }
    
    private native long getConnectedAt0(long ptr);
    
    /**
     * Get the last activity timestamp.
     * 
     * @return the last activity timestamp
     */
    public long getLastActivity() {
        return getLastActivity0(nativePtr);
    }
    
    private native long getLastActivity0(long ptr);
    
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
