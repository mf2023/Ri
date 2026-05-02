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

package com.dunimd.ri.servicemesh;

import com.dunimd.ri.NativeLoader;
import java.util.List;
import java.util.ArrayList;

/**
 * Traffic manager for Ri service mesh.
 */
public class RiTrafficManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiTrafficManager(boolean enabled) {
        this.nativePtr = new0(enabled);
    }
    
    private native long new0(boolean enabled);
    
    RiTrafficManager(long ptr) {
        this.nativePtr = ptr;
    }
    
    public void addRoute(RiTrafficRoute route) {
        addRoute0(nativePtr, route.getNativePtr());
    }
    
    private native void addRoute0(long ptr, long routePtr);
    
    public void removeRoute(String sourceService, String routeName) {
        removeRoute0(nativePtr, sourceService, routeName);
    }
    
    private native void removeRoute0(long ptr, String sourceService, String routeName);
    
    public List<RiTrafficRoute> getRoutes(String sourceService) {
        long[] ptrs = getRoutes0(nativePtr, sourceService);
        List<RiTrafficRoute> routes = new ArrayList<>();
        for (long ptr : ptrs) {
            routes.add(new RiTrafficRoute(ptr));
        }
        return routes;
    }
    
    private native long[] getRoutes0(long ptr, String sourceService);
    
    public void setCircuitBreakerConfig(String service, int consecutiveErrors, double maxEjectionPercent) {
        setCircuitBreakerConfig0(nativePtr, service, consecutiveErrors, maxEjectionPercent);
    }
    
    private native void setCircuitBreakerConfig0(long ptr, String service, int consecutiveErrors, double maxEjectionPercent);
    
    public void setRateLimitConfig(String service, int requestsPerSecond, int burstSize) {
        setRateLimitConfig0(nativePtr, service, requestsPerSecond, burstSize);
    }
    
    private native void setRateLimitConfig0(long ptr, String service, int requestsPerSecond, int burstSize);
    
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
