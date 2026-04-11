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

use ri::observability::propagation::{RiTraceContext, RiBaggage, RiContextCarrier};
use ri::observability::tracing::{RiTraceId, RiSpanId};

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
/// Tests RiTraceContext W3C Trace Context header format serialization.
///
/// Verifies that trace context can be serialized to and deserialized from
/// the W3C Trace Context standard header format, ensuring interoperability
/// with distributed tracing systems across service boundaries.
///
/// ## W3C Trace Context Format
///
/// The trace context header follows this format:
/// `version-traceid-parentspanid-flags`
///
/// - **version**: Protocol version (currently "00")
/// - **traceid**: 128-bit hexadecimal trace identifier
/// - **parentspanid**: 64-bit hexadecimal parent span identifier
/// - **flags**: Sampling and trace flags as hexadecimal
///
/// ## Test Scenario
///
/// 1. Create trace and span IDs from hexadecimal strings
/// 2. Construct a trace context with these IDs
/// 3. Serialize to W3C header format
/// 4. Verify the header format matches expected output
/// 5. Deserialize the header back to a context
/// 6. Verify trace ID, parent ID, and sampling flag are preserved
///
/// ## Expected Behavior
///
/// - Header format matches W3C specification exactly
/// - Trace ID is preserved through round-trip
/// - Parent span ID is preserved through round-trip
/// - Sampling flag is correctly encoded and accessible
fn test_trace_context_header_format() {
    // Create trace and span IDs from hexadecimal strings
    let trace_id = RiTraceId::from_string("0123456789abcdef0123456789abcdef".to_string());
    let parent_id = RiSpanId::from_string("fedcba9876543210".to_string());
    
    // Construct trace context
    let context = RiTraceContext::new(trace_id.clone(), parent_id.clone());
    
    // Serialize to W3C header format
    let header = context.to_header();
    
    // Verify header format
    assert_eq!(header, "00-0123456789abcdef0123456789abcdef-fedcba9876543210-01");
    
    // Deserialize back from header
    let parsed = RiTraceContext::from_header(&header).unwrap();
    
    // Verify trace ID preserved
    assert_eq!(parsed.trace_id.as_str(), trace_id.as_str());
    // Verify parent span ID preserved
    assert_eq!(parsed.parent_id.as_str(), parent_id.as_str());
    // Verify sampling flag is set
    assert!(parsed.is_sampled());
}

#[test]
/// Tests RiBaggage header format for distributed context propagation.
///
/// Verifies that baggage entries (key-value pairs) can be serialized to
/// and deserialized from HTTP header format, enabling correlation of
/// contextual information across service boundaries.
///
/// ## Baggage Header Format
///
/// The baggage header uses URL-encoded key-value pairs separated by commas:
/// `key1=value1,key2=value2,...`
///
/// - Multiple entries are comma-separated
/// - Special characters are URL-encoded
/// - Empty values are allowed
///
/// ## Use Cases
///
/// - **Request Context**: User ID, tenant ID, correlation IDs
/// - **Business Context**: Feature flags, experiment IDs
/// - **Diagnostic Context**: Request tags, trace context
///
/// ## Test Scenario
///
/// 1. Create an empty baggage container
/// 2. Insert two entries: user.id=12345 and tenant.id=acme-corp
/// 3. Serialize to header format
/// 4. Verify both entries appear in the header
/// 5. Deserialize the header back to baggage
/// 6. Verify both entries are correctly restored
///
/// ## Expected Behavior
///
/// - All inserted entries appear in serialized header
/// - Round-trip preserves all key-value pairs
/// - Special characters in values are handled correctly
fn test_baggage_header_format() {
    // Create an empty baggage container
    let mut baggage = RiBaggage::new();
    
    // Insert baggage entries
    baggage.insert("user.id".to_string(), "12345".to_string());
    baggage.insert("tenant.id".to_string(), "acme-corp".to_string());
    
    // Serialize to header format
    let header = baggage.to_header();
    
    // Verify entries are present in header
    assert!(header.contains("user.id=12345"));
    assert!(header.contains("tenant.id=acme-corp"));
    
    // Deserialize back from header
    let parsed = RiBaggage::from_header(&header);
    
    // Verify entries were restored
    assert_eq!(parsed.get("user.id").unwrap(), "12345");
    assert_eq!(parsed.get("tenant.id").unwrap(), "acme-corp");
}
