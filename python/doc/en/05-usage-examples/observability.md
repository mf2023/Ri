<div align="center">

# Observability Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's observability module for distributed tracing, metrics collection, health checks, performance analysis, and alerting.

## Example Overview

This example creates a DMSC Python application with the following features:

- Distributed tracing and span management
- Metrics collection and performance monitoring
- Health checks and status monitoring
- Log aggregation and analysis
- Alerting and notifications
- Performance analysis and optimization

## Prerequisites

- Python 3.8+
- Understanding of observability concepts
- (Optional) Jaeger, Prometheus, Grafana for visualization

## Complete Code Example

```python
import asyncio
import time
from datetime import datetime
from typing import Dict, List, Optional, Any
from enum import Enum
from dataclasses import dataclass
from collections import deque

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCObservabilityModule, DMSCObservabilityConfig,
    DMSCOpenTelemetryTracer, DMSCMetricsCollector,
    DMSCHealthCheck, DMSCHealthStatus,
    DMSCLocalMetric, DMSCMetricType, DMSCTimer,
    DMSCConfig, DMSCError
)

# Component status
class ComponentStatus(Enum):
    HEALTHY = "healthy"
    DEGRADED = "degraded"
    UNHEALTHY = "unhealthy"
    UNKNOWN = "unknown"

# Alert severity
class AlertSeverity(Enum):
    INFO = "info"
    WARNING = "warning"
    ERROR = "error"
    CRITICAL = "critical"

# Alert data class
@dataclass
class Alert:
    alert_id: str
    severity: AlertSeverity
    component: str
    message: str
    timestamp: datetime
    acknowledged: bool
    metadata: Dict[str, Any]

# Span data class
@dataclass
class TraceSpan:
    span_id: str
    trace_id: str
    parent_span_id: Optional[str]
    operation_name: str
    start_time: datetime
    end_time: Optional[datetime]
    duration_ms: Optional[float]
    status: str
    attributes: Dict[str, Any]
    events: List[Dict]

# Observability service
class ObservabilityService:
    def __init__(self, obs_module: DMSCObservabilityModule, context: DMSCServiceContext):
        self.obs_module = obs_module
        self.context = context
        self.logger = context.logger
        self.tracer = obs_module.get_tracer()
        self.metrics = obs_module.get_metrics_collector()
        self.health_checks: Dict[str, DMSCHealthCheck] {}
        self.component_status: Dict[str, ComponentStatus] = {}
        self.alerts: List[Alert] = []
        self.alert_history: deque = deque(maxlen=1000)
        self.spans: List[TraceSpan] = []
        self.span_history: deque = deque(maxlen=10000)
    
    def create_span(
        self,
        operation_name: str,
        parent_span_id: Optional[str] = None,
        attributes: Optional[Dict] = None
    ) -> TraceSpan:
        """Create a new trace span"""
        span_id = f"span_{datetime.now().timestamp()}"
        trace_id = self.tracer.get_current_trace_id()
        
        span = TraceSpan(
            span_id=span_id,
            trace_id=trace_id,
            parent_span_id=parent_span_id,
            operation_name=operation_name,
            start_time=datetime.now(),
            end_time=None,
            duration_ms=None,
            status="started",
            attributes=attributes or {},
            events=[]
        )
        
        self.spans.append(span)
        self.logger.info("tracing", f"Span created: {span_id} - {operation_name}")
        
        return span
    
    def end_span(self, span: TraceSpan, status: str = "ok", events: Optional[List[Dict]] = None):
        """End a trace span"""
        span.end_time = datetime.now()
        span.duration_ms = (span.end_time - span.start_time).total_seconds() * 1000
        span.status = status
        if events:
            span.events.extend(events)
        
        self.span_history.append(span)
        self.spans.remove(span)
        
        self.logger.info("tracing", f"Span ended: {span.span_id} - {span.duration_ms:.2f}ms")
    
    async def record_metric(
        self,
        name: str,
        value: float,
        metric_type: DMSCMetricType,
        labels: Optional[Dict] = None
    ):
        """Record a custom metric"""
        metric = DMSCLocalMetric(
            name=name,
            value=value,
            type=metric_type,
            labels=labels or {},
            timestamp=datetime.now()
        )
        
        await self.metrics.record(metric)
        self.logger.info("metrics", f"Metric recorded: {name} = {value}")
    
    def register_health_check(self, component: str, check: DMSCHealthCheck):
        """Register a health check for a component"""
        self.health_checks[component] = check
        self.logger.info("health", f"Health check registered: {component}")
    
    async def check_component_health(self, component: str) -> ComponentStatus:
        """Check the health of a component"""
        if component not in self.health_checks:
            return ComponentStatus.UNKNOWN
        
        check = self.health_checks[component]
        result = await check.run()
        
        if result.healthy:
            status = ComponentStatus.HEALTHY
        elif result.degraded:
            status = ComponentStatus.DEGRADED
        else:
            status = ComponentStatus.UNHEALTHY
        
        self.component_status[component] = status
        return status
    
    async def check_all_health(self) -> Dict[str, ComponentStatus]:
        """Check health of all registered components"""
        results = {}
        
        for component in self.health_checks:
            results[component] = await self.check_component_health(component)
        
        self.logger.info("health", f"Health check completed: {len(results)} components")
        return results
    
    def create_alert(
        self,
        severity: AlertSeverity,
        component: str,
        message: str,
        metadata: Optional[Dict] = None
    ) -> Alert:
        """Create a new alert"""
        alert_id = f"alert_{datetime.now().timestamp()}"
        
        alert = Alert(
            alert_id=alert_id,
            severity=severity,
            component=component,
            message=message,
            timestamp=datetime.now(),
            acknowledged=False,
            metadata=metadata or {}
        )
        
        self.alerts.append(alert)
        self.alert_history.append(alert)
        
        self.logger.warn("alerting", f"Alert created: {severity.value} - {component}: {message}")
        
        return alert
    
    def acknowledge_alert(self, alert_id: str) -> bool:
        """Acknowledge an alert"""
        for alert in self.alerts:
            if alert.alert_id == alert_id:
                alert.acknowledged = True
                self.logger.info("alerting", f"Alert acknowledged: {alert_id}")
                return True
        return False
    
    def get_active_alerts(self, severity: Optional[AlertSeverity] = None) -> List[Alert]:
        """Get active (unacknowledged) alerts"""
        alerts = [a for a in self.alerts if not a.acknowledged]
        
        if severity:
            alerts = [a for a in alerts if a.severity == severity]
        
        return sorted(alerts, key=lambda x: x.timestamp, reverse=True)
    
    def get_trace_spans(self, trace_id: Optional[str] = None) -> List[TraceSpan]:
        """Get trace spans, optionally filtered by trace ID"""
        if trace_id:
            return [s for s in self.span_history if s.trace_id == trace_id]
        return list(self.span_history)
    
    def get_metrics_summary(self) -> Dict:
        """Get a summary of all metrics"""
        return {
            "total_spans": len(self.span_history),
            "active_spans": len(self.spans),
            "active_alerts": len([a for a in self.alerts if not a.acknowledged]),
            "component_status": {k: v.value for k, v in self.component_status.items()}
        }

# Request handlers
async def handle_create_span(context: DMSCServiceContext):
    """Create a new trace span"""
    data = await context.http.request.json()
    
    operation_name = data.get("operation_name", "unknown")
    parent_span_id = data.get("parent_span_id")
    attributes = data.get("attributes", {})
    
    obs_service = context.observability_service
    span = obs_service.create_span(operation_name, parent_span_id, attributes)
    
    return {
        "status": "success",
        "data": {
            "span_id": span.span_id,
            "trace_id": span.trace_id,
            "operation_name": span.operation_name,
            "start_time": span.start_time.isoformat()
        }
    }

async def handle_end_span(context: DMSCServiceContext):
    """End a trace span"""
    data = await context.http.request.json()
    
    span_id = data.get("span_id")
    status = data.get("status", "ok")
    events = data.get("events", [])
    
    obs_service = context.observability_service
    
    for span in obs_service.spans:
        if span.span_id == span_id:
            obs_service.end_span(span, status, events)
            return {
                "status": "success",
                "data": {
                    "span_id": span.span_id,
                    "duration_ms": span.duration_ms,
                    "status": span.status
                }
            }
    
    return {"status": "error", "message": "Span not found"}, 404

async def handle_record_metric(context: DMSCServiceContext):
    """Record a custom metric"""
    data = await context.http.request.json()
    
    name = data.get("name")
    value = data.get("value", 0)
    metric_type_str = data.get("type", "counter")
    labels = data.get("labels", {})
    
    try:
        metric_type = DMSCMetricType(metric_type_str)
    except ValueError:
        metric_type = DMSCMetricType.COUNTER
    
    obs_service = context.observability_service
    await obs_service.record_metric(name, value, metric_type, labels)
    
    return {"status": "success", "message": f"Metric recorded: {name}"}

async def handle_check_health(context: DMSCServiceContext):
    """Check health of all components"""
    obs_service = context.observability_service
    results = await obs_service.check_all_health()
    
    return {
        "status": "success",
        "data": {
            "components": {k: v.value for k, v in results.items()},
            "timestamp": datetime.now().isoformat()
        }
    }

async def handle_create_alert(context: DMSCServiceContext):
    """Create a new alert"""
    data = await context.http.request.json()
    
    severity_str = data.get("severity", "warning")
    component = data.get("component")
    message = data.get("message")
    metadata = data.get("metadata", {})
    
    try:
        severity = AlertSeverity(severity_str)
    except ValueError:
        severity = AlertSeverity.WARNING
    
    if not component or not message:
        return {"status": "error", "message": "component and message required"}, 400
    
    obs_service = context.observability_service
    alert = obs_service.create_alert(severity, component, message, metadata)
    
    return {
        "status": "success",
        "data": {
            "alert_id": alert.alert_id,
            "severity": alert.severity.value,
            "timestamp": alert.timestamp.isoformat()
        }
    }

async def handle_get_alerts(context: DMSCServiceContext):
    """Get active alerts"""
    data = await context.http.request.json()
    
    severity_str = data.get("severity")
    
    try:
        severity = AlertSeverity(severity_str) if severity_str else None
    except ValueError:
        severity = None
    
    obs_service = context.observability_service
    alerts = obs_service.get_active_alerts(severity)
    
    return {
        "status": "success",
        "data": {
            "count": len(alerts),
            "alerts": [
                {
                    "id": a.alert_id,
                    "severity": a.severity.value,
                    "component": a.component,
                    "message": a.message,
                    "timestamp": a.timestamp.isoformat()
                }
                for a in alerts[:50]
            ]
        }
    }

async def handle_acknowledge_alert(context: DMSCServiceContext):
    """Acknowledge an alert"""
    data = await context.http.request.json()
    
    alert_id = data.get("alert_id")
    
    if not alert_id:
        return {"status": "error", "message": "alert_id required"}, 400
    
    obs_service = context.observability_service
    success = obs_service.acknowledge_alert(alert_id)
    
    if success:
        return {"status": "success", "message": "Alert acknowledged"}
    else:
        return {"status": "error", "message": "Alert not found"}, 404

async def handle_get_traces(context: DMSCServiceContext):
    """Get trace spans"""
    data = await context.http.request.json()
    
    trace_id = data.get("trace_id")
    
    obs_service = context.observability_service
    spans = obs_service.get_trace_spans(trace_id)
    
    return {
        "status": "success",
        "data": {
            "count": len(spans),
            "spans": [
                {
                    "span_id": s.span_id,
                    "trace_id": s.trace_id,
                    "operation": s.operation_name,
                    "duration_ms": s.duration_ms,
                    "status": s.status
                }
                for s in spans[-100:]
            ]
        }
    }

async def handle_get_summary(context: DMSCServiceContext):
    """Get observability summary"""
    obs_service = context.observability_service
    summary = obs_service.get_metrics_summary()
    
    return {"status": "success", "data": summary}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    # Configure observability
    app.with_observability(DMSCObservabilityConfig(
        enable_metrics=True,
        enable_tracing=True,
        enable_health_checks=True,
        metrics_backend="prometheus",
        tracing_backend="jaeger"
    ))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize observability module
    obs_config = DMSCObservabilityConfig(
        enable_metrics=True,
        enable_tracing=True,
        enable_health_checks=True
    )
    obs_module = DMSCObservabilityModule(obs_config)
    
    # Initialize observability service
    obs_service = ObservabilityService(obs_module, dms_app.context)
    dms_app.context.observability_service = obs_service
    
    # Register health checks
    async def database_health_check():
        return DMSCHealthStatus(healthy=True, message="Database connected")
    
    async def cache_health_check():
        return DMSCHealthStatus(healthy=True, message="Cache operational")
    
    async def api_health_check():
        return DMSCHealthStatus(healthy=True, message="API responding")
    
    obs_service.register_health_check("database", DMSCHealthCheck(database_health_check))
    obs_service.register_health_check("cache", DMSCHealthCheck(cache_health_check))
    obs_service.register_health_check("api", DMSCHealthCheck(api_health_check))
    
    # Add routes
    dms_app.router.add_route("POST", "/traces/span", handle_create_span)
    dms_app.router.add_route("POST", "/traces/span/end", handle_end_span)
    dms_app.router.add_route("POST", "/metrics/record", handle_record_metric)
    dms_app.router.add_route("GET", "/health", handle_check_health)
    dms_app.router.add_route("POST", "/alerts", handle_create_alert)
    dms_app.router.add_route("POST", "/alerts/list", handle_get_alerts)
    dms_app.router.add_route("POST", "/alerts/acknowledge", handle_acknowledge_alert)
    dms_app.router.add_route("POST", "/traces", handle_get_traces)
    dms_app.router.add_route("GET", "/summary", handle_get_summary)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Observability Architecture

1. **Distributed Tracing**: Create and manage trace spans with parent-child relationships
2. **Metrics Collection**: Record custom metrics with different types (counter, gauge, histogram)
3. **Health Checks**: Register and run health checks for components
4. **Alerting**: Create and manage alerts with severity levels
5. **Span Management**: Track and analyze trace spans

### Key Components

- **DMSCObservabilityModule**: Main observability interface
- **DMSCOpenTelemetryTracer**: Distributed tracing
- **DMSCMetricsCollector**: Metrics collection and storage
- **DMSCHealthCheck**: Health check registration and execution
- **DMSCLocalMetric**: Custom metric representation

## Running Steps

1. Save the code to `observability_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc opentelemetry-api opentelemetry-sdk prometheus-client
   ```
3. Run the application:
   ```bash
   python observability_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Create a trace span
   curl -X POST http://localhost:8080/traces/span \
     -H "Content-Type: application/json" \
     -d '{"operation_name": "process_order", "attributes": {"order_id": "123"}}'
   
   # End a trace span
   curl -X POST http://localhost:8080/traces/span/end \
     -H "Content-Type: application/json" \
     -d '{"span_id": "span_xxx", "status": "ok", "events": [{"name": "step1_completed"}]}'
   
   # Record a metric
   curl -X POST http://localhost:8080/metrics/record \
     -H "Content-Type: application/json" \
     -d '{"name": "orders_processed", "value": 1, "type": "counter", "labels": {"region": "us-east"}}'
   
   # Check health
   curl http://localhost:8080/health
   
   # Create an alert
   curl -X POST http://localhost:8080/alerts \
     -H "Content-Type: application/json" \
     -d '{"severity": "warning", "component": "database", "message": "High latency detected"}'
   
   # Get active alerts
   curl -X POST http://localhost:8080/alerts/list \
     -H "Content-Type: application/json" \
     -d '{"severity": "warning"}'
   
   # Get trace spans
   curl -X POST http://localhost:8080/traces \
     -H "Content-Type: application/json" \
     -d '{"trace_id": "trace_xxx"}'
   
   # Get summary
   curl http://localhost:8080/summary
   ```

## Expected Output

### Create Span Response

```json
{
  "status": "success",
  "data": {
    "span_id": "span_1705313400.123",
    "trace_id": "trace_abc123",
    "operation_name": "process_order",
    "start_time": "2024-01-15T10:30:00"
  }
}
```

### Health Check Response

```json
{
  "status": "success",
  "data": {
    "components": {
      "database": "healthy",
      "cache": "healthy",
      "api": "healthy"
    },
    "timestamp": "2024-01-15T10:30:00"
  }
}
```

### Summary Response

```json
{
  "status": "success",
  "data": {
    "total_spans": 1500,
    "active_spans": 5,
    "active_alerts": 2,
    "component_status": {
      "database": "healthy",
      "cache": "healthy",
      "api": "healthy"
    }
  }
}
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
