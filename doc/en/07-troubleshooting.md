<div align="center">

# Troubleshooting

**Version: 0.1.8**

**Last modified date: 2026-02-20**

This chapter introduces common issues and troubleshooting methods when using the DMSC framework, helping you quickly locate and resolve problems.

## 1. Compilation Errors

</div>

### 1.1 Dependency Not Found

**Error Message**:
```
error: failed to select a version for the requirement `dms = ...`
```

**Solutions**:
- Ensure DMSC dependency is correctly specified in `Cargo.toml`
- Check if the Git URL is correct: `dms = { git = "https://github.com/mf2023/DMSC" }`
- Try running `cargo update` to update dependencies

### 1.2 Version Conflicts

**Error Message**:
```
error: failed to resolve: could not find `some-crate` in `dependencies`
```

**Solutions**:
- Check for dependency version conflicts using `cargo tree` to view the dependency tree
- Try specifying compatible dependency versions
- Consider using `cargo vendor` to manage dependencies

### 1.3 Type Mismatch

**Error Message**:
```
error[E0308]: mismatched types
```

**Solutions**:
- Check if the function call arguments have the correct types
- Ensure the return type matches the function signature
- Use `into()` or `as` for type conversion
- Consult the API documentation to understand the correct type usage

### 1.4 Missing Feature Flags

**Error Message**:
```
error: the `full` feature is required
```

**Solutions**:
- Add the correct feature flags for Tokio: `tokio = { version = "1.0", features = ["full"] }`
- Check if other dependencies require specific feature flags

<div align="center">

## 2. Runtime Errors

</div>

### 2.1 Configuration File Not Found

**Error Message**:
```
Error: Could not find config file: config.yaml
```

**Solutions**:
- Ensure the configuration file exists at the specified path
- Check if the configuration file path is correct
- Consider using an absolute path to specify the configuration file

### 2.2 Invalid Configuration Format

**Error Message**:
```
Error: Invalid configuration format: YAML parsing error
```

**Solutions**:
- Check if the configuration file has correct YAML/TOML/JSON format
- Use online tools to validate the configuration file format
- Ensure proper indentation and syntax in the configuration file

### 2.3 Port Already in Use

**Error Message**:
```
Error: Address already in use: 0.0.0.0:9090
```

**Solutions**:
- Check if another process is using the same port
- Use `lsof -i :9090` (Linux/macOS) or `netstat -ano | findstr :9090` (Windows) to find the process using the port
- Modify the port number in the configuration file

### 2.4 Insufficient Permissions

**Error Message**:
```
Error: Permission denied (os error 13)
```

**Solutions**:
- Check if the application has sufficient permissions to access files or directories
- Ensure configuration files and log directories have correct permissions
- Try running the application as administrator/root (only in development environments)

### 2.5 Module Initialization Failure

**Error Message**:
```
Error: Module initialization failed: auth
```

**Solutions**:
- Check if the module configuration is correct
- Check detailed logs to understand the specific error cause
- Ensure dependent services are available

<div align="center">

## 3. Performance Issues

</div>

### 3.1 High CPU Usage

**Symptoms**:
- Application CPU usage remains consistently high
- Increased response times

**Solutions**:
- Use `tokio-console` or `perf` to analyze CPU usage
- Check for infinite loops
- Optimize CPU-intensive operations, consider using `tokio::spawn_blocking`
- Check if there's excessive log output

### 3.2 High Memory Usage

**Symptoms**:
- Application memory usage continues to grow
- Out-of-memory (OOM) errors occur

**Solutions**:
- Use `valgrind` or `heaptrack` for memory analysis
- Check for memory leaks
- Optimize handling of large objects, consider using references instead of cloning
- Adjust caching strategy, reduce cache size

### 3.3 Long Response Times

**Symptoms**:
- API response times exceed expectations
- Client request timeouts

**Solutions**:
- Use distributed tracing to find bottlenecks
- Check database query performance
- Optimize network requests, reduce external calls
- Consider adding caching to reduce duplicate calculations

### 3.4 Low Throughput

**Symptoms**:
- Application processes fewer requests than expected
- Request queue backlog

**Solutions**:
- Optimize concurrent processing, increase number of worker threads
- Check for shared resource contention
- Use async I/O to avoid blocking operations
- Consider using batch processing to reduce system calls

<div align="center">

## 4. Logging Issues

</div>

### 4.1 No Log Output

**Symptoms**:
- Application runs but produces no log output

**Solutions**:
- Check logging configuration, ensure `console_enabled` or `file_enabled` is true
- Check log level, ensure it's set to an appropriate level (e.g., INFO)
- Ensure log directories exist and are writable
- Check if the application is capturing log output

### 4.2 Incorrect Log Format

**Symptoms**:
- Log format doesn't match expectations
- Structured logs are missing fields

**Solutions**:
- Check logging configuration, ensure `format` is set correctly (e.g., json or text)
- Ensure log events include all required fields
- Check if custom log formatters are implemented correctly

### 4.3 Log Level Setting Ineffective

**Symptoms**:
- Log level settings don't take effect
- Still seeing DEBUG level logs

**Solutions**:
- Check log level setting in configuration file
- Ensure no environment variables are overriding the log level
- Check if there are hardcoded log levels in the code

<div align="center">

## 5. Configuration Issues

</div>

### 5.1 Configuration Not Taking Effect

**Symptoms**:
- Application behavior doesn't change after modifying configuration file

**Solutions**:
- Check if configuration hot reload is enabled
- Ensure configuration file path is correct
- Try restarting the application
- Check if configuration is being overridden by environment variables

### 5.2 Sensitive Information Leakage

**Symptoms**:
- Sensitive information appears in logs or error messages

**Solutions**:
- Ensure sensitive information uses environment variables or secret management services
- Check log configuration, ensure sensitive fields are filtered
- Check error handling, ensure sensitive information isn't leaked

### 5.3 Configuration Inheritance Issues

**Symptoms**:
- Child configuration files don't correctly inherit from parent configuration

**Solutions**:
- Check the configuration file inheritance mechanism
- Ensure parent configuration file path is correct
- Check configuration merging logic

<div align="center">

## 6. Module Issues

</div>

### 6.1 Module Not Found

**Error Message**:
```
Error: Module not found: custom_module
```

**Solutions**:
- Ensure the module is correctly registered
- Check if the module name is spelled correctly
- Check if the module implements the correct trait

### 6.2 Circular Module Dependencies

**Error Message**:
```
Error: Circular dependency detected between modules
```

**Solutions**:
- Check dependencies between modules
- Refactor modules to break circular dependencies
- Consider using event-driven or message passing instead of direct dependencies

### 6.3 Incorrect Module Initialization Order

**Error Message**:
```
Error: Module initialization failed due to missing dependency
```

**Solutions**:
- Set correct priorities for modules
- Ensure dependent modules initialize first
- Check the return value of the module's `priority()` method

<div align="center">

## 7. Network Issues

</div>

### 7.1 Connection Timeout

**Error Message**:
```
Error: Connection timed out
```

**Solutions**:
- Check if network connection is normal
- Ensure target service is available
- Check firewall settings, ensure ports are open
- Adjust connection timeout settings

### 7.2 DNS Resolution Failure

**Error Message**:
```
Error: Failed to resolve hostname
```

**Solutions**:
- Check if DNS configuration is correct
- Try using IP address instead of domain name
- Check network proxy settings

### 7.3 TLS/SSL Errors

**Error Message**:
```
Error: TLS handshake failed
```

**Solutions**:
- Check if certificates are valid
- Ensure correct TLS version is being used
- Check if certificate chain is complete
- Consider temporarily disabling TLS verification (only in development environments)

<div align="center">

## 8. Debugging Techniques

</div>

### 8.1 Using Logs for Debugging

- **Increase log level**: Set log level to DEBUG to get more detailed information
- **Add context logs**: Add logs at key code locations to track execution flow
- **Use structured logging**: Add additional context fields for easier log analysis

### 8.2 Using Debuggers

- **Use rust-gdb**: Debug Rust programs on Linux/macOS using gdb
- **Use rust-lldb**: Debug Rust programs on macOS using lldb
- **Use VS Code**: Debug using the Rust extension for VS Code

### 8.3 Using Distributed Tracing

- **Enable tracing**: Ensure `tracing_enabled` is true in observability configuration
- **View trace data**: Use Jaeger or Zipkin to view distributed trace data
- **Analyze latency**: Find performance bottlenecks through trace data

### 8.4 Using Metrics Monitoring

- **View Prometheus metrics**: Access `http://localhost:9090/metrics` to view metrics
- **Use Grafana**: Configure Grafana dashboards to visualize metric data
- **Set up alerts**: Set alerts for key metrics to detect issues promptly

### 8.5 Checking System Resources

- **Use top/htop**: Monitor CPU, memory, and process status
- **Use iostat**: Monitor disk I/O
- **Use netstat/ss**: Monitor network connections

<div align="center">

## 9. Common Questions

</div>

### Q: How to check the DMSC version?

A: Currently, DMSC is distributed through Git repositories. You can check the dependency version in the `Cargo.toml` file, or use `cargo tree | grep dms` to view the currently used DMSC version.

### Q: How to update DMSC to the latest version?

A: Run `cargo update` to update all dependencies, or manually modify the DMSC dependency version in `Cargo.toml`.

### Q: How to contribute code to DMSC?

A: You can contribute code through the following steps:
1. Fork the DMSC repository
2. Create a feature branch
3. Commit code changes
4. Create a Pull Request

### Q: Which databases does DMSC support?

A: DMSC itself doesn't directly support specific databases, but can integrate with any database through custom modules. It's recommended to use async database drivers, such as:
- PostgreSQL: `tokio-postgres`
- MySQL: `mysql_async`
- Redis: `redis` crate

### Q: How to deploy DMSC applications in production?

A: Containerized deployment is recommended, such as Docker + Kubernetes:
1. Write a Dockerfile
2. Build the Docker image
3. Deploy to a Kubernetes cluster
4. Configure auto-scaling and rolling updates

### Q: Which operating systems does DMSC support?

A: DMSC supports all major operating systems:
- Linux
- macOS
- Windows

### Q: How to handle errors in async tasks?

A: Use the `DMSCResult` type and `?` operator to propagate errors, or use `tokio::spawn` with error handling:

```rust
tokio::spawn(async move {
    if let Err(e) = some_async_function().await {
        // Handle error
    }
});
```

<div align="center">

## 10. Getting Help

</div>

If you encounter unresolved issues, you can get help through the following methods:

1. **Check documentation**: Carefully read DMSC's official documentation
2. **Examine examples**: Look at example code to understand correct usage
3. **Search Issues**: Search for related Issues in the GitHub/Gitee repository
4. **Submit an Issue**: If the problem is not resolved, submit a new Issue
5. **Join the community**: Join the DMSC community to communicate with other developers

<div align="center">

## Summary

</div>

Troubleshooting is an important part of the development process. By understanding common issues and solutions, you can locate and resolve problems more quickly, improving development efficiency.

When troubleshooting issues, it's recommended to:

1. **Start with logs**: First check application logs to get detailed error information
2. **Narrow down the scope**: Gradually narrow down the problem scope through elimination
3. **Use debugging tools**: Utilize logs, debuggers, and monitoring tools
4. **Check documentation**: Refer to official documentation and example code
5. **Seek community help**: If you can't resolve the issue, ask the community for help

We hope this chapter helps you solve problems encountered while using DMSC.

<div align="center">

## Next Steps

</div> 

- [Glossary](./08-glossary.md): Core terminology explanations