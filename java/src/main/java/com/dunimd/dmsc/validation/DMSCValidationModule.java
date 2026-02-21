// Copyright 2025-2026 Wenze Wei. All Rights Reserved.
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

package com.dunimd.dmsc.validation;

import com.dunimd.dmsc.NativeLoader;

/**
 * Validation module for DMSC.
 */
public class DMSCValidationModule {
    static {
        NativeLoader.autoLoad();
    }
    
    public static DMSCValidationResult validateEmail(String email) {
        long ptr = nativeValidateEmail(email);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long nativeValidateEmail(String email);
    
    public static DMSCValidationResult validateUsername(String username) {
        long ptr = nativeValidateUsername(username);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long nativeValidateUsername(String username);
    
    public static DMSCValidationResult validatePassword(String password) {
        long ptr = nativeValidatePassword(password);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long nativeValidatePassword(String password);
    
    public static DMSCValidationResult validateUrl(String url) {
        long ptr = nativeValidateUrl(url);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long nativeValidateUrl(String url);
    
    public static DMSCValidationResult validateIp(String ip) {
        long ptr = nativeValidateIp(ip);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long nativeValidateIp(String ip);
}
