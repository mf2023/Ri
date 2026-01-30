<div align="center">

# Hooks System 使用指南

**Version: 0.1.6**

**Last modified date: 2026-01-30**

本文档提供 DMSC Hooks System 的完整使用示例，演示如何利用事件驱动架构进行模块生命周期管理。

## 目录

</div>

1. [基础钩子总线操作](#基础钩子总线操作)
2. [注册钩子处理器](#注册钩子处理器)
3. [触发钩子事件](#触发钩子事件)
4. [生命周期事件处理](#生命周期事件处理)
5. [高级用法](#高级用法)
6. [完整示例](#完整示例)

---

## 基础钩子总线操作

钩子总线作为 DMSC 的中央事件管理系统，允许组件为特定的生命周期事件注册处理器，并在这些事件发生时触发事件。

### 创建钩子总线

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};

// 创建新的钩子总线实例
let hook_bus = DMSCHookBus::new();
```

钩子总线创建时为空，没有注册任何处理器。您可以立即开始为各种钩子类型注册处理器。

### 钩子类型参考

DMSC 支持以下应用程序生命周期管理的钩子类型：

| 钩子类型 | 描述 |
|-----------|-------------|
| `Startup` | 应用程序启动时触发 |
| `Shutdown` | 应用程序关闭时触发 |
| `BeforeModulesInit` | 模块初始化前触发 |
| `AfterModulesInit` | 模块初始化后触发 |
| `BeforeModulesStart` | 模块启动前触发 |
| `AfterModulesStart` | 模块启动后触发 |
| `BeforeModulesShutdown` | 模块关闭前触发 |
| `AfterModulesShutdown` | 模块关闭后触发 |

---

## 注册钩子处理器

钩子处理器是在特定钩子事件触发时执行的闭包。每个处理器必须符合 `DMSCHookHandler` 类型签名。

### 简单处理器注册

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_basic_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册启动处理器
    hook_bus._register(
        DMSCHookKind::Startup,
        "logger.startup".to_string(),
        |_ctx, _event| {
            println!("检测到应用程序启动");
            Ok(())
        },
    );
    
    // 注册关闭处理器
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "logger.shutdown".to_string(),
        |_ctx, _event| {
            println!("检测到应用程序关闭");
            Ok(())
        },
    );
    
    Ok(())
}
```

### 带上下文信息的处理器

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookEvent};
use dmsc::prelude::*;

fn register_contextual_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册记录事件详细信息的处理器
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "logger.module_init".to_string(),
        |ctx, event| {
            println!("模块已初始化: {:?}", event.module);
            println!("服务上下文: {:?}", ctx);
            Ok(())
        },
    );
    
    Ok(())
}
```

### 模块特定处理器

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_module_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 为特定模块注册处理器
    hook_bus._register(
        DMSCHookKind::BeforeModulesStart,
        "cache.warmup".to_string(),
        |_ctx, event| {
            if let Some(module_name) = &event.module {
                if module_name == "cache" {
                    println!("正在预热缓存模块");
                    // 执行缓存预热操作
                }
            }
            Ok(())
        },
    );
    
    Ok(())
}
```

---

## 触发钩子事件

钩子总线提供了两种触发事件的方法：`emit()` 用于简单事件，`emit_with()` 用于带有额外上下文的事件。

### 基本事件触发

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn emit_basic_event() -> DMSCResult<()> {
    let hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new();
    
    // 触发启动事件
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    // 触发关闭事件
    hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
    
    Ok(())
}
```

### 带上下文的事件触发

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase};
use dmsc::prelude::*;

fn emit_contextual_event() -> DMSCResult<()> {
    let hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new();
    
    // 触发带有模块和阶段信息的事件
    hook_bus._emit_with(
        &DMSCHookKind::BeforeModulesStart,
        &ctx,
        Some("database"),
        Some(DMSCModulePhase::BeforeStart),
    )?;
    
    Ok(())
}
```

### 模块阶段参考

DMSC 支持同步和异步两种模块阶段：

**同步阶段：**
- `Init` - 同步初始化阶段
- `BeforeStart` - 启动前阶段
- `Start` - 启动阶段
- `AfterStart` - 启动后阶段
- `BeforeShutdown` - 关闭前阶段
- `Shutdown` - 关闭阶段
- `AfterShutdown` - 关闭后阶段

**异步阶段：**
- `AsyncInit` - 异步初始化阶段
- `AsyncBeforeStart` - 异步启动前阶段
- `AsyncStart` - 异步启动阶段
- `AsyncAfterStart` - 异步启动后阶段
- `AsyncBeforeShutdown` - 异步关闭前阶段
- `AsyncShutdown` - 异步关闭阶段
- `AsyncAfterShutdown` - 异步关闭后阶段

---

## 生命周期事件处理

本节演示全面的生命周期事件处理模式。

### 应用程序启动序列

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

async fn handle_startup_sequence() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册启动处理器
    hook_bus._register(
        DMSCHookKind::Startup,
        "config.load".to_string(),
        |_ctx, _event| {
            println!("正在加载配置");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::BeforeModulesInit,
        "resource.allocate".to_string(),
        |_ctx, _event| {
            println!("正在分配系统资源");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "service.register".to_string(),
        |_ctx, _event| {
            println!("正在注册服务");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesStart,
        "health.check".to_string(),
        |_ctx, _event| {
            println!("正在执行初始健康检查");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // 按顺序触发生命周期事件
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    hook_bus.emit(&DMSCHookKind::BeforeModulesInit, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesInit, &ctx)?;
    hook_bus.emit(&DMSCHookKind::BeforeModulesStart, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesStart, &ctx)?;
    
    Ok(())
}
```

### 应用程序关闭序列

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

async fn handle_shutdown_sequence() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册关闭处理器
    hook_bus._register(
        DMSCHookKind::BeforeModulesShutdown,
        "cache.flush".to_string(),
        |_ctx, _event| {
            println!("正在将缓存刷新到磁盘");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::BeforeModulesShutdown,
        "connection.close".to_string(),
        |_ctx, _event| {
            println!("正在关闭活动连接");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesShutdown,
        "resource.release".to_string(),
        |_ctx, _event| {
            println!("正在释放系统资源");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "logger.close".to_string(),
        |_ctx, _event| {
            println!("正在关闭日志记录器");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // 按顺序触发关闭事件
    hook_bus.emit(&DMSCHookKind::BeforeModulesShutdown, &ctx)?;
    hook_bus.emit(&DMSCHookKind::AfterModulesShutdown, &ctx)?;
    hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
    
    Ok(())
}
```

---

## 高级用法

### 钩子中的错误处理

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_error_handling_hooks() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册可以返回错误的处理器
    hook_bus._register(
        DMSCHookKind::Startup,
        "critical.startup".to_string(),
        |_ctx, _event| {
            // 模拟可能的失败
            let startup_success = true;
            if !startup_success {
                return Err(DMSCError::HookError(
                    "关键启动失败".to_string()
                ));
            }
            Ok(())
        },
    );
    
    // 注册错误恢复处理器
    hook_bus._register(
        DMSCHookKind::Startup,
        "error.recovery".to_string(),
        |ctx, event| {
            // 检查前一个处理器是否失败
            println!("恢复处理器已执行: {:?}", event.kind);
            Ok(())
        },
    );
    
    Ok(())
}
```

### 条件处理器执行

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_conditional_handler() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 注册只在特定条件下执行的处理器
    hook_bus._register(
        DMSCHookKind::AfterModulesStart,
        "monitoring.enable".to_string(),
        |_ctx, event| {
            // 只为特定模块启用监控
            if let Some(module) = &event.module {
                if module == "api-server" || module == "websocket-server" {
                    println!("正在为 {} 启用监控", module);
                }
            }
            Ok(())
        },
    );
    
    Ok(())
}
```

### 同一钩子的多个处理器

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};
use dmsc::prelude::*;

fn register_multiple_handlers() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    // 同一钩子类型的多个处理器按注册顺序执行
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.1".to_string(),
        |_ctx, _event| {
            println!("处理器 1 已执行");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.2".to_string(),
        |_ctx, _event| {
            println!("处理器 2 已执行");
            Ok(())
        },
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "handler.3".to_string(),
        |_ctx, _event| {
            println!("处理器 3 已执行");
            Ok(())
        },
    );
    
    let ctx = DMSCServiceContext::new();
    
    // 所有三个处理器都会被执行
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    Ok(())
}
```

---

## 完整示例

以下示例演示了钩子系统的完整集成：

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase, DMSCHookEvent};
use dmsc::prelude::*;

struct Application {
    hook_bus: DMSCHookBus,
    is_running: bool,
}

impl Application {
    fn new() -> Self {
        let mut hook_bus = DMSCHookBus::new();
        
        // 初始化应用程序生命周期处理器
        Self::register_lifecycle_handlers(&mut hook_bus);
        Self::register_module_handlers(&mut hook_bus);
        Self::register_monitoring_handlers(&mut hook_bus);
        
        Self {
            hook_bus,
            is_running: false,
        }
    }
    
    fn register_lifecycle_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::Startup,
            "app.config".to_string(),
            |_ctx, _event| {
                println!("[启动] 正在加载应用程序配置");
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::Shutdown,
            "app.cleanup".to_string(),
            |_ctx, _event| {
                println!("[关闭] 正在执行最终清理");
                Ok(())
            },
        );
    }
    
    fn register_module_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::BeforeModulesInit,
            "module.validate".to_string(),
            |_ctx, event| {
                if let Some(module) = &event.module {
                    println!("[模块] 正在验证模块: {}", module);
                }
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::AfterModulesInit,
            "module.register".to_string(),
            |_ctx, event| {
                if let Some(module) = &event.module {
                    println!("[模块] 模块已注册: {}", module);
                }
                Ok(())
            },
        );
    }
    
    fn register_monitoring_handlers(hook_bus: &mut DMSCHookBus) {
        hook_bus._register(
            DMSCHookKind::AfterModulesStart,
            "monitor.start".to_string(),
            |_ctx, _event| {
                println!("[监控] 正在启动性能监控");
                Ok(())
            },
        );
        
        hook_bus._register(
            DMSCHookKind::BeforeModulesShutdown,
            "monitor.stop".to_string(),
            |_ctx, _event| {
                println!("[监控] 正在停止性能监控");
                Ok(())
            },
        );
    }
    
    async fn start(&mut self) -> DMSCResult<()> {
        let ctx = DMSCServiceContext::new();
        
        println!("正在启动应用程序生命周期");
        
        self.hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
        
        let modules = vec!["config", "cache", "database", "api"];
        for module in &modules {
            self.hook_bus._emit_with(
                &DMSCHookKind::BeforeModulesInit,
                &ctx,
                Some(module),
                Some(DMSCModulePhase::Init),
            )?;
            
            self.hook_bus._emit_with(
                &DMSCHookKind::AfterModulesInit,
                &ctx,
                Some(module),
                Some(DMSCModulePhase::AfterStart),
            )?;
        }
        
        self.hook_bus.emit(&DMSCHookKind::AfterModulesStart, &ctx)?;
        
        self.is_running = true;
        println!("应用程序启动成功");
        
        Ok(())
    }
    
    async fn stop(&mut self) -> DMSCResult<()> {
        if !self.is_running {
            return Ok(());
        }
        
        let ctx = DMSCServiceContext::new();
        
        println!("正在停止应用程序生命周期");
        
        self.hook_bus.emit(&DMSCHookKind::BeforeModulesShutdown, &ctx)?;
        self.hook_bus.emit(&DMSCHookKind::AfterModulesShutdown, &ctx)?;
        self.hook_bus.emit(&DMSCHookKind::Shutdown, &ctx)?;
        
        self.is_running = false;
        println!("应用程序停止成功");
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let mut app = Application::new();
    
    app.start().await?;
    
    // 模拟应用程序运行
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    
    app.stop().await?;
    
    Ok(())
}
```

### 预期输出

```
正在启动应用程序生命周期
[启动] 正在加载应用程序配置
[模块] 正在验证模块: config
[模块] 模块已注册: config
[模块] 正在验证模块: cache
[模块] 模块已注册: cache
[模块] 正在验证模块: database
[模块] 模块已注册: database
[模块] 正在验证模块: api
[模块] 模块已注册: api
[监控] 正在启动性能监控
应用程序启动成功
正在停止应用程序生命周期
[监控] 正在停止性能监控
[关闭] 正在执行最终清理
应用程序停止成功
```

<div align="center">

## 相关模块

</div>

- [README](./README.md)：使用示例总览，提供快速导航
- [authentication](./authentication.md)：认证示例，包括JWT、OAuth2和多因素认证
- [basic-app](./basic-app.md)：基础应用示例
- [caching](./caching.md)：缓存示例，包括内存缓存和分布式缓存
- [database](./database.md)：数据库操作示例
- [device](./device.md)：设备控制示例
- [fs](./fs.md)：文件系统操作示例
- [gateway](./gateway.md)：API网关示例
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [http](./http.md)：HTTP服务器和客户端示例
- [mq](./mq.md)：消息队列示例
- [observability](./observability.md)：可观测性示例
- [protocol](./protocol.md)：协议模块示例
- [security](./security.md)：安全和加密示例
- [service_mesh](./service_mesh.md)：服务网格示例
- [storage](./storage.md)：云存储示例
- [validation](./validation.md)：数据验证示例
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信
