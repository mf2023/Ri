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

package com.dunimd.ri.observability;

/**
 * CPU metrics for monitoring CPU usage.
 */
public class RiCPUMetrics {
    private final double totalUsagePercent;
    private final double[] perCoreUsage;
    private final long contextSwitches;
    private final long interrupts;
    
    public RiCPUMetrics(
        double totalUsagePercent,
        double[] perCoreUsage,
        long contextSwitches,
        long interrupts
    ) {
        this.totalUsagePercent = totalUsagePercent;
        this.perCoreUsage = perCoreUsage;
        this.contextSwitches = contextSwitches;
        this.interrupts = interrupts;
    }
    
    public double getTotalUsagePercent() {
        return totalUsagePercent;
    }
    
    public double[] getPerCoreUsage() {
        return perCoreUsage;
    }
    
    public long getContextSwitches() {
        return contextSwitches;
    }
    
    public long getInterrupts() {
        return interrupts;
    }
}
