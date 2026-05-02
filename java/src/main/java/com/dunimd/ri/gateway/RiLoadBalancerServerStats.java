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

package com.dunimd.ri.gateway;

import com.dunimd.ri.NativeLoader;

/**
 * Load balancer server statistics for monitoring and reporting.
 */
public class RiLoadBalancerServerStats {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiLoadBalancerServerStats(long activeConnections, long totalRequests, long failedRequests, long responseTimeMs) {
        this.nativePtr = new0(activeConnections, totalRequests, failedRequests, responseTimeMs);
    }
    
    private RiLoadBalancerServerStats(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(long activeConnections, long totalRequests, long failedRequests, long responseTimeMs);
    
    public long getActiveConnections() {
        return getActiveConnections0(nativePtr);
    }
    
    private native long getActiveConnections0(long ptr);
    
    public long getTotalRequests() {
        return getTotalRequests0(nativePtr);
    }
    
    private native long getTotalRequests0(long ptr);
    
    public long getFailedRequests() {
        return getFailedRequests0(nativePtr);
    }
    
    private native long getFailedRequests0(long ptr);
    
    public long getResponseTimeMs() {
        return getResponseTimeMs0(nativePtr);
    }
    
    private native long getResponseTimeMs0(long ptr);
    
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
