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
 * Represents a backend server in the load balancer.
 */
public class RiBackendServer {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiBackendServer(String id, String url) {
        this.nativePtr = new0(id, url);
    }
    
    RiBackendServer(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0(String id, String url);
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public String getUrl() {
        return getUrl0(nativePtr);
    }
    
    private native String getUrl0(long ptr);
    
    public int getWeight() {
        return getWeight0(nativePtr);
    }
    
    private native int getWeight0(long ptr);
    
    public void setWeight(int weight) {
        setWeight0(nativePtr, weight);
    }
    
    private native void setWeight0(long ptr, int weight);
    
    public int getMaxConnections() {
        return getMaxConnections0(nativePtr);
    }
    
    private native int getMaxConnections0(long ptr);
    
    public void setMaxConnections(int maxConnections) {
        setMaxConnections0(nativePtr, maxConnections);
    }
    
    private native void setMaxConnections0(long ptr, int maxConnections);
    
    public String getHealthCheckPath() {
        return getHealthCheckPath0(nativePtr);
    }
    
    private native String getHealthCheckPath0(long ptr);
    
    public void setHealthCheckPath(String path) {
        setHealthCheckPath0(nativePtr, path);
    }
    
    private native void setHealthCheckPath0(long ptr, String path);
    
    public boolean isHealthy() {
        return isHealthy0(nativePtr);
    }
    
    private native boolean isHealthy0(long ptr);
    
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
