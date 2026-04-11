# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri Queue Module Example

This example demonstrates how to use the Ri queue module for distributed
message queue operations with support for multiple backends.
"""

import asyncio
from ri import (
    RiQueueModule,
    RiQueueConfig,
    RiQueueManager,
    RiQueueMessage,
    RiQueueStats,
    RiQueueBackendType,
    RiRetryPolicy,
    RiDeadLetterConfig,
)


async def main():
    # Create queue configuration
    config = RiQueueConfig()
    config.backend_type = RiQueueBackendType.MEMORY
    config.max_queue_size = 10000
    config.default_ttl_seconds = 3600
    config.enable_dead_letter = True
    config.enable_priority = True

    # Create retry policy
    retry_policy = RiRetryPolicy()
    retry_policy.max_retries = 3
    retry_policy.retry_delay_seconds = 5.0
    retry_policy.exponential_backoff = True
    config.retry_policy = retry_policy

    # Create dead letter configuration
    dlq_config = RiDeadLetterConfig()
    dlq_config.max_retries = 5
    dlq_config.ttl_seconds = 86400
    config.dead_letter_config = dlq_config

    # Initialize queue module
    queue_module = RiQueueModule(config)

    # Create queue manager
    manager = RiQueueManager()

    # Create queues for different purposes
    orders = manager.create_queue("orders")
    notifications = manager.create_queue("notifications")
    events = manager.create_queue("events")

    # Create messages with metadata
    order_msg = RiQueueMessage()
    order_msg.payload = b'{"order_id": "12345", "amount": 99.99}'
    order_msg.priority = 10
    order_msg.headers = {"content-type": "application/json"}

    notification_msg = RiQueueMessage()
    notification_msg.payload = b'{"type": "email", "to": "user@example.com"}'
    notification_msg.priority = 5

    event_msg = RiQueueMessage()
    event_msg.payload = b'{"event": "user_login", "user_id": "user123"}'
    event_msg.priority = 1

    # Publish messages to queues
    print("Publishing messages to queues...")
    orders.publish(order_msg)
    notifications.publish(notification_msg)
    events.publish(event_msg)

    # Consume messages with different strategies
    print("\nConsuming messages...")

    # Consume with timeout
    consumed_order = orders.consume(timeout_seconds=5.0)
    if consumed_order:
        print(f"Consumed order: {consumed_order.payload}")

    # Consume with auto-acknowledge
    consumed_notification = notifications.consume(auto_ack=True)
    if consumed_notification:
        print(f"Consumed notification: {consumed_notification.payload}")

    # Peek at message without removing
    peeked_event = events.peek()
    if peeked_event:
        print(f"Peeked event: {peeked_event.payload}")

    # Get queue statistics
    print("\nQueue Statistics:")
    orders_stats = manager.get_queue_stats("orders")
    print(f"Orders queue - Size: {orders_stats.message_count}, Consumers: {orders_stats.consumer_count}")

    notifications_stats = manager.get_queue_stats("notifications")
    print(f"Notifications queue - Size: {notifications_stats.message_count}")

    events_stats = manager.get_queue_stats("events")
    print(f"Events queue - Size: {events_stats.message_count}")

    # List all queues
    print("\nAll queues:")
    all_queues = manager.list_queues()
    for queue_name in all_queues:
        print(f"  - {queue_name}")

    # Delete a queue
    print("\nDeleting events queue...")
    manager.delete_queue("events")

    print("\nQueue operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
