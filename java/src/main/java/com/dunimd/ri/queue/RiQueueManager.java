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
 * Central queue management component.
 */
public class RiQueueManager {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiQueueManager() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    RiQueueManager(long ptr) {
        this.nativePtr = ptr;
    }
    
    public void init() {
        init0(nativePtr);
    }
    
    private native void init0(long ptr);
    
    public String createQueue(String name) {
        return createQueue0(nativePtr, name);
    }
    
    private native String createQueue0(long ptr, String name);
    
    public boolean queueExists(String name) {
        return queueExists0(nativePtr, name);
    }
    
    private native boolean queueExists0(long ptr, String name);
    
    public String[] listQueues() {
        return listQueues0(nativePtr);
    }
    
    private native String[] listQueues0(long ptr);
    
    public boolean deleteQueue(String name) {
        return deleteQueue0(nativePtr, name);
    }
    
    private native boolean deleteQueue0(long ptr, String name);
    
    public void publish(String queueName, byte[] message) {
        publish0(nativePtr, queueName, message);
    }
    
    private native void publish0(long ptr, String queueName, byte[] message);
    
    public RiQueueMessage consume(String queueName) {
        long msgPtr = consume0(nativePtr, queueName);
        return msgPtr != 0 ? new RiQueueMessage(msgPtr) : null;
    }
    
    private native long consume0(long ptr, String queueName);
    
    public RiQueueStats stats(String queueName) {
        long statsPtr = stats0(nativePtr, queueName);
        return statsPtr != 0 ? new RiQueueStats(statsPtr) : null;
    }
    
    private native long stats0(long ptr, String queueName);
    
    public void shutdown() {
        shutdown0(nativePtr);
    }
    
    private native void shutdown0(long ptr);
    
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
