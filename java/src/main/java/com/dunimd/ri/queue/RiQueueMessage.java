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

package com.dunimd.ri.queue;

import com.dunimd.ri.NativeLoader;

/**
 * Message structure for queue operations.
 */
public class RiQueueMessage {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiQueueMessage(byte[] payload) {
        this.nativePtr = new0(payload);
    }
    
    private native long new0(byte[] payload);
    
    RiQueueMessage(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getId() {
        return getId0(nativePtr);
    }
    
    private native String getId0(long ptr);
    
    public byte[] getPayload() {
        return getPayload0(nativePtr);
    }
    
    private native byte[] getPayload0(long ptr);
    
    public String getPayloadString() {
        return getPayloadString0(nativePtr);
    }
    
    private native String getPayloadString0(long ptr);
    
    public int getRetryCount() {
        return getRetryCount0(nativePtr);
    }
    
    private native int getRetryCount0(long ptr);
    
    public int getMaxRetries() {
        return getMaxRetries0(nativePtr);
    }
    
    private native int getMaxRetries0(long ptr);
    
    public void setMaxRetries(int maxRetries) {
        setMaxRetries0(nativePtr, maxRetries);
    }
    
    private native void setMaxRetries0(long ptr, int maxRetries);
    
    public boolean canRetry() {
        return canRetry0(nativePtr);
    }
    
    private native boolean canRetry0(long ptr);
    
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
