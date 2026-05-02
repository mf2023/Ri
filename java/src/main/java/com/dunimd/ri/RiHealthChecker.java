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
 * Health checker service that manages and executes health checks.
 */
public class RiHealthChecker {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiHealthChecker with default configuration.
     */
    public RiHealthChecker() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Create a new RiHealthChecker with custom configuration.
     * 
     * @param config the health check configuration
     */
    public RiHealthChecker(RiHealthCheckConfig config) {
        this.nativePtr = withConfig0(config.getNativePtr());
    }
    
    private native long withConfig0(long configPtr);
    
    /**
     * Get the number of registered health checks.
     * 
     * @return the check count
     */
    public int getCheckCount() {
        return getCheckCount0(nativePtr);
    }
    
    private native int getCheckCount0(long ptr);
    
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
