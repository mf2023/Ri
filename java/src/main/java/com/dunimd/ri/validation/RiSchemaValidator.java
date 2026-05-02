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
 * Schema validator for Ri.
 * 
 * Provides JSON schema validation capabilities.
 */
public class RiSchemaValidator {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiSchemaValidator(String schemaJson) {
        this.nativePtr = new0(schemaJson);
    }
    
    private native long new0(String schemaJson);
    
    /**
     * Validate JSON data against the schema.
     * 
     * @param dataJson the JSON data to validate
     * @return the validation result
     */
    public RiValidationResult validate(String dataJson) {
        long ptr = validate0(nativePtr, dataJson);
        return new RiValidationResult(ptr);
    }
    
    private native long validate0(long ptr, String dataJson);
    
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
