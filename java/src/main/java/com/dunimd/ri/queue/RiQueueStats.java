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
 * Statistics for queue monitoring.
 */
public class RiQueueStats {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    RiQueueStats(long ptr) {
        this.nativePtr = ptr;
    }
    
    public String getQueueName() {
        return getQueueName0(nativePtr);
    }
    
    private native String getQueueName0(long ptr);
    
    public long getMessageCount() {
        return getMessageCount0(nativePtr);
    }
    
    private native long getMessageCount0(long ptr);
    
    public int getConsumerCount() {
        return getConsumerCount0(nativePtr);
    }
    
    private native int getConsumerCount0(long ptr);
    
    public int getProducerCount() {
        return getProducerCount0(nativePtr);
    }
    
    private native int getProducerCount0(long ptr);
    
    public long getProcessedMessages() {
        return getProcessedMessages0(nativePtr);
    }
    
    private native long getProcessedMessages0(long ptr);
    
    public long getFailedMessages() {
        return getFailedMessages0(nativePtr);
    }
    
    private native long getFailedMessages0(long ptr);
    
    public double getAvgProcessingTimeMs() {
        return getAvgProcessingTimeMs0(nativePtr);
    }
    
    private native double getAvgProcessingTimeMs0(long ptr);
    
    public long getTotalBytesSent() {
        return getTotalBytesSent0(nativePtr);
    }
    
    private native long getTotalBytesSent0(long ptr);
    
    public long getTotalBytesReceived() {
        return getTotalBytesReceived0(nativePtr);
    }
    
    private native long getTotalBytesReceived0(long ptr);
    
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
