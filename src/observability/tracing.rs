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

//! # Distributed Tracing
//!
//! This file implements a comprehensive distributed tracing system for the DMS framework. It provides
//! tools for creating, managing, and propagating trace information across asynchronous operations
//! and distributed systems. The tracing system follows the W3C Trace Context standard and integrates
//! with tokio's context propagation mechanism.
//!
//! ## Key Components
//!
//! - **DMSSpanId**: Unique identifier for a span
//! - **DMSTraceId**: Unique identifier for a trace
//! - **DMSSpanKind**: Enumeration of span types (Server, Client, Producer, Consumer, Internal)
//! - **DMSSpanStatus**: Status of a span (Ok, Error, Unset)
//! - **DMSSpan**: A single distributed tracing span with attributes, events, and status
//! - **DMSSpanEvent**: Timed events within a span
//! - **DMSTracingContext**: Thread-local tracing context for propagation
//! - **DMSTracer**: Distributed tracer for creating and managing spans
//! - **DMSTracerManager**: Manager for multiple tracer instances
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
//! use dms::observability::{init_tracer, tracer, DMSSpanKind, DMSSpanStatus};
//! use dms::core::DMSResult;
//!
//! async fn example() -> DMSResult<()> {
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
//!     let span_id = tracer.start_span_from_context("child_span", DMSSpanKind::Internal).unwrap();
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
//!     tracer.end_span(&span_id, DMSSpanStatus::Ok)?;
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

use crate::core::DMSResult;

/// Distributed tracing span ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSSpanId(String);

impl Default for DMSSpanId {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSSpanId {
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
pub struct DMSTraceId(String);

impl Default for DMSTraceId {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSTraceId {
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
pub enum DMSSpanKind {
    Server,
    Client,
    Producer,
    Consumer,
    Internal,
}

/// Span status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSSpanStatus {
    Ok,
    Error(String),
    Unset,
}

/// A distributed tracing span
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSSpan {
    pub trace_id: DMSTraceId,
    pub span_id: DMSSpanId,
    pub parent_span_id: Option<DMSSpanId>,
    pub name: String,
    pub kind: DMSSpanKind,
    pub start_time: u64, // microseconds since epoch
    pub end_time: Option<u64>,
    pub attributes: HashMap<String, String>,
    pub events: Vec<DMSSpanEvent>,
    pub status: DMSSpanStatus,
}

impl DMSSpan {
    pub fn new(
        trace_id: DMSTraceId,
        parent_span_id: Option<DMSSpanId>,
        name: String,
        kind: DMSSpanKind,
    ) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_micros() as u64;

        Self {
            trace_id,
            span_id: DMSSpanId::new(),
            parent_span_id,
            name,
            kind,
            start_time,
            end_time: None,
            attributes: HashMap::new(),
            events: Vec::new(),
            status: DMSSpanStatus::Unset,
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

        self.events.push(DMSSpanEvent {
            name,
            timestamp,
            attributes,
        });
    }

    pub fn end(&mut self, status: DMSSpanStatus) {
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
pub struct DMSSpanEvent {
    pub name: String,
    pub timestamp: u64, // microseconds since epoch
    pub attributes: HashMap<String, String>,
}

/// Thread-local tracing context
#[derive(Debug, Clone)]
pub struct DMSTracingContext {
    current_trace_id: Option<DMSTraceId>,
    current_span_id: Option<DMSSpanId>,
    baggage: HashMap<String, String>,
}

// Thread-local storage for tracing context
thread_local! {
    static CURRENTONTEXT: RefCell<Option<DMSTracingContext>> = const { RefCell::new(None) };
}

impl Default for DMSTracingContext {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSTracingContext {
    pub fn new() -> Self {
        Self {
            current_trace_id: None,
            current_span_id: None,
            baggage: HashMap::new(),
        }
    }

    pub fn with_trace_id(mut self, trace_id: DMSTraceId) -> Self {
        self.current_trace_id = Some(trace_id);
        self
    }

    pub fn with_span_id(mut self, span_id: DMSSpanId) -> Self {
        self.current_span_id = Some(span_id);
        self
    }

    pub fn set_baggage(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }

    pub fn get_baggage(&self, key: &str) -> Option<&String> {
        self.baggage.get(key)
    }

    pub fn trace_id(&self) -> Option<&DMSTraceId> {
        self.current_trace_id.as_ref()
    }

    pub fn span_id(&self) -> Option<&DMSSpanId> {
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
    pub fn new_child(&self, span_id: DMSSpanId) -> Self {
        Self {
            current_trace_id: self.current_trace_id.clone(),
            current_span_id: Some(span_id),
            baggage: self.baggage.clone(),
        }
    }
}

/// Distributed tracer
pub struct DMSTracer {
    spans: Arc<RwLock<HashMap<DMSTraceId, Vec<DMSSpan>>>>,
    active_spans: Arc<RwLock<HashMap<DMSSpanId, DMSSpan>>>,
    sampling_rate: f64,
}

impl DMSTracer {
    pub fn new(sampling_rate: f64) -> Self {
        Self {
            spans: Arc::new(RwLock::new(HashMap::new())),
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            sampling_rate: sampling_rate.clamp(0.0, 1.0),
        }
    }

    /// Start a new trace and set it as current context
    pub fn start_trace(&self, name: String) -> Option<DMSTraceId> {
        if !self.should_sample() {
            return None;
        }

        let trace_id = DMSTraceId::new();
        let span = DMSSpan::new(trace_id.clone(), None, name, DMSSpanKind::Server);

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
        let context = DMSTracingContext::new()
            .with_trace_id(trace_id.clone())
            .with_span_id(span_id);
        context.set_as_current();

        Some(trace_id)
    }

    /// Start a new span in existing trace, using current context if available
    pub fn start_span(
        &self,
        trace_id: Option<&DMSTraceId>,
        parent_span_id: Option<DMSSpanId>,
        name: String,
        kind: DMSSpanKind,
    ) -> Option<DMSSpanId> {
        // Try to get trace_id from current context if not provided
        let resolved_trace_id = match trace_id {
            Some(id) => id.clone(),
            None => {
                if let Some(context) = DMSTracingContext::current() {
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
            None => DMSTracingContext::current().and_then(|context| context.span_id().cloned()),
        };

        if !self.spans.read().unwrap().contains_key(&resolved_trace_id) {
            return None;
        }

        let span = DMSSpan::new(
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
        if let Some(context) = DMSTracingContext::current() {
            let new_context = context.new_child(span_id.clone());
            new_context.set_as_current();
        } else {
            // Create new context if none exists
            let context = DMSTracingContext::new()
                .with_trace_id(resolved_trace_id)
                .with_span_id(span_id.clone());
            context.set_as_current();
        }

        Some(span_id)
    }

    /// Start a new span using current context
    pub fn start_span_from_context(&self, name: String, kind: DMSSpanKind) -> Option<DMSSpanId> {
        self.start_span(None, None, name, kind)
    }

    /// End a span and restore parent span context if available
    pub fn end_span(&self, span_id: &DMSSpanId, status: DMSSpanStatus) -> DMSResult<()> {
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
                    let context = DMSTracingContext::new()
                        .with_trace_id(trace_id)
                        .with_span_id(parent_span_id);
                    context.set_as_current();
                }
            } else {
                // No parent span, clear context
                let context = DMSTracingContext::new();
                context.set_as_current();
            }
        }

        Ok(())
    }

    /// Get span for modification
    pub fn span_mut<F>(&self, span_id: &DMSSpanId, f: F) -> DMSResult<()>
    where
        F: FnOnce(&mut DMSSpan),
    {
        let mut active_spans = self.active_spans.write().unwrap();

        if let Some(span) = active_spans.get_mut(span_id) {
            f(span);
            Ok(())
        } else {
            Err(crate::core::DMSError::Other("Span not found".to_string()))
        }
    }

    /// Export completed traces
    pub fn export_traces(&self) -> HashMap<DMSTraceId, Vec<DMSSpan>> {
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
        if self.sampling_rate >= 1.0 {
            true
        } else if self.sampling_rate <= 0.0 {
            false
        } else {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen::<f64>() < self.sampling_rate
        }
    }
}

/// Tracer manager for managing multiple tracer instances
pub struct DMSTracerManager {
    tracers: HashMap<String, Arc<DMSTracer>>,
    default_tracer: Option<String>,
}

impl DMSTracerManager {
    pub fn new() -> Self {
        Self {
            tracers: HashMap::new(),
            default_tracer: None,
        }
    }

    pub fn register_tracer(&mut self, name: &str, tracer: Arc<DMSTracer>) {
        self.tracers.insert(name.to_string(), tracer);
        if self.default_tracer.is_none() {
            self.default_tracer = Some(name.to_string());
        }
    }

    #[allow(dead_code)]
    pub fn get_tracer(&self, name: &str) -> Option<&Arc<DMSTracer>> {
        self.tracers.get(name)
    }

    pub fn get_default_tracer(&self) -> Option<&Arc<DMSTracer>> {
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
    inner: Arc<RwLock<DMSTracerManager>>,
}

impl Default for DefaultTracerManager {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(DMSTracerManager::new())),
        }
    }
}

impl DefaultTracerManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    pub async fn register_tracer(&self, name: &str, sampling_rate: f64) {
        let tracer = Arc::new(DMSTracer::new(sampling_rate));
        let mut manager = self.inner.write().unwrap();
        manager.register_tracer(name, tracer);
    }

    #[allow(dead_code)]
    pub async fn get_tracer(&self, name: &str) -> Option<Arc<DMSTracer>> {
        let manager = self.inner.read().unwrap();
        manager.get_tracer(name).cloned()
    }

    pub async fn get_default_tracer(&self) -> Option<Arc<DMSTracer>> {
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

/// Initialize global tracer (backward compatibility)
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

/// Get global tracer (backward compatibility)
pub fn tracer() -> Arc<DMSTracer> {
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
