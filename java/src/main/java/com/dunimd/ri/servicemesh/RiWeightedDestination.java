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
 * Weighted destination for traffic routing in Ri service mesh.
 */
public class RiWeightedDestination {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiWeightedDestination(String service, int weight) {
        this.nativePtr = new0(service, weight);
    }
    
    private native long new0(String service, int weight);
    
    RiWeightedDestination(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getService() {
        return getService0(nativePtr);
    }
    
    private native String getService0(long ptr);
    
    public int getWeight() {
        return getWeight0(nativePtr);
    }
    
    private native int getWeight0(long ptr);
    
    public String getSubset() {
        return getSubset0(nativePtr);
    }
    
    private native String getSubset0(long ptr);
    
    public void setSubset(String subset) {
        setSubset0(nativePtr, subset);
    }
    
    private native void setSubset0(long ptr, String subset);
    
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
