<div align="center">

# Storage API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

storage模块提供文件存储与对象存储功能，支持本地文件系统、云存储服务和分布式存储。

## 模块概述

</div>

storage模块包含以下子模块：

- **local**: 本地文件系统存储
- **s3**: Amazon S3兼容存储
- **azure**: Azure Blob存储
- **gcs**: Google Cloud Storage
- **minio**: MinIO对象存储
- **distributed**: 分布式存储
- **encryption**: 存储加密
- **compression**: 存储压缩
- **metadata**: 元数据管理

<div align="center">

## 核心组件

</div>

### DMSCStorageManager

存储管理器主接口，提供统一的存储访问。

<div align="center">

#### 方法表

</div>

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `put(key, data)` | 上传数据 | `key: &str`, `data: &[u8]` | `DMSCResult<()>` |
| `put_stream(key, stream)` | 流式上传 | `key: &str`, `stream: impl AsyncRead` | `DMSCResult<()>` |
| `get(key)` | 下载数据 | `key: &str` | `DMSCResult<Vec<u8>>` |
| `get_stream(key)` | 流式下载 | `key: &str` | `DMSCResult<impl AsyncRead>` |
| `delete(key)` | 删除对象 | `key: &str` | `DMSCResult<()>` |
| `exists(key)` | 检查存在 | `key: &str` | `DMSCResult<bool>` |
| `metadata(key)` | 获取元数据 | `key: &str` | `DMSCResult<DMSCStorageMetadata>` |
| `list(prefix)` | 列出对象 | `prefix: &str` | `DMSCResult<Vec<DMSCStorageObject>>` |
| `copy(source, dest)` | 复制对象 | `source: &str`, `dest: &str` | `DMSCResult<()>` |
| `move_object(source, dest)` | 移动对象 | `source: &str`, `dest: &str` | `DMSCResult<()>` |

<div align="center">

### DMSCStorageConfig

</div>

存储配置结构体。

<div align="center">

#### 字段表

</div>

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `backend` | `DMSCStorageBackend` | 存储后端 | `Local` |
| `bucket` | `String` | 存储桶名称 | `"default"` |
| `region` | `String` | 存储区域 | `"us-east-1"` |
| `endpoint` | `String` | 存储端点 | 后端默认值 |
| `access_key` | `String` | 访问密钥 | 可选 |
| `secret_key` | `String` | 密钥 | 可选 |
| `encryption` | `DMSCStorageEncryption` | 加密配置 | 可选 |
| `compression` | `DMSCStorageCompression` | 压缩配置 | 可选 |
| `max_file_size` | `u64` | 最大文件大小 | `100MB` |
| `chunk_size` | `u64` | 分块大小 | `5MB` |

<div align="center">

### DMSCStorageBackend

</div>

存储后端枚举。

<div align="center">

#### 变体表

</div>

| 变体 | 描述 |
|:--------|:-------------|
| `Local` | 本地文件系统 |
| `S3` | Amazon S3兼容存储 |
| `Azure` | Azure Blob存储 |
| `GCS` | Google Cloud Storage |
| `MinIO` | MinIO对象存储 |
| `Distributed` | 分布式存储 |

<div align="center">

## 使用示例

</div>

### 基本文件操作

```python
from dms import DMSContext

# 上传文件
file_content = b"Hello, World! This is a test file."
await ctx.storage().put("documents/test.txt", file_content)
ctx.log().info("File uploaded successfully")

# 下载文件
data = await ctx.storage().get("documents/test.txt")
content = data.decode('utf-8')
ctx.log().info(f"Downloaded content: {content}")

# 检查文件是否存在
if await ctx.storage().exists("documents/test.txt"):
    ctx.log().info("File exists")
else:
    ctx.log().info("File does not exist")

# 获取元数据
metadata = await ctx.storage().metadata("documents/test.txt")
ctx.log().info(f"File metadata - Size: {metadata.size}, Modified: {metadata.last_modified}")

# 列出对象
objects = await ctx.storage().list("documents/")
for obj in objects:
    ctx.log().info(f"Found object: {obj.key} ({obj.size} bytes)")

# 复制对象
await ctx.storage().copy("documents/test.txt", "documents/test_backup.txt")
ctx.log().info("File copied successfully")

# 移动对象
await ctx.storage().move_object("documents/test.txt", "archive/test.txt")
ctx.log().info("File moved successfully")

# 删除对象
await ctx.storage().delete("documents/test_backup.txt")
ctx.log().info("File deleted successfully")
```

### 存储配置

```python
from dms import DMSCStorageConfig, DMSCStorageBackend

# 本地存储配置
local_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.Local,
    bucket="local_files",
    endpoint="/var/lib/dms/storage",
    max_file_size=1024 * 1024 * 1024,  # 1GB
    chunk_size=5 * 1024 * 1024,  # 5MB
)

# S3配置
s3_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.S3,
    bucket="my-app-bucket",
    region="us-west-2",
    endpoint="https://s3.amazonaws.com",
    access_key="AKIAIOSFODNN7EXAMPLE",
    secret_key="wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
    max_file_size=5 * 1024 * 1024 * 1024,  # 5GB
    chunk_size=10 * 1024 * 1024,  # 10MB
)

# Azure Blob配置
azure_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.Azure,
    bucket="my-container",
    endpoint="https://mystorageaccount.blob.core.windows.net",
    access_key="DefaultEndpointsProtocol=https;AccountName=mystorageaccount;AccountKey=example...",
)

# Google Cloud Storage配置
gcs_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.GCS,
    bucket="my-gcs-bucket",
    region="us-central1",
    endpoint="https://storage.googleapis.com",
    access_key="my-service-account-key",
)
```

### 多文件上传

```python
from dms import DMSCError
import datetime

async def handle_file_upload(files):
    uploaded_keys = []
    
    for file in files:
        key = f"uploads/{datetime.datetime.now().strftime('%Y/%m/%d')}/{file.filename}"
        
        # 验证文件类型
        if not is_allowed_file_type(file.content_type):
            raise DMSCError.validation(f"File type not allowed: {file.content_type}")
        
        # 验证文件大小
        if file.size > 10 * 1024 * 1024:  # 10MB limit
            raise DMSCError.validation("File too large")
        
        # 上传文件
        await ctx.storage().put(key, file.content)
        uploaded_keys.append(key)
        
        ctx.log().info(f"Uploaded file: {file.filename} ({file.size} bytes)")
    
    return uploaded_keys

def is_allowed_file_type(content_type):
    allowed_types = [
        "image/jpeg", "image/png", "image/gif",
        "application/pdf", "text/plain",
        "application/msword", 
        "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    ]
    return content_type in allowed_types
```

### 分块上传

```python
import asyncio

# 初始化分块上传
upload_id = await ctx.storage().init_multipart_upload("large_file.zip")
ctx.log().info(f"Started multipart upload: {upload_id}")

# 上传分块
part_number = 1
uploaded_parts = []

with open("large_file.zip", "rb") as file:
    while True:
        chunk = file.read(5 * 1024 * 1024)  # 5MB chunks
        if not chunk:
            break
        
        part_etag = await ctx.storage().upload_part(
            "large_file.zip",
            upload_id,
            part_number,
            chunk
        )
        
        uploaded_parts.append({
            "part_number": part_number,
            "etag": part_etag,
        })
        
        ctx.log().info(f"Uploaded part {part_number} ({len(chunk)} bytes)")
        part_number += 1

# 完成分块上传
await ctx.storage().complete_multipart_upload("large_file.zip", upload_id, uploaded_parts)
ctx.log().info("Multipart upload completed")
```

### 断点续传下载

```python
import os
import aiofiles

async def resumable_download(key, output_path):
    metadata = await ctx.storage().metadata(key)
    total_size = metadata.size
    
    start_byte = 0
    
    # 检查是否已有部分下载
    if os.path.exists(output_path):
        start_byte = os.path.getsize(output_path)
        ctx.log().info(f"Resuming download from byte {start_byte}")
    
    if start_byte >= total_size:
        ctx.log().info("File already fully downloaded")
        return
    
    async with aiofiles.open(output_path, "ab") as output_file:
        chunk_size = 1024 * 1024  # 1MB chunks
        current_byte = start_byte
        
        while current_byte < total_size:
            end_byte = min(current_byte + chunk_size - 1, total_size - 1)
            range_str = f"bytes={current_byte}-{end_byte}"
            
            chunk_data = await ctx.storage().get_range(key, range_str)
            await output_file.write(chunk_data)
            
            current_byte = end_byte + 1
            
            progress = (current_byte / total_size) * 100.0
            ctx.log().info(f"Download progress: {progress:.1f}%")
    
    ctx.log().info("Download completed")
```

### 临时URL

```python
from datetime import timedelta

# 生成临时下载URL
download_url = await ctx.storage().generate_presigned_url(
    "documents/confidential.pdf",
    "GET",
    timedelta(hours=1)  # 1小时有效期
)
ctx.log().info(f"Generated presigned URL: {download_url}")

# 生成临时上传URL
upload_url = await ctx.storage().generate_presigned_url(
    "uploads/user_upload.jpg",
    "PUT",
    timedelta(minutes=30)  # 30分钟有效期
)
ctx.log().info(f"Generated presigned upload URL: {upload_url}")
```

### 元数据管理

```python
# 上传带元数据的文件
metadata = {
    "content-type": "application/pdf",
    "author": "John Doe",
    "department": "Engineering",
    "classification": "internal",
}

await ctx.storage().put_with_metadata(
    "documents/report.pdf",
    file_content,
    metadata
)

# 更新元数据
new_metadata = {
    "reviewed": "true",
    "reviewer": "Jane Smith",
    "review_date": datetime.datetime.now().isoformat(),
}

await ctx.storage().update_metadata("documents/report.pdf", new_metadata)

# 获取元数据
metadata = await ctx.storage().metadata("documents/report.pdf")
for key, value in metadata.metadata.items():
    ctx.log().info(f"{key}: {value}")
```

### 标签管理

```python
# 设置对象标签
tags = [
    "project:alpha",
    "team:engineering",
    "environment:production",
    "cost-center:1234",
]

await ctx.storage().set_tags("documents/report.pdf", tags)

# 按标签搜索
tagged_objects = await ctx.storage().find_by_tag("team:engineering")
for obj in tagged_objects:
    ctx.log().info(f"Found object with tag: {obj.key}")

# 删除标签
await ctx.storage().remove_tag("documents/report.pdf", "project:alpha")
```

### 存储加密

```python
from dms import DMSCStorageEncryption, DMSCStorageEncryptionAlgorithm

# 配置客户端加密
encryption_config = DMSCStorageEncryption.ClientSide(
    algorithm=DMSCStorageEncryptionAlgorithm.AES256GCM,
    key_id="my-encryption-key",
    key_rotation=timedelta(days=90),
)

storage_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.S3,
    bucket="encrypted-bucket",
    encryption=encryption_config,
)

# 上传加密文件
sensitive_data = b"This is sensitive information that needs encryption"
await ctx.storage().put_with_encryption("confidential/data.txt", sensitive_data)

# 下载并解密
decrypted_data = await ctx.storage().get_and_decrypt("confidential/data.txt")
content = decrypted_data.decode('utf-8')
ctx.log().info(f"Decrypted content: {content}")
```

### 存储压缩

```python
from dms import DMSCStorageCompression, DMSCStorageCompressionAlgorithm

# 配置自动压缩
compression_config = DMSCStorageCompression(
    enabled=True,
    algorithm=DMSCStorageCompressionAlgorithm.Gzip,
    threshold=1024,  # 1KB以上文件才压缩
    extensions=["txt", "json", "xml", "csv"],
)

storage_config = DMSCStorageConfig(
    backend=DMSCStorageBackend.S3,
    bucket="compressed-bucket",
    compression=compression_config,
)

# 上传会自动压缩
large_text = "A" * 10000  # 10KB文本
await ctx.storage().put("large_text_file.txt", large_text.encode('utf-8'))

# 下载会自动解压
decompressed_data = await ctx.storage().get("large_text_file.txt")
ctx.log().info(f"Decompressed size: {len(decompressed_data)} bytes")
```

### 生命周期管理

```python
from dms import DMSCStorageLifecycleRule, DMSCStorageClass

# 配置生命周期规则
lifecycle_rules = [
    DMSCStorageLifecycleRule(
        name="old_files_to_ia",
        prefix="logs/",
        transitions=[
            {"days": 30, "storage_class": DMSCStorageClass.InfrequentAccess},
            {"days": 90, "storage_class": DMSCStorageClass.Glacier},
        ],
        expiration={"days": 365},
    ),
    DMSCStorageLifecycleRule(
        name="temp_files_cleanup",
        prefix="temp/",
        transitions=[],
        expiration={"days": 7},
    ),
]

await ctx.storage().set_lifecycle_rules(lifecycle_rules)

# 手动转换存储类别
await ctx.storage().change_storage_class("old_document.pdf", DMSCStorageClass.Glacier)
```

### 版本控制

```python
# 启用版本控制
await ctx.storage().enable_versioning("my-bucket")

# 上传多个版本
await ctx.storage().put("documents/report.pdf", b"Version 1 content")
await ctx.storage().put("documents/report.pdf", b"Version 2 content")
await ctx.storage().put("documents/report.pdf", b"Version 3 content")

# 列出所有版本
versions = await ctx.storage().list_versions("documents/report.pdf")
for version in versions:
    ctx.log().info(f"Version {version.version_id} ({version.is_latest}): {version.size} bytes, modified {version.last_modified}")

# 获取特定版本
version_data = await ctx.storage().get_version("documents/report.pdf", "version_123")

# 恢复到特定版本
await ctx.storage().restore_version("documents/report.pdf", "version_123")

# 删除特定版本
await ctx.storage().delete_version("documents/report.pdf", "version_456")
```

### 监控与统计

```python
# 获取存储统计
stats = await ctx.storage().get_storage_stats()
ctx.log().info(f"Storage stats - Total objects: {stats.total_objects}, Total size: {stats.total_size} bytes, Average size: {stats.average_size} bytes")

# 获取桶统计
bucket_stats = await ctx.storage().get_bucket_stats("my-bucket")
ctx.log().info(f"Bucket stats - Objects: {bucket_stats.object_count}, Size: {bucket_stats.total_size} bytes, Oldest object: {bucket_stats.oldest_object}, Newest object: {bucket_stats.newest_object}")

# 按前缀统计
prefix_stats = await ctx.storage().get_prefix_stats("documents/")
for prefix, stats in prefix_stats.items():
    ctx.log().info(f"Prefix {prefix}: {stats.object_count} objects, {stats.total_size} bytes")
```

<div align="center">

## 错误处理

</div>

### 存储错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `STORAGE_CONNECTION_ERROR` | 存储连接错误 |
| `STORAGE_NOT_FOUND` | 对象不存在 |
| `STORAGE_PERMISSION_DENIED` | 权限不足 |
| `STORAGE_QUOTA_EXCEEDED` | 存储配额超限 |
| `STORAGE_ENCRYPTION_ERROR` | 加密错误 |
| `STORAGE_COMPRESSION_ERROR` | 压缩错误 |

### 错误处理示例

```python
from dms import DMSCError

try:
    data = await ctx.storage().get("important_file.pdf")
    ctx.log().info("File retrieved successfully")
    # 处理文件数据
except DMSCError as e:
    if e.code == "STORAGE_NOT_FOUND":
        ctx.log().warn("File not found, using default")
        # 使用默认文件或返回错误
        default_data = get_default_file_data()
        # ...
    elif e.code == "STORAGE_CONNECTION_ERROR":
        ctx.log().error("Storage connection failed")
        # 尝试备用存储
        await ctx.storage().use_backup_storage()
        # 重试操作
    else:
        ctx.log().error(f"Storage error: {e}")
        raise e
```

<div align="center">

## 最佳实践

</div>

1. **使用流式操作**: 大文件使用流式上传下载，避免内存问题
2. **合理设置分块大小**: 根据网络条件和文件大小调整分块大小
3. **启用版本控制**: 重要数据启用版本控制，防止意外删除
4. **使用生命周期管理**: 自动管理旧数据的存储类别和清理
5. **加密敏感数据**: 对敏感数据进行客户端加密
6. **压缩文本数据**: 对文本数据启用压缩，节省存储空间
7. **监控存储使用**: 定期监控存储使用情况和性能指标
8. **备份重要数据**: 关键数据配置跨区域复制或备份
9. **使用临时URL**: 提供文件访问时使用临时URL，避免暴露凭证
10. **验证文件类型**: 上传时验证文件类型和内容，防止恶意文件

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置