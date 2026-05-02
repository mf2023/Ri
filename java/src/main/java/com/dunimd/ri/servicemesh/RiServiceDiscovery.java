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
import java.util.Map;

/**
 * Service discovery for Ri service mesh.
 */
public class RiServiceDiscovery {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiServiceDiscovery(boolean enabled) {
        this.nativePtr = new0(enabled);
    }
    
    private native long new0(boolean enabled);
    
    public String register(String serviceName, String host, int port, Map<String, String> metadata) {
        String[] keys = metadata != null ? metadata.keySet().toArray(new String[0]) : new String[0];
        String[] values = new String[keys.length];
        if (metadata != null) {
            for (int i = 0; i < keys.length; i++) {
                values[i] = metadata.get(keys[i]);
            }
        }
        return register0(nativePtr, serviceName, host, port, keys, values);
    }
    
    private native String register0(long ptr, String serviceName, String host, int port, String[] keys, String[] values);
    
    public void deregister(String instanceId) {
        deregister0(nativePtr, instanceId);
    }
    
    private native void deregister0(long ptr, String instanceId);
    
    public List<RiServiceInstance> discover(String serviceName) {
        long[] ptrs = discover0(nativePtr, serviceName);
        List<RiServiceInstance> instances = new ArrayList<>();
        for (long ptr : ptrs) {
            instances.add(new RiServiceInstance(ptr));
        }
        return instances;
    }
    
    private native long[] discover0(long ptr, String serviceName);
    
    public void updateHeartbeat(String instanceId) {
        updateHeartbeat0(nativePtr, instanceId);
    }
    
    private native void updateHeartbeat0(long ptr, String instanceId);
    
    public void setServiceStatus(String instanceId, RiServiceStatus status) {
        setServiceStatus0(nativePtr, instanceId, status.ordinal());
    }
    
    private native void setServiceStatus0(long ptr, String instanceId, int statusOrdinal);
    
    public List<String> getAllServices() {
        return getAllServices0(nativePtr);
    }
    
    private native List<String> getAllServices0(long ptr);
    
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
