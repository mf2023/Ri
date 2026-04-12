# Ri CLI Command Reference

This document provides detailed documentation for all Ri CLI commands.

## Table of Contents

- [ric new](#ric-new)
- [ric build](#ric-build)
- [ric run](#ric-run)
- [ric config](#ric-config)
- [ric generate](#ric-generate)
- [ric doctor](#ric-doctor)
- [ric test](#ric-test)
- [ric check](#ric-check)
- [ric clean](#ric-clean)
- [ric info](#ric-info)
- [ric version](#ric-version)

---

## ric new

Create a new Ri project with the specified template.

### Usage

```bash
ric new <name> [options]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `<name>` | Yes | Project name. Used as directory name and package name. Must follow Rust naming conventions. |

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--template` | `-t` | `minimal` | Project template to use |
| `--path` | `-p` | Current directory | Custom path for project creation |

### Available Templates

| Template | Description |
|----------|-------------|
| `minimal` | Minimal Ri application with basic structure |
| `web` | Full-featured web application with HTTP server |
| `api` | RESTful API service with validation |
| `worker` | Background job processing service |
| `microservice` | Distributed microservice with service mesh |

### Examples

```bash
# Create a minimal project
ric new my-project

# Create a web application
ric new my-web-app --template web

# Create an API service at custom path
ric new my-api --template api --path /projects

# Create a worker service
ric new my-worker -t worker

# Create a microservice
ric new my-service --template microservice
```

### Project Structure

After creation, the project will have the following structure:

```
my-project/
├── Cargo.toml          # Package configuration
├── src/
│   └── main.rs         # Application entry point
├── config/             # Configuration files (web, api, worker, microservice)
│   ├── development.yaml
│   └── production.yaml
├── .gitignore          # Git ignore rules
└── README.md           # Project documentation
```

### Common Errors

#### Project Already Exists

```
Error: Project directory 'my-project' already exists
```

**Solution**: Remove the existing directory or use a different name.

```bash
rm -rf my-project
ric new my-project
```

#### Invalid Project Name

```
Error: Project name must contain only alphanumeric characters, dashes, and underscores
```

**Solution**: Use a valid Rust package name.

```bash
# Invalid
ric new "my project"
ric new "123project"

# Valid
ric new my-project
ric new my_project
ric new project123
```

---

## ric build

Build the Ri project for the specified target.

### Usage

```bash
ric build [options]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--release` | `-r` | `false` | Build in release mode with optimizations |
| `--target` | `-t` | `all` | Build target platform |

### Build Targets

| Target | Description |
|--------|-------------|
| `all` | Build all targets (default) |
| `python` | Build Python bindings |
| `java` | Build Java bindings |
| `c` | Build C/C++ bindings |

### Examples

```bash
# Build in debug mode
ric build

# Build in release mode
ric build --release

# Build Python bindings
ric build --target python

# Build Java bindings in release mode
ric build -r -t java

# Build all targets
ric build --target all
```

### Build Output

```
Building my-project...
   Compiling ri v0.1.9
   Compiling my-project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 12.34s
```

### Common Errors

#### Compilation Error

```
error: could not compile `my-project`
```

**Solution**: Check the error messages and fix the code issues.

```bash
# Get detailed error information
ric build 2>&1 | less

# Or use check command for faster feedback
ric check
```

#### Missing Dependencies

```
error: no matching package named `ri` found
```

**Solution**: Ensure Ri is available in your Cargo.toml.

```toml
[dependencies]
ri = { path = ".." }  # For local development
# or
ri = { version = "0.1.9" }  # For crates.io
```

---

## ric run

Run the Ri project in development or release mode.

### Usage

```bash
ric run [options]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--release` | `-r` | `false` | Run in release mode |
| `--config` | `-c` | `ric.yaml` | Path to configuration file |

### Examples

```bash
# Run in debug mode
ric run

# Run in release mode
ric run --release

# Run with custom configuration
ric run --config config/production.yaml

# Run release with custom config
ric run -r -c config/staging.yaml
```

### Runtime Output

```
Starting my-project v0.1.0
Author: Your Name
Initializing modules...
Setting up gateway...
Server listening on 0.0.0.0:8080
Press Ctrl+C to stop
```

### Graceful Shutdown

Press `Ctrl+C` to gracefully shutdown the application:

```
^C
Shutting down server...
Goodbye!
```

---

## ric config

Manage project configuration.

### Subcommands

| Command | Description |
|---------|-------------|
| `init` | Initialize a new configuration file |
| `show` | Display current configuration |
| `validate` | Validate configuration file |
| `check` | Check environment variables |
| `set` | Set a configuration value |
| `get` | Get a configuration value |

### ric config init

Initialize a new `ric.yaml` configuration file.

```bash
ric config init
```

Creates a default configuration file in the current directory.

### ric config show

Display the current configuration.

```bash
ric config show
```

Output:
```yaml
project:
  name: my-project
  version: 0.1.0
  template: web

build:
  release: false
  target: all

runtime:
  log_level: info
  workers: 4
```

### ric config validate

Validate the configuration file for errors.

```bash
# Validate default configuration
ric config validate

# Validate specific file
ric config validate path/to/config.yaml
```

Output:
```
Validating configuration...
✅ Configuration is valid
```

Error output:
```
Validating configuration...
❌ Configuration has errors:

  • cache.default_ttl_secs: must be at least 1 second
    Suggestion: Set default_ttl_secs to at least 1

  • gateway.listen_port: must be between 1 and 65535
    Suggestion: Use a valid port number between 1 and 65535
```

### ric config check

Check environment variables that affect Ri projects.

```bash
ric config check
```

Output:
```
Environment Variables:
  ✅ RUST_LOG: info
  ✅ CARGO_HOME: /home/user/.cargo
  ⚠️  RI_CONFIG_PATH: not set (optional)
  ⚠️  RI_LOG_LEVEL: not set (optional)
```

### ric config set

Set a configuration value using dot notation.

```bash
ric config set <key> <value>
```

Examples:
```bash
# Set project name
ric config set project.name my-new-project

# Set worker count
ric config set runtime.workers 8

# Enable release mode
ric config set build.release true

# Set log level
ric config set runtime.log_level debug

# Set cache TTL
ric config set cache.default_ttl_secs 7200
```

### ric config get

Get a configuration value.

```bash
ric config get <key>
```

Examples:
```bash
# Get project name
ric config get project.name
# Output: project.name: my-project

# Get worker count
ric config get runtime.workers
# Output: runtime.workers: 4

# Get nested value
ric config get gateway.listen_port
# Output: gateway.listen_port: 8080
```

---

## ric generate

Generate code artifacts for Ri projects.

### Subcommands

| Command | Description |
|---------|-------------|
| `module` | Generate a new Ri module |
| `middleware` | Generate middleware template |
| `config` | Generate Rust struct from config file |

### ric generate module

Generate a complete Ri module with scaffolding.

```bash
ric generate module <type> <name>
```

#### Module Types

| Type | Description | Dependencies |
|------|-------------|--------------|
| `cache` | Caching module | redis, memcached |
| `queue` | Message queue module | lapin, rdkafka |
| `gateway` | API Gateway module | hyper, tower |
| `auth` | Authentication module | jsonwebtoken, oauth2 |
| `device` | IoT device module | mqtt, coap |
| `observability` | Monitoring module | tracing, metrics |

#### Examples

```bash
# Generate a cache module
ric generate module cache my-cache

# Generate a queue module
ric generate module queue message-queue

# Generate an auth module
ric generate module auth user-auth

# Generate a gateway module
ric generate module gateway api-gateway
```

#### Generated Structure

```
src/modules/my-cache/
├── mod.rs           # Module entry point
├── config.rs        # Configuration structures
├── handler.rs       # Request handlers
├── service.rs       # Business logic
└── tests/
    └── mod.rs       # Test module
```

### ric generate middleware

Generate a middleware template.

```bash
ric generate middleware <name>
```

Examples:
```bash
# Generate authentication middleware
ric generate middleware auth

# Generate logging middleware
ric generate middleware request-logger

# Generate rate limiting middleware
ric generate middleware rate-limiter
```

Generated file: `src/middleware/auth.rs`

### ric generate config

Generate Rust struct from YAML or JSON configuration file.

```bash
ric generate config <file>
```

Examples:
```bash
# Generate from YAML
ric generate config config.yaml

# Generate from JSON
ric generate config config.json

# Save to file
ric generate config app.yaml > src/config/app_config.rs
```

---

## ric doctor

Run comprehensive diagnostic checks on the development environment.

### Usage

```bash
ric doctor [options]
```

### Options

| Option | Short | Default | Description |
|--------|-------|---------|-------------|
| `--verbose` | `-v` | `false` | Show detailed diagnostic information |
| `--fix` | `-f` | `false` | Attempt to auto-fix detected issues |

### Diagnostic Categories

1. **Rust Toolchain**: rustc, cargo, rustup versions
2. **Development Tools**: git, build tools
3. **Environment Variables**: RUST_LOG, CARGO_HOME, etc.
4. **Port Availability**: Common development ports
5. **Dependencies**: Version conflicts
6. **File System**: Permissions, disk space

### Examples

```bash
# Basic diagnostics
ric doctor

# Detailed diagnostics
ric doctor --verbose

# Auto-fix issues
ric doctor --fix

# Verbose with auto-fix
ric doctor -v -f
```

### Output Format

```
Running diagnostics...

Rust Toolchain:
  ✅ rustc 1.75.0 (stable)
  ✅ cargo 1.75.0
  ✅ rustup 1.26.0

Development Tools:
  ✅ git 2.42.0
  ✅ build-essential

Environment:
  ✅ RUST_LOG: info
  ✅ CARGO_HOME: /home/user/.cargo
  ⚠️  RI_CONFIG_PATH: not set (optional)

Port Availability:
  ✅ Port 8080: available
  ✅ Port 8081: available
  ⚠️  Port 3000: in use

File System:
  ✅ Write permissions: OK
  ✅ Disk space: 50GB available

Summary: 12 passed, 2 warnings, 0 errors
```

---

## ric test

Test connections to external services.

### Subcommands

| Command | Description |
|---------|-------------|
| `redis` | Test Redis connection |
| `postgres` | Test PostgreSQL connection |
| `mysql` | Test MySQL connection |
| `kafka` | Test Kafka connection |

### ric test redis

Test Redis server connectivity.

```bash
ric test redis <url>
```

URL Format: `redis://[password@]host:port[/database]`

Examples:
```bash
# Test local Redis
ric test redis redis://localhost:6379

# Test with password
ric test redis redis://:mypassword@localhost:6379

# Test specific database
ric test redis redis://localhost:6379/1
```

Output:
```
Testing Redis connection...
URL: redis://localhost:6379

✅ Connection successful
  Version: Redis 7.2.3
  Response time: 2.34ms
  Mode: standalone
```

### ric test postgres

Test PostgreSQL database connectivity.

```bash
ric test postgres <url>
```

URL Format: `postgresql://user:password@host:port/database`

Examples:
```bash
# Test local PostgreSQL
ric test postgres postgresql://postgres:password@localhost:5432/mydb

# Test with SSL
ric test postgres postgresql://user:pass@host:5432/db?sslmode=require
```

Output:
```
Testing PostgreSQL connection...
URL: postgresql://postgres:***@localhost:5432/mydb

✅ Connection successful
  Version: PostgreSQL 15.4
  Response time: 5.67ms
  Database: mydb
  SSL: disabled
```

### ric test mysql

Test MySQL database connectivity.

```bash
ric test mysql <url>
```

URL Format: `mysql://user:password@host:port/database`

Examples:
```bash
# Test local MySQL
ric test mysql mysql://root:password@localhost:3306/mydb

# Test with custom port
ric test mysql mysql://root:password@localhost:3307/mydb
```

### ric test kafka

Test Kafka broker connectivity.

```bash
ric test kafka <url>
```

URL Format: `host:port` or `host1:port1,host2:port2`

Examples:
```bash
# Test single broker
ric test kafka localhost:9092

# Test cluster
ric test kafka broker1:9092,broker2:9092,broker3:9092
```

---

## ric check

Check the project for compilation errors without building.

### Usage

```bash
ric check
```

This command runs `cargo check` internally, which is faster than a full build.

### Examples

```bash
# Check for errors
ric check
```

Output:
```
Checking my-project...
    Checking ri v0.1.9
    Checking my-project v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 2.34s
✅ No errors found
```

---

## ric clean

Clean build artifacts from the project.

### Usage

```bash
ric clean
```

This removes the `target/` directory and all build artifacts.

### Examples

```bash
# Clean build artifacts
ric clean
```

Output:
```
Cleaning build artifacts...
Removed: target/debug
Removed: target/release
✅ Clean complete
```

---

## ric info

Display comprehensive project and environment information.

### Usage

```bash
ric info
```

### Output

```
Project Information:
  Name: my-project
  Version: 0.1.0
  Template: web
  Path: /home/user/projects/my-project

Ri Framework:
  CLI Version: 0.1.0
  Framework Version: 0.1.9

Rust Toolchain:
  rustc: 1.75.0 (stable)
  cargo: 1.75.0
  rustup: 1.26.0

Features:
  ✅ Gateway
  ✅ Cache
  ✅ Auth
  ✅ Queue
  ✅ Observability
```

---

## ric version

Display version information for ric and Ri framework.

### Usage

```bash
ric version
```

### Output

```
ric 0.1.0
ri  0.1.9
```
