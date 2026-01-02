<div align="center">

# Protocol Layer 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC Protocol 模块的使用示例。

</div>

## 基础协议操作

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

async def basic_protocol_example():
    """基础协议操作示例"""
    manager = DMSCProtocolManager()
    
    # 使用默认配置初始化
    await manager.initialize(DMSCProtocolConfig.default())
    
    # 使用全局协议发送消息
    response = await manager.send_message("device-001", b"Hello DMSC Protocol")
    print(f"响应: {response}")
    
    # 关闭
    await manager.shutdown()
```

## 协议类型

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

async def protocol_types_example():
    """协议类型示例"""
    manager = DMSCProtocolManager()
    
    # 初始化
    config = DMSCProtocolConfig(
        default_protocol=DMSCProtocolType.GLOBAL,
        enable_security=True,
        enable_state_sync=True
    )
    await manager.initialize(config)
    
    # 全局协议 - 标准通信
    global_response = await manager.send_message(
        "monitor-001",
        b"Get system status"
    )
    print(f"全局协议响应: {global_response}")
    
    # 切换到私有协议进行敏感操作
    await manager.switch_protocol(DMSCProtocolType.PRIVATE)
    
    # 私有协议 - 增强安全性
    private_response = await manager.send_message(
        "secure-gateway",
        b"Execute critical operation"
    )
    print(f"私有协议响应: {private_response}")
    
    # 检查当前协议
    current = await manager.get_current_protocol()
    print(f"当前协议: {current.name}")
    
    await manager.shutdown()
```

## 消息发送

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

async def message_sending_example():
    """消息发送示例"""
    manager = DMSCProtocolManager()
    await manager.initialize(DMSCProtocolConfig.default())
    
    # 发送文本消息
    text_response = await manager.send_message(
        "text-device",
        b"Hello, World!"
    )
    print(f"文本响应: {text_response}")
    
    # 发送二进制数据
    binary_data = bytes([0x00, 0x01, 0x02, 0x03, 0xff, 0xfe])
    binary_response = await manager.send_message(
        "binary-device",
        binary_data
    )
    print(f"二进制响应长度: {len(binary_response)}")
    
    # 发送 JSON 数据
    import json
    json_data = json.dumps({"command": "get_status"}).encode()
    json_response = await manager.send_message(
        "json-device",
        json_data
    )
    print(f"JSON 响应: {json_response}")
    
    # 使用特定协议发送
    specific_response = await manager.send_message_with_protocol(
        "device-001",
        b"Message",
        DMSCProtocolType.GLOBAL
    )
    print(f"特定协议响应: {specific_response}")
    
    await manager.shutdown()
```

## 协议统计

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig

async def statistics_example():
    """协议统计示例"""
    manager = DMSCProtocolManager()
    await manager.initialize(DMSCProtocolConfig.default())
    
    # 发送一些消息
    for i in range(10):
        await manager.send_message(f"device-{i:03d}", f"Message {i}".encode())
    
    # 获取统计信息
    stats = await manager.get_stats()
    
    print("=== 协议统计 ===")
    print(f"发送消息数: {stats.total_messages_sent}")
    print(f"接收消息数: {stats.total_messages_received}")
    print(f"发送字节数: {stats.total_bytes_sent}")
    print(f"接收字节数: {stats.total_bytes_received}")
    print(f"平均延迟: {stats.average_latency_ms}ms")
    print(f"错误数: {stats.error_count}")
    print(f"成功率: {stats.success_rate * 100:.2f}%")
    
    await manager.shutdown()
```

## 完整示例

```python
from dmsc import DMSCProtocolManager, DMSCProtocolConfig, DMSCProtocolType

class ProtocolApplication:
    def __init__(self):
        self.manager = DMSCProtocolManager()
    
    async def initialize(self):
        """初始化协议管理器"""
        config = DMSCProtocolConfig(
            default_protocol=DMSCProtocolType.GLOBAL,
            enable_security=True,
            enable_state_sync=True,
            performance_optimization=True,
            connection_timeout=30,
            max_connections_per_protocol=1000,
            protocol_switching_enabled=True
        )
        await self.manager.initialize(config)
        print("协议管理器已初始化")
    
    async def send_command(self, device: str, command: str, 
                          use_private: bool = False) -> bytes:
        """向设备发送命令
        
        Args:
            device: 目标设备
            command: 要发送的命令
            use_private: 是否使用私有协议
            
        Returns:
            响应数据
        """
        if use_private:
            await self.manager.switch_protocol(DMSCProtocolType.PRIVATE)
        else:
            await self.manager.switch_protocol(DMSCProtocolType.GLOBAL)
        
        response = await self.manager.send_message(device, command.encode())
        return response
    
    async def get_statistics(self) -> dict:
        """获取协议统计信息
        
        Returns:
            统计字典
        """
        stats = await self.manager.get_stats()
        return {
            "messages_sent": stats.total_messages_sent,
            "messages_received": stats.total_messages_received,
            "bytes_sent": stats.total_bytes_sent,
            "bytes_received": stats.total_bytes_received,
            "average_latency_ms": stats.average_latency_ms,
            "error_count": stats.error_count,
            "success_rate": stats.success_rate
        }
    
    async def shutdown(self):
        """关闭协议管理器"""
        await self.manager.shutdown()
        print("协议管理器已关闭")

async def main():
    """主函数"""
    app = ProtocolApplication()
    
    # 初始化
    await app.initialize()
    
    # 发送命令
    print("\n发送全局协议消息...")
    response1 = await app.send_command(
        "monitor-001",
        "Get system status",
        use_private=False
    )
    print(f"响应: {response1}")
    
    print("\n发送私有协议消息...")
    response2 = await app.send_command(
        "secure-gateway",
        "Execute critical operation",
        use_private=True
    )
    print(f"响应: {response2}")
    
    # 获取统计信息
    stats = await app.get_statistics()
    print(f"\n=== 最终统计 ===")
    print(f"总消息数: {stats['messages_sent']}")
    print(f"成功率: {stats['success_rate'] * 100:.2f}%")
    
    # 关闭
    await app.shutdown()

import asyncio
asyncio.run(main())
```

### 预期输出

```
协议管理器已初始化

发送全局协议消息...
响应: b'System status: OK'

发送私有协议消息...
响应: b'Operation completed successfully'

=== 最终统计 ===
总消息数: 2
成功率: 100.00%
协议管理器已关闭
```
