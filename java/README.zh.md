<div align="center">

> ⚠️ **DMSC 已更名为 Ri**，从 v0.1.9 起生效。
> [📖 阅读迁移指南](../ANNOUNCEMENT.zh.md)

---

<img src="../assets/svg/ri.svg" width="36" height="36">

[English](README.md) | 简体中文

[帮助文档](https://mf2023.github.io/Ri/ri/) | [更新日志](CHANGELOG.md) | [安全](../SECURITY.md) | [贡献](../CONTRIBUTING.md) | [行为准则](../CODE_OF_CONDUCT.md)

<a href="https://github.com/mf2023/Ri" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-Ri-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd/ri" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Ri-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://gitcode.com/dunimd/ri.git" target="_blank">
    <img alt="GitCode" src="https://img.shields.io/badge/GitCode-Ri-FF6B35?style=flat-square&logo=git"/>
</a>

<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>
<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://huggingface.co/dunimd" target="_blank">
    <img alt="Hugging Face" src="https://img.shields.io/badge/Hugging%20Face-Dunimd-FFD21E?style=flat-square&logo=huggingface"/>
</a>
<a href="https://modelscope.cn/organization/dunimd" target="_blank">
    <img alt="ModelScope" src="https://img.shields.io/badge/ModelScope-Dunimd-1E6CFF?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTcuMDA2IDBDMy4xNDIgMCAwIDMuMTQyIDAgNy4wMDZTMy4xNDIgMTQuMDEyIDcuMDA2IDE0LjAxMkMxMC44NyAxNC4wMTIgMTQuMDEyIDEwLjg3IDE0LjAxMiA3LjAwNkMxNC4wMTIgMy4xNDIgMTAuODcgMCA3LjAwNiAwWiIgZmlsbD0iIzFFNkNGRiIvPgo8L3N2Zz4K"/>
</a>


<a href="https://search.maven.org/artifact/com.dunimd/ri" target="_blank">
    <img alt="Maven Central" src="https://img.shields.io/badge/Maven-Ri-007396?style=flat-square&logo=apachemaven"/>
</a>

**Ri (Ri)** — 一个高性能的 Rust 中间件框架，带有 Java 绑定。专为企业级规模构建，具有模块化架构、内置可观测性和分布式系统支持。

</div>

<h2 align="center">🏗️ 核心架构</h2>

### 📐 模块化设计
Ri 采用高度模块化的架构，拥有 18 个核心模块，支持按需组合和无缝扩展：

<div align="center">

| 模块 | 描述 | Java 支持 |
|:--------|:------------|:------------|
| **auth** | 认证与授权（JWT、OAuth、权限） | ✅ 完整 |
| **cache** | 多后端缓存抽象（内存、Redis、混合） | ✅ 完整 |
| **config** | 多源配置管理与热重载 | ✅ 完整 |
| **core** | 运行时、错误处理和服务上下文 | ✅ 完整 |
| **database** | 数据库抽象层，支持 PostgreSQL、MySQL、SQLite | ✅ 完整 |
| **device** | 设备控制、发现和智能调度 | ✅ 完整 |
| **fs** | 安全的文件系统操作和管理 | ✅ 完整 |
| **gateway** | API 网关，支持负载均衡、限流和熔断 | ✅ 完整 |
| **grpc** | gRPC 服务器和客户端支持 | ✅ 完整 |
| **hooks** | 生命周期事件钩子（启动、关闭等） | ✅ 完整 |
| **log** | 结构化日志与追踪上下文集成 | ✅ 完整 |
| **module_rpc** | 模块间 RPC 通信，支持分布式方法调用 | ✅ 完整 |
| **observability** | 指标、追踪和 Grafana 集成 | ✅ 完整 |
| **queue** | 分布式队列抽象（Kafka、RabbitMQ、Redis、内存） | ✅ 完整 |
| **service_mesh** | 服务发现、健康检查和流量管理 | ✅ 完整 |
| **validation** | 数据验证和清理工具 | ✅ 完整 |
| **ws** | WebSocket 服务器支持 | ✅ 完整 |
| **protocol** | 协议抽象层，支持多种通信协议 | ✅ 完整 |

</div>

<h2 align="center">🛠️ 安装与环境</h2>

### 前置要求
- **Java**: JDK 8 及以上版本
- **Maven** 或 **Gradle**
- **平台**: Linux、macOS、Windows

### 快速设置

#### Maven

```xml
<dependency>
    <groupId>com.dunimd</groupId>
    <artifactId>ri</artifactId>
    <version>0.1.9</version>
</dependency>
```

#### Gradle

```groovy
implementation 'com.dunimd:ri:0.1.9'
```

<h2 align="center">⚡ 快速开始</h2>

### 核心 API 使用

```java
import com.dunimd.ri.*;

public class Main {
    public static void main(String[] args) {
        // 构建服务运行时
        RiAppRuntime runtime = new RiAppBuilder()
            .withConfig("config.yaml")
            .build();
        
        // 检查运行状态
        if (runtime.isRunning()) {
            System.out.println("Ri is running!");
        }
        
        // 关闭应用
        runtime.shutdown();
    }
}
```

### 缓存管理示例

```java
import com.dunimd.ri.cache.*;

// 创建缓存配置
RiCacheConfig config = new RiCacheConfig()
    .setEnabled(true)
    .setDefaultTtlSecs(3600)
    .setBackendType(RiCacheBackendType.Memory);

// 创建缓存模块
RiCacheModule cache = new RiCacheModule(config);

// 设置缓存值
cache.set("user:123", "John Doe", 3600);

// 获取缓存值
String value = cache.get("user:123");

// 检查键是否存在
if (cache.exists("user:123")) {
    cache.delete("user:123");
}

// 获取统计信息
RiCacheStats stats = cache.getStats();
System.out.println("Hits: " + stats.getHits());
System.out.println("Hit rate: " + stats.getHitRate());
```

### 验证示例

```java
import com.dunimd.ri.validation.*;

// 验证邮箱
RiValidationResult emailResult = RiValidationModule.validateEmail("user@example.com");
if (emailResult.isValid()) {
    System.out.println("Email is valid");
}

// 使用验证器构建器
RiValidatorBuilder builder = new RiValidatorBuilder("email")
    .notEmpty()
    .maxLength(255)
    .isEmail();

RiValidationRunner runner = builder.build();
RiValidationResult result = runner.validate("user@example.com");
```

### 配置管理示例

```java
import com.dunimd.ri.*;

// 从 YAML 创建配置
RiConfig config = RiConfig.fromYaml("key: value");

// 获取配置值
String value = config.get("key");
```

<h2 align="center">🔧 配置</h2>

### 配置示例

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"

logging:
  level: "info"
  file_format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090
```

<h2 align="center">🚀 自动加载机制</h2>

Ri Java 绑定使用自动加载机制，用户无需手动配置原生库路径：

```java
// 无需手动加载，首次使用时自动加载
RiCacheModule cache = new RiCacheModule(config);
// NativeLoader.autoLoad() 会自动调用
```

### 支持的平台

| 平台 | 架构 |
|------|------|
| Windows | x64, arm64 |
| Linux | x64, arm64 |
| macOS | x64, arm64 |
| Android | arm64-v8a, armeabi-v7a, x86_64 |

<h2 align="center">❓ 常见问题</h2>

**Q: 支持哪些 Java 版本？**
A: 支持 JDK 8 及以上版本。

**Q: Rust 后端是否包含在内？**
A: 是的，该包包含了编译后的 Rust 后端和 JNI 绑定，嵌入在 JAR 文件中。

**Q: 如何处理异常？**
A: 使用 try-catch 捕获 `RiError` 异常。

**Q: 如何配置日志级别？**
A: 在配置文件中设置 `logging.level`，支持 DEBUG/INFO/WARN/ERROR 级别。

<h2 align="center">🌏 社区与引用</h2>

- 欢迎提交 Issues 和 PRs！
- Github: https://github.com/mf2023/Ri.git
- Gitee: https://gitee.com/dunimd/ri.git
- GitCode: https://gitcode.com/dunimd/ri.git

<div align="center">

## 📄 许可证与开源协议

### 🏛️ 项目许可证

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

本项目使用 **Apache License 2.0** 开源协议，详见 [LICENSE](LICENSE) 文件。

### 📋 依赖包许可证

<div align="center">

| 📦 包 | 📜 许可证 | 📦 包 | 📜 许可证 |
|:-----------|:-----------|:-----------|:-----------|
| jackson-databind | Apache 2.0 | jni | MIT/Apache-2.0 |

</div>  

</div>
