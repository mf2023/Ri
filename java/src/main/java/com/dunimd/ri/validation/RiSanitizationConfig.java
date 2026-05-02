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
 * Sanitization configuration for Ri.
 */
public class RiSanitizationConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSanitizationConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Check if whitespace trimming is enabled.
     * 
     * @return true if whitespace trimming is enabled
     */
    public boolean isTrimWhitespace() {
        return isTrimWhitespace0(nativePtr);
    }
    
    private native boolean isTrimWhitespace0(long ptr);
    
    /**
     * Set whitespace trimming enabled.
     * 
     * @param enabled whether whitespace trimming is enabled
     */
    public void setTrimWhitespace(boolean enabled) {
        setTrimWhitespace0(nativePtr, enabled);
    }
    
    private native void setTrimWhitespace0(long ptr, boolean enabled);
    
    /**
     * Check if lowercase conversion is enabled.
     * 
     * @return true if lowercase conversion is enabled
     */
    public boolean isLowercase() {
        return isLowercase0(nativePtr);
    }
    
    private native boolean isLowercase0(long ptr);
    
    /**
     * Set lowercase conversion enabled.
     * 
     * @param enabled whether lowercase conversion is enabled
     */
    public void setLowercase(boolean enabled) {
        setLowercase0(nativePtr, enabled);
    }
    
    private native void setLowercase0(long ptr, boolean enabled);
    
    /**
     * Check if uppercase conversion is enabled.
     * 
     * @return true if uppercase conversion is enabled
     */
    public boolean isUppercase() {
        return isUppercase0(nativePtr);
    }
    
    private native boolean isUppercase0(long ptr);
    
    /**
     * Set uppercase conversion enabled.
     * 
     * @param enabled whether uppercase conversion is enabled
     */
    public void setUppercase(boolean enabled) {
        setUppercase0(nativePtr, enabled);
    }
    
    private native void setUppercase0(long ptr, boolean enabled);
    
    /**
     * Check if HTML tag removal is enabled.
     * 
     * @return true if HTML tag removal is enabled
     */
    public boolean isRemoveHtmlTags() {
        return isRemoveHtmlTags0(nativePtr);
    }
    
    private native boolean isRemoveHtmlTags0(long ptr);
    
    /**
     * Set HTML tag removal enabled.
     * 
     * @param enabled whether HTML tag removal is enabled
     */
    public void setRemoveHtmlTags(boolean enabled) {
        setRemoveHtmlTags0(nativePtr, enabled);
    }
    
    private native void setRemoveHtmlTags0(long ptr, boolean enabled);
    
    /**
     * Check if special character escaping is enabled.
     * 
     * @return true if special character escaping is enabled
     */
    public boolean isEscapeSpecialChars() {
        return isEscapeSpecialChars0(nativePtr);
    }
    
    private native boolean isEscapeSpecialChars0(long ptr);
    
    /**
     * Set special character escaping enabled.
     * 
     * @param enabled whether special character escaping is enabled
     */
    public void setEscapeSpecialChars(boolean enabled) {
        setEscapeSpecialChars0(nativePtr, enabled);
    }
    
    private native void setEscapeSpecialChars0(long ptr, boolean enabled);
    
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
    
    long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
