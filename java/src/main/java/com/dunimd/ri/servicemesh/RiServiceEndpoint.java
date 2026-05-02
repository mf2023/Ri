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
import java.util.Map;
import java.util.HashMap;

/**
 * Service endpoint representation for Ri service mesh.
 */
public class RiServiceEndpoint {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiServiceEndpoint(long ptr) {
        this.nativePtr = ptr;
    }
    
    public RiServiceEndpoint(String serviceName, String endpoint, int weight) {
        this.nativePtr = new0(serviceName, endpoint, weight);
    }
    
    private native long new0(String serviceName, String endpoint, int weight);
    
    public String getServiceName() {
        return getServiceName0(nativePtr);
    }
    
    private native String getServiceName0(long ptr);
    
    public String getEndpoint() {
        return getEndpoint0(nativePtr);
    }
    
    private native String getEndpoint0(long ptr);
    
    public int getWeight() {
        return getWeight0(nativePtr);
    }
    
    private native int getWeight0(long ptr);
    
    public RiServiceHealthStatus getHealthStatus() {
        int ordinal = getHealthStatus0(nativePtr);
        return RiServiceHealthStatus.values()[ordinal];
    }
    
    private native int getHealthStatus0(long ptr);
    
    public Map<String, String> getMetadata() {
        String[] keys = getMetadataKeys0(nativePtr);
        Map<String, String> metadata = new HashMap<>();
        for (String key : keys) {
            metadata.put(key, getMetadataValue0(nativePtr, key));
        }
        return metadata;
    }
    
    private native String[] getMetadataKeys0(long ptr);
    
    private native String getMetadataValue0(long ptr, String key);
    
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
