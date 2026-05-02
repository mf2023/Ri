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
 * Protocol frame for Ri.
 */
public class RiFrame {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiFrame() {
        this.nativePtr = new0();
    }
    
    RiFrame(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Get the frame header.
     * 
     * @return the frame header
     */
    public RiFrameHeader getHeader() {
        long ptr = getHeader0(nativePtr);
        return new RiFrameHeader(ptr);
    }
    
    private native long getHeader0(long ptr);
    
    /**
     * Get the payload.
     * 
     * @return the payload
     */
    public byte[] getPayload() {
        return getPayload0(nativePtr);
    }
    
    private native byte[] getPayload0(long ptr);
    
    /**
     * Set the payload.
     * 
     * @param payload the payload
     */
    public void setPayload(byte[] payload) {
        setPayload0(nativePtr, payload);
    }
    
    private native void setPayload0(long ptr, byte[] payload);
    
    /**
     * Get the source ID.
     * 
     * @return the source ID
     */
    public String getSourceId() {
        return getSourceId0(nativePtr);
    }
    
    private native String getSourceId0(long ptr);
    
    /**
     * Set the source ID.
     * 
     * @param sourceId the source ID
     */
    public void setSourceId(String sourceId) {
        setSourceId0(nativePtr, sourceId);
    }
    
    private native void setSourceId0(long ptr, String sourceId);
    
    /**
     * Get the target ID.
     * 
     * @return the target ID
     */
    public String getTargetId() {
        return getTargetId0(nativePtr);
    }
    
    private native String getTargetId0(long ptr);
    
    /**
     * Set the target ID.
     * 
     * @param targetId the target ID
     */
    public void setTargetId(String targetId) {
        setTargetId0(nativePtr, targetId);
    }
    
    private native void setTargetId0(long ptr, String targetId);
    
    /**
     * Convert the frame to bytes.
     * 
     * @return the frame as bytes
     */
    public byte[] toBytes() {
        return toBytes0(nativePtr);
    }
    
    private native byte[] toBytes0(long ptr);
    
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
