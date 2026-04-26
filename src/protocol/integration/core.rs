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
use std::time::{Duration, Instant};
use log::{info, warn, debug, error};

use crate::core::{RiResult, RiError, RiServiceContext};
use crate::hooks::{RiHookKind, RiModulePhase};
use crate::protocol::global_state::RiSystemStatus;
use super::super::{RiProtocolType, RiProtocol, RiProtocolConnection, RiProtocolAdapter, 
                   RiGlobalStateManager, RiStateUpdate, RiStateCategory, RiSecurityLevel,
                   RiProtocolStrategy, RiSecurityContext, RiPerformanceContext};
use super::config::{RiIntegrationConfig};
use super::connection::{RiConnectionCoordinator, RiCrossProtocolConnection, RiCrossProtocolConnectionState, 
                      RiConnectionRoutingTable};
use super::security::{RiSecurityCoordinator};
use super::performance::{RiPerformanceCoordinator, RiPerformanceMetrics, RiCrossProtocolMetrics, 
                        RiSystemPerformanceMetrics};
use super::events::{RiIntegrationEventBus, RiIntegrationEvent, RiIntegrationEventType, 
                   RiIntegrationStats};

#[derive(Debug, Clone)]
pub enum RiExternalControlAction {
    TriggerHook {
        hook: RiHookKind,
        module: Option<String>,
        phase: Option<RiModulePhase>,
    },
    UpdateState(RiStateUpdate),
    SetGlobalSystemStatus(RiSystemStatus),
}

#[derive(Debug, Clone)]
pub enum RiExternalControlResult {
    HookTriggered,
    StateUpdated,
}

pub struct RiControlCenter {
    state_manager: Arc<RiGlobalStateManager>,
    service_context: RiServiceContext,
}

impl RiControlCenter {
    pub fn new(state_manager: Arc<RiGlobalStateManager>, service_context: RiServiceContext) -> Self {
        RiControlCenter {
            state_manager,
            service_context,
        }
    }

    pub async fn handle_action(
        &self,
        action: RiExternalControlAction,
    ) -> RiResult<RiExternalControlResult> {
        match action {
            RiExternalControlAction::TriggerHook { hook, module, phase } => {
                let hooks = self.service_context.hooks();
                hooks.emit_with(&hook, &self.service_context, module.as_deref(), phase)?;
                Ok(RiExternalControlResult::HookTriggered)
            }
            RiExternalControlAction::UpdateState(update) => {
                self.state_manager.update_state(update).await?;
                Ok(RiExternalControlResult::StateUpdated)
            }
            RiExternalControlAction::SetGlobalSystemStatus(system_status) => {
                let global_state = self.state_manager.get_global_state().await?;
                let update = RiStateUpdate::Global {
                    system_status,
                    global_config: global_state.global_config,
                    active_protocols: global_state.active_protocols,
                };
                self.state_manager.update_state(update).await?;
                Ok(RiExternalControlResult::StateUpdated)
            }
        }
    }
}

/// Global system integration coordinator.
pub struct RiGlobalSystemIntegration {
    /// Integration configuration
    config: Arc<tokio::sync::RwLock<RiIntegrationConfig>>,
    /// Protocol adapter for unified protocol interface
    protocol_adapter: Arc<RiProtocolAdapter>,
    /// Global state manager for state coordination
    state_manager: Arc<RiGlobalStateManager>,
    /// Protocol registry
    protocol_registry: Arc<tokio::sync::RwLock<FxHashMap<RiProtocolType, Arc<dyn RiProtocol>>>>,
    /// Connection coordinator
    connection_coordinator: Arc<RiConnectionCoordinator>,
    /// Security coordinator
    security_coordinator: Arc<RiSecurityCoordinator>,
    /// Performance coordinator
    performance_coordinator: Arc<RiPerformanceCoordinator>,
    /// Integration event bus
    event_bus: Arc<RiIntegrationEventBus>,
    /// Integration statistics
    stats: Arc<tokio::sync::RwLock<RiIntegrationStats>>,
    /// Initialization status
    initialized: Arc<tokio::sync::RwLock<bool>>,
}

impl RiGlobalSystemIntegration {
    /// Create a new global system integration.
    pub fn new(config: RiIntegrationConfig) -> Self {
        let protocol_adapter = Arc::new(RiProtocolAdapter::new());
        let state_manager = Arc::new(RiGlobalStateManager::new());
        
        let connection_coordinator = Arc::new(RiConnectionCoordinator {
            connections: Arc::new(tokio::sync::RwLock::new(FxHashMap::default())),
            routing_table: Arc::new(tokio::sync::RwLock::new(RiConnectionRoutingTable {
                entries: FxHashMap::default(),
                default_protocol: RiProtocolType::Global,
                routing_policies: vec![],
            })),
            health_monitor: Arc::new(crate::protocol::integration::connection::RiConnectionHealthMonitor {
                health_results: Arc::new(tokio::sync::RwLock::new(FxHashMap::default())),
                config: Arc::new(crate::protocol::integration::connection::RiHealthCheckConfig {
                    check_interval: Duration::from_secs(30),
                    timeout: Duration::from_secs(5),
                    retry_attempts: 3,
                    healthy_threshold: 2,
                    unhealthy_threshold: 3,
                }),
            }),
        });
        
        let security_coordinator = Arc::new(RiSecurityCoordinator {
            policies: Arc::new(tokio::sync::RwLock::new(vec![])),
            enforcement_engine: Arc::new(crate::protocol::integration::security::RiSecurityEnforcementEngine {
                rules: Arc::new(tokio::sync::RwLock::new(FxHashMap::default())),
                actions: Arc::new(tokio::sync::RwLock::new(vec![])),
                stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::security::RiEnforcementStats::default())),
            }),
            event_monitor: Arc::new(crate::protocol::integration::security::RiSecurityEventMonitor {
                events: Arc::new(tokio::sync::RwLock::new(vec![])),
                subscribers: Arc::new(tokio::sync::RwLock::new(vec![])),
                stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::security::RiSecurityEventStats::default())),
            }),
        });
        
        let performance_coordinator = Arc::new(RiPerformanceCoordinator {
            metrics: Arc::new(tokio::sync::RwLock::new(RiPerformanceMetrics {
                protocol_metrics: FxHashMap::default(),
                cross_protocol_metrics: RiCrossProtocolMetrics {
                    cross_protocol_latency: Duration::from_millis(0),
                    protocol_switching_time: Duration::from_millis(0),
                    state_sync_time: Duration::from_millis(0),
                    message_routing_efficiency: 1.0,
                },
                system_metrics: RiSystemPerformanceMetrics {
                    cpu_utilization: 0.0,
                    memory_utilization: 0.0,
                    network_utilization: 0.0,
                    disk_utilization: 0.0,
                },
                last_update: Instant::now(),
            })),
            optimizations: Arc::new(tokio::sync::RwLock::new(vec![])),
            monitor: Arc::new(crate::protocol::integration::performance::RiPerformanceMonitor {
                config: Arc::new(crate::protocol::integration::performance::RiPerformanceMonitoringConfig {
                    monitoring_interval: Duration::from_secs(60),
                    thresholds: crate::protocol::integration::performance::RiPerformanceThresholds {
                        max_latency: Duration::from_millis(1000),
                        min_throughput: 1000000, // 1MB/s
                        max_error_rate: 0.05, // 5%
                        max_cpu_utilization: 0.8, // 80%
                        max_memory_utilization: 0.8, // 80%
                    },
                    alert_config: crate::protocol::integration::performance::RiPerformanceAlertConfig {
                        alert_enabled: true,
                        alert_severity_levels: vec![crate::protocol::integration::performance::RiAlertSeverityLevel::Warning, 
                                                   crate::protocol::integration::performance::RiAlertSeverityLevel::Error, 
                                                   crate::protocol::integration::performance::RiAlertSeverityLevel::Critical],
                        alert_destinations: vec!["console".to_string(), "log".to_string()],
                    },
                }),
                results: Arc::new(tokio::sync::RwLock::new(vec![])),
                alerts: Arc::new(tokio::sync::RwLock::new(vec![])),
            }),
        });
        
        let event_bus = Arc::new(RiIntegrationEventBus {
            subscribers: Arc::new(tokio::sync::RwLock::new(FxHashMap::default())),
            stats: Arc::new(tokio::sync::RwLock::new(crate::protocol::integration::events::RiIntegrationEventStats::default())),
        });
        
        Self {
            config: Arc::new(tokio::sync::RwLock::new(config)),
            protocol_adapter,
            state_manager,
            protocol_registry: Arc::new(tokio::sync::RwLock::new(FxHashMap::default())),
            connection_coordinator,
            security_coordinator,
            performance_coordinator,
            event_bus,
            stats: Arc::new(tokio::sync::RwLock::new(RiIntegrationStats::default())),
            initialized: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }
    
    /// Initialize the global system integration.
    pub async fn initialize(&self) -> RiResult<()> {
        if *self.initialized.read().await {
            return Ok(());
        }
        
        // Initialize protocol adapter
        let security_context = RiSecurityContext {
            required_security_level: RiSecurityLevel::Standard,
            threat_level: super::super::adapter::RiThreatLevel::Normal,
            data_classification: super::super::adapter::RiDataClassification::Internal,
            network_environment: super::super::adapter::RiNetworkEnvironment::Trusted,
            compliance_requirements: vec![],
        };
        
        let strategy = RiProtocolStrategy::SecurityBased(security_context);
        let mut adapter = self.protocol_adapter.clone();
        adapter.initialize(strategy).await?;
        
        // Initialize state manager
        self.state_manager.initialize().await?;
        
        *self.initialized.write().await = true;
        Ok(())
    }
    
    /// Register a protocol.
    pub async fn register_protocol(&self, protocol_type: RiProtocolType) -> RiResult<()> {
        if !*self.initialized.read().await {
            return Err(RiError::InvalidState("Integration not initialized".to_string()));
        }
        
        // Create protocol instance based on type
        let protocol: Box<dyn RiProtocol> = match protocol_type {
            RiProtocolType::Global => {
                Box::new(super::super::global::RiGlobalProtocol::new())
            }
            RiProtocolType::Private => {
                Box::new(super::super::private::RiPrivateProtocol::new(super::super::private::RiPrivateProtocolConfig::default()))
            }
        };
        
        // Register with protocol adapter
        self.protocol_adapter.register_protocol(protocol_type, protocol).await?;
        
        // Update protocol registry
        self.protocol_registry.write().await.insert(protocol_type, Arc::new(protocol));
        
        // Publish event
        self.publish_event(RiIntegrationEventType::ProtocolRegistered, FxHashMap::default()).await?;
        
        Ok(())
    }
    
    /// Start protocol coordination.
    pub async fn start_coordination(&self) -> RiResult<()> {
        if !*self.initialized.read().await {
            return Err(RiError::InvalidState("Integration not initialized".to_string()));
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
        strategy: RiProtocolStrategy,
    ) -> RiResult<RiProtocolType> {
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
        source_protocol: RiProtocolType,
        target_protocol: RiProtocolType,
        message: &[u8],
    ) -> RiResult<Vec<u8>> {
        let start_time = Instant::now();
        
        // Update statistics
        self.stats.write().await.total_cross_protocol_messages += 1;
        
        // Validate protocols
        if source_protocol == target_protocol {
            return Err(RiError::InvalidInput("Source and target protocols cannot be the same".to_string()));
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
        source_protocol: RiProtocolType,
        target_protocol: RiProtocolType,
        message: &[u8],
    ) -> RiResult<Vec<u8>> {
        // Create cross-protocol connection if needed
        let connection_id = format!("cross-{}-{}-{}", source_protocol as u8, target_protocol as u8, target_device);
        
        // Check if connection exists
        let mut connections = self.connection_coordinator.connections.write().await;
        if !connections.contains_key(&connection_id) {
            // Create new cross-protocol connection
            let connection = RiCrossProtocolConnection {
                connection_id: connection_id.clone(),
                source_protocol,
                target_protocol,
                target_device: target_device.to_string(),
                state: RiCrossProtocolConnectionState::Initializing,
                metadata: FxHashMap::default(),
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
            connection.state = RiCrossProtocolConnectionState::Active;
            connection.last_activity = Instant::now();
        }
        
        Ok(response)
    }
    
    /// Start connection health monitoring.
    async fn start_connection_health_monitoring(&self) -> RiResult<()> {
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
                let mut to_remove = Vec::with_capacity(4);
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
    async fn start_state_synchronization(&self) -> RiResult<()> {
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
    async fn start_performance_monitoring(&self) -> RiResult<()> {
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
                if let Err(e) = event_bus.publish_event(RiIntegrationEventType::PerformanceMetrics, event_data).await {
                    error!("Failed to publish performance metrics: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    /// Publish integration event.
    async fn publish_event(&self, event_type: RiIntegrationEventType, event_data: FxHashMap<String, String>) -> RiResult<()> {
        let event = RiIntegrationEvent {
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
    pub async fn get_stats(&self) -> RiIntegrationStats {
        *self.stats.read().await
    }
    
    /// Shutdown the global system integration.
    pub async fn shutdown(&mut self) -> RiResult<()> {
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


