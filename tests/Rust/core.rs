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

//! # Core Module Comprehensive Tests
//!
//! This file contains comprehensive tests for the Ri core module based on the documentation.
//! Tests cover all documented APIs and usage patterns.

use ri::core::{RiAppBuilder, RiServiceContext, RiError, RiModule};
use ri::log::RiLogConfig;
use ri::core::lock::{RiLockError, RwLockExtensions, MutexExtensions};
use std::sync::{RwLock, Mutex};
use tokio::runtime::Runtime;

mod app_builder_tests {
    use super::*;

    #[test]
    /// Tests RiAppBuilder creation with new() constructor.
    ///
    /// Verifies that an application builder can be created successfully
    /// and is ready for configuration with default settings.
    ///
    /// ## Expected Behavior
    ///
    /// - Builder is created without errors
    /// - The builder is ready for configuration methods
    fn test_app_builder_new() {
        let builder = RiAppBuilder::new();
        assert!(builder.with_config("test.yaml").is_ok());
    }

    #[test]
    /// Tests RiAppBuilder configuration loading with with_config().
    ///
    /// Verifies that the builder can load configuration from a YAML file
    /// and that subsequent configuration methods can be chained.
    ///
    /// ## Configuration Loading
    ///
    /// - The with_config() method accepts a file path string
    /// - Returns a Result indicating success or failure
    /// - Configuration is validated during loading
    ///
    /// ## Expected Behavior
    ///
    /// - Configuration file is processed successfully
    /// - Builder state is updated with loaded configuration
    fn test_app_builder_with_config_string() {
        let result = RiAppBuilder::new().with_config("config.yaml");
        assert!(result.is_ok());
        let builder = result.unwrap();
        assert!(builder.with_logging(RiLogConfig::default()).is_ok());
    }

    #[test]
    /// Tests RiAppBuilder logging configuration with with_logging().
    ///
    /// Verifies that the builder can be configured with logging settings
    /// using a RiLogConfig instance.
    ///
    /// ## Logging Configuration
    ///
    /// - The with_logging() method accepts a RiLogConfig
    /// - Logging configuration controls log level, output, and formatting
    /// - Returns a Result for error handling
    ///
    /// ## Expected Behavior
    ///
    /// - Logging configuration is accepted successfully
    /// - The builder can be used for further configuration
    fn test_app_builder_with_logging() {
        let config = RiLogConfig::default();
        let result = RiAppBuilder::new().with_logging(config);
        assert!(result.is_ok());
    }

    #[test]
    /// Tests RiAppBuilder method chaining for fluent configuration.
    ///
    /// Verifies that multiple configuration methods can be chained
    /// together in a fluent API pattern for clean configuration code.
    ///
    /// ## Method Chaining
    ///
    /// - Configuration methods return the builder for chaining
    /// - build() is called at the end to create the application
    /// - Each method call modifies the builder state
    ///
    /// ## Expected Behavior
    ///
    /// - All configuration methods return the builder
    /// - build() completes the configuration and creates the app
    /// - The resulting application is ready for use
    fn test_app_builder_chaining() {
        let app = RiAppBuilder::new()
            .with_config("config.yaml")
            .unwrap()
            .with_logging(RiLogConfig::default())
            .unwrap()
            .build();
        assert!(app.is_ok());
    }

    #[test]
    /// Tests RiAppBuilder module registration with with_dms_module().
    ///
    /// Verifies that custom Ri modules can be registered with the
    /// application builder for extensible functionality.
    ///
    ## Module Registration
    ///
    /// - Modules implement the RiModule trait
    /// - Modules are registered using Box<dyn RiModule>
    /// - Multiple modules can be registered for the application
    ///
    /// ## Expected Behavior
    ///
    /// - Custom module is accepted by the builder
    /// - The module is included in the built application
    fn test_app_builder_with_dms_module() {
        struct TestDmsModule;
        #[async_trait::async_trait]
        impl RiModule for TestDmsModule {
            fn name(&self) -> &str { "test_dms" }
        }

        let app = RiAppBuilder::new()
            .with_dms_module(Box::new(TestDmsModule))
            .build();
        assert!(app.is_ok());
    }

    #[test]
    /// Tests RiAppBuilder build without explicit configuration.
    ///
    /// Verifies that the builder can create an application without
    /// providing explicit configuration files.
    ///
    /// ## Default Configuration
    ///
    /// - When no config is provided, defaults are used
    /// - Default configuration is suitable for development
    /// - Critical features are still initialized
    ///
    /// ## Expected Behavior
    ///
    /// - Application is created with defaults
    /// - No configuration errors are returned
    fn test_app_builder_build_without_config() {
        let app = RiAppBuilder::new().build();
        assert!(app.is_ok());
    }
}

mod service_context_tests {
    use super::*;

    #[test]
    /// Tests RiServiceContext creation with new_default().
    ///
    /// Verifies that a service context can be created with default
    /// configuration and initializes all core components.
    ///
    /// ## Expected Behavior
    ///
    /// - Service context is created successfully
    /// - All subsystems are initialized
    fn test_service_context_new_default() {
        let ctx = RiServiceContext::new_default();
        assert!(ctx.is_ok());
    }

    #[test]
    /// Tests RiServiceContext filesystem access through fs().
    ///
    /// Verifies that the service context provides access to the
    /// filesystem abstraction layer for file operations.
    ///
    /// ## Filesystem Access
    ///
    /// - The fs() method returns a RiFileSystem instance
    /// - Filesystem provides project root and category directories
    /// - File operations are available through this interface
    ///
    /// ## Expected Behavior
    ///
    /// - Filesystem is accessible through the context
    /// - The project root directory exists
    fn test_service_context_fs_access() {
        let ctx = RiServiceContext::new_default().unwrap();
        let fs = ctx.fs();
        assert!(fs.project_root().exists());
    }

    #[test]
    /// Tests RiServiceContext logger access through logger().
    ///
    /// Verifies that the service context provides access to the
    /// logging subsystem for application logging.
    ///
    /// ## Logger Access
    ///
    /// - The logger() method returns a RiLogger instance
    /// - Logger supports different log levels
    /// - Logging is configured according to settings
    ///
    /// ## Expected Behavior
    ///
    /// - Logger is accessible through the context
    /// - Logging operations succeed
    fn test_service_context_logger_access() {
        let ctx = RiServiceContext::new_default().unwrap();
        let logger = ctx.logger();
        assert!(logger.info("test", "Logger access test").is_ok());
    }

    #[test]
    /// Tests RiServiceContext configuration access through config().
    ///
    /// Verifies that the service context provides access to the
    /// configuration subsystem for application settings.
    ///
    /// ## Configuration Access
    ///
    /// - The config() method returns a configuration manager
    /// - Configuration values can be retrieved by key
    /// - Type-safe accessors are provided for common types
    ///
    /// ## Expected Behavior
    ///
    /// - Configuration is accessible through the context
    /// - Non-existent keys return None
    fn test_service_context_config_access() {
        let ctx = RiServiceContext::new_default().unwrap();
        let config = ctx.config();
        assert!(config.config().get_str("nonexistent").is_none());
    }

    #[test]
    /// Tests RiServiceContext hooks access through hooks().
    ///
    /// Verifies that the service context provides access to the
    /// hooks subsystem for lifecycle event handling.
    ///
    /// ## Hooks Access
    ///
    /// - The hooks() method returns a hook bus
    /// - Hooks enable extensible lifecycle management
    /// - Modules can register handlers for events
    ///
    /// ## Expected Behavior
    ///
    /// - Hooks subsystem is accessible
    fn test_service_context_hooks_access() {
        let ctx = RiServiceContext::new_default().unwrap();
        let _hooks = ctx.hooks();
    }
}

mod ri_module_tests {
    use super::*;

    #[test]
    /// Tests custom RiModule creation and registration.
    ///
    /// Verifies that custom modules can implement the RiModule
    /// trait with custom name, priority, and dependencies.
    ///
    /// ## Module Implementation
    ///
    /// - Modules must implement the RiModule trait
    /// - Required methods: name(), is_critical(), priority(), dependencies()
    /// - Modules can have custom initialization and shutdown logic
    ///
    /// ## Expected Behavior
    ///
    /// - Custom module is created successfully
    /// - Module can be registered with the application builder
    fn test_custom_ri_module_build() {
        struct CustomModule;
        #[async_trait::async_trait]
        impl RiModule for CustomModule {
            fn name(&self) -> &str { "custom" }
            fn is_critical(&self) -> bool { false }
            fn priority(&self) -> i32 { 10 }
            fn dependencies(&self) -> Vec<&str> { vec![] }
        }

        let runtime = Runtime::new().unwrap();
        let result = runtime.block_on(async {
            RiAppBuilder::new()
                .with_dms_module(Box::new(CustomModule))
                .build()
        });
        assert!(result.is_ok());
    }

    #[test]
    /// Tests custom RiModule execution with run().
    ///
    /// Verifies that custom modules can execute async code
    /// when the application runs.
    ///
    /// ## Module Execution
    ///
    /// - The run() method accepts an async closure
    /// - The closure receives the service context
    /// - Module lifecycle events are triggered appropriately
    ///
    /// ## Expected Behavior
    ///
    /// - Module executes without errors
    /// - Async code runs to completion
    fn test_custom_ri_module_run() {
        struct TestModule;
        #[async_trait::async_trait]
        impl RiModule for TestModule {
            fn name(&self) -> &str { "test" }
        }

        let runtime = Runtime::new().unwrap();
        let result = runtime.block_on(async {
            let app = RiAppBuilder::new()
                .with_dms_module(Box::new(TestModule))
                .build()
                .unwrap();

            app.run(|_ctx| async move {
                Ok(())
            }).await
        });
        assert!(result.is_ok());
    }
}

mod error_tests {
    use super::*;

    #[test]
    /// Tests RiError IO variant message formatting.
    ///
    /// Verifies that IO errors are properly formatted with
    /// descriptive error messages.
    ///
    /// ## Error Variants
    ///
    /// - RiError::Io - Input/output errors
    /// - RiError::Config - Configuration errors
    /// - RiError::Other - Generic errors
    ///
    /// ## Expected Behavior
    ///
    /// - IO errors contain "IO error" in the message
    fn test_error_io_variant() {
        let error = RiError::Io("test io error".to_string());
        let msg = error.to_string();
        assert!(msg.contains("IO error"));
    }

    #[test]
    /// Tests RiError Config variant message formatting.
    ///
    /// Verifies that configuration errors are properly formatted
    /// with descriptive error messages.
    ///
    /// ## Expected Behavior
    ///
    /// - Config errors contain "Configuration error" in the message
    fn test_error_config_variant() {
        let error = RiError::Config("test config error".to_string());
        let msg = error.to_string();
        assert!(msg.contains("Configuration error"));
    }

    #[test]
    /// Tests RiError Other variant message preservation.
    ///
    /// Verifies that generic errors preserve the original
    /// error message.
    ///
    /// ## Expected Behavior
    ///
    /// - Other errors contain the original message
    fn test_error_other_variant() {
        let error = RiError::Other("test error".to_string());
        let msg = error.to_string();
        assert!(msg.contains("test error"));
    }

    #[test]
    /// Tests RiError conversion from std::io::Error.
    ///
    /// Verifies that standard IO errors can be converted
    /// into RiError with proper message translation.
    ///
    /// ## Error Conversion
    ///
    /// - std::io::Error implements Into<RiError>
    /// - IO error kinds are translated to RiError::Io
    /// - Original error context is preserved
    ///
    /// ## Expected Behavior
    ///
    /// - IO errors are converted to RiError::Io
    /// - The message contains "IO error"
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let ri_err: RiError = io_err.into();
        let msg = ri_err.to_string();
        assert!(msg.contains("IO error"));
    }
}

mod lock_tests {
    use super::*;

    #[test]
    /// Tests RiLockError creation with new().
    ///
    /// Verifies that lock errors can be created with custom
    /// context information.
    ///
    /// ## Expected Behavior
    ///
    /// - Error is created with the specified context
    /// - The error is not marked as poisoned
    fn test_lock_error_new() {
        let error = RiLockError::new("test context");
        assert!(!error.is_poisoned());
        assert_eq!(error.context(), "test context");
    }

    #[test]
    /// Tests RiLockError poisoned state with poisoned().
    ///
    /// Verifies that lock errors can indicate a poisoned
    /// lock state.
    ///
    /// ## Poisoned Locks
    ///
    /// - Poisoned locks indicate a thread panic while holding the lock
    /// - The poisoned() method creates an error with poison flag set
    /// - Poisoned locks should generally not be recovered
    ///
    /// ## Expected Behavior
    ///
    /// - Error is marked as poisoned
    /// - The context is preserved
    fn test_lock_error_poisoned() {
        let error = RiLockError::poisoned("poisoned lock");
        assert!(error.is_poisoned());
        assert_eq!(error.context(), "poisoned lock");
    }

    #[test]
    /// Tests RwLockExtensions read_safe() for safe reading.
    ///
    /// Verifies that read locks can be acquired safely with
    /// proper error handling.
    ///
    /// ## Safe Lock Acquisition
    ///
    /// - read_safe() acquires a read lock with context
    /// - Multiple readers can hold the lock simultaneously
    /// - Writers are blocked until all readers release
    ///
    /// ## Expected Behavior
    ///
    /// - Read lock is acquired successfully
    /// - The protected value is accessible
    fn test_rwlock_read_safe() {
        let lock = RwLock::new(42);
        let result = lock.read_safe("test read");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    /// Tests RwLockExtensions write_safe() for safe writing.
    ///
    /// Verifies that write locks can be acquired safely with
    /// proper error handling.
    ///
    /// ## Safe Write Locking
    ///
    /// - write_safe() acquires an exclusive write lock
    /// - Only one writer can hold the lock
    /// - All readers are blocked during write
    ///
    /// ## Expected Behavior
    ///
    /// - Write lock is acquired successfully
    /// - The value can be modified
    fn test_rwlock_write_safe() {
        let lock = RwLock::new(0);
        let result = lock.write_safe("test write");
        assert!(result.is_ok());
        *result.unwrap() = 100;
        assert_eq!(*lock.read_safe("verify").unwrap(), 100);
    }

    #[test]
    /// Tests RwLockExtensions try_read_safe() for non-blocking reads.
    ///
    /// Verifies that read locks can be attempted without blocking.
    ///
    /// ## Try Lock Operations
    ///
    /// - try_read_safe() attempts to acquire a read lock
    /// - Returns immediately if lock is not available
    /// - Returns None if the lock is held by a writer
    ///
    /// ## Expected Behavior
    ///
    /// - Try read succeeds when lock is available
    /// - Some is returned with the guard
    fn test_rwlock_try_read_safe() {
        let lock = RwLock::new(42);
        let result = lock.try_read_safe("try read");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    /// Tests RwLockExtensions try_write_safe() for non-blocking writes.
    ///
    /// Verifies that write locks can be attempted without blocking.
    ///
    /// ## Expected Behavior
    ///
    /// - Try write succeeds when lock is available
    /// - Some is returned with the guard
    fn test_rwlock_try_write_safe() {
        let lock = RwLock::new(42);
        let result = lock.try_write_safe("try write");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    /// Tests MutexExtensions lock_safe() for safe mutex locking.
    ///
    /// Verifies that mutex locks can be acquired safely with
    /// proper error handling.
    ///
    /// ## Safe Mutex Locking
    ///
    /// - lock_safe() acquires the mutex lock with context
    /// - Panics are converted to lock errors
    /// - The lock is held until the guard is dropped
    ///
    /// ## Expected Behavior
    ///
    /// - Mutex lock is acquired successfully
    /// - The protected value is accessible
    fn test_mutex_lock_safe() {
        let mutex = Mutex::new(42);
        let result = mutex.lock_safe("test mutex");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    /// Tests MutexExtensions try_lock_safe() for non-blocking mutex.
    ///
    /// Verifies that mutex locks can be attempted without blocking.
    ///
    /// ## Expected Behavior
    ///
    /// - Try lock succeeds when lock is available
    /// - Some is returned with the guard
    fn test_mutex_try_lock_safe() {
        let mutex = Mutex::new(42);
        let result = mutex.try_lock_safe("try mutex");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
}
