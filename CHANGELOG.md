<div align="center">
<img src="assets/svg/ri.svg" width="36" height="36">
</div>

## [0.1.8] - 2026-04-05

- Fix some known issues
- Optimize the performance of some scenarios

---

## [0.1.7] - 2026-02-17

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

- Fix some known issues

---

## [0.1.6] - 2026-02-01

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### Core Module
- Added `with_cache_module()` method to RiAppBuilder for cache module configuration
- Added `with_auth_module()` method to RiAppBuilder for auth module configuration
- Added `with_queue_module()` method to RiAppBuilder for queue module configuration
- Added `with_device_module()` method to RiAppBuilder for device module configuration
- Changed `with_logging()` and `with_observability()` return types from `RiResult<Self>` to `Self`

#### Python Bindings
- Added RiAppBuilder to Python bindings (previously Rust-only)
- Added RiHealthCheckType enum export
- Added RiHealthSummary struct export
- Added RiTrafficManager export
- Added Python ORM Repository support (py_repository.rs)
- Added comprehensive Python import tests (tests/Python/dmsc_imports.py)
- Added comprehensive Python example (examples/Python/comprehensive_example.py)

#### Cache Module
- Added `with_config()` method to RiCacheModule for backend-based configuration

#### Project Configuration
- Added `.cargo/config.toml` for Cargo configuration
- Added `CODE_OF_CONDUCT.md`
- Added `CONTRIBUTING.md`
- Added `SECURITY.md`
- Added project logo `assets/svg/dmsc.svg`

### 🔧 Changed

#### Documentation Restructure
- Renamed `mq.md` to `queue.md` (message queue documentation unified)
- Removed `http.md`, `orm.md`, `security.md`, `storage.md` (content merged into other docs)
- Added comprehensive `ws.md` (WebSocket documentation)
- Updated all API reference documentation
- Updated all usage example documentation

#### Python Examples
- Completely rewrote all Python examples to match actual API

#### Rust Source
- Improved error handling in auth module (replaced expect() with proper error returns)
- Improved PyErr to RiError conversion
- Updated post-quantum cryptography implementations (dilithium, falcon, guomi, kyber)
- Improved Kafka backend and queue manager

### 🗑️ Removed

- Removed `src/queue/backends/kafka_windows_stub.rs`

### 🐛 Fixed

- Fixed missing Python type exports in lib.rs
- Fixed auth module RedisError import reference
- Fixed cache module with_config() backend creation
- Fixed README.md and README.zh.md code examples
- Fixed documentation code examples (missing semicolons, incorrect package names)
- Fixed RiAppBuilder method return types in documentation (with_logging, with_observability)
- Fixed OAuth API documentation to match actual implementation
- Fixed RiCacheManager method signatures in Chinese documentation
- Fixed RiAppRuntime documentation (removed non-existent methods)
- Ensured Chinese and English documentation consistency

---

## [0.1.5] - 2026-01-24

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### WebSocket Client Support
- Added WebSocket client implementation (src/ws/client.rs)
- Added RiFrameParser for protocol frame parsing
- Added RiFrameBuilder for protocol frame construction

#### Python Test Suite
- Added comprehensive Python test suite (tests/Python/)
- Added 12 Python test files covering core, cache, device, gateway, and service_mesh modules
- Added pytest integration for Python testing

#### Python Examples
- Added Python usage examples (examples/Python/)
- Added 10 example files demonstrating various Ri features
- Added examples for authentication, caching, device management, and more

#### Health Check Subsystem
- Added RiHealthStatus enum for health state representation
- Added RiHealthCheckResult struct for individual check results
- Added RiHealthCheckConfig struct for check configuration
- Added RiHealthReport struct for comprehensive health reports
- Added RiHealthChecker trait for custom health check implementations

#### Lifecycle Management
- Added RiLifecycleObserver for module lifecycle monitoring
- Added RiLogAnalyticsModule for log analytics

#### Error Chain Support
- Added RiErrorChain for comprehensive error chain traversal
- Added RiErrorChainIter for iterator-based error traversal
- Added RiErrorContext for contextual error information
- Added RiOptionErrorContext for optional error context

#### Distributed Lock
- Added RiLockError for lock operation errors

#### Python Module Integration
- Added RiPythonModule for Python module integration
- Added RiPythonModuleAdapter for module adaptation
- Added RiPythonServiceModule for service module support
- Added RiPythonAsyncServiceModule for async service modules

#### Auth Module Enhancements
- Added RiJWTClaims for comprehensive JWT claims
- Added RiJWTValidationOptions for JWT validation configuration
- Added RiOAuthProvider enum for OAuth provider selection
- Added RiJWTRevocationList for token revocation
- Added RiRevokedTokenInfo for revoked token metadata

#### Device Module Enhancements
- Added RiDeviceSchedulingConfig for scheduling configuration
- Added RiResourceScheduler for resource scheduling
- Added RiDeviceScheduler for device-level scheduling
- Added RiSchedulingPolicy for scheduling strategy
- Added RiAllocationRecord for allocation tracking
- Added RiAllocationRequest for allocation requests
- Added RiAllocationStatistics for allocation metrics
- Added RiSchedulingRecommendation for scheduling suggestions
- Added RiSchedulingRecommendationType for recommendation types
- Added RiDeviceTypeStatistics for device type metrics

#### Service Mesh Enhancements
- Added RiServiceMeshStats for service mesh statistics
- Added RiTrafficManager for traffic management
- Added RiHealthChecker for health checking

#### Database ORM Enhancements
- Added ColumnDefinition for column schema
- Added IndexDefinition for index schema
- Added ForeignKeyDefinition for foreign key constraints
- Added TableDefinition for table schema
- Added LogicalOperator for query logic
- Added Criteria for query criteria
- Added JoinClause for table joins
- Added ComparisonOperator for query comparisons
- Added SortOrder for sorting options
- Added Pagination for paginated queries
- Added QueryBuilder for query construction
- Added JoinType for join types

#### Gateway Enhancements
- Added RiBackendServer for backend server representation
- Added LoadBalancerServerStats for load balancer statistics

#### Observability
- Added RiObservabilityData for unified observability data

### 🔧 Changed

#### Protocol Module
- Removed experimental warning, now production-ready
- Improved protocol safety and stability

#### Auth Module
- Renamed JWTClaims to RiJWTClaims following Ri naming convention
- Renamed JWTRevocationList to RiJWTRevocationList
- Added Ri prefix to OAuth provider types

#### Device Module
- Enhanced device scheduling capabilities
- Improved resource allocation algorithms

#### Database ORM
- Expanded ORM functionality with comprehensive schema definitions
- Enhanced query building capabilities

### 🗑️ Removed

#### Python API
- Removed RiAppBuilder from Python bindings (Rust-only)
- Removed RiSecurityManager from Python bindings (simplified)

#### Deprecated Tests
- Removed tests/core/core_error_chain.rs
- Removed tests/core/core_health.rs
- Removed tests/protocol/protocol_crypto.rs
- Removed tests/protocol/protocol_frames.rs
- Removed tests/protocol/protocol_integration_core.rs

#### Test Structure
- Moved tests from tests/*.rs to tests/Rust/*.rs

---

## [0.1.4] - 2026-01-17

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### Device Discovery Module
- Added new Device Discovery module with cross-platform support
- Added platform detection (Linux, macOS, Windows)
- Added hardware providers for CPU, Memory, Storage, Network, GPU, USB
- Added extensible plugin system for custom discovery implementations
- Added ProviderRegistry for centralized provider management
- Added PluginRegistry for plugin lifecycle management

#### Cache Module Enhancements
- Added bulk operations to RiCache trait (get_multi, set_multi, delete_multi, exists_multi)
- Added keys() method for cache key enumeration
- Added delete_by_pattern() for pattern-based cache invalidation
- Added last_accessed field to RiCachedValue for LRU support
- Added touch() and is_stale() methods for cache entry tracking
- Added async new() method to RiCacheModule with Redis auto-fallback

#### Logging Module Enhancements
- Added RiLogLevel::from_env() for environment-based log level configuration
- Added RiLogLevel::from_str() for string parsing
- Added RiLogConfig::from_env() for environment-based logging configuration

### 🔧 Changed

#### Module Import Path Fix
- Fixed module import path from `dms::prelude::*` to `dmsc::prelude::*`

#### Error Handling Improvements
- Improved lock poisoning handling with expect instead of unwrap

#### Documentation
- Added comprehensive Rustdoc comments to cache core module
- Added usage examples to documentation

---

## [0.1.3] - 2026-01-04

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### Python Module Support
- Added Python module adapter support, including RiPythonModule, RiPythonModuleAdapter, RiPythonServiceModule, RiPythonAsyncServiceModule
- Created modular Python submodule structure (dmsc.log, dmsc.config, dmsc.device, dmsc.cache, dmsc.fs, dmsc.hooks, dmsc.observability, dmsc.queue, dmsc.gateway, dmsc.service_mesh, dmsc.auth)
- Each submodule contains corresponding type bindings, providing clearer API organization

#### Device Module Enhancements
- Added RiDiscoveryResult type for structured device discovery result representation
- Added RiResourceRequest type for standardized resource request definition
- Added RiRequestSlaClass type for SLA-level resource request classification
- Added RiResourceWeights type for flexible resource weight configuration
- Added RiAffinityRules type for affinity rule definition and application
- Added RiResourceAllocation type for resource allocation result representation
- Added RiDeviceStatus enum for standardized device status definition
- Added RiDeviceCapabilities struct for detailed device capability description
- Added RiDeviceHealthMetrics struct for device health metrics collection and representation

#### Observability Module Enhancements
- Added RiObservabilityData type for unified observability data structure representation
- Added RiMetricsRegistry type for metrics registry creation and management
- Added RiTracer type for standardized distributed tracing implementation

#### Service Mesh Module Enhancements
- Added RiServiceInstance type for standardized service instance representation
- Added RiHealthChecker type for unified health check implementation
- Added RiTrafficManager type for flexible traffic management configuration

### 🔧 Changed

#### Prelude Export Optimization
- Reorganized prelude module export structure
- Provided clearer API import paths and usage methods
- Optimized import dependency relationships between modules

#### Documentation Structure Refactoring
- Migrated documentation directory from python/doc/ to doc/
- Achieved centralized document management and unified maintenance
- Synchronized Chinese and English document content updates

#### Precompiled Binary Support
- Added precompiled Python wheel packages (dmsc_0.1.3_linux.whl, dmsc_0.1.3_windows.whl)
- Added precompiled Rust library files (libdmsc.rlib, libdmsc.dll)
- Provided more convenient deployment and integration methods

### 🗑️ Removed
- Removed python/setup.py file

### ⚠️ Breaking Changes

#### Python Module Structure Changes
- Python submodule structure has changed, existing code may need to update import statements
- Recommendation: Adjust import statements from single module import to submodule import

#### Documentation Path Changes
- python/doc/ directory has been deleted, all documentation has been migrated to doc/ directory
- References relying on old documentation paths need to be updated

---

## [0.1.2] - 2025-12-13

### ✨ Added
- Initial release version
- Provided 12 core functional modules: core, config, log, auth, cache, queue, device, gateway, service_mesh, protocol, observability, fs
- Supported Rust and Python dual-language bindings
- Provided complete error handling and logging functionality
- Supported multi-backend cache and message queue
- Provided device management and resource scheduling functions
- Supported service mesh and service discovery
- Provided observability functions (metrics, tracing)
- Supported configuration management and hot reload
