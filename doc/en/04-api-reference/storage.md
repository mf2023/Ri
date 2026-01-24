<div align="center">

# Storage API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-24**

The storage module provides file storage and object storage functionality, supporting local file systems, cloud storage services, and distributed storage.

## Module Overview

</div>

The storage module includes the following sub-modules:

- **local**: Local file system storage
- **s3**: Amazon S3-compatible storage
- **azure**: Azure Blob storage
- **gcs**: Google Cloud Storage
- **minio**: MinIO object storage
- **distributed**: Distributed storage
- **encryption**: Storage encryption
- **compression**: Storage compression
- **metadata**: Metadata management

<div align="center">

## Core Components

</div>

### DMSCStorageManager

Storage manager main interface, providing unified storage access.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `put(key, data)` | Upload data | `key: &str`, `data: &[u8]` | `DMSCResult<()>` |
| `put_stream(key, stream)` | Stream upload | `key: &str`, `stream: impl AsyncRead` | `DMSCResult<()>` |
| `get(key)` | Download data | `key: &str` | `DMSCResult<Vec<u8>>` |
| `get_stream(key)` | Stream download | `key: &str` | `DMSCResult<impl AsyncRead>` |
| `delete(key)` | Delete object | `key: &str` | `DMSCResult<()>` |
| `exists(key)` | Check existence | `key: &str` | `DMSCResult<bool>` |
| `metadata(key)` | Get metadata | `key: &str` | `DMSCResult<DMSCStorageMetadata>` |
| `list(prefix)` | List objects | `prefix: &str` | `DMSCResult<Vec<DMSCStorageObject>>` |
| `copy(source, dest)` | Copy object | `source: &str`, `dest: &str` | `DMSCResult<()>` |
| `move_object(source, dest)` | Move object | `source: &str`, `dest: &str` | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

// Upload file
let file_content = b"Hello, World! This is a test file.";
ctx.storage().put("documents/test.txt", file_content).await?;
ctx.log().info("File uploaded successfully");

// Stream upload large file
let mut file = File::open("large_file.zip").await?;
ctx.storage().put_stream("uploads/large_file.zip", &mut file).await?;
ctx.log().info("Large file uploaded successfully");

// Download file
let data = ctx.storage().get("documents/test.txt").await?;
let content = String::from_utf8(data)?;
ctx.log().info(format!("Downloaded content: {}", content));

// Stream download
let mut stream = ctx.storage().get_stream("uploads/large_file.zip").await?;
let mut output_file = File::create("downloaded_file.zip").await?;
tokio::io::copy(&mut stream, &mut output_file).await?;

// Check if file exists
if ctx.storage().exists("documents/test.txt").await? {
    ctx.log().info("File exists");
} else {
    ctx.log().info("File does not exist");
}

// Get metadata
let metadata = ctx.storage().metadata("documents/test.txt").await?;
ctx.log().info(format!(
    "File metadata - Size: {}, Modified: {}, ETag: {}",
    metadata.size, metadata.last_modified, metadata.etag
));

// List objects
let objects = ctx.storage().list("documents/").await?;
for obj in objects {
    ctx.log().info(format!("Found object: {} ({} bytes)", obj.key, obj.size));
}

// Copy object
ctx.storage().copy("documents/test.txt", "documents/test_backup.txt").await?;
ctx.log().info("File copied successfully");

// Move object
ctx.storage().move_object("documents/test.txt", "archive/test.txt").await?;
ctx.log().info("File moved successfully");

// Delete object
ctx.storage().delete("documents/test_backup.txt").await?;
ctx.log().info("File deleted successfully");
```

### DMSCStorageConfig

Storage configuration struct.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `backend` | `DMSCStorageBackend` | Storage backend | `Local` |
| `bucket` | `String` | Bucket name | `"default"` |
| `region` | `String` | Storage region | `"us-east-1"` |
| `endpoint` | `String` | Storage endpoint | Backend default |
| `access_key` | `String` | Access key | Optional |
| `secret_key` | `String` | Secret key | Optional |
| `encryption` | `DMSCStorageEncryption` | Encryption configuration | Optional |
| `compression` | `DMSCStorageCompression` | Compression configuration | Optional |
| `max_file_size` | `u64` | Maximum file size | `100MB` |
| `chunk_size` | `u64` | Chunk size | `5MB` |

#### Configuration Example

```rust
use dmsc::prelude::*;

// Local storage configuration
let local_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::Local,
    bucket: "local_files".to_string(),
    endpoint: "/var/lib/dms/storage".to_string(),
    max_file_size: 1024 * 1024 * 1024, // 1GB
    chunk_size: 5 * 1024 * 1024, // 5MB
    ..Default::default()
};

// S3 configuration
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

// Azure Blob configuration
let azure_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::Azure,
    bucket: "my-container".to_string(),
    endpoint: "https://mystorageaccount.blob.core.windows.net".to_string(),
    access_key: "DefaultEndpointsProtocol=https;AccountName=mystorageaccount;AccountKey=example...".to_string(),
    encryption: Some(DMSCStorageEncryption::ClientSide),
    ..Default::default()
};

// Google Cloud Storage configuration
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

Storage backend enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Local` | Local file system |
| `S3` | Amazon S3-compatible storage |
| `Azure` | Azure Blob storage |
| `GCS` | Google Cloud Storage |
| `MinIO` | MinIO object storage |
| `Distributed` | Distributed storage |

<div align="center">

## File Upload

</div>

### Multi-File Upload

```rust
use dmsc::prelude::*;
use tokio::fs::File;

// Handle multi-file upload
async fn handle_file_upload(files: Vec<UploadFile>) -> DMSCResult<Vec<String>> {
    let mut uploaded_keys = Vec::new();
    
    for file in files {
        let key = format!("uploads/{}/{}", chrono::Utc::now().format("%Y/%m/%d"), file.filename);
        
        // Validate file type
        if !is_allowed_file_type(&file.content_type) {
            return Err(DMSCError::validation(format!("File type not allowed: {}", file.content_type)));
        }
        
        // Validate file size
        if file.size > 10 * 1024 * 1024 { // 10MB limit
            return Err(DMSCError::validation("File too large".to_string()));
        }
        
        // Upload file
        ctx.storage().put(&key, &file.content).await?;
        uploaded_keys.push(key);
        
        ctx.log().info(format!("Uploaded file: {} ({} bytes)", file.filename, file.size));
    }
    
    Ok(uploaded_keys)
}

// Struct definition
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

### Multipart Upload

```rust
use dmsc::prelude::*;
use tokio::io::AsyncReadExt;

// Initialize multipart upload
let upload_id = ctx.storage().init_multipart_upload("large_file.zip").await?;
ctx.log().info(format!("Started multipart upload: {}", upload_id));

// Upload parts
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

// Complete multipart upload
ctx.storage().complete_multipart_upload("large_file.zip", &upload_id, &uploaded_parts).await?;
ctx.log().info("Multipart upload completed");
```
<div align="center">

## File Download

</div>

### Resumable Download

```rust
use dmsc::prelude::*;
use tokio::io::AsyncWriteExt;

// Resumable download
async fn resumable_download(key: &str, output_path: &str) -> DMSCResult<()> {
    let metadata = ctx.storage().metadata(key).await?;
    let total_size = metadata.size;
    
    let mut start_byte = 0u64;
    
    // Check if partial download exists
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

### Presigned URL

```rust
use dmsc::prelude::*;

// Generate presigned download URL
let download_url = ctx.storage().generate_presigned_url(
    "documents/confidential.pdf",
    DMSCPresignedUrlOperation::Get,
    Duration::from_hours(1) // 1 hour validity
).await?;

ctx.log().info(format!("Generated presigned URL: {}", download_url));

// Generate presigned upload URL
let upload_url = ctx.storage().generate_presigned_url(
    "uploads/user_upload_{}.jpg",
    DMSCPresignedUrlOperation::Put,
    Duration::from_minutes(30) // 30 minutes validity
).await?;

ctx.log().info(format!("Generated presigned upload URL: {}", upload_url));
```
<div align="center">

## Metadata Management

</div>

### Object Metadata

```rust
use dmsc::prelude::*;

// Upload file with metadata
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

// Update metadata
let mut new_metadata = HashMap::new();
new_metadata.insert("reviewed".to_string(), "true".to_string());
new_metadata.insert("reviewer".to_string(), "Jane Smith".to_string());
new_metadata.insert("review_date".to_string(), chrono::Utc::now().to_rfc3339());

ctx.storage().update_metadata("documents/report.pdf", &new_metadata).await?;

// Get metadata
let metadata = ctx.storage().metadata("documents/report.pdf").await?;
for (key, value) in &metadata.metadata {
    ctx.log().info(format!("{}: {}", key, value));
}
```

### Tag Management

```rust
use dmsc::prelude::*;

// Set object tags
let tags = vec![
    "project:alpha".to_string(),
    "team:engineering".to_string(),
    "environment:production".to_string(),
    "cost-center:1234".to_string(),
];

ctx.storage().set_tags("documents/report.pdf", &tags).await?;

// Search by tag
let tagged_objects = ctx.storage().find_by_tag("team:engineering").await?;
for obj in tagged_objects {
    ctx.log().info(format!("Found object with tag: {}", obj.key));
}

// Remove tag
ctx.storage().remove_tag("documents/report.pdf", "project:alpha").await?;
```
<div align="center">

## Storage Encryption

</div>

### Client-Side Encryption

```rust
use dmsc::prelude::*;

// Configure client-side encryption
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

// Upload encrypted file
let sensitive_data = b"This is sensitive information that needs encryption";
ctx.storage().put_with_encryption("confidential/data.txt", sensitive_data).await?;

// Download and decrypt
let decrypted_data = ctx.storage().get_and_decrypt("confidential/data.txt").await?;
let content = String::from_utf8(decrypted_data)?;
ctx.log().info(format!("Decrypted content: {}", content));
```

### Key Management

```rust
use dmsc::prelude::*;

// Generate data encryption key
let data_key = ctx.storage().generate_data_encryption_key()?;

// Encrypt key with KMS
let kms_config = DMSCKMSConfig {
    key_id: "arn:aws:kms:us-west-2:123456789012:key/12345678-1234-1234-1234-123456789012",
    region: "us-west-2".to_string(),
    endpoint: Some("https://kms.us-west-2.amazonaws.com".to_string()),
};

let encrypted_key = ctx.storage().encrypt_with_kms(&data_key, &kms_config).await?;
let decrypted_key = ctx.storage().decrypt_with_kms(&encrypted_key, &kms_config).await?;
```

<div align="center">

## Storage Compression

</div>

### Automatic Compression

```rust
use dmsc::prelude::*;

// Configure automatic compression
let compression_config = DMSCStorageCompression {
    enabled: true,
    algorithm: DMSCStorageCompressionAlgorithm::Gzip,
    threshold: 1024, // Compress files larger than 1KB
    extensions: vec!["txt".to_string(), "json".to_string(), "xml".to_string(), "csv".to_string()],
};

let mut storage_config = DMSCStorageConfig {
    backend: DMSCStorageBackend::S3,
    bucket: "compressed-bucket".to_string(),
    compression: Some(compression_config),
    ..Default::default()
};

// Upload will be automatically compressed
let large_text = "A".repeat(10000); // 10KB text
ctx.storage().put("large_text_file.txt", large_text.as_bytes()).await?;

// Download will be automatically decompressed
let decompressed_data = ctx.storage().get("large_text_file.txt").await?;
ctx.log().info(format!("Decompressed size: {} bytes", decompressed_data.len()));
```

<div align="center">

## Lifecycle Management

</div>

### Storage Class Transition

```rust
use dmsc::prelude::*;

// Configure lifecycle rules
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

// Manually change storage class
ctx.storage().change_storage_class("old_document.pdf", DMSCStorageClass::Glacier).await?;
```

<div align="center">

## Version Control

</div>

### Object Version Management

```rust
use dmsc::prelude::*;

// Enable version control
ctx.storage().enable_versioning("my-bucket").await?;

// Upload multiple versions
ctx.storage().put("documents/report.pdf", b"Version 1 content").await?;
ctx.storage().put("documents/report.pdf", b"Version 2 content").await?;
ctx.storage().put("documents/report.pdf", b"Version 3 content").await?;

// List all versions
let versions = ctx.storage().list_versions("documents/report.pdf").await?;
for version in versions {
    ctx.log().info(format!(
        "Version {} ({}): {} bytes, modified {}",
        version.version_id, version.is_latest, version.size, version.last_modified
    ));
}

// Get specific version
let version_data = ctx.storage().get_version("documents/report.pdf", "version_123").await?;

// Restore to specific version
ctx.storage().restore_version("documents/report.pdf", "version_123").await?;

// Delete specific version
ctx.storage().delete_version("documents/report.pdf", "version_456").await?;
```

<div align="center">

## Monitoring and Statistics

</div>

### Storage Statistics

```rust
use dmsc::prelude::*;

// Get storage statistics
let stats = ctx.storage().get_storage_stats().await?;
ctx.log().info(format!(
    "Storage stats - Total objects: {}, Total size: {} bytes, Average size: {} bytes",
    stats.total_objects, stats.total_size, stats.average_size
));

// Get bucket statistics
let bucket_stats = ctx.storage().get_bucket_stats("my-bucket").await?;
ctx.log().info(format!(
    "Bucket stats - Objects: {}, Size: {} bytes, Oldest object: {}, Newest object: {}",
    bucket_stats.object_count, bucket_stats.total_size, bucket_stats.oldest_object, bucket_stats.newest_object
));

// Get statistics by prefix
let prefix_stats = ctx.storage().get_prefix_stats("documents/").await?;
for (prefix, stats) in prefix_stats {
    ctx.log().info(format!(
        "Prefix {}: {} objects, {} bytes",
        prefix, stats.object_count, stats.total_size
    ));
}
```

<div align="center">

## Error Handling

</div>

### Storage Error Codes

| Error Code | Description |
|:--------|:-------------|
| `STORAGE_CONNECTION_ERROR` | Storage connection error |
| `STORAGE_NOT_FOUND` | Object not found |
| `STORAGE_PERMISSION_DENIED` | Permission denied |
| `STORAGE_QUOTA_EXCEEDED` | Storage quota exceeded |
| `STORAGE_ENCRYPTION_ERROR` | Encryption error |
| `STORAGE_COMPRESSION_ERROR` | Compression error |

### Error Handling Example

```rust
use dmsc::prelude::*;

match ctx.storage().get("important_file.pdf").await {
    Ok(data) => {
        ctx.log().info("File retrieved successfully");
        // Process file data
    }
    Err(DMSCError { code, .. }) if code == "STORAGE_NOT_FOUND" => {
        ctx.log().warn("File not found, using default");
        // Use default file or return error
        let default_data = get_default_file_data();
        // ...
    }
    Err(DMSCError { code, .. }) if code == "STORAGE_CONNECTION_ERROR" => {
        ctx.log().error("Storage connection failed");
        // Try backup storage
        ctx.storage().use_backup_storage()?;
        // Retry operation
    }
    Err(e) => {
        ctx.log().error(format!("Storage error: {}", e));
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Use streaming operations**: Use streaming upload/download for large files to avoid memory issues
2. **Set appropriate chunk size**: Adjust chunk size based on network conditions and file size
3. **Enable version control**: Enable version control for important data to prevent accidental deletion
4. **Use lifecycle management**: Automatically manage storage class transitions and cleanup of old data
5. **Encrypt sensitive data**: Perform client-side encryption for sensitive data
6. **Compress text data**: Enable compression for text data to save storage space
7. **Monitor storage usage**: Regularly monitor storage usage and performance metrics
8. **Backup important data**: Configure cross-region replication or backup for critical data
9. **Use presigned URLs**: Use presigned URLs when providing file access to avoid exposing credentials
10. **Validate file types**: Validate file types and content during upload to prevent malicious files

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
