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

//! File System Utilities Module
//!
//! This module provides essential file system operations for the CLI tool.
//! All functions are designed with proper error handling using `anyhow::Result`
//! to provide detailed error context when operations fail.
//!
//! # Features
//!
//! - **Directory Operations**: Create directories recursively with `create_dir_all`
//! - **File Operations**: Copy, read, and write files with comprehensive error handling
//! - **Path Validation**: Check existence of files and directories
//!
//! # Error Handling
//!
//! All functions return `anyhow::Result<T>` which provides:
//! - Detailed error messages with context
//! - Automatic error chaining for debugging
//! - Clean error propagation throughout the CLI
//!
//! # Examples
//!
//! ```rust,ignore
//! use ric::utils::fs;
//!
//! // Create a directory structure
//! fs::create_dir_all("src/components")?;
//!
//! // Write content to a file
//! fs::write_file("src/main.rs", "fn main() {}")?;
//!
//! // Read file content
//! let content = fs::read_file("src/main.rs")?;
//!
//! // Check if file exists
//! if fs::file_exists("Cargo.toml") {
//!     println!("Found Cargo.toml");
//! }
//! ```

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Create a directory and all parent directories recursively
///
/// Creates the specified directory path, including any necessary parent directories.
/// This function is idempotent - if the directory already exists, it succeeds silently.
///
/// # Arguments
///
/// * `path` - The directory path to create (can be relative or absolute)
///
/// # Returns
///
/// Returns `Ok(())` if the directory was created or already exists.
/// Returns an error if:
/// - The path exists but is not a directory
/// - Insufficient permissions to create the directory
/// - Invalid path characters or path too long
///
/// # Examples
///
/// ```rust,ignore
/// // Create a nested directory structure
/// create_dir_all("project/src/utils")?;
///
/// // Works with absolute paths
/// create_dir_all("/tmp/my-project/config")?;
///
/// // Idempotent - safe to call multiple times
/// create_dir_all("project/src")?;
/// create_dir_all("project/src")?; // No error
/// ```
///
/// # Errors
///
/// Returns an error with context if the directory creation fails.
/// The error message includes the path that failed and the underlying OS error.
pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();

    fs::create_dir_all(path).with_context(|| {
        format!(
            "Failed to create directory: {}",
            path.display()
        )
    })?;

    Ok(())
}

/// Copy a file from source to destination
///
/// Copies the content of a file from the source path to the destination path.
/// The destination path's parent directory must exist; this function does not
/// create parent directories automatically.
///
/// # Arguments
///
/// * `src` - The source file path to copy from
/// * `dst` - The destination file path to copy to
///
/// # Returns
///
/// Returns `Ok(())` if the file was successfully copied.
/// Returns an error if:
/// - The source file does not exist
/// - The destination's parent directory does not exist
/// - Insufficient permissions to read source or write destination
/// - The source path is a directory, not a file
///
/// # Examples
///
/// ```rust,ignore
/// // Copy a configuration file
/// copy_file("templates/config.yaml", "project/config.yaml")?;
///
/// // Copy with absolute paths
/// copy_file("/etc/default.conf", "/home/user/app/default.conf")?;
/// ```
///
/// # Errors
///
/// Returns an error with context if the copy operation fails.
/// The error message includes both source and destination paths.
///
/// # Note
///
/// This function overwrites the destination file if it already exists.
/// Use `file_exists()` to check before copying if overwriting is undesirable.
pub fn copy_file<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    fs::copy(src, dst).with_context(|| {
        format!(
            "Failed to copy file from '{}' to '{}'",
            src.display(),
            dst.display()
        )
    })?;

    Ok(())
}

/// Write content to a file
///
/// Writes the specified content to a file, creating the file if it doesn't exist
/// or overwriting it if it does. The file is written atomically when possible.
///
/// # Arguments
///
/// * `path` - The file path to write to
/// * `content` - The content to write to the file
///
/// # Returns
///
/// Returns `Ok(())` if the content was successfully written.
/// Returns an error if:
/// - The parent directory does not exist
/// - Insufficient permissions to write the file
/// - Disk is full or other I/O error occurs
///
/// # Examples
///
/// ```rust,ignore
/// // Write source code to a file
/// write_file("src/main.rs", "fn main() { println!(\"Hello\"); }")?;
///
/// // Write configuration
/// write_file("config.yaml", "app:\n  name: my-app")?;
///
/// // Write with absolute path
/// write_file("/tmp/output.txt", "temporary data")?;
/// ```
///
/// # Errors
///
/// Returns an error with context if the write operation fails.
/// The error message includes the file path and underlying error.
///
/// # Note
///
/// This function does not create parent directories automatically.
/// Use `create_dir_all()` to ensure parent directories exist.
pub fn write_file<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, content: C) -> Result<()> {
    let path = path.as_ref();

    fs::write(path, content).with_context(|| {
        format!(
            "Failed to write file: {}",
            path.display()
        )
    })?;

    Ok(())
}

/// Read the content of a file as a string
///
/// Reads the entire content of a file into a string. The file is assumed to be
/// valid UTF-8 text. For binary files, use `std::fs::read` directly.
///
/// # Arguments
///
/// * `path` - The file path to read from
///
/// # Returns
///
/// Returns `Ok(String)` containing the file content if successful.
/// Returns an error if:
/// - The file does not exist
/// - Insufficient permissions to read the file
/// - The file content is not valid UTF-8
/// - The file is too large to fit in memory
///
/// # Examples
///
/// ```rust,ignore
/// // Read a configuration file
/// let config = read_file("config.yaml")?;
/// println!("Configuration:\n{}", config);
///
/// // Read source code
/// let source = read_file("src/main.rs")?;
///
/// // Read with error handling
/// match read_file("optional-file.txt") {
///     Ok(content) => println!("Content: {}", content),
///     Err(e) => println!("File not found or unreadable: {}", e),
/// }
/// ```
///
/// # Errors
///
/// Returns an error with context if the read operation fails.
/// The error message includes the file path and underlying error.
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<String> {
    let path = path.as_ref();

    fs::read_to_string(path).with_context(|| {
        format!(
            "Failed to read file: {}",
            path.display()
        )
    })
}

/// Check if a file exists at the specified path
///
/// Determines whether a file exists at the given path. Returns `false` if:
/// - The path does not exist
/// - The path exists but is a directory, not a file
/// - Insufficient permissions to access the path
///
/// # Arguments
///
/// * `path` - The file path to check
///
/// # Returns
///
/// Returns `true` if a file exists at the path, `false` otherwise.
///
/// # Examples
///
/// ```rust,ignore
/// // Check for configuration file
/// if file_exists("Cargo.toml") {
///     println!("Found Cargo.toml");
/// } else {
///     println!("Not a Rust project");
/// }
///
/// // Check before reading
/// if file_exists("config.yaml") {
///     let config = read_file("config.yaml")?;
/// }
///
/// // Check for optional files
/// let env_file = if file_exists(".env") {
///     Some(read_file(".env")?)
/// } else {
///     None
/// };
/// ```
///
/// # Note
///
/// This function only checks for files, not directories.
/// Use `dir_exists()` to check for directories.
/// This function does not return an error - use `false` for all failure cases.
pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.exists() && path.is_file()
}

/// Check if a directory exists at the specified path
///
/// Determines whether a directory exists at the given path. Returns `false` if:
/// - The path does not exist
/// - The path exists but is a file, not a directory
/// - Insufficient permissions to access the path
///
/// # Arguments
///
/// * `path` - The directory path to check
///
/// # Returns
///
/// Returns `true` if a directory exists at the path, `false` otherwise.
///
/// # Examples
///
/// ```rust,ignore
/// // Check for source directory
/// if dir_exists("src") {
///     println!("Found source directory");
/// }
///
/// // Check before creating
/// if !dir_exists("build") {
///     create_dir_all("build")?;
/// }
///
/// // Conditional logic based on directory existence
/// if dir_exists(".git") {
///     println!("Git repository detected");
/// }
/// ```
///
/// # Note
///
/// This function only checks for directories, not files.
/// Use `file_exists()` to check for files.
/// This function does not return an error - use `false` for all failure cases.
pub fn dir_exists<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.exists() && path.is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_dir_all() {
        let dir = tempdir().unwrap();
        let test_path = dir.path().join("nested/dirs/test");

        assert!(create_dir_all(&test_path).is_ok());
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_write_and_read_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let content = "Hello, World!";

        assert!(write_file(&file_path, content).is_ok());
        assert!(file_exists(&file_path));

        let read_content = read_file(&file_path).unwrap();
        assert_eq!(read_content, content);
    }

    #[test]
    fn test_copy_file() {
        let dir = tempdir().unwrap();
        let src_path = dir.path().join("source.txt");
        let dst_path = dir.path().join("dest.txt");
        let content = "Copy test";

        write_file(&src_path, content).unwrap();
        assert!(copy_file(&src_path, &dst_path).is_ok());
        assert!(file_exists(&dst_path));

        let copied_content = read_file(&dst_path).unwrap();
        assert_eq!(copied_content, content);
    }

    #[test]
    fn test_file_exists() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("exists.txt");
        let dir_path = dir.path().join("subdir");

        assert!(!file_exists(&file_path));

        write_file(&file_path, "test").unwrap();
        assert!(file_exists(&file_path));

        fs::create_dir(&dir_path).unwrap();
        assert!(!file_exists(&dir_path)); // Directory, not file
    }

    #[test]
    fn test_dir_exists() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        let file_path = dir.path().join("file.txt");

        assert!(!dir_exists(&subdir));

        fs::create_dir(&subdir).unwrap();
        assert!(dir_exists(&subdir));

        write_file(&file_path, "test").unwrap();
        assert!(!dir_exists(&file_path)); // File, not directory
    }
}
