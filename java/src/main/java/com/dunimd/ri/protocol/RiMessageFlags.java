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
 * Message flags for Ri.
 * 
 * Flags for protocol messages.
 */
public class RiMessageFlags {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiMessageFlags() {
        this.nativePtr = new0();
    }
    
    RiMessageFlags(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Check if the message is compressed.
     * 
     * @return true if compressed
     */
    public boolean isCompressed() {
        return isCompressed0(nativePtr);
    }
    
    private native boolean isCompressed0(long ptr);
    
    /**
     * Set compressed flag.
     * 
     * @param compressed whether the message is compressed
     */
    public void setCompressed(boolean compressed) {
        setCompressed0(nativePtr, compressed);
    }
    
    private native void setCompressed0(long ptr, boolean compressed);
    
    /**
     * Check if the message is encrypted.
     * 
     * @return true if encrypted
     */
    public boolean isEncrypted() {
        return isEncrypted0(nativePtr);
    }
    
    private native boolean isEncrypted0(long ptr);
    
    /**
     * Set encrypted flag.
     * 
     * @param encrypted whether the message is encrypted
     */
    public void setEncrypted(boolean encrypted) {
        setEncrypted0(nativePtr, encrypted);
    }
    
    private native void setEncrypted0(long ptr, boolean encrypted);
    
    /**
     * Check if the message requires acknowledgment.
     * 
     * @return true if requires acknowledgment
     */
    public boolean isRequiresAck() {
        return isRequiresAck0(nativePtr);
    }
    
    private native boolean isRequiresAck0(long ptr);
    
    /**
     * Set requires acknowledgment flag.
     * 
     * @param requiresAck whether the message requires acknowledgment
     */
    public void setRequiresAck(boolean requiresAck) {
        setRequiresAck0(nativePtr, requiresAck);
    }
    
    private native void setRequiresAck0(long ptr, boolean requiresAck);
    
    /**
     * Check if the message is priority.
     * 
     * @return true if priority
     */
    public boolean isPriority() {
        return isPriority0(nativePtr);
    }
    
    private native boolean isPriority0(long ptr);
    
    /**
     * Set priority flag.
     * 
     * @param priority whether the message is priority
     */
    public void setPriority(boolean priority) {
        setPriority0(nativePtr, priority);
    }
    
    private native void setPriority0(long ptr, boolean priority);
    
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
    
    long getNativePtr() {
        return nativePtr;
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
