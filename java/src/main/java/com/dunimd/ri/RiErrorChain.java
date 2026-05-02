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
 * A chain of errors with contextual information.
 * 
 * This class provides enhanced error handling with error chain support, allowing
 * errors to be wrapped with additional context while preserving the original error.
 */
public class RiErrorChain {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiErrorChain from an error message.
     * 
     * @param message the error message
     */
    public RiErrorChain(String message) {
        this.nativePtr = new0(message);
    }
    
    private native long new0(String message);
    
    /**
     * Create a new RiErrorChain with context.
     * 
     * @param message the error message
     * @param context the context message
     */
    public RiErrorChain(String message, String context) {
        this.nativePtr = withContext0(message, context);
    }
    
    private native long withContext0(String message, String context);
    
    /**
     * Create a RiErrorChain from a native pointer.
     * 
     * @param ptr the native pointer
     */
    RiErrorChain(long ptr) {
        this.nativePtr = ptr;
    }
    
    /**
     * Get the context message.
     * 
     * @return the context message
     */
    public String getContext() {
        return getContext0(nativePtr);
    }
    
    private native String getContext0(long ptr);
    
    /**
     * Get the source error message.
     * 
     * @return the source error message
     */
    public String getSourceError() {
        return getSourceError0(nativePtr);
    }
    
    private native String getSourceError0(long ptr);
    
    /**
     * Get the pretty formatted error chain.
     * 
     * @return the formatted error chain
     */
    public String prettyFormat() {
        return prettyFormat0(nativePtr);
    }
    
    private native String prettyFormat0(long ptr);
    
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
