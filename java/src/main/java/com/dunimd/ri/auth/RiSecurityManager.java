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

package com.dunimd.ri.auth;

import com.dunimd.ri.NativeLoader;

/**
 * Security manager for Ri.
 * 
 * Provides encryption, decryption, and HMAC signing utilities.
 */
public class RiSecurityManager {
    static {
        NativeLoader.autoLoad();
    }
    
    public static String encrypt(String plaintext) {
        return encrypt0(plaintext);
    }
    
    private static native String encrypt0(String plaintext);
    
    public static String decrypt(String encrypted) {
        return decrypt0(encrypted);
    }
    
    private static native String decrypt0(String encrypted);
    
    public static String hmacSign(String data) {
        return hmacSign0(data);
    }
    
    private static native String hmacSign0(String data);
    
    public static boolean hmacVerify(String data, String signature) {
        return hmacVerify0(data, signature);
    }
    
    private static native boolean hmacVerify0(String data, String signature);
    
    public static String generateEncryptionKey() {
        return generateEncryptionKey0();
    }
    
    private static native String generateEncryptionKey0();
    
    public static String generateHmacKey() {
        return generateHmacKey0();
    }
    
    private static native String generateHmacKey0();
}
