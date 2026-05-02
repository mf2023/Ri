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
 * Hook kind enumeration for Ri.
 * 
 * Defines the different types of hooks that can be emitted
 * during the application lifecycle.
 */
public enum RiHookKind {
    /**
     * Emitted when the application starts up.
     */
    STARTUP(0),
    
    /**
     * Emitted when the application shuts down.
     */
    SHUTDOWN(1),
    
    /**
     * Emitted before modules are initialized.
     */
    BEFORE_MODULES_INIT(2),
    
    /**
     * Emitted after modules are initialized.
     */
    AFTER_MODULES_INIT(3),
    
    /**
     * Emitted before modules are started.
     */
    BEFORE_MODULES_START(4),
    
    /**
     * Emitted after modules are started.
     */
    AFTER_MODULES_START(5),
    
    /**
     * Emitted before modules are shut down.
     */
    BEFORE_MODULES_SHUTDOWN(6),
    
    /**
     * Emitted after modules are shut down.
     */
    AFTER_MODULES_SHUTDOWN(7),
    
    /**
     * Emitted when configuration is reloaded.
     */
    CONFIG_RELOAD(8);
    
    private final int ordinal;
    
    RiHookKind(int ordinal) {
        this.ordinal = ordinal;
    }
    
    /**
     * Get the ordinal value of this hook kind.
     * 
     * @return the ordinal value
     */
    public int getOrdinal() {
        return ordinal;
    }
    
    /**
     * Get a hook kind by its ordinal value.
     * 
     * @param ordinal the ordinal value
     * @return the corresponding hook kind, or STARTUP if invalid
     */
    public static RiHookKind fromOrdinal(int ordinal) {
        switch (ordinal) {
            case 0: return STARTUP;
            case 1: return SHUTDOWN;
            case 2: return BEFORE_MODULES_INIT;
            case 3: return AFTER_MODULES_INIT;
            case 4: return BEFORE_MODULES_START;
            case 5: return AFTER_MODULES_START;
            case 6: return BEFORE_MODULES_SHUTDOWN;
            case 7: return AFTER_MODULES_SHUTDOWN;
            case 8: return CONFIG_RELOAD;
            default: return STARTUP;
        }
    }
}
