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

package com.dunimd.ri.protocol;

import com.dunimd.ri.NativeLoader;

/**
 * Frame parser for Ri.
 * 
 * Parses binary data into protocol frames.
 */
public class RiFrameParser {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiFrameParser() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Parse a frame from bytes.
     * 
     * @param data the binary data
     * @return the parsed frame, or null if parsing failed
     */
    public RiFrame parse(byte[] data) {
        long ptr = parse0(nativePtr, data);
        if (ptr == 0) {
            return null;
        }
        return new RiFrame(ptr);
    }
    
    private native long parse0(long ptr, byte[] data);
    
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
