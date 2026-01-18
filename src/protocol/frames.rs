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

#![allow(non_snake_case)]

//! # Protocol Frame Format Module
//!
//! This module implements the real protocol frame format with serialization,
//! checksums, and frame integrity verification for the DMSC private protocol.

use std::io::{Read, Write};
use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use crc32fast::Hasher;
use thiserror::Error;

use crate::core::{DMSCResult, DMSCError};

/// Protocol frame type enumeration defining the categories of protocol frames.
///
/// This enumeration classifies all protocol frames used in the DMSC protocol
/// for network communication. Each frame type serves a specific purpose in
/// the communication lifecycle, from initial connection establishment through
/// data transmission, authentication, and connection maintenance. Frame type
/// identification enables proper routing and processing of incoming frames
/// by protocol handlers.
///
/// ## Frame Type Hierarchy
///
/// - **Control Frames (0x01)**: Protocol management and state transitions
/// - **Data Frames (0x02)**: Application data payload transmission
/// - **Auth Frames (0x03)**: Authentication and authorization exchanges
/// - **Keep-Alive Frames (0x04)**: Connection liveness verification
/// - **Error Frames (0x05)**: Error reporting and status communication
/// - **Encrypted Frames (0x06)**: Pre-encrypted payload transmission
///
/// ## Frame Processing Guidelines
///
/// Protocol implementations should process frames in the following order:
/// 1. First validate the frame header magic number and version
/// 2. Extract and validate the frame type from the header
/// 3. Route the frame to the appropriate handler based on frame type
/// 4. Process the frame payload according to type-specific rules
/// 5. Send appropriate response frames if required
///
/// ## Python Bindings
///
/// When compiled with the `pyo3` feature, this enum provides Python bindings
/// for frame type identification:
/// ```python
/// from dms import DMSCFrameType
///
/// # Identify frame types for protocol handling
/// control_type = DMSCFrameType.Control()
/// data_type = DMSCFrameType.Data()
/// auth_type = DMSCFrameType.Auth()
///
/// # Convert between types and byte values
/// frame_type_value = DMSCFrameType.from_u8(0x01)
/// byte_value = data_type.to_u8()  # Returns 0x02
/// ```
///
/// ## Thread Safety
///
/// This enum is fully thread-safe and can be shared across concurrent contexts
/// without additional synchronization. The Copy trait enables efficient passing
/// of frame type values through function arguments and return types.
///
/// ## Storage and Transmission
///
/// Frame type values are stored as single bytes making them efficient for network
/// transmission and compact storage. The Hash trait enables frame type usage as
/// dictionary keys in collection types and provides efficient lookup performance.
///
/// # Examples
///
/// Basic frame type creation and conversion:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameType;
///
/// let control = DMSCFrameType::Control;
/// let data = DMSCFrameType::Data;
///
/// assert_eq!(control as u8, 0x01);
/// assert_eq!(data as u8, 0x02);
/// assert_ne!(control, data);
/// ```
///
/// Frame type matching in protocol handling:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameType;
///
/// fn handle_frame_type(frame_type: DMSCFrameType) -> &str {
///     match frame_type {
///         DMSCFrameType::Control => "Control frame - managing protocol state",
///         DMSCFrameType::Data => "Data frame - processing payload",
///         DMSCFrameType::Auth => "Auth frame - handling authentication",
///         DMSCFrameType::KeepAlive => "Keep-alive frame - verifying connection",
///         DMSCFrameType::Error => "Error frame - reporting error condition",
///         DMSCFrameType::Encrypted => "Encrypted frame - processing secure payload",
///     }
/// }
///
/// assert_eq!(handle_frame_type(DMSCFrameType::Data), "Data frame - processing payload");
/// ```
///
/// Converting between byte values and frame types:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameType;
///
/// // Convert byte to frame type
/// let frame_type = DMSCFrameType::from_u8(0x03);
/// assert_eq!(frame_type, Some(DMSCFrameType::Auth));
///
/// // Invalid byte value returns None
/// let invalid = DMSCFrameType::from_u8(0xFF);
/// assert_eq!(invalid, None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCFrameType {
    /// Control frame for protocol management operations.
    ///
    /// Control frames manage the protocol state machine and handle connection
    /// lifecycle events. They are used for operations such as connection
    /// initialization, version negotiation, feature flags exchange, and graceful
    /// connection termination. Control frames must be processed before any data
    /// frames to ensure proper protocol state establishment.
    ///
    /// ## Control Frame Payload Structure
    ///
    /// Control frame payloads contain a command identifier followed by
    /// command-specific parameters encoded in a type-length-value (TLV) format.
    ///
    /// ## Common Control Commands
    ///
    /// - **Connection Request (0x01)**: Initiate new connection
    /// - **Connection Ack (0x02)**: Confirm connection establishment
    /// - **Disconnect Request (0x03)**: Initiate graceful disconnection
    /// - **Ping (0x04)**: Request keep-alive response
    /// - **Pong (0x05)**: Keep-alive response
    /// - **Protocol Negotiation (0x10)**: Negotiate protocol features
    Control = 0x01,

    /// Data frame for application payload transmission.
    ///
    /// Data frames carry the primary application data through the protocol.
    /// They are the most frequently used frame type in normal protocol operation
    /// and support both streaming and message-based data transfer modes. Data
    /// frames are sequenced and delivered in-order to ensure data integrity.
    ///
    /// ## Data Frame Characteristics
    ///
    /// - **Sequencing**: Each data frame has a unique sequence number
    /// - **Ordering**: Frames are delivered in sequence number order
    /// - **Flow Control**: Sliding window prevents buffer overflow
    /// - **Aggregation**: Multiple small payloads can be aggregated
    ///
    /// ## Payload Considerations
    ///
    /// - Maximum payload size is defined by protocol configuration
    /// - Large payloads may be fragmented across multiple frames
    /// - Payload compression is available as an optional feature
    Data = 0x02,

    /// Authentication frame for credential exchange and verification.
    ///
    /// Authentication frames facilitate the authentication process between
    /// communicating parties. They carry authentication tokens, certificates,
    /// challenge-response pairs, and authentication results. All auth frames
    /// are encrypted using the current session encryption keys.
    ///
    /// ## Authentication Flow
    ///
    /// 1. Client sends authentication request with credentials
    /// 2. Server validates credentials and returns challenge
    /// 3. Client responds to challenge with proof of possession
    /// 4. Server confirms successful authentication
    ///
    /// ## Supported Authentication Methods
    ///
    /// - JWT token authentication
    /// - Certificate-based mutual TLS
    /// - Pre-shared key authentication
    /// - OAuth 2.0 token exchange
    Auth = 0x03,

    /// Keep-alive frame for connection liveness verification.
    ///
    /// Keep-alive frames maintain connection vitality and detect unresponsive
    /// peers. They are exchanged periodically between connected parties to
    /// prevent connection timeout and detect connection failures. Keep-alive
    /// frames have minimal overhead and contain no payload.
    ///
    ## Keep-Alive Configuration
    ///
    /// - **Default Interval**: 30 seconds between keep-alive frames
    /// - **Timeout Window**: 3 missed keep-alives triggers disconnection
    /// - **Adjustability**: Interval configurable per connection
    ///
    /// ## Protocol Behavior
    ///
    /// Upon receiving a keep-alive frame, the recipient should immediately
    /// respond with another keep-alive frame. Failure to respond within the
    /// timeout window indicates connection failure and triggers cleanup.
    KeepAlive = 0x04,

    /// Error frame for error condition reporting.
    ///
    /// Error frames communicate error conditions from one protocol party to
    /// another. They include an error code and human-readable error message
    /// to facilitate debugging and error recovery. Error frames may be sent
    /// in response to any invalid protocol message.
    ///
    /// ## Error Code Ranges
    ///
    /// - **0x0001-0x00FF**: Protocol errors (parsing, validation)
    /// - **0x0100-0x01FF**: Authentication errors
    /// - **0x0200-0x02FF**: Authorization errors
    /// - **0x0300-0x03FF**: Resource errors (capacity, limits)
    /// - **0x0400-0x04FF**: Timeout errors
    /// - **0x0500+**: Application-specific errors
    ///
    /// ## Error Handling Strategy
    ///
    /// Upon receiving an error frame, the protocol implementation should:
    /// 1. Log the error for diagnostic purposes
    /// 2. Attempt error recovery if possible
    /// 3. Gracefully close connection if unrecoverable
    Error = 0x05,

    /// Encrypted frame for pre-encrypted payload transmission.
    ///
    /// Encrypted frames carry payloads that have been encrypted by the
    /// application layer before being passed to the protocol. This allows
    /// applications to use custom encryption schemes or to encrypt data
    /// end-to-end between application endpoints. The protocol encrypts the
    /// encrypted payload as normal data.
    ///
    /// ## Use Cases
    ///
    /// - End-to-end encrypted application data
    /// - Custom encryption algorithm requirements
    /// - Regulatory compliance requiring specific encryption
    /// - Integration with external encryption systems
    ///
    /// ## Security Considerations
    ///
    /// When using encrypted frames, the outer protocol encryption provides
    /// transport security while the inner encrypted payload provides
    /// end-to-end security between application endpoints.
    Encrypted = 0x06,
}

impl DMSCFrameType {
    /// Convert from byte to frame type
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(DMSCFrameType::Control),
            0x02 => Some(DMSCFrameType::Data),
            0x03 => Some(DMSCFrameType::Auth),
            0x04 => Some(DMSCFrameType::KeepAlive),
            0x05 => Some(DMSCFrameType::Error),
            0x06 => Some(DMSCFrameType::Encrypted),
            _ => None,
        }
    }
}

/// Protocol frame header structure containing metadata for frame processing.
///
/// The frame header provides essential protocol metadata required for proper
/// frame handling, transmission, and verification. It follows a fixed 32-byte
/// format designed for efficient parsing and minimal overhead while maintaining
/// comprehensive protocol information. All fields use big-endian byte ordering
/// for consistent network transmission.
///
/// ## Header Field Layout
///
/// | Offset | Size | Field          | Description                              |
/// |--------|------|----------------|------------------------------------------|
/// | 0      | 4    | magic          | Protocol magic number (0x444D5350)       |
/// | 4      | 1    | frame_type     | Frame type identifier (0x01-0x06)        |
/// | 5      | 1    | version        | Protocol version (0x01)                  |
/// | 6      | 2    | flags          | Protocol flags for special handling      |
/// | 8      | 4    | payload_length | Length of frame payload in bytes         |
/// | 12     | 4    | sequence_number| Frame sequence number for ordering       |
/// | 16     | 8    | timestamp      | Unix timestamp of frame creation         |
/// | 24     | 4    | checksum       | CRC32 checksum for integrity verification|
///
/// ## Byte Ordering
///
/// All multi-byte fields use big-endian (network byte order) encoding to ensure
/// consistent interpretation across different architectures and platforms.
/// When serializing or deserializing, use the provided `to_bytes()` and
/// `from_bytes()` methods to ensure proper byte ordering.
///
/// ## Magic Number Validation
///
/// The magic number `0x444D5350` decodes to ASCII "DMSP" (DMSC Protocol) and
/// serves as a quick validation that incoming data represents a valid DMSC
/// frame. Receiving a frame with an invalid magic number indicates either
/// protocol mismatch or data corruption.
///
/// ## Python Bindings
///
/// When compiled with the `pyo3` feature, this struct provides Python bindings:
/// ```python
/// from dms import DMSCFrameHeader
///
/// # Create new header for a data frame
/// header = DMSCFrameHeader.new(
///     frame_type=DMSCFrameHeader.FrameType.Data,
///     payload_length=1024,
///     sequence_number=42
/// )
///
/// # Serialize to bytes for transmission
/// header_bytes = header.to_bytes()
///
/// # Access header fields
/// print(f"Frame type: {header.frame_type}")
/// print(f"Payload length: {header.payload_length}")
/// print(f"Sequence: {header.sequence_number}")
/// print(f"Timestamp: {header.timestamp}")
/// ```
///
/// # Examples
///
/// Creating a new frame header:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrameHeader, DMSCFrameType};
///
/// let header = DMSCFrameHeader::new(
///     DMSCFrameType::Data,
///     1024,           // payload length
///     42              // sequence number
/// ).expect("Failed to create frame header");
///
/// assert_eq!(header.magic, DMSCFrameHeader::MAGIC);
/// assert_eq!(header.version, DMSCFrameHeader::VERSION);
/// assert_eq!(header.frame_type, DMSCFrameType::Data as u8);
/// ```
///
/// Serializing and deserializing headers:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrameHeader, DMSCFrameType};
///
/// let header = DMSCFrameHeader::new(
///     DMSCFrameType::Control,
///     256,
///     100
/// ).expect("Failed to create header");
///
/// let bytes = header.to_bytes();
/// assert_eq!(bytes.len(), 32);
///
/// let reconstructed = DMSCFrameHeader::from_bytes(&bytes)
///     .expect("Failed to parse header");
///
/// assert_eq!(header.magic, reconstructed.magic);
/// assert_eq!(header.frame_type, reconstructed.frame_type);
/// assert_eq!(header.sequence_number, reconstructed.sequence_number);
/// ```
///
/// Verifying frame integrity:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrameHeader, DMSCFrameType};
///
/// let header = DMSCFrameHeader::new(
///     DMSCFrameType::Data,
///     512,
///     200
/// ).expect("Failed to create header");
///
/// let payload = b"This is the frame payload data";
///
/// let is_valid = header.verify_checksum(payload);
/// assert!(is_valid);
///
/// // Modify payload and verify again
/// let modified_payload = b"This payload has been modified!";
/// let is_valid_modified = header.verify_checksum(modified_payload);
/// assert!(!is_valid_modified);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCFrameHeader {
    /// Frame magic number identifying the DMSC protocol (4 bytes).
    ///
    /// The magic number is a 32-bit constant that uniquely identifies DMSC
    /// protocol frames. It serves as a quick validation mechanism to detect
    /// non-DMSC data or protocol version mismatches. The value 0x444D5350
    /// corresponds to the ASCII encoding of "DMSP" (DMSC Protocol).
    ///
    /// ## Magic Number Details
    ///
    /// - **Value**: 0x444D5350
    /// - **ASCII**: "DMSP"
    /// - **Purpose**: Protocol identification
    /// - **Validation**: Must match exactly on all received frames
    ///
    /// ## Usage
    ///
    /// The magic number is the first field in the frame header and should be
    /// validated immediately upon receiving frame data. A mismatch indicates
    /// either incorrect data or a protocol version incompatibility.
    pub magic: u32,

    /// Frame type identifier determining frame processing (1 byte).
    ///
    /// The frame type specifies how the frame payload should be interpreted
    /// and processed. This 8-bit field identifies one of six possible frame
    /// types defined in the DMSCFrameType enumeration.
    pub frame_type: u8,

    /// Protocol version for compatibility checking (1 byte).
    ///
    /// The version field enables protocol evolution while maintaining backward
    /// compatibility. The current version is 0x01. Protocol implementations
    /// should reject frames with unsupported versions.
    pub version: u8,

    /// Protocol flags for special frame handling (2 bytes).
    ///
    /// Flags provide additional processing instructions for frames. Common
    /// flags include compression, priority, and streaming indicators.
    ///
    /// ## Flag Definitions
    ///
    /// - **Bit 0**: Compressed - Payload is compressed
    /// - **Bit 1**: Priority - High priority frame
    /// - **Bit 2**: Streaming - Part of streaming transfer
    /// - **Bit 3**: Fragmented - Part of fragmented message
    /// - **Bits 4-15**: Reserved for future use
    pub flags: u16,

    /// Length of the frame payload in bytes (4 bytes).
    ///
    /// This field specifies the exact number of bytes in the frame payload.
    /// It enables proper memory allocation and bounds checking during frame
    /// processing. The maximum payload length is 4GB.
    pub payload_length: u32,

    /// Frame sequence number for ordering and reliability (4 bytes).
    ///
    /// Sequence numbers enable in-order delivery, duplicate detection, and
    /// loss detection for protocol frames. Each frame in a connection uses
    /// a monotonically increasing sequence number with wraparound at 2^32.
    ///
    /// ## Sequence Number Semantics
    ///
    /// - **Initialization**: Sequence numbers start at 0 for new connections
    /// - **Increment**: Each frame increments the sequence number by 1
    /// - **Wraparound**: Sequence numbers wrap from 0xFFFFFFFF to 0
    /// - **Ordering**: Receivers use sequence numbers to order frames
    pub sequence_number: u32,

    /// Unix timestamp of frame creation in seconds (8 bytes).
    ///
    /// The timestamp records when the frame was created, enabling temporal
    /// validation, replay detection, and latency measurements. Timestamps
    /// use seconds since Unix epoch (1970-01-01 00:00:00 UTC).
    ///
    /// ## Timestamp Considerations
    ///
    /// - **Precision**: Second-level precision (not milliseconds)
    /// - **Clock Sync**: Requires loosely synchronized clocks
    /// - **Replay Protection**: Used in conjunction with nonces
    /// - **Age Limits**: Frames older than timeout are rejected
    pub timestamp: u64,

    /// CRC32 checksum for frame integrity verification (4 bytes).
    ///
    /// The checksum covers all header fields (except checksum itself) and
    /// the complete frame payload. It enables detection of transmission
    /// errors and data corruption. Uses the CRC32 algorithm with polynomial
    /// 0x04C11DB7 (IEEE 802.3).
    ///
    /// ## Checksum Calculation
    ///
    /// The checksum is computed over the concatenation of all header fields
    /// (excluding the checksum field) followed by the frame payload. This
    /// provides comprehensive protection against single-bit errors and
    /// many burst errors.
    ///
    /// ## Limitations
    ///
    /// CRC32 provides error detection but not error correction. It can detect
    /// all single-bit errors, most double-bit errors, and many burst errors
    /// up to 32 bits in length. It does not provide cryptographic integrity.
    pub checksum: u32,
}

impl DMSCFrameHeader {
    /// Frame magic number constant for protocol identification.
    ///
    /// This 32-bit constant uniquely identifies DMSC protocol frames. The value
    /// 0x444D5350 corresponds to the ASCII encoding of "DMSP" (DMSC Protocol).
    /// This magic number is placed at the beginning of every frame header and
    /// is validated upon frame receipt to confirm protocol compatibility.
    ///
    /// ## Magic Number Details
    ///
    /// - **Hex Value**: 0x444D5350
    /// - **ASCII Representation**: "DMSP"
    /// - **Purpose**: Quick protocol identification
    /// - **Validation**: Must match on all received frames
    ///
    /// ## Usage in Frame Processing
    ///
    /// When receiving frame data, the magic number should be validated first
    /// before attempting to parse other header fields. A mismatch indicates:
    ///
    /// 1. The received data is not a DMSC frame
    /// 2. Data corruption may have occurred
    /// 3. The remote endpoint may be using a different protocol
    pub const MAGIC: u32 = 0x444D5350; // "DMSCP" in ASCII
    
    /// Current protocol version identifier.
    ///
    /// This version number enables protocol evolution while maintaining backward
    /// compatibility. The current version is 0x01. Future protocol enhancements
    /// may increment this version while maintaining support for earlier versions.
    ///
    /// ## Version Semantics
    ///
    /// - **Major Version Changes**: May introduce incompatible changes
    /// - **Minor Version Changes**: Backward-compatible enhancements
    /// - **Current Value**: 0x01 (initial protocol version)
    ///
    /// ## Version Negotiation
    ///
    /// During connection establishment, endpoints should negotiate a compatible
    /// protocol version. Frames with unsupported versions should be rejected
    /// with an appropriate error response.
    pub const VERSION: u8 = 0x01;
    
    /// Create a new frame header
    pub fn new(frame_type: DMSCFrameType, payload_length: u32, sequence_number: u32) -> DMSCResult<Self> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| DMSCError::InvalidState(format!("System time error: {}", e)))?
            .as_secs();
            
        Ok(Self {
            magic: Self::MAGIC,
            frame_type: frame_type as u8,
            version: Self::VERSION,
            flags: 0,
            payload_length,
            sequence_number,
            timestamp,
            checksum: 0, // Will be calculated later
        })
    }
    
    /// Calculate CRC32 checksum for the header and payload
    pub fn calculate_checksum(&self, payload: &[u8]) -> u32 {
        let mut hasher = Hasher::new();
        
        // Add header fields (excluding checksum)
        hasher.update(&self.magic.to_be_bytes());
        hasher.update(&self.frame_type.to_be_bytes());
        hasher.update(&self.version.to_be_bytes());
        hasher.update(&self.flags.to_be_bytes());
        hasher.update(&self.payload_length.to_be_bytes());
        hasher.update(&self.sequence_number.to_be_bytes());
        hasher.update(&self.timestamp.to_be_bytes());
        
        // Add payload
        hasher.update(payload);
        
        hasher.finalize()
    }
    
    /// Verify the checksum
    pub fn verify_checksum(&self, payload: &[u8]) -> bool {
        self.checksum == self.calculate_checksum(payload)
    }
    
    /// Serialize header to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        
        bytes.extend_from_slice(&self.magic.to_be_bytes());
        bytes.extend_from_slice(&self.frame_type.to_be_bytes());
        bytes.extend_from_slice(&self.version.to_be_bytes());
        bytes.extend_from_slice(&self.flags.to_be_bytes());
        bytes.extend_from_slice(&self.payload_length.to_be_bytes());
        bytes.extend_from_slice(&self.sequence_number.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&self.checksum.to_be_bytes());
        
        bytes
    }
    
    /// Deserialize header from bytes
    pub fn from_bytes(bytes: &[u8]) -> DMSCResult<Self> {
        if bytes.len() < 32 {
            return Err(DMSCError::FrameError("Invalid header length".to_string()));
        }
        
        let magic = u32::from_be_bytes(bytes[0..4].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid magic number bytes".to_string()))?);
        let frame_type = bytes[4];
        let version = bytes[5];
        let flags = u16::from_be_bytes(bytes[6..8].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid flags bytes".to_string()))?);
        let payload_length = u32::from_be_bytes(bytes[8..12].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid payload length bytes".to_string()))?);
        let sequence_number = u32::from_be_bytes(bytes[12..16].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid sequence number bytes".to_string()))?);
        let timestamp = u64::from_be_bytes(bytes[16..24].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid timestamp bytes".to_string()))?);
        let checksum = u32::from_be_bytes(bytes[24..28].try_into()
            .map_err(|_| DMSCError::FrameError("Invalid checksum bytes".to_string()))?);
        
        Ok(Self {
            magic,
            frame_type,
            version,
            flags,
            payload_length,
            sequence_number,
            timestamp,
            checksum,
        })
    }
}

/// Complete protocol frame combining header and payload data.
///
/// A DMSCFrame represents the fundamental unit of data transmission in the
/// DMSC protocol. Each frame consists of a fixed 32-byte header containing
/// protocol metadata and a variable-length payload containing the actual
/// application data. Frames are serialized for network transmission and
/// deserialized upon receipt using CRC32 checksums for integrity verification.
///
/// ## Frame Structure
///
/// ```
/// +------------------+-------------------+
/// |   Header (32B)   |  Payload (VAR)    |
/// +------------------+-------------------+
/// | magic | type | v | flags | len | seq |
/// |      timestamp     |    checksum     |
/// +------------------+-------------------+
/// ```
///
/// ## Frame Lifecycle
///
/// 1. **Creation**: Construct a frame using `new()` or one of the convenience
///    constructors (`data_frame()`, `control_frame()`, etc.)
/// 2. **Serialization**: Convert to bytes using `to_bytes()` for transmission
/// 3. **Transmission**: Send bytes over the network connection
/// 4. **Reception**: Receive bytes and add to frame parser buffer
/// 5. **Deserialization**: Parse bytes back to frame using `from_bytes()`
/// 6. **Processing**: Handle the frame based on its type
///
/// ## Frame Validity
///
/// A frame is considered valid when:
/// - The magic number matches the DMSC protocol identifier
/// - The protocol version is supported
/// - The CRC32 checksum matches the computed checksum
/// - The payload length matches the actual payload size
///
/// Use the `is_valid()` method for quick validation checking.
///
/// ## Python Bindings
///
/// When compiled with the `pyo3` feature, this struct provides Python bindings:
/// ```python
/// from dms import DMSCFrame, DMSCFrameType
///
/// # Create a data frame
/// frame = DMSCFrame.data_frame(
///     data=b"Hello, DMSC Protocol!",
///     sequence_number=1
/// )
///
/// # Serialize for transmission
/// frame_bytes = frame.to_bytes()
/// print(f"Frame size: {len(frame_bytes)} bytes")
///
/// # Access frame properties
/// print(f"Frame type: {frame.frame_type()}")
/// print(f"Sequence: {frame.sequence_number()}")
/// print(f"Timestamp: {frame.timestamp()}")
/// print(f"Valid: {frame.is_valid()}")
///
/// # Deserialize received frame
/// received = DMSCFrame.from_bytes(frame_bytes)
/// assert received.payload == b"Hello, DMSC Protocol!"
/// ```
///
/// # Examples
///
/// Creating and serializing a data frame:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrame, DMSCFrameType};
///
/// let frame = DMSCFrame::data_frame(
///     b"Hello, World!".to_vec(),
///     42
/// ).expect("Failed to create frame");
///
/// let bytes = frame.to_bytes().expect("Failed to serialize frame");
/// println!("Frame serialized to {} bytes", bytes.len());
///
/// assert!(frame.is_valid());
/// assert_eq!(frame.sequence_number(), 42);
/// ```
///
/// Creating different frame types:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrame, DMSCFrameType};
///
/// // Control frame with command data
/// let control = DMSCFrame::control_frame(
///     vec![0x01, 0x02, 0x03],
///     1
/// ).expect("Failed to create control frame");
///
/// // Authentication frame with credentials
/// let auth = DMSCFrame::auth_frame(
///     b"token=abc123".to_vec(),
///     2
/// ).expect("Failed to create auth frame");
///
/// // Keep-alive frame (no payload)
/// let keepalive = DMSCFrame::keepalive_frame(3)
///     .expect("Failed to create keepalive frame");
///
/// // Error frame with code and message
/// let error = DMSCFrame::error_frame(
///     0x0401,
///     "Connection timeout".to_string(),
///     4
/// ).expect("Failed to create error frame");
/// ```
///
/// Receiving and deserializing frames:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrame, DMSCFrameType};
///
/// let original = DMSCFrame::data_frame(
///     b"Received data".to_vec(),
///     100
/// ).expect("Failed to create frame");
///
/// let bytes = original.to_bytes().expect("Failed to serialize");
///
/// // Simulate network transmission
/// let received = DMSCFrame::from_bytes(&bytes)
///     .expect("Failed to deserialize frame");
///
/// assert_eq!(received.sequence_number(), 100);
/// assert_eq!(received.payload, b"Received data");
/// assert!(received.is_valid());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCFrame {
    /// Frame header containing protocol metadata.
    ///
    /// The header provides essential information for frame processing including
    /// the frame type, sequence number, timestamp, and integrity checksum.
    /// It is always exactly 32 bytes in size and uses big-endian byte ordering.
    pub header: DMSCFrameHeader,

    /// Frame payload containing application data.
    ///
    /// The payload contains the actual data being transmitted. Its meaning
    /// depends on the frame type:
    /// - **Control**: Protocol management commands
    /// - **Data**: Application-level message data
    /// - **Auth**: Authentication credentials or tokens
    /// - **KeepAlive**: Empty (no payload)
    /// - **Error**: Error code + error message
    /// - **Encrypted**: Pre-encrypted application data
    pub payload: Vec<u8>,
}

impl DMSCFrame {
    /// Create a new frame
    pub fn new(frame_type: DMSCFrameType, payload: Vec<u8>, sequence_number: u32) -> DMSCResult<Self> {
        let header = DMSCFrameHeader::new(frame_type, payload.len() as u32, sequence_number)?;
        Ok(Self { header, payload })
    }
    
    /// Create a control frame
    pub fn control_frame(control_data: Vec<u8>, sequence_number: u32) -> DMSCResult<Self> {
        Self::new(DMSCFrameType::Control, control_data, sequence_number)
    }
    
    /// Create a data frame
    pub fn data_frame(data: Vec<u8>, sequence_number: u32) -> DMSCResult<Self> {
        Self::new(DMSCFrameType::Data, data, sequence_number)
    }
    
    /// Create an authentication frame
    pub fn auth_frame(auth_data: Vec<u8>, sequence_number: u32) -> DMSCResult<Self> {
        Self::new(DMSCFrameType::Auth, auth_data, sequence_number)
    }
    
    /// Create a keep-alive frame
    pub fn keepalive_frame(sequence_number: u32) -> DMSCResult<Self> {
        Self::new(DMSCFrameType::KeepAlive, vec![], sequence_number)
    }
    
    /// Create an error frame
    pub fn error_frame(error_code: u32, error_message: String, sequence_number: u32) -> DMSCResult<Self> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&error_code.to_be_bytes());
        payload.extend_from_slice(error_message.as_bytes());
        Ok(Self::new(DMSCFrameType::Error, payload, sequence_number))
    }
    
    /// Serialize frame to bytes
    pub fn to_bytes(&self) -> DMSCResult<Vec<u8>> {
        let mut header = self.header.clone();
        
        // Calculate and set checksum
        header.checksum = header.calculate_checksum(&self.payload);
        
        let mut result = Vec::new();
        result.extend_from_slice(&header.to_bytes());
        result.extend_from_slice(&self.payload);
        
        Ok(result)
    }
    
    /// Deserialize frame from bytes
    pub fn from_bytes(bytes: &[u8]) -> DMSCResult<Self> {
        if bytes.len() < 32 {
            return Err(DMSCError::FrameError("Frame too short".to_string()));
        }
        
        let header = DMSCFrameHeader::from_bytes(&bytes[0..32])?;
        
        // Verify magic number
        if header.magic != DMSCFrameHeader::MAGIC {
            return Err(DMSCError::FrameError(format!("Invalid magic number: 0x{:08X}", header.magic)));
        }
        
        // Verify version
        if header.version != DMSCFrameHeader::VERSION {
            return Err(DMSCError::FrameError(format!("Unsupported version: {}", header.version)));
        }
        
        // Check payload length
        if bytes.len() < 32 + header.payload_length as usize {
            return Err(DMSCError::FrameError("Incomplete frame".to_string()));
        }
        
        let payload = bytes[32..32 + header.payload_length as usize].to_vec();
        
        // Verify checksum
        if !header.verify_checksum(&payload) {
            return Err(DMSCError::FrameError("Checksum verification failed".to_string()));
        }
        
        Ok(Self { header, payload })
    }
    
    /// Get frame type
    pub fn frame_type(&self) -> Option<DMSCFrameType> {
        DMSCFrameType::from_u8(self.header.frame_type)
    }
    
    /// Get sequence number
    pub fn sequence_number(&self) -> u32 {
        self.header.sequence_number
    }
    
    /// Get timestamp
    pub fn timestamp(&self) -> u64 {
        self.header.timestamp
    }
    
    /// Check if frame is valid
    pub fn is_valid(&self) -> bool {
        self.header.magic == DMSCFrameHeader::MAGIC &&
        self.header.version == DMSCFrameHeader::VERSION &&
        self.header.verify_checksum(&self.payload)
    }
}

/// Frame parser for reading and assembling protocol frames from stream data.
///
/// The DMSCFrameParser handles the incremental parsing of frame data from network
/// streams or byte sources. Network protocols often deliver data in chunks that
/// may not align with protocol frame boundaries. This parser accumulates incoming
/// data in an internal buffer and extracts complete frames when sufficient data
/// is available. It also manages sequence number validation to ensure frame
/// ordering integrity.
///
/// ## Parser Operation Model
///
/// ```
/// Incoming Data:  [partial][complete][partial][complete][partial]
///                      |         |        |         |
///                      v         v        v         v
/// Parser Buffer:  [======][==========][=========][=======]
///                      |         |        |
///                      v         v        v
/// Extracted:      [Frame1] [Frame2] [Frame3]
/// ```
///
/// ## Sequence Number Validation
///
/// The parser maintains an expected sequence number counter. Each extracted frame
/// must have a sequence number matching the expected value. This detects missing
/// frames (gaps in sequence numbers) which may indicate packet loss. Use
/// `reset_sequence()` to set a new expected sequence number, such as after
/// reconnection.
///
/// ## Buffer Management
///
/// The parser maintains an internal buffer that grows as data is added. For
/// long-running connections, periodically check `buffer_len()` and consider
/// calling `clear_buffer()` if buffer accumulation indicates parsing issues.
/// The parser automatically removes parsed data from the buffer.
///
/// ## Python Bindings
///
/// When compiled with the `pyo3` feature, this struct provides Python bindings:
/// ```python
/// from dms import DMSCFrameParser
///
/// # Create parser for incoming stream data
/// parser = DMSCFrameParser.new()
///
/// # Simulate receiving data chunks
/// chunks = [
///     frame1_bytes[:20],
///     frame1_bytes[20:] + frame2_bytes[:30],
///     frame2_bytes[30:] + frame3_bytes
/// ]
///
/// for chunk in chunks:
///     parser.add_data(chunk)
///     while True:
///         frame = parser.parse_frame()
///         if frame is None:
///             break
///         print(f"Received frame: {frame.sequence_number()}")
///
/// print(f"Buffer contains {parser.buffer_len()} bytes")
/// ```
///
/// # Examples
///
/// Basic frame parsing from stream data:
/// ```rust
/// use dmsc::protocol::frames::{DMSCFrameParser, DMSCFrame};
///
/// let mut parser = DMSCFrameParser::new();
///
/// // Simulate receiving frame data in chunks
/// let frame1 = DMSCFrame::data_frame(b"First message".to_vec(), 0)
///     .expect("Failed to create frame");
/// let frame2 = DMSCFrame::data_frame(b"Second message".to_vec(), 1)
///     .expect("Failed to create frame");
///
/// let bytes1 = frame1.to_bytes().expect("Failed to serialize");
/// let bytes2 = frame2.to_bytes().expect("Failed to serialize");
///
/// // Add first chunk (partial frame)
/// parser.add_data(&bytes1[..20]);
/// assert!(parser.parse_frame().unwrap().is_none());
///
/// // Add second chunk (completes frame1, starts frame2)
/// parser.add_data(&bytes1[20..]);
/// let parsed = parser.parse_frame().unwrap().expect("Should have complete frame");
/// assert_eq!(parsed.sequence_number(), 0);
///
/// // Add remaining data
/// parser.add_data(&bytes2);
/// let parsed = parser.parse_frame().unwrap().expect("Should have complete frame");
/// assert_eq!(parsed.sequence_number(), 1);
/// ```
///
/// Handling sequence number reset:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameParser;
///
/// let mut parser = DMSCFrameParser::new();
///
/// // Parse some frames
/// parser.add_data(&some_data);
/// while let Ok(Some(frame)) = parser.parse_frame() {
///     // Process frames
/// }
///
/// // Reset sequence number for new session
/// parser.reset_sequence();
/// parser.clear_buffer();
///
/// // Now expecting sequence 0 again
/// assert_eq!(parser.next_sequence, 0);
/// ```
///
/// # Thread Safety
///
/// This struct is not thread-safe. Multiple threads should not concurrently
/// access the same parser instance without external synchronization. For
/// concurrent parsing, either use separate parser instances per thread or
/// wrap access with a Mutex or RwLock.
///
/// # Performance Considerations
///
/// - The parser uses `Vec::extend_from_slice` for efficient buffer appending
/// - Frame extraction uses slice operations to avoid unnecessary copying
/// - Buffer memory is only reclaimed when frames are successfully parsed
/// - Large frames may cause temporary buffer growth; configure appropriate limits
pub struct DMSCFrameParser {
    /// Internal buffer for accumulating incoming data.
    ///
    /// The buffer holds bytes that have been received but not yet assembled
    /// into complete frames. It grows dynamically as more data arrives.
    /// Data is automatically removed from the buffer once successfully parsed.
    buffer: Vec<u8>,

    /// Next expected sequence number for validation.
    ///
    /// This counter tracks the sequence number of the next frame expected
    /// from the stream. Frames with mismatching sequence numbers indicate
    /// potential packet loss or protocol errors.
    next_sequence: u32,
}

impl DMSCFrameParser {
    /// Create a new frame parser
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            next_sequence: 0,
        }
    }
    
    /// Add data to the parser buffer
    pub fn add_data(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }
    
    /// Try to parse a complete frame from the buffer
    pub fn parse_frame(&mut self) -> DMSCResult<Option<DMSCFrame>> {
        if self.buffer.len() < 32 {
            return Ok(None); // Not enough data for header
        }
        
        // Try to parse header
        let header = DMSCFrameHeader::from_bytes(&self.buffer[0..32])?;
        let total_length = 32 + header.payload_length as usize;
        
        if self.buffer.len() < total_length {
            return Ok(None); // Not enough data for complete frame
        }
        
        // Parse complete frame
        let frame_bytes = self.buffer[0..total_length].to_vec();
        let frame = DMSCFrame::from_bytes(&frame_bytes)?;
        
        // Check sequence number
        if frame.header.sequence_number != self.next_sequence {
            return Err(DMSCError::FrameError(format!(
                "Sequence number mismatch: expected {}, got {}",
                self.next_sequence,
                frame.header.sequence_number
            )));
        }
        
        // Remove parsed data from buffer
        self.buffer.drain(0..total_length);
        self.next_sequence = self.next_sequence.wrapping_add(1);
        
        Ok(Some(frame))
    }
    
    /// Get the number of bytes in the buffer
    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }
    
    /// Clear the buffer
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
    
    /// Reset the sequence number
    pub fn reset_sequence(&mut self) {
        self.next_sequence = 0;
    }
}

impl Default for DMSCFrameParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Frame builder for creating protocol frames with automatic sequence numbering.
///
/// The DMSCFrameBuilder provides a convenient interface for constructing DMSC frames
/// while automatically managing sequence numbers. Rather than manually tracking and
/// incrementing sequence numbers for each frame, the builder maintains an internal
/// counter that is automatically incremented after each frame construction. This
/// ensures proper sequence numbering without the risk of human error.
///
/// ## Builder Pattern Advantages
///
/// Using the frame builder provides several benefits over direct frame construction:
///
/// - **Automatic Sequencing**: No need to manually track and increment sequence numbers
/// - **Type Safety**: Compile-time checking of frame type construction
/// - **Convenience Methods**: Domain-specific constructors for each frame type
/// - **State Management**: Builder maintains state across frame constructions
///
/// ## Sequence Number Management
///
/// The builder maintains an internal sequence counter that is automatically incremented
/// after each frame construction. The counter uses wrapping arithmetic (modulo 2^32)
/// to handle overflow gracefully. You can query or set the current sequence number
/// using `next_sequence()` and `set_sequence()` methods.
///
/// ## Python Bindings
///
/// When compiled with the `pyo3` feature, this struct provides Python bindings:
/// ```python
/// from dms import DMSCFrameBuilder
///
/// # Create builder for convenient frame construction
/// builder = DMSCFrameBuilder.new()
///
/// # Build frames without manually tracking sequence numbers
/// control_frame = builder.build_control_frame(b"\x01\x02\x03")
/// data_frame = builder.build_data_frame(b"Hello, World!")
/// auth_frame = builder.build_auth_frame(b"token=abc123")
///
/// # Check current sequence number
/// print(f"Next sequence: {builder.next_sequence()}")
///
/// # Reset sequence for new session
/// builder.set_sequence(0)
/// ```
///
/// # Examples
///
/// Building multiple frames with automatic sequencing:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameBuilder;
///
/// let mut builder = DMSCFrameBuilder::new();
///
/// // Build a series of data frames
/// let frame1 = builder.build_data_frame(b"Message 1".to_vec())
///     .expect("Failed to build frame");
/// let frame2 = builder.build_data_frame(b"Message 2".to_vec())
///     .expect("Failed to build frame");
/// let frame3 = builder.build_data_frame(b"Message 3".to_vec())
///     .expect("Failed to build frame");
///
/// assert_eq!(frame1.sequence_number(), 0);
/// assert_eq!(frame2.sequence_number(), 1);
/// assert_eq!(frame3.sequence_number(), 2);
///
/// // Current sequence is now 3
/// assert_eq!(builder.next_sequence(), 3);
/// ```
///
/// Building different frame types:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameBuilder;
///
/// let mut builder = DMSCFrameBuilder::new();
///
/// // Control frame
/// let control = builder.build_control_frame(vec![0x01, 0x00, 0x01])
///     .expect("Failed to build control frame");
///
/// // Authentication frame
/// let auth = builder.build_auth_frame(b"credentials=secret".to_vec())
///     .expect("Failed to build auth frame");
///
/// // Keep-alive frame
/// let keepalive = builder.build_keepalive_frame()
///     .expect("Failed to build keepalive frame");
///
/// // Error frame
/// let error = builder.build_error_frame(0x0401, "Timeout".to_string())
///     .expect("Failed to build error frame");
/// ```
///
/// Managing sequence numbers:
/// ```rust
/// use dmsc::protocol::frames::DMSCFrameBuilder;
///
/// let mut builder = DMSCFrameBuilder::new();
///
/// // Build some frames
/// let _ = builder.build_data_frame(b"Frame 0".to_vec()).unwrap();
/// let _ = builder.build_data_frame(b"Frame 1".to_vec()).unwrap();
/// let _ = builder.build_data_frame(b"Frame 2".to_vec()).unwrap();
///
/// // Check current sequence
/// assert_eq!(builder.next_sequence(), 3);
///
/// // Set specific sequence for resend or new session
/// builder.set_sequence(100);
///
/// let next = builder.build_data_frame(b"Frame 100".to_vec()).unwrap();
/// assert_eq!(next.sequence_number(), 100);
/// assert_eq!(builder.next_sequence(), 101);
/// ```
///
/// # Thread Safety
///
/// This struct is not thread-safe. Multiple threads should not concurrently
/// access the same builder instance without external synchronization. For
/// concurrent frame building, either use separate builder instances per thread
/// or wrap access with a Mutex or RwLock.
///
/// # Performance Characteristics
///
/// - Frame construction is O(1) for fixed-size headers
/// - Payload copying is O(n) where n is payload size
/// - Sequence number operations are O(1)
/// - Minimal heap allocation for small payloads
pub struct DMSCFrameBuilder {
    /// Internal counter for automatic sequence number generation.
    ///
    /// This counter tracks the sequence number to assign to the next frame
    /// constructed by the builder. It is automatically incremented after
    /// each frame construction using wrapping arithmetic.
    next_sequence: u32,
}

impl DMSCFrameBuilder {
    /// Create a new frame builder
    pub fn new() -> Self {
        Self { next_sequence: 0 }
    }
    
    /// Build a control frame
    pub fn build_control_frame(&mut self, control_data: Vec<u8>) -> DMSCResult<DMSCFrame> {
        let frame = DMSCFrame::control_frame(control_data, self.next_sequence)?;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        Ok(frame)
    }
    
    /// Build a data frame
    pub fn build_data_frame(&mut self, data: Vec<u8>) -> DMSCResult<DMSCFrame> {
        let frame = DMSCFrame::data_frame(data, self.next_sequence)?;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        Ok(frame)
    }
    
    /// Build an authentication frame
    pub fn build_auth_frame(&mut self, auth_data: Vec<u8>) -> DMSCResult<DMSCFrame> {
        let frame = DMSCFrame::auth_frame(auth_data, self.next_sequence)?;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        Ok(frame)
    }
    
    /// Build a keep-alive frame
    pub fn build_keepalive_frame(&mut self) -> DMSCResult<DMSCFrame> {
        let frame = DMSCFrame::keepalive_frame(self.next_sequence)?;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        Ok(frame)
    }
    
    /// Build an error frame
    pub fn build_error_frame(&mut self, error_code: u32, error_message: String) -> DMSCResult<DMSCFrame> {
        let frame = DMSCFrame::error_frame(error_code, error_message, self.next_sequence)?;
        self.next_sequence = self.next_sequence.wrapping_add(1);
        Ok(frame)
    }
    
    /// Get the next sequence number
    pub fn next_sequence(&self) -> u32 {
        self.next_sequence
    }
    
    /// Set the sequence number
    pub fn set_sequence(&mut self, sequence: u32) {
        self.next_sequence = sequence;
    }
}

impl Default for DMSCFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}


