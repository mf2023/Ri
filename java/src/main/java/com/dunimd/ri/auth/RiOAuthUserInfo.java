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
 * OAuth user info for Ri.
 * 
 * Represents user information retrieved from an OAuth provider.
 */
public class RiOAuthUserInfo {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiOAuthUserInfo(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiOAuthUserInfo(String id, String email, String name, String avatarUrl, String provider) {
        this.nativePtr = new0(id, email, name, avatarUrl, provider);
    }
    
    private native long new0(String id, String email, String name, String avatarUrl, String provider);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public String getEmail() {
        return getEmail0(nativePtr);
    }
    
    private native String getEmail0(long ptr);
    
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    public String getAvatarUrl() {
        return getAvatarUrl0(nativePtr);
    }
    
    private native String getAvatarUrl0(long ptr);
    
    public String getProvider() {
        return getProvider0(nativePtr);
    }
    
    private native String getProvider0(long ptr);
    
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
