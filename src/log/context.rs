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

#![allow(non_snake_case)]

//! # Log Context
//! 
//! This module provides a thread-local logging context for Ri, similar to MDC (Mapped Diagnostic Context)
//! with built-in support for distributed tracing. It allows adding contextual information to logs
//! that will be automatically included in all log messages from the same thread.
//! 
//! ## Key Features
//! 
//! - **Thread-Local Storage**: Context is stored per-thread, ensuring thread safety
//! - **Distributed Tracing**: Built-in support for trace IDs, span IDs, and parent span IDs
//! - **Flexible Context**: Can store arbitrary key-value pairs
//! - **Easy API**: Simple methods for putting, getting, and removing context values
//! 
//! ## Design Principles
//! 
//! 1. **Thread Safety**: Uses thread-local storage to ensure thread safety
//! 2. **Performance**: Efficient access to context values with minimal overhead
//! 3. **Distributed Tracing Integration**: Built-in support for W3C Trace Context standard
//! 4. **Flexibility**: Can be extended to support additional context types
//! 5. **Simplicity**: Easy-to-use API for adding and removing context values
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! fn example() {
//!     // Set a context value
//!     RiLogContext::put("user_id", "12345");
//!     
//!     // Set distributed tracing context
//!     RiLogContext::set_trace_id(RiLogContext::generate_trace_id());
//!     RiLogContext::set_span_id(RiLogContext::generate_span_id());
//!     
//!     // Get a context value
//!     if let Some(user_id) = RiLogContext::get("user_id") {
//!         println!("User ID: {}", user_id);
//!     }
//!     
//!     // Get all context values
//!     let all_ctx = RiLogContext::get_all();
//!     println!("All context: {:?}", all_ctx);
//!     
//!     // Remove a context value
//!     RiLogContext::remove("user_id");
//!     
//!     // Clear all context values
//!     RiLogContext::clear();
//! }
//! ```

// Logging context for Ri, similar to MDC with distributed tracing support.

use std::cell::RefCell;
use std::collections::HashMap as FxHashMap;
use uuid::Uuid;

// Thread-local logging context storage.
// 
// This thread-local variable stores the logging context for each thread, allowing
// contextual information to be added to logs without passing it explicitly through
// all function calls.
thread_local! {
    static LOGONTEXT: RefCell<FxHashMap<String, String>> = RefCell::new(FxFxHashMap::default());
}

/// Log context for Ri, similar to MDC with distributed tracing support.
/// 
/// This struct provides a thread-local logging context that can be used to add
/// contextual information to logs. It includes built-in support for distributed tracing
/// with trace IDs, span IDs, and parent span IDs.
pub struct RiLogContext;

impl RiLogContext {
    /// Puts a key-value pair into the log context.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The context key
    /// - `value`: The context value
    pub fn put(key: impl Into<String>, value: impl Into<String>) {
        let k = key.into();
        let v = value.into();
        LOGONTEXT.with(|ctx| {
            ctx.borrow_mut().insert(k, v);
        });
    }

    /// Puts multiple key-value pairs into the log context.
    /// 
    /// # Parameters
    /// 
    /// - `values`: A HashMap of key-value pairs to add to the context
    pub fn put_all(values: FxHashMap<String, String>) {
        LOGONTEXT.with(|ctx| {
            let mut ctx = ctx.borrow_mut();
            for (k, v) in values {
                ctx.insert(k, v);
            }
        });
    }

    /// Gets a value from the log context.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The context key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<String>` containing the value if it exists
    pub fn get(key: &str) -> Option<String> {
        LOGONTEXT.with(|ctx| ctx.borrow().get(key).cloned())
    }

    /// Removes a key-value pair from the log context.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The context key to remove
    pub fn remove(key: &str) {
        LOGONTEXT.with(|ctx| {
            ctx.borrow_mut().remove(key);
        });
    }

    /// Gets all key-value pairs from the log context.
    /// 
    /// # Returns
    /// 
    /// A HashMap containing all key-value pairs in the context
    pub fn get_all() -> FxHashMap<String, String> {
        LOGONTEXT.with(|ctx| ctx.borrow().clone())
    }

    /// Clears all key-value pairs from the log context.
    pub fn clear() {
        LOGONTEXT.with(|ctx| ctx.borrow_mut().clear());
    }

    /// Sets the trace ID in the log context.
    /// 
    /// # Parameters
    /// 
    /// - `trace_id`: The trace ID to set
    pub fn set_trace_id(trace_id: impl Into<String>) {
        Self::put("trace_id", trace_id);
    }

    /// Gets the trace ID from the log context.
    /// 
    /// # Returns
    /// 
    /// An `Option<String>` containing the trace ID if it exists
    pub fn get_trace_id() -> Option<String> {
        Self::get("trace_id")
    }

    /// Generates a new trace ID.
    /// 
    /// # Returns
    /// 
    /// A new UUID string suitable for use as a trace ID
    pub fn generate_trace_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Sets the span ID in the log context.
    /// 
    /// # Parameters
    /// 
    /// - `span_id`: The span ID to set
    pub fn set_span_id(span_id: impl Into<String>) {
        Self::put("span_id", span_id);
    }

    /// Gets the span ID from the log context.
    /// 
    /// # Returns
    /// 
    /// An `Option<String>` containing the span ID if it exists
    pub fn get_span_id() -> Option<String> {
        Self::get("span_id")
    }

    /// Generates a new span ID.
    /// 
    /// # Returns
    /// 
    /// A new UUID string suitable for use as a span ID
    pub fn generate_span_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// Sets the parent span ID in the log context.
    /// 
    /// # Parameters
    /// 
    /// - `parent_span_id`: The parent span ID to set
    pub fn set_parent_span_id(parent_span_id: impl Into<String>) {
        Self::put("parent_span_id", parent_span_id);
    }

    /// Gets the parent span ID from the log context.
    /// 
    /// # Returns
    /// 
    /// An `Option<String>` containing the parent span ID if it exists
    pub fn get_parent_span_id() -> Option<String> {
        Self::get("parent_span_id")
    }
}
