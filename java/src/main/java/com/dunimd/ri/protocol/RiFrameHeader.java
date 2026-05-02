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
 * Frame header for Ri.
 */
public class RiFrameHeader {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiFrameHeader() {
        this.nativePtr = new0();
    }
    
    RiFrameHeader(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Get the version.
     * 
     * @return the version
     */
    public int getVersion() {
        return getVersion0(nativePtr);
    }
    
    private native int getVersion0(long ptr);
    
    /**
     * Set the version.
     * 
     * @param version the version
     */
    public void setVersion(int version) {
        setVersion0(nativePtr, version);
    }
    
    private native void setVersion0(long ptr, int version);
    
    /**
     * Get the frame type.
     * 
     * @return the frame type
     */
    public RiFrameType getFrameType() {
        int ordinal = getFrameType0(nativePtr);
        return RiFrameType.values()[ordinal];
    }
    
    private native int getFrameType0(long ptr);
    
    /**
     * Set the frame type.
     * 
     * @param frameType the frame type
     */
    public void setFrameType(RiFrameType frameType) {
        setFrameType0(nativePtr, frameType.ordinal());
    }
    
    private native void setFrameType0(long ptr, int frameType);
    
    /**
     * Get the sequence number.
     * 
     * @return the sequence number
     */
    public long getSequenceNumber() {
        return getSequenceNumber0(nativePtr);
    }
    
    private native long getSequenceNumber0(long ptr);
    
    /**
     * Set the sequence number.
     * 
     * @param sequenceNumber the sequence number
     */
    public void setSequenceNumber(long sequenceNumber) {
        setSequenceNumber0(nativePtr, sequenceNumber);
    }
    
    private native void setSequenceNumber0(long ptr, long sequenceNumber);
    
    /**
     * Get the length.
     * 
     * @return the length
     */
    public int getLength() {
        return getLength0(nativePtr);
    }
    
    private native int getLength0(long ptr);
    
    /**
     * Get the timestamp.
     * 
     * @return the timestamp
     */
    public long getTimestamp() {
        return getTimestamp0(nativePtr);
    }
    
    private native long getTimestamp0(long ptr);
    
    /**
     * Get the flags.
     * 
     * @return the flags
     */
    public int getFlags() {
        return getFlags0(nativePtr);
    }
    
    private native int getFlags0(long ptr);
    
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
