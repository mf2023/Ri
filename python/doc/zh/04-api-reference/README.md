<div align="center">

# DMSC Python API 参考

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

详细的模块API文档，包括核心、认证、缓存、配置等模块

</div>

## 目录

- [核心模块 (core)](#核心模块-core)
- [日志模块 (log)](#日志模块-log)
- [配置模块 (config)](./config.md)
- [缓存模块 (cache)](./cache.md)
- [HTTP模块 (http)](./http.md)
- [文件系统模块 (fs)](#文件系统模块-fs)
- [认证模块 (auth)](./auth.md)
- [可观测性模块 (observability)](./observability.md)
- [消息队列模块 (mq)](./mq.md)
- [数据库模块 (database)](./database.md)
- [安全模块 (security)](./security.md)
- [存储模块 (storage)](./storage.md)
- [验证模块 (validation)](./validation.md)

---

## 核心模块 (core)

核心模块提供DMSC Python的基础功能，包括应用构建器、服务上下文、错误处理等。

### DMSCAppBuilder

应用构建器，用于配置和构建DMSC应用。

```python
from dmsc import DMSCAppBuilder

class DMSCAppBuilder:
    """DMSC应用构建器"""
    
    def __init__(self):
        """初始化应用构建器"""
        pass
    
    def with_logging(self, config: DMSCLogConfig) -> 'DMSCAppBuilder':
        """添加日志模块
        
        Args:
            config: 日志配置
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_config(self, config: Union[str, DMSCConfig]) -> 'DMSCAppBuilder':
        """添加配置模块
        
        Args:
            config: 配置对象或配置文件路径
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_cache(self, config: DMSCCacheConfig) -> 'DMSCAppBuilder':
        """添加缓存模块
        
        Args:
            config: 缓存配置
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_http(self, config: DMSCHTTPConfig) -> 'DMSCAppBuilder':
        """添加HTTP模块
        
        Args:
            config: HTTP配置
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_fs(self, config: Optional[DMSCFSConfig] = None) -> 'DMSCAppBuilder':
        """添加文件系统模块
        
        Args:
            config: 文件系统配置（可选）
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_auth(self, config: DMSCAuthConfig) -> 'DMSCAppBuilder':
        """添加认证模块
        
        Args:
            config: 认证配置
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_observability(self, config: DMSCObservabilityConfig) -> 'DMSCAppBuilder':
        """添加可观测性模块
        
        Args:
            config: 可观测性配置
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def with_module(self, module: DMSCModule) -> 'DMSCAppBuilder':
        """添加自定义模块
        
        Args:
            module: 自定义模块实例
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def on_init(self, func: Callable) -> 'DMSCAppBuilder':
        """注册初始化钩子
        
        Args:
            func: 初始化钩子函数
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def on_start(self, func: Callable) -> 'DMSCAppBuilder':
        """注册启动钩子
        
        Args:
            func: 启动钩子函数
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def on_shutdown(self, func: Callable) -> 'DMSCAppBuilder':
        """注册关闭钩子
        
        Args:
            func: 关闭钩子函数
            
        Returns:
            self，支持链式调用
        """
        pass
    
    def build(self) -> DMSCApplication:
        """构建DMSC应用
        
        Returns:
            DMSC应用实例
            
        Raises:
            DMSCError: 构建失败时抛出异常
        """
        pass
```

### DMSCApplication

DMSC应用实例，代表构建完成的应用。

```python
from dmsc import DMSCApplication

class DMSCApplication:
    """DMSC应用"""
    
    async def run_async(self, func: Callable[[DMSCServiceContext], Any]) -> Any:
        """异步运行应用
        
        Args:
            func: 业务逻辑函数，接收服务上下文
            
        Returns:
            业务逻辑函数的返回值
            
        Raises:
            DMSCError: 运行失败时抛出异常
        """
        pass
    
    async def start_async(self) -> None:
        """异步启动应用"""
        pass
    
    async def shutdown_async(self) -> None:
        """异步关闭应用"""
        pass
    
    def get_context(self) -> DMSCServiceContext:
        """获取服务上下文
        
        Returns:
            服务上下文
        """
        pass
```

### DMSCServiceContext

服务上下文，提供对所有模块功能的访问。

```python
from dmsc import DMSCServiceContext

class DMSCServiceContext:
    """服务上下文"""
    
    @property
    def logger(self) -> DMSCLogger:
        """获取日志器
        
        Returns:
            日志器实例
        """
        pass
    
    @property
    def config(self) -> DMSCConfig:
        """获取配置管理器
        
        Returns:
            配置管理器实例
        """
        pass
    
    @property
    def cache(self) -> DMSCCache:
        """获取缓存管理器（如果已启用）
        
        Returns:
            缓存管理器实例
            
        Raises:
            AttributeError: 如果缓存模块未启用
        """
        pass
    
    @property
    def http(self) -> DMSCHTTPClient:
        """获取HTTP客户端（如果已启用）
        
        Returns:
            HTTP客户端实例
            
        Raises:
            AttributeError: 如果HTTP模块未启用
        """
        pass
    
    @property
    def fs(self) -> DMSCFS:
        """获取文件系统管理器（如果已启用）
        
        Returns:
            文件系统管理器实例
            
        Raises:
            AttributeError: 如果文件系统模块未启用
        """
        pass
    
    @property
    def auth(self) -> DMSCAuth:
        """获取认证管理器（如果已启用）
        
        Returns:
            认证管理器实例
            
        Raises:
            AttributeError: 如果认证模块未启用
        """
        pass
    
    @property
    def metrics(self) -> DMSCMetrics:
        """获取指标管理器（如果已启用）
        
        Returns:
            指标管理器实例
            
        Raises:
            AttributeError: 如果可观测性模块未启用
        """
        pass
```

### DMSCError

DMSC错误类型，所有DMSC异常的基类。

```python
from dmsc import DMSCError

class DMSCError(Exception):
    """DMSC错误基类"""
    
    def __init__(self, code: str, message: str, details: dict = None):
        """初始化错误
        
        Args:
            code: 错误代码
            message: 错误消息
            details: 错误详情（可选）
        """
        super().__init__(message)
        self.code = code
        self.message = message
        self.details = details or {}
    
    def to_dict(self) -> dict:
        """转换为字典格式
        
        Returns:
            错误信息的字典表示
        """
        pass
```

---

## 日志模块 (log)

日志模块提供结构化日志功能，支持多种输出格式和目的地。

### DMSCLogConfig

日志配置类。

```python
from dmsc import DMSCLogConfig

class DMSCLogConfig:
    """日志配置"""
    
    def __init__(self, level: str = "INFO", format: str = "json",
                 enable_console: bool = True, enable_file: bool = False,
                 file_path: str = "logs/app.log", max_file_size: str = "100MB",
                 max_files: int = 10, compress: bool = True,
                 enable_trace: bool = True, sampling_rate: float = 1.0):
        """初始化日志配置
        
        Args:
            level: 日志级别 (DEBUG, INFO, WARN, ERROR)
            format: 日志格式 (json, text)
            enable_console: 是否输出到控制台
            enable_file: 是否输出到文件
            file_path: 日志文件路径
            max_file_size: 单个日志文件最大大小
            max_files: 保留的日志文件数量
            compress: 是否压缩旧日志文件
            enable_trace: 是否启用追踪上下文
            sampling_rate: 日志采样率 (0.0-1.0)
        """
        pass
    
    @classmethod
    def default(cls) -> 'DMSCLogConfig':
        """获取默认配置
        
        Returns:
            默认日志配置
        """
        pass
    
    @classmethod
    def development(cls) -> 'DMSCLogConfig':
        """获取开发环境配置
        
        Returns:
            开发环境日志配置
        """
        pass
    
    @classmethod
    def production(cls) -> 'DMSCLogConfig':
        """获取生产环境配置
        
        Returns:
            生产环境日志配置
        """
        pass
```

### DMSCLogger

日志器接口。

```python
from dmsc import DMSCLogger

class DMSCLogger:
    """日志器"""
    
    def debug(self, module: str, message: str, **kwargs) -> None:
        """记录调试日志
        
        Args:
            module: 模块名称
            message: 日志消息
            **kwargs: 额外的日志字段
        """
        pass
    
    def info(self, module: str, message: str, **kwargs) -> None:
        """记录信息日志
        
        Args:
            module: 模块名称
            message: 日志消息
            **kwargs: 额外的日志字段
        """
        pass
    
    def warn(self, module: str, message: str, **kwargs) -> None:
        """记录警告日志
        
        Args:
            module: 模块名称
            message: 日志消息
            **kwargs: 额外的日志字段
        """
        pass
    
    def error(self, module: str, message: str, **kwargs) -> None:
        """记录错误日志
        
        Args:
            module: 模块名称
            message: 日志消息
            **kwargs: 额外的日志字段
        """
        pass
    
    def log(self, level: str, module: str, message: str, **kwargs) -> None:
        """记录日志（通用方法）
        
        Args:
            level: 日志级别
            module: 模块名称
            message: 日志消息
            **kwargs: 额外的日志字段
        """
        pass
```

---

## 配置模块 (config)

配置模块提供多源配置管理，支持热重载和类型转换。

### DMSCConfig

配置管理类。

```python
from dmsc import DMSCConfig

class DMSCConfig:
    """配置管理器"""
    
    def __init__(self):
        """初始化配置管理器"""
        pass
    
    def set(self, key: str, value: Any) -> None:
        """设置配置值
        
        Args:
            key: 配置键（支持点分隔的层级结构）
            value: 配置值
        """
        pass
    
    def get(self, key: str, default: Any = None) -> Any:
        """获取配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            配置值，如果不存在则返回默认值
        """
        pass
    
    def get_int(self, key: str, default: int = 0) -> int:
        """获取整型配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            整型配置值
        """
        pass
    
    def get_float(self, key: str, default: float = 0.0) -> float:
        """获取浮点型配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            浮点型配置值
        """
        pass
    
    def get_bool(self, key: str, default: bool = False) -> bool:
        """获取布尔型配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            布尔型配置值
        """
        pass
    
    def get_string(self, key: str, default: str = "") -> str:
        """获取字符串配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            字符串配置值
        """
        pass
    
    def get_list(self, key: str, default: list = None) -> list:
        """获取列表配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            列表配置值
        """
        pass
    
    def get_dict(self, key: str, default: dict = None) -> dict:
        """获取字典配置值
        
        Args:
            key: 配置键
            default: 默认值
            
        Returns:
            字典配置值
        """
        pass
    
    def has(self, key: str) -> bool:
        """检查配置键是否存在
        
        Args:
            key: 配置键
            
        Returns:
            如果存在返回True，否则返回False
        """
        pass
    
    def remove(self, key: str) -> bool:
        """删除配置键
        
        Args:
            key: 配置键
            
        Returns:
            如果删除成功返回True，否则返回False
        """
        pass
    
    def clear(self) -> None:
        """清空所有配置"""
        pass
    
    def load_file(self, file_path: str) -> None:
        """从文件加载配置
        
        Args:
            file_path: 配置文件路径（支持YAML和JSON格式）
            
        Raises:
            DMSCError: 文件加载失败时抛出异常
        """
        pass
    
    def load_env(self, prefix: str = "") -> None:
        """从环境变量加载配置
        
        Args:
            prefix: 环境变量前缀，只加载以此前缀开头的变量
        """
        pass
    
    def enable_hot_reload(self, enable: bool = True) -> None:
        """启用/禁用配置热重载
        
        Args:
            enable: 是否启用热重载
        """
        pass
    
    def to_dict(self) -> dict:
        """转换为字典格式
        
        Returns:
            配置的字典表示
        """
        pass
```

---

## 缓存模块 (cache)

缓存模块提供统一的缓存抽象，支持多种后端。

### DMSCCacheConfig

缓存配置类。

```python
from dmsc import DMSCCacheConfig, DMSCCacheBackend

class DMSCCacheConfig:
    """缓存配置"""
    
    def __init__(self, backend: DMSCCacheBackend = DMSCCacheBackend.MEMORY,
                 ttl: int = 3600, max_size: int = 1000,
                 redis_url: str = None, max_connections: int = 10,
                 memory_max_size: int = 100):
        """初始化缓存配置
        
        Args:
            backend: 缓存后端类型
            ttl: 默认过期时间（秒）
            max_size: 最大缓存项数
            redis_url: Redis连接URL（使用Redis后端时）
            max_connections: 最大连接数（使用Redis后端时）
            memory_max_size: 内存缓存最大项数（使用混合后端时）
        """
        pass
    
    @classmethod
    def memory_cache(cls, ttl: int = 3600, max_size: int = 1000) -> 'DMSCCacheConfig':
        """创建内存缓存配置
        
        Args:
            ttl: 默认过期时间（秒）
            max_size: 最大缓存项数
            
        Returns:
            内存缓存配置
        """
        pass
    
    @classmethod
    def redis_cache(cls, redis_url: str, ttl: int = 3600,
                    max_connections: int = 10) -> 'DMSCCacheConfig':
        """创建Redis缓存配置
        
        Args:
            redis_url: Redis连接URL
            ttl: 默认过期时间（秒）
            max_connections: 最大连接数
            
        Returns:
            Redis缓存配置
        """
        pass
    
    @classmethod
    def hybrid_cache(cls, redis_url: str, ttl: int = 3600,
                     memory_max_size: int = 100,
                     max_connections: int = 10) -> 'DMSCCacheConfig':
        """创建混合缓存配置
        
        Args:
            redis_url: Redis连接URL
            ttl: 默认过期时间（秒）
            memory_max_size: 内存缓存最大项数
            max_connections: 最大连接数
            
        Returns:
            混合缓存配置
        """
        pass
```

### DMSCCacheBackend

缓存后端枚举。

```python
from dmsc import DMSCCacheBackend

class DMSCCacheBackend(Enum):
    """缓存后端类型"""
    MEMORY = "memory"      # 内存缓存
    REDIS = "redis"        # Redis缓存
    HYBRID = "hybrid"     # 混合缓存（内存+Redis）
```

### DMSCCache

缓存管理器接口。

```python
from dmsc import DMSCCache

class DMSCCache:
    """缓存管理器"""
    
    async def get(self, key: str) -> Optional[Any]:
        """获取缓存值
        
        Args:
            key: 缓存键
            
        Returns:
            缓存值，如果不存在则返回None
        """
        pass
    
    async def set(self, key: str, value: Any, ttl: int = None) -> bool:
        """设置缓存值
        
        Args:
            key: 缓存键
            value: 缓存值
            ttl: 过期时间（秒），如果不指定则使用默认值
            
        Returns:
            如果设置成功返回True，否则返回False
        """
        pass
    
    async def delete(self, key: str) -> bool:
        """删除缓存值
        
        Args:
            key: 缓存键
            
        Returns:
            如果删除成功返回True，否则返回False
        """
        pass
    
    async def exists(self, key: str) -> bool:
        """检查缓存键是否存在
        
        Args:
            key: 缓存键
            
        Returns:
            如果存在返回True，否则返回False
        """
        pass
    
    async def clear(self) -> bool:
        """清空所有缓存
        
        Returns:
            如果清空成功返回True，否则返回False
        """
        pass
    
    async def keys(self, pattern: str = "*") -> List[str]:
        """获取匹配的缓存键列表
        
        Args:
            pattern: 匹配模式（支持通配符）
            
        Returns:
            匹配的缓存键列表
        """
        pass
    
    async def ttl(self, key: str) -> int:
        """获取缓存键的剩余过期时间
        
        Args:
            key: 缓存键
            
        Returns:
            剩余过期时间（秒），如果不存在或永不过期返回-1
        """
        pass
    
    async def expire(self, key: str, ttl: int) -> bool:
        """设置缓存键的过期时间
        
        Args:
            key: 缓存键
            ttl: 过期时间（秒）
            
        Returns:
            如果设置成功返回True，否则返回False
        """
        pass
```

---

## HTTP模块 (http)

HTTP模块提供Web服务功能，包括HTTP客户端和服务器。

### DMSCHTTPConfig

HTTP配置类。

```python
from dmsc import DMSCHTTPConfig

class DMSCHTTPConfig:
    """HTTP配置"""
    
    def __init__(self, host: str = "0.0.0.0", port: int = 8080,
                 workers: int = 4, max_requests: int = 1000,
                 keepalive_timeout: int = 30, request_timeout: int = 60,
                 cors_enabled: bool = True, cors_origins: List[str] = None,
                 rate_limit_enabled: bool = False, rate_limit_requests: int = 100,
                 rate_limit_window: int = 60):
        """初始化HTTP配置
        
        Args:
            host: 服务器监听地址
            port: 服务器监听端口
            workers: 工作进程数
            max_requests: 单个工作进程的最大请求数
            keepalive_timeout: keepalive超时时间（秒）
            request_timeout: 请求超时时间（秒）
            cors_enabled: 是否启用CORS
            cors_origins: 允许的CORS源列表
            rate_limit_enabled: 是否启用限流
            rate_limit_requests: 限流窗口内的最大请求数
            rate_limit_window: 限流窗口时间（秒）
        """
        pass
```

### DMSCHTTPClient

HTTP客户端接口。

```python
from dmsc import DMSCHTTPClient

class DMSCHTTPClient:
    """HTTP客户端"""
    
    async def get(self, url: str, params: dict = None, headers: dict = None,
                  timeout: int = None) -> DMSCHTTPResponse:
        """发送GET请求
        
        Args:
            url: 请求URL
            params: 查询参数
            headers: 请求头
            timeout: 超时时间（秒）
            
        Returns:
            HTTP响应
        """
        pass
    
    async def post(self, url: str, data: Any = None, json: dict = None,
                   headers: dict = None, timeout: int = None) -> DMSCHTTPResponse:
        """发送POST请求
        
        Args:
            url: 请求URL
            data: 请求体数据
            json: JSON格式的请求体
            headers: 请求头
            timeout: 超时时间（秒）
            
        Returns:
            HTTP响应
        """
        pass
    
    async def put(self, url: str, data: Any = None, json: dict = None,
                  headers: dict = None, timeout: int = None) -> DMSCHTTPResponse:
        """发送PUT请求
        
        Args:
            url: 请求URL
            data: 请求体数据
            json: JSON格式的请求体
            headers: 请求头
            timeout: 超时时间（秒）
            
        Returns:
            HTTP响应
        """
        pass
    
    async def delete(self, url: str, headers: dict = None,
                     timeout: int = None) -> DMSCHTTPResponse:
        """发送DELETE请求
        
        Args:
            url: 请求URL
            headers: 请求头
            timeout: 超时时间（秒）
            
        Returns:
            HTTP响应
        """
        pass
    
    async def request(self, method: str, url: str, **kwargs) -> DMSCHTTPResponse:
        """发送通用HTTP请求
        
        Args:
            method: HTTP方法
            url: 请求URL
            **kwargs: 其他请求参数
            
        Returns:
            HTTP响应
        """
        pass
```

### DMSCHTTPResponse

HTTP响应类。

```python
from dmsc import DMSCHTTPResponse

class DMSCHTTPResponse:
    """HTTP响应"""
    
    @property
    def status_code(self) -> int:
        """获取状态码
        
        Returns:
            HTTP状态码
        """
        pass
    
    @property
    def headers(self) -> dict:
        """获取响应头
        
        Returns:
            响应头字典
        """
        pass
    
    @property
    def text(self) -> str:
        """获取响应文本
        
        Returns:
            响应文本内容
        """
        pass
    
    @property
    def content(self) -> bytes:
        """获取响应内容
        
        Returns:
            响应二进制内容
        """
        pass
    
    def json(self) -> dict:
        """解析JSON响应
        
        Returns:
            解析后的JSON数据
            
        Raises:
            ValueError: 如果响应不是有效的JSON
        """
        pass
    
    def is_success(self) -> bool:
        """检查响应是否成功
        
        Returns:
            如果状态码在200-299范围内返回True
        """
        pass
    
    def is_error(self) -> bool:
        """检查响应是否错误
        
        Returns:
            如果状态码表示错误返回True
        """
        pass
```

---

## 文件系统模块 (fs)

文件系统模块提供安全的文件系统操作。

### DMSCFSConfig

文件系统配置类。

```python
from dmsc import DMSCFSConfig

class DMSCFSConfig:
    """文件系统配置"""
    
    def __init__(self, root_path: str = None, max_file_size: int = 100 * 1024 * 1024,
                 allowed_extensions: List[str] = None, 
                 forbidden_paths: List[str] = None):
        """初始化文件系统配置
        
        Args:
            root_path: 根路径，如果不指定则使用项目根目录
            max_file_size: 最大文件大小（字节）
            allowed_extensions: 允许的文件扩展名列表
            forbidden_paths: 禁止访问的路径列表
        """
        pass
```

### DMSCFS

文件系统管理器接口。

```python
from dmsc import DMSCFS

class DMSCFS:
    """文件系统管理器"""
    
    async def read_file(self, file_path: str, encoding: str = "utf-8") -> str:
        """读取文本文件
        
        Args:
            file_path: 文件路径（相对路径）
            encoding: 文件编码
            
        Returns:
            文件内容
            
        Raises:
            DMSCError: 文件读取失败时抛出异常
        """
        pass
    
    async def read_binary(self, file_path: str) -> bytes:
        """读取二进制文件
        
        Args:
            file_path: 文件路径（相对路径）
            
        Returns:
            文件内容
            
        Raises:
            DMSCError: 文件读取失败时抛出异常
        """
        pass
    
    async def write_file(self, file_path: str, content: str,
                         encoding: str = "utf-8") -> None:
        """写入文本文件
        
        Args:
            file_path: 文件路径（相对路径）
            content: 文件内容
            encoding: 文件编码
            
        Raises:
            DMSCError: 文件写入失败时抛出异常
        """
        pass
    
    async def write_binary(self, file_path: str, content: bytes) -> None:
        """写入二进制文件
        
        Args:
            file_path: 文件路径（相对路径）
            content: 文件内容
            
        Raises:
            DMSCError: 文件写入失败时抛出异常
        """
        pass
    
    async def delete_file(self, file_path: str) -> bool:
        """删除文件
        
        Args:
            file_path: 文件路径（相对路径）
            
        Returns:
            如果删除成功返回True，否则返回False
        """
        pass
    
    async def exists(self, file_path: str) -> bool:
        """检查文件是否存在
        
        Args:
            file_path: 文件路径（相对路径）
            
        Returns:
            如果存在返回True，否则返回False
        """
        pass
    
    async def is_file(self, file_path: str) -> bool:
        """检查路径是否为文件
        
        Args:
            file_path: 文件路径（相对路径）
            
        Returns:
            如果是文件返回True，否则返回False
        """
        pass
    
    async def is_dir(self, file_path: str) -> bool:
        """检查路径是否为目录
        
        Args:
            file_path: 文件路径（相对路径）
            
        Returns:
            如果是目录返回True，否则返回False
        """
        pass
    
    async def list_dir(self, dir_path: str = "") -> List[str]:
        """列出目录内容
        
        Args:
            dir_path: 目录路径（相对路径）
            
        Returns:
            目录中的文件和子目录列表
        """
        pass
    
    async def create_dir(self, dir_path: str, parents: bool = False) -> None:
        """创建目录
        
        Args:
            dir_path: 目录路径（相对路径）
            parents: 是否创建父目录
            
        Raises:
            DMSCError: 目录创建失败时抛出异常
        """
        pass
    
    async def remove_dir(self, dir_path: str, recursive: bool = False) -> bool:
        """删除目录
        
        Args:
            dir_path: 目录路径（相对路径）
            recursive: 是否递归删除
            
        Returns:
            如果删除成功返回True，否则返回False
        """
        pass
```

---

## 认证模块 (auth)

认证模块提供身份验证和授权功能。

### DMSCAuthConfig

认证配置类。

```python
from dmsc import DMSCAuthConfig

class DMSCAuthConfig:
    """认证配置"""
    
    def __init__(self, jwt_secret: str = None, jwt_expiry: int = 3600,
                 jwt_algorithm: str = "HS256", oauth_providers: dict = None,
                 enable_permissions: bool = True, rbac_enabled: bool = False):
        """初始化认证配置
        
        Args:
            jwt_secret: JWT密钥
            jwt_expiry: JWT过期时间（秒）
            jwt_algorithm: JWT算法
            oauth_providers: OAuth提供者配置
            enable_permissions: 是否启用权限检查
            rbac_enabled: 是否启用RBAC
        """
        pass
```

### DMSCAuth

认证管理器接口。

```python
from dmsc import DMSCAuth

class DMSCAuth:
    """认证管理器"""
    
    async def authenticate(self, credentials: dict) -> DMSCAuthResult:
        """用户认证
        
        Args:
            credentials: 认证凭据（用户名密码等）
            
        Returns:
            认证结果
        """
        pass
    
    async def verify_token(self, token: str) -> DMSCAuthResult:
        """验证令牌
        
        Args:
            token: 认证令牌
            
        Returns:
            认证结果
        """
        pass
    
    async def generate_token(self, user_id: str, claims: dict = None) -> str:
        """生成认证令牌
        
        Args:
            user_id: 用户ID
            claims: 额外的声明信息
            
        Returns:
            认证令牌
        """
        pass
    
    async def refresh_token(self, token: str) -> str:
        """刷新令牌
        
        Args:
            token: 旧令牌
            
        Returns:
            新令牌
        """
        pass
    
    async def revoke_token(self, token: str) -> bool:
        """吊销令牌
        
        Args:
            token: 要吊销的令牌
            
        Returns:
            如果吊销成功返回True，否则返回False
        """
        pass
    
    async def check_permission(self, user_id: str, permission: str) -> bool:
        """检查用户权限
        
        Args:
            user_id: 用户ID
            permission: 权限标识
            
        Returns:
            如果有权限返回True，否则返回False
        """
        pass
    
    async def get_user_roles(self, user_id: str) -> List[str]:
        """获取用户角色
        
        Args:
            user_id: 用户ID
            
        Returns:
            用户角色列表
        """
        pass
```

### DMSCAuthResult

认证结果类。

```python
from dmsc import DMSCAuthResult

class DMSCAuthResult:
    """认证结果"""
    
    def __init__(self, success: bool, user_id: str = None, token: str = None,
                 expires_at: int = None, claims: dict = None,
                 error: str = None):
        """初始化认证结果
        
        Args:
            success: 认证是否成功
            user_id: 用户ID
            token: 认证令牌
            expires_at: 过期时间戳
            claims: 声明信息
            error: 错误信息（认证失败时）
        """
        pass
    
    @property
    def is_success(self) -> bool:
        """检查认证是否成功
        
        Returns:
            如果成功返回True，否则返回False
        """
        pass
    
    @property
    def is_expired(self) -> bool:
        """检查令牌是否过期
        
        Returns:
            如果过期返回True，否则返回False
        """
        pass
```

---

## 可观测性模块 (observability)

可观测性模块提供指标、追踪和监控功能。

### DMSCObservabilityConfig

可观测性配置类。

```python
from dmsc import DMSCObservabilityConfig

class DMSCObservabilityConfig:
    """可观测性配置"""
    
    def __init__(self, metrics_enabled: bool = True, tracing_enabled: bool = True,
                 prometheus_enabled: bool = True, prometheus_port: int = 9090,
                 jaeger_enabled: bool = False, jaeger_endpoint: str = None,
                 health_check_enabled: bool = True, health_check_path: str = "/health"):
        """初始化可观测性配置
        
        Args:
            metrics_enabled: 是否启用指标收集
            tracing_enabled: 是否启用分布式追踪
            prometheus_enabled: 是否启用Prometheus导出
            prometheus_port: Prometheus端口
            jaeger_enabled: 是否启用Jaeger追踪
            jaeger_endpoint: Jaeger端点
            health_check_enabled: 是否启用健康检查
            health_check_path: 健康检查路径
        """
        pass
```

### DMSCMetrics

指标管理器接口。

```python
from dmsc import DMSCMetrics

class DMSCMetrics:
    """指标管理器"""
    
    def counter(self, name: str, description: str = "", labels: dict = None) -> DMSCMetricCounter:
        """创建计数器指标
        
        Args:
            name: 指标名称
            description: 指标描述
            labels: 标签
            
        Returns:
            计数器实例
        """
        pass
    
    def gauge(self, name: str, description: str = "", labels: dict = None) -> DMSCMetricGauge:
        """创建计量器指标
        
        Args:
            name: 指标名称
            description: 指标描述
            labels: 标签
            
        Returns:
            计量器实例
        """
        pass
    
    def histogram(self, name: str, description: str = "", buckets: List[float] = None,
                  labels: dict = None) -> DMSCMetricHistogram:
        """创建直方图指标
        
        Args:
            name: 指标名称
            description: 指标描述
            buckets: 桶边界
            labels: 标签
            
        Returns:
            直方图实例
        """
        pass
    
    def summary(self, name: str, description: str = "", 
                labels: dict = None) -> DMSCMetricSummary:
        """创建摘要指标
        
        Args:
            name: 指标名称
            description: 指标描述
            labels: 标签
            
        Returns:
            摘要实例
        """
        pass
```

### DMSCMetricCounter

计数器指标接口。

```python
from dmsc import DMSCMetricCounter

class DMSCMetricCounter:
    """计数器指标"""
    
    def inc(self, amount: float = 1, labels: dict = None) -> None:
        """增加计数
        
        Args:
            amount: 增加的数量
            labels: 标签值
        """
        pass
    
    def get(self, labels: dict = None) -> float:
        """获取当前值
        
        Args:
            labels: 标签值
            
        Returns:
            当前计数值
        """
        pass
```

### DMSCMetricGauge

计量器指标接口。

```python
from dmsc import DMSCMetricGauge

class DMSCMetricGauge:
    """计量器指标"""
    
    def set(self, value: float, labels: dict = None) -> None:
        """设置值
        
        Args:
            value: 要设置的值
            labels: 标签值
        """
        pass
    
    def inc(self, amount: float = 1, labels: dict = None) -> None:
        """增加值
        
        Args:
            amount: 增加的数量
            labels: 标签值
        """
        pass
    
    def dec(self, amount: float = 1, labels: dict = None) -> None:
        """减少值
        
        Args:
            amount: 减少的数量
            labels: 标签值
        """
        pass
    
    def get(self, labels: dict = None) -> float:
        """获取当前值
        
        Args:
            labels: 标签值
            
        Returns:
            当前值
        """
        pass
```

### DMSCMetricHistogram

直方图指标接口。

```python
from dmsc import DMSCMetricHistogram

class DMSCMetricHistogram:
    """直方图指标"""
    
    def observe(self, value: float, labels: dict = None) -> None:
        """观察值
        
        Args:
            value: 要观察的值
            labels: 标签值
        """
        pass
    
    def get_count(self, labels: dict = None) -> int:
        """获取观察次数
        
        Args:
            labels: 标签值
            
        Returns:
            观察次数
        """
        pass
    
    def get_sum(self, labels: dict = None) -> float:
        """获取值的总和
        
        Args:
            labels: 标签值
            
        Returns:
            值的总和
        """
        pass
```

### DMSCMetricSummary

摘要指标接口。

```python
from dmsc import DMSCMetricSummary

class DMSCMetricSummary:
    """摘要指标"""
    
    def observe(self, value: float, labels: dict = None) -> None:
        """观察值
        
        Args:
            value: 要观察的值
            labels: 标签值
        """
        pass
    
    def get_count(self, labels: dict = None) -> int:
        """获取观察次数
        
        Args:
            labels: 标签值
            
        Returns:
            观察次数
        """
        pass
    
    def get_sum(self, labels: dict = None) -> float:
        """获取值的总和
        
        Args:
            labels: 标签值
            
        Returns:
            值的总和
        """
        pass
```

<div align="center">

## 下一步

</div>

- [使用示例](./05-usage-examples/README.md) - 查看各种功能的使用示例
- [最佳实践](./06-best-practices.md) - 了解开发最佳实践
- [故障排除](./07-troubleshooting.md) - 常见问题和解决方案