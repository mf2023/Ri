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
 * Database connection pool for Ri.
 * 
 * Provides connection pooling with dynamic scaling capabilities.
 */
public class RiDatabasePool {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDatabasePool(RiDatabaseConfig config) {
        this.nativePtr = new0(config.getNativePtr());
    }
    
    private native long new0(long configPtr);
    
    /**
     * Execute a SQL statement.
     * 
     * @param sql the SQL statement
     * @return number of rows affected
     */
    public long execute(String sql) {
        return execute0(nativePtr, sql);
    }
    
    private native long execute0(long ptr, String sql);
    
    /**
     * Execute a SQL query.
     * 
     * @param sql the SQL query
     * @return the query result
     */
    public RiDBResult query(String sql) {
        long resultPtr = query0(nativePtr, sql);
        return new RiDBResult(resultPtr);
    }
    
    private native long query0(long ptr, String sql);
    
    /**
     * Get pool metrics.
     * 
     * @return the pool metrics
     */
    public RiDatabaseMetrics getMetrics() {
        long metricsPtr = getMetrics0(nativePtr);
        return new RiDatabaseMetrics(metricsPtr);
    }
    
    private native long getMetrics0(long ptr);
    
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
     * Get the dynamic pool configuration.
     * 
     * @return the dynamic pool configuration
     */
    public RiDynamicPoolConfig getDynamicConfig() {
        long configPtr = getDynamicConfig0(nativePtr);
        return new RiDynamicPoolConfig(configPtr);
    }
    
    private native long getDynamicConfig0(long ptr);
    
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
