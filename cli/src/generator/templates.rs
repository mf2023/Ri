// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Code Templates Module
//!
//! This module provides predefined code templates for generating various
//! module types, middleware components, and configuration structures.
//!
//! # Features
//!
//! - **Module Templates**: Templates for cache, queue, gateway, auth, device, and observability modules
//! - **Middleware Template**: Standard middleware pattern template
//! - **Config Template**: Configuration struct generation template
//!
//! # Architecture
//!
//! Each template is designed to:
//! - Follow Ri framework conventions
//! - Include comprehensive documentation
//! - Support placeholder substitution
//! - Provide sensible defaults
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::generator::templates::{get_module_template, get_middleware_template, get_config_template};
//! use ric::generator::ModuleType;
//!
//! // Get a module template
//! let cache_template = get_module_template(ModuleType::Cache);
//!
//! // Get a middleware template
//! let middleware_template = get_middleware_template();
//!
//! // Get a config template
//! let config_template = get_config_template();
//! ```

use super::engine::ModuleType;

// =============================================================================
// Module Templates
// =============================================================================

/// Get the template for a specific module type
///
/// Returns a complete template string for the specified module type.
/// The template includes placeholders that should be replaced:
/// - `{{MODULE_NAME}}` - The module name in snake_case
/// - `{{MODULE_NAME_PASCAL}}` - The module name in PascalCase
/// - `{{MODULE_TYPE}}` - The module type identifier
///
/// # Arguments
///
/// * `module_type` - The type of module template to retrieve
///
/// # Returns
///
/// Returns a string containing the module template.
///
/// # Example
///
/// ```rust,ignore
/// use ric::generator::templates::get_module_template;
/// use ric::generator::ModuleType;
///
/// let template = get_module_template(ModuleType::Cache);
/// assert!(template.contains("{{MODULE_NAME}}"));
/// ```
pub fn get_module_template(module_type: ModuleType) -> String {
    match module_type {
        ModuleType::Cache => get_cache_module_template(),
        ModuleType::Queue => get_queue_module_template(),
        ModuleType::Gateway => get_gateway_module_template(),
        ModuleType::Auth => get_auth_module_template(),
        ModuleType::Device => get_device_module_template(),
        ModuleType::Observability => get_observability_module_template(),
    }
}

/// Get the cache module template
///
/// Returns a template for a distributed caching module with support for
/// Redis, Memcached, and in-memory caching.
///
/// # Returns
///
/// Returns a string containing the cache module template.
fn get_cache_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Cache Module
//!
//! This module provides distributed caching capabilities for the Ri framework.
//! It supports multiple cache backends including Redis, Memcached, and in-memory caching.
//!
//! # Features
//!
//! - **Multi-backend Support**: Redis, Memcached, and in-memory caching
//! - **Cache Invalidation**: TTL-based and event-driven invalidation
//! - **Connection Pooling**: Efficient connection management
//! - **Async Operations**: Full async/await support
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let cache = {{MODULE_NAME_PASCAL}}::new("redis://localhost:6379")?;
//! cache.set("key", "value", Some(3600)).await?;
//! let value = cache.get("key").await?;
//! ```

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

// =============================================================================
// Cache Backend Trait
// =============================================================================

/// Cache backend trait
///
/// Defines the interface for cache backend implementations.
/// All cache backends must implement this trait to provide
/// consistent caching operations.
#[async_trait]
pub trait CacheBackend: Send + Sync {
    /// Get a value from the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    ///
    /// # Returns
    ///
    /// Returns the cached value if it exists, or None if not found.
    async fn get<T: DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<T>>;

    /// Set a value in the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    /// * `value` - The value to cache
    /// * `ttl` - Optional time-to-live in seconds
    async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<u64>,
    ) -> Result<()>;

    /// Delete a value from the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to delete
    async fn delete(&self, key: &str) -> Result<bool>;

    /// Check if a key exists in the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to check
    async fn exists(&self, key: &str) -> Result<bool>;

    /// Clear all entries in the cache
    async fn clear(&self) -> Result<()>;
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
///
/// Contains all settings required to initialize and configure
/// the cache module.
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// Cache backend URL (e.g., redis://localhost:6379)
    pub backend_url: String,

    /// Connection pool size
    pub pool_size: usize,

    /// Default TTL for cache entries (in seconds)
    pub default_ttl: u64,

    /// Enable cache statistics collection
    pub enable_stats: bool,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            backend_url: "redis://localhost:6379".to_string(),
            pool_size: 10,
            default_ttl: 3600,
            enable_stats: true,
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main cache module struct
///
/// Provides a high-level interface for caching operations.
/// Supports multiple backends and provides automatic serialization.
pub struct {{MODULE_NAME_PASCAL}} {
    /// The underlying cache backend
    backend: Box<dyn CacheBackend>,

    /// Configuration settings
    config: {{MODULE_NAME_PASCAL}}Config,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the cache module
    ///
    /// # Returns
    ///
    /// Returns a new {{MODULE_NAME_PASCAL}} instance.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = {{MODULE_NAME_PASCAL}}Config::default();
    /// let cache = {{MODULE_NAME_PASCAL}}::new(config)?;
    /// ```
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        // Initialize the appropriate backend based on URL
        let backend = Self::create_backend(&config.backend_url)?;

        Ok(Self { backend, config })
    }

    /// Create the cache backend based on URL
    fn create_backend(url: &str) -> Result<Box<dyn CacheBackend>> {
        if url.starts_with("redis://") {
            // Redis backend initialization
            // TODO: Implement Redis backend
            unimplemented!("Redis backend not yet implemented")
        } else if url.starts_with("memcached://") {
            // Memcached backend initialization
            // TODO: Implement Memcached backend
            unimplemented!("Memcached backend not yet implemented")
        } else if url.starts_with("memory://") {
            // In-memory backend initialization
            // TODO: Implement in-memory backend
            unimplemented!("In-memory backend not yet implemented")
        } else {
            Err(anyhow::anyhow!(
                "Unsupported cache backend URL: {}",
                url
            ))
        }
    }

    /// Get a value from the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    ///
    /// # Returns
    ///
    /// Returns the cached value if it exists.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let value: Option<String> = cache.get("my_key").await?;
    /// ```
    pub async fn get<T: DeserializeOwned + Send + Sync>(&self, key: &str) -> Result<Option<T>> {
        self.backend.get(key).await
    }

    /// Set a value in the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key
    /// * `value` - The value to cache
    /// * `ttl` - Optional TTL (uses default if None)
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// cache.set("my_key", &"my_value", Some(60)).await?;
    /// ```
    pub async fn set<T: Serialize + Send + Sync>(
        &self,
        key: &str,
        value: &T,
        ttl: Option<u64>,
    ) -> Result<()> {
        let effective_ttl = ttl.unwrap_or(self.config.default_ttl);
        self.backend.set(key, value, Some(effective_ttl)).await
    }

    /// Delete a value from the cache
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to delete
    ///
    /// # Returns
    ///
    /// Returns true if the key was deleted, false if it didn't exist.
    pub async fn delete(&self, key: &str) -> Result<bool> {
        self.backend.delete(key).await
    }

    /// Check if a key exists
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to check
    pub async fn exists(&self, key: &str) -> Result<bool> {
        self.backend.exists(key).await
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> Result<()> {
        self.backend.clear().await
    }
}
"#.to_string()
}

/// Get the queue module template
///
/// Returns a template for a message queue module with support for
/// RabbitMQ, Kafka, and Redis-based queues.
///
/// # Returns
///
/// Returns a string containing the queue module template.
fn get_queue_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Queue Module
//!
//! This module provides message queue capabilities for the Ri framework.
//! It supports multiple queue backends including RabbitMQ, Kafka, and Redis.
//!
//! # Features
//!
//! - **Multi-backend Support**: RabbitMQ, Kafka, and Redis queues
//! - **Job Scheduling**: Delayed and scheduled job execution
//! - **Retry Logic**: Automatic retry with exponential backoff
//! - **Dead Letter Queue**: Failed job handling
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let queue = {{MODULE_NAME_PASCAL}}::new("amqp://localhost:5672")?;
//! queue.publish("my_queue", &my_job).await?;
//! queue.consume("my_queue", |job| async move {
//!     // Process job
//!     Ok(())
//! }).await?;
//! ```

use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

// =============================================================================
// Queue Backend Trait
// =============================================================================

/// Queue backend trait
///
/// Defines the interface for queue backend implementations.
/// All queue backends must implement this trait to provide
/// consistent message queue operations.
#[async_trait]
pub trait QueueBackend: Send + Sync {
    /// Publish a message to a queue
    ///
    /// # Arguments
    ///
    /// * `queue` - The queue name
    /// * `message` - The message to publish
    async fn publish<T: Serialize + Send + Sync>(&self, queue: &str, message: &T) -> Result<()>;

    /// Consume messages from a queue
    ///
    /// # Arguments
    ///
    /// * `queue` - The queue name
    /// * `handler` - The message handler function
    async fn consume<T, F, Fut>(&self, queue: &str, handler: F) -> Result<()>
    where
        T: DeserializeOwned + Send + Sync + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static;

    /// Acknowledge a message
    async fn ack(&self, message_id: &str) -> Result<()>;

    /// Reject a message
    async fn reject(&self, message_id: &str, requeue: bool) -> Result<()>;
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// Queue backend URL
    pub backend_url: String,

    /// Maximum number of concurrent consumers
    pub max_consumers: usize,

    /// Enable message persistence
    pub persistent: bool,

    /// Default message TTL (in seconds)
    pub default_ttl: Option<u64>,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            backend_url: "amqp://localhost:5672".to_string(),
            max_consumers: 10,
            persistent: true,
            default_ttl: None,
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main queue module struct
pub struct {{MODULE_NAME_PASCAL}} {
    backend: Box<dyn QueueBackend>,
    config: {{MODULE_NAME_PASCAL}}Config,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        let backend = Self::create_backend(&config.backend_url)?;
        Ok(Self { backend, config })
    }

    fn create_backend(url: &str) -> Result<Box<dyn QueueBackend>> {
        if url.starts_with("amqp://") {
            unimplemented!("RabbitMQ backend not yet implemented")
        } else if url.starts_with("kafka://") {
            unimplemented!("Kafka backend not yet implemented")
        } else if url.starts_with("redis://") {
            unimplemented!("Redis queue backend not yet implemented")
        } else {
            Err(anyhow::anyhow!("Unsupported queue backend URL: {}", url))
        }
    }

    /// Publish a message to a queue
    pub async fn publish<T: Serialize + Send + Sync>(
        &self,
        queue: &str,
        message: &T,
    ) -> Result<()> {
        self.backend.publish(queue, message).await
    }

    /// Consume messages from a queue
    pub async fn consume<T, F, Fut>(&self, queue: &str, handler: F) -> Result<()>
    where
        T: DeserializeOwned + Send + Sync + 'static,
        F: Fn(T) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Result<()>> + Send + 'static,
    {
        self.backend.consume(queue, handler).await
    }
}
"#.to_string()
}

/// Get the gateway module template
///
/// Returns a template for an API gateway module with routing,
/// rate limiting, and load balancing capabilities.
///
/// # Returns
///
/// Returns a string containing the gateway module template.
fn get_gateway_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Gateway Module
//!
//! This module provides API gateway capabilities for the Ri framework.
//! It includes request routing, rate limiting, and load balancing.
//!
//! # Features
//!
//! - **Request Routing**: Path-based and header-based routing
//! - **Rate Limiting**: Token bucket and sliding window algorithms
//! - **Load Balancing**: Round-robin, weighted, and least-connections
//! - **Request Transformation**: Header manipulation and body transformation
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let gateway = {{MODULE_NAME_PASCAL}}::new(config)?;
//! gateway.add_route("/api/v1/*", "http://backend:8080")?;
//! gateway.start().await?;
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Route Configuration
// =============================================================================

/// Route configuration for the gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    /// Route path pattern (supports wildcards)
    pub path: String,

    /// Backend service URL
    pub backend: String,

    /// Request timeout in seconds
    pub timeout: u64,

    /// Enable rate limiting
    pub rate_limit: Option<RateLimitConfig>,

    /// Custom headers to add
    pub headers: HashMap<String, String>,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u64,

    /// Window duration in seconds
    pub window_seconds: u64,
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// Gateway listening port
    pub port: u16,

    /// Routes configuration
    pub routes: Vec<RouteConfig>,

    /// Enable TLS
    pub enable_tls: bool,

    /// TLS certificate path
    pub tls_cert_path: Option<String>,

    /// TLS key path
    pub tls_key_path: Option<String>,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            port: 8080,
            routes: Vec::new(),
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main gateway module struct
pub struct {{MODULE_NAME_PASCAL}} {
    config: {{MODULE_NAME_PASCAL}}Config,
    routes: HashMap<String, RouteConfig>,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        let mut routes = HashMap::new();
        for route in &config.routes {
            routes.insert(route.path.clone(), route.clone());
        }

        Ok(Self { config, routes })
    }

    /// Add a route to the gateway
    pub fn add_route(&mut self, route: RouteConfig) -> Result<()> {
        self.routes.insert(route.path.clone(), route);
        Ok(())
    }

    /// Remove a route from the gateway
    pub fn remove_route(&mut self, path: &str) -> Result<bool> {
        Ok(self.routes.remove(path).is_some())
    }

    /// Start the gateway server
    pub async fn start(&self) -> Result<()> {
        // TODO: Implement gateway server
        unimplemented!("Gateway server not yet implemented")
    }

    /// Stop the gateway server
    pub async fn stop(&self) -> Result<()> {
        // TODO: Implement gateway shutdown
        unimplemented!("Gateway shutdown not yet implemented")
    }
}
"#.to_string()
}

/// Get the auth module template
///
/// Returns a template for an authentication and authorization module
/// with JWT, OAuth2, and role-based access control support.
///
/// # Returns
///
/// Returns a string containing the auth module template.
fn get_auth_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Auth Module
//!
//! This module provides authentication and authorization capabilities for the Ri framework.
//! It supports JWT, OAuth2, and role-based access control (RBAC).
//!
//! # Features
//!
//! - **JWT Authentication**: Token generation and validation
//! - **OAuth2 Integration**: Authorization code, client credentials flows
//! - **RBAC**: Role-based access control with permissions
//! - **Session Management**: Secure session handling
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let auth = {{MODULE_NAME_PASCAL}}::new(config)?;
//! let token = auth.login("user", "password").await?;
//! let claims = auth.validate_token(&token)?;
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// =============================================================================
// User and Role Types
// =============================================================================

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique user identifier
    pub id: String,

    /// Username
    pub username: String,

    /// Email address
    pub email: String,

    /// User roles
    pub roles: Vec<String>,

    /// User permissions
    pub permissions: HashSet<String>,
}

/// JWT claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: String,

    /// Issuer
    pub iss: String,

    /// Expiration time
    pub exp: u64,

    /// Issued at time
    pub iat: u64,

    /// User roles
    pub roles: Vec<String>,
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// JWT secret key
    pub jwt_secret: String,

    /// Token expiration time in seconds
    pub token_expiration: u64,

    /// Token issuer
    pub issuer: String,

    /// OAuth2 client ID
    pub oauth_client_id: Option<String>,

    /// OAuth2 client secret
    pub oauth_client_secret: Option<String>,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            token_expiration: 3600,
            issuer: "ri-auth".to_string(),
            oauth_client_id: None,
            oauth_client_secret: None,
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main auth module struct
pub struct {{MODULE_NAME_PASCAL}} {
    config: {{MODULE_NAME_PASCAL}}Config,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        Ok(Self { config })
    }

    /// Authenticate a user and generate a token
    ///
    /// # Arguments
    ///
    /// * `username` - The username
    /// * `password` - The password
    ///
    /// # Returns
    ///
    /// Returns a JWT token on successful authentication.
    pub async fn login(&self, username: &str, password: &str) -> Result<String> {
        // TODO: Implement actual authentication
        // This is a placeholder that should be replaced with
        // proper credential verification
        unimplemented!("Authentication not yet implemented")
    }

    /// Validate a JWT token
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT token to validate
    ///
    /// # Returns
    ///
    /// Returns the claims if the token is valid.
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        // TODO: Implement JWT validation
        unimplemented!("Token validation not yet implemented")
    }

    /// Check if a user has a specific permission
    ///
    /// # Arguments
    ///
    /// * `user` - The user to check
    /// * `permission` - The permission to check for
    pub fn has_permission(&self, user: &User, permission: &str) -> bool {
        user.permissions.contains(permission)
    }

    /// Check if a user has a specific role
    ///
    /// # Arguments
    ///
    /// * `user` - The user to check
    /// * `role` - The role to check for
    pub fn has_role(&self, user: &User, role: &str) -> bool {
        user.roles.iter().any(|r| r == role)
    }

    /// Logout a user (invalidate token)
    ///
    /// # Arguments
    ///
    /// * `token` - The token to invalidate
    pub async fn logout(&self, token: &str) -> Result<()> {
        // TODO: Implement token invalidation
        unimplemented!("Logout not yet implemented")
    }
}
"#.to_string()
}

/// Get the device module template
///
/// Returns a template for an IoT device management module with
/// MQTT support and device state management.
///
/// # Returns
///
/// Returns a string containing the device module template.
fn get_device_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Device Module
//!
//! This module provides IoT device management capabilities for the Ri framework.
//! It supports device registration, MQTT communication, and telemetry collection.
//!
//! # Features
//!
//! - **Device Registration**: Automatic and manual device registration
//! - **MQTT Support**: Publish/subscribe messaging for devices
//! - **State Management**: Track and manage device states
//! - **Telemetry Collection**: Collect and store device telemetry data
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let device_manager = {{MODULE_NAME_PASCAL}}::new(config)?;
//! device_manager.register_device("device-001", DeviceMetadata::default()).await?;
//! device_manager.publish_telemetry("device-001", telemetry).await?;
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// =============================================================================
// Device Types
// =============================================================================

/// Device metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceMetadata {
    /// Device type identifier
    pub device_type: String,

    /// Device firmware version
    pub firmware_version: String,

    /// Device capabilities
    pub capabilities: Vec<String>,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl Default for DeviceMetadata {
    fn default() -> Self {
        Self {
            device_type: "generic".to_string(),
            firmware_version: "1.0.0".to_string(),
            capabilities: Vec::new(),
            metadata: HashMap::new(),
        }
    }
}

/// Device state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceState {
    /// Device ID
    pub device_id: String,

    /// Online status
    pub online: bool,

    /// Last seen timestamp
    pub last_seen: u64,

    /// Current state data
    pub state: HashMap<String, serde_json::Value>,
}

/// Telemetry data from a device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Telemetry {
    /// Device ID
    pub device_id: String,

    /// Timestamp
    pub timestamp: u64,

    /// Telemetry metrics
    pub metrics: HashMap<String, f64>,
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// MQTT broker URL
    pub mqtt_url: String,

    /// Device registry storage path
    pub registry_path: String,

    /// Telemetry retention period in days
    pub telemetry_retention_days: u32,

    /// Enable device auto-registration
    pub auto_register: bool,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            mqtt_url: "mqtt://localhost:1883".to_string(),
            registry_path: "./devices.json".to_string(),
            telemetry_retention_days: 30,
            auto_register: true,
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main device management module struct
pub struct {{MODULE_NAME_PASCAL}} {
    config: {{MODULE_NAME_PASCAL}}Config,
    devices: HashMap<String, DeviceState>,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        Ok(Self {
            config,
            devices: HashMap::new(),
        })
    }

    /// Register a new device
    ///
    /// # Arguments
    ///
    /// * `device_id` - Unique device identifier
    /// * `metadata` - Device metadata
    pub async fn register_device(
        &mut self,
        device_id: &str,
        metadata: DeviceMetadata,
    ) -> Result<()> {
        let state = DeviceState {
            device_id: device_id.to_string(),
            online: false,
            last_seen: 0,
            state: HashMap::new(),
        };

        self.devices.insert(device_id.to_string(), state);
        Ok(())
    }

    /// Unregister a device
    ///
    /// # Arguments
    ///
    /// * `device_id` - Device ID to unregister
    pub async fn unregister_device(&mut self, device_id: &str) -> Result<bool> {
        Ok(self.devices.remove(device_id).is_some())
    }

    /// Get device state
    ///
    /// # Arguments
    ///
    /// * `device_id` - Device ID
    pub fn get_device_state(&self, device_id: &str) -> Option<&DeviceState> {
        self.devices.get(device_id)
    }

    /// Update device state
    ///
    /// # Arguments
    ///
    /// * `device_id` - Device ID
    /// * `online` - Online status
    pub async fn update_device_state(&mut self, device_id: &str, online: bool) -> Result<()> {
        if let Some(state) = self.devices.get_mut(device_id) {
            state.online = online;
            state.last_seen = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
        Ok(())
    }

    /// Publish telemetry data from a device
    ///
    /// # Arguments
    ///
    /// * `device_id` - Device ID
    /// * `telemetry` - Telemetry data
    pub async fn publish_telemetry(&self, device_id: &str, telemetry: Telemetry) -> Result<()> {
        // TODO: Implement telemetry storage
        unimplemented!("Telemetry publishing not yet implemented")
    }

    /// Subscribe to device events
    ///
    /// # Arguments
    ///
    /// * `device_id` - Device ID (or "*" for all devices)
    /// * `callback` - Event callback function
    pub async fn subscribe_events<F>(&self, device_id: &str, callback: F) -> Result<()>
    where
        F: Fn(DeviceState) + Send + Sync + 'static,
    {
        // TODO: Implement event subscription
        unimplemented!("Event subscription not yet implemented")
    }
}
"#.to_string()
}

/// Get the observability module template
///
/// Returns a template for an observability module with logging,
/// metrics, and distributed tracing capabilities.
///
/// # Returns
///
/// Returns a string containing the observability module template.
fn get_observability_module_template() -> String {
    r#"//! {{MODULE_NAME_PASCAL}} - Observability Module
//!
//! This module provides observability capabilities for the Ri framework.
//! It includes structured logging, metrics collection, and distributed tracing.
//!
//! # Features
//!
//! - **Structured Logging**: JSON-formatted logs with context
//! - **Metrics Collection**: Prometheus-compatible metrics
//! - **Distributed Tracing**: OpenTelemetry-compatible tracing
//! - **Health Checks**: Application health monitoring
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_{{MODULE_TYPE}}::{{MODULE_NAME_PASCAL}};
//!
//! let observability = {{MODULE_NAME_PASCAL}}::new(config)?;
//! observability.init_logging()?;
//!
//! // Log with context
//! observability.log_info("User logged in", &[("user_id", "123")]);
//!
//! // Record metrics
//! observability.increment_counter("requests_total", &[("method", "GET")]);
//!
//! // Create span for tracing
//! let span = observability.start_span("database_query");
//! // ... do work
//! span.end();
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

// =============================================================================
// Log Types
// =============================================================================

/// Log level enumeration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    /// Trace level (most verbose)
    Trace,

    /// Debug level
    Debug,

    /// Info level
    Info,

    /// Warning level
    Warn,

    /// Error level
    Error,
}

/// Log entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// Timestamp (Unix epoch)
    pub timestamp: u64,

    /// Log level
    pub level: LogLevel,

    /// Log message
    pub message: String,

    /// Context key-value pairs
    pub context: HashMap<String, String>,

    /// Source module
    pub module: Option<String>,
}

// =============================================================================
// Metric Types
// =============================================================================

/// Metric type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    /// Counter metric (only increases)
    Counter,

    /// Gauge metric (can increase or decrease)
    Gauge,

    /// Histogram metric (distribution of values)
    Histogram,
}

/// Metric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSample {
    /// Metric name
    pub name: String,

    /// Metric type
    pub metric_type: MetricType,

    /// Metric value
    pub value: f64,

    /// Labels
    pub labels: HashMap<String, String>,
}

// =============================================================================
// Span Types
// =============================================================================

/// Tracing span
pub struct Span {
    /// Span name
    pub name: String,

    /// Start time
    pub start_time: Instant,

    /// Span ID
    pub span_id: String,

    /// Parent span ID
    pub parent_id: Option<String>,

    /// Span attributes
    pub attributes: HashMap<String, String>,
}

impl Span {
    /// End the span
    pub fn end(self) {
        let duration = self.start_time.elapsed();
        // TODO: Export span to tracing backend
        let _ = duration;
    }

    /// Add an attribute to the span
    pub fn set_attribute(&mut self, key: &str, value: &str) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MODULE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MODULE_NAME_PASCAL}}Config {
    /// Service name for tracing
    pub service_name: String,

    /// Log level
    pub log_level: LogLevel,

    /// Enable metrics collection
    pub enable_metrics: bool,

    /// Metrics endpoint (Prometheus format)
    pub metrics_endpoint: String,

    /// Enable distributed tracing
    pub enable_tracing: bool,

    /// Tracing endpoint (OpenTelemetry format)
    pub tracing_endpoint: String,
}

impl Default for {{MODULE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            service_name: "ri-service".to_string(),
            log_level: LogLevel::Info,
            enable_metrics: true,
            metrics_endpoint: "/metrics".to_string(),
            enable_tracing: true,
            tracing_endpoint: "http://localhost:4317".to_string(),
        }
    }
}

// =============================================================================
// {{MODULE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MODULE_NAME_PASCAL}} - Main observability module struct
pub struct {{MODULE_NAME_PASCAL}} {
    config: {{MODULE_NAME_PASCAL}}Config,
    metrics: HashMap<String, MetricSample>,
}

impl {{MODULE_NAME_PASCAL}} {
    /// Create a new {{MODULE_NAME_PASCAL}} instance
    pub fn new(config: {{MODULE_NAME_PASCAL}}Config) -> Result<Self> {
        Ok(Self {
            config,
            metrics: HashMap::new(),
        })
    }

    /// Initialize logging system
    pub fn init_logging(&self) -> Result<()> {
        // TODO: Initialize structured logging
        Ok(())
    }

    /// Log a message at the specified level
    ///
    /// # Arguments
    ///
    /// * `level` - Log level
    /// * `message` - Log message
    /// * `context` - Context key-value pairs
    pub fn log(&self, level: LogLevel, message: &str, context: &[(&str, &str)]) {
        let entry = LogEntry {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            level,
            message: message.to_string(),
            context: context
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
            module: None,
        };

        // TODO: Write to log output
        let _ = entry;
    }

    /// Log an info message
    pub fn log_info(&self, message: &str, context: &[(&str, &str)]) {
        self.log(LogLevel::Info, message, context);
    }

    /// Log a warning message
    pub fn log_warn(&self, message: &str, context: &[(&str, &str)]) {
        self.log(LogLevel::Warn, message, context);
    }

    /// Log an error message
    pub fn log_error(&self, message: &str, context: &[(&str, &str)]) {
        self.log(LogLevel::Error, message, context);
    }

    /// Increment a counter metric
    ///
    /// # Arguments
    ///
    /// * `name` - Metric name
    /// * `labels` - Metric labels
    pub fn increment_counter(&mut self, name: &str, labels: &[(&str, &str)]) {
        let key = format!("{}:{:?}", name, labels);
        let entry = self.metrics.entry(key).or_insert(MetricSample {
            name: name.to_string(),
            metric_type: MetricType::Counter,
            value: 0.0,
            labels: labels
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        });
        entry.value += 1.0;
    }

    /// Set a gauge metric value
    ///
    /// # Arguments
    ///
    /// * `name` - Metric name
    /// * `value` - Metric value
    /// * `labels` - Metric labels
    pub fn set_gauge(&mut self, name: &str, value: f64, labels: &[(&str, &str)]) {
        let key = format!("{}:{:?}", name, labels);
        self.metrics.insert(
            key,
            MetricSample {
                name: name.to_string(),
                metric_type: MetricType::Gauge,
                value,
                labels: labels
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            },
        );
    }

    /// Start a new tracing span
    ///
    /// # Arguments
    ///
    /// * `name` - Span name
    pub fn start_span(&self, name: &str) -> Span {
        Span {
            name: name.to_string(),
            start_time: Instant::now(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_id: None,
            attributes: HashMap::new(),
        }
    }

    /// Start a child span
    ///
    /// # Arguments
    ///
    /// * `name` - Span name
    /// * `parent` - Parent span
    pub fn start_child_span(&self, name: &str, parent: &Span) -> Span {
        Span {
            name: name.to_string(),
            start_time: Instant::now(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_id: Some(parent.span_id.clone()),
            attributes: HashMap::new(),
        }
    }

    /// Export metrics in Prometheus format
    pub fn export_prometheus_metrics(&self) -> String {
        let mut output = String::new();
        for sample in self.metrics.values() {
            let labels_str = sample
                .labels
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect::<Vec<_>>()
                .join(",");

            output.push_str(&format!(
                "{}{{{}}} {}\n",
                sample.name, labels_str, sample.value
            ));
        }
        output
    }
}
"#.to_string()
}

// =============================================================================
// Middleware Template
// =============================================================================

/// Get the middleware template
///
/// Returns a template for a middleware component with standard patterns
/// for request/response handling, error management, and async processing.
///
/// # Returns
///
/// Returns a string containing the middleware template.
///
/// # Example
///
/// ```rust,ignore
/// use ric::generator::templates::get_middleware_template;
///
/// let template = get_middleware_template();
/// assert!(template.contains("{{MIDDLEWARE_NAME}}"));
/// ```
pub fn get_middleware_template() -> String {
    r#"//! {{MIDDLEWARE_NAME_PASCAL}} - Middleware
//!
//! This middleware provides {{MIDDLEWARE_NAME}} functionality for the Ri framework.
//! It intercepts requests and responses to perform specific operations.
//!
//! # Features
//!
//! - **Request Interception**: Process incoming requests before they reach handlers
//! - **Response Modification**: Modify responses before they are sent to clients
//! - **Error Handling**: Centralized error handling and recovery
//! - **Async Support**: Full async/await support for non-blocking operations
//!
//! # Usage
//!
//! ```rust,ignore
//! use ri_middleware::{{MIDDLEWARE_NAME_PASCAL}};
//!
//! let middleware = {{MIDDLEWARE_NAME_PASCAL}}::new(config);
//! app.use(middleware);
//! ```

use anyhow::{Context, Result};
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

// =============================================================================
// Middleware Trait
// =============================================================================

/// Middleware trait
///
/// Defines the interface for middleware implementations.
/// All middleware must implement this trait to be used in the request pipeline.
#[async_trait]
pub trait Middleware: Send + Sync {
    /// Process an incoming request
    ///
    /// This method is called before the request reaches the handler.
    /// It can modify the request or short-circuit the pipeline by returning an early response.
    ///
    /// # Arguments
    ///
    /// * `request` - The incoming request
    ///
    /// # Returns
    ///
    /// Returns the (possibly modified) request, or an early response.
    async fn process_request(
        &self,
        request: Request,
    ) -> Result<MiddlewareResult<Request, Response>>;

    /// Process an outgoing response
    ///
    /// This method is called after the handler has processed the request.
    /// It can modify the response before it is sent to the client.
    ///
    /// # Arguments
    ///
    /// * `response` - The outgoing response
    ///
    /// # Returns
    ///
    /// Returns the (possibly modified) response.
    async fn process_response(&self, response: Response) -> Result<Response>;
}

/// Result type for middleware request processing
///
/// Allows middleware to either pass the request to the next handler
/// or return an early response (short-circuit).
pub enum MiddlewareResult<Req, Res> {
    /// Continue to the next middleware/handler with the request
    Continue(Req),

    /// Short-circuit with an early response
    EarlyResponse(Res),
}

// =============================================================================
// Request and Response Types (Placeholder)
// =============================================================================

/// HTTP Request type
///
/// This is a placeholder type that should be replaced with
/// the actual request type from your web framework.
#[derive(Debug, Clone)]
pub struct Request {
    /// Request method
    pub method: String,

    /// Request path
    pub path: String,

    /// Request headers
    pub headers: std::collections::HashMap<String, String>,

    /// Request body
    pub body: Vec<u8>,
}

/// HTTP Response type
///
/// This is a placeholder type that should be replaced with
/// the actual response type from your web framework.
#[derive(Debug, Clone)]
pub struct Response {
    /// Response status code
    pub status: u16,

    /// Response headers
    pub headers: std::collections::HashMap<String, String>,

    /// Response body
    pub body: Vec<u8>,
}

// =============================================================================
// {{MIDDLEWARE_NAME_PASCAL}} Configuration
// =============================================================================

/// Configuration for {{MIDDLEWARE_NAME_PASCAL}}
#[derive(Debug, Clone)]
pub struct {{MIDDLEWARE_NAME_PASCAL}}Config {
    /// Enable the middleware
    pub enabled: bool,

    /// Skip paths (middleware will not process these)
    pub skip_paths: Vec<String>,

    /// Custom configuration options
    pub options: std::collections::HashMap<String, String>,
}

impl Default for {{MIDDLEWARE_NAME_PASCAL}}Config {
    fn default() -> Self {
        Self {
            enabled: true,
            skip_paths: Vec::new(),
            options: std::collections::HashMap::new(),
        }
    }
}

// =============================================================================
// {{MIDDLEWARE_NAME_PASCAL}} Implementation
// =============================================================================

/// {{MIDDLEWARE_NAME_PASCAL}} - Main middleware struct
///
/// Implements the Middleware trait to provide {{MIDDLEWARE_NAME}} functionality.
pub struct {{MIDDLEWARE_NAME_PASCAL}} {
    /// Configuration settings
    config: {{MIDDLEWARE_NAME_PASCAL}}Config,
}

impl {{MIDDLEWARE_NAME_PASCAL}} {
    /// Create a new {{MIDDLEWARE_NAME_PASCAL}} instance
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the middleware
    ///
    /// # Returns
    ///
    /// Returns a new {{MIDDLEWARE_NAME_PASCAL}} instance.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = {{MIDDLEWARE_NAME_PASCAL}}Config::default();
    /// let middleware = {{MIDDLEWARE_NAME_PASCAL}}::new(config);
    /// ```
    pub fn new(config: {{MIDDLEWARE_NAME_PASCAL}}Config) -> Self {
        Self { config }
    }

    /// Check if a path should be skipped
    ///
    /// # Arguments
    ///
    /// * `path` - The request path
    ///
    /// # Returns
    ///
    /// Returns true if the path should be skipped.
    fn should_skip(&self, path: &str) -> bool {
        self.config
            .skip_paths
            .iter()
            .any(|skip_path| path.starts_with(skip_path))
    }
}

#[async_trait]
impl Middleware for {{MIDDLEWARE_NAME_PASCAL}} {
    /// Process an incoming request
    ///
    /// This middleware performs the following operations:
    /// 1. Checks if the path should be skipped
    /// 2. Processes the request according to its logic
    /// 3. Either continues to the next handler or returns an early response
    async fn process_request(
        &self,
        mut request: Request,
    ) -> Result<MiddlewareResult<Request, Response>> {
        // Skip if path is in skip_paths
        if self.should_skip(&request.path) {
            return Ok(MiddlewareResult::Continue(request));
        }

        // Skip if middleware is disabled
        if !self.config.enabled {
            return Ok(MiddlewareResult::Continue(request));
        }

        // TODO: Implement your middleware logic here
        // For example:
        // - Authentication check
        // - Rate limiting
        // - Request logging
        // - Request modification

        // Continue to the next middleware/handler
        Ok(MiddlewareResult::Continue(request))
    }

    /// Process an outgoing response
    ///
    /// This middleware can modify the response before it is sent to the client.
    async fn process_response(&self, mut response: Response) -> Result<Response> {
        // Skip if middleware is disabled
        if !self.config.enabled {
            return Ok(response);
        }

        // TODO: Implement your response modification logic here
        // For example:
        // - Add security headers
        // - Response logging
        // - Response transformation

        Ok(response)
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Create a default {{MIDDLEWARE_NAME_PASCAL}} instance
///
/// Convenience function for creating a middleware with default configuration.
///
/// # Returns
///
/// Returns a new {{MIDDLEWARE_NAME_PASCAL}} instance with default settings.
pub fn create_default() -> {{MIDDLEWARE_NAME_PASCAL}} {
    {{MIDDLEWARE_NAME_PASCAL}}::new({{MIDDLEWARE_NAME_PASCAL}}Config::default())
}
"#.to_string()
}

// =============================================================================
// Config Template
// =============================================================================

/// Get the config struct template
///
/// Returns a template for generating Rust configuration structs from
/// YAML configuration files. The template includes serde derive macros
/// for serialization support.
///
/// # Returns
///
/// Returns a string containing the config struct template.
///
/// # Example
///
/// ```rust,ignore
/// use ric::generator::templates::get_config_template;
///
/// let template = get_config_template();
/// assert!(template.contains("{{CONFIG_NAME}}"));
/// ```
pub fn get_config_template() -> String {
    r#"//! {{CONFIG_NAME}} Configuration
//!
//! This module provides configuration structures for the application.
//! Generated from YAML configuration file.
//!
//! # Usage
//!
//! ```rust,ignore
//! use config::{{CONFIG_NAME}};
//!
//! // Load configuration from file
//! let config = {{CONFIG_NAME}}::from_file("config/app.yaml")?;
//!
//! // Access configuration values
//! println!("Server port: {}", config.server.port);
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

// =============================================================================
// {{CONFIG_NAME}} Structure
// =============================================================================

/// Main configuration structure
///
/// Contains all application configuration settings loaded from YAML.
/// This struct is automatically generated from the configuration file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct {{CONFIG_NAME}} {
{{CONFIG_FIELDS}}}

impl {{CONFIG_NAME}} {
    /// Load configuration from a YAML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the YAML configuration file
    ///
    /// # Returns
    ///
    /// Returns the loaded configuration on success.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let config = {{CONFIG_NAME}}::from_file("config/app.yaml")?;
    /// ```
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;

        let config: Self = serde_yaml::from_str(&content)
            .with_context(|| "Failed to parse configuration YAML")?;

        Ok(config)
    }

    /// Load configuration from a YAML string
    ///
    /// # Arguments
    ///
    /// * `yaml` - YAML content as a string
    ///
    /// # Returns
    ///
    /// Returns the parsed configuration on success.
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let config: Self = serde_yaml::from_str(yaml)
            .with_context(|| "Failed to parse configuration YAML")?;

        Ok(config)
    }

    /// Save configuration to a YAML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the configuration file
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on success.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let yaml = serde_yaml::to_string(self)
            .with_context(|| "Failed to serialize configuration")?;

        std::fs::write(path.as_ref(), yaml)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Convert configuration to YAML string
    ///
    /// # Returns
    ///
    /// Returns the YAML representation of the configuration.
    pub fn to_yaml(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .with_context(|| "Failed to serialize configuration")
    }

    /// Validate the configuration
    ///
    /// Performs validation checks on the configuration values.
    ///
    /// # Returns
    ///
    /// Returns Ok(()) if validation passes.
    pub fn validate(&self) -> Result<()> {
        // TODO: Add validation logic for configuration values
        Ok(())
    }
}

impl Default for {{CONFIG_NAME}} {
    fn default() -> Self {
        // TODO: Provide sensible default values
        Self {
            // Initialize fields with default values
        }
    }
}
"#.to_string()
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_module_template_cache() {
        let template = get_module_template(ModuleType::Cache);
        assert!(template.contains("{{MODULE_NAME}}"));
        assert!(template.contains("{{MODULE_NAME_PASCAL}}"));
        assert!(template.contains("CacheBackend"));
    }

    #[test]
    fn test_get_module_template_queue() {
        let template = get_module_template(ModuleType::Queue);
        assert!(template.contains("QueueBackend"));
    }

    #[test]
    fn test_get_module_template_gateway() {
        let template = get_module_template(ModuleType::Gateway);
        assert!(template.contains("RouteConfig"));
    }

    #[test]
    fn test_get_module_template_auth() {
        let template = get_module_template(ModuleType::Auth);
        assert!(template.contains("JWT"));
        assert!(template.contains("Claims"));
    }

    #[test]
    fn test_get_module_template_device() {
        let template = get_module_template(ModuleType::Device);
        assert!(template.contains("DeviceMetadata"));
        assert!(template.contains("Telemetry"));
    }

    #[test]
    fn test_get_module_template_observability() {
        let template = get_module_template(ModuleType::Observability);
        assert!(template.contains("LogEntry"));
        assert!(template.contains("MetricSample"));
        assert!(template.contains("Span"));
    }

    #[test]
    fn test_get_middleware_template() {
        let template = get_middleware_template();
        assert!(template.contains("{{MIDDLEWARE_NAME}}"));
        assert!(template.contains("{{MIDDLEWARE_NAME_PASCAL}}"));
        assert!(template.contains("Middleware trait"));
    }

    #[test]
    fn test_get_config_template() {
        let template = get_config_template();
        assert!(template.contains("{{CONFIG_NAME}}"));
        assert!(template.contains("{{CONFIG_FIELDS}}"));
        assert!(template.contains("Serialize, Deserialize"));
    }
}
