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
 * Database row for Ri.
 * 
 * Represents a single row from a database query result.
 */
public class RiDBRow {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiDBRow(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get a string value by column name.
     * 
     * @param name the column name
     * @return the string value, or null if not found
     */
    public String getString(String name) {
        return getString0(nativePtr, name);
    }
    
    private native String getString0(long ptr, String name);
    
    /**
     * Get an integer value by column name.
     * 
     * @param name the column name
     * @return the integer value, or 0 if not found
     */
    public int getInt(String name) {
        return getInt0(nativePtr, name);
    }
    
    private native int getInt0(long ptr, String name);
    
    /**
     * Get a long value by column name.
     * 
     * @param name the column name
     * @return the long value, or 0 if not found
     */
    public long getLong(String name) {
        return getLong0(nativePtr, name);
    }
    
    private native long getLong0(long ptr, String name);
    
    /**
     * Get a double value by column name.
     * 
     * @param name the column name
     * @return the double value, or 0.0 if not found
     */
    public double getDouble(String name) {
        return getDouble0(nativePtr, name);
    }
    
    private native double getDouble0(long ptr, String name);
    
    /**
     * Get a boolean value by column name.
     * 
     * @param name the column name
     * @return the boolean value, or false if not found
     */
    public boolean getBoolean(String name) {
        return getBoolean0(nativePtr, name);
    }
    
    private native boolean getBoolean0(long ptr, String name);
    
    /**
     * Check if a column is null.
     * 
     * @param name the column name
     * @return true if the column is null
     */
    public boolean isNull(String name) {
        return isNull0(nativePtr, name);
    }
    
    private native boolean isNull0(long ptr, String name);
    
    /**
     * Check if a column exists.
     * 
     * @param name the column name
     * @return true if the column exists
     */
    public boolean hasColumn(String name) {
        return hasColumn0(nativePtr, name);
    }
    
    private native boolean hasColumn0(long ptr, String name);
    
    /**
     * Get the number of columns.
     * 
     * @return the column count
     */
    public int getColumnCount() {
        return getColumnCount0(nativePtr);
    }
    
    private native int getColumnCount0(long ptr);
    
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
