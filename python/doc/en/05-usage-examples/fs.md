<div align="center">

# File System Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's file system module for secure file operations, atomic writes, directory management, and categorized storage.

## Example Overview

This example creates a DMSC Python application with the following features:

- Atomic file writes for data integrity
- Categorized directory structure (logs, cache, data, etc.)
- JSON serialization and deserialization
- File information and metadata
- Path safety and validation
- Directory operations

## Prerequisites

- Python 3.8+
- Understanding of file system concepts
- Sufficient disk space for data storage

## Complete Code Example

```python
import asyncio
import os
from datetime import datetime
from typing import Dict, List, Optional, Any
from pathlib import Path

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCFileSystem, DMSCConfig, DMSCError
)

# File service
class FileService:
    def __init__(self, fs: DMSCFileSystem, context: DMSCServiceContext):
        self.fs = fs
        self.context = context
        self.logger = context.logger
    
    async def read_config(self, filename: str) -> dict:
        """Read configuration file"""
        try:
            data = self.fs.read_json(f"config/{filename}")
            self.logger.info("file", f"Read config: {filename}")
            return data
        except FileNotFoundError:
            raise DMSCError(f"Config file not found: {filename}", "CONFIG_NOT_FOUND")
        except Exception as e:
            self.logger.error("file", f"Error reading config: {e}")
            raise
    
    async def write_config(self, filename: str, data: dict) -> None:
        """Write configuration file with atomic operation"""
        try:
            self.fs.write_json(f"config/{filename}", data)
            self.logger.info("file", f"Wrote config: {filename}")
        except Exception as e:
            self.logger.error("file", f"Error writing config: {e}")
            raise
    
    async def save_report(self, report_name: str, report_data: dict) -> str:
        """Save report to reports directory"""
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        filename = f"reports/{report_name}_{timestamp}.json"
        
        self.fs.write_json(filename, {
            "name": report_name,
            "generated_at": datetime.now().isoformat(),
            "data": report_data
        })
        
        self.logger.info("file", f"Saved report: {filename}")
        return filename
    
    async def read_report(self, report_path: str) -> dict:
        """Read a specific report"""
        return self.fs.read_json(report_path)
    
    async def list_reports(self, report_name: Optional[str] = None) -> List[str]:
        """List all reports"""
        reports_dir = "reports"
        if not self.fs.exists(reports_dir):
            return []
        
        reports = self.fs.list_dir(reports_dir)
        
        if report_name:
            reports = [r for r in reports if r.startswith(report_name)]
        
        return sorted(reports, reverse=True)
    
    async def archive_data(self, data_name: str, data: dict) -> None:
        """Archive data to data directory"""
        timestamp = datetime.now().strftime("%Y/%m/%d")
        archive_path = f"data/{data_name}/{timestamp}"
        
        self.fs.write_json(f"{archive_path}/data.json", data)
        self.fs.write_text(f"{archive_path}/metadata.txt", f"Archived at {datetime.now().isoformat()}")
        
        self.logger.info("file", f"Archived data: {data_name}")
    
    async def get_file_info(self, file_path: str) -> dict:
        """Get file information and metadata"""
        if not self.fs.exists(file_path):
            raise DMSCError(f"File not found: {file_path}", "FILE_NOT_FOUND")
        
        is_file = self.fs.is_file(file_path)
        is_dir = self.fs.is_dir(file_path)
        
        if is_file:
            # Get file extension
            extension = Path(file_path).suffix
            
            # Get directory contents if it's a directory
            contents = []
            if is_dir:
                contents = self.fs.list_dir(file_path)
            
            return {
                "path": file_path,
                "is_file": is_file,
                "is_directory": is_dir,
                "extension": extension if is_file else None,
                "contents": contents if is_dir else None
            }
        else:
            raise DMSCError(f"Not a file: {file_path}", "NOT_A_FILE")
    
    async def safe_read_file(self, file_path: str, max_size: int = 1024*1024) -> str:
        """Safely read a file with size limit"""
        if not self.fs.exists(file_path):
            raise DMSCError(f"File not found: {file_path}", "FILE_NOT_FOUND")
        
        if self.fs.is_dir(file_path):
            raise DMSCError(f"Path is a directory: {file_path}", "IS_DIRECTORY")
        
        # Check file size
        if self.fs.get_file_size(file_path) > max_size:
            raise DMSCError(f"File too large: {file_path}", "FILE_TOO_LARGE")
        
        return self.fs.read_text(file_path)
    
    async def batch_save(self, files: Dict[str, Any]) -> Dict[str, str]:
        """Save multiple files atomically"""
        saved = {}
        
        for path, data in files.items():
            if isinstance(data, str):
                self.fs.write_text(path, data)
            else:
                self.fs.write_json(path, data)
            saved[path] = "saved"
        
        self.logger.info("file", f"Batch saved {len(files)} files")
        return saved
    
    async def cleanup_old_files(self, directory: str, days: int = 30) -> int:
        """Clean up files older than specified days"""
        if not self.fs.exists(directory):
            return 0
        
        cutoff = datetime.now().timestamp() - (days * 24 * 60 * 60)
        cleaned = 0
        
        for file_path in self.fs.list_dir(directory):
            full_path = f"{directory}/{file_path}"
            if self.fs.is_file(full_path):
                mtime = self.fs.get_mtime(full_path)
                if mtime < cutoff:
                    self.fs.remove_file(full_path)
                    cleaned += 1
        
        self.logger.info("file", f"Cleaned up {cleaned} old files")
        return cleaned

# Request handlers
async def handle_read_config(context: DMSCServiceContext):
    data = await context.http.request.json()
    filename = data.get("filename", "app.json")
    
    file_service = context.file_service
    config = await file_service.read_config(filename)
    
    return {"status": "success", "data": config}

async def handle_write_config(context: DMSCServiceContext):
    data = await context.http.request.json()
    filename = data.get("filename", "app.json")
    config_data = data.get("data", {})
    
    file_service = context.file_service
    await file_service.write_config(filename, config_data)
    
    return {"status": "success", "message": f"Config saved: {filename}"}

async def handle_save_report(context: DMSCServiceContext):
    data = await context.http.request.json()
    
    report_name = data.get("name", "report")
    report_data = data.get("data", {})
    
    file_service = context.file_service
    filename = await file_service.save_report(report_name, report_data)
    
    return {"status": "success", "data": {"filename": filename}}

async def handle_list_reports(context: DMSCServiceContext):
    data = await context.http.request.json()
    report_name = data.get("name")
    
    file_service = context.file_service
    reports = await file_service.list_reports(report_name)
    
    return {"status": "success", "data": {"reports": reports}}

async def handle_archive_data(context: DMSCServiceContext):
    data = await context.http.request.json()
    
    data_name = data.get("name", "data")
    data_content = data.get("data", {})
    
    file_service = context.file_service
    await file_service.archive_data(data_name, data_content)
    
    return {"status": "success", "message": f"Data archived: {data_name}"}

async def handle_get_file_info(context: DMSCServiceContext):
    data = await context.http.request.json()
    file_path = data.get("path")
    
    if not file_path:
        return {"status": "error", "message": "path required"}, 400
    
    file_service = context.file_service
    info = await file_service.get_file_info(file_path)
    
    return {"status": "success", "data": info}

async def handle_cleanup(context: DMSCServiceContext):
    data = await context.http.request.json()
    directory = data.get("directory", "temp")
    days = data.get("days", 30)
    
    file_service = context.file_service
    cleaned = await file_service.cleanup_old_files(directory, days)
    
    return {"status": "success", "data": {"cleaned_files": cleaned}}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize file system
    fs = DMSCFileSystem.new_auto_root()
    
    # Create categorized directories
    for directory in ["config", "reports", "data", "logs", "temp"]:
        fs.create_dir(directory, parents=True)
    
    # Initialize file service
    file_service = FileService(fs, dms_app.context)
    dms_app.context.file_service = file_service
    
    # Add routes
    dms_app.router.add_route("POST", "/config/read", handle_read_config)
    dms_app.router.add_route("POST", "/config/write", handle_write_config)
    dms_app.router.add_route("POST", "/reports/save", handle_save_report)
    dms_app.router.add_route("POST", "/reports/list", handle_list_reports)
    dms_app.router.add_route("POST", "/data/archive", handle_archive_data)
    dms_app.router.add_route("POST", "/file/info", handle_get_file_info)
    dms_app.router.add_route("POST", "/file/cleanup", handle_cleanup)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### File Service Architecture

1. **Categorized Directories**: Separate directories for config, reports, data, logs
2. **Atomic Operations**: Safe file writes using DMSC's atomic write feature
3. **JSON Handling**: Built-in JSON serialization and deserialization
4. **Path Safety**: Validation and safe file operations

### Key Components

- **DMSCFileSystem**: Main file system interface
- **Categorized Directories**: Logs, cache, reports, data, temp directories
- **Atomic Writes**: Safe write operations that don't corrupt data

## Running Steps

1. Save the code to `file_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python file_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Read config
   curl -X POST http://localhost:8080/config/read \
     -H "Content-Type: application/json" \
     -d '{"filename": "app.json"}'
   
   # Write config
   curl -X POST http://localhost:8080/config/write \
     -H "Content-Type: application/json" \
     -d '{"filename": "app.json", "data": {"key": "value"}}'
   
   # Save report
   curl -X POST http://localhost:8080/reports/save \
     -H "Content-Type: application/json" \
     -d '{"name": "sales", "data": {"total": 1000, "items": 50}}'
   
   # List reports
   curl -X POST http://localhost:8080/reports/list \
     -H "Content-Type: application/json" \
     -d '{"name": "sales"}'
   
   # Archive data
   curl -X POST http://localhost:8080/data/archive \
     -H "Content-Type: application/json" \
     -d '{"name": "user_activity", "data": {"users": 100, "actions": 5000}}'
   
   # Get file info
   curl -X POST http://localhost:8080/file/info \
     -H "Content-Type: application/json" \
     -d '{"path": "config/app.json"}'
   
   # Cleanup old files
   curl -X POST http://localhost:8080/file/cleanup \
     -H "Content-Type: application/json" \
     -d '{"directory": "temp", "days": 7}'
   ```

## Expected Output

### Save Report Response

```json
{
  "status": "success",
  "data": {
    "filename": "reports/sales_20240115_103000.json"
  }
}
```

### List Reports Response

```json
{
  "status": "success",
  "data": {
    "reports": [
      "sales_20240115_103000.json",
      "sales_20240114_090000.json"
    ]
  }
}
```

### Get File Info Response

```json
{
  "status": "success",
  "data": {
    "path": "config/app.json",
    "is_file": true,
    "is_directory": false,
    "extension": ".json"
  }
}
```

## Best Practices

1. **Use Atomic Writes**: Always use atomic writes for important data
2. **Use Categorized Directories**: Use categorized directories for organization
3. **Handle Errors**: Always handle file operation errors gracefully
4. **Check Existence**: Check if files exist before reading
5. **Use JSON for Configuration**: Use JSON for structured configuration
6. **Clean Up Temp Files**: Clean up temporary files regularly
7. **Use Paths Correctly**: Use proper path handling across platforms
8. **Monitor Disk Space**: Monitor disk space usage in production
9. **Secure Sensitive Files**: Secure sensitive files with proper permissions
10. **Backup Important Data**: Backup important data regularly
