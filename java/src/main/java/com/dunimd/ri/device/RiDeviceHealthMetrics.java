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

import com.dunimd.ri.NativeLoader;

/**
 * Device health metrics for Ri.
 * 
 * Contains health metrics for monitoring device performance and reliability.
 */
public class RiDeviceHealthMetrics {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    public RiDeviceHealthMetrics() {
        this.nativePtr = new0();
    }
    
    RiDeviceHealthMetrics(long ptr) {
        this.nativePtr = ptr;
    }
    
    private native long new0();
    
    /**
     * Get the CPU usage percentage (0-100).
     * 
     * @return the CPU usage percentage
     */
    public double getCpuUsagePercent() {
        return getCpuUsagePercent0(nativePtr);
    }
    
    private native double getCpuUsagePercent0(long ptr);
    
    /**
     * Set the CPU usage percentage (0-100).
     * 
     * @param percent the CPU usage percentage
     */
    public void setCpuUsagePercent(double percent) {
        setCpuUsagePercent0(nativePtr, percent);
    }
    
    private native void setCpuUsagePercent0(long ptr, double percent);
    
    /**
     * Get the memory usage percentage (0-100).
     * 
     * @return the memory usage percentage
     */
    public double getMemoryUsagePercent() {
        return getMemoryUsagePercent0(nativePtr);
    }
    
    private native double getMemoryUsagePercent0(long ptr);
    
    /**
     * Set the memory usage percentage (0-100).
     * 
     * @param percent the memory usage percentage
     */
    public void setMemoryUsagePercent(double percent) {
        setMemoryUsagePercent0(nativePtr, percent);
    }
    
    private native void setMemoryUsagePercent0(long ptr, double percent);
    
    /**
     * Get the device temperature in Celsius.
     * 
     * @return the temperature in Celsius
     */
    public double getTemperatureCelsius() {
        return getTemperatureCelsius0(nativePtr);
    }
    
    private native double getTemperatureCelsius0(long ptr);
    
    /**
     * Set the device temperature in Celsius.
     * 
     * @param temperature the temperature in Celsius
     */
    public void setTemperatureCelsius(double temperature) {
        setTemperatureCelsius0(nativePtr, temperature);
    }
    
    private native void setTemperatureCelsius0(long ptr, double temperature);
    
    /**
     * Get the error count.
     * 
     * @return the error count
     */
    public long getErrorCount() {
        return getErrorCount0(nativePtr);
    }
    
    private native long getErrorCount0(long ptr);
    
    /**
     * Set the error count.
     * 
     * @param count the error count
     */
    public void setErrorCount(long count) {
        setErrorCount0(nativePtr, count);
    }
    
    private native void setErrorCount0(long ptr, long count);
    
    /**
     * Get the throughput in operations per second.
     * 
     * @return the throughput
     */
    public long getThroughput() {
        return getThroughput0(nativePtr);
    }
    
    private native long getThroughput0(long ptr);
    
    /**
     * Set the throughput in operations per second.
     * 
     * @param throughput the throughput
     */
    public void setThroughput(long throughput) {
        setThroughput0(nativePtr, throughput);
    }
    
    private native void setThroughput0(long ptr, long throughput);
    
    /**
     * Get the network latency in milliseconds.
     * 
     * @return the network latency in ms
     */
    public double getNetworkLatencyMs() {
        return getNetworkLatencyMs0(nativePtr);
    }
    
    private native double getNetworkLatencyMs0(long ptr);
    
    /**
     * Set the network latency in milliseconds.
     * 
     * @param latencyMs the network latency in ms
     */
    public void setNetworkLatencyMs(double latencyMs) {
        setNetworkLatencyMs0(nativePtr, latencyMs);
    }
    
    private native void setNetworkLatencyMs0(long ptr, double latencyMs);
    
    /**
     * Get the disk IOPS.
     * 
     * @return the disk IOPS
     */
    public long getDiskIops() {
        return getDiskIops0(nativePtr);
    }
    
    private native long getDiskIops0(long ptr);
    
    /**
     * Set the disk IOPS.
     * 
     * @param iops the disk IOPS
     */
    public void setDiskIops(long iops) {
        setDiskIops0(nativePtr, iops);
    }
    
    private native void setDiskIops0(long ptr, long iops);
    
    /**
     * Get the response time in milliseconds.
     * 
     * @return the response time in ms
     */
    public double getResponseTimeMs() {
        return getResponseTimeMs0(nativePtr);
    }
    
    private native double getResponseTimeMs0(long ptr);
    
    /**
     * Set the response time in milliseconds.
     * 
     * @param responseTimeMs the response time in ms
     */
    public void setResponseTimeMs(double responseTimeMs) {
        setResponseTimeMs0(nativePtr, responseTimeMs);
    }
    
    private native void setResponseTimeMs0(long ptr, double responseTimeMs);
    
    /**
     * Release native resources.
     */
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    @Override
    protected void finalize() {
        close();
    }
}
