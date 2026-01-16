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

use dmsc::core::error_chain::*;
use std::io;

/// Error chain test module for DMSC core error handling system.
///
/// This module provides comprehensive test coverage for the ErrorChain type,
/// which implements a hierarchical error context management system. The ErrorChain
/// type enables attaching contextual information to errors at different levels of
/// the call stack, facilitating more informative error reporting and debugging.
///
/// ## Test Coverage
///
/// - **Error Chain Creation**: Verifies that ErrorChain instances can be created
///   from standard I/O errors and other error types, with proper initialization
///   of context storage.
///
/// - **Context Management**: Tests the ability to attach contextual information
///   to errors using the `.context()` method, which prepends new context layers
///   to the error chain for enhanced error diagnostics.
///
/// - **Chain Iteration**: Validates the iterator implementation that traverses
///   the error chain from most recent to oldest context, supporting diagnostic
///   tools that need to analyze the full error history.
///
/// - **Result Integration**: Tests the `chain_context()` extension method on
///   Result types, enabling ergonomic context attachment during error handling.
///
/// - **Type Detection**: Verifies the `contains()` method for type-safe error
///   type checking within the error chain, supporting conditional error handling.
///
/// ## Design Principles
///
/// The test suite follows a layered testing approach, beginning with fundamental
/// creation and basic operations before advancing to complex multi-level context
/// scenarios. Each test isolates a specific behavior to ensure precise failure
/// identification and maintain test readability.
///
/// Tests prioritize verification of actual behavior over implementation details,
/// allowing the underlying ErrorChain implementation to evolve without requiring
/// test modifications. This behavioral testing approach ensures tests remain
/// valid across refactoring efforts.
///
/// The error chain design supports the following operational semantics:
/// - Context layers are prepended (LIFO order) to maintain chronological accuracy
/// - Each context layer preserves the underlying error for complete traceability
/// - Iteration yields contexts from most recent to oldest for diagnostic clarity
/// - Type checking supports polymorphic error handling across different error types

#[test]
fn test_error_chain_creation() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::new(io_err);
    assert!(chain.get_context().is_empty());
    assert!(chain.previous().is_none());
}

#[test]
fn test_error_chain_with_context() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::with_context(io_err, "Failed to load configuration");
    assert_eq!(chain.get_context(), "Failed to load configuration");
}

#[test]
fn test_error_chain_iteration() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::with_context(io_err, "Level 1")
        .context("Level 2")
        .context("Level 3");

    let contexts: Vec<String> = chain.chain().map(|e| e.get_context().to_string()).collect();
    assert_eq!(contexts, vec!["Level 3", "Level 2", "Level 1"]);
}

#[test]
fn test_result_error_context() {
    let result: Result<i32, io::Error> = Err(io::Error::new(io::ErrorKind::Other, "test error"));
    let chained = result.chain_context("Operation failed");
    assert!(chained.is_err());
    
    let err = chained.unwrap_err();
    assert_eq!(err.get_context(), "Operation failed");
}

#[test]
fn test_error_chain_contains() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::new(io_err).context("Failed to load config");
    
    assert!(chain.contains::<io::Error>());
    assert!(!chain.contains::<std::fmt::Error>());
}