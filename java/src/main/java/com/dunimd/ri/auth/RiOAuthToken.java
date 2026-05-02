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
 * OAuth token for Ri.
 * 
 * Represents an OAuth token response from a provider.
 */
public class RiOAuthToken {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiOAuthToken(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiOAuthToken(String accessToken, String tokenType, String refreshToken, 
                       String scope, long expiresIn) {
        this.nativePtr = new0(accessToken, tokenType, refreshToken, scope, expiresIn);
    }
    
    private native long new0(String accessToken, String tokenType, String refreshToken, 
                            String scope, long expiresIn);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getAccessToken() {
        return getAccessToken0(nativePtr);
    }
    
    private native String getAccessToken0(long ptr);
    
    public String getTokenType() {
        return getTokenType0(nativePtr);
    }
    
    private native String getTokenType0(long ptr);
    
    public String getRefreshToken() {
        return getRefreshToken0(nativePtr);
    }
    
    private native String getRefreshToken0(long ptr);
    
    public String getScope() {
        return getScope0(nativePtr);
    }
    
    private native String getScope0(long ptr);
    
    public long getExpiresIn() {
        return getExpiresIn0(nativePtr);
    }
    
    private native long getExpiresIn0(long ptr);
    
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
