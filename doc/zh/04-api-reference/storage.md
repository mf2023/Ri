<div align="center">

# Storage API参考

**Version: 0.1.6**

**Last modified date: 2026-01-24**

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

#### 方法

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

#### 使用示例

```rust
use dmsc::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// 上传文件
let file_content = b"Hello, World! This is a test file.";
ctx.storage().put("documents/test.txt", file_content).await?;
ctx.log().info("File uploaded successfully");

// 流式上传大文件
let mut file = File::open("large_file.zip").await?;
ctx.storage().put_stream("uploads/large_file.zip", &mut file).await?;
ctx.log().info("Large file uploaded successfully");

// 下载文件
let data = ctx.storage().get("documents/test.txt").await?;
let content = String::from_utf8(data)?;
ctx.log().info(format!("Downloaded content: {}", content));

// 流式下载
let mut stream = ctx.storage().get_stream("uploads/large_file.zip").await?;
let mut output_file = File::create("downloaded_file.zip").await?;
tokio::io::copy(&mut stream, &mut output_file).await?;

// 检查文件是否存在
if ctx.storage().exists("documents/test.txt").await? {
    ctx.log().info("File exists");
} else {
    ctx.log().info("File does not exist");
}

// 获取元数据
let metadata = ctx.storage().metadata("documents/test.txt").await?;
ctx.log().info(format!(
    "File metadata - Size: {}, Modified: {}, ETag: {}",
    metadata.size, metadata.last_modified, metadata.etag
));

// 列出对象
let objects = ctx.storage().list("documents/").await?;
for obj in objects {
    ctx.log().info(format!("Found object: {} ({} bytes)", obj.key, obj.size));
}

// 复制对象
ctx.storage().copy("documents/test.txt", "documents/test_backup.txt").await?;
ctx.log().info("File copied successfully");

// 移动对象
ctx.storage().move_object("documents/test.txt", "archive/test.txt").await?;
ctx.log().info("File moved successfully");

// 删除对象
ctx.storage().delete("documents/test_backup.txt").await?;
ctx.log().info("File deleted successfully");
```

### DMSCStorageConfig

存储配置结构体。

#### 字段

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

#### 配置示例

```rust
use dmsc::prelude::*;

// 本地存储配置
let local_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::Local,
    bucket: "local_files".to_string(),
    endpoint: "/var/lib/dms/storage".to_string(),
    max_file_size: 1024 * 1024 * 1024, // 1GB
    chunk_size: 5 * 1024 * 1024, // 5MB
    ..Default::default()
};

// S3配置
let s3_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::S3,
    bucket: "my-app-bucket".to_string(),
    region: "us-west-2".to_string(),
    endpoint: "https://s3.amazonaws.com".to_string(),
    access_key: "AKIAIOSFODNN7EXAMPLE".to_string(),
    secret_key: "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY".to_string(),
    encryption: Some(DMSCStorageEncryption::ServerSide),
    max_file_size: 5 * 1024 * 1024 * 1024, // 5GB
    chunk_size: 10 * 1024 * 1024, // 10MB
};

// Azure Blob配置
let azure_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::Azure,
    bucket: "my-container".to_string(),
    endpoint: "https://mystorageaccount.blob.core.windows.net".to_string(),
    access_key: "DefaultEndpointsProtocol=https;AccountName=mystorageaccount;AccountKey=example...".to_string(),
    encryption: Some(DMSCStorageEncryption::ClientSide),
    ..Default::default()
};

// Google Cloud Storage配置
let gcs_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::GCS,
    bucket: "my-gcs-bucket".to_string(),
    region: "us-central1".to_string(),
    endpoint: "https://storage.googleapis.com".to_string(),
    access_key: "my-service-account-key".to_string(),
    ..Default::default()
};
```

### DMSCStorageBackend

存储后端枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Local` | 本地文件系统 |
| `S3` | Amazon S3兼容存储 |
| `Azure` | Azure Blob存储 |
| `GCS` | Google Cloud Storage |
| `MinIO` | MinIO对象存储 |
| `Distributed` | 分布式存储 |

<div align="center">

## 文件上传

</div>

### 多文件上传

```rust
use dmsc::prelude::*;
use tokio::fs::File;

// 处理多文件上传
async fn handle_file_upload(files: Vec<UploadFile>) -> DMSCResult<Vec<String>> {
    let mut uploaded_keys = Vec::new();
    
    for file in files {
        let key = format!("uploads/{}/{}", chrono::Utc::now().format("%Y/%m/%d"), file.filename);
        
        // 验证文件类型
        if !is_allowed_file_type(&file.content_type) {
            return Err(DMSCError::validation(format!("File type not allowed: {}", file.content_type)));
        }
        
        // 验证文件大小
        if file.size > 10 * 1024 * 1024 { // 10MB limit
            return Err(DMSCError::validation("File too large".to_string()));
        }
        
        // 上传文件
        ctx.storage().put(&key, &file.content).await?;
        uploaded_keys.push(key);
        
        ctx.log().info(format!("Uploaded file: {} ({} bytes)", file.filename, file.size));
    }
    
    Ok(uploaded_keys)
}

// 结构体定义
struct UploadFile {
    filename: String,
    content_type: String,
    size: usize,
    content: Vec<u8>,
}

fn is_allowed_file_type(content_type: &str) -> bool {
    matches!(
        content_type,
        "image/jpeg" | "image/png" | "image/gif" |
        "application/pdf" | "text/plain" |
        "application/msword" | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
    )
}
```

### 分块上传

```rust
use dmsc::prelude::*;
use tokio::io::AsyncReadExt;

// 初始化分块上传
let upload_id = ctx.storage().init_multipart_upload("large_file.zip").await?;
ctx.log().info(format!("Started multipart upload: {}", upload_id));

// 上传分块
let mut file = File::open("large_file.zip").await?;
let mut part_number = 1;
let mut uploaded_parts = Vec::new();

loop {
    let mut chunk = vec![0u8; 5 * 1024 * 1024]; // 5MB chunks
    let bytes_read = file.read(&mut chunk).await?;
    
    if bytes_read == 0 {
        break;
    }
    
    chunk.truncate(bytes_read);
    
    let part_etag = ctx.storage().upload_part(
        "large_file.zip",
        &upload_id,
        part_number,
        &chunk
    ).await?;
    
    uploaded_parts.push(DMSCUploadedPart {
        part_number,
        etag: part_etag,
    });
    
    ctx.log().info(format!("Uploaded part {} ({} bytes)", part_number, bytes_read));
    part_number += 1;
}

// 完成分块上传
ctx.storage().complete_multipart_upload("large_file.zip", &upload_id, &uploaded_parts).await?;
ctx.log().info("Multipart upload completed");
```
<div align="center">

## 文件下载

</div>

### 断点续传

```rust
use dmsc::prelude::*;
use tokio::io::AsyncWriteExt;

// 断点续传下载
async fn resumable_download(key: &str, output_path: &str) -> DMSCResult<()> {
    let metadata = ctx.storage().metadata(key).await?;
    let total_size = metadata.size;
    
    let mut start_byte = 0u64;
    
    // 检查是否已有部分下载
    if let Ok(metadata) = tokio::fs::metadata(output_path).await {
        start_byte = metadata.len();
        ctx.log().info(format!("Resuming download from byte {}", start_byte));
    }
    
    if start_byte >= total_size {
        ctx.log().info("File already fully downloaded");
        return Ok(());
    }
    
    let mut output_file = File::options()
        .create(true)
        .append(true)
        .open(output_path)
        .await?;
    
    let chunk_size = 1024 * 1024; // 1MB chunks
    let mut current_byte = start_byte;
    
    while current_byte < total_size {
        let end_byte = std::cmp::min(current_byte + chunk_size - 1, total_size - 1);
        let range = format!("bytes={}-{}", current_byte, end_byte);
        
        let chunk_data = ctx.storage().get_range(key, &range).await?;
        output_file.write_all(&chunk_data).await?;
        
        current_byte = end_byte + 1;
        
        let progress = (current_byte as f64 / total_size as f64) * 100.0;
        ctx.log().info(format!("Download progress: {:.1}%", progress));
    }
    
    ctx.log().info("Download completed");
    Ok(())
}
```

### 临时URL

```rust
use dmsc::prelude::*;

// 生成临时下载URL
let download_url = ctx.storage().generate_presigned_url(
    "documents/confidential.pdf",
    DMSCPresignedUrlOperation::Get,
    Duration::from_hours(1) // 1小时有效期
).await?;

ctx.log().info(format!("Generated presigned URL: {}", download_url));

// 生成临时上传URL
let upload_url = ctx.storage().generate_presigned_url(
    "uploads/user_upload_{}.jpg",
    DMSCPresignedUrlOperation::Put,
    Duration::from_minutes(30) // 30分钟有效期
).await?;

ctx.log().info(format!("Generated presigned upload URL: {}", upload_url));
```
<div align="center">

## 元数据管理

</div>

### 对象元数据

```rust
use dmsc::prelude::*;

// 上传带元数据的文件
let mut metadata = HashMap::new();
metadata.insert("content-type".to_string(), "application/pdf".to_string());
metadata.insert("author".to_string(), "John Doe".to_string());
metadata.insert("department".to_string(), "Engineering".to_string());
metadata.insert("classification".to_string(), "internal".to_string());

ctx.storage().put_with_metadata(
    "documents/report.pdf",
    file_content,
    &metadata
).await?;

// 更新元数据
let mut new_metadata = HashMap::new();
new_metadata.insert("reviewed".to_string(), "true".to_string());
new_metadata.insert("reviewer".to_string(), "Jane Smith".to_string());
new_metadata.insert("review_date".to_string(), chrono::Utc::now().to_rfc3339());

ctx.storage().update_metadata("documents/report.pdf", &new_metadata).await?;

// 获取元数据
let metadata = ctx.storage().metadata("documents/report.pdf").await?;
for (key, value) in &metadata.metadata {
    ctx.log().info(format!("{}: {}", key, value));
}
```

### 标签管理

```rust
use dmsc::prelude::*;

// 设置对象标签
let tags = vec![
    "project:alpha".to_string(),
    "team:engineering".to_string(),
    "environment:production".to_string(),
    "cost-center:1234".to_string(),
];

ctx.storage().set_tags("documents/report.pdf", &tags).await?;

// 按标签搜索
let tagged_objects = ctx.storage().find_by_tag("team:engineering").await?;
for obj in tagged_objects {
    ctx.log().info(format!("Found object with tag: {}", obj.key));
}

// 删除标签
ctx.storage().remove_tag("documents/report.pdf", "project:alpha").await?;
```
<div align="center">

## 存储加密

</div>

### 客户端加密

```rust
use dmsc::prelude::*;

// 配置客户端加密
let encryption_config = DMSCStorageEncryption::ClientSide {
    algorithm: DMSCStorageEncryptionAlgorithm::AES256GCM,
    key_id: "my-encryption-key".to_string(),
    key_rotation: Duration::from_days(90),
};

let mut storage_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::S3,
    bucket: "encrypted-bucket".to_string(),
    encryption: Some(encryption_config),
    ..Default::default()
};

// 上传加密文件
let sensitive_data = b"This is sensitive information that needs encryption";
ctx.storage().put_with_encryption("confidential/data.txt", sensitive_data).await?;

// 下载并解密
let decrypted_data = ctx.storage().get_and_decrypt("confidential/data.txt").await?;
let content = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted content: {}", content));
```

### 密钥管理

```rust
use dmsc::prelude::*;

// 生成数据加密密钥
let data_key = ctx.storage().generate_data_encryption_key()?;

// 使用KMS加密密钥
let kms_config = DMSCKMSConfig {
    key_id: "arn:aws:kms:us-west-2:123456789012:key/12345678-1234-1234-1234-123456789012",
    region: "us-west-2".to_string(),
    endpoint: Some("https://kms.us-west-2.amazonaws.com".to_string()),
};

let encrypted_key = ctx.storage().encrypt_with_kms(&data_key, &kms_config).await?;
let decrypted_key = ctx.storage().decrypt_with_kms(&encrypted_key, &kms_config).await?;
```

<div align="center">

## 存储压缩

</div>

### 自动压缩

```rust
use dmsc::prelude::*;

// 配置自动压缩
let compression_config = DMSCStorageCompression {
    enabled: true,
    algorithm: DMSCStorageCompressionAlgorithm::Gzip,
    threshold: 1024, // 1KB以上文件才压缩
    extensions: vec!["txt".to_string(), "json".to_string(), "xml".to_string(), "csv".to_string()],
};

let mut storage_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::S3,
    bucket: "compressed-bucket".to_string(),
    compression: Some(compression_config),
    ..Default::default()
};

// 上传会自动压缩
let large_text = "A".repeat(10000); // 10KB文本
ctx.storage().put("large_text_file.txt", large_text.as_bytes()).await?;

// 下载会自动解压
let decompressed_data = ctx.storage().get("large_text_file.txt").await?;
ctx.log().info(format!("Decompressed size: {} bytes", decompressed_data.len()));
```

<div align="center">

## 生命周期管理

</div>

### 存储类别转换

```rust
use dmsc::prelude::*;

// 配置生命周期规则
let lifecycle_rules = vec![
    DMSCStorageLifecycleRule {
        name: "old_files_to_ia".to_string(),
        prefix: "logs/".to_string(),
        transitions: vec![
            DMSCTransition {
                days: 30,
                storage_class: DMSCStorageClass::InfrequentAccess,
            },
            DMSCTransition {
                days: 90,
                storage_class: DMSCStorageClass::Glacier,
            },
        ],
        expiration: Some(DMSCExpiration { days: 365 }),
    },
    DMSCStorageLifecycleRule {
        name: "temp_files_cleanup".to_string(),
        prefix: "temp/".to_string(),
        transitions: vec![],
        expiration: Some(DMSCExpiration { days: 7 }),
    },
];

ctx.storage().set_lifecycle_rules(&lifecycle_rules).await?;

// 手动转换存储类别
ctx.storage().change_storage_class("old_document.pdf", DMSCStorageClass::Glacier).await?;
```

<div align="center">

## 版本控制

</div>

### 对象版本管理

```rust
use dmsc::prelude::*;

// 启用版本控制
ctx.storage().enable_versioning("my-bucket").await?;

// 上传多个版本
ctx.storage().put("documents/report.pdf", b"Version 1 content").await?;
ctx.storage().put("documents/report.pdf", b"Version 2 content").await?;
ctx.storage().put("documents/report.pdf", b"Version 3 content").await?;

// 列出所有版本
let versions = ctx.storage().list_versions("documents/report.pdf").await?;
for version in versions {
    ctx.log().info(format!(
        "Version {} ({}): {} bytes, modified {}",
        version.version_id, version.is_latest, version.size, version.last_modified
    ));
}

// 获取特定版本
let version_data = ctx.storage().get_version("documents/report.pdf", "version_123").await?;

// 恢复到特定版本
ctx.storage().restore_version("documents/report.pdf", "version_123").await?;

// 删除特定版本
ctx.storage().delete_version("documents/report.pdf", "version_456").await?;
```

<div align="center">

## 监控与统计

</div>

### 存储统计

```rust
use dmsc::prelude::*;

// 获取存储统计
let stats = ctx.storage().get_storage_stats().await?;
ctx.log().info(format!(
    "Storage stats - Total objects: {}, Total size: {} bytes, Average size: {} bytes",
    stats.total_objects, stats.total_size, stats.average_size
));

// 获取桶统计
let bucket_stats = ctx.storage().get_bucket_stats("my-bucket").await?;
ctx.log().info(format!(
    "Bucket stats - Objects: {}, Size: {} bytes, Oldest object: {}, Newest object: {}",
    bucket_stats.object_count, bucket_stats.total_size, bucket_stats.oldest_object, bucket_stats.newest_object
));

// 按前缀统计
let prefix_stats = ctx.storage().get_prefix_stats("documents/").await?;
for (prefix, stats) in prefix_stats {
    ctx.log().info(format!(
        "Prefix {}: {} objects, {} bytes",
        prefix, stats.object_count, stats.total_size
    ));
}
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

```rust
use dmsc::prelude::*;

match ctx.storage().get("important_file.pdf").await {
    Ok(data) => {
        ctx.log().info("File retrieved successfully");
        // 处理文件数据
    }
    Err(DMSCError { code, .. }) if code == "STORAGE_NOT_FOUND" => {
        ctx.log().warn("File not found, using default");
        // 使用默认文件或返回错误
        let default_data = get_default_file_data();
        // ...
    }
    Err(DMSCError { code, .. }) if code == "STORAGE_CONNECTION_ERROR" => {
        ctx.log().error("Storage connection failed");
        // 尝试备用存储
        ctx.storage().use_backup_storage()?;
        // 重试操作
    }
    Err(e) => {
        ctx.log().error(format!("Storage error: {}", e));
        return Err(e);
    }
}
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
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信