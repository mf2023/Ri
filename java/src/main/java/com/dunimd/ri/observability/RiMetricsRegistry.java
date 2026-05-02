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
 * Metrics registry for managing multiple metrics.
 */
public class RiMetricsRegistry {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiMetricsRegistry() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public void register(RiMetric metric) {
        register0(nativePtr, metric.getNativePtr());
    }
    
    private native void register0(long ptr, long metricPtr);
    
    public RiMetric get(String name) {
        long metricPtr = get0(nativePtr, name);
        if (metricPtr == 0) {
            return null;
        }
        return new RiMetric(metricPtr);
    }
    
    private native long get0(long ptr, String name);
    
    public double getMetricValue(String name) {
        return getMetricValue0(nativePtr, name);
    }
    
    private native double getMetricValue0(long ptr, String name);
    
    public String[] getAllMetricNames() {
        return getAllMetricNames0(nativePtr);
    }
    
    private native String[] getAllMetricNames0(long ptr);
    
    public String exportPrometheus() {
        return exportPrometheus0(nativePtr);
    }
    
    private native String exportPrometheus0(long ptr);
    
    public int getMetricCount() {
        return getMetricCount0(nativePtr);
    }
    
    private native int getMetricCount0(long ptr);
    
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
    
    private RiMetric(long ptr) {
        this.nativePtr = ptr;
    }
}
