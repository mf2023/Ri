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

package com.dunimd.ri;

/**
 * Log analytics module for Ri.
 * 
 * This module implements analytics functionality by listening to hook events
 * and generating comprehensive reports.
 */
public class RiLogAnalyticsModule {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiLogAnalyticsModule.
     */
    public RiLogAnalyticsModule() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get the name of the analytics module.
     * 
     * @return the module name
     */
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    /**
     * Check if the analytics module is critical.
     * 
     * @return false, as the analytics module is non-critical
     */
    public boolean isCritical() {
        return isCritical0(nativePtr);
    }
    
    private native boolean isCritical0(long ptr);
    
    /**
     * Check if the analytics module is enabled.
     * 
     * @return true if enabled
     */
    public boolean isEnabled() {
        return isEnabled0(nativePtr);
    }
    
    private native boolean isEnabled0(long ptr);
    
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
