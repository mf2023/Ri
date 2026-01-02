<div align="center">

# Message Queue API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The mq module provides unified message queue functionality, supporting multiple message queue backends.

## Module Overview

</div>

The mq module contains the following sub-modules:

- **connections**: Message queue connection management
- **producers**: Message producers
- **consumers**: Message consumers
- **exchanges**: Message exchanges
- **queues**: Message queue management
- **routing**: Message routing
- **retry**: Retry mechanism
- **deadletter**: Dead letter queue handling
- **metrics**: Performance monitoring
- **tracing**: Distributed tracing

<div align="center">

## Core Components

</div>

### DMSCMQConfig

Message queue configuration class, used to configure message queue behavior.

#### Constructor

```python
DMSCMQConfig(
    backend: str = "redis",
    host: str = "localhost",
    port: int = 6379,
    username: str = "",
    password: str = "",
    database: int = 0,
    virtual_host: str = "/",
    connection_pool_size: int = 10,
    connection_timeout: int = 30,
)
```

#### Properties

| Property | Type | Description | Default |
|:---------|:-----|:------------|:--------|
| `backend` | `str` | Message queue backend (redis, kafka, rabbitmq) | `"redis"` |
| `host` | `str` | Server hostname | `"localhost"` |
| `port` | `int` | Server port | `6379` |
| `username` | `str` | Authentication username | `""` |
| `password` | `str` | Authentication password | `""` |
| `database` | `int` | Database number | `0` |
| `virtual_host` | `str` | Virtual host (RabbitMQ) | `"/"` |
| `connection_pool_size` | `int` | Connection pool size | `10` |
| `connection_timeout` | `int` | Connection timeout in seconds | `30` |

### DMSCMQModule

Message queue module main interface.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `connect()` | Establish connection | None | `None` |
| `disconnect()` | Close connection | None | `None` |
| `is_connected()` | Check connection status | None | `bool` |
| `create_producer(queue, **opts)` | Create message producer | `queue: str`, `**opts` | `DMSCProducer` |
| `create_consumer(queue, handler, **opts)` | Create message consumer | `queue: str`, `handler: Callable`, `**opts` | `DSMCConsumer` |
| `publish(queue, message, **opts)` | Publish message | `queue: str`, `message: Any`, `**opts` | `str` |
| `consume(queue, handler, **opts)` | Consume messages | `queue: str`, `handler: Callable`, `**opts` | `None` |

#### Usage Example

```python
from dmsc import DMSCMQModule, DMSCMQConfig

# Initialize message queue module
config = DMSCMQConfig(
    backend="redis",
    host="localhost",
    port=6379,
    password="your-password"
)
mq_module = DMSCMQModule(config)

# Connect to message queue
await mq_module.connect()

# Publish message
message_id = await mq_module.publish(
    "notifications",
    {"type": "welcome", "user_id": 123}
)
print(f"Message published: {message_id}")
```

## Message Producers

### Creating a Producer

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Create producer
producer = await mq_module.create_producer(
    queue="orders",
    exchange="order_exchange",
    routing_key="order.created"
)

# Send messages
for order_id in range(100):
    await producer.send(
        {"order_id": order_id, "status": "pending"},
        properties={"delivery_mode": 2}  # Persistent
    )
```

### Producer Configuration

```python
producer = await mq_module.create_producer(
    queue="orders",
    batch_size=100,
    batch_timeout_ms=1000,
    compression="gzip",
    priority=True,
    max_retries=3,
    retry_delay_ms=1000
)
```

## Message Consumers

### Creating a Consumer

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

async def handle_message(message):
    print(f"Received: {message}")
    # Process message
    return {"status": "processed"}

# Create consumer
consumer = await mq_module.create_consumer(
    queue="notifications",
    handler=handle_message,
    prefetch_count=10,
    auto_ack=False,
    dead_letter_queue="notifications_dlq"
)

# Start consuming
await consumer.start()

# Stop consuming
await consumer.stop()
```

### Consumer Groups

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Create consumer with group
consumer = await mq_module.create_consumer(
    queue="orders",
    handler=handle_order,
    consumer_group="order_processors",
    consumer_id="worker_1"
)
```

## Message Routing

### Exchange Types

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Direct exchange - route by exact key
await mq_module.declare_exchange(
    "notifications",
    type="direct",
    routing_keys=["email", "sms", "push"]
)

# Topic exchange - route by pattern
await mq_module.declare_exchange(
    "events",
    type="topic",
    routing_keys=["user.*", "order.#", "system.*"]
)

# Fanout exchange - broadcast to all queues
await mq_module.declare_exchange(
    "broadcasts",
    type="fanout"
)

# Headers exchange - route by headers
await mq_module.declare_exchange(
    "headers_exchange",
    type="headers"
)
```

### Queue Binding

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Bind queue to exchange
await mq_module.bind_queue(
    queue="user_emails",
    exchange="notifications",
    routing_key="email"
)

# Bind with multiple routing keys
await mq_module.bind_queue(
    queue="all_notifications",
    exchange="notifications",
    routing_key=["email", "sms", "push"]
)
```

## Retry Mechanism

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Configure retry policy
retry_config = {
    "max_retries": 3,
    "retry_delay_ms": 1000,
    "max_delay_ms": 60000,
    "exponential_base": 2,
    "retry_on": [ConnectionError, TimeoutError],
    "retry_off": [ValueError, KeyError]
}

# Create producer with retry
producer = await mq_module.create_producer(
    queue="critical",
    retry_config=retry_config
)
```

## Dead Letter Queue

```python
from dmsc import DMSCMQModule, DMSCMQConfig

mq_module = DMSCMQModule(DMSCMQConfig())

# Create dead letter queue
await mq_module.create_queue(
    queue="orders_dlq",
    dead_letter_exchange="dlx",
    dead_letter_routing_key="failed_orders"
)

# Configure main queue with DLQ
await mq_module.create_queue(
    queue="orders",
    dead_letter_queue="orders_dlq",
    dead_letter_exchange="dlx",
    dead_letter_routing_key="failed_orders",
    message_ttl=86400000  # 24 hours
)
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
