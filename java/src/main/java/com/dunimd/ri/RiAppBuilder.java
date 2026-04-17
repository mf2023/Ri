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

package com.dunimd.ri;

/**
 * Application builder for creating Ri applications.
 * 
 * This class provides a fluent API for configuring and building Ri applications.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * RiAppRuntime runtime = new RiAppBuilder()
 *     .withConfig("config.yaml")
 *     .build();
 * }</pre>
 */
public class RiAppBuilder {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiAppBuilder instance.
     */
    public RiAppBuilder() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Configure the application from a YAML file.
     * 
     * @param configPath the path to the configuration file
     * @return a new RiAppBuilder with the configuration applied
     */
    public RiAppBuilder withConfig(String configPath) {
        long newPtr = withConfig0(nativePtr, configPath);
        return new RiAppBuilder(newPtr);
    }
    
    private native long withConfig0(long ptr, String configPath);
    
    /**
     * Build and return the application runtime.
     * 
     * @return the RiAppRuntime instance
     * @throws RiError if the build fails
     */
    public RiAppRuntime build() {
        long runtimePtr = build0(nativePtr);
        if (runtimePtr == 0) {
            throw new RiError("Failed to build Ri application");
        }
        return new RiAppRuntime(runtimePtr);
    }
    
    private native long build0(long ptr);
    
    private RiAppBuilder(long ptr) {
        this.nativePtr = ptr;
    }
    
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
