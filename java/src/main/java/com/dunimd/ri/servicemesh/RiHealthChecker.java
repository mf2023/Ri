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
 * Health checker for Ri service mesh.
 */
public class RiHealthChecker {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiHealthChecker(long checkIntervalSeconds) {
        this.nativePtr = new0(checkIntervalSeconds);
    }
    
    private native long new0(long checkIntervalSeconds);
    
    public void startHealthCheck(String serviceName, String endpoint) {
        startHealthCheck0(nativePtr, serviceName, endpoint);
    }
    
    private native void startHealthCheck0(long ptr, String serviceName, String endpoint);
    
    public void startHealthCheckWithType(String serviceName, String endpoint, RiHealthCheckType checkType) {
        startHealthCheckWithType0(nativePtr, serviceName, endpoint, checkType.ordinal());
    }
    
    private native void startHealthCheckWithType0(long ptr, String serviceName, String endpoint, int checkTypeOrdinal);
    
    public void stopHealthCheck(String serviceName, String endpoint) {
        stopHealthCheck0(nativePtr, serviceName, endpoint);
    }
    
    private native void stopHealthCheck0(long ptr, String serviceName, String endpoint);
    
    public RiHealthSummary getServiceHealthSummary(String serviceName) {
        long ptr = getServiceHealthSummary0(nativePtr, serviceName);
        return new RiHealthSummary(ptr);
    }
    
    private native long getServiceHealthSummary0(long ptr, String serviceName);
    
    public RiHealthSummary check(String serviceName) {
        return getServiceHealthSummary(serviceName);
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
