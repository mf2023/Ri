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

//! # Protocol Module C API
//!
//! This module provides C language bindings for DMSC's protocol handling infrastructure. The protocol
//! module delivers comprehensive support for encoding, decoding, and transforming data across various
//! wire formats and communication protocols. This C API enables C/C++ applications to leverage DMSC's
//! protocol capabilities for building interoperable distributed systems with standardized data exchange.
//!
//! ## Module Architecture
//!
//! The protocol module comprises three primary components that together provide complete protocol
//! management capabilities:
//!
//! - **DMSCProtocolConfig**: Configuration container for protocol codec parameters including encoding
//!   formats, framing options, compression settings, and validation rules. The configuration object
//!   controls how data is serialized and deserialized, ensuring consistent behavior across the
//!   application.
//!
//! - **DMSCProtocolManager**: Central manager for protocol registration, codec lookup, and protocol
//!   negotiation. The manager handles the complete lifecycle of protocol operations including codec
//!   selection, error handling, and protocol switching.
//!
//! - **DMSCFrame**: Low-level frame abstraction for message framing and boundary management. Frames
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
//! DMSCProtocolConfig* config = dmsc_protocol_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create protocol config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure protocol settings
//! dmsc_protocol_config_set_format(config, PROTOCOL_FORMAT_MSGPACK);
//! dmsc_protocol_config_set_compression(config, COMPRESSION_SNAPPY);
//! dmsc_protocol_config_set_validation_enabled(config, true);
//!
//! // Create protocol manager
//! DMSCProtocolManager* manager = dmsc_protocol_manager_new(config);
//! if (manager == NULL) {
//!     fprintf(stderr, "Failed to create protocol manager\n");
//!     dmsc_protocol_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Register custom schema
//! int result = dmsc_protocol_manager_register_schema(
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
//! DMSCFrame* frame = dmsc_frame_new();
//! if (frame == NULL) {
//!     fprintf(stderr, "Failed to create frame\n");
//!     dmsc_protocol_manager_free(manager);
//!     dmsc_protocol_config_free(config);
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
//! result = dmsc_protocol_manager_encode(
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
//!         int decode_result = dmsc_protocol_manager_decode(
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
//!             dmsc_string_free(decoded_buffer);
//!         }
//!
//!     dmsc_string_free(output_buffer);
//! }
//!
//! // Frame the message for transport
//! dmsc_frame_reset(frame);
//! dmsc_frame_append(frame, output_buffer, output_len);
//!
//! // Read framed data
//! const char* frame_data = dmsc_frame_data(frame);
//! size_t frame_size = dmsc_frame_size(frame);
//!
//! // Cleanup
//! dmsc_frame_free(frame);
//! dmsc_protocol_manager_free(manager);
//! dmsc_protocol_config_free(config);
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
//! This module depends on the following DMSC components:
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

use crate::protocol::{DMSCFrame, DMSCProtocolConfig, DMSCProtocolManager};


c_wrapper!(CDMSCProtocolConfig, DMSCProtocolConfig);
c_wrapper!(CDMSCProtocolManager, DMSCProtocolManager);
c_wrapper!(CDMSCFrame, DMSCFrame);

// DMSCProtocolConfig constructors and destructors
c_constructor!(
    dmsc_protocol_config_new,
    CDMSCProtocolConfig,
    DMSCProtocolConfig,
    DMSCProtocolConfig::default()
);
c_destructor!(dmsc_protocol_config_free, CDMSCProtocolConfig);
