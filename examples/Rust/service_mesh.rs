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

//! # Ri Service Mesh Module Example
//!
//! This example demonstrates how to use the service mesh module in Ri,
//! including service discovery, health checks, and traffic management.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example service_mesh --features service_mesh
//! ```
//!
//! ## Features Demonstrated
//!
//! - Service registration and discovery
//! - Health check configuration
//! - Instance management
//! - Service status monitoring

use ri::service_mesh::{RiServiceMesh, RiServiceMeshConfig, RiServiceDiscovery, RiServiceInstance, RiServiceStatus};
use ri::core::RiResult;

/// Main entry point for the service mesh module example.
///
/// This function demonstrates the complete service mesh workflow including:
/// - Service mesh configuration and initialization
/// - Service instance registration with metadata (host, port, version)
/// - Service discovery and instance lookup by service name
/// - Instance health status management and updates
/// - Health check execution and reporting
/// - Service mesh statistics and monitoring
/// - Service deregistration and cleanup
///
/// The example shows how Ri handles service mesh functionality for
/// microservices architecture with dynamic service discovery and health monitoring
/// in a Rust async runtime environment.
fn main() -> RiResult<()> {
    println!("=== Ri Service Mesh Module Example ===\n");

    // Create async runtime for handling asynchronous service mesh operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute all async service mesh operations within the runtime
    rt.block_on(async {
        // Configuration Setup: Create service mesh configuration
        // Using builder pattern for configuration parameters:
        // - service_name: Name of this service instance for mesh identification
        // - namespace: Logical grouping of services (e.g., Kubernetes namespace)
        // - instance_id: Unique identifier for this service instance
        // - host: Network host where this service is running
        // - port: Network port where this service listens
        // - health_check_interval_secs: How often to perform health checks (30 seconds)
        // - failure_threshold: Consecutive failures before marking unhealthy (3)
        // - recovery_threshold: Consecutive successes before marking healthy (2)
        // - build(): Finalizes configuration into RiServiceMeshConfig struct
        let config = RiServiceMeshConfig::new()
            .with_service_name("ri-example")
            .with_namespace("default")
            .with_instance_id("instance-001")
            .with_host("localhost")
            .with_port(8080)
            .with_health_check_interval_secs(30)
            .with_failure_threshold(3)
            .with_recovery_threshold(2)
            .build();

        // Module Initialization: Create service mesh instance
        // The mesh provides service discovery, load balancing, and health management
        println!("1. Creating service mesh...");
        let service_mesh = RiServiceMesh::new(config).await?;
        println!("   Service mesh initialized\n");

        // Step 2: Register first service instance
        // Demonstrates service registration with full metadata
        // Services are registered to make them discoverable by other services
        println!("2. Registering service instance...");

        // Create service instance with connection and version information
        // RiServiceInstance::new() parameters:
        // - service_name: Logical name of the service (user-service)
        // - host: Where the service is running (localhost)
        // - port: Service port (8081)
        // - version: Service version for routing (v1.0.0)
        let instance = RiServiceInstance::new(
            "user-service".to_string(),
            "localhost".to_string(),
            8081,
            "v1.0.0".to_string(),
        );
        
        // Register the instance with the service mesh
        // Mesh will track this instance for discovery and health monitoring
        // Parameters:
        // - instance: RiServiceInstance containing service details
        service_mesh.register_instance(instance).await?;
        println!("   User service registered\n");

        // Step 3: Register additional service instance
        // Demonstrates registering multiple instances of same service
        // Multiple instances enable load balancing and high availability
        println!("3. Registering another instance...");
        
        let instance2 = RiServiceInstance::new(
            "user-service".to_string(),
            "localhost".to_string(),
            8082,
            "v1.0.0".to_string(),
        );
        service_mesh.register_instance(instance2).await?;
        println!("   Second user service instance registered\n");

        // Step 4: Register different service type
        // Demonstrates registering completely separate service
        // Different service types are tracked separately in the mesh
        println!("4. Registering order service...");
        
        let order_instance = RiServiceInstance::new(
            "order-service".to_string(),
            "localhost".to_string(),
            8083,
            "v1.0.0".to_string(),
        );
        service_mesh.register_instance(order_instance).await?;
        println!("   Order service registered\n");

        // Step 5: Service discovery
        // Demonstrates finding service instances by name
        // Discovery is fundamental to microservices communication
        // Other services use discovery to find where user-service is running
        println!("5. Discovering services...");
        
        // Query for all instances of user-service
        // Returns vector of available instances with their metadata
        // If no instances are available, returns empty vector
        let instances = service_mesh.discover_instances("user-service").await?;
        println!("   Found {} user-service instance(s):", instances.len());
        for instance in &instances {
            // Display instance connection details
            println!("   - {}:{} (status: {:?})",
                instance.host(),
                instance.port(),
                instance.status()
            );
        }
        println!();

        // Step 6: List all registered services
        // Demonstrates enumeration of all services in the mesh
        println!("6. Listing all registered services...");
        
        // Get list of unique service names in the mesh
        // Returns Vec<String> containing service names
        let services = service_mesh.list_services().await?;
        println!("   Registered services:");
        for service in &services {
            // For each service, discover its instances
            // and display instance count
            let instances = service_mesh.discover_instances(service).await?;
            println!("   - {}: {} instance(s)", service, instances.len());
        }
        println!();

        // Step 7: Update instance health
        // Demonstrates manual health status management
        // Health status affects load balancer routing decisions
        // Services marked as unhealthy won't receive traffic
        println!("7. Updating instance health...");
        
        // Get instances and update first one's health
        let instances = service_mesh.discover_instances("user-service").await?;
        if let Some(first_instance) = instances.first() {
            // Update health status to indicate instance state
            // Status can be: Healthy, Unhealthy, Unknown, etc.
            // This is useful for maintenance mode or temporary issues
            service_mesh.update_instance_health(
                first_instance.id(),
                RiServiceStatus::Healthy,
            ).await?;
            println!("   Updated instance {} to Healthy\n", first_instance.id());
        }

        // Step 8: Run health checks
        // Demonstrates automatic health monitoring
        // Health checks verify service instances are responding correctly
        println!("8. Simulating health check...");
        
        // Execute health checks on all registered instances
        // Returns comprehensive report with pass/fail status
        // Health checks probe each instance's health endpoint
        let health_report = service_mesh.run_health_checks().await?;
        println!("   Health check completed:");
        println!("   - Total checks: {}", health_report.total_checks());
        println!("   - Passed: {}", health_report.passed_checks());
        println!("   - Failed: {}\n", health_report.failed_checks());

        // Step 9: Get service mesh statistics
        // Demonstrates aggregate monitoring of mesh state
        println!("9. Getting service mesh statistics...");
        
        // Get aggregated metrics about registered instances
        // Statistics help understand mesh capacity and health
        let stats = service_mesh.get_statistics().await?;
        println!("   Service mesh statistics:");
        println!("   - Total instances: {}", stats.total_instances());
        println!("   - Healthy instances: {}", stats.healthy_instances());
        println!("   - Unhealthy instances: {}", stats.unhealthy_instances());
        println!();

        // Step 10: Deregister service
        // Demonstrates removing service from mesh
        // Cleanup is important for scaling down or decommissioning services
        println!("10. Deregistering service...");
        
        // Remove specific instance from mesh
        // This immediately stops discovery of this instance
        // Parameters:
        // - service_name: Name of the service to deregister from
        // - instance_id: ID of the specific instance to remove
        service_mesh.deregister_instance("user-service", "instance-001").await?;
        println!("   Deregistered user-service instance-001\n");

        // Step 11: Final discovery
        // Verify instance was successfully removed
        println!("11. Final service discovery...");
        
        let instances = service_mesh.discover_instances("user-service").await?;
        println!("   Remaining user-service instances: {}\n", instances.len());

        println!("=== Service Mesh Example Completed ===");
        Ok::<(), RiError>(())
    })?
}
