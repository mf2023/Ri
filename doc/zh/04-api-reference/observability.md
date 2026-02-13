<div align="center">

# Observability API参考

**Version: 0.1.7**

**Last modified date: 2026-02-13**

observability模块提供系统可观测性支持，包括指标收集、分布式追踪等功能。

## 模块概述

</div>

observability模块包含以下子模块：

- **metrics**: 指标收集与注册表
- **tracing**: 分布式追踪

<div align="center">

## 核心组件

</div>

### DMSCObservabilityModule

可观测性模块主接口，提供统一的指标和追踪访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建可观测性模块 | `config: DMSCObservabilityConfig` | `Self` |
| `tracer()` | 获取分布式追踪器 | 无 | `Option<Arc<DMSCTracer>>` |
| `metrics_registry()` | 获取指标注册表 | 无 | `Option<Arc<DMSCMetricsRegistry>>` |
| `collect_metrics()` | 收集指标数据 | 无 | `DMSCObservabilityData` |

### DMSCObservabilityConfig

可观测性模块配置。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `tracing_enabled` | `bool` | 启用分布式追踪 | `true` |
| `metrics_enabled` | `bool` | 启用指标收集 | `true` |
| `sampling_rate` | `f64` | 追踪采样率 (0.0-1.0) | `1.0` |
| `metrics_endpoint` | `String` | 指标端点路径 | `"/metrics"` |

#### 使用示例

```rust
use dmsc::observability::DMSCObservabilityModule;

let config = DMSCObservabilityConfig::default();
let observability = DMSCObservabilityModule::new(config);
```

### DMSCObservabilityData

可观测性数据。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `metrics` | `String` | Prometheus格式的指标数据 |
| `active_traces` | `usize` | 活跃追踪数 |
| `active_spans` | `usize` | 活跃跨度数 |

<div align="center">

## 指标收集

</div>

### DMSCMetricsRegistry

指标注册表，用于收集和聚合指标。

#### 使用示例

```rust
use dmsc::observability::DMSCMetricsRegistry;

let registry = DMSCMetricsRegistry::new();
```

### DMSCMetricType

指标类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Counter` | 计数器 |
| `Gauge` | 仪表盘 |
| `Histogram` | 直方图 |
| `Summary` | 汇总 |

### DMSCMetric

指标结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `name` | `String` | 指标名称 |
| `metric_type` | `DMSCMetricType` | 指标类型 |
| `value` | `f64` | 指标值 |
| `labels` | `HashMap<String, String>` | 标签 |

<div align="center">

## 分布式追踪

</div>

### DMSCTracer

分布式追踪器。

#### 使用示例

```rust
use dmsc::observability::DMSCTracer;

let tracer = DMSCTracer::new();
```

### DMSCTraceId

追踪ID。

### DMSCSpanId

跨度ID。

### DMSCSpanKind

跨度类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Server` | 服务器端跨度 |
| `Client` | 客户端跨度 |
| `Producer` | 生产者跨度 |
| `Consumer` | 消费者跨度 |
| `Internal` | 内部跨度 |

### DMSCSpanStatus

跨度状态枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Ok` | 成功 |
| `Error` | 错误 |
| `Unset` | 未设置 |

<div align="center">

## 最佳实践

</div>

1. **启用追踪**: 在分布式系统中启用分布式追踪
2. **合理采样**: 根据流量调整采样率
3. **添加标签**: 为指标添加有意义的标签
4. **监控关键指标**: 重点监控业务关键指标

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
