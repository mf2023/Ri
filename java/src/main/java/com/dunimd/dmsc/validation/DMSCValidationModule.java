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

package com.dunimd.dmsc.validation;

import com.dunimd.dmsc.NativeLoader;

/**
 * Validation module for DMSC.
 * 
 * Provides data validation and sanitization functionality.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * DMSCValidationResult result = DMSCValidationModule.validateEmail("user@example.com");
 * if (result.isValid()) {
 *     System.out.println("Email is valid");
 * } else {
 *     System.out.println("Email is invalid: " + result.getErrors());
 * }
 * }</pre>
 */
public class DMSCValidationModule {
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Validate an email address.
     * 
     * @param email the email address to validate
     * @return the validation result
     */
    public static DMSCValidationResult validateEmail(String email) {
        long ptr = validateEmail0(email);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long validateEmail0(String email);
    
    /**
     * Validate a username.
     * 
     * @param username the username to validate
     * @return the validation result
     */
    public static DMSCValidationResult validateUsername(String username) {
        long ptr = validateUsername0(username);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long validateUsername0(String username);
    
    /**
     * Validate a password.
     * 
     * @param password the password to validate
     * @return the validation result
     */
    public static DMSCValidationResult validatePassword(String password) {
        long ptr = validatePassword0(password);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long validatePassword0(String password);
    
    /**
     * Validate a URL.
     * 
     * @param url the URL to validate
     * @return the validation result
     */
    public static DMSCValidationResult validateUrl(String url) {
        long ptr = validateUrl0(url);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long validateUrl0(String url);
    
    /**
     * Validate an IP address.
     * 
     * @param ip the IP address to validate
     * @return the validation result
     */
    public static DMSCValidationResult validateIp(String ip) {
        long ptr = validateIp0(ip);
        return new DMSCValidationResult(ptr);
    }
    
    private static native long validateIp0(String ip);
}
