// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

package com.dunimd.dmsc.auth;

import com.dunimd.dmsc.NativeLoader;

/**
 * JWT token manager for DMSC.
 * 
 * Provides JWT token generation and validation.
 */
public class DMSCJWTManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public DMSCJWTManager(long nativePtr) {
        this.nativePtr = nativePtr;
    }
    
    public String generateToken(DMSCJWTClaims claims) {
        return generateToken0(nativePtr, claims.getNativePtr());
    }
    
    private native String generateToken0(long ptr, long claimsPtr);
    
    public DMSCJWTClaims validateToken(String token) {
        long claimsPtr = validateToken0(nativePtr, token);
        return new DMSCJWTClaims(claimsPtr);
    }
    
    private native long validateToken0(long ptr, String token);
}
