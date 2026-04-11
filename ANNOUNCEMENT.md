# Important Announcement: DMSC Project Renaming to RI

**Date**: 2026-04-10  
**Version**: 0.1.9

---

## Summary

The DMSC (Dunimd Middleware Service) project will be renamed to **RI** starting from version 0.1.9. This change reflects our vision for a more concise and memorable project identity.

## What's Changing

### 1. Project Name
- **Old Name**: DMSC (Dunimd Middleware Service)
- **New Name**: RI

### 2. Package Names
- **Rust**: `dmsc` → `ri`
- **Python**: `dmsc` → `ri`

### 3. Type Names
All public types will be renamed:
- `DMSCConfig` → `RIConfig`
- `DMSCConfigManager` → `RIConfigManager`
- `DMSCLogger` → `RILogger`
- `DMSCError` → `RIError`
- `DMSCResult` → `RIResult`
- `DMSCAppBuilder` → `RIAppBuilder`
- `DMSCServiceContext` → `RIServiceContext`
- `DMSCFileSystem` → `RIFileSystem`
- `DMSCHookBus` → `RIHookBus`
- `DMSCValidator` → `RIValidator`
- And all other `DMSC*` types...

### 4. Import Statements
- **Old**: `use dmsc::prelude::*;`
- **New**: `use ri::prelude::*;`

- **Old**: `import dmsc`
- **New**: `import ri`

## Timeline

- **0.1.8 (Current)**: Final version with DMSC naming
- **0.1.9 (Upcoming)**: First version with RI naming, includes CLI tools
- **Future**: Enhanced features and ecosystem

## Migration Guide

### For Rust Users

**Before (0.1.8)**:
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

**After (0.1.9)**:
```rust
use ri::prelude::*;

#[tokio::main]
async fn main() -> RIResult<()> {
    let app = RIAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(RILogConfig::default())
        .build()?;
    
    app.run(|ctx: &RIServiceContext| async move {
        ctx.logger().info("service", "RI service started")?;
        Ok(())
    }).await
}
```

### For Python Users

**Before (0.1.8)**:
```python
from dmsc import DMSCConfig, DMSCLogger, DMSCAppBuilder

config = DMSCConfig()
logger = DMSCLogger()
app = DMSCAppBuilder()
```

**After (0.1.9)**:
```python
from ri import RIConfig, RILogger, RIAppBuilder

config = RIConfig()
logger = RILogger()
app = RIAppBuilder()
```

## Why This Change?

1. **Simplicity**: "RI" is shorter and easier to remember
2. **Brand Identity**: A unique and distinctive name
3. **Future Growth**: Better positioning for future development
4. **Community**: Easier for community adoption and recognition

## What's New in 0.1.9

In addition to the renaming, version 0.1.9 will introduce:

1. **CLI Tools**: Command-line interface for easier project management
   - Configuration management
   - Service lifecycle control
   - Module management
   - Diagnostic tools

2. **Enhanced Documentation**: Improved and comprehensive documentation

3. **Better Developer Experience**: Streamlined workflows and tooling

## What You Need to Do

1. **Update Dependencies**: Change `dmsc` to `ri` in your `Cargo.toml` or `requirements.txt`
2. **Update Imports**: Replace all `dmsc` imports with `ri`
3. **Update Type Names**: Replace all `Ri*` types with `RI*`
4. **Test Your Code**: Ensure everything works with the new names
5. **Review Documentation**: Check the updated documentation for any API changes

## Backward Compatibility

Version 0.1.8 will remain available but will not receive future updates. We recommend migrating to 0.1.9+ for the latest features, bug fixes, and security updates.

## Support

If you encounter any issues during migration:
- Open an issue on [GitHub](https://github.com/mf2023/Ri)
- Check the migration guide in the documentation
- Review the updated examples in the repository

## Breaking Changes Summary

| Component | 0.1.8 | 0.1.9+ |
|-----------|-------|--------|
| Package Name (Rust) | `dmsc` | `ri` |
| Package Name (Python) | `dmsc` | `ri` |
| Type Prefix | `Ri*` | `RI*` |
| Import Path | `dmsc::*` | `ri::*` |

## Thank You

Thank you for your continued support of this project. We believe this change will benefit the project's long-term growth and make it more accessible to developers worldwide.

The new name "RI" represents our commitment to providing a **R**eliable and **I**nnovative middleware framework for the community.

---

**The RI Team**  
**Date**: 2026-04-10
