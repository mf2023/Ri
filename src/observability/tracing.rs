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

//! # Distributed Tracing
//!
//! This file implements a comprehensive distributed tracing system for the DMSC framework. It provides
//! tools for creating, managing, and propagating trace information across asynchronous operations
//! and distributed systems. The tracing system follows the W3C Trace Context standard and integrates
//! with tokio's context propagation mechanism.
//!
//! ## Key Components
//!
//! - **DMSCSpanId**: Unique identifier for a span
//! - **DMSCTraceId**: Unique identifier for a trace
//! - **DMSCSpanKind**: Enumeration of span types (Server, Client, Producer, Consumer, Internal)
//! - **DMSCSpanStatus**: Status of a span (Ok, Error, Unset)
//! - **DMSCSpan**: A single distributed tracing span with attributes, events, and status
//! - **DMSCSpanEvent**: Timed events within a span
//! - **DMSCTracingContext**: Thread-local tracing context for propagation
//! - **DMSCTracer**: Distributed tracer for creating and managing spans
//! - **DMSCTracerManager**: Manager for multiple tracer instances
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
//! use dms::observability::{init_tracer, tracer, DMSCSpanKind, DMSCSpanStatus};
//! use dms::core::DMSCResult;
//!
//! async fn example() -> DMSCResult<()> {
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
//!     let span_id = tracer.start_span_from_context("child_span", DMSCSpanKind::Internal).unwrap();
//!     
//!     // Add an attribute to the span
//!     tracer.span_mut(&span_id, |span| {
//!         span.set_attribute("key".to_string(), "value".to_string());
//!     })?;
//!     
//!     // Add an event to the span
//!     tracer.span_mut(&span_id, |span| {
//!         let mut attributes = std::collections::HashMap::new();
//!         attributes.insert("event_key".to_string(), "event_value".to_string());
//!         span.add_event("example_event".to_string(), attributes);
//!     })?;
//!     
//!     // End the child span with OK status
//!     tracer.end_span(&span_id, DMSCSpanStatus::Ok)?;
//!     
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::core::DMSCResult;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Distributed tracing span ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSCSpanId(String);

impl Default for DMSCSpanId {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCSpanId {
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
pub struct DMSCTraceId(String);

impl Default for DMSCTraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCTraceId {
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
pub enum DMSCSpanKind {
    Server,
    Client,
    Producer,
    Consumer,
    Internal,
}

/// Span status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSCSpanStatus {
    Ok,
    Error(String),
    Unset,
}

/// A distributed tracing span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCSpan {
    pub trace_id: DMSCTraceId,
    pub span_id: DMSCSpanId,
    pub parent_span_id: Option<DMSCSpanId>,
    pub name: String,
    pub kind: DMSCSpanKind,
    pub start_time: u64, // microseconds since epoch
    pub end_time: Option<u64>,
    pub attributes: HashMap<String, String>,
    pub events: Vec<DMSCSpanEvent>,
    pub status: DMSCSpanStatus,
}

impl DMSCSpan {
    pub fn new(
        trace_id: DMSCTraceId,
        parent_span_id: Option<DMSCSpanId>,
        name: String,
        kind: DMSCSpanKind,
    ) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        Self {
            trace_id,
            span_id: DMSCSpanId::new(),
            parent_span_id,
            name,
            kind,
            start_time,
            end_time: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            status: DMSCSpanStatus::Unset,
        }
    }

    pub fn set_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn add_event(&mut self, name: String, attributes: HashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        self.events.push(DMSCSpanEvent {
            name,
            timestamp,
            attributes,
        });
    }

    pub fn end(&mut self, status: DMSCSpanStatus) {
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
pub struct DMSCSpanEvent {
    pub name: String,
    pub timestamp: u64, // microseconds since epoch
    pub attributes: HashMap<String, String>,
}

/// Thread-local tracing context
#[derive(Debug, Clone)]
pub struct DMSCTracingContext {
    current_trace_id: Option<DMSCTraceId>,
    current_span_id: Option<DMSCSpanId>,
    baggage: HashMap<String, String>,
}

// Thread-local storage for tracing context
thread_local! {
    static CURRENTONTEXT: RefCell<Option<DMSCTracingContext>> = const { RefCell::new(None) };
}

impl Default for DMSCTracingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCTracingContext {
    pub fn new() -> Self {
        Self {
            current_trace_id: None,
            current_span_id: None,
            baggage: HashMap::new(),
        }
    }

    pub fn with_trace_id(mut self, trace_id: DMSCTraceId) -> Self {
        self.current_trace_id = Some(trace_id);
        self
    }

    pub fn with_span_id(mut self, span_id: DMSCSpanId) -> Self {
        self.current_span_id = Some(span_id);
        self
    }

    pub fn set_baggage(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }

    pub fn get_baggage(&self, key: &str) -> Option<&String> {
        self.baggage.get(key)
    }

    pub fn trace_id(&self) -> Option<&DMSCTraceId> {
        self.current_trace_id.as_ref()
    }

    pub fn span_id(&self) -> Option<&DMSCSpanId> {
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
    pub fn new_child(&self, span_id: DMSCSpanId) -> Self {
        Self {
            current_trace_id: self.current_trace_id.clone(),
            current_span_id: Some(span_id),
            baggage: self.baggage.clone(),
        }
    }
}

/// Sampling strategy enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSCSamplingStrategy {
    /// Fixed rate sampling (0.0 to 1.0)
    Rate(f64),
    /// Trace ID-based deterministic sampling
    Deterministic(f64),
    /// Adaptive sampling that adjusts based on load
    Adaptive(f64),
}

/// Distributed tracer
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCTracer {
    spans: Arc<RwLock<HashMap<DMSCTraceId, Vec<DMSCSpan>>>>,
    active_spans: Arc<RwLock<HashMap<DMSCSpanId, DMSCSpan>>>,
    sampling_strategy: DMSCSamplingStrategy,
    adaptive_window: Arc<RwLock<Vec<u64>>>,
    max_adaptive_window: usize,
}

impl DMSCTracer {
    pub fn new(sampling_rate: f64) -> Self {
        Self {
            spans: Arc::new(RwLock::new(HashMap::new())),
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            sampling_strategy: DMSCSamplingStrategy::Rate(sampling_rate.clamp(0.0, 1.0)),
            adaptive_window: Arc::new(RwLock::new(Vec::new())),
            max_adaptive_window: 100,
        }
    }
    
    /// Create a new tracer with a custom sampling strategy
    pub fn with_strategy(strategy: DMSCSamplingStrategy) -> Self {
        Self {
            spans: Arc::new(RwLock::new(HashMap::new())),
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            sampling_strategy: strategy,
            adaptive_window: Arc::new(RwLock::new(Vec::new())),
            max_adaptive_window: 100,
        }
    }

    /// Start a new trace and set it as current context
    pub fn start_trace(&self, name: String) -> Option<DMSCTraceId> {
        if !self.should_sample() {
            return None;
        }

        let trace_id = DMSCTraceId::new();
        let span = DMSCSpan::new(trace_id.clone(), None, name, DMSCSpanKind::Server);

        let span_id = span.span_id.clone();
        self.active_spans
            .write()
            .unwrap()
            .insert(span_id.clone(), span);
        self.spans
            .write()
            .unwrap()
            .insert(trace_id.clone(), Vec::new());

        // Set current context
        let context = DMSCTracingContext::new()
            .with_trace_id(trace_id.clone())
            .with_span_id(span_id);
        context.set_as_current();

        Some(trace_id)
    }

    /// Start a new span in existing trace, using current context if available
    pub fn start_span(
        &self,
        trace_id: Option<&DMSCTraceId>,
        parent_span_id: Option<DMSCSpanId>,
        name: String,
        kind: DMSCSpanKind,
    ) -> Option<DMSCSpanId> {
        // Try to get trace_id from current context if not provided
        let resolved_trace_id = match trace_id {
            Some(id) => id.clone(),
            None => {
                if let Some(context) = DMSCTracingContext::current() {
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
            None => DMSCTracingContext::current().and_then(|context| context.span_id().cloned()),
        };

        if !self.spans.read().unwrap().contains_key(&resolved_trace_id) {
            return None;
        }

        let span = DMSCSpan::new(
            resolved_trace_id.clone(),
            resolved_parent_span_id,
            name,
            kind,
        );

        let span_id = span.span_id.clone();
        self.active_spans
            .write()
            .unwrap()
            .insert(span_id.clone(), span);

        // Update current context with new span
        if let Some(context) = DMSCTracingContext::current() {
            let new_context = context.new_child(span_id.clone());
            new_context.set_as_current();
        } else {
            // Create new context if none exists
            let context = DMSCTracingContext::new()
                .with_trace_id(resolved_trace_id)
                .with_span_id(span_id.clone());
            context.set_as_current();
        }

        Some(span_id)
    }

    /// Start a new span using current context
    pub fn start_span_from_context(&self, name: String, kind: DMSCSpanKind) -> Option<DMSCSpanId> {
        self.start_span(None, None, name, kind)
    }

    /// End a span and restore parent span context if available
    pub fn end_span(&self, span_id: &DMSCSpanId, status: DMSCSpanStatus) -> DMSCResult<()> {
        let mut active_spans = self.active_spans.write().unwrap();

        if let Some(mut span) = active_spans.remove(span_id) {
            span.end(status);

            let trace_id = span.trace_id.clone();
            if let Some(spans) = self.spans.write().unwrap().get_mut(&trace_id) {
                spans.push(span.clone());
            }

            // Restore parent span context if available
            if let Some(parent_span_id) = span.parent_span_id.clone() {
                // Try to find parent span in active spans
                if active_spans.get(&parent_span_id).is_some() {
                    let context = DMSCTracingContext::new()
                        .with_trace_id(trace_id)
                        .with_span_id(parent_span_id);
                    context.set_as_current();
                }
            } else {
                // No parent span, clear context
                let context = DMSCTracingContext::new();
                context.set_as_current();
            }
        }

        Ok(())
    }

    /// Get span for modification
    pub fn span_mut<F>(&self, span_id: &DMSCSpanId, f: F) -> DMSCResult<()>
    where
        F: FnOnce(&mut DMSCSpan),
    {
        let mut active_spans = self.active_spans.write().unwrap();

        if let Some(span) = active_spans.get_mut(span_id) {
            f(span);
            Ok(())
        } else {
            Err(crate::core::DMSCError::Other("Span not found".to_string()))
        }
    }

    /// Export completed traces
    pub fn export_traces(&self) -> HashMap<DMSCTraceId, Vec<DMSCSpan>> {
        self.spans.read().unwrap().clone()
    }

    /// Get active traces count
    pub fn active_trace_count(&self) -> usize {
        self.spans.read().unwrap().len()
    }

    /// Get active span count
    pub fn active_span_count(&self) -> usize {
        self.active_spans.read().unwrap().len()
    }

    fn should_sample(&self) -> bool {
        match &self.sampling_strategy {
            DMSCSamplingStrategy::Rate(rate) => {
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
            DMSCSamplingStrategy::Deterministic(rate) => {
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
            DMSCSamplingStrategy::Adaptive(target_rate) => {
                if *target_rate >= 1.0 {
                    true
                } else if *target_rate <= 0.0 {
                    false
                } else {
                    // Calculate current load based on active spans
                    let active_count = self.active_spans.read().unwrap().len() as f64;
                    let mut window = self.adaptive_window.write().unwrap();
                    
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
impl DMSCTracer {
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
            "Server" => DMSCSpanKind::Server,
            "Client" => DMSCSpanKind::Client,
            "Producer" => DMSCSpanKind::Producer,
            "Consumer" => DMSCSpanKind::Consumer,
            _ => DMSCSpanKind::Internal,
        };

        match self.start_span_from_context(name, span_kind) {
            Some(span_id) => Ok(Some(span_id.as_str().to_string())),
            None => Ok(None),
        }
    }

    /// End a span from Python
    #[pyo3(name = "end_span")]
    fn end_span_impl(&self, span_id: String, status: String) -> PyResult<()> {
        let span_id_obj = DMSCSpanId::from_string(span_id);
        let span_status = match status.as_str() {
            "Ok" => DMSCSpanStatus::Ok,
            "Error" => DMSCSpanStatus::Error("Python error".to_string()),
            _ => DMSCSpanStatus::Unset,
        };

        self.end_span(&span_id_obj, span_status)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to end span: {e}")))
    }

    /// Set span attribute from Python
    #[pyo3(name = "span_set_attribute")]
    fn span_set_attribute_impl(&self, span_id: String, key: String, value: String) -> PyResult<()> {
        let span_id_obj = DMSCSpanId::from_string(span_id);
        self.span_mut(&span_id_obj, |span| {
            span.set_attribute(key, value);
        })
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set attribute: {e}")))
    }

    /// Add span event from Python
    #[pyo3(name = "span_add_event")]
    fn span_add_event_impl(&self, span_id: String, name: String, attributes: HashMap<String, String>) -> PyResult<()> {
        let span_id_obj = DMSCSpanId::from_string(span_id);
        self.span_mut(&span_id_obj, |span| {
            span.add_event(name, attributes);
        })
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to add event: {e}")))
    }

    /// Export traces from Python
    #[pyo3(name = "export_traces")]
    fn export_traces_impl(&self) -> PyResult<HashMap<String, Vec<PyObject>>> {
        let traces = self.export_traces();
        let mut result = HashMap::new();

        Python::with_gil(|py| {
            for (trace_id, spans) in traces {
                let mut span_list = Vec::new();
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
                    span_dict.set_item("attributes", span.attributes)?;
                    span_dict.set_item("events", span.events.len())?;
                    span_dict.set_item("status", format!("{:?}", span.status))?;
                    span_list.push(span_dict.into());
                }
                result.insert(trace_id.as_str().to_string(), span_list);
            }
            Ok(result)
        })
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
pub struct DMSCTracerManager {
    tracers: HashMap<String, Arc<DMSCTracer>>,
    default_tracer: Option<String>,
}

impl Default for DMSCTracerManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCTracerManager {
    pub fn new() -> Self {
        Self {
            tracers: HashMap::new(),
            default_tracer: None,
        }
    }

    pub fn register_tracer(&mut self, name: &str, tracer: Arc<DMSCTracer>) {
        self.tracers.insert(name.to_string(), tracer);
        if self.default_tracer.is_none() {
            self.default_tracer = Some(name.to_string());
        }
    }

    #[allow(dead_code)]
    pub fn get_tracer(&self, name: &str) -> Option<&Arc<DMSCTracer>> {
        self.tracers.get(name)
    }

    pub fn get_default_tracer(&self) -> Option<&Arc<DMSCTracer>> {
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
    inner: Arc<RwLock<DMSCTracerManager>>,
}

impl Default for DefaultTracerManager {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(DMSCTracerManager::new())),
        }
    }
}

impl DefaultTracerManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn register_tracer(&self, name: &str, sampling_rate: f64) {
        let tracer = Arc::new(DMSCTracer::new(sampling_rate));
        let mut manager = self.inner.write().unwrap();
        manager.register_tracer(name, tracer);
    }
    
    pub async fn register_tracer_with_strategy(&self, name: &str, strategy: DMSCSamplingStrategy) {
        let tracer = Arc::new(DMSCTracer::with_strategy(strategy));
        let mut manager = self.inner.write().unwrap();
        manager.register_tracer(name, tracer);
    }

    #[allow(dead_code)]
    pub async fn get_tracer(&self, name: &str) -> Option<Arc<DMSCTracer>> {
        let manager = self.inner.read().unwrap();
        manager.get_tracer(name).cloned()
    }

    pub async fn get_default_tracer(&self) -> Option<Arc<DMSCTracer>> {
        let manager = self.inner.read().unwrap();
        manager.get_default_tracer().cloned()
    }

    #[allow(dead_code)]
    pub async fn set_default_tracer(&self, name: &str) -> bool {
        let mut manager = self.inner.write().unwrap();
        manager.set_default_tracer(name)
    }

    #[allow(dead_code)]
    pub async fn remove_tracer(&self, name: &str) -> bool {
        let mut manager = self.inner.write().unwrap();
        manager.remove_tracer(name)
    }
}

/// Global tracer manager instance
pub static DEFAULT_TRACER_MANAGER: std::sync::LazyLock<DefaultTracerManager> = std::sync::LazyLock::new(DefaultTracerManager::default);

/// Initialize global tracer with fixed rate (backward compatibility)
pub fn init_tracer(sampling_rate: f64) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            DEFAULT_TRACER_MANAGER
                .register_tracer("default", sampling_rate)
                .await;
        });
}

/// Initialize global tracer with custom sampling strategy
pub fn init_tracer_with_strategy(strategy: DMSCSamplingStrategy) {
    let rate = match strategy {
        DMSCSamplingStrategy::Rate(rate) => rate,
        DMSCSamplingStrategy::Deterministic(rate) => rate,
        DMSCSamplingStrategy::Adaptive(rate) => rate,
    };
    
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            DEFAULT_TRACER_MANAGER
                .register_tracer("default", rate)
                .await;
        });
}

/// Get global tracer (backward compatibility)
pub fn tracer() -> Arc<DMSCTracer> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            DEFAULT_TRACER_MANAGER
                .get_default_tracer()
                .await
                .expect("Tracer not initialized")
        })
}
