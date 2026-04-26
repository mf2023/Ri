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

#![allow(non_snake_case)]

use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{RwLock, mpsc};
use uuid::Uuid;

use crate::core::{RiResult};

/// Integration event bus for event-driven coordination.
pub struct RiIntegrationEventBus {
    /// Event subscribers
    pub subscribers: Arc<RwLock<FxHashMap<RiIntegrationEventType, Vec<mpsc::Sender<RiIntegrationEvent>>>>>,
    /// Event statistics
    pub stats: Arc<RwLock<RiIntegrationEventStats>>,
}

/// Integration event structure.
#[derive(Debug, Clone)]
pub struct RiIntegrationEvent {
    /// Event identifier
    pub event_id: String,
    /// Event type
    pub event_type: RiIntegrationEventType,
    /// Event data
    pub event_data: FxHashMap<String, String>,
    /// Event timestamp
    pub event_timestamp: Instant,
    /// Event source
    pub event_source: String,
}

/// Integration event type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiIntegrationEventType {
    /// Protocol registered
    ProtocolRegistered,
    /// Protocol unregistered
    ProtocolUnregistered,
    /// Protocol switched
    ProtocolSwitched,
    /// Connection established
    ConnectionEstablished,
    /// Connection terminated
    ConnectionTerminated,
    /// State synchronized
    StateSynchronized,
    /// Security event
    SecurityEvent,
    /// Performance event
    PerformanceEvent,
    /// Error event
    ErrorEvent,
    /// Performance metrics
    PerformanceMetrics,
}

/// Integration event statistics structure.
#[derive(Debug, Default)]
pub struct RiIntegrationEventStats {
    /// Total events
    pub total_events: u64,
    /// Events by type
    pub events_by_type: FxHashMap<RiIntegrationEventType, u64>,
    /// Average event processing time
    pub avg_event_processing_time_ms: u64,
}

/// Integration statistics structure.
#[derive(Debug, Default)]
pub struct RiIntegrationStats {
    /// Total cross-protocol messages
    pub total_cross_protocol_messages: u64,
    /// Successful cross-protocol messages
    pub successful_cross_protocol_messages: u64,
    /// Failed cross-protocol messages
    pub failed_cross_protocol_messages: u64,
    /// Protocol switches
    pub protocol_switches: u64,
    /// Successful protocol switches
    pub successful_protocol_switches: u64,
    /// Failed protocol switches
    pub failed_protocol_switches: u64,
    /// State synchronizations
    pub state_synchronizations: u64,
    /// Average cross-protocol latency
    pub avg_cross_protocol_latency_ms: u64,
    /// Average protocol switching time
    pub avg_protocol_switching_time_ms: u64,
}

impl RiIntegrationEventBus {
    /// Publish an integration event.
    pub async fn publish_event(&self, event_type: RiIntegrationEventType, event_data: FxHashMap<String, String>) -> RiResult<()> {
        let event = RiIntegrationEvent {
            event_id: Uuid::new_v4().to_string(),
            event_type,
            event_data,
            event_timestamp: Instant::now(),
            event_source: "global-system-integration".to_string(),
        };
        
        // Update statistics
        self.stats.write().await.total_events += 1;
        
        // Notify subscribers
        let subscribers = self.subscribers.read().await;
        if let Some(subscribers) = subscribers.get(&event_type) {
            for subscriber in subscribers {
                let _ = subscriber.send(event.clone()).await;
            }
        }
        
        Ok(())
    }
}
