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
 * Match criteria for traffic routing in Ri service mesh.
 */
public class RiMatchCriteria {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiMatchCriteria() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    RiMatchCriteria(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getPathPrefix() {
        return getPathPrefix0(nativePtr);
    }
    
    private native String getPathPrefix0(long ptr);
    
    public void setPathPrefix(String pathPrefix) {
        setPathPrefix0(nativePtr, pathPrefix);
    }
    
    private native void setPathPrefix0(long ptr, String pathPrefix);
    
    public String getMethod() {
        return getMethod0(nativePtr);
    }
    
    private native String getMethod0(long ptr);
    
    public void setMethod(String method) {
        setMethod0(nativePtr, method);
    }
    
    private native void setMethod0(long ptr, String method);
    
    public Map<String, String> getHeaders() {
        String[] keys = getHeaderKeys0(nativePtr);
        Map<String, String> headers = new HashMap<>();
        for (String key : keys) {
            headers.put(key, getHeaderValue0(nativePtr, key));
        }
        return headers;
    }
    
    private native String[] getHeaderKeys0(long ptr);
    
    private native String getHeaderValue0(long ptr, String key);
    
    public void addHeader(String key, String value) {
        addHeader0(nativePtr, key, value);
    }
    
    private native void addHeader0(long ptr, String key, String value);
    
    public Map<String, String> getQueryParameters() {
        String[] keys = getQueryParamKeys0(nativePtr);
        Map<String, String> params = new HashMap<>();
        for (String key : keys) {
            params.put(key, getQueryParamValue0(nativePtr, key));
        }
        return params;
    }
    
    private native String[] getQueryParamKeys0(long ptr);
    
    private native String getQueryParamValue0(long ptr, String key);
    
    public void addQueryParameter(String key, String value) {
        addQueryParameter0(nativePtr, key, value);
    }
    
    private native void addQueryParameter0(long ptr, String key, String value);
    
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
