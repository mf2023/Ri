<div align="center">

# C/C++ API参考

**版本: 0.1.7**

**最后修改日期: 2026-02-11**

C/C++ API模块为DMSC框架提供全面的C语言绑定，使C和C++应用程序能够利用DMSC的功能。

## 模块概述

</div>

C API模块按功能组织为子模块，每个子模块为特定的DMSC功能提供绑定：

- **core**: 应用初始化、配置管理和生命周期控制
- **auth**: 认证和授权服务，包括JWT令牌处理
- **cache**: 内存缓存，支持可配置的驱逐策略
- **database**: 数据库连接池和查询执行支持
- **device**: 设备抽象层，用于管理计算资源
- **fs**: 跨平台文件系统操作
- **gateway**: HTTP API网关，支持请求路由和中间件链
- **grpc**: gRPC服务器和客户端支持，用于RPC通信
- **hooks**: 事件钩子系统，通过回调注册实现可扩展性
- **log**: 结构化日志基础设施，支持多个输出目标
- **module_rpc**: DMSC框架内的模块间RPC通信
- **observability**: 指标收集、追踪集成和健康检查端点
- **protocol**: 各种线格式和序列化方案的协议处理
- **queue**: 异步任务处理的消息队列操作
- **service_mesh**: 分布式系统的服务网格集成
- **validation**: 数据验证，支持模式定义和自定义验证规则
- **ws**: WebSocket协议支持，用于全双工通信

<div align="center">

## 全局函数

</div>

### 初始化和清理

| 函数 | 描述 | 参数 | 返回值 |
|:-----|:------------|:-----------|:-------------|
| `dmsc_init()` | 初始化DMSC库 | 无 | `int` (0表示成功) |
| `dmsc_cleanup()` | 清理DMSC库资源 | 无 | `void` |
| `dmsc_version()` | 获取DMSC版本字符串 | 无 | `char*` (需要释放) |
| `dmsc_string_free(char* s)` | 释放DMSC返回的字符串 | `s`: 要释放的字符串 | `void` |

#### 使用示例

```c
#include <stdio.h>
#include "dmsc.h"

int main() {
    // 初始化DMSC
    int result = dmsc_init();
    if (result != 0) {
        fprintf(stderr, "DMSC初始化失败\n");
        return 1;
    }
    
    // 获取版本
    char* version = dmsc_version();
    printf("DMSC版本: %s\n", version);
    dmsc_string_free(version);
    
    // 清理
    dmsc_cleanup();
    return 0;
}
```

<div align="center">

## Core模块

</div>

### 类型

| 类型 | 描述 |
|:-----|:------------|
| `DMSCApplication` | 应用实例句柄 |
| `DMSCAppConfig` | 应用配置句柄 |
| `DMSCServiceContext` | 服务上下文句柄 |

### 函数

| 函数 | 描述 | 参数 | 返回值 |
|:---------|:------------|:-----------|:-------------|
| `dmsc_app_config_new()` | 创建新的应用配置 | 无 | `DMSCAppConfig*` |
| `dmsc_app_config_free(DMSCAppConfig* config)` | 释放应用配置 | `config`: 配置句柄 | `void` |
| `dmsc_app_config_set_name(DMSCAppConfig* config, const char* name)` | 设置应用名称 | `config`, `name` | `int` |
| `dmsc_app_config_set_environment(DMSCAppConfig* config, const char* env)` | 设置环境 | `config`, `env` | `int` |
| `dmsc_application_new(DMSCAppConfig* config)` | 创建应用实例 | `config` | `DMSCApplication*` |
| `dmsc_application_free(DMSCApplication* app)` | 释放应用实例 | `app` | `void` |
| `dmsc_application_start(DMSCApplication* app)` | 启动应用 | `app` | `int` |
| `dmsc_application_stop(DMSCApplication* app)` | 停止应用 | `app` | `int` |
| `dmsc_application_get_context(DMSCApplication* app)` | 获取服务上下文 | `app` | `DMSCServiceContext*` |

<div align="center">

## Log模块

</div>

### 类型

| 类型 | 描述 |
|:-----|:------------|
| `DMSCLogger` | 日志记录器实例句柄 |
| `DMSCLogConfig` | 日志配置句柄 |
| `DMSCLogLevel` | 日志级别枚举 |

### 日志级别

| 级别 | 值 | 描述 |
|:------|:------|:------------|
| `DMSC_LOG_TRACE` | 0 | 跟踪级别 |
| `DMSC_LOG_DEBUG` | 1 | 调试级别 |
| `DMSC_LOG_INFO` | 2 | 信息级别 |
| `DMSC_LOG_WARN` | 3 | 警告级别 |
| `DMSC_LOG_ERROR` | 4 | 错误级别 |
| `DMSC_LOG_FATAL` | 5 | 致命级别 |

### 函数

| 函数 | 描述 | 参数 | 返回值 |
|:---------|:------------|:-----------|:-------------|
| `dmsc_log_config_new()` | 创建日志配置 | 无 | `DMSCLogConfig*` |
| `dmsc_log_config_free(DMSCLogConfig* config)` | 释放日志配置 | `config` | `void` |
| `dmsc_log_config_set_level(DMSCLogConfig* config, DMSCLogLevel level)` | 设置日志级别 | `config`, `level` | `int` |
| `dmsc_logger_new(DMSCLogConfig* config)` | 创建日志记录器 | `config` | `DMSCLogger*` |
| `dmsc_logger_free(DMSCLogger* logger)` | 释放日志记录器 | `logger` | `void` |
| `dmsc_logger_log(DMSCLogger* logger, DMSCLogLevel level, const char* message)` | 记录日志消息 | `logger`, `level`, `message` | `int` |
| `dmsc_logger_trace(DMSCLogger* logger, const char* message)` | 记录跟踪日志 | `logger`, `message` | `int` |
| `dmsc_logger_debug(DMSCLogger* logger, const char* message)` | 记录调试日志 | `logger`, `message` | `int` |
| `dmsc_logger_info(DMSCLogger* logger, const char* message)` | 记录信息日志 | `logger`, `message` | `int` |
| `dmsc_logger_warn(DMSCLogger* logger, const char* message)` | 记录警告日志 | `logger`, `message` | `int` |
| `dmsc_logger_error(DMSCLogger* logger, const char* message)` | 记录错误日志 | `logger`, `message` | `int` |
| `dmsc_logger_fatal(DMSCLogger* logger, const char* message)` | 记录致命日志 | `logger`, `message` | `int` |

#### 使用示例

```c
DMSCLogConfig* log_config = dmsc_log_config_new();
dmsc_log_config_set_level(log_config, DMSC_LOG_INFO);

DMSCLogger* logger = dmsc_logger_new(log_config);
dmsc_log_config_free(log_config);

dmsc_logger_info(logger, "应用已启动");
dmsc_logger_warn(logger, "这是一个警告");
dmsc_logger_error(logger, "这是一个错误");

dmsc_logger_free(logger);
```

<div align="center">

## Cache模块

</div>

### 类型

| 类型 | 描述 |
|:-----|:------------|
| `DMSCCache` | 缓存实例句柄 |
| `DMSCCacheConfig` | 缓存配置句柄 |

### 函数

| 函数 | 描述 | 参数 | 返回值 |
|:---------|:------------|:-----------|:-------------|
| `dmsc_cache_config_new()` | 创建缓存配置 | 无 | `DMSCCacheConfig*` |
| `dmsc_cache_config_free(DMSCCacheConfig* config)` | 释放缓存配置 | `config` | `void` |
| `dmsc_cache_config_set_capacity(DMSCCacheConfig* config, size_t capacity)` | 设置缓存容量 | `config`, `capacity` | `int` |
| `dmsc_cache_config_set_ttl(DMSCCacheConfig* config, uint64_t ttl_seconds)` | 设置默认TTL | `config`, `ttl_seconds` | `int` |
| `dmsc_cache_new(DMSCCacheConfig* config)` | 创建缓存实例 | `config` | `DMSCCache*` |
| `dmsc_cache_free(DMSCCache* cache)` | 释放缓存实例 | `cache` | `void` |
| `dmsc_cache_set(DMSCCache* cache, const char* key, const void* value, size_t value_len, uint64_t ttl_seconds)` | 设置缓存值 | `cache`, `key`, `value`, `value_len`, `ttl_seconds` | `int` |
| `dmsc_cache_get(DMSCCache* cache, const char* key, void** value, size_t* value_len)` | 获取缓存值 | `cache`, `key`, `value`, `value_len` | `int` |
| `dmsc_cache_delete(DMSCCache* cache, const char* key)` | 删除缓存条目 | `cache`, `key` | `int` |
| `dmsc_cache_exists(DMSCCache* cache, const char* key)` | 检查键是否存在 | `cache`, `key` | `int` (1表示存在) |
| `dmsc_cache_clear(DMSCCache* cache)` | 清除所有缓存条目 | `cache` | `int` |

<div align="center">

## 错误处理

</div>

### 错误代码

| 代码 | 值 | 描述 |
|:-----|:------|:------------|
| `DMSC_OK` | 0 | 成功 |
| `DMSC_ERROR_GENERAL` | -1 | 一般错误 |
| `DMSC_ERROR_INVALID_ARG` | -2 | 无效参数 |
| `DMSC_ERROR_ALLOCATION` | -3 | 内存分配失败 |
| `DMSC_ERROR_NOT_FOUND` | -4 | 资源未找到 |
| `DMSC_ERROR_PERMISSION` | -5 | 权限被拒绝 |
| `DMSC_ERROR_TIMEOUT` | -6 | 超时 |
| `DMSC_ERROR_NETWORK` | -7 | 网络错误 |

<div align="center">

## 内存管理

</div>

C API使用手动内存管理，遵循C语言惯例：

- **对象创建**: 构造函数返回新分配的对象。所有对象必须使用相应的析构函数释放。
- **字符串处理**: 返回字符串的函数分配C字符串，必须使用`dmsc_string_free()`释放。不要对这些字符串使用标准`free()`。
- **NULL安全**: 所有函数都优雅地处理NULL指针，返回错误代码或NULL输出，而不是导致未定义行为。

<div align="center">

## 线程安全

</div>

DMSC C API专为线程安全的并发访问而设计：

- **对象级安全**: 除非另有说明，否则单个对象可安全地供多个线程并发使用。
- **全局状态**: 全局初始化是线程安全的；从多个线程后续调用`dmsc_init()`会被正确处理。
- **资源共享**: 对象可以跨线程共享，遵循与底层Rust实现相同的模式。

<div align="center">

## 特性标志

</div>

DMSC C API支持条件编译的特性标志：

| 特性 | 描述 |
|:--------|:------------|
| `c` | 启用C/C++ FFI绑定（必需） |
| `gateway` | 启用API网关功能 |
| `grpc` | 启用gRPC服务器和客户端支持 |
| `observability` | 启用指标和追踪 |
| `service-mesh` | 启用服务网格集成 |
| `websocket` | 启用WebSocket支持 |

<div align="center">

## 构建集成

</div>

要在C/C++项目中使用DMSC C API：

1. **编译**: 包含生成的C头文件，并链接到DMSC共享或静态库。
2. **头文件**: 包含主DMSC头文件，通过统一的API接口访问所有子模块。
3. **链接**: 使用适合您构建系统的链接器标志链接到DMSC库。

### 示例 CMakeLists.txt

```cmake
cmake_minimum_required(VERSION 3.10)
project(my_dmsc_app)

find_library(DMSC_LIB dmsc PATHS /path/to/dmsc/lib)

add_executable(my_app main.c)
target_include_directories(my_app PRIVATE /path/to/dmsc/include)
target_link_libraries(my_app ${DMSC_LIB})
```

### 示例构建命令

```bash
# 编译
gcc -c main.c -I/path/to/dmsc/include

# 链接
gcc main.o -L/path/to/dmsc/lib -ldmsc -o my_app
```
