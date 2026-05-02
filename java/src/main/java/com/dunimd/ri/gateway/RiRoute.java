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
 * Represents a single API route with method and path.
 */
public class RiRoute {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRoute(String method, String path) {
        this.nativePtr = new0(method, path);
    }
    
    private native long new0(String method, String path);
    
    public String getMethod() {
        return getMethod0(nativePtr);
    }
    
    private native String getMethod0(long ptr);
    
    public String getPath() {
        return getPath0(nativePtr);
    }
    
    private native String getPath0(long ptr);
    
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
