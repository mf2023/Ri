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

package com.dunimd.ri.log;

/**
 * Log level enumeration for Ri.
 * 
 * Defines the severity levels for logging messages.
 * The levels are ordered from least to most severe.
 */
public enum RiLogLevel {
    /**
     * Debug level: Detailed information for debugging purposes.
     */
    DEBUG(0),
    
    /**
     * Info level: General information about application operation.
     */
    INFO(1),
    
    /**
     * Warn level: Warning messages about potential issues.
     */
    WARN(2),
    
    /**
     * Error level: Error messages about failures.
     */
    ERROR(3);
    
    private final int ordinal;
    
    RiLogLevel(int ordinal) {
        this.ordinal = ordinal;
    }
    
    /**
     * Get the ordinal value of this log level.
     * 
     * @return the ordinal value
     */
    public int getOrdinal() {
        return ordinal;
    }
    
    /**
     * Get a log level by its ordinal value.
     * 
     * @param ordinal the ordinal value
     * @return the corresponding log level, or INFO if invalid
     */
    public static RiLogLevel fromOrdinal(int ordinal) {
        switch (ordinal) {
            case 0: return DEBUG;
            case 1: return INFO;
            case 2: return WARN;
            case 3: return ERROR;
            default: return INFO;
        }
    }
}
