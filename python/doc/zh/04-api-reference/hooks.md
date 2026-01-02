<div align="center">

# Hooks API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

hooks模块提供事件总线系统，支持组件在各种生命周期事件期间进行通信。

</div>

## 模块概述

hooks模块包含以下核心组件：

- **DMSCHookBus**: 钩子事件总线，用于注册和触发钩子
- **DMSCHookEvent**: 钩子事件表示
- **DMSCHookKind**: 钩子类型枚举
- **DMSCModulePhase**: 模块生命周期阶段枚举

<div align="center">

## 核心组件

</div>

### DMSCHookKind

钩子类型枚举，定义不同类型的钩子。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `STARTUP` | 启动钩子 |
| `SHUTDOWN` | 关闭钩子 |
| `BEFORE_MODULES_INIT` | 模块初始化前 |
| `AFTER_MODULES_INIT` | 模块初始化后 |
| `BEFORE_MODULES_START` | 模块启动前 |
| `AFTER_MODULES_START` | 模块启动后 |
| `BEFORE_MODULES_SHUTDOWN` | 模块关闭前 |
| `AFTER_MODULES_SHUTDOWN` | 模块关闭后 |

### DMSCModulePhase

模块生命周期阶段枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `INIT` | 初始化阶段 |
| `BEFORE_START` | 启动前阶段 |
| `START` | 启动阶段 |
| `AFTER_START` | 启动后阶段 |
| `BEFORE_SHUTDOWN` | 关闭前阶段 |
| `SHUTDOWN` | 关闭阶段 |
| `AFTER_SHUTDOWN` | 关闭后阶段 |
| `ASYNC_INIT` | 异步初始化 |
| `ASYNC_BEFORE_START` | 异步启动前 |
| `ASYNC_START` | 异步启动 |
| `ASYNC_AFTER_START` | 异步启动后 |
| `ASYNC_BEFORE_SHUTDOWN` | 异步关闭前 |
| `ASYNC_SHUTDOWN` | 异步关闭 |
| `ASYNC_AFTER_SHUTDOWN` | 异步关闭后 |

### DMSCHookEvent

钩子事件结构，表示一个钩子事件。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `kind` | `DMSCHookKind` | 钩子类型 |
| `module` | `str` | 模块名称（可选） |
| `phase` | `DMSCModulePhase` | 模块阶段（可选） |

#### 使用示例

```python
from dmsc import DMSCHookEvent, DMSCHookKind, DMSCModulePhase

# 创建基本事件
event = DMSCHookEvent(kind=DMSCHookKind.STARTUP)

# 创建带模块信息的事件
event_with_module = DMSCHookEvent(
    kind=DMSCHookKind.BEFORE_MODULES_INIT,
    module="cache",
    phase=DMSCModulePhase.INIT
)

print(f"钩子类型: {event.kind}")
print(f"模块: {event_with_module.module}")
print(f"阶段: {event_with_module.phase}")
```

### DMSCHookBus

钩子事件总线，用于注册和触发钩子。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `register(kind, handler_id, handler)` | 注册钩子处理器 | `kind: DMSCHookKind`, `handler_id: str`, `handler: Callable` | `None` |
| `unregister(handler_id)` | 注销钩子处理器 | `handler_id: str` | `bool` |
| `emit(kind, event)` | 触发钩子事件 | `kind: DMSCHookKind`, `event: DMSCHookEvent` | `None` |
| `emit_with(kind, module, phase)` | 触发带模块信息的钩子 | `kind: DMSCHookKind`, `module: str`, `phase: DMSCModulePhase` | `None` |
| `get_registered_hooks()` | 获取已注册的钩子ID列表 | 无 | `List[str]` |

#### 使用示例

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCModulePhase

def startup_handler(event):
    print(f"启动钩子执行: {event}")

def cache_init_handler(event):
    if event.module == "cache":
        print("cache 模块正在初始化")

# 创建钩子总线
hook_bus = DMSCHookBus()

# 注册启动处理器
hook_bus._register(
    DMSCHookKind.STARTUP,
    "app.startup",
    startup_handler
)

# 注册模块特定的处理器
hook_bus._register(
    DMSCHookKind.BEFORE_MODULES_INIT,
    "cache.init",
    cache_init_handler
)

# 获取已注册的钩子
hooks = hook_bus.get_registered_hooks()
print(f"已注册的钩子: {hooks}")

# 触发钩子
hook_bus.emit(DMSCHookKind.STARTUP)

hook_bus._emit_with(
    DMSCHookKind.BEFORE_MODULES_INIT,
    module="cache",
    phase=DMSCModulePhase.INIT
)

# 注销钩子
hook_bus.unregister("app.startup")
```

<div align="center">

## 完整使用示例

</div>

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCModulePhase, DMSCHookEvent

class HookSystemExample:
    """钩子系统完整示例"""
    
    def __init__(self):
        self.hook_bus = DMSCHookBus()
        self.hooks_registered = []
    
    def setup_hooks(self):
        """设置所有钩子处理器"""
        
        # 启动钩子
        def on_startup(event):
            print("应用启动")
        
        # 关闭钩子
        def on_shutdown(event):
            print("应用关闭")
        
        # 模块初始化前钩子
        def on_before_modules_init(event):
            print(f"模块初始化前: {event.module}")
        
        # 模块启动钩子
        def on_module_start(event):
            print(f"模块 {event.module} 启动到阶段: {event.phase}")
        
        # 注册所有钩子
        self.hook_bus._register(DMSCHookKind.STARTUP, "app.startup", on_startup)
        self.hook_bus._register(DMSCHookKind.SHUTDOWN, "app.shutdown", on_shutdown)
        self.hook_bus._register(DMSCHookKind.BEFORE_MODULES_INIT, "modules.pre_init", on_before_modules_init)
        self.hook_bus._register(DMSCHookKind.AFTER_MODULES_START, "modules.post_start", on_module_start)
        
        self.hooks_registered = self.hook_bus.get_registered_hooks()
        print(f"已注册 {len(self.hooks_registered)} 个钩子")
    
    def run_lifecycle(self):
        """运行应用生命周期"""
        
        # 触发启动
        self.hook_bus.emit(DMSCHookKind.STARTUP)
        
        # 触发模块初始化前
        self.hook_bus._emit_with(
            DMSCHookKind.BEFORE_MODULES_INIT,
            module="cache",
            phase=DMSCModulePhase.INIT
        )
        
        # 触发模块启动后
        self.hook_bus._emit_with(
            DMSCHookKind.AFTER_MODULES_START,
            module="auth",
            phase=DMSCModulePhase.AFTER_START
        )
        
        # 触发关闭
        self.hook_bus.emit(DMSCHookKind.SHUTDOWN)
    
    def cleanup(self):
        """清理所有钩子"""
        for hook_id in self.hooks_registered:
            self.hook_bus.unregister(hook_id)
        self.hooks_registered.clear()

async def hooks_complete_example():
    """钩子系统完整示例"""
    
    example = HookSystemExample()
    
    # 设置钩子
    example.setup_hooks()
    
    # 运行生命周期
    example.run_lifecycle()
    
    # 清理
    example.cleanup()
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [core](./core.md): 核心模块，提供模块系统支持
- [auth](./auth.md): 认证模块，支持认证钩子
- [device](./device.md): 设备模块，支持设备生命周期钩子
- [service_mesh](./service_mesh.md): 服务网格模块，支持服务发现钩子
