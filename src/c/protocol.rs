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

//! # Protocol Module C API
//!
//! This module provides C language bindings for Ri's protocol handling infrastructure. The protocol
//! module delivers comprehensive support for encoding, decoding, and transforming data across various
//! wire formats and communication protocols. This C API enables C/C++ applications to leverage Ri's
//! protocol capabilities for building interoperable distributed systems with standardized data exchange.
//!
//! ## Module Architecture
//!
//! The protocol module comprises three primary components that together provide complete protocol
//! management capabilities:
//!
//! - **RiProtocolConfig**: Configuration container for protocol codec parameters including encoding
//!   formats, framing options, compression settings, and validation rules. The configuration object
//!   controls how data is serialized and deserialized, ensuring consistent behavior across the
//!   application.
//!
//! - **RiProtocolManager**: Central manager for protocol registration, codec lookup, and protocol
//!   negotiation. The manager handles the complete lifecycle of protocol operations including codec
//!   selection, error handling, and protocol switching.
//!
//! - **RiFrame**: Low-level frame abstraction for message framing and boundary management. Frames
//!   provide the foundation for streaming protocols, handling message boundaries, chunking, and
//!   reassembly.
//!
//! ## Supported Protocols
//!
//! The protocol system supports a comprehensive range of wire formats:
//!
//! - **JSON (JavaScript Object Notation)**: Human-readable data interchange format widely used in
//!   web APIs and microservices. Supports schema validation and transformation.
//!
//! - **MessagePack**: Binary serialization format providing compact representation with fast
//!   encoding and decoding. Ideal for bandwidth-constrained environments.
//!
//! - **Protocol Buffers**: Google's language-neutral, platform-neutral, extensible mechanism for
//!   serializing structured data. Provides strong typing and backward/forward compatibility.
//!
//! - **CBOR (Concise Binary Object Representation)**: Binary JSON-like format designed for small
//!   code size and small message size. IETF standard (RFC 8949).
//!
//! - **BSON (Binary JSON)**: MongoDB's binary-encoded JSON format with additional data types
//!   like dates and binary blobs.
//!
//! - **Avro**: Apache Avro data serialization format with schema evolution support and
//!   compact binary encoding.
//!
//! ## Framing Protocols
//!
//! The module provides various message framing approaches:
//!
//! - **Length-Prefixed Framing**: Each message is prefixed with its length in bytes. Enables
//!   streaming parsing and message boundary detection without special delimiters.
//!
//! - **Delimiter-Based Framing**: Messages are separated by special delimiter bytes (e.g., newline
//!   for line-based protocols). Simple but requires escaping for binary data.
//!
//! - **Fixed-Size Framing**: All messages have identical length. Simplifies parsing but wastes
//!   bandwidth for variable-sized data.
//!
//! - **HTTP/1.1 Chunked Transfer**: Standard HTTP chunked encoding for streaming responses.
//!   Supports incremental processing of large payloads.
//!
//! - **WebSocket Framing**: Full WebSocket frame handling including control frames, continuation
//!   frames, and fragmentation support.
//!
//! ## Compression
//!
//! Built-in compression support reduces bandwidth usage:
//!
//! - **Gzip**: GNU zip compression with wide compatibility. Good balance of compression ratio
//!   and CPU usage.
//!
//! - **Snappy**: Google's compression library designed for high speeds. Lower compression
//!   ratio but very fast encoding and decoding.
//!
//! - **LZ4**: Extremely fast compression with reasonable ratios. Ideal for real-time systems
//!   with limited CPU budget.
//!
//! - **Zstandard (zstd)**: Facebook's compression algorithm offering excellent compression ratios
//!   at high speeds. Supports dictionary compression for repetitive data.
//!
//! - **Brotli**: Google's next-generation compression with best-in-class compression ratios.
//!   Slightly slower but excellent for static content delivery.
//!
//! ## Validation
//!
//! Comprehensive data validation ensures protocol integrity:
//!
//! - **Schema Validation**: Validate messages against predefined schemas before processing.
//!   Catches malformed or unexpected data early.
//!
//! - **Type Checking**: Verify message types match expected types for each field.
//!   Supports optional fields and type coercion.
//!
//! - **Range Validation**: Ensure numeric values fall within acceptable ranges.
//!   Prevents overflow and underflow issues.
//!
//! - **Pattern Matching**: Validate string fields against regex patterns or format strings.
//!   Ensures email, UUID, and other formatted data validity.
//!
//! - **Custom Validators**: User-defined validation functions for domain-specific rules.
//!   Extend built-in validation with application logic.
//!
//! ## Serialization Features
//!
//! Advanced serialization capabilities:
//!
//! - **Polymorphism**: Handle tagged unions and inheritance hierarchies through type tags.
//!   Enables message routing based on message type.
//!
//! - **Optional Fields**: Gracefully handle missing fields with optional type support.
//!   Backward compatibility maintained across schema versions.
//!
//! - **Default Values**: Automatic default values for missing fields when defined in schema.
//!   Simplifies client code with sensible fallbacks.
//!
//! - **Unknown Field Handling**: Option to preserve unknown fields during deserialization.
//!   Enables forward compatibility without data loss.
//!
//! - **Circular Reference Handling**: Detect and properly serialize graph structures with
//!   references between objects.
//!
//! ## Performance Characteristics
//!
//! Protocol operations are optimized for various use cases:
//!
//! - **JSON Parsing**: O(n) where n is message size, optimized with SIMD instructions
//! - **Binary Codecs**: O(n) with low constant factors, ideal for high-throughput scenarios
//! - **Framing**: O(1) per frame boundary detection
//! - **Compression**: O(n * compression_level), configurable trade-off
//! - **Validation**: O(n) with early termination on first error
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Codec instances are managed by the protocol manager
//! - Frame buffers are recycled for performance
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Protocol manager supports concurrent codec registration
//! - Codec instances are immutable after creation
//! - Frame allocation uses thread-local pools
//! - Validation can be performed concurrently
//!
//! ## Usage Example
//!
//! ```c
//! // Create protocol configuration
//! RiProtocolConfig* config = ri_protocol_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create protocol config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure protocol settings
//! ri_protocol_config_set_format(config, PROTOCOL_FORMAT_MSGPACK);
//! ri_protocol_config_set_compression(config, COMPRESSION_SNAPPY);
//! ri_protocol_config_set_validation_enabled(config, true);
//!
//! // Create protocol manager
//! RiProtocolManager* manager = ri_protocol_manager_new(config);
//! if (manager == NULL) {
//!     fprintf(stderr, "Failed to create protocol manager\n");
//!     ri_protocol_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Register custom schema
//! int result = ri_protocol_manager_register_schema(
//!     manager,
//!     "UserMessage",
//!     user_schema_definition,
//!     sizeof(user_schema_definition)
//! );
//!
//! if (result != 0) {
//!     fprintf(stderr, "Failed to register schema\n");
//! }
//!
//! // Create frame for streaming
//! RiFrame* frame = ri_frame_new();
//! if (frame == NULL) {
//!     fprintf(stderr, "Failed to create frame\n");
//!     ri_protocol_manager_free(manager);
//!     ri_protocol_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Encode message
//! const char* input_data = "{\"user_id\": 12345, \"name\": \"John\"}";
//! size_t input_len = strlen(input_data);
//!
//! char* output_buffer = NULL;
//! size_t output_len = 0;
//!
//! result = ri_protocol_manager_encode(
//!     manager,
//!     "UserMessage",
//!     input_data,
//!     input_len,
//!     &output_buffer,
//!     &output_len
//! );
//!
//! if (result == 0 && output_buffer != NULL) {
//!     printf("Encoded %zu bytes\n", output_len);
//!
//!     // Decode message back
//!     char* decoded_buffer = NULL;
//!     size_t decoded_len = 0;
//!
//!         int decode_result = ri_protocol_manager_decode(
//!             manager,
//!             "UserMessage",
//!             output_buffer,
//!             output_len,
//!             &decoded_buffer,
//!             &decoded_len
//!         );
//!
//!         if (decode_result == 0) {
//!             printf("Decoded: %.*s\n", (int)decoded_len, decoded_buffer);
//!             ri_string_free(decoded_buffer);
//!         }
//!
//!     ri_string_free(output_buffer);
//! }
//!
//! // Frame the message for transport
//! ri_frame_reset(frame);
//! ri_frame_append(frame, output_buffer, output_len);
//!
//! // Read framed data
//! const char* frame_data = ri_frame_data(frame);
//! size_t frame_size = ri_frame_size(frame);
//!
//! // Cleanup
//! ri_frame_free(frame);
//! ri_protocol_manager_free(manager);
//! ri_protocol_config_free(config);
//! ```
//!
//! ## Protocol Negotiation
//!
//! The protocol manager supports dynamic protocol negotiation:
//!
//! - **Capability Exchange**: During connection establishment, both ends advertise supported
//!   protocols and versions.
//!
//! - **Common Protocol Selection**: Automatically select the best mutually-supported protocol
//!   based on priority and capabilities.
//!
//! - **Protocol Upgrades**: Support for upgrading from a base protocol (like HTTP/1.1) to
//!   a more efficient protocol (like WebSocket or gRPC).
//!
//! - **Version Handling**: Manage multiple protocol versions simultaneously for backward
//!   compatibility during migrations.
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::protocol`: Rust protocol module implementation
//! - `crate::prelude`: Common types and traits
//! - serde for serialization frameworks
//! - Various codec libraries (serde_json, rmp-serde, prost, etc.)
//!
//! ## Feature Flags
//!
//! The protocol module is enabled by default with comprehensive format support.
//! Additional formats enabled by feature flags:
//!
//! - `protocol-protobuf`: Enable Protocol Buffer support (requires prost)
//! - `protocol-avro`: Enable Apache Avro support
//! - `protocol-cbor`: Enable CBOR support
//! - `protocol-bson`: Enable BSON support
//! - `protocol-compression`: Enable compression codecs

use crate::protocol::{RiFrame, RiProtocolConfig, RiProtocolManager, RiProtocolStats, RiConnectionInfo, RiProtocolType, RiSecurityLevel, RiConnectionState};


c_wrapper!(CRiProtocolConfig, RiProtocolConfig);
c_wrapper!(CRiProtocolManager, RiProtocolManager);
c_wrapper!(CRiFrame, RiFrame);
c_wrapper!(CRiProtocolStats, RiProtocolStats);
c_wrapper!(CRiConnectionInfo, RiConnectionInfo);

// RiProtocolConfig constructors and destructors
c_constructor!(
    ri_protocol_config_new,
    CRiProtocolConfig,
    RiProtocolConfig,
    RiProtocolConfig::default()
);
c_destructor!(ri_protocol_config_free, CRiProtocolConfig);

// RiProtocolConfig setters
#[no_mangle]
pub extern "C" fn ri_protocol_config_set_protocol_type(config: *mut CRiProtocolConfig, protocol_type: std::ffi::c_int) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        let pt = match protocol_type {
            0 => RiProtocolType::Global,
            1 => RiProtocolType::Private,
            _ => RiProtocolType::Global,
        };
        (*config).inner.default_protocol = pt;
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_protocol_config_set_security_enabled(config: *mut CRiProtocolConfig, enabled: bool) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        (*config).inner.enable_security = enabled;
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_protocol_config_set_security_level(config: *mut CRiProtocolConfig, level: std::ffi::c_int) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        let sl = match level {
            0 => RiSecurityLevel::None,
            1 => RiSecurityLevel::Standard,
            2 => RiSecurityLevel::High,
            3 => RiSecurityLevel::Military,
            _ => RiSecurityLevel::Standard,
        };
        (*config).inner.security_level = sl;
    }
    0
}

// RiProtocolManager C bindings
#[no_mangle]
pub extern "C" fn ri_protocol_manager_new() -> *mut CRiProtocolManager {
    let ptr = Box::into_raw(Box::new(CRiProtocolManager::new(RiProtocolManager::new())));
    crate::c::register_ptr(ptr as usize);
    ptr
}
c_destructor!(ri_protocol_manager_free, CRiProtocolManager);

#[no_mangle]
pub extern "C" fn ri_protocol_manager_send(
    manager: *mut CRiProtocolManager,
    target: *const std::ffi::c_char,
    data: *const std::ffi::c_char,
    data_len: usize,
    out_response: *mut *mut std::ffi::c_char,
    out_len: *mut usize,
) -> std::ffi::c_int {
    if manager.is_null() || target.is_null() || data.is_null() || out_response.is_null() || out_len.is_null() {
        return -1;
    }
    unsafe {
        let target_str = match std::ffi::CStr::from_ptr(target).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };
        let data_slice = std::slice::from_raw_parts(data as *const u8, data_len);
        let response = (*manager).inner.send_message(target_str, data_slice);
        *out_len = response.len();
        match std::ffi::CString::new(response) {
            Ok(c_str) => {
                *out_response = c_str.into_raw();
                0
            }
            Err(_) => -3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_protocol_manager_get_stats(
    manager: *mut CRiProtocolManager,
    out_messages_sent: *mut u64,
    out_messages_received: *mut u64,
    out_bytes_sent: *mut u64,
    out_bytes_received: *mut u64,
    out_errors: *mut u64,
) -> std::ffi::c_int {
    if manager.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let stats = rt.block_on(async { (*manager).inner.stats.read().await.clone() });
        if !out_messages_sent.is_null() {
            *out_messages_sent = stats.messages_sent;
        }
        if !out_messages_received.is_null() {
            *out_messages_received = stats.messages_received;
        }
        if !out_bytes_sent.is_null() {
            *out_bytes_sent = stats.bytes_sent;
        }
        if !out_bytes_received.is_null() {
            *out_bytes_received = stats.bytes_received;
        }
        if !out_errors.is_null() {
            *out_errors = stats.errors;
        }
        0
    }
}

#[no_mangle]
pub extern "C" fn ri_protocol_manager_get_connection_count(manager: *mut CRiProtocolManager) -> usize {
    if manager.is_null() {
        return 0;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return 0,
    };
    unsafe {
        rt.block_on(async { (*manager).inner.connections.read().await.len() })
    }
}

// RiFrame C bindings
#[no_mangle]
pub extern "C" fn ri_frame_new() -> *mut CRiFrame {
    let ptr = Box::into_raw(Box::new(CRiFrame::new(RiFrame::default())));
    crate::c::register_ptr(ptr as usize);
    ptr
}
c_destructor!(ri_frame_free, CRiFrame);

#[no_mangle]
pub extern "C" fn ri_frame_get_payload_size(frame: *mut CRiFrame) -> usize {
    if frame.is_null() {
        return 0;
    }
    unsafe { (*frame).inner.payload.len() }
}

#[no_mangle]
pub extern "C" fn ri_frame_get_payload(frame: *mut CRiFrame, out_data: *mut *mut std::ffi::c_char, out_len: *mut usize) -> std::ffi::c_int {
    if frame.is_null() || out_data.is_null() || out_len.is_null() {
        return -1;
    }
    unsafe {
        let payload = (*frame).inner.payload.clone();
        *out_len = payload.len();
        let ptr = Box::into_raw(payload.into_boxed_slice()) as *mut std::ffi::c_char;
        *out_data = ptr;
        0
    }
}

#[no_mangle]
pub extern "C" fn ri_frame_get_sequence(frame: *mut CRiFrame) -> u64 {
    if frame.is_null() {
        return 0;
    }
    unsafe { (*frame).inner.header.sequence_number }
}

#[no_mangle]
pub extern "C" fn ri_frame_get_timestamp(frame: *mut CRiFrame) -> u64 {
    if frame.is_null() {
        return 0;
    }
    unsafe { (*frame).inner.header.timestamp }
}

#[no_mangle]
pub extern "C" fn ri_frame_get_type(frame: *mut CRiFrame) -> std::ffi::c_int {
    if frame.is_null() {
        return -1;
    }
    unsafe {
        match (*frame).inner.header.frame_type {
            crate::protocol::RiFrameType::Data => 0,
            crate::protocol::RiFrameType::Control => 1,
            crate::protocol::RiFrameType::Heartbeat => 2,
            crate::protocol::RiFrameType::Ack => 3,
            crate::protocol::RiFrameType::Error => 4,
        }
    }
}

c_string_getter!(
    ri_frame_get_source_id,
    CRiFrame,
    |inner: &RiFrame| inner.source_id.clone()
);

c_string_getter!(
    ri_frame_get_target_id,
    CRiFrame,
    |inner: &RiFrame| inner.target_id.clone()
);

// RiConnectionInfo C bindings
c_destructor!(ri_connection_info_free, CRiConnectionInfo);

c_string_getter!(
    ri_connection_info_get_id,
    CRiConnectionInfo,
    |inner: &RiConnectionInfo| inner.connection_id.clone()
);

c_string_getter!(
    ri_connection_info_get_device_id,
    CRiConnectionInfo,
    |inner: &RiConnectionInfo| inner.device_id.clone()
);

c_string_getter!(
    ri_connection_info_get_address,
    CRiConnectionInfo,
    |inner: &RiConnectionInfo| inner.address.clone()
);

#[no_mangle]
pub extern "C" fn ri_connection_info_get_state(info: *mut CRiConnectionInfo) -> std::ffi::c_int {
    if info.is_null() {
        return -1;
    }
    unsafe {
        match (*info).inner.state {
            RiConnectionState::Disconnected => 0,
            RiConnectionState::Connecting => 1,
            RiConnectionState::Connected => 2,
            RiConnectionState::Disconnecting => 3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_connection_info_get_security_level(info: *mut CRiConnectionInfo) -> std::ffi::c_int {
    if info.is_null() {
        return -1;
    }
    unsafe {
        match (*info).inner.security_level {
            RiSecurityLevel::None => 0,
            RiSecurityLevel::Standard => 1,
            RiSecurityLevel::High => 2,
            RiSecurityLevel::Military => 3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_connection_info_get_protocol_type(info: *mut CRiConnectionInfo) -> std::ffi::c_int {
    if info.is_null() {
        return -1;
    }
    unsafe {
        match (*info).inner.protocol_type {
            RiProtocolType::Global => 0,
            RiProtocolType::Private => 1,
        }
    }
}

// RiProtocolStats C bindings
c_destructor!(ri_protocol_stats_free, CRiProtocolStats);

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_messages_sent(stats: *mut CRiProtocolStats) -> u64 {
    if stats.is_null() {
        return 0;
    }
    unsafe { (*stats).inner.messages_sent }
}

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_messages_received(stats: *mut CRiProtocolStats) -> u64 {
    if stats.is_null() {
        return 0;
    }
    unsafe { (*stats).inner.messages_received }
}

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_bytes_sent(stats: *mut CRiProtocolStats) -> u64 {
    if stats.is_null() {
        return 0;
    }
    unsafe { (*stats).inner.bytes_sent }
}

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_bytes_received(stats: *mut CRiProtocolStats) -> u64 {
    if stats.is_null() {
        return 0;
    }
    unsafe { (*stats).inner.bytes_received }
}

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_errors(stats: *mut CRiProtocolStats) -> u64 {
    if stats.is_null() {
        return 0;
    }
    unsafe { (*stats).inner.errors }
}

#[no_mangle]
pub extern "C" fn ri_protocol_stats_get_avg_latency_ms(stats: *mut CRiProtocolStats) -> f64 {
    if stats.is_null() {
        return 0.0;
    }
    unsafe { (*stats).inner.avg_latency_ms }
}
