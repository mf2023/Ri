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

//! # Observability Module C API
//!
//! This module provides C language bindings for Ri's observability infrastructure. The observability
//! module delivers comprehensive system monitoring capabilities including distributed tracing, metrics
//! collection, and health checking. This C API enables C/C++ applications to leverage Ri's
//! observability features for understanding system behavior, debugging issues, and monitoring performance
//! in production environments.
//!
//! ## Module Architecture
//!
//! The observability module comprises three primary components that together provide complete
//! monitoring and tracing capabilities:
//!
//! - **RiObservabilityConfig**: Configuration container for observability infrastructure parameters
//!   including tracing settings, metrics collection options, export destinations, and sampling
//!   configurations. The configuration object controls resource allocation, export strategies, and
//!   behavioral characteristics for all observability features.
//!
//! - **RiTracer**: Distributed tracing interface for creating and managing trace spans, propagating
//!   context across service boundaries, and exporting trace data to analysis systems. The tracer
//!   implements OpenTelemetry-compatible tracing with automatic instrumentation support.
//!
//! - **RiMetricsRegistry**: Metrics collection and aggregation system supporting multiple metric types
//!   including counters, gauges, histograms, and summaries. The registry manages metric lifecycle,
//!   provides dimensional labeling, and exports metrics to monitoring backends.
//!
//! ## Distributed Tracing
//!
//! The tracing system implements comprehensive distributed tracing capabilities:
//!
//! - **Span Creation**: Create trace spans with parent-child relationships to model request flows
//!   across service boundaries. Spans capture timing, status, attributes, and events.
//!
//! - **Context Propagation**: Propagate trace context across process boundaries using W3C Trace Context
//!   standard. Supports propagation via HTTP headers, gRPC metadata, and message queue properties.
//!
//! - **Automatic Instrumentation**: Built-in instrumentation for common frameworks and libraries
//!   including HTTP servers/clients, database drivers, and message queues.
//!
//! - **Sampling Strategies**: Configurable sampling to balance observability with performance impact.
//!   Supports rate-based, probabilistic, and tail-based sampling strategies.
//!
//! - **Span Attributes**: Attach key-value attributes to spans for filtering and aggregating traces.
//!   Supports automatic attributes (HTTP method, status code) and custom application attributes.
//!
//! - **Span Events**: Record timestamped events within spans for debugging and audit trails.
//!   Events capture discrete occurrences during span execution.
//!
//! - **Error Recording**: Automatic error capturing with stack traces and error attributes.
//!   Errors are marked on spans with full exception information.
//!
//! ## Metrics System
//!
//! The metrics system provides comprehensive performance and operational monitoring:
//!
//! - **Counter Metrics**: Increment-only metrics for counting events like requests, errors, or
//!   operations. Useful for tracking cumulative totals and rates.
//!
//! - **Gauge Metrics**: Arbitrary value metrics that can increase or decrease over time.
//!   Suitable for tracking current values like queue depth, memory usage, or active connections.
//!
//! - **Histogram Metrics**: Statistical distribution metrics that bucket values into configurable
//!   quantiles. Useful for tracking latency distributions, request sizes, and response times.
//!
//! - **Summary Metrics**: Client-side calculated quantiles with optional sum tracking.
//!   Provides pre-computed percentiles for high-cardinality metrics.
//!
//! - **Dimensional Labels**: Attach multiple key-value labels to metrics for flexible filtering
//!   and aggregation. Labels enable drill-down analysis across service versions, regions, or
//!   deployment environments.
//!
//! - **Metric Views**: Define custom aggregations and label combinations to control storage
//!   costs and query performance.
//!
//! ## Health Checking
//!
//! The observability module includes health check infrastructure:
//!
//! - **Liveness Probes**: Indicate whether the service is running. Used by orchestrators like
//!   Kubernetes to restart unhealthy containers.
//!
//! - **Readiness Probes**: Indicate whether the service is ready to accept traffic. Prevents
//!   routing requests to services that are still initializing or overloaded.
//!
//! - **Startup Probes**: Slow-startup detection for applications with long initialization times.
//!   Allows services time to become ready before liveness checks begin.
//!
//! - **Custom Health Checks**: Register application-specific health check functions that examine
//!   dependencies like database connectivity, cache availability, or external services.
//!
//! ## Export Pipelines
//!
//! The observability system supports multiple export destinations:
//!
//! - **OpenTelemetry Protocol**: Standardized export format for compatibility with observability
//!   backends. Supports both push and pull-based export models.
//!
//! - **Prometheus**: Pull-based metrics export compatible with Prometheus and related tools.
//!   Supports both metrics and trace exposition endpoints.
//!
//! - **Jaeger**: Direct export to Jaeger tracing backend for distributed tracing visualization.
//!
//! - **Zipkin**: Export to Zipkin tracing backend for distributed tracing analysis.
//!
//! - **Logging Export**: Export traces and metrics to application logs for unified log analysis.
//!
//! - **Custom Exporters**: User-defined exporters for integration with proprietary monitoring
//!   systems or specialized analysis pipelines.
//!
//! ## Performance Characteristics
//!
//! Observability operations are designed for minimal performance impact:
//!
//! - **Async Export**: Non-blocking metric and trace export using background threads.
//!   Producer-consumer patterns prevent slow exporters from impacting application performance.
//!
//! - **Sampling**: Configurable sampling reduces trace volume for high-traffic services.
//!   Default conservative sampling minimizes overhead while preserving important traces.
//!
//! - **Batching**: Multiple metrics and traces batched together for efficient network export.
//!   Reduces connection overhead and improves throughput.
//!
//! - **Lazy Initialization**: Observability infrastructure initialized on first use when possible.
//!   Reduces startup time and memory footprint for unused features.
//!
//! - **Memory Bounds**: Internal buffers and queues have configurable size limits to prevent
//!   unbounded memory growth under high load or exporter failures.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Tracer instances should be properly shut down before freeing
//! - Metrics registries can be shared across components
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Concurrent span creation from multiple threads is supported
//! - Metric recording operations are lock-free for performance
//! - Export pipelines handle concurrent data from multiple threads
//! - Configuration can be modified at runtime for some parameters
//!
//! ## Usage Example
//!
//! ```c
//! // Create observability configuration
//! RiObservabilityConfig* config = ri_observability_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create observability config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure tracing
//! ri_observability_config_set_tracing_enabled(config, true);
//! ri_observability_config_set_tracing_samplerate(config, 0.1);  // 10% sampling
//! ri_observability_config_set_tracing_exporter(config, "otlp");
//!
//! // Configure metrics
//! ri_observability_config_set_metrics_enabled(config, true);
//! ri_observability_config_set_metrics_export_interval(config, 60000);  // 60 seconds
//!
//! // Configure health checks
//! ri_observability_config_set_health_check_enabled(config, true);
//!
//! // Create tracer instance
//! RiTracer* tracer = ri_tracer_new(config);
//! if (tracer == NULL) {
//!     fprintf(stderr, "Failed to create tracer\n");
//!     ri_observability_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Create metrics registry
//! RiMetricsRegistry* metrics = ri_metrics_registry_new(config);
//! if (metrics == NULL) {
//!     fprintf(stderr, "Failed to create metrics registry\n");
//!     ri_tracer_free(tracer);
//!     ri_observability_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Create a span for tracing
//! RiTraceSpan* span = ri_tracer_start_span(tracer, "handle_request");
//! ri_trace_span_set_attribute(span, "http.method", "GET");
//! ri_trace_span_set_attribute(span, "http.url", "/api/users");
//!
//! // Record metrics
//! ri_metrics_registry_counter_increment(metrics, "http_requests_total",
//!     1,  // value
//!     2,  // label count
//!     "method", "GET",       // label name, value
//!     "path", "/api/users"  // label name, value
//! );
//!
//! // Simulate work
//! // ... application logic ...
//!
//! // End span
//! ri_trace_span_end(span);
//!
//! // Graceful shutdown
//! ri_tracer_shutdown(tracer);  // Flush remaining traces
//! ri_tracer_free(tracer);
//! ri_metrics_registry_free(metrics);
//! ri_observability_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::observability`: Rust observability module implementation
//! - `crate::prelude`: Common types and traits
//! - OpenTelemetry SDK for tracing
//! - Metrics library for metric collection
//!
//! ## Feature Flags
//!
//! The observability module is enabled by the "observability" feature flag.
//! Disable this feature to reduce binary size when observability is not required.
//!
//! Additional features:
//!
//! - observability-tracing: Enable distributed tracing
//! - observability-metrics: Enable metrics collection
//! - observability-health: Enable health check endpoints
//! - observability-opentelemetry: Enable OpenTelemetry export
//! - observability-prometheus: Enable Prometheus export

use crate::observability::{RiMetricsRegistry, RiObservabilityConfig, RiTracer, RiSpanKind, RiSpanStatus, RiSpanId, RiTraceId};


c_wrapper!(CRiObservabilityConfig, RiObservabilityConfig);
c_wrapper!(CRiTracer, RiTracer);
c_wrapper!(CRiMetricsRegistry, RiMetricsRegistry);
c_wrapper!(CRiSpanId, RiSpanId);
c_wrapper!(CRiTraceId, RiTraceId);

// RiObservabilityConfig constructors and destructors
c_constructor!(
    ri_observability_config_new,
    CRiObservabilityConfig,
    RiObservabilityConfig,
    RiObservabilityConfig::default()
);
c_destructor!(ri_observability_config_free, CRiObservabilityConfig);

// RiTracer C bindings
#[no_mangle]
pub extern "C" fn ri_tracer_new(sampling_rate: f64) -> *mut CRiTracer {
    Box::into_raw(Box::new(CRiTracer::new(RiTracer::new(sampling_rate))))
}
c_destructor!(ri_tracer_free, CRiTracer);

#[no_mangle]
pub extern "C" fn ri_tracer_start_trace(
    tracer: *mut CRiTracer,
    name: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if tracer.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        match (*tracer).inner.start_trace(name_str) {
            Some(trace_id) => match std::ffi::CString::new(trace_id.as_str()) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_tracer_start_span(
    tracer: *mut CRiTracer,
    name: *const std::ffi::c_char,
    kind: std::ffi::c_int,
) -> *mut std::ffi::c_char {
    if tracer.is_null() || name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let span_kind = match kind {
            0 => RiSpanKind::Server,
            1 => RiSpanKind::Client,
            2 => RiSpanKind::Producer,
            3 => RiSpanKind::Consumer,
            _ => RiSpanKind::Internal,
        };
        match (*tracer).inner.start_span_from_context(name_str, span_kind) {
            Some(span_id) => match std::ffi::CString::new(span_id.as_str()) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_tracer_end_span(
    tracer: *mut CRiTracer,
    span_id: *const std::ffi::c_char,
    status: std::ffi::c_int,
) -> std::ffi::c_int {
    if tracer.is_null() || span_id.is_null() {
        return -1;
    }
    unsafe {
        let span_id_str = match std::ffi::CStr::from_ptr(span_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };
        let span_status = match status {
            0 => RiSpanStatus::Ok,
            1 => RiSpanStatus::Error("C API error".to_string()),
            _ => RiSpanStatus::Unset,
        };
        let span_id_obj = RiSpanId::from_string(span_id_str.to_string());
        match (*tracer).inner.end_span(&span_id_obj, span_status) {
            Ok(_) => 0,
            Err(_) => -3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_tracer_set_attribute(
    tracer: *mut CRiTracer,
    span_id: *const std::ffi::c_char,
    key: *const std::ffi::c_char,
    value: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if tracer.is_null() || span_id.is_null() || key.is_null() || value.is_null() {
        return -1;
    }
    unsafe {
        let span_id_str = match std::ffi::CStr::from_ptr(span_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -2,
        };
        let value_str = match std::ffi::CStr::from_ptr(value).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -2,
        };
        let span_id_obj = RiSpanId::from_string(span_id_str.to_string());
        match (*tracer).inner.span_mut(&span_id_obj, |span| {
            span.set_attribute(key_str, value_str);
        }) {
            Ok(_) => 0,
            Err(_) => -3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_tracer_get_active_trace_count(tracer: *mut CRiTracer) -> usize {
    if tracer.is_null() {
        return 0;
    }
    unsafe { (*tracer).inner.active_trace_count() }
}

#[no_mangle]
pub extern "C" fn ri_tracer_get_active_span_count(tracer: *mut CRiTracer) -> usize {
    if tracer.is_null() {
        return 0;
    }
    unsafe { (*tracer).inner.active_span_count() }
}

// RiMetricsRegistry C bindings
#[no_mangle]
pub extern "C" fn ri_metrics_registry_new() -> *mut CRiMetricsRegistry {
    Box::into_raw(Box::new(CRiMetricsRegistry::new(RiMetricsRegistry::new())))
}
c_destructor!(ri_metrics_registry_free, CRiMetricsRegistry);

#[no_mangle]
pub extern "C" fn ri_metrics_registry_get_metric_count(registry: *mut CRiMetricsRegistry) -> usize {
    if registry.is_null() {
        return 0;
    }
    unsafe { (*registry).inner.get_all_metrics().len() }
}

#[no_mangle]
pub extern "C" fn ri_metrics_registry_get_metric_value(
    registry: *mut CRiMetricsRegistry,
    name: *const std::ffi::c_char,
) -> f64 {
    if registry.is_null() || name.is_null() {
        return 0.0;
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        match (*registry).inner.get_metric(name_str) {
            Some(metric) => metric.get_value(),
            None => 0.0,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_metrics_registry_export_prometheus(
    registry: *mut CRiMetricsRegistry,
) -> *mut std::ffi::c_char {
    if registry.is_null() {
        return std::ptr::null_mut();
    }
    #[allow(unused_unsafe)]
    unsafe {
        #[cfg(feature = "observability")]
        {
            let output = (*registry).inner.export_prometheus();
            match std::ffi::CString::new(output) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
        #[cfg(not(feature = "observability"))]
        {
            match std::ffi::CString::new("# Observability feature not enabled") {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            }
        }
    }
}

// RiSystemMetrics C bindings (if system_info feature is enabled)
#[cfg(feature = "system_info")]
use crate::observability::{RiSystemMetrics, RiSystemMetricsCollector};

#[cfg(feature = "system_info")]
c_wrapper!(CRiSystemMetrics, RiSystemMetrics);

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_new() -> *mut CRiSystemMetrics {
    Box::into_raw(Box::new(CRiSystemMetrics::new(RiSystemMetrics::default())))
}

#[cfg(feature = "system_info")]
c_destructor!(ri_system_metrics_free, CRiSystemMetrics);

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_get_cpu_usage(metrics: *mut CRiSystemMetrics) -> f64 {
    if metrics.is_null() {
        return 0.0;
    }
    unsafe { (*metrics).inner.cpu.usage }
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_get_memory_used(metrics: *mut CRiSystemMetrics) -> u64 {
    if metrics.is_null() {
        return 0;
    }
    unsafe { (*metrics).inner.memory.used }
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_get_memory_total(metrics: *mut CRiSystemMetrics) -> u64 {
    if metrics.is_null() {
        return 0;
    }
    unsafe { (*metrics).inner.memory.total }
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_get_disk_used(metrics: *mut CRiSystemMetrics) -> u64 {
    if metrics.is_null() {
        return 0;
    }
    unsafe { (*metrics).inner.disk.used }
}

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_get_disk_total(metrics: *mut CRiSystemMetrics) -> u64 {
    if metrics.is_null() {
        return 0;
    }
    unsafe { (*metrics).inner.disk.total }
}

#[cfg(feature = "system_info")]
c_wrapper!(CRiSystemMetricsCollector, RiSystemMetricsCollector);

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_collector_new() -> *mut CRiSystemMetricsCollector {
    Box::into_raw(Box::new(CRiSystemMetricsCollector::new(RiSystemMetricsCollector::new())))
}

#[cfg(feature = "system_info")]
c_destructor!(ri_system_metrics_collector_free, CRiSystemMetricsCollector);

#[cfg(feature = "system_info")]
#[no_mangle]
pub extern "C" fn ri_system_metrics_collector_collect(
    collector: *mut CRiSystemMetricsCollector,
    out_metrics: *mut *mut CRiSystemMetrics,
) -> std::ffi::c_int {
    if collector.is_null() || out_metrics.is_null() {
        return -1;
    }
    unsafe {
        let metrics = (*collector).inner.collect();
        *out_metrics = Box::into_raw(Box::new(CRiSystemMetrics::new(metrics)));
        0
    }
}
