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
 * Validator builder for Ri.
 * 
 * Provides a fluent API for building complex validation rules.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * RiValidatorBuilder builder = new RiValidatorBuilder("email")
 *     .notEmpty()
 *     .maxLength(255)
 *     .isEmail();
 * }</pre>
 */
public class RiValidatorBuilder {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiValidatorBuilder for the given field.
     * 
     * @param fieldName the field name to validate
     */
    public RiValidatorBuilder(String fieldName) {
        this.nativePtr = new0(fieldName);
    }
    
    private native long new0(String fieldName);
    
    /**
     * Add a not-empty validation rule.
     * 
     * @return this builder instance
     */
    public RiValidatorBuilder notEmpty() {
        notEmpty0(nativePtr);
        return this;
    }
    
    private native void notEmpty0(long ptr);
    
    /**
     * Add a minimum length validation rule.
     * 
     * @param min the minimum length
     * @return this builder instance
     */
    public RiValidatorBuilder minLength(int min) {
        minLength0(nativePtr, min);
        return this;
    }
    
    private native void minLength0(long ptr, int min);
    
    /**
     * Add a maximum length validation rule.
     * 
     * @param max the maximum length
     * @return this builder instance
     */
    public RiValidatorBuilder maxLength(int max) {
        maxLength0(nativePtr, max);
        return this;
    }
    
    private native void maxLength0(long ptr, int max);
    
    /**
     * Add an email format validation rule.
     * 
     * @return this builder instance
     */
    public RiValidatorBuilder isEmail() {
        isEmail0(nativePtr);
        return this;
    }
    
    private native void isEmail0(long ptr);
    
    /**
     * Build the validator.
     * 
     * @return the built validator
     */
    public RiValidationRunner build() {
        long runnerPtr = build0(nativePtr);
        return new RiValidationRunner(runnerPtr);
    }
    
    private native long build0(long ptr);
    
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
