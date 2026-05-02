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

package com.dunimd.ri;

/**
 * Health status enumeration representing the state of a component or service.
 */
public enum RiHealthStatus {
    /**
     * Component is functioning normally.
     */
    Healthy(0),
    
    /**
     * Component is experiencing issues but still operational.
     */
    Degraded(1),
    
    /**
     * Component is not functioning and requires attention.
     */
    Unhealthy(2),
    
    /**
     * Health status is unknown (check failed or not performed).
     */
    Unknown(3);
    
    private final int value;
    
    RiHealthStatus(int value) {
        this.value = value;
    }
    
    /**
     * Get the integer value of this status.
     * 
     * @return the integer value
     */
    public int getValue() {
        return value;
    }
    
    /**
     * Returns true if the status is considered healthy (Healthy or Degraded).
     * 
     * @return true if healthy
     */
    public boolean isHealthy() {
        return this == Healthy || this == Degraded;
    }
    
    /**
     * Returns true if the status requires immediate attention.
     * 
     * @return true if requires attention
     */
    public boolean requiresAttention() {
        return this == Unhealthy;
    }
    
    /**
     * Create a RiHealthStatus from an integer value.
     * 
     * @param value the integer value
     * @return the corresponding RiHealthStatus
     */
    public static RiHealthStatus fromValue(int value) {
        switch (value) {
            case 0: return Healthy;
            case 1: return Degraded;
            case 2: return Unhealthy;
            default: return Unknown;
        }
    }
}
