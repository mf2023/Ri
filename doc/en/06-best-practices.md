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

## 10. Development Process Design

</div>

### 10.1 Code Style

- **Follow the Rust Style Guide**: Adhere to the official Rust Style Guide
- **Use rustfmt**: Format code with `rustfmt`
- **Use clippy**: Check code quality with `clippy`
- **Conduct code reviews**: Perform code reviews to ensure code quality

### 10.2 Version Management

- **Use semantic versioning**: Follow semantic versioning conventions
- **Update CHANGELOG**: Update the CHANGELOG with each version release
- **Use Git tags**: Mark versions with Git tags

### 10.3 Documentation

- **Write documentation**: Create detailed documentation for public APIs
- **Update examples**: Keep example code current
- **Maintain README**: Regularly update README files
- **Use comments**: Add comments to complex code

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