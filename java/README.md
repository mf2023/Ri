<div align="center">

<h1 style="display: flex; flex-direction: column; align-items: center; gap: 12px; margin-bottom: 8px;">
  <span style="display: flex; align-items: center; gap: 12px;"><img src="../assets/svg/dmsc.svg" width="48" height="48" alt="DMSC">Dunimd Middleware Service</span>
  <span style="font-size: 0.6em; color: #666; font-weight: normal;">DMSC Library for Java</span>
</h1>

[English](README.md) | 简体中文

[Help Documentation](../doc/en/index.md) | [Changelog](CHANGELOG.md) | [Security](../SECURITY.md) | [Contributing](../CONTRIBUTING.md) | [Code of Conduct](../CODE_OF_CONDUCT.md)

<a href="https://space.bilibili.com/3493284091529457" target="_blank">
    <img alt="BiliBili" src="https://img.shields.io/badge/BiliBili-Dunimd-00A1D6?style=flat-square&logo=bilibili"/>
</a>
<a href="https://x.com/Dunimd2025" target="_blank">
    <img alt="X" src="https://img.shields.io/badge/X-Dunimd-000000?style=flat-square&logo=x"/>
</a>

<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://github.com/mf2023/DMSC" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMSC-181717?style=flat-square&logo=github"/>
</a>
<a href="https://huggingface.co/dunimd" target="_blank">
    <img alt="Hugging Face" src="https://img.shields.io/badge/Hugging%20Face-Dunimd-FFD21E?style=flat-square&logo=huggingface"/>
</a>
<a href="https://modelscope.cn/organization/dunimd" target="_blank">
    <img alt="ModelScope" src="https://img.shields.io/badge/ModelScope-Dunimd-1E6CFF?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMTQiIGhlaWdodD0iMTQiIHZpZXdCb3g9IjAgMCAxNCAxNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTcuMDA2IDBDMy4xNDIgMCAwIDMuMTQyIDAgNy4wMDZTMy4xNDIgMTQuMDEyIDcuMDA2IDE0LjAxMkMxMC44NyAxNC4wMTIgMTQuMDEyIDEwLjg3IDE0LjAxMiA3LjAwNkMxNC4wMTIgMy4xNDIgMTAuODcgMCA3LjAwNiAwWiIgZmlsbD0iIzFFNkNGRiIvPgo8L3N2Zz4K"/>
</a>

<a href="https://search.maven.org/artifact/com.dunimd/dmsc" target="_blank">
    <img alt="Maven Central" src="https://img.shields.io/badge/Maven-DMSC-007396?style=flat-square&logo=apachemaven"/>
</a>

**DMSC (Dunimd Middleware Service)** — A high-performance Rust middleware framework with Java bindings. Built for enterprise-scale with modular architecture, built-in observability, and distributed systems support.

</div>

<h2 align="center">🏗️ Core Architecture</h2>

### 📐 Modular Design
DMSC adopts a highly modular architecture with 18 core modules, enabling on-demand composition and seamless extension:

<div align="center">

| Module | Description | Java Support |
|:--------|:------------|:------------|
| **auth** | Authentication & authorization (JWT, OAuth, permissions) | ✅ Full |
| **cache** | Multi-backend cache abstraction (Memory, Redis, Hybrid) | ✅ Full |
| **config** | Multi-source configuration management with hot reload | ✅ Full |
| **core** | Runtime, error handling, and service context | ✅ Full |
| **database** | Database abstraction with PostgreSQL, MySQL, SQLite support | ✅ Full |
| **device** | Device control, discovery, and intelligent scheduling | ✅ Full |
| **fs** | Secure file system operations and management | ✅ Full |
| **gateway** | API gateway with load balancing, rate limiting, and circuit breaking | ✅ Full |
| **grpc** | gRPC server and client support | ✅ Full |
| **hooks** | Lifecycle event hooks (Startup, Shutdown, etc.) | ✅ Full |
| **log** | Structured logging with tracing context integration | ✅ Full |
| **module_rpc** | Inter-module RPC communication for distributed method calls | ✅ Full |
| **observability** | Metrics, tracing, and Grafana integration | ✅ Full |
| **queue** | Distributed queue abstraction (Kafka, RabbitMQ, Redis, Memory) | ✅ Full |
| **service_mesh** | Service discovery, health checking, and traffic management | ✅ Full |
| **validation** | Input validation and data sanitization utilities | ✅ Full |
| **ws** | WebSocket server support | ✅ Full |
| **protocol** | Protocol abstraction layer for multiple communication protocols | ✅ Full |

</div>

<h2 align="center">🛠️ Installation & Environment</h2>

### Prerequisites
- **Java**: JDK 8+
- **Maven** or **Gradle**
- **Platforms**: Linux, macOS, Windows

### Quick Setup

#### Maven

```xml
<dependency>
    <groupId>com.dunimd</groupId>
    <artifactId>dmsc</artifactId>
    <version>0.1.8</version>
</dependency>
```

#### Gradle

```groovy
implementation 'com.dunimd:dmsc:0.1.8'
```

<h2 align="center">⚡ Quick Start</h2>

### Core API Usage

```java
import com.dunimd.dmsc.*;

public class Main {
    public static void main(String[] args) {
        // Build service runtime
        DMSCAppRuntime runtime = new DMSCAppBuilder()
            .withConfig("config.yaml")
            .build();
        
        // Check running status
        if (runtime.isRunning()) {
            System.out.println("DMSC is running!");
        }
        
        // Shutdown application
        runtime.shutdown();
    }
}
```

### Cache Management Example

```java
import com.dunimd.dmsc.cache.*;

// Create cache config
DMSCCacheConfig config = new DMSCCacheConfig()
    .setEnabled(true)
    .setDefaultTtlSecs(3600)
    .setBackendType(DMSCCacheBackendType.Memory);

// Create cache module
DMSCCacheModule cache = new DMSCCacheModule(config);

// Set cache value
cache.set("user:123", "John Doe", 3600);

// Get cache value
String value = cache.get("user:123");

// Check if key exists
if (cache.exists("user:123")) {
    cache.delete("user:123");
}

// Get statistics
DMSCCacheStats stats = cache.getStats();
System.out.println("Hits: " + stats.getHits());
System.out.println("Hit rate: " + stats.getHitRate());
```

### Validation Example

```java
import com.dunimd.dmsc.validation.*;

// Validate email
DMSCValidationResult emailResult = DMSCValidationModule.validateEmail("user@example.com");
if (emailResult.isValid()) {
    System.out.println("Email is valid");
}

// Using validator builder
DMSCValidatorBuilder builder = new DMSCValidatorBuilder("email")
    .notEmpty()
    .maxLength(255)
    .isEmail();

DMSCValidationRunner runner = builder.build();
DMSCValidationResult result = runner.validate("user@example.com");
```

### Configuration Management Example

```java
import com.dunimd.dmsc.*;

// Create config from YAML
DMSCConfig config = DMSCConfig.fromYaml("key: value");

// Get config value
String value = config.get("key");
```

<h2 align="center">🔧 Configuration</h2>

### Configuration Example

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

<h2 align="center">🚀 Auto-Loading Mechanism</h2>

DMSC Java bindings use an auto-loading mechanism. Users don't need to manually configure native library paths:

```java
// No manual loading required, auto-loads on first use
DMSCCacheModule cache = new DMSCCacheModule(config);
// NativeLoader.autoLoad() is called automatically
```

### Supported Platforms

| Platform | Architecture |
|----------|--------------|
| Windows | x64, arm64 |
| Linux | x64, arm64 |
| macOS | x64, arm64 |
| Android | arm64-v8a, armeabi-v7a, x86_64 |

<h2 align="center">❓ Frequently Asked Questions</h2>

**Q: What Java versions are supported?**
A: JDK 8 and above are supported.

**Q: Is the Rust backend included?**
A: Yes, the package includes the compiled Rust backend with JNI bindings, embedded in the JAR file.

**Q: How to handle exceptions?**
A: Use try-catch to catch `DMSCError` exceptions.

**Q: How to configure logging level?**
A: Set `logging.level` in the configuration file, supporting DEBUG/INFO/WARN/ERROR levels.

<h2 align="center">🌏 Community & Citation</h2>

- Welcome to submit Issues and PRs!
- Gitee: https://gitee.com/dunimd/dmsc.git
- Github: https://github.com/mf2023/DMSC.git

<div align="center">

## 📄 License & Open Source Agreements

### 🏛️ Project License

<p align="center">
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="Apache License 2.0">
  </a>
</p>

This project uses **Apache License 2.0** open source agreement, see [LICENSE](LICENSE) file.

### 📋 Dependencies License

<div align="center">

| 📦 Package | 📜 License |
|:-----------|:-----------|
| jackson-databind | Apache 2.0 |
| jni-rs | Apache 2.0 |

</div>  

</div>
