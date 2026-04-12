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

//! Background Worker Template
//!
//! This module defines the background worker template for creating job processing
//! services with Ri framework. It provides a comprehensive starting point for
//! building asynchronous task processing systems.
//!
//! # Template Features
//!
//! The worker template includes:
//!
//! - **Task Queue Integration**: Built-in support for multiple queue backends
//! - **Worker Pool Management**: Configurable worker pool with dynamic scaling
//! - **Job Scheduling**: Cron-like job scheduling capabilities
//! - **Error Handling**: Automatic retry logic with exponential backoff
//! - **Progress Tracking**: Real-time job progress monitoring
//! - **Dead Letter Queue**: Failed job handling and recovery
//! - **Metrics**: Performance metrics and monitoring endpoints
//! - **Graceful Shutdown**: Safe worker shutdown with job completion
//!
//! # Generated Files
//!
//! The template generates the following project structure:
//!
//! ```text
//! my-worker/
//! ├── Cargo.toml              # Package manifest with dependencies
//! ├── src/
//! │   ├── main.rs            # Application entry point
//! │   ├── worker/            # Worker implementation
//! │   │   ├── mod.rs
//! │   │   ├── pool.rs        # Worker pool management
//! │   │   └── executor.rs    # Job execution logic
//! │   ├── jobs/              # Job definitions
//! │   │   ├── mod.rs
//! │   │   ├── email.rs       # Email job example
//! │   │   └── report.rs      # Report generation job
//! │   ├── queue/             # Queue abstraction
//! │   │   ├── mod.rs
//! │   │   ├── memory.rs      # In-memory queue
//! │   │   └── redis.rs       # Redis queue
//! │   └── scheduler/         # Job scheduler
//! │       └── mod.rs
//! ├── config/
//! │   └── config.yaml        # Application configuration
//! └── scripts/
//!     └── jobs.sh            # Job management scripts
//! ```
//!
//! # Template Variables
//!
//! The template supports the following variables:
//!
//! | Variable | Type | Required | Default | Description |
//! |----------|------|----------|---------|-------------|
//! | `project_name` | string | Yes | - | Project name |
//! | `version` | string | No | "0.1.0" | Project version |
//! | `author` | string | No | "Dunimd Team" | Project author |
//! | `description` | string | No | "A Ri worker service" | Project description |
//! | `queue_type` | string | No | "memory" | Queue backend type (memory/redis) |
//! | `max_workers` | integer | No | "4" | Maximum number of worker threads |
//! | `enable_persistence` | boolean | No | "false" | Enable job persistence |
//! | `enable_scheduler` | boolean | No | "true" | Enable job scheduler |
//! | `retry_attempts` | integer | No | "3" | Maximum retry attempts for failed jobs |
//! | `redis_url` | string | No | "" | Redis connection URL (if queue_type is redis) |
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::worker;
//!
//! // Get template information
//! let info = worker::get_template_info();
//! println!("Template: {}", info.display_name);
//!
//! // List template features
//! for feature in &info.features {
//!     println!("- {}", feature);
//! }
//!
//! // Get template variables
//! for var in &info.variables {
//!     println!("{}: {} (default: {})", var.name, var.description, var.default_value);
//! }
//! ```
//!
//! # Example Generated Code
//!
//! The generated `main.rs` will look like:
//!
//! ```rust,ignore
//! use ri::core::RiAppBuilder;
//! use ri::worker::{WorkerPool, WorkerConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = RiAppBuilder::new("my-worker")
//!         .with_worker(WorkerPool::new(4))
//!         .build()
//!         .await?;
//!
//!     app.run().await
//! }
//! ```
//!
//! # Worker Design Principles
//!
//! - **Reliability**: Automatic retries and dead letter queue
//! - **Scalability**: Dynamic worker pool scaling
//! - **Observability**: Metrics and progress tracking
//! - **Flexibility**: Multiple queue backend support
//! - **Safety**: Graceful shutdown and job completion guarantees
//! - **Testing**: Built-in testing utilities for jobs

use super::engine::{TemplateInfo, TemplateVariable, TemplateFile};

/// Get worker template metadata
///
/// Returns the complete metadata for the worker template,
/// including features, variables, and file definitions.
///
/// # Returns
///
/// Returns a `TemplateInfo` struct containing all template metadata.
///
/// # Example
///
/// ```rust,ignore
/// let info = get_template_info();
/// assert_eq!(info.name, "worker");
/// assert!(!info.features.is_empty());
/// ```
pub fn get_template_info() -> TemplateInfo {
    TemplateInfo {
        name: "worker".to_string(),
        display_name: "Background Worker".to_string(),
        description: "Background job processing service with task queues, scheduling, and retry logic".to_string(),
        author: "Dunimd Team".to_string(),
        version: "1.0.0".to_string(),
        features: vec![
            "Task queue integration".to_string(),
            "Worker pool management".to_string(),
            "Job scheduling with cron syntax".to_string(),
            "Automatic retry with exponential backoff".to_string(),
            "Real-time progress tracking".to_string(),
            "Dead letter queue for failed jobs".to_string(),
            "Performance metrics and monitoring".to_string(),
            "Graceful shutdown support".to_string(),
        ],
        variables: vec![
            TemplateVariable {
                name: "project_name".to_string(),
                description: "Project name used in package manifest and worker".to_string(),
                default_value: "my-worker".to_string(),
                required: true,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Project version following semantic versioning".to_string(),
                default_value: "0.1.0".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "author".to_string(),
                description: "Project author or maintainer".to_string(),
                default_value: "Dunimd Team".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "description".to_string(),
                description: "Brief description of the worker service".to_string(),
                default_value: "A Ri worker service".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "queue_type".to_string(),
                description: "Queue backend type (memory, redis)".to_string(),
                default_value: "memory".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "max_workers".to_string(),
                description: "Maximum number of concurrent worker threads".to_string(),
                default_value: "4".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "enable_persistence".to_string(),
                description: "Enable job persistence to survive restarts".to_string(),
                default_value: "false".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_scheduler".to_string(),
                description: "Enable cron-like job scheduler".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "retry_attempts".to_string(),
                description: "Maximum retry attempts for failed jobs".to_string(),
                default_value: "3".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "redis_url".to_string(),
                description: "Redis connection URL (required if queue_type is redis)".to_string(),
                default_value: "".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
        ],
        files: vec![
            TemplateFile {
                source: "Cargo.toml.tera".to_string(),
                destination: "Cargo.toml".to_string(),
            },
            TemplateFile {
                source: "src/main.rs.tera".to_string(),
                destination: "src/main.rs".to_string(),
            },
            TemplateFile {
                source: "src/worker/mod.rs.tera".to_string(),
                destination: "src/worker/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/worker/pool.rs.tera".to_string(),
                destination: "src/worker/pool.rs".to_string(),
            },
            TemplateFile {
                source: "src/worker/executor.rs.tera".to_string(),
                destination: "src/worker/executor.rs".to_string(),
            },
            TemplateFile {
                source: "src/jobs/mod.rs.tera".to_string(),
                destination: "src/jobs/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/jobs/email.rs.tera".to_string(),
                destination: "src/jobs/email.rs".to_string(),
            },
            TemplateFile {
                source: "src/jobs/report.rs.tera".to_string(),
                destination: "src/jobs/report.rs".to_string(),
            },
            TemplateFile {
                source: "src/queue/mod.rs.tera".to_string(),
                destination: "src/queue/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/queue/memory.rs.tera".to_string(),
                destination: "src/queue/memory.rs".to_string(),
            },
            TemplateFile {
                source: "src/queue/redis.rs.tera".to_string(),
                destination: "src/queue/redis.rs".to_string(),
            },
            TemplateFile {
                source: "src/scheduler/mod.rs.tera".to_string(),
                destination: "src/scheduler/mod.rs".to_string(),
            },
            TemplateFile {
                source: "config/config.yaml.tera".to_string(),
                destination: "config/config.yaml".to_string(),
            },
            TemplateFile {
                source: "scripts/jobs.sh.tera".to_string(),
                destination: "scripts/jobs.sh".to_string(),
            },
        ],
    }
}

/// Get default template variables
///
/// Returns a map of variable names to their default values.
/// This is useful for pre-populating forms or providing suggestions.
///
/// # Returns
///
/// Returns a HashMap with all variable names mapped to their default values.
///
/// # Example
///
/// ```rust,ignore
/// let defaults = get_default_variables();
/// assert_eq!(defaults.get("queue_type"), Some(&"memory".to_string()));
/// assert_eq!(defaults.get("max_workers"), Some(&"4".to_string()));
/// ```
pub fn get_default_variables() -> std::collections::HashMap<String, String> {
    let info = get_template_info();
    info.variables
        .into_iter()
        .map(|v| (v.name, v.default_value))
        .collect()
}

/// Get required template variables
///
/// Returns a list of variable names that are required for this template.
///
/// # Returns
///
/// Returns a vector of required variable names.
///
/// # Example
///
/// ```rust,ignore
/// let required = get_required_variables();
/// assert!(required.contains(&"project_name".to_string()));
/// ```
pub fn get_required_variables() -> Vec<String> {
    get_template_info()
        .variables
        .into_iter()
        .filter(|v| v.required)
        .map(|v| v.name)
        .collect()
}

/// Validate template-specific variables
///
/// Performs additional validation for worker template variables beyond
/// basic type checking. This includes queue type validation and
/// Redis URL validation when required.
///
/// # Arguments
///
/// * `variables` - Map of variable names to values
///
/// # Returns
///
/// Returns `Ok(())` if all variables are valid.
/// Returns an error if any variable fails validation.
///
/// # Example
///
/// ```rust,ignore
/// let mut vars = HashMap::new();
/// vars.insert("queue_type".to_string(), "memory".to_string());
/// vars.insert("max_workers".to_string(), "4".to_string());
///
/// validate_variables(&vars)?; // Ok
///
/// vars.insert("queue_type".to_string(), "redis".to_string());
/// // Missing redis_url will cause validation error
/// validate_variables(&vars)?; // Error
/// ```
pub fn validate_variables(variables: &std::collections::HashMap<String, String>) -> anyhow::Result<()> {
    // Validate queue type
    if let Some(queue_type) = variables.get("queue_type") {
        if !["memory", "redis"].contains(&queue_type.as_str()) {
            return Err(anyhow::anyhow!(
                "Queue type must be 'memory' or 'redis', got: {}",
                queue_type
            ));
        }

        // If queue type is redis, redis_url must be provided
        if queue_type == "redis" {
            let redis_url = variables.get("redis_url").map(|s| s.as_str()).unwrap_or("");
            if redis_url.is_empty() {
                return Err(anyhow::anyhow!(
                    "Redis URL is required when queue_type is 'redis'"
                ));
            }
        }
    }

    // Validate max_workers (1-1024)
    if let Some(max_workers) = variables.get("max_workers") {
        let workers_num: usize = max_workers.parse().map_err(|_| {
            anyhow::anyhow!("Max workers must be a valid positive number")
        })?;
        if workers_num == 0 || workers_num > 1024 {
            return Err(anyhow::anyhow!("Max workers must be between 1 and 1024"));
        }
    }

    // Validate retry_attempts (0-10)
    if let Some(retry_attempts) = variables.get("retry_attempts") {
        let attempts: usize = retry_attempts.parse().map_err(|_| {
            anyhow::anyhow!("Retry attempts must be a valid number")
        })?;
        if attempts > 10 {
            return Err(anyhow::anyhow!("Retry attempts cannot exceed 10"));
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template_info() {
        let info = get_template_info();
        assert_eq!(info.name, "worker");
        assert!(!info.features.is_empty());
        assert!(!info.variables.is_empty());
        assert!(!info.files.is_empty());
    }

    #[test]
    fn test_get_default_variables() {
        let defaults = get_default_variables();
        assert_eq!(defaults.get("queue_type"), Some(&"memory".to_string()));
        assert_eq!(defaults.get("max_workers"), Some(&"4".to_string()));
        assert_eq!(defaults.get("enable_scheduler"), Some(&"true".to_string()));
    }

    #[test]
    fn test_get_required_variables() {
        let required = get_required_variables();
        assert!(required.contains(&"project_name".to_string()));
        assert_eq!(required.len(), 1); // Only project_name is required
    }

    #[test]
    fn test_validate_variables_valid() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("queue_type".to_string(), "memory".to_string());
        vars.insert("max_workers".to_string(), "4".to_string());
        vars.insert("retry_attempts".to_string(), "3".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_invalid_queue_type() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("queue_type".to_string(), "invalid".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_redis_without_url() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("queue_type".to_string(), "redis".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_redis_with_url() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("queue_type".to_string(), "redis".to_string());
        vars.insert("redis_url".to_string(), "redis://localhost:6379".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_invalid_retry_attempts() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("retry_attempts".to_string(), "15".to_string());
        assert!(validate_variables(&vars).is_err());
    }
}
