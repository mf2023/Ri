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

package com.dunimd.ri.protocol;

import com.dunimd.ri.NativeLoader;

/**
 * Connection statistics for Ri.
 */
public class RiConnectionStats {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiConnectionStats(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the total connections.
     * 
     * @return the total connections
     */
    public long getTotalConnections() {
        return getTotalConnections0(nativePtr);
    }
    
    private native long getTotalConnections0(long ptr);
    
    /**
     * Get the active connections.
     * 
     * @return the active connections
     */
    public long getActiveConnections() {
        return getActiveConnections0(nativePtr);
    }
    
    private native long getActiveConnections0(long ptr);
    
    /**
     * Get the total bytes sent.
     * 
     * @return the total bytes sent
     */
    public long getBytesSent() {
        return getBytesSent0(nativePtr);
    }
    
    private native long getBytesSent0(long ptr);
    
    /**
     * Get the total bytes received.
     * 
     * @return the total bytes received
     */
    public long getBytesReceived() {
        return getBytesReceived0(nativePtr);
    }
    
    private native long getBytesReceived0(long ptr);
    
    /**
     * Get the connection duration in seconds.
     * 
     * @return the connection duration in seconds
     */
    public long getConnectionDurationSecs() {
        return getConnectionDurationSecs0(nativePtr);
    }
    
    private native long getConnectionDurationSecs0(long ptr);
    
    /**
     * Release native resources.
     */
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
