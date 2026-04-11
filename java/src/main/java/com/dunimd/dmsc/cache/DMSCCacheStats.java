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

package com.dunimd.ri.cache;

import com.dunimd.ri.NativeLoader;

/**
 * Cache statistics for Ri.
 */
public class RiCacheStats {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiCacheStats(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the number of cache hits.
     * 
     * @return the number of cache hits
     */
    public long getHits() {
        return getHits0(nativePtr);
    }
    
    private native long getHits0(long ptr);
    
    /**
     * Get the number of cache misses.
     * 
     * @return the number of cache misses
     */
    public long getMisses() {
        return getMisses0(nativePtr);
    }
    
    private native long getMisses0(long ptr);
    
    /**
     * Get the hit rate.
     * 
     * @return the hit rate (0.0 to 1.0)
     */
    public double getHitRate() {
        long hits = getHits();
        long misses = getMisses();
        long total = hits + misses;
        if (total == 0) {
            return 0.0;
        }
        return (double) hits / total;
    }
    
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
