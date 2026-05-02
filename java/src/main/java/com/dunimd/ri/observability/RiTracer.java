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

package com.dunimd.ri.observability;

import com.dunimd.ri.NativeLoader;

/**
 * Distributed tracer for creating and managing spans.
 */
public class RiTracer {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiTracer(double samplingRate) {
        this.nativePtr = new0(samplingRate);
    }
    
    private native long new0(double samplingRate);
    
    public String startTrace(String name) {
        return startTrace0(nativePtr, name);
    }
    
    private native String startTrace0(long ptr, String name);
    
    public String span(String name, String kind) {
        return span0(nativePtr, name, kind);
    }
    
    private native String span0(long ptr, String name, String kind);
    
    public void finishSpan(String spanId, String status) {
        finishSpan0(nativePtr, spanId, status);
    }
    
    private native void finishSpan0(long ptr, String spanId, String status);
    
    public void setAttribute(String spanId, String key, String value) {
        setAttribute0(nativePtr, spanId, key, value);
    }
    
    private native void setAttribute0(long ptr, String spanId, String key, String value);
    
    public void addEvent(String spanId, String name) {
        addEvent0(nativePtr, spanId, name);
    }
    
    private native void addEvent0(long ptr, String spanId, String name);
    
    public int getActiveTraceCount() {
        return getActiveTraceCount0(nativePtr);
    }
    
    private native int getActiveTraceCount0(long ptr);
    
    public int getActiveSpanCount() {
        return getActiveSpanCount0(nativePtr);
    }
    
    private native int getActiveSpanCount0(long ptr);
    
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
