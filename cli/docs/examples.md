# Ri CLI Usage Examples

This document provides practical examples for using Ri CLI commands.

## Table of Contents

- [Creating Projects](#creating-projects)
- [Building Projects](#building-projects)
- [Running Projects](#running-projects)
- [Configuration Management](#configuration-management)
- [Code Generation](#code-generation)
- [Connection Testing](#connection-testing)
- [Diagnostics](#diagnostics)
- [Complete Workflows](#complete-workflows)

---

## Creating Projects

### Create a Minimal Project

The minimal template is perfect for simple applications or learning Ri basics.

```bash
# Create a minimal project
ric new my-cli-tool

# Navigate to the project
cd my-cli-tool

# View the structure
ls -la
# Output:
# Cargo.toml
# src/
#   main.rs
# .gitignore
# README.md
```

### Create a Web Application

The web template creates a full-featured web application.

```bash
# Create a web application
ric new my-web-app --template web

# Navigate to the project
cd my-web-app

# View the structure
ls -la
# Output:
# Cargo.toml
# src/
#   main.rs
# config/
#   development.yaml
#   production.yaml
# .gitignore
# README.md
```

### Create an API Service

The API template creates a RESTful API service.

```bash
# Create an API service
ric new my-api --template api

# Navigate to the project
cd my-api

# View the configuration
cat config/development.yaml
```

### Create a Worker Service

The worker template creates a background job processor.

```bash
# Create a worker service
ric new my-worker --template worker

# Navigate to the project
cd my-worker

# View the main file
cat src/main.rs
```

### Create a Microservice

The microservice template creates a distributed service.

```bash
# Create a microservice
ric new my-service --template microservice

# Navigate to the project
cd my-service

# View the configuration
cat config/development.yaml
```

### Create Project at Custom Path

```bash
# Create project at specific location
ric new my-project --path /home/user/projects

# Create project with relative path
ric new my-project --path ./workspace
```

---

## Building Projects

### Basic Build

```bash
# Build in debug mode (fast compilation, includes debug symbols)
ric build

# Output:
# Building my-project...
#    Compiling ri v0.1.9
#    Compiling my-project v0.1.0
#     Finished dev [unoptimized + debuginfo] target(s) in 12.34s
```

### Release Build

```bash
# Build in release mode (optimized, slower compilation)
ric build --release

# Output:
# Building my-project in release mode...
#    Compiling ri v0.1.9
#    Compiling my-project v0.1.0
#     Finished release [optimized] target(s) in 45.67s
```

### Build for Specific Target

```bash
# Build Python bindings
ric build --target python

# Build Java bindings
ric build --target java

# Build C bindings
ric build --target c

# Build all targets
ric build --target all
```

### Build with Specific Features

Edit `Cargo.toml` to enable features:

```toml
[features]
default = ["cache", "gateway"]
cache = ["ri/cache"]
gateway = ["ri/gateway"]
auth = ["ri/auth"]
```

Then build:

```bash
# Build with default features
ric build

# Build with specific features (via cargo)
cargo build --features auth
```

---

## Running Projects

### Run in Development Mode

```bash
# Run with debug configuration
ric run

# Output:
# Starting my-project v0.1.0
# Author: Your Name
# Initializing modules...
# Setting up gateway...
# Server listening on 0.0.0.0:8080
# Press Ctrl+C to stop
```

### Run in Release Mode

```bash
# Run with release configuration
ric run --release

# Output:
# Starting my-project v0.1.0
# Server listening on 0.0.0.0:8080
```

### Run with Custom Configuration

```bash
# Run with specific configuration file
ric run --config config/staging.yaml

# Run with production configuration
ric run --release --config config/production.yaml
```

### Graceful Shutdown

```bash
# Run the project
ric run

# Press Ctrl+C to stop
^C
# Output:
# Shutting down server...
# Goodbye!
```

---

## Configuration Management

### Initialize Configuration

```bash
# Create a new configuration file
ric config init

# Output:
# Creating ric.yaml...
# Configuration file created successfully
```

### View Configuration

```bash
# Display current configuration
ric config show

# Output:
# project:
#   name: my-project
#   version: 0.1.0
#   template: web
# 
# build:
#   release: false
#   target: all
# 
# runtime:
#   log_level: info
#   workers: 4
```

### Validate Configuration

```bash
# Validate configuration file
ric config validate

# Success output:
# Validating configuration...
# ✅ Configuration is valid

# Error output:
# Validating configuration...
# ❌ Configuration has errors:
# 
#   • cache.default_ttl_secs: must be at least 1 second
#     Suggestion: Set default_ttl_secs to at least 1
```

### Set Configuration Values

```bash
# Set project name
ric config set project.name my-awesome-project

# Set worker count
ric config set runtime.workers 8

# Enable release mode
ric config set build.release true

# Set log level
ric config set runtime.log_level debug

# Set cache TTL
ric config set cache.default_ttl_secs 7200

# Set gateway port
ric config set gateway.listen_port 3000
```

### Get Configuration Values

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

### Check Environment Variables

```bash
# Check environment variables
ric config check

# Output:
# Environment Variables:
#   ✅ RUST_LOG: info
#   ✅ CARGO_HOME: /home/user/.cargo
#   ⚠️  RI_CONFIG_PATH: not set (optional)
#   ⚠️  RI_LOG_LEVEL: not set (optional)
```

---

## Code Generation

### Generate a Module

```bash
# Generate a cache module
ric generate module cache my-cache

# Output:
# Generating module 'my-cache' of type 'cache'...
# Creating src/modules/my-cache/mod.rs
# Creating src/modules/my-cache/config.rs
# Creating src/modules/my-cache/handler.rs
# Creating src/modules/my-cache/service.rs
# Creating src/modules/my-cache/tests/mod.rs
# Module generated successfully
```

**Generated Structure**:
```
src/modules/my-cache/
├── mod.rs           # Module entry point
├── config.rs        # Configuration structures
├── handler.rs       # Request handlers
├── service.rs       # Business logic
└── tests/
    └── mod.rs       # Test module
```

### Generate Different Module Types

```bash
# Generate a queue module
ric generate module queue message-queue

# Generate an auth module
ric generate module auth user-auth

# Generate a gateway module
ric generate module gateway api-gateway

# Generate a device module
ric generate module device iot-manager

# Generate an observability module
ric generate module observability metrics-collector
```

### Generate Middleware

```bash
# Generate authentication middleware
ric generate middleware auth

# Output:
# Generating middleware 'auth'...
# Creating src/middleware/auth.rs
# Middleware generated successfully

# Generate logging middleware
ric generate middleware request-logger

# Generate rate limiting middleware
ric generate middleware rate-limiter

# Generate CORS middleware
ric generate middleware cors-handler
```

### Generate Configuration Struct

```bash
# Generate from YAML
ric generate config config.yaml

# Output:
# #[derive(Debug, Clone, Serialize, Deserialize)]
# pub struct Config {
#     pub server: ServerConfig,
#     pub cache: CacheConfig,
# }
# 
# #[derive(Debug, Clone, Serialize, Deserialize)]
# pub struct ServerConfig {
#     pub bind_address: String,
#     pub workers: usize,
# }

# Generate from JSON
ric generate config config.json

# Save to file
ric generate config app.yaml > src/config/app_config.rs
```

---

## Connection Testing

### Test Redis Connection

```bash
# Test local Redis
ric test redis redis://localhost:6379

# Output:
# Testing Redis connection...
# URL: redis://localhost:6379
# 
# ✅ Connection successful
#   Version: Redis 7.2.3
#   Response time: 2.34ms
#   Mode: standalone

# Test with password
ric test redis redis://:mypassword@localhost:6379

# Test specific database
ric test redis redis://localhost:6379/1

# Test remote Redis
ric test redis redis://user:pass@redis.example.com:6379
```

### Test PostgreSQL Connection

```bash
# Test local PostgreSQL
ric test postgres postgresql://postgres:password@localhost:5432/mydb

# Output:
# Testing PostgreSQL connection...
# URL: postgresql://postgres:***@localhost:5432/mydb
# 
# ✅ Connection successful
#   Version: PostgreSQL 15.4
#   Response time: 5.67ms
#   Database: mydb
#   SSL: disabled

# Test with SSL
ric test postgres postgresql://user:pass@host:5432/db?sslmode=require

# Test remote PostgreSQL
ric test postgres postgresql://admin:secret@db.example.com:5432/production
```

### Test MySQL Connection

```bash
# Test local MySQL
ric test mysql mysql://root:password@localhost:3306/mydb

# Output:
# Testing MySQL connection...
# URL: mysql://root:***@localhost:3306/mydb
# 
# ✅ Connection successful
#   Version: MySQL 8.0.35
#   Response time: 4.12ms
#   Database: mydb

# Test with custom port
ric test mysql mysql://root:password@localhost:3307/mydb

# Test remote MySQL
ric test mysql mysql://admin:secret@db.example.com:3306/production
```

### Test Kafka Connection

```bash
# Test single Kafka broker
ric test kafka localhost:9092

# Output:
# Testing Kafka connection...
# Brokers: localhost:9092
# 
# ✅ Connection successful
#   Cluster ID: abc123
#   Broker count: 1
#   Topics: 5
#   Response time: 12.34ms

# Test Kafka cluster
ric test kafka broker1:9092,broker2:9092,broker3:9092

# Test remote Kafka
ric test kafka kafka.example.com:9092
```

---

## Diagnostics

### Run Basic Diagnostics

```bash
# Run basic diagnostics
ric doctor

# Output:
# Running diagnostics...
# 
# Rust Toolchain:
#   ✅ rustc 1.75.0 (stable)
#   ✅ cargo 1.75.0
#   ✅ rustup 1.26.0
# 
# Development Tools:
#   ✅ git 2.42.0
#   ✅ build-essential
# 
# Environment:
#   ✅ RUST_LOG: info
#   ✅ CARGO_HOME: /home/user/.cargo
#   ⚠️  RI_CONFIG_PATH: not set (optional)
# 
# Port Availability:
#   ✅ Port 8080: available
#   ✅ Port 8081: available
#   ⚠️  Port 3000: in use
# 
# File System:
#   ✅ Write permissions: OK
#   ✅ Disk space: 50GB available
# 
# Summary: 12 passed, 2 warnings, 0 errors
```

### Run Verbose Diagnostics

```bash
# Run with detailed output
ric doctor --verbose

# Output:
# Running diagnostics (verbose)...
# 
# Rust Toolchain:
#   ✅ rustc 1.75.0 (82e1608df5c4c1e572e10d3d0343c0c1a9b7f4e4)
#       Path: /home/user/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc
#   ✅ cargo 1.75.0
#       Path: /home/user/.cargo/bin/cargo
#   ✅ rustup 1.26.0
#       Path: /home/user/.cargo/bin/rustup
#       Active toolchain: stable-x86_64-unknown-linux-gnu
#       Installed targets: x86_64-unknown-linux-gnu
# 
# ... (more detailed output)
```

### Auto-Fix Issues

```bash
# Run with auto-fix
ric doctor --fix

# Output:
# Running diagnostics with auto-fix...
# 
# Checking environment variables...
#   ⚠️  RI_CONFIG_PATH: not set
#       Creating default configuration directory...
#       ✅ Fixed: Created ~/.config/ric/
# 
# Checking file system...
#   ⚠️  Missing directory: target/
#       Creating build directory...
#       ✅ Fixed: Created target/
# 
# Summary: 2 issues fixed
```

### Combined Options

```bash
# Verbose with auto-fix
ric doctor --verbose --fix

# Short form
ric doctor -v -f
```

---

## Complete Workflows

### Workflow 1: Create and Run a Web Service

```bash
# Step 1: Create the project
ric new my-web-service --template web

# Step 2: Navigate to the project
cd my-web-service

# Step 3: Review the configuration
cat config/development.yaml

# Step 4: Build the project
ric build

# Step 5: Run the project
ric run

# Step 6: Test the service
curl http://localhost:8080/health

# Step 7: Stop the service (Ctrl+C)
^C

# Step 8: Build for production
ric build --release

# Step 9: Run in production mode
ric run --release --config config/production.yaml
```

### Workflow 2: Create an API Service with Modules

```bash
# Step 1: Create the API project
ric new my-api-service --template api

# Step 2: Navigate to the project
cd my-api-service

# Step 3: Generate a cache module
ric generate module cache api-cache

# Step 4: Generate an auth module
ric generate module auth api-auth

# Step 5: Generate authentication middleware
ric generate middleware auth-check

# Step 6: Configure the modules
ric config set cache.enabled true
ric config set cache.backend_type Memory
ric config set auth.enabled true

# Step 7: Validate configuration
ric config validate

# Step 8: Build the project
ric build

# Step 9: Run the project
ric run
```

### Workflow 3: Create a Worker Service

```bash
# Step 1: Create the worker project
ric new my-worker-service --template worker

# Step 2: Navigate to the project
cd my-worker-service

# Step 3: Configure the queue
ric config set queue.backend_type Redis
ric config set queue.connection_string redis://localhost:6379

# Step 4: Test Redis connection
ric test redis redis://localhost:6379

# Step 5: Configure workers
ric config set runtime.workers 8

# Step 6: Build the project
ric build

# Step 7: Run the worker
ric run
```

### Workflow 4: Create a Microservice with Service Mesh

```bash
# Step 1: Create the microservice
ric new my-microservice --template microservice

# Step 2: Navigate to the project
cd my-microservice

# Step 3: Configure service mesh
ric config set gateway.listen_port 8080
ric config set observability.tracing_enabled true
ric config set observability.metrics_enabled true

# Step 4: Generate observability module
ric generate module observability metrics

# Step 5: Validate configuration
ric config validate

# Step 6: Build the project
ric build --release

# Step 7: Run the service
ric run --release
```

### Workflow 5: Development to Production

```bash
# Step 1: Create the project
ric new my-app --template web

# Step 2: Navigate to the project
cd my-app

# Step 3: Develop with debug settings
ric config set runtime.log_level debug
ric config set cache.backend_type Memory

# Step 4: Run in development
ric run

# Step 5: Test connections
ric test redis redis://localhost:6379

# Step 6: Prepare for production
# Edit config/production.yaml with production settings
# Set environment variables for secrets

# Step 7: Validate production config
ric config validate config/production.yaml

# Step 8: Build for production
ric build --release

# Step 9: Run diagnostics
ric doctor

# Step 10: Deploy
ric run --release --config config/production.yaml
```

### Workflow 6: Troubleshooting a Project

```bash
# Step 1: Run diagnostics
ric doctor --verbose

# Step 2: Check configuration
ric config validate

# Step 3: Check environment
ric config check

# Step 4: Test connections
ric test redis redis://localhost:6379
ric test postgres postgresql://user:pass@localhost:5432/db

# Step 5: Check for compilation errors
ric check

# Step 6: Clean and rebuild
ric clean
ric build

# Step 7: Run with debug logging
ric config set runtime.log_level debug
ric run
```

---

## Tips and Tricks

### Quick Project Setup

```bash
# Create and run in one go
ric new my-project && cd my-project && ric run
```

### Configuration Shortcuts

```bash
# Use short options
ric new my-project -t web -p ./projects
ric build -r -t python
ric run -r -c config/prod.yaml
ric doctor -v -f
```

### Environment Setup

```bash
# Set up environment for development
export RI_LOG_LEVEL=debug
export RUST_LOG=ri=debug,info

# Set up environment for production
export RI_LOG_LEVEL=info
export RI_JWT_SECRET=$(openssl rand -base64 32)
```

### Useful Aliases

Add to your shell configuration:

```bash
# ~/.bashrc or ~/.zshrc
alias rn='ric new'
alias rb='ric build'
alias rr='ric run'
alias rc='ric config'
alias rd='ric doctor'
alias rt='ric test'
```

Then use:

```bash
rn my-project -t web
rb -r
rr
rc show
rd -v
rt redis redis://localhost:6379
```
