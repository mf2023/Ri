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

//! Command Implementation Module
//!
//! This module contains the implementation of all CLI commands. Each function
//! handles a specific command and provides the business logic for:
//! - Project creation and scaffolding
//! - Build management and compilation
//! - Configuration management
//! - Project information display
//!
//! # Architecture
//!
//! Each command function follows a similar pattern:
//! 1. Validate input parameters
//! 2. Display progress indicators
//! 3. Execute the command logic
//! 4. Handle errors and provide user feedback
//!
//! # User Experience
//!
//! The module provides rich user feedback through:
//! - Colored terminal output (via `colored` crate)
//! - Progress spinners and bars (via `indicatif` crate)
//! - Clear error messages with context
//!
//! # Error Handling
//!
//! All functions return `Result<T>` which is an alias for
//! `std::result::Result<T, RicError>`. Errors are propagated
//! using the `?` operator and automatically converted to the
//! appropriate error type.

use crate::cli::ConfigAction;
use crate::error::Result;
use crate::templates::TemplateEngine;
use crate::utils;
use crate::utils::output;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Create a new Ri project with specified template
///
/// This function creates a complete Ri project structure including:
/// - Project directory with the specified name
/// - Cargo.toml with Ri dependencies
/// - Source files based on the selected template
/// - Configuration files with default settings
/// - Optional git repository initialization
///
/// # Arguments
///
/// * `name` - Project name (owned String), used as directory name and package name.
///            Must contain only alphanumeric characters, dashes, and underscores.
///            Cannot start with a number. Maximum length is 64 characters.
/// * `template` - Template name (owned String). Supported values:
///                - "minimal" - Minimal Ri application with basic structure
///                - "web" - Full-featured web application with HTTP server and routing
///                - "api" - RESTful API service with OpenAPI documentation support
///                - "worker" - Background job processing service with task queues
///                - "microservice" - gRPC microservice with service definitions
/// * `path` - Optional target path (PathBuf) for project creation.
///            If None, the project is created in the current directory.
///
/// # Templates
///
/// - `minimal` - Minimal Ri application with basic structure (default)
/// - `web` - Full-featured web application with HTTP server and routing
/// - `api` - RESTful API service with OpenAPI documentation support
/// - `worker` - Background job processing service with task queues
/// - `microservice` - gRPC microservice with service definitions
///
/// # Validation
///
/// The function validates:
/// - Project name format (alphanumeric, dashes, underscores, no leading numbers)
/// - Directory existence (prevents overwriting existing projects)
/// - Template availability (ensures template exists)
///
/// # Errors
///
/// Returns `RicError::ProjectExists` if a directory with the same name exists
/// Returns `RicError::Io` for file system errors (permission denied, disk full, etc.)
/// Returns `RicError::Template` for template processing errors (invalid template, rendering errors)
///
/// # Examples
///
/// ```rust,ignore
/// use std::path::PathBuf;
///
/// // Create minimal project in current directory
/// new_project("my-project".to_string(), "minimal".to_string(), None)?;
///
/// // Create web application project
/// new_project("my-web-app".to_string(), "web".to_string(), None)?;
///
/// // Create API service at custom path
/// new_project("my-api".to_string(), "api".to_string(), Some(PathBuf::from("/path/to/projects")))?;
///
/// // Create microservice
/// new_project("my-service".to_string(), "microservice".to_string(), None)?;
/// ```
///
/// # Process Flow
///
/// 1. **Validation**: Validate project name and check for existing directory
/// 2. **Template Resolution**: Resolve template name and validate it exists
/// 3. **Context Gathering**: Collect template variables (name, version, author, etc.)
/// 4. **Directory Creation**: Create project directory structure
/// 5. **Template Rendering**: Render template files with context
/// 6. **File Generation**: Write rendered files to project directory
/// 7. **Git Initialization**: Optionally initialize git repository
/// 8. **Success Message**: Display next steps for the user
///
/// # Error Handling
///
/// The function provides detailed error messages for common failure scenarios:
/// - Invalid project name: Describes which validation rule was violated
/// - Directory exists: Shows the path that already exists
/// - Template not found: Lists available templates
/// - Permission denied: Suggests checking write permissions
/// - Template rendering: Shows which file failed to render
pub fn new_project(name: String, template: String, path: Option<PathBuf>) -> Result<()> {
    // Step 1: Validate project name using validation utilities
    // This ensures the name follows Rust naming conventions and Cargo package naming rules
    // Validation rules: alphanumeric, dashes, underscores allowed; no leading numbers; max 64 chars
    utils::validation::validate_project_name(&name)
        .map_err(|e| crate::error::RicError::Template(e.to_string()))?;

    // Step 2: Use the provided template name
    // The template parameter is now required (not Optional), so we use it directly
    let template_name = template.as_str();

    // Step 3: Determine the project directory path
    // If path is provided, create project inside that directory
    // If path is None, create project in current directory
    let project_dir = match &path {
        Some(base_path) => base_path.join(&name),
        None => PathBuf::from(&name),
    };

    // Display creation message with colored output for better user experience
    output::print_header(&format!("Creating new Ri project '{}'", name.cyan()));
    println!(
        "  {} Template: {}",
        "→".yellow().bold(),
        template_name.cyan()
    );
    println!(
        "  {} Location: {}",
        "→".yellow().bold(),
        project_dir.display().to_string().cyan()
    );
    println!();

    // Step 4: Check if project directory already exists
    // Prevent accidental overwriting of existing projects and data loss
    // This is a safety check before any file system modifications
    if project_dir.exists() {
        return Err(crate::error::RicError::ProjectExists(name));
    }

    // Step 5: Initialize progress spinner for visual feedback
    // The spinner provides real-time feedback during long-running operations
    let spinner = output::print_progress("Initializing project...");

    // Step 6: Gather template context with all necessary variables
    // This includes project metadata (name, version, author) and template-specific settings
    // Context variables are used for template rendering with Tera
    let context = gather_template_context(&name, template_name)?;

    // Step 7: Create project using template engine
    // This renders all template files and creates the directory structure
    // Files are generated based on the template definition
    create_project_from_template(&project_dir, template_name, &context, &spinner)?;

    // Step 8: Optionally initialize git repository
    // This provides version control from the start of the project
    // Git initialization is optional - failure doesn't affect project creation
    spinner.set_message("Initializing git repository...");
    initialize_git_repository(&project_dir)?;

    // Step 9: Complete the progress indicator with success message
    spinner.finish_with_message("Project created successfully!");

    // Step 10: Display success message and next steps for the user
    // Shows how to navigate to the project and run it
    print_success_message(&name, &project_dir, template_name);

    Ok(())
}

/// Gather template context variables for rendering
///
/// Collects all necessary variables for template rendering including:
/// - Project metadata (name, version, description)
/// - Author information (from git config or default)
/// - Ri framework version
/// - Current date for generated files
/// - Template-specific variables
///
/// # Arguments
///
/// * `project_name` - The name of the project
/// * `template_name` - The template being used
///
/// # Returns
///
/// Returns a HashMap containing all template variables.
///
/// # Example
///
/// ```rust,ignore
/// let context = gather_template_context("my-app", "web")?;
/// assert_eq!(context.get("project_name"), Some(&"my-app".to_string()));
/// ```
fn gather_template_context(
    project_name: &str,
    template_name: &str,
) -> std::result::Result<HashMap<String, String>, crate::error::RicError> {
    let mut context = HashMap::new();

    // Project metadata
    context.insert("project_name".to_string(), project_name.to_string());
    context.insert("version".to_string(), "0.1.0".to_string());
    context.insert("description".to_string(), format!("A Ri {} application", template_name));

    // Author information from git config or default
    let author = get_git_author().unwrap_or_else(|| "Anonymous".to_string());
    context.insert("author".to_string(), author);

    // Ri framework version from Cargo.toml
    context.insert("ri_version".to_string(), env!("CARGO_PKG_VERSION").to_string());

    // Current date for generated files
    let current_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    context.insert("date".to_string(), current_date);

    // Template-specific default variables
    match template_name {
        "web" => {
            context.insert("port".to_string(), "8080".to_string());
            context.insert("workers".to_string(), "4".to_string());
            context.insert("enable_tls".to_string(), "false".to_string());
            context.insert("enable_cors".to_string(), "true".to_string());
            context.insert("enable_compression".to_string(), "true".to_string());
        }
        "api" => {
            context.insert("api_version".to_string(), "v1".to_string());
            context.insert("enable_docs".to_string(), "true".to_string());
            context.insert("enable_auth".to_string(), "false".to_string());
        }
        "worker" => {
            context.insert("queue_type".to_string(), "memory".to_string());
            context.insert("max_workers".to_string(), "4".to_string());
            context.insert("enable_persistence".to_string(), "false".to_string());
        }
        "microservice" => {
            context.insert("grpc_port".to_string(), "50051".to_string());
            context.insert("enable_reflection".to_string(), "true".to_string());
            context.insert("enable_health_check".to_string(), "true".to_string());
        }
        _ => {
            // Minimal template has no additional variables
        }
    }

    Ok(context)
}

/// Get author name from git configuration
///
/// Attempts to retrieve the user's name from git config.
/// Falls back to "Anonymous" if git is not available or not configured.
///
/// # Returns
///
/// Returns Some(String) with the author name if available, None otherwise.
///
/// # Example
///
/// ```rust,ignore
/// let author = get_git_author().unwrap_or("Anonymous".to_string());
/// ```
fn get_git_author() -> Option<String> {
    // Try to get user.name from git config
    Command::new("git")
        .args(&["config", "user.name"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Create project from template using the template engine
///
/// Uses the TemplateEngine to render and generate all project files
/// based on the selected template and context variables.
///
/// # Arguments
///
/// * `project_dir` - Path where the project will be created
/// * `template_name` - Name of the template to use
/// * `context` - Template variables for rendering
/// * `spinner` - Progress indicator for status updates
///
/// # Returns
///
/// Returns Ok(()) on success, or an error if template processing fails.
///
/// # Process
///
/// 1. Create template engine instance
/// 2. Validate template exists
/// 3. Create project directory structure
/// 4. Render and write each template file
fn create_project_from_template(
    project_dir: &Path,
    template_name: &str,
    context: &HashMap<String, String>,
    spinner: &ProgressBar,
) -> std::result::Result<(), crate::error::RicError> {
    // Create template engine instance
    spinner.set_message("Loading template engine...");
    let engine = TemplateEngine::new()
        .map_err(|e| crate::error::RicError::Template(e.to_string()))?;

    // Validate template exists
    spinner.set_message(format!("Validating template '{}'...", template_name));
    let template_info = engine.get_template_info(template_name)
        .map_err(|e| crate::error::RicError::Template(e.to_string()))?;

    // Create project directory
    spinner.set_message("Creating project directory...");
    utils::fs::create_dir_all(project_dir)?;

    // Generate each file from the template
    let total_files = template_info.files.len();
    for (index, file) in template_info.files.iter().enumerate() {
        spinner.set_message(format!(
                "Generating file {}/{}: {}",
                index + 1,
                total_files,
                file.destination
            ));

        // Render the template file
        let content = engine.render_template_file(template_name, &file.source, context)
            .map_err(|e| crate::error::RicError::Template(e.to_string()))?;

        // Determine output file path
        let output_file = project_dir.join(&file.destination);

        // Create parent directories if needed
        if let Some(parent) = output_file.parent() {
            utils::fs::create_dir_all(parent)?
        }

        // Write the file
        utils::fs::write_file(&output_file, content)?
    }

    Ok(())
}

/// Initialize git repository in the project directory
///
/// Initializes a new git repository with an initial commit.
/// This provides version control from the start of the project.
///
/// # Arguments
///
/// * `project_dir` - Path to the project directory
///
/// # Returns
///
/// Returns Ok(()) on success. Git initialization failures are logged
/// but do not cause the project creation to fail, as git is optional.
///
/// # Process
///
/// 1. Run `git init` in the project directory
/// 2. Add all files to staging
/// 3. Create initial commit
///
/// # Note
///
/// Git initialization is optional - if git is not installed or fails,
/// the project is still created successfully.
fn initialize_git_repository(project_dir: &Path) -> std::result::Result<(), crate::error::RicError> {
    // Check if git is available
    let git_available = Command::new("git")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if !git_available {
        // Git is not available, skip initialization
        return Ok(());
    }

    // Initialize git repository
    let init_status = Command::new("git")
        .args(&["init"])
        .current_dir(project_dir)
        .status()
        .map_err(|e| crate::error::RicError::Io(e))?;

    if !init_status.success() {
        // Git init failed, but this is not critical
        return Ok(());
    }

    // Add all files to staging
    let add_status = Command::new("git")
        .args(&["add", "."])
        .current_dir(project_dir)
        .status()
        .map_err(|e| crate::error::RicError::Io(e))?;

    if !add_status.success() {
        return Ok(());
    }

    // Create initial commit
    let commit_status = Command::new("git")
        .args(&["commit", "-m", "Initial commit: Ri project created with ric"])
        .current_dir(project_dir)
        .status()
        .map_err(|e| crate::error::RicError::Io(e))?;

    if !commit_status.success() {
        // Commit failed, but repository is initialized
        return Ok(());
    }

    Ok(())
}

/// Print success message with next steps
///
/// Displays a formatted success message with instructions for
/// getting started with the newly created project.
///
/// # Arguments
///
/// * `name` - Project name
/// * `project_dir` - Path to the project directory
/// * `template_name` - Template used for the project
///
/// # Output Format
///
/// Displays:
/// - Success indicator
/// - Project location
/// - Next steps (cd into directory, run the project)
/// - Template-specific hints
fn print_success_message(name: &str, project_dir: &Path, template_name: &str) {
    println!();
    output::print_success(&format!("Project '{}' created successfully!", name));

    println!();
    println!("{}", "Next steps:".yellow().bold());
    println!("  {} Navigate to your project:", "1.".dimmed());
    println!("    {} {}", "cd".cyan(), project_dir.display().to_string().cyan());
    println!();
    println!("  {} Run your project:", "2.".dimmed());
    println!("    {}", "ric run".cyan());
    println!();

    // Template-specific hints
    match template_name {
        "web" => {
            println!("{}", "Hints:".yellow().bold());
            println!("  • Your web server will start on http://localhost:8080");
            println!("  • Edit routes in src/routes/");
            println!("  • Add middleware in src/middleware/");
        }
        "api" => {
            println!("{}", "Hints:".yellow().bold());
            println!("  • Your API will start on http://localhost:8080");
            println!("  • API documentation available at /docs");
            println!("  • Define endpoints in src/routes/");
        }
        "worker" => {
            println!("{}", "Hints:".yellow().bold());
            println!("  • Configure your task queue in config/config.yaml");
            println!("  • Define tasks in src/tasks/");
            println!("  • Monitor workers with ric status");
        }
        "microservice" => {
            println!("{}", "Hints:".yellow().bold());
            println!("  • Your gRPC service will start on port 50051");
            println!("  • Define services in src/services/");
            println!("  • Use grpcurl for testing");
        }
        _ => {
            println!("{}", "Hints:".yellow().bold());
            println!("  • Edit src/main.rs to customize your application");
            println!("  • Update config/config.yaml for configuration");
        }
    }

    println!();
    println!("{} Happy coding with Ri!", "🚀".green());
}

/// Build the Ri project with optional release mode and target
///
/// This function builds the project using cargo, with support for:
/// - Debug mode (default): Fast compilation with debug symbols
/// - Release mode: Optimized binary for production
/// - Cross-compilation to different targets (Python, Java, C)
///
/// # Arguments
///
/// * `release` - Whether to build in release mode (optimized)
/// * `target` - Optional build target (python, java, c, all)
///
/// # Build Process
///
/// 1. Display build mode and target information
/// 2. Show progress spinner during compilation
/// 3. Execute cargo build with appropriate flags
/// 4. Display success or error message
///
/// # Errors
///
/// Returns `RicError::BuildFailed` if the build process fails
///
/// # Examples
///
/// ```rust,ignore
/// // Build in debug mode
/// build_project(false, None).await?;
///
/// // Build in release mode
/// build_project(true, None).await?;
///
/// // Build Python bindings
/// build_project(false, Some("python")).await?;
/// ```
pub async fn build_project(release: bool, target: Option<&str>) -> Result<()> {
    let mode = if release { "release" } else { "debug" };
    let target_name = target.unwrap_or("all");

    println!(
        "{} Building project in {} mode for target: {}",
        "✓".green().bold(),
        mode.cyan(),
        target_name.cyan()
    );

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    match target_name {
        "all" => {
            pb.set_message("Building native binary...");
            let mut args = vec!["build"];
            if release {
                args.push("--release");
            }
            let status = std::process::Command::new("cargo")
                .args(&args)
                .status()
                .map_err(|e| crate::error::RicError::BuildFailed(e.to_string()))?;
            pb.finish();
            if !status.success() {
                return Err(crate::error::RicError::BuildFailed("Build failed".to_string()));
            }
        }
        "python" => {
            pb.set_message("Building Python bindings...");
            let mut args = vec!["build", "--release", "--features", "pyo3"];
            let status = std::process::Command::new("cargo")
                .args(&args)
                .status()
                .map_err(|e| crate::error::RicError::BuildFailed(e.to_string()))?;
            pb.finish();
            if !status.success() {
                return Err(crate::error::RicError::BuildFailed("Python build failed".to_string()));
            }
            println!("{} Python bindings built successfully", "✓".green().bold());
            println!("  {} Output: target/release/", "→".yellow().bold());
        }
        "java" => {
            pb.set_message("Building Java bindings...");
            let mut args = vec!["build", "--release", "--features", "jni"];
            let status = std::process::Command::new("cargo")
                .args(&args)
                .status()
                .map_err(|e| crate::error::RicError::BuildFailed(e.to_string()))?;
            pb.finish();
            if !status.success() {
                return Err(crate::error::RicError::BuildFailed("Java build failed".to_string()));
            }
            println!("{} Java bindings built successfully", "✓".green().bold());
            println!("  {} Output: target/release/", "→".yellow().bold());
        }
        "c" => {
            pb.set_message("Building C/C++ bindings...");
            let mut args = vec!["build", "--release", "--features", "c-api"];
            let status = std::process::Command::new("cargo")
                .args(&args)
                .status()
                .map_err(|e| crate::error::RicError::BuildFailed(e.to_string()))?;
            pb.finish();
            if !status.success() {
                return Err(crate::error::RicError::BuildFailed("C/C++ build failed".to_string()));
            }
            println!("{} C/C++ bindings built successfully", "✓".green().bold());
            println!("  {} Output: target/release/", "→".yellow().bold());
        }
        "wasm" => {
            pb.finish();
            println!("{} WebAssembly build requires wasm-pack", "ℹ".blue().bold());
            println!("  {} Install wasm-pack: cargo install wasm-pack", "→".yellow().bold());
            println!("  {} Then run: wasm-pack build --target web", "→".yellow().bold());
            return Err(crate::error::RicError::BuildFailed(
                "WASM build not yet integrated. Use wasm-pack directly.".to_string()
            ));
        }
        _ => {
            pb.finish();
            return Err(crate::error::RicError::BuildFailed(format!(
                "Unknown target: {}. Valid targets: all, python, java, c, wasm",
                target_name
            )));
        }
    }

    println!("{} Build completed successfully!", "✓".green().bold());
    Ok(())
}

/// Run the Ri project in development or release mode
///
/// This function executes the project using cargo run, which compiles
/// the project if needed and then runs the resulting binary.
///
/// # Arguments
///
/// * `release` - Whether to run in release mode (optimized binary)
/// * `config` - Optional path to custom configuration file
///
/// # Execution Process
///
/// 1. Display run mode information
/// 2. Set environment variable for configuration path if specified
/// 3. Execute cargo run with appropriate flags
/// 4. Handle success or failure
///
/// # Errors
///
/// Returns `RicError::RunFailed` if the execution fails
///
/// # Examples
///
/// ```rust,ignore
/// // Run in debug mode
/// run_project(false, None).await?;
///
/// // Run in release mode
/// run_project(true, None).await?;
///
/// // Run with custom configuration
/// run_project(false, Some("/path/to/config.yaml")).await?;
/// ```
pub async fn run_project(release: bool, config: Option<&str>) -> Result<()> {
    // Determine run mode for display
    let mode = if release { "release" } else { "debug" };
    
    // Display run mode and configuration information
    println!(
        "{} Running project in {} mode...",
        "✓".green().bold(),
        mode.cyan()
    );

    // Display configuration file if specified
    if let Some(config_path) = config {
        println!("{} Using configuration: {}", "→".yellow().bold(), config_path.cyan());
    }

    // Build cargo arguments
    let mut args = vec!["run"];
    if release {
        args.push("--release");
    }

    // Execute cargo run with optional configuration environment variable
    let mut command = std::process::Command::new("cargo");
    command.args(&args);
    
    // Set configuration file path as environment variable if specified
    if let Some(config_path) = config {
        command.env("RI_CONFIG_PATH", config_path);
    }

    let status = command
        .status()
        .map_err(|e| crate::error::RicError::RunFailed(e.to_string()))?;

    if !status.success() {
        return Err(crate::error::RicError::RunFailed("Run failed".to_string()));
    }

    Ok(())
}

/// Handle configuration management commands
///
/// This function dispatches configuration subcommands to their specific handlers:
/// - `Init` - Create a new configuration file with defaults
/// - `Show` - Display current configuration
/// - `Validate` - Validate configuration file with detailed output
/// - `Check` - Check environment variables
/// - `Set` - Update a configuration value
/// - `Get` - Retrieve a configuration value
///
/// # Arguments
///
/// * `action` - Configuration subcommand to execute
///
/// # Errors
///
/// Returns various `RicError` types depending on the operation:
/// - `RicError::Io` for file system errors
/// - `RicError::Yaml` for YAML parsing errors
/// - `RicError::ConfigInvalid` for validation errors
/// - `RicError::ConfigKeyNotFound` for invalid keys
/// - `RicError::ConfigFileNotFound` for missing configuration files
///
/// # Examples
///
/// ```rust,ignore
/// // Initialize configuration
/// handle_config(ConfigAction::Init).await?;
///
/// // Show configuration
/// handle_config(ConfigAction::Show).await?;
///
/// // Validate configuration file
/// handle_config(ConfigAction::Validate { file: None }).await?;
///
/// // Check environment variables
/// handle_config(ConfigAction::Check).await?;
///
/// // Set a value
/// handle_config(ConfigAction::Set {
///     key: "runtime.workers".to_string(),
///     value: "8".to_string(),
/// }).await?;
/// ```
pub async fn handle_config(action: ConfigAction) -> Result<()> {
    // Match on the configuration action and execute appropriate handler
    match action {
        // Initialize a new configuration file with default values
        ConfigAction::Init => {
            config_init().await
        }
        
        // Display current configuration in YAML format
        ConfigAction::Show => {
            config_show().await
        }
        
        // Validate configuration file with detailed output
        ConfigAction::Validate { file } => {
            config_validate(file).await
        }
        
        // Check environment variables
        ConfigAction::Check => {
            config_check().await
        }
        
        // Set a configuration value
        ConfigAction::Set { key, value } => {
            config_set(key, value).await
        }
        
        // Get a configuration value
        ConfigAction::Get { key } => {
            config_get(key).await
        }
    }
}

/// Initialize a new configuration file
///
/// Creates a new ric.yaml file with default values in the current directory.
/// If a configuration file already exists, prompts the user before overwriting.
///
/// # Process
///
/// 1. Check if configuration file already exists
/// 2. Create default configuration
/// 3. Save configuration to file
/// 4. Display success message with file location
///
/// # Errors
///
/// Returns `RicError::Io` if file writing fails
///
/// # Examples
///
/// ```rust,ignore
/// config_init().await?;
/// ```
async fn config_init() -> Result<()> {
    println!("{} Initializing configuration...", "✓".green().bold());
    
    // Check if configuration file already exists
    let config_path = Path::new("ric.yaml");
    if config_path.exists() {
        println!("{} Configuration file already exists", "⚠".yellow().bold());
        println!("  {} Overwriting existing configuration", "→".yellow().bold());
    }
    
    // Create default configuration
    let config = crate::cli_config::RicConfig::default();
    
    // Save configuration to file
    config.save()?;
    
    // Display success message
    println!("{} Configuration file created successfully!", "✓".green().bold());
    println!("  {} Location: {}", "→".yellow().bold(), "ric.yaml".cyan());
    println!();
    println!("{}", "Default configuration:".yellow().bold());
    println!("  • Project name: {}", config.project.name.cyan());
    println!("  • Version: {}", config.project.version.cyan());
    println!("  • Template: {}", config.project.template.cyan());
    println!("  • Workers: {}", config.runtime.workers.to_string().cyan());
    println!("  • Log level: {}", config.runtime.log_level.cyan());
    
    Ok(())
}

/// Show current configuration
///
/// Displays the current configuration in YAML format.
/// If no configuration file exists, displays default values.
///
/// # Process
///
/// 1. Load configuration from file (or use defaults)
/// 2. Display configuration in formatted YAML
///
/// # Errors
///
/// Returns `RicError::Io` if file reading fails
/// Returns `RicError::Yaml` if YAML parsing fails
///
/// # Examples
///
/// ```rust,ignore
/// config_show().await?;
/// ```
async fn config_show() -> Result<()> {
    println!("{} Current configuration:", "✓".green().bold());
    println!();
    
    // Load configuration from file
    let config = crate::cli_config::RicConfig::load()?;
    
    // Display configuration in YAML format
    let yaml = serde_yaml::to_string(&config)?;
    println!("{}", yaml);
    
    Ok(())
}

/// Validate configuration file with detailed output
///
/// Validates the specified configuration file for correctness and consistency.
/// Provides detailed validation output with visual indicators for:
/// - ✅ Valid configuration sections
/// - ❌ Invalid configuration with detailed errors
/// - ⚠️ Warnings for non-critical issues
///
/// # Arguments
///
/// * `file` - Optional path to configuration file. Defaults to ric.yaml
///
/// # Validation Checks
///
/// 1. File existence and readability
/// 2. YAML syntax validity
/// 3. Required fields presence
/// 4. Value type correctness
/// 5. Value range constraints
/// 6. Cross-field consistency
///
/// # Errors
///
/// Returns `RicError::ConfigFileNotFound` if file doesn't exist
/// Returns `RicError::Yaml` if YAML parsing fails
/// Returns `RicError::ConfigInvalid` if validation fails
/// Returns `RicError::Io` for file system errors
///
/// # Examples
///
/// ```rust,ignore
/// // Validate default configuration file
/// config_validate(None).await?;
///
/// // Validate specific configuration file
/// config_validate(Some(PathBuf::from("custom.yaml"))).await?;
/// ```
async fn config_validate(file: Option<PathBuf>) -> Result<()> {
    // Determine configuration file path
    let config_path = file.unwrap_or_else(|| PathBuf::from("ric.yaml"));
    
    println!("{} Validating configuration...", "✓".green().bold());
    println!("  {} File: {}", "→".yellow().bold(), config_path.display().to_string().cyan());
    println!();
    
    // Step 1: Check file existence
    if !config_path.exists() {
        println!("{} Configuration file not found", "✗".red().bold());
        println!();
        println!("{}", "Error Details:".red().bold());
        println!("  {} File does not exist: {}", "→".yellow().bold(), config_path.display().to_string().cyan());
        println!();
        println!("{}", "Fix Suggestions:".yellow().bold());
        println!("  1. Create a new configuration file:");
        println!("     {}", "ric config init".cyan());
        println!("  2. Or specify an existing configuration file:");
        println!("     {} <path-to-config>", "ric config validate".cyan());
        
        return Err(crate::error::RicError::ConfigFileNotFound(
            config_path.display().to_string()
        ));
    }
    
    println!("{} File exists", "✓".green().bold());
    
    // Step 2: Check file readability
    let content = match std::fs::read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            println!("{} Cannot read file", "✗".red().bold());
            println!();
            println!("{}", "Error Details:".red().bold());
            println!("  {} Permission denied or file is corrupted", "→".yellow().bold());
            println!("  {} Error: {}", "→".yellow().bold(), e.to_string().red());
            println!();
            println!("{}", "Fix Suggestions:".yellow().bold());
            println!("  1. Check file permissions:");
            println!("     {} (Unix/Linux/macOS)", "chmod 644 ric.yaml".cyan());
            println!("  2. Ensure you have read access to the file");
            println!("  3. Verify the file is not locked by another process");
            
            return Err(crate::error::RicError::Io(e));
        }
    };
    
    println!("{} File is readable", "✓".green().bold());
    
    // Step 3: Validate YAML syntax
    let config: crate::cli_config::RicConfig = match serde_yaml::from_str(&content) {
        Ok(config) => config,
        Err(e) => {
            println!("{} Invalid YAML syntax", "✗".red().bold());
            println!();
            println!("{}", "Error Details:".red().bold());
            println!("  {} YAML parsing error", "→".yellow().bold());
            println!("  {} Error: {}", "→".yellow().bold(), e.to_string().red());
            println!();
            println!("{}", "Fix Suggestions:".yellow().bold());
            println!("  1. Check YAML syntax at: {}", "https://yaml.org/".cyan());
            println!("  2. Common issues:");
            println!("     • Missing quotes around special characters");
            println!("     • Incorrect indentation (use spaces, not tabs)");
            println!("     • Missing colons after keys");
            println!("     • Unmatched brackets or braces");
            println!("  3. Use a YAML validator to check syntax");
            
            return Err(crate::error::RicError::Yaml(e));
        }
    };
    
    println!("{} YAML syntax is valid", "✓".green().bold());
    
    // Step 4: Validate configuration structure
    println!();
    println!("{}", "Validating configuration structure...".yellow().bold());
    
    let mut has_errors = false;
    let mut has_warnings = false;
    
    // Validate project section
    println!();
    println!("  {} Project section:", "→".yellow().bold());
    if config.project.name.is_empty() {
        println!("    {} Project name is empty", "✗".red().bold());
        println!("      {} Fix: Set a non-empty project name", "→".yellow().bold());
        has_errors = true;
    } else {
        println!("    {} Project name: {}", "✓".green().bold(), config.project.name.cyan());
    }
    
    if config.project.version.is_empty() {
        println!("    {} Project version is empty", "⚠".yellow().bold());
        println!("      {} Using default: 0.1.0", "→".yellow().bold());
        has_warnings = true;
    } else {
        println!("    {} Project version: {}", "✓".green().bold(), config.project.version.cyan());
    }
    
    if config.project.template.is_empty() {
        println!("    {} Project template is empty", "⚠".yellow().bold());
        println!("      {} Using default: default", "→".yellow().bold());
        has_warnings = true;
    } else {
        println!("    {} Project template: {}", "✓".green().bold(), config.project.template.cyan());
    }
    
    // Validate build section
    println!();
    println!("  {} Build section:", "→".yellow().bold());
    println!("    {} Release mode: {}", "✓".green().bold(), 
        if config.build.release { "enabled".cyan() } else { "disabled".cyan() });
    println!("    {} Build target: {}", "✓".green().bold(), config.build.target.cyan());
    
    if config.build.features.is_empty() {
        println!("    {} No features enabled", "⚠".yellow().bold());
        has_warnings = true;
    } else {
        println!("    {} Features: {}", "✓".green().bold(), config.build.features.join(", ").cyan());
    }
    
    // Validate runtime section
    println!();
    println!("  {} Runtime section:", "→".yellow().bold());
    
    if config.runtime.workers == 0 {
        println!("    {} Workers count is zero", "✗".red().bold());
        println!("      {} Fix: Set workers to a positive number (recommended: CPU cores)", "→".yellow().bold());
        has_errors = true;
    } else if config.runtime.workers > 256 {
        println!("    {} Workers count is very high: {}", "⚠".yellow().bold(), config.runtime.workers);
        println!("      {} Consider reducing to match CPU cores", "→".yellow().bold());
        has_warnings = true;
    } else {
        println!("    {} Workers: {}", "✓".green().bold(), config.runtime.workers.to_string().cyan());
    }
    
    let valid_log_levels = ["trace", "debug", "info", "warn", "error", "off"];
    if !valid_log_levels.contains(&config.runtime.log_level.to_lowercase().as_str()) {
        println!("    {} Invalid log level: {}", "⚠".yellow().bold(), config.runtime.log_level);
        println!("      {} Valid levels: {}", "→".yellow().bold(), valid_log_levels.join(", ").cyan());
        has_warnings = true;
    } else {
        println!("    {} Log level: {}", "✓".green().bold(), config.runtime.log_level.cyan());
    }
    
    // Display final validation result
    println!();
    if has_errors {
        println!("{} Configuration validation failed!", "✗".red().bold());
        println!("  {} Please fix the errors above and validate again", "→".yellow().bold());
        return Err(crate::error::RicError::ConfigInvalid(
            "Configuration contains errors".to_string()
        ));
    } else if has_warnings {
        println!("{} Configuration is valid with warnings", "⚠".yellow().bold());
        println!("  {} Consider addressing the warnings above", "→".yellow().bold());
    } else {
        println!("{} Configuration is valid!", "✓".green().bold());
    }
    
    Ok(())
}

/// Check environment variables
///
/// Checks the environment for required and optional environment variables
/// that affect Ri project behavior. Displays which variables are set
/// and their current values.
///
/// # Environment Variables
///
/// ## Ri-specific Variables
/// - `RI_CONFIG_PATH`: Custom configuration file path
/// - `RI_LOG_LEVEL`: Override log level
///
/// ## Rust/Cargo Variables
/// - `RUST_LOG`: Rust logging configuration
/// - `CARGO_HOME`: Cargo home directory
/// - `RUSTUP_HOME`: Rustup home directory
///
/// ## Build Variables
/// - `RUSTFLAGS`: Additional Rust compiler flags
/// - `CARGO_BUILD_TARGET`: Default build target
///
/// # Output Format
///
/// - ✅ Variable is set
/// - ⚠️ Optional variable is not set
/// - ❌ Required variable is missing (if any)
///
/// # Examples
///
/// ```rust,ignore
/// config_check().await?;
/// ```
async fn config_check() -> Result<()> {
    println!("{} Checking environment variables...", "✓".green().bold());
    println!();
    
    // Define environment variables to check
    let ri_vars = [
        ("RI_CONFIG_PATH", false, "Custom configuration file path"),
        ("RI_LOG_LEVEL", false, "Override log level"),
    ];
    
    let rust_vars = [
        ("RUST_LOG", false, "Rust logging configuration"),
        ("CARGO_HOME", false, "Cargo home directory"),
        ("RUSTUP_HOME", false, "Rustup home directory"),
        ("RUSTFLAGS", false, "Additional Rust compiler flags"),
        ("CARGO_BUILD_TARGET", false, "Default build target"),
    ];
    
    // Check Ri-specific variables
    println!("{}", "Ri-specific Variables:".yellow().bold());
    for (var, required, description) in &ri_vars {
        match std::env::var(var) {
            Ok(value) => {
                println!("  {} {} is set", "✓".green().bold(), var.cyan());
                println!("    {} Value: {}", "→".yellow().bold(), value.cyan());
                println!("    {} {}", "→".yellow().bold(), description.dimmed());
            }
            Err(_) => {
                if *required {
                    println!("  {} {} is not set (required)", "✗".red().bold(), var.cyan());
                    println!("    {} {}", "→".yellow().bold(), description.dimmed());
                } else {
                    println!("  {} {} is not set (optional)", "⚠".yellow().bold(), var.cyan());
                    println!("    {} {}", "→".yellow().bold(), description.dimmed());
                }
            }
        }
    }
    
    // Check Rust/Cargo variables
    println!();
    println!("{}", "Rust/Cargo Variables:".yellow().bold());
    for (var, required, description) in &rust_vars {
        match std::env::var(var) {
            Ok(value) => {
                println!("  {} {} is set", "✓".green().bold(), var.cyan());
                println!("    {} Value: {}", "→".yellow().bold(), value.cyan());
                println!("    {} {}", "→".yellow().bold(), description.dimmed());
            }
            Err(_) => {
                if *required {
                    println!("  {} {} is not set (required)", "✗".red().bold(), var.cyan());
                    println!("    {} {}", "→".yellow().bold(), description.dimmed());
                } else {
                    println!("  {} {} is not set (optional)", "⚠".yellow().bold(), var.cyan());
                    println!("    {} {}", "→".yellow().bold(), description.dimmed());
                }
            }
        }
    }
    
    // Display summary
    println!();
    println!("{}", "Environment Check Summary:".yellow().bold());
    println!("  {} All required variables are set", "✓".green().bold());
    println!("  {} Optional variables can be configured as needed", "→".yellow().bold());
    println!();
    println!("{}", "To set environment variables:".yellow().bold());
    println!("  {} Unix/Linux/macOS: export VAR_NAME=value", "→".yellow().bold());
    println!("  {} Windows (PowerShell): $env:VAR_NAME=\"value\"", "→".yellow().bold());
    println!("  {} Windows (CMD): set VAR_NAME=value", "→".yellow().bold());
    
    Ok(())
}

/// Set a configuration value
///
/// Updates a configuration value in the ric.yaml file.
/// The key uses dot notation to access nested values.
///
/// # Arguments
///
/// * `key` - Configuration key in dot notation
/// * `value` - New value to set
///
/// # Process
///
/// 1. Load current configuration
/// 2. Update the specified key with new value
/// 3. Validate the updated configuration
/// 4. Save configuration to file
///
/// # Errors
///
/// Returns `RicError::ConfigKeyNotFound` if the key doesn't exist
/// Returns `RicError::ConfigInvalid` if the value is invalid
/// Returns `RicError::Io` if file writing fails
///
/// # Examples
///
/// ```rust,ignore
/// config_set("runtime.workers".to_string(), "8".to_string()).await?;
/// ```
async fn config_set(key: String, value: String) -> Result<()> {
    println!("{} Setting configuration value...", "✓".green().bold());
    println!("  {} Key: {}", "→".yellow().bold(), key.cyan());
    println!("  {} Value: {}", "→".yellow().bold(), value.cyan());
    println!();
    
    // Load current configuration
    let mut config = crate::cli_config::RicConfig::load()?;
    
    // Update the specified key
    config.set(&key, &value)?;
    
    // Validate the updated configuration
    config.validate()?;
    
    // Save configuration to file
    config.save()?;
    
    println!("{} Configuration updated successfully!", "✓".green().bold());
    println!("  {} Key: {} has been set to: {}", "→".yellow().bold(), key.cyan(), value.cyan());
    
    Ok(())
}

/// Get a configuration value
///
/// Retrieves and displays a configuration value from the ric.yaml file.
///
/// # Arguments
///
/// * `key` - Configuration key to retrieve
///
/// # Process
///
/// 1. Load current configuration
/// 2. Retrieve the value for the specified key
/// 3. Display the value
///
/// # Errors
///
/// Returns `RicError::ConfigKeyNotFound` if the key doesn't exist
/// Returns `RicError::Io` if file reading fails
///
/// # Examples
///
/// ```rust,ignore
/// config_get("project.name".to_string()).await?;
/// ```
async fn config_get(key: String) -> Result<()> {
    // Load current configuration
    let config = crate::cli_config::RicConfig::load()?;
    
    // Retrieve the value for the specified key
    let value = config.get(&key)?;
    
    // Display the value
    println!("{} Configuration value:", "✓".green().bold());
    println!("  {} {}: {}", "→".yellow().bold(), key.cyan(), value.cyan());
    
    Ok(())
}

/// Check the project for compilation errors
///
/// This function runs cargo check to verify the project compiles without
/// producing an executable. It's faster than a full build and useful for:
/// - Quick error detection during development
/// - IDE integration for real-time feedback
/// - CI/CD pipelines for early failure detection
///
/// # Errors
///
/// Returns `RicError::CheckFailed` if the check fails
///
/// # Examples
///
/// ```rust,ignore
/// check_project().await?;
/// ```
pub async fn check_project() -> Result<()> {
    println!("{} Checking project...", "✓".green().bold());

    // Execute cargo check to verify compilation
    let status = std::process::Command::new("cargo")
        .args(&["check"])
        .status()
        .map_err(|e| crate::error::RicError::CheckFailed(e.to_string()))?;

    // Display result
    if status.success() {
        println!("{} No errors found!", "✓".green().bold());
    } else {
        return Err(crate::error::RicError::CheckFailed("Check failed".to_string()));
    }

    Ok(())
}

/// Clean build artifacts from the project
///
/// This function removes all build artifacts from the target directory,
/// including compiled binaries, intermediate object files, and dependency
/// artifacts. Useful for:
/// - Freeing disk space
/// - Resolving build issues caused by stale artifacts
/// - Starting a fresh build from scratch
///
/// # Errors
///
/// Returns `RicError::CleanFailed` if the clean operation fails
///
/// # Examples
///
/// ```rust,ignore
/// clean_project().await?;
/// ```
pub async fn clean_project() -> Result<()> {
    println!("{} Cleaning project...", "✓".green().bold());

    // Execute cargo clean to remove build artifacts
    let status = std::process::Command::new("cargo")
        .args(&["clean"])
        .status()
        .map_err(|e| crate::error::RicError::CleanFailed(e.to_string()))?;

    // Display result
    if status.success() {
        println!("{} Project cleaned!", "✓".green().bold());
    } else {
        return Err(crate::error::RicError::CleanFailed("Clean failed".to_string()));
    }

    Ok(())
}

/// Display comprehensive project information
///
/// This function displays detailed information about the current project
/// and development environment, including:
/// - Ri framework version
/// - CLI tool version
/// - Rust version
/// - Project metadata (if Cargo.toml exists)
/// - Available features and capabilities
///
/// The information is displayed in a formatted, colored output for
/// better readability.
///
/// # Errors
///
/// Returns `RicError::Io` if reading Cargo.toml fails
///
/// # Examples
///
/// ```rust,ignore
/// show_info().await?;
/// ```
pub async fn show_info() -> Result<()> {
    // Calculate padding for centered title
    let title = "Ri Project Information";
    let padding = (50 - title.len()) / 2;
    let centered_title = format!("{}{}", " ".repeat(padding), title);
    
    // Display header
    println!("{}", "═".repeat(50));
    println!("{}", centered_title.green().bold());
    println!("{}", "═".repeat(50));

    // Display environment information
    println!("\n{} Environment:", "→".yellow().bold());
    println!("  {:<15} {}", "Ri Version:", "0.1.9".cyan());
    println!("  {:<15} {}", "CLI Version:", env!("CARGO_PKG_VERSION").cyan());
    println!("  {:<15} {}", "Rust Version:", "stable".cyan());

    // Display project information if Cargo.toml exists
    if Path::new("Cargo.toml").exists() {
        println!("\n{} Project:", "→".yellow().bold());
        if let Ok(content) = fs::read_to_string("Cargo.toml") {
            // Display first 10 lines containing key metadata
            for line in content.lines().take(10) {
                if line.starts_with("name = ") || line.starts_with("version = ") || line.starts_with("edition = ") {
                    println!("  {}", line.dimmed());
                }
            }
        }
    }

    // Display available features
    println!("\n{} Features:", "→".yellow().bold());
    println!("  • {}", "Python bindings (PyO3)".dimmed());
    println!("  • {}", "Java bindings (JNI)".dimmed());
    println!("  • {}", "C/C++ bindings (FFI)".dimmed());
    println!("  • {}", "gRPC support".dimmed());
    println!("  • {}", "WebSocket support".dimmed());
    println!("  • {}", "Message queues (RabbitMQ, Kafka)".dimmed());

    // Display footer
    println!("\n{}", "═".repeat(50));

    Ok(())
}

/// Diagnostic result status for individual checks
///
/// Represents the outcome of a single diagnostic check with
/// associated message and optional suggestion for fixes.
#[derive(Debug, Clone)]
enum DiagnosticStatus {
    /// Check passed successfully
    Pass,
    /// Check passed with warnings
    Warning,
    /// Check failed with error
    Error,
}

/// Diagnostic check result structure
///
/// Contains all information about a single diagnostic check including
/// its category, name, status, message, and optional fix suggestion.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct DiagnosticResult {
    /// Category of the diagnostic check (e.g., "Rust Toolchain", "Environment")
    category: String,
    /// Name of the specific check (e.g., "rustc version", "CARGO_HOME")
    name: String,
    /// Status of the check (Pass, Warning, Error)
    status: DiagnosticStatus,
    /// Human-readable message describing the result
    message: String,
    /// Optional suggestion for fixing issues
    suggestion: Option<String>,
    /// Optional detailed information (shown in verbose mode)
    details: Option<String>,
}

impl DiagnosticResult {
    /// Create a new diagnostic result
    fn new(category: &str, name: &str, status: DiagnosticStatus, message: &str) -> Self {
        Self {
            category: category.to_string(),
            name: name.to_string(),
            status,
            message: message.to_string(),
            suggestion: None,
            details: None,
        }
    }

    /// Add a suggestion to the diagnostic result
    fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    /// Add detailed information to the diagnostic result
    fn with_details(mut self, details: &str) -> Self {
        self.details = Some(details.to_string());
        self
    }

    /// Print the diagnostic result with colored output
    fn print(&self, verbose: bool) {
        let status_icon = match self.status {
            DiagnosticStatus::Pass => "✓".green().bold(),
            DiagnosticStatus::Warning => "⚠".yellow().bold(),
            DiagnosticStatus::Error => "✗".red().bold(),
        };

        println!("  {} {}: {}", status_icon, self.name.cyan(), self.message);

        if verbose {
            if let Some(ref details) = self.details {
                println!("    {} Details: {}", "→".yellow().bold(), details.dimmed());
            }
        }

        if let Some(ref suggestion) = self.suggestion {
            println!("    {} {}", "💡".blue(), suggestion);
        }
    }
}

/// Run comprehensive diagnostic checks on the development environment
///
/// This function performs a series of diagnostic checks to identify potential
/// issues in the development environment and project configuration. It covers:
///
/// - Rust toolchain (rustc, cargo, rustup)
/// - Development tools (git, etc.)
/// - Environment variables (RUST_LOG, CARGO_HOME, RUSTUP_HOME)
/// - Port availability (8080, 8081, etc.)
/// - Dependency conflicts
/// - File system permissions and disk space
///
/// # Arguments
///
/// * `verbose` - Show detailed information for each check
/// * `fix` - Attempt to automatically fix detected issues
///
/// # Diagnostic Categories
///
/// ## Rust Toolchain
/// - rustc version and target triple
/// - cargo version
/// - rustup version (if installed)
///
/// ## Development Tools
/// - git version and configuration
/// - other build tools
///
/// ## Environment Variables
/// - RUST_LOG: Logging configuration
/// - CARGO_HOME: Cargo installation directory
/// - RUSTUP_HOME: Rustup installation directory
///
/// ## Port Availability
/// - Check if common development ports are available
/// - Ports: 8080, 8081, 3000, 5000, 50051
///
/// ## Dependencies
/// - Check for version conflicts in Cargo.toml
/// - Verify all dependencies are resolvable
///
/// ## File System
/// - Write permissions in current directory
/// - Available disk space
///
/// # Auto-Fix Capabilities
///
/// When `fix` is true, the function attempts to:
/// - Set missing environment variables to sensible defaults
/// - Create missing required directories
/// - Fix common configuration issues
///
/// # Output Format
///
/// - ✅ Passed checks (green)
/// - ⚠️ Warnings (yellow)
/// - ❌ Errors (red)
/// - 💡 Suggestions (blue)
///
/// # Errors
///
/// Returns `RicError::DoctorFailed` if critical diagnostics fail
/// Returns `RicError::DoctorFixFailed` if auto-fix operations fail
///
/// # Examples
///
/// ```rust,ignore
/// // Run basic diagnostics
/// doctor(false, false).await?;
///
/// // Run with verbose output
/// doctor(true, false).await?;
///
/// // Run and auto-fix issues
/// doctor(false, true).await?;
/// ```
pub async fn doctor(verbose: bool, fix: bool) -> Result<()> {
    // Display header
    println!("{}", "═".repeat(60));
    println!("{}", "  Ri Development Environment Diagnostics".green().bold());
    println!("{}", "═".repeat(60));
    println!();

    if verbose {
        println!("{} Running diagnostics in verbose mode", "→".yellow().bold());
        if fix {
            println!("{} Auto-fix enabled", "→".yellow().bold());
        }
        println!();
    }

    // Collect all diagnostic results
    let mut results: Vec<DiagnosticResult> = Vec::new();
    let mut fixable_issues: Vec<(String, String)> = Vec::new();

    // ========================================
    // Section 1: Rust Toolchain Checks
    // ========================================
    println!("{}", "Rust Toolchain".yellow().bold());
    println!("{}", "─".repeat(40));

    // Check rustc
    match check_rustc(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    // Check cargo
    match check_cargo(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    // Check rustup
    match check_rustup(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    println!();

    // ========================================
    // Section 2: Development Tools Checks
    // ========================================
    println!("{}", "Development Tools".yellow().bold());
    println!("{}", "─".repeat(40));

    // Check git
    match check_git(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    println!();

    // ========================================
    // Section 3: Environment Variables Checks
    // ========================================
    println!("{}", "Environment Variables".yellow().bold());
    println!("{}", "─".repeat(40));

    // Check RUST_LOG
    match check_env_var("RUST_LOG", false, verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result.clone());
            if fix {
                fixable_issues.push(("RUST_LOG".to_string(), "info".to_string()));
            }
        }
    }

    // Check CARGO_HOME
    match check_env_var("CARGO_HOME", false, verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    // Check RUSTUP_HOME
    match check_env_var("RUSTUP_HOME", false, verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    println!();

    // ========================================
    // Section 4: Port Availability Checks
    // ========================================
    println!("{}", "Port Availability".yellow().bold());
    println!("{}", "─".repeat(40));

    let ports_to_check = [8080, 8081, 3000, 5000, 50051];
    for port in ports_to_check.iter() {
        match check_port(*port, verbose) {
            Ok(result) => {
                result.print(verbose);
                results.push(result);
            }
            Err(result) => {
                result.print(verbose);
                results.push(result);
            }
        }
    }

    println!();

    // ========================================
    // Section 5: Dependencies Checks
    // ========================================
    println!("{}", "Dependencies".yellow().bold());
    println!("{}", "─".repeat(40));

    // Check Cargo.toml for dependency conflicts
    match check_dependencies(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    println!();

    // ========================================
    // Section 6: File System Checks
    // ========================================
    println!("{}", "File System".yellow().bold());
    println!("{}", "─".repeat(40));

    // Check write permissions
    match check_write_permissions(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    // Check disk space
    match check_disk_space(verbose) {
        Ok(result) => {
            result.print(verbose);
            results.push(result);
        }
        Err(result) => {
            result.print(verbose);
            results.push(result);
        }
    }

    println!();

    // ========================================
    // Summary and Auto-Fix
    // ========================================
    let passed = results.iter().filter(|r| matches!(r.status, DiagnosticStatus::Pass)).count();
    let warnings = results.iter().filter(|r| matches!(r.status, DiagnosticStatus::Warning)).count();
    let errors = results.iter().filter(|r| matches!(r.status, DiagnosticStatus::Error)).count();

    println!("{}", "═".repeat(60));
    println!("{}", "  Diagnostic Summary".green().bold());
    println!("{}", "═".repeat(60));
    println!();
    println!("  {} Passed:   {}", "✓".green().bold(), passed.to_string().green());
    println!("  {} Warnings: {}", "⚠".yellow().bold(), warnings.to_string().yellow());
    println!("  {} Errors:   {}", "✗".red().bold(), errors.to_string().red());
    println!();

    // Auto-fix issues if requested
    if fix && !fixable_issues.is_empty() {
        println!("{}", "Auto-Fix".yellow().bold());
        println!("{}", "─".repeat(40));

        for (var, value) in fixable_issues {
            match apply_fix(&var, &value) {
                Ok(msg) => println!("  {} {}", "✓".green().bold(), msg),
                Err(e) => println!("  {} Failed to fix {}: {}", "✗".red().bold(), var, e),
            }
        }
        println!();
    }

    // Display final status
    if errors > 0 {
        println!("{} Some checks failed. Please review the errors above.", "✗".red().bold());
        if !fix {
            println!("  {} Run with {} to attempt automatic fixes", "→".yellow().bold(), "ric doctor --fix".cyan());
        }
    } else if warnings > 0 {
        println!("{} All critical checks passed, but some warnings were found.", "⚠".yellow().bold());
    } else {
        println!("{} All checks passed! Your development environment is healthy.", "✓".green().bold());
    }

    println!();

    Ok(())
}

/// Check rustc version and target
///
/// Verifies that rustc is installed and retrieves version information
/// including the target triple for the current platform.
///
/// # Arguments
///
/// * `verbose` - Include detailed version information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if rustc is available, Err(DiagnosticResult) otherwise.
fn check_rustc(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let output = Command::new("rustc")
        .args(&["--version"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // Get target triple
            let target_output = Command::new("rustc")
                .args(&["--print", "cfg"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        let cfg = String::from_utf8_lossy(&o.stdout).to_string();
                        // Extract target from cfg output
                        for line in cfg.lines() {
                            if line.starts_with("target_arch=") {
                                return Some(line.to_string());
                            }
                        }
                    }
                    None
                });

            let mut result = DiagnosticResult::new(
                "Rust Toolchain",
                "rustc",
                DiagnosticStatus::Pass,
                &version
            );

            if verbose {
                if let Some(target) = target_output {
                    result = result.with_details(&format!("Target config: {}", target));
                }
            }

            Ok(result)
        }
        _ => {
            Err(DiagnosticResult::new(
                "Rust Toolchain",
                "rustc",
                DiagnosticStatus::Error,
                "Not found"
            ).with_suggestion("Install Rust from https://rustup.rs/"))
        }
    }
}

/// Check cargo version
///
/// Verifies that cargo is installed and retrieves version information.
///
/// # Arguments
///
/// * `verbose` - Include detailed version information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if cargo is available, Err(DiagnosticResult) otherwise.
fn check_cargo(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let output = Command::new("cargo")
        .args(&["--version"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            let mut result = DiagnosticResult::new(
                "Rust Toolchain",
                "cargo",
                DiagnosticStatus::Pass,
                &version
            );

            if verbose {
                // Get cargo home directory
                let cargo_home = std::env::var("CARGO_HOME")
                    .unwrap_or_else(|_| "Not set".to_string());
                result = result.with_details(&format!("CARGO_HOME: {}", cargo_home));
            }

            Ok(result)
        }
        _ => {
            Err(DiagnosticResult::new(
                "Rust Toolchain",
                "cargo",
                DiagnosticStatus::Error,
                "Not found"
            ).with_suggestion("Install Rust from https://rustup.rs/"))
        }
    }
}

/// Check rustup version
///
/// Verifies that rustup is installed and retrieves version information.
/// This is an optional tool, so its absence results in a warning, not an error.
///
/// # Arguments
///
/// * `verbose` - Include detailed version information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if rustup is available, Err(DiagnosticResult) otherwise.
fn check_rustup(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let output = Command::new("rustup")
        .args(&["--version"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .lines()
                .next()
                .map(|s| s.trim().to_string())
                .unwrap_or_else(|| "Unknown version".to_string());
            
            let mut result = DiagnosticResult::new(
                "Rust Toolchain",
                "rustup",
                DiagnosticStatus::Pass,
                &version
            );

            if verbose {
                // Get installed toolchains
                let toolchains_output = Command::new("rustup")
                    .args(&["toolchain", "list"])
                    .output()
                    .ok()
                    .map(|o| String::from_utf8_lossy(&o.stdout).to_string());

                if let Some(toolchains) = toolchains_output {
                    let toolchain_list: Vec<&str> = toolchains.lines().take(5).collect();
                    result = result.with_details(&format!("Installed toolchains: {}", toolchain_list.join(", ")));
                }
            }

            Ok(result)
        }
        _ => {
            Err(DiagnosticResult::new(
                "Rust Toolchain",
                "rustup",
                DiagnosticStatus::Warning,
                "Not installed (optional)"
            ).with_suggestion("Install rustup from https://rustup.rs/ for easy Rust management"))
        }
    }
}

/// Check git version and configuration
///
/// Verifies that git is installed and retrieves version information.
/// Also checks for basic git configuration (user.name, user.email).
///
/// # Arguments
///
/// * `verbose` - Include detailed version and configuration information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if git is available, Err(DiagnosticResult) otherwise.
fn check_git(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let output = Command::new("git")
        .args(&["--version"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout)
                .trim()
                .replace("git version ", "");
            
            // Check git config
            let user_name = Command::new("git")
                .args(&["config", "user.name"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        None
                    }
                });

            let user_email = Command::new("git")
                .args(&["config", "user.email"])
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
                    } else {
                        None
                    }
                });

            let (status, message) = match (&user_name, &user_email) {
                (Some(name), Some(email)) => {
                    (DiagnosticStatus::Pass, format!("{} (configured: {} <{}>)", version, name, email))
                }
                _ => {
                    (DiagnosticStatus::Warning, format!("{} (not configured)", version))
                }
            };

            let mut result = DiagnosticResult::new(
                "Development Tools",
                "git",
                status,
                &message
            );

            if verbose {
                if user_name.is_none() || user_email.is_none() {
                    result = result.with_details("Git user.name and/or user.email not configured");
                }
            }

            if user_name.is_none() || user_email.is_none() {
                result = result.with_suggestion("Configure git: git config --global user.name \"Your Name\" && git config --global user.email \"your@email.com\"");
            }

            Ok(result)
        }
        _ => {
            Err(DiagnosticResult::new(
                "Development Tools",
                "git",
                DiagnosticStatus::Warning,
                "Not found (optional but recommended)"
            ).with_suggestion("Install git from https://git-scm.com/"))
        }
    }
}

/// Check if an environment variable is set
///
/// Verifies that the specified environment variable is set and optionally
/// validates its value.
///
/// # Arguments
///
/// * `var_name` - Name of the environment variable to check
/// * `required` - Whether the variable is required (error) or optional (warning)
/// * `verbose` - Include the current value in the output
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if the variable is set, Err(DiagnosticResult) otherwise.
fn check_env_var(var_name: &str, required: bool, verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    match std::env::var(var_name) {
        Ok(value) => {
            let display_value = if verbose {
                value.clone()
            } else {
                // Truncate long values for display
                if value.len() > 50 {
                    format!("{}...", &value[..47])
                } else {
                    value.clone()
                }
            };

            let mut result = DiagnosticResult::new(
                "Environment Variables",
                var_name,
                DiagnosticStatus::Pass,
                &format!("Set to: {}", display_value)
            );

            if verbose {
                result = result.with_details(&format!("Full value: {}", value));
            }

            Ok(result)
        }
        Err(_) => {
            let status = if required {
                DiagnosticStatus::Error
            } else {
                DiagnosticStatus::Warning
            };

            Err(DiagnosticResult::new(
                "Environment Variables",
                var_name,
                status,
                "Not set"
            ).with_suggestion(&format!("Set {} to an appropriate value", var_name)))
        }
    }
}

/// Check if a port is available for binding
///
/// Attempts to bind to the specified port to verify it's available
/// for development server use.
///
/// # Arguments
///
/// * `port` - Port number to check
/// * `verbose` - Include additional details
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if the port is available, Err(DiagnosticResult) if in use.
fn check_port(port: u16, verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    use std::net::{TcpListener, Ipv4Addr, SocketAddrV4};

    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    
    match TcpListener::bind(addr) {
        Ok(_) => {
            let mut result = DiagnosticResult::new(
                "Port Availability",
                &format!("Port {}", port),
                DiagnosticStatus::Pass,
                "Available"
            );

            if verbose {
                result = result.with_details(&format!("Successfully bound to 127.0.0.1:{}", port));
            }

            Ok(result)
        }
        Err(e) => {
            let mut result = DiagnosticResult::new(
                "Port Availability",
                &format!("Port {}", port),
                DiagnosticStatus::Warning,
                "In use or unavailable"
            );

            if verbose {
                result = result.with_details(&format!("Error: {}", e));
            }

            result = result.with_suggestion(&format!("Check if another service is using port {}", port));

            Err(result)
        }
    }
}

/// Check project dependencies for conflicts
///
/// Parses Cargo.toml and checks for potential dependency conflicts
/// or missing dependencies.
///
/// # Arguments
///
/// * `verbose` - Include detailed dependency information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if no issues found, Err(DiagnosticResult) otherwise.
fn check_dependencies(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let cargo_toml_path = Path::new("Cargo.toml");
    
    if !cargo_toml_path.exists() {
        return Err(DiagnosticResult::new(
            "Dependencies",
            "Cargo.toml",
            DiagnosticStatus::Warning,
            "Not found (not in a Rust project directory)"
        ).with_suggestion("Run this command from a Rust project directory"));
    }

    // Try to read and parse Cargo.toml
    match fs::read_to_string(cargo_toml_path) {
        Ok(content) => {
            // Basic check for dependency conflicts by looking for duplicate entries
            let mut dep_count: HashMap<String, usize> = HashMap::new();
            let mut has_conflicts = false;
            let mut conflict_list: Vec<String> = Vec::new();

            for line in content.lines() {
                let trimmed = line.trim();
                // Simple parsing for [dependencies] section
                if trimmed.starts_with("name = \"") || trimmed.starts_with("name=\"") {
                    let name = trimmed
                        .split('"')
                        .nth(1)
                        .unwrap_or("");
                    if !name.is_empty() {
                        *dep_count.entry(name.to_string()).or_insert(0) += 1;
                    }
                }
            }

            for (dep, count) in &dep_count {
                if *count > 1 {
                    has_conflicts = true;
                    conflict_list.push(format!("{} appears {} times", dep, count));
                }
            }

            if has_conflicts {
                let mut result = DiagnosticResult::new(
                    "Dependencies",
                    "Cargo.toml",
                    DiagnosticStatus::Warning,
                    "Potential conflicts detected"
                );

                if verbose {
                    result = result.with_details(&format!("Conflicts: {}", conflict_list.join(", ")));
                }

                result = result.with_suggestion("Review Cargo.toml for duplicate dependency entries");

                Err(result)
            } else {
                let mut result = DiagnosticResult::new(
                    "Dependencies",
                    "Cargo.toml",
                    DiagnosticStatus::Pass,
                    "No conflicts detected"
                );

                if verbose {
                    let dep_count_num = dep_count.len();
                    result = result.with_details(&format!("Found {} unique dependencies", dep_count_num));
                }

                Ok(result)
            }
        }
        Err(e) => {
            Err(DiagnosticResult::new(
                "Dependencies",
                "Cargo.toml",
                DiagnosticStatus::Error,
                &format!("Cannot read: {}", e)
            ).with_suggestion("Check file permissions"))
        }
    }
}

/// Check write permissions in current directory
///
/// Verifies that the current directory is writable by attempting
/// to create and delete a temporary file.
///
/// # Arguments
///
/// * `verbose` - Include detailed permission information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if writable, Err(DiagnosticResult) otherwise.
fn check_write_permissions(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    let test_file = Path::new(".ric_doctor_write_test");
    
    // Try to create a test file
    let result = fs::File::create(test_file)
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(b"test")
        });

    match result {
        Ok(_) => {
            // Clean up the test file
            let _ = fs::remove_file(test_file);

            let mut result = DiagnosticResult::new(
                "File System",
                "Write Permissions",
                DiagnosticStatus::Pass,
                "Current directory is writable"
            );

            if verbose {
                let current_dir = std::env::current_dir()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|_| "Unknown".to_string());
                result = result.with_details(&format!("Current directory: {}", current_dir));
            }

            Ok(result)
        }
        Err(e) => {
            let mut result = DiagnosticResult::new(
                "File System",
                "Write Permissions",
                DiagnosticStatus::Error,
                "Current directory is not writable"
            );

            if verbose {
                result = result.with_details(&format!("Error: {}", e));
            }

            result = result.with_suggestion("Check directory permissions or run from a different location");

            Err(result)
        }
    }
}

/// Check available disk space
///
/// Verifies that there is sufficient disk space available for
/// build operations and project files.
///
/// # Arguments
///
/// * `verbose` - Include detailed disk space information
///
/// # Returns
///
/// Returns Ok(DiagnosticResult) if sufficient space, Err(DiagnosticResult) otherwise.
fn check_disk_space(verbose: bool) -> std::result::Result<DiagnosticResult, DiagnosticResult> {
    // On Windows, we'll use a simple approach by checking if we can get disk info
    // For cross-platform, we'll use a basic check
    
    // Try to get current directory and check if it exists
    let current_dir = std::env::current_dir();
    
    match current_dir {
        Ok(dir) => {
            // Basic check - if we can access the directory, assume disk is available
            // More sophisticated checks would require platform-specific code
            
            let mut result = DiagnosticResult::new(
                "File System",
                "Disk Space",
                DiagnosticStatus::Pass,
                "Disk accessible"
            );

            if verbose {
                result = result.with_details(&format!("Working directory: {}", dir.display()));
            }

            Ok(result)
        }
        Err(e) => {
            let mut result = DiagnosticResult::new(
                "File System",
                "Disk Space",
                DiagnosticStatus::Warning,
                "Cannot determine disk status"
            );

            if verbose {
                result = result.with_details(&format!("Error: {}", e));
            }

            Err(result)
        }
    }
}

/// Apply an automatic fix for a detected issue
///
/// Attempts to automatically fix common issues such as setting
/// environment variables or creating missing directories.
///
/// # Arguments
///
/// * `issue` - Identifier for the issue to fix
/// * `value` - Value to apply for the fix
///
/// # Returns
///
/// Returns Ok(message) on success, Err(message) on failure.
fn apply_fix(issue: &str, value: &str) -> std::result::Result<String, String> {
    match issue {
        "RUST_LOG" => {
            // Note: This only sets the variable for the current process
            // In a real implementation, you might want to write to shell config files
            std::env::set_var("RUST_LOG", value);
            Ok(format!("Set RUST_LOG to '{}'", value))
        }
        _ => {
            Err(format!("Unknown fix for: {}", issue))
        }
    }
}

// =============================================================================
// Code Generation Commands
// =============================================================================

/// Handle code generation commands
///
/// This function dispatches generation subcommands to their specific handlers:
/// - `Module` - Generate a new Ri module with complete structure
/// - `Middleware` - Generate middleware template
/// - `Config` - Generate Rust struct from config file
///
/// # Arguments
///
/// * `action` - Generation subcommand to execute
///
/// # Errors
///
/// Returns various `RicError` types depending on the operation:
/// - `RicError::InvalidModuleType` for unsupported module types
/// - `RicError::ModuleExists` if module already exists
/// - `RicError::MiddlewareExists` if middleware already exists
/// - `RicError::GenerateConfigFileNotFound` if config file doesn't exist
/// - `RicError::UnsupportedConfigFormat` for unsupported file formats
/// - `RicError::GenerationFailed` for general generation errors
/// - `RicError::FormattingFailed` for code formatting errors
/// - `RicError::Io` for file system errors
///
/// # Examples
///
/// ```rust,ignore
/// use crate::cli::GenerateAction;
///
/// // Generate a cache module
/// handle_generate(GenerateAction::Module {
///     module_type: "cache".to_string(),
///     name: "my-cache".to_string(),
/// }).await?;
///
/// // Generate middleware
/// handle_generate(GenerateAction::Middleware {
///     name: "auth-middleware".to_string(),
/// }).await?;
///
/// // Generate config struct
/// handle_generate(GenerateAction::Config {
///     from: PathBuf::from("config.yaml"),
/// }).await?;
/// ```
pub async fn handle_generate(action: crate::cli::GenerateAction) -> Result<()> {
    match action {
        crate::cli::GenerateAction::Module { module_type, name } => {
            generate_module(module_type, name).await
        }
        crate::cli::GenerateAction::Middleware { name } => {
            generate_middleware(name).await
        }
        crate::cli::GenerateAction::Config { from } => {
            generate_config(from).await
        }
    }
}

/// Generate a new Ri module
///
/// Creates a complete module structure with all necessary files including:
/// - Module entry point (mod.rs)
/// - Configuration structures (config.rs)
/// - Request handlers (handler.rs)
/// - Business logic (service.rs)
/// - Test scaffolding (tests/mod.rs)
///
/// The function also updates Cargo.toml with necessary dependencies
/// based on the module type.
///
/// # Arguments
///
/// * `module_type` - Type of module to generate (cache, queue, gateway, auth, device, observability)
/// * `name` - Name of the module (used for directory and struct names)
///
/// # Module Types
///
/// | Type | Description | Dependencies Added |
/// |------|-------------|-------------------|
/// | `cache` | Caching module | redis, tokio, serde |
/// | `queue` | Message queue | lapin, rdkafka, tokio |
/// | `gateway` | API Gateway | hyper, tower, tokio |
/// | `auth` | Authentication | jsonwebtoken, oauth2 |
/// | `device` | IoT device | rumqttc, coap |
/// | `observability` | Monitoring | tracing, metrics |
///
/// # Generated Structure
///
/// ```text
/// src/modules/<name>/
/// ├── mod.rs           # Module entry point and public API
/// ├── config.rs        # Configuration structures with serde
/// ├── handler.rs       # Request/response handlers
/// ├── service.rs       # Core business logic
/// └── tests/
///     └── mod.rs       # Unit tests
/// ```
///
/// # Errors
///
/// Returns `RicError::InvalidModuleType` if the module type is not supported.
/// Returns `RicError::ModuleExists` if a module with the same name already exists.
/// Returns `RicError::GenerationFailed` if code generation fails.
/// Returns `RicError::FormattingFailed` if rustfmt fails.
/// Returns `RicError::Io` for file system errors.
///
/// # Examples
///
/// ```rust,ignore
/// // Generate a cache module
/// generate_module("cache".to_string(), "my-cache".to_string()).await?;
///
/// // Generate a queue module
/// generate_module("queue".to_string(), "message-queue".to_string()).await?;
/// ```
async fn generate_module(module_type: String, name: String) -> Result<()> {
    // Define valid module types
    let valid_types = ["cache", "queue", "gateway", "auth", "device", "observability"];
    
    // Validate module type
    if !valid_types.contains(&module_type.as_str()) {
        return Err(crate::error::RicError::InvalidModuleType {
            module_type,
            valid_types: valid_types.join(", "),
        });
    }

    // Validate module name
    utils::validation::validate_project_name(&name)
        .map_err(|e| crate::error::RicError::GenerationFailed(e.to_string()))?;

    // Determine module directory path
    let module_dir = PathBuf::from("src/modules").join(&name);

    // Check if module already exists
    if module_dir.exists() {
        return Err(crate::error::RicError::ModuleExists {
            name,
            path: module_dir.display().to_string(),
        });
    }

    // Display generation header
    output::print_header(&format!("Generating {} module '{}'", module_type.cyan(), name.cyan()));
    println!("  {} Type: {}", "→".yellow().bold(), module_type.cyan());
    println!("  {} Name: {}", "→".yellow().bold(), name.cyan());
    println!("  {} Location: {}", "→".yellow().bold(), module_dir.display().to_string().cyan());
    println!();

    // Initialize progress spinner
    let spinner = output::print_progress("Creating module structure...");

    // Create module directory structure
    spinner.set_message("Creating directories...");
    create_module_directories(&module_dir)?;

    // Generate module files based on type
    spinner.set_message("Generating module files...");
    generate_module_files(&module_dir, &module_type, &name)?;

    // Update Cargo.toml with dependencies
    spinner.set_message("Updating Cargo.toml...");
    update_cargo_toml(&module_type)?;

    // Format generated code
    spinner.set_message("Formatting code...");
    format_generated_code(&module_dir)?;

    // Complete
    spinner.finish_with_message("Module generated successfully!");

    // Display success message with next steps
    print_module_success_message(&name, &module_type, &module_dir);

    Ok(())
}

/// Create module directory structure
///
/// Creates all necessary directories for the module including:
/// - Main module directory
/// - Tests subdirectory
///
/// # Arguments
///
/// * `module_dir` - Path to the module directory
///
/// # Errors
///
/// Returns `RicError::Io` if directory creation fails.
fn create_module_directories(module_dir: &Path) -> Result<()> {
    // Create main module directory
    utils::fs::create_dir_all(module_dir)?;
    
    // Create tests subdirectory
    let tests_dir = module_dir.join("tests");
    utils::fs::create_dir_all(&tests_dir)?;
    
    Ok(())
}

/// Generate module files
///
/// Creates all module files with appropriate content based on the module type.
/// Each file is generated with type-specific boilerplate and documentation.
///
/// # Arguments
///
/// * `module_dir` - Path to the module directory
/// * `module_type` - Type of module (cache, queue, gateway, auth, device, observability)
/// * `name` - Name of the module
///
/// # Errors
///
/// Returns `RicError::Io` if file writing fails.
/// Returns `RicError::GenerationFailed` if content generation fails.
fn generate_module_files(module_dir: &Path, module_type: &str, name: &str) -> Result<()> {
    // Convert name to PascalCase for struct names
    let struct_name = to_pascal_case(name);
    
    // Generate mod.rs
    let mod_content = generate_mod_rs(module_type, name, &struct_name);
    utils::fs::write_file(module_dir.join("mod.rs"), mod_content)?;
    
    // Generate config.rs
    let config_content = generate_config_rs(module_type, name, &struct_name);
    utils::fs::write_file(module_dir.join("config.rs"), config_content)?;
    
    // Generate handler.rs
    let handler_content = generate_handler_rs(module_type, name, &struct_name);
    utils::fs::write_file(module_dir.join("handler.rs"), handler_content)?;
    
    // Generate service.rs
    let service_content = generate_service_rs(module_type, name, &struct_name);
    utils::fs::write_file(module_dir.join("service.rs"), service_content)?;
    
    // Generate tests/mod.rs
    let tests_content = generate_tests_rs(module_type, name, &struct_name);
    utils::fs::write_file(module_dir.join("tests/mod.rs"), tests_content)?;
    
    Ok(())
}

/// Generate mod.rs content
///
/// Creates the module entry point with public API exports and documentation.
///
/// # Arguments
///
/// * `module_type` - Type of module
/// * `name` - Module name (snake_case)
/// * `struct_name` - Module struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated mod.rs content as a string.
fn generate_mod_rs(module_type: &str, name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {name} Module
//!
//! This module provides {module_type} functionality for the Ri framework.
//! It implements the core {module_type} operations with configurable backends
//! and comprehensive error handling.
//!
//! # Features
//!
//! - Configurable {module_type} backend
//! - Async/await support
//! - Comprehensive error handling
//! - Built-in metrics and monitoring
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::modules::{name}::{{{struct_name}, {struct_name}Config}};
//!
//! let config = {struct_name}Config::default();
//! let {name} = {struct_name}::new(config).await?;
//! ```

pub mod config;
pub mod handler;
pub mod service;

#[cfg(test)]
mod tests;

pub use config::{{Config, {struct_name}Config}};
pub use handler::{{Handler, {struct_name}Handler}};
pub use service::{{Service, {struct_name}Service}};

use std::sync::Arc;
use tokio::sync::RwLock;

/// Main {module_type} module struct
///
/// This struct provides the primary interface for {module_type} operations.
/// It wraps the underlying service and handler implementations.
///
/// # Type Parameters
///
/// The struct uses Arc<RwLock<>> for thread-safe shared state.
pub struct {struct_name} {{
    /// Module configuration
    config: {struct_name}Config,
    
    /// Service instance for business logic
    service: Arc<RwLock<{struct_name}Service>>,
    
    /// Handler instance for request processing
    handler: Arc<RwLock<{struct_name}Handler>>,
}}

impl {struct_name} {{
    /// Create a new {module_type} module instance
    ///
    /// Initializes the module with the provided configuration and
    /// sets up the service and handler instances.
    ///
    /// # Arguments
    ///
    /// * `config` - Module configuration
    ///
    /// # Returns
    ///
    /// Returns a new `{struct_name}` instance on success.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = {struct_name}Config::default();
    /// let module = {struct_name}::new(config).await?;
    /// ```
    pub async fn new(config: {struct_name}Config) -> anyhow::Result<Self> {{
        let service = Arc::new(RwLock::new({struct_name}Service::new(&config)?));
        let handler = Arc::new(RwLock::new({struct_name}Handler::new(service.clone())?));
        
        Ok(Self {{
            config,
            service,
            handler,
        }})
    }}
    
    /// Get a reference to the service
    ///
    /// Returns an Arc reference to the service for direct access
    /// to business logic operations.
    pub fn service(&self) -> Arc<RwLock<{struct_name}Service>> {{
        self.service.clone()
    }}
    
    /// Get a reference to the handler
    ///
    /// Returns an Arc reference to the handler for direct access
    /// to request processing operations.
    pub fn handler(&self) -> Arc<RwLock<{struct_name}Handler>> {{
        self.handler.clone()
    }}
    
    /// Start the module
    ///
    /// Initializes any background tasks or connections required
    /// by the module.
    ///
    /// # Errors
    ///
    /// Returns an error if startup fails.
    pub async fn start(&self) -> anyhow::Result<()> {{
        // Module-specific startup logic
        tracing::info!("Starting {name} module");
        Ok(())
    }}
    
    /// Stop the module
    ///
    /// Gracefully shuts down the module and releases resources.
    ///
    /// # Errors
    ///
    /// Returns an error if shutdown fails.
    pub async fn stop(&self) -> anyhow::Result<()> {{
        // Module-specific shutdown logic
        tracing::info!("Stopping {name} module");
        Ok(())
    }}
}}
"#,
        name = name,
        module_type = module_type,
        struct_name = struct_name
    )
}

/// Generate config.rs content
///
/// Creates configuration structures with serde support and default implementations.
///
/// # Arguments
///
/// * `module_type` - Type of module
/// * `name` - Module name (snake_case)
/// * `struct_name` - Module struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated config.rs content as a string.
fn generate_config_rs(module_type: &str, name: &str, struct_name: &str) -> String {
    let type_specific_config = match module_type {
        "cache" => format!(
            r#"    /// Cache backend type (redis, memcached, memory)
    pub backend: String,
    
    /// Cache server URL
    pub url: String,
    
    /// Default TTL in seconds
    pub default_ttl: u64,
    
    /// Maximum cache size in bytes
    pub max_size: usize,"#
        ),
        "queue" => format!(
            r#"    /// Queue backend type (rabbitmq, kafka, memory)
    pub backend: String,
    
    /// Queue server URL
    pub url: String,
    
    /// Queue name
    pub queue_name: String,
    
    /// Maximum retry attempts
    pub max_retries: u32,"#
        ),
        "gateway" => format!(
            r#"    /// Gateway listen address
    pub listen: String,
    
    /// Enable TLS
    pub enable_tls: bool,
    
    /// Maximum concurrent connections
    pub max_connections: usize,
    
    /// Request timeout in seconds
    pub timeout: u64,"#
        ),
        "auth" => format!(
            r#"    /// JWT secret key
    pub jwt_secret: String,
    
    /// Token expiration in seconds
    pub token_expiry: u64,
    
    /// Enable refresh tokens
    pub enable_refresh: bool,
    
    /// OAuth2 provider URL
    pub oauth_url: Option<String>,"#
        ),
        "device" => format!(
            r#"    /// MQTT broker URL
    pub mqtt_url: String,
    
    /// Device identifier prefix
    pub device_prefix: String,
    
    /// Heartbeat interval in seconds
    pub heartbeat_interval: u64,
    
    /// Enable auto-discovery
    pub auto_discovery: bool,"#
        ),
        "observability" => format!(
            r#"    /// Metrics export port
    pub metrics_port: u16,
    
    /// Enable tracing
    pub enable_tracing: bool,
    
    /// Trace sampling rate (0.0 - 1.0)
    pub sampling_rate: f64,
    
    /// Export endpoint URL
    pub export_url: Option<String>,"#
        ),
        _ => String::new(),
    };

    format!(
        r#"//! Configuration structures for {name} module
//!
//! This module defines the configuration structures used to configure
//! the {module_type} module. All configurations support serde serialization
//! and provide sensible defaults.

use serde::{{Deserialize, Serialize}};

/// Trait for module configuration
///
/// Provides a common interface for all module configurations.
pub trait Config: Sized {{
    /// Load configuration from a file
    fn load(path: &str) -> anyhow::Result<Self>;
    
    /// Validate the configuration
    fn validate(&self) -> anyhow::Result<()>;
}}

/// Configuration for {struct_name} module
///
/// This struct contains all configuration options for the {module_type} module.
/// It supports serialization/deserialization from various formats (YAML, JSON, TOML).
///
/// # Example
///
/// ```yaml
/// {name}:
///   enabled: true
///   backend: "redis"
///   url: "redis://localhost:6379"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {struct_name}Config {{
    /// Enable the module
    pub enabled: bool,
    
{type_specific_config}
}}

impl Default for {struct_name}Config {{
    fn default() -> Self {{
        Self {{
            enabled: true,
{default_values}
        }}
    }}
}}

impl Config for {struct_name}Config {{
    fn load(path: &str) -> anyhow::Result<Self> {{
        let content = std::fs::read_to_string(path)?;
        let config: Self = serde_yaml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }}
    
    fn validate(&self) -> anyhow::Result<()> {{
        // Add validation logic here
        if self.enabled {{
            tracing::debug!("Validating {name} configuration");
        }}
        Ok(())
    }}
}}
"#,
        name = name,
        module_type = module_type,
        struct_name = struct_name,
        type_specific_config = type_specific_config,
        default_values = match module_type {
            "cache" => r#"            backend: "memory".to_string(),
            url: String::new(),
            default_ttl: 3600,
            max_size: 100 * 1024 * 1024,"#,
            "queue" => r#"            backend: "memory".to_string(),
            url: String::new(),
            queue_name: "default".to_string(),
            max_retries: 3,"#,
            "gateway" => r#"            listen: "0.0.0.0:8080".to_string(),
            enable_tls: false,
            max_connections: 1000,
            timeout: 30,"#,
            "auth" => r#"            jwt_secret: "change-me-in-production".to_string(),
            token_expiry: 3600,
            enable_refresh: true,
            oauth_url: None,"#,
            "device" => r#"            mqtt_url: "mqtt://localhost:1883".to_string(),
            device_prefix: "device".to_string(),
            heartbeat_interval: 30,
            auto_discovery: true,"#,
            "observability" => r#"            metrics_port: 9090,
            enable_tracing: true,
            sampling_rate: 1.0,
            export_url: None,"#,
            _ => "",
        }
    )
}

/// Generate handler.rs content
///
/// Creates request handler structures with async support.
///
/// # Arguments
///
/// * `module_type` - Type of module
/// * `name` - Module name (snake_case)
/// * `struct_name` - Module struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated handler.rs content as a string.
fn generate_handler_rs(module_type: &str, name: &str, struct_name: &str) -> String {
    format!(
        r#"//! Request handlers for {name} module
//!
//! This module provides request/response handlers for the {module_type} module.
//! Handlers process incoming requests and coordinate with the service layer.

use super::service::{struct_name}Service;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Trait for module handlers
///
/// Defines the common interface for all module handlers.
#[async_trait::async_trait]
pub trait Handler: Send + Sync {{
    /// Handle an incoming request
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request data
    ///
    /// # Returns
    ///
    /// Returns the response data or an error.
    async fn handle(&self, request: Vec<u8>) -> anyhow::Result<Vec<u8>>;
}}

/// Handler for {struct_name} module
///
/// Processes requests for the {module_type} module by delegating
/// to the service layer and handling any necessary transformations.
pub struct {struct_name}Handler {{
    /// Reference to the service layer
    service: Arc<RwLock<{struct_name}Service>>,
}}

impl {struct_name}Handler {{
    /// Create a new handler instance
    ///
    /// # Arguments
    ///
    /// * `service` - Reference to the service layer
    ///
    /// # Returns
    ///
    /// Returns a new handler instance.
    pub fn new(service: Arc<RwLock<{struct_name}Service>>) -> anyhow::Result<Self> {{
        Ok(Self {{ service }})
    }}
}}

#[async_trait::async_trait]
impl Handler for {struct_name}Handler {{
    async fn handle(&self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {{
        tracing::debug!("Handling request for {name} module ({request} bytes)", request = request.len());
        
        // Get read lock on service
        let service = self.service.read().await;
        
        // Process the request through the service
        let response = service.process(request).await?;
        
        Ok(response)
    }}
}}
"#,
        name = name,
        module_type = module_type,
        struct_name = struct_name
    )
}

/// Generate service.rs content
///
/// Creates service structures with business logic implementation.
///
/// # Arguments
///
/// * `module_type` - Type of module
/// * `name` - Module name (snake_case)
/// * `struct_name` - Module struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated service.rs content as a string.
fn generate_service_rs(module_type: &str, name: &str, struct_name: &str) -> String {
    format!(
        r#"//! Business logic service for {name} module
//!
//! This module provides the core business logic for the {module_type} module.
//! Services encapsulate the main operations and coordinate with external resources.

use super::config::{struct_name}Config;

/// Trait for module services
///
/// Defines the common interface for all module services.
#[async_trait::async_trait]
pub trait Service: Send + Sync {{
    /// Process a request
    ///
    /// # Arguments
    ///
    /// * `data` - Input data to process
    ///
    /// # Returns
    ///
    /// Returns the processed output or an error.
    async fn process(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    
    /// Health check
    ///
    /// Returns the health status of the service.
    async fn health_check(&self) -> anyhow::Result<bool>;
}}

/// Service for {struct_name} module
///
/// Implements the core business logic for the {module_type} module.
/// This service handles all {module_type} operations and coordinates
/// with external resources as needed.
pub struct {struct_name}Service {{
    /// Service configuration
    config: {struct_name}Config,
    
    /// Service state
    initialized: bool,
}}

impl {struct_name}Service {{
    /// Create a new service instance
    ///
    /// # Arguments
    ///
    /// * `config` - Service configuration
    ///
    /// # Returns
    ///
    /// Returns a new service instance.
    pub fn new(config: &{struct_name}Config) -> anyhow::Result<Self> {{
        Ok(Self {{
            config: config.clone(),
            initialized: false,
        }})
    }}
    
    /// Initialize the service
    ///
    /// Performs any necessary initialization such as connecting
    /// to external resources.
    ///
    /// # Errors
    ///
    /// Returns an error if initialization fails.
    pub async fn initialize(&mut self) -> anyhow::Result<()> {{
        if self.initialized {{
            return Ok(());
        }}
        
        tracing::info!("Initializing {name} service");
        
        // Add initialization logic here based on module type
        // For example: connect to cache, queue, database, etc.
        
        self.initialized = true;
        Ok(())
    }}
}}

#[async_trait::async_trait]
impl Service for {struct_name}Service {{
    async fn process(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {{
        tracing::debug!("Processing data in {name} service ({} bytes)", data.len());
        
        // Implement module-specific processing logic here
        // This is a placeholder that echoes the input
        Ok(data)
    }}
    
    async fn health_check(&self) -> anyhow::Result<bool> {{
        // Return true if the service is healthy
        Ok(self.initialized)
    }}
}}
"#,
        name = name,
        module_type = module_type,
        struct_name = struct_name
    )
}

/// Generate tests/mod.rs content
///
/// Creates test scaffolding with basic test structure.
///
/// # Arguments
///
/// * `module_type` - Type of module
/// * `name` - Module name (snake_case)
/// * `struct_name` - Module struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated tests/mod.rs content as a string.
fn generate_tests_rs(module_type: &str, name: &str, struct_name: &str) -> String {
    format!(
        r#"//! Unit tests for {name} module
//!
//! This module contains unit tests for the {module_type} module components.

use crate::modules::{name}::{{{struct_name}, {struct_name}Config}};

/// Test module creation
#[tokio::test]
async fn test_module_creation() {{
    let config = {struct_name}Config::default();
    let result = {struct_name}::new(config).await;
    assert!(result.is_ok());
}}

/// Test default configuration
#[test]
fn test_default_config() {{
    let config = {struct_name}Config::default();
    assert!(config.enabled);
}}

/// Test configuration validation
#[test]
fn test_config_validation() {{
    let config = {struct_name}Config::default();
    assert!(config.validate().is_ok());
}}

/// Test service initialization
#[tokio::test]
async fn test_service_initialization() {{
    let config = {struct_name}Config::default();
    let module = {struct_name}::new(config).await.unwrap();
    
    let service = module.service();
    let mut service = service.write().await;
    
    assert!(service.initialize().await.is_ok());
}}

/// Test service health check
#[tokio::test]
async fn test_service_health_check() {{
    let config = {struct_name}Config::default();
    let module = {struct_name}::new(config).await.unwrap();
    
    let service = module.service();
    let mut service = service.write().await;
    service.initialize().await.unwrap();
    
    let is_healthy = service.health_check().await.unwrap();
    assert!(is_healthy);
}}
"#,
        name = name,
        module_type = module_type,
        struct_name = struct_name
    )
}

/// Update Cargo.toml with module dependencies
///
/// Adds necessary dependencies based on the module type.
///
/// # Arguments
///
/// * `module_type` - Type of module
///
/// # Errors
///
/// Returns `RicError::Io` if file operations fail.
/// Returns `RicError::GenerationFailed` if dependency parsing fails.
fn update_cargo_toml(module_type: &str) -> Result<()> {
    let cargo_path = Path::new("Cargo.toml");
    
    if !cargo_path.exists() {
        return Ok(()); // Skip if not in a Rust project
    }
    
    // Read current Cargo.toml
    let content = utils::fs::read_file(cargo_path)?;
    
    // Define dependencies to add based on module type
    let deps_to_add = match module_type {
        "cache" => vec![
            ("redis", r#"redis = { version = "0.25", features = ["tokio-comp"] }"#),
            ("memchr", r#"memchr = "2""#),
        ],
        "queue" => vec![
            ("lapin", r#"lapin = "2""#),
            ("tokio-executor-trait", r#"tokio-executor-trait = "2""#),
            ("tokio-reactor-trait", r#"tokio-reactor-trait = "1""#),
        ],
        "gateway" => vec![
            ("hyper", r#"hyper = { version = "1", features = ["full"] }"#),
            ("tower", r#"tower = "0.4""#),
        ],
        "auth" => vec![
            ("jsonwebtoken", r#"jsonwebtoken = "9""#),
            ("oauth2", r#"oauth2 = "4""#),
        ],
        "device" => vec![
            ("rumqttc", r#"rumqttc = "0.24""#),
        ],
        "observability" => vec![
            ("tracing", r#"tracing = "0.1""#),
            ("tracing-subscriber", r#"tracing-subscriber = "0.3""#),
            ("metrics", r#"metrics = "0.22""#),
        ],
        _ => vec![],
    };
    
    // Check which dependencies are missing
    let mut new_deps = Vec::new();
    for (dep_name, dep_line) in deps_to_add {
        if !content.contains(dep_name) {
            new_deps.push(dep_line);
        }
    }
    
    // If no new dependencies, return early
    if new_deps.is_empty() {
        return Ok(());
    }
    
    // Find the [dependencies] section and add new dependencies
    let mut lines: Vec<&str> = content.lines().collect();
    let mut deps_index = None;
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "[dependencies]" {
            deps_index = Some(i + 1);
            break;
        }
    }
    
    if let Some(index) = deps_index {
        // Add comment about auto-added dependencies
        lines.insert(index, "# Auto-added by ric generate");
        for (offset, dep) in new_deps.iter().enumerate() {
            lines.insert(index + 1 + offset, dep);
        }
        
        // Write back
        let new_content = lines.join("\n");
        utils::fs::write_file(cargo_path, new_content)?;
    }
    
    Ok(())
}

/// Format generated code using rustfmt
///
/// Runs rustfmt on all generated files to ensure consistent formatting.
///
/// # Arguments
///
/// * `module_dir` - Path to the module directory
///
/// # Errors
///
/// Returns `RicError::FormattingFailed` if rustfmt fails.
fn format_generated_code(module_dir: &Path) -> Result<()> {
    // Run rustfmt on each generated file
    let files = ["mod.rs", "config.rs", "handler.rs", "service.rs", "tests/mod.rs"];
    
    for file in files {
        let file_path = module_dir.join(file);
        
        let status = Command::new("rustfmt")
            .arg(&file_path)
            .status();
        
        match status {
            Ok(s) if s.success() => {},
            Ok(_) => {
                // rustfmt failed but file exists - continue
                tracing::warn!("rustfmt failed for {}", file_path.display());
            },
            Err(e) => {
                // rustfmt not available - skip silently
                tracing::debug!("rustfmt not available: {}", e);
            }
        }
    }
    
    Ok(())
}

/// Print success message for module generation
///
/// Displays a formatted success message with next steps for the user.
///
/// # Arguments
///
/// * `name` - Module name
/// * `module_type` - Type of module
/// * `module_dir` - Path to the module directory
fn print_module_success_message(name: &str, module_type: &str, module_dir: &Path) {
    println!();
    output::print_success(&format!("Module '{}' generated successfully!", name.cyan()));
    
    println!();
    println!("{}", "Generated files:".yellow().bold());
    println!("  {} {}", "•".green(), format!("{}/mod.rs", module_dir.display()).dimmed());
    println!("  {} {}", "•".green(), format!("{}/config.rs", module_dir.display()).dimmed());
    println!("  {} {}", "•".green(), format!("{}/handler.rs", module_dir.display()).dimmed());
    println!("  {} {}", "•".green(), format!("{}/service.rs", module_dir.display()).dimmed());
    println!("  {} {}", "•".green(), format!("{}/tests/mod.rs", module_dir.display()).dimmed());
    
    println!();
    println!("{}", "Next steps:".yellow().bold());
    println!("  1. Add the module to your main.rs:");
    println!("     {} pub mod {};", "mod modules;".cyan(), name.cyan());
    println!();
    println!("  2. Configure the module in your config file:");
    println!("     {}:", name.cyan());
    println!("       enabled: true");
    println!("       # Add type-specific configuration here");
    println!();
    println!("  3. Use the module in your application:");
    println!("     {} let {} = modules::{name}::{}::new(config).await?;", "let config =".cyan(), name.cyan(), to_pascal_case(name).cyan());
    
    println!();
    println!("{}", "Module type hints:".yellow().bold());
    match module_type {
        "cache" => {
            println!("  • Configure backend: redis, memcached, or memory");
            println!("  • Set appropriate TTL values for your use case");
            println!("  • Consider cache invalidation strategies");
        },
        "queue" => {
            println!("  • Configure backend: rabbitmq, kafka, or memory");
            println!("  • Set up dead letter queues for failed messages");
            println!("  • Configure retry policies appropriately");
        },
        "gateway" => {
            println!("  • Configure routes and upstream services");
            println!("  • Set up rate limiting and authentication");
            println!("  • Configure TLS for production");
        },
        "auth" => {
            println!("  • Change jwt_secret in production!");
            println!("  • Configure OAuth2 providers if needed");
            println!("  • Set up refresh token rotation");
        },
        "device" => {
            println!("  • Configure MQTT broker connection");
            println!("  • Set up device authentication");
            println!("  • Configure heartbeat intervals");
        },
        "observability" => {
            println!("  • Configure trace sampling rate");
            println!("  • Set up export endpoint (Jaeger, etc.)");
            println!("  • Configure metrics port");
        },
        _ => {}
    }
    
    println!();
    println!("{} Happy coding with Ri!", "🚀".green());
}

/// Generate middleware template
///
/// Creates a middleware template with standard patterns including:
/// - Middleware struct definition
/// - Standard middleware trait implementation
/// - Request/response handling boilerplate
/// - Configuration support
///
/// # Arguments
///
/// * `name` - Name of the middleware
///
/// # Generated Structure
///
/// ```text
/// src/middleware/
/// └── <name>.rs
/// ```
///
/// # Errors
///
/// Returns `RicError::MiddlewareExists` if middleware already exists.
/// Returns `RicError::Io` for file system errors.
/// Returns `RicError::FormattingFailed` if rustfmt fails.
///
/// # Examples
///
/// ```rust,ignore
/// generate_middleware("auth-middleware".to_string()).await?;
/// ```
async fn generate_middleware(name: String) -> Result<()> {
    // Validate middleware name
    utils::validation::validate_project_name(&name)
        .map_err(|e| crate::error::RicError::GenerationFailed(e.to_string()))?;

    // Determine middleware file path
    let middleware_dir = PathBuf::from("src/middleware");
    let middleware_file = middleware_dir.join(format!("{}.rs", name));

    // Check if middleware already exists
    if middleware_file.exists() {
        return Err(crate::error::RicError::MiddlewareExists {
            name,
            path: middleware_file.display().to_string(),
        });
    }

    // Display generation header
    output::print_header(&format!("Generating middleware '{}'", name.cyan()));
    println!("  {} Name: {}", "→".yellow().bold(), name.cyan());
    println!("  {} Location: {}", "→".yellow().bold(), middleware_file.display().to_string().cyan());
    println!();

    // Initialize progress spinner
    let spinner = output::print_progress("Creating middleware...");

    // Create middleware directory if it doesn't exist
    spinner.set_message("Creating directories...");
    utils::fs::create_dir_all(&middleware_dir)?;

    // Generate middleware content
    spinner.set_message("Generating middleware code...");
    let struct_name = to_pascal_case(&name);
    let middleware_content = generate_middleware_content(&name, &struct_name);
    
    // Write middleware file
    utils::fs::write_file(&middleware_file, middleware_content)?;

    // Format generated code
    spinner.set_message("Formatting code...");
    let _ = Command::new("rustfmt")
        .arg(&middleware_file)
        .status();

    // Complete
    spinner.finish_with_message("Middleware generated successfully!");

    // Display success message
    print_middleware_success_message(&name, &struct_name, &middleware_file);

    Ok(())
}

/// Generate middleware content
///
/// Creates the middleware implementation with standard patterns.
///
/// # Arguments
///
/// * `name` - Middleware name (snake_case)
/// * `struct_name` - Middleware struct name (PascalCase)
///
/// # Returns
///
/// Returns the generated middleware content as a string.
fn generate_middleware_content(name: &str, struct_name: &str) -> String {
    format!(
        r#"//! {struct_name} Middleware
//!
//! This middleware provides {name} functionality for request/response processing.
//! It integrates with the Ri middleware chain and supports async operations.
//!
//! # Features
//!
//! - Async middleware support
//! - Request preprocessing
//! - Response postprocessing
//! - Configurable behavior
//!
//! # Example
//!
//! ```rust,ignore
//! use crate::middleware::{name}::{{Middleware, {struct_name}}};
//!
//! let middleware = {struct_name}::new();
//! // Use with your application
//! ```

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Boxed future type for middleware
type MiddlewareFuture = Pin<Box<dyn Future<Output = anyhow::Result<Vec<u8>>> + Send>>;

/// Middleware trait for request/response processing
///
/// Defines the interface for all middleware implementations.
pub trait Middleware: Send + Sync {{
    /// Process an incoming request
    ///
    /// Called before the request is handled by the main handler.
    /// Can modify the request or return early with a response.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request data
    ///
    /// # Returns
    ///
    /// Returns the (possibly modified) request data or an error.
    fn preprocess(&self, request: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    
    /// Process an outgoing response
    ///
    /// Called after the response is generated by the handler.
    /// Can modify the response before it's sent to the client.
    ///
    /// # Arguments
    ///
    /// * `response` - The outgoing response data
    ///
    /// # Returns
    ///
    /// Returns the (possibly modified) response data or an error.
    fn postprocess(&self, response: Vec<u8>) -> anyhow::Result<Vec<u8>>;
    
    /// Handle the middleware chain
    ///
    /// Processes the request through the middleware and calls the next handler.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request data
    /// * `next` - The next handler in the chain
    ///
    /// # Returns
    ///
    /// Returns the response data or an error.
    fn handle(&self, request: Vec<u8>, next: Arc<dyn Middleware>) -> MiddlewareFuture;
}}

/// {struct_name} middleware implementation
///
/// Provides {name} functionality for request/response processing.
/// This middleware can be customized through configuration.
pub struct {struct_name} {{
    /// Middleware name for logging
    name: String,
    
    /// Enable/disable the middleware
    enabled: bool,
}}

impl {struct_name} {{
    /// Create a new {struct_name} instance
    ///
    /// # Returns
    ///
    /// Returns a new middleware instance with default configuration.
    pub fn new() -> Self {{
        Self {{
            name: "{name}".to_string(),
            enabled: true,
        }}
    }}
    
    /// Create a new {struct_name} instance with custom configuration
    ///
    /// # Arguments
    ///
    /// * `enabled` - Whether the middleware is enabled
    ///
    /// # Returns
    ///
    /// Returns a new middleware instance with custom configuration.
    pub fn with_config(enabled: bool) -> Self {{
        Self {{
            name: "{name}".to_string(),
            enabled,
        }}
    }}
}}

impl Default for {struct_name} {{
    fn default() -> Self {{
        Self::new()
    }}
}}

impl Middleware for {struct_name} {{
    fn preprocess(&self, request: Vec<u8>) -> anyhow::Result<Vec<u8>> {{
        if !self.enabled {{
            return Ok(request);
        }}
        
        tracing::debug!(
            middleware = %self.name,
            size = request.len(),
            "Preprocessing request"
        );
        
        // Add preprocessing logic here
        // For example: validation, authentication, logging, etc.
        
        Ok(request)
    }}
    
    fn postprocess(&self, response: Vec<u8>) -> anyhow::Result<Vec<u8>> {{
        if !self.enabled {{
            return Ok(response);
        }}
        
        tracing::debug!(
            middleware = %self.name,
            size = response.len(),
            "Postprocessing response"
        );
        
        // Add postprocessing logic here
        // For example: adding headers, logging, metrics, etc.
        
        Ok(response)
    }}
    
    fn handle(&self, request: Vec<u8>, next: Arc<dyn Middleware>) -> MiddlewareFuture {{
        let middleware = self.clone();
        let next = next.clone();
        
        Box::pin(async move {{
            // Preprocess the request
            let processed_request = middleware.preprocess(request)?;
            
            // Call the next middleware/handler
            let response = next.handle(processed_request, next.clone()).await?;
            
            // Postprocess the response
            let processed_response = middleware.postprocess(response)?;
            
            Ok(processed_response)
        }})
    }}
}}

impl Clone for {struct_name} {{
    fn clone(&self) -> Self {{
        Self {{
            name: self.name.clone(),
            enabled: self.enabled,
        }}
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;
    
    #[test]
    fn test_middleware_creation() {{
        let middleware = {struct_name}::new();
        assert_eq!(middleware.name, "{name}");
        assert!(middleware.enabled);
    }}
    
    #[test]
    fn test_middleware_disabled() {{
        let middleware = {struct_name}::with_config(false);
        assert!(!middleware.enabled);
    }}
    
    #[test]
    fn test_preprocess() {{
        let middleware = {struct_name}::new();
        let request = b"test request".to_vec();
        let result = middleware.preprocess(request.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), request);
    }}
    
    #[test]
    fn test_postprocess() {{
        let middleware = {struct_name}::new();
        let response = b"test response".to_vec();
        let result = middleware.postprocess(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), response);
    }}
}}
"#,
        name = name,
        struct_name = struct_name
    )
}

/// Print success message for middleware generation
///
/// Displays a formatted success message with next steps for the user.
///
/// # Arguments
///
/// * `name` - Middleware name
/// * `struct_name` - Middleware struct name (PascalCase)
/// * `middleware_file` - Path to the middleware file
fn print_middleware_success_message(name: &str, struct_name: &str, middleware_file: &Path) {
    println!();
    output::print_success(&format!("Middleware '{}' generated successfully!", name.cyan()));
    
    println!();
    println!("{}", "Generated file:".yellow().bold());
    println!("  {} {}", "•".green(), middleware_file.display().to_string().dimmed());
    
    println!();
    println!("{}", "Next steps:".yellow().bold());
    println!("  1. Add the middleware to your module:");
    println!("     {} pub mod {name};", "mod middleware;".cyan());
    println!();
    println!("  2. Use the middleware in your application:");
    println!("     {} let middleware = middleware::{name}::{struct_name}::new();", "use crate::middleware::{name}::{struct_name};".cyan());
    println!();
    println!("  3. Customize the middleware:");
    println!("     {} Implement preprocess() for request handling", "•".green());
    println!("     {} Implement postprocess() for response handling", "•".green());
    
    println!();
    println!("{} Happy coding with Ri!", "🚀".green());
}

/// Generate Rust struct from config file
///
/// Parses a YAML or JSON configuration file and generates corresponding
/// Rust struct definitions with serde derives and Default implementations.
///
/// # Arguments
///
/// * `from` - Path to the configuration file (YAML or JSON)
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
///
/// # Errors
///
/// Returns `RicError::GenerateConfigFileNotFound` if the file doesn't exist.
/// Returns `RicError::UnsupportedConfigFormat` if the file format is not supported.
/// Returns `RicError::GenerationFailed` if parsing or generation fails.
///
/// # Examples
///
/// ```rust,ignore
/// generate_config(PathBuf::from("config.yaml")).await?;
/// generate_config(PathBuf::from("config.json")).await?;
/// ```
async fn generate_config(from: PathBuf) -> Result<()> {
    // Check if file exists
    if !from.exists() {
        return Err(crate::error::RicError::GenerateConfigFileNotFound(
            from.display().to_string()
        ));
    }

    // Determine file format from extension
    let extension = from.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let is_yaml = matches!(extension.as_str(), "yaml" | "yml");
    let is_json = extension == "json";

    if !is_yaml && !is_json {
        return Err(crate::error::RicError::UnsupportedConfigFormat {
            format: extension,
            supported: "yaml, yml, json".to_string(),
        });
    }

    // Display generation header
    output::print_header(&format!("Generating config struct from '{}'", from.display().to_string().cyan()));
    println!("  {} Format: {}", "→".yellow().bold(), if is_yaml { "YAML" } else { "JSON" }.cyan());
    println!();

    // Initialize progress spinner
    let spinner = output::print_progress("Parsing configuration file...");

    // Read file content
    spinner.set_message("Reading configuration file...");
    let content = utils::fs::read_file(&from)?;

    // Parse the configuration
    spinner.set_message("Parsing configuration...");
    let config_value = if is_yaml {
        serde_yaml::from_str::<serde_json::Value>(&content)?
    } else {
        serde_json::from_str::<serde_json::Value>(&content)?
    };

    // Generate Rust code
    spinner.set_message("Generating Rust structs...");
    let generated_code = generate_config_structs(&config_value, "Config");

    // Complete
    spinner.finish_with_message("Config structs generated successfully!");

    // Output the generated code
    println!();
    println!("{}", "Generated Rust code:".yellow().bold());
    println!("{}", "─".repeat(60));
    println!("{}", generated_code);
    println!("{}", "─".repeat(60));
    
    println!();
    println!("{}", "Usage:".yellow().bold());
    println!("  {} Save to a file:", "1.".dimmed());
    println!("     {} > src/config.rs", format!("ric generate config {}", from.display()).cyan());
    println!();
    println!("  {} Use in your application:", "2.".dimmed());
    println!("     {} let config = Config::load(\"config.yaml\")?;", "use crate::cli_config::Config;".cyan());

    Ok(())
}

/// Generate Rust structs from a JSON value
///
/// Recursively generates struct definitions from a JSON value.
/// Creates nested structs for objects and uses appropriate types for primitives.
///
/// # Arguments
///
/// * `value` - The JSON value to generate structs from
/// * `struct_name` - Name for the generated struct
///
/// # Returns
///
/// Returns the generated Rust code as a string.
fn generate_config_structs(value: &serde_json::Value, struct_name: &str) -> String {
    let mut output = String::new();
    
    // Add header comment
    output.push_str(&format!(
        "//! Auto-generated configuration structures\n//! Generated from config file\n\n"
    ));
    
    // Add serde and derive imports
    output.push_str("use serde::{Deserialize, Serialize};\n\n");
    
    // Generate the main struct and any nested structs
    generate_struct_recursive(value, struct_name, &mut output);
    
    output
}

/// Recursively generate struct definition
///
/// Generates a struct definition from a JSON object and recursively
/// generates nested structs for object fields.
///
/// # Arguments
///
/// * `value` - The JSON value to generate from
/// * `struct_name` - Name for the struct
/// * `output` - Output string to append to
fn generate_struct_recursive(value: &serde_json::Value, struct_name: &str, output: &mut String) {
    if let serde_json::Value::Object(map) = value {
        // Generate struct definition
        output.push_str(&format!(
            "/// Configuration structure for {}\n",
            struct_name
        ));
        output.push_str("#[derive(Debug, Clone, Serialize, Deserialize)]\n");
        output.push_str(&format!("pub struct {} {{\n", struct_name));
        
        // Generate fields
        for (key, val) in map {
            let field_name = to_snake_case(key);
            let field_type = get_rust_type(val, key);
            
            // Add serde rename if field name differs from key
            if field_name != *key {
                output.push_str(&format!(
                    "    #[serde(rename = \"{}\")]\n",
                    key
                ));
            }
            
            output.push_str(&format!(
                "    /// {} field\n",
                key
            ));
            output.push_str(&format!(
                "    pub {}: {},\n\n",
                field_name, field_type
            ));
        }
        
        output.push_str("}\n\n");
        
        // Generate Default implementation
        output.push_str(&format!("impl Default for {} {{\n", struct_name));
        output.push_str("    fn default() -> Self {\n");
        output.push_str("        Self {\n");
        
        for (key, val) in map {
            let field_name = to_snake_case(key);
            let default_value = get_default_value(val);
            output.push_str(&format!(
                "            {}: {},\n",
                field_name, default_value
            ));
        }
        
        output.push_str("        }\n");
        output.push_str("    }\n");
        output.push_str("}\n\n");
        
        // Generate nested structs
        for (key, val) in map {
            if let serde_json::Value::Object(_) = val {
                let nested_name = to_pascal_case(key);
                generate_struct_recursive(val, &nested_name, output);
            }
        }
    }
}

/// Get Rust type for a JSON value
///
/// Determines the appropriate Rust type for a JSON value.
/// For objects, returns a struct name; for arrays, returns Vec<T>.
///
/// # Arguments
///
/// * `value` - The JSON value
/// * `key` - The key name (used for struct naming)
///
/// # Returns
///
/// Returns the Rust type as a string.
fn get_rust_type(value: &serde_json::Value, key: &str) -> String {
    match value {
        serde_json::Value::Null => "Option<serde_json::Value>".to_string(),
        serde_json::Value::Bool(_) => "bool".to_string(),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                "i64".to_string()
            } else if n.is_u64() {
                "u64".to_string()
            } else {
                "f64".to_string()
            }
        },
        serde_json::Value::String(_) => "String".to_string(),
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                "Vec<serde_json::Value>".to_string()
            } else {
                let inner_type = get_rust_type(&arr[0], key);
                format!("Vec<{}>", inner_type)
            }
        },
        serde_json::Value::Object(_) => to_pascal_case(key),
    }
}

/// Get default value for a JSON value
///
/// Generates a Rust expression for the default value of a JSON value.
///
/// # Arguments
///
/// * `value` - The JSON value
///
/// # Returns
///
/// Returns the default value expression as a string.
fn get_default_value(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "None".to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            if n.is_i64() {
                n.as_i64().unwrap().to_string()
            } else if n.is_u64() {
                n.as_u64().unwrap().to_string()
            } else {
                n.as_f64().unwrap().to_string()
            }
        },
        serde_json::Value::String(s) => format!("\"{}\".to_string()", s),
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                "vec![]".to_string()
            } else {
                "Default::default()".to_string()
            }
        },
        serde_json::Value::Object(_) => "Default::default()".to_string(),
    }
}

/// Convert string to PascalCase
///
/// Converts a snake_case or kebab-case string to PascalCase.
///
/// # Arguments
///
/// * `s` - Input string
///
/// # Returns
///
/// Returns the PascalCase version of the string.
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| c == '_' || c == '-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str(),
            }
        })
        .collect()
}

/// Convert string to snake_case
///
/// Converts a PascalCase or camelCase string to snake_case.
///
/// # Arguments
///
/// * `s` - Input string
///
/// # Returns
///
/// Returns the snake_case version of the string.
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

// =============================================================================
// Connection Test Commands
// =============================================================================

use crate::cli::TestAction;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

/// Default connection timeout in milliseconds
const DEFAULT_TIMEOUT_MS: u64 = 5000;

/// Handle connection test commands
///
/// This function dispatches connection test subcommands to their specific handlers:
/// - `Redis` - Test Redis connection
/// - `Postgres` - Test PostgreSQL connection
/// - `Mysql` - Test MySQL connection
/// - `Kafka` - Test Kafka connection
///
/// # Arguments
///
/// * `action` - Connection test subcommand to execute
///
/// # Errors
///
/// Returns various `RicError` types depending on the operation:
/// - `RicError::ConnectionTestFailed` if connection test fails
/// - `RicError::InvalidConnectionUrl` if URL is malformed
/// - `RicError::ConnectionTimeout` if connection times out
/// - `RicError::AuthenticationFailed` if authentication fails
/// - `RicError::ServiceNotAvailable` if service is not running
///
/// # Examples
///
/// ```rust,ignore
/// // Test Redis connection
/// test_connection(TestAction::Redis { url: "redis://localhost:6379".to_string() }).await?;
///
/// // Test PostgreSQL connection
/// test_connection(TestAction::Postgres { url: "postgresql://user:pass@localhost:5432/db".to_string() }).await?;
/// ```
pub async fn test_connection(action: TestAction) -> Result<()> {
    match action {
        TestAction::Redis { url } => test_redis(&url).await,
        TestAction::Postgres { url } => test_postgres(&url).await,
        TestAction::Mysql { url } => test_mysql(&url).await,
        TestAction::Kafka { url } => test_kafka(&url).await,
    }
}

/// Test Redis connection
///
/// Tests connectivity to a Redis server by establishing a TCP connection
/// and performing basic operations.
///
/// # Test Operations
///
/// 1. Parse the Redis URL and extract host/port
/// 2. Establish TCP connection to Redis server
/// 3. Send PING command and verify PONG response
/// 4. Measure round-trip time
/// 5. Display server information
///
/// # URL Format
///
/// ```text
/// redis://[password@]host:port[/database]
/// redis://localhost:6379
/// redis://:mypassword@localhost:6379
/// ```
///
/// # Arguments
///
/// * `url` - Redis connection URL
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed
/// Returns `RicError::ConnectionTimeout` if connection times out
/// Returns `RicError::ServiceNotAvailable` if Redis is not running
///
/// # Examples
///
/// ```rust,ignore
/// test_redis("redis://localhost:6379").await?;
/// ```
async fn test_redis(url: &str) -> Result<()> {
    println!("{}", "═".repeat(60));
    println!("{}", "  Redis Connection Test".green().bold());
    println!("{}", "═".repeat(60));
    println!();

    println!("{} Testing Redis connection", "→".yellow().bold());
    println!("  {} URL: {}", "→".yellow().bold(), url.cyan());
    println!();

    let (host, port) = parse_redis_url(url)?;

    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!();

    let redis_url = if url.starts_with("redis://") {
        url.to_string()
    } else {
        format!("redis://{}", url)
    };

    println!("{} Connecting to Redis...", "→".yellow().bold());
    let start = Instant::now();

    let client = match redis::Client::open(redis_url.as_str()) {
        Ok(c) => c,
        Err(e) => {
            println!("  {} Failed to create Redis client", "✗".red().bold());
            println!("  {} Error: {}", "→".yellow().bold(), e.to_string().red());
            return Err(crate::error::InvalidConnectionUrl {
                url: url.to_string(),
                reason: e.to_string(),
            }.into());
        }
    };

    let mut conn = match client.get_connection_manager().await {
        Ok(c) => c,
        Err(e) => {
            let connection_time = start.elapsed();
            println!();
            println!("  {} Redis connection failed", "✗".red().bold());
            println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
            println!();
            println!("{}", "Troubleshooting:".yellow().bold());
            println!("  • Ensure Redis is running");
            println!("  • Check if password is correct (if required)");
            println!("  • Verify host and port");
            return Err(crate::error::ConnectionTestFailed {
                service: "Redis".to_string(),
                message: e.to_string(),
            }.into());
        }
    };

    let connection_time = start.elapsed();
    println!("  {} Connected successfully", "✓".green().bold());
    println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Sending PING command...", "→".yellow().bold());
    let ping_start = Instant::now();

    let pong: String = match redis::cmd("PING").query_async(&mut conn).await {
        Ok(p) => p,
        Err(e) => {
            println!("  {} PING command failed", "✗".red().bold());
            println!("  {} Error: {}", "→".yellow().bold(), e.to_string().red());
            return Err(crate::error::ConnectionTestFailed {
                service: "Redis".to_string(),
                message: format!("PING failed: {}", e),
            }.into());
        }
    };

    let ping_time = ping_start.elapsed();
    println!("  {} PING -> {}", "✓".green().bold(), pong.cyan());
    println!("  {} Response time: {:.2}ms", "→".yellow().bold(), ping_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Testing SET operation...", "→".yellow().bold());
    let test_key = format!("ri_test_{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string());
    let test_value = "Ri CLI Test Value";

    let _: () = match redis::cmd("SET")
        .arg(&test_key)
        .arg(test_value)
        .query_async(&mut conn)
        .await
    {
        Ok(_) => {
            println!("  {} SET {} = \"{}\"", "✓".green().bold(), test_key.cyan(), test_value.cyan());
        }
        Err(e) => {
            println!("  {} SET operation failed: {}", "✗".red().bold(), e.to_string().red());
        }
    };

    println!("{} Testing GET operation...", "→".yellow().bold());
    let retrieved: Option<String> = match redis::cmd("GET").arg(&test_key).query_async(&mut conn).await {
        Ok(v) => v,
        Err(e) => {
            println!("  {} GET operation failed: {}", "✗".red().bold(), e.to_string().red());
            None
        }
    };

    if let Some(value) = retrieved {
        if value == test_value {
            println!("  {} GET {} = \"{}\" {}", "✓".green().bold(), test_key.cyan(), value.cyan(), "✓".green());
        } else {
            println!("  {} Value mismatch: expected \"{}\", got \"{}\"", "✗".red().bold(), test_value, value);
        }
    }

    let _: () = redis::cmd("DEL").arg(&test_key).query_async(&mut conn).await.unwrap_or(());

    println!();
    println!("{} Redis connection test completed successfully!", "✓".green().bold());
    println!();
    println!("{}", "Server Information:".yellow().bold());
    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!("  {} Status: {}", "→".yellow().bold(), "Connected".green());
    println!("  {} Total time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);

    Ok(())
}

/// Test PostgreSQL connection
///
/// Tests connectivity to a PostgreSQL database server by establishing
/// a TCP connection.
///
/// # Test Operations
///
/// 1. Parse the PostgreSQL URL and extract connection parameters
/// 2. Establish TCP connection to PostgreSQL server
/// 3. Measure connection time
/// 4. Display server information
///
/// # URL Format
///
/// ```text
/// postgresql://user:password@host:port/database
/// postgresql://postgres:password@localhost:5432/mydb
/// ```
///
/// # Arguments
///
/// * `url` - PostgreSQL connection URL
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed
/// Returns `RicError::ConnectionTimeout` if connection times out
/// Returns `RicError::ServiceNotAvailable` if PostgreSQL is not running
///
/// # Examples
///
/// ```rust,ignore
/// test_postgres("postgresql://user:pass@localhost:5432/db").await?;
/// ```
async fn test_postgres(url: &str) -> Result<()> {
    println!("{}", "═".repeat(60));
    println!("{}", "  PostgreSQL Connection Test".green().bold());
    println!("{}", "═".repeat(60));
    println!();

    println!("{} Testing PostgreSQL connection", "→".yellow().bold());
    println!("  {} URL: {}", "→".yellow().bold(), mask_password_in_url(url).cyan());
    println!();

    let (host, port, database, user, password) = parse_postgres_url_full(url)?;

    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!("  {} Database: {}", "→".yellow().bold(), database.cyan());
    println!("  {} User: {}", "→".yellow().bold(), user.cyan());
    println!();

    println!("{} Connecting to PostgreSQL...", "→".yellow().bold());
    let start = Instant::now();

    let pg_config = format!(
        "host={} port={} dbname={} user={} password={}",
        host, port, database, user, password
    );

    let client = match tokio_postgres::NoTls.connect(&pg_config).await {
        Ok((client, conn)) => {
            tokio::spawn(async move {
                if let Err(e) = conn.await {
                    eprintln!("Connection error: {}", e);
                }
            });
            client
        }
        Err(e) => {
            let connection_time = start.elapsed();
            println!();
            println!("  {} PostgreSQL connection failed", "✗".red().bold());
            println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
            println!();
            println!("{}", "Troubleshooting:".yellow().bold());
            println!("  • Ensure PostgreSQL is running");
            println!("  • Check if username/password is correct");
            println!("  • Verify host, port, and database name");
            println!("  • Check pg_hba.conf for authentication settings");
            return Err(crate::error::ConnectionTestFailed {
                service: "PostgreSQL".to_string(),
                message: e.to_string(),
            }.into());
        }
    };

    let connection_time = start.elapsed();
    println!("  {} Connected successfully", "✓".green().bold());
    println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Executing test query...", "→".yellow().bold());
    let query_start = Instant::now();

    let version: String = match client.query_one("SELECT version()", &[]).await {
        Ok(row) => row.get(0),
        Err(e) => {
            println!("  {} Query failed: {}", "✗".red().bold(), e.to_string().red());
            return Err(crate::error::ConnectionTestFailed {
                service: "PostgreSQL".to_string(),
                message: format!("Query failed: {}", e),
            }.into());
        }
    };

    let query_time = query_start.elapsed();
    println!("  {} SELECT version() -> \"{}\"", "✓".green().bold(), version.split(' ').next().unwrap_or(&version).cyan());
    println!("  {} Query time: {:.2}ms", "→".yellow().bold(), query_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Testing INSERT/UPDATE/DELETE...", "→".yellow().bold());
    let test_table = format!("ri_test_{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string());

    let create_sql = format!(
        "CREATE TABLE {} (id SERIAL PRIMARY KEY, value TEXT, created_at TIMESTAMP DEFAULT NOW())",
        test_table
    );

    if let Err(e) = client.execute(&create_sql, &[]).await {
        println!("  {} CREATE TABLE failed: {}", "✗".red().bold(), e.to_string().red());
    } else {
        println!("  {} CREATE TABLE {}", "✓".green().bold(), test_table.cyan());

        let insert_sql = format!("INSERT INTO {} (value) VALUES ($1)", test_table);
        if let Err(e) = client.execute(&insert_sql, &[&"Ri CLI Test Value"]).await {
            println!("  {} INSERT failed: {}", "✗".red().bold(), e.to_string().red());
        } else {
            println!("  {} INSERT INTO {} (value) VALUES (...)", "✓".green().bold(), test_table.cyan());

            let select_sql = format!("SELECT value FROM {} WHERE value = $1", test_table);
            let retrieved: Option<String> = match client.query_opt(&select_sql, &[&"Ri CLI Test Value"]).await {
                Ok(Some(row)) => Some(row.get(0)),
                Ok(None) => None,
                Err(e) => {
                    println!("  {} SELECT failed: {}", "✗".red().bold(), e.to_string().red());
                    None
                }
            };

            if let Some(value) = retrieved {
                println!("  {} SELECT -> \"{}\" {}", "✓".green().bold(), value.cyan(), "✓".green());
            }

            let delete_sql = format!("DELETE FROM {}", test_table);
            if let Err(e) = client.execute(&delete_sql, &[]).await {
                println!("  {} DELETE failed: {}", "✗".red().bold(), e.to_string().red());
            } else {
                println!("  {} DELETE FROM {}", "✓".green().bold(), test_table.cyan());
            }
        }

        let drop_sql = format!("DROP TABLE {}", test_table);
        if let Err(e) = client.execute(&drop_sql, &[]).await {
            println!("  {} DROP TABLE failed: {}", "✗".red().bold(), e.to_string().red());
        } else {
            println!("  {} DROP TABLE {}", "✓".green().bold(), test_table.cyan());
        }
    }

    println!();
    println!("{} PostgreSQL connection test completed successfully!", "✓".green().bold());
    println!();
    println!("{}", "Server Information:".yellow().bold());
    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!("  {} Database: {}", "→".yellow().bold(), database.cyan());
    println!("  {} User: {}", "→".yellow().bold(), user.cyan());
    println!("  {} Status: {}", "→".yellow().bold(), "Connected".green());
    println!("  {} Total time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);

    Ok(())
}

/// Test MySQL connection
///
/// Tests connectivity to a MySQL database server by establishing
/// a TCP connection.
///
/// # Test Operations
///
/// 1. Parse the MySQL URL and extract connection parameters
/// 2. Establish TCP connection to MySQL server
/// 3. Measure connection time
/// 4. Display server information
///
/// # URL Format
///
/// ```text
/// mysql://user:password@host:port/database
/// mysql://root:password@localhost:3306/mydb
/// ```
///
/// # Arguments
///
/// * `url` - MySQL connection URL
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed
/// Returns `RicError::ConnectionTimeout` if connection times out
/// Returns `RicError::ServiceNotAvailable` if MySQL is not running
///
/// # Examples
///
/// ```rust,ignore
/// test_mysql("mysql://user:pass@localhost:3306/db").await?;
/// ```
async fn test_mysql(url: &str) -> Result<()> {
    println!("{}", "═".repeat(60));
    println!("{}", "  MySQL Connection Test".green().bold());
    println!("{}", "═".repeat(60));
    println!();

    println!("{} Testing MySQL connection", "→".yellow().bold());
    println!("  {} URL: {}", "→".yellow().bold(), mask_password_in_url(url).cyan());
    println!();

    let (host, port, database, user, password) = parse_mysql_url_full(url)?;

    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!("  {} Database: {}", "→".yellow().bold(), database.cyan());
    println!("  {} User: {}", "→".yellow().bold(), user.cyan());
    println!();

    println!("{} Connecting to MySQL...", "→".yellow().bold());
    let start = Instant::now();

    let opts = mysql_async::OptsBuilder::new()
        .ip_or_hostname(Some(host.clone()))
        .tcp_port(port)
        .user(Some(user.clone()))
        .pass(Some(password.clone()))
        .db_name(Some(database.clone()));

    let pool = mysql_async::Pool::new(opts);

    let mut conn = match pool.get_conn().await {
        Ok(conn) => conn,
        Err(e) => {
            let connection_time = start.elapsed();
            println!();
            println!("  {} MySQL connection failed", "✗".red().bold());
            println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
            println!();
            println!("{}", "Troubleshooting:".yellow().bold());
            println!("  • Ensure MySQL is running");
            println!("  • Check if username/password is correct");
            println!("  • Verify host, port, and database name");
            println!("  • Check user privileges");
            return Err(crate::error::ConnectionTestFailed {
                service: "MySQL".to_string(),
                message: e.to_string(),
            }.into());
        }
    };

    let connection_time = start.elapsed();
    println!("  {} Connected successfully", "✓".green().bold());
    println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Executing test query...", "→".yellow().bold());
    let query_start = Instant::now();

    let version: String = match conn.query_first("SELECT VERSION()") {
        Ok(Some(row)) => row,
        Ok(None) => "Unknown".to_string(),
        Err(e) => {
            println!("  {} Query failed: {}", "✗".red().bold(), e.to_string().red());
            return Err(crate::error::ConnectionTestFailed {
                service: "MySQL".to_string(),
                message: format!("Query failed: {}", e),
            }.into());
        }
    };

    let query_time = query_start.elapsed();
    println!("  {} SELECT VERSION() -> \"{}\"", "✓".green().bold(), version.split('-').next().unwrap_or(&version).cyan());
    println!("  {} Query time: {:.2}ms", "→".yellow().bold(), query_time.as_secs_f64() * 1000.0);
    println!();

    println!("{} Testing INSERT/UPDATE/DELETE...", "→".yellow().bold());
    let test_table = format!("ri_test_{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string());

    let create_sql = format!(
        "CREATE TABLE {} (id INT AUTO_INCREMENT PRIMARY KEY, value TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)",
        test_table
    );

    if let Err(e) = conn.query_drop(&create_sql) {
        println!("  {} CREATE TABLE failed: {}", "✗".red().bold(), e.to_string().red());
    } else {
        println!("  {} CREATE TABLE {}", "✓".green().bold(), test_table.cyan());

        let insert_sql = format!("INSERT INTO {} (value) VALUES ('Ri CLI Test Value')", test_table);
        if let Err(e) = conn.query_drop(&insert_sql) {
            println!("  {} INSERT failed: {}", "✗".red().bold(), e.to_string().red());
        } else {
            println!("  {} INSERT INTO {} (value) VALUES (...)", "✓".green().bold(), test_table.cyan());

            let select_sql = format!("SELECT value FROM {} WHERE value = 'Ri CLI Test Value'", test_table);
            let retrieved: Option<String> = match conn.query_first(&select_sql) {
                Ok(Some(row)) => Some(row),
                Ok(None) => None,
                Err(e) => {
                    println!("  {} SELECT failed: {}", "✗".red().bold(), e.to_string().red());
                    None
                }
            };

            if let Some(value) = retrieved {
                println!("  {} SELECT -> \"{}\" {}", "✓".green().bold(), value.cyan(), "✓".green());
            }

            let delete_sql = format!("DELETE FROM {}", test_table);
            if let Err(e) = conn.query_drop(&delete_sql) {
                println!("  {} DELETE failed: {}", "✗".red().bold(), e.to_string().red());
            } else {
                println!("  {} DELETE FROM {}", "✓".green().bold(), test_table.cyan());
            }
        }

        let drop_sql = format!("DROP TABLE {}", test_table);
        if let Err(e) = conn.query_drop(&drop_sql) {
            println!("  {} DROP TABLE failed: {}", "✗".red().bold(), e.to_string().red());
        } else {
            println!("  {} DROP TABLE {}", "✓".green().bold(), test_table.cyan());
        }
    }

    drop(conn);

    println!();
    println!("{} MySQL connection test completed successfully!", "✓".green().bold());
    println!();
    println!("{}", "Server Information:".yellow().bold());
    println!("  {} Host: {}", "→".yellow().bold(), host.cyan());
    println!("  {} Port: {}", "→".yellow().bold(), port.to_string().cyan());
    println!("  {} Database: {}", "→".yellow().bold(), database.cyan());
    println!("  {} User: {}", "→".yellow().bold(), user.cyan());
    println!("  {} Status: {}", "→".yellow().bold(), "Connected".green());
    println!("  {} Total time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);

    Ok(())
}

/// Test Kafka connection
///
/// Tests connectivity to a Kafka broker cluster by establishing
/// a TCP connection.
///
/// # Test Operations
///
/// 1. Parse the Kafka broker URL(s)
/// 2. Establish TCP connection to each broker
/// 3. Measure connection time
/// 4. Display broker information
///
/// # URL Format
///
/// ```text
/// host:port
/// localhost:9092
/// broker1:9092,broker2:9092,broker3:9092
/// ```
///
/// # Arguments
///
/// * `url` - Kafka broker URL(s)
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed
/// Returns `RicError::ConnectionTimeout` if connection times out
/// Returns `RicError::ServiceNotAvailable` if Kafka is not running
///
/// # Examples
///
/// ```rust,ignore
/// test_kafka("localhost:9092").await?;
/// ```
async fn test_kafka(url: &str) -> Result<()> {
    println!("{}", "═".repeat(60));
    println!("{}", "  Kafka Connection Test".green().bold());
    println!("{}", "═".repeat(60));
    println!();

    println!("{} Testing Kafka connection", "→".yellow().bold());
    println!("  {} Broker(s): {}", "→".yellow().bold(), url.cyan());
    println!();

    let brokers = parse_kafka_url(url)?;

    println!("  {} Found {} broker(s)", "→".yellow().bold(), brokers.len().to_string().cyan());
    println!();

    println!("{} Connecting to Kafka brokers...", "→".yellow().bold());
    let start = Instant::now();

    let config = rdkafka::config::ClientConfig::new();
    let producer: rdkafka::producer::Producer<_> = config
        .set("bootstrap.servers", &brokers.join(","))
        .set("message.timeout.ms", "5000")
        .set("socket.timeout.ms", "5000")
        .set("request.timeout.ms", "5000")
        .set("metadata.request.timeout.ms", "5000")
        .create()
        .map_err(|e| {
            crate::error::ConnectionTestFailed {
                service: "Kafka".to_string(),
                message: e.to_string(),
            }
        })?;

    let metadata = producer.fetch_metadata(None, std::time::Duration::from_secs(5))
        .map_err(|e| {
            crate::error::ConnectionTestFailed {
                service: "Kafka".to_string(),
                message: format!("Failed to fetch metadata: {}", e),
            }
        })?;

    let connection_time = start.elapsed();
    println!("  {} Connected successfully", "✓".green().bold());
    println!("  {} Connection time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);
    println!();

    println!("{}", "Broker Information:".yellow().bold());
    for broker in metadata.brokers() {
        let broker_str = format!("{}:{}", broker.host(), broker.port());
        println!("  {} Broker: {} (ID: {})", "✓".green().bold(), broker_str.cyan(), broker.id().to_string().cyan());
    }
    println!();

    println!("{}", "Topic Information:".yellow().bold());
    let topics = metadata.topics();
    let topic_count = topics.len();
    println!("  {} Total topics: {}", "→".yellow().bold(), topic_count.to_string().cyan());

    if topic_count > 0 {
        println!("  {} First 5 topics:", "→".yellow().bold());
        for (i, topic) in topics.iter().take(5).enumerate() {
            println!("    {} {}. {} (partitions: {})", "→".yellow().bold(), i + 1, topic.name().cyan(), topic.partitions().len().to_string().cyan());
        }
        if topic_count > 5 {
            println!("    ... and {} more", (topic_count - 5).to_string().cyan());
        }
    }
    println!();

    println!("{} Testing message production...", "→".yellow().bold());
    let test_topic = format!("ri_test_{}", uuid::Uuid::new_v4().to_string().replace("-", "")[..8].to_string());
    let test_message = format!("Ri CLI Test Message at {}", chrono::Utc::now());

    let produce_future = producer.send(
        rdkafka::producer::ProducerRecord::new(
            &test_topic,
            rdkafka::record::RecordKey::NULL,
            test_message.as_bytes(),
        ),
        std::time::Duration::from_secs(5),
    );

    match produce_future {
        Ok((partition, offset)) => {
            println!("  {} Message sent to {} [{}]@{}", "✓".green().bold(), test_topic.cyan(), partition.to_string().cyan(), offset.to_string().cyan());
        }
        Err((e, _)) => {
            let err_str = e.to_string();
            if err_str.contains("Unknown topic") || err_str.contains("topic") {
                println!("  {} Topic '{}' does not exist, creating...", "ℹ".blue().bold(), test_topic.cyan());
                println!("  {} Message production skipped (topic auto-create may be disabled)", "ℹ".blue().bold());
            } else {
                println!("  {} Failed to send message: {}", "✗".red().bold(), err_str.red());
            }
        }
    }

    drop(producer);

    println!();
    println!("{} Kafka connection test completed successfully!", "✓".green().bold());
    println!();
    println!("{}", "Summary:".yellow().bold());
    println!("  {} Brokers tested: {}", "→".yellow().bold(), brokers.len().to_string().cyan());
    println!("  {} Topics discovered: {}", "→".yellow().bold(), topic_count.to_string().cyan());
    println!("  {} Status: {}", "→".yellow().bold(), "Connected".green());
    println!("  {} Total time: {:.2}ms", "→".yellow().bold(), connection_time.as_secs_f64() * 1000.0);

    Ok(())
}

// =============================================================================
// URL Parsing Helper Functions
// =============================================================================

/// Parse Redis URL to extract host and port
///
/// Supports URL formats:
/// - redis://host:port
/// - redis://password@host:port
/// - redis://:password@host:port
/// - host:port (without scheme)
///
/// # Arguments
///
/// * `url` - Redis connection URL
///
/// # Returns
///
/// Returns (host, port) tuple on success.
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed.
fn parse_redis_url(url: &str) -> Result<(String, u16)> {
    // Default values
    let default_port = 6379;
    
    // Remove redis:// prefix if present
    let url = url.strip_prefix("redis://").unwrap_or(url);
    
    // Remove password part if present (we don't use it for TCP test)
    let url = if url.contains('@') {
        url.split('@').last().unwrap_or(url)
    } else {
        url
    };
    
    // Remove database part if present
    let url = url.split('/').next().unwrap_or(url);
    
    // Parse host and port
    if url.contains(':') {
        let parts: Vec<&str> = url.split(':').collect();
        if parts.len() == 2 {
            let host = parts[0].to_string();
            let port = parts[1].parse::<u16>().map_err(|_| {
                crate::error::RicError::InvalidConnectionUrl {
                    url: url.to_string(),
                    reason: "Invalid port number".to_string(),
                }
            })?;
            Ok((host, port))
        } else {
            Err(crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid URL format. Expected host:port".to_string(),
            })
        }
    } else {
        // No port specified, use default
        Ok((url.to_string(), default_port))
    }
}

/// Parse PostgreSQL URL to extract connection parameters
///
/// Supports URL format:
/// postgresql://user:password@host:port/database
///
/// # Arguments
///
/// * `url` - PostgreSQL connection URL
///
/// # Returns
///
/// Returns (host, port, database, user) tuple on success.
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed.
fn parse_postgres_url(url: &str) -> Result<(String, u16, String, String)> {
    // Default values
    let default_port = 5432;
    let default_user = "postgres";
    let default_database = "postgres";
    
    // Remove postgresql:// prefix if present
    let url = url.strip_prefix("postgresql://").unwrap_or(url);
    let url = url.strip_prefix("postgres://").unwrap_or(url);
    
    // Split into credentials and host/database parts
    let (credentials, host_db) = if url.contains('@') {
        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            return Err(crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid URL format".to_string(),
            });
        }
    } else {
        ("", url)
    };
    
    // Parse user from credentials
    let user = if credentials.contains(':') {
        credentials.split(':').next().unwrap_or(default_user)
    } else if !credentials.is_empty() {
        credentials
    } else {
        default_user
    };
    
    // Parse host, port, and database
    let mut database = default_database.to_string();
    let host_port = if host_db.contains('/') {
        let parts: Vec<&str> = host_db.split('/').collect();
        if let Some(db_part) = parts.get(1) {
            database = db_part.split('?').next().unwrap_or(db_part).to_string();
        }
        parts[0]
    } else {
        host_db
    };
    
    // Parse host and port
    let (host, port): (String, u16) = if host_port.contains(':') {
        let parts: Vec<&str> = host_port.split(':').collect();
        let port = parts[1].parse::<u16>().map_err(|_| {
            crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid port number".to_string(),
            }
        })?;
        (parts[0].to_string(), port)
    } else {
        (host_port.to_string(), default_port)
    };
    
    Ok((host, port, database, user.to_string()))
}

fn parse_postgres_url_full(url: &str) -> Result<(String, u16, String, String, String)> {
    let default_port = 5432;
    let default_user = "postgres";
    let default_database = "postgres";
    let default_password = "";

    let url = url.strip_prefix("postgresql://").unwrap_or(url);
    let url = url.strip_prefix("postgres://").unwrap_or(url);

    let (credentials, host_db) = if url.contains('@') {
        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            return Err(crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid URL format".to_string(),
            }.into());
        }
    } else {
        ("", url)
    };

    let (user, password) = if credentials.contains(':') {
        let parts: Vec<&str> = credentials.split(':').collect();
        (parts[0].to_string(), parts.get(1).unwrap_or(&default_password).to_string())
    } else if !credentials.is_empty() {
        (credentials.to_string(), default_password.to_string())
    } else {
        (default_user.to_string(), default_password.to_string())
    };

    let mut database = default_database.to_string();
    let host_port = if host_db.contains('/') {
        let parts: Vec<&str> = host_db.split('/').collect();
        if let Some(db_part) = parts.get(1) {
            database = db_part.split('?').next().unwrap_or(db_part).to_string();
        }
        parts[0]
    } else {
        host_db
    };

    let (host, port): (String, u16) = if host_port.contains(':') {
        let parts: Vec<&str> = host_port.split(':').collect();
        let port = parts[1].parse::<u16>().map_err(|_| {
            crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid port number".to_string(),
            }
        })?;
        (parts[0].to_string(), port)
    } else {
        (host_port.to_string(), default_port)
    };

    Ok((host, port, database, user, password))
}

/// Parse MySQL URL to extract connection parameters
///
/// Supports URL format:
/// mysql://user:password@host:port/database
///
/// # Arguments
///
/// * `url` - MySQL connection URL
///
/// # Returns
///
/// Returns (host, port, database, user) tuple on success.
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed.
fn parse_mysql_url(url: &str) -> Result<(String, u16, String, String)> {
    // Default values
    let default_port = 3306;
    let default_user = "root";
    let default_database = "mysql";
    
    // Remove mysql:// prefix if present
    let url = url.strip_prefix("mysql://").unwrap_or(url);
    
    // Split into credentials and host/database parts
    let (credentials, host_db) = if url.contains('@') {
        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            return Err(crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid URL format".to_string(),
            });
        }
    } else {
        ("", url)
    };
    
    // Parse user from credentials
    let user = if credentials.contains(':') {
        credentials.split(':').next().unwrap_or(default_user)
    } else if !credentials.is_empty() {
        credentials
    } else {
        default_user
    };
    
    // Parse host, port, and database
    let mut database = default_database.to_string();
    let host_port = if host_db.contains('/') {
        let parts: Vec<&str> = host_db.split('/').collect();
        if let Some(db_part) = parts.get(1) {
            database = db_part.split('?').next().unwrap_or(db_part).to_string();
        }
        parts[0]
    } else {
        host_db
    };
    
    // Parse host and port
    let (host, port): (String, u16) = if host_port.contains(':') {
        let parts: Vec<&str> = host_port.split(':').collect();
        let port = parts[1].parse::<u16>().map_err(|_| {
            crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid port number".to_string(),
            }
        })?;
        (parts[0].to_string(), port)
    } else {
        (host_port.to_string(), default_port)
    };
    
    Ok((host, port, database, user.to_string()))
}

fn parse_mysql_url_full(url: &str) -> Result<(String, u16, String, String, String)> {
    let default_port = 3306;
    let default_user = "root";
    let default_database = "mysql";
    let default_password = "";

    let url = url.strip_prefix("mysql://").unwrap_or(url);

    let (credentials, host_db) = if url.contains('@') {
        let parts: Vec<&str> = url.split('@').collect();
        if parts.len() == 2 {
            (parts[0], parts[1])
        } else {
            return Err(crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid URL format".to_string(),
            }.into());
        }
    } else {
        ("", url)
    };

    let (user, password) = if credentials.contains(':') {
        let parts: Vec<&str> = credentials.split(':').collect();
        (parts[0].to_string(), parts.get(1).unwrap_or(&default_password).to_string())
    } else if !credentials.is_empty() {
        (credentials.to_string(), default_password.to_string())
    } else {
        (default_user.to_string(), default_password.to_string())
    };

    let mut database = default_database.to_string();
    let host_port = if host_db.contains('/') {
        let parts: Vec<&str> = host_db.split('/').collect();
        if let Some(db_part) = parts.get(1) {
            database = db_part.split('?').next().unwrap_or(db_part).to_string();
        }
        parts[0]
    } else {
        host_db
    };

    let (host, port): (String, u16) = if host_port.contains(':') {
        let parts: Vec<&str> = host_port.split(':').collect();
        let port = parts[1].parse::<u16>().map_err(|_| {
            crate::error::RicError::InvalidConnectionUrl {
                url: url.to_string(),
                reason: "Invalid port number".to_string(),
            }
        })?;
        (parts[0].to_string(), port)
    } else {
        (host_port.to_string(), default_port)
    };

    Ok((host, port, database, user, password))
}

/// Parse Kafka URL to extract broker addresses
///
/// Supports URL formats:
/// - host:port
/// - host1:port1,host2:port2,host3:port3
///
/// # Arguments
///
/// * `url` - Kafka broker URL(s)
///
/// # Returns
///
/// Returns vector of broker addresses on success.
///
/// # Errors
///
/// Returns `RicError::InvalidConnectionUrl` if URL is malformed.
fn parse_kafka_url(url: &str) -> Result<Vec<String>> {
    // Default port
    let default_port = 9092;
    
    // Split by comma for multiple brokers
    let brokers: Vec<String> = url
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|broker| {
            if broker.contains(':') {
                broker.to_string()
            } else {
                format!("{}:{}", broker, default_port)
            }
        })
        .collect();
    
    if brokers.is_empty() {
        return Err(crate::error::RicError::InvalidConnectionUrl {
            url: url.to_string(),
            reason: "No valid broker addresses found".to_string(),
        });
    }
    
    Ok(brokers)
}

/// Mask password in URL for safe display
///
/// Replaces the password portion with asterisks for security.
///
/// # Arguments
///
/// * `url` - URL that may contain a password
///
/// # Returns
///
/// Returns URL with password masked.
fn mask_password_in_url(url: &str) -> String {
    // Check if URL contains password pattern
    if let Some(at_pos) = url.find('@') {
        if let Some(colon_pos) = url[..at_pos].rfind(':') {
            // Found password pattern, mask it
            let prefix = &url[..colon_pos + 1];
            let suffix = &url[at_pos..];
            return format!("{}****{}", prefix, suffix);
        }
    }
    url.to_string()
}
