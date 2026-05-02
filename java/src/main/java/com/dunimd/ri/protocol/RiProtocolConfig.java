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
 * Protocol configuration for Ri.
 */
public class RiProtocolConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiProtocolConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get the default protocol type.
     * 
     * @return the default protocol type
     */
    public RiProtocolType getDefaultProtocol() {
        int ordinal = getDefaultProtocol0(nativePtr);
        return RiProtocolType.values()[ordinal];
    }
    
    private native int getDefaultProtocol0(long ptr);
    
    /**
     * Set the default protocol type.
     * 
     * @param protocolType the default protocol type
     */
    public void setDefaultProtocol(RiProtocolType protocolType) {
        setDefaultProtocol0(nativePtr, protocolType.ordinal());
    }
    
    private native void setDefaultProtocol0(long ptr, int protocolType);
    
    /**
     * Check if security is enabled.
     * 
     * @return true if security is enabled
     */
    public boolean isSecurityEnabled() {
        return isSecurityEnabled0(nativePtr);
    }
    
    private native boolean isSecurityEnabled0(long ptr);
    
    /**
     * Set security enabled.
     * 
     * @param enabled whether security is enabled
     */
    public void setSecurityEnabled(boolean enabled) {
        setSecurityEnabled0(nativePtr, enabled);
    }
    
    private native void setSecurityEnabled0(long ptr, boolean enabled);
    
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
     * Set the security level.
     * 
     * @param level the security level
     */
    public void setSecurityLevel(RiSecurityLevel level) {
        setSecurityLevel0(nativePtr, level.ordinal());
    }
    
    private native void setSecurityLevel0(long ptr, int level);
    
    /**
     * Check if state sync is enabled.
     * 
     * @return true if state sync is enabled
     */
    public boolean isStateSyncEnabled() {
        return isStateSyncEnabled0(nativePtr);
    }
    
    private native boolean isStateSyncEnabled0(long ptr);
    
    /**
     * Set state sync enabled.
     * 
     * @param enabled whether state sync is enabled
     */
    public void setStateSyncEnabled(boolean enabled) {
        setStateSyncEnabled0(nativePtr, enabled);
    }
    
    private native void setStateSyncEnabled0(long ptr, boolean enabled);
    
    /**
     * Check if performance optimization is enabled.
     * 
     * @return true if performance optimization is enabled
     */
    public boolean isPerformanceOptimization() {
        return isPerformanceOptimization0(nativePtr);
    }
    
    private native boolean isPerformanceOptimization0(long ptr);
    
    /**
     * Set performance optimization enabled.
     * 
     * @param enabled whether performance optimization is enabled
     */
    public void setPerformanceOptimization(boolean enabled) {
        setPerformanceOptimization0(nativePtr, enabled);
    }
    
    private native void setPerformanceOptimization0(long ptr, boolean enabled);
    
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
