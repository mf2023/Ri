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

package com.dunimd.ri.hooks;

import com.dunimd.ri.NativeLoader;

/**
 * Hook event for Ri.
 * 
 * Represents a hook event that can be emitted and processed
 * by the hook system.
 */
public class RiHookEvent {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new hook event.
     * 
     * @param kind the hook kind
     * @param module the module name (can be null)
     * @param phase the module phase (can be null)
     */
    public RiHookEvent(RiHookKind kind, String module, RiModulePhase phase) {
        int phaseOrdinal = phase != null ? phase.getOrdinal() : -1;
        this.nativePtr = new0(kind.getOrdinal(), module, phaseOrdinal);
    }
    
    private native long new0(int kindOrdinal, String module, int phaseOrdinal);
    
    /**
     * Close and release resources.
     */
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    /**
     * Get the hook kind.
     * 
     * @return the hook kind
     */
    public RiHookKind getKind() {
        if (nativePtr == 0) {
            return RiHookKind.STARTUP;
        }
        int ordinal = getKind0(nativePtr);
        return RiHookKind.fromOrdinal(ordinal);
    }
    
    private native int getKind0(long ptr);
    
    /**
     * Get the module name.
     * 
     * @return the module name, or null if not set
     */
    public String getModule() {
        if (nativePtr == 0) {
            return null;
        }
        return getModule0(nativePtr);
    }
    
    private native String getModule0(long ptr);
    
    /**
     * Get the module phase.
     * 
     * @return the module phase, or null if not set
     */
    public RiModulePhase getPhase() {
        if (nativePtr == 0) {
            return null;
        }
        int ordinal = getPhase0(nativePtr);
        if (ordinal < 0) {
            return null;
        }
        return RiModulePhase.fromOrdinal(ordinal);
    }
    
    private native int getPhase0(long ptr);
    
    /**
     * Create a config reload event.
     * 
     * @return a new RiHookEvent for config reload
     */
    public static RiHookEvent configReload() {
        return new RiHookEvent(RiHookKind.CONFIG_RELOAD, null, null);
    }
    
    /**
     * Create a startup event.
     * 
     * @return a new RiHookEvent for startup
     */
    public static RiHookEvent startup() {
        return new RiHookEvent(RiHookKind.STARTUP, null, null);
    }
    
    /**
     * Create a shutdown event.
     * 
     * @return a new RiHookEvent for shutdown
     */
    public static RiHookEvent shutdown() {
        return new RiHookEvent(RiHookKind.SHUTDOWN, null, null);
    }
    
    @Override
    protected void finalize() {
        close();
    }
}
