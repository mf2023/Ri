//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::observability::tracing::{DMSTraceId, DMSSpanId};

/// W3C Trace Context propagation format
/// https://www.w3.org/TR/trace-context/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSTraceContext {
    pub version: u8,
    pub trace_id: DMSTraceId,
    pub parent_id: DMSSpanId,
    pub trace_flags: u8,
    pub trace_state: Option<String>,
}

impl DMSTraceContext {
    pub fn _Fnew(trace_id: DMSTraceId, parent_id: DMSSpanId) -> Self {
        Self {
            version: 0x00,
            trace_id,
            parent_id,
            trace_flags: 0x01, // Sampled flag
            trace_state: None,
        }
    }
    
    /// Parse from W3C Trace Context header format
    /// Format: "00-{trace-id}-{parent-id}-{trace-flags}"
    pub fn _Ffrom_header(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split('-').collect();
        if parts.len() != 4 {
            return None;
        }
        
        let version = u8::from_str_radix(parts[0], 16).ok()?;
        let trace_id = DMSTraceId::_Ffrom_string(parts[1].to_string());
        let parent_id = DMSSpanId::_Ffrom_string(parts[2].to_string());
        let trace_flags = u8::from_str_radix(parts[3], 16).ok()?;
        
        Some(Self {
            version,
            trace_id,
            parent_id,
            trace_flags,
            trace_state: None,
        })
    }
    
    /// Convert to W3C Trace Context header format
    pub fn _Fto_header(&self) -> String {
        format!(
            "{:02x}-{}-{}-{:02x}",
            self.version,
            self.trace_id._Fas_str(),
            self.parent_id._Fas_str(),
            self.trace_flags
        )
    }
    
    /// Check if trace is sampled
    pub fn _Fis_sampled(&self) -> bool {
        (self.trace_flags & 0x01) != 0
    }
    
    /// Set sampled flag
    pub fn _Fset_sampled(&mut self, sampled: bool) {
        if sampled {
            self.trace_flags |= 0x01;
        } else {
            self.trace_flags &= !0x01;
        }
    }
}

/// Baggage propagation for cross-cutting concerns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSBaggage {
    items: HashMap<String, String>,
}

impl DMSBaggage {
    pub fn _Fnew() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    
    pub fn _Finsert(&mut self, key: String, value: String) {
        self.items.insert(key, value);
    }
    
    pub fn _Fget(&self, key: &str) -> Option<&String> {
        self.items.get(key)
    }
    
    pub fn _Fremove(&mut self, key: &str) {
        self.items.remove(key);
    }
    
    /// Parse from W3C Baggage header format
    /// Format: "key1=value1,key2=value2"
    pub fn _Ffrom_header(header: &str) -> Self {
        let mut baggage = Self::_Fnew();
        
        for item in header.split(',') {
            let item = item.trim();
            if let Some(eq_pos) = item.find('=') {
                let key = item[..eq_pos].trim().to_string();
                let value = item[eq_pos + 1..].trim().to_string();
                baggage._Finsert(key, value);
            }
        }
        
        baggage
    }
    
    /// Convert to W3C Baggage header format
    pub fn _Fto_header(&self) -> String {
        self.items
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(",")
    }
}

/// Context carrier for distributed tracing
#[derive(Debug, Clone)]
pub struct DMSContextCarrier {
    pub trace_context: Option<DMSTraceContext>,
    pub baggage: DMSBaggage,
}

impl DMSContextCarrier {
    pub fn _Fnew() -> Self {
        Self {
            trace_context: None,
            baggage: DMSBaggage::_Fnew(),
        }
    }
    
    pub fn _Fwith_trace_context(mut self, trace_context: DMSTraceContext) -> Self {
        self.trace_context = Some(trace_context);
        self
    }
    
    pub fn _Fwith_baggage(mut self, baggage: DMSBaggage) -> Self {
        self.baggage = baggage;
        self
    }
    
    /// Extract from HTTP headers
    pub fn _Ffrom_headers(headers: &HashMap<String, String>) -> Self {
        let mut carrier = Self::_Fnew();
        
        // Extract trace context from traceparent header
        if let Some(traceparent) = headers.get("traceparent") {
            if let Some(trace_context) = DMSTraceContext::_Ffrom_header(traceparent) {
                carrier.trace_context = Some(trace_context);
            }
        }
        
        // Extract baggage from baggage header
        if let Some(baggage_header) = headers.get("baggage") {
            carrier.baggage = DMSBaggage::_Ffrom_header(baggage_header);
        }
        
        carrier
    }
    
    /// Inject into HTTP headers
    pub fn _Finject_into_headers(&self, headers: &mut HashMap<String, String>) {
        if let Some(ref trace_context) = self.trace_context {
            headers.insert("traceparent".to_string(), trace_context._Fto_header());
        }
        
        let baggage_header = self.baggage._Fto_header();
        if !baggage_header.is_empty() {
            headers.insert("baggage".to_string(), baggage_header);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
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
}