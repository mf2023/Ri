// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

package com.dunimd.dmsc.queue;

import com.dunimd.dmsc.NativeLoader;

/**
 * Message queue module for DMSC.
 * 
 * Provides message queuing with RabbitMQ and Redis Pub/Sub support.
 */
public class DMSCQueueModule {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public DMSCQueueModule(DMSCQueueConfig config) {
        this.nativePtr = new0(config.getNativePtr());
    }
    
    private native long new0(long configPtr);
    
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
