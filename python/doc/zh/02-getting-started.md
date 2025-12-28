<div align="center">

# DMSC Python 快速开始

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

安装、配置和运行第一个DMSC Python应用

</div>

## 系统要求

在开始之前，请确保您的系统满足以下要求：

### 操作系统支持
- **Windows**: Windows 10 或更高版本
- **Linux**: Ubuntu 18.04+, CentOS 7+, Debian 9+
- **macOS**: macOS 10.15 (Catalina) 或更高版本

### Python版本
- **Python 3.8** 或更高版本
- **推荐版本**: Python 3.10+

### 硬件要求
- **内存**: 最少 2GB RAM (推荐 4GB+)
- **存储**: 至少 500MB 可用磁盘空间
- **CPU**: x86_64 或 ARM64 架构

## 安装方式

### 使用 pip 安装（推荐）

```bash
# 安装最新稳定版本
pip install dmsc

# 或者指定版本
pip install dmsc==1.0.0

# 升级到最新版本
pip install --upgrade dmsc
```

### 使用 Poetry 安装

```bash
# 添加到项目依赖
poetry add dmsc

# 或者添加到开发依赖
poetry add dmsc --group dev
```

### 使用 pipenv 安装

```bash
# 安装到虚拟环境
pipenv install dmsc

# 安装开发版本
pipenv install dmsc --dev
```

### 从源代码安装

```bash
# 克隆仓库
git clone https://github.com/dunimd/dmsc-python.git

# 进入目录
cd dmsc-python

# 安装构建依赖
pip install maturin

# 构建并安装
maturin develop --release
```

## 验证安装

安装完成后，可以通过以下方式验证：

```python
# 检查版本
import dmsc
print(f"DMSC Python版本: {dmsc.__version__}")

# 检查核心模块
print(f"可用模块: {dmsc.available_modules()}")

# 运行健康检查
result = dmsc.health_check()
print(f"系统状态: {result}")
```

## 第一个应用

让我们创建一个简单的DMSC Python应用：

### 1. 创建项目目录

```bash
mkdir my-dmsc-app
cd my-dmsc-app

# 创建虚拟环境（推荐）
python -m venv venv
source venv/bin/activate  # Linux/Mac
# 或者
venv\Scripts\activate  # Windows

# 安装DMSC
pip install dmsc
```

### 2. 创建主应用文件

创建 `main.py` 文件：

```python
import asyncio
import logging
from dmsc import DMSCAppBuilder, DMSCLogConfig, DMSCConfig

async def main():
    """主应用入口"""
    
    # 配置日志
    log_config = DMSCLogConfig(
        level="INFO",
        format="json",
        enable_console=True,
        enable_file=True,
        file_path="logs/app.log"
    )
    
    # 创建应用构建器
    app = DMSCAppBuilder()
    
    # 添加日志模块
    app.with_logging(log_config)
    
    # 添加配置
    config = DMSCConfig()
    config.set("app.name", "My First DMSC App")
    config.set("app.version", "1.0.0")
    app.with_config(config)
    
    # 构建应用
    dms_app = app.build()
    
    # 运行应用
    await dms_app.run_async(application_logic)

async def application_logic(ctx):
    """应用业务逻辑"""
    
    # 获取日志器
    logger = ctx.logger
    
    # 记录应用启动
    logger.info("application", "DMSC Python应用启动成功！")
    
    # 获取配置
    app_name = ctx.config.get("app.name", "Unknown App")
    app_version = ctx.config.get("app.version", "0.0.0")
    
    logger.info("application", f"应用信息: {app_name} v{app_version}")
    
    # 模拟一些业务逻辑
    logger.info("business", "开始执行业务逻辑...")
    
    # 使用缓存（如果可用）
    if hasattr(ctx, 'cache'):
        await ctx.cache.set("startup_time", "2025-01-01 12:00:00", ttl=3600)
        startup_time = await ctx.cache.get("startup_time")
        logger.info("cache", f"启动时间: {startup_time}")
    
    # 模拟异步操作
    await asyncio.sleep(2)
    
    logger.info("business", "业务逻辑执行完成！")
    
    return {
        "status": "success",
        "message": "Hello from DMSC Python!",
        "app": app_name,
        "version": app_version
    }

if __name__ == "__main__":
    # 运行应用
    asyncio.run(main())
```

### 3. 运行应用

```bash
# 运行应用
python main.py

# 你应该会看到类似输出：
# {"timestamp": "2025-01-01T12:00:00Z", "level": "INFO", "module": "application", "message": "DMSC Python应用启动成功！"}
# {"timestamp": "2025-01-01T12:00:00Z", "level": "INFO", "module": "application", "message": "应用信息: My First DMSC App v1.0.0"}
# {"timestamp": "2025-01-01T12:00:00Z", "level": "INFO", "module": "business", "message": "开始执行业务逻辑..."}
# {"timestamp": "2025-01-01T12:00:00Z", "level": "INFO", "module": "business", "message": "业务逻辑执行完成！"}
```

## 配置详解

### 日志配置

```python
from dmsc import DMSCLogConfig

# 基础配置
log_config = DMSCLogConfig(
    level="INFO",           # DEBUG, INFO, WARN, ERROR
    format="json",          # json 或 text
    enable_console=True,    # 输出到控制台
    enable_file=True,       # 输出到文件
    file_path="logs/app.log" # 日志文件路径
)

# 高级配置
log_config = DMSCLogConfig(
    level="DEBUG",
    format="json",
    enable_console=True,
    enable_file=True,
    file_path="logs/app.log",
    max_file_size="100MB",  # 单个文件最大大小
    max_files=10,           # 保留的日志文件数量
    compress=True,          # 是否压缩旧日志
    enable_trace=True       # 启用追踪上下文
)
```

### 配置管理

```python
from dmsc import DMSCConfig

# 创建配置
config = DMSCConfig()

# 设置配置值
config.set("database.host", "localhost")
config.set("database.port", 5432)
config.set("database.name", "myapp")

# 获取配置值（带默认值）
host = config.get("database.host", "127.0.0.1")
port = config.get("database.port", 5432)

# 从文件加载配置
config.load_file("config.yaml")
config.load_file("config.json")

# 从环境变量加载
config.load_env("MYAPP_")  # 加载MYAPP_*开头的环境变量

# 配置热重载
config.enable_hot_reload(True)
```

### 缓存配置

```python
from dmsc import DMSCAppBuilder, DMSCCacheConfig, DMSCCacheBackend

# 内存缓存
app = DMSCAppBuilder()
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.MEMORY,
        max_size=1000,
        ttl=3600
    )
)

# Redis缓存
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.REDIS,
        redis_url="redis://localhost:6379/0",
        max_connections=10,
        ttl=7200
    )
)

# 混合缓存（内存+Redis）
app.with_cache(
    DMSCCacheConfig(
        backend=DMSCCacheBackend.HYBRID,
        memory_max_size=100,
        redis_url="redis://localhost:6379/0",
        ttl=3600
    )
)
```

## 项目结构建议

一个典型的DMSC Python项目结构：

```
my-dmsc-project/
├── src/
│   ├── __init__.py
│   ├── main.py              # 应用入口
│   ├── config/
│   │   ├── __init__.py
│   │   ├── settings.py      # 配置管理
│   │   └── logging.py       # 日志配置
│   ├── services/
│   │   ├── __init__.py
│   │   ├── user_service.py  # 用户服务
│   │   └── order_service.py # 订单服务
│   ├── models/
│   │   ├── __init__.py
│   │   ├── user.py          # 用户模型
│   │   └── order.py         # 订单模型
│   └── utils/
│       ├── __init__.py
│       └── helpers.py       # 工具函数
├── config/
│   ├── app.yaml             # 应用配置
│   ├── database.yaml        # 数据库配置
│   └── logging.yaml         # 日志配置
├── tests/
│   ├── __init__.py
│   ├── test_services.py
│   └── test_models.py
├── logs/                    # 日志文件目录
├── requirements.txt         # Python依赖
├── pyproject.toml          # 项目配置
└── README.md               # 项目文档
```

## 最佳实践

### 1. 异步编程

```python
import asyncio
from dmsc import DMSCAppBuilder

async def async_operation(ctx):
    """异步操作示例"""
    # 并发执行多个异步任务
    tasks = [
        fetch_data_from_api(ctx, "api1"),
        fetch_data_from_api(ctx, "api2"),
        fetch_data_from_database(ctx)
    ]
    
    # 等待所有任务完成
    results = await asyncio.gather(*tasks)
    
    return results

async def fetch_data_from_api(ctx, api_name):
    """从API获取数据"""
    ctx.logger.info("api", f"Fetching data from {api_name}")
    # 模拟API调用
    await asyncio.sleep(0.1)
    return {"api": api_name, "data": "sample"}

async def fetch_data_from_database(ctx):
    """从数据库获取数据"""
    ctx.logger.info("database", "Fetching data from database")
    # 模拟数据库查询
    await asyncio.sleep(0.2)
    return {"source": "database", "data": "sample"}
```

### 2. 错误处理

```python
from dmsc import DMSCError, DMSCResult

async def robust_operation(ctx) -> DMSCResult:
    """健壮的错误处理示例"""
    try:
        # 可能失败的操作
        result = await risky_operation(ctx)
        
        # 验证结果
        if not result:
            return DMSCError("empty_result", "操作返回了空结果")
        
        ctx.logger.info("success", "操作成功完成")
        return result
        
    except DMSCError as e:
        # 处理DMSC特定的错误
        ctx.logger.error("dmsc_error", f"DMSC错误: {e}")
        return e
        
    except Exception as e:
        # 处理其他异常
        ctx.logger.error("unexpected", f"意外错误: {e}")
        return DMSCError("unexpected_error", str(e))

async def risky_operation(ctx):
    """可能失败的操作"""
    # 模拟随机失败
    import random
    if random.random() < 0.1:  # 10%失败率
        raise DMSCError("random_failure", "随机失败用于测试")
    
    return {"status": "success", "data": "important_data"}
```

### 3. 配置管理

```python
from dmsc import DMSCConfig

class AppConfig:
    """应用配置类"""
    
    def __init__(self):
        self.config = DMSCConfig()
        self._load_config()
    
    def _load_config(self):
        """加载配置"""
        # 加载默认配置
        self._load_defaults()
        
        # 从文件加载配置（如果存在）
        self._load_from_files()
        
        # 从环境变量加载（覆盖文件配置）
        self._load_from_env()
    
    def _load_defaults(self):
        """加载默认配置"""
        defaults = {
            "app.name": "My DMSC App",
            "app.version": "1.0.0",
            "app.debug": False,
            "server.host": "0.0.0.0",
            "server.port": 8080,
            "database.url": "sqlite:///app.db",
            "cache.ttl": 3600,
            "logging.level": "INFO"
        }
        
        for key, value in defaults.items():
            self.config.set(key, value)
    
    def _load_from_files(self):
        """从配置文件加载"""
        import os
        
        config_files = [
            "config/app.yaml",
            "config/app.json",
            "config.yaml",
            "config.json"
        ]
        
        for file_path in config_files:
            if os.path.exists(file_path):
                self.config.load_file(file_path)
                break
    
    def _load_from_env(self):
        """从环境变量加载"""
        self.config.load_env("MYAPP_")
    
    # 属性访问器
    @property
    def app_name(self):
        return self.config.get("app.name")
    
    @property
    def app_version(self):
        return self.config.get("app.version")
    
    @property
    def debug(self):
        return self.config.get("app.debug", False)
    
    @property
    def server_host(self):
        return self.config.get("server.host", "0.0.0.0")
    
    @property
    def server_port(self):
        return self.config.get("server.port", 8080)
```

## 故障排除

### 常见问题

#### 1. 安装失败
```bash
# 确保pip是最新版本
pip install --upgrade pip

# 如果编译失败，安装构建工具
# Ubuntu/Debian
sudo apt-get install build-essential python3-dev

# macOS
xcode-select --install

# Windows
# 安装Visual Studio Build Tools
```

#### 2. 导入错误
```python
# 确保使用正确的导入
from dmsc import DMSCAppBuilder  # ✅ 正确
import dmsc.DMSCAppBuilder       # ❌ 错误

# 检查Python路径
import sys
print(sys.path)
```

#### 3. 异步问题
```python
# 确保正确使用asyncio
import asyncio

# ✅ 正确方式
async def main():
    # 你的代码
    pass

if __name__ == "__main__":
    asyncio.run(main())

# ❌ 错误方式
# main()  # 这会报错
```

#### 4. 配置加载失败
```python
# 检查配置文件路径
import os
print(f"当前工作目录: {os.getcwd()}")
print(f"配置文件存在: {os.path.exists('config.yaml')}")

# 使用绝对路径
config.load_file("/absolute/path/to/config.yaml")
```

## 性能调优

### 1. 异步优化

```python
import asyncio
from dmsc import DMSCAppBuilder

# 优化连接池
app = DMSCAppBuilder()
app.with_http(
    max_connections=1000,
    keepalive_timeout=30,
    request_timeout=60
)

# 优化缓存
app.with_cache(
    max_size=10000,
    ttl=3600,
    cleanup_interval=300
)
```

### 2. 内存优化

```python
# 使用生成器减少内存占用
async def process_large_dataset(ctx):
    """处理大数据集"""
    async for batch in fetch_data_in_batches(ctx):
        # 处理每个批次
        result = await process_batch(ctx, batch)
        yield result

# 及时清理缓存
async def cleanup_operation(ctx):
    """清理操作"""
    try:
        # 业务逻辑
        result = await do_something(ctx)
        return result
    finally:
        # 确保资源清理
        await ctx.cache.clear_pattern("temp:*")
```

## 下一步

- [核心概念](./03-core-concepts.md) - 深入了解DMSC Python的核心概念
- [API参考](./04-api-reference/README.md) - 查看完整的API文档
- [使用示例](./05-usage-examples/README.md) - 学习更多实际应用示例