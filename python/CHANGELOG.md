<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">Changelog</span>
</h1>

</div>

## [0.1.7] - 2026-02-17

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

- Fix some known issues

---

## [0.1.7] - 2026-02-17

## 此版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

- 修复一些已知问题

---

## [0.1.6] - 2026-02-01

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### Core Module
- Added `with_cache_module()` method to DMSCAppBuilder for cache module configuration
- Added `with_auth_module()` method to DMSCAppBuilder for auth module configuration
- Added `with_queue_module()` method to DMSCAppBuilder for queue module configuration
- Added `with_device_module()` method to DMSCAppBuilder for device module configuration
- Changed `with_logging()` and `with_observability()` return types from `DMSCResult<Self>` to `Self`

#### Python Bindings
- Added DMSCAppBuilder to Python bindings (previously Rust-only)
- Added DMSCHealthCheckType enum export
- Added DMSCHealthSummary struct export
- Added DMSCTrafficManager export
- Added Python ORM Repository support (py_repository.rs)
- Added comprehensive Python import tests (tests/Python/dmsc_imports.py)
- Added comprehensive Python example (examples/Python/comprehensive_example.py)

#### Cache Module
- Added `with_config()` method to DMSCCacheModule for backend-based configuration

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
- Improved PyErr to DMSCError conversion
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
- Fixed DMSCAppBuilder method return types in documentation (with_logging, with_observability)
- Fixed OAuth API documentation to match actual implementation
- Fixed DMSCCacheManager method signatures in Chinese documentation
- Fixed DMSCAppRuntime documentation (removed non-existent methods)
- Ensured Chinese and English documentation consistency

---

## [0.1.6] - 2026-02-01

## 此版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

### ✨ 新增功能

#### 核心模块
- 新增 `with_cache_module()` 方法到 DMSCAppBuilder 用于缓存模块配置
- 新增 `with_auth_module()` 方法到 DMSCAppBuilder 用于认证模块配置
- 新增 `with_queue_module()` 方法到 DMSCAppBuilder 用于队列模块配置
- 新增 `with_device_module()` 方法到 DMSCAppBuilder 用于设备模块配置
- 修改 `with_logging()` 和 `with_observability()` 返回类型从 `DMSCResult<Self>` 改为 `Self`

#### Python 绑定
- 新增 DMSCAppBuilder 到 Python 绑定（之前仅 Rust 可用）
- 新增 DMSCHealthCheckType 枚举导出
- 新增 DMSCHealthSummary 结构体导出
- 新增 DMSCTrafficManager 导出
- 新增 Python ORM Repository 支持 (py_repository.rs)
- 新增 Python 导入测试 (tests/Python/dmsc_imports.py)
- 新增综合 Python 示例 (examples/Python/comprehensive_example.py)

#### 缓存模块
- 新增 `with_config()` 方法到 DMSCCacheModule 用于基于配置的后端初始化

#### 项目配置
- 新增 `.cargo/config.toml` Cargo 配置文件
- 新增 `CODE_OF_CONDUCT.md` 行为准则
- 新增 `CONTRIBUTING.md` 贡献指南
- 新增 `SECURITY.md` 安全政策
- 新增项目 Logo `assets/svg/dmsc.svg`

### 🔧 改进优化

#### 文档重组
- 重命名 `mq.md` 为 `queue.md`（消息队列文档统一）
- 删除 `http.md`、`orm.md`、`security.md`、`storage.md`（内容合并到其他文档）
- 新增完整 `ws.md`（WebSocket 文档）
- 更新所有 API 参考文档
- 更新所有使用示例文档

#### Python 示例
- 完全重写所有 Python 示例以匹配实际 API

#### Rust 源码
- 改进认证模块错误处理（用 proper error returns 替代 expect()）
- 改进 PyErr 到 DMSCError 的转换
- 更新后量子加密实现（dilithium、falcon、guomi、kyber）
- 改进 Kafka 后端和队列管理器

### 🗑️ 移除内容

- 删除 `src/queue/backends/kafka_windows_stub.rs`

### 🐛 问题修复

- 修复 lib.rs 中缺失的 Python 类型导出
- 修复认证模块 RedisError 导入引用
- 修复缓存模块 with_config() 后端创建
- 修复 README.md 和 README.zh.md 代码示例
- 修复文档代码示例（缺失分号、错误的包名）
- 修复 DMSCAppBuilder 方法返回类型文档（with_logging、with_observability）
- 修复 OAuth API 文档以匹配实际实现
- 修复中文文档中 DMSCCacheManager 方法签名
- 修复 DMSCAppRuntime 文档（删除不存在的方法）
- 确保中英文文档一致性

---

## [0.1.5] - 2026-01-24

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### WebSocket Client Support
- Added WebSocket client implementation (src/ws/client.rs)
- Added DMSCFrameParser for protocol frame parsing
- Added DMSCFrameBuilder for protocol frame construction

#### Python Test Suite
- Added comprehensive Python test suite (tests/Python/)
- Added 12 Python test files covering core, cache, device, gateway, and service_mesh modules
- Added pytest integration for Python testing

#### Python Examples
- Added Python usage examples (examples/Python/)
- Added 10 example files demonstrating various DMSC features
- Added examples for authentication, caching, device management, and more

#### Health Check Subsystem
- Added DMSCHealthStatus enum for health state representation
- Added DMSCHealthCheckResult struct for individual check results
- Added DMSCHealthCheckConfig struct for check configuration
- Added DMSCHealthReport struct for comprehensive health reports
- Added DMSCHealthChecker trait for custom health check implementations

#### Lifecycle Management
- Added DMSCLifecycleObserver for module lifecycle monitoring
- Added DMSCLogAnalyticsModule for log analytics

#### Error Chain Support
- Added DMSCErrorChain for comprehensive error chain traversal
- Added DMSCErrorChainIter for iterator-based error traversal
- Added DMSCErrorContext for contextual error information
- Added DMSCOptionErrorContext for optional error context

#### Distributed Lock
- Added DMSCLockError for lock operation errors

#### Python Module Integration
- Added DMSCPythonModule for Python module integration
- Added DMSCPythonModuleAdapter for module adaptation
- Added DMSCPythonServiceModule for service module support
- Added DMSCPythonAsyncServiceModule for async service modules

#### Auth Module Enhancements
- Added DMSCJWTClaims for comprehensive JWT claims
- Added DMSCJWTValidationOptions for JWT validation configuration
- Added DMSCOAuthProvider enum for OAuth provider selection
- Added DMSCJWTRevocationList for token revocation
- Added DMSCRevokedTokenInfo for revoked token metadata

#### Device Module Enhancements
- Added DMSCDeviceSchedulingConfig for scheduling configuration
- Added DMSCResourceScheduler for resource scheduling
- Added DMSCDeviceScheduler for device-level scheduling
- Added DMSCSchedulingPolicy for scheduling strategy
- Added DMSCAllocationRecord for allocation tracking
- Added DMSCAllocationRequest for allocation requests
- Added DMSCAllocationStatistics for allocation metrics
- Added DMSCSchedulingRecommendation for scheduling suggestions
- Added DMSCSchedulingRecommendationType for recommendation types
- Added DMSCDeviceTypeStatistics for device type metrics

#### Service Mesh Enhancements
- Added DMSCServiceMeshStats for service mesh statistics
- Added DMSCTrafficManager for traffic management
- Added DMSCHealthChecker for health checking

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
- Added DMSCBackendServer for backend server representation
- Added LoadBalancerServerStats for load balancer statistics

#### Observability
- Added DMSCObservabilityData for unified observability data

### 🔧 Changed

#### Protocol Module
- Removed experimental warning, now production-ready
- Improved protocol safety and stability

#### Auth Module
- Renamed JWTClaims to DMSCJWTClaims following DMSC naming convention
- Renamed JWTRevocationList to DMSCJWTRevocationList
- Added DMSC prefix to OAuth provider types

#### Device Module
- Enhanced device scheduling capabilities
- Improved resource allocation algorithms

#### Database ORM
- Expanded ORM functionality with comprehensive schema definitions
- Enhanced query building capabilities

### 🗑️ Removed

#### Python API
- Removed DMSCAppBuilder from Python bindings (Rust-only)
- Removed DMSCSecurityManager from Python bindings (simplified)

#### Deprecated Tests
- Removed tests/core/core_error_chain.rs
- Removed tests/core/core_health.rs
- Removed tests/protocol/protocol_crypto.rs
- Removed tests/protocol/protocol_frames.rs
- Removed tests/protocol/protocol_integration_core.rs

#### Test Structure
- Moved tests from tests/*.rs to tests/Rust/*.rs

---

## [0.1.5] - 2026-01-24

## 此版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

### ✨ 新增功能

#### WebSocket客户端支持
- 新增WebSocket客户端实现（src/ws/client.rs）
- 新增DMSCFrameParser用于协议帧解析
- 新增DMSCFrameBuilder用于协议帧构建

#### Python测试套件
- 新增Python测试套件（tests/Python/）
- 新增12个Python测试文件，涵盖core、cache、device、gateway和service_mesh模块
- 新增pytest集成用于Python测试

#### Python示例
- 新增Python使用示例（examples/Python/）
- 新增10个示例文件，展示各种DMSC功能
- 新增认证、缓存、设备管理等功能示例

#### 健康检查子系统
- 新增DMSCHealthStatus枚举用于健康状态表示
- 新增DMSCHealthCheckResult结构体用于单个检查结果
- 新增DMSCHealthCheckConfig结构体用于检查配置
- 新增DMSCHealthReport结构体用于综合健康报告
- 新增DMSCHealthChecker特征用于自定义健康检查实现

#### 生命周期管理
- 新增DMSCLifecycleObserver用于模块生命周期监控
- 新增DMSCLogAnalyticsModule用于日志分析

#### 错误链支持
- 新增DMSCErrorChain用于综合错误链遍历
- 新增DMSCErrorChainIter用于迭代器式错误遍历
- 新增DMSCErrorContext用于上下文错误信息
- 新增DMSCOptionErrorContext用于可选错误上下文

#### 分布式锁
- 新增DMSCLockError用于锁操作错误

#### Python模块集成
- 新增DMSCPythonModule用于Python模块集成
- 新增DMSCPythonModuleAdapter用于模块适配
- 新增DMSCPythonServiceModule用于服务模块支持
- 新增DMSCPythonAsyncServiceModule用于异步服务模块

#### 认证模块增强
- 新增DMSCJWTClaims用于综合JWT声明
- 新增DMSCJWTValidationOptions用于JWT验证配置
- 新增DMSCOAuthProvider枚举用于OAuth提供商选择
- 新增DMSCJWTRevocationList用于令牌撤销
- 新增DMSCRevokedTokenInfo用于撤销令牌元数据

#### 设备模块增强
- 新增DMSCDeviceSchedulingConfig用于调度配置
- 新增DMSCResourceScheduler用于资源调度
- 新增DMSCDeviceScheduler用于设备级调度
- 新增DMSCSchedulingPolicy用于调度策略
- 新增DMSCAllocationRecord用于分配追踪
- 新增DMSCAllocationRequest用于分配请求
- 新增DMSCAllocationStatistics用于分配指标
- 新增DMSCSchedulingRecommendation用于调度建议
- 新增DMSCSchedulingRecommendationType用于建议类型
- 新增DMSCDeviceTypeStatistics用于设备类型指标

#### 服务网格增强
- 新增DMSCServiceMeshStats用于服务网格统计
- 新增DMSCTrafficManager用于流量管理
- 新增DMSCHealthChecker用于健康检查

#### 数据库ORM增强
- 新增ColumnDefinition用于列模式定义
- 新增IndexDefinition用于索引模式定义
- 新增ForeignKeyDefinition用于外键约束定义
- 新增TableDefinition用于表模式定义
- 新增LogicalOperator用于查询逻辑
- 新增Criteria用于查询条件
- 新增JoinClause用于表连接
- 新增ComparisonOperator用于查询比较
- 新增SortOrder用于排序选项
- 新增Pagination用于分页查询
- 新增QueryBuilder用于查询构建
- 新增JoinType用于连接类型

#### 网关增强
- 新增DMSCBackendServer用于后端服务器表示
- 新增LoadBalancerServerStats用于负载均衡器统计

#### 可观测性
- 新增DMSCObservabilityData用于统一可观测性数据

### 🔧 改进优化

#### 协议模块
- 移除实验性警告，现已可用于生产环境
- 提升协议安全性和稳定性

#### 认证模块
- 将JWTClaims重命名为DMSCJWTClaims，遵循DMSC命名规范
- 将JWTRevocationList重命名为DMSCJWTRevocationList
- OAuth提供商类型添加DMSC前缀

#### 设备模块
- 增强设备调度能力
- 改进资源分配算法

#### 数据库ORM
- 扩展ORM功能，提供全面的模式定义
- 增强查询构建能力

### 🗑️ 移除内容

#### Python API
- 从Python绑定中移除DMSCAppBuilder（仅Rust可用）
- 从Python绑定中移除DMSCSecurityManager（已简化）

#### 废弃测试
- 移除tests/core/core_error_chain.rs
- 移除tests/core/core_health.rs
- 移除tests/protocol/protocol_crypto.rs
- 移除tests/protocol/protocol_frames.rs
- 移除tests/protocol/protocol_integration_core.rs

#### 测试结构
- 将测试从tests/*.rs移动到tests/Rust/*.rs

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
- Added bulk operations to DMSCCache trait (get_multi, set_multi, delete_multi, exists_multi)
- Added keys() method for cache key enumeration
- Added delete_by_pattern() for pattern-based cache invalidation
- Added last_accessed field to DMSCCachedValue for LRU support
- Added touch() and is_stale() methods for cache entry tracking
- Added async new() method to DMSCCacheModule with Redis auto-fallback

#### Logging Module Enhancements
- Added DMSCLogLevel::from_env() for environment-based log level configuration
- Added DMSCLogLevel::from_str() for string parsing
- Added DMSCLogConfig::from_env() for environment-based logging configuration

### 🔧 Changed

#### Module Import Path Fix
- Fixed module import path from `dms::prelude::*` to `dmsc::prelude::*`

#### Error Handling Improvements
- Improved lock poisoning handling with expect instead of unwrap

#### Documentation
- Added comprehensive Rustdoc comments to cache core module
- Added usage examples to documentation

---

## [0.1.4] - 2026-01-17

## 该版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

### ✨ 新增功能

#### 设备发现模块
- 新增设备发现模块，支持跨平台硬件发现
- 新增平台检测（Linux、macOS、Windows）
- 新增CPU、Memory、Storage、Network、GPU、USB硬件Provider
- 新增可扩展插件系统，支持自定义发现实现
- 新增ProviderRegistry，提供Provider集中管理
- 新增PluginRegistry，支持插件生命周期管理

#### 缓存模块增强
- 新增批量操作到DMSCCache特征（get_multi、set_multi、delete_multi、exists_multi）
- 新增keys()方法用于缓存键枚举
- 新增delete_by_pattern()用于模式匹配缓存失效
- 新增last_accessed字段到DMSCCachedValue，支持LRU
- 新增touch()和is_stale()方法用于缓存条目追踪
- 新增DMSCCacheModule的异步new()方法，支持Redis自动降级

#### 日志模块增强
- 新增DMSCLogLevel::from_env()，支持基于环境的日志级别配置
- 新增DMSCLogLevel::from_str()，支持字符串解析
- 新增DMSCLogConfig::from_env()，支持基于环境的日志配置

### 🔧 改进优化

#### 模块导入路径修复
- 修复模块导入路径从`dms::prelude::*`到`dmsc::prelude::*`

#### 错误处理改进
- 改进锁中毒处理，使用expect代替unwrap

#### 文档
- 新增缓存核心模块的完整Rustdoc注释
- 新增使用示例到文档

---

## [0.1.3] - 2026-01-04

## This version is a beta version with incomplete functionality and may pose risks. It is not recommended for production environments or official use.

### ✨ Added

#### Python Module Support
- Added Python module adapter support, including DMSCPythonModule, DMSCPythonModuleAdapter, DMSCPythonServiceModule, DMSCPythonAsyncServiceModule
- Created modular Python submodule structure (dmsc.log, dmsc.config, dmsc.device, dmsc.cache, dmsc.fs, dmsc.hooks, dmsc.observability, dmsc.queue, dmsc.gateway, dmsc.service_mesh, dmsc.auth)
- Each submodule contains corresponding type bindings, providing clearer API organization

#### Device Module Enhancements
- Added DMSCDiscoveryResult type for structured device discovery result representation
- Added DMSCResourceRequest type for standardized resource request definition
- Added DMSCRequestSlaClass type for SLA-level resource request classification
- Added DMSCResourceWeights type for flexible resource weight configuration
- Added DMSCAffinityRules type for affinity rule definition and application
- Added DMSCResourceAllocation type for resource allocation result representation
- Added DMSCDeviceStatus enum for standardized device status definition
- Added DMSCDeviceCapabilities struct for detailed device capability description
- Added DMSCDeviceHealthMetrics struct for device health metrics collection and representation

#### Observability Module Enhancements
- Added DMSCObservabilityData type for unified observability data structure representation
- Added DMSCMetricsRegistry type for metrics registry creation and management
- Added DMSCTracer type for standardized distributed tracing implementation

#### Service Mesh Module Enhancements
- Added DMSCServiceInstance type for standardized service instance representation
- Added DMSCHealthChecker type for unified health check implementation
- Added DMSCTrafficManager type for flexible traffic management configuration

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

## [0.1.3] - 2026-01-04

## 此版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

### ✨ 新增功能

#### Python模块化支持
- 新增Python模块适配器支持，包括DMSCPythonModule、DMSCPythonModuleAdapter、DMSCPythonServiceModule、DMSCPythonAsyncServiceModule
- 创建模块化的Python子模块结构（dmsc.log、dmsc.config、dmsc.device、dmsc.cache、dmsc.fs、dmsc.hooks、dmsc.observability、dmsc.queue、dmsc.gateway、dmsc.service_mesh、dmsc.auth）
- 每个子模块包含对应的类型绑定，提供更清晰的API组织结构

#### 设备模块增强
- 新增DMSCDiscoveryResult类型，支持设备发现结果的结构化表示
- 新增DMSCResourceRequest类型，支持资源请求的标准化定义
- 新增DMSCRequestSlaClass类型，支持SLA级别的资源请求分类
- 新增DMSCResourceWeights类型，支持资源权重的灵活配置
- 新增DMSCAffinityRules类型，支持亲和性规则的定义和应用
- 新增DMSCResourceAllocation类型，支持资源分配结果的表示
- 新增DMSCDeviceStatus枚举，标准化设备状态定义
- 新增DMSCDeviceCapabilities结构体，支持设备能力的详细描述
- 新增DMSCDeviceHealthMetrics结构体，支持设备健康指标的采集和表示

#### 可观测性模块增强
- 新增DMSCObservabilityData类型，统一可观测性数据的结构化表示
- 新增DMSCMetricsRegistry类型，支持指标注册表的创建和管理
- 新增DMSCTracer类型，支持分布式追踪的标准化实现

#### 服务网格模块增强
- 新增DMSCServiceInstance类型，支持服务实例的标准化表示
- 新增DMSCHealthChecker类型，支持健康检查的统一实现
- 新增DMSCTrafficManager类型，支持流量管理的灵活配置

### 🔧 改进优化

#### Prelude导出优化
- 重新组织prelude模块的导出结构
- 提供更清晰的API导入路径和使用方式
- 优化模块间的导入依赖关系

#### 文档结构重构
- 将文档目录从python/doc/迁移至doc/
- 实现文档的集中管理和统一维护
- 同步更新中英文文档内容

#### 预编译产物支持
- 新增预编译的Python wheel包（dmsc_0.1.3_linux.whl、dmsc_0.1.3_windows.whl）
- 新增预编译的Rust库文件（libdmsc.rlib、libdmsc.dll）
- 提供更便捷的部署和集成方式

### 🗑️ 移除内容
- 移除python/setup.py文件

### ⚠️ 破坏性变更

#### Python模块结构变化
- Python子模块结构发生变更，现有代码可能需要更新导入语句
- 建议：将导入语句从单一模块导入调整为子模块导入

#### 文档路径变更
- python/doc/目录已被删除，所有文档已迁移至doc/目录
- 依赖旧文档路径的引用需要更新

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

---

## [0.1.2] - 2025-12-13

### ✨ 新增功能
- 初始版本发布
- 提供12个核心功能模块：core、config、log、auth、cache、queue、device、gateway、service_mesh、protocol、observability、fs
- 支持Rust和Python双语言绑定
- 提供完整的错误处理和日志记录功能
- 支持多后端缓存和消息队列
- 提供设备管理和资源调度功能
- 支持服务网格和服务发现
- 提供可观测性功能（指标、追踪）
- 支持配置管理和热重载
