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
 * JWT claims for Ri.
 * 
 * Represents the claims payload in a JWT token.
 */
public class RiJWTClaims {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiJWTClaims(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public RiJWTClaims(String sub, List<String> roles, List<String> permissions) {
        String[] rolesArr = roles.toArray(new String[0]);
        String[] permsArr = permissions.toArray(new String[0]);
        this.nativePtr = new0(sub, rolesArr, permsArr);
    }
    
    private native long new0(String sub, String[] roles, String[] permissions);
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public String getSub() {
        return getSub0(nativePtr);
    }
    
    private native String getSub0(long ptr);
    
    public long getExp() {
        return getExp0(nativePtr);
    }
    
    private native long getExp0(long ptr);
    
    public long getIat() {
        return getIat0(nativePtr);
    }
    
    private native long getIat0(long ptr);
    
    public List<String> getRoles() {
        String[] roles = getRoles0(nativePtr);
        return java.util.Arrays.asList(roles);
    }
    
    private native String[] getRoles0(long ptr);
    
    public List<String> getPermissions() {
        String[] perms = getPermissions0(nativePtr);
        return java.util.Arrays.asList(perms);
    }
    
    private native String[] getPermissions0(long ptr);
    
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
