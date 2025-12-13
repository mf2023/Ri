# 存储使用示例

storage模块提供文件上传下载、元数据管理、存储加密、压缩、生命周期管理、版本控制和监控统计功能的使用示例。

## 基本文件操作

### 文件上传

```rust
use dms::prelude::*;
use serde_json::json;
use std::path::Path;

// 初始化存储管理器
let storage_config = DMSStorageConfig {
    default_backend: DMSStorageBackend::Local,
    max_file_size: 100 * 1024 * 1024, // 100MB
    allowed_mime_types: vec![
        "image/jpeg".to_string(),
        "image/png".to_string(),
        "application/pdf".to_string(),
        "text/plain".to_string(),
        "application/json".to_string(),
    ],
    blocked_extensions: vec![
        ".exe".to_string(),
        ".bat".to_string(),
        ".sh".to_string(),
        ".cmd".to_string(),
    ],
    enable_virus_scanning: true,
    enable_content_validation: true,
    compression_enabled: true,
    encryption_enabled: true,
    retention_days: 365,
    auto_backup: true,
    backup_count: 3,
};

ctx.storage().init_storage(storage_config).await?;

// 单文件上传
let file_path = Path::new("/path/to/document.pdf");
let file_name = "project_document.pdf";
let content_type = "application/pdf";

let upload_result = ctx.storage()
    .upload_file(file_path, file_name, content_type)
    .await?;

ctx.log().info(format!(
    "File uploaded successfully: {} (ID: {}, Size: {} bytes)",
    upload_result.file_name,
    upload_result.file_id,
    upload_result.file_size
));

// 带元数据的上传
let metadata = json!({
    "project_id": "proj_123",
    "department": "engineering",
    "document_type": "technical_spec",
    "version": "1.0",
    "tags": ["api", "documentation", "v1"],
    "author": "john.doe@company.com",
    "confidentiality": "internal",
});

let upload_with_metadata = ctx.storage()
    .upload_file_with_metadata(file_path, file_name, content_type, metadata)
    .await?;

ctx.log().info(format!(
    "File uploaded with metadata: {:?}",
    upload_with_metadata.metadata
));
```

### 多文件上传

```rust
use dms::prelude::*;
use std::path::Path;

// 批量文件上传
let files = vec![
    (Path::new("/path/to/image1.jpg"), "product_image_1.jpg", "image/jpeg"),
    (Path::new("/path/to/image2.png"), "product_image_2.png", "image/png"),
    (Path::new("/path/to/spec.pdf"), "product_spec.pdf", "application/pdf"),
];

let batch_upload_result = ctx.storage()
    .upload_multiple_files(files)
    .await?;

ctx.log().info(format!(
    "Batch upload completed: {} files uploaded successfully, {} failed",
    batch_upload_result.successful.len(),
    batch_upload_result.failed.len()
));

// 处理上传结果
for success in &batch_upload_result.successful {
    ctx.log().info(format!(
        "✓ Uploaded: {} ({} bytes)",
        success.file_name,
        success.file_size
    ));
}

for failure in &batch_upload_result.failed {
    ctx.log().error(format!(
        "✗ Failed to upload {}: {}",
        failure.file_name,
        failure.error
    ));
}

// 带进度回调的上传
let progress_callback = |progress: DMSUploadProgress| {
    ctx.log().info(format!(
        "Upload progress: {}% ({} / {} bytes)",
        progress.percentage,
        progress.bytes_uploaded,
        progress.total_bytes
    ));
};

let large_file_path = Path::new("/path/to/large_video.mp4");
let large_file_result = ctx.storage()
    .upload_file_with_progress(large_file_path, "training_video.mp4", "video/mp4", progress_callback)
    .await?;

ctx.log().info(format!("Large file upload completed: {:?}", large_file_result));
```

### 分块上传

```rust
use dms::prelude::*;
use std::path::Path;

// 大文件分块上传配置
let chunk_config = DMSChunkUploadConfig {
    chunk_size: 5 * 1024 * 1024, // 5MB chunks
    max_concurrent_chunks: 4,
    retry_attempts: 3,
    enable_checksum: true,
    compression_threshold: 1024 * 1024, // 1MB
};

// 初始化分块上传
let large_file_path = Path::new("/path/to/large_dataset.csv");
let file_name = "customer_data_2024.csv";
let content_type = "text/csv";

let upload_session = ctx.storage()
    .init_chunked_upload(large_file_path, file_name, content_type, chunk_config)
    .await?;

ctx.log().info(format!(
    "Chunked upload initialized: session_id={}, total_chunks={}",
    upload_session.session_id,
    upload_session.total_chunks
));

// 上传分块
for chunk_index in 0..upload_session.total_chunks {
    let chunk_result = ctx.storage()
        .upload_chunk(&upload_session.session_id, chunk_index)
        .await?;
    
    ctx.log().info(format!(
        "Chunk {} uploaded successfully (checksum: {})",
        chunk_index,
        chunk_result.checksum
    ));
}

// 完成分块上传
let final_result = ctx.storage()
    .complete_chunked_upload(&upload_session.session_id)
    .await?;

ctx.log().info(format!(
    "Chunked upload completed: file_id={}, total_size={} bytes",
    final_result.file_id,
    final_result.total_size
));

// 取消分块上传
// ctx.storage().abort_chunked_upload(&upload_session.session_id).await?;
```

## 文件下载

### 基本文件下载

```rust
use dms::prelude::*;
use std::path::Path;

// 通过文件ID下载
let file_id = "file_123";
let download_path = Path::new("/downloads/document.pdf");

let download_result = ctx.storage()
    .download_file(file_id, download_path)
    .await?;

ctx.log().info(format!(
    "File downloaded: {} ({} bytes, checksum: {})",
    download_result.file_name,
    download_result.file_size,
    download_result.checksum
));

// 通过文件名下载
let file_name = "project_document.pdf";
let download_by_name = ctx.storage()
    .download_file_by_name(file_name, download_path)
    .await?;

ctx.log().info(format!("File downloaded by name: {}", download_by_name.file_name));

// 带进度回调的下载
let progress_callback = |progress: DMSDownloadProgress| {
    ctx.log().info(format!(
        "Download progress: {}% ({} / {} bytes)",
        progress.percentage,
        progress.bytes_downloaded,
        progress.total_bytes
    ));
};

let large_download = ctx.storage()
    .download_file_with_progress(file_id, download_path, progress_callback)
    .await?;

ctx.log().info(format!("Large file download completed: {:?}", large_download));
```

### 断点续传

```rust
use dms::prelude::*;
use std::path::Path;

// 断点续传下载
let resume_config = DMSResumeDownloadConfig {
    chunk_size: 1024 * 1024, // 1MB chunks
    max_retries: 3,
    enable_checksum: true,
    use_temp_file: true,
};

let file_id = "large_file_123";
let download_path = Path::new("/downloads/large_file.zip");

// 检查是否支持断点续传
let resume_info = ctx.storage()
    .check_resume_download(file_id)
    .await?;

if resume_info.supports_resume {
    ctx.log().info(format!(
        "File supports resume download: total_size={}, can_resume_from={}",
        resume_info.total_size,
        resume_info.resume_from
    ));
    
    // 从指定位置开始下载
    let resume_result = ctx.storage()
        .resume_download(file_id, download_path, resume_info.resume_from, resume_config)
        .await?;
    
    ctx.log().info(format!(
        "Resume download completed: downloaded={}, resumed_from={}",
        resume_result.bytes_downloaded,
        resume_result.resumed_from
    ));
} else {
    // 不支持断点续传，重新开始下载
    ctx.log().info("File does not support resume, starting fresh download");
    let fresh_download = ctx.storage().download_file(file_id, download_path).await?;
    ctx.log().info(format!("Fresh download completed: {:?}", fresh_download));
}
```

### 临时下载链接

```rust
use dms::prelude::*;

// 生成临时下载链接
let file_id = "sensitive_document.pdf";
let expiration_duration = Duration::from_hours(2);

let temp_url = ctx.storage()
    .generate_temporary_download_url(file_id, expiration_duration)
    .await?;

ctx.log().info(format!("Temporary download URL: {}", temp_url));

// 验证临时链接
let is_valid = ctx.storage()
    .validate_temporary_url(&temp_url)
    .await?;

ctx.log().info(format!("Temporary URL is valid: {}", is_valid));

// 带访问控制的临时链接
let access_control = DMSAccessControl {
    allowed_ips: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
    max_downloads: 5,
    require_authentication: true,
    user_groups: vec!["employees".to_string(), "managers".to_string()],
};

let secure_temp_url = ctx.storage()
    .generate_secure_temporary_url(file_id, expiration_duration, access_control)
    .await?;

ctx.log().info(format!("Secure temporary URL: {}", secure_temp_url));

// 撤销临时链接
ctx.storage().revoke_temporary_url(&temp_url).await?;
ctx.log().info("Temporary URL revoked");
```

## 元数据管理

### 文件元数据操作

```rust
use dms::prelude::*;
use serde_json::json;

// 获取文件元数据
let file_id = "file_123";
let metadata = ctx.storage()
    .get_file_metadata(file_id)
    .await?;

ctx.log().info(format!(
    "File metadata: name={}, size={}, created={}, tags={:?}",
    metadata.file_name,
    metadata.file_size,
    metadata.created_at,
    metadata.tags
));

// 更新文件元数据
let updated_metadata = json!({
    "project_id": "proj_456",
    "department": "marketing",
    "tags": ["updated", "reviewed", "approved"],
    "review_date": chrono::Utc::now().to_rfc3339(),
    "reviewer": "jane.smith@company.com",
});

ctx.storage()
    .update_file_metadata(file_id, updated_metadata)
    .await?;

ctx.log().info("File metadata updated successfully");

// 添加标签
let new_tags = vec!["important", "client-facing", "v2.0"];
ctx.storage()
    .add_file_tags(file_id, new_tags)
    .await?;

ctx.log().info("Tags added to file");

// 移除标签
let tags_to_remove = vec!["draft", "temporary"];
ctx.storage()
    .remove_file_tags(file_id, tags_to_remove)
    .await?;

ctx.log().info("Tags removed from file");

// 搜索文件
let search_criteria = DMSSearchCriteria {
    query: "project documentation".to_string(),
    tags: vec!["documentation".to_string(), "api".to_string()],
    date_range: Some(DMSDateRange {
        start: chrono::Utc::now() - chrono::Duration::days(30),
        end: chrono::Utc::now(),
    }),
    file_types: vec!["application/pdf".to_string(), "text/plain".to_string()],
    size_range: Some(DMSSizeRange {
        min: 1024,      // 1KB
        max: 10 * 1024 * 1024, // 10MB
    }),
};

let search_results = ctx.storage()
    .search_files(search_criteria)
    .await?;

ctx.log().info(format!("Search found {} files", search_results.files.len()));
for file in &search_results.files {
    ctx.log().info(format!("Found: {} ({})", file.file_name, file.file_size));
}
```

### 文件组织

```rust
use dms::prelude::*;
use serde_json::json;

// 创建文件夹结构
let folder_structure = vec![
    ("/projects/project1/documents", "Project 1 Documents"),
    ("/projects/project1/images", "Project 1 Images"),
    ("/projects/project2/documents", "Project 2 Documents"),
    ("/shared/templates", "Shared Templates"),
    ("/archive/2024", "2024 Archive"),
];

for (path, description) in folder_structure {
    ctx.storage()
        .create_folder(path, description)
        .await?;
    
    ctx.log().info(format!("Created folder: {}", path));
}

// 移动文件到文件夹
let file_id = "file_123";
let target_folder = "/projects/project1/documents";

ctx.storage()
    .move_file(file_id, target_folder)
    .await?;

ctx.log().info(format!("File moved to folder: {}", target_folder));

// 复制文件
let copied_file = ctx.storage()
    .copy_file(file_id, "/backup/copied_file.pdf")
    .await?;

ctx.log().info(format!("File copied: new_file_id={}", copied_file.file_id));

// 获取文件夹内容
let folder_contents = ctx.storage()
    .get_folder_contents("/projects/project1", Some(100), Some(0))
    .await?;

ctx.log().info(format!(
    "Folder contents: {} files, {} subfolders",
    folder_contents.files.len(),
    folder_contents.subfolders.len()
));

// 递归删除文件夹
ctx.storage()
    .delete_folder_recursive("/projects/project1")
    .await?;

ctx.log().info("Folder and all contents deleted");
```

## 存储加密

### 文件加密

```rust
use dms::prelude::*;
use serde_json::json;

// 配置存储加密
let encryption_config = DMSStorageEncryptionConfig {
    enabled: true,
    algorithm: DMSEncryptionAlgorithm::AES256GCM,
    key_rotation_interval: Duration::from_days(90),
    encrypt_at_rest: true,
    encrypt_in_transit: true,
    key_management_service: DMSKeyManagementService::AWSKMS,
    customer_managed_keys: true,
};

ctx.storage().init_encryption(encryption_config).await?;

// 上传加密文件
let sensitive_file = Path::new("/path/to/sensitive_data.xlsx");
let encrypted_upload = ctx.storage()
    .upload_encrypted_file(sensitive_file, "encrypted_data.xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .await?;

ctx.log().info(format!(
    "Encrypted file uploaded: file_id={}, encryption_key_id={}",
    encrypted_upload.file_id,
    encrypted_upload.encryption_key_id
));

// 下载并解密文件
let decrypted_download = ctx.storage()
    .download_and_decrypt_file(&encrypted_upload.file_id, Path::new("/downloads/decrypted_data.xlsx"))
    .await?;

ctx.log().info(format!("File decrypted and downloaded: {:?}", decrypted_download));

// 轮换加密密钥
let new_key_id = ctx.storage()
    .rotate_encryption_key(&encrypted_upload.file_id)
    .await?;

ctx.log().info(format!("Encryption key rotated: new_key_id={}", new_key_id));

// 客户端加密上传
let client_encrypted_data = b"client encrypted file content";
let client_upload = ctx.storage()
    .upload_client_encrypted_file(
        client_encrypted_data,
        "client_encrypted_file.bin",
        "application/octet-stream",
        "client_key_id"
    )
    .await?;

ctx.log().info("Client encrypted file uploaded");
```

### 密钥管理

```rust
use dms::prelude::*;

// 生成数据加密密钥
let data_key = ctx.storage()
    .generate_data_encryption_key(DMSKeyAlgorithm::AES256)
    .await?;

ctx.log().info(format!("Data encryption key generated: key_id={}", data_key.key_id));

// 加密数据密钥
let master_key_id = "master-key-123";
let encrypted_data_key = ctx.storage()
    .encrypt_data_key(&data_key.plaintext_key, master_key_id)
    .await?;

ctx.log().info("Data key encrypted with master key");

// 解密数据密钥
let decrypted_data_key = ctx.storage()
    .decrypt_data_key(&encrypted_data_key, master_key_id)
    .await?;

ctx.log().info("Data key decrypted successfully");

// 安全删除密钥
ctx.storage()
    .secure_delete_key(&data_key.key_id)
    .await?;

ctx.log().info("Encryption key securely deleted");
```

## 存储压缩

### 文件压缩

```rust
use dms::prelude::*;
use serde_json::json;

// 配置存储压缩
let compression_config = DMSCompressionConfig {
    enabled: true,
    algorithm: DMSCompressionAlgorithm::Zstd,
    compression_level: 3,
    threshold_size: 1024, // 1KB
    compressible_types: vec![
        "text/plain".to_string(),
        "text/csv".to_string(),
        "application/json".to_string(),
        "application/xml".to_string(),
        "text/html".to_string(),
        "application/javascript".to_string(),
        "text/css".to_string(),
    ],
    exclude_extensions: vec![".jpg".to_string(), ".png".to_string(), ".mp4".to_string()],
};

ctx.storage().init_compression(compression_config).await?;

// 上传并压缩文件
let text_file = Path::new("/path/to/large_log_file.txt");
let compressed_upload = ctx.storage()
    .upload_and_compress_file(text_file, "compressed_log.txt", "text/plain")
    .await?;

ctx.log().info(format!(
    "File uploaded and compressed: original_size={}, compressed_size={}, compression_ratio={:.2}%",
    compressed_upload.original_size,
    compressed_upload.compressed_size,
    compressed_upload.compression_ratio * 100.0
));

// 下载并解压缩文件
let decompressed_download = ctx.storage()
    .download_and_decompress_file(&compressed_upload.file_id, Path::new("/downloads/decompressed_log.txt"))
    .await?;

ctx.log().info(format!("File downloaded and decompressed: {:?}", decompressed_download));

// 批量压缩现有文件
let files_to_compress = vec!["file1.txt", "file2.json", "file3.csv"];
let batch_compression = ctx.storage()
    .compress_existing_files(files_to_compress)
    .await?;

ctx.log().info(format!(
    "Batch compression completed: {} files compressed, {} failed",
    batch_compression.successful.len(),
    batch_compression.failed.len()
));
```

### 智能压缩

```rust
use dms::prelude::*;

// 智能压缩决策
let smart_compression = ctx.storage()
    .should_compress_file("large_dataset.csv", 1024 * 1024, "text/csv")
    .await?;

if smart_compression.should_compress {
    ctx.log().info(format!(
        "File should be compressed: estimated_savings={:.1}%, algorithm={}",
        smart_compression.estimated_savings * 100.0,
        smart_compression.recommended_algorithm
    ));
    
    // 执行智能压缩
    let compression_result = ctx.storage()
        .smart_compress_file("large_dataset.csv")
        .await?;
    
    ctx.log().info(format!(
        "Smart compression completed: actual_savings={:.1}%",
        compression_result.actual_savings * 100.0
    ));
} else {
    ctx.log().info("File should not be compressed - savings would be minimal");
}
```

## 生命周期管理

### 文件生命周期

```rust
use dms::prelude::*;
use serde_json::json;

// 配置生命周期策略
let lifecycle_config = DMSLifecycleConfig {
    enabled: true,
    default_retention_days: 365,
    archive_after_days: 90,
    delete_after_days: 1095, // 3 years
    auto_archive: true,
    auto_delete: false, // 需要手动确认删除
    lifecycle_policies: vec![
        DMSLifecyclePolicy {
            name: "temporary_files".to_string(),
            file_patterns: vec!["*.tmp".to_string(), "*.temp".to_string()],
            retention_days: 7,
            archive_after_days: 3,
            delete_after_days: 30,
        },
        DMSLifecyclePolicy {
            name: "log_files".to_string(),
            file_patterns: vec!["*.log".to_string(), "*.txt".to_string()],
            retention_days: 90,
            archive_after_days: 30,
            delete_after_days: 365,
        },
        DMSLifecyclePolicy {
            name: "backup_files".to_string(),
            file_patterns: vec!["*.bak".to_string(), "*.backup".to_string()],
            retention_days: 180,
            archive_after_days: 60,
            delete_after_days: 730, // 2 years
        },
    ],
};

ctx.storage().init_lifecycle_management(lifecycle_config).await?;

// 应用生命周期策略
ctx.storage()
    .apply_lifecycle_policies()
    .await?;

ctx.log().info("Lifecycle policies applied");

// 归档文件
let file_id = "old_file_123";
let archive_result = ctx.storage()
    .archive_file(file_id, "cold_storage")
    .await?;

ctx.log().info(format!(
    "File archived: archive_tier={}, archive_date={}",
    archive_result.archive_tier,
    archive_result.archive_date
));

// 恢复归档文件
let restore_result = ctx.storage()
    .restore_archived_file(file_id, Duration::from_days(7))
    .await?;

ctx.log().info(format!(
    "File restored: restore_tier={}, available_by={}",
    restore_result.restore_tier,
    restore_result.available_by
));

// 批量归档
let old_files = vec!["file1", "file2", "file3"];
let batch_archive = ctx.storage()
    .batch_archive_files(old_files, "glacier")
    .await?;

ctx.log().info(format!(
    "Batch archive completed: {} files archived, {} failed",
    batch_archive.successful.len(),
    batch_archive.failed.len()
));
```

### 自动清理

```rust
use dms::prelude::*;

// 运行自动清理任务
let cleanup_result = ctx.storage()
    .run_cleanup_task()
    .await?;

ctx.log().info(format!(
    "Cleanup task completed: {} files archived, {} files deleted, {} errors",
    cleanup_result.archived_count,
    cleanup_result.deleted_count,
    cleanup_result.error_count
));

// 获取清理报告
let cleanup_report = ctx.storage()
    .get_cleanup_report(chrono::Utc::now() - chrono::Duration::days(30), chrono::Utc::now())
    .await?;

ctx.log().info(format!(
    "Cleanup report for last 30 days: {} files processed, {} storage space reclaimed",
    cleanup_report.total_files_processed,
    cleanup_report.storage_space_reclaimed
));

// 配置自动清理计划
let cleanup_schedule = DMSCleanupSchedule {
    enabled: true,
    schedule: "0 2 * * *".to_string(), // 每天凌晨2点
    max_files_per_run: 1000,
    dry_run: false,
    notification_email: "admin@company.com".to_string(),
};

ctx.storage()
    .configure_cleanup_schedule(cleanup_schedule)
    .await?;

ctx.log().info("Cleanup schedule configured");
```

## 版本控制

### 文件版本管理

```rust
use dms::prelude::*;
use serde_json::json;

// 启用版本控制
let versioning_config = DMSVersioningConfig {
    enabled: true,
    max_versions: 10,
    auto_versioning: true,
    version_on_metadata_change: true,
    retention_policy: DMSVersionRetentionPolicy::KeepLastN(5),
    version_labels: vec!["draft".to_string(), "review".to_string(), "approved".to_string(), "final".to_string()],
};

ctx.storage().init_versioning(versioning_config).await?;

// 上传新版本
let document_path = Path::new("/path/to/updated_document_v2.pdf");
let file_id = "original_file_123";

let new_version = ctx.storage()
    .upload_new_version(file_id, document_path, "Updated project documentation")
    .await?;

ctx.log().info(format!(
    "New version uploaded: version_id={}, version_number={}, size={}",
    new_version.version_id,
    new_version.version_number,
    new_version.file_size
));

// 列出文件版本
let version_history = ctx.storage()
    .list_file_versions(file_id)
    .await?;

ctx.log().info(format!("File has {} versions:", version_history.versions.len()));
for version in &version_history.versions {
    ctx.log().info(format!(
        "Version {}: created={}, size={}, label={:?}",
        version.version_number,
        version.created_at,
        version.file_size,
        version.label
    ));
}

// 下载特定版本
let version_number = 2;
let version_download = ctx.storage()
    .download_file_version(file_id, version_number, Path::new("/downloads/version2.pdf"))
    .await?;

ctx.log().info(format!("Version {} downloaded successfully", version_number));

// 恢复文件到特定版本
let restored_version = ctx.storage()
    .restore_file_version(file_id, version_number)
    .await?;

ctx.log().info(format!(
    "File restored to version {}: new_version_id={}",
    version_number,
    restored_version.version_id
));

// 删除特定版本
ctx.storage()
    .delete_file_version(file_id, 1) // 删除版本1
    .await?;

ctx.log().info("File version deleted");

// 标记版本标签
ctx.storage()
    .label_file_version(file_id, 3, "approved")
    .await?;

ctx.log().info("Version 3 labeled as 'approved'");
```

### 版本比较

```rust
use dms::prelude::*;

// 比较两个版本
let version1 = 1;
let version2 = 3;

let version_comparison = ctx.storage()
    .compare_file_versions(file_id, version1, version2)
    .await?;

ctx.log().info(format!(
    "Version comparison: size_change={}, metadata_changes={}, content_similarity={:.1}%",
    version_comparison.size_change,
    version_comparison.metadata_changes.len(),
    version_comparison.content_similarity * 100.0
));

// 获取版本差异报告
let diff_report = ctx.storage()
    .generate_version_diff_report(file_id, version1, version2)
    .await?;

ctx.log().info(format!("Version diff report generated: {}", diff_report.report_id));
```

## 监控统计

### 存储统计

```rust
use dms::prelude::*;

// 获取存储使用统计
let storage_stats = ctx.storage()
    .get_storage_statistics()
    .await?;

ctx.log().info(format!(
    "Storage statistics: total_files={}, total_size={}, average_file_size={}",
    storage_stats.total_files,
    storage_stats.total_size,
    storage_stats.average_file_size
));

// 按文件类型统计
let stats_by_type = ctx.storage()
    .get_statistics_by_file_type()
    .await?;

ctx.log().info("Storage usage by file type:");
for (file_type, stats) in stats_by_type {
    ctx.log().info(format!(
        "  {}: {} files, {} bytes",
        file_type,
        stats.file_count,
        stats.total_size
    ));
}

// 按时间范围统计
let time_range_stats = ctx.storage()
    .get_statistics_by_time_range(
        chrono::Utc::now() - chrono::Duration::days(30),
        chrono::Utc::now()
    )
    .await?;

ctx.log().info(format!(
    "Last 30 days: {} files uploaded, {} files downloaded, {} bytes transferred",
    time_range_stats.uploads,
    time_range_stats.downloads,
    time_range_stats.bytes_transferred
));

// 存储容量监控
let capacity_info = ctx.storage()
    .get_capacity_info()
    .await?;

ctx.log().info(format!(
    "Capacity info: used={}, available={}, total={}, utilization={:.1}%",
    capacity_info.used_capacity,
    capacity_info.available_capacity,
    capacity_info.total_capacity,
    capacity_info.utilization_percentage * 100.0
));

if capacity_info.utilization_percentage > 0.8 {
    ctx.log().warn("Storage utilization is above 80%");
}
```

### 性能监控

```rust
use dms::prelude::*;

// 获取性能指标
let performance_metrics = ctx.storage()
    .get_performance_metrics()
    .await?;

ctx.log().info(format!(
    "Performance metrics: avg_upload_time={}ms, avg_download_time={}ms, cache_hit_rate={:.1}%",
    performance_metrics.average_upload_time_ms,
    performance_metrics.average_download_time_ms,
    performance_metrics.cache_hit_rate * 100.0
));

// 获取操作统计
let operation_stats = ctx.storage()
    .get_operation_statistics()
    .await?;

ctx.log().info(format!(
    "Operation statistics: total_uploads={}, total_downloads={}, failed_operations={}",
    operation_stats.total_uploads,
    operation_stats.total_downloads,
    operation_stats.failed_operations
));

// 错误统计
let error_stats = ctx.storage()
    .get_error_statistics()
    .await?;

ctx.log().info("Error statistics by type:");
for (error_type, count) in error_stats.error_counts {
    ctx.log().info(format!("  {}: {} occurrences", error_type, count));
}
```

## 错误处理

### 存储错误处理

```rust
use dms::prelude::*;
use serde_json::json;

// 文件上传错误处理
match ctx.storage().upload_file(file_path, file_name, content_type).await {
    Ok(upload_result) => {
        ctx.log().info(format!("File uploaded successfully: {:?}", upload_result));
    }
    Err(DMSError::StorageFull(e)) => {
        ctx.log().error(format!("Storage is full: {}", e));
        
        // 尝试清理空间
        let cleanup_result = ctx.storage().run_cleanup_task().await?;
        ctx.log().info(format!("Cleanup completed: {} bytes freed", cleanup_result.storage_space_reclaimed));
        
        // 重试上传
        match ctx.storage().upload_file(file_path, file_name, content_type).await {
            Ok(retry_result) => {
                ctx.log().info("File uploaded successfully after cleanup");
            }
            Err(retry_err) => {
                ctx.log().error(format!("Upload failed even after cleanup: {}", retry_err));
                return Err(retry_err);
            }
        }
    }
    Err(DMSError::FileTooLarge(e)) => {
        ctx.log().error(format!("File is too large: {}", e));
        
        // 尝试分块上传
        let chunk_config = DMSChunkUploadConfig {
            chunk_size: 5 * 1024 * 1024,
            max_concurrent_chunks: 4,
            retry_attempts: 3,
            enable_checksum: true,
            compression_threshold: 1024 * 1024,
        };
        
        let chunked_result = ctx.storage()
            .init_chunked_upload(file_path, file_name, content_type, chunk_config)
            .await?;
        
        ctx.log().info("Large file upload initiated with chunked approach");
    }
    Err(DMSError::InvalidFileType(e)) => {
        ctx.log().error(format!("Invalid file type: {}", e));
        
        // 验证文件类型
        let allowed_types = vec!["image/jpeg", "image/png", "application/pdf"];
        let actual_type = ctx.storage().detect_file_type(file_path).await?;
        
        ctx.log().info(format!("Actual file type detected: {}", actual_type));
        
        if !allowed_types.contains(&actual_type.as_str()) {
            return Err(DMSError::validation(format!("File type {} is not allowed", actual_type)));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected upload error: {}", e));
        return Err(e);
    }
}

// 存储服务健康检查
let health_check = ctx.storage()
    .health_check()
    .await?;

if !health_check.is_healthy {
    ctx.log().error(format!("Storage service is unhealthy: {:?}", health_check.issues));
    
    // 尝试重新连接存储服务
    ctx.storage().reconnect().await?;
    ctx.log().info("Storage service reconnected");
}
```

## 最佳实践

1. **文件命名**: 使用有意义的文件命名规范
2. **元数据**: 充分利用元数据进行文件组织
3. **生命周期**: 配置适当的文件生命周期策略
4. **版本控制**: 为重要文件启用版本控制
5. **加密**: 对敏感文件启用加密
6. **压缩**: 对可压缩文件启用压缩
7. **监控**: 监控存储使用和性能指标
8. **备份**: 实施定期备份策略
9. **访问控制**: 实施文件访问控制
10. **错误处理**: 妥善处理存储错误和异常情况
11. **分块上传**: 对大文件使用分块上传
12. **临时链接**: 对敏感文件使用临时下载链接
13. **存储分层**: 根据访问频率使用不同存储层级
14. **容量规划**: 定期监控和规划存储容量
15. **合规性**: 确保存储方案符合相关法规要求

<div align="center">

## 运行步骤

</div>

### 环境准备

```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装DMS CLI
cargo install dms-cli
```

### 创建项目

```bash
# 创建新的DMS项目
dms new storage-app
cd storage-app

# 添加存储依赖
cargo add dms-storage
```

### 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dms = "1.0"
dms-storage = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### 配置创建

创建 `config/storage.toml`：

```toml
[storage]
default_backend = "local"
max_file_size = 104857600  # 100MB
allowed_mime_types = ["image/jpeg", "image/png", "application/pdf", "text/plain", "application/json"]
blocked_extensions = [".exe", ".bat", ".sh", ".cmd"]
enable_virus_scanning = true
enable_content_validation = true
compression_enabled = true
encryption_enabled = true
retention_days = 365
auto_backup = true
backup_count = 3

[storage.encryption]
algorithm = "AES256GCM"
key_rotation_interval = "90d"
encrypt_at_rest = true
encrypt_in_transit = true

[storage.compression]
algorithm = "zstd"
compression_level = 3
threshold_size = 1024
compressible_types = ["text/plain", "text/csv", "application/json", "application/xml", "text/html"]
```

### 运行示例

```bash
# 编译项目
cargo build --release

# 运行存储示例
cargo run --example storage-demo
```

<div align="center">

## 预期结果

</div>

运行成功后，您将看到类似以下输出：

```
[2024-01-15 10:30:15] INFO: Storage service initialized successfully
[2024-01-15 10:30:15] INFO: File uploaded successfully: document.pdf (ID: file_123, Size: 2456789 bytes)
[2024-01-15 10:30:15] INFO: File metadata: name=document.pdf, size=2456789, created=2024-01-15T10:30:15Z, tags=["project", "documentation"]
[2024-01-15 10:30:15] INFO: Encrypted file uploaded: file_id=file_124, encryption_key_id=key_456
[2024-01-15 10:30:15] INFO: File uploaded and compressed: original_size=1024576, compressed_size=245789, compression_ratio=76.0%
[2024-01-15 10:30:16] INFO: Storage statistics: total_files=156, total_size=45678912, average_file_size=292813
[2024-01-15 10:30:16] INFO: Performance metrics: avg_upload_time=234ms, avg_download_time=156ms, cache_hit_rate=89.2%
[2024-01-15 10:30:16] INFO: Cleanup task completed: 23 files archived, 12 files deleted, 0 errors
[2024-01-15 10:30:16] INFO: Storage service is healthy: all checks passed
```

<div align="center">

## 扩展功能

</div>

### 分布式存储集群

```rust
use dms::prelude::*;
use serde_json::json;

// 配置分布式存储集群
let cluster_config = DMSStorageClusterConfig {
    nodes: vec![
        DMSStorageNode {
            id: "node-1".to_string(),
            address: "storage1.company.com:9000".to_string(),
            capacity: 10 * 1024 * 1024 * 1024, // 10TB
            region: "us-east-1".to_string(),
            priority: 1,
            health_check_interval: Duration::from_secs(30),
        },
        DMSStorageNode {
            id: "node-2".to_string(),
            address: "storage2.company.com:9000".to_string(),
            capacity: 10 * 1024 * 1024 * 1024, // 10TB
            region: "us-west-1".to_string(),
            priority: 2,
            health_check_interval: Duration::from_secs(30),
        },
        DMSStorageNode {
            id: "node-3".to_string(),
            address: "storage3.company.com:9000".to_string(),
            capacity: 15 * 1024 * 1024 * 1024, // 15TB
            region: "eu-central-1".to_string(),
            priority: 3,
            health_check_interval: Duration::from_secs(30),
        },
    ],
    replication_factor: 3,
    consistency_level: DMSConsistencyLevel::Quorum,
    load_balancing: DMSLoadBalancing::RoundRobin,
    failover_timeout: Duration::from_secs(60),
    health_check_enabled: true,
    auto_rebalance: true,
};

ctx.storage().init_cluster(cluster_config).await?;

// 上传文件到集群（自动选择最优节点）
let file_path = Path::new("/path/to/large_dataset.csv");
let cluster_upload = ctx.storage()
    .upload_to_cluster(file_path, "dataset.csv", "text/csv")
    .await?;

ctx.log().info(format!(
    "File uploaded to cluster: file_id={}, nodes={:?}, replication_factor={}",
    cluster_upload.file_id,
    cluster_upload.storage_nodes,
    cluster_upload.replication_factor
));

// 获取集群健康状态
let cluster_health = ctx.storage()
    .get_cluster_health()
    .await?;

ctx.log().info(format!(
    "Cluster health: healthy_nodes={}, total_nodes={}, cluster_capacity={}TB",
    cluster_health.healthy_nodes,
    cluster_health.total_nodes,
    cluster_health.total_capacity / (1024 * 1024 * 1024 * 1024)
));
```

### 智能存储分层

```rust
use dms::prelude::*;
use serde_json::json;

// 配置智能存储分层
let tiering_config = DMSTieringConfig {
    tiers: vec![
        DMSTier {
            name: "hot".to_string(),
            storage_class: DMSStorageClass::Hot,
            access_frequency_threshold: 0.8, // 80% 访问频率
            retention_days: 30,
            cost_per_gb: 0.023,
            performance_class: "high".to_string(),
        },
        DMSTier {
            name: "warm".to_string(),
            storage_class: DMSStorageClass::Warm,
            access_frequency_threshold: 0.3, // 30% 访问频率
            retention_days: 90,
            cost_per_gb: 0.0125,
            performance_class: "medium".to_string(),
        },
        DMSTier {
            name: "cold".to_string(),
            storage_class: DMSStorageClass::Cold,
            access_frequency_threshold: 0.1, // 10% 访问频率
            retention_days: 365,
            cost_per_gb: 0.004,
            performance_class: "low".to_string(),
        },
        DMSTier {
            name: "archive".to_string(),
            storage_class: DMSStorageClass::Archive,
            access_frequency_threshold: 0.01, // 1% 访问频率
            retention_days: 2555, // 7 years
            cost_per_gb: 0.00099,
            performance_class: "minimal".to_string(),
        },
    ],
    auto_tiering_enabled: true,
    tiering_interval: Duration::from_hours(24),
    access_tracking_enabled: true,
    cost_optimization_enabled: true,
};

ctx.storage().init_tiering(tiering_config).await?;

// 获取文件推荐的存储层级
let file_id = "large_dataset.csv";
let recommended_tier = ctx.storage()
    .get_recommended_tier(file_id)
    .await?;

ctx.log().info(format!(
    "Recommended tier for file {}: tier={}, estimated_savings=${:.2}/month",
    file_id,
    recommended_tier.tier_name,
    recommended_tier.estimated_monthly_savings
));

// 执行自动分层
let tiering_result = ctx.storage()
    .execute_auto_tiering()
    .await?;

ctx.log().info(format!(
    "Auto-tiering completed: {} files moved, ${:.2} monthly savings achieved",
    tiering_result.files_moved,
    tiering_result.total_monthly_savings
));
```

### 实时存储分析

```rust
use dms::prelude::*;
use serde_json::json;

// 配置实时存储分析
let analytics_config = DMSStorageAnalyticsConfig {
    enabled: true,
    real_time_monitoring: true,
    metrics_collection_interval: Duration::from_secs(60),
    anomaly_detection_enabled: true,
    predictive_analysis_enabled: true,
    cost_analysis_enabled: true,
    performance_optimization_enabled: true,
    alerting_enabled: true,
};

ctx.storage().init_analytics(analytics_config).await?;

// 获取实时存储指标
let real_time_metrics = ctx.storage()
    .get_real_time_metrics()
    .await?;

ctx.log().info(format!(
    "Real-time metrics: upload_throughput={}MB/s, download_throughput={}MB/s, active_connections={}",
    real_time_metrics.upload_throughput_mbps,
    real_time_metrics.download_throughput_mbps,
    real_time_metrics.active_connections
));

// 检测存储异常
let anomalies = ctx.storage()
    .detect_storage_anomalies()
    .await?;

if !anomalies.is_empty() {
    ctx.log().warn(format!("Detected {} storage anomalies:", anomalies.len()));
    for anomaly in &anomalies {
        ctx.log().warn(format!(
            "  Anomaly: type={}, severity={}, description={}",
            anomaly.anomaly_type,
            anomaly.severity,
            anomaly.description
        ));
    }
}

// 预测存储需求
let storage_forecast = ctx.storage()
    .forecast_storage_needs(30) // 预测未来30天
    .await?;

ctx.log().info(format!(
    "Storage forecast: predicted_usage={}GB, growth_rate={:.1}%, recommended_capacity={}GB",
    storage_forecast.predicted_usage_gb,
    storage_forecast.growth_rate * 100.0,
    storage_forecast.recommended_capacity_gb
));
```

### 多云存储管理

```rust
use dms::prelude::*;
use serde_json::json;

// 配置多云存储提供商
let multi_cloud_config = DMSMultiCloudConfig {
    providers: vec![
        DMSCloudProvider {
            name: "aws-s3".to_string(),
            provider_type: DMSCloudProviderType::AWSS3,
            region: "us-east-1".to_string(),
            bucket_name: "company-storage-bucket".to_string(),
            priority: 1,
            cost_per_gb: 0.023,
            encryption_enabled: true,
            versioning_enabled: true,
        },
        DMSCloudProvider {
            name: "azure-blob".to_string(),
            provider_type: DMSCloudProviderType::AzureBlob,
            region: "East US".to_string(),
            bucket_name: "company-storage-container".to_string(),
            priority: 2,
            cost_per_gb: 0.0208,
            encryption_enabled: true,
            versioning_enabled: true,
        },
        DMSCloudProvider {
            name: "gcp-storage".to_string(),
            provider_type: DMSCloudProviderType::GCPStorage,
            region: "us-central1".to_string(),
            bucket_name: "company-storage-bucket".to_string(),
            priority: 3,
            cost_per_gb: 0.020,
            encryption_enabled: true,
            versioning_enabled: true,
        },
    ],
    load_balancing: DMSCloudLoadBalancing::CostOptimized,
    failover_enabled: true,
    data_replication: DMSCloudDataReplication::CrossProvider,
    cost_optimization: true,
    provider_failover_timeout: Duration::from_secs(300),
};

ctx.storage().init_multi_cloud(multi_cloud_config).await?;

// 选择最优云提供商上传文件
let file_path = Path::new("/path/to/business_document.pdf");
let optimal_upload = ctx.storage()
    .upload_to_optimal_cloud(file_path, "document.pdf", "application/pdf")
    .await?;

ctx.log().info(format!(
    "File uploaded to optimal cloud provider: provider={}, region={}, cost=${:.4}",
    optimal_upload.provider_name,
    optimal_upload.region,
    optimal_upload.estimated_cost
));

// 获取多云存储成本分析
let cost_analysis = ctx.storage()
    .get_multi_cloud_cost_analysis()
    .await?;

ctx.log().info(format!(
    "Multi-cloud cost analysis: total_monthly_cost=${:.2}, potential_savings=${:.2}, recommended_provider={}",
    cost_analysis.total_monthly_cost,
    cost_analysis.potential_savings,
    cost_analysis.recommended_provider
));
```

<div align="center">

## 总结

</div>

本示例展示了DMS框架强大的存储管理功能，帮助您构建高效、安全、可扩展的文件存储系统。通过文件上传下载、元数据管理、存储加密、压缩、生命周期管理、版本控制和监控统计等核心功能，您可以轻松管理企业级文件存储需求。

### 核心功能

1. **文件上传下载**: 支持单文件、多文件、分块上传，断点续传下载
2. **元数据管理**: 灵活的元数据操作，标签管理，文件搜索
3. **存储加密**: AES256-GCM加密，密钥轮换，客户端加密支持
4. **存储压缩**: 智能压缩算法，支持多种文件类型压缩
5. **生命周期管理**: 自动归档、删除策略，分层存储
6. **版本控制**: 文件版本管理，版本比较，标签标记
7. **监控统计**: 存储使用统计，性能监控，错误统计
8. **错误处理**: 完善的错误处理机制，自动重试策略
9. **临时链接**: 安全的一次性下载链接，访问控制
10. **文件组织**: 文件夹结构，批量操作，递归处理

### 高级特性

1. **分布式集群**: 多节点存储集群，负载均衡，故障转移
2. **智能分层**: 基于访问频率的自动存储分层
3. **实时分析**: 存储异常检测，需求预测，成本优化
4. **多云管理**: 多云提供商支持，成本优化，故障转移
5. **访问控制**: 细粒度权限控制，IP限制，用户组管理
6. **性能优化**: 缓存机制，并发处理，网络优化

### 最佳实践

- 使用有意义的文件命名规范和组织结构
- 充分利用元数据和标签进行文件分类管理
- 根据业务需求配置适当的生命周期策略
- 为重要文件启用版本控制和自动备份
- 对敏感文件启用加密和访问控制
- 对可压缩文件启用压缩以节省存储空间
- 定期监控存储使用和性能指标
- 实施完善的错误处理和重试机制
- 对大文件使用分块上传提高可靠性
- 使用临时链接保护敏感文件下载
- 根据访问频率实施存储分层策略
- 定期进行容量规划和成本优化
- 确保存储方案符合相关法规要求

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMS应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践

- [validation](./validation.md): 验证示例，数据验证和清理操作
