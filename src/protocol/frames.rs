//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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
//! checksums, and frame integrity verification for the DMSC private protocol.

use std::io::{Read, Write};
use std::convert::TryInto;
use serde::{Deserialize, Serialize};
use crc32fast::Hasher;
use thiserror::Error;

use crate::core::{DMSCResult, DMSCError};

/// Protocol frame types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCFrameType {
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

/// Protocol frame header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCFrameHeader {
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

impl DMSCFrameHeader {
    /// Frame magic number
    pub const MAGIC: u32 = 0x444D5350; // "DMSCP" in ASCII
    
    /// Current protocol version
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

/// Complete protocol frame
#[derive(Debug, Clone)]
pub struct DMSCFrame {
    /// Frame header
    pub header: DMSCFrameHeader,
    /// Frame payload
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

/// Frame parser for reading frames from a stream
pub struct DMSCFrameParser {
    /// Buffer for incomplete frames
    buffer: Vec<u8>,
    /// Next expected sequence number
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

/// Frame builder for creating frames
pub struct DMSCFrameBuilder {
    /// Next sequence number
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


