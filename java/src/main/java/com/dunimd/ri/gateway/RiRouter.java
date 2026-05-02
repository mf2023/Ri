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
 * Router for managing API routes and matching requests to handlers.
 */
public class RiRouter {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRouter() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public void addRoute(RiRoute route) {
        addRoute0(nativePtr, route.getNativePtr());
    }
    
    private native void addRoute0(long ptr, long routePtr);
    
    public void addGetRoute(String path) {
        addGetRoute0(nativePtr, path);
    }
    
    private native void addGetRoute0(long ptr, String path);
    
    public void addPostRoute(String path) {
        addPostRoute0(nativePtr, path);
    }
    
    private native void addPostRoute0(long ptr, String path);
    
    public void addPutRoute(String path) {
        addPutRoute0(nativePtr, path);
    }
    
    private native void addPutRoute0(long ptr, String path);
    
    public void addDeleteRoute(String path) {
        addDeleteRoute0(nativePtr, path);
    }
    
    private native void addDeleteRoute0(long ptr, String path);
    
    public void addPatchRoute(String path) {
        addPatchRoute0(nativePtr, path);
    }
    
    private native void addPatchRoute0(long ptr, String path);
    
    public void addOptionsRoute(String path) {
        addOptionsRoute0(nativePtr, path);
    }
    
    private native void addOptionsRoute0(long ptr, String path);
    
    public void addCustomRoute(String method, String path) {
        addCustomRoute0(nativePtr, method, path);
    }
    
    private native void addCustomRoute0(long ptr, String method, String path);
    
    public int getRouteCount() {
        return getRouteCount0(nativePtr);
    }
    
    private native int getRouteCount0(long ptr);
    
    public void clearRoutes() {
        clearRoutes0(nativePtr);
    }
    
    private native void clearRoutes0(long ptr);
    
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
