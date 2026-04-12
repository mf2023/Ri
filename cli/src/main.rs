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

//! Ri CLI (ric) - Binary Entry Point
//!
//! This is the main entry point for the Ri CLI tool. It provides a command-line
//! interface for managing Ri framework projects, including project creation,
//! building, running, and configuration management.
//!
//! # Architecture
//!
//! The CLI follows a command pattern where:
//! 1. Command-line arguments are parsed using `clap`
//! 2. Commands are dispatched to their respective handlers
//! 3. Each handler executes the command logic asynchronously
//!
//! # Commands
//!
//! - `new` - Create a new Ri project with specified template
//! - `build` - Build the project for specified target
//! - `run` - Run the project in development or release mode
//! - `config` - Manage project configuration
//! - `check` - Check project for errors
//! - `clean` - Clean build artifacts
//! - `info` - Display project information
//! - `version` - Show version information
//!
//! # Usage
//!
//! ```bash
//! # Create a new project
//! ric new my-project
//!
//! # Build the project
//! ric build --release
//!
//! # Run the project
//! ric run
//!
//! # Show help
//! ric --help
//! ```

use clap::Parser;
use colored::Colorize;
use ric::cli::{Cli, Commands};
use std::path::PathBuf;

/// Main entry point for the Ri CLI application
///
/// This function serves as the primary entry point for the CLI tool. It:
/// 1. Parses command-line arguments using clap
/// 2. Dispatches the command to the appropriate handler
/// 3. Executes the command asynchronously using tokio runtime
/// 4. Handles errors and provides user-friendly error messages
///
/// # Async Runtime
///
/// The function uses `#[tokio::main]` to set up an asynchronous runtime,
/// allowing all command handlers to perform async operations such as:
/// - File I/O operations
/// - Network requests
/// - Concurrent task execution
///
/// # Error Handling
///
/// All errors are propagated using `anyhow::Result`, which provides:
/// - Automatic error conversion
/// - Rich error context
/// - Stack trace information in debug mode
///
/// # Exit Codes
///
/// - `0` - Success
/// - `1` - Error (displayed to user with colored output)
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Parse command-line arguments using clap's derive macro
    // This automatically handles --help, --version, and argument validation
    let cli = Cli::parse();

    // Dispatch command to appropriate handler
    // Each command has its own handler function in the commands module
    match cli.command {
        // Create a new Ri project with specified name, template, and optional path
        // Templates: minimal, web, api, worker, microservice
        Some(Commands::New { name, template, path }) => {
            // Convert template Option<String> to String with default "minimal"
            let template_name = template.unwrap_or_else(|| "minimal".to_string());
            
            // Convert path Option<String> to Option<PathBuf>
            let project_path = path.map(PathBuf::from);
            
            // Call new_project with owned values (no async)
            ric::commands::new_project(name, template_name, project_path)?;
        }
        
        // Build the project with optional release mode and target specification
        // Targets: python, java, c, all
        Some(Commands::Build { release, target }) => {
            ric::commands::build_project(release, target.as_deref()).await?;
        }
        
        // Run the project in development or release mode with optional config
        // Uses cargo run internally
        Some(Commands::Run { release, config }) => {
            ric::commands::run_project(release, config.as_deref()).await?;
        }
        
        // Handle configuration management commands
        // Subcommands: init, show, validate, set, get
        Some(Commands::Config { action }) => {
            ric::commands::handle_config(action).await?;
        }
        
        // Display version information for ric and ri framework
        Some(Commands::Version) => {
            println!(
                "{} {}",
                "ric".green().bold(),
                env!("CARGO_PKG_VERSION").cyan()
            );
            println!("{} {}", "ri".green().bold(), "0.1.9".cyan());
        }
        
        // Check the project for compilation errors
        // Uses cargo check internally
        Some(Commands::Check) => {
            ric::commands::check_project().await?;
        }
        
        // Clean build artifacts from the project
        // Uses cargo clean internally
        Some(Commands::Clean) => {
            ric::commands::clean_project().await?;
        }
        
        // Display comprehensive project information
        // Shows environment, project details, and available features
        Some(Commands::Info) => {
            ric::commands::show_info().await?;
        }
        
        // Test connections to external services
        // Supports: Redis, PostgreSQL, MySQL, Kafka
        Some(Commands::Test { action }) => {
            ric::commands::test_connection(action).await?;
        }
        
        // Run comprehensive diagnostic checks on the development environment
        // Checks Rust toolchain, tools, environment, ports, dependencies, and file system
        Some(Commands::Doctor { verbose, fix }) => {
            ric::commands::doctor(verbose, fix).await?;
        }
        
        // Generate code artifacts (modules, middleware, config structs)
        // Supports: module, middleware, config subcommands
        Some(Commands::Generate { action }) => {
            ric::commands::handle_generate(action).await?;
        }
        
        // No command specified - display welcome message
        // Guides user to use --help for available commands
        None => {
            println!("{}", "Welcome to Ri CLI (ric)".green().bold());
            println!("Use {} to see available commands", "ric --help".cyan());
        }
    }

    Ok(())
}
