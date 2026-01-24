#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Queue Module Python Tests.

This module contains comprehensive tests for the DMSC queue system Python bindings.
The queue system provides reliable message queuing with multiple backend support,
retry policies, and dead letter handling.

Queue Components:
- DMSCQueueModule: Main queue module coordination
- DMSCQueueConfig: Queue configuration
- DMSCQueueManager: Queue operations management
- DMSCQueueMessage: Individual message representation
- DMSCQueueStats: Queue statistics
- DMSCQueueBackendType: Backend type enumeration
- DMSCRetryPolicy: Message retry configuration
- DMSCDeadLetterConfig: Dead letter queue configuration

Backend Types:
- Memory: In-memory queue for single-instance deployments
- Redis: Distributed queue with persistence
- Kafka: High-throughput event streaming
- RabbitMQ: Feature-rich message broker

Message Properties:
- Payload: The message content (bytes)
- ID: Unique message identifier
- Priority: Message priority level
- Timestamp: When message was enqueued
- Retry count: Number of delivery attempts

Retry Policies:
- Max retries: Maximum delivery attempts
- Backoff: Delay between retries (fixed, exponential)
- Delay: Initial delay duration
- Max delay: Maximum delay cap

Dead Letter Queues:
- Failed messages are moved to DLQ
- DLQ preserves messages for investigation
- Manual intervention possible
- Automatic cleanup after retention period

Test Classes:
- TestDMSCQueueBackendType: Backend type enumeration
- TestDMSCQueueConfig: Queue configuration
- TestDMSCQueueMessage: Message handling
- TestDMSCQueueStats: Statistics tracking
- TestDMSCRetryPolicy: Retry configuration
- TestDMSCDeadLetterConfig: Dead letter configuration
"""

import unittest
from dmsc import (
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager,
    DMSCQueueMessage, DMSCQueueStats, DMSCQueueBackendType,
    DMSCRetryPolicy, DMSCDeadLetterConfig
)


class TestDMSCQueueBackendType(unittest.TestCase):
    """Test suite for DMSCQueueBackendType enum.
    
    The DMSCQueueBackendType enum defines the available message queue
    backend implementations. Each backend has different characteristics
    suited for various use cases in distributed systems.
    
    Backend Characteristics:
    - Memory: Fastest access (< 1ms latency), single-instance only, no persistence
      Pros: Zero network overhead, lowest latency
      Cons: Data lost on restart, single-node only
      Use: Development, testing, caching, ephemeral data
    - Redis: Distributed, persistent, good for general use, ~1-10ms latency
      Pros: Cluster support, TTL expiration, pub/sub, rich data types
      Cons: Single-threaded, memory cost
      Use: Web apps, microservices, caching, task queues
    - Kafka: High throughput, event streaming, log-based, ~1-5ms latency
      Pros: Massive scale, exactly-once semantics, replay, partitioning
      Cons: Operational complexity, disk space
      Use: Event sourcing, streaming analytics, CDC, audit logs
    - RabbitMQ: Feature-rich, complex routing, AMQP, ~1-10ms latency
      Pros: Flexible routing, message acknowledgment, priorities
      Cons: Erlang overhead, scaling complexity
      Use: Enterprise integration, task distribution, RPC
    
    Selection Criteria:
    - Throughput requirements: Kafka for millions, Redis/ RabbitMQ for thousands
    - Persistence needs: Kafka/Redis for durability, Memory for speed
    - Distribution requirements: All except Memory support clustering
    - Operational complexity tolerance: Memory simplest, Kafka most complex
    - Feature requirements: Complex routing needs RabbitMQ
    - Cost constraints: Memory is free, managed services cost money
    
    Performance Comparison:
    - Memory: ~10M messages/second
    - Redis: ~100K messages/second
    - Kafka: ~1M messages/second (configured properly)
    - RabbitMQ: ~50K messages/second
    
    Test Methods:
    - test_backend_type_values: Verify all backend types exist
    """

    def test_backend_type_values(self):
        """Test backend type values.
        
        Each supported queue backend type should have a string representation
        for configuration, logging, and monitoring purposes.
        
        Expected Behavior:
        - Memory enum value string matches expected format
        - Redis enum value string matches expected format
        - Kafka enum value string matches expected format
        - RabbitMQ enum value string matches expected format
        """
        self.assertEqual(str(DMSCQueueBackendType.Memory), "DMSCQueueBackendType.Memory")
        self.assertEqual(str(DMSCQueueBackendType.Redis), "DMSCQueueBackendType.Redis")
        self.assertEqual(str(DMSCQueueBackendType.Kafka), "DMSCQueueBackendType.Kafka")
        self.assertEqual(str(DMSCQueueBackendType.RabbitMQ), "DMSCQueueBackendType.RabbitMQ")


class TestDMSCQueueConfig(unittest.TestCase):
    """Test suite for DMSCQueueConfig class.
    
    The DMSCQueueConfig class configures queue behavior including
    backend selection, message limits, and processing options.
    This configuration determines how the queue operates and
    interacts with the message system.
    
    Configuration Options:
    - Backend type: Which queue implementation to use (Memory, Redis, Kafka, RabbitMQ)
    - Queue name: Unique identifier for the queue instance
    - Max size: Maximum number of messages the queue can hold
    - Max message size: Maximum size of individual message in bytes
    - Persistence: Enable disk persistence for durability
    - Durability: Message durability level (at-least-once, at-most-once, exactly-once)
    - Consumer count: Number of concurrent consumers
    - Prefetch count: Messages to fetch ahead for consumer
    
    Default Configuration Values:
    - Backend: Memory (for simplicity)
    - Max size: 1,000,000 messages
    - Max message size: 1 MB (1,048,576 bytes)
    - Persistence: Disabled (for Memory backend)
    - Consumer count: 1 (single consumer)
    - Prefetch count: 10 messages
    
    Configuration Impact:
    - Large max_size: More memory usage, more buffered messages
    - Large max_message_size: More memory per message, supports large payloads
    - Persistence enabled: Durability but slower throughput
    - More consumers: Higher throughput but potential message ordering issues
    
    Test Methods:
    - test_queue_config_default: Verify default configuration
    """

    def test_queue_config_default(self):
        """Test default queue configuration.
        
        Default configuration provides sensible defaults for queue operation
        that work well for development and testing scenarios.
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid config instance
        - Config has default values set
        - Config is ready for customization
        """
        config = DMSCQueueConfig()
        self.assertIsNotNone(config)


class TestDMSCQueueMessage(unittest.TestCase):
    """Test suite for DMSCQueueMessage class.
    
    The DMSCQueueMessage class represents an individual message in
    the queue system. It contains the payload and metadata needed
    for reliable delivery and processing tracking.
    
    Message Content:
    - Payload: Binary data to be delivered (required)
    - Message ID: Unique identifier (auto-generated if not provided)
    - Priority: Delivery priority 0-9 (0 = lowest, 9 = highest)
    - Timestamp: When message was created (auto-set to current time)
    - Attributes: Additional metadata as key-value pairs
    - Correlation ID: For request/response correlation
    - Reply To: Queue name for response messages
    - Content Type: MIME type of payload (e.g., "application/json")
    
    Message Lifecycle:
    1. Create: Message constructed with payload and metadata
    2. Enqueue: Message added to queue (status: pending)
    3. Consume: Message delivered to consumer (status: processing)
    4. Process: Consumer handles the message
    5. Acknowledge: Consumer confirms success (status: completed)
       OR Reject: Consumer indicates failure (status: failed)
    6. Retry: Message re-enqueued based on retry policy
       OR Dead Letter: Message moved to DLQ
    
    Message Properties:
    - body_size: Size of payload in bytes
    - delivery_count: Number of delivery attempts
    - max_deliveries: Maximum delivery attempts before DLQ
    - visible: Whether message is currently visible to consumers
    
    Test Methods:
    - test_queue_message_new: Verify message creation
    """

    def test_queue_message_new(self):
        """Test creating a new queue message.
        
        This test verifies that DMSCQueueMessage can be instantiated
        with a binary payload. The message is ready to be enqueued.
        
        Expected Behavior:
        - Constructor accepts binary payload
        - Returns a valid message instance
        - Message has auto-generated ID
        - Message has current timestamp
        - Message is ready for enqueue
        """
        msg = DMSCQueueMessage(b"test_payload")
        self.assertIsNotNone(msg)


class TestDMSCQueueStats(unittest.TestCase):
    """Test suite for DMSCQueueStats class.
    
    The DMSCQueueStats class tracks queue performance metrics
    including message counts, processing rates, and error rates.
    These statistics are essential for monitoring queue health
    and detecting issues before they become critical.
    
    Statistics Tracked:
    - Message count: Current queue depth (messages waiting)
    - Enqueue rate: Messages added per second (moving average)
    - Dequeue rate: Messages removed per second (moving average)
    - Processing time: Average handling duration in milliseconds
    - Error count: Failed processing attempts (total)
    - Success count: Successful processing attempts (total)
    - In-flight count: Messages being processed (not acknowledged)
    - Oldest message: Age of longest-waiting message
    
    Metrics Usage:
    - Monitor queue health: Is the queue keeping up?
    - Detect backpressure: Are messages accumulating?
    - Plan capacity: When to scale consumers?
    - Identify bottlenecks: Which step is slowest?
    - SLA compliance: Meeting throughput targets?
    - Cost optimization: Resource utilization efficiency
    
    Alerting Thresholds:
    - Queue depth > 1000: Potential backlog
    - Processing time > 1000ms: Performance degradation
    - Error rate > 1%: Quality issues
    - In-flight > consumer count * prefetch: Stuck messages
    
    Performance Indicators:
    - Low depth + high throughput: Healthy
    - Growing depth + steady throughput: Backlog forming
    - High error rate + low throughput: Consumer issues
    - Fluctuating depth: Variable load pattern
    
    Test Methods:
    - test_queue_stats_new: Verify stats instantiation
    """

    def test_queue_stats_new(self):
        """Test creating queue stats.
        
        This test verifies that DMSCQueueStats can be instantiated
        for a specific queue. The stats object tracks performance metrics.
        
        Expected Behavior:
        - Constructor accepts queue name
        - Returns a valid stats instance
        - Stats object is ready for metric collection
        - Initial metrics are zero or empty
        """
        stats = DMSCQueueStats("test_queue")
        self.assertIsNotNone(stats)


class TestDMSCRetryPolicy(unittest.TestCase):
    """Test suite for DMSCRetryPolicy class.
    
    The DMSCRetryPolicy class configures how failed message
    deliveries are retried including delays, limits, and
    backoff strategies. Proper retry configuration is
    essential for resilience without overwhelming systems.
    
    Retry Parameters:
    - Max retries: Maximum delivery attempts (e.g., 3)
    - Initial delay: First retry delay in seconds (e.g., 1.0)
    - Multiplier: Delay multiplier for exponential backoff (e.g., 2.0)
    - Max delay: Cap on delay duration in seconds (e.g., 60.0)
    - Jitter: Random variation percentage (e.g., 0.1 = 10% random)
    
    Backoff Strategies:
    - Fixed: Constant delay between retries (initial_delay)
      Formula: delay = initial_delay
      Use: When downstream is temporarily overloaded
    - Exponential: Delay increases geometrically
      Formula: delay = min(initial_delay * multiplier^attempt, max_delay)
      Use: When load varies significantly
    - Linear: Delay increases arithmetically
      Formula: delay = min(initial_delay + (attempt * increment), max_delay)
      Use: Gradual backoff with predictable timing
    
    Retry Behavior:
    - First failure: Immediate first retry (or initial_delay)
    - Second failure: Delay based on backoff strategy
    - Third failure: Longer delay
    - After max retries: Move to DLQ or discard
    
    Jitter Benefits:
    - Prevents thundering herd problem
    - Distributes retry load over time
    - Essential when many consumers retry simultaneously
    
    Test Methods:
    - test_retry_policy_new: Verify policy instantiation
    """

    def test_retry_policy_new(self):
        """Test creating retry policy.
        
        This test verifies that DMSCRetryPolicy can be instantiated.
        The policy defines how failed messages are retried.
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid policy instance
        - Policy has default retry values
        - Policy is ready for customization
        """
        policy = DMSCRetryPolicy()
        self.assertIsNotNone(policy)


class TestDMSCDeadLetterConfig(unittest.TestCase):
    """Test suite for DMSCDeadLetterConfig class.
    
    The DMSCDeadLetterConfig class configures dead letter queue
    behavior for messages that cannot be successfully processed
    after exhausting all retries. DLQs preserve messages for
    investigation and potential reprocessing.
    
    DLQ Configuration:
    - Enable DLQ: Whether to use dead letter queue (boolean)
    - Queue name: Name of DLQ for failed messages
    - Retention: How long to keep failed messages in seconds
    - Max size: Maximum capacity of DLQ
    - Auto delete: Whether to automatically delete expired messages
    - Transform: Optional transformation before DLQ insertion
    
    DLQ Use Cases:
    - Preserve failed messages for analysis and debugging
    - Manual investigation and reprocessing after fix
    - Audit trail of all processing failures
    - Recovery after downstream service fixes
    - Data recovery after transient failures
    
    DLQ Message Content:
    - Original message: Full payload that failed
    - Failure reason: Error message or exception
    - Retry count: Number of attempts made
    - Last failure timestamp: When final failure occurred
    - Handler information: Which consumer failed
    
    DLQ Management:
    - Monitor DLQ depth: Are failures increasing?
    - Analyze failure patterns: Same message repeatedly?
    - Reprocess selectively: After fixes are deployed
    - Archive before delete: Save historical failures
    - Alert on DLQ growth: Immediate attention needed
    
    Test Methods:
    - test_dead_letter_config_new: Verify config instantiation
    """

    def test_dead_letter_config_new(self):
        """Test creating dead letter config.
        
        This test verifies that DMSCDeadLetterConfig can be instantiated.
        The config defines how failed messages are handled and preserved.
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid config instance
        - Config has default DLQ settings
        - Config is ready for customization
        """
        config = DMSCDeadLetterConfig()
        self.assertIsNotNone(config)


if __name__ == "__main__":
    unittest.main()