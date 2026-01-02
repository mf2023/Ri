<div align="center">

# Getting Started with DMSC Python

**Version: 0.0.3**

**Last modified date: 2026-01-01**

Install, configure, and run your first DMSC Python application

</div>

## Installation

### Prerequisites

- Python 3.8 or higher
- pip, poetry, or pipenv package manager
- At least 512MB of available RAM
- 100MB of disk space

### Install from PyPI

The simplest way to install DMSC Python is via pip:

```bash
pip install dmsc
```

### Install with Poetry

If you prefer using Poetry for dependency management:

```bash
poetry add dmsc
```

### Install with Pipenv

For Pipenv users:

```bash
pipenv install dmsc
```

### Install from Source

For development or latest features:

```bash
git clone https://github.com/dunimd/dmsc.git
cd dmsc/python
pip install -e .
```

## Your First Application

### Basic Example

Create a file named `app.py`:

```python
import asyncio
from dmsc import DMSCAppBuilder, DMSCLogConfig

async def main():
    # Create application builder
    app = DMSCAppBuilder()
    
    # Configure logging
    app.with_logging(DMSCLogConfig.default())
    
    # Build application
    dms_app = app.build()
    
    # Run application
    await dms_app.run_async(my_service_logic)

async def my_service_logic(ctx):
    # Use service context
    ctx.logger.info("demo", "Hello from DMSC Python!")
    
    # Access configuration
    config_value = ctx.config.get("my.key", "default")
    
    # Use cache
    await ctx.cache.set("key", "value", ttl=3600)
    
    return {"status": "success"}

if __name__ == "__main__":
    asyncio.run(main())
```

### Run the Application

```bash
python app.py
```

## Configuration

### Configuration Files

DMSC Python supports multiple configuration sources:

- `config.yaml` - Main configuration file
- `config.json` - JSON format configuration
- Environment variables
- Command-line arguments

### Basic Configuration

Create a `config.yaml` file:

```yaml
app:
  name: my-dmsc-app
  host: "0.0.0.0"
  port: 8080

logging:
  level: INFO
  format: json

cache:
  backend: memory
  ttl: 3600
```

### Load Configuration

```python
from dmsc import DMSCAppBuilder, DMSCConfig

app = DMSCAppBuilder()
app.with_config(DMSCConfig.from_file("config.yaml"))
dms_app = app.build()
```

## Project Structure

Recommended project structure:

```
my-dmsc-project/
├── config.yaml
├── app.py
├── requirements.txt
└── src/
    ├── services/
    ├── models/
    └── utils/
```

## Next Steps

- [Core Concepts](./03-core-concepts.md) - Learn about DMSC Python's core concepts
- [API Reference](./04-api-reference/README.md) - Explore detailed API documentation
- [Usage Examples](./05-usage-examples/README.md) - See practical examples
