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

package com.dunimd.ri.device;

/**
 * Device type enumeration.
 */
public enum RiDeviceType {
    /**
     * Central Processing Unit - General purpose computing.
     */
    CPU,
    
    /**
     * Graphics Processing Unit - Parallel computing and graphics.
     */
    GPU,
    
    /**
     * Memory - RAM and other memory devices.
     */
    Memory,
    
    /**
     * Storage - Hard drives, SSDs, and other storage devices.
     */
    Storage,
    
    /**
     * Network - Network interfaces and devices.
     */
    Network,
    
    /**
     * Sensor - Devices that collect data from the environment.
     */
    Sensor,
    
    /**
     * Actuator - Devices that perform physical actions.
     */
    Actuator,
    
    /**
     * Custom - User-defined device types.
     */
    Custom
}
