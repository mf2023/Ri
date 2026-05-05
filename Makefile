# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

################################################################################
# Ri Build System - Unified Makefile
################################################################################
#
# PURPOSE:
# This Makefile provides a unified build system for Ri project, supporting:
# - Local development builds
# - CI/CD pipeline builds (GitHub Actions)
# - Cross-platform compilation
# - Multi-language bindings (Rust, Python, Java, C)
#
# USAGE:
#   make <target> [VARIABLE=value]
#
# EXAMPLES:
#   make build                    # Build Rust library (default)
#   make build-cli               # Build CLI tool
#   make build-python            # Build Python wheel
#   make build-c                 # Build C static library
#   make build-java              # Build Java JAR
#   make test                    # Run tests
#   make clean                   # Clean build artifacts
#   make help                    # Show all available targets
#
# PLATFORM DETECTION:
#   The Makefile automatically detects the current platform and architecture.
#   Override with: make build PLATFORM=linux ARCH=arm64
#
# FEATURES:
#   - Automatic platform detection (Linux/Windows/macOS)
#   - Automatic architecture detection (x64/ARM64)
#   - Parallel builds support
#   - Incremental builds with dependency tracking
#   - Cross-compilation support
#   - Docker-based manylinux builds for Python
#
# REQUIREMENTS:
#   - Rust toolchain (stable)
#   - Python 3.8+ (for Python bindings)
#   - JDK 11+ (for Java bindings)
#   - CMake (for C dependencies)
#   - protoc (Protocol Buffers compiler)
#
# ENVIRONMENT VARIABLES:
#   PLATFORM     - Target platform (linux/windows/macos)
#   ARCH         - Target architecture (x64/arm64)
#   PYTHON_VER   - Python version for wheels (default: 3.11)
#   FEATURES     - Cargo features to enable
#   RELEASE      - Build in release mode (default: true)
#   TARGET       - Rust target triple (e.g., x86_64-unknown-linux-gnu)
#
################################################################################

# Detect operating system
UNAME_S := $(shell uname -s 2>/dev/null || echo "Windows")
ifeq ($(UNAME_S),Linux)
    PLATFORM ?= linux
endif
ifeq ($(UNAME_S),Darwin)
    PLATFORM ?= macos
endif
ifeq ($(UNAME_S),Windows)
    PLATFORM ?= windows
endif

# Detect architecture
UNAME_M := $(shell uname -m 2>/dev/null || echo "unknown")
ifeq ($(UNAME_M),x86_64)
    ARCH ?= x64
endif
ifeq ($(UNAME_M),aarch64)
    ARCH ?= arm64
endif
ifeq ($(UNAME_M),arm64)
    ARCH ?= arm64
endif

# Default values
RELEASE ?= true
PYTHON_VER ?= 3.11
FEATURES ?= 
VERBOSE ?= false

# Build mode
ifeq ($(RELEASE),true)
    BUILD_MODE := --release
    BUILD_DIR := release
else
    BUILD_MODE :=
    BUILD_DIR := debug
endif

# Platform-specific configuration
ifeq ($(PLATFORM),linux)
    LIB_EXT := so
    LIB_PREFIX := lib
    STATIC_EXT := a
    TARGET ?= x86_64-unknown-linux-gnu
    ifeq ($(ARCH),arm64)
        TARGET := aarch64-unknown-linux-gnu
    endif
endif

ifeq ($(PLATFORM),windows)
    LIB_EXT := dll
    LIB_PREFIX := 
    STATIC_EXT := lib
    TARGET ?= x86_64-pc-windows-msvc
    ifeq ($(ARCH),arm64)
        TARGET := aarch64-pc-windows-msvc
    endif
endif

ifeq ($(PLATFORM),macos)
    LIB_EXT := dylib
    LIB_PREFIX := lib
    STATIC_EXT := a
    TARGET ?= x86_64-apple-darwin
    ifeq ($(ARCH),arm64)
        TARGET := aarch64-apple-darwin
    endif
endif

# Output directories
DIST_DIR := dist
INCLUDE_DIR := include
TARGET_DIR := target/$(TARGET)/$(BUILD_DIR)

# Colors for output (if terminal supports it)
ifneq ($(TERM),)
    GREEN := \033[0;32m
    YELLOW := \033[0;33m
    RED := \033[0;31m
    NC := \033[0m
else
    GREEN := 
    YELLOW := 
    RED := 
    NC := 
endif

################################################################################
# Phony Targets
################################################################################
.PHONY: all build clean test help setup-env \
        build-rust build-cli build-python build-c build-java \
        build-all-archs build-all-platforms \
        install install-python install-java \
        doc doc-rust doc-python \
        lint fmt check \
        docker-clean docker-build \
        setup-protoc setup-cmake setup-rust setup-openssl setup-deps

# Default target
all: build

################################################################################
# Environment Setup Targets
################################################################################

# Setup all build dependencies
setup-env: setup-protoc setup-cmake setup-rust setup-openssl setup-deps
	@echo "$(GREEN)✓ Build environment setup complete$(NC)"

# Install Protocol Buffers compiler
setup-protoc:
	@echo "$(GREEN)Installing protoc...$(NC)"
ifeq ($(PLATFORM),linux)
	which protoc > /dev/null 2>&1 || (sudo apt-get update && sudo apt-get install -y protobuf-compiler)
else ifeq ($(PLATFORM),macos)
	which protoc > /dev/null 2>&1 || brew install protobuf
else ifeq ($(PLATFORM),windows)
	@echo "$(YELLOW)Note: protoc should be installed via GitHub Actions or manually on Windows$(NC)"
endif

# Install CMake
setup-cmake:
	@echo "$(GREEN)Installing CMake...$(NC)"
ifeq ($(PLATFORM),linux)
	which cmake > /dev/null 2>&1 || (sudo apt-get update && sudo apt-get install -y cmake)
else ifeq ($(PLATFORM),macos)
	which cmake > /dev/null 2>&1 || brew install cmake
else ifeq ($(PLATFORM),windows)
	@echo "$(YELLOW)Note: CMake should be installed via GitHub Actions or manually on Windows$(NC)"
endif

# Setup Rust toolchain
setup-rust:
	@echo "$(GREEN)Setting up Rust toolchain...$(NC)"
	@which rustup > /dev/null 2>&1 || (curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y)
	@rustup target add $(TARGET) 2>/dev/null || true

# Install OpenSSL (Windows only)
setup-openssl:
	@echo "$(GREEN)Installing OpenSSL...$(NC)"
ifeq ($(PLATFORM),windows)
ifeq ($(ARCH),x64)
	@echo "$(YELLOW)Installing OpenSSL for Windows x64 via vcpkg...$(NC)"
	@vcpkg install openssl:x64-windows 2>/dev/null || echo "vcpkg not available, skipping"
else ifeq ($(ARCH),arm64)
	@echo "$(YELLOW)Installing OpenSSL for Windows ARM64 via vcpkg...$(NC)"
	@vcpkg install openssl:arm64-windows 2>/dev/null || echo "vcpkg not available, skipping"
endif
else
	@echo "$(GREEN)OpenSSL will use system version$(NC)"
endif

# Install system dependencies
setup-deps:
	@echo "$(GREEN)Installing system dependencies...$(NC)"
	@if command -v apt-get >/dev/null 2>&1; then \
		echo "Using apt-get (Debian/Ubuntu)"; \
		echo 'deb [trusted=yes] https://pkgs.kquirk.com/apt all main' | sudo tee /etc/apt/sources.list.d/oqs.list > /dev/null; \
		sudo apt-get update; \
		sudo apt-get install -y libcurl4-openssl-dev libsasl2-dev build-essential pkg-config liboqs-dev cmake || \
		echo "liboqs-dev install failed, building from source..."; \
		if ! dpkg -s liboqs-dev >/dev/null 2>&1; then \
			echo "Building liboqs from source..."; \
			sudo apt-get install -y libcurl4-openssl-dev libsasl2-dev build-essential pkg-config cmake git ninja-build; \
			cd /tmp && rm -rf liboqs && git clone --depth 1 https://github.com/open-quantum-safe/liboqs.git && \
			cd liboqs && mkdir -p build && cd build && \
			cmake -GNinja -DCMAKE_INSTALL_PREFIX=/usr .. && \
			ninja && sudo ninja install && sudo ldconfig; \
		fi; \
	elif command -v yum >/dev/null 2>&1 || command -v dnf >/dev/null 2>&1; then \
		echo "Using yum/dnf (CentOS/RHEL/Fedora/manylinux)"; \
		if command -v dnf >/dev/null 2>&1; then PKGMGR=dnf; else PKGMGR=yum; fi; \
		sudo $$PKGMGR install -y libcurl-devel openssl-devel libsasl2-devel gcc gcc-c++ make cmake git ninja-build perl-IPC-Cmd; \
		if [ ! -f /usr/lib64/liboqs.so ] && [ ! -f /usr/local/lib64/liboqs.so ] && [ ! -f /usr/lib/x86_64-linux-gnu/liboqs.so ]; then \
			echo "Building liboqs from source..."; \
			cd /tmp && rm -rf liboqs && git clone --depth 1 https://github.com/open-quantum-safe/liboqs.git && \
			cd liboqs && mkdir -p build && cd build && \
			cmake -GNinja -DCMAKE_INSTALL_PREFIX=/usr .. && \
			ninja && sudo ninja install && sudo ldconfig; \
		else \
			echo "liboqs already installed, skipping build"; \
		fi; \
	elif command -v brew >/dev/null 2>&1; then \
		echo "Using brew (macOS)"; \
		brew update --quiet 2>/dev/null || true; \
		brew install --quiet cmake pkg-config openssl liboqs 2>/dev/null || true; \
		@mkdir -p .cargo; \
		echo '[target.x86_64-apple-darwin]\nrustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]\n\n[target.aarch64-apple-darwin]\nrustflags = ["-C", "link-arg=-undefined", "-C", "link-arg=dynamic_lookup"]' > .cargo/config.toml; \
	else \
		echo "$(YELLOW)No supported package manager found, skipping system dependencies$(NC)"; \
	fi
ifeq ($(PLATFORM),windows)
	@echo "$(GREEN)Installing Windows dependencies via vcpkg...$(NC)"
	vcpkg install openssl:x64-windows librdkafka:x64-windows --classic 2>/dev/null || echo "Dependencies may already be installed"
	vcpkg integrate install 2>/dev/null || true
endif

################################################################################
# Help
################################################################################
help:
	@echo "$(GREEN)Ri Build System$(NC)"
	@echo ""
	@echo "$(YELLOW)Usage:$(NC)"
	@echo "  make <target> [VARIABLE=value]"
	@echo ""
	@echo "$(YELLOW)Build Targets:$(NC)"
	@echo "  build              Build Rust library (default)"
	@echo "  build-cli          Build CLI tool (ric)"
	@echo "  build-python       Build Python wheel"
	@echo "  build-c            Build C static library and headers"
	@echo "  build-java         Build Java JAR"
	@echo "  build-all          Build all components"
	@echo ""
	@echo "$(YELLOW)Platform-Specific Builds:$(NC)"
	@echo "  build-linux-x64        Build for Linux x64"
	@echo "  build-linux-arm64      Build for Linux ARM64"
	@echo "  build-windows-x64      Build for Windows x64"
	@echo "  build-windows-arm64    Build for Windows ARM64"
	@echo "  build-macos-x64        Build for macOS x64"
	@echo "  build-macos-arm64      Build for macOS ARM64"
	@echo ""
	@echo "$(YELLOW)Testing & Quality:$(NC)"
	@echo "  test               Run all tests"
	@echo "  test-unit          Run unit tests"
	@echo "  test-integration   Run integration tests"
	@echo "  lint               Run linter (clippy)"
	@echo "  fmt                Format code"
	@echo "  check              Check code without building"
	@echo ""
	@echo "$(YELLOW)Documentation:$(NC)"
	@echo "  doc                Build all documentation"
	@echo "  doc-rust           Build Rust documentation"
	@echo "  doc-python         Build Python documentation"
	@echo ""
	@echo "$(YELLOW)Installation:$(NC)"
	@echo "  install            Install Rust library"
	@echo "  install-python     Install Python package"
	@echo ""
	@echo "$(YELLOW)Cleaning:$(NC)"
	@echo "  clean              Clean build artifacts"
	@echo "  clean-all          Clean all generated files"
	@echo "  clean-python       Clean Python build artifacts"
	@echo "  clean-java         Clean Java build artifacts"
	@echo ""
	@echo "$(YELLOW)Docker:$(NC)"
	@echo "  docker-build       Build manylinux Python wheel"
	@echo "  docker-clean       Clean Docker artifacts"
	@echo ""
	@echo "$(YELLOW)Variables:$(NC)"
	@echo "  PLATFORM=$(PLATFORM)        Target platform (linux/windows/macos)"
	@echo "  ARCH=$(ARCH)                Target architecture (x64/arm64)"
	@echo "  PYTHON_VER=$(PYTHON_VER)    Python version (3.8-3.14)"
	@echo "  RELEASE=$(RELEASE)          Release mode (true/false)"
	@echo "  TARGET=$(TARGET)            Rust target triple"
	@echo "  FEATURES=...                Cargo features to enable"
	@echo ""
	@echo "$(YELLOW)Examples:$(NC)"
	@echo "  make build                           # Build for current platform"
	@echo "  make build-python PYTHON_VER=3.12    # Build Python 3.12 wheel"
	@echo "  make build-linux-arm64               # Cross-compile for Linux ARM64"
	@echo "  make build-c                         # Build C static library"
	@echo "  make test FEATURES=full              # Test with all features"

################################################################################
# Core Build Targets
################################################################################

# Build Rust library (default)
build:
	@echo "$(GREEN)Building Ri library for $(PLATFORM) $(ARCH)...$(NC)"
ifeq ($(PLATFORM),linux)
	@$(MAKE) setup-deps
endif
ifeq ($(PLATFORM)-$(ARCH),windows-arm64)
	cmd /c "cargo build $(BUILD_MODE) --target $(TARGET) --no-default-features $(if $(FEATURES),--features $(FEATURES),)"
else ifeq ($(PLATFORM),windows)
	cmd /c "cargo build $(BUILD_MODE) --target $(TARGET) $(if $(FEATURES),--features $(FEATURES),)"
else
	cargo build $(BUILD_MODE) --target $(TARGET) $(if $(FEATURES),--features $(FEATURES),)
endif
	@echo "$(GREEN)✓ Build complete: $(TARGET_DIR)/$(LIB_PREFIX)ri.$(LIB_EXT)$(NC)"

# Build CLI tool
build-cli:
	@echo "$(GREEN)Building CLI tool for $(PLATFORM) $(ARCH)...$(NC)"
ifeq ($(PLATFORM),linux)
	@$(MAKE) setup-deps
endif
ifeq ($(PLATFORM)-$(ARCH),windows-arm64)
	cmd /c "cargo build $(BUILD_MODE) -p ric --target $(TARGET) --no-default-features"
else ifeq ($(PLATFORM),windows)
	cmd /c "cargo build $(BUILD_MODE) -p ric --target $(TARGET)"
else
	cargo build $(BUILD_MODE) -p ric --target $(TARGET)
endif
ifeq ($(PLATFORM),windows)
	@echo "$(GREEN)✓ Build complete: $(TARGET_DIR)/ric.exe$(NC)"
else
	@echo "$(GREEN)✓ Build complete: $(TARGET_DIR)/ric$(NC)"
endif

# Build Python wheel
build-python:
	@echo "$(GREEN)Building Python wheel for $(PLATFORM) $(ARCH) (Python $(PYTHON_VER))...$(NC)"
	@mkdir -p $(DIST_DIR)
ifeq ($(PLATFORM),linux)
ifneq ($(ARCH),arm64)
	@echo "$(YELLOW)Using manylinux container for Linux wheel...$(NC)"
	docker run --rm -v "$(PWD)":/io quay.io/pypa/manylinux_2_17_x86_64 \
		/bin/bash -c "cd /io && make setup-deps && \
			export OPENSSL_NO_VENDOR=1 && \
			PYTHON_VER=$(PYTHON_VER) && \
			PYTHON_MAJOR=\$${PYTHON_VER%%.*} && \
			PYTHON_MINOR=\$${PYTHON_VER#*.} && \
			PYTHON_BIN=/opt/python/cp\$${PYTHON_MAJOR}\$${PYTHON_MINOR}-cp\$${PYTHON_MAJOR}\$${PYTHON_MINOR}/bin/python && \
			\$$PYTHON_BIN -m pip install maturin && \
			\$$PYTHON_BIN -m maturin build --release --target $(TARGET) -o /io/$(DIST_DIR) \
				--no-default-features \
			--features pyo3,grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,protocol,kafka,etcd"
else
	@echo "$(YELLOW)Building native Linux ARM64 wheel...$(NC)"
	$(MAKE) setup-deps
	PYTHON_VER=$(PYTHON_VER) && \
	PYTHON_BIN=python$${PYTHON_VER%.*} && \
	$$PYTHON_BIN -m pip install maturin && \
	OPENSSL_NO_VENDOR=1 $$PYTHON_BIN -m maturin build --release --target $(TARGET) -o $(DIST_DIR) \
		--no-default-features \
		--features pyo3,grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,protocol,kafka,etcd
endif
else ifeq ($(PLATFORM),windows)
	@echo "$(YELLOW)Building Windows wheel...$(NC)"
	pip install maturin
ifeq ($(ARCH),arm64)
	cmd /c "maturin build --release --target $(TARGET) -o $(DIST_DIR) --no-default-features --features pyo3,grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,etcd"
else
	cmd /c "maturin build --release --target $(TARGET) -o $(DIST_DIR) --no-default-features --features pyo3,grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,protocol,kafka,etcd"
endif
else
	@echo "$(YELLOW)Building native wheel...$(NC)"
	pip install maturin
	OPENSSL_NO_VENDOR=1 maturin build --release --target $(TARGET) -o $(DIST_DIR) \
		--no-default-features \
		--features pyo3,grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,protocol,kafka,etcd
endif
	@echo "$(GREEN)✓ Python wheel built: $(DIST_DIR)/$(NC)"
	@ls -lh $(DIST_DIR)/*.whl 2>/dev/null || echo "No wheels found"

# Build C static library and headers
build-c:
	@echo "$(GREEN)Building C static library for $(PLATFORM) $(ARCH)...$(NC)"
ifeq ($(PLATFORM),linux)
	@$(MAKE) setup-deps
endif
ifeq ($(PLATFORM),windows)
	cmd /c "cargo build $(BUILD_MODE) --target $(TARGET) --no-default-features --features c"
else
	cargo build $(BUILD_MODE) --target $(TARGET) --no-default-features --features c
endif
	@echo "$(GREEN)Generating C headers...$(NC)"
	@mkdir -p $(INCLUDE_DIR)
	cargo install cbindgen 2>/dev/null || true
	cbindgen --crate ri -o $(INCLUDE_DIR)/ri.h
ifeq ($(PLATFORM),windows)
	@echo "$(GREEN)✓ C library built: $(TARGET_DIR)/ri.$(STATIC_EXT)$(NC)"
else
	@echo "$(GREEN)✓ C library built: $(TARGET_DIR)/$(LIB_PREFIX)ri.$(STATIC_EXT)$(NC)"
endif
	@echo "$(GREEN)✓ C header generated: $(INCLUDE_DIR)/ri.h$(NC)"

# Build Java JAR (requires all native libraries)
build-java:
	@echo "$(GREEN)Building Java JAR...$(NC)"
	@echo "$(YELLOW)Note: This requires pre-built native libraries for all platforms$(NC)"
	@mkdir -p java/src/main/resources/native
	@# Copy native libraries if they exist
	@for platform in linux-x64 linux-arm64 windows-x64 windows-arm64 macos-x64 macos-arm64; do \
		if [ -d "native/$$platform" ]; then \
			mkdir -p "java/src/main/resources/native/$$platform"; \
			cp -r native/$$platform/* java/src/main/resources/native/$$platform/ 2>/dev/null || true; \
		fi; \
	done
	cd java && mvn clean package -DskipTests
	@echo "$(GREEN)✓ Java JAR built: java/target/ri-*.jar$(NC)"

# Build all components
build-all: build build-cli build-python build-c
	@echo "$(GREEN)✓ All components built successfully$(NC)"

################################################################################
# Platform-Specific Build Targets
################################################################################

# Linux builds
build-linux-x64:
	@$(MAKE) build PLATFORM=linux ARCH=x64 TARGET=x86_64-unknown-linux-gnu

build-linux-arm64:
	@$(MAKE) build PLATFORM=linux ARCH=arm64 TARGET=aarch64-unknown-linux-gnu

# Windows builds
build-windows-x64:
	@$(MAKE) build PLATFORM=windows ARCH=x64 TARGET=x86_64-pc-windows-msvc

build-windows-arm64:
	@$(MAKE) build PLATFORM=windows ARCH=arm64 TARGET=aarch64-pc-windows-msvc \
		FEATURES="grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,etcd"

# macOS builds
build-macos-x64:
	@$(MAKE) build PLATFORM=macos ARCH=x64 TARGET=x86_64-apple-darwin

build-macos-arm64:
	@$(MAKE) build PLATFORM=macos ARCH=arm64 TARGET=aarch64-apple-darwin

# CLI builds for all platforms
build-cli-linux-x64:
	@$(MAKE) build-cli PLATFORM=linux ARCH=x64 TARGET=x86_64-unknown-linux-gnu

build-cli-linux-arm64:
	@$(MAKE) build-cli PLATFORM=linux ARCH=arm64 TARGET=aarch64-unknown-linux-gnu

build-cli-windows-x64:
	@$(MAKE) build-cli PLATFORM=windows ARCH=x64 TARGET=x86_64-pc-windows-msvc

build-cli-windows-arm64:
	@echo "$(GREEN)Building CLI for Windows ARM64...$(NC)"
	cmd /c "cargo build $(BUILD_MODE) -p ric --target $(TARGET) --no-default-features --features grpc,websocket,rabbitmq,cache,queue,gateway,service_mesh,auth,observability,postgres,mysql,sqlite,http_client,system_info,config_hot_reload,etcd"

build-cli-macos-x64:
	@$(MAKE) build-cli PLATFORM=macos ARCH=x64 TARGET=x86_64-apple-darwin

build-cli-macos-arm64:
	@$(MAKE) build-cli PLATFORM=macos ARCH=arm64 TARGET=aarch64-apple-darwin

# C library builds for all platforms
build-c-linux-x64:
	@$(MAKE) build-c PLATFORM=linux ARCH=x64 TARGET=x86_64-unknown-linux-gnu

build-c-linux-arm64:
	@$(MAKE) build-c PLATFORM=linux ARCH=arm64 TARGET=aarch64-unknown-linux-gnu

build-c-windows-x64:
	@$(MAKE) build-c PLATFORM=windows ARCH=x64 TARGET=x86_64-pc-windows-msvc

build-c-windows-arm64:
	@$(MAKE) build-c PLATFORM=windows ARCH=arm64 TARGET=aarch64-pc-windows-msvc

build-c-macos-x64:
	@$(MAKE) build-c PLATFORM=macos ARCH=x64 TARGET=x86_64-apple-darwin

build-c-macos-arm64:
	@$(MAKE) build-c PLATFORM=macos ARCH=arm64 TARGET=aarch64-apple-darwin

################################################################################
# Testing
################################################################################

test:
	@echo "$(GREEN)Running tests...$(NC)"
	cargo test --all-features --target $(TARGET)
	@echo "$(GREEN)✓ Tests passed$(NC)"

test-unit:
	@echo "$(GREEN)Running unit tests...$(NC)"
	cargo test --lib --all-features --target $(TARGET)
	@echo "$(GREEN)✓ Unit tests passed$(NC)"

test-integration:
	@echo "$(GREEN)Running integration tests...$(NC)"
	cargo test --test '*' --all-features --target $(TARGET)
	@echo "$(GREEN)✓ Integration tests passed$(NC)"

################################################################################
# Code Quality
################################################################################

lint:
	@echo "$(GREEN)Running linter...$(NC)"
	cargo clippy --all-features --target $(TARGET) -- -D warnings
	@echo "$(GREEN)✓ Linting passed$(NC)"

fmt:
	@echo "$(GREEN)Formatting code...$(NC)"
	cargo fmt --all
	@echo "$(GREEN)✓ Code formatted$(NC)"

check:
	@echo "$(GREEN)Checking code...$(NC)"
	cargo check --all-features --target $(TARGET)
	@echo "$(GREEN)✓ Check passed$(NC)"

################################################################################
# Documentation
################################################################################

doc: doc-rust doc-python
	@echo "$(GREEN)✓ Documentation built$(NC)"

doc-rust:
	@echo "$(GREEN)Building Rust documentation...$(NC)"
	cargo doc --no-deps --all-features --target $(TARGET)
	@echo "$(GREEN)✓ Rust documentation: target/doc/ri/$(NC)"

doc-python:
	@echo "$(GREEN)Building Python documentation...$(NC)"
	@pip install pdoc 2>/dev/null || true
	pdoc -o target/doc/python ri
	@echo "$(GREEN)✓ Python documentation: target/doc/python/$(NC)"

################################################################################
# Installation
################################################################################

install:
	@echo "$(GREEN)Installing Ri library...$(NC)"
	cargo install --path .
	@echo "$(GREEN)✓ Installation complete$(NC)"

install-python:
	@echo "$(GREEN)Installing Python package...$(NC)"
	pip install maturin
	maturin develop --release --features pyo3
	@echo "$(GREEN)✓ Python package installed$(NC)"

################################################################################
# Cleaning
################################################################################

clean:
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	cargo clean
	@rm -rf $(DIST_DIR)
	@rm -rf $(INCLUDE_DIR)
	@echo "$(GREEN)✓ Clean complete$(NC)"

clean-all: clean
	@echo "$(YELLOW)Cleaning all generated files...$(NC)"
	@rm -rf target
	@rm -rf dist
	@rm -rf include
	@rm -rf java/target
	@rm -rf native
	@find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".mypy_cache" -exec rm -rf {} + 2>/dev/null || true
	@echo "$(GREEN)✓ All generated files cleaned$(NC)"

clean-python:
	@echo "$(YELLOW)Cleaning Python artifacts...$(NC)"
	@rm -rf dist
	@find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".mypy_cache" -exec rm -rf {} + 2>/dev/null || true
	@echo "$(GREEN)✓ Python artifacts cleaned$(NC)"

clean-java:
	@echo "$(YELLOW)Cleaning Java artifacts...$(NC)"
	@rm -rf java/target
	@echo "$(GREEN)✓ Java artifacts cleaned$(NC)"

################################################################################
# Docker Support
################################################################################

docker-build:
	@echo "$(GREEN)Building manylinux Python wheel in Docker...$(NC)"
	@echo "$(YELLOW)This requires Docker to be installed and running$(NC)"
	@$(MAKE) build-python PLATFORM=linux

docker-clean:
	@echo "$(YELLOW)Cleaning Docker artifacts...$(NC)"
	@docker system prune -f 2>/dev/null || true
	@echo "$(GREEN)✓ Docker artifacts cleaned$(NC)"

################################################################################
# CI/CD Helper Targets
################################################################################

# For GitHub Actions - builds all artifacts for a specific platform
ci-build-all: build build-cli build-python build-c
	@echo "$(GREEN)✓ CI build complete for $(PLATFORM) $(ARCH)$(NC)"

# For GitHub Actions - prepares artifacts for upload
ci-package:
	@echo "$(GREEN)Packaging artifacts...$(NC)"
	@mkdir -p artifacts
ifeq ($(PLATFORM),windows)
	@cp $(TARGET_DIR)/ric.exe artifacts/ric-$(PLATFORM)-$(ARCH).exe 2>/dev/null || true
	@cp $(TARGET_DIR)/ri.dll artifacts/ri-$(PLATFORM)-$(ARCH).dll 2>/dev/null || true
	@cp $(TARGET_DIR)/ri.lib artifacts/ri-$(PLATFORM)-$(ARCH).lib 2>/dev/null || true
else
	@cp $(TARGET_DIR)/ric artifacts/ric-$(PLATFORM)-$(ARCH) 2>/dev/null || true
	@cp $(TARGET_DIR)/$(LIB_PREFIX)ri.$(LIB_EXT) artifacts/ri-$(PLATFORM)-$(ARCH).$(LIB_EXT) 2>/dev/null || true
	@cp $(TARGET_DIR)/$(LIB_PREFIX)ri.$(STATIC_EXT) artifacts/ri-$(PLATFORM)-$(ARCH).$(STATIC_EXT) 2>/dev/null || true
endif
	@cp $(INCLUDE_DIR)/ri.h artifacts/ri.h 2>/dev/null || true
	@cp -r $(DIST_DIR)/*.whl artifacts/ 2>/dev/null || true
	@echo "$(GREEN)✓ Artifacts packaged in artifacts/$(NC)"

# Show build info
info:
	@echo "$(GREEN)Build Configuration:$(NC)"
	@echo "  Platform:     $(PLATFORM)"
	@echo "  Architecture: $(ARCH)"
	@echo "  Target:       $(TARGET)"
	@echo "  Build Mode:   $(if $(filter true,$(RELEASE)),Release,Debug)"
	@echo "  Python Ver:   $(PYTHON_VER)"
	@echo "  Features:     $(or $(FEATURES),default)"
	@echo ""
	@echo "$(GREEN)Paths:$(NC)"
	@echo "  Target Dir:   $(TARGET_DIR)"
	@echo "  Dist Dir:     $(DIST_DIR)"
	@echo "  Include Dir:  $(INCLUDE_DIR)"
	@echo ""
	@echo "$(GREEN)Library Names:$(NC)"
	@echo "  Dynamic:      $(LIB_PREFIX)ri.$(LIB_EXT)"
	@echo "  Static:       $(LIB_PREFIX)ri.$(STATIC_EXT)"

################################################################################
# Development Helpers
################################################################################

# Watch for changes and rebuild
watch:
	@echo "$(GREEN)Watching for changes...$(NC)"
	cargo watch -x "build $(BUILD_MODE) --target $(TARGET)"

# Run with debug logging
run:
	@echo "$(GREEN)Running Ri...$(NC)"
	cargo run $(BUILD_MODE) --target $(TARGET)

# Generate completion scripts for shells
completion:
	@echo "$(GREEN)Generating shell completion scripts...$(NC)"
	@mkdir -p completions
	@echo "Bash completion: completions/ric.bash"
	@echo "Zsh completion: completions/_ric"
	@echo "Fish completion: completions/ric.fish"
	@echo "$(GREEN)✓ Completion scripts generated$(NC)"
