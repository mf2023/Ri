<div align="center">

# 数据库使用示例

**Version: 1.0.0**

**最后更新日期: 2025-12-27**

本示例展示如何使用DMSC Python的database模块进行数据库连接、查询构建、事务管理、连接池和迁移功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC Python应用，实现以下功能：

- PostgreSQL、MySQL、SQLite数据库连接
- 查询构建器和复杂查询
- 事务管理和连接池
- 数据库迁移和模式管理
- 数据访问对象(DAO)模式
- 错误处理和连接监控

<div align="center">

## 前置要求

</div>

- Python 3.8+
- pip 20.0+
- 基本的Python编程知识
- 了解SQL和数据库基本概念
- （可选）PostgreSQL、MySQL或SQLite数据库服务器

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
mkdir dms-database-example
cd dms-database-example
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate
```

### 2. 添加依赖

创建`requirements.txt`文件：

```txt
dmsc>=1.0.0
asyncpg>=0.28.0
aiomysql>=0.2.0
aiosqlite>=0.19.0
alembic>=1.12.0
sqlalchemy>=2.0.0
```

安装依赖：

```bash
pip install -r requirements.txt
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-database-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

database:
  default: "postgresql"
  connections:
    postgresql:
      database_type: "postgresql"
      host: "localhost"
      port: 5432
      database: "dms_example"
      username: "postgres"
      password: "password"
      pool_size: 20
      connection_timeout: 30
      idle_timeout: 600
      max_lifetime: 3600
      ssl_mode: "require"
    mysql:
      database_type: "mysql"
      host: "localhost"
      port: 3306
      database: "dms_example"
      username: "root"
      password: "password"
      pool_size: 15
      charset: "utf8mb4"
      collation: "utf8mb4_unicode_ci"
    sqlite:
      database_type: "sqlite"
      database_path: "./data/dms_example.db"
      pool_size: 5
```

### 4. 编写主代码

创建`main.py`文件：

```python
import asyncio
import datetime
from dataclasses import dataclass
from typing import List, Optional
from dmsc import DMSCAppBuilder, DMSCDatabaseConfig

@dataclass
class User:
    """用户数据模型"""
    id: Optional[int] = None
    username: str = ""
    email: str = ""
    age: int = 0
    created_at: Optional[datetime.datetime] = None

@dataclass
class Product:
    """产品数据模型"""
    id: Optional[int] = None
    name: str = ""
    price: float = 0.0
    stock: int = 0
    category: str = ""

async def main():
    """主函数"""
    # 构建服务运行时
    app = DMSCAppBuilder()
    
    # 配置数据库连接
    db_config = DMSCDatabaseConfig.postgresql(
        host="localhost",
        port=5432,
        database="dms_example",
        username="postgres",
        password="password",
        pool_size=20
    )
    
    # 或使用SQLite进行测试
    # db_config = DMSCDatabaseConfig.sqlite("./data/dms_example.db")
    
    # 构建应用
    dms_app = (app
               .with_database(db_config)
               .with_config("config.yaml")
               .build())
    
    # 定义业务逻辑
    async def business_logic(ctx):
        """业务逻辑函数"""
        ctx.logger.info("db_demo", "=== 数据库使用示例开始 ===")
        
        # 1. 数据库连接和表创建
        await setup_database(ctx)
        
        # 2. 基本CRUD操作
        await basic_crud_operations(ctx)
        
        # 3. 查询构建器使用
        await query_builder_examples(ctx)
        
        # 4. 事务管理
        await transaction_examples(ctx)
        
        # 5. 连接池管理
        await connection_pool_demo(ctx)
        
        # 6. 数据访问对象模式
        await dao_pattern_example(ctx)
        
        # 7. 错误处理
        await error_handling_demo(ctx)
        
        ctx.logger.info("db_demo", "=== 数据库使用示例完成 ===")
        return "数据库示例执行成功"
    
    # 运行应用
    result = await dms_app.run_async(business_logic)
    print(f"结果: {result}")

async def setup_database(ctx):
    """设置数据库"""
    ctx.logger.info("db_demo", "--- 数据库连接和表创建 ---")
    
    # 创建用户表
    create_users_table = """
    CREATE TABLE IF NOT EXISTS users (
        id SERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        email VARCHAR(100) UNIQUE NOT NULL,
        age INTEGER CHECK (age >= 0 AND age <= 150),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    """
    
    # 创建产品表
    create_products_table = """
    CREATE TABLE IF NOT EXISTS products (
        id SERIAL PRIMARY KEY,
        name VARCHAR(100) NOT NULL,
        price DECIMAL(10, 2) CHECK (price >= 0),
        stock INTEGER CHECK (stock >= 0),
        category VARCHAR(50),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    """
    
    # 创建索引
    create_indexes = """
    CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
    CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);
    CREATE INDEX IF NOT EXISTS idx_products_price ON products(price);
    """
    
    # 执行DDL语句
    await ctx.db.execute(create_users_table)
    await ctx.db.execute(create_products_table)
    await ctx.db.execute(create_indexes)
    
    ctx.logger.info("db_demo", "数据库表创建完成")

async def basic_crud_operations(ctx):
    """基本CRUD操作"""
    ctx.logger.info("db_demo", "--- 基本CRUD操作 ---")
    
    # 清空表数据
    await ctx.db.execute("TRUNCATE TABLE users, products RESTART IDENTITY CASCADE")
    
    # 插入用户数据
    user_data = [
        ("alice", "alice@example.com", 25),
        ("bob", "bob@example.com", 30),
        ("charlie", "charlie@example.com", 35),
        ("diana", "diana@example.com", 28),
    ]
    
    insert_query = "INSERT INTO users (username, email, age) VALUES ($1, $2, $3)"
    for username, email, age in user_data:
        await ctx.db.execute(insert_query, username, email, age)
    
    ctx.logger.info("db_demo", f"插入了 {len(user_data)} 个用户")
    
    # 插入产品数据
    product_data = [
        ("iPhone 15", 999.99, 100, "Electronics"),
        ("MacBook Pro", 1999.99, 50, "Electronics"),
        ("Coffee Maker", 79.99, 200, "Kitchen"),
        ("Running Shoes", 129.99, 150, "Sports"),
        ("Book: Python Guide", 39.99, 300, "Books"),
    ]
    
    insert_product_query = "INSERT INTO products (name, price, stock, category) VALUES ($1, $2, $3, $4)"
    for name, price, stock, category in product_data:
        await ctx.product_query.execute(insert_product_query, name, price, stock, category)
    
    ctx.logger.info("db_demo", f"插入了 {len(product_data)} 个产品")
    
    # 查询数据
    select_users = "SELECT * FROM users WHERE age > $1 ORDER BY username"
    users = await ctx.db.fetch_all(select_users, 25)
    ctx.logger.info("db_demo", f"查询到 {len(users)} 个年龄大于25的用户")
    
    # 更新数据
    update_query = "UPDATE users SET age = age + 1 WHERE username = $1"
    await ctx.db.execute(update_query, "alice")
    ctx.logger.info("db_demo", "更新了alice的年龄")
    
    # 删除数据
    delete_query = "DELETE FROM users WHERE username = $1"
    await ctx.db.execute(delete_query, "diana")
    ctx.logger.info("db_demo", "删除了diana用户")

async def query_builder_examples(ctx):
    """查询构建器使用"""
    ctx.logger.info("db_demo", "--- 查询构建器使用 ---")
    
    # 简单查询
    users = await ctx.db.table("users").select("*").where("age", ">=", 30).get()
    ctx.logger.info("db_demo", f"年龄>=30的用户: {len(users)} 个")
    
    # 复杂查询
    results = await (ctx.db.table("products")
                     .select("category", "COUNT(*) as count", "AVG(price) as avg_price")
                     .where("stock", ">", 0)
                     .group_by("category")
                     .having("COUNT(*)", ">=", 2)
                     .order_by("avg_price", "DESC")
                     .limit(5)
                     .get())
    
    ctx.logger.info("db_demo", "按类别分组的产品统计:")
    for row in results:
        ctx.logger.info("db_demo", f"  {row['category']}: {row['count']} 个产品, 平均价格 ${row['avg_price']:.2f}")
    
    # 连接查询
    # 注意：这里需要创建订单表来演示连接查询
    create_orders_table = """
    CREATE TABLE IF NOT EXISTS orders (
        id SERIAL PRIMARY KEY,
        user_id INTEGER REFERENCES users(id),
        product_id INTEGER REFERENCES products(id),
        quantity INTEGER CHECK (quantity > 0),
        total_price DECIMAL(10, 2),
        status VARCHAR(20) DEFAULT 'pending',
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )
    """
    
    await ctx.db.execute(create_orders_table)
    
    # 插入订单数据
    orders_data = [
        (1, 1, 2, 1999.98),  # alice买了2个iPhone
        (2, 3, 1, 79.99),    # bob买了1个Coffee Maker
        (1, 2, 1, 1999.99),  # alice买了1个MacBook
    ]
    
    insert_order_query = "INSERT INTO orders (user_id, product_id, quantity, total_price) VALUES ($1, $2, $3, $4)"
    for user_id, product_id, quantity, total_price in orders_data:
        await ctx.db.execute(insert_order_query, user_id, product_id, quantity, total_price)
    
    # 执行连接查询
    join_query = """
    SELECT u.username, p.name, o.quantity, o.total_price, o.status
    FROM orders o
    JOIN users u ON o.user_id = u.id
    JOIN products p ON o.product_id = p.id
    WHERE o.status = 'pending'
    ORDER BY o.created_at DESC
    """
    
    orders = await ctx.db.fetch_all(join_query)
    ctx.logger.info("db_demo", f"待处理订单: {len(orders)} 个")
    for order in orders:
        ctx.logger.info("db_demo", f"  {order['username']} 购买了 {order['quantity']} 个 {order['name']}, 总计 ${order['total_price']:.2f}")

async def transaction_examples(ctx):
    """事务管理"""
    ctx.logger.info("db_demo", "--- 事务管理 ---")
    
    # 简单事务
    async with ctx.db.transaction() as tx:
        # 在事务中执行多个操作
        await tx.execute("UPDATE products SET stock = stock - 1 WHERE id = $1", 1)
        await tx.execute("INSERT INTO orders (user_id, product_id, quantity, total_price) VALUES ($1, $2, $3, $4)", 
                        1, 1, 1, 999.99)
        
        # 如果发生错误，事务会自动回滚
        # 如果成功，事务会自动提交
    
    ctx.logger.info("db_demo", "简单事务完成")
    
    # 手动控制事务
    tx = await ctx.db.begin_transaction()
    
    try:
        # 检查库存
        product = await tx.fetch_one("SELECT stock FROM products WHERE id = $1", 2)
        if product['stock'] < 5:
            raise Exception("库存不足")
        
        # 扣减库存
        await tx.execute("UPDATE products SET stock = stock - 5 WHERE id = $1", 2)
        
        # 创建订单
        await tx.execute("INSERT INTO orders (user_id, product_id, quantity, total_price) VALUES ($1, $2, $3, $4)",
                        2, 2, 5, 9999.95)
        
        # 提交事务
        await tx.commit()
        ctx.logger.info("db_demo", "手动事务提交成功")
        
    except Exception as e:
        # 回滚事务
        await tx.rollback()
        ctx.logger.error("db_demo", f"事务回滚: {e}")
    
    # 保存点事务
    async with ctx.db.transaction() as tx:
        # 创建保存点
        savepoint1 = await tx.create_savepoint("sp1")
        
        try:
            await tx.execute("UPDATE users SET age = age + 1 WHERE id = $1", 1)
            
            # 创建第二个保存点
            savepoint2 = await tx.create_savepoint("sp2")
            
            try:
                await tx.execute("UPDATE products SET price = price * 1.1 WHERE id = $1", 3)
                
                # 模拟错误
                raise Exception("模拟错误")
                
            except Exception as e:
                # 回滚到保存点2
                await tx.rollback_to_savepoint(savepoint2)
                ctx.logger.info("db_demo", f"回滚到保存点2: {e}")
            
            # 继续执行其他操作
            await tx.execute("INSERT INTO users (username, email, age) VALUES ($1, $2, $3)",
                           "eve", "eve@example.com", 22)
            
        except Exception as e:
            # 回滚到保存点1
            await tx.rollback_to_savepoint(savepoint1)
            ctx.logger.error("db_demo", f"回滚到保存点1: {e}")

async def connection_pool_demo(ctx):
    """连接池管理"""
    ctx.logger.info("db_demo", "--- 连接池管理 ---")
    
    # 获取连接池统计信息
    pool_stats = await ctx.db.get_pool_stats()
    ctx.logger.info("db_demo", f"连接池统计: {pool_stats}")
    
    # 执行并发查询测试连接池
    async def concurrent_query(user_id):
        """并发查询函数"""
        user = await ctx.db.fetch_one("SELECT * FROM users WHERE id = $1", user_id)
        return user
    
    # 并发执行多个查询
    tasks = []
    for user_id in range(1, 6):
        task = asyncio.create_task(concurrent_query(user_id))
        tasks.append(task)
    
    # 等待所有查询完成
    results = await asyncio.gather(*tasks)
    ctx.logger.info("db_demo", f"并发查询完成: {len(results)} 个结果")
    
    # 监控连接使用情况
    for i in range(3):
        stats = await ctx.db.get_pool_stats()
        ctx.logger.info("db_demo", f"连接池状态 {i+1}: 使用中={stats['in_use']}, 空闲={stats['idle']}")
        await asyncio.sleep(1)

async def dao_pattern_example(ctx):
    """数据访问对象模式"""
    ctx.logger.info("db_demo", "--- 数据访问对象模式 ---")
    
    class UserDAO:
        """用户数据访问对象"""
        
        def __init__(self, db):
            self.db = db
        
        async def create(self, user: User) -> User:
            """创建用户"""
            query = """
            INSERT INTO users (username, email, age) 
            VALUES ($1, $2, $3) 
            RETURNING id, username, email, age, created_at
            """
            result = await self.db.fetch_one(query, user.username, user.email, user.age)
            return User(**result)
        
        async def get_by_id(self, user_id: int) -> Optional[User]:
            """根据ID获取用户"""
            query = "SELECT * FROM users WHERE id = $1"
            result = await self.db.fetch_one(query, user_id)
            return User(**result) if result else None
        
        async def get_by_username(self, username: str) -> Optional[User]:
            """根据用户名获取用户"""
            query = "SELECT * FROM users WHERE username = $1"
            result = await self.db.fetch_one(query, username)
            return User(**result) if result else None
        
        async def update(self, user_id: int, **kwargs) -> bool:
            """更新用户"""
            if not kwargs:
                return False
            
            set_clause = ", ".join([f"{key} = ${i+2}" for i, key in enumerate(kwargs.keys())])
            query = f"UPDATE users SET {set_clause} WHERE id = $1"
            
            result = await self.db.execute(query, user_id, *kwargs.values()))
            return result > 0
        
        async def delete(self, user_id: int) -> bool:
            """删除用户"""
            query = "DELETE FROM users WHERE id = $1"
            result = await self.db.execute(query, user_id)
            return result > 0
        
        async def list_users(self, limit: int = 100, offset: int = 0) -> List[User]:
            """获取用户列表"""
            query = "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2"
            results = await self.db.fetch_all(query, limit, offset)
            return [User(**row) for row in results]
        
        async def count_users(self) -> int:
            """统计用户数量"""
            query = "SELECT COUNT(*) as count FROM users"
            result = await self.db.fetch_one(query)
            return result['count'] if result else 0
    
    # 使用DAO
    user_dao = UserDAO(ctx.db)
    
    # 创建新用户
    new_user = User(username="frank", email="frank@example.com", age=27)
    created_user = await user_dao.create(new_user)
    ctx.logger.info("db_demo", f"创建用户: {created_user}")
    
    # 查询用户
    found_user = await user_dao.get_by_id(created_user.id)
    ctx.logger.info("db_demo", f"查询用户: {found_user}")
    
    # 更新用户
    updated = await user_dao.update(created_user.id, age=28, email="frank.updated@example.com")
    ctx.logger.info("db_demo", f"更新用户: {updated}")
    
    # 获取用户列表
    users = await user_dao.list_users(limit=5)
    ctx.logger.info("db_demo", f"用户列表: {len(users)} 个用户")
    for user in users:
        ctx.logger.info("db_demo", f"  - {user.username} ({user.email})")
    
    # 统计用户
    user_count = await user_dao.count_users()
    ctx.logger.info("db_demo", f"用户总数: {user_count}")

async def error_handling_demo(ctx):
    """错误处理"""
    ctx.logger.info("db_demo", "--- 错误处理 ---")
    
    # 处理连接错误
    try:
        # 尝试连接到不存在的数据库
        await ctx.db.execute("SELECT * FROM nonexistent_table")
    except Exception as e:
        ctx.logger.error("db_demo", f"数据库错误: {e}")
    
    # 处理查询错误
    try:
        # 违反唯一约束
        await ctx.db.execute("INSERT INTO users (username, email, age) VALUES ($1, $2, $3)",
                           "alice", "alice@example.com", 25)
    except Exception as e:
        ctx.logger.error("db_demo", f"约束违反: {e}")
    
    # 处理超时
    try:
        # 设置查询超时
        await ctx.db.set_query_timeout(1)  # 1秒超时
        
        # 执行可能耗时的查询
        await ctx.db.execute("SELECT pg_sleep(2)")  # PostgreSQL sleep函数
    except Exception as e:
        ctx.logger.error("db_demo", f"查询超时: {e}")
    
    # 重置超时
    await ctx.db.set_query_timeout(30)  # 重置为30秒
    
    # 健康检查
    is_healthy = await ctx.db.health_check()
    ctx.logger.info("db_demo", f"数据库健康状态: {is_healthy}")

if __name__ == "__main__":
    asyncio.run(main())
```

<div align="center">

## 代码解析

</div>

### 1. 数据库连接配置

- **多数据库支持**: 支持PostgreSQL、MySQL、SQLite
- **连接池配置**: 设置池大小、超时时间等参数
- **SSL模式**: 支持加密连接
- **字符集设置**: 支持UTF-8和多语言

### 2. 查询构建器

- **链式调用**: 支持链式构建复杂查询
- **条件构造**: 灵活的条件表达式
- **聚合函数**: 支持COUNT、AVG、SUM等
- **分组和排序**: GROUP BY和ORDER BY支持
- **限制和偏移**: LIMIT和OFFSET支持

### 3. 事务管理

- **自动事务**: 使用async with语法自动提交/回滚
- **手动事务**: 显式控制提交和回滚
- **保存点**: 支持部分回滚的保存点机制
- **事务隔离**: 支持不同的事务隔离级别

### 4. 连接池管理

- **连接复用**: 高效的数据库连接复用
- **并发支持**: 支持高并发场景
- **连接监控**: 实时监控连接状态
- **自动回收**: 自动回收空闲连接

### 5. 数据访问对象(DAO)

- **封装数据访问**: 将数据访问逻辑封装在DAO中
- **类型安全**: 使用数据类确保类型安全
- **CRUD操作**: 标准化的创建、读取、更新、删除操作
- **业务逻辑分离**: 分离数据访问和业务逻辑

<div align="center">

## 运行步骤

</div>

### 1. 准备环境

```bash
# 创建项目目录
mkdir dms-database-example
cd dms-database-example

# 创建虚拟环境
python -m venv venv
source venv/bin/activate  # Windows: venv\\Scripts\\activate

# 安装依赖
pip install dmsc>=1.0.0 asyncpg>=0.28.0 aiomysql>=0.2.0 aiosqlite>=0.19.0
```

### 2. 创建配置文件

创建`config.yaml`文件，内容如上所示。

### 3. 运行示例

```bash
python main.py
```

### 4. 使用SQLite进行测试

修改配置文件使用SQLite：

```yaml
database:
  default: "sqlite"
  connections:
    sqlite:
      database_type: "sqlite"
      database_path: "./data/dms_example.db"
      pool_size: 5
```

<div align="center">

## 预期结果

</div>

运行示例后，您将看到类似以下输出：

```
[INFO] db_demo: === 数据库使用示例开始 ===
[INFO] db_demo: --- 数据库连接和表创建 ---
[INFO] db_demo: 数据库表创建完成
[INFO] db_demo: --- 基本CRUD操作 ---
[INFO] db_demo: 插入了 4 个用户
[INFO] db_demo: 插入了 5 个产品
[INFO] db_demo: 查询到 3 个年龄大于25的用户
[INFO] db_demo: 更新了alice的年龄
[INFO] db_demo: 删除了diana用户
[INFO] db_demo: --- 查询构建器使用 ---
[INFO] db_demo: 年龄>=30的用户: 2 个
[INFO] db_demo: 按类别分组的产品统计:
[INFO] db_demo:   Electronics: 2 个产品, 平均价格 $1499.99
[INFO] db_demo:   Kitchen: 1 个产品, 平均价格 $79.99
[INFO] db_demo:   Sports: 1 个产品, 平均价格 $129.99
[INFO] db_demo:   Books: 1 个产品, 平均价格 $39.99
[INFO] db_demo: 插入了 3 个订单
[INFO] db_demo: 待处理订单: 3 个
[INFO] db_demo:   alice 购买了 2 个 iPhone 15, 总计 $1999.98
[INFO] db_demo:   bob 购买了 1 个 Coffee Maker, 总计 $79.99
[INFO] db_demo:   alice 购买了 1 个 MacBook Pro, 总计 $1999.99
[INFO] db_demo: --- 事务管理 ---
[INFO] db_demo: 简单事务完成
[INFO] db_demo: 手动事务提交成功
[INFO] db_demo: 回滚到保存点2: 模拟错误
[INFO] db_demo: --- 连接池管理 ---
[INFO] db_demo: 连接池统计: {'total': 20, 'in_use': 2, 'idle': 18}
[INFO] db_demo: 并发查询完成: 4 个结果
[INFO] db_demo: 连接池状态 1: 使用中=1, 空闲=19
[INFO] db_demo: 连接池状态 2: 使用中=0, 空闲=20
[INFO] db_demo: 连接池状态 3: 使用中=0, 空闲=20
[INFO] db_demo: --- 数据访问对象模式 ---
[INFO] db_demo: 创建用户: User(id=5, username='frank', email='frank@example.com', age=27, created_at=datetime.datetime(...))
[INFO] db_demo: 查询用户: User(id=5, username='frank', email='frank@example.com', age=27, created_at=datetime.datetime(...))
[INFO] db_demo: 更新用户: True
[INFO] db_demo: 用户列表: 5 个用户
[INFO] db_demo:   - frank (frank.updated@example.com)
[INFO] db_demo:   - eve (eve@example.com)
[INFO] db_demo:   - charlie (charlie@example.com)
[INFO] db_demo:   - bob (bob@example.com)
[INFO] db_demo:   - alice (alice@example.com)
[INFO] db_demo: 用户总数: 5
[INFO] db_demo: --- 错误处理 ---
[INFO] db_demo: 数据库错误: 关系 "nonexistent_table" 不存在
[INFO] db_demo: 约束违反: 重复键值违反唯一约束 "users_username_key"
[INFO] db_demo: 查询超时: 查询超时
[INFO] db_demo: 数据库健康状态: True
[INFO] db_demo: === 数据库使用示例完成 ===
结果: 数据库示例执行成功
```

<div align="center">

## 最佳实践

</div>

1. **使用连接池**: 始终使用连接池管理数据库连接
2. **参数化查询**: 使用参数化查询防止SQL注入
3. **事务管理**: 合理使用事务确保数据一致性
4. **错误处理**: 妥善处理数据库错误和异常
5. **索引优化**: 为常用查询字段创建索引
6. **连接监控**: 监控连接池状态和数据库健康
7. **DAO模式**: 使用DAO模式封装数据访问逻辑
8. **类型安全**: 使用数据模型确保类型安全
9. **查询优化**: 避免N+1查询问题，使用批量操作
10. **迁移管理**: 使用数据库迁移工具管理模式变更

<div align="center">

## 相关示例

</div>

- [基础应用](./basic-app.md): 构建简单的DMSC应用
- [认证与授权](./authentication.md): 使用JWT和OAuth进行认证
- [缓存使用](./caching.md): 缓存的基本操作和高级用法
- [HTTP服务](./http.md): 构建Web应用和RESTful API
- [消息队列](./mq.md): 异步消息处理和事件驱动架构
- [可观测性](./observability.md): 分布式追踪、指标收集和监控
- [安全实践](./security.md): 加密、哈希和安全最佳实践
- [存储管理](./storage.md): 文件上传下载和存储管理
- [数据验证](./validation.md): 数据验证、清理和自定义验证器