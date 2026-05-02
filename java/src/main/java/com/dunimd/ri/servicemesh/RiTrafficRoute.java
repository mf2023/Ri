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

/**
 * Traffic route for Ri service mesh.
 */
public class RiTrafficRoute {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiTrafficRoute(String name, String sourceService, String destinationService) {
        this.nativePtr = new0(name, sourceService, destinationService);
    }
    
    private native long new0(String name, String sourceService, String destinationService);
    
    RiTrafficRoute(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    public String getSourceService() {
        return getSourceService0(nativePtr);
    }
    
    private native String getSourceService0(long ptr);
    
    public String getDestinationService() {
        return getDestinationService0(nativePtr);
    }
    
    private native String getDestinationService0(long ptr);
    
    public RiMatchCriteria getMatchCriteria() {
        long ptr = getMatchCriteria0(nativePtr);
        return new RiMatchCriteria(ptr);
    }
    
    private native long getMatchCriteria0(long ptr);
    
    public void setMatchCriteria(RiMatchCriteria criteria) {
        setMatchCriteria0(nativePtr, criteria.getNativePtr());
    }
    
    private native void setMatchCriteria0(long ptr, long criteriaPtr);
    
    public RiRouteAction getRouteAction() {
        long ptr = getRouteAction0(nativePtr);
        return new RiRouteAction(ptr);
    }
    
    private native long getRouteAction0(long ptr);
    
    public void setRouteAction(RiRouteAction action) {
        setRouteAction0(nativePtr, action.getNativePtr());
    }
    
    private native void setRouteAction0(long ptr, long actionPtr);
    
    public long getTimeoutMs() {
        return getTimeoutMs0(nativePtr);
    }
    
    private native long getTimeoutMs0(long ptr);
    
    public void setTimeoutMs(long timeoutMs) {
        setTimeoutMs0(nativePtr, timeoutMs);
    }
    
    private native void setTimeoutMs0(long ptr, long timeoutMs);
    
    public int getRetryAttempts() {
        return getRetryAttempts0(nativePtr);
    }
    
    private native int getRetryAttempts0(long ptr);
    
    public void setRetryAttempts(int attempts) {
        setRetryAttempts0(nativePtr, attempts);
    }
    
    private native void setRetryAttempts0(long ptr, int attempts);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
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
