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

//! # WebSocket Module C API
//!
//! This module provides C language bindings for Ri's WebSocket communication infrastructure. The WebSocket
//! module delivers full-duplex, real-time communication capabilities for building interactive applications,
//! live dashboards, chat systems, gaming backends, and streaming services. This C API enables C/C++
//! applications to leverage Ri's WebSocket functionality for building responsive, bidirectional communication
//! layers over persistent connections.
//!
//! ## Module Architecture
//!
//! The WebSocket module comprises three primary components that together provide complete WebSocket
//! communication capabilities:
//!
//! - **RiWSServerConfig**: Configuration container for WebSocket server parameters including connection
//!   limits, message size limits, heartbeat settings, and security options. The configuration object
//!   controls server behavior, resource allocation, and operational characteristics.
//!
//! - **RiWSSession**: Individual WebSocket connection representation, tracking connection state,
//!   session metadata, message queues, and communication properties. Each session represents a unique
//!   client connection to the WebSocket server.
//!
//! - **RiWSSessionManager**: Central manager for all WebSocket sessions, handling session lifecycle,
//!   broadcasting, routing, and connection administration. The manager handles the complete WebSocket
//!   communication workflow including connection handling, message distribution, and cleanup.
//!
//! ## WebSocket Protocol
//!
//! The WebSocket implementation provides complete RFC 6455 compliance:
//!
//! - **Handshake Handling**: Complete WebSocket handshake processing with HTTP upgrade mechanism.
//!   Validates request headers, verifies protocol version, and generates proper responses.
//!
//! - **Frame Processing**: Full frame lifecycle management including text frames, binary frames,
//!   continuation frames, close frames, ping frames, and pong frames. Handles fragmentation
//!   and reassembly transparently.
//!
//! - **Extensions Support**: Configurable WebSocket extensions including per-message deflate
//!   compression (RFC 7692) for reduced bandwidth usage. Support for custom extensions.
//!
//! - **Subprotocol Negotiation**: Support for WebSocket subprotocols for application-specific
//!   messaging semantics. Includes built-in support for common subprotocols.
//!
//! - **Status Codes**: Complete handling of WebSocket status codes including normal closure (1000),
//!   going away (1001), protocol error (1002), unsupported data (1003), and application codes.
//!
//! - **Reason Phrases**: Proper close frame reason phrases for debugging and logging. Includes
//!   support for custom application reason codes.
//!
//! ## Message Types
//!
//! The WebSocket module supports comprehensive message types:
//!
//! - **Text Messages**: UTF-8 encoded text messages with automatic validation. Supports
//!   JSON, XML, and other text-based protocols. Validates UTF-8 encoding compliance.
//!
//! - **Binary Messages**: Raw binary data transmission for efficiency. Ideal for protocol buffers,
//!   images, files, and other binary formats. No encoding overhead or validation.
//!
//! - **Continuation Messages**: Support for message fragmentation across multiple frames.
//!   Automatic reassembly of fragmented messages on receive, transparent fragmentation on send.
//!
//! - **Control Frames**: Ping and Pong frames for connection health monitoring. Close frames
//!   for graceful connection termination with configurable reason codes.
//!
//! - **Pong Payloads**: Custom payload support in Pong frames for application-specific
//!   heartbeat data. Includes timestamp and custom application data.
//!
//! ## Connection Management
//!
//! The session manager provides comprehensive connection handling:
//!
//! - **Connection Lifecycle**: Complete connection lifecycle from TCP handshake through WebSocket
//!   upgrade to graceful close. Handles all intermediate states and transitions.
//!
//! - **Keep-Alive Mechanisms**: Configurable keep-alive intervals with automatic Ping/Pong
//!   exchange. Detects half-open connections and zombie clients.
//!
//! - **Heartbeat System**: Application-level heartbeat for more sophisticated connection
//!   monitoring. Custom heartbeat intervals and timeout values per session.
//!
//! - **Graceful Shutdown**: Proper WebSocket close handshake with configurable timeout.
//!   Ensures all pending messages are sent before connection termination.
//!
//! - **Forced Disconnection**: Emergency disconnection capability for abuse prevention
//!   and resource cleanup. Includes configurable disconnect behavior and cleanup.
//!
//! - **Connection Pooling**: Resource pooling for high-connection-count scenarios.
//!   Reduces memory allocation overhead for large numbers of concurrent connections.
//!
//! ## Session Features
//!
//! Individual sessions provide rich functionality:
//!
//! - **Session Metadata**: Store and retrieve arbitrary session data including user context,
//!   authentication state, and application-specific information. Key-value storage per session.
//!
//! - **Remote Address**: Access client network information including IP address, port,
//!   and protocol (IPv4/IPv6). Includes proxy protocol header parsing if enabled.
//!
//! - **Connection Time**: Track when the session was established for session duration
//!   calculations and timeout management.
//!
//! - **Message Counters**: Statistics tracking including messages sent, messages received,
//!   bytes sent, bytes received. Useful for monitoring and rate limiting.
//!
//! - **Last Activity**: Timestamp of last received message for idle detection and
//!   timeout management. Updated automatically on message receipt.
//!
//! - **Request Information**: Access to original HTTP upgrade request headers. Includes
//!   cookies, origin, subprotocols, and custom headers.
//!
//! ## Broadcasting
//!
//! The session manager supports efficient message distribution:
//!
//! - **Broadcast to All**: Send message to all connected sessions with single call.
//!   Optimized for minimal overhead in multi-connection scenarios.
//!
//! - **Selective Broadcast**: Target specific sessions using filter criteria.
//!   Filter by session data, connection time, message counts, or custom predicates.
//!
//! - **Room/Channel System**: Group sessions into named rooms for targeted messaging.
//!   Supports dynamic room membership and room statistics.
//!
//! - **Topic-Based Routing**: Subscribe sessions to topics for pub/sub-style messaging.
//!   Efficient topic-based message routing with automatic topic management.
//!
//! - **Exclusion**: Broadcast to all except specific sessions. Useful for acknowledging
//!   messages back to sender while excluding them from broadcast.
//!
//! - **Batching**: Configurable batching for high-volume broadcasting. Reduces system
//!   calls and improves throughput for mass messaging.
//!
//! ## Security Features
//!
//! The WebSocket module implements comprehensive security measures:
//!
//! - **Origin Validation**: Verify incoming requests against allowed origin list.
//!   Prevents cross-site WebSocket hijacking (CSWSH) attacks.
//!
//! - **Subprotocol Validation**: Validate requested subprotocols against allowed list.
//!   Ensures only negotiated subprotocols are used.
//!
//! - **Request Validation**: Validate HTTP upgrade request headers for security.
//!   Rejects malformed or suspicious requests.
//!
//! - **Rate Limiting**: Per-session and global rate limiting for message frequency.
//!   Configurable limits with customizable behavior (drop, queue, reject).
//!
//! - **Message Size Limits**: Configurable maximum message sizes for text and binary.
//!   Protects against memory exhaustion from oversized messages.
//!
//! - **Connection Limits**: Maximum concurrent connections per server and per IP.
//!   Prevents resource exhaustion from connection flooding.
//!
//! - **TLS/SSL Support**: Complete TLS integration for WSS (WebSocket Secure) connections.
//!   Modern TLS versions with configurable cipher suites.
//!
//! - **Authentication Integration**: Built-in support for token-based authentication.
//!   WebSocket-specific authentication handshake extensions.
//!
//! ## Compression
//!
//! Built-in compression reduces bandwidth usage:
//!
//! - **Per-Message Deflate**: RFC 7692 compression for WebSocket messages. Transparent
//!   compression and decompression without application changes.
//!
//! - **Compression Level**: Configurable compression levels trading CPU for compression.
//!   Supports levels from fastest (no compression) to best compression.
//!
//! - **Context Takeover**: Optional context takeover for improved compression ratios.
//!   Memory trade-off for better compression efficiency.
//!
//! - **Server Window Bits**: Configurable server window size for compression.
//!   Tuning for specific bandwidth/CPU requirements.
//!
//! ## Performance Characteristics
//!
//! WebSocket operations are optimized for high throughput:
//!
//! - **Connection Handling**: O(1) for new connections with constant-time session creation
//! - **Message Sending**: O(1) to O(n) depending on broadcast targets
//! - **Message Receiving**: O(1) for frame processing, O(n) for large message reassembly
//! - **Broadcasting**: O(n) where n is number of target sessions
//! - **Throughput**: Supports millions of messages per second on modern hardware
//! - **Latency**: Sub-millisecond message processing for local connections
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Session managers coordinate session cleanup
//! - Message buffers are recycled for performance
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Concurrent connection handling from multiple threads supported
//! - Message sending can be done from any thread
//! - Session operations use internal synchronization
//! - Broadcasting coordinates access across sessions
//!
//! ## Usage Example
//!
//! ```c
//! // Create WebSocket server configuration
//! RiWSServerConfig* config = ri_ws_server_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create WebSocket config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure server settings
//! ri_ws_server_config_set_host(config, "0.0.0.0");
//! ri_ws_server_config_set_port(config, 8080);
//! ri_ws_server_config_set_max_connections(config, 10000);
//! ri_ws_server_config_set_max_message_size(config, 1024 * 1024);  // 1MB
//! ri_ws_server_config_set_ping_interval(config, 30000);  // 30 seconds
//! ri_ws_server_config_set_ping_timeout(config, 5000);  // 5 seconds
//!
//! // Enable compression
//! ri_ws_server_config_set_compression_enabled(config, true);
//! ri_ws_server_config_set_compression_level(config, 6);
//!
//! // Enable security features
//! ri_ws_server_config_set_origin_validation(config, true);
//! ri_ws_server_config_add_allowed_origin(config, "https://example.com");
//! ri_ws_server_config_set_rate_limit_enabled(config, true);
//! ri_ws_server_config_set_rate_limit_messages(config, 100);
//! ri_ws_server_config_set_rate_limit_window(config, 1000);  // 1 second
//!
//! // Create session manager
//! RiWSSessionManager* manager = ri_ws_session_manager_new(config);
//! if (manager == NULL) {
//!     fprintf(stderr, "Failed to create session manager\n");
//!     ri_ws_server_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Start WebSocket server
//! int result = ri_ws_session_manager_start(manager);
//! if (result != 0) {
//!     fprintf(stderr, "Failed to start WebSocket server: %d\n", result);
//!     ri_ws_session_manager_free(manager);
//!     ri_ws_server_config_free(config);
//!     return ERROR_START;
//! }
//!
//! printf("WebSocket server started on port %d\n",
//!        ri_ws_server_config_get_port(config));
//!
//! // Set up event callbacks
//! ri_ws_session_manager_set_on_connect(manager, on_connect_callback, NULL);
//! ri_ws_session_manager_set_on_message(manager, on_message_callback, NULL);
//! ri_ws_session_manager_set_on_close(manager, on_close_callback, NULL);
//! ri_ws_session_manager_set_on_error(manager, on_error_callback, NULL);
//!
//! // Application main loop - process events
//! while (running) {
//!     // Handle events with timeout
//!     ri_ws_session_manager_poll(manager, 1000);  // 1 second timeout
//!
//!     // Process pending operations
//!     process_application_tasks();
//!
//!     // Periodic tasks
//!     if (should_check_health()) {
//!         uint32_t active_count = ri_ws_session_manager_get_active_count(manager);
//!         uint32_t total_count = ri_ws_session_manager_get_total_count(manager);
//!         uint64_t total_messages = ri_ws_session_manager_get_total_messages(manager);
//!
//!         printf("Active sessions: %u/%u, Messages: %lu\n",
//!                active_count, total_count, total_messages);
//!     }
//! }
//!
//! // Broadcast message to all clients
//! const char* broadcast_msg = "{\"type\": \"broadcast\", \"message\": \"Hello all!\"}";
//! int send_count = ri_ws_session_manager_broadcast(manager, broadcast_msg, strlen(broadcast_msg));
//! printf("Broadcast sent to %d clients\n", send_count);
//!
//! // Get session information
//! uint32_t session_count = ri_ws_session_manager_get_session_ids(manager, session_ids, max_sessions);
//!
//! for (uint32_t i = 0; i < session_count && i < 10; i++) {
//!     RiWSSession* session = ri_ws_session_manager_get_session(manager, session_ids[i]);
//!     if (session != NULL) {
//!         const char* remote_addr = ri_ws_session_get_remote_addr(session);
//!         uint64_t connected_at = ri_ws_session_get_connected_at(session);
//!         uint64_t messages_sent = ri_ws_session_get_messages_sent(session);
//!         uint64_t bytes_sent = ri_ws_session_get_bytes_sent(session);
//!
//!         printf("Session %u: %s, connected: %lu, sent: %lu/%lu\n",
//!                session_ids[i], remote_addr, connected_at, messages_sent, bytes_sent);
//!
//!         // Send message to specific session
//!         ri_ws_session_send(session, "Welcome!", 8);
//!
//!         ri_ws_session_free(session);
//!     }
//! }
//!
//! // Room management example
//! const char* room_name = "chat_room_1";
//! ri_ws_session_manager_join_room(manager, session_ids[0], room_name);
//!
//! // Send to room members only
//! const char* room_msg = "{\"type\": \"room\", \"content\": \"Hello room!\"}";
//! int room_count = ri_ws_session_manager_send_to_room(manager, room_name, room_msg, strlen(room_msg));
//! printf("Room message sent to %d clients\n", room_count);
//!
//! // Send to room except sender
//! ri_ws_session_manager_send_to_room_except(
//!     manager, room_name, session_ids[0],
//!     room_msg, strlen(room_msg)
//! );
//!
//! // Leave room
//! ri_ws_session_manager_leave_room(manager, session_ids[0], room_name);
//!
//! // Get room members
//! uint32_t room_members[100];
//! uint32_t room_count = ri_ws_session_manager_get_room_members(
//!     manager, room_name, room_members, 100
//! );
//! printf("Room '%s' has %u members\n", room_name, room_count);
//!
//! // Graceful shutdown
//! printf("Shutting down WebSocket server...\n");
//! ri_ws_session_manager_stop(manager, 5000);  // 5 second timeout
//! ri_ws_session_manager_free(manager);
//! ri_ws_server_config_free(config);
//!
//! printf("WebSocket server shutdown complete\n");
//! ```
//!
//! ## Event Callback Signatures
//!
//! Event handlers must conform to the following signatures:
//!
//! ```c
//! // Connection established
//! typedef void (*DMSWSConnectCallback)(
//!     RiWSSessionManager* manager,
//!     RiWSSession* session,
//!     void* user_data
//! );
//!
//! // Message received
//! typedef void (*DMSWSMessageCallback)(
//!     RiWSSessionManager* manager,
//!     RiWSSession* session,
//!     DMSWSMessageType type,
//!     const char* data,
//!     size_t length,
//!     void* user_data
//! );
//!
//! // Connection closed
//! typedef void (*DMSWSCloseCallback)(
//!     RiWSSessionManager* manager,
//!     RiWSSession* session,
//!     uint16_t code,
//!     const char* reason,
//!     void* user_data
//! );
//!
//! // Error occurred
//! typedef void (*DMSWSErrorCallback)(
//!     RiWSSessionManager* manager,
//!     RiWSSession* session,
//!     int error_code,
//!     const char* error_message,
//!     void* user_data
//! );
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::ws`: Rust WebSocket module implementation
//! - `crate::prelude`: Common types and traits
//! - tokio for async runtime (when async features enabled)
//! - tungstenite for WebSocket protocol (when using pure Rust implementation)
//!
//! ## Feature Flags
//!
//! The WebSocket module is enabled by the "ws" feature flag.
//! Disable this feature to reduce binary size when WebSocket is not required.
//!
//! Additional features:
//!
//! - `ws-tungstenite`: Enable Tungstenite-based WebSocket implementation
//! - `ws-async`: Enable async message handling
//! - `ws-compression`: Enable per-message deflate compression
//! - `ws-tls`: Enable TLS/WSS support
//! - `ws-rate-limit`: Enable rate limiting

use crate::ws::{RiWSServerConfig, RiWSSession, RiWSSessionManager};


c_wrapper!(CRiWSServerConfig, RiWSServerConfig);
c_wrapper!(CRiWSSession, RiWSSession);
c_wrapper!(CRiWSSessionManager, RiWSSessionManager);

// RiWSServerConfig constructors and destructors
c_constructor!(
    ri_ws_server_config_new,
    CRiWSServerConfig,
    RiWSServerConfig,
    RiWSServerConfig::default()
);
c_destructor!(ri_ws_server_config_free, CRiWSServerConfig);
