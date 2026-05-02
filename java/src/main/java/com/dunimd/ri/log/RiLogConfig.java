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

import com.dunimd.ri.NativeLoader;

/**
 * Log configuration for Ri.
 * 
 * Provides configuration options for the logging system.
 * Use the Builder pattern to create instances.
 */
public class RiLogConfig {
    private long nativePtr;
    
    static {
        NativeLoader.autoLoad();
    }
    
    private RiLogConfig() {
        this.nativePtr = new0();
    }
    
    private native long new0();
    
    /**
     * Close and release resources.
     */
    public void close() {
        if (nativePtr != 0) {
            free0(nativePtr);
            nativePtr = 0;
        }
    }
    
    private native void free0(long ptr);
    
    /**
     * Get the native pointer for internal use.
     * 
     * @return the native pointer
     */
    public long getNativePtr() {
        return nativePtr;
    }
    
    private native void setLevel0(long ptr, int level);
    
    private native void setConsoleEnabled0(long ptr, boolean enabled);
    
    private native void setFileEnabled0(long ptr, boolean enabled);
    
    private native void setSamplingDefault0(long ptr, float rate);
    
    private native void setFileName0(long ptr, String fileName);
    
    private native void setJsonFormat0(long ptr, boolean jsonFormat);
    
    private native void setRotateWhen0(long ptr, String rotateWhen);
    
    private native void setMaxBytes0(long ptr, long maxBytes);
    
    private native void setColorBlocks0(long ptr, boolean colorBlocks);
    
    @Override
    protected void finalize() {
        close();
    }
    
    /**
     * Builder for RiLogConfig.
     */
    public static class Builder {
        private RiLogLevel level = RiLogLevel.INFO;
        private boolean consoleEnabled = true;
        private boolean fileEnabled = false;
        private float samplingDefault = 1.0f;
        private String fileName = "app.log";
        private boolean jsonFormat = false;
        private String rotateWhen = "none";
        private long maxBytes = 10 * 1024 * 1024;
        private boolean colorBlocks = true;
        
        /**
         * Set the minimum log level.
         * 
         * @param level the log level
         * @return this builder
         */
        public Builder level(RiLogLevel level) {
            this.level = level;
            return this;
        }
        
        /**
         * Enable or disable console logging.
         * 
         * @param enabled true to enable
         * @return this builder
         */
        public Builder consoleEnabled(boolean enabled) {
            this.consoleEnabled = enabled;
            return this;
        }
        
        /**
         * Enable or disable file logging.
         * 
         * @param enabled true to enable
         * @return this builder
         */
        public Builder fileEnabled(boolean enabled) {
            this.fileEnabled = enabled;
            return this;
        }
        
        /**
         * Set the default sampling rate.
         * 
         * @param rate the sampling rate (0.0 to 1.0)
         * @return this builder
         */
        public Builder samplingDefault(float rate) {
            this.samplingDefault = Math.max(0.0f, Math.min(1.0f, rate));
            return this;
        }
        
        /**
         * Set the log file name.
         * 
         * @param fileName the file name
         * @return this builder
         */
        public Builder fileName(String fileName) {
            this.fileName = fileName;
            return this;
        }
        
        /**
         * Enable or disable JSON format.
         * 
         * @param jsonFormat true to enable JSON format
         * @return this builder
         */
        public Builder jsonFormat(boolean jsonFormat) {
            this.jsonFormat = jsonFormat;
            return this;
        }
        
        /**
         * Set when to rotate logs.
         * 
         * @param rotateWhen the rotation policy ("none" or "size")
         * @return this builder
         */
        public Builder rotateWhen(String rotateWhen) {
            this.rotateWhen = rotateWhen;
            return this;
        }
        
        /**
         * Set the maximum file size in bytes before rotation.
         * 
         * @param maxBytes the maximum bytes
         * @return this builder
         */
        public Builder maxBytes(long maxBytes) {
            this.maxBytes = maxBytes;
            return this;
        }
        
        /**
         * Enable or disable color blocks in log output.
         * 
         * @param colorBlocks true to enable
         * @return this builder
         */
        public Builder colorBlocks(boolean colorBlocks) {
            this.colorBlocks = colorBlocks;
            return this;
        }
        
        /**
         * Build the RiLogConfig instance.
         * 
         * @return a new RiLogConfig instance
         */
        public RiLogConfig build() {
            RiLogConfig config = new RiLogConfig();
            config.setLevel0(config.nativePtr, level.getOrdinal());
            config.setConsoleEnabled0(config.nativePtr, consoleEnabled);
            config.setFileEnabled0(config.nativePtr, fileEnabled);
            config.setSamplingDefault0(config.nativePtr, samplingDefault);
            config.setFileName0(config.nativePtr, fileName);
            config.setJsonFormat0(config.nativePtr, jsonFormat);
            config.setRotateWhen0(config.nativePtr, rotateWhen);
            config.setMaxBytes0(config.nativePtr, maxBytes);
            config.setColorBlocks0(config.nativePtr, colorBlocks);
            return config;
        }
    }
    
    /**
     * Create a new Builder instance.
     * 
     * @return a new Builder
     */
    public static Builder builder() {
        return new Builder();
    }
}
