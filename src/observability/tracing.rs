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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};
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
    
    /// Start a new trace
    pub fn _Fstart_trace(&self, name: String) -> Option<DMSTraceId> {
        if !self._Fshould_sample() {
            return None;
        }
        
        let trace_id = DMSTraceId::_Fnew();
        let span = DMSSpan::_Fnew(
            trace_id.clone(),
            None,
            name,
            DMSSpanKind::Server,
        );
        
        let span_id = span.span_id.clone();
        self.active_spans.write().unwrap().insert(span_id.clone(), span);
        self.spans.write().unwrap().insert(trace_id.clone(), Vec::new());
        
        Some(trace_id)
    }
    
    /// Start a new span in existing trace
    pub fn _Fstart_span(
        &self,
        trace_id: &DMSTraceId,
        parent_span_id: Option<DMSSpanId>,
        name: String,
        kind: DMSSpanKind,
    ) -> Option<DMSSpanId> {
        if !self.spans.read().unwrap().contains_key(trace_id) {
            return None;
        }
        
        let span = DMSSpan::_Fnew(
            trace_id.clone(),
            parent_span_id,
            name,
            kind,
        );
        
        let span_id = span.span_id.clone();
        self.active_spans.write().unwrap().insert(span_id.clone(), span);
        
        Some(span_id)
    }
    
    /// End a span
    pub fn _Fend_span(&self, span_id: &DMSSpanId, status: DMSSpanStatus) -> DMSResult<()> {
        let mut active_spans = self.active_spans.write().unwrap();
        
        if let Some(mut span) = active_spans.remove(span_id) {
            span._Fend(status);
            
            let trace_id = span.trace_id.clone();
            if let Some(spans) = self.spans.write().unwrap().get_mut(&trace_id) {
                spans.push(span);
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

/// Global tracer instance
pub static mut GLOBAL_TRACER: Option<DMSTracer> = None;

/// Initialize global tracer
pub fn _Finit_tracer(sampling_rate: f64) {
    unsafe {
        GLOBAL_TRACER = Some(DMSTracer::_Fnew(sampling_rate));
    }
}

/// Get global tracer
pub fn _Ftracer() -> &'static DMSTracer {
    unsafe {
        #[allow(static_mut_refs)]
        GLOBAL_TRACER.as_ref().expect("Tracer not initialized")
    }
}