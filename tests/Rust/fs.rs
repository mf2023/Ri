//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # FileSystem Module Tests
//!
//! This module contains comprehensive tests for the Ri filesystem abstraction layer,
//! covering filesystem initialization, safe directory operations, atomic file operations,
//! JSON serialization/deserialization, and category-based path management.
//!
//! ## Test Coverage
//!
//! - **RiFileSystem Initialization**: Tests for filesystem root setup including explicit
//!   root path configuration and automatic root detection based on system conventions
//! - **Directory Operations**: Tests for safe directory creation, parent directory
//!   enforcement, and recursive directory handling
//! - **File Operations**: Tests for atomic text and binary writes that prevent partial
//!   file corruption, file reading, existence checking, and file removal
//! - **JSON Serialization**: Tests for structured data storage with automatic JSON
//!   parsing and type-safe deserialization using Serde
//! - **Directory Management**: Tests for recursive directory removal and file copying
//!   operations
//! - **Category Path Management**: Tests for standardized directory categories including
//!   app data, logs, cache, reports, observability data, and temporary files
//!
//! ## Design Principles
//!
//! The RiFileSystem abstraction provides a safe, consistent interface for filesystem
//! operations with the following principles:
//! - **Path Safety**: All paths are normalized and validated to prevent path traversal
//!   attacks and ensure operations stay within the project root
//! - **Atomic Operations**: Write operations use temporary files with atomic rename
//!   to prevent data corruption from crashes or interruptions
//! - **Category Organization**: Files are organized into standard categories (logs,
//!   cache, reports, etc.) for consistent structure and easy management
//! - **Error Handling**: Operations return proper error types with context rather
//!   than panicking on common filesystem issues
//!
//! ## Category Directory Structure
//!
//! The filesystem maintains a standard category directory layout:
//! - **app/**: Application-specific data and configuration
//! - **logs/**: Log files and logging-related data
//! - **cache/**: Temporary cached data that can be evicted
//! - **reports/**: Generated reports and output files
//! - **observability/**: Metrics, traces, and observability data
//! - **temp/**: Temporary files with automatic cleanup semantics
//!
//! ## Path Normalization
//!
//! The filesystem provides several path normalization mechanisms:
//! - `ensure_parent_dir()`: Ensures the parent directory exists before file operations
//! - `normalize_under_category()`: Validates and normalizes paths to ensure they
//!   remain within the specified category directory
//! - Category path helpers: Each category directory has a dedicated helper method
//!   (app_dir(), logs_dir(), cache_dir(), etc.) that returns the category root path
//!
//! ## Atomic Write Pattern
//!
//! The atomic write pattern ensures data integrity:
//! 1. Content is written to a temporary file in the same directory (e.g., filename.tmp)
//! 2. On successful write, the temporary file is renamed to the target filename
//! 3. If a crash occurs during write, the temporary file can be cleaned up on restart
//! 4. The rename operation is atomic on most filesystems, ensuring either complete
//!    old or complete new content is visible

use ri::fs::RiFileSystem;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
/// Tests RiFileSystem creation with explicit root path.
///
/// Verifies that a filesystem can be created with a specific root directory
/// using the new_with_root() constructor. The filesystem should use this
/// root as the base for all file operations.
///
/// ## Expected Behavior
///
/// - Filesystem is created with the specified root path
/// - The project_root() method returns the configured root
/// - The root path exists (it's the temp directory)
fn test_fs_new_with_root() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    assert_eq!(fs.project_root(), temp_dir.path());
}

#[test]
/// Tests RiFileSystem automatic root detection.
///
/// Verifies that the filesystem can automatically detect and use an
/// appropriate project root based on system conventions. The auto-detected
/// root should exist and be usable for file operations.
///
/// ## Expected Behavior
///
/// - Filesystem is created with an auto-detected root
/// - The project_root() returns a valid, existing path
/// - The root is suitable for file operations
fn test_fs_new_auto_root() {
    let fs = RiFileSystem::new_auto_root().unwrap();
    assert!(fs.project_root().exists());
}

#[test]
/// Tests safe directory creation with safe_mkdir().
///
/// Verifies that the safe_mkdir() method creates directories safely,
/// returning the path of the created directory and ensuring it exists.
///
/// ## Directory Creation Behavior
///
/// - Creates the specified directory if it doesn't exist
/// - Returns the path to the created directory
/// - Does not error if the directory already exists
/// - Parent directories are not automatically created
///
/// ## Expected Behavior
///
/// - The returned path matches the requested directory
/// - The directory exists after the operation
fn test_fs_safe_mkdir() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let new_dir = temp_dir.path().join("test_dir");
    let result = fs.safe_mkdir(&new_dir).unwrap();
    assert_eq!(result, new_dir);
    assert!(new_dir.exists());
}

#[test]
/// Tests parent directory creation with ensure_parent_dir().
///
/// Verifies that the ensure_parent_dir() method creates parent directories
/// as needed before file operations, ensuring the path is ready for use.
///
/// ## Parent Directory Behavior
///
/// - Creates all missing parent directories recursively
/// - Returns the parent directory path
/// - Does not create the final file or directory
///
/// ## Expected Behavior
///
/// - Parent directory is created if missing
/// - The returned path is the parent directory
/// - The parent directory exists after the operation
fn test_fs_ensure_parent_dir() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("parent_dir").join("child_file.txt");
    let result = fs.ensure_parent_dir(&file_path).unwrap();
    assert_eq!(result, temp_dir.path().join("parent_dir"));
    assert!(result.exists());
}

#[test]
/// Tests atomic text file writing with atomic_write_text().
///
/// Verifies that text content can be written atomically to files,
/// preventing data corruption from crashes or interruptions.
///
/// ## Atomic Write Pattern
///
/// 1. Content is written to a temporary file (filename.tmp)
/// 2. On success, the temporary file is renamed to the target
/// 3. This ensures either old or new content is visible, never partial
///
/// ## Expected Behavior
///
/// - Text content is written to the file
/// - The written content can be read back correctly
/// - The atomic write completes without errors
fn test_fs_atomic_write_text() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content = "Hello, Ri!";
    fs.atomic_write_text(&file_path, content).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, content);
}

#[test]
/// Tests atomic binary file writing with atomic_write_bytes().
///
/// Verifies that binary content can be written atomically to files,
/// ensuring data integrity for non-text data.
///
/// ## Expected Behavior
///
/// - Binary content is written to the file
/// - The written content can be read back correctly
/// - Binary data is preserved without modification
fn test_fs_atomic_write_bytes() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_bytes.bin");
    let content = b"Hello, Ri in bytes!";
    fs.atomic_write_bytes(&file_path, content).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, String::from_utf8_lossy(content));
}

#[test]
/// Tests JSON file reading with read_json().
///
/// Verifies that JSON data can be read from files and automatically
/// deserialized into typed structures using Serde.
///
/// ## Type Safety
///
/// - The target type must implement Deserialize
/// - Type mismatches result in deserialization errors
/// - Complex nested structures are supported
///
/// ## Expected Behavior
///
/// - JSON content is read from the file
/// - The data is deserialized into the target type
/// - The deserialized data matches the original
fn test_fs_read_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    let json_str = serde_json::to_string(&test_data).unwrap();
    fs.atomic_write_text(&file_path, &json_str).unwrap();
    
    let read_data: TestData = fs.read_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
/// Tests file existence checking with exists().
///
/// Verifies that the exists() method correctly reports whether
/// a file or directory exists at the specified path.
///
/// ## Expected Behavior
///
/// - Non-existent paths return false
/// - Existing files return true
/// - The check is accurate and immediate
fn test_fs_exists() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    assert!(!fs.exists(&file_path));
    fs.atomic_write_text(&file_path, "test").unwrap();
    assert!(fs.exists(&file_path));
}

#[test]
/// Tests file removal with remove_file().
///
/// Verifies that files can be safely removed from the filesystem,
/// and that existence checks reflect the removal.
///
/// ## Expected Behavior
///
/// - remove_file() deletes the specified file
/// - The file no longer exists after removal
/// - Attempting to remove non-existent files may error
fn test_fs_remove_file() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    fs.atomic_write_text(&file_path, "test").unwrap();
    assert!(fs.exists(&file_path));
    fs.remove_file(&file_path).unwrap();
    assert!(!fs.exists(&file_path));
}

#[test]
/// Tests recursive directory removal with remove_dir_all().
///
/// Verifies that directories and their contents can be removed
/// recursively, including all files and subdirectories.
///
/// ## Expected Behavior
///
/// - The directory and all contents are removed
/// - No files remain in or under the directory
/// - The operation is recursive (handles nested structures)
fn test_fs_remove_dir_all() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let dir_path = temp_dir.path().join("test_dir");
    fs.safe_mkdir(&dir_path).unwrap();
    let file_path = dir_path.join("test_file.txt");
    fs.atomic_write_text(&file_path, "test").unwrap();
    assert!(fs.exists(&dir_path));
    assert!(fs.exists(&file_path));
    fs.remove_dir_all(&dir_path).unwrap();
    assert!(!fs.exists(&dir_path));
    assert!(!fs.exists(&file_path));
}

#[test]
/// Tests file copying with copy_file().
///
/// Verifies that files can be copied from a source path to a
/// destination path, preserving the content.
///
/// ## Copy Behavior
///
/// - Creates the destination file with the same content
/// - Does not modify the source file
/// - Overwrites destination if it exists
///
/// ## Expected Behavior
///
/// - The destination file is created
/// - The destination content matches the source
fn test_fs_copy_file() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let src_path = temp_dir.path().join("src.txt");
    let dst_path = temp_dir.path().join("dst.txt");
    let content = "Hello, Ri!";
    fs.atomic_write_text(&src_path, content).unwrap();
    fs.copy_file(&src_path, &dst_path).unwrap();
    let dst_content = fs.read_text(&dst_path).unwrap();
    assert_eq!(dst_content, content);
}

#[test]
/// Tests text appending with append_text().
///
/// Verifies that text can be appended to existing files,
/// adding content at the end without overwriting.
///
/// ## Append Behavior
///
/// - Creates the file if it doesn't exist
/// - Adds content at the end of existing content
/// - Does not modify existing content
///
/// ## Expected Behavior
///
/// - The file contains the original content
/// - The appended content follows the original
fn test_fs_append_text() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content1 = "Hello, ";
    let content2 = "Ri!";
    fs.atomic_write_text(&file_path, content1).unwrap();
    fs.append_text(&file_path, content2).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, content1.to_owned() + content2);
}

#[test]
/// Tests JSON file writing with write_json().
///
/// Verifies that typed data can be serialized to JSON and
/// written to files in a single operation.
///
/// ## Serialization
///
/// - The data is serialized to JSON format
/// - The JSON is written atomically to the file
/// - Complex types are supported through Serde
///
/// ## Expected Behavior
///
/// - The data is serialized and written
/// - The file contains valid JSON
/// - The data can be read back correctly
fn test_fs_write_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    fs.write_json(&file_path, &test_data).unwrap();
    
    let read_data: TestData = fs.read_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
/// Tests category directory creation and access.
///
/// Verifies that standard category directories (app, logs, cache,
/// reports, observability, temp) are automatically created and
/// accessible through dedicated methods.
///
/// ## Category Directories
///
/// - app/: Application-specific data
/// - logs/: Log files
/// - cache/: Cached data
/// - reports/: Generated reports
/// - observability/: Metrics and traces
/// - temp/: Temporary files
///
/// ## Expected Behavior
///
/// - All category directories exist
/// - Each directory is under the project root
/// - Directories can be used for file operations
fn test_fs_category_dirs() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    assert!(fs.app_dir().exists());
    assert!(fs.logs_dir().exists());
    assert!(fs.cache_dir().exists());
    assert!(fs.reports_dir().exists());
    assert!(fs.observability_dir().exists());
    assert!(fs.temp_dir().exists());
}

#[test]
/// Tests category path validation with ensure_category_path().
///
/// Verifies that paths can be created within category directories
/// with proper validation and parent directory creation.
///
/// ## Expected Behavior
///
/// - The path is created within the category
/// - Parent directories are created as needed
/// - The path starts with the category directory
fn test_fs_ensure_category_path() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let path = fs.ensure_category_path("logs", "test.log");
    assert!(path.parent().unwrap().exists());
    assert!(path.starts_with(fs.logs_dir()));
}

#[test]
/// Tests category path normalization with normalize_under_category().
///
/// Verifies that paths are validated and normalized to ensure
/// they remain within the specified category directory.
///
/// ## Path Safety
///
/// - Validates the path is within the category
/// - Normalizes path separators
/// - Prevents path traversal attacks
///
/// ## Expected Behavior
///
/// - The normalized path is within the category
/// - The filename is preserved
/// - Invalid paths are rejected or normalized
fn test_fs_normalize_under_category() {
    let temp_dir = tempdir().unwrap();
    let fs = RiFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let path = fs.normalize_under_category("cache", "subdir/test.cache");
    assert!(path.starts_with(fs.cache_dir()));
    assert_eq!(path.file_name().unwrap(), "test.cache");
}
