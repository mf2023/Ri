<div align="center">

# 缓存使用示例

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

本示例展示如何使用DMSC Python的cache模块进行多种缓存后端和高级缓存功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC Python应用，实现以下功能：

- 内存缓存和Redis缓存的使用
- 缓存标签和原子操作
- 分布式锁实现
- 缓存健康检查和监控
- 数据序列化与压缩
- 错误处理和降级策略

<div align="center">

## 前置要求

</div>

- Python 3.8+
- pip 20.0+
- 基本的Python编程知识
- 了解缓存基本概念
- （可选）Redis服务器用于Redis缓存示例

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-cache-example
cd dms-cache-example
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate
```

### 2. 添加依赖

创建`requirements.txt`文件：

```txt
dmsc>=1.0.0
redis>=4.0.0
msgpack>=1.0.0
```

安装依赖：

```bash
pip install -r requirements.txt
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-cache-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

cache:
  backend: "memory"  # 或 "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
  memory:
    max_size: 1000
    ttl: 3600
```

### 4. 编写主代码

创建`main.py`文件：

```python
import asyncio
import time
import json
from dmsc import DMSCAppBuilder, DMSCCacheConfig, DMSCCacheBackend

async def main():
    """主函数"""
    # 构建服务运行时
    app = DMSCAppBuilder()
    
    # 配置内存缓存
    cache_config = DMSCCacheConfig.memory_cache(
        ttl=300,  # 5分钟过期
        max_size=100
    )
    
    # 或使用Redis缓存
    # cache_config = DMSCCacheConfig.redis_cache(
    #     redis_url="redis://localhost:6379/0",
    #     ttl=3600,  # 1小时过期
    #     max_connections=10
    # )
    
    # 构建应用
    dms_app = (app
               .with_cache(cache_config)
               .with_config("config.yaml")
               .build())
    
    # 定义业务逻辑
    async def business_logic(ctx):
        """业务逻辑函数"""
        ctx.logger.info("cache_demo", "=== 缓存使用示例开始 ===")
        
        # 1. 基本缓存操作
        await basic_cache_operations(ctx)
        
        # 2. 缓存标签使用
        await cache_tags_example(ctx)
        
        # 3. 原子操作
        await atomic_operations(ctx)
        
        # 4. 分布式锁
        await distributed_lock_example(ctx)
        
        # 5. 缓存装饰器
        await cache_decorator_example(ctx)
        
        # 6. 错误处理和降级
        await error_handling_and_fallback(ctx)
        
        ctx.logger.info("cache_demo", "=== 缓存使用示例完成 ===")
        return "缓存示例执行成功"
    
    # 运行应用
    result = await dms_app.run_async(business_logic)
    print(f"结果: {result}")

async def basic_cache_operations(ctx):
    """基本缓存操作"""
    ctx.logger.info("cache_demo", "--- 基本缓存操作 ---")
    
    # 设置缓存
    await ctx.cache.set("user:123", {"name": "Alice", "age": 30})
    await ctx.cache.set("config:theme", "dark", ttl=3600)  # 1小时过期
    
    # 获取缓存
    user_data = await ctx.cache.get("user:123")
    if user_data:
        ctx.logger.info("cache_demo", f"缓存用户: {user_data}")
    
    # 检查缓存是否存在
    exists = await ctx.cache.exists("user:123")
    ctx.logger.info("cache_demo", f"缓存存在: {exists}")
    
    # 获取剩余过期时间
    ttl = await ctx.cache.ttl("user:123")
    ctx.logger.info("cache_demo", f"缓存TTL: {ttl}秒")
    
    # 删除缓存
    deleted = await ctx.cache.delete("user:123")
    ctx.logger.info("cache_demo", f"缓存删除: {deleted}")

async def cache_tags_example(ctx):
    """缓存标签使用"""
    ctx.logger.info("cache_demo", "--- 缓存标签使用 ---")
    
    # 设置带标签的缓存
    await ctx.cache.set("article:1", {"title": "DMSC介绍", "content": "..."}, tags=["article", "tech"])
    await ctx.cache.set("article:2", {"title": "Python指南", "content": "..."}, tags=["article", "python"])
    await ctx.cache.set("user:profile:1", {"name": "Bob", "bio": "..."}, tags=["user", "profile"])
    
    # 通过标签查找
    article_keys = await ctx.cache.find_by_tags(["article"])
    ctx.logger.info("cache_demo", f"文章键: {article_keys}")
    
    # 通过标签删除
    deleted_count = await ctx.cache.delete_by_tags(["article"])
    ctx.logger.info("cache_demo", f"通过标签删除数量: {deleted_count}")

async def atomic_operations(ctx):
    """原子操作"""
    ctx.logger.info("cache_demo", "--- 原子操作 ---")
    
    # 原子递增
    await ctx.cache.set("counter", 0)
    new_value = await ctx.cache.increment("counter", 1)
    ctx.logger.info("cache_demo", f"递增后值: {new_value}")
    
    # 原子递减
    new_value = await ctx.cache.decrement("counter", 1)
    ctx.logger.info("cache_demo", f"递减后值: {new_value}")
    
    # 比较并设置 (CAS)
    success = await ctx.cache.cas("counter", 0, 100)
    ctx.logger.info("cache_demo", f"CAS操作成功: {success}")
    
    # 获取并设置
    old_value = await ctx.cache.get_set("counter", 50)
    ctx.logger.info("cache_demo", f"旧值: {old_value}, 新值: 50")

async def distributed_lock_example(ctx):
    """分布式锁示例"""
    ctx.logger.info("cache_demo", "--- 分布式锁示例 ---")
    
    lock_key = "resource_lock:123"
    lock_timeout = 30  # 30秒
    
    # 获取锁
    lock_acquired = await ctx.cache.acquire_lock(lock_key, timeout=lock_timeout)
    ctx.logger.info("cache_demo", f"锁获取成功: {lock_acquired}")
    
    if lock_acquired:
        try:
            # 模拟临界区操作
            ctx.logger.info("cache_demo", "执行临界区操作...")
            await asyncio.sleep(2)  # 模拟耗时操作
            
            # 更新共享资源
            current_value = await ctx.cache.get("shared_resource") or 0
            await ctx.cache.set("shared_resource", current_value + 1)
            
            ctx.logger.info("cache_demo", "临界区操作完成")
            
        finally:
            # 释放锁
            lock_released = await ctx.cache.release_lock(lock_key)
            ctx.logger.info("cache_demo", f"锁释放成功: {lock_released}")
    else:
        ctx.logger.warning("cache_demo", "无法获取锁，跳过操作")

async def cache_decorator_example(ctx):
    """缓存装饰器示例"""
    ctx.logger.info("cache_demo", "--- 缓存装饰器示例 ---")
    
    def cache_result(cache_key: str, ttl: int = None):
        """缓存装饰器"""
        def decorator(func):
            async def wrapper(ctx_inner, *args, **kwargs):
                import hashlib
                # 生成缓存键
                args_str = str(args) + str(sorted(kwargs.items()))
                key = f"{cache_key}:{hashlib.md5(args_str.encode()).hexdigest()}"
                
                # 尝试从缓存获取
                cached_result = await ctx_inner.cache.get(key)
                if cached_result is not None:
                    ctx_inner.logger.info("cache_demo", f"缓存命中: {key}")
                    return cached_result
                
                # 执行函数
                result = await func(ctx_inner, *args, **kwargs)
                
                # 缓存结果
                await ctx_inner.cache.set(key, result, ttl=ttl)
                ctx_inner.logger.info("cache_demo", f"缓存结果: {key}")
                
                return result
            return wrapper
        return decorator
    
    @cache_result("expensive_computation", ttl=120)
    async def expensive_computation(ctx, n: int):
        """模拟耗时计算"""
        ctx.logger.info("cache_demo", f"计算中: n={n}")
        await asyncio.sleep(1)  # 模拟耗时操作
        return {"result": n * n, "computed_at": time.time()}
    
    # 第一次调用，会执行计算
    result1 = await expensive_computation(ctx, 5)
    ctx.logger.info("cache_demo", f"第一次调用结果: {result1}")
    
    # 第二次调用，会从缓存获取
    result2 = await expensive_computation(ctx, 5)
    ctx.logger.info("cache_demo", f"第二次调用结果: {result2}")

async def error_handling_and_fallback(ctx):
    """错误处理和降级策略"""
    ctx.logger.info("cache_demo", "--- 错误处理和降级策略 ---")
    
    # 模拟缓存不可用
    original_cache = ctx.cache
    
    try:
        # 尝试正常缓存操作
        await ctx.cache.set("test_key", "test_value")
        value = await ctx.cache.get("test_key")
        ctx.logger.info("cache_demo", f"正常缓存操作: {value}")
        
    except Exception as e:
        ctx.logger.error("cache_demo", f"缓存操作失败: {e}")
        
        # 降级到内存字典
        fallback_cache = {}
        
        # 模拟降级缓存操作
        fallback_cache["test_key"] = "fallback_value"
        ctx.logger.info("cache_demo", f"降级缓存值: {fallback_cache.get('test_key')}")
    
    # 批量操作优化
    await batch_operations(ctx)

async def batch_operations(ctx):
    """批量操作优化"""
    ctx.logger.info("cache_demo", "--- 批量操作优化 ---")
    
    # 批量设置
    items = {}
    for i in range(10):
        items[f"batch_item:{i}"] = {"id": i, "name": f"Item {i}", "value": i * 10}
    
    await ctx.cache.set_multiple(items, ttl=300)
    ctx.logger.info("cache_demo", f"批量设置完成: {len(items)} 个项目")
    
    # 批量获取
    keys = [f"batch_item:{i}" for i in range(10)]
    values = await ctx.cache.get_multiple(keys)
    ctx.logger.info("cache_demo", f"批量获取完成: {len(values)} 个值")
    
    # 显示前3个
    for i, (key, value) in enumerate(list(values.items())[:3]):
        ctx.logger.info("cache_demo", f"项目 {i}: {key} = {value}")

if __name__ == "__main__":
    asyncio.run(main())
```

<div align="center">

## 代码解析

</div>

### 1. 基本缓存操作

- **设置缓存**: 使用`set()`方法存储键值对，可设置TTL
- **获取缓存**: 使用`get()`方法获取值，不存在返回None
- **检查存在**: 使用`exists()`方法检查键是否存在
- **获取TTL**: 使用`ttl()`方法获取剩余过期时间
- **删除缓存**: 使用`delete()`方法删除指定键

### 2. 缓存标签

- **设置标签**: 在`set()`时使用`tags`参数添加标签
- **标签查找**: 使用`find_by_tags()`查找带特定标签的键
- **标签删除**: 使用`delete_by_tags()`批量删除带标签的缓存

### 3. 原子操作

- **递增/递减**: 使用`increment()`和`decrement()`进行原子数值操作
- **比较并设置**: 使用`cas()`实现乐观锁机制
- **获取并设置**: 使用`get_set()`原子地获取旧值并设置新值

### 4. 分布式锁

- **获取锁**: 使用`acquire_lock()`获取分布式锁
- **释放锁**: 使用`release_lock()`释放锁
- **超时机制**: 设置锁的超时时间防止死锁

### 5. 缓存装饰器

- **自动缓存**: 装饰器自动缓存函数结果
- **缓存键生成**: 基于函数参数生成唯一缓存键
- **TTL控制**: 可设置缓存过期时间

<div align="center">

## 运行步骤

</div>

### 1. 准备环境

```bash
# 创建项目目录
mkdir dms-cache-example
cd dms-cache-example

# 创建虚拟环境
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate

# 安装依赖
pip install dmsc>=1.0.0 redis>=4.0.0 msgpack>=1.0.0
```

### 2. 创建配置文件

创建`config.yaml`文件，内容如上所示。

### 3. 运行示例

```bash
python main.py
```

### 4. 使用Redis后端（可选）

确保Redis服务器正在运行，然后修改配置文件：

```yaml
cache:
  backend: "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
```

<div align="center">

## 预期结果

</div>

运行示例后，您将看到类似以下输出：

```
[INFO] cache_demo: === 缓存使用示例开始 ===
[INFO] cache_demo: --- 基本缓存操作 ---
[INFO] cache_demo: 缓存用户: {'name': 'Alice', 'age': 30}
[INFO] cache_demo: 缓存存在: True
[INFO] cache_demo: 缓存TTL: 299秒
[INFO] cache_demo: 缓存删除: True
[INFO] cache_demo: --- 缓存标签使用 ---
[INFO] cache_demo: 文章键: ['article:1', 'article:2']
[INFO] cache_demo: 通过标签删除数量: 2
[INFO] cache_demo: --- 原子操作 ---
[INFO] cache_demo: 递增后值: 1
[INFO] cache_demo: 递减后值: 0
[INFO] cache_demo: CAS操作成功: True
[INFO] cache_demo: 旧值: 0, 新值: 50
[INFO] cache_demo: --- 分布式锁示例 ---
[INFO] cache_demo: 锁获取成功: True
[INFO] cache_demo: 执行临界区操作...
[INFO] cache_demo: 临界区操作完成
[INFO] cache_demo: 锁释放成功: True
[INFO] cache_demo: --- 缓存装饰器示例 ---
[INFO] cache_demo: 计算中: n=5
[INFO] cache_demo: 缓存结果: cache_result:41d8... (缓存键哈希)
[INFO] cache_demo: 第一次调用结果: {'result': 25, 'computed_at': 1234567890.123}
[INFO] cache_demo: 缓存命中: cache_result:41d8... (缓存键哈希)
[INFO] cache_demo: 第二次调用结果: {'result': 25, 'computed_at': 1234567890.123}
[INFO] cache_demo: === 缓存使用示例完成 ===
结果: 缓存示例执行成功
```

<div align="center">

## 最佳实践

</div>

1. **合理设置TTL**: 根据数据更新频率设置合适的过期时间
2. **使用标签**: 对相关缓存使用标签，便于批量管理
3. **原子操作**: 对计数器等使用原子操作避免竞态条件
4. **分布式锁**: 在分布式环境中使用锁保护共享资源
5. **错误处理**: 实现降级策略，缓存失败时不影响主流程
6. **批量操作**: 使用批量操作减少网络开销
7. **监控缓存命中率**: 定期监控缓存性能和命中率
8. **避免缓存穿透**: 对不存在的数据也进行缓存（布隆过滤器）
9. **缓存预热**: 系统启动时预加载热点数据
10. **序列化优化**: 使用高效的序列化格式如MessagePack

<div align="center">

## 相关示例

</div>

- [基础应用](./basic-app.md): 构建简单的DMSC应用
- [认证与授权](./authentication.md): 使用JWT和OAuth进行认证
- [数据库操作](./database.md): 数据库连接、查询和事务管理
- [HTTP服务](./http.md): 构建Web应用和RESTful API
- [消息队列](./mq.md): 异步消息处理和事件驱动架构
- [可观测性](./observability.md): 分布式追踪、指标收集和监控
- [安全实践](./security.md): 加密、哈希和安全最佳实践
- [存储管理](./storage.md): 文件上传下载和存储管理
- [数据验证](./validation.md): 数据验证、清理和自定义验证器