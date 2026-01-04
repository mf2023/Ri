// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # FileSystem Module Tests
//!
//! This module contains comprehensive tests for the DMSC filesystem abstraction layer,
//! covering filesystem initialization, safe directory operations, atomic file operations,
//! JSON serialization/deserialization, and category-based path management.
//!
//! ## Test Coverage
//!
//! - **DMSCFileSystem Initialization**: Tests for filesystem root setup including explicit
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
//! The DMSCFileSystem abstraction provides a safe, consistent interface for filesystem
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

use dmsc::fs::DMSCFileSystem;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_fs_new_with_root() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    assert_eq!(fs.project_root(), temp_dir.path());
}

#[test]
fn test_fs_new_auto_root() {
    let fs = DMSCFileSystem::new_auto_root().unwrap();
    assert!(fs.project_root().exists());
}

#[test]
fn test_fs_safe_mkdir() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let new_dir = temp_dir.path().join("test_dir");
    let result = fs.safe_mkdir(&new_dir).unwrap();
    assert_eq!(result, new_dir);
    assert!(new_dir.exists());
}

#[test]
fn test_fs_ensure_parent_dir() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("parent_dir").join("child_file.txt");
    let result = fs.ensure_parent_dir(&file_path).unwrap();
    assert_eq!(result, temp_dir.path().join("parent_dir"));
    assert!(result.exists());
}

#[test]
fn test_fs_atomic_write_text() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content = "Hello, DMSC!";
    fs.atomic_write_text(&file_path, content).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, content);
}

#[test]
fn test_fs_atomic_write_bytes() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_bytes.bin");
    let content = b"Hello, DMSC in bytes!";
    fs.atomic_write_bytes(&file_path, content).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, String::from_utf8_lossy(content));
}

#[test]
fn test_fs_read_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    let json_str = serde_json::to_string(&test_data).unwrap();
    fs.atomic_write_text(&file_path, &json_str).unwrap();
    
    let read_data: TestData = fs.read_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
fn test_fs_exists() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    assert!(!fs.exists(&file_path));
    fs.atomic_write_text(&file_path, "test").unwrap();
    assert!(fs.exists(&file_path));
}

#[test]
fn test_fs_remove_file() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    fs.atomic_write_text(&file_path, "test").unwrap();
    assert!(fs.exists(&file_path));
    fs.remove_file(&file_path).unwrap();
    assert!(!fs.exists(&file_path));
}

#[test]
fn test_fs_remove_dir_all() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
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
fn test_fs_copy_file() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let src_path = temp_dir.path().join("src.txt");
    let dst_path = temp_dir.path().join("dst.txt");
    let content = "Hello, DMSC!";
    fs.atomic_write_text(&src_path, content).unwrap();
    fs.copy_file(&src_path, &dst_path).unwrap();
    let dst_content = fs.read_text(&dst_path).unwrap();
    assert_eq!(dst_content, content);
}

#[test]
fn test_fs_append_text() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content1 = "Hello, ";
    let content2 = "DMSC!";
    fs.atomic_write_text(&file_path, content1).unwrap();
    fs.append_text(&file_path, content2).unwrap();
    let read_content = fs.read_text(&file_path).unwrap();
    assert_eq!(read_content, content1.to_owned() + content2);
}

#[test]
fn test_fs_write_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    fs.write_json(&file_path, &test_data).unwrap();
    
    let read_data: TestData = fs.read_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
fn test_fs_category_dirs() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    assert!(fs.app_dir().exists());
    assert!(fs.logs_dir().exists());
    assert!(fs.cache_dir().exists());
    assert!(fs.reports_dir().exists());
    assert!(fs.observability_dir().exists());
    assert!(fs.temp_dir().exists());
}

#[test]
fn test_fs_ensure_category_path() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let path = fs.ensure_category_path("logs", "test.log");
    assert!(path.parent().unwrap().exists());
    assert!(path.starts_with(fs.logs_dir()));
}

#[test]
fn test_fs_normalize_under_category() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let path = fs.normalize_under_category("cache", "subdir/test.cache");
    assert!(path.starts_with(fs.cache_dir()));
    assert_eq!(path.file_name().unwrap(), "test.cache");
}
