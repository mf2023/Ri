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

//! # DMSC Observability Module Example
//!
//! This example demonstrates how to use the observability module in DMSC,
//! including metrics collection, distributed tracing, and health monitoring.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example observability --features observability
//! ```
//!
//! ## Features Demonstrated
//!
//! - Metrics creation and recording
//! - Distributed tracing with spans
//! - Custom metrics (counter, gauge, histogram)
//! - Trace propagation
//! - Health checks

use dmsc::observability::{DMSCObservabilityModule, DMSCObservabilityConfig, DMSCMetricsRegistry, DMSCTracer, DMSCMetricType};
use dmsc::core::DMSCResult;

/// Main entry point for the observability module example.
///
/// This function demonstrates the complete observability workflow including:
/// - Observability module initialization and configuration
/// - Custom metrics creation (counter, gauge, histogram) with labels
/// - Metrics recording and updating with various metric types
/// - Distributed tracing with parent and child spans
/// - Metrics snapshot retrieval and Prometheus export format
/// - Health check registration and execution
/// - System metrics collection
///
/// The example shows how DMSC provides comprehensive observability
/// capabilities for monitoring, tracing, and health checking
/// in a Rust async runtime environment.
fn main() -> DMSCResult<()> {
    println!("=== DMSC Observability Module Example ===\n");

    // Create async runtime for handling asynchronous observability operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute all async observability operations within the runtime
    rt.block_on(async {
        // Configuration Setup: Create observability module configuration
        // Using builder pattern for configuration parameters:
        // - service_name: Name of the service for identification
        // - service_version: Version string for metrics organization
        // - environment: Deployment environment (development, staging, production)
        // - metrics_enabled: Enable/disable metrics collection
        // - tracing_enabled: Enable/disable distributed tracing
        // - tracing_sample_rate: Percentage of traces to sample (0.0 to 1.0)
        // - build(): Finalizes configuration into DMSCObservabilityConfig struct
        let config = DMSCObservabilityConfig::new()
            .with_service_name("dmsc-example")
            .with_service_version("1.0.0")
            .with_environment("development")
            .with_metrics_enabled(true)
            .with_tracing_enabled(true)
            .with_tracing_sample_rate(1.0)
            .build();

        // Module Initialization: Create observability module instance
        // The module provides unified observability including metrics, tracing, and health checks
        println!("1. Creating observability module...");
        let observability = DMSCObservabilityModule::new(config).await?;
        
        // Get sub-modules for observability operations
        // - Metrics: For collecting and reporting measurements
        // - Tracer: For distributed tracing operations
        let metrics = observability.get_metrics();
        let tracer = observability.get_tracer();
        println!("   Observability module initialized\n");

        // Step 2: Create custom metrics
        // Demonstrates metric definition with different types
        // Metrics are identified by name and can have labels for dimensional analysis
        println!("2. Creating custom metrics...");

        // Counter metric: Incremental count of events
        // Use for: request counts, error counts, operation counts
        // create_metric() parameters:
        // - name: &str unique metric identifier
        // - description: &str human-readable explanation
        // - metric_type: DMSCMetricType enum variant
        // - labels: Vec<&str> dimensions for filtering and grouping
        let request_counter = metrics.create_metric(
            "http_requests_total",
            "Total number of HTTP requests",
            DMSCMetricType::Counter,
            vec!["method", "endpoint", "status"],
        )?;
        println!("   Created counter: http_requests_total\n");

        // Gauge metric: Current value that can go up or down
        // Use for: active connections, queue length, memory usage
        let active_connections = metrics.create_metric(
            "active_connections",
            "Number of active connections",
            DMSCMetricType::Gauge,
            vec!["protocol"],
        )?;
        println!("   Created gauge: active_connections\n");

        // Histogram metric: Distribution of values (percentiles, averages)
        // Use for: request duration, response size, processing time
        let request_duration = metrics.create_metric(
            "http_request_duration_seconds",
            "HTTP request duration in seconds",
            DMSCMetricType::Histogram,
            vec!["method", "endpoint"],
        )?;
        println!("   Created histogram: http_request_duration_seconds\n");

        // Step 3: Record metrics
        // Demonstrates how to update metric values
        println!("3. Recording metrics...");

        // Increment counter with specific label values
        // Each combination of label values creates a distinct time series
        // increment() takes Vec<&str> matching the label configuration
        request_counter.increment(vec!["GET", "/api/users", "200"])?;
        request_counter.increment(vec!["GET", "/api/users", "200"])?;
        request_counter.increment(vec!["POST", "/api/users", "201"])?;
        request_counter.increment(vec!["GET", "/api/orders", "500"])?;
        println!("   Recorded request metrics\n");

        // Set gauge to specific value
        // Gauges represent current state (can increase or decrease)
        // set() takes value (f64) and labels
        active_connections.set(100, vec!["HTTP"])?;
        active_connections.set(25, vec!["WebSocket"])?;
        println!("   Updated connection gauges\n");

        // Record histogram values (duration measurements)
        // Histograms automatically calculate percentiles (p50, p95, p99, etc.)
        // record() takes observation value (f64) and labels
        request_duration.record(0.125, vec!["GET", "/api/users"])?;
        request_duration.record(0.250, vec!["GET", "/api/users"])?;
        request_duration.record(0.075, vec!["POST", "/api/users"])?;
        println!("   Recorded duration metrics\n");

        // Step 4: Business metrics
        // Demonstrates custom application-specific metrics
        println!("4. Recording custom business metrics...");

        // User registration counter by source
        let user_registrations = metrics.create_metric(
            "user_registrations_total",
            "Total number of user registrations",
            DMSCMetricType::Counter,
            vec!["source"],
        )?;
        user_registrations.increment(vec!["web"])?;
        user_registrations.increment(vec!["mobile"])?;
        user_registrations.increment(vec!["web"])?;
        println!("   Recorded user registration metrics\n");

        // Step 5: Distributed tracing
        // Demonstrates tracing of request flows across services
        // Traces consist of spans representing individual operations
        println!("5. Creating distributed trace...");

        // Create parent span for main request processing
        // Spans track timing and context through the request lifecycle
        // span() creates a new span with name and attributes
        // with_attribute() adds key-value metadata to the span
        // start() begins the span (creates it in the tracing system)
        let _span = tracer.span("process_user_request")
            .with_attribute("user_id", "user-123")
            .with_attribute("operation", "update_profile")
            .start();

        {
            // Create child span for input validation phase
            // Child spans are automatically linked to parent via context
            let child_span = tracer.span("validate_input")
                .parent(&_span)
                .start();
            
            println!("   Created parent span: process_user_request");
            println!("   Created child span: validate_input");
            
            // End child span when scope closes
            child_span.end();
        }

        // Create another child span for database operation
        let error_span = tracer.span("database_operation")
            .parent(&_span)
            .with_attribute("operation", "update")
            .with_attribute("table", "users")
            .start();

        println!("   Created span: database_operation");
        // Record an error on the span for debugging
        // Errors are tracked separately and can trigger alerts
        error_span.record_error("Connection timeout");
        error_span.end();

        // End parent span when done
        _span.end();
        println!("   All spans ended\n");

        // Step 6: Metrics snapshot
        // Retrieves current state of all registered metrics
        println!("6. Getting metrics snapshot...");
        let snapshot = metrics.snapshot();
        println!("   Metrics snapshot:");
        println!("   - Total metric families: {}\n", snapshot.len());

        // Step 7: Prometheus export format
        // Exports metrics in Prometheus text format for scraping
        // Prometheus format is widely supported by monitoring systems
        println!("7. Exporting metrics in Prometheus format...");
        let prometheus_output = metrics.export_prometheus();
        println!("   Prometheus metrics (first 500 chars):");
        // Use .min() to safely slice if string is shorter than 500 chars
        println!("   {}\n", &prometheus_output[..500.min(prometheus_output.len())]);

        // Step 8: Health checks
        // Demonstrates health check registration and execution
        println!("8. Setting up health check...");

        // Register health check with custom async function
        // Health checks verify component availability
        // Each health check has a name and async evaluation function
        observability.register_health_check("database", async {
            Ok(dmsc::observability::DMSCHealthStatus::Healthy)
        }).await;
        println!("   Registered 'database' health check\n");

        // Step 9: Run health checks
        // Executes all registered health checks and reports status
        println!("9. Running health checks...");
        let health_report = observability.run_health_checks().await?;
        println!("   Health report:");
        println!("   - Overall status: {:?}", health_report.overall_status());
        println!("   - Check results:");
        for (name, result) in health_report.results() {
            println!("     - {}: {:?}", name, result.status());
        }
        println!();

        // Step 10: System metrics
        // Demonstrates infrastructure monitoring metrics
        println!("10. Recording system metrics...");

        // CPU usage by core
        let cpu_usage = metrics.create_metric(
            "system_cpu_usage_percent",
            "CPU usage percentage",
            DMSCMetricType::Gauge,
            vec!["core"],
        )?;
        cpu_usage.set(45.5, vec!["core-0"])?;
        cpu_usage.set(52.3, vec!["core-1"])?;
        
        // Memory usage by type (heap, stack, etc.)
        let memory_usage = metrics.create_metric(
            "system_memory_usage_bytes",
            "Memory usage in bytes",
            DMSCMetricType::Gauge,
            vec!["type"],
        )?;
        memory_usage.set(1024 * 1024 * 512, vec!["heap"])?;
        memory_usage.set(1024 * 1024 * 128, vec!["stack"])?;
        println!("   Recorded system metrics\n");

        // Step 11: Observability statistics
        // Get aggregated statistics about observability operations
        println!("11. Getting observability statistics...");
        let stats = observability.get_statistics().await?;
        println!("   Observability statistics:");
        println!("   - Metrics recorded: {}", stats.metrics_recorded());
        println!("   - Spans created: {}", stats.spans_created());
        println!("   - Errors recorded: {}\n", stats.errors_recorded());

        println!("=== Observability Example Completed ===");
        Ok::<(), DMSCError>(())
    })?
}
