<div align="center">
<img src="assets/svg/ri.svg" width="36" height="36">
</div>

First off, thank you for considering contributing to Ri (Ri)! It's people like you that make Ri such a great tool.

This document provides guidelines and instructions for contributing to the Ri project. By participating, you are expected to uphold this code and help us maintain a welcoming and productive community.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How Can I Contribute?](#how-can-i-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Pull Requests](#pull-requests)
- [Development Guidelines](#development-guidelines)
  - [Setting Up Development Environment](#setting-up-development-environment)
  - [Building the Project](#building-the-project)
  - [Running Tests](#running-tests)
  - [Code Style](#code-style)
  - [Commit Messages](#commit-messages)
- [Project Structure](#project-structure)
- [Community](#community)
- [License](#license)

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

- Make sure you have a [GitHub account](https://github.com/signup/free)
- Fork the repository on GitHub
- Set up your development environment (see [Development Guidelines](#development-guidelines))
- Familiarize yourself with the [project structure](#project-structure)

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the [existing issues](https://github.com/mf2023/Ri/issues) to see if the problem has already been reported. When you are creating a bug report, please include as many details as possible:

#### Before Submitting a Bug Report

- **Check the [documentation](https://github.com/mf2023/Ri/tree/master/doc)** for information that might help
- **Check if the bug has already been reported** by searching on GitHub under [Issues](https://github.com/mf2023/Ri/issues)
- **Determine which repository the problem should be reported in** (Ri has multiple related repositories)

#### How to Submit a Good Bug Report

Bugs are tracked as [GitHub issues](https://github.com/mf2023/Ri/issues). Create an issue and provide the following information:

- **Use a clear and descriptive title** for the issue to identify the problem
- **Describe the exact steps to reproduce the problem** in as many details as possible
- **Provide specific examples to demonstrate the steps**. Include links to files or GitHub projects, or copy/pasteable snippets
- **Describe the behavior you observed** and why it's a problem
- **Explain which behavior you expected to see instead and why**
- **Include code samples and screenshots** which show you demonstrating the problem

**Example:**

```markdown
**Description:**
WebSocket client fails to connect when using TLS on Windows

**Steps to Reproduce:**
1. Create a RiWSClient instance
2. Call connect() with wss:// URL
3. Observe the error

**Expected Behavior:**
Connection should succeed with TLS handshake

**Actual Behavior:**
Error: "TLS handshake failed: certificate validation error"

**Environment:**
- OS: Windows 11
- Ri Version: 0.1.9
- Rust Version: 1.75.0
```

### Suggesting Enhancements

Enhancement suggestions are tracked as [GitHub issues](https://github.com/mf2023/Ri/issues). When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title** for the issue to identify the suggestion
- **Provide a step-by-step description of the suggested enhancement** in as many details as possible
- **Provide specific examples to demonstrate the enhancement**
- **Explain why this enhancement would be useful** to most Ri users
- **List some other middleware frameworks or libraries where this enhancement exists**

### Pull Requests

1. Fork the repo and create your branch from `master`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code follows the style guidelines
6. Issue that pull request!

#### Pull Request Process

1. Update the [CHANGELOG.md](CHANGELOG.md) with details of changes if applicable
2. Update the [README.md](README.md) with details of changes to the interface if applicable
3. The PR will be merged once you have the sign-off of at least one maintainer

## Development Guidelines

### Setting Up Development Environment

#### Prerequisites

- **Rust** (latest stable version): [Install Rust](https://www.rust-lang.org/tools/install)
- **Python** (3.8+ for Python bindings development): [Install Python](https://www.python.org/downloads/)
- **CMake** (for building with Kafka support): [Install CMake](https://cmake.org/download/)
- **Protocol Buffers Compiler (protoc)**: [Install protoc](https://github.com/protocolbuffers/protobuf/releases)

#### Windows-Specific Requirements

For building with Kafka support on Windows:
- Visual Studio 2022 with C++ workload, **OR**
- MinGW-w64: `choco install mingw`

#### Clone the Repository

```bash
git clone https://github.com/mf2023/Ri.git
cd Ri
```

#### Install Python Dependencies (for Python bindings)

```bash
cd python
pip install maturin
```

### Building the Project

#### Build Rust Library

```bash
# Build with all features
cargo build --release

# Build with specific features
cargo build --release --features "protocol,grpc,websocket"

# Build without Kafka (if you don't have CMake installed)
cargo build --release --no-default-features --features "pyo3,grpc,websocket"
```

#### Build Python Wheels

```bash
cd python
maturin build --release
```

### Running Tests

#### Rust Tests

```bash
# Run all tests
cargo test

# Run tests with all features
cargo test --all-features

# Run tests for a specific module
cargo test protocol::
```

#### Python Tests

```bash
cd python
pip install -e .
python -m pytest tests/
```

### Code Style

#### Rust Code Style

We follow the official [Rust Style Guide](https://doc.rust-lang.org/style-guide/) and use `rustfmt` for formatting:

```bash
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt -- --check

# Run clippy for linting
cargo clippy --all-features
```

#### Python Code Style

For Python bindings, we follow [PEP 8](https://www.python.org/dev/peps/pep-0008/):

```bash
# Format with black
black python/

# Lint with flake8
flake8 python/
```

#### Documentation

- All public APIs must have documentation comments
- Use `cargo doc` to generate documentation
- Documentation should include examples where appropriate

### Commit Messages

This project uses **date-based commit messages** in the format `YYYY.MM.DD`:

```
2026.01.31
```

#### Format

- Use the **current date** in `YYYY.MM.DD` format
- No additional description
- No body or footer

#### Examples

```bash
# Good
git commit -m "2026.01.31"

# Bad - don't use conventional commits or descriptions
git commit -m "feat(protocol): add SM4 support"
git commit -m "fix bug in websocket"
```

#### Why Date-Based?

- **Simple**: No need to think about commit message format
- **Clear timeline**: Easy to see when changes were made
- **Consistent**: All commits follow the same pattern
- **Changelog**: Detailed changes are tracked in [CHANGELOG.md](CHANGELOG.md)

#### Tracking Changes

Since commit messages are minimal, detailed change information is maintained in:

- **[CHANGELOG.md](CHANGELOG.md)**: Version history and release notes
- **GitHub Issues/PRs**: Detailed discussion and context
- **Code comments**: Inline documentation for complex changes

## Project Structure

```
Ri/
├── src/                    # Rust source code
│   ├── auth/              # Authentication and authorization
│   ├── cache/             # Caching module
│   ├── core/              # Core framework (app builder, runtime, etc.)
│   ├── database/          # Database support (PostgreSQL, MySQL, SQLite)
│   ├── device/            # Device management
│   ├── gateway/           # API Gateway (routing, load balancing, rate limiting)
│   ├── grpc/              # gRPC client and server
│   ├── observability/     # Metrics, tracing, logging
│   ├── protocol/          # Protocol implementations (crypto, frames)
│   ├── queue/             # Message queues (Redis, RabbitMQ, Kafka)
│   ├── service_mesh/      # Service mesh functionality
│   └── ws/                # WebSocket client and server
├── python/                # Python bindings
│   └── ri/             # Python package
├── doc/                   # Documentation
│   ├── en/               # English documentation
│   └── zh/               # Chinese documentation
├── .github/               # GitHub configuration
│   └── workflows/        # CI/CD workflows
├── Cargo.toml            # Rust package configuration
├── README.md             # Project readme
├── CHANGELOG.md          # Version changelog
└── LICENSE               # Apache 2.0 License
```

## Module Contribution Guidelines

### Adding a New Module

1. Create a new directory under `src/`
2. Add `mod.rs` as the module entry point
3. Implement the module following the existing patterns
4. Add Python bindings if applicable
5. Add documentation and examples
6. Update the main `lib.rs` to export the module

### Module Requirements

Each module should:
- Have a clear purpose and scope
- Follow the existing error handling patterns
- Include comprehensive documentation
- Provide Python bindings (if applicable)
- Include unit tests

## Community

### Communication Channels

- **Gitee Issues** (Primary): Bug reports, feature requests, and general discussion - https://gitee.com/dunimd/ri/issues
- **GitHub Issues** (Mirror): Alternative access - https://github.com/mf2023/Ri/issues
- **GitHub Discussions**: For questions and community interaction

### Repositories

- **Gitee** (Primary): https://gitee.com/dunimd/ri.git
- **GitHub** (Mirror): https://github.com/mf2023/Ri.git

### Recognition

Contributors will be recognized in our [CHANGELOG.md](CHANGELOG.md) and release notes.

## License

By contributing to Ri, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).

---

Thank you for contributing to Ri! 🎉
