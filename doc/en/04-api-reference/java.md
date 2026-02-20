<div align="center">

# DMSC Java API Reference

**Version: 0.1.8**

**Last modified date: 2026-02-20**

DMSC Java bindings provide complete Java API support for DMSC middleware services.

</div>

## Quick Start

### Maven Dependency

```xml
<dependency>
    <groupId>com.dunimd</groupId>
    <artifactId>dmsc</artifactId>
    <version>0.1.8</version>
</dependency>
```

### Gradle Dependency

```groovy
implementation 'com.dunimd:dmsc:0.1.8'
```

### Basic Usage

```java
import com.dunimd.dmsc.*;

public class Main {
    public static void main(String[] args) {
        // Create application
        DMSCAppRuntime runtime = new DMSCAppBuilder()
            .withConfig("config.yaml")
            .build();
        
        // Check status
        if (runtime.isRunning()) {
            System.out.println("DMSC is running!");
        }
        
        // Shutdown
        runtime.shutdown();
    }
}
```

---

## Core Module

### DMSCAppBuilder

Application builder for configuring and creating DMSC applications.

```java
DMSCAppBuilder builder = new DMSCAppBuilder();

// Load from config file
DMSCAppBuilder configured = builder.withConfig("config.yaml");

// Build runtime
DMSCAppRuntime runtime = configured.build();
```

### DMSCAppRuntime

Application runtime representing a running DMSC application.

```java
// Check running status
boolean running = runtime.isRunning();

// Shutdown application
runtime.shutdown();
```

### DMSCConfig

Configuration management class.

```java
// Create config from YAML
DMSCConfig config = DMSCConfig.fromYaml("key: value");

// Get config value
String value = config.get("key");
```

### DMSCError

DMSC error exception class.

```java
try {
    // DMSC operations
} catch (DMSCError e) {
    System.out.println("Error: " + e.getMessage());
    System.out.println("Code: " + e.getErrorCode());
}
```

---

## Cache Module

### DMSCCacheModule

Cache module with multi-backend support.

```java
import com.dunimd.dmsc.cache.*;

// Create config
DMSCCacheConfig config = new DMSCCacheConfig()
    .setEnabled(true)
    .setDefaultTtlSecs(3600)
    .setBackendType(DMSCCacheBackendType.Memory);

// Create cache module
DMSCCacheModule cache = new DMSCCacheModule(config);

// Set value
cache.set("key", "value", 3600);

// Get value
String value = cache.get("key");

// Check existence
boolean exists = cache.exists("key");

// Delete value
cache.delete("key");

// Get statistics
DMSCCacheStats stats = cache.getStats();
System.out.println("Hits: " + stats.getHits());
System.out.println("Hit rate: " + stats.getHitRate());
```

### DMSCCacheBackendType

Cache backend type enumeration.

| Value | Description |
|-------|-------------|
| `Memory` | In-memory cache |
| `Redis` | Redis cache |
| `Hybrid` | Hybrid cache (Memory + Redis) |

---

## Validation Module

### DMSCValidationModule

Validation module for data validation.

```java
import com.dunimd.dmsc.validation.*;

// Validate email
DMSCValidationResult emailResult = DMSCValidationModule.validateEmail("user@example.com");
if (emailResult.isValid()) {
    System.out.println("Email is valid");
}

// Validate username
DMSCValidationResult userResult = DMSCValidationModule.validateUsername("john_doe");

// Validate password
DMSCValidationResult passResult = DMSCValidationModule.validatePassword("Secure123!");

// Validate URL
DMSCValidationResult urlResult = DMSCValidationModule.validateUrl("https://example.com");

// Validate IP
DMSCValidationResult ipResult = DMSCValidationModule.validateIp("192.168.1.1");
```

### DMSCValidatorBuilder

Validator builder for constructing complex validation rules.

```java
DMSCValidatorBuilder builder = new DMSCValidatorBuilder("email")
    .notEmpty()
    .maxLength(255)
    .isEmail();

DMSCValidationRunner runner = builder.build();
DMSCValidationResult result = runner.validate("user@example.com");
```

---

## Auto-Loading Mechanism

DMSC Java bindings use an auto-loading mechanism. Users don't need to manually configure native library paths.

```java
// No manual loading required, auto-loads on first use
DMSCCacheModule cache = new DMSCCacheModule(config);
// NativeLoader.autoLoad() is called automatically
```

### Supported Platforms

| Platform | Architecture |
|----------|--------------|
| Windows | x64, x86 |
| Linux | x64, arm64 |
| macOS | x64, arm64 |

---

## Best Practices

### Resource Management

Use try-with-resources to ensure proper resource cleanup:

```java
try (DMSCCacheModule cache = new DMSCCacheModule(config)) {
    cache.set("key", "value", 3600);
    // Resources automatically released
}
```

### Error Handling

Always catch DMSCError exceptions:

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

## Related Modules

- [README](./README.md) - Module overview
- [core](./core.md) - Core module
- [cache](./cache.md) - Cache module
- [auth](./auth.md) - Auth module
- [gateway](./gateway.md) - Gateway module
