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

extern crate dms;

use dms::observability::propagation::{DMSTraceContext, DMSBaggage, DMSContextCarrier};
use dms::observability::tracing::{DMSTraceId, DMSSpanId};

#[test]
fn test_trace_context_header_format() {
    let trace_id = DMSTraceId::_Ffrom_string("0123456789abcdef0123456789abcdef".to_string());
    let parent_id = DMSSpanId::_Ffrom_string("fedcba9876543210".to_string());
    
    let context = DMSTraceContext::_Fnew(trace_id.clone(), parent_id.clone());
    let header = context._Fto_header();
    
    assert_eq!(header, "00-0123456789abcdef0123456789abcdef-fedcba9876543210-01");
    
    let parsed = DMSTraceContext::_Ffrom_header(&header).unwrap();
    assert_eq!(parsed.trace_id._Fas_str(), trace_id._Fas_str());
    assert_eq!(parsed.parent_id._Fas_str(), parent_id._Fas_str());
    assert!(parsed._Fis_sampled());
}

#[test]
fn test_baggage_header_format() {
    let mut baggage = DMSBaggage::_Fnew();
    baggage._Finsert("user.id".to_string(), "12345".to_string());
    baggage._Finsert("tenant.id".to_string(), "acme-corp".to_string());
    
    let header = baggage._Fto_header();
    assert!(header.contains("user.id=12345"));
    assert!(header.contains("tenant.id=acme-corp"));
    
    let parsed = DMSBaggage::_Ffrom_header(&header);
    assert_eq!(parsed._Fget("user.id").unwrap(), "12345");
    assert_eq!(parsed._Fget("tenant.id").unwrap(), "acme-corp");
}
