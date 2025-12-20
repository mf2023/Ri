// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dms::protocol::frames::*;
use dms::core::{DMSCResult, DMSCError};

#[test]
fn test_frame_creation() {
    let payload = b"Hello, DMSC Protocol!";
    let frame = DMSCFrame::data_frame(payload.to_vec(), 12345).unwrap();
    
    assert_eq!(frame.header.magic, DMSCFrameHeader::MAGIC);
    assert_eq!(frame.header.version, DMSCFrameHeader::VERSION);
    assert_eq!(frame.header.payload_length, payload.len() as u32);
    assert_eq!(frame.header.sequence_number, 12345);
    assert_eq!(frame.frame_type(), Some(DMSCFrameType::Data));
    assert!(frame.is_valid());
}

#[test]
fn test_frame_serialization() {
    let payload = b"Test payload for serialization";
    let original_frame = DMSCFrame::data_frame(payload.to_vec(), 67890).unwrap();
    
    // Serialize to bytes
    let serialized = original_frame.to_bytes()
        .expect("Failed to serialize frame in test");
    
    // Deserialize from bytes
    let deserialized_frame = DMSCFrame::from_bytes(&serialized)
        .expect("Failed to deserialize frame in test");
    
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
    let mut builder = DMSCFrameBuilder::new();
    let frame1 = builder.build_data_frame(b"First frame".to_vec());
    let frame2 = builder.build_data_frame(b"Second frame".to_vec());
    
    let serialized1 = frame1.to_bytes()
        .expect("Failed to serialize frame1 in test");
    let serialized2 = frame2.to_bytes()
        .expect("Failed to serialize frame2 in test");
    
    let mut combined = Vec::new();
    combined.extend_from_slice(&serialized1);
    combined.extend_from_slice(&serialized2);
    
    let mut parser = DMSCFrameParser::new();
    parser.add_data(&combined);
    
    let parsed1 = parser.parse_frame()
        .expect("Failed to parse frame1 in test")
        .expect("Frame1 should be Some");
    let parsed2 = parser.parse_frame()
        .expect("Failed to parse frame2 in test")
        .expect("Frame2 should be Some");
    
    assert_eq!(parsed1.payload, b"First frame");
    assert_eq!(parsed2.payload, b"Second frame");
}

#[test]
fn test_checksum_verification() {
    let payload = b"Checksum test payload";
    let mut frame = DMSCFrame::data_frame(payload.to_vec(), 99999).unwrap();
    
    // Calculate correct checksum
    frame.header.checksum = frame.header.calculate_checksum(&frame.payload);
    
    // Verify checksum is correct
    assert!(frame.is_valid());
    
    // Corrupt the payload
    frame.payload[0] ^= 0xFF;
    
    // Verify checksum fails
    assert!(!frame.is_valid());
}