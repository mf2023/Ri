<div align="center">

# Observability API Reference

**Version: 0.1.8**

**Last modified date: 2026-01-16**

The observability module provides distributed tracing, metrics collection, Prometheus integration, and Grafana dashboard generation functionality.

## Module Overview

</div>

The observability module contains the following sub-modules:

- **tracing**: Distributed tracing
- **metrics**: Metrics collection
- **propagation**: Trace context propagation
- **prometheus**: Prometheus metrics export
- **grafana**: Grafana dashboard generation
- **metrics_collector**: System metrics collector

<div align="center">

## Core Components

</div>

### DMSCObservabilityModule

Main interface for the observability manager, providing unified monitoring functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create observability module | `config: DMSCObservabilityConfig` | `Self` |
| `start_trace(name)` | Start tracing | `name: String` | `Option<DMSCTraceId>` |
| `start_span(...)` | Start span | Multiple parameters | `Option<DMSCSpanId>` |
| `end_span(id, status)` | End span | `id: &DMSCSpanId`, `status: DMSCSpanStatus` | `DMSCResult<()>` |
| `get_tracer()` | Get tracer reference | None | `Option<Arc<DMSCTracer>>` |
| `get_metrics_registry()` | Get metrics registry | None | `Option<Arc<DMSCMetricsRegistry>>` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Start tracing
let trace_id = observability.start_trace("user_request".to_string());

// Start child span
let span_id = observability.start_span(
    None, // Use current trace ID
    None, // No parent span
    "database_query".to_string(),
    DMSCSpanKind::Client
);

// End span
observability.end_span(&span_id, DMSCSpanStatus::Ok)?;
```

### DMSCMetricsRegistry

Metrics registry for managing application metrics.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new metrics registry | None | `Self` |
| `register(metric)` | Register metric | `metric: Arc<DMSCMetric>` | `DMSCResult<()>` |
| `get_metric(name)` | Get metric | `name: &str` | `Option<Arc<DMSCMetric>>` |
| `get_all_metrics()` | Get all metrics | None | `HashMap<String, Arc<DMSCMetric>>` |
| `export_prometheus()` | Export in Prometheus format | None | `String` |

#### Usage Example

```rust
use dmsc::prelude::*;
use std::sync::Arc;

let registry = DMSCMetricsRegistry::new();

// Create and register counter metric
let counter = Arc::new(DMSCMetric::new(DMSCMetricConfig {
    name: "requests_total".to_string(),
    metric_type: DMSCMetricType::Counter,
    description: "Total number of requests".to_string(),
    buckets: None,
}));

registry.register(counter.clone())?;

// Record metric value
counter.record(1.0);

// Export Prometheus format
let prometheus_output = registry.export_prometheus();
```

### DMSCMetric

Metric type supporting counter, gauge, and histogram.

#### DMSCMetricType

| Type | Description |
|:--------|:-------------|
| `Counter` | Counter, increment-only |
| `Gauge` | Gauge, can increment or decrement |
| `Histogram` | Histogram, records distribution |

### DMSCTracer

Distributed tracer.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new tracer | None | `Self` |
| `start_trace(name)` | Start tracing | `name: String` | `Option<DMSCTraceId>` |
| `start_span(...)` | Start span | Multiple parameters | `Option<DMSCSpanId>` |
| `end_span(id, status)` | End span | `id: &DMSCSpanId`, `status: DMSCSpanStatus` | `DMSCResult<()>` |
| `span_mut(id, f)` | Modify span | `id: &DMSCSpanId`, `f: F` | `DMSCResult<()>` |
| `export_traces()` | Export traces | None | `HashMap<DMSCTraceId, Vec<DMSCSpan>>` |
| `active_trace_count()` | Active trace count | None | `usize` |
| `active_span_count()` | Active span count | None | `usize` |

#### Usage Example

```rust
use dmsc::prelude::*;

let tracer = DMSCTracer::new();

// Start tracing
let trace_id = tracer.start_trace("HTTP request".to_string())?;
assert!(trace_id.is_some());

// Start child span
let span_id = tracer.start_span(
    trace_id.as_ref(),
    None,
    "database_query".to_string(),
    DMSCSpanKind::Client
)?;
assert!(span_id.is_some());

// End span
tracer.end_span(&span_id, DMSCSpanStatus::Ok)?;
```

### DMSCTraceId

Trace ID type.

```rust
/// Trace ID type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSCTraceId(pub String);
```

### DMSCSpanId

Span ID type.

```rust
/// Span ID type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSCSpanId(pub String);
```

### DMSCSpanKind

Span type.

| Variant | Description |
|:--------|:-------------|
| `Server` | Server-side processing |
| `Client` | Client request |
| `Producer` | Message producer |
| `Consumer` | Message consumer |
| `Internal` | Internal operation |

### DMSCSpanStatus

Span status.

| Variant | Description |
|:--------|:-------------|
| `Unset` | Not set |
| `Ok` | Success |
| `Error` | Error |
| `DeadlineExceeded` | Timeout |

### DMSCSamplingStrategy

Trace sampling strategy.

| Variant | Description |
|:--------|:-------------|
| `Rate(rate)` | Fixed sampling rate |
| `Deterministic(rate)` | Deterministic sampling |
| `Adaptive(target_rate)` | Adaptive sampling |

<div align="center">

## Trace Propagation

</div>

### DMSCTraceContext

Trace context.

```rust
use dmsc::prelude::*;

let context = DMSCTraceContext::new()
    .with_trace_id("trace-123".to_string())
    .with_span_id("span-456".to_string());

// Set as current context
context.set_as_current();

// Get current context
if let Some(current) = DMSCTraceContext::current() {
    let trace_id = current.trace_id();
    let span_id = current.span_id();
}
```

### W3CTracePropagator

W3C trace propagator.

```rust
use dmsc::prelude::*;

let propagator = W3CTracePropagator::new();

// Inject into carrier
let mut carrier = HashMap::new();
propagator.inject(&context, &mut carrier);

// Extract from carrier
let extracted = propagator.extract(&carrier);
```

<div align="center">

## Prometheus Integration

</div>

### DMSCPrometheusExporter

Prometheus metrics exporter.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new exporter | None | `Self` |
| `register_counter(name, help)` | Register counter | `name: &str`, `help: &str` | `DMSCResult<()>` |
| `register_gauge(name, help)` | Register gauge | `name: &str`, `help: &str` | `DMSCResult<()>` |
| `register_histogram(name, help, buckets)` | Register histogram | Multiple parameters | `DMSCResult<()>` |
| `increment_counter(name)` | Increment counter | `name: &str` | `DMSCResult<()>` |
| `set_gauge(name, value)` | Set gauge | `name: &str`, `value: f64` | `DMSCResult<()>` |
| `observe_histogram(name, value)` | Observe histogram | `name: &str`, `value: f64` | `DMSCResult<()>` |
| `render()` | Render metrics | None | `DMSCResult<String>` |

#### Usage Example

```rust
use dmsc::prelude::*;

let exporter = DMSCPrometheusExporter::new();

// Register metrics
exporter.register_counter("http_requests_total", "Total HTTP requests")?;
exporter.register_gauge("http_active_connections", "Active HTTP connections")?;
exporter.register_histogram(
    "http_request_duration_seconds",
    "HTTP request duration in seconds",
    vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
)?;

// Record metrics
exporter.increment_counter("http_requests_total")?;
exporter.set_gauge("http_active_connections", 42.0)?;
exporter.observe_histogram("http_request_duration_seconds", 0.256)?;

// Export Prometheus format
let output = exporter.render()?;
println!("{}", output);
```

<div align="center">

## Grafana Integration

</div>

### DMSCGrafanaDashboard

Grafana dashboard.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(title)` | Create new dashboard | `title: &str` | `Self` |
| `add_panel(panel)` | Add panel | `panel: DMSCGrafanaPanel` | `DMSCResult<()>` |
| `to_json()` | Export JSON | None | `DMSCResult<String>` |

### DMSCGrafanaPanel

Grafana panel.

```rust
use dmsc::prelude::*;

let panel = DMSCGrafanaPanel {
    title: "Request Rate".to_string(),
    query: "rate(http_requests_total[5m])".to_string(),
    panel_type: "timeseries".to_string(),
    grid_pos: DMSCGridPos { h: 8, w: 12, x: 0, y: 0 },
};
```

### DMSCGrafanaDashboardGenerator

Grafana dashboard generator.

```rust
use dmsc::prelude::*;

let mut generator = DMSCGrafanaDashboardGenerator::new();

// Generate default dashboard
let dashboard = generator.generate_default_dashboard()?;

// Generate auto dashboard
let metrics = vec!["http_requests_total", "http_request_duration_seconds"];
let auto_dashboard = generator.generate_auto_dashboard(metrics, "Auto Dashboard")?;

// Export JSON
let json = auto_dashboard.to_json()?;
```

<div align="center">

## Configuration

</div>

### DMSCObservabilityConfig

Observability module configuration.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `tracing_enabled` | `bool` | Enable tracing | `true` |
| `metrics_enabled` | `bool` | Enable metrics | `true` |
| `tracing_sampling_rate` | `f64` | Tracing sampling rate | `1.0` |
| `tracing_sampling_strategy` | `String` | Sampling strategy | `"rate"` |
| `metrics_window_size_secs` | `u64` | Metrics window size (seconds) | `60` |
| `metrics_bucket_size_secs` | `u64` | Metrics bucket size (seconds) | `10` |

#### Usage Example

```rust
use dmsc::prelude::*;

let config = DMSCObservabilityConfig {
    tracing_enabled: true,
    metrics_enabled: true,
    tracing_sampling_rate: 0.1,
    tracing_sampling_strategy: "adaptive".to_string(),
    metrics_window_size_secs: 300,
    metrics_bucket_size_secs: 30,
};

let observability = DMSCObservabilityModule::new(config);
```

<div align="center">

## Best Practices

</div>

1. **Configure sampling rate appropriately**: Adjust tracing sampling rate based on traffic volume to avoid excessive overhead
2. **Use appropriate metric types**: Choose Counter, Gauge, or Histogram based on metric characteristics
3. **Add meaningful labels**: Use labels to distinguish different dimensions, but avoid high-cardinality labels
4. **Export and clean up regularly**: Configure metric export intervals to avoid excessive memory usage
5. **Use context propagation**: Properly propagate trace context in distributed systems
6. **Use safe lock acquisition methods**: Use `read_safe()` and `write_safe()` instead of `.read()` and `.write()`

<div align="center">

## Related Modules

</div>

- [core](./core.md): Core module providing error handling and runtime support
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [database](./database.md): Database module providing database operation support
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [log](./log.md): Logging module for protocol events
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
