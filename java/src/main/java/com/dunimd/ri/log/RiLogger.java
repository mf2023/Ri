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

package com.dunimd.ri.log;

import com.dunimd.ri.NativeLoader;

/**
 * Logger for Ri.
 * 
 * Provides structured logging with multiple output formats.
 */
public class RiLogger {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiLogger() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    /**
     * Log a debug message.
     * 
     * @param target the target/module name
     * @param message the message to log
     */
    public void debug(String target, String message) {
        debug0(nativePtr, target, message);
    }
    
    private native void debug0(long ptr, String target, String message);
    
    /**
     * Log an info message.
     * 
     * @param target the target/module name
     * @param message the message to log
     */
    public void info(String target, String message) {
        info0(nativePtr, target, message);
    }
    
    private native void info0(long ptr, String target, String message);
    
    /**
     * Log a warning message.
     * 
     * @param target the target/module name
     * @param message the message to log
     */
    public void warn(String target, String message) {
        warn0(nativePtr, target, message);
    }
    
    private native void warn0(long ptr, String target, String message);
    
    /**
     * Log an error message.
     * 
     * @param target the target/module name
     * @param message the message to log
     */
    public void error(String target, String message) {
        error0(nativePtr, target, message);
    }
    
    private native void error0(long ptr, String target, String message);
    
    @Override
    protected void finalize() {
        close();
    }
}
