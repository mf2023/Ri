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
 * Network metrics for monitoring network usage.
 */
public class RiNetworkMetrics {
    private final long totalReceivedBytes;
    private final long totalTransmittedBytes;
    private final long receivedBytesPerSec;
    private final long transmittedBytesPerSec;
    private final long totalReceivedPackets;
    private final long totalTransmittedPackets;
    private final long receivedPacketsPerSec;
    private final long transmittedPacketsPerSec;
    
    public RiNetworkMetrics(
        long totalReceivedBytes,
        long totalTransmittedBytes,
        long receivedBytesPerSec,
        long transmittedBytesPerSec,
        long totalReceivedPackets,
        long totalTransmittedPackets,
        long receivedPacketsPerSec,
        long transmittedPacketsPerSec
    ) {
        this.totalReceivedBytes = totalReceivedBytes;
        this.totalTransmittedBytes = totalTransmittedBytes;
        this.receivedBytesPerSec = receivedBytesPerSec;
        this.transmittedBytesPerSec = transmittedBytesPerSec;
        this.totalReceivedPackets = totalReceivedPackets;
        this.totalTransmittedPackets = totalTransmittedPackets;
        this.receivedPacketsPerSec = receivedPacketsPerSec;
        this.transmittedPacketsPerSec = transmittedPacketsPerSec;
    }
    
    public long getTotalReceivedBytes() {
        return totalReceivedBytes;
    }
    
    public long getTotalTransmittedBytes() {
        return totalTransmittedBytes;
    }
    
    public long getReceivedBytesPerSec() {
        return receivedBytesPerSec;
    }
    
    public long getTransmittedBytesPerSec() {
        return transmittedBytesPerSec;
    }
    
    public long getTotalReceivedPackets() {
        return totalReceivedPackets;
    }
    
    public long getTotalTransmittedPackets() {
        return totalTransmittedPackets;
    }
    
    public long getReceivedPacketsPerSec() {
        return receivedPacketsPerSec;
    }
    
    public long getTransmittedPacketsPerSec() {
        return transmittedPacketsPerSec;
    }
}
