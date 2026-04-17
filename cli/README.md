<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 8px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../assets/svg/ri.svg" width="36" height="36" alt="Ri">Ri CLI (ric)</span>
</h1>

English | [简体中文](README.zh.md)

<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>
<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>

<a href="https://github.com/mf2023/Ri" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-Ri-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://gitcode.com/dunimd/ri.git" target="_blank">
    <img alt="GitCode" src="https://img.shields.io/badge/GitCode-Ri-FF6B35?style=flat-square&logo=git"/>
</a>

<img alt="Version" src="https://img.shields.io/badge/version-0.1.0-green?style=flat-square"/>
<img alt="Rust" src="https://img.shields.io/badge/rust-1.65%2B-orange?style=flat-square"/>
<img alt="License" src="https://img.shields.io/badge/license-Apache--2.0-blue?style=flat-square"/>

**Ri CLI (ric)** — A powerful command-line interface tool for managing Ri framework projects. Provides comprehensive commands for project creation, building, running, and configuration management.

</div>

<h2 align="center">✨ Features</h2>

<div align="center">

| Feature | Description |
|:--------|:-------------|
| **Project Scaffolding** | Create new Ri projects with multiple templates (web, api, worker, microservice, minimal) |
| **Build Management** | Build projects for different targets (Python, Java, C, native) |
| **Configuration Management** | Initialize, validate, and manage project configuration |
| **Connection Testing** | Test connectivity to Redis, PostgreSQL, MySQL, and Kafka |
| **Environment Diagnostics** | Comprehensive environment checking with auto-fix capabilities |
| **Code Generation** | Generate modules, middleware, and configuration structures |
| **Colored Output** | Rich terminal output with progress indicators |
| **Interactive Prompts** | User-friendly prompts for project creation |

</div>

<h2 align="center">🛠️ Installation</h2>

### From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/mf2023/Ri.git
cd Ri/cli

# Build in release mode
cargo build --release

# The binary will be available at:
# ./target/release/ric

# Optional: Add to PATH
cp ./target/release/ric /usr/local/bin/
```

### Using Cargo

```bash
# Install directly from the repository
cargo install --git https://github.com/mf2023/Ri --bin ric
```

### Binary Download

Pre-built binaries are available for major platforms:

- **Linux**: `ric-linux-x86_64`
- **macOS**: `ric-darwin-x86_64`
- **Windows**: `ric-windows-x86_64.exe`

Download from the [Releases](https://github.com/mf2023/Ri/releases) page.

<h2 align="center">⚡ Quick Start</h2>

### Create a New Project

```bash
# Create a minimal project (default)
ric new my-project

# Create a web application
ric new my-web-app --template web

# Create an API service
ric new my-api --template api

# Create a worker service
ric new my-worker --template worker

# Create a microservice
ric new my-service --template microservice
```

### Build and Run

```bash
# Navigate to your project
cd my-project

# Build the project
ric build

# Build in release mode
ric build --release

# Run the project
ric run

# Run with release mode
ric run --release
```

### Configuration Management

```bash
# Initialize configuration file
ric config init

# Show current configuration
ric config show

# Validate configuration
ric config validate

# Set a configuration value
ric config set runtime.workers 8

# Get a configuration value
ric config get project.name
```

<h2 align="center">📋 Command Reference</h2>

### Project Management

| Command | Description |
|:--------|:-------------|
| `ric new <name>` | Create a new Ri project |
| `ric build` | Build the project |
| `ric run` | Run the project |
| `ric check` | Check project for errors |
| `ric clean` | Clean build artifacts |
| `ric info` | Show project information |

### Configuration

| Command | Description |
|:--------|:-------------|
| `ric config init` | Initialize configuration file |
| `ric config show` | Display current configuration |
| `ric config validate` | Validate configuration file |
| `ric config check` | Check environment variables |
| `ric config set <key> <value>` | Set a configuration value |
| `ric config get <key>` | Get a configuration value |

### Connection Testing

| Command | Description |
|:--------|:-------------|
| `ric test redis <url>` | Test Redis connection |
| `ric test postgres <url>` | Test PostgreSQL connection |
| `ric test mysql <url>` | Test MySQL connection |
| `ric test kafka <url>` | Test Kafka connection |

### Code Generation

| Command | Description |
|:--------|:-------------|
| `ric generate module <type> <name>` | Generate a new module |
| `ric generate middleware <name>` | Generate middleware template |
| `ric generate config <file>` | Generate Rust struct from config |

### Diagnostics

| Command | Description |
|:--------|:-------------|
| `ric doctor` | Run environment diagnostics |
| `ric doctor --verbose` | Detailed diagnostics |
| `ric doctor --fix` | Auto-fix detected issues |
| `ric version` | Show version information |

<h2 align="center">📁 Templates</h2>

Ri CLI provides five project templates for different use cases:

### Minimal

The simplest template with just the application builder and logger. Perfect for simple applications or learning Ri basics.

```bash
ric new my-minimal --template minimal
```

**Features**:
- RiAppBuilder for application initialization
- RiLogger for structured logging
- Minimal dependencies

### Web

Full-featured web application template with HTTP server, caching, and authentication.

```bash
ric new my-web --template web
```

**Features**:
- RiGateway for HTTP routing
- RiCacheModule for response caching
- RiAuthModule for authentication
- CORS and TLS support
- Development and production configurations

### API

RESTful API service template with validation and OpenAPI documentation support.

```bash
ric new my-api --template api
```

**Features**:
- RiGateway for API routing
- RiValidationModule for request validation
- Standard API response types
- Health check endpoints

### Worker

Background job processing service with queue management and device control.

```bash
ric new my-worker --template worker
```

**Features**:
- RiQueueModule for task processing
- RiDeviceControlModule for device management
- Configurable worker pools
- Dead letter queue support

### Microservice

Distributed microservice template with service mesh and observability.

```bash
ric new my-service --template microservice
```

**Features**:
- RiServiceMesh for service discovery
- RiObservabilityModule for metrics and tracing
- Distributed tracing support
- Health check server

<h2 align="center">🔧 Configuration</h2>

Ri CLI uses YAML configuration files (`ric.yaml`) for project settings.

### Configuration Structure

```yaml
# Project metadata
project:
  name: my-project
  version: 0.1.0
  template: web

# Build settings
build:
  release: false
  target: all
  features:
    - default

# Runtime settings
runtime:
  log_level: info
  workers: 4

# Module configurations
cache:
  enabled: true
  backend_type: Memory
  default_ttl_secs: 3600

gateway:
  listen_address: "0.0.0.0"
  listen_port: 8080
  cors_enabled: true
```

### Configuration File Locations

1. **Project Configuration**: `./ric.yaml` (current directory)
2. **Environment Override**: `RI_CONFIG_PATH` environment variable
3. **User Configuration**: `~/.config/ric/config.yaml`

### Environment Variables

| Variable | Description |
|:---------|:-------------|
| `RI_CONFIG_PATH` | Custom configuration file path |
| `RI_LOG_LEVEL` | Override log level |
| `RUST_LOG` | Rust logging configuration |
| `CARGO_HOME` | Cargo home directory |
| `RUSTUP_HOME` | Rustup home directory |

<h2 align="center">❓ Troubleshooting</h2>

### Common Issues

#### Project Creation Fails

**Error**: `Project directory already exists`

**Solution**: Choose a different project name or remove the existing directory.

```bash
# Remove existing directory
rm -rf my-project
ric new my-project
```

#### Build Fails

**Error**: `Could not find Ri in registry`

**Solution**: Ensure you're building from the Ri repository or have Ri available locally.

```bash
# Build from source
cd Ri
cargo build --release
```

#### Configuration Validation Fails

**Error**: `Invalid configuration: missing required field`

**Solution**: Use `ric config validate` to identify issues and fix them.

```bash
ric config validate
# Follow the suggestions to fix errors
```

#### Connection Test Fails

**Error**: `Connection refused`

**Solution**: Ensure the service is running and accessible.

```bash
# Check if Redis is running
redis-cli ping

# Test with correct URL
ric test redis redis://localhost:6379
```

### Diagnostic Tools

Run diagnostics to identify and fix common issues:

```bash
# Basic diagnostics
ric doctor

# Detailed diagnostics
ric doctor --verbose

# Auto-fix issues
ric doctor --fix
```

### Getting Help

1. **CLI Help**: Use `--help` flag for command-specific help
   ```bash
   ric --help
   ric new --help
   ric config --help
   ```

2. **Documentation**: Check the [docs](docs/) directory for detailed guides

3. **Issues**: Report bugs on [GitHub Issues](https://github.com/mf2023/Ri/issues)

<h2 align="center">🤝 Contributing</h2>

We welcome contributions! Please follow these guidelines:

### Development Setup

```bash
# Clone the repository
git clone https://github.com/mf2023/Ri.git
cd Ri/cli

# Install development dependencies
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

### Contribution Guidelines

1. **Fork the repository** and create a feature branch
2. **Write tests** for new functionality
3. **Follow Rust conventions** and run `cargo fmt`
4. **Update documentation** for changed functionality
5. **Submit a pull request** with a clear description

### Code Style

- Follow standard Rust conventions
- Use `cargo fmt` for formatting
- Run `cargo clippy` for linting
- Add documentation comments for public APIs

<div align="center">

## 📄 License & Open Source Agreements

### 🏛️ Project License

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

This project uses **Apache License 2.0** open source agreement, see [LICENSE](LICENSE) file.

### 📋 Dependency Package Open Source Agreements

Open source packages and their agreement information used by this project:

### Dependencies License

<div align="center">

| 📦 Package | 📜 License | 📦 Package | 📜 License |
|:-----------|:-----------|:-----------|:-----------|
| clap | MIT/Apache-2.0 | serde | MIT/Apache-2.0 |
| serde_yaml | MIT/Apache-2.0 | serde_json | MIT/Apache-2.0 |
| tera | MIT | anyhow | MIT/Apache-2.0 |
| thiserror | MIT/Apache-2.0 | colored | MIT |
| indicatif | MIT | dialoguer | MIT |
| tokio | MIT | chrono | MIT/Apache-2.0 |
| uuid | MIT/Apache-2.0 | regex | MIT/Apache-2.0 |
| walkdir | MIT/Apache-2.0 | toml | MIT/Apache-2.0 |
| async-trait | MIT/Apache-2.0 | ri | Apache-2.0 |
| redis | MIT | tokio-postgres | MIT/Apache-2.0 |
| mysql_async | MIT | rdkafka | BSD-2-Clause |

</div>

</div>