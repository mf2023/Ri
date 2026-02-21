// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

package com.dunimd.dmsc;

import java.io.*;
import java.nio.file.*;

/**
 * Native library loader for DMSC.
 * 
 * This class automatically loads the appropriate native library for the current
 * platform from the JAR file. Users do not need to manually configure anything.
 * 
 * Supported platforms:
 * - Windows (x64, x86)
 * - Linux (x64, arm64)
 * - macOS (x64, arm64)
 */
public class NativeLoader {
    private static boolean loaded = false;
    private static final Object lock = new Object();
    private static final String TEMP_DIR_NAME = "dmsc-native";
    
    /**
     * Automatically load the native library for the current platform.
     * This method is thread-safe and will only load the library once.
     * 
     * @throws RuntimeException if the native library cannot be loaded
     */
    public static void autoLoad() {
        synchronized (lock) {
            if (loaded) {
                return;
            }
            
            try {
                String os = detectOS();
                String arch = detectArch();
                String libName = getLibName(os);
                String libPath = "native/" + os + "/" + arch + "/" + libName;
                
                File tempFile = extractToTemp(libPath, libName);
                System.load(tempFile.getAbsolutePath());
                
                loaded = true;
            } catch (Exception e) {
                throw new RuntimeException("Failed to load DMSC native library: " + e.getMessage(), e);
            }
        }
    }
    
    /**
     * Check if the native library has been loaded.
     * 
     * @return true if loaded, false otherwise
     */
    public static boolean isLoaded() {
        return loaded;
    }
    
    /**
     * Detect the current operating system.
     * 
     * @return the OS name (windows, linux, macos)
     * @throws UnsupportedOperationException if the OS is not supported
     */
    private static String detectOS() {
        String os = System.getProperty("os.name").toLowerCase();
        
        if (os.contains("win")) {
            return "windows";
        } else if (os.contains("mac")) {
            return "macos";
        } else if (os.contains("nux") || os.contains("nix")) {
            return "linux";
        }
        
        throw new UnsupportedOperationException("Unsupported OS: " + os);
    }
    
    /**
     * Detect the current CPU architecture.
     * 
     * @return the architecture name (x64, x86, arm64)
     */
    private static String detectArch() {
        String arch = System.getProperty("os.arch").toLowerCase();
        
        if (arch.contains("64") && !arch.contains("arm")) {
            return "x64";
        } else if (arch.contains("arm") || arch.contains("aarch")) {
            return "arm64";
        } else if (arch.contains("86")) {
            return "x86";
        }
        
        return "x64";
    }
    
    /**
     * Get the native library name for the given OS.
     * 
     * @param os the operating system name
     * @return the library file name
     */
    private static String getLibName(String os) {
        switch (os) {
            case "windows":
                return "dmsc.dll";
            case "macos":
                return "libdmsc.dylib";
            default:
                return "libdmsc.so";
        }
    }
    
    /**
     * Extract the native library from the JAR to a temporary file.
     * 
     * @param libPath the path to the library in the JAR
     * @param libName the library file name
     * @return the temporary file containing the library
     * @throws IOException if extraction fails
     */
    private static File extractToTemp(String libPath, String libName) throws IOException {
        InputStream in = NativeLoader.class.getClassLoader().getResourceAsStream(libPath);
        
        if (in == null) {
            throw new FileNotFoundException("Native library not found: " + libPath + 
                ". Please ensure you are using a supported platform.");
        }
        
        File tempDir = new File(System.getProperty("java.io.tmpdir"), TEMP_DIR_NAME);
        if (!tempDir.exists()) {
            tempDir.mkdirs();
        }
        
        File tempFile = new File(tempDir, libName + "_" + System.currentTimeMillis());
        
        try {
            Files.copy(in, tempFile.toPath(), StandardCopyOption.REPLACE_EXISTING);
        } finally {
            in.close();
        }
        
        tempFile.deleteOnExit();
        
        return tempFile;
    }
}
