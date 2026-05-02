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
 * Result of a health check execution.
 */
public class RiHealthCheckResult {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    /**
     * Create a new RiHealthCheckResult.
     * 
     * @param name the name of the health check
     * @param status the health status
     * @param message an optional message
     */
    public RiHealthCheckResult(String name, RiHealthStatus status, String message) {
        this.nativePtr = new0(name, status.getValue(), message);
    }
    
    private native long new0(String name, int status, String message);
    
    /**
     * Create a healthy result.
     * 
     * @param name the name of the health check
     * @param message an optional message
     * @return a healthy RiHealthCheckResult
     */
    public static RiHealthCheckResult healthy(String name, String message) {
        return new RiHealthCheckResult(name, RiHealthStatus.Healthy, message);
    }
    
    /**
     * Create a degraded result.
     * 
     * @param name the name of the health check
     * @param message an optional message
     * @return a degraded RiHealthCheckResult
     */
    public static RiHealthCheckResult degraded(String name, String message) {
        return new RiHealthCheckResult(name, RiHealthStatus.Degraded, message);
    }
    
    /**
     * Create an unhealthy result.
     * 
     * @param name the name of the health check
     * @param message an optional message
     * @return an unhealthy RiHealthCheckResult
     */
    public static RiHealthCheckResult unhealthy(String name, String message) {
        return new RiHealthCheckResult(name, RiHealthStatus.Unhealthy, message);
    }
    
    /**
     * Create an unknown result.
     * 
     * @param name the name of the health check
     * @param message an optional message
     * @return an unknown RiHealthCheckResult
     */
    public static RiHealthCheckResult unknown(String name, String message) {
        return new RiHealthCheckResult(name, RiHealthStatus.Unknown, message);
    }
    
    /**
     * Get the name of the health check.
     * 
     * @return the name
     */
    public String getName() {
        return getName0(nativePtr);
    }
    
    private native String getName0(long ptr);
    
    /**
     * Get the health status.
     * 
     * @return the status
     */
    public RiHealthStatus getStatus() {
        int statusValue = getStatus0(nativePtr);
        return RiHealthStatus.fromValue(statusValue);
    }
    
    private native int getStatus0(long ptr);
    
    /**
     * Get the message.
     * 
     * @return the message, or null if not set
     */
    public String getMessage() {
        return getMessage0(nativePtr);
    }
    
    private native String getMessage0(long ptr);
    
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
