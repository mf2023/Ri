#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Observability Module Example

This example demonstrates how to use the observability module in DMSC,
including metrics collection, distributed tracing, and health monitoring.

Features Demonstrated:
- Metrics creation and recording
- Distributed tracing with spans
- Custom metrics (counter, gauge, histogram)
- Trace propagation
- Health checks
"""

import dmsc
from dmsc.observability import (
    DMSCObservabilityModule, DMSCObservabilityConfig,
    DMSCMetricsRegistry, DMSCTracer, DMSCMetricType,
)
import asyncio


async def main():
    """
    Main async entry point for the observability module example.
    
    This function demonstrates the complete observability workflow including:
    - Observability module initialization and configuration
    - Custom metrics creation (counter, gauge, histogram)
    - Metrics recording and updating
    - Distributed tracing with parent and child spans
    - Metrics snapshot retrieval
    - Prometheus metrics export format
    - Health check registration and execution
    - System metrics collection
    
    The example shows how DMSC provides comprehensive observability
    capabilities for monitoring, tracing, and health checking.
    """
    print("=== DMSC Observability Module Example ===\n")
    
    # Configuration Setup: Create observability module configuration
    # Parameters:
    # - service_name: Name of the service for identification
    # - service_version: Version string for metrics organization
    # - environment: Deployment environment (development, staging, production)
    # - metrics_enabled: Enable/disable metrics collection
    # - tracing_enabled: Enable/disable distributed tracing
    # - tracing_sample_rate: Percentage of traces to sample (0.0 to 1.0)
    config = DMSCObservabilityConfig(
        service_name="dmsc-example",
        service_version="1.0.0",
        environment="development",
        metrics_enabled=True,
        tracing_enabled=True,
        tracing_sample_rate=1.0,
    )
    
    # Module Initialization: Create observability module instance
    # The module provides unified observability including metrics, tracing, and health checks
    print("1. Creating observability module...")
    observability = await DMSCObservabilityModule.create(config)
    
    # Get sub-modules for observability operations
    # - Metrics: For collecting and reporting measurements
    # - Tracer: For distributed tracing operations
    metrics = observability.get_metrics()
    tracer = observability.get_tracer()
    print("   Observability module initialized\n")
    
    # Step 2: Create custom metrics
    # Demonstrates metric definition with different types
    # Metrics are identified by name and can have labels for dimensional analysis
    print("2. Creating custom metrics...")
    
    # Counter metric: Incremental count of events
    # Use for: request counts, error counts, operation counts
    # - name: Unique metric identifier
    # - description: Human-readable explanation of what the metric measures
    # - metric_type: Type of metric (COUNTER, GAUGE, HISTOGRAM)
    # - labels: Dimensions for filtering and grouping (method, endpoint, status)
    request_counter = metrics.create_metric(
        name="http_requests_total",
        description="Total number of HTTP requests",
        metric_type=DMSCMetricType.COUNTER,
        labels=["method", "endpoint", "status"],
    )
    print("   Created counter: http_requests_total\n")
    
    # Gauge metric: Current value that can go up or down
    # Use for: active connections, queue length, memory usage
    active_connections = metrics.create_metric(
        name="active_connections",
        description="Number of active connections",
        metric_type=DMSCMetricType.GAUGE,
        labels=["protocol"],
    )
    print("   Created gauge: active_connections\n")
    
    # Histogram metric: Distribution of values (percentiles, averages)
    # Use for: request duration, response size, processing time
    request_duration = metrics.create_metric(
        name="http_request_duration_seconds",
        description="HTTP request duration in seconds",
        metric_type=DMSCMetricType.HISTOGRAM,
        labels=["method", "endpoint"],
    )
    print("   Created histogram: http_request_duration_seconds\n")
    
    # Step 3: Record metrics
    # Demonstrates how to update metric values
    print("3. Recording metrics...")
    
    # Increment counter with specific label values
    # Each combination of label values creates a distinct time series
    request_counter.increment(["GET", "/api/users", "200"])
    request_counter.increment(["GET", "/api/users", "200"])
    request_counter.increment(["POST", "/api/users", "201"])
    request_counter.increment(["GET", "/api/orders", "500"])
    print("   Recorded request metrics\n")
    
    # Set gauge to specific value
    # Gauges represent current state (can increase or decrease)
    active_connections.set(100, ["HTTP"])
    active_connections.set(25, ["WebSocket"])
    print("   Updated connection gauges\n")
    
    # Record histogram values (duration measurements)
    # Histograms automatically calculate percentiles (p50, p95, p99, etc.)
    request_duration.record(0.125, ["GET", "/api/users"])
    request_duration.record(0.250, ["GET", "/api/users"])
    request_duration.record(0.075, ["POST", "/api/users"])
    print("   Recorded duration metrics\n")
    
    # Step 4: Business metrics
    # Demonstrates custom application-specific metrics
    print("4. Recording custom business metrics...")
    
    # User registration counter by source
    user_registrations = metrics.create_metric(
        name="user_registrations_total",
        description="Total number of user registrations",
        metric_type=DMSCMetricType.COUNTER,
        labels=["source"],
    )
    user_registrations.increment(["web"])
    user_registrations.increment(["mobile"])
    user_registrations.increment(["web"])
    print("   Recorded user registration metrics\n")
    
    # Step 5: Distributed tracing
    # Demonstrates tracing of request flows across services
    # Traces consist of spans representing individual operations
    print("5. Creating distributed trace...")
    
    # Create parent span for main request processing
    # Spans track timing and context through the request lifecycle
    # Context propagation allows linking spans across services
    with tracer.span("process_user_request", {"user_id": "user-123", "operation": "update_profile"}) as parent_span:
        # Child span for input validation phase
        # Child spans are automatically linked to parent
        with parent_span.child_span("validate_input") as child_span:
            print("   Created parent span: process_user_request")
            print("   Created child span: validate_input")
        
        # Child span for database operation
        # Can record errors and attributes on spans
        with parent_span.child_span("database_operation", {"operation": "update", "table": "users"}) as error_span:
            print("   Created span: database_operation")
            # Record an error on the span for debugging
            error_span.record_error("Connection timeout")
    
    print("   All spans ended\n")
    
    # Step 6: Metrics snapshot
    # Retrieves current state of all registered metrics
    print("6. Getting metrics snapshot...")
    snapshot = metrics.snapshot()
    print(f"   Metrics snapshot:")
    print(f"   - Total metric families: {len(snapshot)}\n")
    
    # Step 7: Prometheus export format
    # Exports metrics in Prometheus text format for scraping
    print("7. Exporting metrics in Prometheus format...")
    prometheus_output = metrics.export_prometheus()
    print("   Prometheus metrics (first 500 chars):")
    print(f"   {prometheus_output[:500]}\n")
    
    # Step 8: Health checks
    # Demonstrates health check registration and execution
    print("8. Setting up health check...")
    
    # Register health check with custom async function
    # Health checks verify component availability
    await observability.register_health_check("database", async lambda: dmsc.observability.DMSCHealthStatus.Healthy)
    print("   Registered 'database' health check\n")
    
    # Step 9: Run health checks
    # Executes all registered health checks and reports status
    print("9. Running health checks...")
    health_report = await observability.run_health_checks()
    print("   Health report:")
    print(f"   - Overall status: {health_report.overall_status()}")
    print("   - Check results:")
    for name, result in health_report.results().items():
        print(f"     - {name}: {result.status()}")
    print()
    
    # Step 10: System metrics
    # Demonstrates infrastructure monitoring metrics
    print("10. Recording system metrics...")
    
    # CPU usage by core
    cpu_usage = metrics.create_metric(
        name="system_cpu_usage_percent",
        description="CPU usage percentage",
        metric_type=DMSCMetricType.GAUGE,
        labels=["core"],
    )
    cpu_usage.set(45.5, ["core-0"])
    cpu_usage.set(52.3, ["core-1"])
    
    # Memory usage by type (heap, stack, etc.)
    memory_usage = metrics.create_metric(
        name="system_memory_usage_bytes",
        description="Memory usage in bytes",
        metric_type=DMSCMetricType.GAUGE,
        labels=["type"],
    )
    memory_usage.set(1024 * 1024 * 512, ["heap"])
    memory_usage.set(1024 * 1024 * 128, ["stack"])
    print("   Recorded system metrics\n")
    
    # Step 11: Observability statistics
    # Get aggregated statistics about observability operations
    print("11. Getting observability statistics...")
    stats = await observability.get_statistics()
    print("   Observability statistics:")
    print(f"   - Metrics recorded: {stats.metrics_recorded()}")
    print(f"   - Spans created: {stats.spans_created()}")
    print(f"   - Errors recorded: {stats.errors_recorded()}\n")
    
    print("=== Observability Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
