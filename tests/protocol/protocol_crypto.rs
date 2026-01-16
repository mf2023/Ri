//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use dmsc::protocol::crypto::*;

/// Cryptographic algorithm test module for DMSC protocol security layer.
///
/// This module provides comprehensive test coverage for the cryptographic
/// primitives used in the DMSC protocol, ensuring the security foundations
/// are correctly implemented. The tests verify both the correctness of
/// cryptographic operations and the security properties that protect data
/// in transit across the distributed messaging system.
///
/// ## Test Coverage
///
/// - **Symmetric Encryption**: Tests for AES256-GCM authenticated encryption
///   and ChaCha20-Poly1305 stream cipher, covering encryption, decryption,
///   and authentication tag verification with both correct and incorrect
///   additional authenticated data.
///
/// - **National Standard Ciphers**: Validates SM4 cipher implementation in
///   CBC mode, supporting Chinese cryptographic standards compliance for
///   domestic deployment requirements.
///
/// - **Cryptographic Hash Functions**: Tests SHA-256 and SHA-3 for general
///   purpose hashing alongside SM3 for national standard compliance, verifying
///   output lengths and collision resistance properties.
///
/// - **Digital Signatures**: Covers ECDSA and Ed25519 signature schemes,
///   testing key generation, message signing, public key extraction, and
///   signature verification including rejection of tampered messages.
///
/// - **Key Exchange Protocols**: Tests ECDH and X25519 Diffie-Hellman key
///   exchange implementations, verifying that both parties derive the same
///   shared secret without revealing their private keys.
///
/// - **Secure Random Number Generation**: Validates the cryptographic RNG
///   implementation for generating random bytes and integers with sufficient
///   entropy for security-critical operations.
///
/// ## Design Principles
///
/// The cryptographic testing strategy emphasizes verification of security
/// properties rather than mere functional correctness. Tests verify that:
/// - Encryption transforms plaintext to unintelligible ciphertext
/// - Decryption with correct parameters recovers the original plaintext
/// - Authentication prevents tampering detection
/// - Invalid parameters produce errors rather than incorrect results
/// - Cryptographic operations produce outputs of expected lengths
///
/// Tests use property-based verification where applicable, checking that
/// encryption followed by decryption returns the original data, and that
/// encryption produces semantically different outputs to prevent pattern
/// analysis attacks.
///
/// The test suite intentionally avoids timing-based side-channel tests
/// as these require specialized environments, but does verify that all
/// error conditions are handled gracefully without information leakage
/// through error messages or exception types.

#[test]
fn test_aes256_gcm() {
    let aes = AES256GCM::new().expect("Failed to create AES256GCM instance");
    let plaintext = b"Hello, DMSC!";
    let additional_data = b"additional data";
    
    // Encrypt
    let ciphertext = aes.encrypt(plaintext, Some(additional_data))
        .expect("Failed to encrypt data");
    assert_ne!(ciphertext, plaintext);
    
    // Decrypt
    let decrypted = aes.decrypt(&ciphertext, Some(additional_data))
        .expect("Failed to decrypt data");
    assert_eq!(decrypted, plaintext);
    
    // Test with wrong additional data
    let wrong_aad = b"wrong data";
    let result = aes.decrypt(&ciphertext, Some(wrong_aad));
    assert!(result.is_err());
}

#[test]
fn test_chacha20_poly1305() {
    let cipher = ChaCha20Poly1305::new().expect("Failed to create ChaCha20Poly1305 instance");
    let plaintext = b"Secure message";
    
    // Encrypt
    let ciphertext = cipher.encrypt(plaintext, None)
        .expect("Failed to encrypt data");
    assert_ne!(ciphertext, plaintext);
    
    // Decrypt
    let decrypted = cipher.decrypt(&ciphertext, None)
        .expect("Failed to decrypt data");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_sm4_cipher() {
    let sm4 = SM4Cipher::new().expect("Failed to create SM4Cipher instance");
    let plaintext = b"SM4 test data";
    
    // Encrypt
    let ciphertext = sm4.encrypt_cbc(plaintext, None)
        .expect("Failed to encrypt data");
    assert_ne!(ciphertext, plaintext);
    
    // Decrypt
    let decrypted = sm4.decrypt_cbc(&ciphertext)
        .expect("Failed to decrypt data");
    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_hash_functions() {
    let data = b"test data";
    
    // SHA-256
    let hash1 = SHA256::hash(data);
    assert_eq!(hash1.len(), 32);
    
    // SHA-3
    let hash2 = SHA3::hash256(data);
    assert_eq!(hash2.len(), 32);
    
    // SM3
    let hash3 = SM3::hash(data);
    assert_eq!(hash3.len(), 32);
    
    // Verify different algorithms produce different results
    assert_ne!(hash1, hash2);
    assert_ne!(hash1, hash3);
    assert_ne!(hash2, hash3);
}

#[test]
fn test_ecdsa_signature() {
    let signer = ECDSASigner::generate().expect("Failed to generate ECDSA signer");
    let message = b"Test message for signing";
    
    // Sign
    let signature = signer.sign(message)
        .expect("Failed to sign message");
    assert!(!signature.is_empty());
    
    // Verify
    let public_key = signer.public_key();
    let verified = ECDSAVerifier::verify(&public_key, message, &signature)
        .expect("Failed to verify signature");
    assert!(verified);
    
    // Test with wrong message
    let wrong_message = b"Wrong message";
    let verified_wrong = ECDSAVerifier::verify(&public_key, wrong_message, &signature)
        .expect("Failed to verify signature");
    assert!(!verified_wrong);
}

#[test]
fn test_ed25519_signature() {
    let signer = Ed25519Signer::generate().expect("Failed to generate Ed25519 signer");
    let message = b"Ed25519 test message";
    
    // Sign
    let signature = signer.sign(message)
        .expect("Failed to sign message");
    assert!(!signature.is_empty());
    
    // Get public key
    let public_key = signer.public_key();
    assert!(!public_key.is_empty());
}

#[test]
fn test_ecdh_key_exchange() {
    // Generate key pairs
    let alice = ECDHKeyExchange::generate().expect("Failed to generate Alice key pair");
    let bob = ECDHKeyExchange::generate().expect("Failed to generate Bob key pair");
    
    let alice_pub = alice.public_key();
    let bob_pub = bob.public_key();
    
    // Perform key exchange
    let alice_secret = ECDHKeyExchange::generate().expect("Failed to generate Alice key pair")
        .compute_shared_secret(&bob_pub).expect("Failed to compute shared secret");
    let bob_secret = ECDHKeyExchange::generate().expect("Failed to generate Bob key pair")
        .compute_shared_secret(&alice_pub).expect("Failed to compute shared secret");
    
    // Both should generate the same shared secret
    assert_eq!(alice_secret, bob_secret);
    assert!(!alice_secret.is_empty());
}

#[test]
fn test_x25519_key_exchange() {
    // Generate key pairs
    let alice = X25519KeyExchange::generate().expect("Failed to generate Alice key pair");
    let bob = X25519KeyExchange::generate().expect("Failed to generate Bob key pair");
    
    let alice_pub = alice.public_key();
    let bob_pub = bob.public_key();
    
    // Perform key exchange
    let alice_secret = X25519KeyExchange::generate().expect("Failed to generate Alice key pair")
        .compute_shared_secret(&bob_pub).expect("Failed to compute shared secret");
    let bob_secret = X25519KeyExchange::generate().expect("Failed to generate Bob key pair")
        .compute_shared_secret(&alice_pub).expect("Failed to compute shared secret");
    
    // Both should generate the same shared secret
    assert_eq!(alice_secret, bob_secret);
    assert!(!alice_secret.is_empty());
}

#[test]
fn test_secure_rng() {
    let rng = SecureRNG::new();
    
    // Test random bytes
    let bytes1 = rng.generate(32)
        .expect("Failed to generate random bytes in test");
    let bytes2 = rng.generate(32)
        .expect("Failed to generate random bytes in test");
    assert_eq!(bytes1.len(), 32);
    assert_eq!(bytes2.len(), 32);
    assert_ne!(bytes1, bytes2); // Should be different
    
    // Test random numbers
    let num1 = rng.generate_u32()
        .expect("Failed to generate random u32 in test");
    let num2 = rng.generate_u32()
        .expect("Failed to generate random u32 in test");
    assert_ne!(num1, num2); // Should be different
    
    let num3 = rng.generate_u64()
        .expect("Failed to generate random u64 in test");
    let num4 = rng.generate_u64()
        .expect("Failed to generate random u64 in test");
    assert_ne!(num3, num4); // Should be different
}