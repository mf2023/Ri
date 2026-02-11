//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # File System Module C API
//!
//! This module provides C language bindings for DMSC's file system abstraction layer. The file
//! system module delivers cross-platform file and directory operations with unified interfaces
//! across different operating systems. This C API enables C/C++ applications to leverage DMSC's
//! file handling capabilities including path manipulation, file I/O operations, directory
//! traversal, symbolic link management, and file metadata operations.
//!
//! ## Module Architecture
//!
//! The file system module provides a single primary component:
//!
//! - **DMSCFileSystem**: Unified file system abstraction providing portable file and directory
//!   operations. The abstraction layer normalizes platform differences while preserving access
//!   to platform-specific features when needed. The file system object provides methods for
//!   reading, writing, creating, deleting, and managing files and directories across Windows,
//!   Linux, and macOS platforms.
//!
//! ## Cross-Platform Design
//!
//! The file system module implements comprehensive cross-platform compatibility:
//!
//! - **Path Representation**: Uses abstract path representation that normalizes platform-specific
//!   path separators, conventions, and edge cases. Supports both Windows-style paths (C:\)
//!   and Unix-style paths (/). Path operations handle relative and absolute paths uniformly.
//!
//! - **File Operations**: Provides consistent file I/O semantics across platforms including
//!   atomic file operations, proper handling of file locks, and consistent error semantics.
//!   Supports both blocking and asynchronous file operations through the Tokio integration.
//!
//! - **Directory Operations**: Cross-platform directory traversal, creation, and management.
//!   Handles differences in directory structures, permissions, and special directories
//!   across operating systems.
//!
//! - **Symbolic Links**: Proper handling of symbolic links on platforms that support them.
//!   Detects link loops, resolves link targets, and provides control over link resolution.
//!
//! - **Metadata Access**: Uniform interface to file metadata including size, timestamps,
//!   permissions, and file type information. Handles platform-specific metadata differences.
//!
//! ## Supported Operations
//!
//! The file system module provides comprehensive file and directory operations:
//!
//! - **Path Operations**: Join paths, normalize paths, resolve relative paths to absolute,
//!   extract components (filename, extension, parent directory), and check path properties.
//!
//! - **File I/O**: Open files for reading, writing, or appending with various sharing modes.
//!   Read and write operations with configurable buffering. Support for memory-mapped files
//!   for large file operations.
//!
//! - **Directory Operations**: Create directories (including nested paths), list directory
//!   contents, iterate directories recursively, remove directories (with or without contents).
//!
//! - **File Management**: Copy files (with optional overwrite), move/rename files, delete
//!   files, check file existence, and create temporary files.
//!
//! - **Metadata Operations**: Get file size, access/modification/creation times, file
//!   permissions, and file type (regular file, directory, symlink, etc.).
//!
//! - **Permission Management**: Get and set file permissions (Unix mode bits, Windows ACLs).
//!   Handle permission propagation and default permissions for new files.
//!
//! ## File Operations Modes
//!
//! Files can be opened with various modes controlling access and behavior:
//!
//! - **Read Mode**: Open file for reading only. Multiple readers allowed concurrently.
//!
//! - **Write Mode**: Open file for writing. Truncates existing file by default.
//!   Exclusive access for writing.
//!
//! - **Append Mode**: Open file for writing at end only. Multiple appenders allowed.
//!   Preserves existing content.
//!
//! - **Create Mode**: Create file if it doesn't exist. Fail if file exists.
//!
//! - **Truncate Mode**: Truncate file to zero length when opened.
//!
//! - **Binary Mode**: Open file for binary data (no text translation).
//!
//! - **Text Mode**: Open file for text data with platform-specific line ending handling.
//!
//! ## Performance Characteristics
//!
//! File operations have the following performance profiles:
//!
//! - File open: O(1) typically, O(log n) for deep directory traversal
//! - Sequential read: O(n) where n is bytes read, optimized by OS caching
//! - Random read: O(1) per read operation, may cause disk seeks
//! - Directory listing: O(n) where n is directory entry count
//! - Metadata queries: O(1) for cached metadata, O(log n) otherwise
//!
//! ## Asynchronous Operations
//!
//! The file system module supports asynchronous operations through Tokio integration:
//!
//! - Async file I/O for high-concurrency scenarios
//! - Non-blocking directory iteration
//! - Cancellation support for long-running operations
//! - Proper integration with async/await patterns
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - File handles must be properly closed after use
//! - Path strings must be freed after use
//!
//! ## Thread Safety
//!
//! The underlying implementations provide:
//!
//! - File handles are not thread-safe (use synchronization for concurrent access)
//! - File system instance is thread-safe for metadata queries
//! - Path resolution operations are thread-safe
//! - Consider using separate handles for concurrent file operations
//!
//! ## Error Handling
//!
//! File operations return error codes with optional messages:
//!
//! - Error codes follow standard POSIX conventions where possible
//! - Platform-specific errors are mapped to portable error codes
//! - Detailed error messages available for debugging
//! - Permission errors distinguished from other access errors
//!
//! ## Usage Example
//!
//! ```c
//! // Create file system instance with automatic root detection
//! DMSCFileSystem* fs = dmsc_fs_new_auto();
//! if (fs == NULL) {
//!     fprintf(stderr, "Failed to create file system\n");
//!     return ERROR_FILESYSTEM;
//! }
//!
//! // Read file contents
//! char* content = NULL;
//! size_t size = 0;
//! int result = dmsc_fs_read_file(fs, "/path/to/file.txt", &content, &size);
//!
//! if (result == 0 && content != NULL) {
//!     printf("Read %zu bytes: %.*s\n", size, (int)size, content);
//!     dmsc_fs_string_free(content);
//! }
//!
//! // Write to file
//! const char* data = "Hello, World!";
//! result = dmsc_fs_write_file(fs, "/path/to/output.txt", data, strlen(data));
//!
//! if (result != 0) {
//!     fprintf(stderr, "Failed to write file: %s\n", dmsc_fs_last_error(fs));
//! }
//!
//! // List directory contents
//! char** entries = NULL;
//! size_t entry_count = 0;
//! result = dmsc_fs_list_dir(fs, "/path/to/directory", &entries, &entry_count);
//!
//! if (result == 0) {
//!     for (size_t i = 0; i < entry_count; i++) {
//!         printf("  %s\n", entries[i]);
//!         dmsc_fs_string_free(entries[i]);
//!     }
//!     free(entries);
//! }
//!
//! // Cleanup
//! dmsc_fs_free(fs);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::fs`: Rust file system module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The file system module is always enabled as it provides fundamental infrastructure
//! for file operations in DMSC applications.

use crate::fs::DMSCFileSystem;


/// Opaque C wrapper structure for DMSCFileSystem.
///
/// Unified file system abstraction providing portable file and directory operations
/// across different operating systems.
///
/// # Core Responsibilities
///
/// The file system wrapper manages:
///
/// - **Path Management**: Path normalization, joining, resolution, and component extraction.
///   Handles platform-specific path conventions uniformly.
/// - **File Operations**: Reading, writing, creating, deleting, and modifying files with
///   configurable modes and options.
/// - **Directory Operations**: Creating, listing, traversing, and removing directories.
/// - **Metadata Access**: Querying and modifying file attributes including size, timestamps,
///   permissions, and file type.
/// - **Symbolic Links**: Creating, resolving, and detecting symbolic links.
///
/// # Platform Abstraction
///
/// The file system abstraction normalizes platform differences:
///
/// - **Path Separators**: Automatic conversion between Windows (\) and Unix (/) separators.
/// - **Root Directories**: Handles drive letters (Windows) and mount points (Unix).
/// - **Path Length Limits**: Abstracts platform-specific path length constraints.
/// - **Special Characters**: Handles reserved characters and naming restrictions.
/// - **Line Endings**: Configurable text/binary mode for line ending translation.
///
/// # Instance Lifecycle
///
/// File system instances are created once and reused:
///
/// 1. Create via dmsc_fs_new_auto() or filesystem-specific constructor
/// 2. Use for multiple file operations
/// 3. Free via dmsc_fs_free() when no longer needed
///
/// Creating multiple instances for the same filesystem is supported and may be
/// necessary for concurrent operations on some platforms.
///
/// # Thread Safety
///
/// The file system instance is thread-safe for most operations:
///
/// - Metadata queries can be performed concurrently
/// - Path resolution is thread-safe
/// - File operations require external synchronization
/// - Consider using Tokio async runtime for concurrent file I/O
c_wrapper!(CDMSCFileSystem, DMSCFileSystem);

/// Creates a new DMSCFileSystem instance with automatic root directory detection.
///
/// Initializes a file system abstraction with automatic detection of the root directory
/// and appropriate platform configuration. The created instance can perform all
/// supported file operations on the local file system.
///
/// # Returns
///
/// Pointer to newly allocated DMSCFileSystem on success, or NULL if:
/// - Memory allocation fails
/// - Root directory detection fails
/// - Platform initialization fails
///
/// # Automatic Detection
///
/// The function detects and configures:
///
/// - **Root Directory**: System root (/ on Unix, C:\ on Windows)
/// - **Current Directory**: Process current working directory
/// - **Temporary Directory**: Platform temp directory location
/// - **Home Directory**: User home directory
/// - **Path Separators**: Configured based on platform
/// - **Case Sensitivity**: Determined by underlying file system
///
/// # Initial Capabilities
///
/// A newly created file system instance can:
///
/// - Perform all path operations
/// - Access files in any accessible location
/// - Create and manipulate files and directories
/// - Query file metadata
///
/// # Usage Pattern
///
/// ```c
/// DMSCFileSystem* fs = dmsc_fs_new_auto();
/// if (fs == NULL) {
///     fprintf(stderr, "File system initialization failed\n");
///     return ERROR_INIT;
/// }
///
/// // Use file system operations...
///
/// dmsc_fs_free(fs);
/// ```
///
/// # Platform-Specific Notes
///
/// - **Windows**: Uses backslash path separator, supports UNC paths
/// - **Linux/macOS**: Uses forward slash path separator, case-sensitive
/// - **All Platforms**: Supports both relative and absolute paths
#[no_mangle]
pub extern "C" fn dmsc_fs_new_auto() -> *mut CDMSCFileSystem {
    match DMSCFileSystem::new_auto_root() {
        Ok(fs) => Box::into_raw(Box::new(CDMSCFileSystem::new(fs))),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Frees a previously allocated DMSCFileSystem instance.
///
/// Releases all memory associated with the file system instance including any
/// cached paths, handles, or internal state.
///
/// # Parameters
///
/// - `fs`: Pointer to DMSCFileSystem to free. If NULL, the function returns
///   immediately without error.
///
/// # Preconditions
///
/// Before freeing a file system instance:
///
/// 1. Ensure all open file handles are closed
/// 2. Complete any ongoing file operations
/// 3. Release any directory iterators
///
/// # Safety
///
/// This function is safe to call with NULL. Calling with a pointer that has
/// already been freed results in undefined behavior.
///
/// # Behavior
///
/// The destructor:
///
/// - Closes any open file handles
/// - Releases cached resources
/// - Clears internal buffers
/// - Frees allocated memory
c_destructor!(dmsc_fs_free, CDMSCFileSystem);
