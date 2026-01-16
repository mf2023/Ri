<div align="center">

# Message Queue API Reference

**Version: 0.1.4**

**Last modified date: 2026-01-15**

The mq module provides message queue and event-driven functionality, supporting multiple message queue backends, publish-subscribe, delayed messages, and dead letter queues.

## Module Overview

</div>

The mq module includes the following sub-modules:

- **core**: Message queue core interfaces and type definitions
- **publishers**: Publisher implementations
- **consumers**: Consumer implementations
- **routing**: Message routing
- **dead_letter**: Dead letter queues

<div align="center">

## Core Components

</div>

### DMSCMessageQueue

Main interface for message queue manager, providing unified message queue access.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `publish(topic, message)` | Publish message | `topic: &str`, `message: impl Serialize` | `DMSCResult<()>` |
| `publish_delayed(topic, message, delay)` | Publish delayed message | `topic: &str`, `message: impl Serialize`, `delay: Duration` | `DMSCResult<()>` |
| `subscribe(topic, handler)` | Subscribe to messages | `topic: &str`, `handler: impl MessageHandler` | `DMSCResult<DMSCConsumer>` |
| `create_queue(name)` | Create queue | `name: &str` | `DMSCResult<()>` |
| `delete_queue(name)` | Delete queue | `name: &str` | `DMSCResult<()>` |
| `get_queue_stats(name)` | Get queue statistics | `name: &str` | `DMSCResult<QueueStats>` |
| `purge_queue(name)` | Purge queue | `name: &str` | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Publish message
let message = serde_json::json!({
    "type": "user_registered",
    "user_id": 12345,
    "email": "user@example.com",
    "timestamp": chrono::Utc::now()
});

ctx.mq().publish("user.events", message)?;

// Publish delayed message
ctx.mq().publish_delayed(
    "reminder.notifications",
    serde_json::json!({
        "type": "reminder",
        "user_id": 12345,
        "message": "Don't forget to complete your profile!"
    }),
    Duration::from_hours(24)
)?;

// Subscribe to messages
let consumer = ctx.mq().subscribe("user.events", |message: DMSCMessage| async move {
    match message.payload.get("type").and_then(|v| v.as_str()) {
        Some("user_registered") => {
            let user_id = message.payload["user_id"].as_i64().unwrap();
            ctx.log().info(format!("Processing user registration: {}", user_id));
            
            // Send welcome email
            send_welcome_email(user_id).await?;
        }
        Some("user_updated") => {
            let user_id = message.payload["user_id"].as_i64().unwrap();
            ctx.log().info(format!("Processing user update: {}", user_id));
            
            // Update user cache
            update_user_cache(user_id).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown message type: {:?}", message.payload));
        }
    }
    
    Ok(())
})?;
```

### DMSCMessageQueueConfig

Message queue configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `backend` | `DMSCMQBackend` | Message queue backend | `Memory` |
| `host` | `String` | Message queue host | `"localhost"` |
| `port` | `u16` | Message queue port | `5672` |
| `username` | `String` | Username | `"guest"` |
| `password` | `String` | Password | `"guest"` |
| `virtual_host` | `String` | Virtual host | `"/"` |
| `max_retries` | `u32` | Maximum retry count | `3` |
| `retry_delay` | `Duration` | Retry delay | `5s` |
| `prefetch_count` | `u16` | Prefetch count | `10` |

#### Configuration Example

```rust
use dmsc::prelude::*;

let mq_config = DMSCMessageQueueConfig {
    backend: DMSCMQBackend::RabbitMQ,
    host: "localhost".to_string(),
    port: 5672,
    username: "guest".to_string(),
    password: "guest".to_string(),
    virtual_host: "/".to_string(),
    max_retries: 5,
    retry_delay: Duration::from_secs(10),
    prefetch_count: 20,
};
```

### DMSCMQBackend

Message queue backend enumeration.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Memory` | In-memory queue |
| `RabbitMQ` | RabbitMQ |
| `Redis` | Redis queue |
| `ApacheKafka` | Apache Kafka |
| `AmazonSQS` | Amazon SQS |
| `GooglePubSub` | Google Cloud Pub/Sub |


<div align="center">

## Message Processing

</div>

### DMSCMessage

Message structure.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Message ID |
| `topic` | `String` | Message topic |
| `payload` | `serde_json::Value` | Message payload |
| `timestamp` | `DateTime<Utc>` | Message timestamp |
| `headers` | `HashMap<String, String>` | Message headers |
| `retry_count` | `u32` | Retry count |

### DMSCMessageHandler

Message handler trait.

```rust
use dmsc::prelude::*;

struct EmailNotificationHandler;

impl DMSCMessageHandler for EmailNotificationHandler {
    async fn handle(&self, message: DMSCMessage) -> DMSCResult<()> {
        let email_type = message.payload["type"].as_str().unwrap();
        let recipient = message.payload["recipient"].as_str().unwrap();
        let subject = message.payload["subject"].as_str().unwrap();
        let content = message.payload["content"].as_str().unwrap();
        
        ctx.log().info(format!(
            "Sending {} email to: {}",
            email_type, recipient
        ));
        
        // Send email
        send_email(recipient, subject, content).await?;
        
        ctx.log().info(format!("Email sent successfully to: {}", recipient));
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "email_notification_handler"
    }
}

// Register handler
ctx.mq().register_handler("email.notifications", EmailNotificationHandler)?;
```

### Message Acknowledgment

```rust
use dmsc::prelude::*;

let consumer = ctx.mq().subscribe("order.events", |message: DMSCMessage| async move {
    // Process message
    match process_order(message.payload.clone()).await {
        Ok(_) => {
            // Processing successful, acknowledge message
            message.ack()?;
            ctx.log().info("Order processed successfully");
        }
        Err(e) => {
            // Processing failed, reject message
            message.nack(false)?; // false means do not requeue
            ctx.log().error(format!("Order processing failed: {}", e));
        }
    }
    
    Ok(())
})?;

// Manual acknowledgment mode
let consumer = ctx.mq().subscribe_with_ack_mode(
    "important.events",
    AckMode::Manual,
    |message: DMSCMessage| async move {
        // Process message
        let result = process_important_event(message.payload.clone()).await;
        
        match result {
            Ok(_) => {
                // Manual acknowledgment
                message.ack()?;
            }
            Err(e) => {
                // Log error but do not acknowledge, message will be redelivered
                ctx.log().error(format!("Processing failed, will retry: {}", e));
            }
        }
        
        Ok(())
    }
)?;
```


<div align="center">

## Message Routing

</div>

### Routing Rules

```rust
use dmsc::prelude::*;

// Content-based routing
let router = DMSCMessageRouter::new()
    .when("user.events", |message: &DMSCMessage| {
        message.payload["type"].as_str() == Some("user_registered")
    })
    .route_to("email.welcome_queue")
    
    .when("user.events", |message: &DMSCMessage| {
        message.payload["type"].as_str() == Some("user_premium_upgraded")
    })
    .route_to("billing.premium_queue")
    
    .otherwise("user.general_queue");

// Use router
ctx.mq().set_router(router)?;
```

### Topic Wildcards

```rust
use dmsc::prelude::*;

// Subscribe to multiple topics
let consumer = ctx.mq().subscribe("user.*", |message: DMSCMessage| async move {
    match message.topic.as_str() {
        "user.created" => {
            ctx.log().info("New user created");
        }
        "user.updated" => {
            ctx.log().info("User updated");
        }
        "user.deleted" => {
            ctx.log().info("User deleted");
        }
        _ => {}
    }
    Ok(())
})?;

// Subscribe to all log topics
let log_consumer = ctx.mq().subscribe("logs.>", |message: DMSCMessage| async move {
    ctx.log().info(format!(
        "Log message from {}: {:?}",
        message.topic, message.payload
    ));
    Ok(())
})?;
```


<div align="center">

## Dead Letter Queues

</div>

### Configuring Dead Letter Queues

```rust
use dmsc::prelude::*;

// Configure dead letter queue when creating queue
let queue_config = DMSCQueueConfig {
    name: "order.processing",
    durable: true,
    auto_delete: false,
    dead_letter_exchange: "dlx.orders",
    dead_letter_routing_key: "failed.orders",
    max_retry_count: 3,
    message_ttl: Duration::from_hours(24),
    ..Default::default()
};

ctx.mq().create_queue_with_config(queue_config)?;

// Create dead letter queue consumer
let dlq_consumer = ctx.mq().subscribe("failed.orders", |message: DMSCMessage| async move {
    ctx.log().error(format!(
        "Processing failed message: {} (retry count: {})",
        message.id, message.retry_count
    ));
    
    // Analyze failure reason
    let error_info = message.payload.get("error").and_then(|v| v.as_str()).unwrap_or("Unknown error");
    
    match error_info {
        "invalid_data" => {
            // Invalid data, log and discard
            ctx.log().error("Invalid data, discarding message");
            message.ack()?;
        }
        "temporary_error" => {
            // Temporary error, can retry later
            ctx.log().warn("Temporary error, keeping in DLQ");
            // Do not acknowledge message, keep in dead letter queue
        }
        _ => {
            // Other errors, manual intervention required
            ctx.log().error("Unknown error, manual intervention required");
            // Send alert notification
            send_alert("Failed message requires manual intervention", &message).await?;
        }
    }
    
    Ok(())
})?;
```

### Dead Letter Queue Management

```rust
use dmsc::prelude::*;

// Redeliver dead letter messages
let redelivered_count = ctx.mq().redeliver_dead_letters("failed.orders", "order.processing")?;
ctx.log().info(format!("Redelivered {} messages", redelivered_count));

// Get dead letter queue statistics
let dlq_stats = ctx.mq().get_queue_stats("failed.orders")?;
println!("Dead letter queue stats:");
println!("  Messages: {}", dlq_stats.message_count);
println!("  Ready: {}", dlq_stats.ready_count);
println!("  Unacked: {}", dlq_stats.unacked_count);
```


<div align="center">

## Delayed Messages

</div>

### Delayed Queues

```rust
use dmsc::prelude::*;

// Create delayed queue
let delayed_queue_config = DMSCQueueConfig {
    name: "delayed.notifications",
    durable: true,
    auto_delete: false,
    message_ttl: Duration::from_minutes(5),
    dead_letter_exchange: "notifications",
    ..Default::default()
};

ctx.mq().create_queue_with_config(delayed_queue_config)?;

// Publish delayed message (implemented via TTL and DLX)
let delayed_message = serde_json::json!({
    "type": "scheduled_notification",
    "user_id": 12345,
    "content": "Your subscription expires in 3 days"
});

// Send directly to delayed queue, message will be automatically forwarded to target queue after TTL
ctx.mq().publish("delayed.notifications", delayed_message)?;
```

### Scheduled Tasks

```rust
use dmsc::prelude::*;
use chrono::{DateTime, Utc, Duration as ChronoDuration};

// Schedule task
fn schedule_task(task_name: &str, execute_at: DateTime<Utc>, payload: serde_json::Value) -> DMSCResult<()> {
    let now = Utc::now();
    let delay = execute_at - now;
    
    if delay > ChronoDuration::zero() {
        ctx.mq().publish_delayed(
            "scheduled.tasks",
            serde_json::json!({
                "task_name": task_name,
                "execute_at": execute_at,
                "payload": payload
            }),
            delay.to_std().unwrap()
        )?;
        
        ctx.log().info(format!(
            "Scheduled task '{}' for {}",
            task_name, execute_at
        ));
    } else {
        // Execute immediately
        ctx.mq().publish("scheduled.tasks", payload)?;
    }
    
    Ok(())
}

// Usage example
let tomorrow_at_9am = Utc::now() + ChronoDuration::days(1);
schedule_task(
    "send_daily_report",
    tomorrow_at_9am,
    serde_json::json!({
        "report_type": "daily_summary",
        "recipients": ["admin@example.com"]
    })
)?;
```


<div align="center">

## Message Persistence

</div>

### Persistence Configuration

```rust
use dmsc::prelude::*;

// Create persistent queue
let persistent_queue_config = DMSCQueueConfig {
    name: "critical.events",
    durable: true,           // Queue persistence
    auto_delete: false,      // Do not auto-delete
    message_persistent: true, // Message persistence
    max_priority: 10,        // Maximum priority
    ..Default::default()
};

ctx.mq().create_queue_with_config(persistent_queue_config)?;

// Publish persistent message
let critical_message = serde_json::json!({
    "type": "system_alert",
    "severity": "critical",
    "message": "Database connection lost"
});

ctx.mq().publish_persistent("critical.events", critical_message)?;
```

### Message Acknowledgment Modes

```rust
use dmsc::prelude::*;

// Auto acknowledgment mode (default)
let auto_ack_consumer = ctx.mq().subscribe("auto.ack.queue", |message: DMSCMessage| async move {
    // Message will be automatically acknowledged
    process_message(message.payload).await?;
    Ok(())
})?;

// Manual acknowledgment mode
let manual_ack_consumer = ctx.mq().subscribe_with_ack_mode(
    "manual.ack.queue",
    AckMode::Manual,
    |message: DMSCMessage| async move {
        match process_message(message.payload.clone()).await {
            Ok(_) => {
                // Manually acknowledge message
                message.ack()?;
            }
            Err(e) => {
                // Reject message and requeue
                message.nack(true)?;
                ctx.log().error(format!("Message processing failed: {}", e));
            }
        }
        Ok(())
    }
)?;

// Batch acknowledgment mode
let batch_ack_consumer = ctx.mq().subscribe_with_ack_mode(
    "batch.ack.queue",
    AckMode::Batch(100), // Acknowledge every 100 messages
    |messages: Vec<DMSCMessage>| async move {
        for message in &messages {
            process_message(message.payload.clone()).await?;
        }
        
        // Batch acknowledgment
        DMSCMessage::ack_batch(&messages)?;
        Ok(())
    }
)?;
```

<div align="center">

## Message Priority

</div>

### Priority Queues

```rust
use dmsc::prelude::*;

// Create priority queue
let priority_queue_config = DMSCQueueConfig {
    name: "priority.tasks",
    durable: true,
    max_priority: 10,  // Priority range 0-10
    ..Default::default()
};

ctx.mq().create_queue_with_config(priority_queue_config)?;

// Publish high priority message
let urgent_task = serde_json::json!({
    "type": "urgent_task",
    "description": "Fix critical bug in production"
});

ctx.mq().publish_with_priority("priority.tasks", urgent_task, 9)?;

// Publish normal priority message
let normal_task = serde_json::json!({
    "type": "normal_task",
    "description": "Update documentation"
});

ctx.mq().publish_with_priority("priority.tasks", normal_task, 5)?;

// Publish low priority message
let low_task = serde_json::json!({
    "type": "low_priority_task",
    "description": "Clean up old log files"
});

ctx.mq().publish_with_priority("priority.tasks", low_task, 1)?;
```

<div align="center">

## Message Filtering

</div>  

### Content Filtering

```rust
use dmsc::prelude::*;

// Create consumer with filter
let filtered_consumer = ctx.mq().subscribe_with_filter(
    "user.events",
    |message: &DMSCMessage| {
        // Only process specific types of user events
        message.payload.get("type")
            .and_then(|v| v.as_str())
            .map(|t| t == "user_registered" || t == "user_premium_upgraded")
            .unwrap_or(false)
    },
    |message: DMSCMessage| async move {
        ctx.log().info(format!("Processing filtered message: {:?}", message.payload));
        // Process messages that match the criteria
        Ok(())
    }
)?;
```

<div align="center">

## Configuration

</div>

### DMSCQueueConfig

Queue configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `name` | `String` | Queue name | Required |
| `durable` | `bool` | Queue durability | `true` |
| `auto_delete` | `bool` | Auto-delete | `false` |
| `exclusive` | `bool` | Exclusive queue | `false` |
| `max_priority` | `u8` | Maximum priority | `0` |
| `message_ttl` | `Duration` | Message TTL | Unlimited |
| `max_length` | `u32` | Maximum message count | Unlimited |
| `dead_letter_exchange` | `String` | Dead letter exchange | None |
| `dead_letter_routing_key` | `String` | Dead letter routing key | None |

<div align="center">

## Error Handling

</div>

### Message Queue Error Codes

| Error Code | Description |
|:--------|:-------------|
| `MQ_CONNECTION_ERROR` | Message queue connection error |
| `MQ_PUBLISH_ERROR` | Message publish error |
| `MQ_CONSUME_ERROR` | Message consume error |
| `MQ_QUEUE_ERROR` | Queue operation error |
| `MQ_ROUTING_ERROR` | Message routing error |

### Error Handling Example

```rust
use dmsc::prelude::*;

match ctx.mq().publish("user.events", message) {
    Ok(_) => {
        ctx.log().info("Message published successfully");
    }
    Err(DMSCError { code, .. }) if code == "MQ_CONNECTION_ERROR" => {
        // Connection error, attempt to reconnect
        ctx.log().error("MQ connection lost, attempting to reconnect");
        ctx.mq().reconnect()?;
        
        // Retry publish
        ctx.mq().publish("user.events", message)?;
    }
    Err(e) => {
        // Other errors, log and handle
        ctx.log().error(format!("Failed to publish message: {}", e));
        // Can save to local storage for retry later
        save_failed_message_to_local_storage(message)?;
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Message Idempotency**: Ensure message processing is idempotent to avoid issues caused by duplicate processing
2. **Reasonable Retry Settings**: Set appropriate retry count and retry intervals
3. **Use Dead Letter Queues**: Use dead letter queues for subsequent processing of messages that cannot be handled
4. **Message Size Limits**: Avoid sending oversized messages, use object storage for large content
5. **Monitor Queue Status**: Monitor metrics such as queue length and consumption rate
6. **Error Handling**: Comprehensive error handling and recovery mechanisms
7. **Message Version Control**: Version control message formats to support backward compatibility
8. **Resource Cleanup**: Clean up unused queues and consumers in a timely manner

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
