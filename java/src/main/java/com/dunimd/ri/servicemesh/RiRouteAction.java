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
 * Route action for traffic routing in Ri service mesh.
 */
public class RiRouteAction {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public static final int TYPE_ROUTE = 0;
    public static final int TYPE_REDIRECT = 1;
    public static final int TYPE_DIRECT_RESPONSE = 2;
    
    private RiRouteAction(long ptr) {
        this.nativePtr = ptr;
    }
    
    public static RiRouteAction route(List<RiWeightedDestination> destinations) {
        long[] ptrs = new long[destinations.size()];
        for (int i = 0; i < destinations.size(); i++) {
            ptrs[i] = destinations.get(i).getNativePtr();
        }
        return new RiRouteAction(route0(ptrs));
    }
    
    private static native long route0(long[] destinationPtrs);
    
    public static RiRouteAction redirect(String uri) {
        return new RiRouteAction(redirect0(uri));
    }
    
    private static native long redirect0(String uri);
    
    public static RiRouteAction directResponse(int statusCode, String body) {
        return new RiRouteAction(directResponse0(statusCode, body));
    }
    
    private static native long directResponse0(int statusCode, String body);
    
    public int getType() {
        return getType0(nativePtr);
    }
    
    private native int getType0(long ptr);
    
    public List<RiWeightedDestination> getDestinations() {
        long[] ptrs = getDestinations0(nativePtr);
        List<RiWeightedDestination> destinations = new ArrayList<>();
        for (long ptr : ptrs) {
            destinations.add(new RiWeightedDestination(ptr));
        }
        return destinations;
    }
    
    private native long[] getDestinations0(long ptr);
    
    public String getRedirectUri() {
        return getRedirectUri0(nativePtr);
    }
    
    private native String getRedirectUri0(long ptr);
    
    public int getDirectResponseStatusCode() {
        return getDirectResponseStatusCode0(nativePtr);
    }
    
    private native int getDirectResponseStatusCode0(long ptr);
    
    public String getDirectResponseBody() {
        return getDirectResponseBody0(nativePtr);
    }
    
    private native String getDirectResponseBody0(long ptr);
    
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
