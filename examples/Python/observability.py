# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri Observability Module Example

This example demonstrates how to use the Ri observability module for
metrics collection, distributed tracing, and monitoring.
"""

import asyncio
from ri import (
    RiObservabilityModule,
    RiObservabilityConfig,
    RiMetricsRegistry,
    RiTracer,
    RiMetricType,
    RiMetricConfig,
    RiMetricSample,
    RiMetric,
    RiObservabilityData,
)


async def main():
    # Create observability configuration
    config = RiObservabilityConfig()
    config.enable_metrics = True
    config.enable_tracing = True
    config.enable_logging = True
    config.metrics_export_interval_seconds = 60
    config.trace_sample_rate = 0.1
    config.service_name = "ri-example-service"
    config.service_version = "1.0.0"

    # Initialize observability module
    observability = RiObservabilityModule(config)

    # Create metrics registry
    metrics = RiMetricsRegistry()

    # Create metric configurations
    print("Creating metrics...")

    # Counter metric
    request_count_config = RiMetricConfig()
    request_count_config.name = "http_requests_total"
    request_count_config.description = "Total HTTP requests"
    request_count_config.metric_type = RiMetricType.COUNTER
    request_count_config.labels = ["method", "endpoint", "status"]

    # Gauge metric
    active_connections_config = RiMetricConfig()
    active_connections_config.name = "active_connections"
    active_connections_config.description = "Number of active connections"
    active_connections_config.metric_type = RiMetricType.GAUGE
    active_connections_config.labels = ["service"]

    # Histogram metric
    request_duration_config = RiMetricConfig()
    request_duration_config.name = "http_request_duration_seconds"
    request_duration_config.description = "HTTP request duration in seconds"
    request_duration_config.metric_type = RiMetricType.HISTOGRAM
    request_duration_config.labels = ["method", "endpoint"]
    request_duration_config.buckets = [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0]

    # Register metrics
    request_counter = metrics.register(request_count_config)
    active_gauge = metrics.register(active_connections_config)
    duration_histogram = metrics.register(request_duration_config)

    print(f"Registered 3 metrics")

    # Record metric samples
    print("\nRecording metric samples...")

    # Increment counter
    request_counter.increment({"method": "GET", "endpoint": "/api/users", "status": "200"})
    request_counter.increment({"method": "POST", "endpoint": "/api/users", "status": "201"})
    request_counter.increment({"method": "GET", "endpoint": "/api/orders", "status": "200"})

    # Set gauge
    active_gauge.set(42, {"service": "user-service"})
    active_gauge.set(15, {"service": "order-service"})

    # Record histogram
    duration_histogram.observe(0.023, {"method": "GET", "endpoint": "/api/users"})
    duration_histogram.observe(0.156, {"method": "POST", "endpoint": "/api/users"})
    duration_histogram.observe(0.089, {"method": "GET", "endpoint": "/api/orders"})

    print("Metrics recorded")

    # Create metric samples
    print("\nCreating metric samples...")

    sample1 = RiMetricSample()
    sample1.value = 100.0
    sample1.timestamp = 0
    sample1.labels = {"metric": "cpu_usage"}

    sample2 = RiMetricSample()
    sample2.value = 75.5
    sample2.timestamp = 0
    sample2.labels = {"metric": "memory_usage"}

    print(f"Created {2} metric samples")

    # Create tracer for distributed tracing
    print("\nConfiguring distributed tracing...")

    tracer = RiTracer()
    tracer.service_name = config.service_name
    tracer.sample_rate = config.trace_sample_rate

    # Simulate trace spans
    print("Creating trace spans...")

    # Root span
    root_span = tracer.start_span("http_request", None)
    root_span.operation = "GET /api/users"
    root_span.start_time = 0

    # Child spans
    db_span = tracer.start_span("database_query", root_span)
    db_span.operation = "SELECT * FROM users"
    db_span.start_time = 1
    db_span.end_time = 5

    cache_span = tracer.start_span("cache_lookup", root_span)
    cache_span.operation = "GET users:all"
    cache_span.start_time = 6
    cache_span.end_time = 7

    # End root span
    root_span.end_time = 10

    print(f"Created trace with 3 spans")

    # Get observability data
    print("\nGetting observability data...")

    obs_data = RiObservabilityData()
    obs_data.timestamp = 0
    obs_data.service_name = config.service_name
    obs_data.metrics_count = 3
    obs_data.traces_count = 1

    print(f"Observability data collected:")
    print(f"  Service: {obs_data.service_name}")
    print(f"  Metrics: {obs_data.metrics_count}")
    print(f"  Traces: {obs_data.traces_count}")

    # Export metrics
    print("\nExporting metrics...")
    exported_metrics = metrics.export()
    print(f"Exported {len(exported_metrics)} metrics")

    print("\nObservability operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
