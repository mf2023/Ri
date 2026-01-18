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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{RwLock, mpsc};

use crate::core::{DMSCResult, DMSCError};
use super::super::{DMSCProtocolType};
use super::connection::DMSCDeviceType;

/// Security coordinator for cross-protocol security enforcement.
pub struct DMSCSecurityCoordinator {
    /// Security policies
    pub policies: Arc<RwLock<Vec<DMSCSecurityPolicy>>>,
    /// Security enforcement engine
    pub enforcement_engine: Arc<DMSCSecurityEnforcementEngine>,
    /// Security event monitor
    pub event_monitor: Arc<DMSCSecurityEventMonitor>,
}

/// Security policy structure.
pub struct DMSCSecurityPolicy {
    /// Policy identifier
    pub policy_id: String,
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy enabled
    pub enabled: bool,
    /// Policy priority
    pub priority: u32,
    /// Policy version
    pub version: String,
}

/// Security enforcement engine structure.
pub struct DMSCSecurityEnforcementEngine {
    /// Enforcement rules
    pub rules: Arc<RwLock<HashMap<String, DMSCEnforcementRule>>>,
    /// Enforcement actions
    pub actions: Arc<RwLock<Vec<DMSCEnforcementAction>>>,
    /// Enforcement statistics
    pub stats: Arc<RwLock<DMSCEnforcementStats>>,
}

/// Enforcement rule structure.
#[derive(Debug, Clone)]
pub struct DMSCEnforcementRule {
    /// Rule identifier
    pub rule_id: String,
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: DMSCEnforcementCondition,
    /// Rule action
    pub action: DMSCEnforcementAction,
    /// Rule priority
    pub priority: u32,
    /// Rule status
    pub status: DMSCEnforcementRuleStatus,
}

/// Enforcement condition enumeration.
#[derive(Debug, Clone)]
pub enum DMSCEnforcementCondition {
    /// Protocol condition
    Protocol(DMSCProtocolType),
    /// Security level condition
    SecurityLevel(super::super::DMSCSecurityLevel),
    /// Device condition
    Device(DMSCDeviceType),
    /// Threat condition
    Threat(DMSCThreatCondition),
    /// Custom condition
    Custom(String),
}

/// Threat condition structure.
#[derive(Debug, Clone)]
pub struct DMSCThreatCondition {
    /// Threat level
    pub threat_level: DMSCThreatLevel,
    /// Threat type
    pub threat_type: DMSCThreatType,
    /// Confidence level
    pub confidence: f32,
}

/// Threat level enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCThreatLevel {
    /// Normal threat level
    Normal,
    /// Elevated threat level
    Elevated,
    /// High threat level
    High,
    /// Critical threat level
    Critical,
}

/// Threat type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCThreatType {
    /// Malware threat
    Malware,
    /// Intrusion threat
    Intrusion,
    /// Data breach threat
    DataBreach,
    /// Insider threat
    Insider,
    /// Advanced persistent threat
    APT,
}

/// Enforcement action enumeration.
#[derive(Debug, Clone)]
pub enum DMSCEnforcementAction {
    /// Allow action
    Allow,
    /// Deny action
    Deny,
    /// Log action
    Log,
    /// Alert action
    Alert,
    /// Quarantine action
    Quarantine,
    /// Block action
    Block,
    /// Custom action
    Custom(String),
}

/// Enforcement rule status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCEnforcementRuleStatus {
    /// Rule is draft
    Draft,
    /// Rule is active
    Active,
    /// Rule is suspended
    Suspended,
    /// Rule is retired
    Retired,
}

/// Enforcement statistics structure.
#[derive(Debug, Default)]
pub struct DMSCEnforcementStats {
    /// Total enforcement checks
    pub total_checks: u64,
    /// Allowed actions
    pub allowed_actions: u64,
    /// Denied actions
    pub denied_actions: u64,
    /// Quarantined actions
    pub quarantined_actions: u64,
    /// Average enforcement time
    pub avg_enforcement_time_ms: u64,
}

/// Security event monitor structure.
pub struct DMSCSecurityEventMonitor {
    /// Security events
    pub events: Arc<RwLock<Vec<DMSCSecurityEvent>>>,
    /// Event subscribers
    pub subscribers: Arc<RwLock<Vec<mpsc::Sender<DMSCSecurityEvent>>>>,
    /// Event statistics
    pub stats: Arc<RwLock<DMSCSecurityEventStats>>,
}

/// Security event structure.
#[derive(Debug, Clone)]
pub struct DMSCSecurityEvent {
    /// Event identifier
    pub event_id: String,
    /// Event type
    pub event_type: DMSCSecurityEventType,
    /// Event severity
    pub severity: DMSCSecurityEventSeverity,
    /// Event description
    pub description: String,
    /// Affected systems
    pub affected_systems: Vec<String>,
    /// Event time
    pub event_time: Instant,
    /// Event data
    pub event_data: HashMap<String, String>,
}

/// Security event type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCSecurityEventType {
    /// Policy violation
    PolicyViolation,
    /// Threat detection
    ThreatDetection,
    /// Authentication failure
    AuthenticationFailure,
    /// Authorization failure
    AuthorizationFailure,
    /// Encryption failure
    EncryptionFailure,
    /// Protocol violation
    ProtocolViolation,
}

/// Security event severity enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCSecurityEventSeverity {
    /// Information severity
    Information,
    /// Warning severity
    Warning,
    /// Error severity
    Error,
    /// Critical severity
    Critical,
}

/// Security event statistics structure.
#[derive(Debug, Default)]
pub struct DMSCSecurityEventStats {
    /// Total events
    pub total_events: u64,
    /// Events by type
    pub events_by_type: HashMap<DMSCSecurityEventType, u64>,
    /// Events by severity
    pub events_by_severity: HashMap<DMSCSecurityEventSeverity, u64>,
    /// Average event processing time
    pub avg_event_processing_time_ms: u64,
}

impl DMSCSecurityCoordinator {
    /// Enforce cross-protocol security.
    pub async fn enforce_cross_protocol_security(
        &self,
        source_protocol: DMSCProtocolType,
        target_protocol: DMSCProtocolType,
        message: &[u8],
    ) -> DMSCResult<()> {
        log::debug!("Enforcing cross-protocol security: {:?} -> {:?}, message size: {} bytes", 
               source_protocol, target_protocol, message.len());
        
        // Check if protocols are compatible for cross-protocol communication
        let compatible_pairs = vec![
            (DMSCProtocolType::Global, DMSCProtocolType::Private),
            (DMSCProtocolType::Private, DMSCProtocolType::Global),
            (DMSCProtocolType::Global, DMSCProtocolType::Hybrid),
            (DMSCProtocolType::Hybrid, DMSCProtocolType::Global),
            (DMSCProtocolType::Private, DMSCProtocolType::Hybrid),
            (DMSCProtocolType::Hybrid, DMSCProtocolType::Private),
        ];
        
        if !compatible_pairs.contains(&(source_protocol, target_protocol)) {
            log::error!("Incompatible protocol pair detected: {:?} -> {:?}", source_protocol, target_protocol);
            return Err(DMSCError::SecurityViolation(format!(
                "Incompatible protocol pair: {:?} -> {:?}",
                source_protocol, target_protocol
            )));
        }
        
        // Validate message size limits
        const MAX_MESSAGE_SIZE: usize = 10 * 1024 * 1024; // 10MB
        if message.len() > MAX_MESSAGE_SIZE {
            log::error!("Message size {} exceeds maximum allowed size {}", message.len(), MAX_MESSAGE_SIZE);
            return Err(DMSCError::SecurityViolation(format!(
                "Message size {} exceeds maximum allowed size {}",
                message.len(), MAX_MESSAGE_SIZE
            )));
        }
        
        // Enhanced message content validation - check for potential injection patterns and malicious content
        let message_str = String::from_utf8_lossy(message);
        let dangerous_patterns = vec![
            "<script>", "</script>", "javascript:", "data:",
            "<?php", "<%", "${", "{", "eval(", "exec(",
            "onload=", "onerror=", "onclick=", "onmouseover=",
            "vbscript:", "mocha:", "livescript:", "ms-its:",
        ];
        
        for pattern in dangerous_patterns {
            if message_str.to_lowercase().contains(pattern) {
                log::error!("Potentially dangerous content detected: {}", pattern);
                return Err(DMSCError::SecurityViolation(format!(
                    "Potentially dangerous content detected: {}",
                    pattern
                )));
            }
        }
        
        // Additional validation: check for binary content that might be executable
        if message.len() > 2 {
            // Check for common executable file signatures
            let executable_signatures = vec![
                b"\x4D\x5A", // MZ (DOS/Windows executable)
                b"\x7F\x45\x4C\x46", // ELF (Unix executable)
                b"\xFE\xED\xFA", // Mach-O (macOS executable)
                b"\xCA\xFE\xBA\xBE", // Mach-O universal binary
            ];
            
            for signature in executable_signatures {
                if message.starts_with(signature) {
                    log::error!("Executable file signature detected in message");
                    return Err(DMSCError::SecurityViolation(
                        "Executable content detected in message".to_string()
                    ));
                }
            }
        }
        
        // Validate protocol-specific security requirements with enhanced rules
        match (source_protocol, target_protocol) {
            (DMSCProtocolType::Global, DMSCProtocolType::Private) => {
                // Global to Private requires additional validation - strictest rules
                log::info!("Applying Global->Private security validation");
                
                if message.len() < 10 {
                    log::error!("Global to Private message too small: {} bytes (minimum: 10)", message.len());
                    return Err(DMSCError::SecurityViolation(
                        "Global to Private messages must be at least 10 bytes".to_string()
                    ));
                }
                
                // Additional validation for Global->Private: check for sensitive data patterns
                let sensitive_patterns = vec![
                    "password", "secret", "key", "token", "credential",
                    "ssn", "social security", "credit card", "bank account",
                ];
                
                for pattern in sensitive_patterns {
                    if message_str.to_lowercase().contains(pattern) {
                        log::warn!("Potential sensitive data detected in Global->Private message: {}", pattern);
                        // Note: This is a warning, not an error - sensitive data might be legitimate
                    }
                }
            },
            (DMSCProtocolType::Private, DMSCProtocolType::Global) => {
                // Private to Global requires sanitization check - prevent data leakage
                log::info!("Applying Private->Global security validation");
                
                let private_prefixes = vec!["private:", "internal:", "confidential:", "restricted:"];
                for prefix in private_prefixes {
                    if message_str.contains(prefix) {
                        log::error!("Private data prefix '{}' detected in Private->Global message", prefix);
                        return Err(DMSCError::SecurityViolation(
                            format!("Private to Global messages cannot contain '{}' prefixes", prefix)
                        ));
                    }
                }
                
                // Additional check: prevent potential data exfiltration patterns
                if message.len() > 1024 * 1024 { // 1MB threshold for large data
                    log::warn!("Large data transfer detected in Private->Global message: {} bytes", message.len());
                }
            },
            (DMSCProtocolType::Hybrid, _) | (_, DMSCProtocolType::Hybrid) => {
                // Hybrid protocol combinations require balanced validation
                log::info!("Applying Hybrid protocol security validation");
                
                // Hybrid protocols should not carry overly sensitive data
                if message.len() > 5 * 1024 * 1024 { // 5MB limit for hybrid
                    log::error!("Hybrid protocol message too large: {} bytes (maximum: 5MB)", message.len());
                    return Err(DMSCError::SecurityViolation(
                        "Hybrid protocol messages cannot exceed 5MB".to_string()
                    ));
                }
            },
            _ => {
                // Other combinations use standard validation (already handled above)
                log::debug!("Applying standard security validation for protocol combination");
            }
        }
        
        log::info!("Cross-protocol security validation passed for {:?} -> {:?}", source_protocol, target_protocol);
        Ok(())
    }
}
