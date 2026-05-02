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
 * Main load balancer implementation.
 */
public class RiLoadBalancer {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiLoadBalancer(RiLoadBalancerStrategy strategy) {
        this.nativePtr = new0(strategy.ordinal());
    }
    
    private native long new0(int strategy);
    
    public void addServer(RiBackendServer server) {
        addServer0(nativePtr, server.getNativePtr());
    }
    
    private native void addServer0(long ptr, long serverPtr);
    
    public boolean removeServer(String serverId) {
        return removeServer0(nativePtr, serverId);
    }
    
    private native boolean removeServer0(long ptr, String serverId);
    
    public RiBackendServer selectBackend(String clientIp) {
        long serverPtr = selectBackend0(nativePtr, clientIp);
        if (serverPtr == 0) {
            return null;
        }
        return new RiBackendServer(serverPtr);
    }
    
    private native long selectBackend0(long ptr, String clientIp);
    
    public void releaseServer(String serverId) {
        releaseServer0(nativePtr, serverId);
    }
    
    private native void releaseServer0(long ptr, String serverId);
    
    public void recordServerFailure(String serverId) {
        recordServerFailure0(nativePtr, serverId);
    }
    
    private native void recordServerFailure0(long ptr, String serverId);
    
    public void recordResponseTime(String serverId, long responseTimeMs) {
        recordResponseTime0(nativePtr, serverId, responseTimeMs);
    }
    
    private native void recordResponseTime0(long ptr, String serverId, long responseTimeMs);
    
    public RiLoadBalancerServerStats getServerStats(String serverId) {
        long statsPtr = getServerStats0(nativePtr, serverId);
        if (statsPtr == 0) {
            return null;
        }
        return new RiLoadBalancerServerStats(statsPtr);
    }
    
    private native long getServerStats0(long ptr, String serverId);
    
    public void markServerHealthy(String serverId, boolean healthy) {
        markServerHealthy0(nativePtr, serverId, healthy);
    }
    
    private native void markServerHealthy0(long ptr, String serverId, boolean healthy);
    
    public boolean performHealthCheck(String serverId) {
        return performHealthCheck0(nativePtr, serverId);
    }
    
    private native boolean performHealthCheck0(long ptr, String serverId);
    
    public int getServerCount() {
        return getServerCount0(nativePtr);
    }
    
    private native int getServerCount0(long ptr);
    
    public int getHealthyServerCount() {
        return getHealthyServerCount0(nativePtr);
    }
    
    private native int getHealthyServerCount0(long ptr);
    
    public RiLoadBalancerStrategy getStrategy() {
        int strategy = getStrategy0(nativePtr);
        RiLoadBalancerStrategy[] values = RiLoadBalancerStrategy.values();
        if (strategy >= 0 && strategy < values.length) {
            return values[strategy];
        }
        return RiLoadBalancerStrategy.ROUND_ROBIN;
    }
    
    private native int getStrategy0(long ptr);
    
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
