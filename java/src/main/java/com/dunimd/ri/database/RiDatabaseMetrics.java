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

package com.dunimd.ri.database;

import com.dunimd.ri.NativeLoader;

/**
 * Database metrics for Ri.
 * 
 * Provides metrics about database connection pool performance.
 */
public class RiDatabaseMetrics {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiDatabaseMetrics(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the number of active connections.
     * 
     * @return the active connection count
     */
    public long getActiveConnections() {
        return getActiveConnections0(nativePtr);
    }
    
    private native long getActiveConnections0(long ptr);
    
    /**
     * Get the number of idle connections.
     * 
     * @return the idle connection count
     */
    public long getIdleConnections() {
        return getIdleConnections0(nativePtr);
    }
    
    private native long getIdleConnections0(long ptr);
    
    /**
     * Get the total number of connections.
     * 
     * @return the total connection count
     */
    public long getTotalConnections() {
        return getTotalConnections0(nativePtr);
    }
    
    private native long getTotalConnections0(long ptr);
    
    /**
     * Get the number of queries executed.
     * 
     * @return the query count
     */
    public long getQueriesExecuted() {
        return getQueriesExecuted0(nativePtr);
    }
    
    private native long getQueriesExecuted0(long ptr);
    
    /**
     * Get the average query duration in milliseconds.
     * 
     * @return the average query duration
     */
    public double getQueryDurationMs() {
        return getQueryDurationMs0(nativePtr);
    }
    
    private native double getQueryDurationMs0(long ptr);
    
    /**
     * Get the number of errors.
     * 
     * @return the error count
     */
    public long getErrors() {
        return getErrors0(nativePtr);
    }
    
    private native long getErrors0(long ptr);
    
    /**
     * Get the utilization rate.
     * 
     * @return the utilization rate (0.0 to 1.0)
     */
    public double getUtilizationRate() {
        return getUtilizationRate0(nativePtr);
    }
    
    private native double getUtilizationRate0(long ptr);
    
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
