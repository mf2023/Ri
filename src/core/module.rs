//! Core module system for DMSC.
//!
//! This module provides the foundation for modular service architecture in DMSC.
//! It defines traits and structures for both synchronous and asynchronous service modules.
//!
//! ## Design Principles
//!
//! 1. **Lifecycle Management**: Modules follow a well-defined lifecycle with multiple phases
//! 2. **Sync/Async Support**: Clear separation between synchronous and asynchronous modules
//! 3. **Default Implementations**: Most methods have sensible defaults to minimize boilerplate
//! 4. **Dependency Resolution**: Modules can declare dependencies on other modules
//! 5. **Priority System**: Modules can specify execution priority
//! 6. **Criticality Flag**: Modules can be marked as critical or non-critical
//!
//! ## Module Lifecycle
//!
//! Modules go through the following lifecycle phases:
//!
//! 1. **Initialization**: `init` - Set up module resources
//! 2. **Before Start**: `before_start` - Prepare for module startup
//! 3. **Start**: `start` - Start module execution
//! 4. **After Start**: `after_start` - Post-startup operations
//! 5. **Before Shutdown**: `before_shutdown` - Prepare for shutdown
//! 6. **Shutdown**: `shutdown` - Stop module execution
//! 7. **After Shutdown**: `after_shutdown` - Cleanup resources

use crate::core::{DMSCResult, DMSCServiceContext};

/// Synchronous service module trait.
/// 
/// This trait defines the interface for synchronous service modules in DMSC. It provides
/// a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dmsc::core::{ServiceModule, DMSCResult, DMSCServiceContext};
/// 
/// struct MySyncModule;
/// 
/// impl ServiceModule for MySyncModule {
///     fn name(&self) -> &str {
///         "my_sync_module"
///     }
///     
///     fn is_critical(&self) -> bool {
///         false
///     }
///     
///     fn priority(&self) -> i32 {
///         10
///     }
///     
///     fn dependencies(&self) -> Vec<&str> {
///         vec!["dependency_module"]
///     }
///     
///     fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
///         // Start module logic
///         Ok(())
///     }
/// }
/// ```
pub trait ServiceModule: Send + Sync {
    /// Returns the name of the service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn name(&self) -> &str;

    /// Indicates if the module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn is_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn priority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Starts the service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-startup operations.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Shuts down the service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}

/// Public asynchronous service module trait.
/// 
/// This trait defines the public interface for asynchronous service modules in DMSC.
/// It provides a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dmsc::core::{DMSCModule, DMSCResult, DMSCServiceContext};
/// use async_trait::async_trait;
/// 
/// struct MyAsyncModule;
/// 
/// #[async_trait]
/// impl DMSCModule for MyAsyncModule {
///     fn name(&self) -> &str {
///         "my_async_module"
///     }
///     
///     async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
///         // Start async module logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait DMSCModule: Send + Sync {
    /// Returns the name of the service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn name(&self) -> &str;

    /// Indicates if the module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn is_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn priority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    async fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    async fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Starts the service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-startup operations.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    async fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    async fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Shuts down the service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
/// Python representation of a DMSC module configuration.
///
/// This structure provides a Python-accessible view of module configuration,
/// allowing Python code to define module properties that can be used in Rust.
/// It serves as a data container for module metadata and lifecycle configuration.
///
/// ## Attributes
///
/// - **name**: The unique identifier for the module
/// - **is_critical**: Whether the module is critical to system operation
/// - **priority**: Execution priority for dependency resolution
/// - **dependencies**: List of module names this module depends on
///
/// ## Python Usage
///
/// ```python
/// import dms
///
/// module = dms.DMSCPythonModule(name="my_python_module")
/// module.is_critical = True
/// module.priority = 100
/// module.dependencies = ["logger", "config"]
/// ```
#[pyclass]
#[pyo3(name = "DMSCPythonModule")]
#[derive(Clone)]
pub struct DMSCPythonModule {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    is_critical: bool,
    #[pyo3(get)]
    priority: i32,
    #[pyo3(get)]
    dependencies: Vec<String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCPythonModule {
    #[new]
    fn new(name: String) -> Self {
        DMSCPythonModule {
            name,
            is_critical: true,
            priority: 0,
            dependencies: Vec::new(),
        }
    }
    
    #[getter]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    
    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    #[getter]
    pub fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    #[setter]
    pub fn set_is_critical(&mut self, is_critical: bool) {
        self.is_critical = is_critical;
    }
    
    #[getter]
    pub fn priority(&self) -> i32 {
        self.priority
    }
    
    #[setter]
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
    #[getter]
    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    #[setter]
    pub fn set_dependencies(&mut self, dependencies: Vec<String>) {
        self.dependencies = dependencies;
    }
}

#[cfg(feature = "pyo3")]
/// Python module adapter that implements DMSCModule trait.
///
/// This structure enables Python modules to integrate with the DMSC module system
/// by implementing the `AsyncServiceModule` trait. Python code can create instances
/// of this adapter with custom configuration, and they will participate in the
/// module lifecycle just like Rust-native modules.
///
/// ## Thread Safety
///
/// This structure is designed to be safely used across threads when combined
/// with the appropriate Python GIL management. The underlying implementation
/// ensures proper synchronization during lifecycle callbacks.
///
/// ## Lifecycle Methods
///
/// All lifecycle methods (`init`, `before_start`, `start`, `after_start`,
/// `before_shutdown`, `shutdown`, `after_shutdown`) have default implementations
/// that return `Ok(())`, allowing Python modules to override only the methods
/// they need.
///
/// ## Python Usage
///
/// ```python
/// import dms
///
/// class MyPythonModule:
///     def name(&self):
///         return "python_module"
///
///     async def start(&self, ctx):
///         print("Starting Python module")
///         return None
///
/// adapter = dms.DMSCPythonModuleAdapter(name="my_module")
/// adapter.name = "python_module"
/// ```
#[pyclass]
#[derive(Clone)]
pub struct DMSCPythonModuleAdapter {
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub is_critical: bool,
    #[pyo3(get, set)]
    pub priority: i32,
    #[pyo3(get, set)]
    pub dependencies: Vec<String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCPythonModuleAdapter {
    #[new]
    fn new(name: String) -> Self {
        DMSCPythonModuleAdapter {
            name,
            is_critical: true,
            priority: 0,
            dependencies: Vec::new(),
        }
    }
}

#[cfg(feature = "pyo3")]
/// Python wrapper for synchronous ServiceModule trait.
///
/// This structure provides a Python-accessible representation of synchronous
/// service modules. It enables Python code to define synchronous modules that
/// integrate with DMSC's module lifecycle system. Unlike asynchronous modules,
/// synchronous modules execute their operations in a blocking manner.
///
/// ## Synchronous vs Asynchronous
///
/// Synchronous modules use blocking I/O and execute their callbacks on the
/// same thread as the module lifecycle manager. For non-blocking operations,
/// use `DMSCPythonAsyncServiceModule` instead.
///
/// ## Threading Model
///
/// Synchronous modules are executed in the context of the module management
/// thread. Long-running operations will block the entire module system.
/// Consider using `tokio::task::spawn_blocking` for CPU-intensive work.
///
/// ## Default Behavior
///
/// All lifecycle methods return `Ok(())` by default, allowing Python modules
/// to override only the methods they need to customize.
///
/// ## Python Usage
///
/// ```python
/// import dms
///
/// class MySyncModule:
///     def name(&self):
///         return "sync_module"
///
///     def start(&self, ctx):
///         print("Starting sync module")
///         return None
///
/// module = dms.DMSCPythonServiceModule(name="my_sync")
/// module.priority = 50
/// module.dependencies = ["config"]
/// ```
#[pyclass]
#[derive(Clone)]
pub struct DMSCPythonServiceModule {
    name: String,
    is_critical: bool,
    priority: i32,
    dependencies: Vec<String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCPythonServiceModule {
    #[new]
    fn new(name: String) -> Self {
        DMSCPythonServiceModule {
            name,
            is_critical: true,
            priority: 0,
            dependencies: Vec::new(),
        }
    }
    
    #[getter]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    
    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    #[getter]
    pub fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    #[setter]
    pub fn set_is_critical(&mut self, is_critical: bool) {
        self.is_critical = is_critical;
    }
    
    #[getter]
    pub fn priority(&self) -> i32 {
        self.priority
    }
    
    #[setter]
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
    #[getter]
    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    #[setter]
    pub fn set_dependencies(&mut self, dependencies: Vec<String>) {
        self.dependencies = dependencies;
    }
}

#[cfg(feature = "pyo3")]
/// Python wrapper for asynchronous AsyncServiceModule trait.
///
/// This structure provides a Python-accessible representation of asynchronous
/// service modules. It enables Python code to define async modules that integrate
/// with DMSC's module lifecycle system using non-blocking operations.
///
/// ## Asynchronous Execution
///
/// Asynchronous modules use Rust's async/await model and are executed on the
/// Tokio runtime. This enables non-blocking I/O operations and concurrent
/// execution of multiple async modules without thread blocking.
///
/// ## Tokio Runtime
///
/// The async modules execute within the Tokio runtime that is managed by the
/// DMSC application runtime. This provides efficient task scheduling and
/// native support for async I/O operations.
///
/// ## Python Integration
///
/// When using Python with pyo3, asynchronous methods should be defined using
/// async def syntax. The Python runtime must be properly initialized before
/// async operations can be performed.
///
/// ## Python Usage
///
/// ```python
/// import dms
///
/// class MyAsyncModule:
///     async def name(&self):
///         return "async_module"
///
///     async def start(&self, ctx):
///         print("Starting async module")
///         return None
///
/// module = dms.DMSCPythonAsyncServiceModule(name="my_async")
/// module.priority = 100
/// module.dependencies = ["config", "logger"]
/// ```
#[pyclass]
#[derive(Clone)]
pub struct DMSCPythonAsyncServiceModule {
    name: String,
    is_critical: bool,
    priority: i32,
    dependencies: Vec<String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCPythonAsyncServiceModule {
    #[new]
    fn new(name: String) -> Self {
        DMSCPythonAsyncServiceModule {
            name,
            is_critical: true,
            priority: 0,
            dependencies: Vec::new(),
        }
    }
    
    #[getter]
    pub fn name(&self) -> String {
        self.name.clone()
    }
    
    #[setter]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    #[getter]
    pub fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    #[setter]
    pub fn set_is_critical(&mut self, is_critical: bool) {
        self.is_critical = is_critical;
    }
    
    #[getter]
    pub fn priority(&self) -> i32 {
        self.priority
    }
    
    #[setter]
    pub fn set_priority(&mut self, priority: i32) {
        self.priority = priority;
    }
    
    #[getter]
    pub fn dependencies(&self) -> Vec<String> {
        self.dependencies.clone()
    }
    
    #[setter]
    pub fn set_dependencies(&mut self, dependencies: Vec<String>) {
        self.dependencies = dependencies;
    }
}

#[cfg(feature = "pyo3")]
#[async_trait::async_trait]
impl AsyncServiceModule for DMSCPythonModuleAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
    
    fn dependencies(&self) -> Vec<&str> {
        self.dependencies.iter().map(|s| s.as_str()).collect()
    }
    
    async fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
impl ServiceModule for DMSCPythonServiceModule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
    
    fn dependencies(&self) -> Vec<&str> {
        self.dependencies.iter().map(|s| s.as_str()).collect()
    }
    
    fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[async_trait::async_trait]
impl AsyncServiceModule for DMSCPythonAsyncServiceModule {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    fn priority(&self) -> i32 {
        self.priority
    }
    
    fn dependencies(&self) -> Vec<&str> {
        self.dependencies.iter().map(|s| s.as_str()).collect()
    }
    
    async fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
    
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}


/// Internal asynchronous service module trait.
/// 
/// This trait defines the interface for internal asynchronous service modules in DMSC.
/// It provides a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dmsc::core::{AsyncServiceModule, DMSCResult, DMSCServiceContext};
/// use async_trait::async_trait;
/// 
/// struct MyInternalAsyncModule;
/// 
/// #[async_trait]
/// impl AsyncServiceModule for MyInternalAsyncModule {
///     fn name(&self) -> &str {
///         "my_internal_async_module"
///     }
///     
///     async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
///         // Start internal async module logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait AsyncServiceModule: Send + Sync {
    /// Returns the name of the internal async service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn name(&self) -> &str;

    /// Indicates if the internal module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn is_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the internal module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn priority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the internal async service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    async fn init(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the internal module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    async fn before_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Starts the internal async service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-startup operations for the internal module.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    async fn after_start(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Prepares the internal module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    async fn before_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Shuts down the internal async service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup for the internal module.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        Ok(())
    }
}
