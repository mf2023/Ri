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

package com.dunimd.ri.observability;

import com.dunimd.ri.NativeLoader;

/**
 * Configuration for creating metrics.
 */
public class RiMetricConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiMetricConfig(String name, RiMetricType metricType) {
        this.nativePtr = new0(name, metricType.ordinal());
    }
    
    private native long new0(String name, int metricType);
    
    long getNativePtr() {
        return nativePtr;
    }
    
    public RiMetricConfig setHelp(String help) {
        setHelp0(nativePtr, help);
        return this;
    }
    
    private native void setHelp0(long ptr, String help);
    
    public RiMetricConfig setBuckets(double[] buckets) {
        setBuckets0(nativePtr, buckets);
        return this;
    }
    
    private native void setBuckets0(long ptr, double[] buckets);
    
    public RiMetricConfig setQuantiles(double[] quantiles) {
        setQuantiles0(nativePtr, quantiles);
        return this;
    }
    
    private native void setQuantiles0(long ptr, double[] quantiles);
    
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
