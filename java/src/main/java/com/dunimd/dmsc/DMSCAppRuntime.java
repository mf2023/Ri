// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

package com.dunimd.dmsc;

/**
 * Application runtime for DMSC applications.
 * 
 * This class represents a running DMSC application instance.
 */
public class DMSCAppRuntime {
    private long nativePtr;
    
    DMSCAppRuntime(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Check if the application is running.
     * 
     * @return true if the application is running
     */
    public boolean isRunning() {
        return isRunning0(nativePtr);
    }
    
    private native boolean isRunning0(long ptr);
    
    /**
     * Shutdown the application.
     */
    public void shutdown() {
        shutdown0(nativePtr);
    }
    
    private native void shutdown0(long ptr);
    
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
