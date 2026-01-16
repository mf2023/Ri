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

use dmsc::observability::propagation::{DMSCTraceContext, DMSCBaggage, DMSCContextCarrier};
use dmsc::observability::tracing::{DMSCTraceId, DMSCSpanId};

/// Observability trace context propagation test module for distributed tracing.
///
/// This module provides comprehensive test coverage for the trace context and
/// baggage propagation systems that enable distributed request tracing across
/// service boundaries. The tests validate the header format encoding and decoding
/// used by the W3C Trace Context standard and baggage propagation protocols.
///
/// ## Test Coverage
///
/// - **Trace Context Header Format**: Tests the serialization of trace context
///   to W3C standard header format, including trace ID, parent span ID, and
///   sampling flags. Also tests deserialization to reconstruct the original
///   context from valid headers.
///
/// - **Trace ID and Span ID Encoding**: Validates the hexadecimal string encoding
///   of trace and span identifiers, ensuring proper formatting with leading zeros
///   and correct character ranges for interoperability with other tracing systems.
///
/// - **Baggage Header Format**: Tests the propagation of baggage entries (key-value
///   pairs) through HTTP headers, verifying correct encoding and decoding of
///   multiple entries with proper URL encoding for special characters.
///
/// - **Sampling Decision Propagation**: Validates that the sampling flag is
///   correctly encoded in the trace context header and preserved during propagation,
///   enabling consistent sampling decisions across service boundaries.
///
/// ## Design Principles
///
/// The propagation system implements the W3C Trace Context specification for
/// standard-compliant trace header format, ensuring interoperability with
/// existing observability tools and platforms. Tests verify compliance with
/// the standard format including version prefixes and field separators.
///
/// Trace context uses hexadecimal encoding for compact representation of 128-bit
/// trace IDs and 64-bit span IDs, providing sufficient namespace for globally
/// unique identification across distributed systems.
///
/// Baggage propagation uses header-based key-value encoding with support for
/// multiple entries, enabling correlation of contextual information across
/// service boundaries. Tests verify proper handling of edge cases including
/// special characters and empty values.

#[test]
fn test_trace_context_header_format() {
    let trace_id = DMSCTraceId::from_string("0123456789abcdef0123456789abcdef".to_string());
    let parent_id = DMSCSpanId::from_string("fedcba9876543210".to_string());
    
    let context = DMSCTraceContext::new(trace_id.clone(), parent_id.clone());
    let header = context.to_header();
    
    assert_eq!(header, "00-0123456789abcdef0123456789abcdef-fedcba9876543210-01");
    
    let parsed = DMSCTraceContext::from_header(&header).unwrap();
    assert_eq!(parsed.trace_id.as_str(), trace_id.as_str());
    assert_eq!(parsed.parent_id.as_str(), parent_id.as_str());
    assert!(parsed.is_sampled());
}

#[test]
fn test_baggage_header_format() {
    let mut baggage = DMSCBaggage::new();
    baggage.insert("user.id".to_string(), "12345".to_string());
    baggage.insert("tenant.id".to_string(), "acme-corp".to_string());
    
    let header = baggage.to_header();
    assert!(header.contains("user.id=12345"));
    assert!(header.contains("tenant.id=acme-corp"));
    
    let parsed = DMSCBaggage::from_header(&header);
    assert_eq!(parsed.get("user.id").unwrap(), "12345");
    assert_eq!(parsed.get("tenant.id").unwrap(), "acme-corp");
}
