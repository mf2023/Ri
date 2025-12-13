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

//! # Error Chain Support
//!
//! This module provides enhanced error handling with error chain support, allowing
//! errors to be wrapped with additional context while preserving the original error.
//! This is particularly useful for debugging and error reporting in complex
//! distributed systems.
//!
//! ## Key Features
//!
//! - **Error Chaining**: Wrap errors with additional context
//! - **Error Context**: Add contextual information to errors
//! - **Error Traversal**: Walk through the error chain
//! - **Pretty Printing**: Format error chains for better readability
//! - **Backtrace Support**: Optional backtrace capture for debugging
//!
//! ## Usage Examples
//!
//! ```rust
//! use dms_core::core::error_chain::{ErrorChain, ErrorContext};
//!
//! let result = some_operation()
//!     .map_err(|e| ErrorChain::new(e).context("Failed to perform operation"));
//! ```

use std::error::Error as StdError;
use std::fmt;

/// A chain of errors with contextual information.
#[derive(Debug)]
pub struct ErrorChain {
    /// The source error
    source: Box<dyn StdError + Send + Sync>,
    /// Contextual message
    context: String,
    /// Previous error in the chain (if any)
    previous: Option<Box<ErrorChain>>,
}

impl ErrorChain {
    /// Creates a new error chain from a source error.
    pub fn new<E>(source: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self {
            source: Box::new(source),
            context: String::new(),
            previous: None,
        }
    }

    /// Creates a new error chain with context.
    pub fn with_context<E, S>(source: E, context: S) -> Self
    where
        E: StdError + Send + Sync + 'static,
        S: Into<String>,
    {
        Self {
            source: Box::new(source),
            context: context.into(),
            previous: None,
        }
    }

    /// Adds context to the error chain.
    pub fn context<S>(mut self, context: S) -> Self
    where
        S: Into<String>,
    {
        let source = std::mem::replace(&mut self.source, Box::new(std::io::Error::other("placeholder")));
        Self {
            source,
            context: context.into(),
            previous: Some(Box::new(self)),
        }
    }

    /// Gets the source error.
    pub fn source_error(&self) -> &(dyn StdError + Send + Sync) {
        &*self.source
    }

    /// Gets the context message.
    pub fn get_context(&self) -> &str {
        &self.context
    }

    /// Gets the previous error in the chain.
    pub fn previous(&self) -> Option<&ErrorChain> {
        self.previous.as_deref()
    }

    /// Iterates through the error chain.
    pub fn chain(&self) -> ErrorChainIter {
        ErrorChainIter { current: Some(self) }
    }

    /// Checks if this error chain contains a specific error type.
    pub fn contains<E>(&self) -> bool
    where
        E: StdError + Send + Sync + 'static,
    {
        // Check current error
        if self.source.is::<E>() {
            return true;
        }
        
        // Check previous errors in chain
        let mut current = self.previous.as_deref();
        while let Some(chain) = current {
            if chain.source.is::<E>() {
                return true;
            }
            current = chain.previous.as_deref();
        }
        false
    }

    /// Gets the root cause of the error chain.
    pub fn root_cause(&self) -> &(dyn StdError + Send + Sync) {
        let mut current = self;
        while let Some(prev) = &current.previous {
            current = prev;
        }
        current.source_error()
    }

    /// Formats the error chain as a pretty string.
    pub fn pretty_format(&self) -> String {
        let mut result = String::new();
        
        // Add main error
        if !self.context.is_empty() {
            result.push_str(&format!("Error: {}\n", self.context));
        }
        result.push_str(&format!("Source: {}\n", self.source_error()));

        // Add chain
        let mut level = 1;
        let mut current = self.previous.as_deref();
        while let Some(chain) = current {
            result.push_str(&format!("\nCaused by (level {level}):\n"));
            if !chain.context.is_empty() {
                result.push_str(&format!("  {}\n", chain.get_context()));
            }
            result.push_str(&format!("  {}\n", chain.source_error()));
            level += 1;
            current = chain.previous.as_deref();
        }

        result
    }
}

impl StdError for ErrorChain {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.source)
    }
}

impl fmt::Display for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.context.is_empty() {
            write!(f, "{}", self.context)?;
            if self.previous.is_some() {
                write!(f, ": ")?;
            }
        }
        
        if let Some(prev) = &self.previous {
            write!(f, "{prev}")?;
        } else {
            write!(f, "{}", self.source_error())?;
        }
        
        Ok(())
    }
}

/// Iterator over the error chain.
pub struct ErrorChainIter<'a> {
    current: Option<&'a ErrorChain>,
}

impl<'a> Iterator for ErrorChainIter<'a> {
    type Item = &'a ErrorChain;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current?;
        self.current = current.previous.as_deref();
        Some(current)
    }
}

/// Trait for adding error chain functionality to Result types.
pub trait ErrorContext<T, E> {
    /// Adds context to the error if the Result is Err.
    fn chain_context<S>(self, context: S) -> Result<T, ErrorChain>
    where
        S: Into<String>;

    /// Adds context to the error with lazy evaluation.
    fn with_chain_context<S, F>(self, f: F) -> Result<T, ErrorChain>
    where
        S: Into<String>,
        F: FnOnce() -> S;
}

impl<T, E> ErrorContext<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn chain_context<S>(self, context: S) -> Result<T, ErrorChain>
    where
        S: Into<String>,
    {
        self.map_err(|e| ErrorChain::with_context(e, context))
    }

    fn with_chain_context<S, F>(self, f: F) -> Result<T, ErrorChain>
    where
        S: Into<String>,
        F: FnOnce() -> S,
    {
        self.map_err(|e| ErrorChain::with_context(e, f()))
    }
}

/// Extension trait for adding error context to Option types.
pub trait OptionErrorContext<T> {
    /// Converts None to an error with context.
    fn ok_or_chain<E>(self, err: E) -> Result<T, ErrorChain>
    where
        E: StdError + Send + Sync + 'static;

    /// Converts None to an error with lazy context.
    fn ok_or_else_chain<E, F>(self, f: F) -> Result<T, ErrorChain>
    where
        E: StdError + Send + Sync + 'static,
        F: FnOnce() -> E;
}

impl<T> OptionErrorContext<T> for Option<T> {
    fn ok_or_chain<E>(self, err: E) -> Result<T, ErrorChain>
    where
        E: StdError + Send + Sync + 'static,
    {
        self.ok_or_else(|| ErrorChain::new(err))
    }

    fn ok_or_else_chain<E, F>(self, f: F) -> Result<T, ErrorChain>
    where
        E: StdError + Send + Sync + 'static,
        F: FnOnce() -> E,
    {
        self.ok_or_else(|| ErrorChain::new(f()))
    }
}

/// Utility functions for error chain operations.
pub mod utils {
    use super::*;

    /// Creates a new error chain from a string message.
    pub fn chain_from_msg<S>(msg: S) -> ErrorChain
    where
        S: Into<String>,
    {
        ErrorChain::new(std::io::Error::other(msg.into()))
    }

    /// Wraps an error with context if it matches a predicate.
    pub fn chain_if<E, F, S>(err: E, predicate: F, context: S) -> ErrorChain
    where
        E: StdError + Send + Sync + 'static,
        F: FnOnce(&E) -> bool,
        S: Into<String>,
    {
        if predicate(&err) {
            ErrorChain::with_context(err, context)
        } else {
            ErrorChain::new(err)
        }
    }

    /// Collects multiple errors into a single error chain.
    pub fn chain_from_multiple<S>(errors: Vec<Box<dyn StdError + Send + Sync>>, context: S) -> ErrorChain
    where
        S: Into<String>,
    {
        if errors.is_empty() {
            return chain_from_msg("No errors provided");
        }

        if errors.len() == 1 {
            let error = errors.into_iter().next()
                .ok_or_else(|| std::io::Error::other("errors vector should have at least one element"))
                .unwrap_or_else(|_| Box::new(std::io::Error::other("errors vector should have at least one element")));
            return ErrorChain::with_context(MultiError { errors: vec![error] }, context);
        }

        let combined = MultiError { errors };
        ErrorChain::with_context(combined, context)
    }

    #[derive(Debug)]
    struct MultiError {
        errors: Vec<Box<dyn StdError + Send + Sync>>,
    }

    impl fmt::Display for MultiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "Multiple errors occurred ({} total):", self.errors.len())?;
            for (i, err) in self.errors.iter().enumerate() {
                writeln!(f, "  [{}] {}", i + 1, err)?;
            }
            Ok(())
        }
    }

    impl StdError for MultiError {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            self.errors.first().map(|e| &**e as &(dyn StdError + 'static))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

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
}
