<div align="center">

# 消息队列使用示例

**Version: 1.0.0**

**Last modified date: 2025-12-12**

本示例展示如何使用DMSC的mq模块进行消息队列、发布订阅、路由、死信队列、延迟消息、持久化、优先级和过滤功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- RabbitMQ、Kafka、Redis Streams消息队列
- 发布订阅和消息路由
- 死信队列和延迟消息
- 消息持久化和优先级
- 消息过滤和确认机制
- 错误处理和重试策略

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解消息队列基本概念
- （可选）RabbitMQ、Kafka或Redis服务器

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-mq-example
cd dms-mq-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
chrono = "0.4"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

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

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dms::prelude::*;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_message_queue(DMSCMessageQueueConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Message Queue Example started")?;
        
        // 初始化消息队列
        initialize_message_queue(&ctx).await?;
        
        // 发布示例消息
        publish_sample_messages(&ctx).await?;
        
        // 订阅消息队列
        subscribe_to_queues(&ctx).await?;
        
        // 保持服务运行
        tokio::signal::ctrl_c().await?;
        ctx.logger().info("service", "Shutting down message queue service")?;
        
        Ok(())
    }).await
}

async fn initialize_message_queue(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("mq", "Initializing message queue")?;
    
    // RabbitMQ配置
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
    
    // 初始化消息队列
    ctx.mq().init(mq_config).await?;
    ctx.logger().info("mq", "Message queue initialized")?;
    
    // 测试连接
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
    
    // 创建用户注册消息
    let message = DMSCMessage {
        id: Uuid::new_v4().to_string(),
        queue: "user.registrations".to_string(),
        routing_key: "user.created".to_string(),
        body: json!({
            "user_id": 12345,
            "email": "newuser@example.com",
            "name": "John Doe",
            "registration_time": Utc::now().to_rfc3339(),
        }),
        headers: {
            let mut h = HashMap::new();
            h.insert("source".to_string(), "web_app".to_string());
            h.insert("version".to_string(), "1.0".to_string());
            h.insert("correlation_id".to_string(), Uuid::new_v4().to_string());
            h
        },
        priority: DMSCMessagePriority::Normal,
        delivery_mode: DMSCMessageDeliveryMode::Persistent,
        timestamp: Utc::now(),
        expiration: None,
        correlation_id: Some(Uuid::new_v4().to_string()),
        reply_to: Some("user.registration.responses".to_string()),
    };
    
    // 发布消息
    ctx.mq().publish(message).await?;
    ctx.logger().info("mq", "User registration message published")?;
    
    Ok(())
}

async fn subscribe_to_queues(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("mq", "Subscribing to message queues")?;
    
    // 订阅用户注册队列
    ctx.mq().subscribe("user.registrations", |message, ctx| async move {
        ctx.logger().info("mq", &format!("Received message: {:?}", message))?;
        
        match process_user_registration(&message, &ctx).await {
            Ok(result) => {
                ctx.logger().info("mq", &format!("User registration processed: {:?}", result))?;
                
                // 确认消息已处理
                ctx.mq().ack(&message).await?;
            }
            Err(e) => {
                ctx.logger().error("mq", &format!("Failed to process user registration: {}", e))?;
                
                // 拒绝消息并重新排队
                ctx.mq().nack(&message, true).await?;
            }
        }
        
        Ok(())
    }).await?;
    
    ctx.logger().info("mq", "Message queue subscriptions configured")?;
    
    Ok(())
}

async fn process_user_registration(message: &DMSCMessage, ctx: &DMSCServiceContext) -> DMSCResult<serde_json::Value> {
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let email = message.body["email"].as_str().unwrap_or_default();
    let name = message.body["name"].as_str().unwrap_or_default();
    
    // 验证用户数据
    if user_id == 0 || email.is_empty() || name.is_empty() {
        return Err(DMSCError::validation("Invalid user data".to_string()));
    }
    
    // 这里可以添加业务逻辑处理
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

## 代码解析

</div>

mq模块提供消息队列、发布订阅、路由、死信队列、延迟消息、持久化、优先级和过滤功能的使用示例。

## 基本消息队列操作

### 连接和配置

```rust
use dms::prelude::*;
use serde_json::json;

// RabbitMQ配置
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

// Apache Kafka配置
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

// Redis Streams配置
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

// 初始化消息队列
ctx.mq().init(rabbitmq_config).await?;
ctx.log().info("Message queue initialized");

// 测试连接
match ctx.mq().ping().await {
    Ok(_) => ctx.log().info("Message queue connection successful"),
    Err(e) => {
        ctx.log().error(format!("Message queue connection failed: {}", e));
        return Err(e);
    }
}
```

### 基本发布和订阅

```rust
use dms::prelude::*;
use serde_json::json;

// 发布消息到队列
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

// 订阅队列消息
ctx.mq().subscribe("user.registrations", |message, ctx| async move {
    ctx.log().info(format!("Received message: {:?}", message));
    
    match process_user_registration(&message, &ctx).await {
        Ok(result) => {
            ctx.log().info(format!("User registration processed: {:?}", result));
            
            // 发送确认响应
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
            
            // 确认消息已处理
            ctx.mq().ack(&message).await?;
        }
        Err(e) => {
            ctx.log().error(format!("Failed to process user registration: {}", e));
            
            // 拒绝消息并重新排队
            ctx.mq().nack(&message, true).await?;
        }
    }
    
    Ok(())
}).await?;

async fn process_user_registration(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let email = message.body["email"].as_str().unwrap_or_default();
    let name = message.body["name"].as_str().unwrap_or_default();
    
    // 验证用户数据
    if user_id == 0 || email.is_empty() || name.is_empty() {
        return Err(DMSCError::validation("Invalid user data".to_string()));
    }
    
    // 检查邮箱是否已存在
    let existing_user = ctx.database()
        .query_one("SELECT id FROM users WHERE email = $1", vec![email.into()])
        .await?;
    
    if existing_user.is_some() {
        return Err(DMSCError::business("Email already registered".to_string()));
    }
    
    // 创建用户记录
    let new_user_id = ctx.database()
        .execute(
            "INSERT INTO users (email, name, created_at) VALUES ($1, $2, $3) RETURNING id",
            vec![email.into(), name.into(), chrono::Utc::now().to_rfc3339().into()]
        )
        .await?;
    
    // 发送欢迎邮件
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

## 发布订阅模式

### 主题发布

```rust
use dms::prelude::*;
use serde_json::json;

// 发布到主题
let topic_message = DMSCMessage {
    id: uuid::Uuid::new_v4().to_string(),
    queue: "notifications".to_string(), // 主题名称
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

// 批量发布到主题
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

### 主题订阅

```rust
use dms::prelude::*;
use serde_json::json;

// 订阅主题（通配符模式）
ctx.mq().subscribe_to_topic("notifications", "user.activity.*", |message, ctx| async move {
    ctx.log().info(format!("Received user activity: {:?}", message));
    
    let event_type = message.routing_key.split('.').last().unwrap_or("unknown");
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    
    match event_type {
        "login" => {
            // 处理用户登录
            handle_user_login(user_id, &message, &ctx).await?;
        }
        "signup" => {
            // 处理用户注册
            handle_user_signup(user_id, &message, &ctx).await?;
        }
        "purchase" => {
            // 处理用户购买
            handle_user_purchase(user_id, &message, &ctx).await?;
        }
        _ => {
            ctx.log().warn(format!("Unknown event type: {}", event_type));
        }
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

// 多主题订阅
ctx.mq().subscribe_to_topics(vec![
    ("notifications", "user.activity.*"),
    ("system.events", "server.*"),
    ("analytics", "event.*"),
], |message, ctx| async move {
    ctx.log().info(format!("Received multi-topic message: {:?}", message));
    
    // 根据主题和路由键处理消息
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
    
    // 记录用户活动日志
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
            // 执行启动相关的处理
        }
        "shutdown" => {
            ctx.log().info("Server shutdown event received");
            // 执行关闭相关的处理
        }
        "error" => {
            ctx.log().error(format!("Server error: {:?}", message.body));
            // 发送告警通知
        }
        _ => {
            ctx.log().warn(format!("Unknown system event: {}", event_type));
        }
    }
    
    Ok(())
}

async fn process_analytics_event(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    // 处理分析事件，发送到分析系统
    let analytics_data = json!({
        "event": message.body,
        "timestamp": message.timestamp,
        "processed_at": chrono::Utc::now().to_rfc3339(),
    });
    
    // 发送到外部分析服务
    ctx.http().post("https://analytics.example.com/events")
        .json(&analytics_data)
        .send()
        .await?;
    
    Ok(())
}
```

## 路由和绑定

### 复杂路由

```rust
use dms::prelude::*;
use serde_json::json;

// 创建路由规则
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

// 发布带有路由键的消息
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

// 消息将根据路由规则被路由到不同的队列
ctx.mq().publish_with_routing(order_message).await?;

// 订阅特定路由模式的消息
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
    
    // 验证订单数据
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let total_amount = message.body["total_amount"].as_f64().unwrap_or(0.0);
    
    if user_id == 0 || total_amount <= 0.0 {
        return Err(DMSCError::validation("Invalid order data".to_string()));
    }
    
    // 检查用户是否存在
    let user_exists = ctx.database()
        .query_one("SELECT id FROM users WHERE id = $1", vec![user_id.into()])
        .await?
        .is_some();
    
    if !user_exists {
        return Err(DMSCError::not_found("User not found".to_string()));
    }
    
    // 创建订单记录
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
    
    // 发送库存检查消息
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

## 死信队列

### 配置死信队列

```rust
use dms::prelude::*;
use serde_json::json;

// 创建死信队列配置
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

// 处理死信队列消息
ctx.mq().subscribe("dlq.orders.failed", |message, ctx| async move {
    ctx.log().error(format!("Processing dead letter message: {:?}", message));
    
    // 获取原始消息信息
    let original_queue = message.headers.get("x-original-queue").unwrap_or(&"unknown".to_string());
    let retry_count = message.headers.get("x-retry-count")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let failure_reason = message.headers.get("x-failure-reason").unwrap_or(&"unknown".to_string());
    
    ctx.log().error(format!(
        "Message failed after {} retries in queue {}: {}",
        retry_count, original_queue, failure_reason
    ));
    
    // 记录失败消息到数据库
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
    
    // 发送告警通知
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
    
    // 确认死信消息
    ctx.mq().ack(&message).await?;
    
    Ok(())
}).await?;

// 重试机制示例
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
                // 重新排队进行重试
                let mut retry_message = message.clone();
                retry_message.headers.insert("x-retry-count".to_string(), (retry_count + 1).to_string());
                retry_message.headers.insert("x-failure-reason".to_string(), e.to_string());
                
                // 根据重试次数设置延迟
                let delay = match retry_count {
                    0 => Duration::from_secs(60),   // 1分钟
                    1 => Duration::from_secs(300),  // 5分钟
                    2 => Duration::from_secs(900),  // 15分钟
                    _ => Duration::from_secs(3600), // 1小时
                };
                
                retry_message.expiration = Some(chrono::Utc::now() + delay);
                
                ctx.mq().publish(retry_message).await?;
                ctx.mq().ack(message).await?;
            } else {
                // 达到最大重试次数，发送到死信队列
                ctx.mq().reject(message, false).await?;
            }
            
            Err(e)
        }
    }
}

async fn do_message_processing(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<serde_json::Value> {
    // 实际的消息处理逻辑
    let order_id = message.body["order_id"].as_str()
        .ok_or_else(|| DMSCError::validation("Order ID is required".to_string()))?;
    
    // 模拟处理失败的情况
    if order_id.starts_with("FAIL") {
        return Err(DMSCError::business("Simulated processing failure".to_string()));
    }
    
    // 正常处理逻辑
    Ok(json!({
        "status": "processed",
        "order_id": order_id,
        "processed_at": chrono::Utc::now().to_rfc3339(),
    }))
}
```

## 延迟消息

### 延迟消息处理

```rust
use dms::prelude::*;
use serde_json::json;

// 发送延迟消息
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

// 设置7天后过期（自动触发）
ctx.mq().publish_with_delay(delayed_message, Duration::from_days(7)).await?;

// 处理延迟消息
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
    
    // 检查用户是否仍然需要提醒
    let user = ctx.database()
        .query_one("SELECT id, email, subscription_status FROM users WHERE id = $1", vec![user_id.into()])
        .await?;
    
    if let Some(user_data) = user {
        let subscription_status = user_data.get::<String>("subscription_status").unwrap_or_default();
        
        if subscription_status == "active" {
            // 用户订阅仍然有效，不需要发送提醒
            ctx.log().info(format!("User {} subscription is still active, skipping reminder", user_id));
            return Ok(());
        }
        
        // 发送提醒邮件
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
        
        // 记录提醒已发送
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

// 定时任务调度
async fn schedule_periodic_tasks(ctx: &DMSCContext) -> DMSCResult<()> {
    // 每天凌晨2点执行数据清理
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
    
    // 计算到明天凌晨2点的时间差
    let now = chrono::Utc::now();
    let tomorrow_2am = now.date().and_hms(2, 0, 0) + Duration::from_days(1);
    let delay = tomorrow_2am - now;
    
    ctx.mq().publish_with_delay(cleanup_task, delay).await?;
    
    // 每周一上午9点生成报告
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
    
    // 计算到下周一早上的时间差
    let days_until_monday = (8 - now.weekday().num_from_monday() as i64) % 7;
    let next_monday_9am = now.date().and_hms(9, 0, 0) + Duration::from_days(days_until_monday);
    let delay = next_monday_9am - now;
    
    ctx.mq().publish_with_delay(report_task, delay).await?;
    
    Ok(())
}
```

## 消息优先级

### 优先级消息处理

```rust
use dms::prelude::*;
use serde_json::json;

// 发送不同优先级的消息
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

// 发布不同优先级的消息
ctx.mq().publish(urgent_message).await?;
ctx.mq().publish(high_priority_message).await?;
ctx.mq().publish(normal_message).await?;
ctx.mq().publish(low_priority_message).await?;

// 优先级队列订阅
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
    
    // 根据优先级处理消息
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
    
    // 立即发送紧急通知
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
    
    // 记录到监控系统
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
    
    // 发送通知到运维团队
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
    
    // 记录到日志系统
    ctx.log().info(format!("Alert processed: {} - {:?}", 
        message.body["alert_type"].as_str().unwrap_or_default(), 
        message.body));
    
    Ok(())
}

async fn handle_low_priority_alert(message: &DMSCMessage, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().debug(format!("LOW PRIORITY ALERT: {:?}", message.body));
    
    // 可以延迟处理或批量处理
    Ok(())
}
```

## 消息过滤

### 消息过滤器

```rust
use dms::prelude::*;
use serde_json::json;

// 创建消息过滤器
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

// 带过滤器的消息订阅
ctx.mq().subscribe_with_filters("user_events", |message, ctx| async move {
    ctx.log().info(format!("Filtered message received: {:?}", message));
    
    // 只有符合过滤条件的消息才会被接收
    let event_type = message.headers.get("event_type").unwrap_or(&"unknown".to_string());
    let priority = message.headers.get("priority")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    ctx.log().info(format!("Processing {} event with priority {}", event_type, priority));
    
    // 根据事件类型处理
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

// 动态过滤器
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

// 自定义过滤器函数
async fn custom_message_filter(message: &DMSCMessage, ctx: &DMSCContext) -> bool {
    // 自定义过滤逻辑
    let user_id = message.body["user_id"].as_i64().unwrap_or(0);
    let event_time = message.body["timestamp"].as_str().unwrap_or_default();
    
    // 过滤掉测试用户
    if user_id < 1000 {
        return false;
    }
    
    // 过滤掉过期事件（超过1小时）
    if let Ok(event_timestamp) = chrono::DateTime::parse_from_rfc3339(event_time) {
        let now = chrono::Utc::now();
        if now - event_timestamp > Duration::from_hours(1) {
            return false;
        }
    }
    
    // 过滤掉重复事件
    let event_key = format!("{}:{}", user_id, message.routing_key);
    if ctx.cache().exists(&event_key).await? {
        return false;
    }
    
    // 记录事件以防止重复
    ctx.cache().set(&event_key, "1", Duration::from_minutes(10)).await?;
    
    true
}
```

## 消息持久化

### 持久化配置

```rust
use dms::prelude::*;
use serde_json::json;

// 配置消息持久化
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

// 发送持久化消息
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

// 查询历史消息
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

// 消息重放
let replay_result = ctx.mq().replay_messages(
    "important.events",
    chrono::Utc::now() - Duration::from_hours(24),
    chrono::Utc::now(),
    Some("business.critical".to_string()),
).await?;

ctx.log().info(format!("Replayed {} messages", replay_result.replayed_count));
```

## 批量操作

### 批量消息处理

```rust
use dms::prelude::*;
use serde_json::json;

// 批量发布消息
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

// 批量确认消息
let messages_to_ack = vec![
    "msg-001".to_string(),
    "msg-002".to_string(),
    "msg-003".to_string(),
];

ctx.mq().batch_ack(messages_to_ack).await?;

// 批量处理消息（提高吞吐量）
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
    
    // 模拟处理逻辑
    if item_id % 50 == 0 {
        return Err(DMSCError::business(format!("Simulated failure for item {}", item_id)));
    }
    
    // 处理数据
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

## 错误处理

### 消息队列错误处理

```rust
use dms::prelude::*;
use serde_json::json;

// 错误处理示例
match ctx.mq().publish(message).await {
    Ok(_) => {
        ctx.log().info("Message published successfully");
    }
    Err(DMSCError::MessageQueueConnectionError(e)) => {
        ctx.log().error(format!("Message queue connection failed: {}", e));
        // 尝试重新连接或降级处理
        handle_mq_connection_error(&e, ctx).await?;
    }
    Err(DMSCError::MessageQueuePublishError(e)) => {
        ctx.log().error(format!("Message publish failed: {}", e));
        // 尝试重新发布或使用备用队列
        retry_message_publish(message, ctx).await?;
    }
    Err(DMSCError::MessageQueueTimeoutError(e)) => {
        ctx.log().warn(format!("Message queue operation timed out: {}", e));
        // 增加超时时间或分批处理
        handle_mq_timeout(&e, ctx).await?;
    }
    Err(DMSCError::MessageQueueConsumerError(e)) => {
        ctx.log().error(format!("Message consumer error: {}", e));
        // 重新启动消费者或切换到备用消费者
        restart_message_consumer(&e, ctx).await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected message queue error: {}", e));
        return Err(e);
    }
}

async fn handle_mq_connection_error(error: &str, ctx: &DMSCContext) -> DMSCResult<()> {
    ctx.log().warn("Message queue is unavailable, switching to local queue");
    
    // 启用本地队列降级
    ctx.cache().set("mq_fallback_enabled", "true", Duration::from_hours(1)).await?;
    
    // 定期重试连接
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
                    
                    // 发送到失败消息队列
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

## 运行步骤

</div>

### 1. 安装依赖

确保已安装 Rust 和 Cargo（版本 1.65+）：

```bash
cargo --version
```

### 2. 启动消息队列服务

根据配置的消息队列类型启动相应的服务：

**RabbitMQ:**
```bash
# 使用 Docker 启动 RabbitMQ
docker run -d --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3-management

# 访问管理界面 http://localhost:15672 (guest/guest)
```

**Apache Kafka:**
```bash
# 使用 Docker Compose 启动 Kafka
docker-compose up -d zookeeper kafka

# 或者使用 Docker 单独启动
docker run -d --name kafka -p 9092:9092 confluentinc/cp-kafka:latest
```

**Redis Streams:**
```bash
# 使用 Docker 启动 Redis
docker run -d --name redis -p 6379:6379 redis:7-alpine
```

### 3. 配置项目

创建配置文件 `config.yaml`，根据实际环境修改消息队列连接信息：

```yaml
message_queue:
  broker_type: "rabbitmq"  # 可选: rabbitmq, kafka, redis
  rabbitmq:
    host: "localhost"
    port: 5672
    username: "guest"
    password: "guest"
```

### 4. 运行示例

```bash
# 进入项目目录
cd dms-mq-example

# 运行应用
cargo run
```

### 5. 验证功能

应用启动后会自动执行以下操作：
- 连接消息队列服务
- 发布示例消息到用户注册队列
- 订阅并处理消息
- 显示处理结果和日志信息

<div align="center">

## 预期结果

</div>

运行成功后，您将看到类似以下的输出：

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

### 消息队列管理界面

- **RabbitMQ**: 访问 http://localhost:15672 查看队列状态
- **Kafka**: 使用 Kafka 工具查看主题和消息
- **Redis**: 使用 `redis-cli` 查看 Streams 数据

<div align="center">

## 扩展功能

</div>

### 负载均衡支持

```rust
use dms::prelude::*;

// 配置多个消息队列节点实现负载均衡
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

// 监控节点健康状况
ctx.mq().monitor_node_health(|node, status| async move {
    match status {
        DMSCNodeHealthStatus::Healthy => {
            ctx.log().info(format!("Message queue node {} is healthy", node.host));
        }
        DMSCNodeHealthStatus::Unhealthy => {
            ctx.log().warn(format!("Message queue node {} is unhealthy", node.host));
            // 触发故障转移
            ctx.mq().trigger_failover(node).await?;
        }
        DMSCNodeHealthStatus::Offline => {
            ctx.log().error(format!("Message queue node {} is offline", node.host));
            // 从负载均衡池中移除
            ctx.mq().remove_node_from_pool(node).await?;
        }
    }
    Ok(())
}).await?;
```

### 消息队列监控

```rust
use dms::prelude::*;
use serde_json::json;

// 配置消息队列监控
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

// 收集性能指标
ctx.mq().collect_metrics(|metrics| async move {
    ctx.log().info(format!("Queue metrics: {:?}", metrics));
    
    // 发送到监控系统
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

// 设置告警
ctx.mq().setup_alerts(|alert| async move {
    match alert.level {
        DMSCAlertLevel::Warning => {
            ctx.log().warn(format!("MQ Alert: {}", alert.message));
            // 发送警告通知
            send_alert_notification("warning", &alert, ctx).await?;
        }
        DMSCAlertLevel::Critical => {
            ctx.log().error(format!("MQ Critical Alert: {}", alert.message));
            // 发送紧急通知
            send_alert_notification("critical", &alert, ctx).await?;
            // 触发自动修复
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

### 消息追踪

```rust
use dms::prelude::*;
use serde_json::json;

// 配置分布式消息追踪
let tracing_config = DMSCMessageTracingConfig {
    enabled: true,
    sampling_rate: 0.1,  // 10% 采样率
    trace_header_name: "x-trace-id".to_string(),
    span_header_name: "x-span-id".to_string(),
    baggage_header_prefix: "x-baggage-".to_string(),
    max_trace_depth: 100,
    retention_days: 30,
};

ctx.mq().setup_distributed_tracing(tracing_config).await?;

// 发送带追踪信息的消息
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

// 处理带追踪信息的消息
ctx.mq().subscribe_with_tracing("user.events", |message, trace_context, ctx| async move {
    ctx.log().info(format!("Processing traced message: trace_id={}, span_id={}", 
        trace_context.trace_id, trace_context.span_id));
    
    // 记录追踪信息
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
    
    // 处理消息
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

### 消息压缩

```rust
use dms::prelude::*;
use serde_json::json;

// 配置消息压缩
let compression_config = DMSCMessageCompressionConfig {
    enabled: true,
    threshold_size: 1024,  // 1KB 以上启用压缩
    algorithms: vec![
        DMSCCompressionAlgorithm::Gzip,
        DMSCCompressionAlgorithm::Lz4,
        DMSCCompressionAlgorithm::Zstd,
    ],
    compression_level: 6,  // 压缩级别 1-9
    auto_decompress: true,
};

ctx.mq().setup_compression(compression_config).await?;

// 发送大消息（自动压缩）
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
            "compression_ratio": 0.0,  // 将由系统自动计算
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

// 大消息将自动压缩
ctx.mq().publish_compressed(large_message).await?;

// 处理压缩消息（自动解压缩）
ctx.mq().subscribe_compressed("data.updates", |message, compression_info, ctx| async move {
    ctx.log().info(format!("Received compressed message: original_size={}, compressed_size={}, ratio={}, algorithm={}", 
        compression_info.original_size,
        compression_info.compressed_size,
        compression_info.compression_ratio,
        compression_info.algorithm));
    
    // 处理解压缩后的消息
    let batch_id = message.body["batch_id"].as_str().unwrap_or_default();
    let records = message.body["records"].as_array().unwrap_or(&vec![]);
    
    ctx.log().info(format!("Processing batch {} with {} records", batch_id, records.len()));
    
    // 批量处理记录
    for record in records.chunks(100) {
        process_record_batch(record, &ctx).await?;
    }
    
    ctx.mq().ack(&message).await?;
    Ok(())
}).await?;

async fn process_record_batch(records: &[serde_json::Value], ctx: &DMSCContext) -> DMSCResult<()> {
    // 批量处理记录
    for record in records {
        let id = record["id"].as_i64().unwrap_or(0);
        let data = record["data"].as_str().unwrap_or_default();
        let status = record["metadata"]["status"].as_str().unwrap_or_default();
        
        // 处理每个记录
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

## 最佳实践

</div>

1. **消息幂等性**: 确保消息处理是幂等的，避免重复处理导致的问题
2. **消息确认**: 及时确认处理完成的消息，避免消息重复投递
3. **错误处理**: 妥善处理消息处理失败的情况，使用死信队列
4. **消息大小**: 控制消息大小，避免传输过大的消息
5. **连接管理**: 合理管理消息队列连接，使用连接池
6. **监控指标**: 监控消息队列的性能指标和健康状况
7. **备份策略**: 实施消息持久化和备份策略
8. **限流保护**: 实施消息生产和消费的速率限制
9. **版本管理**: 管理消息格式的版本兼容性
10. **安全认证**: 启用消息队列的安全认证和加密
11. **负载均衡**: 配置多节点负载均衡，提高可用性
12. **压缩优化**: 对大消息启用压缩，减少网络传输
13. **追踪监控**: 实施分布式追踪，便于问题定位
14. **批量处理**: 使用批量操作提高吞吐量
15. **优先级管理**: 合理使用消息优先级，确保重要消息优先处理

<div align="center">

## 总结

</div>

本示例全面展示了 DMSC 消息队列模块的核心功能和高级特性，涵盖以下关键能力：

### 🚀 核心功能
- **多消息队列支持**: RabbitMQ、Apache Kafka、Redis Streams 的无缝集成
- **发布订阅模式**: 灵活的主题发布和通配符订阅机制
- **消息路由**: 复杂的路由规则和队列绑定功能
- **死信队列**: 完善的消息重试和失败处理机制
- **延迟消息**: 定时任务和延迟消息处理支持
- **消息优先级**: 多级别优先级队列管理
- **消息过滤**: 基于内容和头部的智能消息过滤
- **消息持久化**: 可靠的消息存储和历史查询功能

### 🔧 高级特性
- **负载均衡**: 多节点负载均衡和故障转移机制
- **性能监控**: 实时队列监控和性能指标收集
- **分布式追踪**: 跨服务的消息链路追踪
- **消息压缩**: 大消息的自动压缩优化
- **批量操作**: 高效的批量消息处理
- **错误处理**: 完善的异常处理和降级策略

### 💡 最佳实践
- 消息幂等性设计，确保重复消息安全处理
- 及时的消息确认机制，避免消息重复投递
- 合理的消息大小控制，优化网络传输性能
- 完善的监控告警，保障系统稳定运行
- 多层次的错误处理，提高系统容错能力

通过本示例，您可以构建高可靠、高性能的分布式消息处理系统，支持复杂的业务场景和大规模数据处理需求。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API

- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作