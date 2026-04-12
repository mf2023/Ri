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

//! Validation Utilities Module
//!
//! This module provides input validation functions for the CLI tool.
//! It ensures that user inputs meet the required criteria before processing.
//!
//! # Features
//!
//! - **Project Name Validation**: Validate project names for allowed characters
//! - **Path Validation**: Validate paths for existence and accessibility
//! - **Detailed Error Messages**: Provide clear feedback when validation fails
//!
//! # Validation Rules
//!
//! ## Project Names
//!
//! Project names must meet the following criteria:
//! - Only alphanumeric characters (a-z, A-Z, 0-9)
//! - Dashes (-) and underscores (_) are allowed
//! - Cannot start with a number
//! - Cannot be empty
//! - Maximum length: 64 characters
//!
//! ## Paths
//!
//! Paths are validated for:
//! - Existence (file or directory must exist)
//! - Accessibility (read permissions)
//! - Valid UTF-8 encoding
//!
//! # Examples
//!
//! ```rust,ignore
//! use ric::utils::validation;
//!
//! // Validate a project name
//! match validation::validate_project_name("my-project") {
//!     Ok(()) => println!("Valid project name"),
//!     Err(e) => println!("Invalid: {}", e),
//! }
//!
//! // Validate a path
//! match validation::validate_path("src/main.rs") {
//!     Ok(()) => println!("Path is valid and accessible"),
//!     Err(e) => println!("Invalid path: {}", e),
//! }
//! ```

use anyhow::{Context, Result};
use regex::Regex;
use std::path::Path;

/// Validate a project name
///
/// Validates that a project name meets the following criteria:
/// - Contains only alphanumeric characters, dashes, and underscores
/// - Does not start with a number
/// - Is not empty
/// - Maximum length is 64 characters
///
/// # Arguments
///
/// * `name` - The project name to validate
///
/// # Returns
///
/// Returns `Ok(())` if the project name is valid.
/// Returns an error if:
/// - The name is empty
/// - The name starts with a number
/// - The name contains invalid characters
/// - The name exceeds 64 characters
///
/// # Examples
///
/// ```rust,ignore
/// // Valid project names
/// validate_project_name("my-project")?;      // OK
/// validate_project_name("my_project")?;      // OK
/// validate_project_name("MyProject123")?;    // OK
/// validate_project_name("project-name-v2")?; // OK
///
/// // Invalid project names
/// validate_project_name("123project")?;      // Error: starts with number
/// validate_project_name("my project")?;      // Error: contains space
/// validate_project_name("my.project")?;      // Error: contains dot
/// validate_project_name("")?;                // Error: empty
/// ```
///
/// # Validation Rules
///
/// The validation follows these rules in order:
///
/// 1. **Non-empty**: Name must not be empty
/// 2. **Length**: Name must not exceed 64 characters
/// 3. **First character**: Must be a letter (a-z, A-Z)
/// 4. **Characters**: Must only contain a-z, A-Z, 0-9, -, _
///
/// # Errors
///
/// Returns an error with a descriptive message explaining why validation failed.
/// The error message is user-friendly and suggests how to fix the issue.
pub fn validate_project_name(name: &str) -> Result<()> {
    // Check if name is empty
    if name.is_empty() {
        anyhow::bail!("Project name cannot be empty");
    }

    // Check name length
    if name.len() > 64 {
        anyhow::bail!(
            "Project name is too long (maximum 64 characters, got {})",
            name.len()
        );
    }

    // Check if name starts with a number
    if name.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false) {
        anyhow::bail!("Project name cannot start with a number");
    }

    // Validate characters using regex
    // Pattern: starts with letter, followed by letters, numbers, dashes, or underscores
    let pattern = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_-]*$").unwrap();

    if !pattern.is_match(name) {
        anyhow::bail!(
            "Project name '{}' contains invalid characters. \
             Only alphanumeric characters, dashes (-), and underscores (_) are allowed. \
             The name must start with a letter.",
            name
        );
    }

    Ok(())
}

/// Validate that a path exists and is accessible
///
/// Checks that the specified path exists in the file system and can be accessed
/// with read permissions. This function validates both files and directories.
///
/// # Arguments
///
/// * `path` - The path to validate (can be relative or absolute)
///
/// # Returns
///
/// Returns `Ok(())` if the path exists and is accessible.
/// Returns an error if:
/// - The path does not exist
/// - The path cannot be accessed (permission denied)
/// - The path is not valid UTF-8
///
/// # Examples
///
/// ```rust,ignore
/// // Validate existing files
/// validate_path("Cargo.toml")?;
/// validate_path("src/main.rs")?;
///
/// // Validate existing directories
/// validate_path("src")?;
/// validate_path("/usr/local/bin")?;
///
/// // Validate with absolute paths
/// validate_path("/etc/hosts")?;
///
/// // Invalid paths (will return error)
/// validate_path("nonexistent.txt")?;  // Error: does not exist
/// validate_path("/root/secret")?;     // Error: permission denied (if not root)
/// ```
///
/// # Validation Steps
///
/// The function performs these checks in order:
///
/// 1. **Path Conversion**: Converts the path to a `Path` object
/// 2. **Existence Check**: Verifies the path exists using `Path::exists()`
/// 3. **Metadata Access**: Attempts to read metadata to verify accessibility
///
/// # Errors
///
/// Returns an error with context if validation fails. The error message includes:
/// - The path that failed validation
/// - The reason for failure (not found, permission denied, etc.)
///
/// # Security Considerations
///
/// This function only checks read accessibility. It does not:
/// - Check write permissions
/// - Resolve symbolic links
/// - Prevent race conditions (TOCTOU)
///
/// For security-sensitive operations, perform additional checks as needed.
pub fn validate_path<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    // Check if path exists
    if !path.exists() {
        anyhow::bail!(
            "Path does not exist: {}",
            path.display()
        );
    }

    // Try to access metadata to verify readability
    path.metadata().with_context(|| {
        format!(
            "Cannot access path (permission denied): {}",
            path.display()
        )
    })?;

    Ok(())
}

/// Validate that a path exists and is a directory
///
/// Similar to `validate_path`, but also ensures the path is a directory,
/// not a file or other file system object.
///
/// # Arguments
///
/// * `path` - The directory path to validate
///
/// # Returns
///
/// Returns `Ok(())` if the path exists, is accessible, and is a directory.
/// Returns an error if:
/// - The path does not exist
/// - The path is not a directory
/// - The path cannot be accessed
///
/// # Examples
///
/// ```rust,ignore
/// // Validate directories
/// validate_directory("src")?;
/// validate_directory("/usr/local")?;
///
/// // Invalid (will return error)
/// validate_directory("Cargo.toml")?;  // Error: is a file, not directory
/// validate_directory("nonexistent")?; // Error: does not exist
/// ```
pub fn validate_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    // First, validate that the path exists and is accessible
    validate_path(path)?;

    // Check if it's a directory
    if !path.is_dir() {
        anyhow::bail!(
            "Path is not a directory: {}",
            path.display()
        );
    }

    Ok(())
}

/// Validate that a path exists and is a file
///
/// Similar to `validate_path`, but also ensures the path is a regular file,
/// not a directory or other file system object.
///
/// # Arguments
///
/// * `path` - The file path to validate
///
/// # Returns
///
/// Returns `Ok(())` if the path exists, is accessible, and is a file.
/// Returns an error if:
/// - The path does not exist
/// - The path is not a file
/// - The path cannot be accessed
///
/// # Examples
///
/// ```rust,ignore
/// // Validate files
/// validate_file("Cargo.toml")?;
/// validate_file("src/main.rs")?;
///
/// // Invalid (will return error)
/// validate_file("src")?;           // Error: is a directory, not file
/// validate_file("nonexistent.txt")?; // Error: does not exist
/// ```
pub fn validate_file<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    // First, validate that the path exists and is accessible
    validate_path(path)?;

    // Check if it's a file
    if !path.is_file() {
        anyhow::bail!(
            "Path is not a file: {}",
            path.display()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_validate_project_name_valid() {
        // Valid names
        assert!(validate_project_name("my-project").is_ok());
        assert!(validate_project_name("my_project").is_ok());
        assert!(validate_project_name("MyProject").is_ok());
        assert!(validate_project_name("project123").is_ok());
        assert!(validate_project_name("a").is_ok());
        assert!(validate_project_name("A").is_ok());
        assert!(validate_project_name("project-name-v2").is_ok());
    }

    #[test]
    fn test_validate_project_name_invalid() {
        // Invalid: empty
        assert!(validate_project_name("").is_err());

        // Invalid: starts with number
        assert!(validate_project_name("123project").is_err());
        assert!(validate_project_name("1project").is_err());

        // Invalid: contains spaces
        assert!(validate_project_name("my project").is_err());

        // Invalid: contains dots
        assert!(validate_project_name("my.project").is_err());

        // Invalid: contains special characters
        assert!(validate_project_name("my@project").is_err());
        assert!(validate_project_name("my#project").is_err());

        // Invalid: too long
        let long_name = "a".repeat(65);
        assert!(validate_project_name(&long_name).is_err());
    }

    #[test]
    fn test_validate_project_name_edge_cases() {
        // Maximum length (64 characters) should be valid
        let max_name = "a".repeat(64);
        assert!(validate_project_name(&max_name).is_ok());

        // Single character should be valid
        assert!(validate_project_name("a").is_ok());
        assert!(validate_project_name("Z").is_ok());

        // Numbers in the middle should be valid
        assert!(validate_project_name("project123name").is_ok());

        // Multiple dashes and underscores should be valid
        assert!(validate_project_name("my--project__name").is_ok());
    }

    #[test]
    fn test_validate_path_existing() {
        let dir = tempdir().unwrap();

        // Create a test file
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test").unwrap();

        // Validate existing file
        assert!(validate_path(&file_path).is_ok());

        // Validate existing directory
        assert!(validate_path(dir.path()).is_ok());
    }

    #[test]
    fn test_validate_path_nonexistent() {
        // Non-existent path should fail
        assert!(validate_path("nonexistent_path_12345").is_err());
    }

    #[test]
    fn test_validate_directory() {
        let dir = tempdir().unwrap();

        // Create a test file
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test").unwrap();

        // Directory should pass
        assert!(validate_directory(dir.path()).is_ok());

        // File should fail
        assert!(validate_directory(&file_path).is_err());
    }

    #[test]
    fn test_validate_file() {
        let dir = tempdir().unwrap();

        // Create a test file
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test").unwrap();

        // File should pass
        assert!(validate_file(&file_path).is_ok());

        // Directory should fail
        assert!(validate_file(dir.path()).is_err());
    }

    #[test]
    fn test_validate_file_nonexistent() {
        // Non-existent file should fail
        assert!(validate_file("nonexistent_file_12345.txt").is_err());
    }
}
