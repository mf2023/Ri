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

- Python 3.8+
- pip 20.0+
- 基本的Python编程知识
- 了解消息队列基本概念
- （可选）RabbitMQ、Kafka或Redis服务器

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-mq-example
cd dms-mq-example
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
```

### 2. 安装依赖

```bash
pip install dmsc aio-pika aiokafka redis asyncio-mqtt
```

### 3. 创建主应用文件

创建 `main.py` 文件：

```python
import asyncio
import json
import logging
from datetime import datetime, timedelta
from typing import Dict, Any, Optional
from dataclasses import dataclass
from enum import Enum

from dms import DMSContext, DMSApplication
from dms.mq import MessageQueue, Message, QueueConfig, ConsumerConfig


# 配置日志
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class MessageType(Enum):
    """消息类型枚举"""
    ORDER_CREATED = "order.created"
    PAYMENT_COMPLETED = "payment.completed"
    INVENTORY_UPDATED = "inventory.updated"
    USER_REGISTERED = "user.registered"
    EMAIL_SENT = "email.sent"


@dataclass
class OrderEvent:
    """订单事件数据"""
    order_id: str
    user_id: str
    amount: float
    status: str
    timestamp: datetime


@dataclass
class PaymentEvent:
    """支付事件数据"""
    payment_id: str
    order_id: str
    amount: float
    method: str
    status: str
    timestamp: datetime


class MQExample:
    """消息队列示例类"""
    
    def __init__(self):
        self.app = DMSApplication()
        self.logger = logger
        
    async def initialize(self):
        """初始化应用"""
        await self.app.initialize()
        
    async def cleanup(self):
        """清理资源"""
        await self.app.cleanup()
        
    async def basic_queue_demo(self):
        """基础队列操作示例"""
        self.logger.info("=== 基础队列操作示例 ===")
        
        # 获取消息队列管理器
        mq = self.app.mq
        
        # 创建队列配置
        queue_config = QueueConfig(
            name="basic_queue",
            durable=True,
            auto_delete=False,
            max_length=1000,
            message_ttl=3600000  # 1小时
        )
        
        # 声明队列
        await mq.declare_queue(queue_config)
        self.logger.info("queue_declared", queue_name="basic_queue")
        
        # 发送消息
        for i in range(5):
            message = Message(
                body=json.dumps({
                    "id": f"msg_{i}",
                    "content": f"Basic message {i}",
                    "timestamp": datetime.now().isoformat()
                }),
                message_id=f"basic_msg_{i}",
                content_type="application/json",
                priority=5,
                delivery_mode=2  # 持久化
            )
            
            await mq.publish("basic_queue", message)
            self.logger.info("message_published", message_id=message.message_id)
            
            await asyncio.sleep(0.1)
        
        # 消费消息
        consumer_config = ConsumerConfig(
            queue_name="basic_queue",
            prefetch_count=2,
            auto_ack=False,
            exclusive=False
        )
        
        async def message_handler(message: Message):
            """消息处理函数"""
            try:
                data = json.loads(message.body)
                self.logger.info("message_received", 
                               message_id=message.message_id,
                               data=data)
                
                # 模拟处理时间
                await asyncio.sleep(0.5)
                
                # 确认消息
                await message.ack()
                self.logger.info("message_acknowledged", message_id=message.message_id)
                
            except Exception as e:
                self.logger.error("message_processing_error", 
                                message_id=message.message_id,
                                error=str(e))
                # 拒绝消息并重新入队
                await message.nack(requeue=True)
        
        # 启动消费者
        consumer = await mq.consume(consumer_config, message_handler)
        self.logger.info("consumer_started", queue_name="basic_queue")
        
        # 等待消费完成
        await asyncio.sleep(3)
        
        # 停止消费者
        await consumer.stop()
        self.logger.info("consumer_stopped")
        
        return True
        
    async def pub_sub_demo(self):
        """发布订阅示例"""
        self.logger.info("=== 发布订阅示例 ===")
        
        mq = self.app.mq
        
        # 创建主题交换机
        exchange_config = {
            "name": "events_exchange",
            "type": "topic",
            "durable": True,
            "auto_delete": False
        }
        
        await mq.declare_exchange(exchange_config)
        self.logger.info("exchange_declared", exchange_name="events_exchange")
        
        # 创建队列并绑定到交换机
        queues = [
            {"name": "order_events", "routing_key": "order.*"},
            {"name": "payment_events", "routing_key": "payment.*"},
            {"name": "all_events", "routing_key": "*.*"}
        ]
        
        for queue_info in queues:
            queue_config = QueueConfig(
                name=queue_info["name"],
                durable=True,
                auto_delete=False
            )
            await mq.declare_queue(queue_config)
            
            # 绑定队列到交换机
            await mq.bind_queue(
                queue_name=queue_info["name"],
                exchange_name="events_exchange",
                routing_key=queue_info["routing_key"]
            )
            
            self.logger.info("queue_bound", 
                           queue_name=queue_info["name"],
                           routing_key=queue_info["routing_key"])
        
        # 发布不同类型的事件
        events = [
            {
                "type": MessageType.ORDER_CREATED,
                "routing_key": "order.created",
                "data": OrderEvent(
                    order_id="ORD001",
                    user_id="USER123",
                    amount=99.99,
                    status="created",
                    timestamp=datetime.now()
                ).__dict__
            },
            {
                "type": MessageType.PAYMENT_COMPLETED,
                "routing_key": "payment.completed",
                "data": PaymentEvent(
                    payment_id="PAY001",
                    order_id="ORD001",
                    amount=99.99,
                    method="credit_card",
                    status="completed",
                    timestamp=datetime.now()
                ).__dict__
            },
            {
                "type": MessageType.USER_REGISTERED,
                "routing_key": "user.registered",
                "data": {
                    "user_id": "USER456",
                    "email": "newuser@example.com",
                    "timestamp": datetime.now().isoformat()
                }
            }
        ]
        
        for event in events:
            message = Message(
                body=json.dumps(event["data"]),
                message_id=f"event_{event['type'].value}_{datetime.now().timestamp()}",
                content_type="application/json",
                timestamp=datetime.now(),
                headers={
                    "event_type": event["type"].value,
                    "version": "1.0"
                }
            )
            
            await mq.publish(
                exchange_name="events_exchange",
                routing_key=event["routing_key"],
                message=message
            )
            
            self.logger.info("event_published", 
                           event_type=event["type"].value,
                           routing_key=event["routing_key"])
        
        # 启动消费者监听不同队列
        consumers = []
        
        async def order_handler(message: Message):
            """订单事件处理器"""
            data = json.loads(message.body)
            self.logger.info("order_event_received", 
                           order_id=data.get("order_id"),
                           amount=data.get("amount"))
            await message.ack()
        
        async def payment_handler(message: Message):
            """支付事件处理器"""
            data = json.loads(message.body)
            self.logger.info("payment_event_received", 
                           payment_id=data.get("payment_id"),
                           amount=data.get("amount"))
            await message.ack()
        
        async def all_events_handler(message: Message):
            """所有事件处理器"""
            data = json.loads(message.body)
            headers = message.headers or {}
            self.logger.info("all_event_received", 
                           event_type=headers.get("event_type", "unknown"),
                           message_id=message.message_id)
            await message.ack()
        
        # 启动消费者
        order_consumer = await mq.consume(
            ConsumerConfig(queue_name="order_events", auto_ack=False),
            order_handler
        )
        consumers.append(order_consumer)
        
        payment_consumer = await mq.consume(
            ConsumerConfig(queue_name="payment_events", auto_ack=False),
            payment_handler
        )
        consumers.append(payment_consumer)
        
        all_consumer = await mq.consume(
            ConsumerConfig(queue_name="all_events", auto_ack=False),
            all_events_handler
        )
        consumers.append(all_consumer)
        
        # 等待事件处理
        await asyncio.sleep(2)
        
        # 停止所有消费者
        for consumer in consumers:
            await consumer.stop()
        
        self.logger.info("pub_sub_demo_completed")
        return True
        
    async def dead_letter_queue_demo(self):
        """死信队列示例"""
        self.logger.info("=== 死信队列示例 ===")
        
        mq = self.app.mq
        
        # 创建主队列配置（带死信队列）
        main_queue_config = QueueConfig(
            name="main_queue",
            durable=True,
            auto_delete=False,
            max_length=100,
            message_ttl=30000,  # 30秒过期
            dead_letter_exchange="dlx_exchange",
            dead_letter_routing_key="dlx.main"
        )
        
        # 创建死信队列
        dlq_config = QueueConfig(
            name="dead_letter_queue",
            durable=True,
            auto_delete=False
        )
        
        # 声明队列
        await mq.declare_queue(main_queue_config)
        await mq.declare_queue(dlq_config)
        
        # 创建死信交换机
        dlx_config = {
            "name": "dlx_exchange",
            "type": "direct",
            "durable": True
        }
        await mq.declare_exchange(dlx_config)
        
        # 绑定死信队列到死信交换机
        await mq.bind_queue(
            queue_name="dead_letter_queue",
            exchange_name="dlx_exchange",
            routing_key="dlx.main"
        )
        
        self.logger.info("dead_letter_queue_setup_complete")
        
        # 发送测试消息
        for i in range(3):
            message = Message(
                body=json.dumps({
                    "id": f"dlq_test_{i}",
                    "content": f"Message that might fail {i}",
                    "timestamp": datetime.now().isoformat()
                }),
                message_id=f"dlq_msg_{i}",
                content_type="application/json"
            )
            
            await mq.publish("main_queue", message)
            self.logger.info("message_sent_to_main_queue", message_id=message.message_id)
        
        # 模拟处理失败的消息
        async def failing_handler(message: Message):
            """模拟失败的消息处理器"""
            try:
                data = json.loads(message.body)
                message_id = data.get("id", "")
                
                # 模拟某些消息处理失败
                if "fail" in message_id:
                    raise Exception("Simulated processing failure")
                
                self.logger.info("message_processed_successfully", message_id=message_id)
                await message.ack()
                
            except Exception as e:
                self.logger.error("message_processing_failed", 
                                message_id=message.message_id,
                                error=str(e))
                # 拒绝消息，不重新入队（会发送到死信队列）
                await message.nack(requeue=False)
        
        # 启动消费者
        consumer = await mq.consume(
            ConsumerConfig(queue_name="main_queue", auto_ack=False),
            failing_handler
        )
        
        # 监听死信队列
        async def dlq_handler(message: Message):
            """死信队列处理器"""
            data = json.loads(message.body)
            self.logger.warning("dead_letter_message_received", 
                              original_message_id=data.get("id"),
                              reason="processing_failed")
            await message.ack()
        
        dlq_consumer = await mq.consume(
            ConsumerConfig(queue_name="dead_letter_queue", auto_ack=False),
            dlq_handler
        )
        
        # 等待处理
        await asyncio.sleep(2)
        
        # 停止消费者
        await consumer.stop()
        await dlq_consumer.stop()
        
        self.logger.info("dead_letter_queue_demo_completed")
        return True
        
    async def delayed_message_demo(self):
        """延迟消息示例"""
        self.logger.info("=== 延迟消息示例 ===")
        
        mq = self.app.mq
        
        # 创建延迟队列（使用插件或TTL+死信实现）
        delayed_queue_config = QueueConfig(
            name="delayed_queue",
            durable=True,
            auto_delete=False,
            message_ttl=10000,  # 10秒延迟
            dead_letter_exchange="delayed_dlx",
            dead_letter_routing_key="delayed.process"
        )
        
        # 创建实际处理队列
        process_queue_config = QueueConfig(
            name="process_queue",
            durable=True,
            auto_delete=False
        )
        
        await mq.declare_queue(delayed_queue_config)
        await mq.declare_queue(process_queue_config)
        
        # 设置延迟交换机
        delayed_dlx_config = {
            "name": "delayed_dlx",
            "type": "direct",
            "durable": True
        }
        await mq.declare_exchange(delayed_dlx_config)
        
        # 绑定处理队列
        await mq.bind_queue(
            queue_name="process_queue",
            exchange_name="delayed_dlx",
            routing_key="delayed.process"
        )
        
        # 发送延迟消息
        delayed_message = Message(
            body=json.dumps({
                "task": "send_reminder_email",
                "email": "user@example.com",
                "delay_seconds": 10,
                "sent_at": datetime.now().isoformat()
            }),
            message_id="delayed_msg_1",
            content_type="application/json"
        )
        
        # 发送到延迟队列
        await mq.publish("delayed_queue", delayed_message)
        self.logger.info("delayed_message_sent", delay_seconds=10)
        
        # 记录当前时间
        send_time = datetime.now()
        
        # 监听处理队列
        async def process_handler(message: Message):
            """延迟消息处理器"""
            data = json.loads(message.body)
            receive_time = datetime.now()
            actual_delay = (receive_time - send_time).total_seconds()
            
            self.logger.info("delayed_message_received", 
                           task=data.get("task"),
                           expected_delay=10,
                           actual_delay=round(actual_delay, 2))
            await message.ack()
        
        # 启动消费者
        consumer = await mq.consume(
            ConsumerConfig(queue_name="process_queue", auto_ack=False),
            process_handler
        )
        
        self.logger.info("waiting_for_delayed_message...")
        
        # 等待消息处理（最多15秒）
        await asyncio.sleep(15)
        
        await consumer.stop()
        self.logger.info("delayed_message_demo_completed")
        return True
        
    async def priority_queue_demo(self):
        """优先级队列示例"""
        self.logger.info("=== 优先级队列示例 ===")
        
        mq = self.app.mq
        
        # 创建优先级队列
        priority_queue_config = QueueConfig(
            name="priority_queue",
            durable=True,
            auto_delete=False,
            max_priority=10  # 最大优先级
        )
        
        await mq.declare_queue(priority_queue_config)
        
        # 发送不同优先级的消息
        messages = [
            {"content": "Low priority message", "priority": 1},
            {"content": "High priority message", "priority": 9},
            {"content": "Medium priority message", "priority": 5},
            {"content": "Critical priority message", "priority": 10},
            {"content": "Normal priority message", "priority": 3}
        ]
        
        for msg_data in messages:
            message = Message(
                body=json.dumps({
                    "content": msg_data["content"],
                    "priority": msg_data["priority"],
                    "timestamp": datetime.now().isoformat()
                }),
                message_id=f"priority_msg_{msg_data['priority']}",
                content_type="application/json",
                priority=msg_data["priority"]
            )
            
            await mq.publish("priority_queue", message)
            self.logger.info("priority_message_sent", 
                           content=msg_data["content"],
                           priority=msg_data["priority"])
        
        # 消费消息（应该按优先级顺序）
        received_messages = []
        
        async def priority_handler(message: Message):
            """优先级消息处理器"""
            data = json.loads(message.body)
            received_messages.append(data)
            
            self.logger.info("priority_message_received", 
                           content=data.get("content"),
                           priority=data.get("priority"))
            await message.ack()
        
        # 启动消费者
        consumer = await mq.consume(
            ConsumerConfig(queue_name="priority_queue", auto_ack=False, prefetch_count=1),
            priority_handler
        )
        
        # 等待消费
        await asyncio.sleep(3)
        
        await consumer.stop()
        
        # 验证优先级顺序
        priorities = [msg.get("priority", 0) for msg in received_messages]
        self.logger.info("priority_order_verified", priorities=priorities)
        
        return True
        
    async def message_filtering_demo(self):
        """消息过滤示例"""
        self.logger.info("=== 消息过滤示例 ===")
        
        mq = self.app.mq
        
        # 创建Headers交换机
        headers_exchange_config = {
            "name": "headers_exchange",
            "type": "headers",
            "durable": True
        }
        await mq.declare_exchange(headers_exchange_config)
        
        # 创建不同过滤条件的队列
        queues = [
            {
                "name": "important_orders",
                "headers": {"x-match": "all", "priority": "high", "type": "order"}
            },
            {
                "name": "all_orders",
                "headers": {"x-match": "any", "type": "order"}
            },
            {
                "name": "high_value",
                "headers": {"x-match": "all", "amount": ">100"}
            }
        ]
        
        for queue_info in queues:
            queue_config = QueueConfig(
                name=queue_info["name"],
                durable=True,
                auto_delete=False
            )
            await mq.declare_queue(queue_config)
            
            # 绑定队列到交换机（带headers）
            await mq.bind_queue(
                queue_name=queue_info["name"],
                exchange_name="headers_exchange",
                arguments=queue_info["headers"]
            )
            
            self.logger.info("queue_bound_with_headers", 
                           queue_name=queue_info["name"],
                           headers=queue_info["headers"])
        
        # 发送带headers的消息
        test_messages = [
            {
                "headers": {"priority": "high", "type": "order", "amount": "150"},
                "content": "Important high-value order"
            },
            {
                "headers": {"priority": "low", "type": "order", "amount": "50"},
                "content": "Regular order"
            },
            {
                "headers": {"priority": "high", "type": "payment", "amount": "200"},
                "content": "High priority payment"
            }
        ]
        
        for msg_data in test_messages:
            message = Message(
                body=json.dumps({
                    "content": msg_data["content"],
                    "timestamp": datetime.now().isoformat()
                }),
                message_id=f"filtered_msg_{datetime.now().timestamp()}",
                content_type="application/json",
                headers=msg_data["headers"]
            )
            
            await mq.publish(
                exchange_name="headers_exchange",
                routing_key="",  # headers交换机忽略routing_key
                message=message
            )
            
            self.logger.info("filtered_message_sent", 
                           headers=msg_data["headers"],
                           content=msg_data["content"])
        
        # 监听不同队列
        received_counts = {}
        
        async def create_handler(queue_name: str):
            """创建队列特定的处理器"""
            async def handler(message: Message):
                data = json.loads(message.body)
                received_counts[queue_name] = received_counts.get(queue_name, 0) + 1
                
                self.logger.info(f"filtered_message_received_{queue_name}", 
                               content=data.get("content"),
                               count=received_counts[queue_name])
                await message.ack()
            return handler
        
        # 启动消费者
        consumers = []
        for queue_info in queues:
            handler = await create_handler(queue_info["name"])
            consumer = await mq.consume(
                ConsumerConfig(queue_name=queue_info["name"], auto_ack=False),
                handler
            )
            consumers.append(consumer)
        
        # 等待处理
        await asyncio.sleep(2)
        
        # 停止消费者
        for consumer in consumers:
            await consumer.stop()
        
        self.logger.info("message_filtering_demo_completed", counts=received_counts)
        return True


async def main():
    """主函数"""
    example = MQExample()
    
    try:
        # 初始化
        await example.initialize()
        
        # 运行基础队列示例
        await example.basic_queue_demo()
        
        # 运行发布订阅示例
        await example.pub_sub_demo()
        
        # 运行死信队列示例
        await example.dead_letter_queue_demo()
        
        # 运行延迟消息示例
        await example.delayed_message_demo()
        
        # 运行优先级队列示例
        await example.priority_queue_demo()
        
        # 运行消息过滤示例
        await example.message_filtering_demo()
        
        logger.info("All MQ examples completed successfully!")
        
    except Exception as e:
        logger.error("Example failed", error=str(e))
        raise
    
    finally:
        # 清理资源
        await example.cleanup()


if __name__ == "__main__":
    # 运行示例
    asyncio.run(main())
```

### 4. 运行示例

```bash
python main.py
```

预期输出：

```
INFO:__main__:=== 基础队列操作示例 ===
INFO:__main__:queue_declared: basic_queue
INFO:__main__:message_published: basic_msg_0
INFO:__main__:message_published: basic_msg_1
...
INFO:__main__:=== 发布订阅示例 ===
INFO:__main__:exchange_declared: events_exchange
...
INFO:__main__:All MQ examples completed successfully!
```

<div align="center">

## 核心概念

</div>

### 消息队列管理器

`MessageQueue` 是DMSC中负责消息队列操作的核心组件，提供以下功能：

- **队列管理**: 声明、删除、配置队列
- **交换机管理**: 声明、绑定交换机
- **消息发布**: 发送消息到队列或交换机
- **消息消费**: 消费队列中的消息
- **连接管理**: 管理消息队列连接

### 队列类型

1. **普通队列**: 基础的消息队列
2. **优先级队列**: 支持消息优先级
3. **延迟队列**: 支持延迟消息投递
4. **死信队列**: 处理失败的消息

### 交换机类型

1. **Direct**: 基于路由键精确匹配
2. **Topic**: 基于路由键模式匹配
3. **Fanout**: 广播到所有绑定的队列
4. **Headers**: 基于消息头匹配

### 消息确认机制

- **自动确认**: 消息投递后自动确认
- **手动确认**: 应用处理完后手动确认
- **拒绝重试**: 拒绝消息并重新入队
- **拒绝丢弃**: 拒绝消息不重新入队

<div align="center">

## 高级特性

</div>

### 消息持久化

```python
# 持久化消息
message = Message(
    body="Persistent message",
    delivery_mode=2,  # 持久化
    persistent=True
)

# 持久化队列
queue_config = QueueConfig(
    name="persistent_queue",
    durable=True,  # 队列持久化
    auto_delete=False
)
```

### 消息TTL

```python
# 消息级别TTL
message = Message(
    body="Message with TTL",
    expiration="60000"  # 60秒过期
)

# 队列级别TTL
queue_config = QueueConfig(
    name="ttl_queue",
    message_ttl=300000  # 5分钟TTL
)
```

### 流量控制

```python
# 消费者预取
consumer_config = ConsumerConfig(
    queue_name="controlled_queue",
    prefetch_count=10,  # 预取10条消息
    qos_global=True
)
```

### 集群和高可用

```python
# 集群配置
cluster_config = {
    "nodes": [
        {"host": "mq1.example.com", "port": 5672},
        {"host": "mq2.example.com", "port": 5672},
        {"host": "mq3.example.com", "port": 5672}
    ],
    "load_balancing": "round_robin",
    "failover_timeout": 30
}

await mq.connect_cluster(cluster_config)
```

<div align="center">

## 错误处理

</div>

### 连接错误

```python
try:
    await mq.connect(connection_config)
except ConnectionError as e:
    logger.error("MQ connection failed", error=str(e))
    # 实现重试逻辑
    await retry_connection()
```

### 消息处理错误

```python
async def robust_handler(message: Message):
    try:
        # 处理消息
        await process_message(message)
        await message.ack()
        
    except ValidationError as e:
        logger.error("Message validation failed", error=str(e))
        # 无效消息，发送到死信队列
        await message.nack(requeue=False)
        
    except ProcessingError as e:
        logger.error("Message processing failed", error=str(e))
        # 处理错误，重新入队重试
        await message.nack(requeue=True)
        
    except Exception as e:
        logger.error("Unexpected error", error=str(e))
        # 未知错误，记录并发送到死信队列
        await log_failed_message(message)
        await message.nack(requeue=False)
```

### 超时处理

```python
# 消息处理超时
async def timeout_handler(message: Message):
    try:
        # 设置处理超时
        await asyncio.wait_for(
            process_message(message),
            timeout=30.0  # 30秒超时
        )
        await message.ack()
        
    except asyncio.TimeoutError:
        logger.error("Message processing timeout")
        await message.nack(requeue=True)
```

<div align="center">

## 性能优化

</div>

### 批量操作

```python
# 批量发布消息
messages = []
for i in range(100):
    message = Message(
        body=f"Batch message {i}",
        message_id=f"batch_{i}"
    )
    messages.append(message)

await mq.publish_batch("batch_queue", messages)
```

### 异步处理

```python
# 并发消费者
async def concurrent_consumption():
    tasks = []
    for i in range(5):  # 5个并发消费者
        task = asyncio.create_task(
            mq.consume(consumer_config, message_handler)
        )
        tasks.append(task)
    
    # 等待所有消费者完成
    await asyncio.gather(*tasks)
```

### 连接池

```python
# 连接池配置
pool_config = {
    "min_connections": 2,
    "max_connections": 10,
    "connection_idle_timeout": 300,
    "health_check_interval": 60
}

await mq.setup_connection_pool(pool_config)
```

<div align="center">

## 监控和运维

</div>

### 队列监控

```python
# 获取队列状态
async def monitor_queues():
    queue_stats = await mq.get_queue_stats("main_queue")
    
    logger.info("queue_metrics", 
               messages_ready=queue_stats.messages_ready,
               messages_unacknowledged=queue_stats.messages_unacknowledged,
               consumers=queue_stats.consumers,
               memory_usage=queue_stats.memory)
```

### 性能指标

```python
# 收集性能指标
async def collect_metrics():
    metrics = await mq.get_metrics()
    
    # 发布速率
    publish_rate = metrics.get("publish_rate", 0)
    
    # 消费速率
    consume_rate = metrics.get("consume_rate", 0)
    
    # 错误率
    error_rate = metrics.get("error_rate", 0)
    
    logger.info("mq_performance_metrics",
               publish_rate=publish_rate,
               consume_rate=consume_rate,
               error_rate=error_rate)
```

### 健康检查

```python
# 健康检查
async def health_check():
    try:
        # 检查连接
        await mq.ping()
        
        # 检查队列状态
        stats = await mq.get_queue_stats("health_check_queue")
        
        if stats.messages_ready > 1000:
            logger.warning("Queue backlog detected", 
                         queue_size=stats.messages_ready)
        
        return {"status": "healthy", "queue_size": stats.messages_ready}
        
    except Exception as e:
        logger.error("Health check failed", error=str(e))
        return {"status": "unhealthy", "error": str(e)}
```

<div align="center">

## 最佳实践

</div>

### 1. 消息设计

- 保持消息小而简单
- 使用标准格式（JSON、Protobuf）
- 包含版本信息和时间戳
- 添加消息ID用于追踪

### 2. 错误处理

- 实现重试机制
- 使用死信队列处理失败消息
- 记录详细的错误日志
- 设置消息TTL防止无限重试

### 3. 性能优化

- 使用批量操作减少网络开销
- 合理设置预取数量
- 实现连接池管理
- 监控队列长度和性能指标

### 4. 可靠性

- 使用持久化消息和队列
- 实现消息确认机制
- 设置集群和高可用
- 定期备份队列数据

### 5. 安全性

- 使用TLS加密连接
- 实现身份验证和授权
- 验证消息内容
- 限制队列访问权限

<div align="center">

## 总结

</div>

本示例展示了DMSC消息队列模块的核心功能，包括：

- 基础队列操作和消息发布订阅
- 高级特性如死信队列、延迟消息、优先级队列
- 消息过滤和路由机制
- 错误处理和性能优化策略
- 监控和运维最佳实践

通过这些示例，您可以构建可靠、高性能的消息队列应用，处理复杂的异步通信场景。

更多详细信息请参考 [消息队列API文档](../04-api-reference/mq.md)。