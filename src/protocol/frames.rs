//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
//! checksums, and frame integrity verification for the DMS private protocol.

use std::io::{Read, Write};
use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use crc32fast::Hasher;
use thiserror::Error;

use crate::core::{DMSResult, DMSError};

/// Protocol frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSFrameType {
    /// Control frame for protocol management
    Control = 0x01,
    /// Data frame for payload transmission
    Data = 0x02,
    /// Authentication frame
    Auth = 0x03,
    /// Keep-alive frame
    KeepAlive = 0x04,
    /// Error frame
    Error = 0x05,
    /// Encrypted frame
    Encrypted = 0x06,
}

impl DMSFrameType {
    /// Convert from byte to frame type
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(DMSFrameType::Control),
            0x02 => Some(DMSFrameType::Data),
            0x03 => Some(DMSFrameType::Auth),
            0x04 => Some(DMSFrameType::KeepAlive),
            0x05 => Some(DMSFrameType::Error),
            0x06 => Some(DMSFrameType::Encrypted),
            _ => None,
        }
    }
}

/// Protocol frame header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSFrameHeader {
    /// Frame magic number (4 bytes)
    pub magic: u32,
    /// Frame type (1 byte)
    pub frame_type: u8,
    /// Protocol version (1 byte)
    pub version: u8,
    /// Frame flags (2 bytes)
    pub flags: u16,
    /// Payload length (4 bytes)
    pub payload_length: u32,
    /// Frame sequence number (4 bytes)
    pub sequence_number: u32,
    /// Timestamp (8 bytes)
    pub timestamp: u64,
    /// CRC32 checksum (4 bytes)
    pub checksum: u32,
}

impl DMSFrameHeader {
    /// Frame magic number
    pub const MAGIC: u32 = 0x444D5350; // "DMSP" in ASCII
    
    /// Current protocol version
    pub const VERSION: u8 = 0x01;
    
    /// Create a new frame header
    pub fn new(frame_type: DMSFrameType, payload_length: u32, sequence_number: u32) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        Self {
            magic: Self::MAGIC,
            frame_type: frame_type as u8,
            version: Self::VERSION,
            flags: 0,
            payload_length,
            sequence_number,
            timestamp,
            checksum: 0, // Will be calculated later
        }
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
    pub fn from_bytes(bytes: &[u8]) -> DMSResult<Self> {
        if bytes.len() < 32 {
            return Err(DMSError::FrameError("Invalid header length".to_string()));
        }
        
        let magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let frame_type = bytes[4];
        let version = bytes[5];
        let flags = u16::from_be_bytes(bytes[6..8].try_into().unwrap());
        let payload_length = u32::from_be_bytes(bytes[8..12].try_into().unwrap());
        let sequence_number = u32::from_be_bytes(bytes[12..16].try_into().unwrap());
        let timestamp = u64::from_be_bytes(bytes[16..24].try_into().unwrap());
        let checksum = u32::from_be_bytes(bytes[24..28].try_into().unwrap());
        
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

/// Complete protocol frame
#[derive(Debug, Clone)]
pub struct DMSFrame {
    /// Frame header
    pub header: DMSFrameHeader,
    /// Frame payload
    pub payload: Vec<u8>,
}

impl DMSFrame {
    /// Create a new frame
    pub fn new(frame_type: DMSFrameType, payload: Vec<u8>, sequence_number: u32) -> Self {
        let header = DMSFrameHeader::new(frame_type, payload.len() as u32, sequence_number);
        Self { header, payload }
    }
    
    /// Create a control frame
    pub fn control_frame(control_data: Vec<u8>, sequence_number: u32) -> Self {
        Self::new(DMSFrameType::Control, control_data, sequence_number)
    }
    
    /// Create a data frame
    pub fn data_frame(data: Vec<u8>, sequence_number: u32) -> Self {
        Self::new(DMSFrameType::Data, data, sequence_number)
    }
    
    /// Create an authentication frame
    pub fn auth_frame(auth_data: Vec<u8>, sequence_number: u32) -> Self {
        Self::new(DMSFrameType::Auth, auth_data, sequence_number)
    }
    
    /// Create a keep-alive frame
    pub fn keepalive_frame(sequence_number: u32) -> Self {
        Self::new(DMSFrameType::KeepAlive, vec![], sequence_number)
    }
    
    /// Create an error frame
    pub fn error_frame(error_code: u32, error_message: String, sequence_number: u32) -> DMSResult<Self> {
        let mut payload = Vec::new();
        payload.extend_from_slice(&error_code.to_be_bytes());
        payload.extend_from_slice(error_message.as_bytes());
        Ok(Self::new(DMSFrameType::Error, payload, sequence_number))
    }
    
    /// Serialize frame to bytes
    pub fn to_bytes(&self) -> DMSResult<Vec<u8>> {
        let mut header = self.header.clone();
        
        // Calculate and set checksum
        header.checksum = header.calculate_checksum(&self.payload);
        
        let mut result = Vec::new();
        result.extend_from_slice(&header.to_bytes());
        result.extend_from_slice(&self.payload);
        
        Ok(result)
    }
    
    /// Deserialize frame from bytes
    pub fn from_bytes(bytes: &[u8]) -> DMSResult<Self> {
        if bytes.len() < 32 {
            return Err(DMSError::FrameError("Frame too short".to_string()));
        }
        
        let header = DMSFrameHeader::from_bytes(&bytes[0..32])?;
        
        // Verify magic number
        if header.magic != DMSFrameHeader::MAGIC {
            return Err(DMSError::FrameError(format!("Invalid magic number: 0x{:08X}", header.magic)));
        }
        
        // Verify version
        if header.version != DMSFrameHeader::VERSION {
            return Err(DMSError::FrameError(format!("Unsupported version: {}", header.version)));
        }
        
        // Check payload length
        if bytes.len() < 32 + header.payload_length as usize {
            return Err(DMSError::FrameError("Incomplete frame".to_string()));
        }
        
        let payload = bytes[32..32 + header.payload_length as usize].to_vec();
        
        // Verify checksum
        if !header.verify_checksum(&payload) {
            return Err(DMSError::FrameError("Checksum verification failed".to_string()));
        }
        
        Ok(Self { header, payload })
    }
    
    /// Get frame type
    pub fn frame_type(&self) -> Option<DMSFrameType> {
        DMSFrameType::from_u8(self.header.frame_type)
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
        self.header.magic == DMSFrameHeader::MAGIC &&
        self.header.version == DMSFrameHeader::VERSION &&
        self.header.verify_checksum(&self.payload)
    }
}

/// Frame parser for reading frames from a stream
pub struct DMSFrameParser {
    /// Buffer for incomplete frames
    buffer: Vec<u8>,
    /// Next expected sequence number
    next_sequence: u32,
}

impl DMSFrameParser {
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
    pub fn parse_frame(&mut self) -> DMSResult<Option<DMSFrame>> {
        if self.buffer.len() < 32 {
            return Ok(None); // Not enough data for header
        }
        
        // Try to parse header
        let header = DMSFrameHeader::from_bytes(&self.buffer[0..32])?;
        let total_length = 32 + header.payload_length as usize;
        
        if self.buffer.len() < total_length {
            return Ok(None); // Not enough data for complete frame
        }
        
        // Parse complete frame
        let frame_bytes = self.buffer[0..total_length].to_vec();
        let frame = DMSFrame::from_bytes(&frame_bytes)?;
        
        // Check sequence number
        if frame.header.sequence_number != self.next_sequence {
            return Err(DMSError::FrameError(format!(
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

impl Default for DMSFrameParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Frame builder for creating frames
pub struct DMSFrameBuilder {
    /// Next sequence number
    next_sequence: u32,
}

impl DMSFrameBuilder {
    /// Create a new frame builder
    pub fn new() -> Self {
        Self { next_sequence: 0 }
    }
    
    /// Build a control frame
    pub fn build_control_frame(&mut self, control_data: Vec<u8>) -> DMSFrame {
        let frame = DMSFrame::control_frame(control_data, self.next_sequence);
        self.next_sequence = self.next_sequence.wrapping_add(1);
        frame
    }
    
    /// Build a data frame
    pub fn build_data_frame(&mut self, data: Vec<u8>) -> DMSFrame {
        let frame = DMSFrame::data_frame(data, self.next_sequence);
        self.next_sequence = self.next_sequence.wrapping_add(1);
        frame
    }
    
    /// Build an authentication frame
    pub fn build_auth_frame(&mut self, auth_data: Vec<u8>) -> DMSFrame {
        let frame = DMSFrame::auth_frame(auth_data, self.next_sequence);
        self.next_sequence = self.next_sequence.wrapping_add(1);
        frame
    }
    
    /// Build a keep-alive frame
    pub fn build_keepalive_frame(&mut self) -> DMSFrame {
        let frame = DMSFrame::keepalive_frame(self.next_sequence);
        self.next_sequence = self.next_sequence.wrapping_add(1);
        frame
    }
    
    /// Build an error frame
    pub fn build_error_frame(&mut self, error_code: u32, error_message: String) -> DMSResult<DMSFrame> {
        let frame = DMSFrame::error_frame(error_code, error_message, self.next_sequence)?;
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

impl Default for DMSFrameBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_frame_creation() {
        let payload = b"Hello, DMS Protocol!";
        let frame = DMSFrame::data_frame(payload.to_vec(), 12345);
        
        assert_eq!(frame.header.magic, DMSFrameHeader::MAGIC);
        assert_eq!(frame.header.version, DMSFrameHeader::VERSION);
        assert_eq!(frame.header.payload_length, payload.len() as u32);
        assert_eq!(frame.header.sequence_number, 12345);
        assert_eq!(frame.frame_type(), Some(DMSFrameType::Data));
        assert!(frame.is_valid());
    }
    
    #[test]
    fn test_frame_serialization() {
        let payload = b"Test payload for serialization";
        let original_frame = DMSFrame::data_frame(payload.to_vec(), 67890);
        
        // Serialize to bytes
        let serialized = original_frame.to_bytes().unwrap();
        
        // Deserialize from bytes
        let deserialized_frame = DMSFrame::from_bytes(&serialized).unwrap();
        
        // Verify integrity
        assert_eq!(original_frame.header.magic, deserialized_frame.header.magic);
        assert_eq!(original_frame.header.frame_type, deserialized_frame.header.frame_type);
        assert_eq!(original_frame.header.version, deserialized_frame.header.version);
        assert_eq!(original_frame.header.payload_length, deserialized_frame.header.payload_length);
        assert_eq!(original_frame.header.sequence_number, deserialized_frame.header.sequence_number);
        assert_eq!(original_frame.header.timestamp, deserialized_frame.header.timestamp);
        assert_eq!(original_frame.payload, deserialized_frame.payload);
    }
    
    #[test]
    fn test_frame_parser() {
        let mut builder = DMSFrameBuilder::new();
        let frame1 = builder.build_data_frame(b"First frame".to_vec());
        let frame2 = builder.build_data_frame(b"Second frame".to_vec());
        
        let serialized1 = frame1.to_bytes().unwrap();
        let serialized2 = frame2.to_bytes().unwrap();
        
        let mut combined = Vec::new();
        combined.extend_from_slice(&serialized1);
        combined.extend_from_slice(&serialized2);
        
        let mut parser = DMSFrameParser::new();
        parser.add_data(&combined);
        
        let parsed1 = parser.parse_frame().unwrap().unwrap();
        let parsed2 = parser.parse_frame().unwrap().unwrap();
        
        assert_eq!(parsed1.payload, b"First frame");
        assert_eq!(parsed2.payload, b"Second frame");
    }
    
    #[test]
    fn test_checksum_verification() {
        let payload = b"Checksum test payload";
        let mut frame = DMSFrame::data_frame(payload.to_vec(), 99999);
        
        // Calculate correct checksum
        frame.header.checksum = frame.header.calculate_checksum(&frame.payload);
        
        // Verify checksum is correct
        assert!(frame.is_valid());
        
        // Corrupt the payload
        frame.payload[0] ^= 0xFF;
        
        // Verify checksum fails
        assert!(!frame.is_valid());
    }
}