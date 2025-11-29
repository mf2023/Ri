//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

#![allow(non_snake_case)]

//! # File System Module
//! 
//! This module provides a comprehensive file system abstraction for DMS, offering safe and reliable
//! file operations with support for atomic writes, directory management, and structured data formats.
//! 
//! ## Key Components
//! 
//! - **DMSFileSystem**: Public-facing file system class
//! - **_CFileSystemImpl**: Internal file system implementation
//! 
//! ## Design Principles
//! 
//! 1. **Safe Operations**: All file operations are designed to be safe and reliable
//! 2. **Atomic Writes**: Uses atomic write operations to prevent data corruption
//! 3. **Directory Management**: Automatically creates necessary directories
//! 4. **Structured Data Support**: Built-in support for JSON serialization and deserialization
//! 5. **Category-Based Organization**: Organizes files into categories (logs, cache, reports, etc.)
//! 6. **Error Handling**: Provides comprehensive error handling for all file operations
//! 7. **Cloneable**: Designed to be easily cloned for use across different components
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::path::PathBuf;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a file system with a project root
//!     let project_root = PathBuf::from(".");
//!     let fs = DMSFileSystem::_Fnew_with_root(project_root);
//!     
//!     // Write text to a file
//!     fs._Fatomic_write_text("example.txt", "Hello, DMS!")?;
//!     
//!     // Read text from a file
//!     let content = fs._Fread_text("example.txt")?;
//!     println!("File content: {}", content);
//!     
//!     // Write JSON to a file
//!     let data = json!({"key": "value"});
//!     fs._Fwrite_json("example.json", &data)?;
//!     
//!     // Read JSON from a file
//!     let read_data: serde_json::Value = fs._Fread_json("example.json")?;
//!     println!("JSON data: {:?}", read_data);
//!     
//!     // Get category directories
//!     let logs_dir = fs._Flogs_dir();
//!     println!("Logs directory: {:?}", logs_dir);
//!     
//!     Ok(())
//! }
//! ```

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::time::SystemTime;

use crate::core::DMSResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Internal filesystem implementation.
/// 
/// This struct provides the internal implementation of the file system functionality, including
/// directory management, file operations, and category-based organization.
#[derive(Clone)]
struct _CFileSystemImpl {
    /// Project root directory
    project_root: PathBuf,
    /// Application data root directory
    app_data_root: PathBuf,
}

impl _CFileSystemImpl {
    /// Creates a new internal file system implementation with specified roots.
    /// 
    /// # Parameters
    /// 
    /// - `project_root`: The project root directory
    /// - `app_data_root`: The application data root directory
    /// 
    /// # Returns
    /// 
    /// A new `_CFileSystemImpl` instance
    fn _Fnew_with_roots(project_root: PathBuf, app_data_root: PathBuf) -> Self {
        _CFileSystemImpl { project_root, app_data_root }
    }

    /// Creates a new internal file system implementation with a project root and default app data root.
    /// 
    /// The default app data root is created under the project root at `.dms`.
    /// 
    /// # Parameters
    /// 
    /// - `project_root`: The project root directory
    /// 
    /// # Returns
    /// 
    /// A new `_CFileSystemImpl` instance
    fn _Fnew_with_root(project_root: PathBuf) -> Self {
        // Default app data root under project root; can be overridden by core/config.
        let app_data_root = project_root.join(".dms");
        _CFileSystemImpl::_Fnew_with_roots(project_root, app_data_root)
    }

    /// Returns the project root directory.
    /// 
    /// # Returns
    /// 
    /// A reference to the project root path
    fn _Fproject_root(&self) -> &Path {
        &self.project_root
    }

    /// Safely creates a directory and all its parent directories if they don't exist.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the directory to create
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<PathBuf>` containing the created directory path
    fn _Fsafe_mkdir(&self, path: &Path) -> DMSResult<PathBuf> {
        fs::create_dir_all(path).map_err(|e| crate::core::DMSError::Other(format!("safe_mkdir failed: {e}")))?;
        Ok(path.to_path_buf())
    }

    /// Ensures that the parent directory of a given path exists.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path whose parent directory should be ensured
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<PathBuf>` containing the parent directory path
    fn _Fensure_parent_dir(&self, path: &Path) -> DMSResult<PathBuf> {
        if let Some(parent) = path.parent() {
            self._Fsafe_mkdir(parent)
        } else {
            Ok(self.project_root.clone())
        }
    }

    /// Atomically writes text to a file.
    /// 
    /// This method writes to a temporary file first, then renames it to the target path,
    /// ensuring that the write operation is atomic.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to write
    /// - `text`: The text to write to the file
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn _Fatomic_write_text(&self, path: &Path, text: &str) -> DMSResult<()> {
        self._Fensure_parent_dir(path)?;
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {e}")))?;
        let tmp_name = format!(".tmp_{}_{}", ts.as_millis(), path.file_name().and_then(|s| s.to_str()).unwrap_or("tmp"));
        let tmp_path = dir.join(tmp_name);

        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text open tmp failed: {e}")))?;
            file.write_all(text.as_bytes())
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text write failed: {e}")))?;
            file.sync_all()
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text sync failed: {e}")))?;
        }

        fs::rename(&tmp_path, path)
            .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text rename failed: {e}")))?;

        Ok(())
    }

    /// Atomically writes bytes to a file.
    /// 
    /// This method writes to a temporary file first, then renames it to the target path,
    /// ensuring that the write operation is atomic.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to write
    /// - `data`: The bytes to write to the file
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn _Fatomic_write_bytes(&self, path: &Path, data: &[u8]) -> DMSResult<()> {
        self._Fensure_parent_dir(path)?;
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {e}")))?;
        let tmp_name = format!(".tmp_{}_{}", ts.as_millis(), path.file_name().and_then(|s| s.to_str()).unwrap_or("tmp"));
        let tmp_path = dir.join(tmp_name);

        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes open tmp failed: {e}")))?;
            file.write_all(data)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes write failed: {e}")))?;
            file.sync_all()
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes sync failed: {e}")))?;
        }

        fs::rename(&tmp_path, path)
            .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes rename failed: {e}")))?;

        Ok(())
    }

    /// Reads text from a file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to read
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<String>` containing the file content
    fn _Fread_text(&self, path: &Path) -> DMSResult<String> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| crate::core::DMSError::Other(format!("read_text open failed: {e}")))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| crate::core::DMSError::Other(format!("read_text read failed: {e}")))?;
        Ok(buf)
    }

    /// Returns the application data directory.
    /// 
    /// Ensures the directory exists before returning it.
    /// 
    /// # Returns
    /// 
    /// The application data directory path
    fn _Fapp_dir(&self) -> PathBuf {
        let _ = fs::create_dir_all(&self.app_data_root);
        self.app_data_root.clone()
    }

    /// Returns a category-specific directory.
    /// 
    /// Ensures the directory exists before returning it.
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the category
    /// 
    /// # Returns
    /// 
    /// The category directory path
    fn _Fcategory_dir(&self, name: &str) -> PathBuf {
        let dir = self._Fapp_dir().join(name);
        let _ = fs::create_dir_all(&dir);
        dir
    }
}

/// Public-facing filesystem class.
/// 
/// This struct provides a comprehensive file system abstraction for DMS, offering safe and reliable
/// file operations with support for atomic writes, directory management, and structured data formats.
#[derive(Clone)]
pub struct DMSFileSystem {
    /// Internal file system implementation
    inner: _CFileSystemImpl,
}

impl DMSFileSystem {
    /// Creates a new file system with a project root and default app data root.
    /// 
    /// # Parameters
    /// 
    /// - `project_root`: The project root directory
    /// 
    /// # Returns
    /// 
    /// A new `DMSFileSystem` instance
    pub fn _Fnew_with_root(project_root: PathBuf) -> Self {
        let inner = _CFileSystemImpl::_Fnew_with_root(project_root);
        DMSFileSystem { inner }
    }

    /// Creates a new file system with specified roots.
    /// 
    /// # Parameters
    /// 
    /// - `project_root`: The project root directory
    /// - `app_data_root`: The application data root directory
    /// 
    /// # Returns
    /// 
    /// A new `DMSFileSystem` instance
    pub fn _Fnew_with_roots(project_root: PathBuf, app_data_root: PathBuf) -> Self {
        let inner = _CFileSystemImpl::_Fnew_with_roots(project_root, app_data_root);
        DMSFileSystem { inner }
    }

    /// Creates a new file system with the current working directory as the project root.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Self>` containing the new `DMSFileSystem` instance
    pub fn _Fnew_auto_root() -> DMSResult<Self> {
        let cwd = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {e}")))?;
        Ok(Self::_Fnew_with_root(cwd))
    }

    /// Returns the project root directory.
    /// 
    /// # Returns
    /// 
    /// A reference to the project root path
    pub fn _Fproject_root(&self) -> &Path {
        self.inner._Fproject_root()
    }

    /// Safely creates a directory and all its parent directories if they don't exist.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the directory to create
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<PathBuf>` containing the created directory path
    pub fn _Fsafe_mkdir<P: AsRef<Path>>(&self, path: P) -> DMSResult<PathBuf> {
        self.inner._Fsafe_mkdir(path.as_ref())
    }

    /// Ensures that the parent directory of a given path exists.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path whose parent directory should be ensured
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<PathBuf>` containing the parent directory path
    pub fn _Fensure_parent_dir<P: AsRef<Path>>(&self, path: P) -> DMSResult<PathBuf> {
        self.inner._Fensure_parent_dir(path.as_ref())
    }

    /// Atomically writes text to a file.
    /// 
    /// This method writes to a temporary file first, then renames it to the target path,
    /// ensuring that the write operation is atomic.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to write
    /// - `text`: The text to write to the file
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fatomic_write_text<P: AsRef<Path>>(&self, path: P, text: &str) -> DMSResult<()> {
        self.inner._Fatomic_write_text(path.as_ref(), text)
    }

    /// Atomically writes bytes to a file.
    /// 
    /// This method writes to a temporary file first, then renames it to the target path,
    /// ensuring that the write operation is atomic.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to write
    /// - `data`: The bytes to write to the file
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fatomic_write_bytes<P: AsRef<Path>>(&self, path: P, data: &[u8]) -> DMSResult<()> {
        self.inner._Fatomic_write_bytes(path.as_ref(), data)
    }

    /// Reads text from a file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to read
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<String>` containing the file content
    pub fn _Fread_text<P: AsRef<Path>>(&self, path: P) -> DMSResult<String> {
        self.inner._Fread_text(path.as_ref())
    }

    /// Reads JSON from a file and deserializes it into a type.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the JSON file to read
    /// 
    /// # Type Parameters
    /// 
    /// - `T`: The type to deserialize the JSON into
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<T>` containing the deserialized data
    pub fn _Fread_json<P: AsRef<Path>, T: DeserializeOwned>(&self, path: P) -> DMSResult<T> {
        let text = self._Fread_text(path)?;
        serde_json::from_str(&text)
            .map_err(|e| crate::core::DMSError::Other(format!("json read failed: {e}")))
    }

    /// Checks if a file or directory exists.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to check
    /// 
    /// # Returns
    /// 
    /// `true` if the path exists, `false` otherwise
    pub fn _Fexists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    /// Removes a file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to remove
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fremove_file<P: AsRef<Path>>(&self, path: P) -> DMSResult<()> {
        let p = path.as_ref();
        match fs::remove_file(p) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(crate::core::DMSError::Other(format!("remove_file failed: {e}"))),
        }
    }

    /// Removes a directory and all its contents.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the directory to remove
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fremove_dir_all<P: AsRef<Path>>(&self, path: P) -> DMSResult<()> {
        let p = path.as_ref();
        match fs::remove_dir_all(p) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(crate::core::DMSError::Other(format!("remove_dir_all failed: {e}"))),
        }
    }

    /// Copies a file from one path to another.
    /// 
    /// # Parameters
    /// 
    /// - `from`: The source file path
    /// - `to`: The destination file path
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fcopy_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> DMSResult<()> {
        let src = from.as_ref();
        let dst = to.as_ref();
        if let Some(parent) = dst.parent() {
            self._Fsafe_mkdir(parent)?;
        }
        fs::copy(src, dst)
            .map_err(|e| crate::core::DMSError::Other(format!("copy_file failed: {e}")))?;
        Ok(())
    }

    /// Appends text to a file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to append to
    /// - `text`: The text to append to the file
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fappend_text<P: AsRef<Path>>(&self, path: P, text: &str) -> DMSResult<()> {
        use std::io::Write as _;

        let path_ref = path.as_ref();
        self._Fensure_parent_dir(path_ref)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_ref)
            .map_err(|e| crate::core::DMSError::Other(format!("append_text open failed: {e}")))?;
        file.write_all(text.as_bytes())
            .map_err(|e| crate::core::DMSError::Other(format!("append_text write failed: {e}")))?;
        file.flush()
            .map_err(|e| crate::core::DMSError::Other(format!("append_text flush failed: {e}")))?;
        Ok(())
    }

    /// Writes a JSON value to a file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the file to write
    /// - `value`: The value to serialize and write
    /// 
    /// # Type Parameters
    /// 
    /// - `T`: The type of the value to serialize
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Fwrite_json<P: AsRef<Path>, T: Serialize>(&self, path: P, value: &T) -> DMSResult<()> {
        let text = serde_json::to_string_pretty(value)
            .map_err(|e| crate::core::DMSError::Other(format!("json serialize failed: {e}")))?;
        self._Fatomic_write_text(path, &text)
    }

    /// Returns the application data directory.
    /// 
    /// # Returns
    /// 
    /// The application data directory path
    pub fn _Fapp_dir(&self) -> PathBuf {
        self.inner._Fapp_dir()
    }

    /// Returns the logs directory.
    /// 
    /// # Returns
    /// 
    /// The logs directory path
    pub fn _Flogs_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("logs")
    }

    /// Returns the cache directory.
    /// 
    /// # Returns
    /// 
    /// The cache directory path
    pub fn _Fcache_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("cache")
    }

    /// Returns the reports directory.
    /// 
    /// # Returns
    /// 
    /// The reports directory path
    pub fn _Freports_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("reports")
    }

    /// Returns the observability directory.
    /// 
    /// # Returns
    /// 
    /// The observability directory path
    pub fn _Fobservability_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("observability")
    }

    /// Returns the temporary directory.
    /// 
    /// # Returns
    /// 
    /// The temporary directory path
    pub fn _Ftemp_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("tmp")
    }

    /// Ensures a path exists under a specific category directory.
    /// 
    /// # Parameters
    /// 
    /// - `category`: The category name ("logs", "cache", "reports", "observability", "tmp", or other)
    /// - `path_or_name`: The path or name to ensure under the category directory
    /// 
    /// # Returns
    /// 
    /// The full path to the ensured file or directory
    pub fn _Fensure_category_path<S: AsRef<str>, P: AsRef<Path>>(&self, category: S, path_or_name: P) -> PathBuf {
        let base = match category.as_ref() {
            "logs" => self._Flogs_dir(),
            "cache" => self._Fcache_dir(),
            "reports" => self._Freports_dir(),
            "observability" => self._Fobservability_dir(),
            "tmp" => self._Ftemp_dir(),
            _ => self._Fapp_dir(),
        };

        let target = base.join(path_or_name.as_ref());
        let _ = fs::create_dir_all(target.parent().unwrap_or(&base));
        target
    }

    /// Normalizes a path under a specific category directory, using only the file name.
    /// 
    /// # Parameters
    /// 
    /// - `category`: The category name ("logs", "cache", "reports", "observability", "tmp", or other)
    /// - `path_or_name`: The path or name to normalize
    /// 
    /// # Returns
    /// 
    /// The full path to the normalized file or directory
    pub fn _Fnormalize_under_category<S: AsRef<str>, P: AsRef<Path>>(&self, category: S, path_or_name: P) -> PathBuf {
        let name = path_or_name.as_ref().file_name().unwrap_or_else(|| std::ffi::OsStr::new(""));
        self._Fensure_category_path(category, PathBuf::from(name))
    }
}
