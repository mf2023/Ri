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

//! # Protocol Module Tests
//!
//! This module contains comprehensive tests for the DMSC protocol system,
//! covering protocol configuration, frame parsing, and protocol state management.
//!
//! ## Test Coverage
//!
//! - **DMSCProtocolConfig**: Tests for configuration creation with custom settings
//! - **DMSCProtocolFrameType**: Tests for frame type enum variants and conversions
//! - **DMSCProtocolState**: Tests for protocol state transitions and initialization
//! - **DMSCProtocolStats**: Tests for statistics tracking and cloning
//! - **DMSCProtocolHealth**: Tests for health status enum variants

use dmsc::protocol::{DMSCProtocolConfig, DMSCProtocolFrameType, DMSCProtocolState, DMSCProtocolStats, DMSCProtocolHealth};
use std::time::Duration;

#[test]
/// Tests DMSCProtocolConfig default creation.
fn test_protocol_config_default() {
    let config = DMSCProtocolConfig::default();
    assert_eq!(config.max_frame_size, 65536);
    assert_eq!(config.default_timeout_secs, 30);
    assert_eq!(config.heartbeat_interval_secs, 15);
    assert_eq!(config.max_retries, 3);
    assert_eq!(config.enable_compression, true);
}

#[test]
/// Tests DMSCProtocolConfig with custom settings.
fn test_protocol_config_custom() {
    let config = DMSCProtocolConfig::new()
        .max_frame_size(131072)
        .default_timeout_secs(60)
        .heartbeat_interval_secs(30)
        .max_retries(5)
        .enable_compression(false);

    assert_eq!(config.max_frame_size, 131072);
    assert_eq!(config.default_timeout_secs, 60);
    assert_eq!(config.heartbeat_interval_secs, 30);
    assert_eq!(config.max_retries, 5);
    assert_eq!(config.enable_compression, false);
}

#[test]
/// Tests DMSCProtocolFrameType enum variants.
fn test_protocol_frame_type_variants() {
    assert_eq!(DMSCProtocolFrameType::Data as u8, 0x01);
    assert_eq!(DMSCProtocolFrameType::Control as u8, 0x02);
    assert_eq!(DMSCProtocolFrameType::Auth as u8, 0x03);
    assert_eq!(DMSCProtocolFrameType::Heartbeat as u8, 0x04);
    assert_eq!(DMSCProtocolFrameType::Error as u8, 0x05);
}

#[test]
/// Tests DMSCProtocolState initialization.
fn test_protocol_state_initialization() {
    let state = DMSCProtocolState::new();
    assert!(!state.is_initialized());
    assert!(!state.is_connected());
    assert!(!state.is_authenticated());
}

#[test]
/// Tests DMSCProtocolState transitions.
fn test_protocol_state_transitions() {
    let mut state = DMSCProtocolState::new();

    state.set_initialized(true);
    assert!(state.is_initialized());

    state.set_connected(true);
    assert!(state.is_connected());

    state.set_authenticated(true);
    assert!(state.is_authenticated());

    state.set_connected(false);
    assert!(!state.is_connected());
    assert!(state.is_initialized());
}

#[test]
/// Tests DMSCProtocolStats creation and cloning.
fn test_protocol_stats() {
    let stats = DMSCProtocolStats::new();
    assert_eq!(stats.messages_sent(), 0);
    assert_eq!(stats.messages_received(), 0);
    assert_eq!(stats.bytes_sent(), 0);
    assert_eq!(stats.bytes_received(), 0);
    assert_eq!(stats.errors(), 0);

    let cloned = stats.clone();
    assert_eq!(cloned.messages_sent(), 0);
}

#[test]
/// Tests DMSCProtocolStats recording operations.
fn test_protocol_stats_recording() {
    let mut stats = DMSCProtocolStats::new();

    stats.record_sent(100);
    assert_eq!(stats.bytes_sent(), 100);
    assert_eq!(stats.messages_sent(), 1);

    stats.record_received(200);
    assert_eq!(stats.bytes_received(), 200);
    assert_eq!(stats.messages_received(), 1);

    stats.record_error();
    assert_eq!(stats.errors(), 1);
}

#[test]
/// Tests DMSCProtocolHealth enum variants.
fn test_protocol_health_variants() {
    assert_eq!(DMSCProtocolHealth::Healthy.to_string(), "healthy");
    assert_eq!(DMSCProtocolHealth::Degraded.to_string(), "degraded");
    assert_eq!(DMSCProtocolHealth::Unhealthy.to_string(), "unhealthy");
    assert_eq!(DMSCProtocolHealth::Unknown.to_string(), "unknown");
}

#[test]
/// Tests DMSCProtocolConfig builder pattern.
fn test_protocol_config_builder() {
    let config = DMSCProtocolConfig::new()
        .max_frame_size(1024)
        .default_timeout_secs(10)
        .heartbeat_interval_secs(5)
        .max_retries(2)
        .enable_compression(true);

    assert!(config.max_frame_size > 0);
    assert!(config.default_timeout_secs > 0);
    assert!(config.heartbeat_interval_secs > 0);
    assert!(config.max_retries > 0);
}
