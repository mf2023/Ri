<div align="center">

# Message Queue API参考

**Version: 0.1.6**

**Last modified date: 2026-01-24**

mq模块提供消息队列与事件驱动功能，支持多种消息队列后端、发布订阅、延迟消息与死信队列。

## 模块概述

</div>

mq模块包含以下子模块：

- **core**: 消息队列核心接口和类型定义
- **publishers**: 发布者实现
- **consumers**: 消费者实现
- **routing**: 消息路由
- **dead_letter**: 死信队列

<div align="center">

## 核心组件

</div>

### DMSCMessageQueue

消息队列管理器主接口，提供统一的消息队列访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `publish(topic, message)` | 发布消息 | `topic: &str`, `message: impl Serialize` | `DMSCResult<()>` |
| `publish_delayed(topic, message, delay)` | 发布延迟消息 | `topic: &str`, `message: impl Serialize`, `delay: Duration` | `DMSCResult<()>` |
| `subscribe(topic, handler)` | 订阅消息 | `topic: &str`, `handler: impl MessageHandler` | `DMSCResult<DMSCConsumer>` |
| `create_queue(name)` | 创建队列 | `name: &str` | `DMSCResult<()>` |
| `delete_queue(name)` | 删除队列 | `name: &str` | `DMSCResult<()>` |
| `get_queue_stats(name)` | 获取队列统计 | `name: &str` | `DMSCResult<QueueStats>` |
| `purge_queue(name)` | 清空队列 | `name: &str` | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 发布消息
let message = serde_json::json!({
    "type": "user_registered",
    "user_id": 12345,
    "email": "user@example.com",
    "timestamp": chrono::Utc::now()
});

ctx.mq().publish("user.events", message)?;

// 发布延迟消息
ctx.mq().publish_delayed(
    "reminder.notifications",
    serde_json::json!({
        "type": "reminder",
        "user_id": 12345,
        "message": "Don't forget to complete your profile!"
    }),
    Duration::from_hours(24)
)?;

// 订阅消息
let consumer = ctx.mq().subscribe("user.events", |message: DMSCMessage| async move {
    match message.payload.get("type").and_then(|v| v.as_str()) {
        Some("user_registered") => {
            let user_id = message.payload["user_id"].as_i64().unwrap();
            ctx.log().info(format!("Processing user registration: {}", user_id));
            
            // 发送欢迎邮件
            send_welcome_email(user_id).await?;
        }
        Some("user_updated") => {
            let user_id = message.payload["user_id"].as_i64().unwrap();
            ctx.log().info(format!("Processing user update: {}", user_id));
            
            // 更新用户缓存
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

消息队列配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `backend` | `DMSCMQBackend` | 消息队列后端 | `Memory` |
| `host` | `String` | 消息队列主机 | `"localhost"` |
| `port` | `u16` | 消息队列端口 | `5672` |
| `username` | `String` | 用户名 | `"guest"` |
| `password` | `String` | 密码 | `"guest"` |
| `virtual_host` | `String` | 虚拟主机 | `"/"` |
| `max_retries` | `u32` | 最大重试次数 | `3` |
| `retry_delay` | `Duration` | 重试延迟 | `5s` |
| `prefetch_count` | `u16` | 预取数量 | `10` |

#### 配置示例

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

消息队列后端枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Memory` | 内存队列 |
| `RabbitMQ` | RabbitMQ |
| `Redis` | Redis队列 |
| `ApacheKafka` | Apache Kafka |
| `AmazonSQS` | Amazon SQS |
| `GooglePubSub` | Google Cloud Pub/Sub |


<div align="center">

## 消息处理

</div>

### DMSCMessage

消息结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 消息ID |
| `topic` | `String` | 消息主题 |
| `payload` | `serde_json::Value` | 消息负载 |
| `timestamp` | `DateTime<Utc>` | 消息时间戳 |
| `headers` | `HashMap<String, String>` | 消息头 |
| `retry_count` | `u32` | 重试次数 |

### DMSCMessageHandler

消息处理器trait。

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
        
        // 发送邮件
        send_email(recipient, subject, content).await?;
        
        ctx.log().info(format!("Email sent successfully to: {}", recipient));
        
        Ok(())
    }
    
    fn name(&self) -> &str {
        "email_notification_handler"
    }
}

// 注册处理器
ctx.mq().register_handler("email.notifications", EmailNotificationHandler)?;
```

### 消息确认

```rust
use dmsc::prelude::*;

let consumer = ctx.mq().subscribe("order.events", |message: DMSCMessage| async move {
    // 处理消息
    match process_order(message.payload.clone()).await {
        Ok(_) => {
            // 处理成功，确认消息
            message.ack()?;
            ctx.log().info("Order processed successfully");
        }
        Err(e) => {
            // 处理失败，拒绝消息
            message.nack(false)?; // false表示不重新入队
            ctx.log().error(format!("Order processing failed: {}", e));
        }
    }
    
    Ok(())
})?;

// 手动确认模式
let consumer = ctx.mq().subscribe_with_ack_mode(
    "important.events",
    AckMode::Manual,
    |message: DMSCMessage| async move {
        // 处理消息
        let result = process_important_event(message.payload.clone()).await;
        
        match result {
            Ok(_) => {
                // 手动确认
                message.ack()?;
            }
            Err(e) => {
                // 记录错误但不确认，消息会重新投递
                ctx.log().error(format!("Processing failed, will retry: {}", e));
            }
        }
        
        Ok(())
    }
)?;
```


<div align="center">

## 消息路由

</div>

### 路由规则

```rust
use dmsc::prelude::*;

// 基于内容的路由
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

// 使用路由器
ctx.mq().set_router(router)?;
```

### 主题通配符

```rust
use dmsc::prelude::*;

// 订阅多个主题
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

// 订阅所有日志主题
let log_consumer = ctx.mq().subscribe("logs.>", |message: DMSCMessage| async move {
    ctx.log().info(format!(
        "Log message from {}: {:?}",
        message.topic, message.payload
    ));
    Ok(())
})?;
```


<div align="center">

## 死信队列

</div>

### 配置死信队列

```rust
use dmsc::prelude::*;

// 创建队列时配置死信队列
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

// 创建死信队列消费者
let dlq_consumer = ctx.mq().subscribe("failed.orders", |message: DMSCMessage| async move {
    ctx.log().error(format!(
        "Processing failed message: {} (retry count: {})",
        message.id, message.retry_count
    ));
    
    // 分析失败原因
    let error_info = message.payload.get("error").and_then(|v| v.as_str()).unwrap_or("Unknown error");
    
    match error_info {
        "invalid_data" => {
            // 数据无效，记录并丢弃
            ctx.log().error("Invalid data, discarding message");
            message.ack()?;
        }
        "temporary_error" => {
            // 临时错误，可以稍后重试
            ctx.log().warn("Temporary error, keeping in DLQ");
            // 不确认消息，保持在死信队列中
        }
        _ => {
            // 其他错误，人工处理
            ctx.log().error("Unknown error, manual intervention required");
            // 发送告警通知
            send_alert("Failed message requires manual intervention", &message).await?;
        }
    }
    
    Ok(())
})?;
```

### 死信队列管理

```rust
use dmsc::prelude::*;

// 重新投递死信消息
let redelivered_count = ctx.mq().redeliver_dead_letters("failed.orders", "order.processing")?;
ctx.log().info(format!("Redelivered {} messages", redelivered_count));

// 获取死信队列统计
let dlq_stats = ctx.mq().get_queue_stats("failed.orders")?;
println!("Dead letter queue stats:");
println!("  Messages: {}", dlq_stats.message_count);
println!("  Ready: {}", dlq_stats.ready_count);
println!("  Unacked: {}", dlq_stats.unacked_count);
```


<div align="center">

## 延迟消息

</div>

### 延迟队列

```rust
use dmsc::prelude::*;

// 创建延迟队列
let delayed_queue_config = DMSCQueueConfig {
    name: "delayed.notifications",
    durable: true,
    auto_delete: false,
    message_ttl: Duration::from_minutes(5),
    dead_letter_exchange: "notifications",
    ..Default::default()
};

ctx.mq().create_queue_with_config(delayed_queue_config)?;

// 发布延迟消息（通过TTL和DLX实现）
let delayed_message = serde_json::json!({
    "type": "scheduled_notification",
    "user_id": 12345,
    "content": "Your subscription expires in 3 days"
});

// 直接发送到延迟队列，消息会在TTL后自动转发到目标队列
ctx.mq().publish("delayed.notifications", delayed_message)?;
```

### 定时任务

```rust
use dmsc::prelude::*;
use chrono::{DateTime, Utc, Duration as ChronoDuration};

// 安排定时任务
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
        // 立即执行
        ctx.mq().publish("scheduled.tasks", payload)?;
    }
    
    Ok(())
}

// 使用示例
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

## 消息持久化

</div>

### 持久化配置

```rust
use dmsc::prelude::*;

// 创建持久化队列
let persistent_queue_config = DMSCQueueConfig {
    name: "critical.events",
    durable: true,           // 队列持久化
    auto_delete: false,      // 不自动删除
    message_persistent: true, // 消息持久化
    max_priority: 10,        // 最大优先级
    ..Default::default()
};

ctx.mq().create_queue_with_config(persistent_queue_config)?;

// 发布持久化消息
let critical_message = serde_json::json!({
    "type": "system_alert",
    "severity": "critical",
    "message": "Database connection lost"
});

ctx.mq().publish_persistent("critical.events", critical_message)?;
```

### 消息确认模式

```rust
use dmsc::prelude::*;

// 自动确认模式（默认）
let auto_ack_consumer = ctx.mq().subscribe("auto.ack.queue", |message: DMSCMessage| async move {
    // 消息会自动确认
    process_message(message.payload).await?;
    Ok(())
})?;

// 手动确认模式
let manual_ack_consumer = ctx.mq().subscribe_with_ack_mode(
    "manual.ack.queue",
    AckMode::Manual,
    |message: DMSCMessage| async move {
        match process_message(message.payload.clone()).await {
            Ok(_) => {
                // 手动确认消息
                message.ack()?;
            }
            Err(e) => {
                // 拒绝消息并重新入队
                message.nack(true)?;
                ctx.log().error(format!("Message processing failed: {}", e));
            }
        }
        Ok(())
    }
)?;

// 批量确认模式
let batch_ack_consumer = ctx.mq().subscribe_with_ack_mode(
    "batch.ack.queue",
    AckMode::Batch(100), // 每100条消息确认一次
    |messages: Vec<DMSCMessage>| async move {
        for message in &messages {
            process_message(message.payload.clone()).await?;
        }
        
        // 批量确认
        DMSCMessage::ack_batch(&messages)?;
        Ok(())
    }
)?;
```

<div align="center">

## 消息优先级

</div>

### 优先级队列

```rust
use dmsc::prelude::*;

// 创建优先级队列
let priority_queue_config = DMSCQueueConfig {
    name: "priority.tasks",
    durable: true,
    max_priority: 10,  // 优先级范围 0-10
    ..Default::default()
};

ctx.mq().create_queue_with_config(priority_queue_config)?;

// 发布高优先级消息
let urgent_task = serde_json::json!({
    "type": "urgent_task",
    "description": "Fix critical bug in production"
});

ctx.mq().publish_with_priority("priority.tasks", urgent_task, 9)?;

// 发布普通优先级消息
let normal_task = serde_json::json!({
    "type": "normal_task",
    "description": "Update documentation"
});

ctx.mq().publish_with_priority("priority.tasks", normal_task, 5)?;

// 发布低优先级消息
let low_task = serde_json::json!({
    "type": "low_priority_task",
    "description": "Clean up old log files"
});

ctx.mq().publish_with_priority("priority.tasks", low_task, 1)?;
```

<div align="center">

## 消息过滤

</div>  

### 内容过滤

```rust
use dmsc::prelude::*;

// 创建带过滤器的消费者
let filtered_consumer = ctx.mq().subscribe_with_filter(
    "user.events",
    |message: &DMSCMessage| {
        // 只处理特定类型的用户事件
        message.payload.get("type")
            .and_then(|v| v.as_str())
            .map(|t| t == "user_registered" || t == "user_premium_upgraded")
            .unwrap_or(false)
    },
    |message: DMSCMessage| async move {
        ctx.log().info(format!("Processing filtered message: {:?}", message.payload));
        // 处理符合条件的消息
        Ok(())
    }
)?;
```

<div align="center">

## 配置

</div>

### DMSCQueueConfig

队列配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `name` | `String` | 队列名称 | 必填 |
| `durable` | `bool` | 队列持久化 | `true` |
| `auto_delete` | `bool` | 自动删除 | `false` |
| `exclusive` | `bool` | 独占队列 | `false` |
| `max_priority` | `u8` | 最大优先级 | `0` |
| `message_ttl` | `Duration` | 消息TTL | 无限制 |
| `max_length` | `u32` | 最大消息数 | 无限制 |
| `dead_letter_exchange` | `String` | 死信交换机 | 无 |
| `dead_letter_routing_key` | `String` | 死信路由键 | 无 |

<div align="center">

## 错误处理

</div>

### 消息队列错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `MQ_CONNECTION_ERROR` | 消息队列连接错误 |
| `MQ_PUBLISH_ERROR` | 消息发布错误 |
| `MQ_CONSUME_ERROR` | 消息消费错误 |
| `MQ_QUEUE_ERROR` | 队列操作错误 |
| `MQ_ROUTING_ERROR` | 消息路由错误 |

### 错误处理示例

```rust
use dmsc::prelude::*;

match ctx.mq().publish("user.events", message) {
    Ok(_) => {
        ctx.log().info("Message published successfully");
    }
    Err(DMSCError { code, .. }) if code == "MQ_CONNECTION_ERROR" => {
        // 连接错误，尝试重新连接
        ctx.log().error("MQ connection lost, attempting to reconnect");
        ctx.mq().reconnect()?;
        
        // 重试发布
        ctx.mq().publish("user.events", message)?;
    }
    Err(e) => {
        // 其他错误，记录并处理
        ctx.log().error(format!("Failed to publish message: {}", e));
        // 可以保存到本地存储，稍后重试
        save_failed_message_to_local_storage(message)?;
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **消息幂等性**: 确保消息处理是幂等的，避免重复处理导致的问题
2. **合理设置重试**: 设置适当的重试次数和重试间隔
3. **使用死信队列**: 对于无法处理的消息，使用死信队列进行后续处理
4. **消息大小限制**: 避免发送过大的消息，大内容使用对象存储
5. **监控队列状态**: 监控队列长度、消费速率等指标
6. **错误处理**: 完善的错误处理和恢复机制
7. **消息版本控制**: 对消息格式进行版本控制，支持向后兼容
8. **资源清理**: 及时清理不再使用的队列和消费者

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
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信