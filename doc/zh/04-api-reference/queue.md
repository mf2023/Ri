<div align="center">

# Queue API 参考

**版本: 0.1.7**

**最后修改日期: 2026-02-11**

队列模块提供全面的队列系统，支持多种后端实现，实现分布式系统中可靠的消息传递和任务调度。

## 模块概述

</div>

队列模块包含以下组件：

- **core**: 核心队列接口和类型
- **backends**: 后端实现（内存、Redis、RabbitMQ、Kafka）
- **config**: 队列配置
- **manager**: 队列管理

<div align="center">

## 核心组件

</div>

### DMSCQueueModule

主队列模块，实现服务模块特性。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建队列模块 | `config: DMSCQueueConfig` | `DMSCResult<Self>` |
| `queue_manager()` | 获取队列管理器 | 无 | `Arc<RwLock<DMSCQueueManager>>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let queue_config = DMSCQueueConfig {
    enabled: true,
    backend_type: DMSCQueueBackendType::Memory,
    connection_string: "memory://localhost".to_string(),
    ..Default::default()
};

let queue_module = DMSCQueueModule::new(queue_config).await?;
```

### DMSCQueueManager

中央队列管理组件。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `queue(name)` | 获取队列实例 | `name: &str` | `DMSCResult<Arc<dyn DMSCQueue>>` |
| `create_queue(name, config)` | 创建新队列 | `name: &str`, `config: DMSCQueueConfig` | `DMSCResult<()>` |
| `delete_queue(name)` | 删除队列 | `name: &str` | `DMSCResult<()>` |
| `list_queues()` | 列出所有队列 | 无 | `Vec<String>` |

### DMSCQueue

所有后端实现的统一队列接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `producer()` | 创建生产者 | 无 | `DMSCResult<Arc<dyn DMSCQueueProducer>>` |
| `consumer()` | 创建消费者 | 无 | `DMSCResult<Arc<dyn DMSCQueueConsumer>>` |
| `stats()` | 获取队列统计 | 无 | `DMSCResult<DMSCQueueStats>` |
| `purge()` | 清空所有消息 | 无 | `DMSCResult<()>` |

### DMSCQueueProducer

向队列生产消息的接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `send(message)` | 发送消息 | `message: impl Serialize` | `DMSCResult<String>` (消息ID) |
| `send_batch(messages)` | 批量发送消息 | `messages: Vec<impl Serialize>` | `DMSCResult<Vec<String>>` |

### DMSCQueueConsumer

从队列消费消息的接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `receive()` | 接收消息 | 无 | `DMSCResult<Option<DMSCQueueMessage>>` |
| `receive_batch(max_count)` | 批量接收消息 | `max_count: usize` | `DMSCResult<Vec<DMSCQueueMessage>>` |

### DMSCQueueMessage

队列操作的消息结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 唯一消息ID |
| `payload` | `Vec<u8>` | 消息负载 |
| `headers` | `HashMap<String, String>` | 消息头 |
| `timestamp` | `SystemTime` | 创建时间戳 |
| `retry_count` | `u32` | 当前重试次数 |
| `max_retries` | `u32` | 最大重试次数 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(payload)` | 创建新消息 | `payload: Vec<u8>` | `Self` |
| `with_headers(headers)` | 添加头 | `headers: HashMap<String, String>` | `Self` |
| `with_max_retries(max)` | 设置最大重试 | `max: u32` | `Self` |
| `increment_retry()` | 增加重试计数 | 无 | `()` |
| `can_retry()` | 检查是否可以重试 | 无 | `bool` |

<div align="center">

## 配置

</div>

### DMSCQueueConfig

队列配置结构。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `enabled` | `bool` | 启用队列 | `true` |
| `backend_type` | `DMSCQueueBackendType` | 后端类型 | `Memory` |
| `connection_string` | `String` | 连接字符串 | `"memory://localhost"` |
| `max_connections` | `u32` | 最大连接数 | `10` |
| `message_max_size` | `usize` | 最大消息大小（字节） | `1048576` (1MB) |
| `consumer_timeout_ms` | `u64` | 消费者超时（毫秒） | `30000` |
| `producer_timeout_ms` | `u64` | 生产者超时（毫秒） | `5000` |
| `retry_policy` | `DMSCRetryPolicy` | 重试策略 | 默认 |
| `dead_letter_config` | `Option<DMSCDeadLetterConfig>` | 死信队列配置 | `None` |

### DMSCQueueBackendType

后端类型枚举。

| 变体 | 描述 |
|:--------|:-------------|
| `Memory` | 内存队列 |
| `Redis` | Redis 后端队列 |
| `RabbitMQ` | RabbitMQ 队列 |
| `Kafka` | Kafka 队列 |

<div align="center">

## 后端

</div>

### 内存后端

内存队列，用于开发和测试。

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Memory,
    ..Default::default()
};
```

### Redis 后端

Redis 后端队列，用于生产环境。

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Redis,
    connection_string: "redis://localhost:6379".to_string(),
    ..Default::default()
};
```

### RabbitMQ 后端

RabbitMQ 队列，用于企业消息传递。

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::RabbitMQ,
    connection_string: "amqp://guest:guest@localhost:5672".to_string(),
    ..Default::default()
};
```

### Kafka 后端

Kafka 队列，用于高吞吐量流处理（需要 `kafka` 特性）。

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Kafka,
    connection_string: "kafka://localhost:9092".to_string(),
    ..Default::default()
};
```

<div align="center">

## 统计

</div>

### DMSCQueueStats

队列统计，用于监控。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `queue_name` | `String` | 队列名称 |
| `message_count` | `u64` | 当前消息数 |
| `consumer_count` | `u32` | 活跃消费者数 |
| `producer_count` | `u32` | 活跃生产者数 |
| `processed_messages` | `u64` | 总处理消息数 |
| `failed_messages` | `u64` | 总失败消息数 |
| `avg_processing_time_ms` | `f64` | 平均处理时间（毫秒） |
| `total_bytes_sent` | `u64` | 总发送字节数 |
| `total_bytes_received` | `u64` | 总接收字节数 |
| `last_message_time` | `u64` | 最后消息时间戳 |

<div align="center">

## 最佳实践

</div>

1. **选择适当的后端**：开发用内存，生产用 Redis/RabbitMQ
2. **配置重试策略**：设置适当的重试次数和延迟
3. **监控统计**：跟踪队列指标进行性能调优
4. **处理错误**：正确处理队列错误并实现死信队列
5. **使用批量操作**：批量发送/接收以提高性能

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览和API参考摘要
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [config](./config.md): 配置模块，管理应用配置
- [observability](./observability.md): 可观测性模块，用于队列监控
