<div align="center">

# Queue API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-30**

The queue module provides a comprehensive queueing system with support for multiple backend implementations, enabling reliable message passing and task scheduling across distributed systems.

## Module Overview

</div>

The queue module includes the following components:

- **core**: Core queue interfaces and types
- **backends**: Backend implementations (Memory, Redis, RabbitMQ, Kafka)
- **config**: Queue configuration
- **manager**: Queue management

<div align="center">

## Core Components

</div>

### DMSCQueueModule

Main queue module implementing service module traits.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create queue module | `config: DMSCQueueConfig` | `DMSCResult<Self>` |
| `queue_manager()` | Get queue manager | None | `Arc<RwLock<DMSCQueueManager>>` |

#### Usage Example

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

Central queue management component.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `queue(name)` | Get queue instance | `name: &str` | `DMSCResult<Arc<dyn DMSCQueue>>` |
| `create_queue(name, config)` | Create new queue | `name: &str`, `config: DMSCQueueConfig` | `DMSCResult<()>` |
| `delete_queue(name)` | Delete queue | `name: &str` | `DMSCResult<()>` |
| `list_queues()` | List all queues | None | `Vec<String>` |

### DMSCQueue

Unified queue interface implemented by all backends.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `producer()` | Create producer | None | `DMSCResult<Arc<dyn DMSCQueueProducer>>` |
| `consumer()` | Create consumer | None | `DMSCResult<Arc<dyn DMSCQueueConsumer>>` |
| `stats()` | Get queue statistics | None | `DMSCResult<DMSCQueueStats>` |
| `purge()` | Purge all messages | None | `DMSCResult<()>` |

### DMSCQueueProducer

Interface for producing messages to queues.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `send(message)` | Send message | `message: impl Serialize` | `DMSCResult<String>` (message ID) |
| `send_batch(messages)` | Send batch messages | `messages: Vec<impl Serialize>` | `DMSCResult<Vec<String>>` |

### DMSCQueueConsumer

Interface for consuming messages from queues.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `receive()` | Receive message | None | `DMSCResult<Option<DMSCQueueMessage>>` |
| `receive_batch(max_count)` | Receive batch messages | `max_count: usize` | `DMSCResult<Vec<DMSCQueueMessage>>` |
| `ack(message_id)` | Acknowledge message | `message_id: &str` | `DMSCResult<()>` |
| `nack(message_id)` | Negative acknowledge | `message_id: &str` | `DMSCResult<()>` |
| `pause()` | Pause consumption | None | `DMSCResult<()>` |
| `resume()` | Resume consumption | None | `DMSCResult<()>` |

### DMSCQueueMessage

Message structure for queue operations.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Unique message ID |
| `payload` | `Vec<u8>` | Message payload |
| `headers` | `HashMap<String, String>` | Message headers |
| `timestamp` | `SystemTime` | Creation timestamp |
| `retry_count` | `u32` | Current retry count |
| `max_retries` | `u32` | Maximum retry attempts |

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(payload)` | Create new message | `payload: Vec<u8>` | `Self` |
| `with_headers(headers)` | Add headers | `headers: HashMap<String, String>` | `Self` |
| `with_max_retries(max)` | Set max retries | `max: u32` | `Self` |
| `increment_retry()` | Increment retry count | None | `()` |
| `can_retry()` | Check if can retry | None | `bool` |

<div align="center">

## Configuration

</div>

### DMSCQueueConfig

Queue configuration structure.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `enabled` | `bool` | Enable queue | `true` |
| `backend_type` | `DMSCQueueBackendType` | Backend type | `Memory` |
| `connection_string` | `String` | Connection string | `"memory://localhost"` |
| `max_connections` | `u32` | Max connections | `10` |
| `message_max_size` | `usize` | Max message size (bytes) | `1048576` (1MB) |
| `consumer_timeout_ms` | `u64` | Consumer timeout (ms) | `30000` |
| `producer_timeout_ms` | `u64` | Producer timeout (ms) | `5000` |
| `retry_policy` | `DMSCRetryPolicy` | Retry policy | Default |
| `dead_letter_config` | `Option<DMSCDeadLetterConfig>` | Dead letter config | `None` |

### DMSCQueueBackendType

Backend type enum.

| Variant | Description |
|:--------|:-------------|
| `Memory` | In-memory queue |
| `Redis` | Redis-backed queue |
| `RabbitMQ` | RabbitMQ queue |
| `Kafka` | Kafka queue |

<div align="center">

## Backends

</div>

### Memory Backend

In-memory queue for development and testing.

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Memory,
    ..Default::default()
};
```

### Redis Backend

Redis-backed queue for production use.

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Redis,
    connection_string: "redis://localhost:6379".to_string(),
    ..Default::default()
};
```

### RabbitMQ Backend

RabbitMQ queue for enterprise messaging.

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::RabbitMQ,
    connection_string: "amqp://guest:guest@localhost:5672".to_string(),
    ..Default::default()
};
```

### Kafka Backend

Kafka queue for high-throughput streaming (requires `kafka` feature).

```rust
use dmsc::queue::{DMSCQueueConfig, DMSCQueueBackendType};

let config = DMSCQueueConfig {
    backend_type: DMSCQueueBackendType::Kafka,
    connection_string: "kafka://localhost:9092".to_string(),
    ..Default::default()
};
```

<div align="center">

## Statistics

</div>

### DMSCQueueStats

Queue statistics for monitoring.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `queue_name` | `String` | Queue name |
| `message_count` | `u64` | Current message count |
| `consumer_count` | `u32` | Active consumer count |
| `producer_count` | `u32` | Active producer count |
| `processed_messages` | `u64` | Total processed messages |
| `failed_messages` | `u64` | Total failed messages |
| `avg_processing_time_ms` | `f64` | Average processing time (ms) |
| `total_bytes_sent` | `u64` | Total bytes sent |
| `total_bytes_received` | `u64` | Total bytes received |
| `last_message_time` | `u64` | Last message timestamp |

<div align="center">

## Best Practices

</div>

1. **Choose appropriate backend**: Use Memory for development, Redis/RabbitMQ for production
2. **Configure retry policy**: Set appropriate retry count and delay
3. **Monitor statistics**: Track queue metrics for performance tuning
4. **Handle errors**: Properly handle queue errors and implement dead letter queues
5. **Use batch operations**: Batch send/receive for better performance

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary
- [core](./core.md): Core module providing error handling and service context
- [config](./config.md): Configuration module managing application configuration
- [observability](./observability.md): Observability module for queue monitoring
