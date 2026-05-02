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
 * Revoked token info for Ri.
 * 
 * Information about a revoked JWT token.
 */
public class RiRevokedTokenInfo {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiRevokedTokenInfo(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiRevokedTokenInfo(String tokenId, String userId, long revokedAt, 
                             long expiresAt, String reason) {
        this.nativePtr = new0(tokenId, userId, revokedAt, expiresAt, reason);
    }
    
    private native long new0(String tokenId, String userId, long revokedAt, 
                            long expiresAt, String reason);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getTokenId() {
        return getTokenId0(nativePtr);
    }
    
    private native String getTokenId0(long ptr);
    
    public String getUserId() {
        return getUserId0(nativePtr);
    }
    
    private native String getUserId0(long ptr);
    
    public long getRevokedAt() {
        return getRevokedAt0(nativePtr);
    }
    
    private native long getRevokedAt0(long ptr);
    
    public long getExpiresAt() {
        return getExpiresAt0(nativePtr);
    }
    
    private native long getExpiresAt0(long ptr);
    
    public String getReason() {
        return getReason0(nativePtr);
    }
    
    private native String getReason0(long ptr);
    
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
