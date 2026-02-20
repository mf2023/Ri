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
 * Application builder for creating DMSC applications.
 * 
 * This class provides a fluent API for configuring and building DMSC applications.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * DMSCAppRuntime runtime = new DMSCAppBuilder()
 *     .withConfig("config.yaml")
 *     .build();
 * }</pre>
 */
public class DMSCAppBuilder {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new DMSCAppBuilder instance.
     */
    public DMSCAppBuilder() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Configure the application from a YAML file.
     * 
     * @param configPath the path to the configuration file
     * @return a new DMSCAppBuilder with the configuration applied
     */
    public DMSCAppBuilder withConfig(String configPath) {
        long newPtr = withConfig0(nativePtr, configPath);
        return new DMSCAppBuilder(newPtr);
    }
    
    private native long withConfig0(long ptr, String configPath);
    
    /**
     * Build and return the application runtime.
     * 
     * @return the DMSCAppRuntime instance
     * @throws DMSCError if the build fails
     */
    public DMSCAppRuntime build() {
        long runtimePtr = build0(nativePtr);
        if (runtimePtr == 0) {
            throw new DMSCError("Failed to build DMSC application");
        }
        return new DMSCAppRuntime(runtimePtr);
    }
    
    private native long build0(long ptr);
    
    private DMSCAppBuilder(long ptr) {
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
