<div align="center">

# 可观测性 API参考

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

observability模块提供全面的监控、追踪和日志功能，帮助开发者了解系统运行状态。

## 模块概述

</div>

observability模块包含以下子模块：

- **metrics**: 指标收集和导出
- **tracing**: 分布式追踪
- **logging**: 结构化日志
- **health**: 健康检查
- **profiling**: 性能分析
- **alerting**: 告警系统
- **dashboard**: 监控仪表板
- **anomaly**: 异常检测

<div align="center">

## 核心组件

</div>

### DMSCObservabilityConfig

可观测性配置类，用于配置监控和追踪行为。

#### 构造函数

```python
DMSCObservabilityConfig(
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    enable_logging: bool = True,
    enable_profiling: bool = False,
    metrics_backend: str = "prometheus",
    metrics_port: int = 9090,
    metrics_path: str = "/metrics",
    metrics_interval: int = 15,
    tracing_backend: str = "jaeger",
    tracing_endpoint: str = "http://localhost:14268/api/traces",
    tracing_sample_rate: float = 1.0,
    tracing_service_name: str = "dmsc-python",
    logging_backend: str = "json",
    logging_level: str = "INFO",
    logging_format: str = "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    logging_file: str = "",
    logging_max_size: int = 104857600,  # 100MB
    logging_backup_count: int = 5,
    profiling_backend: str = "py-spy",
    profiling_interval: int = 60,
    enable_health_checks: bool = True,
    health_check_port: int = 8080,
    health_check_path: str = "/health",
    enable_alerting: bool = True,
    alerting_backend: str = "webhook",
    alerting_webhook_url: str = "",
    alerting_rules: List[Dict] = None,
    enable_dashboard: bool = True,
    dashboard_port: int = 3000,
    enable_anomaly_detection: bool = False,
    anomaly_detection_threshold: float = 0.95,
    enable_performance_monitoring: bool = True,
    performance_monitoring_interval: int = 30,
    enable_error_tracking: bool = True,
    error_tracking_backend: str = "sentry",
    error_tracking_dsn: str = "",
    enable_uptime_monitoring: bool = True,
    uptime_check_interval: int = 60,
    enable_resource_monitoring: bool = True,
    resource_monitoring_interval: int = 10,
    enable_custom_metrics: bool = True,
    custom_metrics_definitions: List[Dict] = None
)
```

### DMSCObservabilityManager

可观测性管理器，提供统一的监控接口。

<div align="center">

#### 方法表

</div>

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start()` | 启动可观测性服务 | 无 | `bool` |
| `stop()` | 停止可观测性服务 | 无 | `bool` |
| `record_metric(name, value, labels=None, timestamp=None)` | 记录指标 | `name: str`, `value: float`, `labels: Optional[Dict]`, `timestamp: Optional[float]` | `bool` |
| `increment_counter(name, value=1, labels=None)` | 增加计数器 | `name: str`, `value: float`, `labels: Optional[Dict]` | `bool` |
| `record_histogram(name, value, labels=None, buckets=None)` | 记录直方图 | `name: str`, `value: float`, `labels: Optional[Dict]`, `buckets: Optional[List[float]]` | `bool` |
| `start_span(name, **kwargs)` | 开始追踪 | `name: str`, `**kwargs` | `object` |
| `end_span(span)` | 结束追踪 | `span: object` | `bool` |
| `inject_context(headers)` | 注入追踪上下文 | `headers: Dict` | `bool` |
| `extract_context(headers)` | 提取追踪上下文 | `headers: Dict` | `Dict` |
| `log(level, message, **kwargs)` | 记录日志 | `level: str`, `message: str`, `**kwargs` | `bool` |
| `debug(message, **kwargs)` | 记录调试日志 | `message: str`, `**kwargs` | `bool` |
| `info(message, **kwargs)` | 记录信息日志 | `message: str`, `**kwargs` | `bool` |
| `warning(message, **kwargs)` | 记录警告日志 | `message: str`, `**kwargs` | `bool` |
| `error(message, **kwargs)` | 记录错误日志 | `message: str`, `**kwargs` | `bool` |
| `critical(message, **kwargs)` | 记录严重错误日志 | `message: str`, `**kwargs` | `bool` |
| `exception(message, **kwargs)` | 记录异常日志 | `message: str`, `**kwargs` | `bool` |
| `add_health_check(name, check_func)` | 添加健康检查 | `name: str`, `check_func: Callable` | `bool` |
| `remove_health_check(name)` | 移除健康检查 | `name: str` | `bool` |
| `get_health_status()` | 获取健康状态 | 无 | `Dict` |
| `record_event(event_type, data, timestamp=None)` | 记录事件 | `event_type: str`, `data: Dict`, `timestamp: Optional[float]` | `bool` |
| `create_alert(name, condition, actions, enabled=True)` | 创建告警 | `name: str`, `condition: Callable`, `actions: List[Callable]`, `enabled: bool` | `bool` |
| `enable_alert(name)` | 启用告警 | `name: str` | `bool` |
| `disable_alert(name)` | 禁用告警 | `name: str` | `bool` |
| `get_metrics(name=None)` | 获取指标 | `name: Optional[str]` | `Dict` |
| `export_metrics(format="json")` | 导出指标 | `format: str` | `str` |

### DMSCMetrics

指标管理器，处理性能指标收集。

#### 构造函数

```python
DMSCMetrics(
    backend: str = "prometheus",
    port: int = 9090,
    path: str = "/metrics",
    interval: int = 15,
    labels: Dict = None
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `counter(name, description, labels=None)` | 创建计数器 | `name: str`, `description: str`, `labels: Optional[List[str]]` | `object` |
| `gauge(name, description, labels=None)` | 创建仪表盘 | `name: str`, `description: str`, `labels: Optional[List[str]]` | `object` |
| `histogram(name, description, buckets=None, labels=None)` | 创建直方图 | `name: str`, `description: str`, `buckets: Optional[List[float]]`, `labels: Optional[List[str]]` | `object` |
| `summary(name, description, objectives=None, labels=None)` | 创建摘要 | `name: str`, `description: str`, `objectives: Optional[List[float]]`, `labels: Optional[List[str]]` | `object` |
| `record_metric(metric, value, labels=None)` | 记录指标 | `metric: object`, `value: float`, `labels: Optional[Dict]` | `bool` |
| `start_collection()` | 开始收集 | 无 | `bool` |
| `stop_collection()` | 停止收集 | 无 | `bool` |
| `get_metrics()` | 获取所有指标 | 无 | `Dict` |
| `export(format="json")` | 导出指标 | `format: str` | `str` |

### DMSCTracing

分布式追踪管理器，处理请求追踪。

#### 构造函数

```python
DMSCTracing(
    backend: str = "jaeger",
    endpoint: str = "http://localhost:14268/api/traces",
    service_name: str = "dmsc-python",
    sample_rate: float = 1.0,
    tags: Dict = None
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_span(name, parent=None, **kwargs)` | 开始span | `name: str`, `parent: Optional[object]`, `**kwargs` | `object` |
| `finish_span(span)` | 结束span | `span: object` | `bool` |
| `set_tag(span, key, value)` | 设置标签 | `span: object`, `key: str`, `value: Any` | `bool` |
| `log_event(span, event, payload=None)` | 记录事件 | `span: object`, `event: str`, `payload: Optional[Dict]` | `bool` |
| `inject_context(carrier)` | 注入上下文 | `carrier: Dict` | `bool` |
| `extract_context(carrier)` | 提取上下文 | `carrier: Dict` | `Dict` |
| `get_trace_id(span)` | 获取追踪ID | `span: object` | `str` |
| `get_span_id(span)` | 获取span ID | `span: object` | `str` |
| `baggage_set(span, key, value)` | 设置baggage | `span: object`, `key: str`, `value: str` | `bool` |
| `baggage_get(span, key)` | 获取baggage | `span: object`, `key: str` | `str` |

### DMSCLogger

日志管理器，处理日志记录。

#### 构造函数

```python
DMSCLogger(
    name: str = "dmsc",
    level: str = "INFO",
    format: str = "%(asctime)s - %(name)s - %(levelname)s - %(message)s",
    backend: str = "json",
    file: str = "",
    max_size: int = 104857600,
    backup_count: int = 5,
    enable_console: bool = True,
    enable_file: bool = False,
    enable_json: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `debug(message, **kwargs)` | 记录调试日志 | `message: str`, `**kwargs` | `bool` |
| `info(message, **kwargs)` | 记录信息日志 | `message: str`, `**kwargs` | `bool` |
| `warning(message, **kwargs)` | 记录警告日志 | `message: str`, `**kwargs` | `bool` |
| `error(message, **kwargs)` | 记录错误日志 | `message: str`, `**kwargs` | `bool` |
| `critical(message, **kwargs)` | 记录严重错误日志 | `message: str`, `**kwargs` | `bool` |
| `exception(message, **kwargs)` | 记录异常日志 | `message: str`, `**kwargs` | `bool` |
| `log(level, message, **kwargs)` | 记录日志 | `level: str`, `message: str`, `**kwargs` | `bool` |
| `add_filter(filter_func)` | 添加过滤器 | `filter_func: Callable` | `bool` |
| `remove_filter(filter_func)` | 移除过滤器 | `filter_func: Callable` | `bool` |
| `add_handler(handler)` | 添加处理器 | `handler: object` | `bool` |
| `remove_handler(handler)` | 移除处理器 | `handler: object` | `bool` |
| `set_level(level)` | 设置日志级别 | `level: str` | `bool` |

### DMSCHealthCheck

健康检查管理器，监控系统健康状态。

#### 构造函数

```python
DMSCHealthCheck(
    port: int = 8080,
    path: str = "/health",
    timeout: int = 30,
    enable_detailed: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_check(name, check_func, timeout=None)` | 添加健康检查 | `name: str`, `check_func: Callable`, `timeout: Optional[int]` | `bool` |
| `remove_check(name)` | 移除健康检查 | `name: str` | `bool` |
| `run_checks()` | 运行所有检查 | 无 | `Dict` |
| `get_status()` | 获取健康状态 | 无 | `Dict` |
| `start_server()` | 启动健康检查服务器 | 无 | `bool` |
| `stop_server()` | 停止健康检查服务器 | 无 | `bool` |

### DMSCAlerting

告警管理器，处理系统告警。

#### 构造函数

```python
DMSCAlerting(
    backend: str = "webhook",
    webhook_url: str = "",
    rules: List[Dict] = None,
    cooldown_period: int = 300,
    max_alerts_per_period: int = 10
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `create_alert(name, condition, actions, enabled=True)` | 创建告警 | `name: str`, `condition: Callable`, `actions: List[Callable]`, `enabled: bool` | `bool` |
| `enable_alert(name)` | 启用告警 | `name: str` | `bool` |
| `disable_alert(name)` | 禁用告警 | `name: str` | `bool` |
| `remove_alert(name)` | 移除告警 | `name: str` | `bool` |
| `check_alerts()` | 检查告警条件 | 无 | `List[Dict]` |
| `send_alert(alert, data)` | 发送告警 | `alert: Dict`, `data: Dict` | `bool` |
| `get_alert_history(name=None)` | 获取告警历史 | `name: Optional[str]` | `List[Dict]` |

### DMSCProfiling

性能分析器，分析系统性能。

#### 构造函数

```python
DMSCProfiling(
    backend: str = "py-spy",
    interval: int = 60,
    duration: int = 300,
    output_format: str = "flamegraph"
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_profiling()` | 开始性能分析 | 无 | `bool` |
| `stop_profiling()` | 停止性能分析 | 无 | `bool` |
| `get_profile()` | 获取性能分析结果 | 无 | `Dict` |
| `export_profile(format="json")` | 导出性能分析结果 | `format: str` | `str` |
| `analyze_bottlenecks()` | 分析性能瓶颈 | 无 | `List[Dict]` |

### DMSCErrorTracking

错误追踪管理器，跟踪和分析错误。

#### 构造函数

```python
DMSCErrorTracking(
    backend: str = "sentry",
    dsn: str = "",
    environment: str = "production",
    release: str = "",
    sample_rate: float = 1.0
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `capture_exception(exception, **kwargs)` | 捕获异常 | `exception: Exception`, `**kwargs` | `str` |
| `capture_message(message, level="error", **kwargs)` | 捕获消息 | `message: str`, `level: str`, `**kwargs` | `str` |
| `set_context(key, data)` | 设置上下文 | `key: str`, `data: Dict` | `bool` |
| `set_user(user_data)` | 设置用户信息 | `user_data: Dict` | `bool` |
| `set_tag(key, value)` | 设置标签 | `key: str`, `value: str` | `bool` |
| `clear_context()` | 清空上下文 | 无 | `bool` |
| `get_errors(limit=100)` | 获取错误列表 | `limit: int` | `List[Dict]` |

### DMSCResourceMonitoring

资源监控器，监控系统资源使用情况。

#### 构造函数

```python
DMSCResourceMonitoring(
    interval: int = 10,
    enable_cpu: bool = True,
    enable_memory: bool = True,
    enable_disk: bool = True,
    enable_network: bool = True,
    enable_process: bool = True
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_monitoring()` | 开始监控 | 无 | `bool` |
| `stop_monitoring()` | 停止监控 | 无 | `bool` |
| `get_cpu_stats()` | 获取CPU统计 | 无 | `Dict` |
| `get_memory_stats()` | 获取内存统计 | 无 | `Dict` |
| `get_disk_stats()` | 获取磁盘统计 | 无 | `Dict` |
| `get_network_stats()` | 获取网络统计 | 无 | `Dict` |
| `get_process_stats()` | 获取进程统计 | 无 | `Dict` |
| `get_all_stats()` | 获取所有统计 | 无 | `Dict` |

### DMSCEventTracking

事件追踪器，追踪业务事件。

#### 构造函数

```python
DMSCEventTracking(
    backend: str = "memory",
    retention_days: int = 30,
    batch_size: int = 100,
    flush_interval: int = 60
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `track_event(event_type, user_id, properties=None, timestamp=None)` | 追踪事件 | `event_type: str`, `user_id: str`, `properties: Optional[Dict]`, `timestamp: Optional[float]` | `bool` |
| `track_page_view(user_id, page, properties=None, timestamp=None)` | 追踪页面访问 | `user_id: str`, `page: str`, `properties: Optional[Dict]`, `timestamp: Optional[float]` | `bool` |
| `track_exception(user_id, exception, properties=None, timestamp=None)` | 追踪异常 | `user_id: str`, `exception: Exception`, `properties: Optional[Dict]`, `timestamp: Optional[float]` | `bool` |
| `get_events(event_type=None, user_id=None, start_time=None, end_time=None, limit=100)` | 获取事件 | `event_type: Optional[str]`, `user_id: Optional[str]`, `start_time: Optional[float]`, `end_time: Optional[float]`, `limit: int` | `List[Dict]` |
| `get_event_stats(event_type=None, start_time=None, end_time=None)` | 获取事件统计 | `event_type: Optional[str]`, `start_time: Optional[float]`, `end_time: Optional[float]` | `Dict` |
| `flush_events()` | 刷新事件 | 无 | `bool` |

### DMSCDashboard

监控仪表板，可视化系统状态。

#### 构造函数

```python
DMSCDashboard(
    port: int = 3000,
    enable_metrics: bool = True,
    enable_traces: bool = True,
    enable_logs: bool = True,
    enable_alerts: bool = True,
    refresh_interval: int = 30
)
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_dashboard()` | 启动仪表板 | 无 | `bool` |
| `stop_dashboard()` | 停止仪表板 | 无 | `bool` |
| `add_widget(widget_type, config)` | 添加组件 | `widget_type: str`, `config: Dict` | `bool` |
| `remove_widget(widget_id)` | 移除组件 | `widget_id: str` | `bool` |
| `update_widget(widget_id, config)` | 更新组件 | `widget_id: str`, `config: Dict` | `bool` |
| `get_dashboard_config()` | 获取仪表板配置 | 无 | `Dict` |

<div align="center">

## 使用示例

</div>

### 基本使用

```python
from dmsc.observability import DMSCObservabilityManager, DMSCObservabilityConfig

# 创建可观测性配置
config = DMSCObservabilityConfig(
    enable_metrics=True,
    enable_tracing=True,
    enable_logging=True,
    metrics_backend="prometheus",
    tracing_backend="jaeger",
    logging_backend="json"
)

# 创建可观测性管理器
obs_manager = DMSCObservabilityManager(config)

# 启动可观测性服务
obs_manager.start()

# 记录指标
obs_manager.record_metric("requests_total", 1, labels={"method": "GET", "status": "200"})

# 记录日志
obs_manager.info("Application started", extra={"version": "1.0.0"})

# 开始追踪
span = obs_manager.start_span("process_request", tags={"user_id": "12345"})

# 结束追踪
obs_manager.end_span(span)
```

### 指标收集

```python
from dmsc.observability import DMSCMetrics

# 创建指标管理器
metrics = DMSCMetrics(
    backend="prometheus",
    port=9090,
    labels={"service": "api", "environment": "production"}
)

# 创建计数器
request_counter = metrics.counter(
    "http_requests_total",
    "Total HTTP requests",
    labels=["method", "endpoint", "status"]
)

# 创建直方图
response_time_histogram = metrics.histogram(
    "http_response_time_seconds",
    "HTTP response time",
    buckets=[0.1, 0.5, 1.0, 2.0, 5.0],
    labels=["method", "endpoint"]
)

# 记录指标
request_counter.inc(labels={"method": "GET", "endpoint": "/api/users", "status": "200"})
response_time_histogram.observe(0.25, labels={"method": "GET", "endpoint": "/api/users"})
```

### 分布式追踪

```python
from dmsc.observability import DMSCTracing

# 创建追踪管理器
tracing = DMSCTracing(
    backend="jaeger",
    endpoint="http://localhost:14268/api/traces",
    service_name="user-service",
    sample_rate=0.1
)

# 开始根span
root_span = tracing.start_span("process_user_request", tags={"user_id": "12345"})

# 开始子span
db_span = tracing.start_span("database_query", parent=root_span)

# 执行数据库操作
user_data = db.query("SELECT * FROM users WHERE id = %s", (user_id,))

# 设置span标签
tracing.set_tag(db_span, "query", "SELECT * FROM users")
tracing.set_tag(db_span, "rows_returned", len(user_data))

# 结束子span
tracing.finish_span(db_span)

# 开始另一个子span
cache_span = tracing.start_span("cache_operation", parent=root_span)

# 执行缓存操作
cache.set(f"user:{user_id}", user_data)

# 结束子span
tracing.finish_span(cache_span)

# 结束根span
tracing.finish_span(root_span)
```

### 结构化日志

```python
from dmsc.observability import DMSCLogger

# 创建日志管理器
logger = DMSCLogger(
    name="user-service",
    level="INFO",
    backend="json",
    enable_console=True,
    enable_file=True
)

# 记录结构化日志
logger.info(
    "User login successful",
    extra={
        "user_id": "12345",
        "username": "john_doe",
        "ip_address": "192.168.1.1",
        "user_agent": "Mozilla/5.0",
        "login_method": "password",
        "duration_ms": 150
    }
)

# 记录异常日志
try:
    result = risky_operation()
except Exception as e:
    logger.exception(
        "Risky operation failed",
        extra={
            "operation": "data_processing",
            "input_data": input_data,
            "error_code": "PROCESSING_ERROR"
        }
    )
```

### 健康检查

```python
from dmsc.observability import DMSCHealthCheck

# 创建健康检查管理器
health_check = DMSCHealthCheck(
    port=8080,
    path="/health",
    enable_detailed=True
)

# 添加数据库健康检查
def check_database():
    try:
        db.execute("SELECT 1")
        return True, "Database is healthy"
    except Exception as e:
        return False, f"Database error: {str(e)}"

health_check.add_check("database", check_database, timeout=5)

# 添加缓存健康检查
def check_cache():
    try:
        cache.set("health_check", "ok", ex=1)
        value = cache.get("health_check")
        return value == "ok", "Cache is healthy"
    except Exception as e:
        return False, f"Cache error: {str(e)}"

health_check.add_check("cache", check_cache, timeout=3)

# 添加外部服务健康检查
def check_external_service():
    try:
        response = requests.get("https://api.external.com/health", timeout=10)
        return response.status_code == 200, "External service is healthy"
    except Exception as e:
        return False, f"External service error: {str(e)}"

health_check.add_check("external_service", check_external_service, timeout=10)

# 启动健康检查服务器
health_check.start_server()

# 获取健康状态
status = health_check.get_status()
print(f"Overall status: {status['status']}")
for check_name, check_result in status['checks'].items():
    print(f"{check_name}: {check_result['status']} - {check_result['message']}")
```

### 告警系统

```python
from dmsc.observability import DMSCAlerting

# 创建告警管理器
alerting = DMSCAlerting(
    backend="webhook",
    webhook_url="https://alerts.example.com/webhook",
    cooldown_period=300,  # 5分钟冷却期
    max_alerts_per_period=10
)

# 创建高错误率告警
def high_error_rate_condition():
    error_rate = metrics.get_metric("error_rate_5m")
    return error_rate > 0.05  # 错误率超过5%

def send_high_error_rate_alert():
    webhook_client.send({
        "alert_type": "high_error_rate",
        "severity": "warning",
        "message": "Error rate is above 5%",
        "value": metrics.get_metric("error_rate_5m"),
        "timestamp": time.time()
    })

alerting.create_alert(
    "high_error_rate",
    high_error_rate_condition,
    [send_high_error_rate_alert]
)

# 创建高响应时间告警
def high_response_time_condition():
    p95_response_time = metrics.get_metric("response_time_p95_5m")
    return p95_response_time > 2.0  # P95响应时间超过2秒

def send_high_response_time_alert():
    webhook_client.send({
        "alert_type": "high_response_time",
        "severity": "warning",
        "message": "P95 response time is above 2 seconds",
        "value": metrics.get_metric("response_time_p95_5m"),
        "timestamp": time.time()
    })

alerting.create_alert(
    "high_response_time",
    high_response_time_condition,
    [send_high_response_time_alert]
)

# 创建服务不可用告警
def service_unavailable_condition():
    return health_check.get_status()["status"] == "unhealthy"

def send_service_unavailable_alert():
    webhook_client.send({
        "alert_type": "service_unavailable",
        "severity": "critical",
        "message": "Service is unhealthy",
        "timestamp": time.time()
    })

alerting.create_alert(
    "service_unavailable",
    service_unavailable_condition,
    [send_service_unavailable_alert]
)
```

### 性能分析

```python
from dmsc.observability import DMSCProfiling

# 创建性能分析器
profiling = DMSCProfiling(
    backend="py-spy",
    interval=60,  # 每60秒采样一次
    duration=300,  # 持续5分钟
    output_format="flamegraph"
)

# 开始性能分析
profiling.start_profiling()

# 运行一段时间后停止分析
time.sleep(300)
profiling.stop_profiling()

# 获取分析结果
profile_data = profiling.get_profile()
print(f"CPU usage: {profile_data['cpu_percent']}%")
print(f"Memory usage: {profile_data['memory_mb']} MB")

# 分析性能瓶颈
bottlenecks = profiling.analyze_bottlenecks()
for bottleneck in bottlenecks:
    print(f"Function: {bottleneck['function']}")
    print(f"CPU time: {bottleneck['cpu_time']}%")
    print(f"Call count: {bottleneck['call_count']}")
```

### 错误追踪

```python
from dmsc.observability import DMSCErrorTracking

# 创建错误追踪管理器
error_tracking = DMSCErrorTracking(
    backend="sentry",
    dsn="https://public@sentry.example.com/1",
    environment="production",
    release="1.0.0",
    sample_rate=1.0
)

# 设置全局用户信息
error_tracking.set_user({
    "id": "12345",
    "username": "john_doe",
    "email": "john@example.com"
})

# 设置全局上下文
error_tracking.set_context("app", {
    "version": "1.0.0",
    "build": "12345",
    "feature_flags": ["new_ui", "beta_features"]
})

# 设置全局标签
error_tracking.set_tag("environment", "production")
error_tracking.set_tag("region", "us-east-1")

# 捕获异常
try:
    result = risky_operation()
except Exception as e:
    error_id = error_tracking.capture_exception(e, extra={
        "operation": "data_processing",
        "input_size": len(input_data),
        "attempt": 3
    })
    print(f"Error tracked with ID: {error_id}")

# 捕获消息
error_tracking.capture_message(
    "User encountered unexpected behavior",
    level="warning",
    extra={
        "user_action": "clicked_submit_button",
        "page_url": "/checkout",
        "browser": "Chrome 96.0"
    }
)

# 获取最近的错误
recent_errors = error_tracking.get_errors(limit=10)
for error in recent_errors:
    print(f"Error: {error['message']}")
    print(f"Count: {error['count']}")
    print(f"Last seen: {error['last_seen']}")
```

### 资源监控

```python
from dmsc.observability import DMSCResourceMonitoring

# 创建资源监控器
resource_monitor = DMSCResourceMonitoring(
    interval=10,  # 每10秒检查一次
    enable_cpu=True,
    enable_memory=True,
    enable_disk=True,
    enable_network=True,
    enable_process=True
)

# 开始监控
resource_monitor.start_monitoring()

# 获取CPU统计
cpu_stats = resource_monitor.get_cpu_stats()
print(f"CPU usage: {cpu_stats['percent']}%")
print(f"CPU count: {cpu_stats['count']}")
print(f"Load average: {cpu_stats['load_average']}")

# 获取内存统计
memory_stats = resource_monitor.get_memory_stats()
print(f"Memory usage: {memory_stats['percent']}%")
print(f"Memory used: {memory_stats['used_mb']} MB")
print(f"Memory available: {memory_stats['available_mb']} MB")

# 获取磁盘统计
disk_stats = resource_monitor.get_disk_stats()
for disk, stats in disk_stats.items():
    print(f"Disk {disk}: {stats['percent']}% used")
    print(f"  Total: {stats['total_gb']} GB")
    print(f"  Used: {stats['used_gb']} GB")
    print(f"  Free: {stats['free_gb']} GB")

# 获取网络统计
network_stats = resource_monitor.get_network_stats()
print(f"Network bytes sent: {network_stats['bytes_sent']}")
print(f"Network bytes received: {network_stats['bytes_recv']}")
print(f"Network packets sent: {network_stats['packets_sent']}")
print(f"Network packets received: {network_stats['packets_recv']}")

# 获取进程统计
process_stats = resource_monitor.get_process_stats()
print(f"Process count: {process_stats['count']}")
print(f"Thread count: {process_stats['thread_count']}")
print(f"File descriptor count: {process_stats['fd_count']}")
```

### 事件追踪

```python
from dmsc.observability import DMSCEventTracking

# 创建事件追踪器
event_tracker = DMSCEventTracking(
    backend="memory",
    retention_days=30,
    batch_size=100,
    flush_interval=60
)

# 追踪用户事件
event_tracker.track_event(
    "user_signup",
    "user_12345",
    properties={
        "signup_method": "email",
        "referrer": "google",
        "campaign": "summer_promotion"
    }
)

# 追踪页面访问
event_tracker.track_page_view(
    "user_12345",
    "/products/123",
    properties={
        "product_category": "electronics",
        "product_price": 999.99,
        "time_on_page": 45.2
    }
)

# 追踪异常事件
try:
    process_payment(user_id, amount)
except PaymentError as e:
    event_tracker.track_exception(
        "user_12345",
        e,
        properties={
            "payment_method": "credit_card",
            "amount": 99.99,
            "currency": "USD"
        }
    )

# 获取事件统计
event_stats = event_tracker.get_event_stats("user_signup", start_time=time.time()-86400)
print(f"Signups in last 24h: {event_stats['count']}")
print(f"Unique users: {event_stats['unique_users']}")
print(f"Average per hour: {event_stats['avg_per_hour']}")

# 获取具体事件
events = event_tracker.get_events(
    event_type="user_signup",
    user_id="user_12345",
    start_time=time.time()-604800,  # 最近7天
    limit=50
)
for event in events:
    print(f"Event: {event['type']} at {event['timestamp']}")
    print(f"Properties: {event['properties']}")
```

### 监控仪表板

```python
from dmsc.observability import DMSCDashboard

# 创建监控仪表板
dashboard = DMSCDashboard(
    port=3000,
    enable_metrics=True,
    enable_traces=True,
    enable_logs=True,
    enable_alerts=True,
    refresh_interval=30
)

# 添加CPU使用率组件
dashboard.add_widget("line_chart", {
    "title": "CPU Usage",
    "metric": "cpu_usage_percent",
    "time_range": "1h",
    "thresholds": [70, 85, 95]
})

# 添加内存使用率组件
dashboard.add_widget("gauge", {
    "title": "Memory Usage",
    "metric": "memory_usage_percent",
    "max_value": 100,
    "thresholds": [80, 90, 95]
})

# 添加请求速率组件
dashboard.add_widget("counter", {
    "title": "Request Rate",
    "metric": "http_requests_per_second",
    "time_range": "5m"
})

# 添加错误率组件
dashboard.add_widget("heatmap", {
    "title": "Error Rate by Endpoint",
    "metric": "error_rate_by_endpoint",
    "time_range": "1h"
})

# 添加追踪组件
dashboard.add_widget("trace_timeline", {
    "title": "Recent Traces",
    "service": "user-service",
    "limit": 10
})

# 添加日志组件
dashboard.add_widget("log_stream", {
    "title": "Error Logs",
    "level": "error",
    "time_range": "15m",
    "max_entries": 100
})

# 添加告警组件
dashboard.add_widget("alert_panel", {
    "title": "Active Alerts",
    "severity_filter": ["warning", "critical"]
})

# 启动仪表板
dashboard.start_dashboard()
```

<div align="center">

## 最佳实践

</div>

### 指标设计

1. **选择合适的指标类型**: 根据监控需求选择计数器、仪表盘、直方图或摘要
2. **合理设置标签**: 使用标签进行维度分析，但避免标签值过多
3. **命名规范**: 使用清晰、一致的指标命名规范
4. **采样率**: 根据系统负载合理设置追踪采样率
5. **指标聚合**: 对高频指标进行适当的聚合处理

### 追踪设计

1. **合理划分span**: 根据业务逻辑和性能关键点划分span
2. **设置有意义的标签**: 在span中设置有助于问题诊断的标签
3. **记录关键事件**: 在关键业务点记录事件信息
4. ** baggage使用**: 谨慎使用baggage传递上下文信息
5. **错误处理**: 在span中正确记录错误信息

### 日志设计

1. **结构化日志**: 使用JSON等结构化格式记录日志
2. **日志级别**: 合理使用不同的日志级别
3. **上下文信息**: 在日志中包含足够的上下文信息
4. **日志采样**: 对高频日志进行采样处理
5. **日志轮转**: 配置合理的日志轮转策略

### 告警设计

1. **告警分级**: 根据严重程度对告警进行分级
2. **告警收敛**: 避免告警风暴，设置合理的收敛规则
3. **告警恢复**: 配置告警恢复通知
4. **告警测试**: 定期测试告警机制的有效性
5. **告警文档**: 为每个告警提供详细的处理文档

### 性能优化

1. **异步处理**: 使用异步方式处理监控数据
2. **批量发送**: 对监控数据进行批量处理
3. **采样策略**: 根据系统负载调整采样策略
4. **缓存机制**: 对频繁访问的监控数据进行缓存
5. **资源限制**: 为监控组件设置合理的资源限制

## 注意事项

1. **监控开销**: 注意监控系统本身对应用性能的影响
2. **数据存储**: 合理规划监控数据的存储和清理策略
3. **隐私保护**: 确保监控数据不包含敏感信息
4. **网络带宽**: 考虑监控数据传输对网络带宽的影响
5. **高可用性**: 确保监控系统本身的高可用性
6. **版本兼容**: 注意监控组件的版本兼容性
7. **配置管理**: 合理管理监控配置，避免配置错误
8. **定期维护**: 定期检查和维护监控系统