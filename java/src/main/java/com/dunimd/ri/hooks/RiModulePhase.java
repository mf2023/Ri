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

/**
 * Module phase enumeration for Ri.
 * 
 * Defines the different phases a module can go through during its lifecycle,
 * including both synchronous and asynchronous phases.
 */
public enum RiModulePhase {
    /**
     * Synchronous initialization phase.
     */
    INIT(0),
    
    /**
     * Synchronous phase before starting.
     */
    BEFORE_START(1),
    
    /**
     * Synchronous start phase.
     */
    START(2),
    
    /**
     * Synchronous phase after starting.
     */
    AFTER_START(3),
    
    /**
     * Synchronous phase before shutting down.
     */
    BEFORE_SHUTDOWN(4),
    
    /**
     * Synchronous shutdown phase.
     */
    SHUTDOWN(5),
    
    /**
     * Synchronous phase after shutting down.
     */
    AFTER_SHUTDOWN(6),
    
    /**
     * Asynchronous initialization phase.
     */
    ASYNC_INIT(7),
    
    /**
     * Asynchronous phase before starting.
     */
    ASYNC_BEFORE_START(8),
    
    /**
     * Asynchronous start phase.
     */
    ASYNC_START(9),
    
    /**
     * Asynchronous phase after starting.
     */
    ASYNC_AFTER_START(10),
    
    /**
     * Asynchronous phase before shutting down.
     */
    ASYNC_BEFORE_SHUTDOWN(11),
    
    /**
     * Asynchronous shutdown phase.
     */
    ASYNC_SHUTDOWN(12),
    
    /**
     * Asynchronous phase after shutting down.
     */
    ASYNC_AFTER_SHUTDOWN(13);
    
    private final int ordinal;
    
    RiModulePhase(int ordinal) {
        this.ordinal = ordinal;
    }
    
    /**
     * Get the ordinal value of this module phase.
     * 
     * @return the ordinal value
     */
    public int getOrdinal() {
        return ordinal;
    }
    
    /**
     * Get a module phase by its ordinal value.
     * 
     * @param ordinal the ordinal value
     * @return the corresponding module phase, or INIT if invalid
     */
    public static RiModulePhase fromOrdinal(int ordinal) {
        switch (ordinal) {
            case 0: return INIT;
            case 1: return BEFORE_START;
            case 2: return START;
            case 3: return AFTER_START;
            case 4: return BEFORE_SHUTDOWN;
            case 5: return SHUTDOWN;
            case 6: return AFTER_SHUTDOWN;
            case 7: return ASYNC_INIT;
            case 8: return ASYNC_BEFORE_START;
            case 9: return ASYNC_START;
            case 10: return ASYNC_AFTER_START;
            case 11: return ASYNC_BEFORE_SHUTDOWN;
            case 12: return ASYNC_SHUTDOWN;
            case 13: return ASYNC_AFTER_SHUTDOWN;
            default: return INIT;
        }
    }
}
