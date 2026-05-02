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
 * Database migration for Ri.
 * 
 * Represents a database schema migration.
 */
public class RiDatabaseMigration {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDatabaseMigration(int version, String name, String sqlUp, String sqlDown) {
        this.nativePtr = new0(version, name, sqlUp, sqlDown);
    }
    
    private native long new0(int version, String name, String sqlUp, String sqlDown);
    
    /**
     * Get the migration version.
     * 
     * @return the version number
     */
    public int getVersion() {
        return getVersion0(nativePtr);
    }
    
    private native int getVersion0(long ptr);
    
    /**
     * Get the migration name.
     * 
     * @return the migration name
     */
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    /**
     * Get the SQL up statement.
     * 
     * @return the SQL up statement
     */
    public String getSqlUp() {
        return getSqlUp0(nativePtr);
    }
    
    private native String getSqlUp0(long ptr);
    
    /**
     * Get the SQL down statement.
     * 
     * @return the SQL down statement, or null if not available
     */
    public String getSqlDown() {
        return getSqlDown0(nativePtr);
    }
    
    private native String getSqlDown0(long ptr);
    
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
