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
 * Device status enumeration.
 */
public enum RiDeviceStatus {
    /**
     * Device status is unknown.
     */
    Unknown,
    
    /**
     * Device is available for use.
     */
    Available,
    
    /**
     * Device is currently in use.
     */
    Busy,
    
    /**
     * Device has encountered an error.
     */
    Error,
    
    /**
     * Device is offline or unreachable.
     */
    Offline,
    
    /**
     * Device is under maintenance.
     */
    Maintenance,
    
    /**
     * Device is degraded but still operational.
     */
    Degraded,
    
    /**
     * Device is allocated to a specific task.
     */
    Allocated
}
