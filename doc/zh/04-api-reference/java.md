<div align="center">

# DMSC Java API 参考

**Version: 0.1.8**

**Last modified date: 2026-02-20**

DMSC Java 绑定提供对 DMSC 中间件服务的完整 Java API 支持。

</div>

## 快速开始

### Maven 依赖

```xml
<dependency>
    <groupId>com.dunimd</groupId>
    <artifactId>dmsc</artifactId>
    <version>0.1.8</version>
</dependency>
```

### Gradle 依赖

```groovy
implementation 'com.dunimd:dmsc:0.1.8'
```

### 基本使用

```java
import com.dunimd.dmsc.*;

public class Main {
    public static void main(String[] args) {
        // 创建应用
        DMSCAppRuntime runtime = new DMSCAppBuilder()
            .withConfig("config.yaml")
            .build();
        
        // 检查状态
        if (runtime.isRunning()) {
            System.out.println("DMSC is running!");
        }
        
        // 关闭
        runtime.shutdown();
    }
}
```

---

## 核心模块

### DMSCAppBuilder

应用构建器，用于配置和创建 DMSC 应用。

```java
DMSCAppBuilder builder = new DMSCAppBuilder();

// 从配置文件加载
DMSCAppBuilder configured = builder.withConfig("config.yaml");

// 构建运行时
DMSCAppRuntime runtime = configured.build();
```

### DMSCAppRuntime

应用运行时，代表一个运行中的 DMSC 应用。

```java
// 检查运行状态
boolean running = runtime.isRunning();

// 关闭应用
runtime.shutdown();
```

### DMSCConfig

配置管理类。

```java
// 从 YAML 创建配置
DMSCConfig config = DMSCConfig.fromYaml("key: value");

// 获取配置值
String value = config.get("key");
```

### DMSCError

DMSC 错误异常类。

```java
try {
    // DMSC 操作
} catch (DMSCError e) {
    System.out.println("Error: " + e.getMessage());
    System.out.println("Code: " + e.getErrorCode());
}
```

---

## 缓存模块

### DMSCCacheModule

缓存模块，提供多后端缓存支持。

```java
import com.dunimd.dmsc.cache.*;

// 创建配置
DMSCCacheConfig config = new DMSCCacheConfig()
    .setEnabled(true)
    .setDefaultTtlSecs(3600)
    .setBackendType(DMSCCacheBackendType.Memory);

// 创建缓存模块
DMSCCacheModule cache = new DMSCCacheModule(config);

// 设置值
cache.set("key", "value", 3600);

// 获取值
String value = cache.get("key");

// 检查存在
boolean exists = cache.exists("key");

// 删除值
cache.delete("key");

// 获取统计
DMSCCacheStats stats = cache.getStats();
System.out.println("Hits: " + stats.getHits());
System.out.println("Hit rate: " + stats.getHitRate());
```

### DMSCCacheBackendType

缓存后端类型枚举。

| 值 | 说明 |
|---|------|
| `Memory` | 内存缓存 |
| `Redis` | Redis 缓存 |
| `Hybrid` | 混合缓存（内存 + Redis） |

---

## 验证模块

### DMSCValidationModule

验证模块，提供数据验证功能。

```java
import com.dunimd.dmsc.validation.*;

// 验证邮箱
DMSCValidationResult emailResult = DMSCValidationModule.validateEmail("user@example.com");
if (emailResult.isValid()) {
    System.out.println("Email is valid");
}

// 验证用户名
DMSCValidationResult userResult = DMSCValidationModule.validateUsername("john_doe");

// 验证密码
DMSCValidationResult passResult = DMSCValidationModule.validatePassword("Secure123!");

// 验证 URL
DMSCValidationResult urlResult = DMSCValidationModule.validateUrl("https://example.com");

// 验证 IP
DMSCValidationResult ipResult = DMSCValidationModule.validateIp("192.168.1.1");
```

### DMSCValidatorBuilder

验证器构建器，用于构建复杂验证规则。

```java
DMSCValidatorBuilder builder = new DMSCValidatorBuilder("email")
    .notEmpty()
    .maxLength(255)
    .isEmail();

DMSCValidationRunner runner = builder.build();
DMSCValidationResult result = runner.validate("user@example.com");
```

---

## 自动加载机制

DMSC Java 绑定使用自动加载机制，用户无需手动配置原生库路径。

```java
// 无需手动加载，首次使用时自动加载
DMSCCacheModule cache = new DMSCCacheModule(config);
// NativeLoader.autoLoad() 会自动调用
```

### 支持的平台

| 平台 | 架构 |
|------|------|
| Windows | x64, x86 |
| Linux | x64, arm64 |
| macOS | x64, arm64 |

---

## 最佳实践

### 资源管理

使用 try-with-resources 确保资源正确释放：

```java
try (DMSCCacheModule cache = new DMSCCacheModule(config)) {
    cache.set("key", "value", 3600);
    // 自动释放资源
}
```

### 错误处理

始终捕获 DMSCError 异常：

```java
try {
    DMSCAppRuntime runtime = new DMSCAppBuilder()
        .withConfig("config.yaml")
        .build();
} catch (DMSCError e) {
    logger.error("Failed to start DMSC: {}", e.getMessage());
}
```

---

## 相关模块

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
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
