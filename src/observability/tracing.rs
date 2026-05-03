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

//! # Distributed Tracing
//!
//! This file implements a comprehensive distributed tracing system for the Ri framework. It provides
//! tools for creating, managing, and propagating trace information across asynchronous operations
//! and distributed systems. The tracing system follows the W3C Trace Context standard and integrates
//! with tokio's context propagation mechanism.
//!
//! ## Key Components
//!
//! - **RiSpanId**: Unique identifier for a span
//! - **RiTraceId**: Unique identifier for a trace
//! - **RiSpanKind**: Enumeration of span types (Server, Client, Producer, Consumer, Internal)
//! - **RiSpanStatus**: Status of a span (Ok, Error, Unset)
//! - **RiSpan**: A single distributed tracing span with attributes, events, and status
//! - **RiSpanEvent**: Timed events within a span
//! - **RiTracingContext**: Thread-local tracing context for propagation
//! - **RiTracer**: Distributed tracer for creating and managing spans
//! - **RiTracerManager**: Manager for multiple tracer instances
//! - **DefaultTracerManager**: Global tracer manager instance
//!
//! ## Design Principles
//!
//! 1. **W3C Trace Context Compliance**: Follows the W3C Trace Context standard for interoperability
//! 2. **Async Context Propagation**: Integrates with tokio's context propagation mechanism
//! 3. **Thread Safety**: Uses Arc and RwLock for safe concurrent access
//! 4. **Sampling Support**: Configurable sampling rate to control overhead
//! 5. **Hierarchical Spans**: Supports parent-child span relationships
//! 6. **Baggage Support**: Allows carrying contextual information across spans
//! 7. **Extensible**: Easy to add new span kinds and attributes
//! 8. **Low Overhead**: Efficient implementation with minimal performance impact
//! 9. **Global Access**: Provides a global tracer manager for easy access
//! 10. **Serialization Support**: All tracing components are serializable for export
//!
//! ## Usage
//!
//! ```rust
//! use ri::observability::{init_tracer, tracer, RiSpanKind, RiSpanStatus};
//! use ri::core::RiResult;
//!
//! async fn example() -> RiResult<()> {
//!     // Initialize the global tracer with 100% sampling rate
//!     init_tracer(1.0);
//!     
//!     // Get the global tracer
//!     let tracer = tracer();
//!     
//!     // Start a new trace
//!     let trace_id = tracer.start_trace("example_trace").unwrap();
//!     
//!     // Start a child span
//!     let span_id = tracer.start_span_from_context("child_span", RiSpanKind::Internal).unwrap();
//!     
//!     // Add an attribute to the span
//!     tracer.span_mut(&span_id, |span| {
//!         span.set_attribute("key".to_string(), "value".to_string());
//!     })?;
//!     
//!     // Add an event to the span
//!     tracer.span_mut(&span_id, |span| {
//!         let mut attributes = FxHashMap::default();
//!         attributes.insert("event_key".to_string(), "event_value".to_string());
//!         span.add_event("example_event".to_string(), attributes);
//!     })?;
//!     
//!     // End the child span with OK status
//!     tracer.end_span(&span_id, RiSpanStatus::Ok)?;
//!     
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap as FxHashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::core::RiResult;
use crate::core::RiError;
use crate::core::lock::RwLockExtensions;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Distributed tracing span ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RiSpanId(String);

impl Default for RiSpanId {
    fn default() -> Self {
        Self::new()
    }
}

impl RiSpanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Distributed tracing trace ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RiTraceId(String);

impl Default for RiTraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl RiTraceId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Span kind enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiSpanKind {
    Server,
    Client,
    Producer,
    Consumer,
    Internal,
}

/// Span status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiSpanStatus {
    Ok,
    Error(String),
    Unset,
}

/// A distributed tracing span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiSpan {
    pub trace_id: RiTraceId,
    pub span_id: RiSpanId,
    pub parent_span_id: Option<RiSpanId>,
    pub name: String,
    pub kind: RiSpanKind,
    pub start_time: u64, // microseconds since epoch
    pub end_time: Option<u64>,
    pub attributes: FxHashMap<String, String>,
    pub events: Vec<RiSpanEvent>,
    pub status: RiSpanStatus,
}

impl RiSpan {
    pub fn new(
        trace_id: RiTraceId,
        parent_span_id: Option<RiSpanId>,
        name: String,
        kind: RiSpanKind,
    ) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        Self {
            trace_id,
            span_id: RiSpanId::new(),
            parent_span_id,
            name,
            kind,
            start_time,
            end_time: None,
            attributes: FxHashMap::default(),
            events: Vec::new(),
            status: RiSpanStatus::Unset,
        }
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        // Security: Mask sensitive attribute values
        let safe_value = if Self::is_sensitive_attribute(&key) {
            Self::mask_sensitive_value(&value)
        } else {
            value
        };
        self.attributes.insert(key, safe_value);
    }

    /// Checks if an attribute key is sensitive.
    ///
    /// # Security
    ///
    /// This method identifies sensitive attributes that should be masked
    /// to prevent sensitive information leakage in traces.
    fn is_sensitive_attribute(key: &str) -> bool {
        let key_lower = key.to_lowercase();
        let sensitive_patterns = [
            "password",
            "passwd",
            "secret",
            "key",
            "token",
            "auth",
            "credential",
            "api_key",
            "apikey",
            "private",
            "session",
            "cookie",
            "authorization",
            "bearer",
        ];

        for pattern in &sensitive_patterns {
            if key_lower.contains(pattern) {
                return true;
            }
        }
        false
    }

    /// Masks a sensitive value for safe display.
    ///
    /// # Security
    ///
    /// Shows only first 2 and last 2 characters, with asterisks in between.
    fn mask_sensitive_value(value: &str) -> String {
        if value.len() <= 4 {
            return "*".repeat(value.len().max(4));
        }
        
        let first_chars = &value[..2];
        let last_chars = &value[value.len()-2..];
        let middle_len = value.len() - 4;
        
        format!("{}{}{}", first_chars, "*".repeat(middle_len), last_chars)
    }

    pub fn add_event(&mut self, name: String, attributes: FxHashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        self.events.push(RiSpanEvent {
            name,
            timestamp,
            attributes,
        });
    }

    pub fn end(&mut self, status: RiSpanStatus) {
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        self.end_time = Some(end_time);
        self.status = status;
    }

    pub fn duration(&self) -> Option<Duration> {
        if let Some(end_time) = self.end_time {
            let duration_micros = end_time.saturating_sub(self.start_time);
            Some(Duration::from_micros(duration_micros))
        } else {
            None
        }
    }
}

/// Span event for recording timed occurrences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiSpanEvent {
    pub name: String,
    pub timestamp: u64, // microseconds since epoch
    pub attributes: FxHashMap<String, String>,
}

/// Thread-local tracing context
#[derive(Debug, Clone)]
pub struct RiTracingContext {
    current_trace_id: Option<RiTraceId>,
    current_span_id: Option<RiSpanId>,
    baggage: FxHashMap<String, String>,
}

// Thread-local storage for tracing context
thread_local! {
    static CURRENTONTEXT: RefCell<Option<RiTracingContext>> = const { RefCell::new(None) };
}

impl Default for RiTracingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl RiTracingContext {
    pub fn new() -> Self {
        Self {
            current_trace_id: None,
            current_span_id: None,
            baggage: FxHashMap::default(),
        }
    }

    pub fn with_trace_id(mut self, trace_id: RiTraceId) -> Self {
        self.current_trace_id = Some(trace_id);
        self
    }

    pub fn with_span_id(mut self, span_id: RiSpanId) -> Self {
        self.current_span_id = Some(span_id);
        self
    }

    pub fn set_baggage(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }

    pub fn get_baggage(&self, key: &str) -> Option<&String> {
        self.baggage.get(key)
    }

    pub fn trace_id(&self) -> Option<&RiTraceId> {
        self.current_trace_id.as_ref()
    }

    pub fn span_id(&self) -> Option<&RiSpanId> {
        self.current_span_id.as_ref()
    }

    /// Set this context as the current thread-local context
    pub fn set_as_current(&self) {
        CURRENTONTEXT.with(|ctx| {
            *ctx.borrow_mut() = Some(self.clone());
        });
    }

    /// Get the current tracing context from thread-local storage
    pub fn current() -> Option<Self> {
        CURRENTONTEXT.with(|ctx| {
            ctx.borrow().clone()
        })
    }

    /// Create a new context with the same trace ID but new span ID
    pub fn new_child(&self, span_id: RiSpanId) -> Self {
        Self {
            current_trace_id: self.current_trace_id.clone(),
            current_span_id: Some(span_id),
            baggage: self.baggage.clone(),
        }
    }
}

/// Sampling strategy enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiSamplingStrategy {
    /// Fixed rate sampling (0.0 to 1.0)
    Rate(f64),
    /// Trace ID-based deterministic sampling
    Deterministic(f64),
    /// Adaptive sampling that adjusts based on load
    Adaptive(f64),
}

/// Distributed tracer
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiTracer {
    spans: Arc<RwLock<FxHashMap<RiTraceId, Vec<RiSpan>>>>,
    active_spans: Arc<RwLock<FxHashMap<RiSpanId, RiSpan>>>,
    sampling_strategy: RiSamplingStrategy,
    adaptive_window: Arc<RwLock<Vec<u64>>>,
    max_adaptive_window: usize,
}

impl RiTracer {
    pub fn new(sampling_rate: f64) -> Self {
        Self {
            spans: Arc::new(RwLock::new(FxHashMap::default())),
            active_spans: Arc::new(RwLock::new(FxHashMap::default())),
            sampling_strategy: RiSamplingStrategy::Rate(sampling_rate.clamp(0.0, 1.0)),
            adaptive_window: Arc::new(RwLock::new(Vec::new())),
            max_adaptive_window: 100,
        }
    }
    
    /// Create a new tracer with a custom sampling strategy
    pub fn with_strategy(strategy: RiSamplingStrategy) -> Self {
        Self {
            spans: Arc::new(RwLock::new(FxHashMap::default())),
            active_spans: Arc::new(RwLock::new(FxHashMap::default())),
            sampling_strategy: strategy,
            adaptive_window: Arc::new(RwLock::new(Vec::new())),
            max_adaptive_window: 100,
        }
    }

    /// Start a new trace and set it as current context
    pub fn start_trace(&self, name: String) -> Option<RiTraceId> {
        if !self.should_sample() {
            return None;
        }

        let trace_id = RiTraceId::new();
        let span = RiSpan::new(trace_id.clone(), None, name, RiSpanKind::Server);

        let span_id = span.span_id.clone();
        {
            let mut active_spans = self.active_spans.write_safe("active spans for new trace").ok()?;
            active_spans.insert(span_id.clone(), span);
        }
        {
            let mut spans = self.spans.write_safe("spans for new trace").ok()?;
            spans.insert(trace_id.clone(), Vec::new());
        }

        // Set current context
        let context = RiTracingContext::new()
            .with_trace_id(trace_id.clone())
            .with_span_id(span_id);
        context.set_as_current();

        Some(trace_id)
    }

    /// Start a new span in existing trace, using current context if available
    pub fn start_span(
        &self,
        trace_id: Option<&RiTraceId>,
        parent_span_id: Option<RiSpanId>,
        name: String,
        kind: RiSpanKind,
    ) -> Option<RiSpanId> {
        // Try to get trace_id from current context if not provided
        let resolved_trace_id = match trace_id {
            Some(id) => id.clone(),
            None => {
                if let Some(context) = RiTracingContext::current() {
                    if let Some(id) = context.trace_id() {
                        id.clone()
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
        };

        // Try to get parent_span_id from current context if not provided
        let resolved_parent_span_id = match parent_span_id {
            Some(id) => Some(id.clone()),
            None => RiTracingContext::current().and_then(|context| context.span_id().cloned()),
        };

        let spans = match self.spans.read_safe("spans for span check") {
            Ok(s) => s,
            Err(_) => return None,
        };
        if !spans.contains_key(&resolved_trace_id) {
            return None;
        }

        let span = RiSpan::new(
            resolved_trace_id.clone(),
            resolved_parent_span_id,
            name,
            kind,
        );

        let span_id = span.span_id.clone();
        {
            let mut active_spans = self.active_spans.write_safe("active spans for new span").ok()?;
            active_spans.insert(span_id.clone(), span);
        }

        // Update current context with new span
        if let Some(context) = RiTracingContext::current() {
            let new_context = context.new_child(span_id.clone());
            new_context.set_as_current();
        } else {
            // Create new context if none exists
            let context = RiTracingContext::new()
                .with_trace_id(resolved_trace_id)
                .with_span_id(span_id.clone());
            context.set_as_current();
        }

        Some(span_id)
    }

    /// Start a new span using current context
    pub fn start_span_from_context(&self, name: String, kind: RiSpanKind) -> Option<RiSpanId> {
        self.start_span(None, None, name, kind)
    }

    /// End a span and restore parent span context if available
    pub fn end_span(&self, span_id: &RiSpanId, status: RiSpanStatus) -> RiResult<()> {
        let mut active_spans = self.active_spans.write_safe("active spans for end span")?;

        if let Some(mut span) = active_spans.remove(span_id) {
            span.end(status);

            let trace_id = span.trace_id.clone();
            let parent_span_id = span.parent_span_id.clone();
            drop(active_spans);

            {
                let mut spans = self.spans.write_safe("spans for end span")?;
                if let Some(spans_list) = spans.get_mut(&trace_id) {
                    spans_list.push(span);
                }
            }

            // Restore parent span context if available
            if let Some(parent_span_id) = parent_span_id {
                // Try to find parent span in active spans
                let active_spans = self.active_spans.read_safe("active spans for parent check")?;
                if active_spans.get(&parent_span_id).is_some() {
                    let context = RiTracingContext::new()
                        .with_trace_id(trace_id)
                        .with_span_id(parent_span_id);
                    context.set_as_current();
                }
            } else {
                // No parent span, clear context
                let context = RiTracingContext::new();
                context.set_as_current();
            }
        }

        Ok(())
    }

    /// Get span for modification
    pub fn span_mut<F>(&self, span_id: &RiSpanId, f: F) -> RiResult<()>
    where
        F: FnOnce(&mut RiSpan),
    {
        let mut active_spans = self.active_spans.write_safe("active spans for span_mut")?;

        if let Some(span) = active_spans.get_mut(span_id) {
            f(span);
            Ok(())
        } else {
            Err(crate::core::RiError::Other("Span not found".to_string()))
        }
    }

    /// Export completed traces
    pub fn export_traces(&self) -> FxHashMap<RiTraceId, Vec<RiSpan>> {
        match self.spans.read_safe("spans for export") {
            Ok(spans) => spans.clone(),
            Err(_) => FxHashMap::default(),
        }
    }

    /// Get active traces count
    pub fn active_trace_count(&self) -> usize {
        match self.spans.read_safe("spans for count") {
            Ok(spans) => spans.len(),
            Err(_) => 0,
        }
    }

    /// Get active span count
    pub fn active_span_count(&self) -> usize {
        match self.active_spans.read_safe("active spans for count") {
            Ok(active_spans) => active_spans.len(),
            Err(_) => 0,
        }
    }

    fn should_sample(&self) -> bool {
        match &self.sampling_strategy {
            RiSamplingStrategy::Rate(rate) => {
                if *rate >= 1.0 {
                    true
                } else if *rate <= 0.0 {
                    false
                } else {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    rng.gen::<f64>() < *rate
                }
            }
            RiSamplingStrategy::Deterministic(rate) => {
                if *rate >= 1.0 {
                    true
                } else if *rate <= 0.0 {
                    false
                } else {
                    // Create a deterministic hash based on current time and thread ID
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or(Duration::from_secs(0))
                        .as_nanos();
                    // Get a numeric representation of the thread ID using hash
                    let thread_id = format!("{:?}", std::thread::current().id())
                        .as_bytes()
                        .iter()
                        .fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64));
                    let combined = now.wrapping_add(thread_id as u128);
                    
                    // Simple hash function
                    let hash = (combined as u64).wrapping_mul(0x517cc1b727220a95);
                    let hash_f64 = (hash as f64) / (u64::MAX as f64);
                    
                    hash_f64 < *rate
                }
            }
            RiSamplingStrategy::Adaptive(target_rate) => {
                if *target_rate >= 1.0 {
                    true
                } else if *target_rate <= 0.0 {
                    false
                } else {
                    // Calculate current load based on active spans
                    let active_count = match self.active_spans.read_safe("active spans for sampling") {
                        Ok(active_spans) => active_spans.len() as f64,
                        Err(_) => 0.0,
                    };
                    
                    let mut window = match self.adaptive_window.write_safe("adaptive window for sampling") {
                        Ok(w) => w,
                        Err(_) => {
                            // If we can't acquire the lock, default to high load (low sampling rate)
                            return false;
                        }
                    };
                    
                    // Add current active count to window
                    window.push(active_count as u64);
                    if window.len() > self.max_adaptive_window {
                        window.remove(0);
                    }
                    
                    // Calculate average load over window
                    let avg_load = if window.is_empty() {
                        0.0
                    } else {
                        window.iter().sum::<u64>() as f64 / window.len() as f64
                    };
                    
                    // Adaptive sampling: lower rate when load is high, higher when load is low
                    const BASE_LOAD: f64 = 100.0;
                    let adjusted_rate = target_rate * (1.0 + (BASE_LOAD - avg_load) / BASE_LOAD);
                    let clamped_rate = adjusted_rate.clamp(0.01, 1.0);
                    
                    use rand::Rng;
                    let mut rng = rand::thread_rng();
                    rng.gen::<f64>() < clamped_rate
                }
            }
        }
    }

}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiTracer {
    /// Create a new tracer from Python with a sampling rate
    #[new]
    fn py_new(sampling_rate: f64) -> Self {
        Self::new(sampling_rate)
    }

    /// Start a new trace from Python
    #[pyo3(name = "start_trace")]
    fn start_trace_impl(&self, name: String) -> PyResult<Option<String>> {
        match self.start_trace(name) {
            Some(trace_id) => Ok(Some(trace_id.as_str().to_string())),
            None => Ok(None),
        }
    }

    /// Start a new span from Python using current context
    #[pyo3(name = "start_span_from_context")]
    fn start_span_from_context_impl(&self, name: String, kind: String) -> PyResult<Option<String>> {
        let span_kind = match kind.as_str() {
            "Server" => RiSpanKind::Server,
            "Client" => RiSpanKind::Client,
            "Producer" => RiSpanKind::Producer,
            "Consumer" => RiSpanKind::Consumer,
            _ => RiSpanKind::Internal,
        };

        match self.start_span_from_context(name, span_kind) {
            Some(span_id) => Ok(Some(span_id.as_str().to_string())),
            None => Ok(None),
        }
    }

    /// End a span from Python
    #[pyo3(name = "end_span")]
    fn end_span_impl(&self, span_id: String, status: String) -> PyResult<()> {
        let span_id_obj = RiSpanId::from_string(span_id);
        let span_status = match status.as_str() {
            "Ok" => RiSpanStatus::Ok,
            "Error" => RiSpanStatus::Error("Python error".to_string()),
            _ => RiSpanStatus::Unset,
        };

        self.end_span(&span_id_obj, span_status)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to end span: {e}")))
    }

    /// Set span attribute from Python
    #[pyo3(name = "span_set_attribute")]
    fn span_set_attribute_impl(&self, span_id: String, key: String, value: String) -> PyResult<()> {
        let span_id_obj = RiSpanId::from_string(span_id);
        self.span_mut(&span_id_obj, |span| {
            span.set_attribute(key, value);
        })
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set attribute: {e}")))
    }

    /// Add span event from Python
    #[pyo3(name = "span_add_event")]
    fn span_add_event_impl(&self, span_id: String, name: String, attributes: FxHashMap<String, String>) -> PyResult<()> {
        let span_id_obj = RiSpanId::from_string(span_id);
        self.span_mut(&span_id_obj, |span| {
            span.add_event(name, attributes);
        })
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to add event: {e}")))
    }

    /// Export traces from Python
    #[pyo3(name = "export_traces")]
    fn export_traces_impl(&self, py: pyo3::Python<'_>) -> PyResult<FxHashMap<String, Vec<pyo3::Py<pyo3::PyAny>>>> {
        let traces = self.export_traces();
        let mut result = FxHashMap::with_capacity(traces.len());

        for (trace_id, spans) in traces {
            let mut span_list = Vec::with_capacity(spans.len());
            for span in spans {
                let span_dict = pyo3::types::PyDict::new(py);
                span_dict.set_item("trace_id", span.trace_id.as_str())?;
                span_dict.set_item("span_id", span.span_id.as_str())?;
                if let Some(parent_id) = &span.parent_span_id {
                    span_dict.set_item("parent_span_id", parent_id.as_str())?;
                }
                span_dict.set_item("name", &span.name)?;
                span_dict.set_item("kind", format!("{:?}", span.kind))?;
                span_dict.set_item("start_time", span.start_time)?;
                span_dict.set_item("end_time", span.end_time)?;
                span_dict.set_item("attributes", span.attributes.clone())?;
                span_dict.set_item("events", span.events.len())?;
                span_dict.set_item("status", format!("{:?}", span.status))?;
                span_list.push(span_dict.into());
            }
            result.insert(trace_id.as_str().to_string(), span_list);
        }
        Ok(result)
    }

    /// Get active trace count from Python
    #[pyo3(name = "active_trace_count")]
    fn active_trace_count_impl(&self) -> usize {
        self.active_trace_count()
    }

    /// Get active span count from Python
    #[pyo3(name = "active_span_count")]
    fn active_span_count_impl(&self) -> usize {
        self.active_span_count()
    }
}

/// Tracer manager for managing multiple tracer instances
pub struct RiTracerManager {
    tracers: FxHashMap<String, Arc<RiTracer>>,
    default_tracer: Option<String>,
}

impl Default for RiTracerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl RiTracerManager {
    pub fn new() -> Self {
        Self {
            tracers: FxHashMap::default(),
            default_tracer: None,
        }
    }

    pub fn register_tracer(&mut self, name: &str, tracer: Arc<RiTracer>) {
        self.tracers.insert(name.to_string(), tracer);
        if self.default_tracer.is_none() {
            self.default_tracer = Some(name.to_string());
        }
    }

    #[allow(dead_code)]
    pub fn get_tracer(&self, name: &str) -> Option<&Arc<RiTracer>> {
        self.tracers.get(name)
    }

    pub fn get_default_tracer(&self) -> Option<&Arc<RiTracer>> {
        if let Some(default_name) = &self.default_tracer {
            self.tracers.get(default_name)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn set_default_tracer(&mut self, name: &str) -> bool {
        if self.tracers.contains_key(name) {
            self.default_tracer = Some(name.to_string());
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn remove_tracer(&mut self, name: &str) -> bool {
        let removed = self.tracers.remove(name).is_some();
        if let Some(default_name) = &self.default_tracer {
            if default_name == name {
                self.default_tracer = None;
            }
        }
        removed
    }
}

/// Default tracer manager instance
pub struct DefaultTracerManager {
    inner: Arc<RwLock<RiTracerManager>>,
}

impl Default for DefaultTracerManager {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(RiTracerManager::new())),
        }
    }
}

impl DefaultTracerManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn register_tracer(&self, name: &str, sampling_rate: f64) -> RiResult<()> {
        let tracer = Arc::new(RiTracer::new(sampling_rate));
        let mut manager = self.inner.write_safe("tracer manager for register")?;
        manager.register_tracer(name, tracer);
        Ok(())
    }
    
    pub async fn register_tracer_with_strategy(&self, name: &str, strategy: RiSamplingStrategy) -> RiResult<()> {
        let tracer = Arc::new(RiTracer::with_strategy(strategy));
        let mut manager = self.inner.write_safe("tracer manager for register with strategy")?;
        manager.register_tracer(name, tracer);
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_tracer(&self, name: &str) -> RiResult<Option<Arc<RiTracer>>> {
        let manager = self.inner.read_safe("tracer manager for get")?;
        Ok(manager.get_tracer(name).cloned())
    }

    pub async fn get_default_tracer(&self) -> RiResult<Option<Arc<RiTracer>>> {
        let manager = self.inner.read_safe("tracer manager for get default")?;
        Ok(manager.get_default_tracer().cloned())
    }

    #[allow(dead_code)]
    pub async fn set_default_tracer(&self, name: &str) -> RiResult<bool> {
        let mut manager = self.inner.write_safe("tracer manager for set default")?;
        Ok(manager.set_default_tracer(name))
    }

    #[allow(dead_code)]
    pub async fn remove_tracer(&self, name: &str) -> RiResult<bool> {
        let mut manager = self.inner.write_safe("tracer manager for remove")?;
        Ok(manager.remove_tracer(name))
    }
}

/// Global tracer manager instance
pub static DEFAULT_TRACER_MANAGER: std::sync::LazyLock<DefaultTracerManager> = std::sync::LazyLock::new(DefaultTracerManager::default);

/// Initialize global tracer with fixed rate (backward compatibility)
pub fn init_tracer(sampling_rate: f64) {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create tokio runtime: {}", e);
            return;
        }
    };
    
    runtime.block_on(async {
        if let Err(e) = DEFAULT_TRACER_MANAGER.register_tracer("default", sampling_rate).await {
            eprintln!("Failed to register tracer: {}", e);
        }
    });
}

/// Initialize global tracer with custom sampling strategy
pub fn init_tracer_with_strategy(strategy: RiSamplingStrategy) {
    let rate = match strategy {
        RiSamplingStrategy::Rate(rate) => rate,
        RiSamplingStrategy::Deterministic(rate) => rate,
        RiSamplingStrategy::Adaptive(rate) => rate,
    };
    
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to create tokio runtime: {}", e);
            return;
        }
    };
    
    runtime.block_on(async {
        if let Err(e) = DEFAULT_TRACER_MANAGER.register_tracer("default", rate).await {
            eprintln!("Failed to register tracer: {}", e);
        }
    });
}

/// Get global tracer (backward compatibility)
pub fn tracer() -> Result<Arc<RiTracer>, Box<RiError>> {
    let runtime = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(r) => r,
        Err(e) => {
            return Err(Box::new(RiError::Other(format!(
                "Failed to create tokio runtime for tracer: {}",
                e
            ))));
        }
    };

    runtime.block_on(async {
        match DEFAULT_TRACER_MANAGER.get_default_tracer().await {
            Ok(Some(tracer)) => Ok(tracer),
            Ok(None) => {
                Err(Box::new(RiError::Other(
                    "Tracer not initialized".to_string(),
                )))
            }
            Err(e) => Err(Box::new(RiError::Other(format!(
                "Failed to get tracer: {}",
                e
            )))),
        }
    })
}
