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
 * Comprehensive health report containing status of all components.
 */
public class RiHealthReport {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new empty RiHealthReport.
     */
    public RiHealthReport() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get the overall health status.
     * 
     * @return the overall status
     */
    public RiHealthStatus getOverallStatus() {
        int statusValue = getOverallStatus0(nativePtr);
        return RiHealthStatus.fromValue(statusValue);
    }
    
    private native int getOverallStatus0(long ptr);
    
    /**
     * Get the total number of components checked.
     * 
     * @return the total components count
     */
    public int getTotalComponents() {
        return getTotalComponents0(nativePtr);
    }
    
    private native int getTotalComponents0(long ptr);
    
    /**
     * Get the number of healthy components.
     * 
     * @return the healthy count
     */
    public int getHealthyCount() {
        return getHealthyCount0(nativePtr);
    }
    
    private native int getHealthyCount0(long ptr);
    
    /**
     * Get the number of degraded components.
     * 
     * @return the degraded count
     */
    public int getDegradedCount() {
        return getDegradedCount0(nativePtr);
    }
    
    private native int getDegradedCount0(long ptr);
    
    /**
     * Get the number of unhealthy components.
     * 
     * @return the unhealthy count
     */
    public int getUnhealthyCount() {
        return getUnhealthyCount0(nativePtr);
    }
    
    private native int getUnhealthyCount0(long ptr);
    
    /**
     * Get the number of unknown components.
     * 
     * @return the unknown count
     */
    public int getUnknownCount() {
        return getUnknownCount0(nativePtr);
    }
    
    private native int getUnknownCount0(long ptr);
    
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
