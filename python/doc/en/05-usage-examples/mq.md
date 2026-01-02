<div align="center">

# Message Queue Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's message queue module for async messaging, publish-subscribe, routing, dead letter queues, delayed messages, persistence, priorities, and filtering.

## Example Overview

This example creates a DMSC Python application with the following features:

- RabbitMQ, Kafka, Redis Streams message queue support
- Publish-subscribe and message routing
- Dead letter queues and delayed messages
- Message persistence and priority
- Message filtering and acknowledgment
- Error handling and retry strategies

## Prerequisites

- Python 3.8+
- Understanding of message queue concepts
- (Optional) RabbitMQ, Kafka, or Redis server for real testing

## Complete Code Example

```python
import asyncio
import json
from datetime import datetime, timedelta
from typing import Dict, List, Optional, Any, Callable
from enum import Enum
from dataclasses import dataclass
from collections import deque
from hashlib import md5

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCMQModule, DMSCMQConfig, DMSCMQMessage, DMSCMQMessageType,
    DMSCMQConsumerConfig, DMSCMQProducerConfig,
    DMSCDeadLetterConfig, DMSCRetryConfig,
    DMSCConfig, DMSCError
)

# Message priority
class MessagePriority(Enum):
    LOW = 0
    NORMAL = 100
    HIGH = 200
    CRITICAL = 300

# Message status
class MessageStatus(Enum):
    PENDING = "pending"
    DELIVERED = "delivered"
    ACKNOWLEDGED = "acknowledged"
    FAILED = "failed"
    RETRYING = "retrying"

# Message data class
@dataclass
class QueueMessage:
    message_id: str
    topic: str
    payload: Dict[str, Any]
    priority: MessagePriority
    status: MessageStatus
    created_at: datetime
    delivered_at: Optional[datetime]
    acknowledged_at: Optional[datetime]
    retry_count: int
    dead_letter_reason: Optional[str]

# Message queue service
class MessageQueueService:
    def __init__(self, mq_module: DMSCMQModule, context: DMSCServiceContext):
        self.mq_module = mq_module
        self.context = context
        self.logger = context.logger
        self.topics: Dict[str, Dict] = {}
        self.subscriptions: Dict[str, List[Callable]] = {}
        self.message_history: List[QueueMessage] = []
        self.pending_messages: Dict[str, QueueMessage] = {}
        self.consumer_tasks: Dict[str, asyncio.Task] = {}
    
    async def create_topic(
        self,
        topic_name: str,
        durable: bool = True,
        message_ttl: int = 86400000,
        max_size: int = 1000000
    ) -> Dict:
        """Create a message queue topic"""
        if topic_name in self.topics:
            raise DMSCError(f"Topic already exists: {topic_name}", "TOPIC_EXISTS")
        
        topic_config = {
            "name": topic_name,
            "durable": durable,
            "message_ttl": message_ttl,
            "max_size": max_size,
            "created_at": datetime.now()
        }
        
        self.topics[topic_name] = topic_config
        self.logger.info("mq", f"Topic created: {topic_name}")
        
        return topic_config
    
    async def publish(
        self,
        topic: str,
        payload: Dict[str, Any],
        priority: MessagePriority = MessagePriority.NORMAL,
        delay_seconds: int = 0,
        correlation_id: Optional[str] = None,
        reply_to: Optional[str] = None
    ) -> QueueMessage:
        """Publish a message to a topic"""
        if topic not in self.topics:
            raise DMSCError(f"Topic not found: {topic}", "TOPIC_NOT_FOUND")
        
        message_id = md5(f"{datetime.now().timestamp()}{topic}{json.dumps(payload)}".encode()).hexdigest()
        
        message = QueueMessage(
            message_id=message_id,
            topic=topic,
            payload=payload,
            priority=priority,
            status=MessageStatus.PENDING,
            created_at=datetime.now(),
            delivered_at=None,
            acknowledged_at=None,
            retry_count=0,
            dead_letter_reason=None
        )
        
        # Create MQ message
        mq_message = DMSCMQMessage(
            body=json.dumps(payload),
            message_id=message_id,
            content_type="application/json",
            priority=priority.value,
            correlation_id=correlation_id,
            reply_to=reply_to,
            delivery_delay=delay_seconds * 1000 if delay_seconds > 0 else None
        )
        
        # Publish to message queue
        await self.mq_module.publish(topic, mq_message)
        
        # Track message
        self.message_history.append(message)
        self.pending_messages[message_id] = message
        
        self.logger.info("mq", f"Message published: {message_id} to {topic}")
        
        return message
    
    async def subscribe(
        self,
        topic: str,
        handler: Callable,
        consumer_group: Optional[str] = None,
        auto_ack: bool = False
    ) -> str:
        """Subscribe to a topic with a handler"""
        if topic not in self.topics:
            raise DMSCError(f"Topic not found: {topic}", "TOPIC_NOT_FOUND")
        
        subscription_id = f"sub_{datetime.now().timestamp()}"
        
        if topic not in self.subscriptions:
            self.subscriptions[topic] = []
        
        self.subscriptions[topic].append({
            "id": subscription_id,
            "handler": handler,
            "consumer_group": consumer_group,
            "auto_ack": auto_ack
        })
        
        self.logger.info("mq", f"Subscription created: {subscription_id} for {topic}")
        
        return subscription_id
    
    async def consume(
        self,
        subscription_id: str,
        max_messages: int = 10,
        timeout_seconds: int = 30
    ) -> List[QueueMessage]:
        """Consume messages from a subscription"""
        messages = []
        
        for topic, subscriptions in self.subscriptions.items():
            for sub in subscriptions:
                if sub["id"] == subscription_id:
                    # Create consumer
                    consumer_config = DMSCMQConsumerConfig(
                        consumer_group=sub["consumer_group"],
                        auto_ack=sub["auto_ack"],
                        prefetch_count=max_messages
                    )
                    
                    consumer = await self.mq_module.create_consumer(
                        topic,
                        sub["handler"],
                        consumer_config
                    )
                    
                    # Consume messages
                    for _ in range(max_messages):
                        msg = await asyncio.wait_for(
                            consumer.receive(),
                            timeout=timeout_seconds
                        )
                        
                        if msg:
                            message = self.pending_messages.get(msg.message_id)
                            if message:
                                message.status = MessageStatus.DELIVERED
                                message.delivered_at = datetime.now()
                                messages.append(message)
                    
                    return messages
        
        raise DMSCError(f"Subscription not found: {subscription_id}", "SUBSCRIPTION_NOT_FOUND")
    
    async def acknowledge(self, message_id: str) -> bool:
        """Acknowledge a message"""
        if message_id in self.pending_messages:
            message = self.pending_messages[message_id]
            message.status = MessageStatus.ACKNOWLEDGED
            message.acknowledged_at = datetime.now()
            del self.pending_messages[message_id]
            
            self.logger.info("mq", f"Message acknowledged: {message_id}")
            return True
        
        return False
    
    async def reject(self, message_id: str, requeue: bool = False, reason: str = None):
        """Reject a message"""
        if message_id in self.pending_messages:
            message = self.pending_messages[message_id]
            
            if requeue:
                message.status = MessageStatus.RETRYING
                message.retry_count += 1
                self.logger.info("mq", f"Message requeued for retry: {message_id}")
            else:
                message.status = MessageStatus.FAILED
                message.dead_letter_reason = reason or "Rejected"
                
                # Send to dead letter queue
                await self._send_to_dead_letter(message)
                
                del self.pending_messages[message_id]
                self.logger.info("mq", f"Message sent to DLQ: {message_id}")
    
    async def _send_to_dead_letter(self, message: QueueMessage):
        """Send failed message to dead letter queue"""
        dlq_topic = f"{message.topic}_dlq"
        
        if dlq_topic not in self.topics:
            await self.create_topic(dlq_topic, durable=True)
        
        await self.publish(
            topic=dlq_topic,
            payload={
                "original_topic": message.topic,
                "original_message_id": message.message_id,
                "payload": message.payload,
                "failed_at": datetime.now().isoformat(),
                "reason": message.dead_letter_reason
            },
            priority=MessagePriority.LOW
        )
    
    async def get_topic_stats(self, topic: str) -> Dict:
        """Get statistics for a topic"""
        if topic not in self.topics:
            raise DMSCError(f"Topic not found: {topic}", "TOPIC_NOT_FOUND")
        
        topic_messages = [m for m in self.message_history if m.topic == topic]
        
        return {
            "topic": topic,
            "total_messages": len(topic_messages),
            "pending": len([m for m in topic_messages if m.status == MessageStatus.PENDING]),
            "delivered": len([m for m in topic_messages if m.status == MessageStatus.DELIVERED]),
            "acknowledged": len([m for m in topic_messages if m.status == MessageStatus.ACKNOWLEDGED]),
            "failed": len([m for m in topic_messages if m.status == MessageStatus.FAILED])
        }
    
    async def get_all_stats(self) -> Dict:
        """Get all message queue statistics"""
        total_messages = len(self.message_history)
        topic_stats = {}
        
        for topic in self.topics:
            topic_stats[topic] = await self.get_topic_stats(topic)
        
        return {
            "total_messages": total_messages,
            "topics": topic_stats,
            "subscriptions": len(self.subscriptions),
            "pending_messages": len(self.pending_messages)
        }

# Request handlers
async def handle_create_topic(context: DMSCServiceContext):
    """Create a new topic"""
    data = await context.http.request.json()
    
    topic_name = data.get("topic_name")
    durable = data.get("durable", True)
    message_ttl = data.get("message_ttl", 86400000)
    
    if not topic_name:
        return {"status": "error", "message": "topic_name required"}, 400
    
    mq_service = context.mq_service
    topic = await mq_service.create_topic(
        topic_name=topic_name,
        durable=durable,
        message_ttl=message_ttl
    )
    
    return {"status": "success", "data": topic}

async def handle_publish(context: DMSCServiceContext):
    """Publish a message"""
    data = await context.http.request.json()
    
    topic = data.get("topic")
    payload = data.get("payload", {})
    priority_str = data.get("priority", "normal")
    delay_seconds = data.get("delay_seconds", 0)
    correlation_id = data.get("correlation_id")
    reply_to = data.get("reply_to")
    
    if not topic:
        return {"status": "error", "message": "topic required"}, 400
    
    try:
        priority = MessagePriority(priority_str)
    except ValueError:
        priority = MessagePriority.NORMAL
    
    mq_service = context.mq_service
    message = await mq_service.publish(
        topic=topic,
        payload=payload,
        priority=priority,
        delay_seconds=delay_seconds,
        correlation_id=correlation_id,
        reply_to=reply_to
    )
    
    return {
        "status": "success",
        "data": {
            "message_id": message.message_id,
            "topic": message.topic,
            "priority": message.priority.name,
            "created_at": message.created_at.isoformat()
        }
    }

async def handle_subscribe(context: DMSCServiceContext):
    """Subscribe to a topic"""
    data = await context.http.request.json()
    
    topic = data.get("topic")
    handler_type = data.get("handler_type", "echo")
    
    if not topic:
        return {"status": "error", "message": "topic required"}, 400
    
    # Select handler based on type
    async def echo_handler(message):
        print(f"Received: {message}")
        return {"received": True}
    
    handler = echo_handler
    
    mq_service = context.mq_service
    subscription_id = await mq_service.subscribe(
        topic=topic,
        handler=handler
    )
    
    return {
        "status": "success",
        "data": {
            "subscription_id": subscription_id,
            "topic": topic
        }
    }

async def handle_consume(context: DMSCServiceContext):
    """Consume messages"""
    data = await context.http.request.json()
    
    subscription_id = data.get("subscription_id")
    max_messages = data.get("max_messages", 10)
    timeout_seconds = data.get("timeout_seconds", 30)
    
    if not subscription_id:
        return {"status": "error", "message": "subscription_id required"}, 400
    
    mq_service = context.mq_service
    messages = await mq_service.consume(
        subscription_id=subscription_id,
        max_messages=max_messages,
        timeout_seconds=timeout_seconds
    )
    
    return {
        "status": "success",
        "data": {
            "count": len(messages),
            "messages": [
                {
                    "message_id": m.message_id,
                    "topic": m.topic,
                    "payload": m.payload,
                    "status": m.status.value
                }
                for m in messages
            ]
        }
    }

async def handle_acknowledge(context: DMSCServiceContext):
    """Acknowledge a message"""
    data = await context.http.request.json()
    
    message_id = data.get("message_id")
    
    if not message_id:
        return {"status": "error", "message": "message_id required"}, 400
    
    mq_service = context.mq_service
    success = await mq_service.acknowledge(message_id)
    
    if success:
        return {"status": "success", "message": "Message acknowledged"}
    else:
        return {"status": "error", "message": "Message not found"}, 404

async def handle_reject(context: DMSCServiceContext):
    """Reject a message"""
    data = await context.http.request.json()
    
    message_id = data.get("message_id")
    requeue = data.get("requeue", False)
    reason = data.get("reason")
    
    if not message_id:
        return {"status": "error", "message": "message_id required"}, 400
    
    mq_service = context.mq_service
    await mq_service.reject(message_id, requeue=requeue, reason=reason)
    
    return {"status": "success", "message": "Message rejected"}

async def handle_get_stats(context: DMSCServiceContext):
    """Get message queue statistics"""
    data = await context.http.request.json()
    topic = data.get("topic")
    
    mq_service = context.mq_service
    
    if topic:
        stats = await mq_service.get_topic_stats(topic)
    else:
        stats = await mq_service.get_all_stats()
    
    return {"status": "success", "data": stats}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    # Configure message queue
    app.with_mq(DMSCMQConfig(
        backend="redis",
        host="localhost",
        port=6379
    ))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize MQ module
    mq_config = DMSCMQConfig(
        backend="memory",
        host="localhost",
        port=6379
    )
    mq_module = DMSCMQModule(mq_config)
    
    # Initialize MQ service
    mq_service = MessageQueueService(mq_module, dms_app.context)
    dms_app.context.mq_service = mq_service
    
    # Add routes
    dms_app.router.add_route("POST", "/topics", handle_create_topic)
    dms_app.router.add_route("POST", "/publish", handle_publish)
    dms_app.router.add_route("POST", "/subscribe", handle_subscribe)
    dms_app.router.add_route("POST", "/consume", handle_consume)
    dms_app.router.add_route("POST", "/acknowledge", handle_acknowledge)
    dms_app.router.add_route("POST", "/reject", handle_reject)
    dms_app.router.add_route("POST", "/stats", handle_get_stats)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Message Queue Architecture

1. **Topics**: Create and manage message queue topics
2. **Publishing**: Send messages with priority and delay
3. **Subscriptions**: Subscribe to topics with handlers
4. **Consumption**: Consume and process messages
5. **Acknowledgments**: Confirm message processing
6. **Dead Letter Queue**: Handle failed messages

### Key Components

- **DMSCMQModule**: Main message queue interface
- **DMSCMQMessage**: Message representation
- **DMSCMQConsumerConfig**: Consumer configuration
- **DMSCRetryConfig**: Retry policy configuration
- **DMSCDeadLetterConfig**: Dead letter queue configuration

## Running Steps

1. Save the code to `mq_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc redis
   ```
3. Run the application:
   ```bash
   python mq_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Create a topic
   curl -X POST http://localhost:8080/topics \
     -H "Content-Type: application/json" \
     -d '{"topic_name": "orders", "durable": true}'
   
   # Publish a message
   curl -X POST http://localhost:8080/publish \
     -H "Content-Type: application/json" \
     -d '{"topic": "orders", "payload": {"order_id": "123", "total": 99.99}, "priority": "high"}'
   
   # Subscribe to a topic
   curl -X POST http://localhost:8080/subscribe \
     -H "Content-Type: application/json" \
     -d '{"topic": "orders", "handler_type": "echo"}'
   
   # Consume messages
   curl -X POST http://localhost:8080/consume \
     -H "Content-Type: application/json" \
     -d '{"subscription_id": "sub_xxx", "max_messages": 10}'
   
   # Acknowledge a message
   curl -X POST http://localhost:8080/acknowledge \
     -H "Content-Type: application/json" \
     -d '{"message_id": "msg_xxx"}'
   
   # Get statistics
   curl -X POST http://localhost:8080/stats \
     -H "Content-Type: application/json" \
     -d '{"topic": "orders"}'
   ```

## Expected Output

### Publish Message Response

```json
{
  "status": "success",
  "data": {
    "message_id": "abc123def456",
    "topic": "orders",
    "priority": "HIGH",
    "created_at": "2024-01-15T10:30:00"
  }
}
```

### Topic Stats Response

```json
{
  "status": "success",
  "data": {
    "topic": "orders",
    "total_messages": 150,
    "pending": 5,
    "delivered": 140,
    "acknowledged": 138,
    "failed": 2
  }
}
```

## Best Practices

1. **Use Persistent Messages**: Set delivery_mode=2 for important messages
2. **Implement Consumer Groups**: Use consumer groups for load balancing
3. **Handle Acknowledgments**: Properly handle message acknowledgments
4. **Configure Dead Letter Queues**: Handle failed messages gracefully
5. **Monitor Queue Depth**: Monitor queue depth to detect backpressure
6. **Use Compression**: Compress large messages to reduce bandwidth
7. **Implement Retries**: Configure retry policies for transient failures
8. **Use Transactions**: Use transactions for critical operations
9. **Set Message TTL**: Set appropriate message TTL to prevent memory issues
10. **Monitor Performance**: Track message throughput and latency
