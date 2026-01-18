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
use std::time::{Duration, Instant};
use log::{info, warn, debug, error};

use crate::core::{DMSCResult, DMSCError, DMSCServiceContext};
use crate::hooks::{DMSCHookKind, DMSCModulePhase};
use crate::protocol::global_state::DMSCSystemStatus;
use super::super::{DMSCProtocolType, DMSCProtocol, DMSCProtocolConnection, DMSCProtocolAdapter, 
                   DMSCGlobalStateManager, DMSCStateUpdate, DMSCStateCategory, DMSCSecurityLevel,
                   DMSCProtocolStrategy, DMSCSecurityContext, DMSCPerformanceContext};
use super::config::{DMSCIntegrationConfig};
use super::connection::{DMSCConnectionCoordinator, DMSCCrossProtocolConnection, DMSCCrossProtocolConnectionState, 
                      DMSCConnectionRoutingTable};
use super::security::{DMSCSecurityCoordinator};
use super::performance::{DMSCPerformanceCoordinator, DMSCPerformanceMetrics, DMSCCrossProtocolMetrics, 
                        DMSCSystemPerformanceMetrics};
use super::events::{DMSCIntegrationEventBus, DMSCIntegrationEvent, DMSCIntegrationEventType, 
                   DMSCIntegrationStats};

#[derive(Debug, Clone)]
pub enum DMSCExternalControlAction {
    TriggerHook {
        hook: DMSCHookKind,
        module: Option<String>,
        phase: Option<DMSCModulePhase>,
    },
    UpdateState(DMSCStateUpdate),
    SetGlobalSystemStatus(DMSCSystemStatus),
}

#[derive(Debug, Clone)]
pub enum DMSCExternalControlResult {
    HookTriggered,
    StateUpdated,
}

pub struct DMSCControlCenter {
    state_manager: Arc<DMSCGlobalStateManager>,
    service_context: DMSCServiceContext,
}

impl DMSCControlCenter {
    pub fn new(state_manager: Arc<DMSCGlobalStateManager>, service_context: DMSCServiceContext) -> Self {
        DMSCControlCenter {
            state_manager,
            service_context,
        }
    }

    pub async fn handle_action(
        &self,
        action: DMSCExternalControlAction,
    ) -> DMSCResult<DMSCExternalControlResult> {
        match action {
            DMSCExternalControlAction::TriggerHook { hook, module, phase } => {
                let hooks = self.service_context.hooks();
                hooks.emit_with(&hook, &self.service_context, module.as_deref(), phase)?;
                Ok(DMSCExternalControlResult::HookTriggered)
            }
            DMSCExternalControlAction::UpdateState(update) => {
                self.state_manager.update_state(update).await?;
                Ok(DMSCExternalControlResult::StateUpdated)
            }
            DMSCExternalControlAction::SetGlobalSystemStatus(system_status) => {
                let global_state = self.state_manager.get_global_state().await?;
                let update = DMSCStateUpdate::Global {
                    system_status,
                    global_config: global_state.global_config,
                    active_protocols: global_state.active_protocols,
                };
                self.state_manager.update_state(update).await?;
                Ok(DMSCExternalControlResult::StateUpdated)
            }
        }
    }
}

/// Global system integration coordinator.
pub struct DMSCGlobalSystemIntegration {
    /// Integration configuration
    config: Arc<tokio::sync::RwLock<DMSCIntegrationConfig>>,
    /// Protocol adapter for unified protocol interface
    protocol_adapter: Arc<DMSCProtocolAdapter>,
    /// Global state manager for state coordination
    state_manager: Arc<DMSCGlobalStateManager>,
    /// Protocol registry
    protocol_registry: Arc<tokio::sync::RwLock<HashMap<DMSCProtocolType, Arc<dyn DMSCProtocol>>>>,
    /// Connection coordinator
    connection_coordinator: Arc<DMSCConnectionCoordinator>,
    /// Security coordinator
    security_coordinator: Arc<DMSCSecurityCoordinator>,
    /// Performance coordinator
    performance_coordinator: Arc<DMSCPerformanceCoordinator>,
    /// Integration event bus
    event_bus: Arc<DMSCIntegrationEventBus>,
    /// Integration statistics
    stats: Arc<tokio::sync::RwLock<DMSCIntegrationStats>>,
    /// Initialization status
    initialized: Arc<tokio::sync::RwLock<bool>>,
}

impl DMSCGlobalSystemIntegration {
    /// Create a new global system integration.
    pub fn new(config: DMSCIntegrationConfig) -> Self {
        let protocol_adapter = Arc::new(DMSCProtocolAdapter::new());
        let state_manager = Arc::new(DMSCGlobalStateManager::new());
        
        let connection_coordinator = Arc::new(DMSCConnectionCoordinator {
            connections: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            routing_table: Arc::new(tokio::sync::RwLock::new(DMSCConnectionRoutingTable {
                entries: HashMap::new(),
                default_protocol: DMSCProtocolType::Global,
                routing_policies: vec![],
            })),
            health_monitor: Arc::new(crate::protocol::integration::connection::DMSCConnectionHealthMonitor {
                health_results: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
                config: Arc::new(crate::protocol::integration::connection::DMSCHealthCheckConfig {
                    check_interval: Duration::from_secs(30),
                    timeout: Duration::from_secs(5),
                    retry_attempts: 3,
                    healthy_threshold: 2,
                    unhealthy_threshold: 3,
                }),
            }),
        });
        
        let security_coordinator = Arc::new(DMSCSecurityCoordinator {
            policies: Arc::new(tokio::sync::RwLock::new(vec![])),
            enforcement_engine: Arc::new(crate::protocol::integration::security::DMSCSecurityEnforcementEngine {
                rules: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
                actions: Arc::new(tokio::sync::RwLock::new(vec![])),
                stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::security::DMSCEnforcementStats::default())),
            }),
            event_monitor: Arc::new(crate::protocol::integration::security::DMSCSecurityEventMonitor {
                events: Arc::new(tokio::sync::RwLock::new(vec![])),
                subscribers: Arc::new(tokio::sync::RwLock::new(vec![])),
                stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::security::DMSCSecurityEventStats::default())),
            }),
        });
        
        let performance_coordinator = Arc::new(DMSCPerformanceCoordinator {
            metrics: Arc::new(tokio::sync::RwLock::new(DMSCPerformanceMetrics {
                protocol_metrics: HashMap::new(),
                cross_protocol_metrics: DMSCCrossProtocolMetrics {
                    cross_protocol_latency: Duration::from_millis(0),
                    protocol_switching_time: Duration::from_millis(0),
                    state_sync_time: Duration::from_millis(0),
                    message_routing_efficiency: 1.0,
                },
                system_metrics: DMSCSystemPerformanceMetrics {
                    cpu_utilization: 0.0,
                    memory_utilization: 0.0,
                    network_utilization: 0.0,
                    disk_utilization: 0.0,
                },
                last_update: Instant::now(),
            })),
            optimizations: Arc::new(tokio::sync::RwLock::new(vec![])),
            monitor: Arc::new(crate::protocol::integration::performance::DMSCPerformanceMonitor {
                config: Arc::new(crate::protocol::integration::performance::DMSCPerformanceMonitoringConfig {
                    monitoring_interval: Duration::from_secs(60),
                    thresholds: crate::protocol::integration::performance::DMSCPerformanceThresholds {
                        max_latency: Duration::from_millis(1000),
                        min_throughput: 1000000, // 1MB/s
                        max_error_rate: 0.05, // 5%
                        max_cpu_utilization: 0.8, // 80%
                        max_memory_utilization: 0.8, // 80%
                    },
                    alert_config: crate::protocol::integration::performance::DMSCPerformanceAlertConfig {
                        alert_enabled: true,
                        alert_severity_levels: vec![crate::protocol::integration::performance::DMSCAlertSeverityLevel::Warning, 
                                                   crate::protocol::integration::performance::DMSCAlertSeverityLevel::Error, 
                                                   crate::protocol::integration::performance::DMSCAlertSeverityLevel::Critical],
                        alert_destinations: vec!["console".to_string(), "log".to_string()],
                    },
                }),
                results: Arc::new(tokio::sync::RwLock::new(vec![])),
                alerts: Arc::new(tokio::sync::RwLock::new(vec![])),
            }),
        });
        
        let event_bus = Arc::new(DMSCIntegrationEventBus {
            subscribers: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::events::DMSCIntegrationEventStats::default())),
        });
        
        Self {
            config: Arc::new(tokio::sync::RwLock::new(config)),
            protocol_adapter,
            state_manager,
            protocol_registry: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            connection_coordinator,
            security_coordinator,
            performance_coordinator,
            event_bus,
            stats: Arc::new(tokio::sync::RwLock::new(DMSCIntegrationStats::default())),
            initialized: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
    
    /// Initialize the global system integration.
    pub async fn initialize(&self) -> DMSCResult<()> {
        if *self.initialized.read().await {
            return Ok(());
        }
        
        // Initialize protocol adapter
        let security_context = DMSCSecurityContext {
            required_security_level: DMSCSecurityLevel::Standard,
            threat_level: super::super::adapter::DMSCThreatLevel::Normal,
            data_classification: super::super::adapter::DMSCDataClassification::Internal,
            network_environment: super::super::adapter::DMSCNetworkEnvironment::Trusted,
            compliance_requirements: vec![],
        };
        
        let strategy = DMSCProtocolStrategy::SecurityBased(security_context);
        let mut adapter = self.protocol_adapter.clone();
        adapter.initialize(strategy).await?;
        
        // Initialize state manager
        self.state_manager.initialize().await?;
        
        *self.initialized.write().await = true;
        Ok(())
    }
    
    /// Register a protocol.
    pub async fn register_protocol(&self, protocol_type: DMSCProtocolType) -> DMSCResult<()> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Integration not initialized".to_string()));
        }
        
        // Create protocol instance based on type
        let protocol: Box<dyn DMSCProtocol> = match protocol_type {
            DMSCProtocolType::Global => {
                Box::new(super::super::global::DMSCGlobalProtocol::new())
            }
            DMSCProtocolType::Private => {
                Box::new(super::super::private::DMSCPrivateProtocol::new(super::super::private::DMSCPrivateProtocolConfig::default()))
            }
        };
        
        // Register with protocol adapter
        self.protocol_adapter.register_protocol(protocol_type, protocol).await?;
        
        // Update protocol registry
        self.protocol_registry.write().await.insert(protocol_type, Arc::new(protocol));
        
        // Publish event
        self.publish_event(DMSCIntegrationEventType::ProtocolRegistered, HashMap::new()).await?;
        
        Ok(())
    }
    
    /// Start protocol coordination.
    pub async fn start_coordination(&self) -> DMSCResult<()> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Integration not initialized".to_string()));
        }
        
        let config = self.config.read().await;
        
        if config.enable_protocol_coordination {
            // Start connection health monitoring
            self.start_connection_health_monitoring().await?;
        }
        
        if config.enable_state_sync {
            // Start state synchronization
            self.start_state_synchronization().await?;
        }
        
        if config.performance_optimization {
            // Start performance monitoring
            self.start_performance_monitoring().await?;
        }
        
        Ok(())
    }
    
    /// Select optimal protocol for target device.
    pub async fn select_protocol_for_device(
        &self,
        target_device: &str,
        strategy: DMSCProtocolStrategy,
    ) -> DMSCResult<DMSCProtocolType> {
        // Check routing table first
        let routing_table = self.connection_coordinator.routing_table.read().await;
        if let Some(entry) = routing_table.entries.get(target_device) {
            // Check if preferred protocol is available
            let protocols = self.protocol_registry.read().await;
            if protocols.contains_key(&entry.preferred_protocol) {
                return Ok(entry.preferred_protocol);
            }
            
            // Check alternative protocols
            for alt_protocol in &entry.alternative_protocols {
                if protocols.contains_key(alt_protocol) {
                    return Ok(*alt_protocol);
                }
            }
        }
        
        // Use protocol adapter to select optimal protocol
        self.protocol_adapter.select_optimal_protocol(&strategy).await
    }
    
    /// Send cross-protocol message.
    pub async fn send_cross_protocol_message(
        &self,
        target_device: &str,
        source_protocol: DMSCProtocolType,
        target_protocol: DMSCProtocolType,
        message: &[u8],
    ) -> DMSCResult<Vec<u8>> {
        let start_time = Instant::now();
        
        // Update statistics
        self.stats.write().await.total_cross_protocol_messages += 1;
        
        // Validate protocols
        if source_protocol == target_protocol {
            return Err(DMSCError::InvalidInput("Source and target protocols cannot be the same".to_string()));
        }
        
        // Check security enforcement
        self.security_coordinator.enforce_cross_protocol_security(
            source_protocol, target_protocol, message
        ).await?;
        
        // Route message through appropriate protocol
        let response = self.route_cross_protocol_message(
            target_device, source_protocol, target_protocol, message
        ).await?;
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.successful_cross_protocol_messages += 1;
        let latency = start_time.elapsed().as_millis() as u64;
        stats.avg_cross_protocol_latency_ms = (stats.avg_cross_protocol_latency_ms + latency) / 2;
        
        Ok(response)
    }
    
    /// Route cross-protocol message.
    async fn route_cross_protocol_message(
        &self,
        target_device: &str,
        source_protocol: DMSCProtocolType,
        target_protocol: DMSCProtocolType,
        message: &[u8],
    ) -> DMSCResult<Vec<u8>> {
        // Create cross-protocol connection if needed
        let connection_id = format!("cross-{}-{}-{}", source_protocol as u8, target_protocol as u8, target_device);
        
        // Check if connection exists
        let mut connections = self.connection_coordinator.connections.write().await;
        if !connections.contains_key(&connection_id) {
            // Create new cross-protocol connection
            let connection = DMSCCrossProtocolConnection {
                connection_id: connection_id.clone(),
                source_protocol,
                target_protocol,
                target_device: target_device.to_string(),
                state: DMSCCrossProtocolConnectionState::Initializing,
                metadata: HashMap::new(),
                established_at: Instant::now(),
                last_activity: Instant::now(),
            };
            
            connections.insert(connection_id.clone(), connection);
        }
        
        // Send message through protocol adapter
        let connection = self.protocol_adapter.connect(target_device).await?;
        let response = connection.send_message(message).await?;
        
        // Update connection state
        if let Some(connection) = connections.get_mut(&connection_id) {
            connection.state = DMSCCrossProtocolConnectionState::Active;
            connection.last_activity = Instant::now();
        }
        
        Ok(response)
    }
    
    /// Start connection health monitoring.
    async fn start_connection_health_monitoring(&self) -> DMSCResult<()> {
        let connections = Arc::clone(&self.connection_coordinator.connections);
        let config = self.config.read().await;
        let health_check_interval = config.health_check_interval;
        drop(config);
        
        // Start background task for health monitoring
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(health_check_interval);
            loop {
                interval.tick().await;
                
                let mut connections = connections.write().await;
                let now = Instant::now();
                
                // Check each connection for timeout
                let mut to_remove = Vec::new();
                for (connection_id, connection) in connections.iter() {
                    if now.duration_since(connection.last_activity) > Duration::from_secs(300) { // 5 minutes timeout
                        to_remove.push(connection_id.clone());
                    }
                }
                
                // Remove timed out connections
                for connection_id in to_remove {
                    connections.remove(&connection_id);
                }
            }
        });
        
        Ok(())
    }
    
    /// Start state synchronization.
    async fn start_state_synchronization(&self) -> DMSCResult<()> {
        let state_manager = Arc::clone(&self.state_manager);
        let config = self.config.read().await;
        let state_sync_interval = config.state_sync_interval;
        drop(config);
        
        // Start background task for state synchronization
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(state_sync_interval);
            loop {
                interval.tick().await;
                
                // Sync state across all protocols
                if let Err(e) = state_manager.sync_all_states().await {
                    error!("State synchronization error: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Start performance monitoring.
    async fn start_performance_monitoring(&self) -> DMSCResult<()> {
        let stats = Arc::clone(&self.stats);
        let event_bus = Arc::clone(&self.event_bus);
        
        // Start background task for performance monitoring
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60)); // 1 minute
            loop {
                interval.tick().await;
                
                let stats = stats.read().await;
                let event_data = HashMap::from([
                    ("total_cross_protocol_messages".to_string(), stats.total_cross_protocol_messages.to_string()),
                    ("successful_cross_protocol_messages".to_string(), stats.successful_cross_protocol_messages.to_string()),
                    ("avg_cross_protocol_latency_ms".to_string(), stats.avg_cross_protocol_latency_ms.to_string()),
                ]);
                drop(stats);
                
                // Publish performance metrics event
                if let Err(e) = event_bus.publish_event(DMSCIntegrationEventType::PerformanceMetrics, event_data).await {
                    error!("Failed to publish performance metrics: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Publish integration event.
    async fn publish_event(&self, event_type: DMSCIntegrationEventType, event_data: HashMap<String, String>) -> DMSCResult<()> {
        let event = DMSCIntegrationEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            event_data,
            event_timestamp: Instant::now(),
            event_source: "global-system-integration".to_string(),
        };
        
        // Update statistics
        self.event_bus.stats.write().await.total_events += 1;
        
        // Notify subscribers
        let subscribers = self.event_bus.subscribers.read().await;
        if let Some(subscribers) = subscribers.get(&event_type) {
            for subscriber in subscribers {
                let _ = subscriber.send(event.clone()).await;
            }
        }
        
        Ok(())
    }
    
    /// Get integration statistics.
    pub async fn get_stats(&self) -> DMSCIntegrationStats {
        *self.stats.read().await
    }
    
    /// Shutdown the global system integration.
    pub async fn shutdown(&mut self) -> DMSCResult<()> {
        // Shutdown protocol adapter
        let mut adapter = self.protocol_adapter.clone();
        adapter.shutdown().await?;
        
        // Shutdown state manager
        let mut state_manager = self.state_manager.clone();
        state_manager.shutdown().await?;
        
        *self.initialized.write().await = false;
        Ok(())
    }
}


