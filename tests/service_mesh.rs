// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Service Mesh Module Tests
//!
//! This module contains comprehensive tests for the DMSC service mesh system,
//! providing service discovery, health checking, traffic management, circuit breaking,
//! and load balancing for building resilient microservice architectures.
//!
//! ## Test Coverage
//!
//! - **DMSCServiceMeshConfig**: Tests for service mesh configuration including feature
//!   toggles (discovery, health checks, traffic management), intervals, timeouts, and
//!   retry policies
//!
//! - **DMSCServiceMesh**: Tests for the core service mesh orchestrating all components
//!   including service registry, health checker, traffic manager, circuit breaker, and
//!   load balancer
//!
//! - **DMSCServiceInstance**: Tests for service instance representation including
//!   identification, endpoint information, status, metadata, and heartbeat tracking
//!
//! - **DMSCHealthCheck**: Tests for health checking including result structures,
//!   summary aggregation, success rate tracking, and response time monitoring
//!
//! - **DMSCTrafficRoute**: Tests for traffic routing including match criteria (path,
//!   headers, method, query), weighted destinations, retry policies, timeouts, and
//!   fault injection configuration
//!
//! - **DMSCServiceStatus**: Tests for service instance states (Running, Stopped, Unknown)
//!
//! ## Architecture
//!
//! The service mesh implements a sidecar-less architecture for DMSC services:
//! - **Service Discovery**: Registry mapping service names to healthy instances
//! - **Health Checking**: Active and passive health verification with configurable intervals
//! - **Traffic Management**: Advanced routing with subset-based deployments and traffic shifting
//! - **Resilience**: Circuit breaking, retry policies, and timeouts for fault tolerance
//! - **Load Balancing**: Multiple strategies with consistent hashing support
//!
//! ## Health Check States
//!
//! The health system tracks service availability:
//! - **Healthy**: Service passes health checks and receives traffic
//! - **Unhealthy**: Service failed checks and is temporarily excluded
//! - **Unknown**: Initial state before first health check completes
//!
//! Health checks can be configured with:
//! - **Interval**: How often to check service health
//! - **Timeout**: Maximum wait for health check response
//! - **Unhealthy Threshold**: Consecutive failures before marking unhealthy
//! - **Healthy Threshold**: Consecutive successes before marking healthy
//!
//! ## Traffic Routing
//!
//! Traffic routes enable sophisticated traffic management:
//! - **Subset Routing**: Route to specific service versions (canary, blue-green)
//! - **Weight-Based Routing**: Split traffic between destinations
//! - **Header-Based Routing**: Route based on request headers
//! - **Path-Based Routing**: Match requests by path prefix or exact match
//!
//! ## Resilience Patterns
//!
//! The service mesh implements multiple resilience patterns:
//! - **Circuit Breaker**: Prevents cascade failures by failing fast
//! - **Retry Policies**: Automatic retries with exponential backoff
//! - **Timeouts**: Request timeout enforcement
//! - **Fault Injection**: Inject delays and aborts for chaos testing

use dmsc::service_mesh::{DMSCServiceMeshConfig, DMSCServiceMesh, DMSCServiceHealthStatus, DMSCServiceEndpoint};
use dmsc::service_mesh::{DMSCServiceInstance, DMSCServiceStatus};
use dmsc::service_mesh::{DMSCHealthCheckResult, DMSCHealthSummary, DMSCHealthStatus};
use dmsc::service_mesh::{DMSCTrafficRoute, DMSCMatchCriteria, DMSCRouteAction};

#[test]
fn test_service_mesh_config_default() {
    let config = DMSCServiceMeshConfig::default();
    
    assert!(config.enable_service_discovery);
    assert!(config.enable_health_check);
    assert!(config.enable_traffic_management);
    assert_eq!(config.health_check_interval.as_secs(), 30);
    assert_eq!(config.max_retry_attempts, 3);
    assert_eq!(config.retry_timeout.as_secs(), 5);
}

#[test]
fn test_service_mesh_new() {
    let config = DMSCServiceMeshConfig::default();
    
    let service_mesh = DMSCServiceMesh::new(config).unwrap();
    
    // Verify components are created
    let _ = service_mesh.get_service_discovery();
    let _ = service_mesh.get_health_checker();
    let _ = service_mesh.get_traffic_manager();
    let _ = service_mesh.get_circuit_breaker();
    let _ = service_mesh.get_load_balancer();
}

#[tokio::test]
async fn test_service_mesh_register_service() {
    let config = DMSCServiceMeshConfig::default();
    let service_mesh = DMSCServiceMesh::new(config).unwrap();
    
    // Test registering a service
    let service_name = "test_service";
    let endpoint = "http://localhost:8080";
    let weight = 100;
    
    service_mesh.register_service(service_name, endpoint, weight).await.unwrap();
    
    // Test discovering the service
    let endpoints = service_mesh.discover_service(service_name).await;
    
    // Should return an error because the service is not healthy yet
    assert!(endpoints.is_err());
}

#[tokio::test]
async fn test_service_mesh_update_service_health() {
    let config = DMSCServiceMeshConfig::default();
    let service_mesh = DMSCServiceMesh::new(config).unwrap();
    
    // Register a service
    let service_name = "test_service";
    let endpoint = "http://localhost:8080";
    let weight = 100;
    
    service_mesh.register_service(service_name, endpoint, weight).await.unwrap();
    
    // Update service health to healthy
    service_mesh.update_service_health(service_name, endpoint, true).await.unwrap();
    
    // Discover the service
    let endpoints = service_mesh.discover_service(service_name).await;
    
    // Should return the service now that it's healthy
    assert!(endpoints.is_ok());
    assert_eq!(endpoints.unwrap().len(), 1);
    
    // Update service health to unhealthy
    service_mesh.update_service_health(service_name, endpoint, false).await.unwrap();
    
    // Discover the service again
    let endpoints = service_mesh.discover_service(service_name).await;
    
    // Should return an error because the service is unhealthy
    assert!(endpoints.is_err());
}

#[test]
fn test_service_instance_new() {
    let instance = DMSCServiceInstance {
        id: "test_instance".to_string(),
        service_name: "test_service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        status: DMSCServiceStatus::Running,
        metadata: std::collections::HashMap::new(),
        registered_at: std::time::SystemTime::now(),
        last_heartbeat: std::time::SystemTime::now(),
    };
    
    assert_eq!(instance.id, "test_instance");
    assert_eq!(instance.service_name, "test_service");
    assert_eq!(instance.host, "localhost");
    assert_eq!(instance.port, 8080);
    assert_eq!(instance.status, DMSCServiceStatus::Running);
}

#[test]
fn test_health_check_result_new() {
    let result = DMSCHealthCheckResult {
        service_name: "test_service".to_string(),
        endpoint: "http://localhost:8080/health".to_string(),
        is_healthy: true,
        status_code: Some(200),
        response_time: std::time::Duration::from_millis(100),
        error_message: None,
        timestamp: std::time::SystemTime::now(),
    };

    assert_eq!(result.service_name, "test_service");
    assert_eq!(result.endpoint, "http://localhost:8080/health");
    assert_eq!(result.is_healthy, true);
    assert_eq!(result.status_code, Some(200));
    assert_eq!(result.response_time, std::time::Duration::from_millis(100));
}

#[test]
fn test_health_summary_new() {
    let summary = DMSCHealthSummary {
        service_name: "test_service".to_string(),
        total_checks: 20,
        healthy_checks: 16,
        unhealthy_checks: 4,
        success_rate: 80.0,
        average_response_time: std::time::Duration::from_millis(100),
        last_check_time: Some(std::time::SystemTime::now()),
        overall_status: DMSCHealthStatus::Healthy,
    };
    
    assert_eq!(summary.total_checks, 20);
    assert_eq!(summary.healthy_checks, 16);
    assert_eq!(summary.unhealthy_checks, 4);
    assert_eq!(summary.success_rate, 80.0);
}

#[test]
fn test_traffic_route_new() {
    let match_criteria = DMSCMatchCriteria {
        path_prefix: Some("/api/v1/".to_string()),
        headers: std::collections::HashMap::new(),
        method: Some("GET".to_string()),
        query_parameters: std::collections::HashMap::new(),
    };
    
    let weighted_destinations = vec![
        dmsc::service_mesh::DMSCWeightedDestination {        
            service: "backend_service".to_string(),
            weight: 100,
            subset: None,
        },
    ];
    
    let route_action = DMSCRouteAction::Route(weighted_destinations);
    
    let route = DMSCTrafficRoute {
        name: "test_route".to_string(),
        source_service: "frontend_service".to_string(),
        destination_service: "backend_service".to_string(),
        match_criteria,
        route_action,
        retry_policy: None,
        timeout: Some(std::time::Duration::from_secs(30)),
        fault_injection: None,
    };

    assert_eq!(route.name, "test_route");
    assert_eq!(route.source_service, "frontend_service");
    assert_eq!(route.destination_service, "backend_service");
}

#[test]
fn test_service_health_status() {
    // Test all health status variants
    assert_eq!(format!("{:?}", DMSCServiceHealthStatus::Healthy), "Healthy");
    assert_eq!(format!("{:?}", DMSCServiceHealthStatus::Unhealthy), "Unhealthy");
    assert_eq!(format!("{:?}", DMSCServiceHealthStatus::Unknown), "Unknown");
}
