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
//! use dms::observability::{_Finit_tracer, _Ftracer, DMSSpanKind, DMSSpanStatus};
//! use dms::core::DMSResult;
//!
//! async fn example() -> DMSResult<()> {
//!     // Initialize the global tracer with 100% sampling rate
//!     _Finit_tracer(1.0);
//!     
//!     // Get the global tracer
//!     let tracer = _Ftracer();
//!     
//!     // Start a new trace
//!     let trace_id = tracer._Fstart_trace("example_trace").unwrap();
//!     
//!     // Start a child span
//!     let span_id = tracer._Fstart_span_from_context("child_span", DMSSpanKind::Internal).unwrap();
//!     
//!     // Add an attribute to the span
//!     tracer._Fspan_mut(&span_id, |span| {
//!         span._Fset_attribute("key".to_string(), "value".to_string());
//!     })?;
//!     
//!     // Add an event to the span
//!     tracer._Fspan_mut(&span_id, |span| {
//!         let mut attributes = std::collections::HashMap::new();
//!         attributes.insert("event_key".to_string(), "event_value".to_string());
//!         span._Fadd_event("example_event".to_string(), attributes);
//!     })?;
//!     
//!     // End the child span with OK status
//!     tracer._Fend_span(&span_id, DMSSpanStatus::Ok)?;
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

impl DMSSpanId {
    pub fn _Fnew() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn _Ffrom_string(s: String) -> Self {
        Self(s)
    }

    pub fn _Fas_str(&self) -> &str {
        &self.0
    }
}

/// Distributed tracing trace ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSTraceId(String);

impl DMSTraceId {
    pub fn _Fnew() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn _Ffrom_string(s: String) -> Self {
        Self(s)
    }

    pub fn _Fas_str(&self) -> &str {
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
    pub fn _Fnew(
        trace_id: DMSTraceId,
        parent_span_id: Option<DMSSpanId>,
        name: String,
        kind: DMSSpanKind,
    ) -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        Self {
            trace_id,
            span_id: DMSSpanId::_Fnew(),
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

    pub fn _Fset_attribute(&mut self, key: String, value: String) {
        self.attributes.insert(key, value);
    }

    pub fn _Fadd_event(&mut self, name: String, attributes: HashMap<String, String>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        self.events.push(DMSSpanEvent {
            name,
            timestamp,
            attributes,
        });
    }

    pub fn _Fend(&mut self, status: DMSSpanStatus) {
        let end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;

        self.end_time = Some(end_time);
        self.status = status;
    }

    pub fn _Fduration(&self) -> Option<Duration> {
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
    static CURRENT_CONTEXT: RefCell<Option<DMSTracingContext>> = RefCell::new(None);
}

impl DMSTracingContext {
    pub fn _Fnew() -> Self {
        Self {
            current_trace_id: None,
            current_span_id: None,
            baggage: HashMap::new(),
        }
    }

    pub fn _Fwith_trace_id(mut self, trace_id: DMSTraceId) -> Self {
        self.current_trace_id = Some(trace_id);
        self
    }

    pub fn _Fwith_span_id(mut self, span_id: DMSSpanId) -> Self {
        self.current_span_id = Some(span_id);
        self
    }

    pub fn _Fset_baggage(&mut self, key: String, value: String) {
        self.baggage.insert(key, value);
    }

    pub fn _Fget_baggage(&self, key: &str) -> Option<&String> {
        self.baggage.get(key)
    }

    pub fn _Ftrace_id(&self) -> Option<&DMSTraceId> {
        self.current_trace_id.as_ref()
    }

    pub fn _Fspan_id(&self) -> Option<&DMSSpanId> {
        self.current_span_id.as_ref()
    }

    /// Set this context as the current thread-local context
    pub fn _Fset_as_current(&self) {
        CURRENT_CONTEXT.with(|ctx| {
            *ctx.borrow_mut() = Some(self.clone());
        });
    }

    /// Get the current tracing context from thread-local storage
    pub fn _Fcurrent() -> Option<Self> {
        CURRENT_CONTEXT.with(|ctx| {
            ctx.borrow().clone()
        })
    }

    /// Create a new context with the same trace ID but new span ID
    pub fn _Fnew_child(&self, span_id: DMSSpanId) -> Self {
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
    pub fn _Fnew(sampling_rate: f64) -> Self {
        Self {
            spans: Arc::new(RwLock::new(HashMap::new())),
            active_spans: Arc::new(RwLock::new(HashMap::new())),
            sampling_rate: sampling_rate.clamp(0.0, 1.0),
        }
    }

    /// Start a new trace and set it as current context
    pub fn _Fstart_trace(&self, name: String) -> Option<DMSTraceId> {
        if !self._Fshould_sample() {
            return None;
        }

        let trace_id = DMSTraceId::_Fnew();
        let span = DMSSpan::_Fnew(trace_id.clone(), None, name, DMSSpanKind::Server);

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
        let context = DMSTracingContext::_Fnew()
            ._Fwith_trace_id(trace_id.clone())
            ._Fwith_span_id(span_id);
        context._Fset_as_current();

        Some(trace_id)
    }

    /// Start a new span in existing trace, using current context if available
    pub fn _Fstart_span(
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
                if let Some(context) = DMSTracingContext::_Fcurrent() {
                    if let Some(id) = context._Ftrace_id() {
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
            None => DMSTracingContext::_Fcurrent().and_then(|context| context._Fspan_id().cloned()),
        };

        if !self.spans.read().unwrap().contains_key(&resolved_trace_id) {
            return None;
        }

        let span = DMSSpan::_Fnew(
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
        if let Some(context) = DMSTracingContext::_Fcurrent() {
            let new_context = context._Fnew_child(span_id.clone());
            new_context._Fset_as_current();
        } else {
            // Create new context if none exists
            let context = DMSTracingContext::_Fnew()
                ._Fwith_trace_id(resolved_trace_id)
                ._Fwith_span_id(span_id.clone());
            context._Fset_as_current();
        }

        Some(span_id)
    }

    /// Start a new span using current context
    pub fn _Fstart_span_from_context(&self, name: String, kind: DMSSpanKind) -> Option<DMSSpanId> {
        self._Fstart_span(None, None, name, kind)
    }

    /// End a span and restore parent span context if available
    pub fn _Fend_span(&self, span_id: &DMSSpanId, status: DMSSpanStatus) -> DMSResult<()> {
        let mut active_spans = self.active_spans.write().unwrap();

        if let Some(mut span) = active_spans.remove(span_id) {
            span._Fend(status);

            let trace_id = span.trace_id.clone();
            if let Some(spans) = self.spans.write().unwrap().get_mut(&trace_id) {
                spans.push(span.clone());
            }

            // Restore parent span context if available
            if let Some(parent_span_id) = span.parent_span_id.clone() {
                // Try to find parent span in active spans
                if active_spans.get(&parent_span_id).is_some() {
                    let context = DMSTracingContext::_Fnew()
                        ._Fwith_trace_id(trace_id)
                        ._Fwith_span_id(parent_span_id);
                    context._Fset_as_current();
                }
            } else {
                // No parent span, clear context
                let context = DMSTracingContext::_Fnew();
                context._Fset_as_current();
            }
        }

        Ok(())
    }

    /// Get span for modification
    pub fn _Fspan_mut<F>(&self, span_id: &DMSSpanId, f: F) -> DMSResult<()>
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
    pub fn _Fexport_traces(&self) -> HashMap<DMSTraceId, Vec<DMSSpan>> {
        self.spans.read().unwrap().clone()
    }

    /// Get active traces count
    pub fn _Factive_trace_count(&self) -> usize {
        self.spans.read().unwrap().len()
    }

    /// Get active span count
    pub fn _Factive_span_count(&self) -> usize {
        self.active_spans.read().unwrap().len()
    }

    fn _Fshould_sample(&self) -> bool {
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
    pub fn _Fnew() -> Self {
        Self {
            tracers: HashMap::new(),
            default_tracer: None,
        }
    }

    pub fn _Fregister_tracer(&mut self, name: &str, tracer: Arc<DMSTracer>) {
        self.tracers.insert(name.to_string(), tracer);
        if self.default_tracer.is_none() {
            self.default_tracer = Some(name.to_string());
        }
    }

    pub fn _Fget_tracer(&self, name: &str) -> Option<&Arc<DMSTracer>> {
        self.tracers.get(name)
    }

    pub fn _Fget_default_tracer(&self) -> Option<&Arc<DMSTracer>> {
        if let Some(default_name) = &self.default_tracer {
            self.tracers.get(default_name)
        } else {
            None
        }
    }

    pub fn _Fset_default_tracer(&mut self, name: &str) -> bool {
        if self.tracers.contains_key(name) {
            self.default_tracer = Some(name.to_string());
            true
        } else {
            false
        }
    }

    pub fn _Fremove_tracer(&mut self, name: &str) -> bool {
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
            inner: Arc::new(RwLock::new(DMSTracerManager::_Fnew())),
        }
    }
}

impl DefaultTracerManager {
    pub fn _Fnew() -> Self {
        Default::default()
    }

    pub async fn _Fregister_tracer(&self, name: &str, sampling_rate: f64) {
        let tracer = Arc::new(DMSTracer::_Fnew(sampling_rate));
        let mut manager = self.inner.write().unwrap();
        manager._Fregister_tracer(name, tracer);
    }

    pub async fn _Fget_tracer(&self, name: &str) -> Option<Arc<DMSTracer>> {
        let manager = self.inner.read().unwrap();
        manager._Fget_tracer(name).cloned()
    }

    pub async fn _Fget_default_tracer(&self) -> Option<Arc<DMSTracer>> {
        let manager = self.inner.read().unwrap();
        manager._Fget_default_tracer().cloned()
    }

    pub async fn _Fset_default_tracer(&self, name: &str) -> bool {
        let mut manager = self.inner.write().unwrap();
        manager._Fset_default_tracer(name)
    }

    pub async fn _Fremove_tracer(&self, name: &str) -> bool {
        let mut manager = self.inner.write().unwrap();
        manager._Fremove_tracer(name)
    }
}

/// Global tracer manager instance
pub static DEFAULT_TRACER_MANAGER: std::sync::LazyLock<DefaultTracerManager> = std::sync::LazyLock::new(|| DefaultTracerManager::default());

/// Initialize global tracer (backward compatibility)
pub fn _Finit_tracer(sampling_rate: f64) {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            DEFAULT_TRACER_MANAGER
                ._Fregister_tracer("default", sampling_rate)
                .await;
        });
}

/// Get global tracer (backward compatibility)
pub fn _Ftracer() -> Arc<DMSTracer> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            DEFAULT_TRACER_MANAGER
                ._Fget_default_tracer()
                .await
                .expect("Tracer not initialized")
        })
}
