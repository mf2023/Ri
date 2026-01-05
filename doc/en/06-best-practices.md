<div align="center">

# Best Practices

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This chapter introduces best practices for using the DMSC framework to help you build efficient, reliable, and secure applications.

## 1. Project Structure

</div>

### 1.1 Recommended Project Structure

```
my-dms-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration-related code
│   ├── modules/             # Custom modules
│   │   ├── mod.rs
│   │   └── my_module.rs
│   ├── services/            # Business services
│   │   ├── mod.rs
│   │   └── user_service.rs
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── helpers.rs
├── config/                  # Configuration files
│   ├── config.yaml          # Main configuration file
│   ├── config.dev.yaml      # Development environment configuration
│   └── config.prod.yaml     # Production environment configuration
├── Cargo.toml
└── README.md
```

### 1.2 Module Organization

- **Organize by functionality**: Group related functionality into the same module
- **Maintain module independence**: Minimize dependencies between modules
- **Use clear naming**: Module and function names should clearly express their purpose

### 1.3 Code Organization

- **Use the prelude module**: Import commonly used types via `use dms::prelude::*`
- **Separate concerns**: Separate configuration, business logic, and utility functions
- **Use layered architecture**: Adopt a layered architecture, such as controller, service, and data access layers

<div align="center">

## 2. Configuration Management

</div>

### 2.1 Multi-environment Configuration

- **Use different configuration files**: Create different configuration files for different environments
- **Leverage environment variables**: Use environment variables to override values in configuration files
- **Configuration inheritance**: Use a base configuration file, with other environment files inheriting and overriding specific values

### 2.2 Secure Storage of Sensitive Information

- **Avoid hardcoding**: Do not hardcode sensitive information in configuration files
- **Use environment variables**: For sensitive information such as keys and credentials, use environment variables
- **Use secret management services**: In production environments, use dedicated secret management services

```yaml
# Bad practice
auth:
  jwt:
    secret: "hardcoded-secret-key"

# Good practice
auth:
  jwt:
    secret: ${DMSC_JWT_SECRET}
```

### 2.3 Configuration Validation

- **Validate configuration integrity**: Verify all required configuration items when the application starts
- **Use type-safe configuration**: Map configuration to strongly-typed structures
- **Provide reasonable default values**: Offer sensible defaults for optional configuration items

### 2.4 Configuration Hot Reload

- **Enable hot reload**: Enable hot reload for configurations that need dynamic adjustment
- **Handle configuration changes**: Implement listeners for configuration changes to update application state promptly
- **Limit hot reload scope**: Only enable hot reload for safe configuration items

<div align="center">

## 3. Logging and Monitoring

</div>

### 3.1 Logging Best Practices

- **Use structured logging**: Use JSON-formatted structured logs for easier log analysis
- **Include context information**: Include request IDs, user IDs, and other context information in logs
- **Set appropriate log levels**:
  - DEBUG: Detailed debugging information, enabled only in development environments
  - INFO: Normal business process information
  - WARN: Warning messages that need attention but don't affect normal operation
  - ERROR: Error messages that affect normal operation
- **Avoid logging sensitive information**: Do not log passwords, API keys, or other sensitive information
- **Implement log rotation**: Configure reasonable log rotation policies to avoid oversized log files

### 3.2 Monitoring Best Practices

- **Enable observability**: Enable observability in all environments
- **Define key metrics**: Identify and monitor key business and system metrics
- **Set alert thresholds**: Establish reasonable alert thresholds for key metrics
- **Use distributed tracing**: In distributed systems, use distributed tracing to track request flows
- **Correlate logs and traces**: Link logs and trace information for easier troubleshooting

### 3.3 Metric Naming Conventions

- **Use dot-separated naming**: Like `service.requests.total`
- **Include dimension information**: Like `service.requests.total{method="GET", status="200"}`
- **Use consistent naming**: Apply consistent metric naming conventions throughout the application

<div align="center">

## 4. Performance Optimization

</div>

### 4.1 Asynchronous Programming

- **Prioritize async APIs**: Fully leverage DMSC's async APIs
- **Avoid blocking operations**: Avoid using blocking operations in async code
- **Use tokio::spawn appropriately**: For CPU-intensive tasks, use `tokio::spawn` to execute in separate tasks
- **Use async locks**: For shared resources, use async locks like `tokio::sync::Mutex`

### 4.2 Caching Strategies

- **Use caching judiciously**: Cache hot data to reduce database queries
- **Set appropriate expiration times**: Configure suitable cache expiration times based on data update frequency
- **Implement cache penetration protection**: Use bloom filters or similar mechanisms to prevent cache penetration
- **Consider cache consistency**: Promptly update or invalidate relevant caches when data changes

### 4.3 Resource Management

- **Use connection pools**: Utilize connection pools for databases, Redis, and other resources
- **Set appropriate connection pool sizes**: Configure connection pool sizes based on system resources and load
- **Release resources promptly**: Use `async drop` or the `Drop` trait to release resources promptly
- **Limit concurrency**: Implement rate limiting for external service calls

### 4.4 Code Optimization

- **Reduce memory allocations**: Use references instead of cloning, and pre-allocate memory with `String::with_capacity`
- **Avoid unnecessary calculations**: Cache calculation results to avoid redundant computations
- **Use efficient data structures**: Select appropriate data structures based on usage scenarios
- **Batch operations**: Use batch operations for database operations to reduce network overhead

<div align="center">

## 5. Security Design

</div>

### 5.1 Authentication and Authorization

- **Use strong password hashing**: Implement bcrypt, Argon2, or other strong password hashing algorithms
- **Apply the principle of least privilege**: Users should only access resources and functions they need
- **Use HTTPS**: Always use HTTPS in production environments
- **Rotate keys regularly**: Periodically rotate JWT keys and OAuth credentials
- **Implement CSRF protection**: For web applications, implement CSRF protection

### 5.2 Input Validation

- **Validate all inputs**: Validate all user inputs
- **Use type-safe inputs**: Receive user inputs in strongly-typed structures
- **Prevent injection attacks**: Use parameterized queries to avoid SQL injection, command injection, etc.
- **Prevent XSS attacks**: Properly escape output to prevent cross-site scripting attacks

### 5.3 Secure Configuration

- **Disable unnecessary services**: Only enable services and ports required by the application
- **Use secure default configurations**: Apply secure defaults, such as disabling debug mode
- **Update dependencies regularly**: Keep dependencies updated to fix security vulnerabilities
- **Use secure random number generators**: Use secure RNGs like `rand::thread_rng()`

### 5.4 Security Logging

- **Record security events**: Log all authentication, authorization, and access control events
- **Monitor abnormal behavior**: Track unusual login attempts, privilege escalations, and other suspicious activities
- **Implement audit logging**: Create audit logs to facilitate operation tracking

<div align="center">

## 6. Module Usage

</div>

### 6.1 Use Modules on Demand

- **Add only necessary modules**: Include only the modules required by the application
- **Avoid unnecessary dependencies**: Depend only on the features needed by the application
- **Configure modules appropriately**: Configure modules based on application requirements, avoiding over-configuration

### 6.2 Custom Modules

- **Implement the DMSCModule trait**: Follow DMSC's module interface specifications
- **Handle lifecycle events**: Properly implement module initialization, startup, and shutdown methods
- **Use the service context**: Access other modules' functionality through the service context
- **Return meaningful errors**: Provide descriptive error messages in module methods

### 6.3 Module Dependencies

- **Explicitly declare dependencies**: Clearly state dependency relationships in modules
- **Handle dependency order**: Control module loading order through priorities
- **Avoid circular dependencies**: Design modules to prevent circular dependencies

<div align="center">

## 7. Error Handling

</div>

### 7.1 Unified Error Types

- **Use DMSCResult**: Return `DMSCResult` from all public methods
- **Provide meaningful error messages**: Error messages should clearly describe the cause
- **Include context information**: Embed relevant context information in errors
- **Use error chains**: Preserve original error information through error chaining

### 7.2 Error Propagation

- **Use the ? operator**: Automatically propagate errors with the `?` operator
- **Avoid excessive unwrap calls**: For potentially failing operations, avoid using `unwrap()`
- **Handle errors appropriately**: Manage errors at the appropriate level, providing user-friendly messages

### 7.3 Error Logging

- **Log errors**: Record error information at the appropriate level
- **Include complete error context**: Embed full error context in log entries
- **Differentiate error types**: Choose appropriate log levels based on error types

<div align="center">

## 8. Test Design

</div>

### 8.1 Unit Testing

- **Test core functionality**: Verify core business logic and utility functions
- **Use mocks**: Employ mock objects for external dependencies
- **Test edge cases**: Validate boundary conditions and exceptional scenarios
- **Maintain test independence**: Test cases should be mutually independent

### 8.2 Integration Testing

- **Test module interactions**: Verify integration between modules
- **Test configuration loading**: Validate loading and handling of different configurations
- **Test lifecycle events**: Verify application initialization, startup, and shutdown

### 8.3 End-to-End Testing

- **Test complete flows**: Validate full request-to-response processes
- **Simulate real scenarios**: Model real user behavior and load
- **Test in different environments**: Run end-to-end tests in various environments

### 8.4 Testing Best Practices

- **Use testing frameworks**: Leverage Rust's testing framework, such as `cargo test`
- **Monitor test coverage**: Track test coverage to ensure critical code is tested
- **Automate testing**: Integrate tests into CI/CD workflows
- **Run tests regularly**: Execute all tests periodically to ensure code quality

<div align="center">

## 9. Deployment Design

</div>

### 9.1 Containerized Deployment

- **Use Docker**: Containerize applications with Docker
- **Implement multi-stage builds**: Reduce image size with multi-stage builds
- **Set appropriate resource limits**: Configure suitable CPU and memory limits
- **Implement health checks**: Provide health checks for container orchestration systems

### 9.2 Configuration Management

- **Use configuration centers**: In production environments, manage configurations with a configuration center
- **Encrypt sensitive configurations**: Use encrypted storage for sensitive configurations
- **Version control configurations**: Apply version control to configurations for easier rollback

### 9.3 Rolling Updates

- **Use rolling updates**: Adopt rolling update strategies to avoid service interruptions
- **Implement graceful shutdown**: Properly handle SIGTERM signals for graceful shutdown
- **Use health checks**: Employ health checks during updates to ensure service availability

<div align="center">

## 11. Key Management and Cryptography Security

</div>

### 11.1 Key Management Best Practices

#### 11.1.1 Key Generation

- **Use secure random number generators**: Always use cryptographically secure random number generators for key generation
- **Choose sufficient key length**: Select appropriate key length based on security requirements, recommend at least 2048 bits for RSA and 256 bits for ECC
- **Avoid weak keys**: Ensure generated keys are not known weak keys or default keys
- **Key entropy**: Ensure keys have sufficient entropy, recommend at least 128-bit security strength

```rust
// Good practice: Use secure random number generator
use ring::rand::SystemRandom;

let rng = SystemRandom::new();
let mut key = [0u8; 32];
rng.fill(&mut key).map_err(|_| DMSCError::SecurityViolation("Failed to generate secure random key".to_string()))?;

// Bad practice: Using pseudorandom or fixed keys
let weak_key = [1, 2, 3, 4, 5, 6, 7, 8]; // Not secure
```

#### 11.1.2 Key Storage

- **Never hardcode keys**: Never hardcode keys in source code or configuration files
- **Use key management services**: In production environments, use HSM (Hardware Security Module) or KMS (Key Management Service)
- **Environment variable storage**: Use environment variables for temporary key storage
- **Encrypted storage**: Static keys should be encrypted using a master key before storage

```yaml
# Bad practice
auth:
  jwt:
    secret: "my-super-secret-key-12345"

# Good practice
auth:
  jwt:
    secret: ${DMSC_JWT_SECRET}  # Read from environment variable
```

#### 11.1.3 Key Rotation

- **Rotate keys regularly**: Establish a key rotation policy, recommend rotating JWT keys every 90 days
- **Dual-key transition**: Support both old and new key verification during rotation to ensure smooth transition
- **Key version management**: Assign unique identifiers to each key version for historical key tracking
- **Automated rotation**: Implement automated key rotation processes to reduce human error risk

```rust
// Implement key rotation support
struct KeyRotationManager {
    current_key: EncodingKey,
    previous_key: Option<EncodingKey>,
    key_version: u64,
    rotation_date: chrono::DateTime<chrono::Utc>,
}

impl KeyRotationManager {
    pub fn rotate_key(&mut self, new_secret: &[u8]) {
        self.previous_key = Some(std::mem::replace(
            &mut self.current_key,
            EncodingKey::from_secret(new_secret),
        ));
        self.key_version += 1;
        self.rotation_date = chrono::Utc::now();
    }

    pub fn validate_with_rotation(&self, token: &str) -> Result<JWTClaims, DMSCError> {
        // First try to validate with current key
        if let Ok(claims) = self.validate_current(token) {
            return Ok(claims);
        }
        // If it fails, try using the previous key (support rotation transition period)
        self.previous_key
            .as_ref()
            .ok_or_else(|| DMSCError::SecurityViolation("Token validation failed".to_string()))?;
        // ... validation logic
        Ok(claims)
    }
}
```

#### 11.1.4 Key Destruction

- **Secure deletion**: When keys are no longer needed, securely delete key data from memory
- **Use sensitive data types**: Use libraries like `zeroize` to securely overwrite memory
- **Audit trail**: Record the creation, usage, and destruction time of keys

```rust
use zeroize::Zeroize;

struct SensitiveKey {
    data: Vec<u8>,
}

impl Drop for SensitiveKey {
    fn drop(&mut self) {
        self.data.zeroize();
    }
}
```

### 11.2 JWT Security Best Practices

#### 11.2.1 Algorithm Selection

- **Use HS256 or higher-level algorithms**: Recommend using HS256, RS256, or ES256
- **Verify algorithm**: Always verify the algorithm claim when validating JWT to prevent algorithm confusion attacks
- **Disable none algorithm**: Ensure validation logic rejects tokens with alg="none"

```rust
// Ensure algorithm verification
let validation = Validation::new(Algorithm::HS256);
let claims = decode::<JWTClaims>(token, &key, &validation)?;
```

#### 11.2.2 Token Security

- **Set reasonable expiration time**: JWT expiration time is recommended to be set between 15 minutes and 1 hour
- **Use short-lived access tokens**: Use short-lived access tokens with refresh token mechanism
- **Verify issued time**: Verify the iat (issued at) claim to prevent using old tokens
- **Secure token storage**: Store tokens securely on the client side to prevent XSS attacks

#### 11.2.3 Sensitive Information Handling

- **Do not store sensitive information in JWT**: JWT uses Base64 encoding and can be easily decoded
- **Store only necessary claims**: Only include necessary information like user ID and roles
- **Encrypt sensitive data**: If sensitive data needs to be transmitted in JWT, encrypt it first

### 11.3 Post-Quantum Cryptography Guide

#### 11.3.1 Algorithm Selection

DMSC supports multiple post-quantum cryptographic algorithms. The choice of algorithm depends on the specific scenario:

| Algorithm | Type | Recommended Scenario | Security Level |
|:----------|:-----|:---------------------|:---------------|
| **Kyber-512** | KEM | Key encapsulation, SSL/TLS | NIST Level 5 |
| **Dilithium-5** | Digital signature | Authentication, software signing | NIST Level 5 |
| **Falcon-512** | Digital signature | Scenarios requiring compact signatures | NIST Level 4 |

```rust
use dmsc::protocol::post_quantum::DMSCPostQuantumManager;

// Generate post-quantum key pair
let pq_manager = DMSCPostQuantumManager::new();
let (public_key, private_key) = pq_manager.generate_kyber_keypair()?;

// Use Kyber for key encapsulation
let (ciphertext, shared_secret) = pq_manager.kyber_encapsulate(&public_key)?;
```

#### 11.3.2 Hybrid Encryption Mode

During the transition period, it is recommended to use hybrid encryption mode with both traditional and post-quantum algorithms:

```rust
struct HybridEncryption {
    traditional_kem: Box<dyn TraditionalKEM>,
    post_quantum_kem: Box<dyn PostQuantumKEM>,
}

impl HybridEncryption {
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<HybridCiphertext, DMSCError> {
        // 1. Generate random session key
        let session_key = self.generate_session_key()?;
        // 2. Encapsulate session key with both traditional and post-quantum KEM
        let traditional_ciphertext = self.traditional_kem.encapsulate(&session_key)?;
        let pq_ciphertext = self.post_quantum_kem.encapsulate(&session_key)?;
        // 3. Encrypt data with session key
        let encrypted_data = self.symmetric_encrypt(&session_key, plaintext)?;
        Ok(HybridCiphertext { traditional_ciphertext, pq_ciphertext, encrypted_data })
    }
}
```

#### 11.3.3 Migration Strategy

- **Gradual migration**: Gradually migrate the system to post-quantum cryptography
- **Maintain compatibility**: Support both traditional and post-quantum algorithms during transition
- **Monitor and evaluate**: Continuously monitor new algorithm security and performance

### 11.4 National Cryptography (Guomi) Algorithm Usage Guide

#### 11.4.1 Algorithm Selection

National cryptography algorithms are suitable for scenarios requiring compliance with Chinese national cryptographic standards:

| Algorithm | Type | Usage | Standard |
|:----------|:-----|:------|:---------|
| **SM2** | Elliptic curve cryptography | Digital signature, key exchange | GM/T 0003 |
| **SM3** | Hash algorithm | Message digest | GM/T 0004 |
| **SM4** | Block cipher | Data encryption | GM/T 0002 |

```rust
use dmsc::protocol::guomi::DMSCGmCrypto;

// SM2 signature
let gm_crypto = DMSCGmCrypto::new();
let (sm2_public, sm2_private) = gm_crypto.generate_sm2_keypair()?;
let signature = gm_crypto.sm2_sign(&message, &sm2_private)?;

// SM3 hash
let sm3_hash = gm_crypto.sm3_hash(&data)?;

// SM4 encryption
let sm4_key = gm_crypto.generate_sm4_key()?;
let encrypted = gm_crypto.sm4_encrypt(&data, &sm4_key)?;
```

#### 11.4.2 Compliance Requirements

- **Key management**: National cryptography keys must use key management devices certified by the State Cryptography Administration
- **Algorithm compliance**: Ensure algorithms and implementations comply with GM/T series standards
- **Cryptographic module certification**: Cryptographic modules used should be certified by the State Cryptography Administration

### 11.5 HSM Integration Guide

#### 11.5.1 HSM Selection

- **Select certified HSM**: Use HSM certified to FIPS 140-2 Level 3 or higher
- **Vendor compatibility**: Ensure HSM is compatible with DMSC, supporting PKCS#11 or vendor-specific APIs
- **High-availability configuration**: In production environments, it is recommended to configure HSM clusters for high availability

#### 11.5.2 HSM Configuration

```rust
use dmsc::protocol::hsm::DMSCHSMInterface;

struct HSMConfig {
    pub pkcs11_library_path: String,
    pub slot_id: u32,
    pub pin: String,
    pub key_label: String,
}

impl DMSCHSMInterface for HSMConfig {
    fn initialize(&self) -> Result<(), DMSCError> {
        // Initialize PKCS#11 library
        // Open session
        // Login to HSM
        Ok(())
    }

    fn sign(&self, data: &[u8]) -> Result<Vec<u8>, DMSCError> {
        // Sign using HSM key
        Ok(signature)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, DMSCError> {
        // Decrypt using HSM key
        Ok(plaintext)
    }
}
```

#### 11.5.3 Key Lifecycle

- **Key generation**: Generate keys within HSM to avoid key export
- **Key usage**: Complete all encryption operations within HSM, private keys never leave HSM
- **Key backup**: Use HSM-provided backup mechanisms to securely back up keys
- **Key destruction**: Use HSM's secure destruction function to delete keys

### 11.6 Security Audit Recommendations

#### 11.6.1 Audit Scope

- **Authentication process**: Regularly review the security of authentication processes
- **Key management**: Review key generation, storage, usage, and destruction processes
- **Encryption implementation**: Review encryption algorithm implementation and usage
- **Access control**: Review permission control and access logs
- **Dependency audit**: Regularly review the security of third-party dependencies

#### 11.6.2 Audit Frequency

- **Code review**: Conduct code review after each major update
- **Penetration testing**: Conduct at least one professional penetration test annually
- **Security scanning**: Run automated security scanning tools monthly
- **Dependency audit**: Check dependency security advisories weekly

#### 11.6.3 Audit Tools

- **Static analysis**: Use cargo-audit, cargo-geiger to detect security risks
- **Dependency checking**: Use cargo-audit to check dependency vulnerabilities
- **Code quality**: Use clippy and rustsec to check for code security issues
- **Dynamic testing**: Use fuzz testing to discover potential vulnerabilities

```bash
# Run security checks regularly
cargo audit                    # Check dependency vulnerabilities
cargo clippy                   # Check code quality issues
cargo sec --check             # Security-related checks
```

#### 11.6.4 Response Process

- **Vulnerability reporting**: Establish vulnerability reporting and response process
- **Impact assessment**: Immediately assess the impact scope when vulnerabilities are discovered
- **Fix priority**: Determine fix priority based on vulnerability severity
- **Update strategy**: Develop security update release strategy

### 11.7 Security Configuration Checklist

The following configuration items are crucial for security:

| Configuration Item | Security Impact | Recommended Value |
|:-------------------|:----------------|:------------------|
| `auth.jwt.secret` | JWT signing key | At least 32 random characters |
| `auth.jwt.expiry` | Token validity period | Recommended 15-60 minutes |
| `encryption.key` | Data encryption key | Recommended 256 bits |
| `database.ssl` | Database connection | Must be enabled |
| `cache.redis.password` | Redis authentication | Strong password |
| `tls.enabled` | Transport encryption | Must be enabled |
| `tls.min_version` | TLS minimum version | 1.2 or higher |
| `rate_limit.enabled` | Brute force prevention | Recommended enabled |
| `audit.enabled` | Security audit | Must be enabled |

<div align="center">

## Summary

</div>

Following these best practices will help you build efficient, reliable, and secure DMSC applications. Of course, best practices are not set in stone, and you should choose the appropriate practices based on your application's specific requirements and scenarios.

During development, continuously learn and practice, accumulate experience, and consistently improve your application's design and implementation.

<div align="center">

## Next Steps

</div> 

- [Troubleshooting](./07-troubleshooting.md): Common issues and solutions
- [Glossary](./08-glossary.md): Core terminology explanations