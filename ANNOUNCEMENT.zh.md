<div align="center">

<img src="assets/svg/ri.svg" width="36" height="36">


[English](ANNOUNCEMENT.md) | 简体中文


# DMSC项目更名为RI

## 概述

</div>

DMSC（Dunimd Middleware Service）项目将从0.1.9版本开始更名为**Ri**。这一变更反映了我们对项目更简洁、更易记的品牌愿景。

<div align="center">

## 变更内容

</div>

### 1. 项目名称
- **旧名称**: DMSC (Dunimd Middleware Service)
- **新名称**: Ri

### 2. 包名称
- **Rust**: `dmsc` → `ri`
- **Python**: `dmsc` → `ri`

### 3. 类型名称
所有公开类型将重命名：
- `DMSCConfig` → `RiConfig`
- `DMSCConfigManager` → `RiConfigManager`
- `DMSCLogger` → `RiLogger`
- `DMSCError` → `RiError`
- `DMSCResult` → `RiResult`
- `DMSCAppBuilder` → `RiAppBuilder`
- `DMSCServiceContext` → `RiServiceContext`
- `DMSCFileSystem` → `RiFileSystem`
- `DMSCHookBus` → `RiHookBus`
- `DMSCValidator` → `RiValidator`
- 以及所有其他`DMSC*`类型...

### 4. 导入语句
- **旧**: `use dmsc::prelude::*;`
- **新**: `use ri::prelude::*;`

- **旧**: `import dmsc`
- **新**: `import ri`

<div align="center">

## 时间线

</div>

- **0.1.8（当前版本）**: 使用DMSC命名的最后版本
- **0.1.9（即将发布）**: 使用Ri命名的首个版本，包含CLI工具
- **未来**: 增强功能和生态系统

<div align="center">

## 迁移指南

</div>

### Rust用户

**之前（0.1.8）**:
```rust
use dmsc::prelude::*;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC service started")?;
        Ok(())
    }).await
}
```

**之后（0.1.9）**:
```rust
use ri::prelude::*;

#[tokio::main]
async fn main() -> RiResult<()> {
    let app = RiAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(RiLogConfig::default())
        .build()?;

    app.run(|ctx: &RiServiceContext| async move {
        ctx.logger().info("service", "Ri service started")?;
        Ok(())
    }).await
}
```

### Python用户

**之前（0.1.8）**:
```python
from dmsc import DMSCConfig, DMSCLogger, DMSCAppBuilder

config = DMSCConfig()
logger = DMSCLogger()
app = DMSCAppBuilder()
```

**之后（0.1.9）**:
```python
from ri import RiConfig, RiLogger, RiAppBuilder

config = RiConfig()
logger = RiLogger()
app = RiAppBuilder()
```

<div align="center">

## 为什么进行此变更？

</div>

1. **简洁性**: "Ri"更短、更易记
2. **品牌识别**: 独特且具有辨识度的名称
3. **未来发展**: 为未来发展提供更好的定位
4. **社区推广**: 更易于社区采用和识别

<div align="center">

## 0.1.9版本新特性

</div>

除了更名之外，0.1.9版本还将引入：

1. **CLI工具**: 命令行界面，便于项目管理
   - 配置管理
   - 服务生命周期控制
   - 模块管理
   - 诊断工具

2. **增强文档**: 改进和完善的文档

3. **更好的开发体验**: 简化的工作流程和工具

<div align="center">

## 您需要做什么

</div>

1. **更新依赖**: 在`Cargo.toml`或`requirements.txt`中将`dmsc`改为`ri`
2. **更新导入**: 将所有`dmsc`导入替换为`ri`
3. **更新类型名称**: 保持所有`Ri*`类型不变（无需重命名）
4. **测试代码**: 确保新名称下一切正常
5. **查阅文档**: 查看更新后的文档了解API变更

<div align="center">

## 向后兼容性

</div>

0.1.8版本将继续可用，但不会收到未来的更新。我们建议迁移到0.1.9+以获取最新功能、错误修复和安全更新。

<div align="center">

## 支持

</div>

如果在迁移过程中遇到任何问题：
- 在[GitHub](https://github.com/mf2023/Ri)上提交issue
- 查阅文档中的迁移指南
- 查看仓库中更新的示例

<div align="center">

## 破坏性变更摘要

| 组件 | 0.1.8 | 0.1.9+ |
|------|-------|--------|
| 包名（Rust） | `dmsc` | `ri` |
| 包名（Python） | `dmsc` | `ri` |
| 类型前缀 | `Ri*` | `Ri*` |
| 导入路径 | `dmsc::*` | `ri::*` |

## 感谢

</div>

感谢您对本项目的持续支持。我们相信这一变更将有利于项目的长期发展，并使其更易于全球开发者使用。

新名称"Ri"代表了我们要为社区提供一个**可靠**（Reliable）和**创新**（Innovative）的中间件框架的承诺。
