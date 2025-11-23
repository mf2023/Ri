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

// Error and result types for DMS core.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DMSError {
    Io(String),
    Serde(String),
    Config(String),
    Hook(String),
    Prometheus(String),
    ServiceMesh(String),
    DeviceNotFound { device_id: String },
    DeviceAllocationFailed { device_id: String, reason: String },
    AllocationNotFound { allocation_id: String },
    Other(String),
}

pub type DMSResult<T> = Result<T, DMSError>;

impl std::fmt::Display for DMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMSError::Io(err) => write!(f, "IO error: {}", err),
            DMSError::Serde(err) => write!(f, "Serialization error: {}", err),
            DMSError::Config(msg) => write!(f, "Configuration error: {}", msg),
            DMSError::Hook(msg) => write!(f, "Hook error: {}", msg),
            DMSError::Prometheus(err) => write!(f, "Prometheus error: {}", err),
            DMSError::ServiceMesh(err) => write!(f, "Service mesh error: {}", err),
            DMSError::DeviceNotFound { device_id } => write!(f, "Device not found: {}", device_id),
            DMSError::DeviceAllocationFailed { device_id, reason } => {
                write!(f, "Device allocation failed for {}: {}", device_id, reason)
            }
            DMSError::AllocationNotFound { allocation_id } => {
                write!(f, "Allocation not found: {}", allocation_id)
            }
            DMSError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for DMSError {}

impl From<std::io::Error> for DMSError {
    fn from(error: std::io::Error) -> Self {
        DMSError::Io(error.to_string())
    }
}

impl From<serde_json::Error> for DMSError {
    fn from(error: serde_json::Error) -> Self {
        DMSError::Serde(error.to_string())
    }
}

impl From<prometheus::Error> for DMSError {
    fn from(error: prometheus::Error) -> Self {
        DMSError::Prometheus(error.to_string())
    }
}

impl From<redis::RedisError> for DMSError {
    fn from(error: redis::RedisError) -> Self {
        DMSError::Other(format!("Redis error: {}", error))
    }
}

impl From<lapin::Error> for DMSError {
    fn from(error: lapin::Error) -> Self {
        DMSError::Other(format!("RabbitMQ error: {}", error))
    }
}
