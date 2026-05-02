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

package com.dunimd.ri.validation;

import com.dunimd.ri.NativeLoader;

/**
 * Sanitizer for Ri.
 * 
 * Provides data sanitization capabilities.
 */
public class RiSanitizer {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSanitizer() {
        this.nativePtr = new0();
    }
    
    public RiSanitizer(RiSanitizationConfig config) {
        this.nativePtr = newWithConfig0(config.getNativePtr());
    }
    
    private native long new0();
    
    private native long newWithConfig0(long configPtr);
    
    /**
     * Sanitize a string.
     * 
     * @param input the input string
     * @return the sanitized string
     */
    public String sanitize(String input) {
        return sanitize0(nativePtr, input);
    }
    
    private native String sanitize0(long ptr, String input);
    
    /**
     * Sanitize an email address.
     * 
     * @param input the input email
     * @return the sanitized email
     */
    public String sanitizeEmail(String input) {
        return sanitizeEmail0(nativePtr, input);
    }
    
    private native String sanitizeEmail0(long ptr, String input);
    
    /**
     * Sanitize a filename.
     * 
     * @param input the input filename
     * @return the sanitized filename
     */
    public String sanitizeFilename(String input) {
        return sanitizeFilename0(nativePtr, input);
    }
    
    private native String sanitizeFilename0(long ptr, String input);
    
    /**
     * Release native resources.
     */
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
