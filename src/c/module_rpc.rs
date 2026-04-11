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

//! # Module RPC C API
//!
//! This module provides C language bindings for Ri's module RPC (Remote Procedure Call) system.
//! The module RPC system enables communication between different modules within the Ri framework,
//! providing a high-performance, type-safe mechanism for inter-module service calls. This C API
//! enables C/C++ modules to participate in Ri's distributed architecture by registering services,
//! making remote calls, and handling asynchronous responses.
//!
//! ## Module Architecture
//!
//! The module RPC system comprises three primary components:
//!
//! - **RiModuleRPC**: Central RPC router and dispatcher managing service registration, method
//!   routing, and call dispatch across modules. The router handles the complete lifecycle of
//!   RPC operations including request routing, response aggregation, and error handling.
//!
//! - **RiModuleClient**: Client interface for making RPC calls to registered services. The
//!   client provides synchronous and asynchronous call capabilities with automatic serialization,
//!   connection management, and retry logic.
//!
//! - **RiModuleEndpoint**: Connection endpoint for module communication, managing the transport
//!   layer, protocol negotiation, and message framing. Endpoints can be local (in-process) or
//!   remote (network-based).
//!
//! ## RPC Communication Model
//!
//! The module RPC system implements a request-response communication model:
//!
//! - **Service Registration**: Modules register services with method signatures, enabling
//!   discovery and invocation by other modules.
//!
//! - **Method Invocation**: Clients call registered methods with serialized parameters,
//!   receiving serialized results or errors.
//!
//! - **Request Routing**: The router dispatches requests to the appropriate service
//!   based on module and method names.
//!
//! - **Response Handling**: Results are serialized and returned to the calling client,
//!   with support for streaming responses.
//!
//! ## Module Discovery
//!
//! The RPC system provides automatic module discovery:
//!
//! - **Service Registry**: Central registry tracks all registered services and their
//!   availability status.
//!
//! - **Health Monitoring**: Automatic health checks detect unavailable services and
//!   remove them from the routing table.
//!
//! - **Load Balancing**: Requests distributed across multiple service instances when
//!   available.
//!
//! - **Service Dependencies**: Dependency graph tracks module relationships for
//!   proper initialization and shutdown sequencing.
//!
//! ## Serialization
//!
//! The RPC system supports multiple serialization formats:
//!
//! - **Protocol Buffers**: Default serialization with schema-based type safety.
//!   Provides efficient binary encoding with backward/forward compatibility.
//!
//! - **MessagePack**: Binary serialization format for compact payloads with
//!   schema-less flexibility.
//!
//! - **JSON**: Human-readable serialization for debugging and HTTP interoperability.
//!
//! - **Custom Codecs**: Extension point for application-specific serialization.
//!
//! ## Transport Mechanisms
//!
//! The module RPC supports multiple transport mechanisms:
//!
//! - **In-Process**: Zero-copy communication between modules in the same process.
//!   Fastest option for co-located modules.
//!
//! - **Shared Memory**: High-performance communication using shared memory segments.
//!   Suitable for high-throughput, low-latency scenarios.
//!
//! - **TCP/IP**: Network-based communication for distributed deployments.
//!   Supports TLS encryption and compression.
//!
//! - **Unix Domain Sockets**: Local socket communication with near-in-process
//!   performance. Available on Unix-like systems.
//!
//! ## Request Patterns
//!
//! The RPC system supports various request patterns:
//!
//! - **Unary Calls**: Simple request-response pattern with single request and response.
//!   Most common pattern for traditional RPC operations.
//!
//! - **Streaming Calls**: Bidirectional streaming for large data transfers or
//!   real-time communication patterns.
//!
//! - **Batch Calls**: Multiple independent requests batched into a single network
//!   round-trip for efficiency.
//!
//! - **Subscription Calls**: Long-polling or push-based subscriptions for event
//!   notification patterns.
//!
//! ## Error Handling
//!
//! The RPC system implements comprehensive error handling:
//!
//! - **Transport Errors**: Connection failures, timeouts, and protocol errors.
//!
//! - **Serialization Errors**: Invalid input data, schema mismatches, codec failures.
//!
//! - **Service Errors**: Application-level errors returned by the service handler.
//!
//! - **Routing Errors**: Service not found, method not found, invalid parameters.
//!
//! ## Performance Characteristics
//!
//! RPC operations have the following performance profiles:
//!
//! - In-process call latency: Near-zero, single function call overhead
//! - Serialization overhead: O(n) where n is message size
//! - Transport latency: O(1) for local, O(network) for remote
//! - Concurrent calls: Hundreds to thousands per endpoint
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Client stubs must be properly cleaned up after use
//! - Request/response buffers must be freed appropriately
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - RPC router handles concurrent requests from multiple threads
//! - Client stubs support concurrent method invocations
//! - Service handlers invoked concurrently for each request
//! - Endpoint management uses internal synchronization
//!
//! ## Usage Example
//!
//! ```c
//! // Create RPC router instance
//! RiModuleRPC* rpc = ri_module_rpc_new();
//! if (rpc == NULL) {
//!     fprintf(stderr, "Failed to create RPC router\n");
//!     return ERROR_INIT;
//! }
//!
//! // Register a service module
//! int result = ri_module_rpc_register_service(
//!     rpc,
//!     "UserService",
//!     user_service_handler,
//!     NULL  // user data passed to handler
//! );
//!
//! if (result != 0) {
//!     fprintf(stderr, "Failed to register service\n");
//! }
//!
//! // Create client for remote service
//! RiModuleClient* client = ri_module_client_new(rpc);
//! if (client == NULL) {
//!     fprintf(stderr, "Failed to create RPC client\n");
//!     ri_module_rpc_free(rpc);
//!     return ERROR_INIT;
//! }
//!
//! // Configure request
//! RiUserRequest request = RiUSER_REQUEST_INIT;
//! request.user_id = 12345;
//! request.include_profile = true;
//!
//! // Execute synchronous RPC call
//! RiUserResponse response = RiUSER_RESPONSE_INIT;
//! int status = ri_module_client_call(
//!     client,
//!     "UserService.GetUser",
//!     &request,
//!     &response,
//!     5000  // timeout in milliseconds
//! );
//!
//! if (status == 0) {
//!     printf("User: %s %s\n", response.first_name, response.last_name);
//! } else {
//!     const char* error = ri_module_client_last_error(client);
//!     fprintf(stderr, "RPC error: %s (code: %d)\n", error, status);
//! }
//!
//! // Cleanup
//! ri_module_client_free(client);
//! ri_module_rpc_free(rpc);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::module_rpc`: Rust module RPC implementation
//! - `crate::prelude`: Common types and traits
//! - Serialization framework (prost, serde, or custom)
//!
//! ## Feature Flags
//!
//! The module RPC module is enabled by the "module-rpc" feature flag.
//! Disable this feature to reduce binary size when module RPC is not required.
//!
//! Additional features:
//!
//! - module-rpc-tls: Enable TLS for network transport
//! - module-rpc-streaming: Enable streaming RPC calls
//! - module-rpc-metrics: Enable RPC metrics collection

use crate::module_rpc::{RiModuleClient, RiModuleEndpoint, RiModuleRPC};


c_wrapper!(CRiModuleRPC, RiModuleRPC);
c_wrapper!(CRiModuleClient, RiModuleClient);
c_wrapper!(CRiModuleEndpoint, RiModuleEndpoint);

// RiModuleRPC constructors and destructors
#[no_mangle]
pub extern "C" fn ri_module_rpc_new() -> *mut CRiModuleRPC {
    let rpc = RiModuleRPC::new();
    Box::into_raw(Box::new(CRiModuleRPC::new(rpc)))
}
c_destructor!(ri_module_rpc_free, CRiModuleRPC);
