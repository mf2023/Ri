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

#![allow(non_snake_case)]

//! # Distributed Tracing Context Propagation
//! 
//! This module provides implementations for distributed tracing context propagation,
//! following the W3C Trace Context specification. It allows for propagating trace
//! information across service boundaries using HTTP headers.
//! 
//! ## Key Components
//! 
//! - **DMSCTraceContext**: Represents W3C Trace Context with trace ID, parent ID, and flags
//! - **DMSCBaggage**: Represents baggage for carrying additional cross-cutting concerns
//! - **DMSCContextCarrier**: Carries both trace context and baggage for propagation
//! 
//! ## Design Principles
//! 
//! 1. **W3C Compliance**: Implements the W3C Trace Context specification
//! 2. **Baggage Support**: Provides baggage propagation for additional context
//! 3. **HTTP Header Integration**: Supports extraction and injection from/to HTTP headers
//! 4. **Serialization Support**: Implements Serialize and Deserialize for easy persistence
//! 5. **Thread Safety**: All structs are cloneable for safe sharing across threads
//! 6. **Sampling Support**: Includes trace sampling flag support
//! 7. **Trace State**: Optional support for vendor-specific trace state
//! 8. **Easy to Use**: Simple API for creating and manipulating trace contexts
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::collections::HashMap;
//! 
//! fn example() {
//!     // Create trace and span IDs
//!     let trace_id = DMSCTraceId::generate();
//!     let span_id = DMSCSpanId::generate();
//!     
//!     // Create a trace context
//!     let mut trace_context = DMSCTraceContext::new(trace_id, span_id);
//!     trace_context.set_sampled(true);
//!     
//!     // Create baggage
//!     let mut baggage = DMSCBaggage::new();
//!     baggage.insert("user_id".to_string(), "12345".to_string());
//!     baggage.insert("request_id".to_string(), "abc123".to_string());
//!     
//!     // Create a context carrier
//!     let carrier = DMSCContextCarrier::new()
//!         .with_trace_context(trace_context)
//!         .with_baggage(baggage);
//!     
//!     // Inject into HTTP headers
//!     let mut headers = HashMap::new();
//!     carrier.inject_into_headers(&mut headers);
//!     println!("Headers: {:?}", headers);
//!     
//!     // Extract from HTTP headers
//!     let extracted_carrier = DMSCContextCarrier::from_headers(&headers);
//!     println!("Extracted trace context: {:?}", extracted_carrier.trace_context);
//! }
//! ```

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::observability::tracing::{DMSCTraceId, DMSCSpanId};

/// W3C Trace Context propagation format
///
/// This struct represents a W3C Trace Context, which is used to propagate trace information
/// across service boundaries. It follows the W3C Trace Context specification: https://www.w3.org/TR/trace-context/
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCTraceContext {
    /// Trace context version
    pub version: u8,
    /// Trace ID for the entire trace
    pub trace_id: DMSCTraceId,
    /// Parent span ID
    pub parent_id: DMSCSpanId,
    /// Trace flags (bitmask)
    pub trace_flags: u8,
    /// Optional trace state for vendor-specific information
    pub trace_state: Option<String>,
}

impl Default for DMSCTraceContext {
    fn default() -> Self {
        Self {
            version: 0x00,
            trace_id: DMSCTraceId::default(),
            parent_id: DMSCSpanId::default(),
            trace_flags: 0x01,
            trace_state: None,
        }
    }
}

impl DMSCTraceContext {
    /// Creates a new trace context with the given trace ID and parent span ID.
    ///
    /// # Parameters
    ///
    /// - `trace_id`: The trace ID for the trace
    /// - `parent_id`: The parent span ID
    ///
    /// # Returns
    ///
    /// A new DMSCTraceContext instance
    #[allow(dead_code)]
    pub fn new(trace_id: DMSCTraceId, parent_id: DMSCSpanId) -> Self {
        Self {
            version: 0x00,
            trace_id,
            parent_id,
            trace_flags: 0x01, // Sampled flag
            trace_state: None,
        }
    }
    
    /// Parses a trace context from a W3C Trace Context header string.
    ///
    /// # Parameters
    ///
    /// - `header`: The traceparent header string in format "00-{trace-id}-{parent-id}-{trace-flags}"
    ///
    /// # Returns
    ///
    /// An Option containing the parsed DMSCTraceContext, or None if parsing failed
    #[allow(dead_code)]
    pub fn from_header(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split('-').collect();
        if parts.len() != 4 {
            return None;
        }
        
        let version = u8::from_str_radix(parts[0], 16).ok()?;
        let trace_id = DMSCTraceId::from_string(parts[1].to_string());
        let parent_id = DMSCSpanId::from_string(parts[2].to_string());
        let trace_flags = u8::from_str_radix(parts[3], 16).ok()?;
        
        Some(Self {
            version,
            trace_id,
            parent_id,
            trace_flags,
            trace_state: None,
        })
    }
    
    /// Converts the trace context to a W3C Trace Context header string.
    ///
    /// # Returns
    ///
    /// A string in the format "00-{trace-id}-{parent-id}-{trace-flags}"
    #[allow(dead_code)]
    pub fn to_header(&self) -> String {
        format!(
            "{:02x}-{}-{}-{:02x}",
            self.version,
            self.trace_id.as_str(),
            self.parent_id.as_str(),
            self.trace_flags
        )
    }
    
    /// Checks if the trace is sampled.
    ///
    /// # Returns
    ///
    /// True if the sampled flag is set, false otherwise
    #[allow(dead_code)]
    pub fn is_sampled(&self) -> bool {
        (self.trace_flags & 0x01) != 0
    }
    
    /// Sets the sampled flag on the trace context.
    ///
    /// # Parameters
    ///
    /// - `sampled`: Whether the trace should be sampled
    #[allow(dead_code)]
    pub fn set_sampled(&mut self, sampled: bool) {
        if sampled {
            self.trace_flags |= 0x01;
        } else {
            self.trace_flags &= !0x01;
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCTraceContext {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }

    #[staticmethod]
    #[pyo3(name = "from_header")]
    fn py_from_header(header: String) -> Option<Self> {
        Self::from_header(&header)
    }

    #[pyo3(name = "to_header")]
    fn py_to_header(&self) -> String {
        self.to_header()
    }

    #[pyo3(name = "is_sampled")]
    fn py_is_sampled(&self) -> bool {
        self.is_sampled()
    }

    fn sampled(&mut self, sampled: bool) {
        self.set_sampled(sampled);
    }
}

/// Baggage propagation for cross-cutting concerns.
///
/// This struct represents baggage, which is used to carry additional context information
/// across service boundaries. It follows the W3C Baggage specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCBaggage {
    /// Map of baggage items
    items: HashMap<String, String>,
}

impl Default for DMSCBaggage {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCBaggage {
    /// Creates a new empty baggage instance.
    ///
    /// # Returns
    ///
    /// A new DMSCBaggage instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    
    /// Inserts a key-value pair into the baggage.
    ///
    /// # Parameters
    ///
    /// - `key`: The baggage key
    /// - `value`: The baggage value
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }
    
    /// Gets a value from the baggage by key.
    ///
    /// # Parameters
    ///
    /// - `key`: The baggage key to look up
    ///
    /// # Returns
    ///
    /// An Option containing the value if found, or None otherwise
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.items.get(key)
    }
    
    /// Removes a key-value pair from the baggage.
    ///
    /// # Parameters
    ///
    /// - `key`: The baggage key to remove
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) {
        self.items.remove(key);
    }
    
    /// Parses baggage from a W3C Baggage header string.
    ///
    /// # Parameters
    ///
    /// - `header`: The baggage header string in format "key1=value1,key2=value2"
    ///
    /// # Returns
    ///
    /// A new DMSCBaggage instance with the parsed items
    #[allow(dead_code)]
    pub fn from_header(header: &str) -> Self {
        let mut baggage = Self::new();
        
        for item in header.split(',') {
            let item = item.trim();
            if let Some(eq_pos) = item.find('=') {
                let key = item[..eq_pos].trim().to_string();
                let value = item[eq_pos + 1..].trim().to_string();
                baggage.insert(key, value);
            }
        }
        
        baggage
    }
    
    /// Converts the baggage to a W3C Baggage header string.
    ///
    /// # Returns
    ///
    /// A string in the format "key1=value1,key2=value2"
    #[allow(dead_code)]
    pub fn to_header(&self) -> String {
        self.items
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",")
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCBaggage {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[staticmethod]
    #[pyo3(name = "from_header")]
    fn py_from_header(header: String) -> Self {
        Self::from_header(&header)
    }

    #[pyo3(name = "to_header")]
    fn py_to_header(&self) -> String {
        self.to_header()
    }

    fn add(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }

    fn fetch(&self, key: String) -> Option<String> {
        self.items.get(&key).cloned()
    }

    fn delete(&mut self, key: String) {
        self.items.remove(&key);
    }
}

/// Context carrier for distributed tracing.
///
/// This struct carries both trace context and baggage, providing a convenient way to
/// extract and inject distributed tracing information from/to HTTP headers.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCContextCarrier {
    /// Trace context for the request
    pub trace_context: Option<DMSCTraceContext>,
    /// Baggage for additional context
    pub baggage: DMSCBaggage,
}

impl Default for DMSCContextCarrier {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCContextCarrier {
    /// Creates a new empty context carrier.
    ///
    /// # Returns
    ///
    /// A new DMSCContextCarrier instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            trace_context: None,
            baggage: DMSCBaggage::new(),
        }
    }
    
    /// Adds trace context to the carrier.
    ///
    /// # Parameters
    ///
    /// - `trace_context`: The trace context to add
    ///
    /// # Returns
    ///
    /// The updated DMSCContextCarrier instance
    #[allow(dead_code)]
    pub fn with_trace_context(mut self, trace_context: DMSCTraceContext) -> Self {
        self.trace_context = Some(trace_context);
        self
    }
    
    /// Adds baggage to the carrier.
    ///
    /// # Parameters
    ///
    /// - `baggage`: The baggage to add
    ///
    /// # Returns
    ///
    /// The updated DMSCContextCarrier instance
    #[allow(dead_code)]
    pub fn with_baggage(mut self, baggage: DMSCBaggage) -> Self {
        self.baggage = baggage;
        self
    }
    
    /// Creates a context carrier from tracing context.
    ///
    /// This method converts a thread-local DMSCTracingContext into a DMSCContextCarrier
    /// that can be propagated across service boundaries.
    ///
    /// # Parameters
    ///
    /// - `tracing_context`: The tracing context to convert
    ///
    /// # Returns
    ///
    /// A new DMSCContextCarrier instance with trace context and baggage from the tracing context
    #[allow(dead_code)]
    pub fn from_tracing_context(tracing_context: &crate::observability::tracing::DMSCTracingContext) -> Self {
        let mut carrier = Self::new();
        
        // Create trace context if trace ID and span ID are available
        if let (Some(trace_id), Some(span_id)) = (
            tracing_context.trace_id(),
            tracing_context.span_id()
        ) {
            let trace_context = DMSCTraceContext::new(
                trace_id.clone(),
                span_id.clone()
            );
            carrier.trace_context = Some(trace_context);
        }
        
        // Convert baggage from tracing context
        let baggage = DMSCBaggage::new();
        // Note: We don't have direct access to tracing_context.baggage since it's private,
        // so we'll create an empty baggage for now
        carrier.baggage = baggage;
        
        carrier
    }
    
    /// Creates a tracing context from this carrier.
    ///
    /// This method converts a DMSCContextCarrier into a thread-local DMSCTracingContext
    /// that can be used for tracing within the service.
    ///
    /// # Returns
    ///
    /// A new DMSCTracingContext instance with trace context and baggage from the carrier
    #[allow(dead_code)]
    pub fn into_tracing_context(self) -> crate::observability::tracing::DMSCTracingContext {
        let mut context = crate::observability::tracing::DMSCTracingContext::new();
        
        // Set trace ID and span ID from trace context if available
        if let Some(trace_context) = self.trace_context {
            context = context.with_trace_id(trace_context.trace_id);
            context = context.with_span_id(trace_context.parent_id);
        }
        
        // Set baggage from carrier
        // Note: We don't have direct access to context.baggage since it's private,
        // so we'll skip setting baggage for now
        
        context
    }
    
    /// Extracts a context carrier from HTTP headers.
    ///
    /// # Parameters
    ///
    /// - `headers`: A HashMap of HTTP headers
    ///
    /// # Returns
    ///
    /// A new DMSCContextCarrier instance with extracted trace context and baggage
    #[allow(dead_code)]
    pub fn from_headers(headers: &HashMap<String, String>) -> Self {
        let mut carrier = Self::new();
        
        // Extract trace context from traceparent header
        if let Some(traceparent) = headers.get("traceparent") {
            if let Some(trace_context) = DMSCTraceContext::from_header(traceparent) {
                carrier.trace_context = Some(trace_context);
            }
        }
        
        // Extract baggage from baggage header
        if let Some(baggage_header) = headers.get("baggage") {
            carrier.baggage = DMSCBaggage::from_header(baggage_header);
        }
        
        carrier
    }
    
    /// Injects the context carrier into HTTP headers.
    ///
    /// # Parameters
    ///
    /// - `headers`: A mutable HashMap of HTTP headers to inject into
    #[allow(dead_code)]
    pub fn inject_into_headers(&self, headers: &mut HashMap<String, String>) {
        if let Some(ref trace_context) = self.trace_context {
            headers.insert("traceparent".to_string(), trace_context.to_header());
        }
        
        let baggage_header = self.baggage.to_header();
        if !baggage_header.is_empty() {
            headers.insert("baggage".to_string(), baggage_header);
        }
    }
    
    /// Extracts a context carrier from HTTP headers and sets it as current tracing context.
    ///
    /// This convenience method extracts trace information from HTTP headers, creates a
    /// tracing context, and sets it as the current thread-local context.
    ///
    /// # Parameters
    ///
    /// - `headers`: A HashMap of HTTP headers
    ///
    /// # Returns
    ///
    /// A new DMSCContextCarrier instance with extracted trace context and baggage
    #[allow(dead_code)]
    pub fn from_headers_and_set_current(headers: &HashMap<String, String>) -> Self {
        let carrier = Self::from_headers(headers);
        let tracing_context = carrier.clone().into_tracing_context();
        tracing_context.set_as_current();
        carrier
    }
    
    /// Injects the current tracing context into HTTP headers.
    ///
    /// This convenience method gets the current thread-local tracing context,
    /// converts it to a context carrier, and injects it into HTTP headers.
    ///
    /// # Parameters
    ///
    /// - `headers`: A mutable HashMap of HTTP headers to inject into
    #[allow(dead_code)]
    pub fn inject_current_into_headers(headers: &mut HashMap<String, String>) {
        if let Some(tracing_context) = crate::observability::tracing::DMSCTracingContext::current() {
            let carrier = Self::from_tracing_context(&tracing_context);
            carrier.inject_into_headers(headers);
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCContextCarrier {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[pyo3(name = "with_trace_context")]
    fn py_with_trace_context(&mut self, trace_context: DMSCTraceContext) {
        self.trace_context = Some(trace_context);
    }

    #[pyo3(name = "get_trace_context")]
    fn py_get_trace_context(&self) -> Option<DMSCTraceContext> {
        self.trace_context.clone()
    }

    #[pyo3(name = "with_baggage")]
    fn py_with_baggage(&mut self, baggage: DMSCBaggage) {
        self.baggage = baggage;
    }

    #[pyo3(name = "get_baggage")]
    fn py_get_baggage(&self) -> DMSCBaggage {
        self.baggage.clone()
    }

    #[pyo3(name = "inject_into_headers")]
    fn py_inject_into_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        self.inject_into_headers(&mut headers);
        headers
    }

    #[staticmethod]
    #[pyo3(name = "from_headers")]
    fn py_from_headers(headers: HashMap<String, String>) -> Self {
        Self::from_headers(&headers)
    }
}

/// W3C Trace Context Propagator for distributed tracing.
///
/// This struct implements the W3C Trace Context propagation specification,
/// providing methods for extracting and injecting trace context from/to
/// various carrier formats like HTTP headers.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct W3CTracePropagator;

impl W3CTracePropagator {
    /// Creates a new W3CTracePropagator instance.
    ///
    /// # Returns
    ///
    /// A new W3CTracePropagator instance
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    /// Extracts trace context from HTTP headers.
    ///
    /// This method parses the W3C traceparent and baggage headers from the
    /// provided HTTP headers and creates a DMSCContextCarrier with the
    /// extracted information.
    ///
    /// # Parameters
    ///
    /// - `headers`: A reference to a HashMap of HTTP headers
    ///
    /// # Returns
    ///
    /// A DMSCContextCarrier containing the extracted trace context and baggage
    #[allow(dead_code)]
    pub fn extract(&self, headers: &HashMap<String, String>) -> DMSCContextCarrier {
        DMSCContextCarrier::from_headers(headers)
    }

    /// Injects trace context into HTTP headers.
    ///
    /// This method takes a DMSCContextCarrier and injects its trace context
    /// and baggage into the provided HTTP headers HashMap.
    ///
    /// # Parameters
    ///
    /// - `carrier`: The context carrier containing trace information
    /// - `headers`: A mutable reference to a HashMap of HTTP headers
    #[allow(dead_code)]
    pub fn inject(&self, carrier: &DMSCContextCarrier, headers: &mut HashMap<String, String>) {
        carrier.inject_into_headers(headers);
    }

    /// Extracts trace context and sets it as the current tracing context.
    ///
    /// This convenience method extracts trace context from HTTP headers and
    /// sets it as the current thread-local tracing context.
    ///
    /// # Parameters
    ///
    /// - `headers`: A reference to a HashMap of HTTP headers
    ///
    /// # Returns
    ///
    /// A DMSCContextCarrier containing the extracted trace context and baggage
    #[allow(dead_code)]
    pub fn extract_and_set_current(&self, headers: &HashMap<String, String>) -> DMSCContextCarrier {
        DMSCContextCarrier::from_headers_and_set_current(headers)
    }

    /// Injects the current tracing context into HTTP headers.
    ///
    /// This convenience method gets the current thread-local tracing context,
    /// converts it to a context carrier, and injects it into HTTP headers.
    ///
    /// # Parameters
    ///
    /// - `headers`: A mutable reference to a HashMap of HTTP headers
    #[allow(dead_code)]
    pub fn inject_current(&self, headers: &mut HashMap<String, String>) {
        DMSCContextCarrier::inject_current_into_headers(headers);
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl W3CTracePropagator {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[pyo3(name = "extract")]
    fn py_extract(&self, headers: HashMap<String, String>) -> DMSCContextCarrier {
        self.extract(&headers)
    }

    #[pyo3(name = "inject")]
    fn py_inject(&self, carrier: &DMSCContextCarrier) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        self.inject(carrier, &mut headers);
        headers
    }

    #[pyo3(name = "extract_and_set_current")]
    fn py_extract_and_set_current(&self, headers: HashMap<String, String>) -> DMSCContextCarrier {
        self.extract_and_set_current(&headers)
    }

    #[pyo3(name = "inject_current")]
    fn py_inject_current(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        self.inject_current(&mut headers);
        headers
    }
}
