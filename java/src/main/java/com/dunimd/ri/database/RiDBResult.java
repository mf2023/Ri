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
 * Database result for Ri.
 * 
 * Represents the result of a database query.
 */
public class RiDBResult {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiDBResult(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the number of rows.
     * 
     * @return the row count
     */
    public int getRowCount() {
        return getRowCount0(nativePtr);
    }
    
    private native int getRowCount0(long ptr);
    
    /**
     * Get the number of affected rows.
     * 
     * @return the affected row count
     */
    public long getAffectedRows() {
        return getAffectedRows0(nativePtr);
    }
    
    private native long getAffectedRows0(long ptr);
    
    /**
     * Get the last insert ID.
     * 
     * @return the last insert ID, or -1 if not available
     */
    public long getLastInsertId() {
        return getLastInsertId0(nativePtr);
    }
    
    private native long getLastInsertId0(long ptr);
    
    /**
     * Check if the result is empty.
     * 
     * @return true if the result is empty
     */
    public boolean isEmpty() {
        return isEmpty0(nativePtr);
    }
    
    private native boolean isEmpty0(long ptr);
    
    /**
     * Get a row by index.
     * 
     * @param index the row index
     * @return the row, or null if index is out of bounds
     */
    public RiDBRow getRow(int index) {
        long rowPtr = getRow0(nativePtr, index);
        if (rowPtr == 0) {
            return null;
        }
        return new RiDBRow(rowPtr);
    }
    
    private native long getRow0(long ptr, int index);
    
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
