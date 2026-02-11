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

//! # gRPC Module C API
//!
//! This module provides C language bindings for DMSC's gRPC communication layer. The gRPC module
//! delivers high-performance Remote Procedure Call (RPC) capabilities using Protocol Buffers for
//! efficient serialization and HTTP/2 for transport. This C API enables C/C++ applications to
//! leverage DMSC's gRPC functionality for building distributed systems with strongly-typed service
//! contracts and bidirectional streaming support.
//!
//! ## Module Architecture
//!
//! The gRPC module comprises three primary components that together provide complete gRPC
//! functionality:
//!
//! - **DMSCGrpcServer**: gRPC server implementation handling service registration, request
//!   processing, and response streaming. The server manages the complete lifecycle of gRPC
//!   services including method dispatch, interceptor chaining, and connection management.
//!
//! - **DMSCGrpcClient**: gRPC client implementation for invoking remote procedures with automatic
//!   serialization, connection pooling, and retry logic. The client supports all gRPC calling
//!   patterns including unary calls, server streaming, client streaming, and bidirectional
//!   streaming.
//!
//! - **DMSCGrpcChannel**: Low-level channel abstraction managing the underlying HTTP/2 connection
//!   pool, transport security, and protocol negotiation. Channels provide the foundation for
//!   client communication and can be shared across multiple client instances.
//!
//! ## gRPC Communication Patterns
//!
//! The module supports all four standard gRPC communication patterns:
//!
//! - **Unary Calls**: Simple request-response pattern where client sends a single request and
//!   receives a single response. The most common pattern for traditional RPC operations.
//!
//! - **Server Streaming**: Client sends a single request and receives a stream of responses.
//!   Useful for scenarios like streaming updates, real-time notifications, or large dataset
//!   retrieval.
//!
//! - **Client Streaming**: Client sends a stream of requests and receives a single response.
//!   Useful for scenarios like batch uploads, upload with aggregation, or time-series data
//!   collection.
//!
//! - **Bidirectional Streaming**: Both client and server send streams of messages independently.
//!   Each side can send messages in any order. Useful for real-time communication, chat
//!   systems, or interactive data processing.
//!
//! ## Protocol Buffer Integration
//!
//! The gRPC module integrates tightly with Protocol Buffers:
//!
//! - **Message Serialization**: Automatic encoding and decoding of Protocol Buffer messages
//!   using the generated code from .proto files.
//!
//! - **Service Definition**: Service interfaces generated from .proto files define the
//!   available remote procedures and message types.
//!
//! - **Schema Evolution**: Support for backward and forward compatible schema changes
//!   through Protocol Buffers' field rules and unknown field handling.
//!
//! - **Custom Serialization**: Extension points for using alternative serialization formats
//!   like JSON, MessagePack, or custom binary formats.
//!
//! ## Transport Features
//!
//! The gRPC transport implements comprehensive HTTP/2 features:
//!
//! - **Connection Multiplexing**: Multiple requests and streams share a single TCP connection,
//!   reducing connection overhead and improving throughput.
//!
//! - **Flow Control**: Automatic flow control prevents fast senders from overwhelming slow
//!   receivers using HTTP/2 window updates.
//!
//! - **Header Compression**: HPACK header compression reduces bandwidth usage for repeated
//!   metadata in requests and responses.
//!
//! - **Stream Prioritization**: Clients can prioritize streams to ensure important requests
//!   get timely responses when connection capacity is limited.
//!
//! - **Ping/Pong Frames**: Keepalive mechanism detects dead connections and enables timely
//!   cleanup of stale resources.
//!
//! ## Security Features
//!
//! The gRPC module provides comprehensive security capabilities:
//!
//! - **TLS Encryption**: Full TLS encryption for all communication with configurable
//!   cipher suites and certificate validation.
//!
//! - **Certificate Management**: Support for PEM certificates, certificate chains, and
//!   custom certificate authorities.
//!
//! - **Client Authentication**: Multiple authentication mechanisms including:
//!   - JWT token authentication
//!   - OAuth 2.0 token exchange
//!   - mTLS (mutual TLS) with client certificates
//!   - Custom authentication interceptor
//!
//! - **Authorization**: Fine-grained authorization using interceptors for checking
//!   permissions before method execution.
//!
//! ## Connection Management
//!
//! The module implements sophisticated connection management:
//!
//! - **Connection Pooling**: Reuse of established connections to reduce latency and
//!   resource consumption across multiple requests.
//!
//! - **Load Balancing**: Client-side load distribution across multiple server instances
//!   with configurable policies (round-robin, pick-first, weighted).
//!
//! - **Health Checking**: Active health checks detect unhealthy server instances and
//!   remove them from the load balancer rotation.
//!
//! - **Retry Logic**: Automatic retry for idempotent requests with configurable retry
//!   policies, backoff strategies, and retry limits.
//!
//! - **Deadline Propagation**: Automatic deadline propagation across service boundaries
//!   ensures requests don't wait indefinitely for responses.
//!
//! ## Interceptors
//!
//! Interceptors provide extensibility for cross-cutting concerns:
//!
//! - **Server Interceptors**: Process incoming requests before they reach the handler
//!   and transform responses before they return to the client. Common uses include:
//!   - Authentication and authorization
//!   - Request/response logging
//!   - Metrics collection
//!   - Request validation
//!   - Response transformation
//!
//! - **Client Interceptors**: Process outgoing requests before they are sent and
//!   transform incoming responses before they reach the application. Common uses include:
//!   - Authentication token injection
//!   - Request tracing headers
//!   - Metrics collection
//!   - Retry handling
//!   - Response validation
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Client stubs must be properly shutdown before freeing
//! - Stream objects must be properly closed when complete
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Channels can be shared across threads for concurrent requests
//! - Client stubs support concurrent method invocations
//! - Server handlers are invoked concurrently for each request
//! - Interceptors should be stateless for thread safety
//!
//! ## Performance Characteristics
//!
//! gRPC operations have the following performance profiles:
//!
//! - Connection establishment: O(1) for pooled connections, O(1 TCP handshake + TLS)
//! - Unary call latency: O(message_size) for serialization, O(1) network
//! - Streaming throughput: O(message_size) per message with flow control
//! - Concurrent streams: Hundreds to thousands per connection
//!
//! ## Usage Example
//!
//! ```c
//! // Create gRPC channel with connection pooling
//! DMSCGrpcChannel* channel = dmsc_grpc_channel_new("localhost", 50051);
//! dmsc_grpc_channel_set_tls_enabled(channel, true);
//! dmsc_grpc_channel_set_connection_pool_size(channel, 10);
//!
//! // Create client stub
//! DMSCGrpcClient* client = dmsc_grpc_client_new(channel);
//!
//! // Configure request
//! DMSCUserRequest request = DMSCGUSER_REQUEST_INIT;
//! request.user_id = 12345;
//! request.include_profile = true;
//!
//! // Execute unary call
//! DMSCUserResponse response = DMSCGUSER_RESPONSE_INIT;
//! int status = dmsc_grpc_client_unary_call(
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
//!     const char* error = dmsc_grpc_client_last_error(client);
//!     fprintf(stderr, "gRPC error: %s (code: %d)\n", error, status);
//! }
//!
//! // Create stream for server streaming
//! DMSCNotificationRequest stream_request = DMSCGNOTIFICATION_REQUEST_INIT;
//! stream_request.notification_type = NOTIFICATION_TYPE_ALL;
//! stream_request.start_time = time(NULL);
//!
//! DMSCGrpcStream* stream = dmsc_grpc_client_server_stream(
//!     client,
//!     "NotificationService.Subscribe",
//!     &stream_request
//! );
//!
//! // Read streaming responses
//! DMSCNotification notification;
//! while (dmsc_grpc_stream_read(stream, &notification) == 0) {
//!     printf("Notification: %s\n", notification.message);
//!     dmsc_grpc_notification_destroy(&notification);
//! }
//!
//! dmsc_grpc_stream_free(stream);
//!
//! // Cleanup
//! dmsc_grpc_client_free(client);
//! dmsc_grpc_channel_free(channel);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components and external libraries:
//!
//! - `crate::grpc`: Rust gRPC module implementation
//! - `crate::prelude`: Common types and traits
//! - tonic for gRPC protocol implementation
//! - prost for Protocol Buffer encoding/decoding
//! - tokio for asynchronous runtime
//! - h2 for HTTP/2 transport
//!
//! ## Feature Flags
//!
//! The gRPC module is enabled by the "grpc" feature flag:
//!
//! - grpc: Enable gRPC server and client functionality
//! - grpc-tls: Enable TLS support for gRPC (requires native-tls or rustls)
//!
//! Disable these features to reduce binary size when gRPC is not required.

use crate::grpc::{DMSCGrpcClient, DMSCGrpcServer};


c_wrapper!(CDMSCGrpcServer, DMSCGrpcServer);

c_wrapper!(CDMSCGrpcClient, DMSCGrpcClient);


