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

/**
 * Session manager for Ri.
 * 
 * Manages user sessions with creation, retrieval, and cleanup.
 */
public class RiSessionManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSessionManager(long timeoutSecs) {
        this.nativePtr = new0(timeoutSecs);
    }
    
    private native long new0(long timeoutSecs);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String createSession(String userId, String ipAddress, String userAgent) {
        return createSession0(nativePtr, userId, ipAddress, userAgent);
    }
    
    private native String createSession0(long ptr, String userId, String ipAddress, String userAgent);
    
    public RiSession getSession(String sessionId) {
        long sessionPtr = getSession0(nativePtr, sessionId);
        if (sessionPtr == 0) {
            return null;
        }
        return new RiSession(sessionPtr);
    }
    
    private native long getSession0(long ptr, String sessionId);
    
    public boolean destroySession(String sessionId) {
        return destroySession0(nativePtr, sessionId);
    }
    
    private native boolean destroySession0(long ptr, String sessionId);
    
    public boolean extendSession(String sessionId) {
        return extendSession0(nativePtr, sessionId);
    }
    
    private native boolean extendSession0(long ptr, String sessionId);
    
    public int cleanupExpired() {
        return cleanupExpired0(nativePtr);
    }
    
    private native int cleanupExpired0(long ptr);
    
    public long getTimeout() {
        return getTimeout0(nativePtr);
    }
    
    private native long getTimeout0(long ptr);
    
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
