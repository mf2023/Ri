# DMSC Changelog

## [0.1.5] - 2026-01-17

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

## [0.1.5] - 2026-01-16

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

## 该版本为测试版，功能尚不完整，可能存在风险，不推荐用于生产环境或正式用途。

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
