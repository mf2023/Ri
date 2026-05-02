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

import com.dunimd.ri.log.RiLogger;
import com.dunimd.ri.fs.RiFileSystem;
import com.dunimd.ri.hooks.RiHookBus;

/**
 * Service context for Ri applications.
 * 
 * The RiServiceContext is the primary way for modules and business logic to
 * access core Ri functionalities. It follows the dependency injection pattern,
 * providing a centralized access point to all core components.
 * 
 * <p>Usage example:</p>
 * <pre>{@code
 * RiServiceContext ctx = new RiServiceContext();
 * ctx.logger().info("app", "Application started");
 * String value = ctx.config().get("app.name");
 * }</pre>
 */
public class RiServiceContext {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiServiceContext with default configuration.
     */
    public RiServiceContext() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Get the structured logger.
     * 
     * @return the RiLogger instance
     */
    public RiLogger logger() {
        long loggerPtr = logger0(nativePtr);
        return new RiLogger(loggerPtr);
    }
    
    private native long logger0(long ptr);
    
    /**
     * Get the configuration manager.
     * 
     * @return the RiConfig instance
     */
    public RiConfig config() {
        long configPtr = config0(nativePtr);
        return new RiConfig(configPtr);
    }
    
    private native long config0(long ptr);
    
    /**
     * Get the file system accessor.
     * 
     * @return the RiFileSystem instance
     */
    public RiFileSystem fs() {
        long fsPtr = fs0(nativePtr);
        return new RiFileSystem(fsPtr);
    }
    
    private native long fs0(long ptr);
    
    /**
     * Get the hook bus for emitting events.
     * 
     * @return the RiHookBus instance
     */
    public RiHookBus hooks() {
        long hooksPtr = hooks0(nativePtr);
        return new RiHookBus(hooksPtr);
    }
    
    private native long hooks0(long ptr);
    
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
