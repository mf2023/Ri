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

package com.dunimd.ri.auth;

import com.dunimd.ri.NativeLoader;
import java.util.Map;
import java.util.HashMap;

/**
 * Session for Ri.
 * 
 * Represents a user session with metadata and data storage.
 */
public class RiSession {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSession(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public String getUserId() {
        return getUserId0(nativePtr);
    }
    
    private native String getUserId0(long ptr);
    
    public long getCreatedAt() {
        return getCreatedAt0(nativePtr);
    }
    
    private native long getCreatedAt0(long ptr);
    
    public long getLastAccessed() {
        return getLastAccessed0(nativePtr);
    }
    
    private native long getLastAccessed0(long ptr);
    
    public long getExpiresAt() {
        return getExpiresAt0(nativePtr);
    }
    
    private native long getExpiresAt0(long ptr);
    
    public String getIpAddress() {
        return getIpAddress0(nativePtr);
    }
    
    private native String getIpAddress0(long ptr);
    
    public String getUserAgent() {
        return getUserAgent0(nativePtr);
    }
    
    private native String getUserAgent0(long ptr);
    
    public boolean isExpired() {
        return isExpired0(nativePtr);
    }
    
    private native boolean isExpired0(long ptr);
    
    public String getData(String key) {
        return getData0(nativePtr, key);
    }
    
    private native String getData0(long ptr, String key);
    
    public void setData(String key, String value) {
        setData0(nativePtr, key, value);
    }
    
    private native void setData0(long ptr, String key, String value);
    
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
