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
DMSC Message Queue Module Example

This example demonstrates how to use the message queue module in DMSC,
including queue creation, message publishing, and consumer patterns.

Features Demonstrated:
- Queue creation and configuration
- Message publishing with different delivery guarantees
- Message consumption with acknowledgment
- Dead letter queue handling
- Queue statistics and monitoring
"""

import dmsc
from dmsc.queue import (
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager,
    DMSCQueueMessage, DMSCRetryPolicy, DMSCDeadLetterConfig,
)
import asyncio
from datetime import datetime


async def main():
    """
    Main async entry point for the message queue module example.
    
    This function demonstrates the complete message queue workflow including:
    - Queue module initialization and configuration
    - Queue creation with optional dead letter queues
    - Message publishing with various features (priority, payload)
    - Message consumption with acknowledgment
    - Message peeking without removal
    - Queue statistics monitoring
    - Retry policy configuration
    - Queue listing and cleanup
    
    The example shows how DMSC handles asynchronous messaging with features
    like reliable delivery, dead letter handling, and retry mechanisms.
    """
    print("=== DMSC Message Queue Module Example ===\n")
    
    # Configuration Setup: Create queue module configuration
    # Using Redis as the message queue backend
    # Parameters:
    # - host: Redis server hostname
    # - port: Redis server port (default: 6379)
    # - password: Redis authentication password (None for no auth)
    # - db: Redis database number (0-15)
    queue_config = DMSCQueueConfig.redis(
        host="localhost",
        port=6379,
        password=None,
        db=0,
    )
    
    # Module Initialization: Create queue module instance
    # The module provides messaging capabilities with reliable delivery
    print("1. Creating queue module...")
    queue_module = DMSCQueueModule(queue_config)
    
    # Get queue manager for queue operations
    # The manager provides operations for queue manipulation and messaging
    manager = queue_module.get_manager()
    print("   Queue module initialized\n")
    
    # Step 2: Create simple queue
    # Demonstrates basic queue creation without additional configuration
    print("2. Creating 'orders' queue...")
    await manager.create_queue("orders")
    print("   Queue 'orders' created\n")
    
    # Step 3: Create queue with dead letter queue (DLQ)
    # Dead letter queues handle messages that fail processing repeatedly
    # DLQ preserves failed messages for later analysis/reprocessing
    print("3. Creating 'notifications' queue with dead letter queue...")
    
    # Configure dead letter queue settings
    # - queue_name: Name of the DLQ for failed messages
    # - max_retries: Number of retry attempts before moving to DLQ
    # - ttl_secs: Time-to-live for messages in DLQ (24 hours)
    dlq_config = DMSCDeadLetterConfig(
        queue_name="notifications_dlq",
        max_retries=3,
        ttl_secs=86400,
    )
    await manager.create_queue("notifications", dlq_config)
    print("   Queue 'notifications' created with DLQ\n")
    
    # Step 4: Publish messages to orders queue
    # Demonstrates message creation and publishing with various features
    print("4. Publishing messages to 'orders' queue...")
    
    # Create and publish multiple order messages
    # Each message has unique ID and payload data
    for i in range(1, 6):
        # Create message with order details
        # - id: Unique message identifier
        # - payload: Dictionary containing message data
        message = DMSCQueueMessage(
            id=f"order-{i}",
            payload={
                "order_id": i,
                "product": f"Product {i}",
                "quantity": i * 2,
                "price": 29.99 * i,
            },
        )
        
        # Set priority for first message (higher priority = processed first)
        # Priority scale varies by implementation (1-10, 1-100, etc.)
        if i == 1:
            message.set_priority(10)
        
        # Publish message to queue
        # Messages are stored and available for consumption
        await manager.publish("orders", message)
        print(f"   Published order #{i}")
    print()
    
    # Step 5: Publish notification messages
    # Demonstrates different message types in separate queue
    print("5. Publishing notification messages...")
    
    # Define notification message types
    notifications = [
        ("welcome", "Welcome to our service!"),
        ("promo", "Special discount available!"),
        ("alert", "Your account needs attention"),
    ]
    
    # Publish each notification message
    for key, content in notifications:
        message = DMSCQueueMessage(
            id=key,
            payload={
                "type": key,
                "content": content,
                "sent_at": datetime.utcnow().isoformat(),
            },
        )
        await manager.publish("notifications", message)
        print(f"   Published notification: {key}")
    print()
    
    # Step 6: Consume messages from orders queue
    # Demonstrates message consumption with acknowledgment
    # Consumer pattern: fetch -> process -> acknowledge
    print("6. Consuming messages from 'orders' queue...")
    order_count = 0
    
    # Consume up to 3 messages
    # consume() removes message from queue (pre-acknowledge)
    while order_count < 3:
        msg = await manager.consume("orders")
        if msg:
            order_count += 1
            print(f"   Received order #{order_count}: id={msg.id()}, "
                  f"product={msg.payload().get('product')}, "
                  f"quantity={msg.payload().get('quantity')}")
            
            # Acknowledge successful processing
            # Prevents message redelivery if consumer crashes
            await manager.ack("orders", msg.id())
            print("   Message acknowledged\n")
    
    # Step 7: Peek at next message
    # Demonstrates message inspection without removal
    # Useful for monitoring or previewing messages
    print("7. Peeking at next order message...")
    msg = await manager.peek("orders")
    if msg:
        print(f"   Next message: id={msg.id()}, payload={msg.payload()}\n")
    
    # Step 8: Get queue statistics
    # Demonstrates monitoring queue metrics
    print("8. Getting queue statistics...")
    
    # Orders queue stats
    orders.get_stats("orders_stats = await manager")
    print("   'orders' queue stats:")
    print(f"   - Messages in queue: {orders_stats.message_count()}")
    print(f"   - Messages published: {orders_stats.published_count()}")
    print(f"   - Messages consumed: {orders_stats.consumed_count()}")
    print()
    
    # Notifications queue stats including DLQ
    notifications_stats = await manager.get_stats("notifications")
    print("   'notifications' queue stats:")
    print(f"   - Messages in queue: {notifications_stats.message_count()}")
    print(f"   - Dead letter count: {notifications_stats.dead_letter_count()}")
    print()
    
    # Step 9: Configure retry policy
    # Demonstrates automatic retry for failed message processing
    print("9. Setting up retry policy for 'orders'...")
    
    # Configure retry behavior
    # - max_retries: Maximum retry attempts before giving up
    # - initial_delay_ms: First retry delay (1 second)
    # - multiplier: Delay multiplier for exponential backoff (2x)
    # - max_delay_ms: Maximum delay cap (30 seconds)
    # Retry pattern: 1s, 2s, 4s, 8s, 16s, 30s
    retry_policy = DMSCRetryPolicy(
        max_retries=3,
        initial_delay_ms=1000,
        multiplier=2.0,
        max_delay_ms=30000,
    )
    await manager.set_retry_policy("orders", retry_policy)
    print("   Retry policy configured\n")
    
    # Step 10: List all queues
    # Demonstrates queue enumeration
    print("10. Listing all queues...")
    queues = await manager.list_queues()
    print("   Available queues:")
    for queue in queues:
        stats = await manager.get_stats(queue)
        print(f"   - {queue}: {stats.message_count()} messages")
    print()
    
    # Step 11: Cleanup
    # Demonstrates queue deletion for resource cleanup
    print("11. Cleaning up (deleting test queues)...")
    
    # Delete queues with force flag to remove even with messages
    await manager.delete_queue("orders", force=True)
    await manager.delete_queue("notifications", force=True)
    print("   Test queues deleted\n")
    
    print("=== Message Queue Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
