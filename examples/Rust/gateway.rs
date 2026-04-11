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

//! # Ri Gateway Module Example
//!
//! This example demonstrates how to use the API gateway module in Ri,
//! including routing, rate limiting, and circuit breaker patterns.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example gateway --features gateway
//! ```
//!
//! ## Features Demonstrated
//!
//! - Route configuration and management
//! - Rate limiting policies
//! - Circuit breaker patterns
//! - Load balancing strategies

use ri::{
    RiAppBuilder,
    RiGateway,
    RiRouter,
    RiRoute,
    RiRouteTarget,
    RiRateLimiter,
    RiCircuitBreaker,
};
use std::time::Duration;

/// Async main function for the gateway module example.
///
/// This function demonstrates the complete API gateway workflow including:
/// - Gateway module initialization with nested sub-module configuration
/// - Route configuration and management for API endpoints
/// - Route lookup and matching with HTTP method validation
/// - Rate limiting configuration and request throttling simulation
/// - Circuit breaker configuration and state management
/// - Gateway statistics and monitoring
///
/// The example shows how Ri handles API gateway functionality with
/// features like request routing, traffic control, and fault tolerance
/// in a Rust async environment.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Ri Gateway Module Example ===\n");

    // Application Builder: Create application with gateway module
    // RiAppBuilder provides a fluent API for configuring modules
    // Gateway module contains nested sub-modules for different features
    let app = RiAppBuilder::new()
        .with_gateway_module(|gateway| {
            // Configure gateway with nested sub-modules
            // Each sub-module handles a specific gateway responsibility
            gateway
                // Section 1: Router Configuration
                // RiRouter manages API route definitions and lookup
                .with_router(|router| {
                    router
                        // API route: User endpoint
                        // Supports GET (retrieve) and POST (create) methods
                        // Routes to user service at localhost:8081
                        .add_route(RiRoute::new(
                            "/api/users",
                            vec!["GET", "POST"],
                            RiRouteTarget::new("http://localhost:8081"),
                        ))
                        // API route: Product endpoint
                        // Read-only endpoint (GET only)
                        // Routes to product service at localhost:8082
                        .add_route(RiRoute::new(
                            "/api/products",
                            vec!["GET"],
                            RiRouteTarget::new("http://localhost:8082"),
                        ))
                        // Admin route with wildcard matching
                        // Catches all sub-paths under /api/admin/*
                        // Supports full CRUD operations
                        // Routes to admin service at localhost:8080
                        .add_route(RiRoute::new(
                            "/api/admin/*",
                            vec!["GET", "POST", "PUT", "DELETE"],
                            RiRouteTarget::new("http://localhost:8080"),
                        ))
                        // Health check endpoint
                        // Lightweight endpoint for service health monitoring
                        // Routes to health check handler
                        .add_route(RiRoute::new(
                            "/health",
                            vec!["GET"],
                            RiRouteTarget::new("http://localhost:8080/health"),
                        ))
                })
                // Section 2: Rate Limiter Configuration
                // RiRateLimiter controls request rate to prevent abuse
                .with_rate_limiter(|limiter| {
                    limiter
                        // requests_per_second: Maximum allowed requests per second
                        // Limits sustained request rate to 100 requests/second
                        .with_requests_per_second(100)
                        // burst_size: Maximum burst capacity above normal rate
                        // Allows temporary spikes up to 200 requests
                        .with_burst_size(200)
                })
                // Section 3: Circuit Breaker Configuration
                // RiCircuitBreaker provides fault tolerance for downstream services
                .with_circuit_breaker(|breaker| {
                    breaker
                        // failure_threshold: Number of failures before opening circuit
                        // Opens circuit after 5 consecutive failures
                        .with_failure_threshold(5)
                        // recovery_timeout: Time to wait before trying again
                        // After 30 seconds, transitions to half-open state
                        .with_recovery_timeout(Duration::from_secs(30))
                })
        })
        .build()?;

    // Get gateway instance from built application
    let gateway = app.get_module::<RiGateway>();
    
    // Get sub-modules for operations and inspection
    // - Router: For route management and lookup
    // - RateLimiter: For rate limit checking
    // - CircuitBreaker: For fault tolerance state management
    let router = gateway.get_router();
    let rate_limiter = gateway.get_rate_liter();
    let circuit_breaker = gateway.get_circuit_breaker();

    // Section 1: Route Management
    // Demonstrates API route configuration and registration
    // Routes define how incoming requests are routed to backend services
    println!("1. Route Management");
    println!("   -----------------");

    // Retrieve and display all configured routes
    let routes = router.get_routes();
    println!("   Total routes configured: {}\n", routes.len());

    // Display route details for each registered route
    for route in &routes {
        println!("   Route: {} -> {}", route.path(), route.target().url());
        println!("   Methods: {:?}\n", route.methods());
    }

    // Section 2: Route Lookup
    // Demonstrates how the router matches incoming requests to routes
    // Uses path and HTTP method for matching
    println!("2. Route Lookup");
    println!("   -------------");

    // Lookup exact match for user API
    // find_route() searches for matching route by path and method
    // Returns Option<RiRoute> - Some(route) if found, None if not
    let user_route = router.find_route("/api/users", "GET");
    match user_route {
        // Pattern match to handle both found and not-found cases
        Some(route) => {
            println!("   ✓ Found route for /api/users (GET)");
            println!("   Target: {}\n", route.target().url());
        }
        None => {
            println!("   ✗ No route found for /api/users (GET)\n");
        }
    }

    // Lookup with wildcard path matching
    // /api/admin/users should match /api/admin/* pattern
    let admin_route = router.find_route("/api/admin/users", "GET");
    match admin_route {
        Some(route) => {
            println!("   ✓ Found route for /api/admin/users (GET)");
            println!("   Target: {}\n", route.target().url());
        }
        None => {
            println!("   ✗ No route found for /api/admin/users (GET)\n");
        }
    }

    // Section 3: Rate Limiting
    // Demonstrates request rate control to prevent service overload
    // Rate limiting protects backend services from excessive traffic
    println!("3. Rate Limiting");
    println!("   --------------");

    // Get current rate limiter configuration
    // Configuration values were set during gateway configuration
    let config = rate_limiter.get_config();
    println!("   Rate limiter configuration:");
    println!("   - Requests per second: {}", config.requests_per_second());
    println!("   - Burst size: {}\n", config.burst_size());

    // Simulate rate-limited requests
    // try_acquire() returns bool: true if allowed, false if rate limited
    println!("   Simulating request rate check...");
    for i in 1..=5 {
        let allowed = rate_limiter.try_acquire();
        println!("   Request {}: {}\n", i, if allowed { "✓ Allowed" } else { "✗ Rate limited" });
    }

    // Section 4: Circuit Breaker
    // Demonstrates fault tolerance pattern for handling downstream failures
    // Circuit breaker prevents cascade failures in distributed systems
    println!("4. Circuit Breaker");
    println!("   ----------------");

    // Get circuit breaker configuration
    // Configuration values were set during gateway configuration
    let breaker_config = circuit_breaker.get_config();
    println!("   Circuit breaker configuration:");
    println!("   - Failure threshold: {}", breaker_config.failure_threshold());
    println!("   - Recovery timeout: {:?}\n", breaker_config.recovery_timeout());

    // Get current circuit breaker status
    // States: CLOSED (normal), OPEN (blocking), HALF_OPEN (testing)
    let status = circuit_breaker.get_status();
    println!("   Circuit breaker status: {:?}\n", status);

    // Simulate circuit breaker operations
    // Record successes and failures to demonstrate state changes
    println!("   Simulating circuit breaker operations...");

    // Record successful operations
    // Successes help close the circuit after it opens
    // Loop 0..3 creates range with 0, 1, 2 (3 iterations)
    for _ in 0..3 {
        circuit_breaker.record_success();
    }
    println!("   Recorded 3 successes");

    // Record failures
    // Accumulated failures can trigger circuit opening
    // When failures exceed threshold, circuit transitions to OPEN
    for _ in 0..2 {
        circuit_breaker.record_failure();
    }
    println!("   Recorded 2 failures");

    // Final status after operations
    let final_status = circuit_breaker.get_status();
    println!("   Final circuit breaker status: {:?}\n", final_status);

    // Section 5: Gateway Statistics
    // Demonstrates gateway metrics and monitoring
    // Statistics track request volume, success/failure rates, and performance
    println!("5. Gateway Statistics");
    println!("   -------------------");

    // Get gateway statistics
    let stats = gateway.get_stats();
    println!("   Gateway statistics:");
    println!("   - Total requests: {}", stats.total_requests());
    println!("   - Successful requests: {}", stats.successful_requests());
    println!("   - Failed requests: {}", stats.failed_requests());
    println!("   - Average response time: {:?}\n", stats.avg_response_time());

    println!("=== Gateway Example Completed ===");
    Ok(())
}
