<div align="center">

# Troubleshooting

**Version: 0.0.3**

**Last modified date: 2026-01-01**

Common issues and solutions to help you quickly locate and resolve problems

</div>

## Installation Issues

### pip Install Fails

**Problem**: pip installation fails with compilation errors

**Solution**:
```bash
# Install dependencies first
pip install setuptools wheel

# Or use pre-built wheel
pip install --only-binary :all: dmsc
```

### Python Version Not Supported

**Problem**: "Python version 3.8+ required"

**Solution**:
```bash
# Check Python version
python --version

# Install newer Python if needed
# On Ubuntu:
sudo apt update
sudo apt install python3.11

# On macOS:
brew install python@3.11
```

## Runtime Issues

### Import Errors

**Problem**: "ModuleNotFoundError: No module named 'dmsc'"

**Solution**:
```bash
# Activate virtual environment
source venv/bin/activate  # Linux/macOS
.\venv\Scripts\activate   # Windows

# Reinstall
pip install dmsc
```

### Async Runtime Errors

**Problem**: "Event loop is already running"

**Solution**:
```python
# Usenest_asyncio for Jupyter notebooks
import nest_asyncio
nest_asyncio.apply()

# Or use proper async entry point
if __name__ == "__main__":
    asyncio.run(main())
```

### Memory Issues

**Problem**: High memory usage or memory leaks

**Solution**:
```python
# Enable memory profiling
app = DMSCAppBuilder()
app.with_monitoring(
    memory_limit_mb=512,
    auto_gc=True
)

# Manually trigger garbage collection
import gc
gc.collect()
```

## Configuration Issues

### Configuration Not Loading

**Problem**: Configuration changes not taking effect

**Solution**:
```python
# Enable hot reload
app = DMSCAppBuilder()
app.with_config(
    DMSCConfig.from_file("config.yaml"),
    watch_changes=True
)

# Or restart the application
```

### Environment Variables Not Read

**Problem**: Environment variables not being read

**Solution**:
```python
# Check environment variable loading
from dmsc import DMSCConfig

config = DMSCConfig.from_env(
    prefix="MYAPP_",
    required=["DATABASE_URL"]
)

# Verify variables are set
import os
print(os.environ.get("MYAPP_DATABASE_URL"))
```

## Performance Issues

### Slow Requests

**Problem**: HTTP requests are slow

**Solution**:
```python
# Enable request logging
app = DMSCAppBuilder()
app.with_logging(
    log_requests=True,
    log_responses=True
)

# Check for N+1 queries
# Use connection pooling
app.with_database(pool_size=20)
```

### High CPU Usage

**Problem**: CPU usage is too high

**Solution**:
```python
# Enable profiling
app = DMSCAppBuilder()
app.with_profiling(
    enabled=True,
    sample_rate=0.1  # Sample 10% of requests
)

# Check for infinite loops
# Use async/await properly
```

## Logging Issues

### Logs Not Appearing

**Problem**: Logs are not being output

**Solution**:
```python
# Configure logging explicitly
from dmsc import DMSCLogConfig

app = DMSCAppBuilder()
app.with_logging(
    DMSCLogConfig(
        level="DEBUG",
        handlers=["console", "file"],
        format="json"
    )
)

# Check log file permissions
```

### JSON Logs Not Parsing

**Problem": JSON logs cannot be parsed

**Solution**:
```python
# Ensure JSON format is enabled
app = DMSCAppBuilder()
app.with_logging(
    DMSCLogConfig(
        format="json",
        escape_html=False
    )
)

# Check for malformed JSON
import json
with open("app.log") as f:
    for line in f:
        try:
            json.loads(line)
        except json.JSONDecodeError as e:
            print(f"Invalid JSON: {e}")
```

## Getting Help

### Check Logs

```bash
# Enable debug logging
DMSC_LOG_LEVEL=DEBUG python app.py

# Save logs to file
python app.py > app.log 2>&1
```

### Report Issues

When reporting issues, include:
1. Python version: `python --version`
2. DMSC Python version: `pip show dmsc`
3. Operating system
4. Full error traceback
5. Minimal reproduction code
