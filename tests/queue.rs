// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate dms;

use dms::queue::{DMSQueueMessage, DMSQueue, DMSQueueConfig, QueueBackendType, DMSQueueManager, DMSQueueModule};
use dms::queue::backends::DMSMemoryQueue;

#[test]
fn test_queue_message_new() {
    let payload = b"test_payload".to_vec();
    
    let message = DMSQueueMessage::new(payload.clone());
    
    assert!(!message.id.is_empty());
    assert_eq!(message.payload, payload);
    assert!(message.headers.is_empty());
    assert_eq!(message.retry_count, 0);
    assert_eq!(message.max_retries, 3);
}

#[test]
fn test_queue_message_with_headers() {
    let payload = b"test_payload".to_vec();
    
    let mut headers = std::collections::HashMap::new();
    headers.insert("key1".to_string(), "value1".to_string());
    headers.insert("key2".to_string(), "value2".to_string());
    
    let message = DMSQueueMessage::new(payload.clone())
        .with_headers(headers.clone());
    
    assert_eq!(message.headers, headers);
}

#[test]
fn test_queue_message_with_max_retries() {
    let payload = b"test_payload".to_vec();
    
    let message = DMSQueueMessage::new(payload.clone())
        .with_max_retries(5);
    
    assert_eq!(message.max_retries, 5);
}

#[test]
fn test_queue_message_retry() {
    let payload = b"test_payload".to_vec();
    
    let mut message = DMSQueueMessage::new(payload.clone())
        .with_max_retries(3);
    
    // Test initial state
    assert_eq!(message.retry_count, 0);
    assert!(message.can_retry());
    
    // Test incrementing retry count
    message.increment_retry();
    assert_eq!(message.retry_count, 1);
    assert!(message.can_retry());
    
    // Test reaching max retries
    message.increment_retry();
    message.increment_retry();
    assert_eq!(message.retry_count, 3);
    assert!(!message.can_retry());
}

#[tokio::test]
async fn test_memory_queue_create_producer() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Test creating a producer
    let producer = queue.create_producer().await.unwrap();
    
    // Verify producer works by sending a message
    let message = DMSQueueMessage::new(b"test_payload".to_vec());
    producer.send(message).await.unwrap();
}

#[tokio::test]
async fn test_memory_queue_create_consumer() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Test creating a consumer
    let consumer = queue.create_consumer("test_consumer_group").await.unwrap();
    
    // Verify consumer works by receiving a message
    let result = consumer.receive().await.unwrap();
    assert!(result.is_none()); // No messages yet
}

#[tokio::test]
async fn test_memory_queue_send_receive() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Create producer and consumer
    let producer = queue.create_producer().await.unwrap();
    let consumer = queue.create_consumer("test_consumer_group").await.unwrap();
    
    // Send a message
    let payload = b"test_payload".to_vec();
    let message = DMSQueueMessage::new(payload.clone());
    producer.send(message.clone()).await.unwrap();
    
    // Receive the message
    let received = consumer.receive().await.unwrap();
    assert!(received.is_some());
    
    if let Some(received_message) = received {
        assert_eq!(received_message.payload, payload);
        // Acknowledge the message
        consumer.ack(&received_message.id).await.unwrap();
    }
}

#[tokio::test]
async fn test_memory_queue_send_batch() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Create producer
    let producer = queue.create_producer().await.unwrap();
    
    // Create multiple messages
    let messages = vec![
        DMSQueueMessage::new(b"payload1".to_vec()),
        DMSQueueMessage::new(b"payload2".to_vec()),
        DMSQueueMessage::new(b"payload3".to_vec()),
    ];
    
    // Send messages in batch
    producer.send_batch(messages).await.unwrap();
    
    // Verify messages were sent by checking queue stats
    let stats = queue.get_stats().await.unwrap();
    assert_eq!(stats.message_count, 3);
}

#[tokio::test]
async fn test_memory_queue_get_stats() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Get initial stats
    let stats = queue.get_stats().await.unwrap();
    
    assert_eq!(stats.queue_name, "test_queue");
    assert_eq!(stats.message_count, 0);
    assert_eq!(stats.consumer_count, 0);
    assert_eq!(stats.producer_count, 1);
}

#[tokio::test]
async fn test_memory_queue_purge() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Send some messages
    let producer = queue.create_producer().await.unwrap();
    let message = DMSQueueMessage::new(b"test_payload".to_vec());
    producer.send(message).await.unwrap();
    
    // Verify messages were sent
    let stats_before = queue.get_stats().await.unwrap();
    assert_eq!(stats_before.message_count, 1);
    
    // Purge the queue
    queue.purge().await.unwrap();
    
    // Verify queue is empty
    let stats_after = queue.get_stats().await.unwrap();
    assert_eq!(stats_after.message_count, 0);
}

#[tokio::test]
async fn test_memory_queue_delete() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Send some messages
    let producer = queue.create_producer().await.unwrap();
    let message = DMSQueueMessage::new(b"test_payload".to_vec());
    producer.send(message).await.unwrap();
    
    // Delete the queue
    queue.delete().await.unwrap();
    
    // Verify queue is empty
    let stats = queue.get_stats().await.unwrap();
    assert_eq!(stats.message_count, 0);
}

#[tokio::test]
async fn test_memory_queue_consumer_pause_resume() {
    let queue = DMSMemoryQueue::new("test_queue");
    
    // Create producer and consumer
    let producer = queue.create_producer().await.unwrap();
    let consumer = queue.create_consumer("test_consumer_group").await.unwrap();
    
    // Send a message
    let message = DMSQueueMessage::new(b"test_payload".to_vec());
    producer.send(message).await.unwrap();
    
    // Pause the consumer
    consumer.pause().await.unwrap();
    
    // Should not receive any messages when paused
    let result = consumer.receive().await.unwrap();
    assert!(result.is_none());
    
    // Resume the consumer
    consumer.resume().await.unwrap();
    
    // Should receive message now
    let result = consumer.receive().await.unwrap();
    assert!(result.is_some());
}

#[tokio::test]
async fn test_queue_manager_new() {
    let config = DMSQueueConfig::default();
    
    // Test creating a queue manager
    let queue_manager = DMSQueueManager::new(config).await.unwrap();
    
    // Test initializing the queue manager
    queue_manager.init().await.unwrap();
    
    // Test shutting down the queue manager
    queue_manager.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_queue_manager_create_queue() {
    let config = DMSQueueConfig::default();
    let queue_manager = DMSQueueManager::new(config).await.unwrap();
    
    // Test creating a queue
    let queue = queue_manager.create_queue("test_queue").await.unwrap();
    
    // Verify queue works by creating a producer
    let producer = queue.create_producer().await.unwrap();
    let message = DMSQueueMessage::new(b"test_payload".to_vec());
    producer.send(message).await.unwrap();
}

#[tokio::test]
async fn test_queue_manager_get_queue() {
    let config = DMSQueueConfig::default();
    let queue_manager = DMSQueueManager::new(config).await.unwrap();
    
    // Create a queue
    queue_manager.create_queue("test_queue").await.unwrap();
    
    // Test getting the queue
    let queue = queue_manager.get_queue("test_queue").await;
    assert!(queue.is_some());
    
    // Test getting a non-existent queue
    let non_existent_queue = queue_manager.get_queue("non_existent_queue").await;
    assert!(non_existent_queue.is_none());
}

#[tokio::test]
async fn test_queue_manager_list_queues() {
    let config = DMSQueueConfig::default();
    let queue_manager = DMSQueueManager::new(config).await.unwrap();
    
    // Test initial state
    let queues = queue_manager.list_queues().await;
    assert!(queues.is_empty());
    
    // Create some queues
    queue_manager.create_queue("test_queue1").await.unwrap();
    queue_manager.create_queue("test_queue2").await.unwrap();
    queue_manager.create_queue("test_queue3").await.unwrap();
    
    // Test listing queues
    let queues = queue_manager.list_queues().await;
    assert_eq!(queues.len(), 3);
    assert!(queues.contains(&"test_queue1".to_string()));
    assert!(queues.contains(&"test_queue2".to_string()));
    assert!(queues.contains(&"test_queue3".to_string()));
}

#[tokio::test]
async fn test_queue_manager_delete_queue() {
    let config = DMSQueueConfig::default();
    let queue_manager = DMSQueueManager::new(config).await.unwrap();
    
    // Create a queue
    queue_manager.create_queue("test_queue").await.unwrap();
    
    // Test deleting the queue
    queue_manager.delete_queue("test_queue").await.unwrap();
    
    // Verify queue was deleted
    let queues = queue_manager.list_queues().await;
    assert!(!queues.contains(&"test_queue".to_string()));
}

#[tokio::test]
async fn test_queue_module_new() {
    let config = DMSQueueConfig::default();
    
    // Test creating a queue module
    let queue_module = DMSQueueModule::new(config).await.unwrap();
    
    // Verify queue manager is accessible
    let queue_manager = queue_module.queue_manager();
    
    // Test creating a queue through the module
    queue_manager.create_queue("test_queue").await.unwrap();
}

#[test]
async fn test_queue_config_default() {
    let config = DMSQueueConfig::default();
    
    assert!(config.enabled);
    assert_eq!(config.backend_type, QueueBackendType::Memory);
    assert_eq!(config.connection_string, "memory://localhost");
    assert_eq!(config.max_connections, 10);
    assert_eq!(config.message_max_size, 1024 * 1024);
    assert_eq!(config.consumer_timeout_ms, 30000);
    assert_eq!(config.producer_timeout_ms, 5000);
    assert_eq!(config.retry_policy.max_retries, 3);
    assert_eq!(config.retry_policy.initial_delay_ms, 1000);
    assert_eq!(config.retry_policy.max_delay_ms, 60000);
    assert_eq!(config.retry_policy.backoff_multiplier, 2.0);
    assert!(config.dead_letter_config.is_none());
}

#[test]
async fn test_queue_backend_type_from_str() {
    // Test valid backend types
    assert_eq!("memory".parse::<QueueBackendType>().unwrap(), QueueBackendType::Memory);
    assert_eq!("rabbitmq".parse::<QueueBackendType>().unwrap(), QueueBackendType::RabbitMQ);
    assert_eq!("kafka".parse::<QueueBackendType>().unwrap(), QueueBackendType::Kafka);
    assert_eq!("redis".parse::<QueueBackendType>().unwrap(), QueueBackendType::Redis);
    
    // Test invalid backend type
    assert!("invalid".parse::<QueueBackendType>().is_err());
}
