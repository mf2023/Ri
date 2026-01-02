<div align="center">

# Observability API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The observability module provides comprehensive monitoring, tracing, and logging features to help developers understand system operation status.

## Module Overview

</div>

The observability module contains the following sub-modules:

- **metrics**: Metrics collection and export
- **tracing**: Distributed tracing
- **logging**: Structured logging
- **health**: Health checks
- **profiling**: Performance profiling
- **alerting**: Alerting system
- **dashboard**: Monitoring dashboard
- **anomaly**: Anomaly detection

<div align="center">

## Core Components

</div>

### DMSCObservabilityConfig

Observability configuration class, used to configure monitoring and tracing behavior.

#### Constructor

```python
DMSCObservabilityConfig(
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    enable_logging: bool = True,
    enable_profiling: bool = False,
    metrics_backend: str = "prometheus",
    metrics_port: int = 9090,
    metrics_path: str = "/metrics",
    metrics_interval: int = 15,
    tracing_backend: str = "jaeger",
    tracing_endpoint: str = "http://localhost:14268/api/traces",
    tracing_sample_rate: float = 1.0,
)
```

#### Properties

| Property | Type | Description | Default |
|:---------|:-----|:------------|:--------|
| `enable_metrics` | `bool` | Enable metrics collection | `True` |
| `enable_tracing` | `bool` | Enable distributed tracing | `True` |
| `enable_logging` | `bool` | Enable structured logging | `True` |
| `enable_profiling` | `bool` | Enable performance profiling | `False` |
| `metrics_backend` | `str` | Metrics backend (prometheus, influxdb) | `"prometheus"` |
| `metrics_port` | `int` | Metrics HTTP server port | `9090` |
| `metrics_path` | `str` | Metrics endpoint path | `"/metrics"` |
| `tracing_backend` | `str` | Tracing backend (jaeger, zipkin) | `"jaeger"` |
| `tracing_endpoint` | `str` | Tracing collector endpoint | Jaeger default |
| `tracing_sample_rate` | `float` | Trace sampling rate (0-1) | `1.0` |

### DMSCObservabilityModule

Observability module main interface.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `start()` | Start observability services | None | `None` |
| `stop()` | Stop observability services | None | `None` |
| `get_metrics()` | Get current metrics | None | `dict` |
| `get_tracer()` | Get tracer instance | None | `DMSCOpenTelemetryTracer` |
| `record_metric(name, value, **labels)` | Record custom metric | `name: str`, `value: float`, `**labels` | `None` |
| `start_span(name, **attrs)` | Start trace span | `name: str`, `**attrs` | `DMSCSpan` |
| `health_check()` | Perform health check | None | `dict` |

#### Usage Example

```python
from dmsc import DMSCObservabilityModule, DMSCObservabilityConfig

# Initialize observability
config = DMSCObservabilityConfig(
    enable_metrics=True,
    enable_tracing=True,
    metrics_backend="prometheus",
    tracing_backend="jaeger",
    tracing_endpoint="http://localhost:14268/api/traces"
)
obs_module = DMSCObservabilityModule(config)

# Start observability services
await obs_module.start()

# Get metrics
metrics = obs_module.get_metrics()
print(f"Request count: {metrics.request_count}")
print(f"Error rate: {metrics.error_rate}")
```

## Metrics

### Metric Types

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

# Counter - increments only
obs_module.counter(
    name="http_requests_total",
    description="Total HTTP requests",
    labels={"method": "GET", "endpoint": "/api/users"}
)

# Gauge - can increase or decrease
obs_module.gauge(
    name="active_connections",
    description="Number of active connections",
    value=100
)

# Histogram - for measuring distributions
obs_module.histogram(
    name="request_duration_seconds",
    description="Request duration in seconds",
    buckets=[0.1, 0.5, 1.0, 5.0],
    labels={"endpoint": "/api/users"}
)

# Summary - for calculating quantiles
obs_module.summary(
    name="request_latency",
    description="Request latency quantiles",
    quantiles=[0.5, 0.9, 0.99],
    labels={"service": "api"}
)
```

### Custom Metrics

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

# Record custom metrics
obs_module.record_metric(
    name="business_orders_total",
    value=1,
    labels={"status": "completed", "region": "us-east"}
)

obs_module.record_metric(
    name="cache_hit_ratio",
    value=0.95,
    labels={"cache": "redis"}
)

obs_module.record_metric(
    name="queue_size",
    value=150,
    labels={"queue": "notifications"}
)
```

## Tracing

### Creating Spans

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

# Create span
with obs_module.start_span(
    name="process_order",
    attributes={
        "order_id": "12345",
        "user_id": "67890"
    }
) as span:
    # Do work
    span.set_attribute("status", "processing")
    span.add_event("Order received")
    
    # Create child span
    with obs_module.start_span(
        name="validate_payment",
        parent_span=span
    ) as payment_span:
        payment_span.set_attribute("payment_method", "credit_card")
```

### Trace Propagation

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

# Extract trace context from incoming request
context = obs_module.extract_context(
    headers={"traceparent": "00-abc123-def456-01"}
)

# Create span from context
with obs_module.start_span(
    name="handle_request",
    context=context
) as span:
    # Process request
    pass

# Inject trace context to outgoing request
headers = obs_module.inject_context(context)
# Use headers in outgoing request
```

### Span Attributes

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

with obs_module.start_span("database_query") as span:
    # Set attributes
    span.set_attribute("db.system", "postgresql")
    span.set_attribute("db.name", "mydb")
    span.set_attribute("db.statement", "SELECT * FROM users")
    span.set_attribute("db.duration_ms", 45.5)
    
    # Add events
    span.add_event("Query started")
    span.add_event("Query completed")
    
    # Record exception
    try:
        # Database operation
        pass
    except Exception as e:
        span.record_exception(e)
        span.set_status("error")
```

## Health Checks

### Built-in Health Checks

```python
from dmsc import DMSCObservabilityModule

obs_module = DMSCObservabilityModule()

# Perform health check
health = obs_module.health_check()
print(f"Status: {health.status}")
print(f"Components: {health.components}")

# Health check result
# {
#     "status": "healthy",
#     "components": {
#         "database": {"status": "healthy", "latency_ms": 5},
#         "cache": {"status": "healthy", "latency_ms": 2},
#         "message_queue": {"status": "degraded", "latency_ms": 150}
#     }
# }
```

### Custom Health Checks

```python
from dmsc import DMSCObservabilityModule, DMSCCustomHealthCheck

obs_module = DMSCObservabilityModule()

# Register custom health check
obs_module.register_health_check(
    "custom_service",
    DMSCCustomHealthCheck(
        name="custom_service_check",
        check_fn=lambda: {"status": "healthy", "detail": "Custom service OK"},
        timeout_seconds=5
    )
)
```

## Prometheus Integration

```python
from dmsc import DMSCObservabilityModule, DMSCObservabilityConfig

# Configure Prometheus metrics
config = DMSCObservabilityConfig(
    enable_metrics=True,
    metrics_backend="prometheus",
    metrics_port=9090,
    metrics_path="/metrics"
)
obs_module = DMSCObservabilityModule(config)

# Access Prometheus registry
registry = obs_module.get_prometheus_registry()

# Custom Prometheus metrics
from prometheus_client import Counter, Histogram

REQUEST_COUNT = Counter(
    'dmsc_http_requests_total',
    'Total HTTP requests',
    ['method', 'endpoint', 'status']
)

REQUEST_LATENCY = Histogram(
    'dmsc_http_request_duration_seconds',
    'HTTP request duration',
    ['method', 'endpoint'],
    buckets=[0.1, 0.5, 1.0, 5.0]
)

# Record metrics
REQUEST_COUNT.labels(method='GET', endpoint='/api', status='200').inc()
REQUEST_LATENCY.labels(method='GET', endpoint='/api').observe(0.15)
```

## Best Practices

1. **Use Standard Tracing**: Always use distributed tracing for microservices
2. **Sample Appropriately**: Use sampling for high-traffic services
3. **Add Meaningful Attributes**: Include relevant attributes in spans
4. **Set Proper Status**: Always set span status (ok/error)
5. **Monitor Key Metrics**: Monitor business KPIs, not just technical metrics
6. **Use Histograms for Latency**: Use histograms for latency distributions
7. **Implement Health Checks**: Implement health checks for all services
8. **Alert on Anomalies**: Set up alerts for unusual patterns
9. **Correlate Logs with Traces**: Link logs to traces using trace_id
10. **Regular Profiling**: Profile performance in production regularly
