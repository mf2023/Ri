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
 * System metrics collector for monitoring CPU, memory, disk, and network.
 */
public class RiSystemMetricsCollector {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSystemMetricsCollector() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public RiSystemMetrics collect() {
        return collect0(nativePtr);
    }
    
    private native RiSystemMetrics collect0(long ptr);
    
    public void refresh() {
        refresh0(nativePtr);
    }
    
    private native void refresh0(long ptr);
    
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
