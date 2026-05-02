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
 * Disk metrics for monitoring disk usage.
 */
public class RiDiskMetrics {
    private final long totalBytes;
    private final long usedBytes;
    private final long freeBytes;
    private final double usagePercent;
    private final long readBytes;
    private final long writeBytes;
    private final long readCount;
    private final long writeCount;
    
    public RiDiskMetrics(
        long totalBytes,
        long usedBytes,
        long freeBytes,
        double usagePercent,
        long readBytes,
        long writeBytes,
        long readCount,
        long writeCount
    ) {
        this.totalBytes = totalBytes;
        this.usedBytes = usedBytes;
        this.freeBytes = freeBytes;
        this.usagePercent = usagePercent;
        this.readBytes = readBytes;
        this.writeBytes = writeBytes;
        this.readCount = readCount;
        this.writeCount = writeCount;
    }
    
    public long getTotalBytes() {
        return totalBytes;
    }
    
    public long getUsedBytes() {
        return usedBytes;
    }
    
    public long getFreeBytes() {
        return freeBytes;
    }
    
    public double getUsagePercent() {
        return usagePercent;
    }
    
    public long getReadBytes() {
        return readBytes;
    }
    
    public long getWriteBytes() {
        return writeBytes;
    }
    
    public long getReadCount() {
        return readCount;
    }
    
    public long getWriteCount() {
        return writeCount;
    }
}
