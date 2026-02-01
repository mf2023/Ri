#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
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
DMSC Comprehensive API Example.

This example demonstrates the complete DMSC API usage across all major modules,
providing a production-ready pattern for building enterprise applications.

Features Demonstrated:
- Application initialization and configuration
- Authentication and authorization with JWT
- Cache operations with multiple backends
- Message queue production and consumption
- Service mesh integration
- Observability and metrics
- Database operations
- Gateway and routing configuration

Usage:
    python comprehensive_example.py
"""

from dmsc import (
    DMSCAppBuilder, DMSCAppRuntime, DMSCServiceContext, DMSCResult,
    DMSCLogConfig, DMSCLogLevel,
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTManager, DMSCJWTClaims,
    DMSCCacheModule, DMSCCacheConfig, DMSCCachePolicy,
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager, DMSCQueueMessage,
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCObservabilityModule, DMSCObservabilityConfig, DMSCMetricsRegistry,
    DMSCGateway, DMSCGatewayConfig, DMSCRouter, DMSCRoute,
    DMSCDatabaseConfig, DMSCDatabasePool, DMSCDBRow,
)
import asyncio
from datetime import datetime, timedelta
from typing import Optional


async def demonstrate_application_initialization():
    """
    Demonstrate application initialization with all core components.
    
    This function shows how to:
    - Create an application builder
    - Configure logging and observability
    - Build and run the application
    - Access service context
    """
    print("=== Application Initialization ===\n")
    
    print("1. Creating application builder...")
    builder = DMSCAppBuilder()
    print("   Builder created successfully\n")
    
    print("2. Configuring logging...")
    log_config = DMSCLogConfig()
    log_config.set_level("info")
    print(f"   Log level set to: {log_config.get_level()}\n")
    
    print("3. Configuring observability...")
    obs_config = DMSCObservabilityConfig.default()
    obs_config.set_metrics_enabled(True)
    obs_config.set_tracing_enabled(True)
    print("   Observability configured\n")
    
    print("4. Building application...")
    app = builder.with_logging(log_config).with_observability(obs_config).build()
    print("   Application built successfully\n")
    
    print("5. Application initialization complete!")
    print(f"   Runtime info: {app.runtime_info()}\n")
    
    return app


async def demonstrate_authentication():
    """
    Demonstrate authentication and authorization functionality.
    
    This function shows how to:
    -    - Create and validate JWT tokens
 Configure authentication module
    - Handle authentication context
    """
    print("=== Authentication Module ===\n")
    
    print("1. Creating authentication configuration...")
    auth_config = DMSCAuthConfig.default()
    auth_config.jwt_secret = "your-secret-key-here"
    auth_config.jwt_expiry_secs = 24 * 3600  # 24 hours in seconds
    print("   Auth config created\n")
    
    print("2. Creating authentication module...")
    auth_module = DMSCAuthModule(auth_config)
    print("   Auth module created\n")
    
    print("3. Creating JWT token...")
    claims = DMSCJWTClaims(
        subject="user123",
        issuer="dmsc-app",
        audience="api",
        expires_at=datetime.utcnow() + timedelta(hours=24),
        issued_at=datetime.utcnow(),
    )
    claims.set_role("admin")
    claims.set_permission("read:users")
    claims.set_permission("write:users")
    
    token = auth_module.create_token(claims)
    print(f"   Token created: {token[:50]}...\n")
    
    print("4. Validating JWT token...")
    validation_result = auth_module.validate_token(token)
    if validation_result.is_valid():
        validated_claims = validation_result.claims()
        print(f"   Token valid for user: {validated_claims.subject()}")
        print(f"   Role: {validated_claims.role()}")
        print(f"   Permissions: {validated_claims.permissions()}\n")
    else:
        print(f"   Token validation failed: {validation_result.error()}\n")
    
    print("5. Authentication demonstration complete!\n")


async def demonstrate_caching():
    """
    Demonstrate cache operations with multiple backends.
    
    This function shows how to:
    - Configure cache with different backends
    - Perform cache operations (get, set, delete)
    - Implement cache policies (LRU, TTL)
    """
    print("=== Cache Module ===\n")
    
    print("1. Creating cache configuration...")
    cache_config = DMSCCacheConfig()
    cache_config.enabled = True
    cache_config.default_ttl_secs = 300
    cache_config.max_memory_mb = 1000
    print("   Cache config created (memory backend)\n")
    
    print("2. Creating cache module...")
    cache_module = DMSCCacheModule(cache_config)
    print("   Cache module created\n")
    
    print("3. Setting cache policy...")
    policy = DMSCCachePolicy()
    policy.max_size = Some(100)
    policy.ttl = Some(600)  # 600 seconds
    cache_module.set_policy("user_data", policy)
    print("   Cache policy set (LRU, 100 items, 600s TTL)\n")
    
    print("4. Performing cache operations...")
    cache_module.set("user:1:name", "Alice")
    cache_module.set("user:1:email", "alice@example.com", ttl=60)
    
    name = cache_module.get("user:1:name")
    email = cache_module.get("user:1:email")
    print(f"   Retrieved user:name: {name}")
    print(f"   Retrieved user:email: {email}\n")
    
    print("5. Cache demonstration complete!\n")


async def demonstrate_message_queue():
    """
    Demonstrate message queue operations.
    
    This function shows how to:
    - Configure message queue
    - Publish messages to queue
    - Consume messages from queue
    - Handle acknowledgments
    """
    print("=== Message Queue Module ===\n")
    
    print("1. Creating queue configuration...")
    queue_config = DMSCQueueConfig.redis(
        host="localhost",
        port=6379,
        password=None,
        db=0,
    )
    print("   Queue config created (Redis backend)\n")
    
    print("2. Creating queue module...")
    queue_module = DMSCQueueModule(queue_config)
    print("   Queue module created\n")
    
    print("3. Creating queue...")
    manager = queue_module.get_manager()
    await manager.create_queue("orders")
    print("   Queue 'orders' created\n")
    
    print("4. Publishing messages...")
    for i in range(1, 4):
        message = DMSCQueueMessage(
            id=f"order-{i}",
            payload={
                "order_id": i,
                "product": f"Product {i}",
                "quantity": i,
                "price": 29.99 * i,
            },
        )
        await manager.publish("orders", message)
        print(f"   Published order #{i}\n")
    
    print("5. Consuming messages...")
    for _ in range(3):
        msg = await manager.consume("orders")
        if msg:
            print(f"   Received: {msg.id()} - {msg.payload()}")
            await manager.ack("orders", msg.id())
            print("   Message acknowledged\n")
    
    print("6. Cleaning up...")
    await manager.delete_queue("orders", force=True)
    print("   Queue deleted\n")
    
    print("7. Message queue demonstration complete!\n")


async def demonstrate_service_mesh():
    """
    Demonstrate service mesh functionality.
    
    This function shows how to:
    - Configure service mesh
    - Register services and endpoints
    - Discover services
    - Health checking
    """
    print("=== Service Mesh Module ===\n")
    
    print("1. Creating service mesh configuration...")
    mesh_config = DMSCServiceMeshConfig()
    mesh_config.enable_service_discovery = True
    mesh_config.enable_health_check = True
    mesh_config.health_check_interval = 30
    print("   Mesh config created\n")
    
    print("2. Creating service mesh...")
    service_mesh = DMSCServiceMesh(mesh_config)
    print("   Service mesh created\n")
    
    print("3. Registering services...")
    await service_mesh.register_service(
        "user-service",
        "http://user-service:8080",
        100,
    )
    print("   Registered 'user-service'\n")

    await service_mesh.register_service(
        "order-service",
        "http://order-service:8080",
        80,
    )
    print("   Registered 'order-service'\n")
    
    print("4. Discovering services...")
    endpoints = await service_mesh.discover_service("user-service")
    print(f"   Found {len(endpoints)} endpoint(s) for 'user-service'\n")
    
    print("5. Getting service mesh stats...")
    stats = await service_mesh.get_stats()
    print(f"   Total services: {stats.total_services()}")
    print(f"   Total endpoints: {stats.total_endpoints()}\n")
    
    print("6. Service mesh demonstration complete!\n")


async def demonstrate_observability():
    """
    Demonstrate observability and metrics functionality.
    
    This function shows how to:
    - Configure observability
    - Record metrics
    - Create traces
    - Export to Prometheus
    """
    print("=== Observability Module ===\n")
    
    print("1. Creating observability configuration...")
    obs_config = DMSCObservabilityConfig()
    obs_config.metrics_enabled = True
    obs_config.tracing_enabled = True
    print("   Observability config created\n")
    
    print("2. Creating observability module...")
    obs_module = DMSCObservabilityModule(obs_config)
    print("   Observability module created\n")
    
    print("3. Observability demonstration complete!\n")


async def demonstrate_database():
    """
    Demonstrate database operations.
    
    This function shows how to:
    - Configure database connection
    - Execute queries
    - Handle transactions
    - Use ORM features
    """
    print("=== Database Module ===\n")
    
    print("1. Creating database configuration...")
    db_config = DMSCDatabaseConfig.create_postgres()
    db_config.host = "localhost"
    db_config.port = 5432
    db_config.database = "dmsc_db"
    db_config.username = "postgres"
    db_config.password = "password"
    db_config.max_connections = 10
    print("   Database config created (PostgreSQL)\n")
    
    print("2. Creating database pool...")
    pool = DMSCDatabasePool(db_config)
    print("   Database pool created\n")
    
    print("3. Executing query...")
    result = await pool.query("SELECT version()")
    if result.rows():
        row = result.rows()[0]
        print(f"   PostgreSQL version: {row.get(0)}\n")
    
    print("4. Database demonstration complete!\n")


async def demonstrate_gateway():
    """
    Demonstrate API gateway functionality.
    
    This function shows how to:
    - Configure gateway
    - Define routes
    - Configure rate limiting
    - Set up circuit breaking
    """
    print("=== Gateway Module ===\n")
    
    print("1. Creating gateway configuration...")
    gateway_config = DMSCGatewayConfig.default()
    gateway_config.set_port(8080)
    gateway_config.set_workers(4)
    print("   Gateway config created\n")
    
    print("2. Creating router...")
    router = DMSCRouter()
    
    print("3. Defining routes...")
    route1 = DMSCRoute(
        path="/api/users",
        method="GET",
        handler="user_handler",
        rate_limit=100,
    )
    router.add_route(route1)
    
    route2 = DMSCRoute(
        path="/api/orders",
        method="POST",
        handler="order_handler",
        rate_limit=50,
    )
    router.add_route(route2)
    print("   Routes defined\n")
    
    print("4. Creating gateway...")
    gateway = DMSCGateway(gateway_config, router)
    print("   Gateway created\n")
    
    print("5. Gateway demonstration complete!\n")


async def main():
    """
    Main entry point for comprehensive DMSC example.
    
    This function demonstrates the complete DMSC API usage across all major
    modules in a sequential manner, showing production-ready patterns for
    building enterprise applications.
    """
    print("=" * 60)
    print("DMSC Comprehensive API Example")
    print("=" * 60)
    print()
    
    try:
        await demonstrate_application_initialization()
        await demonstrate_authentication()
        await demonstrate_caching()
        await demonstrate_message_queue()
        await demonstrate_service_mesh()
        await demonstrate_observability()
        await demonstrate_database()
        await demonstrate_gateway()
        
        print("=" * 60)
        print("All demonstrations completed successfully!")
        print("=" * 60)
        print()
        print("For more examples, see:")
        print("  - examples/Python/*.py")
        print("  - doc/zh/05-usage-examples/")
        
    except Exception as e:
        print(f"Error during demonstration: {e}")
        print("Note: Some features require running services (Redis, PostgreSQL, etc.)")


if __name__ == "__main__":
    asyncio.run(main())
