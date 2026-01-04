<div align="center">

# 术语表

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本章定义了DMSC文档中使用的技术术语和概念，帮助您理解相关内容。

## A

</div>

### API Gateway

API网关是一个服务器，作为API请求的统一入口，提供路由、负载均衡、限流、熔断等功能。

### Async/Await

Rust中的异步编程模型，允许编写非阻塞代码，提高系统的并发处理能力。

### Authentication

认证，验证用户或系统的身份。DMSC支持JWT和OAuth2等认证方式。

<div align="center">

## B

</div>

### Builder Pattern

构建器模式，一种创建复杂对象的设计模式。DMSC使用`DMSCAppBuilder`实现构建器模式。

### Batch Processing

批处理，将多个操作合并为一批执行，减少系统调用和网络开销。

<div align="center">

## C

</div>

### Cache

缓存，临时存储频繁访问的数据，减少对后端系统的访问。DMSC支持多种缓存后端。

### Cache Penetration

缓存穿透，指查询一个不存在的数据，导致请求直接到达后端系统。

### Cache Consistency

缓存一致性，确保缓存中的数据与后端系统中的数据保持一致。

### Configuration Management

配置管理，管理应用程序的配置，支持多环境、热重载等功能。

### Containerization

容器化，将应用程序及其依赖打包到容器中，实现环境隔离和快速部署。

### Core Module

核心模块，DMSC的基础模块，提供运行时、错误处理和服务上下文等功能。

<div align="center">

## D

</div>

### DMSC

Dunimd Middleware Service，一个高性能的Rust中间件框架，统一后端基础设施。

### DMSCError

DMSC的统一错误类型，包含错误代码、消息和上下文信息。

### DMSCResult

结果类型别名，定义为`Result<T, DMSCError>`，简化错误处理。

### DMSCAppBuilder

应用构建器，用于配置和构建DMSC应用。

### DMSCServiceContext

服务上下文，提供对所有模块功能的访问。

### DMSCModule

模块 trait，用于创建自定义同步模块。

### Distributed Tracing

分布式追踪，跟踪请求在分布式系统中的流动，帮助定位性能瓶颈。

### Docker

一个开源的容器化平台，用于构建、运行和管理容器。

<div align="center">

## E

</div>

### Environment Variables

环境变量，在操作系统中设置的变量，用于配置应用程序。

### Error Handling

错误处理，处理应用程序中的错误，确保系统的可靠性。

<div align="center">

## F

</div>

### Fault Tolerance

容错，系统在出现故障时仍能正常运行的能力。

### File System

文件系统，管理文件和目录的系统。DMSC提供安全的文件系统操作。

<div align="center">

## G

</div>

### Gateway

网关，作为系统的入口，处理外部请求并转发到内部服务。

### Grafana

一个开源的监控和可视化平台，用于展示Prometheus指标。

<div align="center">

## H

</div>

### Health Check

健康检查，定期检查服务的健康状态，确保服务正常运行。

### Hooks

钩子，在特定生命周期阶段执行的自定义逻辑。

### HTTPS

安全的HTTP协议，使用TLS/SSL加密数据传输。

<div align="center">

## I

</div>

### Initialization

初始化，模块或应用程序的启动准备阶段。

### Inversion of Control

控制反转，一种设计模式，将对象的创建和依赖管理交给框架。

<div align="center">

## J

</div>

### JWT

JSON Web Token，一种用于认证的令牌格式，包含用户信息和签名。

<div align="center">

## K

</div>

### Kubernetes

一个开源的容器编排平台，用于自动化部署、扩展和管理容器化应用。

<div align="center">

## L

</div>

### Load Balancing

负载均衡，将请求分发到多个服务器，提高系统的可用性和性能。

### Logging

日志记录，记录应用程序的运行状态和事件。

### Log Level

日志级别，定义日志的重要程度，包括DEBUG、INFO、WARN、ERROR等。

<div align="center">

## M

</div>

### Middleware

中间件，位于应用程序和操作系统之间的软件层，提供通用功能。

### Modular Architecture

模块化架构，将系统划分为独立的模块，支持按需组合和扩展。

### Module

模块，DMSC的功能单元，提供特定领域的功能。

### Mutex

互斥锁，用于保护共享资源，防止并发访问导致的数据竞争。

<div align="center">

## N

</div>

### Non-blocking I/O

非阻塞I/O，允许应用程序在等待I/O操作完成时继续执行其他任务。

<div align="center">

## O

</div>

### Observability

可观测性，通过日志、指标和追踪了解系统的内部状态。

### OAuth2

开放授权协议，允许第三方应用访问用户资源，无需共享密码。

### OpenTelemetry

一个开源的可观测性框架，提供统一的日志、指标和追踪解决方案。

<div align="center">

## P

</div>

### Prometheus

一个开源的监控和告警系统，用于收集和存储时间序列指标。

### Priority

优先级，模块的加载顺序，数值越大，优先级越高。

<div align="center">

## Q

</div>

### Queue

队列，用于异步处理任务，实现系统解耦和削峰填谷。

<div align="center">

## R

</div>

### Rate Limiting

限流，限制请求的速率，防止系统过载。

### Redis

一个开源的内存数据库，常用于缓存、消息队列和会话存储。

### Rust

一种系统编程语言，提供高性能、内存安全和并发支持。

<div align="center">

## S

</div>

### Service Mesh

服务网格，用于管理服务间通信，提供服务发现、负载均衡等功能。

### Service Discovery

服务发现，自动检测和注册可用的服务实例。

### SpanID

跨度ID，分布式追踪中的基本单元，代表一个操作的执行。

### Structured Logging

结构化日志，使用键值对格式记录日志，便于日志分析和处理。

### Synchronous Programming

同步编程，代码按顺序执行，一个操作完成后才执行下一个操作。

<div align="center">

## T

</div>

### TLS/SSL

传输层安全/安全套接层，用于加密网络通信。

### Tokio

Rust的异步运行时，提供异步I/O和任务调度。

### TraceID

追踪ID，分布式追踪中标识一个完整请求的唯一ID。

### Transaction

事务，一组操作，要么全部成功，要么全部失败。

<div align="center">

## U

</div>

### Unwrapping

解包，从Result或Option类型中提取值，可能导致程序崩溃。

<div align="center">

## V

</div>

### Virtual Machine

虚拟机，模拟物理计算机的软件环境，用于运行操作系统和应用程序。

<div align="center">

## W

</div>

### W3C Trace Context

W3C分布式追踪上下文标准，定义了TraceID和SpanID的格式和传播方式。

### WebAssembly

一种低级编程语言，可在Web浏览器中运行，提供接近原生的性能。

<div align="center">

## X

</div>

### XSS

跨站脚本攻击，攻击者将恶意脚本注入到网页中，获取用户数据。

<div align="center">

## Y

</div>

### YAML

一种人类可读的数据序列化格式，常用于配置文件。

<div align="center">

## Z

</div>

### Zero Copy

零拷贝，一种I/O优化技术，减少内存拷贝开销，提高性能。

<div align="center">

## 设计模式

</div>

### Dependency Injection

依赖注入，一种设计模式，将对象的依赖注入到对象中，而不是对象自己创建依赖。

### Single Responsibility Principle

单一职责原则，一个类或模块只负责一个特定的功能。

### Loose Coupling

松耦合，模块间的依赖关系较弱，便于维护和扩展。

### High Cohesion

高内聚，相关功能集中在同一模块内，提高模块的可维护性。

<div align="center">

## 性能优化

</div>

### Connection Pool

连接池，预先创建和管理数据库连接，减少连接建立和销毁的开销。

### Memory Leak

内存泄漏，程序在运行过程中未能释放不再使用的内存，导致内存使用率持续增长。

### Throughput

吞吐量，系统在单位时间内处理的请求数量。

### Latency

延迟，请求从发出到收到响应的时间。

<div align="center">

## 安全

</div>

### Principle of Least Privilege

最小权限原则，用户或系统只获得完成任务所需的最小权限。

### CSRF

跨站请求伪造，攻击者利用用户的身份执行未授权的操作。

### SQL Injection

SQL注入，攻击者通过输入恶意SQL代码，破坏数据库的安全性。

### Encryption

加密，将数据转换为密文，防止未授权访问。

### Decryption

解密，将密文转换为原始数据。

<div align="center">

## 部署

</div>

### Rolling Update

滚动更新，逐步替换旧版本的服务实例，避免服务中断。

### Blue-Green Deployment

蓝绿部署，同时运行两个版本的服务，通过切换路由实现无缝更新。

### Canary Deployment

金丝雀部署，将新版本的服务先部署给一小部分用户，验证无误后再全面推广。

### CI/CD

持续集成/持续部署，自动化构建、测试和部署流程，提高开发效率。

<div align="center">

## 监控

</div>

### Metrics

指标，用于衡量系统性能和状态的数据，如CPU使用率、内存使用率等。

### Counter

计数器，单调递增的指标，用于记录事件发生的次数。

### Gauge

仪表盘，可增可减的指标，用于记录当前值。

### Histogram

直方图，用于记录数值的分布情况，如请求延迟。

### Summary

摘要，用于记录数值的分位数，如P50、P95、P99延迟。

<div align="center">

## 缓存

</div>

### Cache Hit

缓存命中，请求的数据在缓存中找到。

### Cache Miss

缓存未命中，请求的数据不在缓存中，需要从后端系统获取。

### Cache Eviction

缓存淘汰，当缓存满时，移除旧的数据，为新数据腾出空间。

### TTL

生存时间，缓存数据的有效期，过期后自动失效。

<div align="center">

## 异步编程

</div>

### Future

未来，代表一个异步操作的结果，最终会完成或失败。

### Task

任务，异步运行的代码单元，由Tokio运行时调度。

### Spawn

生成，创建一个新的异步任务并调度执行。

### Join

等待多个异步任务完成。

<div align="center">

## 数据库

</div>

### ACID

原子性、一致性、隔离性、持久性，数据库事务的四个特性。

### NoSQL

非关系型数据库，不使用传统的关系模型，适用于大规模数据存储。

### SQL

结构化查询语言，用于管理关系型数据库。

### Index

索引，提高数据库查询速度的数据结构。

<div align="center">

## 网络

</div>

### DNS

域名系统，将域名转换为IP地址。

### TCP

传输控制协议，一种可靠的面向连接的协议。

### UDP

用户数据报协议，一种不可靠的无连接协议。

### HTTP

超文本传输协议，用于传输网页和数据。

### REST

表述性状态转移，一种设计Web API的架构风格。

### gRPC

一种高性能、开源的远程过程调用框架，基于HTTP/2。

<div align="center">

## 开发流程

</div>

### Code Review

代码审查，检查代码质量、安全性和正确性，提高代码质量。

### Unit Test

单元测试，测试代码的最小单元，如函数或方法。

### Integration Test

集成测试，测试多个组件之间的交互。

### End-to-End Test

端到端测试，测试整个系统的功能，从用户界面到后端服务。

### Mock

模拟，用一个简化的对象替代真实对象，用于测试。

### Stub

存根，返回预设值的模拟对象，用于测试。

<div align="center">

## 操作系统

</div>

### Process

进程，正在运行的程序实例。

### Thread

线程，进程内的执行单元，共享进程的资源。

### CPU Bound

CPU密集型，程序的执行速度主要受CPU性能限制。

### I/O Bound

I/O密集型，程序的执行速度主要受I/O操作速度限制。

<div align="center">

## 其他

</div>

### DevOps

开发和运维的结合，强调自动化和协作，提高软件交付速度和质量。

### SRE

站点可靠性工程，将软件工程实践应用于运维工作，提高系统可靠性。

### Chaos Engineering

混沌工程，通过主动注入故障，测试系统的容错能力。

### Microservices

微服务，将应用程序划分为独立的小型服务，便于开发、部署和扩展。

### Monolith

单体应用，所有功能都包含在一个应用程序中。

### Serverless

无服务器，一种云计算模型，开发者无需管理服务器，只需编写代码。
