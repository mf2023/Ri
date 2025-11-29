// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
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

extern crate dms;

use core::assert;
use core::assert_eq;
use core::option::Option;
use core::option::Option::{Some, None};
use core::result::Result::Ok;
use std::collections::HashMap;
use std::time::SystemTime;
use std::time::Duration;

use dms::service_mesh::{DMSServiceMeshConfig, DMSServiceMesh, DMSServiceHealthStatus, DMSServiceEndpoint};
use dms::service_mesh::{DMSServiceInstance, DMSServiceStatus};
use dms::service_mesh::{DMSHealthCheckResult, DMSHealthSummary, DMSHealthStatus};
use dms::service_mesh::{DMSTrafficRoute, DMSMatchCriteria, DMSRouteAction};

#[tokio::test]
async fn test_service_mesh_config_default() {
    let config = DMSServiceMeshConfig::default();
    
    assert!(config.enable_service_discovery);
    assert!(config.enable_health_check);
    assert!(config.enable_traffic_management);
    assert_eq!(config.health_check_interval.as_secs(), 30);
    assert_eq!(config.max_retry_attempts, 3);
    assert_eq!(config.retry_timeout.as_secs(), 5);
}

#[tokio::test]
async fn test_service_mesh_new() {
    let config = DMSServiceMeshConfig::default();
    
    let service_mesh = DMSServiceMesh::new(config).unwrap();
    
    // Verify components are created
    assert!(service_mesh.get_service_discovery().is_some());
    assert!(service_mesh.get_health_checker().is_some());
    assert!(service_mesh.get_traffic_manager().is_some());
    assert!(service_mesh.get_circuit_breaker().is_some());
    assert!(service_mesh.get_load_balancer().is_some());
}

#[tokio::test]
async fn test_service_mesh_register_service() {
    let config = DMSServiceMeshConfig::default();
    let service_mesh = DMSServiceMesh::new(config).unwrap();
    
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
    let config = DMSServiceMeshConfig::default();
    let service_mesh = DMSServiceMesh::new(config).unwrap();
    
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

#[tokio::test]
async fn test_service_instance_new() {
    let instance = DMSServiceInstance {
        id: "test_instance".to_string(),
        service_name: "test_service".to_string(),
        host: "localhost".to_string(),
        port: 8080,
        status: DMSServiceStatus::Running,
        metadata: std::collections::HashMap::new(),
        health_check_url: "http://localhost:8080/health".to_string(),
        last_heartbeat: std::time::SystemTime::now(),
    };
    
    assert_eq!(instance.id, "test_instance");
    assert_eq!(instance.service_name, "test_service");
    assert_eq!(instance.host, "localhost");
    assert_eq!(instance.port, 8080);
    assert_eq!(instance.status, DMSServiceStatus::Running);
}

#[tokio::test]
async fn test_health_check_result_new() {
    let result = DMSHealthCheckResult {
        service_id: "test_service".to_string(),
        instance_id: "test_instance".to_string(),
        status: DMSHealthStatus::Healthy,
        details: "All checks passed".to_string(),
        timestamp: std::time::SystemTime::now(),
        response_time_ms: 100,
    };
    
    assert_eq!(result.service_id, "test_service");
    assert_eq!(result.instance_id, "test_instance");
    assert_eq!(result.status, DMSHealthStatus::Healthy);
    assert_eq!(result.details, "All checks passed");
}

#[tokio::test]
async fn test_health_summary_new() {
    let summary = DMSHealthSummary {
        total_services: 10,
        healthy_services: 8,
        unhealthy_services: 2,
        unknown_services: 0,
        total_instances: 20,
        healthy_instances: 16,
        unhealthy_instances: 4,
        unknown_instances: 0,
        last_update: std::time::SystemTime::now(),
    };
    
    assert_eq!(summary.total_services, 10);
    assert_eq!(summary.healthy_services, 8);
    assert_eq!(summary.unhealthy_services, 2);
    assert_eq!(summary.total_instances, 20);
    assert_eq!(summary.healthy_instances, 16);
    assert_eq!(summary.unhealthy_instances, 4);
}

#[tokio::test]
async fn test_traffic_route_new() {
    let match_criteria = DMSMatchCriteria {
        path: Some("/api/v1/*".to_string()),
        method: Some("GET".to_string()),
        headers: std::collections::HashMap::new(),
        query_params: std::collections::HashMap::new(),
    };
    
    let route_action = DMSRouteAction {
        destination: "backend_service".to_string(),
        weight: 100,
        headers: std::collections::HashMap::new(),
        timeout: Some(std::time::Duration::from_secs(30)),
    };
    
    let route = DMSTrafficRoute {
        id: "test_route".to_string(),
        name: "Test Route".to_string(),
        match_criteria,
        action: route_action,
        priority: 100,
        enabled: true,
    };
    
    assert_eq!(route.id, "test_route");
    assert_eq!(route.name, "Test Route");
    assert!(route.enabled);
    assert_eq!(route.priority, 100);
}

#[tokio::test]
async fn test_service_health_status() {
    // Test all health status variants
    assert_eq!(DMSServiceHealthStatus::Healthy.to_string(), "Healthy");
    assert_eq!(DMSServiceHealthStatus::Unhealthy.to_string(), "Unhealthy");
    assert_eq!(DMSServiceHealthStatus::Unknown.to_string(), "Unknown");
}
