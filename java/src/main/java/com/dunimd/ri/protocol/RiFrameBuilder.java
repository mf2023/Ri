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
 * Frame builder for Ri.
 * 
 * Builds protocol frames from data.
 */
public class RiFrameBuilder {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiFrameBuilder() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Set the frame type.
     * 
     * @param frameType the frame type
     * @return this builder
     */
    public RiFrameBuilder setFrameType(RiFrameType frameType) {
        setFrameType0(nativePtr, frameType.ordinal());
        return this;
    }
    
    private native void setFrameType0(long ptr, int frameType);
    
    /**
     * Set the sequence number.
     * 
     * @param sequenceNumber the sequence number
     * @return this builder
     */
    public RiFrameBuilder setSequenceNumber(long sequenceNumber) {
        setSequenceNumber0(nativePtr, sequenceNumber);
        return this;
    }
    
    private native void setSequenceNumber0(long ptr, long sequenceNumber);
    
    /**
     * Set the source ID.
     * 
     * @param sourceId the source ID
     * @return this builder
     */
    public RiFrameBuilder setSourceId(String sourceId) {
        setSourceId0(nativePtr, sourceId);
        return this;
    }
    
    private native void setSourceId0(long ptr, String sourceId);
    
    /**
     * Set the target ID.
     * 
     * @param targetId the target ID
     * @return this builder
     */
    public RiFrameBuilder setTargetId(String targetId) {
        setTargetId0(nativePtr, targetId);
        return this;
    }
    
    private native void setTargetId0(long ptr, String targetId);
    
    /**
     * Build a data frame.
     * 
     * @param payload the payload
     * @return the built frame
     */
    public RiFrame buildDataFrame(byte[] payload) {
        long ptr = buildDataFrame0(nativePtr, payload);
        return new RiFrame(ptr);
    }
    
    private native long buildDataFrame0(long ptr, byte[] payload);
    
    /**
     * Build a heartbeat frame.
     * 
     * @return the built frame
     */
    public RiFrame buildHeartbeatFrame() {
        long ptr = buildHeartbeatFrame0(nativePtr);
        return new RiFrame(ptr);
    }
    
    private native long buildHeartbeatFrame0(long ptr);
    
    /**
     * Build an acknowledgment frame.
     * 
     * @param sequenceNumber the sequence number to acknowledge
     * @return the built frame
     */
    public RiFrame buildAckFrame(long sequenceNumber) {
        long ptr = buildAckFrame0(nativePtr, sequenceNumber);
        return new RiFrame(ptr);
    }
    
    private native long buildAckFrame0(long ptr, long sequenceNumber);
    
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
