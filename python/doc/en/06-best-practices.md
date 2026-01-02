<div align="center">

# Best Practices

**Version: 0.0.3**

**Last modified date: 2026-01-01**

Best practices for building efficient, reliable, and secure DMSC Python applications

</div>

## Performance Optimization

### Use Connection Pooling

```python
from dmsc import DMSCAppBuilder, DMSCDatabaseConfig

app = DMSCAppBuilder()
app.with_database(
    DMSCDatabaseConfig(
        pool_size=10,
        max_overflow=20
    )
)
```

### Cache Frequently Accessed Data

```python
async def get_user_cached(ctx, user_id: str) -> dict:
    # Try cache first
    cached = await ctx.cache.get(f"user:{user_id}")
    if cached:
        return cached
    
    # Fetch from database
    user = await ctx.db.get_user(user_id)
    
    # Cache for future requests
    await ctx.cache.set(f"user:{user_id}", user, ttl=3600)
    
    return user
```

### Batch Operations

```python
async def batch_insert(ctx, items: list):
    # Instead of individual inserts
    # for item in items:
    #     await ctx.db.insert(item)
    
    # Use batch operation
    await ctx.db.insert_many(items)
```

## Security Best Practices

### Validate Input

```python
from pydantic import BaseModel

class UserCreate(BaseModel):
    username: str
    email: str
    age: int
    
    @validator("age")
    def age_must_be_positive(cls, v):
        if v <= 0:
            raise ValueError("Age must be positive")
        return v

async def create_user(ctx, data: UserCreate):
    user = await ctx.auth.create_user(data.dict())
    return user
```

### Use Environment Variables for Secrets

```python
import os
from dmsc import DMSCConfig

config = DMSCConfig.from_env(
    prefix="MYAPP_",
    secrets=["database_password", "api_key"]
)
```

### Implement Rate Limiting

```python
from dmsc import DMSCRateLimiter

limiter = DMSCRateLimiter(
    rate=100,  # requests per minute
    burst=20   # burst capacity
)

async def protected_endpoint(ctx, request):
    await limiter.acquire(ctx.request.ip)
    # Handle request
```

## Error Handling

### Structured Error Responses

```python
from dmsc import DMSCError, DMSCResult

class AppError(DMSCError):
    def __init__(self, message: str, code: str):
        super().__init__(message)
        self.code = code
        self.message = message

async def create_user_handler(ctx, request):
    try:
        user = await ctx.auth.create_user(request.data)
        return {"status": "success", "data": user}
    except AppError as e:
        ctx.logger.error("handler", f"App error: {e.message}")
        return {"status": "error", "code": e.code}
```

### Logging Best Practices

```python
# Use structured logging
ctx.logger.info("user_action", {
    "user_id": user.id,
    "action": "login",
    "ip": ctx.request.ip
})

# Avoid string interpolation
# Bad: ctx.logger.info(f"User {user.id} logged in")
# Good: ctx.logger.info("user_login", {"user_id": user.id})
```

## Testing

### Unit Testing

```python
import pytest
from dmsc import DMSCAppBuilder

@pytest.fixture
def app():
    return DMSCAppBuilder().build()

@pytest.fixture
def ctx(app):
    return app.create_context()

@pytest.mark.asyncio
async def test_service(ctx):
    result = await ctx.cache.set("test", "value")
    assert result is True
    
    cached = await ctx.cache.get("test")
    assert cached == "value"
```

### Integration Testing

```python
@pytest.fixture(scope="module")
def test_app():
    app = DMSCAppBuilder()
    app.with_config(DMSCConfig.from_file("test_config.yaml"))
    return app.build()

@pytest.mark.asyncio
async def test_database_connection(test_app):
    async with test_app.run_test() as ctx:
        result = await ctx.db.query("SELECT 1")
        assert result == [{"?column?": 1}]
```

## Deployment

### Docker Configuration

```dockerfile
FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

CMD ["python", "app.py"]
```

### Environment-Specific Configuration

```python
# config.yaml
app:
  debug: false
  
# config.dev.yaml
app:
  debug: true
  log_level: DEBUG

# config.prod.yaml  
app:
  debug: false
  log_level: INFO
```

### Health Checks

```python
from dmsc import DMSCHTTPEndpoint

@app.router.get("/health")
async def health_check(ctx):
    return {
        "status": "healthy",
        "version": "1.0.0",
        "timestamp": ctx.time.now().isoformat()
    }
```
