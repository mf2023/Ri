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
import java.util.List;

/**
 * OAuth manager for Ri.
 * 
 * Manages OAuth providers and authentication flows.
 */
public class RiOAuthManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiOAuthManager() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public void registerProvider(RiOAuthProvider provider) {
        registerProvider0(nativePtr, provider.getNativePtr());
    }
    
    private native void registerProvider0(long ptr, long providerPtr);
    
    public RiOAuthProvider getProvider(String providerId) {
        long providerPtr = getProvider0(nativePtr, providerId);
        if (providerPtr == 0) {
            return null;
        }
        return new RiOAuthProvider(providerPtr);
    }
    
    private native long getProvider0(long ptr, String providerId);
    
    public String getAuthUrl(String providerId, String state) {
        return getAuthUrl0(nativePtr, providerId, state);
    }
    
    private native String getAuthUrl0(long ptr, String providerId, String state);
    
    public RiOAuthToken exchangeCodeForToken(String providerId, String code, String redirectUri) {
        long tokenPtr = exchangeCodeForToken0(nativePtr, providerId, code, redirectUri);
        if (tokenPtr == 0) {
            return null;
        }
        return new RiOAuthToken(tokenPtr);
    }
    
    private native long exchangeCodeForToken0(long ptr, String providerId, String code, String redirectUri);
    
    public RiOAuthUserInfo getUserInfo(String providerId, String accessToken) {
        long userInfoPtr = getUserInfo0(nativePtr, providerId, accessToken);
        if (userInfoPtr == 0) {
            return null;
        }
        return new RiOAuthUserInfo(userInfoPtr);
    }
    
    private native long getUserInfo0(long ptr, String providerId, String accessToken);
    
    public RiOAuthToken refreshToken(String providerId, String refreshToken) {
        long tokenPtr = refreshToken0(nativePtr, providerId, refreshToken);
        if (tokenPtr == 0) {
            return null;
        }
        return new RiOAuthToken(tokenPtr);
    }
    
    private native long refreshToken0(long ptr, String providerId, String refreshToken);
    
    public boolean revokeToken(String providerId, String accessToken) {
        return revokeToken0(nativePtr, providerId, accessToken);
    }
    
    private native boolean revokeToken0(long ptr, String providerId, String accessToken);
    
    public List<RiOAuthProvider> listProviders() {
        long[] providerPtrs = listProviders0(nativePtr);
        java.util.List<RiOAuthProvider> providers = new java.util.ArrayList<>();
        for (long ptr : providerPtrs) {
            if (ptr != 0) {
                providers.add(new RiOAuthProvider(ptr));
            }
        }
        return providers;
    }
    
    private native long[] listProviders0(long ptr);
    
    public boolean disableProvider(String providerId) {
        return disableProvider0(nativePtr, providerId);
    }
    
    private native boolean disableProvider0(long ptr, String providerId);
    
    public boolean enableProvider(String providerId) {
        return enableProvider0(nativePtr, providerId);
    }
    
    private native boolean enableProvider0(long ptr, String providerId);
    
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
