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

//! Output Formatting Utilities Module
//!
//! This module provides terminal output formatting functions for the CLI tool.
//! It offers colored output, progress indicators, and user-friendly messages.
//!
//! # Features
//!
//! - **Colored Output**: Success (green), error (red), warning (yellow), info (default)
//! - **Progress Indicators**: Animated spinners for long-running operations
//! - **Consistent Styling**: Unified message format across the CLI
//!
//! # Color Support
//!
//! Colors are automatically disabled when:
//! - Output is redirected to a file or pipe
//! - The terminal doesn't support colors
//! - The `NO_COLOR` environment variable is set
//!
//! # Examples
//!
//! ```rust,ignore
//! use ric::utils::output;
//!
//! // Print colored messages
//! output::print_success("Project created successfully");
//! output::print_error("Failed to create project");
//! output::print_warning("Configuration file not found, using defaults");
//! output::print_info("Building project...");
//!
//! // Show progress for long operations
//! let spinner = output::print_progress("Installing dependencies...");
//! // ... perform operation ...
//! spinner.finish_with_message("Dependencies installed");
//! ```
//!
//! # Output Format
//!
//! Each function adds appropriate prefixes and styling:
//!
//! - Success: `✓` symbol in green, followed by message in green
//! - Error: `✗` symbol in red, followed by message in red
//! - Warning: `⚠` symbol in yellow, followed by message in yellow
//! - Info: Message in default terminal color
//! - Progress: Animated spinner with message

use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Print a success message in green
///
/// Displays a success message with a checkmark symbol (✓) in green color.
/// Use this to indicate successful completion of operations.
///
/// # Arguments
///
/// * `msg` - The success message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_success("Project created successfully");
/// print_success("Dependencies installed");
/// print_success("Build completed in 2.5s");
/// ```
///
/// # Output Format
///
/// The message is displayed as:
/// ```text
/// ✓ Project created successfully
/// ```
///
/// The checkmark and message are both displayed in green.
pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg.green());
}

/// Print an error message in red
///
/// Displays an error message with a cross symbol (✗) in red color.
/// Use this to indicate failures and errors.
///
/// # Arguments
///
/// * `msg` - The error message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_error("Failed to create project");
/// print_error("Configuration file not found");
/// print_error("Permission denied");
/// ```
///
/// # Output Format
///
/// The message is displayed as:
/// ```text
/// ✗ Failed to create project
/// ```
///
/// The cross symbol and message are both displayed in red.
pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg.red());
}

/// Print a warning message in yellow
///
/// Displays a warning message with a warning symbol (⚠) in yellow color.
/// Use this to indicate potential issues or non-critical problems.
///
/// # Arguments
///
/// * `msg` - The warning message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_warning("Configuration file not found, using defaults");
/// print_warning("Deprecated API usage detected");
/// print_warning("Low disk space");
/// ```
///
/// # Output Format
///
/// The message is displayed as:
/// ```text
/// ⚠ Configuration file not found, using defaults
/// ```
///
/// The warning symbol and message are both displayed in yellow.
pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg.yellow());
}

/// Print an info message in default color
///
/// Displays an informational message in the terminal's default color.
/// Use this for general information and status updates.
///
/// # Arguments
///
/// * `msg` - The info message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_info("Building project...");
/// print_info("Downloading dependencies...");
/// print_info("Running tests...");
/// ```
///
/// # Output Format
///
/// The message is displayed as plain text in the default terminal color:
/// ```text
/// Building project...
/// ```
pub fn print_info(msg: &str) {
    println!("{}", msg);
}

/// Print a progress message with an animated spinner
///
/// Creates and displays a progress bar with an animated spinner for
/// long-running operations. The spinner animates while the operation
/// is in progress and can be finished with a completion message.
///
/// # Arguments
///
/// * `msg` - The progress message to display
///
/// # Returns
///
/// Returns a `ProgressBar` instance that can be used to:
/// - Update the message: `spinner.set_message("New message")`
/// - Finish with message: `spinner.finish_with_message("Done")`
/// - Clear and hide: `spinner.finish_and_clear()`
///
/// # Examples
///
/// ```rust,ignore
/// // Basic usage
/// let spinner = print_progress("Installing dependencies...");
/// // ... perform installation ...
/// spinner.finish_with_message("Dependencies installed");
///
/// // Update message during operation
/// let spinner = print_progress("Processing files...");
/// for file in files {
///     spinner.set_message(&format!("Processing {}", file));
///     // ... process file ...
/// }
/// spinner.finish_with_message("All files processed");
///
/// // Clean finish without message
/// let spinner = print_progress("Cleaning up...");
/// // ... perform cleanup ...
/// spinner.finish_and_clear();
/// ```
///
/// # Output Format
///
/// The spinner displays as:
/// ```text
/// ⠋ Installing dependencies...
/// ```
///
/// The spinner character animates through different states (⠋, ⠙, ⠹, ⠸, ⠼, ⠴, ⠦, ⠧, ⠇, ⠏).
///
/// # Note
///
/// Remember to call `finish_with_message()` or `finish_and_clear()` when done
/// to stop the spinner thread and clean up resources.
pub fn print_progress(msg: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();

    // Set spinner style with animated characters
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );

    spinner.set_message(msg.to_string());
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    spinner
}

/// Print a header message with emphasis
///
/// Displays a header message in bold and cyan color.
/// Use this to separate sections of output or highlight important information.
///
/// # Arguments
///
/// * `msg` - The header message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_header("Project Information");
/// print_header("Build Configuration");
/// print_header("Test Results");
/// ```
///
/// # Output Format
///
/// The message is displayed in bold cyan:
/// ```text
/// Project Information
/// ```
pub fn print_header(msg: &str) {
    println!("\n{}", msg.bold().cyan());
}

/// Print a step message with numbering
///
/// Displays a numbered step message for multi-step operations.
/// Useful for showing progress through a sequence of operations.
///
/// # Arguments
///
/// * `step` - The step number
/// * `total` - The total number of steps
/// * `msg` - The step message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_step(1, 3, "Creating project directory");
/// print_step(2, 3, "Generating configuration files");
/// print_step(3, 3, "Initializing git repository");
/// ```
///
/// # Output Format
///
/// The message is displayed as:
/// ```text
/// [1/3] Creating project directory
/// ```
pub fn print_step(step: usize, total: usize, msg: &str) {
    println!(
        "{} {}",
        format!("[{}/{}]", step, total).bold(),
        msg
    );
}

/// Print a debug message (only in debug builds)
///
/// Displays a debug message with a debug symbol (🔍) in magenta color.
/// This function only produces output in debug builds; it's a no-op in release builds.
///
/// # Arguments
///
/// * `msg` - The debug message to display
///
/// # Examples
///
/// ```rust,ignore
/// print_debug("Variable value: {:?}", value);
/// print_debug("Entering function: process_file");
/// ```
///
/// # Output Format
///
/// In debug builds, the message is displayed as:
/// ```text
/// 🔍 Variable value: Some(42)
/// ```
#[cfg(debug_assertions)]
pub fn print_debug(msg: &str) {
    println!("{} {}", "🔍".magenta(), msg.magenta());
}

/// Print a debug message (release build - no-op)
///
/// In release builds, this function does nothing.
#[cfg(not(debug_assertions))]
pub fn print_debug(_msg: &str) {
    // No-op in release builds
}

/// Print a formatted table
///
/// Displays data in a simple table format with aligned columns.
/// Useful for displaying structured data like project information or configuration.
///
/// # Arguments
///
/// * `headers` - The table headers
/// * `rows` - The table rows (each row is a vector of strings)
///
/// # Examples
///
/// ```rust,ignore
/// let headers = vec!["Name", "Type", "Size"];
/// let rows = vec![
///     vec!["src", "directory", "4KB"],
///     vec!["Cargo.toml", "file", "1KB"],
///     vec!["README.md", "file", "2KB"],
/// ];
/// print_table(&headers, &rows);
/// ```
///
/// # Output Format
///
/// The table is displayed with aligned columns:
/// ```text
/// Name        Type        Size
/// ----        ----        ----
/// src         directory   4KB
/// Cargo.toml  file        1KB
/// README.md   file        2KB
/// ```
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    // Calculate column widths
    let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i < widths.len() {
                widths[i] = widths[i].max(cell.len());
            }
        }
    }

    // Print headers
    let header_row: String = headers
        .iter()
        .enumerate()
        .map(|(i, h)| format!("{:width$}", h, width = widths[i]))
        .collect::<Vec<_>>()
        .join("  ");

    println!("{}", header_row.bold());

    // Print separator
    let separator: String = widths
        .iter()
        .map(|&w| "-".repeat(w))
        .collect::<Vec<_>>()
        .join("  ");

    println!("{}", separator.dimmed());

    // Print rows
    for row in rows {
        let row_str: String = row
            .iter()
            .enumerate()
            .map(|(i, cell)| {
                if i < widths.len() {
                    format!("{:width$}", cell, width = widths[i])
                } else {
                    cell.clone()
                }
            })
            .collect::<Vec<_>>()
            .join("  ");

        println!("{}", row_str);
    }
}

/// Clear the current line in the terminal
///
/// Clears the current line and moves the cursor to the beginning.
/// Useful for updating progress messages in place.
///
/// # Examples
///
/// ```rust,ignore
/// print_info("Processing...");
/// // ... some work ...
/// clear_line();
/// print_success("Processing complete");
/// ```
pub fn clear_line() {
    print!("\r\x1B[2K");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_functions_dont_panic() {
        // These functions should not panic
        print_success("Test success message");
        print_error("Test error message");
        print_warning("Test warning message");
        print_info("Test info message");
        print_header("Test header");
        print_step(1, 3, "Test step");
        print_debug("Test debug message");
    }

    #[test]
    fn test_print_progress() {
        let spinner = print_progress("Test progress");
        spinner.finish_with_message("Done");
    }

    #[test]
    fn test_print_table() {
        let headers = vec!["Name", "Type"];
        let rows = vec![
            vec!["file1.txt".to_string(), "file".to_string()],
            vec!["dir1".to_string(), "directory".to_string()],
        ];

        // Should not panic
        print_table(&headers, &rows);
    }

    #[test]
    fn test_clear_line() {
        // Should not panic
        clear_line();
    }
}
