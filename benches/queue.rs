//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

//! # Queue Module Benchmarks
//!
//! This module provides performance benchmarks for the Ri message queue system,
//! specifically measuring in-memory queue operations via RiMemoryQueue.
//!
//! ## Benchmark Categories
//!
//! 1. **Send Operations**: Message publishing with varying payload sizes
//!
//! 2. **Receive Operations**: Message consumption from queue
//!
//! 3. **Batch Operations**: Bulk send/receive for efficiency
//!
//! 4. **Ack Operations**: Message acknowledgment and rejection
//!
//! 5. **Stats Operations**: Queue statistics retrieval
//!
//! 6. **Message Creation**: Object allocation overhead
//!
//! ## Queue Architecture
//!
//! RiMemoryQueue provides:
//! - In-process message queue
//! - Producer/Consumer pattern
//! - Message acknowledgment (ack/nack)
//! - At-least-once delivery semantics
//! - Statistics tracking
//!
//! Used for:
//! - Async task processing
//! - Event-driven architectures
//! - Work queue patterns
//! - Decoupling producers from consumers

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ri::queue::backends::memory_backend::RiMemoryQueue;
use ri::queue::{RiQueue, RiQueueMessage, RiQueueProducer, RiQueueConsumer};

/// Benchmark: Queue SEND operations with varying message sizes.
///
/// SEND operations enqueue messages for async processing:
/// - Small: 12 bytes (typical short command)
/// - Medium: 1KB (typical API response)
/// - Large: 64KB (file content, batch data)
fn bench_queue_send(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_send");
    group.throughput(Throughput::Elements(1));

    /// Small message: Short commands, status updates
    group.bench_function("send_small_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let queue = RiMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = RiQueueMessage::new(b"small payload".to_vec());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });

    /// Medium message: Typical JSON payload, API response
    group.bench_function("send_medium_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let payload = vec![0u8; 1024];
        b.iter(|| {
            rt.block_on(async {
                let queue = RiMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = RiQueueMessage::new(payload.clone());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });

    /// Large message: File uploads, batch data
    group.bench_function("send_large_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let payload = vec![0u8; 65536];
        b.iter(|| {
            rt.block_on(async {
                let queue = RiMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = RiQueueMessage::new(payload.clone());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });

    group.finish();
}

/// Benchmark: Queue RECEIVE operations.
///
/// RECEIVE operations dequeue messages for processing:
/// - Blocks if no messages available (optional timeout)
/// - Returns message with unique ID for ack/nack
///
/// Pre-populating queue simulates warmed-up state.
fn bench_queue_receive(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let queue = RiMemoryQueue::new("receive_benchmark_queue");

    /// Pre-populate queue with 1000 messages
    rt.block_on(async {
        let producer = queue.create_producer().await.unwrap();
        for i in 0..1000 {
            let message = RiQueueMessage::new(format!("message_{}", i).into_bytes());
            producer.send(message).await.unwrap();
        }
    });

    let mut group = c.benchmark_group("queue_receive");
    group.throughput(Throughput::Elements(1));

    /// Single message receive: Consume one message from queue
    group.bench_function("receive_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let consumer = queue.create_consumer("benchmark_consumer").await.unwrap();
                let result = consumer.receive().await.unwrap();
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Benchmark: Queue BATCH operations for bulk processing.
///
/// Batch operations improve throughput:
/// - Single network round-trip for multiple messages
/// - Reduced syscall overhead
/// - Better batch processing patterns
///
/// Test with batch sizes: 10, 100, 1000
fn bench_queue_batch_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("queue_batch");

    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        /// Batch send: Publish multiple messages in single operation
        group.bench_with_input(BenchmarkId::new("send_batch", size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async {
                    let queue = RiMemoryQueue::new("batch_queue");
                    let producer = queue.create_producer().await.unwrap();
                    let messages: Vec<RiQueueMessage> = (0..*size)
                        .map(|i| RiQueueMessage::new(format!("batch_{}", i).into_bytes()))
                        .collect();
                    producer.send_batch(messages).await.unwrap();
                    black_box(());
                });
            });
        });
    }

    group.finish();
}

/// Benchmark: Message ACK/NACK operations.
///
/// Acknowledgment operations:
///
/// ACK (acknowledge):
/// - Confirms successful message processing
/// - Removes message from queue
/// - Enables at-least-once delivery
///
/// NACK (negative acknowledge):
/// - Indicates processing failure
/// - Can trigger requeue or dead-letter
/// - Enables retry patterns
fn bench_queue_ack_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("queue_ack");
    group.throughput(Throughput::Elements(1));

    /// Acknowledge: Mark message as successfully processed
    group.bench_function("ack_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let queue = RiMemoryQueue::new("ack_queue");
                let producer = queue.create_producer().await.unwrap();
                let consumer = queue.create_consumer("ack_consumer").await.unwrap();

                let message = RiQueueMessage::new(b"test".to_vec());
                producer.send(message).await.unwrap();

                if let Some(received) = consumer.receive().await.unwrap() {
                    consumer.ack(&received.id).await.unwrap();
                    black_box(());
                }
            });
        });
    });

    /// Negative acknowledge: Mark message for retry/requeue
    group.bench_function("nack_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let queue = RiMemoryQueue::new("nack_queue");
                let producer = queue.create_producer().await.unwrap();
                let consumer = queue.create_consumer("nack_consumer").await.unwrap();

                let message = RiQueueMessage::new(b"test".to_vec());
                producer.send(message).await.unwrap();

                if let Some(received) = consumer.receive().await.unwrap() {
                    consumer.nack(&received.id).await.unwrap();
                    black_box(());
                }
            });
        });
    });

    group.finish();
}

/// Benchmark: Queue statistics retrieval.
///
/// Queue stats provide visibility into:
/// - Queue depth (pending messages)
/// - Consumer count
/// - Throughput metrics
/// - Memory usage estimates
///
/// Typically used for monitoring and alerting.
fn bench_queue_stats(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let queue = RiMemoryQueue::new("stats_queue");

    /// Pre-populate with 100 messages
    rt.block_on(async {
        let producer = queue.create_producer().await.unwrap();
        for i in 0..100 {
            let message = RiQueueMessage::new(format!("stats_msg_{}", i).into_bytes());
            producer.send(message).await.unwrap();
        }
    });

    let mut group = c.benchmark_group("queue_stats");
    group.throughput(Throughput::Elements(1));

    group.bench_function("get_stats", |b| {
        b.iter(|| {
            rt.block_on(async {
                let stats = queue.get_stats().await.unwrap();
                black_box(stats);
            });
        });
    });

    group.finish();
}

/// Benchmark: Queue message object creation.
///
/// Message creation overhead includes:
/// - Unique ID generation (UUID)
/// - Timestamp assignment
/// - Payload allocation
/// - Metadata initialization
///
/// Options like retry configuration add additional overhead.
fn bench_message_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_creation");
    group.throughput(Throughput::Elements(1));

    /// Basic message: Just payload with auto-generated ID
    group.bench_function("create_message", |b| {
        b.iter(|| {
            let message = RiQueueMessage::new(b"payload".to_vec());
            black_box(message);
        });
    });

    /// Message with retry config: Additional options processing
    group.bench_function("create_message_with_retry", |b| {
        b.iter(|| {
            let message = RiQueueMessage::new(b"payload".to_vec())
                .with_max_retries(5);
            black_box(message);
        });
    });

    group.finish();
}

/// Benchmark group registration for queue module benchmarks.
criterion_group!(
    queue_benches,
    bench_queue_send,
    bench_queue_receive,
    bench_queue_batch_operations,
    bench_queue_ack_operations,
    bench_queue_stats,
    bench_message_creation,
);

criterion_main!(queue_benches);
