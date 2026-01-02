<div align="center">

# Hooks System 使用指南

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本文档提供 DMSC Hooks 模块的使用示例。

</div>

## 基础钩子操作

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCModulePhase

def basic_hooks_example():
    """基础钩子示例"""
    hook_bus = DMSCHookBus()
    
    # 注册启动处理器
    def on_startup(event):
        print(f"启动事件: {event.kind}")
    
    hook_bus._register(DMSCHookKind.STARTUP, "app.startup", on_startup)
    
    # 注册关闭处理器
    def on_shutdown(event):
        print(f"关闭事件: {event.kind}")
    
    hook_bus._register(DMSCHookKind.SHUTDOWN, "app.shutdown", on_shutdown)
    
    # 触发启动事件
    hook_bus.emit(DMSCHookKind.STARTUP)
    
    # 触发关闭事件
    hook_bus.emit(DMSCHookKind.SHUTDOWN)
```

## 模块生命周期钩子

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCModulePhase

def lifecycle_hooks_example():
    """模块生命周期钩子示例"""
    hook_bus = DMSCHookBus()
    
    # 注册初始化前处理器
    def on_before_init(event):
        print(f"初始化前: {event.module}")
    
    hook_bus._register(
        DMSCHookKind.BEFORE_MODULES_INIT,
        "module.validate",
        on_before_init
    )
    
    # 注册初始化后处理器
    def on_after_init(event):
        print(f"初始化后: {event.module}")
    
    hook_bus._register(
        DMSCHookKind.AFTER_MODULES_INIT,
        "module.register",
        on_after_init
    )
    
    # 触发带模块信息的事件
    hook_bus._emit_with(
        DMSCHookKind.BEFORE_MODULES_INIT,
        module="cache",
        phase=DMSCModulePhase.INIT
    )
    
    hook_bus._emit_with(
        DMSCHookKind.AFTER_MODULES_INIT,
        module="cache",
        phase=DMSCModulePhase.INIT
    )
```

## 带上下文的钩子

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCHookEvent

def hook_with_context_example():
    """带事件上下文的钩子示例"""
    hook_bus = DMSCHookBus()
    
    # 注册处理事件详细信息的处理器
    def detailed_handler(event: DMSCHookEvent):
        print(f"钩子类型: {event.kind}")
        print(f"模块: {event.module}")
        print(f"阶段: {event.phase}")
        
        # 根据模块进行条件判断
        if event.module == "database":
            print("检测到数据库模块事件")
    
    hook_bus._register(
        DMSCHookKind.AFTER_MODULES_START,
        "monitoring.start",
        detailed_handler
    )
    
    # 触发带完整上下文的事件
    hook_bus.emit(DMSCHookKind.AFTER_MODULES_START)
```

## 多处理器

```python
from dmsc import DMSCHookBus, DMSCHookKind

def multiple_handlers_example():
    """同一钩子多个处理器示例"""
    hook_bus = DMSCHookBus()
    
    # 处理器 1
    def handler_1(event):
        print("处理器 1 已执行")
    
    # 处理器 2
    def handler_2(event):
        print("处理器 2 已执行")
    
    # 处理器 3
    def handler_3(event):
        print("处理器 3 已执行")
    
    # 为同一钩子注册多个处理器
    hook_bus._register(DMSCHookKind.STARTUP, "handler.1", handler_1)
    hook_bus._register(DMSCHookKind.STARTUP, "handler.2", handler_2)
    hook_bus._register(DMSCHookKind.STARTUP, "handler.3", handler_3)
    
    # 触发 - 所有处理器都将执行
    hook_bus.emit(DMSCHookKind.STARTUP)
```

## 完整示例

```python
from dmsc import DMSCHookBus, DMSCHookKind, DMSCModulePhase

class Application:
    def __init__(self):
        self.hook_bus = DMSCHookBus()
        self.is_running = False
        
        # 注册生命周期处理器
        self._register_handlers()
    
    def _register_handlers(self):
        # 启动处理器
        self.hook_bus._register(
            DMSCHookKind.STARTUP,
            "app.config",
            lambda e: print("正在加载配置")
        )
        
        self.hook_bus._register(
            DMSCHookKind.STARTUP,
            "app.init",
            lambda e: print("正在初始化应用")
        )
        
        # 模块生命周期处理器
        self.hook_bus._register(
            DMSCHookKind.BEFORE_MODULES_INIT,
            "module.validate",
            lambda e: print(f"正在验证模块: {e.module}")
        )
        
        self.hook_bus._register(
            DMSCHookKind.AFTER_MODULES_INIT,
            "module.register",
            lambda e: print(f"模块已初始化: {e.module}")
        )
        
        # 关闭处理器
        self.hook_bus._register(
            DMSCHookKind.BEFORE_MODULES_SHUTDOWN,
            "app.cleanup",
            lambda e: print("正在执行清理")
        )
        
        self.hook_bus._register(
            DMSCHookKind.SHUTDOWN,
            "app.terminate",
            lambda e: print("正在终止应用")
        )
    
    def start(self):
        print("正在启动应用...")
        self.hook_bus.emit(DMSCHookKind.STARTUP)
        
        modules = ["cache", "database", "api"]
        for module in modules:
            self.hook_bus._emit_with(
                DMSCHookKind.BEFORE_MODULES_INIT,
                module=module,
                phase=DMSCModulePhase.INIT
            )
            self.hook_bus._emit_with(
                DMSCHookKind.AFTER_MODULES_INIT,
                module=module,
                phase=DMSCModulePhase.INIT
            )
        
        self.is_running = True
        print("应用已启动")
    
    def stop(self):
        if not self.is_running:
            return
        
        print("正在停止应用...")
        self.hook_bus.emit(DMSCHookKind.BEFORE_MODULES_SHUTDOWN)
        self.hook_bus.emit(DMSCHookKind.SHUTDOWN)
        self.is_running = False
        print("应用已停止")

# 运行示例
app = Application()
app.start()
app.stop()
```
