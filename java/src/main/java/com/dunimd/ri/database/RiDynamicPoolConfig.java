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

package com.dunimd.ri.database;

import com.dunimd.ri.NativeLoader;

/**
 * Dynamic pool configuration for Ri.
 * 
 * Configuration for dynamic connection pool scaling.
 */
public class RiDynamicPoolConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDynamicPoolConfig() {
        this.nativePtr = new0();
    }
    
    RiDynamicPoolConfig(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Check if dynamic scaling is enabled.
     * 
     * @return true if dynamic scaling is enabled
     */
    public boolean isDynamicScalingEnabled() {
        return isDynamicScalingEnabled0(nativePtr);
    }
    
    private native boolean isDynamicScalingEnabled0(long ptr);
    
    /**
     * Set whether dynamic scaling is enabled.
     * 
     * @param enabled true to enable dynamic scaling
     */
    public void setDynamicScalingEnabled(boolean enabled) {
        setDynamicScalingEnabled0(nativePtr, enabled);
    }
    
    private native void setDynamicScalingEnabled0(long ptr, boolean enabled);
    
    /**
     * Get the scale-up threshold.
     * 
     * @return the scale-up threshold (0.0 to 1.0)
     */
    public double getScaleUpThreshold() {
        return getScaleUpThreshold0(nativePtr);
    }
    
    private native double getScaleUpThreshold0(long ptr);
    
    /**
     * Set the scale-up threshold.
     * 
     * @param threshold the scale-up threshold (0.0 to 1.0)
     */
    public void setScaleUpThreshold(double threshold) {
        setScaleUpThreshold0(nativePtr, threshold);
    }
    
    private native void setScaleUpThreshold0(long ptr, double threshold);
    
    /**
     * Get the scale-down threshold.
     * 
     * @return the scale-down threshold (0.0 to 1.0)
     */
    public double getScaleDownThreshold() {
        return getScaleDownThreshold0(nativePtr);
    }
    
    private native double getScaleDownThreshold0(long ptr);
    
    /**
     * Set the scale-down threshold.
     * 
     * @param threshold the scale-down threshold (0.0 to 1.0)
     */
    public void setScaleDownThreshold(double threshold) {
        setScaleDownThreshold0(nativePtr, threshold);
    }
    
    private native void setScaleDownThreshold0(long ptr, double threshold);
    
    /**
     * Get the minimum connections.
     * 
     * @return the minimum connections
     */
    public int getMinConnections() {
        return getMinConnections0(nativePtr);
    }
    
    private native int getMinConnections0(long ptr);
    
    /**
     * Set the minimum connections.
     * 
     * @param min the minimum connections
     */
    public void setMinConnections(int min) {
        setMinConnections0(nativePtr, min);
    }
    
    private native void setMinConnections0(long ptr, int min);
    
    /**
     * Get the maximum connections.
     * 
     * @return the maximum connections
     */
    public int getMaxConnections() {
        return getMaxConnections0(nativePtr);
    }
    
    private native int getMaxConnections0(long ptr);
    
    /**
     * Set the maximum connections.
     * 
     * @param max the maximum connections
     */
    public void setMaxConnections(int max) {
        setMaxConnections0(nativePtr, max);
    }
    
    private native void setMaxConnections0(long ptr, int max);
    
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
