//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # DMSC Message Queue Module Example
//!
//! This example demonstrates how to use the message queue module in DMSC,
//! including queue creation, message publishing, and consumer patterns.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example queue --features queue
//! ```
//!
//! ## Features Demonstrated
//!
//! - Queue creation and configuration
//! - Message publishing with different delivery guarantees
//! - Message consumption with acknowledgment
//! - Dead letter queue handling
//! - Queue statistics and monitoring

use dmsc::queue::{DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager, DMSCQueueMessage, DMSCRetryPolicy, DMSCDeadLetterConfig};
use dmsc::core::DMSCResult;

/// Main entry point for the message queue module example.
///
/// This function demonstrates the complete message queue workflow including:
/// - Queue module initialization with Redis backend configuration
/// - Queue creation with optional dead letter queues (DLQ)
/// - Message publishing with various features (priority, payload)
/// - Message consumption with acknowledgment pattern
/// - Message peeking without removal for monitoring
/// - Queue statistics monitoring
/// - Retry policy configuration for failed message handling
/// - Queue listing and cleanup operations
///
/// The example shows how DMSC handles asynchronous messaging with features
/// like reliable delivery, dead letter handling, and retry mechanisms
/// in a Rust async runtime environment.
fn main() -> DMSCResult<()> {
    println!("=== DMSC Message Queue Module Example ===\n");

    // Create async runtime for handling asynchronous queue operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute all async queue operations within the runtime
    rt.block_on(async {
        // Configuration Setup: Create queue module configuration
        // Using Redis as the message queue backend
        // Builder pattern for configuration parameters:
        // - host: Redis server hostname
        // - port: Redis server port (default: 6379)
        // - password: Redis authentication password (None for no auth)
        // - db: Redis database number (0-15)
        // - build(): Finalizes configuration into DMSCQueueConfig struct
        let queue_config = DMSCQueueConfig::redis()
            .with_host("localhost")
            .with_port(6379)
            .with_password(None)
            .with_db(0)
            .build();

        // Module Initialization: Create queue module instance
        // The module provides messaging capabilities with reliable delivery
        println!("1. Creating queue module...");
        let queue_module = DMSCQueueModule::new(queue_config).await?;
        
        // Get queue manager for queue operations
        // The manager provides operations for queue manipulation and messaging
        let manager = queue_module.get_manager();
        println!("   Queue module initialized\n");

        // Step 2: Create simple queue
        // Demonstrates basic queue creation without additional configuration
        // create_queue() parameters:
        // - name: &str unique queue identifier
        // - dlq_config: Option<DMSCDeadLetterConfig> for DLQ setup
        // Passing None means no dead letter queue for this queue
        println!("2. Creating 'orders' queue...");
        manager.create_queue("orders", None).await?;
        println!("   Queue 'orders' created\n");

        // Step 3: Create queue with dead letter queue (DLQ)
        // Dead letter queues handle messages that fail processing repeatedly
        // DLQ preserves failed messages for later analysis/reprocessing
        println!("3. Creating 'notifications' queue with dead letter queue...");

        // Configure dead letter queue settings using builder pattern:
        // - queue_name: Name of the DLQ for failed messages
        // - max_retries: Number of retry attempts before moving to DLQ
        // - ttl_secs: Time-to-live for messages in DLQ (24 hours)
        // - build(): Finalizes configuration
        let dlq_config = DMSCDeadLetterConfig::new()
            .with_queue_name("notifications_dlq")
            .with_max_retries(3)
            .with_ttl_secs(86400)
            .build();
        
        // Create queue with dead letter queue configuration
        // Messages that fail processing 3 times will be moved to notifications_dlq
        manager.create_queue("notifications", Some(dlq_config)).await?;
        println!("   Queue 'notifications' created with DLQ\n");

        // Step 4: Publish messages to orders queue
        // Demonstrates message creation and publishing with various features
        println!("4. Publishing messages to 'orders' queue...");

        // Create and publish multiple order messages
        // Each message has unique ID and payload data
        for i in 1..=5 {
            // Create message with order details using builder pattern
            // DMSCQueueMessage::new() creates message with:
            // - id: Unique message identifier
            // - payload: serde_json::Value containing message data
            let message = DMSCQueueMessage::new(
                format!("order-{}", i),
                serde_json::json!({
                    "order_id": i,
                    "product": format!("Product {}", i),
                    "quantity": i * 2,
                    "price": 29.99 * i as f64
                }),
            ).with_priority(if i == 1 { 10 } else { 5 });
            
            // Publish message to queue
            // Messages are stored and available for consumption
            manager.publish("orders", message).await?;
            println!("   Published order #{}", i);
        }
        println!();

        // Step 5: Publish notification messages
        // Demonstrates different message types in separate queue
        println!("5. Publishing notification messages...");

        // Define notification message types
        let notifications = vec![
            ("welcome", "Welcome to our service!"),
            ("promo", "Special discount available!"),
            ("alert", "Your account needs attention"),
        ];
        
        // Publish each notification message
        for (key, content) in &notifications {
            let message = DMSCQueueMessage::new(
                key.to_string(),
                serde_json::json!({
                    "type": key,
                    "content": content,
                    "sent_at": chrono::Utc::now().to_rfc3339()
                }),
            );
            manager.publish("notifications", message).await?;
            println!("   Published notification: {}", key);
        }
        println!();

        // Step 6: Consume messages from orders queue
        // Demonstrates message consumption with acknowledgment
        // Consumer pattern: fetch -> process -> acknowledge
        println!("6. Consuming messages from 'orders' queue...");
        let mut order_count = 0;
        
        // Consume up to 3 messages
        // consume() removes message from queue (pre-acknowledge pattern)
        // Parameters:
        // - queue_name: &str queue to consume from
        // - options: Option<ConsumerOptions> for consumption behavior
        while order_count < 3 {
            if let Some(msg) = manager.consume("orders", None).await? {
                order_count += 1;
                // Extract message data from JSON payload
                println!("   Received order #{}: id={}, product={}, quantity={}",
                    order_count,
                    msg.id(),
                    msg.payload().get("product").unwrap(),
                    msg.payload().get("quantity").unwrap()
                );
                
                // Acknowledge successful processing
                // Prevents message redelivery if consumer crashes
                // Parameters:
                // - queue_name: &str queue name
                // - message_id: &str ID of message to acknowledge
                manager.ack("orders", msg.id()).await?;
                println!("   Message acknowledged\n");
            }
        }

        // Step 7: Peek at next message
        // Demonstrates message inspection without removal
        // Useful for monitoring or previewing messages
        println!("7. Peeking at next order message...");
        if let Some(msg) = manager.peek("orders").await? {
            println!("   Next message: id={}, payload={}\n", msg.id(), msg.payload());
        }

        // Step 8: Get queue statistics
        // Demonstrates monitoring queue metrics
        println!("8. Getting queue statistics...");

        // Orders queue stats
        let orders_stats = manager.get_stats("orders").await?;
        println!("   'orders' queue stats:");
        println!("   - Messages in queue: {}", orders_stats.message_count());
        println!("   - Messages published: {}", orders_stats.published_count());
        println!("   - Messages consumed: {}", orders_stats.consumed_count());
        println!();

        // Notifications queue stats including DLQ
        let notifications_stats = manager.get_stats("notifications").await?;
        println!("   'notifications' queue stats:");
        println!("   - Messages in queue: {}", notifications_stats.message_count());
        println!("   - Dead letter count: {}\n", notifications_stats.dead_letter_count());

        // Step 9: Configure retry policy
        // Demonstrates automatic retry for failed message processing
        println!("9. Setting up retry policy for 'orders'...");

        // Configure retry behavior using builder pattern:
        // - max_retries: Maximum retry attempts before giving up
        // - initial_delay_ms: First retry delay (1 second)
        // - multiplier: Delay multiplier for exponential backoff (2x)
        // - max_delay_ms: Maximum delay cap (30 seconds)
        // Retry pattern: 1s, 2s, 4s, 8s, 16s, 30s
        let retry_policy = DMSCRetryPolicy::new()
            .with_max_retries(3)
            .with_initial_delay_ms(1000)
            .with_multiplier(2.0)
            .with_max_delay_ms(30000)
            .build();
        
        // Apply retry policy to orders queue
        // Failed messages will be retried with exponential backoff
        manager.set_retry_policy("orders", retry_policy).await?;
        println!("   Retry policy configured\n");

        // Step 10: List all queues
        // Demonstrates queue enumeration
        println!("10. Listing all queues...");
        let queues = manager.list_queues().await?;
        println!("   Available queues:");
        for queue in &queues {
            let stats = manager.get_stats(queue).await?;
            println!("   - {}: {} messages", queue, stats.message_count());
        }
        println!();

        // Step 11: Cleanup
        // Demonstrates queue deletion for resource cleanup
        println!("11. Cleaning up (deleting test queues)...");

        // Delete queues with force flag to remove even with messages
        // force=true bypasses safety checks for non-empty queues
        manager.delete_queue("orders", true).await?;
        manager.delete_queue("notifications", true).await?;
        println!("   Test queues deleted\n");

        println!("=== Message Queue Example Completed ===");
        Ok::<(), DMSCError>(())
    })?
}
