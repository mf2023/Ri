// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
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

use dms_core::observability::propagation::{DMSTraceContext, DMSBaggage, DMSContextCarrier};
use dms_core::observability::tracing::{DMSTraceId, DMSSpanId};

#[test]
fn test_trace_context_header_format() {
    let trace_id = DMSTraceId::from_string("0123456789abcdef0123456789abcdef".to_string());
    let parent_id = DMSSpanId::from_string("fedcba9876543210".to_string());
    
    let context = DMSTraceContext::new(trace_id.clone(), parent_id.clone());
    let header = context.to_header();
    
    assert_eq!(header, "00-0123456789abcdef0123456789abcdef-fedcba9876543210-01");
    
    let parsed = DMSTraceContext::from_header(&header).unwrap();
    assert_eq!(parsed.trace_id.as_str(), trace_id.as_str());
    assert_eq!(parsed.parent_id.as_str(), parent_id.as_str());
    assert!(parsed.is_sampled());
}

#[test]
fn test_baggage_header_format() {
    let mut baggage = DMSBaggage::new();
    baggage.insert("user.id".to_string(), "12345".to_string());
    baggage.insert("tenant.id".to_string(), "acme-corp".to_string());
    
    let header = baggage.to_header();
    assert!(header.contains("user.id=12345"));
    assert!(header.contains("tenant.id=acme-corp"));
    
    let parsed = DMSBaggage::from_header(&header);
    assert_eq!(parsed.get("user.id").unwrap(), "12345");
    assert_eq!(parsed.get("tenant.id").unwrap(), "acme-corp");
}
