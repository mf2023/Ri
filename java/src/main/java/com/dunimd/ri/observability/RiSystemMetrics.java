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
 * System metrics snapshot containing CPU, memory, disk, and network metrics.
 */
public class RiSystemMetrics {
    private final RiCPUMetrics cpu;
    private final RiMemoryMetrics memory;
    private final RiDiskMetrics disk;
    private final RiNetworkMetrics network;
    private final long timestamp;
    
    public RiSystemMetrics(
        RiCPUMetrics cpu,
        RiMemoryMetrics memory,
        RiDiskMetrics disk,
        RiNetworkMetrics network,
        long timestamp
    ) {
        this.cpu = cpu;
        this.memory = memory;
        this.disk = disk;
        this.network = network;
        this.timestamp = timestamp;
    }
    
    public RiCPUMetrics getCpu() {
        return cpu;
    }
    
    public RiMemoryMetrics getMemory() {
        return memory;
    }
    
    public RiDiskMetrics getDisk() {
        return disk;
    }
    
    public RiNetworkMetrics getNetwork() {
        return network;
    }
    
    public long getTimestamp() {
        return timestamp;
    }
}
