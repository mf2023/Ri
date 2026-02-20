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

package com.dunimd.dmsc;

/**
 * DMSC configuration class.
 * 
 * This class provides access to configuration values loaded from various sources.
 */
public class DMSCConfig {
    private long nativePtr;
    
    /**
     * Create a new DMSCConfig instance with default values.
     */
    public DMSCConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Create a DMSCConfig from YAML string.
     * 
     * @param yaml the YAML configuration string
     * @return the DMSCConfig instance
     */
    public static DMSCConfig fromYaml(String yaml) {
        long ptr = fromYaml0(yaml);
        if (ptr == 0) {
            throw new DMSCError("Failed to parse YAML configuration");
        }
        return new DMSCConfig(ptr);
    }
    
    private static native long fromYaml0(String yaml);
    
    private DMSCConfig(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get a configuration value by key.
     * 
     * @param key the configuration key
     * @return the configuration value, or null if not found
     */
    public String get(String key) {
        return get0(nativePtr, key);
    }
    
    private native String get0(long ptr, String key);
    
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
