<div align="center">
<img src="assets/svg/ri.svg" width="36" height="36">
</div>

The Ri (Ri) project takes security seriously. This document outlines our security policy, including supported versions, how to report vulnerabilities, and our disclosure process.

## Supported Versions

The following versions of Ri are currently supported with security updates:

| Version | Supported          | Status                |
| ------- | ------------------ | --------------------- |
| 0.1.x   | :white_check_mark: | Current stable series |
| < 0.1.0 | :x:                | No longer supported   |

We provide security updates for the latest minor version in each major version series. Users are encouraged to upgrade to the latest version to receive security patches.

## Reporting a Vulnerability

If you discover a security vulnerability in Ri, please report it to us as soon as possible. We appreciate your efforts to responsibly disclose your findings.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues or Gitee issues.**

Instead, please report security vulnerabilities via:

📧 **Email**: dunimd@outlook.com

For general questions and non-security issues, please use:
- **Gitee Issues** (Primary): https://gitee.com/dunimd/ri/issues
- **GitHub Issues** (Mirror): https://github.com/mf2023/Ri/issues

Please include the following information in your report:

- **Description**: A clear and concise description of the vulnerability
- **Impact**: What kind of vulnerability is it and what impact could it have
- **Affected Versions**: Which versions of Ri are affected
- **Steps to Reproduce**: Detailed steps to reproduce the vulnerability
- **Proof of Concept**: If possible, include a proof-of-concept or exploit code
- **Suggested Fix**: If you have suggestions for how to fix the vulnerability
- **Your Contact Information**: How we can reach you for clarifications (optional)

### What to Expect

When you submit a security report, you can expect the following:

1. **Acknowledgment**: We will acknowledge receipt of your report within 48 hours
2. **Initial Assessment**: We will provide an initial assessment within 5 business days
3. **Investigation**: We will investigate the vulnerability and determine its impact
4. **Fix Development**: If confirmed, we will work on a fix and may reach out for additional information
5. **Disclosure**: We will coordinate with you on the disclosure timeline

### Response Time

Our target response times are:

| Severity | Initial Response | Fix Timeline |
|----------|-----------------|--------------|
| Critical | 24 hours | 7 days |
| High | 48 hours | 14 days |
| Medium | 5 business days | 30 days |
| Low | 10 business days | 60 days |

## Security Considerations

### Cryptographic Modules

Ri includes several cryptographic implementations:

#### Post-Quantum Cryptography
- **Kyber**: Key Encapsulation Mechanism (KEM) based on Module-LWE
- **Dilithium**: Digital signature algorithm
- **Falcon**: Compact digital signature algorithm

These implementations use the [oqs](https://github.com/open-quantum-safe/liboqs-rust) crate, which provides Rust bindings to liboqs (Open Quantum Safe).

#### Chinese National Cryptography (国密算法)
- **SM2**: Elliptic curve public key cryptography
- **SM3**: Cryptographic hash function
- **SM4**: Block cipher algorithm

These implementations use the [sm-crypto](https://crates.io/crates/sm-crypto) crate.

#### Security Notes
- All cryptographic operations should be performed using the provided APIs
- Do not implement custom cryptographic algorithms
- Keep cryptographic libraries updated to the latest versions
- Use appropriate key sizes and security parameters

### Network Security

#### WebSocket Connections
- Always use TLS (WSS) for production WebSocket connections
- Validate server certificates
- Implement proper authentication and authorization

#### gRPC Connections
- Use TLS for all gRPC connections in production
- Implement mutual TLS (mTLS) for service-to-service communication when appropriate

#### HTTP Gateway
- Enable TLS/HTTPS in production environments
- Use proper CORS configuration
- Implement rate limiting to prevent abuse

### Authentication and Authorization

- Use strong authentication mechanisms (JWT, OAuth 2.0)
- Implement proper session management
- Use role-based access control (RBAC)
- Regularly rotate secrets and API keys
- Store credentials securely (use the `secrecy` crate for sensitive data)

### Data Protection

- Encrypt sensitive data at rest
- Use secure random number generation for tokens and IDs
- Implement proper input validation
- Sanitize data to prevent injection attacks

## Security Best Practices

When using Ri in your applications:

### 1. Keep Dependencies Updated

Regularly update Ri and its dependencies to receive security patches:

```bash
cargo update
cargo audit  # Use cargo-audit to check for known vulnerabilities
```

### 2. Use Latest Stable Version

Always use the latest stable version of Ri to ensure you have the latest security fixes.

### 3. Enable Security Features

Build Ri with security features enabled:

```bash
cargo build --release --features "protocol,auth"
```

### 4. Configure Security Settings

Review and configure security-related settings:

- TLS configuration
- Authentication settings
- Rate limiting parameters
- CORS policies

### 5. Monitor and Log

Enable security logging and monitoring:

- Log authentication attempts
- Monitor for unusual activity
- Set up alerts for security events

### 6. Secure Deployment

Follow secure deployment practices:

- Use container security best practices
- Implement network segmentation
- Regular security audits
- Penetration testing

## Known Security Limitations

### Current Limitations

1. **Kafka on Windows**: The Kafka backend on Windows requires manual build configuration. Ensure proper security settings when building librdkafka.

2. **etcd Client**: Requires protoc for compilation. Ensure protoc is from a trusted source.

3. **Post-Quantum Cryptography**: Requires liboqs to be installed on the system. Ensure liboqs is properly secured.

### Security Considerations for Production

- Review the [deployment guide](doc/en/deployment.md) for production security recommendations
- Implement proper network security (firewalls, VPCs)
- Use secrets management systems for credentials
- Enable audit logging

## Security Updates

Security updates will be announced through:

- GitHub Security Advisories
- GitHub Releases (with security fix notes)
- CHANGELOG.md (with security-related changes marked)

## Vulnerability Disclosure Policy

### Our Commitment

- We will acknowledge receipt of vulnerability reports within 48 hours
- We will provide regular updates on our progress
- We will credit researchers who responsibly disclose vulnerabilities (unless they prefer to remain anonymous)
- We will not take legal action against researchers who follow this policy

### Disclosure Timeline

1. **Day 0**: Vulnerability reported
2. **Day 1-2**: Acknowledgment and initial assessment
3. **Day 3-14**: Investigation and fix development
4. **Day 15-30**: Testing and validation
5. **Day 30+**: Coordinated disclosure

We aim to disclose vulnerabilities within 90 days of the initial report, or sooner if a fix is available.

### Public Disclosure

We will publicly disclose vulnerabilities after:

- A fix has been developed and tested
- Affected users have had reasonable time to update
- The vulnerability has been assigned a CVE identifier (if applicable)

## Security-Related Configuration

### Environment Variables

The following environment variables affect security:

| Variable | Description | Security Impact |
|----------|-------------|-----------------|
| `RUST_LOG` | Logging level | May expose sensitive data if set to `trace` |
| `Ri_ENV` | Environment (dev/staging/prod) | Affects security defaults |

### Configuration Options

Review security-related configuration options in:

- `RiAuthConfig` - Authentication settings
- `RiGatewayConfig` - Gateway security settings
- `RiWSClientConfig` - WebSocket security settings

## Third-Party Security Audits

We welcome third-party security audits. If you are conducting a security audit of Ri:

1. Please follow responsible disclosure practices
2. Contact us in advance if you plan to publish findings
3. We appreciate receiving a copy of the audit report

## Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Rust Security Guidelines](https://doc.rust-lang.org/nomicon/)
- [Cargo Audit](https://github.com/RustSec/cargo-audit)
- [Open Quantum Safe](https://openquantumsafe.org/)

## Contact

For security-related inquiries:

- **Email**: dunimd@outlook.com
- **GPG Key**: [Available upon request]

For general questions and non-security issues, please use:

- **Gitee Issues** (Primary): https://gitee.com/dunimd/ri/issues
- **GitHub Issues** (Mirror): https://github.com/mf2023/Ri/issues
- **GitHub Discussions**: https://github.com/mf2023/Ri/discussions

## Acknowledgments

We thank the following security researchers who have responsibly disclosed vulnerabilities:

*This list will be updated as vulnerabilities are reported and fixed.*

---

**Last Updated**: 2025-01-31

**Version**: 1.0
