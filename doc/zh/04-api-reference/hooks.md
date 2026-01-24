<div align="center">

# Hooks API参考

**Version: 0.1.6**

**Last modified date: 2026-01-24**

hooks模块提供生命周期事件钩子系统，支持在应用启动、关闭等关键时刻执行自定义逻辑。

## 模块概述

</div>

hooks模块基于事件总线模式，提供以下功能：

- **生命周期钩子**：支持应用启动、关闭等生命周期事件
- **模块阶段钩子**：支持模块初始化、启动、关闭等阶段
- **事件驱动架构**：松散耦合的组件通信机制
- **灵活的事件处理**：支持同步和异步处理

<div align="center">

## 核心组件

</div>

### DMSCHookBus

钩子事件总线，管理钩子的注册和触发。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建钩子总线 | 无 | `Self` |
| `register(kind, id, handler)` | 注册钩子处理函数 | `kind: DMSCHookKind`, `id: DMSCHookId`, `handler: F` | `()` |
| `emit(kind, ctx)` | 触发钩子事件 | `kind: &DMSCHookKind`, `ctx: &DMSCServiceContext` | `DMSCResult<()>` |
| `emit_with(kind, ctx, module, phase)` | 触发带模块信息的钩子 | `kind: &DMSCHookKind`, `ctx: &DMSCServiceContext`, `module: Option<&str>`, `phase: Option<DMSCModulePhase>` | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookHandler};

fn example() -> DMSCResult<()> {
    let mut hook_bus = DMSCHookBus::new();
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "example.startup".to_string(),
        |ctx, event| {
            println!("Application starting up!");
            println!("Hook kind: {:?}", event.kind);
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "example.shutdown".to_string(),
        |ctx, event| {
            println!("Application shutting down!");
            Ok(())
        }
    );
    
    let ctx = DMSCServiceContext::new();
    hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
    
    Ok(())
}
```

### DMSCHookKind

钩子类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Startup` | 应用启动时 |
| `Shutdown` | 应用关闭时 |
| `BeforeModulesInit` | 模块初始化之前 |
| `AfterModulesInit` | 模块初始化之后 |
| `BeforeModulesStart` | 模块启动之前 |
| `AfterModulesStart` | 模块启动之后 |
| `BeforeModulesShutdown` | 模块关闭之前 |
| `AfterModulesShutdown` | 模块关闭之后 |

### DMSCModulePhase

模块生命周期阶段枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Init` | 同步初始化阶段 |
| `BeforeStart` | 同步启动前阶段 |
| `Start` | 同步启动阶段 |
| `AfterStart` | 同步启动后阶段 |
| `BeforeShutdown` | 同步关闭前阶段 |
| `Shutdown` | 同步关闭阶段 |
| `AfterShutdown` | 同步关闭后阶段 |
| `AsyncInit` | 异步初始化阶段 |
| `AsyncBeforeStart` | 异步启动前阶段 |
| `AsyncStart` | 异步启动阶段 |
| `AsyncAfterStart` | 异步启动后阶段 |
| `AsyncBeforeShutdown` | 异步关闭前阶段 |
| `AsyncShutdown` | 异步关闭阶段 |
| `AsyncAfterShutdown` | 异步关闭后阶段 |

### DMSCHookEvent

钩子事件结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `kind` | `DMSCHookKind` | 钩子类型 |
| `module` | `Option<String>` | 关联的模块名称 |
| `phase` | `Option<DMSCModulePhase>` | 模块阶段 |

<div align="center">

## 钩子注册

</div>

### 基础钩子注册

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCHookEvent};
use dmsc::core::DMSCServiceContext;

let mut hook_bus = DMSCHookBus::new();

hook_bus._register(
    DMSCHookKind::Startup,
    "my_module.startup".to_string(),
    |ctx: &DMSCServiceContext, event: &DMSCHookEvent| {
        println!("My module is starting up");
        Ok(())
    }
);
```

### 模块生命周期钩子

```rust
hook_bus._register(
    DMSCHookKind::BeforeModulesInit,
    "my_module.before_init".to_string(),
    |ctx, event| {
        println!("Before module initialization");
        Ok(())
    }
);

hook_bus._register(
    DMSCHookKind::AfterModulesShutdown,
    "my_module.after_shutdown".to_string(),
    |ctx, event| {
        println!("After module shutdown");
        Ok(())
    }
);
```

### 带模块信息的钩子

```rust
use dmsc::hooks::{DMSCHookBus, DMSCHookKind, DMSCModulePhase};

hook_bus._emit_with(
    &DMSCHookKind::BeforeModulesInit,
    &ctx,
    Some("auth_module"),
    Some(DMSCModulePhase::Init)
)?;
```

<div align="center">

## 实际应用场景

</div>

### 应用启动初始化

```rust
use dmsc::prelude::*;
use dmsc::hooks::{DMSCHookBus, DMSCHookKind};

fn setup_startup_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::Startup,
        "db.initialize".to_string(),
        |ctx, _event| {
            println!("Initializing database connection...");
            // ctx.database().connect()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "cache.warmup".to_string(),
        |ctx, _event| {
            println!("Warming up cache...");
            // ctx.cache().warm_up()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Startup,
        "metrics.start".to_string(),
        |ctx, _event| {
            println!("Starting metrics collection...");
            // ctx.metrics().start()?;
            Ok(())
        }
    );
}
```

### 资源清理

```rust
fn setup_shutdown_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "cache.flush".to_string(),
        |ctx, _event| {
            println!("Flushing cache to persistent storage...");
            // ctx.cache().flush()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "db.disconnect".to_string(),
        |ctx, _event| {
            println!("Disconnecting from database...");
            // ctx.database().disconnect()?;
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::Shutdown,
        "metrics.stop".to_string(),
        |ctx, _event| {
            println!("Stopping metrics collection...");
            // ctx.metrics().stop()?;
            Ok(())
        }
    );
}
```

### 模块依赖管理

```rust
fn setup_dependency_hooks(hook_bus: &mut DMSCHookBus) {
    hook_bus._register(
        DMSCHookKind::BeforeModulesInit,
        "dependencies.check".to_string(),
        |ctx, event| {
            println!("Checking module dependencies...");
            Ok(())
        }
    );
    
    hook_bus._register(
        DMSCHookKind::AfterModulesInit,
        "dependencies.ready".to_string(),
        |ctx, event| {
            println!("All dependencies are ready");
            Ok(())
        }
    );
}
```

<div align="center">

## 错误处理

</div>

钩子执行过程中的错误会向上传播：

```rust
use dmsc::core::DMSCError;

hook_bus._register(
    DMSCHookKind::Startup,
    "critical.startup".to_string(),
    |ctx, event| {
        match ctx.database().connect() {
            Ok(_) => Ok(()),
            Err(e) => Err(DMSCError::Other("Database connection failed".to_string()))
        }
    }
);
```

<div align="center">

## 最佳实践

</div>

1. **保持钩子简洁**：钩子函数应该快速执行，避免长时间阻塞
2. **处理错误**：在钩子中正确处理错误，避免影响应用启动
3. **使用有意义的ID**：为钩子使用描述性的ID，便于调试
4. **避免循环依赖**：注意钩子之间的依赖关系，避免循环
5. **记录日志**：在钩子中记录关键操作，便于问题排查
6. **按需注册**：只注册必要的钩子，避免不必要的开销

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
