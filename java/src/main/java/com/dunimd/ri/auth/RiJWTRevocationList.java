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
 * JWT revocation list for Ri.
 * 
 * Manages revoked JWT tokens.
 */
public class RiJWTRevocationList {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiJWTRevocationList() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public void revokeToken(String token, String userId, String reason, long ttlSecs) {
        revokeToken0(nativePtr, token, userId, reason, ttlSecs);
    }
    
    private native void revokeToken0(long ptr, String token, String userId, String reason, long ttlSecs);
    
    public boolean isRevoked(String token) {
        return isRevoked0(nativePtr, token);
    }
    
    private native boolean isRevoked0(long ptr, String token);
    
    public boolean revokeByUser(String userId) {
        return revokeByUser0(nativePtr, userId);
    }
    
    private native boolean revokeByUser0(long ptr, String userId);
    
    public RiRevokedTokenInfo getRevocationInfo(String token) {
        long infoPtr = getRevocationInfo0(nativePtr, token);
        if (infoPtr == 0) {
            return null;
        }
        return new RiRevokedTokenInfo(infoPtr);
    }
    
    private native long getRevocationInfo0(long ptr, String token);
    
    public int getRevokedCount() {
        return getRevokedCount0(nativePtr);
    }
    
    private native int getRevokedCount0(long ptr);
    
    public int cleanup() {
        return cleanup0(nativePtr);
    }
    
    private native int cleanup0(long ptr);
    
    public void clear() {
        clear0(nativePtr);
    }
    
    private native void clear0(long ptr);
    
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
