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
 * OAuth provider for Ri.
 * 
 * Configuration for an OAuth identity provider.
 */
public class RiOAuthProvider {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiOAuthProvider(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiOAuthProvider(String id, String name, String clientId, String clientSecret,
                          String authUrl, String tokenUrl, String userInfoUrl,
                          List<String> scopes, boolean enabled, String redirectUri) {
        this.nativePtr = new0(id, name, clientId, clientSecret, authUrl, tokenUrl, 
                              userInfoUrl, scopes.toArray(new String[0]), enabled, redirectUri);
    }
    
    private native long new0(String id, String name, String clientId, String clientSecret,
                            String authUrl, String tokenUrl, String userInfoUrl,
                            String[] scopes, boolean enabled, String redirectUri);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    public String getClientId() {
        return getClientId0(nativePtr);
    }
    
    private native String getClientId0(long ptr);
    
    public String getAuthUrl() {
        return getAuthUrl0(nativePtr);
    }
    
    private native String getAuthUrl0(long ptr);
    
    public String getTokenUrl() {
        return getTokenUrl0(nativePtr);
    }
    
    private native String getTokenUrl0(long ptr);
    
    public String getUserInfoUrl() {
        return getUserInfoUrl0(nativePtr);
    }
    
    private native String getUserInfoUrl0(long ptr);
    
    public List<String> getScopes() {
        String[] scopes = getScopes0(nativePtr);
        return java.util.Arrays.asList(scopes);
    }
    
    private native String[] getScopes0(long ptr);
    
    public boolean isEnabled() {
        return isEnabled0(nativePtr);
    }
    
    private native boolean isEnabled0(long ptr);
    
    public String getRedirectUri() {
        return getRedirectUri0(nativePtr);
    }
    
    private native String getRedirectUri0(long ptr);
    
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
