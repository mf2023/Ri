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

//! # Queue Module Benchmarks
//!
//! This benchmark suite measures the performance of DMSC queue operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dmsc::queue::backends::memory_backend::DMSCMemoryQueue;
use dmsc::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer};

fn bench_queue_send(c: &mut Criterion) {
    let mut group = c.benchmark_group("queue_send");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("send_small_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        b.iter(|| {
            rt.block_on(async {
                let queue = DMSCMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = DMSCQueueMessage::new(b"small payload".to_vec());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });
    
    group.bench_function("send_medium_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let payload = vec![0u8; 1024];
        b.iter(|| {
            rt.block_on(async {
                let queue = DMSCMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = DMSCQueueMessage::new(payload.clone());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });
    
    group.bench_function("send_large_message", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let payload = vec![0u8; 65536];
        b.iter(|| {
            rt.block_on(async {
                let queue = DMSCMemoryQueue::new("benchmark_queue");
                let producer = queue.create_producer().await.unwrap();
                let message = DMSCQueueMessage::new(payload.clone());
                producer.send(message).await.unwrap();
                black_box(());
            });
        });
    });
    
    group.finish();
}

fn bench_queue_receive(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let queue = DMSCMemoryQueue::new("receive_benchmark_queue");
    
    rt.block_on(async {
        let producer = queue.create_producer().await.unwrap();
        for i in 0..1000 {
            let message = DMSCQueueMessage::new(format!("message_{}", i).into_bytes());
            producer.send(message).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("queue_receive");
    group.throughput(Throughput::Elements(1));
    
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

fn bench_queue_batch_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("queue_batch");
    
    for size in [10, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("send_batch", size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async {
                    let queue = DMSCMemoryQueue::new("batch_queue");
                    let producer = queue.create_producer().await.unwrap();
                    let messages: Vec<DMSCQueueMessage> = (0..*size)
                        .map(|i| DMSCQueueMessage::new(format!("batch_{}", i).into_bytes()))
                        .collect();
                    producer.send_batch(messages).await.unwrap();
                    black_box(());
                });
            });
        });
    }
    
    group.finish();
}

fn bench_queue_ack_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("queue_ack");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("ack_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let queue = DMSCMemoryQueue::new("ack_queue");
                let producer = queue.create_producer().await.unwrap();
                let consumer = queue.create_consumer("ack_consumer").await.unwrap();
                
                let message = DMSCQueueMessage::new(b"test".to_vec());
                producer.send(message).await.unwrap();
                
                if let Some(received) = consumer.receive().await.unwrap() {
                    consumer.ack(&received.id).await.unwrap();
                    black_box(());
                }
            });
        });
    });
    
    group.bench_function("nack_message", |b| {
        b.iter(|| {
            rt.block_on(async {
                let queue = DMSCMemoryQueue::new("nack_queue");
                let producer = queue.create_producer().await.unwrap();
                let consumer = queue.create_consumer("nack_consumer").await.unwrap();
                
                let message = DMSCQueueMessage::new(b"test".to_vec());
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

fn bench_queue_stats(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let queue = DMSCMemoryQueue::new("stats_queue");
    
    rt.block_on(async {
        let producer = queue.create_producer().await.unwrap();
        for i in 0..100 {
            let message = DMSCQueueMessage::new(format!("stats_msg_{}", i).into_bytes());
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

fn bench_message_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("message_creation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("create_message", |b| {
        b.iter(|| {
            let message = DMSCQueueMessage::new(b"payload".to_vec());
            black_box(message);
        });
    });
    
    group.bench_function("create_message_with_retry", |b| {
        b.iter(|| {
            let message = DMSCQueueMessage::new(b"payload".to_vec())
                .with_max_retries(5);
            black_box(message);
        });
    });
    
    group.finish();
}

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
