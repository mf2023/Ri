<div align="center">

# 可观测性使用示例

**Version: 1.0.0**

**Last modified date: 2025-12-12**

本示例展示如何使用DMSC的observability模块进行分布式追踪、指标收集、健康检查、性能分析和告警管理的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 分布式追踪和span管理
- 指标收集和性能监控
- 健康检查和状态监控
- 日志聚合和分析
- 告警管理和通知
- 性能分析和优化

<div align="center">

## 前置要求

</div>

- Python 3.8+
- pip 21.0+
- 基本的Python编程知识
- 了解可观测性基本概念
- （可选）Jaeger、Prometheus等监控工具

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-observability-example
cd dms-observability-example
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate
```

### 2. 安装依赖

创建`requirements.txt`文件：

```txt
dms>=1.0.0
aiohttp>=3.8.0
asyncio-mqtt>=0.11.0
prometheus-client>=0.15.0
opentelemetry-api>=1.15.0
opentelemetry-sdk>=1.15.0
opentelemetry-instrumentation>=0.36b0
opentelemetry-exporter-jaeger>=1.15.0
```

安装依赖：

```bash
pip install -r requirements.txt
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-observability-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

observability:
  tracing:
    enabled: true
    service_name: "dms-observability-example"
    sampling_rate: 1.0
    jaeger_endpoint: "http://localhost:14268/api/traces"
  metrics:
    enabled: true
    prometheus_endpoint: "0.0.0.0:9090"
    collect_interval: 15
  health_check:
    enabled: true
    endpoint: "/health"
    interval: 30
  alerting:
    enabled: true
    webhook_url: "http://localhost:8080/alerts"
    thresholds:
      error_rate: 0.05
      response_time: 1000
      cpu_usage: 0.8
      memory_usage: 0.85
```

### 4. 编写主代码

创建`main.py`文件：

```python
import asyncio
import json
import time
import random
from datetime import datetime
from typing import Dict, Any, Optional
from dms import DMSCApp, DMSCConfig, DMSCLogConfig
from dms.observability import (
    DMSCTracingConfig, DMSCMetricsConfig, DMSCHealthConfig,
    TracingSpan, MetricCounter, MetricHistogram, MetricGauge
)

async def main():
    """主函数"""
    # 构建服务运行时
    app = DMSCApp(
        config=DMSCConfig.from_yaml("config.yaml"),
        log_config=DMSCLogConfig(),
        tracing_config=DMSCTracingConfig(),
        metrics_config=DMSCMetricsConfig(),
        health_config=DMSCHealthConfig()
    )
    
    # 运行业务逻辑
    await app.run(initialize_and_demo)

async def initialize_and_demo(app: DMSCApp):
    """初始化和演示可观测性功能"""
    logger = app.logger
    logger.info("DMSC Observability Example started")
    
    # 初始化可观测性
    await initialize_observability(app)
    
    # 演示分布式追踪
    await demonstrate_distributed_tracing(app)
    
    # 演示指标收集
    await demonstrate_metrics_collection(app)
    
    # 演示健康检查
    await demonstrate_health_checks(app)
    
    # 演示告警管理
    await demonstrate_alerting(app)
    
    logger.info("DMSC Observability Example completed")

async def initialize_observability(app: DMSCApp):
    """初始化可观测性"""
    logger = app.logger
    tracer = app.tracer
    
    # 创建根span
    with tracer.start_span("initialize_observability") as span:
        span.set_attributes({
            "service.name": "dms-observability-example",
            "service.version": "1.0.0",
            "timestamp": datetime.now().isoformat(),
            "environment": "development"
        })
        
        logger.info("Initializing observability components")
        
        # 初始化追踪
        await initialize_tracing(app, span)
        
        # 初始化指标
        await initialize_metrics(app, span)
        
        # 初始化健康检查
        await initialize_health_checks(app, span)
        
        span.set_status("OK")
        logger.info("Observability initialization completed")

async def initialize_tracing(app: DMSCApp, parent_span: TracingSpan):
    """初始化分布式追踪"""
    logger = app.logger
    tracer = app.tracer
    
    with tracer.start_span("initialize_tracing", parent=parent_span) as span:
        span.set_attributes({
            "tracing.enabled": True,
            "tracing.service_name": "dms-observability-example",
            "tracing.sampling_rate": 1.0,
            "tracing.jaeger_endpoint": "http://localhost:14268/api/traces"
        })
        
        # 配置Jaeger导出器
        await app.tracer.configure_jaeger_exporter(
            service_name="dms-observability-example",
            endpoint="http://localhost:14268/api/traces"
        )
        
        logger.info("Tracing initialized successfully")
        span.set_status("OK")

async def initialize_metrics(app: DMSCApp, parent_span: TracingSpan):
    """初始化指标收集"""
    logger = app.logger
    
    with app.tracer.start_span("initialize_metrics", parent=parent_span) as span:
        span.set_attributes({
            "metrics.enabled": True,
            "metrics.prometheus_endpoint": "0.0.0.0:9090",
            "metrics.collect_interval": 15
        })
        
        # 创建指标收集器
        app.metrics.create_counter(
            name="request_count",
            description="Total number of requests",
            labels=["method", "endpoint", "status"]
        )
        
        app.metrics.create_histogram(
            name="request_duration",
            description="Request duration in milliseconds",
            buckets=[50, 100, 200, 500, 1000, 2000, 5000],
            labels=["method", "endpoint"]
        )
        
        app.metrics.create_gauge(
            name="active_connections",
            description="Number of active connections",
            labels=["service"]
        )
        
        # 启动Prometheus导出器
        await app.metrics.start_prometheus_exporter("0.0.0.0:9090")
        
        logger.info("Metrics initialized successfully")
        span.set_status("OK")

async def initialize_health_checks(app: DMSCApp, parent_span: TracingSpan):
    """初始化健康检查"""
    logger = app.logger
    
    with app.tracer.start_span("initialize_health_checks", parent=parent_span) as span:
        span.set_attributes({
            "health_check.enabled": True,
            "health_check.endpoint": "/health",
            "health_check.interval": 30
        })
        
        # 注册健康检查
        app.health.register_check("database", check_database_health)
        app.health.register_check("cache", check_cache_health)
        app.health.register_check("external_api", check_external_api_health)
        
        logger.info("Health checks initialized successfully")
        span.set_status("OK")

async def demonstrate_distributed_tracing(app: DMSCApp):
    """演示分布式追踪"""
    logger = app.logger
    tracer = app.tracer
    
    logger.info("=== 分布式追踪演示 ===")
    
    # 创建根span
    with tracer.start_span("user_registration_flow") as root_span:
        root_span.set_attributes({
            "flow.type": "user_registration",
            "user.id": "user_12345",
            "registration.source": "web"
        })
        
        # 模拟用户注册流程
        await validate_user_input(app, root_span)
        await check_user_exists(app, root_span)
        await create_user_record(app, root_span)
        await send_welcome_email(app, root_span)
        await update_user_metrics(app, root_span)
        
        root_span.set_status("OK")
        logger.info("User registration flow completed")

async def validate_user_input(app: DMSCApp, parent_span: TracingSpan):
    """验证用户输入"""
    logger = app.logger
    tracer = app.tracer
    
    with tracer.start_span("validate_user_input", parent=parent_span) as span:
        span.set_attributes({
            "validation.fields": ["email", "password", "username"],
            "validation.rules": ["email_format", "password_strength", "username_length"]
        })
        
        # 模拟验证过程
        await asyncio.sleep(0.1)
        
        # 模拟验证结果
        validation_result = {
            "email": True,
            "password": True,
            "username": True
        }
        
        span.set_attributes({
            "validation.result": validation_result,
            "validation.duration_ms": 100
        })
        
        logger.info("User input validation completed")
        span.set_status("OK")

async def check_user_exists(app: DMSCApp, parent_span: TracingSpan):
    """检查用户是否存在"""
    logger = app.logger
    tracer = app.tracer
    
    with tracer.start_span("check_user_exists", parent=parent_span) as span:
        span.set_attributes({
            "database.query": "SELECT id FROM users WHERE email = ?",
            "database.table": "users",
            "database.operation": "select"
        })
        
        # 模拟数据库查询
        await asyncio.sleep(0.2)
        
        # 模拟查询结果
        user_exists = False
        
        span.set_attributes({
            "user.exists": user_exists,
            "database.query_time_ms": 200
        })
        
        logger.info(f"User existence check completed: exists={user_exists}")
        span.set_status("OK")

async def create_user_record(app: DMSCApp, parent_span: TracingSpan):
    """创建用户记录"""
    logger = app.logger
    tracer = app.tracer
    
    with tracer.start_span("create_user_record", parent=parent_span) as span:
        span.set_attributes({
            "database.query": "INSERT INTO users (email, username, password_hash) VALUES (?, ?, ?)",
            "database.table": "users",
            "database.operation": "insert"
        })
        
        # 模拟数据库插入
        await asyncio.sleep(0.3)
        
        # 模拟插入结果
        user_id = "user_12345"
        
        span.set_attributes({
            "user.id": user_id,
            "database.insert_time_ms": 300
        })
        
        logger.info(f"User record created: user_id={user_id}")
        span.set_status("OK")

async def send_welcome_email(app: DMSCApp, parent_span: TracingSpan):
    """发送欢迎邮件"""
    logger = app.logger
    tracer = app.tracer
    
    with tracer.start_span("send_welcome_email", parent=parent_span) as span:
        span.set_attributes({
            "email.template": "welcome_email",
            "email.recipient": "user@example.com",
            "email.priority": "normal"
        })
        
        # 模拟邮件发送
        await asyncio.sleep(0.5)
        
        # 模拟发送结果
        email_sent = True
        
        span.set_attributes({
            "email.sent": email_sent,
            "email.send_time_ms": 500
        })
        
        logger.info(f"Welcome email sent: success={email_sent}")
        span.set_status("OK")

async def update_user_metrics(app: DMSCApp, parent_span: TracingSpan):
    """更新用户指标"""
    logger = app.logger
    
    # 记录指标
    app.metrics.increment_counter(
        "user_registration_count",
        labels={"source": "web", "status": "success"}
    )
    
    app.metrics.record_histogram(
        "user_registration_duration",
        value=1200,  # 总耗时1.2秒
        labels={"source": "web"}
    )
    
    logger.info("User registration metrics updated")

async def demonstrate_metrics_collection(app: DMSCApp):
    """演示指标收集"""
    logger = app.logger
    
    logger.info("=== 指标收集演示 ===")
    
    # 创建各种指标
    await create_business_metrics(app)
    await create_system_metrics(app)
    await create_custom_metrics(app)
    
    logger.info("Metrics collection demonstration completed")

async def create_business_metrics(app: DMSCApp):
    """创建业务指标"""
    logger = app.logger
    
    # 用户注册指标
    registration_counter = app.metrics.get_counter("user_registration_count")
    registration_histogram = app.metrics.get_histogram("user_registration_duration")
    
    # 订单处理指标
    order_counter = app.metrics.create_counter(
        name="order_processed_count",
        description="Total number of processed orders",
        labels=["status", "payment_method"]
    )
    
    order_value_histogram = app.metrics.create_histogram(
        name="order_value_distribution",
        description="Distribution of order values",
        buckets=[10, 50, 100, 500, 1000, 5000],
        labels=["category"]
    )
    
    # 模拟业务数据
    for i in range(10):
        # 用户注册
        registration_counter.increment(labels={"source": "web", "status": "success"})
        registration_histogram.record(random.randint(800, 1500), labels={"source": "web"})
        
        # 订单处理
        status = random.choice(["success", "failed", "pending"])
        payment_method = random.choice(["credit_card", "paypal", "bank_transfer"])
        order_value = random.randint(50, 2000)
        
        order_counter.increment(labels={"status": status, "payment_method": payment_method})
        order_value_histogram.record(order_value, labels={"category": "electronics"})
        
        await asyncio.sleep(0.1)
    
    logger.info("Business metrics created and populated")

async def create_system_metrics(app: DMSCApp):
    """创建系统指标"""
    logger = app.logger
    
    # CPU使用率
    cpu_gauge = app.metrics.create_gauge(
        name="system_cpu_usage",
        description="Current CPU usage percentage",
        labels=["core"]
    )
    
    # 内存使用率
    memory_gauge = app.metrics.create_gauge(
        name="system_memory_usage",
        description="Current memory usage percentage",
        labels=["type"]
    )
    
    # 磁盘使用率
    disk_gauge = app.metrics.create_gauge(
        name="system_disk_usage",
        description="Current disk usage percentage",
        labels=["device"]
    )
    
    # 网络指标
    network_counter = app.metrics.create_counter(
        name="network_bytes_total",
        description="Total network bytes transmitted",
        labels=["interface", "direction"]
    )
    
    # 模拟系统数据
    for i in range(5):
        # CPU使用率
        cpu_usage = random.uniform(0.2, 0.8)
        cpu_gauge.set(cpu_usage, labels={"core": "cpu0"})
        
        # 内存使用率
        memory_usage = random.uniform(0.4, 0.9)
        memory_gauge.set(memory_usage, labels={"type": "used"})
        
        # 磁盘使用率
        disk_usage = random.uniform(0.3, 0.7)
        disk_gauge.set(disk_usage, labels={"device": "/dev/sda1"})
        
        # 网络流量
        network_bytes = random.randint(1000, 10000)
        network_counter.increment(network_bytes, labels={"interface": "eth0", "direction": "tx"})
        network_counter.increment(network_bytes * 2, labels={"interface": "eth0", "direction": "rx"})
        
        await asyncio.sleep(1)
    
    logger.info("System metrics created and populated")

async def create_custom_metrics(app: DMSCApp):
    """创建自定义指标"""
    logger = app.logger
    
    # API响应时间
    api_histogram = app.metrics.create_histogram(
        name="api_response_time_ms",
        description="API response time in milliseconds",
        buckets=[10, 50, 100, 200, 500, 1000],
        labels=["endpoint", "method"]
    )
    
    # 数据库查询次数
    db_counter = app.metrics.create_counter(
        name="database_query_count",
        description="Total number of database queries",
        labels=["table", "operation"]
    )
    
    # 缓存命中率
    cache_gauge = app.metrics.create_gauge(
        name="cache_hit_ratio",
        description="Cache hit ratio percentage",
        labels=["cache_name"]
    )
    
    # 活跃用户数
    active_users_gauge = app.metrics.create_gauge(
        name="active_users_count",
        description="Number of active users",
        labels=["region"]
    )
    
    # 模拟自定义数据
    endpoints = ["/api/users", "/api/orders", "/api/products"]
    methods = ["GET", "POST", "PUT", "DELETE"]
    
    for i in range(20):
        # API响应时间
        endpoint = random.choice(endpoints)
        method = random.choice(methods)
        response_time = random.randint(20, 800)
        api_histogram.record(response_time, labels={"endpoint": endpoint, "method": method})
        
        # 数据库查询
        table = random.choice(["users", "orders", "products"])
        operation = random.choice(["select", "insert", "update", "delete"])
        db_counter.increment(labels={"table": table, "operation": operation})
        
        # 缓存命中率
        hit_ratio = random.uniform(0.6, 0.95)
        cache_gauge.set(hit_ratio, labels={"cache_name": "user_cache"})
        
        # 活跃用户数
        active_users = random.randint(100, 1000)
        active_users_gauge.set(active_users, labels={"region": "us-east"})
        
        await asyncio.sleep(0.1)
    
    logger.info("Custom metrics created and populated")

async def demonstrate_health_checks(app: DMSCApp):
    """演示健康检查"""
    logger = app.logger
    
    logger.info("=== 健康检查演示 ===")
    
    # 执行健康检查
    health_status = await app.health.run_checks()
    
    logger.info(f"Health check results: {health_status}")
    
    # 模拟服务状态变化
    await simulate_service_health_changes(app)
    
    logger.info("Health check demonstration completed")

async def simulate_service_health_changes(app: DMSCApp):
    """模拟服务健康状态变化"""
    logger = app.logger
    
    # 初始状态：健康
    logger.info("Initial health status: healthy")
    
    # 模拟数据库连接问题
    logger.info("Simulating database connection issues...")
    await asyncio.sleep(2)
    
    # 模拟缓存服务问题
    logger.info("Simulating cache service issues...")
    await asyncio.sleep(2)
    
    # 模拟外部API问题
    logger.info("Simulating external API issues...")
    await asyncio.sleep(2)
    
    # 恢复服务
    logger.info("Services recovered")

async def check_database_health() -> Dict[str, Any]:
    """检查数据库健康状态"""
    # 模拟数据库健康检查
    try:
        # 模拟数据库连接测试
        await asyncio.sleep(0.1)
        
        return {
            "status": "healthy",
            "latency_ms": 50,
            "connections": 10,
            "max_connections": 100
        }
    except Exception as e:
        return {
            "status": "unhealthy",
            "error": str(e)
        }

async def check_cache_health() -> Dict[str, Any]:
    """检查缓存健康状态"""
    # 模拟缓存健康检查
    try:
        # 模拟缓存连接测试
        await asyncio.sleep(0.05)
        
        return {
            "status": "healthy",
            "hit_ratio": 0.85,
            "memory_usage": 0.6,
            "keys_count": 1000
        }
    except Exception as e:
        return {
            "status": "unhealthy",
            "error": str(e)
        }

async def check_external_api_health() -> Dict[str, Any]:
    """检查外部API健康状态"""
    # 模拟外部API健康检查
    try:
        # 模拟API调用
        await asyncio.sleep(0.2)
        
        return {
            "status": "healthy",
            "response_time_ms": 200,
            "status_code": 200,
            "availability": 0.99
        }
    except Exception as e:
        return {
            "status": "unhealthy",
            "error": str(e)
        }

async def demonstrate_alerting(app: DMSCApp):
    """演示告警管理"""
    logger = app.logger
    
    logger.info("=== 告警管理演示 ===")
    
    # 创建告警规则
    await create_alert_rules(app)
    
    # 模拟触发告警
    await simulate_alert_conditions(app)
    
    # 处理告警通知
    await handle_alert_notifications(app)
    
    logger.info("Alerting demonstration completed")

async def create_alert_rules(app: DMSCApp):
    """创建告警规则"""
    logger = app.logger
    
    # 错误率告警
    app.alerts.create_rule(
        name="high_error_rate",
        condition="error_rate > 0.05",
        severity="critical",
        description="Error rate exceeds 5%",
        actions=["webhook", "email"]
    )
    
    # 响应时间告警
    app.alerts.create_rule(
        name="high_response_time",
        condition="response_time > 1000",
        severity="warning",
        description="Average response time exceeds 1 second",
        actions=["webhook"]
    )
    
    # CPU使用率告警
    app.alerts.create_rule(
        name="high_cpu_usage",
        condition="cpu_usage > 0.8",
        severity="warning",
        description="CPU usage exceeds 80%",
        actions=["webhook", "email"]
    )
    
    # 内存使用率告警
    app.alerts.create_rule(
        name="high_memory_usage",
        condition="memory_usage > 0.85",
        severity="critical",
        description="Memory usage exceeds 85%",
        actions=["webhook", "email", "sms"]
    )
    
    logger.info("Alert rules created successfully")

async def simulate_alert_conditions(app: DMSCApp):
    """模拟告警条件"""
    logger = app.logger
    
    # 模拟高错误率
    logger.info("Simulating high error rate condition...")
    for i in range(10):
        if random.random() < 0.7:  # 70%错误率
            app.metrics.increment_counter(
                "request_errors",
                labels={"endpoint": "/api/users", "method": "GET", "error_type": "timeout"}
            )
        
        app.metrics.increment_counter(
            "request_count",
            labels={"endpoint": "/api/users", "method": "GET", "status": "error"}
        )
        
        await asyncio.sleep(0.1)
    
    # 模拟高响应时间
    logger.info("Simulating high response time condition...")
    for i in range(5):
        response_time = random.randint(1200, 2000)  # 1.2-2秒响应时间
        app.metrics.record_histogram(
            "api_response_time_ms",
            value=response_time,
            labels={"endpoint": "/api/users", "method": "GET"}
        )
        
        await asyncio.sleep(0.2)
    
    # 模拟高资源使用率
    logger.info("Simulating high resource usage condition...")
    
    # CPU使用率
    cpu_usage = random.uniform(0.85, 0.95)
    app.metrics.set_gauge("system_cpu_usage", cpu_usage, labels={"core": "cpu0"})
    
    # 内存使用率
    memory_usage = random.uniform(0.88, 0.95)
    app.metrics.set_gauge("system_memory_usage", memory_usage, labels={"type": "used"})
    
    logger.info("Alert conditions simulated")

async def handle_alert_notifications(app: DMSCApp):
    """处理告警通知"""
    logger = app.logger
    
    # 检查告警状态
    alerts = await app.alerts.check_all_rules()
    
    for alert in alerts:
        if alert["triggered"]:
            logger.warning(
                f"Alert triggered: {alert['name']}",
                alert_name=alert["name"],
                severity=alert["severity"],
                condition=alert["condition"],
                current_value=alert["current_value"],
                description=alert["description"]
            )
            
            # 发送通知
            await send_alert_notification(app, alert)

def send_alert_notification(app: DMSCApp, alert: Dict[str, Any]):
    """发送告警通知"""
    logger = app.logger
    
    notification_payload = {
        "alert_name": alert["name"],
        "severity": alert["severity"],
        "description": alert["description"],
        "current_value": alert["current_value"],
        "threshold": alert["threshold"],
        "timestamp": datetime.now().isoformat(),
        "service": "dms-observability-example"
    }
    
    # 发送到webhook
    if "webhook" in alert["actions"]:
        logger.info(f"Sending webhook notification for alert: {alert['name']}")
        # 实际实现中会发送HTTP请求到webhook URL
    
    # 发送邮件
    if "email" in alert["actions"]:
        logger.info(f"Sending email notification for alert: {alert['name']}")
        # 实际实现中会发送邮件
    
    # 发送短信
    if "sms" in alert["actions"]:
        logger.info(f"Sending SMS notification for alert: {alert['name']}")
        # 实际实现中会发送短信

if __name__ == "__main__":
    asyncio.run(main())
```

### 5. 运行示例

```bash
python main.py
```

### 6. 查看结果

运行示例后，你可以：

1. **查看追踪数据**：访问 Jaeger UI (http://localhost:16686)
2. **查看指标数据**：访问 Prometheus UI (http://localhost:9090)
3. **查看健康检查**：访问 http://localhost:8080/health
4. **查看日志输出**：控制台会显示详细的日志信息

<div align="center">

## 最佳实践

</div>

### 分布式追踪

1. **合理设置span粒度**：既不要太细也不要太粗
2. **添加有意义的属性**：包含业务相关的关键信息
3. **设置错误状态**：及时标记失败的操作
4. **使用span层级**：反映真实的调用链关系

### 指标收集

1. **选择合适的指标类型**：
   - Counter：只增不减的计数器
   - Histogram：分布统计
   - Gauge：可增可减的仪表盘
   - Summary：摘要统计

2. **设置合理的标签**：便于后续聚合分析
3. **定义清晰的bucket**：对于histogram类型
4. **定期清理过期指标**：避免内存泄漏

### 健康检查

1. **检查关键依赖**：数据库、缓存、外部服务
2. **设置合理的超时时间**：避免阻塞
3. **区分健康等级**：healthy、degraded、unhealthy
4. **提供详细信息**：便于问题定位

### 告警管理

1. **设置合理的阈值**：避免误报和漏报
2. **分级告警策略**：warning、critical等级
3. **告警去重和抑制**：减少噪音
4. **自动化恢复**：自动处理已知问题

<div align="center">

## 故障排查

</div>

### 追踪数据丢失

1. **检查采样率设置**：确保采样率不为0
2. **验证导出器配置**：确认Jaeger/Prometheus地址正确
3. **查看网络连接**：确保网络可达
4. **检查span上报**：确认span成功发送

### 指标数据异常

1. **验证指标定义**：检查指标名称和标签
2. **查看数据类型**：确认使用正确的指标类型
3. **检查标签值**：避免标签值过多
4. **验证聚合逻辑**：确认查询语句正确

### 健康检查失败

1. **检查依赖服务**：确认外部服务可用
2. **验证检查逻辑**：检查健康检查实现
3. **查看超时设置**：避免检查超时
4. **检查资源限制**：确认系统资源充足

<div align="center">

## 性能优化

</div>

### 追踪性能优化

1. **使用采样**：在高流量场景下使用采样
2. **异步上报**：避免阻塞主流程
3. **批量处理**：批量发送span数据
4. **合理设置span属性**：避免过多属性

### 指标性能优化

1. **预定义标签值**：避免动态标签值
2. **定期聚合**：减少存储压力
3. **使用缓存**：缓存频繁查询的指标
4. **清理过期数据**：定期清理历史数据

### 健康检查优化

1. **缓存检查结果**：避免频繁检查
2. **并行检查**：并行执行多个检查
3. **分级检查**：轻量级和重量级检查分离
4. **智能降级**：在故障时智能降级

<div align="center">

## 相关参考

</div>

- [DMSC 可观测性API参考](../04-api-reference/observability.md)
- [OpenTelemetry 官方文档](https://opentelemetry.io/docs/)
- [Prometheus 最佳实践](https://prometheus.io/docs/practices/)
- [Jaeger 架构指南](https://www.jaegertracing.io/docs/architecture/)
- [分布式追踪最佳实践](https://www.oreilly.com/library/view/distributed-systems-observability/9781492033431/)