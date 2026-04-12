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

//! Utility Functions Module
//!
//! This module provides helper functions and utilities used throughout the CLI tool.
//! It is organized into several sub-modules, each responsible for a specific area:
//!
//! # Sub-modules
//!
//! - `fs` - File system operations (create directories, copy/write/read files)
//!
//! # Direct Functions
//!
//! This module also provides some utility functions directly:
//! - `current_dir_name`: Get the name of the current working directory
//! - `is_ri_project`: Check if the current directory contains a Ri project
//! - `format_duration`: Format a duration for human-readable display

use std::path::Path;

// =============================================================================
// Sub-module Declarations
// =============================================================================

/// File system utilities module
///
/// Provides functions for common file system operations:
/// - Directory creation
/// - File copying, reading, and writing
/// - Path existence checking
pub mod fs;

/// Output formatting utilities module
///
/// Provides functions for terminal output formatting:
/// - Colored output (success, error, warning, info)
/// - Progress indicators (spinners, progress bars)
/// - Consistent message formatting
pub mod output;

/// Input validation utilities module
///
/// Provides functions for input validation:
/// - Project name validation
/// - Path validation
/// - Configuration validation
pub mod validation;

// =============================================================================
// Re-exports
// =============================================================================

// Re-export commonly used functions for convenience
pub use fs::{create_dir_all, copy_file, write_file, read_file, file_exists, dir_exists};
pub use output::{print_success, print_error, print_warning, print_info, print_progress, print_header, print_step};
pub use validation::{validate_project_name, validate_path, validate_directory, validate_file};

// =============================================================================
// Direct Utility Functions
// =============================================================================

/// Get the name of the current working directory
///
/// Retrieves the name of the current working directory without the full path.
/// Useful for displaying the project name when no explicit name is provided.
///
/// # Returns
///
/// Returns `Some(String)` with the directory name if successful,
/// or `None` if the current directory cannot be determined.
///
/// # Examples
///
/// ```rust,ignore
/// if let Some(name) = current_dir_name() {
///     println!("Current directory: {}", name);
/// }
/// ```
pub fn current_dir_name() -> Option<String> {
    std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
}

/// Check if the current directory contains a Ri project
///
/// Determines whether the current directory is a Ri project by checking:
/// 1. Cargo.toml file exists
/// 2. Cargo.toml contains a dependency on the ri crate
///
/// # Returns
///
/// Returns `true` if the current directory is a Ri project, `false` otherwise.
///
/// # Examples
///
/// ```rust,ignore
/// if is_ri_project() {
///     println!("This is a Ri project");
/// } else {
///     println!("Not a Ri project");
/// }
/// ```
pub fn is_ri_project() -> bool {
    Path::new("Cargo.toml").exists() && {
        if let Ok(content) = std::fs::read_to_string("Cargo.toml") {
            content.contains("ri =")
        } else {
            false
        }
    }
}

/// Format a duration for human-readable display
///
/// Converts a `std::time::Duration` into a human-readable string format.
/// The format adapts based on the duration length:
/// - Durations >= 1 second: "X.XXXs" format
/// - Durations < 1 second: "XXXms" format
///
/// # Arguments
///
/// * `duration` - The duration to format
///
/// # Returns
///
/// Returns a formatted string representing the duration.
///
/// # Examples
///
/// ```rust,ignore
/// use std::time::Duration;
///
/// let d1 = Duration::from_secs(5);
/// assert_eq!(format_duration(d1), "5.000s");
///
/// let d2 = Duration::from_millis(250);
/// assert_eq!(format_duration(d2), "250ms");
/// ```
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    if secs > 0 {
        format!("{}.{:03}s", secs, millis)
    } else {
        format!("{}ms", millis)
    }
}
