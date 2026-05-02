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
 * JWT validation options for Ri.
 * 
 * Configuration options for JWT token validation.
 */
public class RiJWTValidationOptions {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiJWTValidationOptions() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public long getNativePtr() {
        return nativePtr;
    }
    
    public boolean isValidateExp() {
        return isValidateExp0(nativePtr);
    }
    
    private native boolean isValidateExp0(long ptr);
    
    public void setValidateExp(boolean validate) {
        setValidateExp0(nativePtr, validate);
    }
    
    private native void setValidateExp0(long ptr, boolean validate);
    
    public boolean isValidateIat() {
        return isValidateIat0(nativePtr);
    }
    
    private native boolean isValidateIat0(long ptr);
    
    public void setValidateIat(boolean validate) {
        setValidateIat0(nativePtr, validate);
    }
    
    private native void setValidateIat0(long ptr, boolean validate);
    
    public List<String> getRequiredRoles() {
        String[] roles = getRequiredRoles0(nativePtr);
        return java.util.Arrays.asList(roles);
    }
    
    private native String[] getRequiredRoles0(long ptr);
    
    public void setRequiredRoles(List<String> roles) {
        setRequiredRoles0(nativePtr, roles.toArray(new String[0]));
    }
    
    private native void setRequiredRoles0(long ptr, String[] roles);
    
    public List<String> getRequiredPermissions() {
        String[] perms = getRequiredPermissions0(nativePtr);
        return java.util.Arrays.asList(perms);
    }
    
    private native String[] getRequiredPermissions0(long ptr);
    
    public void setRequiredPermissions(List<String> permissions) {
        setRequiredPermissions0(nativePtr, permissions.toArray(new String[0]));
    }
    
    private native void setRequiredPermissions0(long ptr, String[] permissions);
    
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
