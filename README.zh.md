<div align="center">

# ⚠️ 合规提示

**根据各国相关法律法规（包括但不限于中国《生成式人工智能服务管理暂行办法》、欧盟《人工智能法案》、美国《AI风险管理框架》、日本《AI指导原则》等），开发者或使用者需自行承担合规责任，未履行相关义务可能导致服务被叫停、面临监管处罚或承担法律责任。**

---

# DMS (Dunimd Middleware Service)

[English](README.md) | [简体中文](README.zh.md)

<a href="https://github.com/dunimd/dms" target="_blank">
    <img alt="GitHub" src="https://img.shields.io/badge/GitHub-DMS-181717?style=flat-square&logo=github"/>
</a>
<a href="https://gitee.com/dunimd" target="_blank">
    <img alt="Gitee" src="https://img.shields.io/badge/Gitee-Dunimd-C71D23?style=flat-square&logo=gitee"/>
</a>
<a href="https://crates.io/crates/DMS" target="_blank">
    <img alt="Crates.io" src="https://img.shields.io/badge/Crates-DMS-000000?style=flat-square&logo=rust"/>
</a>

企业级Rust服务框架，为Dunimd团队所有项目提供统一的基础设施支撑。DMS将原本分散的Python utils工具集重构为现代化、企业级的Rust服务框架，类似于GMS/HMS的定位，为所有后端服务提供统一的基础设施能力。

</div>

<h2 align="center">🚀 核心架构</h2>

### 🌐 分布式链路追踪系统
实现W3C Trace Context标准，支持全链路TraceID/SpanID传播，业务上下文信息Baggage数据透传，标准化追踪上下文载体机制，支持与Java、Go、Python等异构系统的多语言兼容集成。

### 📊 企业级可观测性平台
原生Prometheus指标导出，支持Counter、Gauge、Histogram、Summary类型，开箱即用的Grafana仪表板集成，高性能滑动窗口算法实现实时数据收集（DMSSlidingWindow），精确的分位数计算用于性能统计分析（DMSQuantileCalculator），CPU、内存、I/O、网络等全栈指标的多维度监控。

### 🤖 智能设备控制与调度
智能设备自动发现和注册，高效的设备资源池管理与分配回收，基于优先级的策略化智能调度算法，动态设备负载均衡，以及包含设备状态监控和维护的完整生命周期管理。

### 📝 企业级日志系统
支持JSON和文本格式的结构化日志输出，可配置采样率避免性能影响，基于文件大小的智能日志轮转，自动包含追踪上下文信息，DEBUG/INFO/WARN/ERROR四级日志的多级别支持。

### ⚙️ 配置管理与扩展性
配置文件、环境变量、运行时参数的多源配置加载，运行时动态更新的热配置能力，7大核心模块的模块化架构支持按需组合，Startup、Shutdown等关键事件的生命周期钩子，以及支持自定义模块和扩展点的插件化扩展机制。

### 📁 文件系统与数据管理
统一的项目根目录管理实现文件系统命名空间，保证数据一致性的原子文件操作，分离日志、缓存、报告、观测性、临时目录的分类目录管理，复杂数据结构序列化的JSON数据持久化，以及防止路径穿越和权限问题的安全目录创建。

### 🔧 模块化架构

```
DMS Framework
├── dms-core          # 核心运行时与错误处理
├── dms-config        # 统一配置管理
├── dms-log           # 企业级日志系统
├── dms-observability # 可观测性平台
├── dms-hooks         # 生命周期钩子
├── dms-cache         # 缓存抽象层
├── dms-fs            # 文件系统封装
├── dms-resource      # 资源管理（泛化设备）
└── dms-extension-api # 扩展能力定义
```

### 🎯 核心组件

- **ServiceRuntime**: 统一服务运行时
- **ServiceContext**: 服务上下文，集成所有基础设施
- **AppBuilder**: 声明式服务构建器
- **DMSModule**: 模块化扩展接口

---

<h2 align="center">⚡ 快速开始</h2>

### **1. 添加依赖**

```toml
[dependencies]
DMS = { git = "https://github.com/dunimd/dms" }
```

### **2. 创建服务**

```rust
use DMS::prelude::*;

#[tokio::main]
async fn main() -> DMSResult<()> {
    // 构建服务运行时
    let app = DMSAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(LoggingConfig::default())?
        .with_observability(ObservabilityConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSServiceContext| async move {
        ctx.logger().info("service", "DMS service started")?;
        // 您的业务代码
        Ok(())
    }).await
}
```

### **3. 使用观测性**

```rust
use DMS::observability::*;

#[traced(name = "user_service")]
async fn get_user(ctx: &DMSServiceContext, user_id: u64) -> DMSResult<User> {
    // 自动记录追踪信息和指标
    let user = fetch_user_from_db(user_id).await?;
    Ok(user)
}
```

---

<h2 align="center">📈 性能指标</h2>

| 指标 | 数值 |
|--------|-------|
| **编译时间** | Release构建 < 15秒 |
| **内存占用** | 基础运行时 < 10MB |
| **零成本抽象** | 编译时优化，无运行时开销 |
| **线程安全** | Rust所有权系统保证内存安全 |
| **构建状态** | 零警告，零错误 |

---

<h2 align="center">🔧 配置示例</h2>

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: true
  console_enabled: true

observability:
  metrics_enabled: true
  tracing_enabled: true
  prometheus_port: 9090

resource:
  providers: ["cpu", "gpu", "memory"]
  scheduling_policy: "priority_based"
```

---

<h2 align="center">🧪 开发状态</h2>

| 模块 | 状态 | 描述 |
|--------|--------|-------------|
| **核心模块** | ✅ 完成 | 全部7个核心模块已完成 |
| **扩展机制** | ✅ 已支持 | 插件化架构已就绪 |
| **示例项目** | ✅ 已提供 | 可用示例已提供 |
| **文档** | ✅ 完整 | 完整API文档 |
| **测试** | ✅ 覆盖 | 单元测试覆盖 |
| **CI/CD** | ✅ 自动化 | 自动化构建流水线 |

---

<h2 align="center">📚 模块文档</h2>

- [核心模块](src/core/) - 运行时与错误处理
- [配置管理](src/config/) - 统一配置接口
- [日志系统](src/log/) - 结构化日志
- [可观测性](src/observability/) - 指标与追踪
- [设备控制](src/device/) - 资源调度管理
- [文件系统](src/fs/) - 安全文件操作
- [生命周期钩子](src/hooks/) - 事件系统

---

<h2 align="center">🤝 贡献指南</h2>

1. Fork项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建Pull Request

---

<h2 align="center">📄 许可证</h2>

本项目采用 [MIT许可证](LICENSE) - 详见 [LICENSE](LICENSE) 文件

---

<h2 align="center">🏆 成就</h2>

- **零警告零错误**: 通过`cargo check --quiet`验证
- **企业级质量**: 支持生产环境部署
- **模块化设计**: 支持按需组合和扩展
- **性能优化**: Release构建优化完成

---

<div align="center">

**DMS** - *为Dunimd的每一个服务提供坚实的技术基础* 🦀✨

</div>