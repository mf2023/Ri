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
 * Trait for adding error chain functionality to Result types.
 * 
 * This class provides utility methods for working with error chains.
 */
public class RiErrorContext {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiErrorContext.
     */
    public RiErrorContext() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Create an error chain from a message.
     * 
     * @param message the error message
     * @return the error chain
     */
    public static RiErrorChain chainFromMsg(String message) {
        long ptr = chainFromMsg0(message);
        return new RiErrorChain(ptr);
    }
    
    private static native long chainFromMsg0(String message);
    
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
    
    private RiErrorContext(long ptr) {
        this.nativePtr = ptr;
    }
}
