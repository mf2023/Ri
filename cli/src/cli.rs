// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Command-Line Interface Definitions
//!
//! This module defines the CLI structure using clap's derive macros. It provides
//! a declarative way to define command-line arguments, subcommands, and help text.
//!
//! # Architecture
//!
//! The CLI follows a hierarchical structure:
//! 1. `Cli` - Top-level structure that contains optional subcommands
//! 2. `Commands` - Enum of all available top-level commands
//! 3. `ConfigAction` - Enum of configuration subcommands
//! 4. `GenerateAction` - Enum of code generation subcommands
//!
//! # Features
//!
//! - Automatic help generation with colored output
//! - Version information from Cargo.toml
//! - Argument validation and type conversion
//! - Shell completion support (via clap)
//! - Code generation for modules, middleware, and config structs
//!
//! # Example
//!
//! ```bash
//! # Show help
//! ric --help
//!
//! # Create a new project
//! ric new my-project --template gateway
//!
//! # Build in release mode
//! ric build --release --target python
//!
//! # Manage configuration
//! ric config set project.name "my-app"
//!
//! # Generate code
//! ric generate module cache my-cache
//! ric generate middleware auth-middleware
//! ric generate config config.yaml
//! ```

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Top-level CLI structure for the Ri CLI tool
///
/// This structure represents the main entry point for command-line argument parsing.
/// It uses clap's derive macro to automatically generate:
/// - Help text (--help)
/// - Version information (--version)
/// - Argument parsing and validation
///
/// # Fields
///
/// - `command` - Optional subcommand to execute. If no command is provided,
///   a welcome message is displayed.
///
/// # Examples
///
/// ```rust,ignore
/// // Parse command-line arguments
/// let cli = Cli::parse();
///
/// // Match on the command
/// match cli.command {
///     Some(Commands::New { name, template }) => { /* ... */ }
///     Some(Commands::Build { release, target }) => { /* ... */ }
///     None => { /* Show welcome message */ }
/// }
/// ```
#[derive(Parser, Debug)]
#[command(
    name = "ric",
    author = "Dunimd Team",
    version,
    long_version = concat!(
        env!("CARGO_PKG_VERSION"), "\n",
        "ri  0.1.9"
    ),
    about = "Ri CLI - Command-line interface tool for Ri framework",
    long_about = "Ri CLI (ric) is a powerful command-line tool for managing Ri projects.\n\nIt provides commands for creating, building, running, and managing Ri applications."
)]
pub struct Cli {
    /// Optional subcommand to execute
    ///
    /// If no subcommand is provided, the CLI displays a welcome message
    /// and guides the user to use --help for available commands.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Enum of all available CLI commands
///
/// Each variant represents a top-level command with its specific arguments.
/// Commands are implemented as subcommands in clap, allowing for:
/// - Hierarchical command structure
/// - Command-specific help text
/// - Per-command argument validation
///
/// # Command Categories
///
/// - **Project Management**: `new`, `build`, `run`, `check`, `clean`
/// - **Configuration**: `config`
/// - **Connection Testing**: `test`
/// - **Information**: `info`, `version`
///
/// # Examples
///
/// ```bash
/// # Project management
/// ric new my-project
/// ric build --release
/// ric run
///
/// # Configuration
/// ric config init
/// ric config show
///
/// # Connection testing
/// ric test redis redis://localhost:6379
/// ric test postgres postgresql://user:pass@localhost:5432/db
///
/// # Information
/// ric info
/// ric version
/// ```
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a new Ri project
    ///
    /// This command creates a new Ri project with the specified name and template.
    /// It generates the complete project structure including:
    /// - Cargo.toml with Ri dependencies
    /// - src/main.rs with template-specific code
    /// - config/config.yaml with default settings
    ///
    /// # Arguments
    ///
    /// - `name` - Project name (required). Used as the directory name and package name.
    /// - `template` - Project template (optional). Defaults to "default".
    /// - `path` - Custom path for project creation (optional). Defaults to current directory.
    ///
    /// # Templates
    ///
    /// - `default` - Basic Ri application with minimal setup
    /// - `gateway` - API Gateway with routing and middleware
    /// - `microservice` - gRPC microservice with service definitions
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Create default project
    /// ric new my-project
    ///
    /// # Create gateway project
    /// ric new my-gateway --template gateway
    ///
    /// # Create microservice
    /// ric new my-service --template microservice
    ///
    /// # Create project at custom path
    /// ric new my-project --path /path/to/projects
    /// ```
    New {
        /// Project name
        ///
        /// The name will be used for:
        /// - Directory name
        /// - Cargo.toml package name
        /// - Default application name in config
        #[arg(help = "Project name")]
        name: String,

        /// Project template
        ///
        /// Specifies the template to use for project generation.
        /// Available templates: default, gateway, microservice
        #[arg(short, long, help = "Project template (default, gateway, microservice)")]
        template: Option<String>,

        /// Custom project path
        ///
        /// Specifies the directory where the project will be created.
        /// If not specified, the project is created in the current directory.
        #[arg(short, long, help = "Custom path for project creation")]
        path: Option<String>,
    },

    /// Build the Ri project
    ///
    /// This command builds the project using cargo, with optional support for:
    /// - Release mode optimization
    /// - Cross-compilation to different targets (Python, Java, C)
    ///
    /// # Build Modes
    ///
    /// - **Debug mode** (default): Fast compilation, includes debug symbols
    /// - **Release mode** (--release): Optimized binary, slower compilation
    ///
    /// # Targets
    ///
    /// - `all` - Build all targets (default)
    /// - `python` - Build Python bindings
    /// - `java` - Build Java bindings
    /// - `c` - Build C/C++ bindings
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Build in debug mode
    /// ric build
    ///
    /// # Build in release mode
    /// ric build --release
    ///
    /// # Build Python bindings
    /// ric build --target python
    /// ```
    Build {
        /// Build in release mode
        ///
        /// Enables compiler optimizations for production deployment.
        /// Results in smaller binary size and better performance.
        #[arg(short, long, help = "Build in release mode")]
        release: bool,

        /// Build target
        ///
        /// Specifies the target platform or binding type.
        /// Enables cross-compilation and binding generation.
        #[arg(short, long, help = "Build target (python, java, c, all)")]
        target: Option<String>,
    },

    /// Run the Ri project
    ///
    /// This command runs the project using cargo run, with support for:
    /// - Debug mode (default)
    /// - Release mode (--release)
    /// - Custom configuration file (--config)
    ///
    /// The command compiles the project if needed and executes the binary.
    ///
    /// # Arguments
    ///
    /// - `release` - Run in release mode (optimized binary)
    /// - `config` - Path to custom configuration file
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Run in debug mode
    /// ric run
    ///
    /// # Run in release mode
    /// ric run --release
    ///
    /// # Run with custom configuration
    /// ric run --config /path/to/config.yaml
    /// ```
    Run {
        /// Run in release mode
        ///
        /// Uses the optimized release binary for better performance.
        #[arg(short, long, help = "Run in release mode")]
        release: bool,

        /// Configuration file path
        ///
        /// Specifies a custom configuration file to use.
        /// If not specified, uses the default ric.yaml in the current directory.
        #[arg(short, long, help = "Path to configuration file")]
        config: Option<String>,
    },

    /// Manage configuration
    ///
    /// This command provides subcommands for managing the project configuration
    /// file (ric.yaml). Configuration includes:
    /// - Project metadata (name, version, template)
    /// - Build settings (release mode, target, features)
    /// - Runtime settings (log level, workers)
    ///
    /// # Subcommands
    ///
    /// - `init` - Initialize a new configuration file
    /// - `show` - Display current configuration
    /// - `validate` - Validate configuration file
    /// - `set` - Set a configuration value
    /// - `get` - Get a configuration value
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Initialize configuration
    /// ric config init
    ///
    /// # Show configuration
    /// ric config show
    ///
    /// # Set a value
    /// ric config set runtime.workers 8
    ///
    /// # Get a value
    /// ric config get project.name
    /// ```
    Config {
        /// Configuration subcommand to execute
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Show version information
    ///
    /// Displays version information for:
    /// - ric (CLI tool version)
    /// - ri (Framework version)
    ///
    /// # Example
    ///
    /// ```bash
    /// ric version
    /// # Output:
    /// # ric 0.1.0
    /// # ri 0.1.9
    /// ```
    Version,

    /// Check the project for errors
    ///
    /// This command runs cargo check to verify the project compiles without
    /// producing an executable. It's faster than a full build and useful for:
    /// - Quick error detection
    /// - IDE integration
    /// - CI/CD pipelines
    ///
    /// # Example
    ///
    /// ```bash
    /// ric check
    /// ```
    Check,

    /// Clean build artifacts
    ///
    /// This command removes all build artifacts from the target directory,
    /// including:
    /// - Compiled binaries
    /// - Intermediate object files
    /// - Dependency artifacts
    ///
    /// Useful for:
    /// - Freeing disk space
    /// - Resolving build issues
    /// - Starting a fresh build
    ///
    /// # Example
    ///
    /// ```bash
    /// ric clean
    /// ```
    Clean,

    /// Show project information
    ///
    /// Displays comprehensive information about the current project and environment:
    /// - Ri framework version
    /// - CLI tool version
    /// - Rust version
    /// - Project metadata (if Cargo.toml exists)
    /// - Available features
    ///
    /// # Example
    ///
    /// ```bash
    /// ric info
    /// ```
    Info,

    /// Test connections to external services
    ///
    /// This command tests connectivity to various external services including:
    /// - Redis: In-memory data structure store
    /// - PostgreSQL: Object-relational database
    /// - MySQL: Relational database management system
    /// - Kafka: Distributed event streaming platform
    ///
    /// Each test performs:
    /// - Connection establishment
    /// - Server version retrieval
    /// - Response time measurement
    /// - Basic operation verification
    ///
    /// # Subcommands
    ///
    /// - `redis` - Test Redis connection
    /// - `postgres` - Test PostgreSQL connection
    /// - `mysql` - Test MySQL connection
    /// - `kafka` - Test Kafka connection
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Test Redis connection
    /// ric test redis redis://localhost:6379
    ///
    /// # Test Redis with authentication
    /// ric test redis redis://:password@localhost:6379
    ///
    /// # Test PostgreSQL connection
    /// ric test postgres postgresql://user:password@localhost:5432/database
    ///
    /// # Test MySQL connection
    /// ric test mysql mysql://user:password@localhost:3306/database
    ///
    /// # Test Kafka connection
    /// ric test kafka localhost:9092
    /// ```
    Test {
        /// Connection test subcommand to execute
        #[command(subcommand)]
        action: TestAction,
    },

    /// Diagnose development environment and project configuration
    ///
    /// This command performs comprehensive diagnostic checks on the development
    /// environment and project configuration. It helps identify and fix common
    /// issues that may prevent successful project builds or runs.
    ///
    /// # Diagnostic Categories
    ///
    /// - **Rust Toolchain**: rustc, cargo, rustup versions and targets
    /// - **Development Tools**: git, build tools, and other dependencies
    /// - **Environment Variables**: RUST_LOG, CARGO_HOME, RUSTUP_HOME, etc.
    /// - **Port Availability**: Common ports (8080, 8081, etc.) for development
    /// - **Dependencies**: Version conflicts, missing dependencies
    /// - **File System**: Write permissions, disk space
    ///
    /// # Output Format
    ///
    /// - ✅ Passed checks (green)
    /// - ⚠️ Warnings (yellow)
    /// - ❌ Errors (red)
    /// - 💡 Suggestions (blue)
    ///
    /// # Arguments
    ///
    /// - `verbose` - Show detailed diagnostic information
    /// - `fix` - Attempt to automatically fix detected issues
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Run basic diagnostics
    /// ric doctor
    ///
    /// # Run with verbose output
    /// ric doctor --verbose
    ///
    /// # Run and auto-fix issues
    /// ric doctor --fix
    ///
    /// # Run with both verbose and fix
    /// ric doctor --verbose --fix
    /// ```
    Doctor {
        /// Show detailed diagnostic information
        ///
        /// When enabled, displays additional details for each diagnostic check,
        /// including version numbers, paths, and configuration values.
        #[arg(short, long, help = "Show detailed diagnostic information")]
        verbose: bool,

        /// Attempt to automatically fix detected issues
        ///
        /// When enabled, the command will attempt to fix common issues such as:
        /// - Missing environment variables
        /// - Missing directories
        /// - Common configuration problems
        #[arg(short, long, help = "Attempt to auto-fix detected issues")]
        fix: bool,
    },

    /// Generate code artifacts
    ///
    /// This command provides subcommands for generating various code artifacts:
    /// - Modules: Generate Ri module scaffolding with complete structure
    /// - Middleware: Generate middleware template with standard patterns
    /// - Config: Generate Rust struct from YAML/JSON config file
    ///
    /// # Subcommands
    ///
    /// - `module` - Generate a new Ri module with complete structure
    /// - `middleware` - Generate middleware template
    /// - `config` - Generate Rust struct from YAML/JSON config file
    ///
    /// # Module Types
    ///
    /// Supported module types for the `module` subcommand:
    /// - `cache` - Caching module (Redis, Memcached support)
    /// - `queue` - Message queue module (RabbitMQ, Kafka support)
    /// - `gateway` - API Gateway module with routing
    /// - `auth` - Authentication/Authorization module
    /// - `device` - IoT device management module
    /// - `observability` - Monitoring and tracing module
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Generate a cache module
    /// ric generate module cache my-cache
    ///
    /// # Generate a queue module
    /// ric generate module queue my-queue
    ///
    /// # Generate middleware
    /// ric generate middleware auth-middleware
    ///
    /// # Generate config struct from YAML
    /// ric generate config config.yaml
    ///
    /// # Generate config struct from JSON
    /// ric generate config config.json
    /// ```
    Generate {
        /// Generation subcommand to execute
        #[command(subcommand)]
        action: GenerateAction,
    },
}

/// Enum of connection test subcommands
///
/// These subcommands provide connectivity testing for various external services.
/// Each subcommand accepts a connection URL and performs comprehensive testing
/// including connection establishment, version retrieval, and basic operations.
///
/// # Supported Services
///
/// - **Redis**: In-memory data structure store, used for caching, sessions, and pub/sub
/// - **PostgreSQL**: Advanced open-source relational database with ACID compliance
/// - **MySQL**: Popular relational database management system
/// - **Kafka**: Distributed event streaming platform for real-time data pipelines
///
/// # URL Formats
///
/// - Redis: `redis://[password@]host:port[/database]`
/// - PostgreSQL: `postgresql://user:password@host:port/database`
/// - MySQL: `mysql://user:password@host:port/database`
/// - Kafka: `host:port` or `host1:port1,host2:port2`
///
/// # Test Results
///
/// Each test provides:
/// - Connection status (success/failure)
/// - Server version and information
/// - Response time in milliseconds
/// - Troubleshooting suggestions on failure
///
/// # Examples
///
/// ```bash
/// # Test Redis with default settings
/// ric test redis redis://localhost:6379
///
/// # Test PostgreSQL with SSL
/// ric test postgres postgresql://user:pass@host:5432/db?sslmode=require
///
/// # Test MySQL with custom port
/// ric test mysql mysql://root:password@localhost:3307/mydb
///
/// # Test Kafka cluster
/// ric test kafka broker1:9092,broker2:9092,broker3:9092
/// ```
#[derive(Subcommand, Debug)]
pub enum TestAction {
    /// Test Redis connection
    ///
    /// Tests connectivity to a Redis server and performs basic operations.
    ///
    /// # Test Operations
    ///
    /// 1. Establish TCP connection to Redis server
    /// 2. Send PING command and verify PONG response
    /// 3. Retrieve Redis server version
    /// 4. Test SET/GET operations (optional, if database selected)
    /// 5. Measure round-trip time
    ///
    /// # URL Format
    ///
    /// ```text
    /// redis://[password@]host:port[/database]
    /// redis://localhost:6379
    /// redis://:mypassword@localhost:6379
    /// redis://:mypassword@localhost:6379/0
    /// ```
    ///
    /// # Example
    ///
    /// ```bash
    /// ric test redis redis://localhost:6379
    /// ```
    #[command(about = "Test Redis connection")]
    Redis {
        /// Redis connection URL
        ///
        /// Format: redis://[password@]host:port[/database]
        /// Example: redis://localhost:6379
        #[arg(help = "Redis connection URL (e.g., redis://localhost:6379)")]
        url: String,
    },

    /// Test PostgreSQL connection
    ///
    /// Tests connectivity to a PostgreSQL database server.
    ///
    /// # Test Operations
    ///
    /// 1. Establish TCP connection to PostgreSQL server
    /// 2. Authenticate with provided credentials
    /// 3. Retrieve PostgreSQL server version
    /// 4. Query database information (if database specified)
    /// 5. Measure connection time
    ///
    /// # URL Format
    ///
    /// ```text
    /// postgresql://user:password@host:port/database
    /// postgresql://postgres:password@localhost:5432/mydb
    /// postgresql://user:pass@host:5432/db?sslmode=require
    /// ```
    ///
    /// # Example
    ///
    /// ```bash
    /// ric test postgres postgresql://user:password@localhost:5432/database
    /// ```
    #[command(about = "Test PostgreSQL connection")]
    Postgres {
        /// PostgreSQL connection URL
        ///
        /// Format: postgresql://user:password@host:port/database
        /// Example: postgresql://postgres:password@localhost:5432/mydb
        #[arg(help = "PostgreSQL connection URL (e.g., postgresql://user:pass@localhost:5432/db)")]
        url: String,
    },

    /// Test MySQL connection
    ///
    /// Tests connectivity to a MySQL database server.
    ///
    /// # Test Operations
    ///
    /// 1. Establish TCP connection to MySQL server
    /// 2. Authenticate with provided credentials
    /// 3. Retrieve MySQL server version
    /// 4. Query database information (if database specified)
    /// 5. Measure connection time
    ///
    /// # URL Format
    ///
    /// ```text
    /// mysql://user:password@host:port/database
    /// mysql://root:password@localhost:3306/mydb
    /// mysql://user:pass@host:3306/db?charset=utf8mb4
    /// ```
    ///
    /// # Example
    ///
    /// ```bash
    /// ric test mysql mysql://user:password@localhost:3306/database
    /// ```
    #[command(about = "Test MySQL connection")]
    Mysql {
        /// MySQL connection URL
        ///
        /// Format: mysql://user:password@host:port/database
        /// Example: mysql://root:password@localhost:3306/mydb
        #[arg(help = "MySQL connection URL (e.g., mysql://user:pass@localhost:3306/db)")]
        url: String,
    },

    /// Test Kafka connection
    ///
    /// Tests connectivity to a Kafka broker cluster.
    ///
    /// # Test Operations
    ///
    /// 1. Establish connection to Kafka broker(s)
    /// 2. Retrieve broker metadata and cluster information
    /// 3. List available topics
    /// 4. Measure connection time
    ///
    /// # URL Format
    ///
    /// ```text
    /// host:port
    /// localhost:9092
    /// broker1:9092,broker2:9092,broker3:9092
    /// ```
    ///
    /// # Example
    ///
    /// ```bash
    /// ric test kafka localhost:9092
    /// ```
    #[command(about = "Test Kafka connection")]
    Kafka {
        /// Kafka broker URL
        ///
        /// Format: host:port or host1:port1,host2:port2
        /// Example: localhost:9092
        #[arg(help = "Kafka broker URL (e.g., localhost:9092)")]
        url: String,
    },
}

/// Enum of configuration management subcommands
///
/// These subcommands provide fine-grained control over the project configuration
/// file (ric.yaml). Each subcommand performs a specific configuration operation.
///
/// # Configuration Structure
///
/// The configuration file has the following structure:
/// ```yaml
/// project:
///   name: my-project
///   version: 0.1.0
///   template: default
///
/// build:
///   release: false
///   target: all
///   features:
///     - default
///
/// runtime:
///   log_level: info
///   workers: 4
/// ```
///
/// # Key Format
///
/// Configuration keys use dot notation to access nested values:
/// - `project.name` - Project name
/// - `build.release` - Release mode flag
/// - `runtime.workers` - Number of worker threads
#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    /// Initialize a new configuration file
    ///
    /// Creates a new ric.yaml file with default values in the current directory.
    /// If a configuration file already exists, it will be overwritten.
    ///
    /// # Example
    ///
    /// ```bash
    /// ric config init
    /// ```
    #[command(about = "Initialize a new configuration file")]
    Init,

    /// Show current configuration
    ///
    /// Displays the current configuration in YAML format.
    /// If no configuration file exists, displays default values.
    ///
    /// # Example
    ///
    /// ```bash
    /// ric config show
    /// ```
    #[command(about = "Show current configuration")]
    Show,

    /// Validate configuration file
    ///
    /// Validates the specified configuration file for:
    /// - Required fields presence
    /// - Value type correctness
    /// - Value range constraints
    /// - Syntax validity
    ///
    /// Returns success if valid, error with details if invalid.
    /// Provides detailed error messages and fix suggestions.
    ///
    /// # Arguments
    ///
    /// - `file` - Path to configuration file to validate (optional)
    ///            Defaults to ric.yaml in current directory
    ///
    /// # Output Format
    ///
    /// - ✅ Valid configuration
    /// - ❌ Invalid configuration with detailed errors
    /// - ⚠️ Warnings for non-critical issues
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Validate default configuration file
    /// ric config validate
    ///
    /// # Validate specific configuration file
    /// ric config validate path/to/config.yaml
    /// ```
    #[command(about = "Validate configuration file")]
    Validate {
        /// Path to configuration file
        ///
        /// Specifies the configuration file to validate.
        /// If not provided, defaults to ric.yaml in the current directory.
        #[arg(help = "Path to configuration file (default: ric.yaml)")]
        file: Option<PathBuf>,
    },

    /// Check environment variables
    ///
    /// Checks the environment for required and optional environment variables
    /// that affect Ri project behavior. This includes:
    /// - RI_CONFIG_PATH: Custom configuration file path
    /// - RI_LOG_LEVEL: Override log level
    /// - RUST_LOG: Rust logging configuration
    /// - CARGO_HOME: Cargo home directory
    /// - RUSTUP_HOME: Rustup home directory
    ///
    /// Displays which variables are set and their current values.
    /// Provides warnings for missing optional variables and errors
    /// for missing required variables.
    ///
    /// # Example
    ///
    /// ```bash
    /// ric config check
    /// ```
    #[command(about = "Check environment variables")]
    Check,

    /// Set a configuration value
    ///
    /// Updates a configuration value in the ric.yaml file.
    /// The key uses dot notation to access nested values.
    ///
    /// # Arguments
    ///
    /// - `key` - Configuration key (e.g., 'database.host')
    /// - `value` - New value to set
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Set project name
    /// ric config set project.name my-new-project
    ///
    /// # Set worker count
    /// ric config set runtime.workers 8
    ///
    /// # Enable release mode
    /// ric config set build.release true
    /// ```
    #[command(about = "Set a configuration value")]
    Set {
        /// Configuration key
        ///
        /// Uses dot notation to access nested configuration values.
        /// Example: 'runtime.workers' accesses the 'workers' field
        /// under the 'runtime' section.
        #[arg(help = "Configuration key (e.g., 'database.host')")]
        key: String,

        /// Configuration value
        ///
        /// The value is automatically converted to the appropriate type
        /// based on the key. Boolean values accept "true"/"false",
        /// numeric values accept integer strings, etc.
        #[arg(help = "Configuration value")]
        value: String,
    },

    /// Get a configuration value
    ///
    /// Retrieves and displays a configuration value from the ric.yaml file.
    ///
    /// # Arguments
    ///
    /// - `key` - Configuration key to retrieve
    ///
    /// # Example
    ///
    /// ```bash
    /// # Get project name
    /// ric config get project.name
    /// # Output: project.name: my-project
    /// ```
    #[command(about = "Get a configuration value")]
    Get {
        /// Configuration key to retrieve
        ///
        /// Uses dot notation to access nested configuration values.
        #[arg(help = "Configuration key")]
        key: String,
    },
}

/// Enum of code generation subcommands
///
/// These subcommands provide code generation capabilities for Ri projects.
/// Each subcommand generates specific types of code artifacts with proper
/// structure, dependencies, and formatting.
///
/// # Available Generators
///
/// - `module` - Generate complete Ri module scaffolding
/// - `middleware` - Generate middleware template
/// - `config` - Generate Rust struct from config file
///
/// # Module Types
///
/// The `module` subcommand supports the following module types:
///
/// | Type | Description | Dependencies |
/// |------|-------------|--------------|
/// | `cache` | Caching module | redis, memcached |
/// | `queue` | Message queue module | lapin (RabbitMQ), rdkafka |
/// | `gateway` | API Gateway module | hyper, tower |
/// | `auth` | Auth module | jsonwebtoken, oauth2 |
/// | `device` | IoT device module | mqtt, coap |
/// | `observability` | Monitoring module | tracing, metrics |
///
/// # Examples
///
/// ```bash
/// # Generate a cache module named "my-cache"
/// ric generate module cache my-cache
///
/// # Generate a queue module named "my-queue"
/// ric generate module queue my-queue
///
/// # Generate middleware named "auth-middleware"
/// ric generate middleware auth-middleware
///
/// # Generate Rust struct from config.yaml
/// ric generate config config.yaml
/// ```
#[derive(Subcommand, Debug)]
pub enum GenerateAction {
    /// Generate a new Ri module
    ///
    /// Creates a complete module structure with:
    /// - Module source files (lib.rs, mod.rs)
    /// - Configuration templates
    /// - Test scaffolding
    /// - Documentation stubs
    /// - Cargo.toml dependency updates
    ///
    /// # Arguments
    ///
    /// - `module_type` - Type of module to generate
    /// - `name` - Name of the module (used for directory and struct names)
    ///
    /// # Module Types
    ///
    /// - `cache` - Caching module with Redis/Memcached support
    /// - `queue` - Message queue module with RabbitMQ/Kafka support
    /// - `gateway` - API Gateway module with routing and middleware
    /// - `auth` - Authentication/Authorization module
    /// - `device` - IoT device management module
    /// - `observability` - Monitoring and tracing module
    ///
    /// # Generated Structure
    ///
    /// ```text
    /// src/modules/<name>/
    /// ├── mod.rs           # Module entry point
    /// ├── config.rs        # Configuration structures
    /// ├── handler.rs       # Request handlers
    /// ├── service.rs       # Business logic
    /// └── tests/
    ///     └── mod.rs       # Test module
    /// ```
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Generate a cache module
    /// ric generate module cache my-cache
    ///
    /// # Generate a queue module
    /// ric generate module queue message-queue
    ///
    /// # Generate an auth module
    /// ric generate module auth user-auth
    /// ```
    #[command(about = "Generate a new Ri module")]
    Module {
        /// Module type to generate
        ///
        /// Specifies the type of module to generate. Each type includes
        /// type-specific dependencies, configurations, and boilerplate code.
        ///
        /// Available types: cache, queue, gateway, auth, device, observability
        #[arg(help = "Module type (cache, queue, gateway, auth, device, observability)")]
        module_type: String,

        /// Module name
        ///
        /// The name will be used for:
        /// - Directory name in src/modules/
        /// - Module struct name (PascalCase)
        /// - Configuration section name (snake_case)
        ///
        /// Must follow Rust naming conventions: lowercase with hyphens/underscores.
        #[arg(help = "Module name")]
        name: String,
    },

    /// Generate middleware template
    ///
    /// Creates a middleware template with:
    /// - Middleware struct definition
    /// - Standard middleware trait implementation
    /// - Request/response handling boilerplate
    /// - Configuration support
    /// - Test scaffolding
    ///
    /// # Arguments
    ///
    /// - `name` - Name of the middleware (used for struct name)
    ///
    /// # Generated Structure
    ///
    /// ```text
    /// src/middleware/
    /// └── <name>.rs        # Middleware implementation
    /// ```
    ///
    /// # Generated Code
    ///
    /// The generated middleware includes:
    /// - Async middleware function signature
    /// - Request preprocessing hook
    /// - Response postprocessing hook
    /// - Error handling
    /// - Configuration integration
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Generate authentication middleware
    /// ric generate middleware auth
    ///
    /// # Generate logging middleware
    /// ric generate middleware request-logger
    ///
    /// # Generate rate limiting middleware
    /// ric generate middleware rate-limiter
    /// ```
    #[command(about = "Generate middleware template")]
    Middleware {
        /// Middleware name
        ///
        /// The name will be used for:
        /// - File name in src/middleware/
        /// - Middleware struct name (PascalCase)
        ///
        /// Must follow Rust naming conventions: lowercase with hyphens/underscores.
        #[arg(help = "Middleware name")]
        name: String,
    },

    /// Generate Rust struct from config file
    ///
    /// Parses a YAML or JSON configuration file and generates corresponding
    /// Rust struct definitions with:
    /// - Serde derives for serialization
    /// - Default implementation
    /// - Builder pattern support (optional)
    /// - Validation attributes (optional)
    ///
    /// # Arguments
    ///
    /// - `from` - Path to the configuration file (YAML or JSON)
    ///
    /// # Supported Formats
    ///
    /// - YAML (.yaml, .yml)
    /// - JSON (.json)
    ///
    /// # Generated Output
    ///
    /// The generated code includes:
    /// - Root configuration struct
    /// - Nested struct definitions
    /// - Serde attributes for field renaming
    /// - Default implementations
    /// - Optional validation attributes
    ///
    /// # Output Location
    ///
    /// Generated structs are written to stdout by default.
    /// Use shell redirection to save to a file:
    ///
    /// ```bash
    /// ric generate config config.yaml > src/config.rs
    /// ```
    ///
    /// # Examples
    ///
    /// ```bash
    /// # Generate from YAML config
    /// ric generate config config.yaml
    ///
    /// # Generate from JSON config
    /// ric generate config config.json
    ///
    /// # Save to file
    /// ric generate config app.yaml > src/config/app_config.rs
    /// ```
    #[command(about = "Generate Rust struct from config file")]
    Config {
        /// Path to configuration file
        ///
        /// The configuration file to parse. Supports YAML (.yaml, .yml)
        /// and JSON (.json) formats. The format is detected from the file extension.
        #[arg(help = "Path to configuration file (YAML or JSON)")]
        from: PathBuf,
    },
}
