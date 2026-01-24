<div align="center">

# Message Queue Usage Examples

**Version: 0.1.6**

**Last modified date: 2026-01-24**

This example demonstrates how to use DMSC's mq module for message queues, publish-subscribe, routing, dead letter queues, delayed messages, persistence, priority, and filtering functionality.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- RabbitMQ, Kafka, Redis Streams message queues
- Publish-subscribe and message routing
- Dead letter queues and delayed messages
- Message persistence and priority
- Message filtering and acknowledgment mechanisms
- Error handling and retry strategies

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of message queue basic concepts
- (Optional) RabbitMQ, Kafka, or Redis server

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-mq-example
cd dms-mq-example
```

### 2. Add Dependencies

Add the following dependencies in the `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
chrono = "0.4"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

```yaml
service:
  name: "dms-mq-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

message_queue:
  broker_type: "rabbitmq"
  rabbitmq:
    host: "localhost"
    port: 5672
    username: "guest"
    password: "guest"
    virtual_host: "/"
    connection_timeout: 30
    heartbeat_interval: 60
    prefetch_count: 10
    enable_ssl: false
  kafka:
    host: "localhost"
    port: 9092
    connection_timeout: 30
    heartbeat_interval: 30
    prefetch_count: 100
  redis:
    host: "localhost"
    port: 6379
    connection_timeout: 10
    heartbeat_interval: 30
    prefetch_count: 50
```

### 4. Write Main Code

Replace the `src/main.rs` file with the following content:

```rust
use dmsc::prelude::*;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_message_queue(DMSCMessageQueueConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Message Queue Example started")?;
        
        // Initialize message queue
        initialize_message_queue(&ctx).await?;
        
        // Publish sample messages
        publish_sample_messages(&ctx).await?;
        
        // Subscribe to message queues
        subscribe_to_queues(&ctx).await?;
        
        // Keep service running
        tokio::signal::ctrl_c().await?;
        ctx.logger().info("service", "Shutting down message queue service")?;
        
        Ok(())
    }).await
}

async fn initialize_message_queue(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("mq", "Initializing message queue")?;
    
    // RabbitMQ configuration
    let mq_config = DMSCMessageQueueConfig {
        broker_type: DMSCMessageBrokerType::RabbitMQ,
        host: "localhost".to_string(),
        port: 5672,
        username: Some("guest".to_string()),
        password: Some("guest".to_string()),
        virtual_host: Some("/".to_string()),
        connection_timeout: Duration::from_secs(30),
        heartbeat_interval: Duration::from_secs(60),
        prefetch_count: 10,
        enable_ssl: false,
        ssl_cert: None,
        ssl_key: None,
        ssl_ca_cert: None,
    };
    
    // Initialize message queue
    ctx.mq().init(mq_config).await?;
    ctx.logger().info("mq", "Message queue initialized")?;
    
    // Test connection
    match ctx.mq().ping().await {
        Ok(_) => ctx.logger().info("mq", "Message queue connection successful")?,
        Err(e) => {
            ctx.logger().error("mq", &format!("Message queue connection failed: {}", e))?;
            return Err(e);
        }
    }
    
    Ok(())
}

async fn publish_sample_messages(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("mq", "Publishing sample messages")?;
    
    // Create a queue first
    ctx.mq().create_queue("user_registrations").await?;
    ctx.logger().info("mq", "Queue created: user_registrations")?;
    
    // Publish message
    let message = json!({
        "user_id": 12345,
        "email": "newuser@example.com",
        "name": "John Doe",
        "registration_time": Utc::now().to_rfc3339(),
    });
    
    ctx.mq().push("user_registrations", &message).await?;
    ctx.logger().info("mq", "Message published to user_registrations queue")?;
    
    Ok(())
}

async fn subscribe_to_queues(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("mq", "Subscribing to message queues")?;
    
    // Subscribe to user registration queue
    loop {
        match ctx.mq().pop::<serde_json::Value>("user_registrations").await {
            Ok(Some(message)) => {
                ctx.logger().info("mq", &format!("Received message: {:?}", message))?;
                
                match process_user_registration(&message, &ctx).await {
                    Ok(result) => {
                        ctx.logger().info("mq", &format!("User registration processed: {:?}", result))?;
                    }
                    Err(e) => {
                        ctx.logger().error("mq", &format!("Failed to process user registration: {}", e))?;
                        // Re-push to queue
                        ctx.mq().push("user_registrations", &message).await?;
                    }
                }
            }
            Ok(None) => {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                ctx.logger().error("mq", &format!("Failed to receive message: {}", e))?;
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
    
    ctx.logger().info("mq", "Message queue subscriptions configured")?;
    
    Ok(())
}

async fn process_user_registration(message: &serde_json::Value, ctx: &DMSCServiceContext) -> DMSCResult<serde_json::Value> {
    let user_id = message["user_id"].as_i64().unwrap_or(0);
    let email = message["email"].as_str().unwrap_or_default();
    let name = message["name"].as_str().unwrap_or_default();
    
    // Validate user data
    if user_id == 0 || email.is_empty() || name.is_empty() {
        return Err(DMSCError::validation("Invalid user data".to_string()));
    }
    
    // Business logic processing can be added here
    ctx.logger().info("mq", &format!("Processing user registration for: {}", email))?;
    
    Ok(json!({
        "status": "success",
        "user_id": user_id,
        "email": email,
        "processed_at": Utc::now().to_rfc3339(),
    }))
}
```

<div align="center">

## Code Analysis

</div>

The mq module provides usage examples for message queues, publish-subscribe, routing, dead letter queues, delayed messages, persistence, priority, and filtering functionality.

## Basic Message Queue Operations

### Connection and Configuration

```rust
use dmsc::prelude::*;
use serde_json::json;

// RabbitMQ configuration
let rabbitmq_config = DMSCMessageQueueConfig {
    broker_type: DMSCMessageBrokerType::RabbitMQ,
    host: "localhost".to_string(),
    port: 5672,
    username: "guest".to_string(),
    password: "guest".to_string(),
    virtual_host: "/".to_string(),
    connection_timeout: Duration::from_secs(30),
    heartbeat_interval: Duration::from_secs(60),
    prefetch_count: 10,
    enable_ssl: false,
    ssl_cert: None,
    ssl_key: None,
    ssl_ca_cert: None,
};

// Apache Kafka configuration
let kafka_config = DMSCMessageQueueConfig {
    broker_type: DMSCMessageBrokerType::Kafka,
    host: "localhost".to_string(),
    port: 9092,
    username: None,
    password: None,
    virtual_host: None,
    connection_timeout: Duration::from_secs(30),
    heartbeat_interval: Duration::from_secs(30),
    prefetch_count: 100,
    enable_ssl: false,
    ssl_cert: None,
    ssl_key: None,
    ssl_ca_cert: None,
};

// Redis Streams configuration
let redis_config = DMSCMessageQueueConfig {
    broker_type: DMSCMessageBrokerType::Redis,
    host: "localhost".to_string(),
    port: 6379,
    username: None,
    password: None,
    virtual_host: None,
    connection_timeout: Duration::from_secs(10),
    heartbeat_interval: Duration::from_secs(30),
    prefetch_count: 50,
    enable_ssl: false,
    ssl_cert: None,
    ssl_key: None,
    ssl_ca_cert: None,
};

// Initialize message queue
ctx.mq().init(rabbitmq_config).await?;
ctx.log().info("Message queue initialized");

// Test connection
match ctx.mq().ping().await {
    Ok(_) => ctx.log().info("Message queue connection successful"),
    Err(e) => {
        ctx.log().error(format!("Message queue connection failed: {}", e));
        return Err(e);
    }
}
```

### Basic Publish and Subscribe

```rust
use dmsc::prelude::*;
use serde_json::json;

// Publish message to queue
let message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "user.registrations".to_string(),
    routing_key: "user.created".to_string(),
    body: json!({
        "user_id": 12345,
        "email": "newuser@example.com",
        "name": "John Doe",
        "registration_time": chrono::Utc::now().to_rfc3339(),
    }),
    headers: {
        let mut h = std::collections::HashMap::new();
        h.insert("source".to_string(), "web_app".to_string());
        h.insert("version".to_string(), "1.0".to_string());
        h.insert("correlation_id".to_string(), uuid::Uuid::new_v4().to_string());
        h
    },
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: Some(uuid::Uuid::new_v4().to_string()),
    reply_to: Some("user.registration.responses".to_string()),
};

ctx.mq().publish(message).await?;
ctx.log().info("User registration message published");

// Subscribe to queue messages
ctx.mq().subscribe("user.registrations", |message, ctx| async move {
    ctx.log().info(format!("Received message: {:?}", message));
    
    match process_user_registration(&message, &ctx).await {
        Ok(result) => {
            ctx.log().info(format!("User registration processed: {:?}", result));
            
            // Send confirmation response
            if let Some(reply_to) = &message.reply_to {
                let response = DMSCMessage {
                    id: uuid::Uuid::new_v4().to_string(),
                    queue: reply_to.clone(),
                    routing_key: "user.registration.completed".to_string(),
                    body: json!({
                        "status": "success",
                        "user_id": message.body["user_id"],
                        "message_id": message.id,
                        "processed_at": chrono::Utc::now().to_rfc3339(),
                    }),
                    headers: message.headers.clone(),
                    priority: DMSCMessagePriority::Normal,
                    delivery_mode: DMSCMessageDeliveryMode::Persistent,
                    timestamp: chrono::Utc::now(),
                    expiration: None,
                    correlation_id: message.correlation_id.clone(),
                    reply_to: None,
                };
                
                ctx.mq().publish(response).await?;
            }
            
            // Acknowledge message processed
            ctx.mq().ack(&message).await?;
        }
        Err(e) => {
            ctx.log().error(format!("Failed to process user registration: {}", e));
            
            // Reject message and requeue
            ctx.mq().nack(&message, true).await?;
        }
    }
    
    Ok(())
}).await?;

async fn process_user_registration(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let email = message.body["email"].as_str().unwrap_or_default();
    let name = message.body["name"].as_str().unwrap_or_default();
    
    // Validate user data
    if user_id == 0 || email.is_empty() || name.is_empty() {
        return Err(DMSCError::validation("Invalid user data".to_string()));
    }
    
    // Check if email already exists
    let existing_user = ctx.database()
        .query_one("SELECT id FROM users WHERE email = $1", vec![email.into()])
        .await?;
    
    if existing_user.is_some() {
        return Err(DMSCError::business("Email already registered".to_string()));
    }
    
    // Create user record
    let new_user_id = ctx.database()
        .execute(
            "INSERT INTO users (email, name, created_at) VALUES ($1, $2, $3) RETURNING id",
            vec![email.into(), name.into(), chrono::Utc::now().to_rfc3339().into()]
        )
        .await?;
    
    // Send welcome email
    let email_message = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "email.welcome".to_string(),
        routing_key: "email.welcome.send".to_string(),
        body: json!({
            "to": email,
            "subject": "Welcome to our service!",
            "template": "welcome_email",
            "data": {
                "user_name": name,
                "user_id": new_user_id,
            }
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::Normal,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    };
    
    ctx.mq().publish(email_message).await?;
    
    Ok(json!({
        "user_id": new_user_id,
        "email": email,
        "status": "registered"
    }))
}
```

## Publish-Subscribe Pattern

### Topic Publishing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Publish to topic
let topic_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(), // Topic name
    routing_key: "user.activity.login".to_string(),
    body: json!({
        "user_id": 12345,
        "login_time": chrono::Utc::now().to_rfc3339(),
        "ip_address": "192.168.1.1",
        "user_agent": "Mozilla/5.0...",
    }),
    headers: {
        let mut h = std::collections::HashMap::new();
        h.insert("event_type".to_string(), "user_login".to_string());
        h.insert("priority".to_string(), "normal".to_string());
        h
    },
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

ctx.mq().publish_to_topic("notifications", topic_message).await?;

// Batch publish to topic
let messages = vec![
    DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "notifications".to_string(),
        routing_key: "user.activity.signup".to_string(),
        body: json!({"user_id": 12345, "action": "signup"}),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::High,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    },
    DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "notifications".to_string(),
        routing_key: "user.activity.purchase".to_string(),
        body: json!({"user_id": 12345, "action": "purchase", "amount": 99.99}),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::Normal,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    },
];

ctx.mq().batch_publish_to_topic("notifications", messages).await?;
```

### Topic Subscription

```rust
use dmsc::prelude::*;
use serde_json::json;

// Subscribe to topic (wildcard pattern)
ctx.mq().subscribe_to_topic("notifications", "user.activity.*", |message, ctx| async move {
    ctx.log().info(format!("Received user activity: {:?}", message));
    
    let event_type = message.routing_key.split('.').last().unwrap_or("unknown");
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    
    match event_type {
        "login" => {
            // Handle user login
            handle_user_login(user_id, &message, &ctx).await?;
        }
        "signup" => {
            // Handle user signup
            handle_user_signup(user_id, &message, &ctx).await?;
        }
        "purchase" => {
            // Handle user purchase
            handle_user_purchase(user_id, &message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown event type: {}", event_type));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

// Multi-topic subscription
ctx.mq().subscribe_to_topics(vec![
    ("notifications", "user.activity.*"),
    ("system.events", "server.*"),
    ("analytics", "event.*"),
], |message, ctx| async move {
    ctx.log().info(format!("Received multi-topic message: {:?}", message));
    
    // Process message based on topic and routing key
    match (message.queue.as_str(), message.routing_key.as_str()) {
        ("notifications", key) if key.starts_with("user.activity.") => {
            process_user_activity(&message, &ctx).await?;
        }
        ("system.events", key) if key.starts_with("server.") => {
            process_system_event(&message, &ctx).await?;
        }
        ("analytics", key) if key.starts_with("event.") => {
            process_analytics_event(&message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unhandled message: {} - {}", message.queue, message.routing_key));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn process_user_activity(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let activity_type = message.routing_key.split('.').last().unwrap_or("unknown");
    
    // Record user activity log
    ctx.database()
        .execute(
            "INSERT INTO user_activity_logs (user_id, activity_type, data, created_at) VALUES ($1, $2, $3, $4)",
            vec![
                user_id.into(),
                activity_type.into(),
                serde_json::to_string(&message.body)?.into(),
                chrono::Utc::now().to_rfc3339().into(),
            ]
        )
        .await?;
    
    Ok(())
}

async fn process_system_event(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let event_type = message.routing_key.split('.').last().unwrap_or("unknown");
    
    match event_type {
        "startup" => {
            ctx.log().info("Server startup event received");
            // Execute startup-related processing
        }
        "shutdown" => {
            ctx.log().info("Server shutdown event received");
            // Execute shutdown-related processing
        }
        "error" => {
            ctx.log().error(format!("Server error: {:?}", message.body));
            // Send alert notification
        }
        _ => {
            ctx.log().warn(format!("Unknown system event: {}", event_type));
        }
    }
    
    Ok(())
}

async fn process_analytics_event(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    // Process analytics event, send to analytics system
    let analytics_data = json!({
        "event": message.body,
        "timestamp": message.timestamp,
        "processed_at": chrono::Utc::now().to_rfc3339(),
    });
    
    // Send to external analytics service
    ctx.http().post("https://analytics.example.com/events")
        .json(&analytics_data)
        .send()
        .await?;
    
    Ok(())
}
```

## Routing and Binding

### Complex Routing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create routing rules
let routing_rules = vec![
    DMSCRoutingRule {
        pattern: "order.*".to_string(),
        queue: "order_processing".to_string(),
        priority: 1,
    },
    DMSCRoutingRule {
        pattern: "order.created".to_string(),
        queue: "order_notifications".to_string(),
        priority: 2,
    },
    DMSCRoutingRule {
        pattern: "payment.*".to_string(),
        queue: "payment_processing".to_string(),
        priority: 1,
    },
    DMSCRoutingRule {
        pattern: "user.*".to_string(),
        queue: "user_notifications".to_string(),
        priority: 3,
    },
];

ctx.mq().setup_routing_rules(routing_rules).await?;

// Publish message with routing key
let order_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "orders".to_string(),
    routing_key: "order.created".to_string(),
    body: json!({
        "order_id": "ORD-12345",
        "user_id": 67890,
        "total_amount": 299.99,
        "items": [
            {"product_id": "PROD-001", "quantity": 2, "price": 99.99},
            {"product_id": "PROD-002", "quantity": 1, "price": 99.99},
        ],
        "created_at": chrono::Utc::now().to_rfc3339(),
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::High,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: Some("order-12345".to_string()),
    reply_to: Some("order.responses".to_string()),
};

// Messages will be routed to different queues based on routing rules
ctx.mq().publish_with_routing(order_message).await?;

// Subscribe to specific routing pattern messages
ctx.mq().subscribe_with_routing("order_processing", "order.*", |message, ctx| async move {
    ctx.log().info(format!("Processing order: {:?}", message));
    
    let order_id = message.body["order_id"].as_str().unwrap_or_default();
    let event_type = message.routing_key.split('.').last().unwrap_or("unknown");
    
    match event_type {
        "created" => {
            handle_order_created(order_id, &message, &ctx).await?;
        }
        "paid" => {
            handle_order_paid(order_id, &message, &ctx).await?;
        }
        "shipped" => {
            handle_order_shipped(order_id, &message, &ctx).await?;
        }
        "delivered" => {
            handle_order_delivered(order_id, &message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown order event type: {}", event_type));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn handle_order_created(order_id: &str, message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().info(format!("Handling order created: {}", order_id));
    
    // Validate order data
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let total_amount = message.body["total_amount"].as_f64().unwrap_or(0.0);
    
    if user_id == 0 || total_amount <= 0.0 {
        return Err(DMSCError::validation("Invalid order data".to_string()));
    }
    
    // Check if user exists
    let user_exists = ctx.database()
        .query_one("SELECT id FROM users WHERE id = $1", vec![user_id.into()])
        .await?
        .is_some();
    
    if !user_exists {
        return Err(DMSCError::not_found("User not found".to_string()));
    }
    
    // Create order record
    ctx.database()
        .execute(
            "INSERT INTO orders (id, user_id, total_amount, status, created_at) VALUES ($1, $2, $3, $4, $5)",
            vec![
                order_id.into(),
                user_id.into(),
                total_amount.into(),
                "created".into(),
                chrono::Utc::now().to_rfc3339().into(),
            ]
        )
        .await?;
    
    // Send inventory check message
    let inventory_message = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "inventory".to_string(),
        routing_key: "inventory.check".to_string(),
        body: json!({
            "order_id": order_id,
            "items": message.body["items"],
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::High,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: Some(order_id.to_string()),
        reply_to: Some("inventory.responses".to_string()),
    };
    
    ctx.mq().publish(inventory_message).await?;
    
    Ok(())
}
```

## Dead Letter Queue

### Configure Dead Letter Queue

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create dead letter queue configuration
let dlq_config = DMSCDeadLetterQueueConfig {
    enabled: true,
    max_retry_count: 3,
    retry_delay: Duration::from_secs(60),
    dead_letter_exchange: "dlx.orders".to_string(),
    dead_letter_queue: "dlq.orders.failed".to_string(),
    retry_exchange: "retry.orders".to_string(),
    retry_queues: vec![
        "retry.orders.1m".to_string(),
        "retry.orders.5m".to_string(),
        "retry.orders.15m".to_string(),
    ],
    retry_delays: vec![
        Duration::from_secs(60),
        Duration::from_secs(300),
        Duration::from_secs(900),
    ],
};

ctx.mq().setup_dead_letter_queue("order_processing", dlq_config).await?;

// Process dead letter queue message
ctx.mq().subscribe("dlq.orders.failed", |message, ctx| async move {
    ctx.log().error(format!("Processing dead letter message: {:?}", message));
    
    // Get original message information
    let original_queue = message.headers.get("x-original-queue").unwrap_or(&"unknown".to_string());
    let retry_count = message.headers.get("x-retry-count")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let failure_reason = message.headers.get("x-failure-reason").unwrap_or(&"unknown".to_string());
    
    ctx.log().error(format!(
        "Message failed after {} retries in queue {}: {}",
        retry_count, original_queue, failure_reason
    ));
    
    // Record failed message to database
    ctx.database()
        .execute(
            "INSERT INTO failed_messages (message_id, queue, routing_key, body, failure_reason, retry_count, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            vec![
                message.id.clone().into(),
                original_queue.clone().into(),
                message.routing_key.clone().into(),
                serde_json::to_string(&message.body)?.into(),
                failure_reason.clone().into(),
                retry_count.into(),
                chrono::Utc::now().to_rfc3339().into(),
            ]
        )
        .await?;
    
    // Send alert notification
    let alert_message = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "alerts".to_string(),
        routing_key: "alert.message_failed".to_string(),
        body: json!({
            "alert_type": "message_processing_failed",
            "message_id": message.id,
            "queue": original_queue,
            "retry_count": retry_count,
            "failure_reason": failure_reason,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::High,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    };
    
    ctx.mq().publish(alert_message).await?;
    
    // Acknowledge dead letter message
    ctx.mq().ack(&message).await?;
    
    Ok(())
}).await?;

// Retry mechanism example
async fn process_message_with_retry(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let retry_count = message.headers.get("x-retry-count")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    match do_message_processing(message, ctx).await {
        Ok(result) => {
            ctx.log().info(format!("Message processed successfully: {:?}", result));
            ctx.mq().ack(message).await?;
            Ok(())
        }
        Err(e) => {
            ctx.log().error(format!("Message processing failed (attempt {}): {}", retry_count + 1, e));
            
            if retry_count < 3 {
                // Requeue for retry
                let mut retry_message = message.clone();
                retry_message.headers.insert("x-retry-count".to_string(), (retry_count + 1).to_string());
                retry_message.headers.insert("x-failure-reason".to_string(), e.to_string());
                
                // Set delay based on retry count
                let delay = match retry_count {
                    0 => Duration::from_secs(60),   // 1 minute
                    1 => Duration::from_secs(300),  // 5 minutes
                    2 => Duration::from_secs(900),  // 15 minutes
                    _ => Duration::from_secs(3600), // 1 hour
                };
                
                retry_message.expiration = Some(chrono::Utc::now() + delay);
                
                ctx.mq().publish(retry_message).await?;
                ctx.mq().ack(message).await?;
            } else {
                // Reached maximum retry count, send to dead letter queue
                ctx.mq().reject(message, false).await?;
            }
            
            Err(e)
        }
    }
}

async fn do_message_processing(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    // Actual message processing logic
    let order_id = message.body["order_id"].as_str()
        .ok_or_else(|| DMSCError::validation("Order ID is required".to_string()))?;
    
    // Simulate processing failure scenario
    if order_id.starts_with("FAIL") {
        return Err(DMSCError::business("Simulated processing failure".to_string()));
    }
    
    // Normal processing logic
    Ok(json!({
        "status": "processed",
        "order_id": order_id,
        "processed_at": chrono::Utc::now().to_rfc3339(),
    }))
}
```

## Delayed Messages

### Delayed Message Processing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Send delayed message
let delayed_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "delayed.tasks".to_string(),
    routing_key: "task.scheduled".to_string(),
    body: json!({
        "task_type": "send_reminder_email",
        "user_id": 12345,
        "email": "user@example.com",
        "reminder_type": "subscription_renewal",
        "scheduled_for": (chrono::Utc::now() + Duration::from_days(7)).to_rfc3339(),
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: Some(chrono::Utc::now() + Duration::from_days(7)),
    correlation_id: None,
    reply_to: None,
};

// Set expiration in 7 days (auto-trigger)
ctx.mq().publish_with_delay(delayed_message, Duration::from_days(7)).await?;

// Process delayed messages
ctx.mq().subscribe("delayed.tasks", |message, ctx| async move {
    ctx.log().info(format!("Processing delayed task: {:?}", message));
    
    let task_type = message.body["task_type"].as_str().unwrap_or_default();
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let email = message.body["email"].as_str().unwrap_or_default();
    
    match task_type {
        "send_reminder_email" => {
            handle_reminder_email(user_id, email, &message, &ctx).await?;
        }
        "cleanup_temp_data" => {
            handle_cleanup_task(&message, &ctx).await?;
        }
        "generate_report" => {
            handle_report_generation(&message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown delayed task type: {}", task_type));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn handle_reminder_email(user_id: i64, email: &str, message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().info(format!("Sending reminder email to {} for user {}", email, user_id));
    
    // Check if user still needs reminder
    let user = ctx.database()
        .query_one("SELECT id, email, subscription_status FROM users WHERE id = $1", vec![user_id.into()])
        .await?;
    
    if let Some(user_data) = user {
        let subscription_status = user_data.get::<String>("subscription_status").unwrap_or_default();
        
        if subscription_status == "active" {
            // User subscription is still active, no reminder needed
            ctx.log().info(format!("User {} subscription is still active, skipping reminder", user_id));
            return Ok(());
        }
        
        // Send reminder email
        let email_message = DMSCMessage {
            id: uuid::Uuid::new_v4().to_string(),
            queue: "email.outbound".to_string(),
            routing_key: "email.reminder.send".to_string(),
            body: json!({
                "to": email,
                "subject": "Don't forget to renew your subscription!",
                "template": "subscription_renewal_reminder",
                "data": {
                    "user_name": user_data.get::<String>("name").unwrap_or_default(),
                    "renewal_url": format!("https://example.com/renew?user_id={}", user_id),
                }
            }),
            headers: std::collections::HashMap::new(),
            priority: DMSCMessagePriority::Normal,
            delivery_mode: DMSCMessageDeliveryMode::Persistent,
            timestamp: chrono::Utc::now(),
            expiration: None,
            correlation_id: Some(message.id.clone()),
            reply_to: None,
        };
        
        ctx.mq().publish(email_message).await?;
        
        // Record reminder sent
        ctx.database()
            .execute(
                "INSERT INTO reminder_logs (user_id, reminder_type, sent_at, message_id) VALUES ($1, $2, $3, $4)",
                vec![
                    user_id.into(),
                    "subscription_renewal".into(),
                    chrono::Utc::now().to_rfc3339().into(),
                    message.id.clone().into(),
                ]
            )
            .await?;
    }
    
    Ok(())
}

// Scheduled task scheduler
async fn schedule_periodic_tasks(ctx: &DMSCContext) -> DMSCResult<()> {
    // Execute data cleanup daily at 2 AM
    let cleanup_task = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "scheduled.tasks".to_string(),
        routing_key: "task.daily_cleanup".to_string(),
        body: json!({
            "task_type": "cleanup_temp_data",
            "retention_days": 30,
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::Low,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: Some(chrono::Utc::now() + Duration::from_hours(24)),
        correlation_id: None,
        reply_to: None,
    };
    
    // Calculate time difference until tomorrow 2 AM
    let now = chrono::Utc::now();
    let tomorrow_2am = now.date().and_hms(2, 0, 0) + Duration::from_days(1);
    let delay = tomorrow_2am - now;
    
    ctx.mq().publish_with_delay(cleanup_task, delay).await?;
    
    // Generate reports every Monday at 9 AM
    let report_task = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "scheduled.tasks".to_string(),
        routing_key: "task.weekly_report".to_string(),
        body: json!({
            "task_type": "generate_weekly_report",
            "report_type": "sales_summary",
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::Normal,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: Some(chrono::Utc::now() + Duration::from_days(7)),
        correlation_id: None,
        reply_to: None,
    };
    
    // Calculate time difference until next Monday morning
    let days_until_monday = (8 - now.weekday().num_from_monday() as i64) % 7;
    let next_monday_9am = now.date().and_hms(9, 0, 0) + Duration::from_days(days_until_monday);
    let delay = next_monday_9am - now;
    
    ctx.mq().publish_with_delay(report_task, delay).await?;
    
    Ok(())
}
```

## Message Priority

### Priority Message Processing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Send messages with different priorities
let urgent_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(),
    routing_key: "alert.critical".to_string(),
    body: json!({
        "alert_type": "system_failure",
        "severity": "critical",
        "message": "Database connection lost",
        "affected_services": ["user_service", "order_service"],
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::Critical,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

let high_priority_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(),
    routing_key: "alert.high".to_string(),
    body: json!({
        "alert_type": "performance_degradation",
        "severity": "high",
        "message": "API response time increased",
        "threshold": "95th_percentile > 2s",
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::High,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

let normal_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(),
    routing_key: "alert.normal".to_string(),
    body: json!({
        "alert_type": "daily_summary",
        "severity": "normal",
        "message": "Daily system summary",
        "stats": {
            "total_users": 15420,
            "new_signups": 234,
            "total_orders": 892,
        },
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

let low_priority_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(),
    routing_key: "alert.low".to_string(),
    body: json!({
        "alert_type": "maintenance_reminder",
        "severity": "low",
        "message": "Scheduled maintenance in 7 days",
        "maintenance_window": "2024-02-01 02:00:00",
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::Low,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

// Publish messages with different priorities
ctx.mq().publish(urgent_message).await?;
ctx.mq().publish(high_priority_message).await?;
ctx.mq().publish(normal_message).await?;
ctx.mq().publish(low_priority_message).await?;

// Priority queue subscription
ctx.mq().subscribe_with_priority("notifications", |message, ctx| async move {
    let priority = message.priority;
    let routing_key = message.routing_key.clone();
    
    ctx.log().info(format!("Processing {} priority message: {}", 
        match priority {
            DMSCMessagePriority::Critical => "CRITICAL",
            DMSCMessagePriority::High => "HIGH",
            DMSCMessagePriority::Normal => "NORMAL",
            DMSCMessagePriority::Low => "LOW",
        }, routing_key));
    
    // Process message based on priority
    match priority {
        DMSCMessagePriority::Critical => {
            handle_critical_alert(&message, &ctx).await?;
        }
        DMSCMessagePriority::High => {
            handle_high_priority_alert(&message, &ctx).await?;
        }
        DMSCMessagePriority::Normal => {
            handle_normal_alert(&message, &ctx).await?;
        }
        DMSCMessagePriority::Low => {
            handle_low_priority_alert(&message, &ctx).await?;
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn handle_critical_alert(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let alert_type = message.body["alert_type"].as_str().unwrap_or_default();
    
    ctx.log().error(format!("CRITICAL ALERT: {} - {:?}", alert_type, message.body));
    
    // Send urgent notification immediately
    let alert_notification = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "urgent_notifications".to_string(),
        routing_key: "urgent.critical_alert".to_string(),
        body: json!({
            "alert_type": alert_type,
            "message": message.body,
            "notify_channels": ["sms", "email", "slack"],
            "escalation_level": 1,
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::Critical,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: Some(message.id.clone()),
        reply_to: None,
    };
    
    ctx.mq().publish(alert_notification).await?;
    
    // Record to monitoring system
    ctx.observability().record_event("critical_alert", json!({
        "alert_type": alert_type,
        "message_id": message.id,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    })).await?;
    
    Ok(())
}

async fn handle_high_priority_alert(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let alert_type = message.body["alert_type"].as_str().unwrap_or_default();
    
    ctx.log().warn(format!("HIGH PRIORITY ALERT: {} - {:?}", alert_type, message.body));
    
    // Send notification to operations team
    let notification = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "team_notifications".to_string(),
        routing_key: "team.high_priority".to_string(),
        body: json!({
            "alert_type": alert_type,
            "message": message.body,
            "notify_channels": ["email", "slack"],
        }),
        headers: std::collections::HashMap::new(),
        priority: DMSCMessagePriority::High,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: Some(message.id.clone()),
        reply_to: None,
    };
    
    ctx.mq().publish(notification).await?;
    
    Ok(())
}

async fn handle_normal_alert(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().info(format!("NORMAL ALERT: {:?}", message.body));
    
    // Record to logging system
    ctx.log().info(format!("Alert processed: {} - {:?}", 
        message.body["alert_type"].as_str().unwrap_or_default(), 
        message.body));
    
    Ok(())
}

async fn handle_low_priority_alert(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().debug(format!("LOW PRIORITY ALERT: {:?}", message.body));
    
    // Can be processed with delay or in batch
    Ok(())
}
```

## Message Filtering

### Message Filter

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create message filter
let filters = vec![
    DMSCMessageFilter {
        field: "headers.event_type".to_string(),
        operator: DMSCFilterOperator::Equals,
        value: "user_login".to_string(),
        action: DMSCFilterAction::Accept,
    },
    DMSCMessageFilter {
        field: "headers.priority".to_string(),
        operator: DMSCFilterOperator::GreaterThan,
        value: "5".to_string(),
        action: DMSCFilterAction::Accept,
    },
    DMSCMessageFilter {
        field: "body.user_id".to_string(),
        operator: DMSCFilterOperator::LessThan,
        value: "1000".to_string(),
        action: DMSCFilterAction::Reject,
    },
    DMSCMessageFilter {
        field: "routing_key".to_string(),
        operator: DMSCFilterOperator::Contains,
        value: "error".to_string(),
        action: DMSCFilterAction::Accept,
    },
];

ctx.mq().setup_message_filters("user_events", filters).await?;

// Message subscription with filters
ctx.mq().subscribe_with_filters("user_events", |message, ctx| async move {
    ctx.log().info(format!("Filtered message received: {:?}", message));
    
    // Only messages that meet filter criteria will be received
    let event_type = message.headers.get("event_type").unwrap_or(&"unknown".to_string());
    let priority = message.headers.get("priority")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    ctx.log().info(format!("Processing {} event with priority {}", event_type, priority));
    
    // Process based on event type
    match event_type.as_str() {
        "user_login" => {
            handle_user_login_event(&message, &ctx).await?;
        }
        "user_logout" => {
            handle_user_logout_event(&message, &ctx).await?;
        }
        "user_signup" => {
            handle_user_signup_event(&message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown event type: {}", event_type));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

// Dynamic filters
async fn create_dynamic_filter(ctx: &DMSCContext, user_id_threshold: i32) -> DMSCResult<()> {
    let dynamic_filters = vec![
        DMSCMessageFilter {
            field: "body.user_id".to_string(),
            operator: DMSCFilterOperator::GreaterThanOrEqual,
            value: user_id_threshold.to_string(),
            action: DMSCFilterAction::Accept,
        },
        DMSCMessageFilter {
            field: "headers.source".to_string(),
            operator: DMSCFilterOperator::NotEquals,
            value: "test".to_string(),
            action: DMSCFilterAction::Reject,
        },
    ];
    
    ctx.mq().update_message_filters("user_events", dynamic_filters).await?;
    
    Ok(())
}

// Custom filter function
async fn custom_message_filter(message: &DMSCMessage, ctx: &DMSCContext) -> bool {
    // Custom filtering logic
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let event_time = message.body["timestamp"].as_str().unwrap_or_default();
    
    // Filter out test users
    if user_id < 1000 {
        return false;
    }
    
    // Filter out expired events (over 1 hour)
    if let Ok(event_timestamp) = chrono::DateTime::parse_from_rfc3339(event_time) {
        let now = chrono::Utc::now();
        if now - event_timestamp > Duration::from_hours(1) {
            return false;
        }
    }
    
    // Filter out duplicate events
    let event_key = format!("{}:{}", user_id, message.routing_key);
    if ctx.cache().exists(&event_key).await? {
        return false;
    }
    
    // Record event to prevent duplicates
    ctx.cache().set(&event_key, "1", Duration::from_minutes(10)).await?;
    
    true
}
```

## Message Persistence

### Persistence Configuration

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure message persistence
let persistence_config = DMSCMessagePersistenceConfig {
    enabled: true,
    storage_type: DMSCMessageStorageType::Database,
    retention_days: 30,
    cleanup_interval: Duration::from_hours(24),
    compression_enabled: true,
    encryption_enabled: true,
    archive_after_days: 90,
    archive_storage: "s3://message-archive/".to_string(),
};

ctx.mq().setup_message_persistence(persistence_config).await?;

// Send persistent message
let persistent_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "important.events".to_string(),
    routing_key: "business.critical".to_string(),
    body: json!({
        "event_type": "financial_transaction",
        "transaction_id": "TXN-123456789",
        "amount": 15000.00,
        "currency": "USD",
        "parties": {
            "sender": "ACC-001",
            "receiver": "ACC-002",
        },
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }),
    headers: {
        let mut h = std::collections::HashMap::new();
        h.insert("persistence".to_string(), "required".to_string());
        h.insert("retention".to_string(), "permanent".to_string());
        h
    },
    priority: DMSCMessagePriority::High,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: Some("txn-123456789".to_string()),
    reply_to: None,
};

ctx.mq().publish_persistent(persistent_message).await?;

// Query historical messages
let historical_messages = ctx.mq().query_messages(
    "important.events",
    chrono::Utc::now() - Duration::from_days(7),
    chrono::Utc::now(),
    Some("business.critical".to_string()),
    100,
).await?;

for message in historical_messages {
    ctx.log().info(format!("Historical message: {:?}", message));
}

// Message replay
let replay_result = ctx.mq().replay_messages(
    "important.events",
    chrono::Utc::now() - Duration::from_hours(24),
    chrono::Utc::now(),
    Some("business.critical".to_string()),
).await?;

ctx.log().info(format!("Replayed {} messages", replay_result.replayed_count));
```

## Batch Operations

### Batch Message Processing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Batch publish messages
let batch_messages = (0..1000).map(|i| {
    DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "batch.processing".to_string(),
        routing_key: format!("batch.item.{}", i % 10),
        body: json!({
            "item_id": i,
            "data": format!("Item {} data", i),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        headers: std::collections::HashMap::new(),
        priority: if i % 100 == 0 { DMSCMessagePriority::High } else { DMSCMessagePriority::Normal },
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    }
}).collect::<Vec<_>>();

ctx.mq().batch_publish(batch_messages).await?;

// Batch acknowledge messages
let messages_to_ack = vec![
    "msg-001".to_string(),
    "msg-002".to_string(),
    "msg-003".to_string(),
];

ctx.mq().batch_ack(messages_to_ack).await?;

// Batch process messages (improve throughput)
ctx.mq().subscribe_batch("batch.processing", 50, |messages, ctx| async move {
    ctx.log().info(format!("Processing batch of {} messages", messages.len()));
    
    let mut processed_count = 0;
    let mut failed_count = 0;
    
    for message in messages {
        match process_batch_item(&message, &ctx).await {
            Ok(_) => {
                processed_count += 1;
                ctx.mq().ack(&message).await?;
            }
            Err(e) => {
                failed_count += 1;
                ctx.log().error(format!("Failed to process message {}: {}", message.id, e));
                ctx.mq().nack(&message, true).await?;
            }
        }
    }
    
    ctx.log().info(format!("Batch processing completed: {} successful, {} failed", processed_count, failed_count));
    
    Ok(())
}).await?;

async fn process_batch_item(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let item_id = message.body["item_id"].as_i64().unwrap_or(0);
    let data = message.body["data"].as_str().unwrap_or_default();
    
    // Simulate processing logic
    if item_id % 50 == 0 {
        return Err(DMSCError::business(format!("Simulated failure for item {}", item_id)));
    }
    
    // Process data
    ctx.database()
        .execute(
            "INSERT INTO processed_items (item_id, data, processed_at) VALUES ($1, $2, $3)",
            vec![
                item_id.into(),
                data.into(),
                chrono::Utc::now().to_rfc3339().into(),
            ]
        )
        .await?;
    
    Ok(())
}
```

## Error Handling

### Message Queue Error Handling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Error handling example
match ctx.mq().publish(message).await {
    Ok(_) => {
        ctx.log().info("Message published successfully");
    }
    Err(DMSCError::MessageQueueConnectionError(e)) => {
        ctx.log().error(format!("Message queue connection failed: {}", e));
        // Try to reconnect or fallback
        handle_mq_connection_error(&e, ctx).await?;
    }
    Err(DMSCError::MessageQueuePublishError(e)) => {
        ctx.log().error(format!("Message publish failed: {}", e));
        // Try to republish or use backup queue
        retry_message_publish(message, ctx).await?;
    }
    Err(DMSCError::MessageQueueTimeoutError(e)) => {
        ctx.log().warn(format!("Message queue operation timed out: {}", e));
        // Increase timeout or batch process
        handle_mq_timeout(&e, ctx).await?;
    }
    Err(DMSCError::MessageQueueConsumerError(e)) => {
        ctx.log().error(format!("Message consumer error: {}", e));
        // Restart consumer or switch to backup consumer
        restart_message_consumer(&e, ctx).await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected message queue error: {}", e));
        return Err(e);
    }
}

async fn handle_mq_connection_error(error: &str, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().warn("Message queue is unavailable, switching to local queue");
    
    // Enable local queue fallback
    ctx.cache().set("mq_fallback_enabled", "true", Duration::from_hours(1)).await?;
    
    // Retry connection periodically
    let mut retry_count = 0;
    while retry_count < 10 {
        match ctx.mq().ping().await {
            Ok(_) => {
                ctx.log().info("Message queue connection restored");
                ctx.cache().delete("mq_fallback_enabled").await?;
                break;
            }
            Err(e) => {
                ctx.log().warn(format!("Message queue still unavailable (retry {}): {}", retry_count + 1, e));
                sleep(Duration::from_secs(30)).await;
                retry_count += 1;
            }
        }
    }
    
    if retry_count >= 10 {
        return Err(DMSCError::service_unavailable("Message queue is still unavailable after 10 retries".to_string()));
    }
    
    Ok(())
}

async fn retry_message_publish(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    let mut retry_count = 0;
    let max_retries = 3;
    
    while retry_count < max_retries {
        match ctx.mq().publish(message.clone()).await {
            Ok(_) => {
                ctx.log().info(format!("Message published successfully after {} retries", retry_count));
                return Ok(());
            }
            Err(e) => {
                retry_count += 1;
                if retry_count < max_retries {
                    let delay = Duration::from_millis(1000 * 2_u64.pow(retry_count as u32));
                    ctx.log().warn(format!("Message publish failed (retry {}), retrying in {:?}: {}", retry_count, delay, e));
                    sleep(delay).await;
                } else {
                    ctx.log().error(format!("Message publish failed after {} retries: {}", max_retries, e));
                    
                    // Send to failed message queue
                    let failed_message = DMSCMessage {
                        id: uuid::Uuid::new_v4().to_string(),
                        queue: "failed.messages".to_string(),
                        routing_key: "message.publish_failed".to_string(),
                        body: json!({
                            "original_message": message,
                            "failure_reason": e.to_string(),
                            "retry_count": retry_count,
                        }),
                        headers: std::collections::HashMap::new(),
                        priority: DMSCMessagePriority::High,
                        delivery_mode: DMSCMessageDeliveryMode::Persistent,
                        timestamp: chrono::Utc::now(),
                        expiration: None,
                        correlation_id: Some(message.id.clone()),
                        reply_to: None,
                    };
                    
                    ctx.mq().publish(failed_message).await?;
                    return Err(e);
                }
            }
        }
    }
    
    Ok(())
}
```

<div align="center">

## Running Steps

</div>

### 1. Install Dependencies

Ensure Rust and Cargo are installed (version 1.65+):

```bash
cargo --version
```

### 2. Start Message Queue Service

Start the corresponding service based on the configured message queue type:

**RabbitMQ:**
```bash
# Start RabbitMQ using Docker
docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management

# Access management interface http://localhost:15672 (guest/guest)
```

**Apache Kafka:**
```bash
# Start Kafka using Docker Compose
docker-compose up -d zookeeper kafka

# Or start individually using Docker
docker run -d --name kafka -p 9092:9092 confluentinc/cp-kafka:latest
```

**Redis Streams:**
```bash
# Start Redis using Docker
docker run -d --name redis -p 6379:6379 redis:7-alpine
```

### 3. Configure Project

Create configuration file `config.yaml`, modify message queue connection information based on actual environment:

```yaml
message_queue:
  broker_type: "rabbitmq"  # Options: rabbitmq, kafka, redis
  rabbitmq:
    host: "localhost"
    port: 5672
    username: "guest"
    password: "guest"
```

### 4. Run Example

```bash
# Enter project directory
cd dms-mq-example

# Run application
cargo run
```

### 5. Verify Functionality

After application starts, it will automatically perform the following operations:
- Connect to message queue service
- Publish sample messages to user registration queue
- Subscribe and process messages
- Display processing results and log information

<div align="center">

## Expected Results

</div>

After successful execution, you will see output similar to the following:

```
[2025-12-12 10:30:45] INFO  service: DMSC Message Queue Example started
[2025-12-12 10:30:45] INFO  mq: Initializing message queue
[2025-12-12 10:30:45] INFO  mq: Message queue initialized
[2025-12-12 10:30:45] INFO  mq: Message queue connection successful
[2025-12-12 10:30:45] INFO  mq: Publishing sample messages
[2025-12-12 10:30:45] INFO  mq: User registration message published
[2025-12-12 10:30:45] INFO  mq: Subscribing to message queues
[2025-12-12 10:30:45] INFO  mq: Message queue subscriptions configured
[2025-12-12 10:30:45] INFO  mq: Received message: DMSCMessage { id: "...", queue: "user.registrations", ... }
[2025-12-12 10:30:45] INFO  mq: Processing user registration for: newuser@example.com
[2025-12-12 10:30:45] INFO  mq: User registration processed: {"status": "success", "user_id": 12345, "email": "newuser@example.com", "processed_at": "2025-12-12T10:30:45Z"}
```

### Message Queue Management Interface

- **RabbitMQ**: Access http://localhost:15672 to view queue status
- **Kafka**: Use Kafka tools to view topics and messages
- **Redis**: Use `redis-cli` to view Streams data

<div align="center">

## Extended Features

</div>

### Load Balancing Support

```rust
use dmsc::prelude::*;

// Configure multiple message queue nodes for load balancing
let load_balanced_config = DMSCMessageQueueLoadBalanceConfig {
    enabled: true,
    strategy: DMSCLoadBalanceStrategy::RoundRobin,
    nodes: vec![
        DMSCMessageQueueNode {
            host: "mq-node-1.example.com".to_string(),
            port: 5672,
            weight: 1,
            health_check_interval: Duration::from_secs(30),
        },
        DMSCMessageQueueNode {
            host: "mq-node-2.example.com".to_string(),
            port: 5672,
            weight: 2,
            health_check_interval: Duration::from_secs(30),
        },
        DMSCMessageQueueNode {
            host: "mq-node-3.example.com".to_string(),
            port: 5672,
            weight: 1,
            health_check_interval: Duration::from_secs(30),
        },
    ],
    failover_enabled: true,
    failover_timeout: Duration::from_secs(10),
};

ctx.mq().setup_load_balancing(load_balanced_config).await?;

// Monitor node health status
ctx.mq().monitor_node_health(|node, status| async move {
    match status {
        DMSCNodeHealthStatus::Healthy => {
            ctx.log().info(format!("Message queue node {} is healthy", node.host));
        }
        DMSCNodeHealthStatus::Unhealthy => {
            ctx.log().warn(format!("Message queue node {} is unhealthy", node.host));
            // Trigger failover
            ctx.mq().trigger_failover(node).await?;
        }
        DMSCNodeHealthStatus::Offline => {
            ctx.log().error(format!("Message queue node {} is offline", node.host));
            // Remove from load balancing pool
            ctx.mq().remove_node_from_pool(node).await?;
        }
    }
    Ok(())
}).await?;
```

### Message Queue Monitoring

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure message queue monitoring
let monitoring_config = DMSCMessageQueueMonitoringConfig {
    enabled: true,
    metrics_interval: Duration::from_secs(60),
    alert_thresholds: DMSCMessageQueueAlertThresholds {
        queue_size_warning: 1000,
        queue_size_critical: 5000,
        processing_time_warning: Duration::from_secs(30),
        processing_time_critical: Duration::from_secs(60),
        error_rate_warning: 0.05,  // 5%
        error_rate_critical: 0.1,   // 10%
    },
    dashboards: vec![
        "queue_depth".to_string(),
        "processing_latency".to_string(),
        "error_rate".to_string(),
        "throughput".to_string(),
    ],
};

ctx.mq().setup_monitoring(monitoring_config).await?;

// Collect performance metrics
ctx.mq().collect_metrics(|metrics| async move {
    ctx.log().info(format!("Queue metrics: {:?}", metrics));
    
    // Send to monitoring system
    let metrics_data = json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "queue_metrics": {
            "total_messages": metrics.total_messages,
            "processed_messages": metrics.processed_messages,
            "failed_messages": metrics.failed_messages,
            "average_processing_time": metrics.average_processing_time,
            "queue_depth": metrics.queue_depth,
            "error_rate": metrics.error_rate,
        }
    });
    
    ctx.observability().record_metric("message_queue_performance", metrics_data).await?;
    Ok(())
}).await?;

// Setup alerts
ctx.mq().setup_alerts(|alert| async move {
    match alert.level {
        DMSCAlertLevel::Warning => {
            ctx.log().warn(format!("MQ Alert: {}", alert.message));
            // Send warning notification
            send_alert_notification("warning", &alert, ctx).await?;
        }
        DMSCAlertLevel::Critical => {
            ctx.log().error(format!("MQ Critical Alert: {}", alert.message));
            // Send urgent notification
            send_alert_notification("critical", &alert, ctx).await?;
            // Trigger auto remediation
            ctx.mq().trigger_auto_remediation(&alert).await?;
        }
    }
    Ok(())
}).await?;

async fn send_alert_notification(level: &str, alert: &DMSCAlert, ctx: &DMSCContext) -> DMSCResult<()> {
    let notification = DMSCMessage {
        id: uuid::Uuid::new_v4().to_string(),
        queue: "notifications".to_string(),
        routing_key: format!("alert.{}", level),
        body: json!({
            "alert_type": "message_queue",
            "level": level,
            "message": alert.message,
            "metric": alert.metric,
            "threshold": alert.threshold,
            "current_value": alert.current_value,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        headers: std::collections::HashMap::new(),
        priority: if level == "critical" { 
            DMSCMessagePriority::Critical 
        } else { 
            DMSCMessagePriority::High 
        },
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: chrono::Utc::now(),
        expiration: None,
        correlation_id: None,
        reply_to: None,
    };
    
    ctx.mq().publish(notification).await?;
    Ok(())
}
```

### Message Tracing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure distributed message tracing
let tracing_config = DMSCMessageTracingConfig {
    enabled: true,
    sampling_rate: 0.1,  // 10% sampling rate
    trace_header_name: "x-trace-id".to_string(),
    span_header_name: "x-span-id".to_string(),
    baggage_header_prefix: "x-baggage-".to_string(),
    max_trace_depth: 100,
    retention_days: 30,
};

ctx.mq().setup_distributed_tracing(tracing_config).await?;

// Send message with tracing information
let traced_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "user.events".to_string(),
    routing_key: "user.activity.login".to_string(),
    body: json!({
        "user_id": 12345,
        "activity": "login",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }),
    headers: {
        let mut h = std::collections::HashMap::new();
        h.insert("x-trace-id".to_string(), "trace-12345".to_string());
        h.insert("x-span-id".to_string(), "span-67890".to_string());
        h.insert("x-baggage-user-tier".to_string(), "premium".to_string());
        h.insert("x-baggage-request-id".to_string(), "req-abc123".to_string());
        h
    },
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

ctx.mq().publish_with_tracing(traced_message).await?;

// Process messages with tracing information
ctx.mq().subscribe_with_tracing("user.events", |message, trace_context, ctx| async move {
    ctx.log().info(format!("Processing traced message: trace_id={}, span_id={}", 
        trace_context.trace_id, trace_context.span_id));
    
    // Record trace information
    ctx.observability().record_trace_span(
        "message_processing",
        json!({
            "trace_id": trace_context.trace_id,
            "span_id": trace_context.span_id,
            "parent_span_id": trace_context.parent_span_id,
            "baggage": trace_context.baggage,
            "message_id": message.id,
            "queue": message.queue,
            "routing_key": message.routing_key,
            "start_time": chrono::Utc::now().to_rfc3339(),
        })
    ).await?;
    
    // Process message
    match process_user_activity(&message, &ctx).await {
        Ok(result) => {
            ctx.observability().record_trace_event(
                "message_processed_successfully",
                json!({
                    "trace_id": trace_context.trace_id,
                    "result": result,
                    "end_time": chrono::Utc::now().to_rfc3339(),
                })
            ).await?;
            
            ctx.mq().ack(&message).await?;
        }
        Err(e) => {
            ctx.observability().record_trace_event(
                "message_processing_failed",
                json!({
                    "trace_id": trace_context.trace_id,
                    "error": e.to_string(),
                    "end_time": chrono::Utc::now().to_rfc3339(),
                })
            ).await?;
            
            ctx.mq().nack(&message, true).await?;
        }
    }
    
    Ok(())
}).await?;
```

### Message Compression

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure message compression
let compression_config = DMSCMessageCompressionConfig {
    enabled: true,
    threshold_size: 1024,  // Enable compression for messages above 1KB
    algorithms: vec![
        DMSCCompressionAlgorithm::Gzip,
        DMSCCompressionAlgorithm::Lz4,
        DMSCCompressionAlgorithm::Zstd,
    ],
    compression_level: 6,  // Compression level 1-9
    auto_decompress: true,
};

ctx.mq().setup_compression(compression_config).await?;

// Send large message (auto-compression)
let large_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "data.updates".to_string(),
    routing_key: "batch.data.sync".to_string(),
    body: json!({
        "batch_id": "batch-12345",
        "records": (0..1000).map(|i| {
            json!({
                "id": i,
                "data": format!("Large data payload for item {}", i),
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "metadata": {
                    "source": "sensor_".to_string() + &format!("{}", i % 10),
                    "location": format!("location_{}", i % 100),
                    "status": if i % 2 == 0 { "active" } else { "inactive" },
                }
            })
        }).collect::<Vec<_>>(),
        "metadata": {
            "total_records": 1000,
            "compressed": true,
            "compression_ratio": 0.0,  // Will be automatically calculated by the system
        }
    }),
    headers: std::collections::HashMap::new(),
    priority: DMSCMessagePriority::Normal,
    delivery_mode: DMSCMessageDeliveryMode::Persistent,
    timestamp: chrono::Utc::now(),
    expiration: None,
    correlation_id: None,
    reply_to: None,
};

// Large messages will be automatically compressed
ctx.mq().publish_compressed(large_message).await?;

// Process compressed messages (auto-decompression)
ctx.mq().subscribe_compressed("data.updates", |message, compression_info, ctx| async move {
    ctx.log().info(format!("Received compressed message: original_size={}, compressed_size={}, ratio={}, algorithm={}", 
        compression_info.original_size,
        compression_info.compressed_size,
        compression_info.compression_ratio,
        compression_info.algorithm));
    
    // Process decompressed message
    let batch_id = message.body["batch_id"].as_str().unwrap_or_default();
    let records = message.body["records"].as_array().unwrap_or(&vec![]);
    
    ctx.log().info(format!("Processing batch {} with {} records", batch_id, records.len()));
    
    // Batch process records
    for record in records.chunks(100) {
        process_record_batch(record, &ctx).await?;
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn process_record_batch(records: &[serde_json::Value], ctx: &DMSCContext) -> DMSCResult<()> {
    // Batch process records
    for record in records {
        let id = record["id"].as_i64().unwrap_or(0);
        let data = record["data"].as_str().unwrap_or_default();
        let status = record["metadata"]["status"].as_str().unwrap_or_default();
        
        // Process each record
        ctx.database()
            .execute(
                "INSERT INTO sensor_data (id, data, status, processed_at) VALUES ($1, $2, $3, $4)",
                vec![
                    id.into(),
                    data.into(),
                    status.into(),
                    chrono::Utc::now().to_rfc3339().into(),
                ]
            )
            .await?;
    }
    
    Ok(())
}
```

<div align="center">

## Best Practices

</div>

1. **Message Idempotency**: Ensure message processing is idempotent to avoid issues from duplicate processing
2. **Message Acknowledgment**: Acknowledge processed messages promptly to avoid duplicate delivery
3. **Error Handling**: Handle message processing failures properly, use dead letter queues
4. **Message Size**: Control message size, avoid transmitting oversized messages
5. **Connection Management**: Manage message queue connections properly, use connection pooling
6. **Monitoring Metrics**: Monitor message queue performance metrics and health status
7. **Backup Strategy**: Implement message persistence and backup strategies
8. **Rate Limiting**: Implement rate limiting for message production and consumption
9. **Version Management**: Manage message format version compatibility
10. **Security Authentication**: Enable message queue security authentication and encryption
11. **Load Balancing**: Configure multi-node load balancing to improve availability
12. **Compression Optimization**: Enable compression for large messages to reduce network transmission
13. **Tracing Monitoring**: Implement distributed tracing for easier problem location
14. **Batch Processing**: Use batch operations to improve throughput
15. **Priority Management**: Use message priorities reasonably to ensure important messages are processed first

<div align="center">

## Summary

</div>

This example comprehensively demonstrates the core functionality and advanced features of the DMSC message queue module, covering the following key capabilities:

### 🚀 Core Features
- **Multi-message Queue Support**: Seamless integration of RabbitMQ, Apache Kafka, and Redis Streams
- **Publish-Subscribe Pattern**: Flexible topic publishing and wildcard subscription mechanisms
- **Message Routing**: Complex routing rules and queue binding functionality
- **Dead Letter Queue**: Comprehensive message retry and failure handling mechanisms
- **Delayed Messages**: Scheduled tasks and delayed message processing support
- **Message Priority**: Multi-level priority queue management
- **Message Filtering**: Intelligent message filtering based on content and headers
- **Message Persistence**: Reliable message storage and historical query functionality

### 🔧 Advanced Features
- **Load Balancing**: Multi-node load balancing and failover mechanisms
- **Performance Monitoring**: Real-time queue monitoring and performance metrics collection
- **Distributed Tracing**: Cross-service message chain tracking
- **Message Compression**: Automatic compression optimization for large messages
- **Batch Operations**: Efficient batch message processing
- **Error Handling**: Comprehensive exception handling and fallback strategies

### 💡 Best Practices
- Message idempotency design to ensure safe handling of duplicate messages
- Timely message acknowledgment mechanism to avoid duplicate message delivery
- Reasonable message size control to optimize network transmission performance
- Comprehensive monitoring and alerting to ensure stable system operation
- Multi-level error handling to improve system fault tolerance

Through this example, you can build a highly reliable and high-performance distributed message processing system that supports complex business scenarios and large-scale data processing requirements.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, learn how to use caching modules to improve application performance
- [database](./database.md): Database examples, learn database connection and query operations
- [http](./http.md): HTTP service examples, build web applications and RESTful APIs
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication

- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing and security best practices
- [storage](./storage.md): Storage examples, file upload/download and storage management
- [validation](./validation.md): Validation examples, data validation and cleanup operations