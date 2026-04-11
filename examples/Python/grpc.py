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
Ri gRPC Module Example

This example demonstrates how to use the Ri gRPC module for gRPC server
and client operations with service registry.
"""

import asyncio
from ri import (
    RiGrpcConfig,
    RiGrpcStats,
    RiGrpcServiceRegistry,
    RiGrpcPythonService,
    RiGrpcServiceRegistryPy,
    RiGrpcServer,
    RiGrpcClient,
)


async def main():
    # Create gRPC configuration
    config = RiGrpcConfig()
    config.host = "0.0.0.0"
    config.port = 50051
    config.max_concurrent_streams = 100
    config.keepalive_time_seconds = 60
    config.keepalive_timeout_seconds = 20
    config.enable_reflection = True

    # Create gRPC server
    print("Creating gRPC server...")
    server = RiGrpcServer(config)

    # Create service registry
    registry = RiGrpcServiceRegistryPy()

    # Define a Python gRPC service
    print("\nDefining gRPC services...")

    user_service = RiGrpcPythonService()
    user_service.service_name = "UserService"
    user_service.methods = ["GetUser", "CreateUser", "UpdateUser", "DeleteUser"]
    user_service.proto_file = "user.proto"

    order_service = RiGrpcPythonService()
    order_service.service_name = "OrderService"
    order_service.methods = ["GetOrder", "CreateOrder", "ListOrders"]
    order_service.proto_file = "order.proto"

    # Register services
    registry.register_service(user_service)
    registry.register_service(order_service)

    print(f"Registered {len(registry.list_services())} services")

    # List all services
    print("\nRegistered services:")
    for service_name in registry.list_services():
        print(f"  - {service_name}")

    # Get service info
    service_info = registry.get_service("UserService")
    if service_info:
        print(f"\nUserService methods: {service_info.methods}")

    # Create gRPC client
    print("\nCreating gRPC client...")
    client_config = RiGrpcConfig()
    client_config.host = "localhost"
    client_config.port = 50051
    client_config.timeout_seconds = 10

    client = RiGrpcClient(client_config)

    # Get gRPC statistics
    print("\ngRPC Statistics:")
    stats = RiGrpcStats()
    print(f"Total requests: {stats.total_requests}")
    print(f"Active connections: {stats.active_connections}")
    print(f"Average latency: {stats.average_latency_ms}ms")

    # Simulate service calls
    print("\nSimulating service calls...")

    # Unary call
    print("Unary call: GetUser")
    request_data = b'{"user_id": "123"}'
    print(f"Request: {request_data}")

    # Streaming call
    print("\nStreaming call: ListOrders")
    stream_request = b'{"user_id": "123", "page": 1, "page_size": 10}'
    print(f"Stream request: {stream_request}")

    # Bidirectional streaming
    print("\nBidirectional streaming: Chat")
    chat_messages = [
        b'{"message": "Hello"}',
        b'{"message": "How are you?"}',
        b'{"message": "Goodbye"}',
    ]
    print(f"Chat messages: {len(chat_messages)}")

    print("\ngRPC operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
