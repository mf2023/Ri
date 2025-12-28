# Storage Usage Examples

The storage module provides usage examples for file upload/download, metadata management, storage encryption, compression, lifecycle management, version control, and monitoring statistics.

## Basic File Operations

### File Upload

```rust
use dms::prelude::*;
use serde_json::json;
use std::path::Path;

// Initialize storage manager
let storage_config = DMSCStorageConfig {
    default_backend: DMSCStorageBackend::Local,
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

// Single file upload
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

// Upload with metadata
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

));

### Multiple File Upload

```rust
use dms::prelude::*;
use std::path::Path;

// Batch file upload
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

// Process upload results
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

// Upload with progress callback
let progress_callback = |progress: DMSCUploadProgress| {
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

### Chunked Upload

```rust
use dms::prelude::*;
use std::path::Path;

// Large file chunked upload configuration
let chunk_config = DMSCChunkUploadConfig {
    chunk_size: 5 * 1024 * 1024, // 5MB chunks
    max_concurrent_chunks: 4,
    retry_attempts: 3,
    enable_checksum: true,
    compression_threshold: 1024 * 1024, // 1MB
};

// Initialize chunked upload
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

// Upload chunks
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

// Complete chunked upload
let final_result = ctx.storage()
    .complete_chunked_upload(&upload_session.session_id)
    .await?;

ctx.log().info(format!(
    "Chunked upload completed: file_id={}, total_size={} bytes",
    final_result.file_id,
    final_result.total_size
));

// Abort chunked upload
// ctx.storage().abort_chunked_upload(&upload_session.session_id).await?;
```

## File Download

### Basic File Download

```rust
use dms::prelude::*;
use std::path::Path;

// Download by file ID
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

// Download by file name
let file_name = "project_document.pdf";
let download_by_name = ctx.storage()
    .download_file_by_name(file_name, download_path)
    .await?;

ctx.log().info(format!("File downloaded by name: {}", download_by_name.file_name));

// Download with progress callback
let progress_callback = |progress: DMSCDownloadProgress| {
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

### Resume Download

```rust
use dms::prelude::*;
use std::path::Path;

// Resume download configuration
let resume_config = DMSCResumeDownloadConfig {
    chunk_size: 1024 * 1024, // 1MB chunks
    max_retries: 3,
    enable_checksum: true,
    use_temp_file: true,
};

let file_id = "large_file_123";
let download_path = Path::new("/downloads/large_file.zip");

// Check if resume download is supported
let resume_info = ctx.storage()
    .check_resume_download(file_id)
    .await?;

if resume_info.supports_resume {
    ctx.log().info(format!(
        "File supports resume download: total_size={}, can_resume_from={}",
        resume_info.total_size,
        resume_info.resume_from
    ));
    
    // Resume download from specified position
    let resume_result = ctx.storage()
        .resume_download(file_id, download_path, resume_info.resume_from, resume_config)
        .await?;
    
    ctx.log().info(format!(
        "Resume download completed: downloaded={}, resumed_from={}",
        resume_result.bytes_downloaded,
        resume_result.resumed_from
    ));
} else {
    // Does not support resume download, start fresh download
    ctx.log().info("File does not support resume, starting fresh download");
    let fresh_download = ctx.storage().download_file(file_id, download_path).await?;
    ctx.log().info(format!("Fresh download completed: {:?}", fresh_download));
}
```

### Temporary Download Links

```rust
use dms::prelude::*;

// Generate temporary download link
let file_id = "sensitive_document.pdf";
let expiration_duration = Duration::from_hours(2);

let temp_url = ctx.storage()
    .generate_temporary_download_url(file_id, expiration_duration)
    .await?;

ctx.log().info(format!("Temporary download URL: {}", temp_url));

// Validate temporary link
let is_valid = ctx.storage()
    .validate_temporary_url(&temp_url)
    .await?;

ctx.log().info(format!("Temporary URL is valid: {}", is_valid));

// Temporary link with access control
let access_control = DMSCAccessControl {
    allowed_ips: vec!["192.168.1.0/24".to_string(), "10.0.0.0/8".to_string()],
    max_downloads: 5,
    require_authentication: true,
    user_groups: vec!["employees".to_string(), "managers".to_string()],
};

let secure_temp_url = ctx.storage()
    .generate_secure_temporary_url(file_id, expiration_duration, access_control)
    .await?;

ctx.log().info(format!("Secure temporary URL: {}", secure_temp_url));

// Revoke temporary link
ctx.storage().revoke_temporary_url(&temp_url).await?;
ctx.log().info("Temporary URL revoked");
```

## Metadata Management

### File Metadata Operations

```rust
use dms::prelude::*;
use serde_json::json;

// Get file metadata
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

// Update file metadata
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

// Add tags
let new_tags = vec!["important", "client-facing", "v2.0"];
ctx.storage()
    .add_file_tags(file_id, new_tags)
    .await?;

ctx.log().info("Tags added to file");

// Remove tags
let tags_to_remove = vec!["draft", "temporary"];
ctx.storage()
    .remove_file_tags(file_id, tags_to_remove)
    .await?;

ctx.log().info("Tags removed from file");

// Search files
let search_criteria = DMSCSearchCriteria {
    query: "project documentation".to_string(),
    tags: vec!["documentation".to_string(), "api".to_string()],
    date_range: Some(DMSCDateRange {
        start: chrono::Utc::now() - chrono::Duration::days(30),
        end: chrono::Utc::now(),
    }),
    file_types: vec!["application/pdf".to_string(), "text/plain".to_string()],
    size_range: Some(DMSCSizeRange {
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

### File Organization

```rust
use dms::prelude::*;
use serde_json::json;

// Create folder structure
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

// Move file to folder
let file_id = "file_123";
let target_folder = "/projects/project1/documents";

ctx.storage()
    .move_file(file_id, target_folder)
    .await?;

ctx.log().info(format!("File moved to folder: {}", target_folder));

// Copy file
let copied_file = ctx.storage()
    .copy_file(file_id, "/backup/copied_file.pdf")
    .await?;

ctx.log().info(format!("File copied: new_file_id={}", copied_file.file_id));

// Get folder contents
let folder_contents = ctx.storage()
    .get_folder_contents("/projects/project1", Some(100), Some(0))
    .await?;

ctx.log().info(format!(
    "Folder contents: {} files, {} subfolders",
    folder_contents.files.len(),
    folder_contents.subfolders.len()
));

// Recursively delete folder
ctx.storage()
    .delete_folder_recursive("/projects/project1")
    .await?;

ctx.log().info("Folder and all contents deleted");
```

## Storage Encryption

### File Encryption

```rust
use dms::prelude::*;
use serde_json::json;

// Configure storage encryption
let encryption_config = DMSCStorageEncryptionConfig {
    enabled: true,
    algorithm: DMSCEncryptionAlgorithm::AES256GCM,
    key_rotation_interval: Duration::from_days(90),
    encrypt_at_rest: true,
    encrypt_in_transit: true,
    key_management_service: DMSCKeyManagementService::AWSKMS,
    customer_managed_keys: true,
};

ctx.storage().init_encryption(encryption_config).await?;

// Upload encrypted file
let sensitive_file = Path::new("/path/to/sensitive_data.xlsx");
let encrypted_upload = ctx.storage()
    .upload_encrypted_file(sensitive_file, "encrypted_data.xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet")
    .await?;

ctx.log().info(format!(
    "Encrypted file uploaded: file_id={}, encryption_key_id={}",
    encrypted_upload.file_id,
    encrypted_upload.encryption_key_id
));

// Download and decrypt file
let decrypted_download = ctx.storage()
    .download_and_decrypt_file(&encrypted_upload.file_id, Path::new("/downloads/decrypted_data.xlsx"))
    .await?;

ctx.log().info(format!("File decrypted and downloaded: {:?}", decrypted_download));

// Rotate encryption key
let new_key_id = ctx.storage()
    .rotate_encryption_key(&encrypted_upload.file_id)
    .await?;

ctx.log().info(format!("Encryption key rotated: new_key_id={}", new_key_id));

// Client-side encrypted upload
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

### Key Management

```rust
use dms::prelude::*;

// Generate data encryption key
let data_key = ctx.storage()
    .generate_data_encryption_key(DMSCKeyAlgorithm::AES256)
    .await?;

ctx.log().info(format!("Data encryption key generated: key_id={}", data_key.key_id));

// Encrypt data key
let master_key_id = "master-key-123";
let encrypted_data_key = ctx.storage()
    .encrypt_data_key(&data_key.plaintext_key, master_key_id)
    .await?;

ctx.log().info("Data key encrypted with master key");

// Decrypt data key
let decrypted_data_key = ctx.storage()
    .decrypt_data_key(&encrypted_data_key, master_key_id)
    .await?;

ctx.log().info("Data key decrypted successfully");

// Securely delete key
ctx.storage()
    .secure_delete_key(&data_key.key_id)
    .await?;

ctx.log().info("Encryption key securely deleted");
```

## Storage Compression

### File Compression

```rust
use dms::prelude::*;
use serde_json::json;

// Configure storage compression
let compression_config = DMSCCompressionConfig {
    enabled: true,
    algorithm: DMSCCompressionAlgorithm::Zstd,
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

// Upload and compress file
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

// Download and decompress file
let decompressed_download = ctx.storage()
    .download_and_decompress_file(&compressed_upload.file_id, Path::new("/downloads/decompressed_log.txt"))
    .await?;

ctx.log().info(format!("File downloaded and decompressed: {:?}", decompressed_download));

// Batch compress existing files
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

### Smart Compression

```rust
use dms::prelude::*;

// Smart compression decision
let smart_compression = ctx.storage()
    .should_compress_file("large_dataset.csv", 1024 * 1024, "text/csv")
    .await?;

if smart_compression.should_compress {
    ctx.log().info(format!(
        "File should be compressed: estimated_savings={:.1}%, algorithm={}",
        smart_compression.estimated_savings * 100.0,
        smart_compression.recommended_algorithm
    ));
    
    // Execute smart compression
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

## Lifecycle Management

### File Lifecycle

```rust
use dms::prelude::*;
use serde_json::json;

// Configure lifecycle policy
let lifecycle_config = DMSCLifecycleConfig {
    enabled: true,
    default_retention_days: 365,
    archive_after_days: 90,
    delete_after_days: 1095, // 3 years
    auto_archive: true,
    auto_delete: false, // Requires manual confirmation for deletion
    lifecycle_policies: vec![
        DMSCLifecyclePolicy {
            name: "temporary_files".to_string(),
            file_patterns: vec!["*.tmp".to_string(), "*.temp".to_string()],
            retention_days: 7,
            archive_after_days: 3,
            delete_after_days: 30,
        },
        DMSCLifecyclePolicy {
            name: "log_files".to_string(),
            file_patterns: vec!["*.log".to_string(), "*.txt".to_string()],
            retention_days: 90,
            archive_after_days: 30,
            delete_after_days: 365,
        },
        DMSCLifecyclePolicy {
            name: "backup_files".to_string(),
            file_patterns: vec!["*.bak".to_string(), "*.backup".to_string()],
            retention_days: 180,
            archive_after_days: 60,
            delete_after_days: 730, // 2 years
        },
    ],
};

ctx.storage().init_lifecycle_management(lifecycle_config).await?;

// Apply lifecycle policies
ctx.storage()
    .apply_lifecycle_policies()
    .await?;

ctx.log().info("Lifecycle policies applied");

// Archive file
let file_id = "old_file_123";
let archive_result = ctx.storage()
    .archive_file(file_id, "cold_storage")
    .await?;

ctx.log().info(format!(
    "File archived: archive_tier={}, archive_date={}",
    archive_result.archive_tier,
    archive_result.archive_date
));

// Restore archived file
let restore_result = ctx.storage()
    .restore_archived_file(file_id, Duration::from_days(7))
    .await?;

ctx.log().info(format!(
    "File restored: restore_tier={}, available_by={}",
    restore_result.restore_tier,
    restore_result.available_by
));

// Batch archive
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

### Automatic Cleanup

```rust
use dms::prelude::*;

// Run automatic cleanup task
let cleanup_result = ctx.storage()
    .run_cleanup_task()
    .await?;

ctx.log().info(format!(
    "Cleanup task completed: {} files archived, {} files deleted, {} errors",
    cleanup_result.archived_count,
    cleanup_result.deleted_count,
    cleanup_result.error_count
));

// Get cleanup report
let cleanup_report = ctx.storage()
    .get_cleanup_report(chrono::Utc::now() - chrono::Duration::days(30), chrono::Utc::now())
    .await?;

ctx.log().info(format!(
    "Cleanup report for last 30 days: {} files processed, {} storage space reclaimed",
    cleanup_report.total_files_processed,
    cleanup_report.storage_space_reclaimed
));

// Configure automatic cleanup schedule
let cleanup_schedule = DMSCCleanupSchedule {
    enabled: true,
    schedule: "0 2 * * *".to_string(), // Daily at 2 AM
    max_files_per_run: 1000,
    dry_run: false,
    notification_email: "admin@company.com".to_string(),
};

ctx.storage()
    .configure_cleanup_schedule(cleanup_schedule)
    .await?;

ctx.log().info("Cleanup schedule configured");
```

## Version Control

### File Version Management

```rust
use dms::prelude::*;
use serde_json::json;

// Enable version control
let versioning_config = DMSCVersioningConfig {
    enabled: true,
    max_versions: 10,
    auto_versioning: true,
    version_on_metadata_change: true,
    retention_policy: DMSCVersionRetentionPolicy::KeepLastN(5),
    version_labels: vec!["draft".to_string(), "review".to_string(), "approved".to_string(), "final".to_string()],
};

ctx.storage().init_versioning(versioning_config).await?;

// Upload new version
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

// List file versions
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

// Download specific version
let version_number = 2;
let version_download = ctx.storage()
    .download_file_version(file_id, version_number, Path::new("/downloads/version2.pdf"))
    .await?;

ctx.log().info(format!("Version {} downloaded successfully", version_number));

// Restore file to specific version
let restored_version = ctx.storage()
    .restore_file_version(file_id, version_number)
    .await?;

ctx.log().info(format!(
    "File restored to version {}: new_version_id={}",
    version_number,
    restored_version.version_id
));

// Delete specific version
ctx.storage()
    .delete_file_version(file_id, 1) // Delete version 1
    .await?;

ctx.log().info("File version deleted");

// Label file version
ctx.storage()
    .label_file_version(file_id, 3, "approved")
    .await?;

ctx.log().info("Version 3 labeled as 'approved'");
```

### Version Comparison

```rust
use dms::prelude::*;

// Compare two versions
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

// Get version diff report
let diff_report = ctx.storage()
    .generate_version_diff_report(file_id, version1, version2)
    .await?;

ctx.log().info(format!("Version diff report generated: {}", diff_report.report_id));
```

## Monitoring and Statistics

### Storage Statistics

```rust
use dms::prelude::*;

// Get storage usage statistics
let storage_stats = ctx.storage()
    .get_storage_statistics()
    .await?;

ctx.log().info(format!(
    "Storage statistics: total_files={}, total_size={}, average_file_size={}",
    storage_stats.total_files,
    storage_stats.total_size,
    storage_stats.average_file_size
));

// Statistics by file type
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

// Statistics by time range
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

// Storage capacity monitoring
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

### Performance Monitoring

```rust
use dms::prelude::*;

// Get performance metrics
let performance_metrics = ctx.storage()
    .get_performance_metrics()
    .await?;

ctx.log().info(format!(
    "Performance metrics: avg_upload_time={}ms, avg_download_time={}ms, cache_hit_rate={:.1}%",
    performance_metrics.average_upload_time_ms,
    performance_metrics.average_download_time_ms,
    performance_metrics.cache_hit_rate * 100.0
));

// Get operation statistics
let operation_stats = ctx.storage()
    .get_operation_statistics()
    .await?;

ctx.log().info(format!(
    "Operation statistics: total_uploads={}, total_downloads={}, failed_operations={}",
    operation_stats.total_uploads,
    operation_stats.total_downloads,
    operation_stats.failed_operations
));

// Error statistics
let error_stats = ctx.storage()
    .get_error_statistics()
    .await?;

ctx.log().info("Error statistics by type:");
for (error_type, count) in error_stats.error_counts {
    ctx.log().info(format!("  {}: {} occurrences", error_type, count));
}
```

## Error Handling

### Storage Error Handling

```rust
use dms::prelude::*;
use serde_json::json;

// File upload error handling
match ctx.storage().upload_file(file_path, file_name, content_type).await {
    Ok(upload_result) => {
        ctx.log().info(format!("File uploaded successfully: {:?}", upload_result));
    }
    Err(DMSCError::StorageFull(e)) => {
        ctx.log().error(format!("Storage is full: {}", e));
        
        // Try to free up space
        let cleanup_result = ctx.storage().run_cleanup_task().await?;
        ctx.log().info(format!("Cleanup completed: {} bytes freed", cleanup_result.storage_space_reclaimed));
        
        // Retry upload
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
    Err(DMSCError::FileTooLarge(e)) => {
        ctx.log().error(format!("File is too large: {}", e));
        
        // Try chunked upload
        let chunk_config = DMSCChunkUploadConfig {
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
    Err(DMSCError::InvalidFileType(e)) => {
        ctx.log().error(format!("Invalid file type: {}", e));
        
        // Validate file type
        let allowed_types = vec!["image/jpeg", "image/png", "application/pdf"];
        let actual_type = ctx.storage().detect_file_type(file_path).await?;
        
        ctx.log().info(format!("Actual file type detected: {}", actual_type));
        
        if !allowed_types.contains(&actual_type.as_str()) {
            return Err(DMSCError::validation(format!("File type {} is not allowed", actual_type)));
        }
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected upload error: {}", e));
        return Err(e);
    }
}

// Storage service health check
let health_check = ctx.storage()
    .health_check()
    .await?;

if !health_check.is_healthy {
    ctx.log().error(format!("Storage service is unhealthy: {:?}", health_check.issues));
    
    // Try to reconnect storage service
    ctx.storage().reconnect().await?;
    ctx.log().info("Storage service reconnected");
}
```

## Best Practices

1. **File Naming**: Use meaningful file naming conventions
2. **Metadata**: Fully utilize metadata for file organization
3. **Lifecycle**: Configure appropriate file lifecycle policies
4. **Version Control**: Enable version control for important files
5. **Encryption**: Enable encryption for sensitive files
6. **Compression**: Enable compression for compressible files
7. **Monitoring**: Monitor storage usage and performance metrics
8. **Backup**: Implement regular backup strategies
9. **Access Control**: Implement file access control
10. **Error Handling**: Properly handle storage errors and exceptions
11. **Chunked Upload**: Use chunked upload for large files
12. **Temporary Links**: Use temporary download links for sensitive files
13. **Storage Tiers**: Use different storage tiers based on access frequency
14. **Capacity Planning**: Regularly monitor and plan storage capacity
15. **Compliance**: Ensure storage solution complies with relevant regulations

<div align="center">

## Running Steps

</div>

### Environment Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install DMSC CLI
cargo install dms-cli
```

### Create Project

```bash
# Create new DMSC project
dms new storage-app
cd storage-app

# Add storage dependency
cargo add dms-storage
```

### Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dms = "1.0"
dms-storage = "1.0"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### Configuration Creation

Create `config/storage.toml`:

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

### Run Example

```bash
# Build project
cargo build --release

# Run storage example
cargo run --example storage-demo
```

<div align="center">

## Expected Results

</div>

After successful execution, you will see output similar to:

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

## Extended Features

</div>

### Distributed Storage Cluster

```rust
use dms::prelude::*;
use serde_json::json;

// Configure distributed storage cluster
let cluster_config = DMSCStorageClusterConfig {
    nodes: vec![
        DMSCStorageNode {
            id: "node-1".to_string(),
            address: "storage1.company.com:9000".to_string(),
            capacity: 10 * 1024 * 1024 * 1024, // 10TB
            region: "us-east-1".to_string(),
            priority: 1,
            health_check_interval: Duration::from_secs(30),
        },
        DMSCStorageNode {
            id: "node-2".to_string(),
            address: "storage2.company.com:9000".to_string(),
            capacity: 10 * 1024 * 1024 * 1024, // 10TB
            region: "us-west-1".to_string(),
            priority: 2,
            health_check_interval: Duration::from_secs(30),
        },
        DMSCStorageNode {
            id: "node-3".to_string(),
            address: "storage3.company.com:9000".to_string(),
            capacity: 15 * 1024 * 1024 * 1024, // 15TB
            region: "eu-central-1".to_string(),
            priority: 3,
            health_check_interval: Duration::from_secs(30),
        },
    ],
    replication_factor: 3,
    consistency_level: DMSCConsistencyLevel::Quorum,
    load_balancing: DMSCLoadBalancing::RoundRobin,
    failover_timeout: Duration::from_secs(60),
    health_check_enabled: true,
    auto_rebalance: true,
};

ctx.storage().init_cluster(cluster_config).await?;

// Upload file to cluster (automatically selects optimal node)
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

// Get cluster health status
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

### Intelligent Storage Tiering

```rust
use dms::prelude::*;
use serde_json::json;

// Configure intelligent storage tiering
let tiering_config = DMSCTieringConfig {
    tiers: vec![
        DMSCTier {
            name: "hot".to_string(),
            storage_class: DMSCStorageClass::Hot,
            access_frequency_threshold: 0.8, // 80% access frequency
            retention_days: 30,
            cost_per_gb: 0.023,
            performance_class: "high".to_string(),
        },
        DMSCTier {
            name: "warm".to_string(),
            storage_class: DMSCStorageClass::Warm,
            access_frequency_threshold: 0.3, // 30% access frequency
            retention_days: 90,
            cost_per_gb: 0.0125,
            performance_class: "medium".to_string(),
        },
        DMSCTier {
            name: "cold".to_string(),
            storage_class: DMSCStorageClass::Cold,
            access_frequency_threshold: 0.1, // 10% access frequency
            retention_days: 365,
            cost_per_gb: 0.004,
            performance_class: "low".to_string(),
        },
        DMSCTier {
            name: "archive".to_string(),
            storage_class: DMSCStorageClass::Archive,
            access_frequency_threshold: 0.01, // 1% access frequency
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

// Get recommended storage tier for file
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

// Execute auto tiering
let tiering_result = ctx.storage()
    .execute_auto_tiering()
    .await?;

ctx.log().info(format!(
    "Auto-tiering completed: {} files moved, ${:.2} monthly savings achieved",
    tiering_result.files_moved,
    tiering_result.total_monthly_savings
));
```

### Real-time Storage Analysis

```rust
use dms::prelude::*;
use serde_json::json;

// Configure real-time storage analysis
let analytics_config = DMSCStorageAnalyticsConfig {
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

// Get real-time storage metrics
let real_time_metrics = ctx.storage()
    .get_real_time_metrics()
    .await?;

ctx.log().info(format!(
    "Real-time metrics: upload_throughput={}MB/s, download_throughput={}MB/s, active_connections={}",
    real_time_metrics.upload_throughput_mbps,
    real_time_metrics.download_throughput_mbps,
    real_time_metrics.active_connections
));

// Detect storage anomalies
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

// Predict storage needs
let storage_forecast = ctx.storage()
    .forecast_storage_needs(30) // Predict next 30 days
    .await?;

ctx.log().info(format!(
    "Storage forecast: predicted_usage={}GB, growth_rate={:.1}%, recommended_capacity={}GB",
    storage_forecast.predicted_usage_gb,
    storage_forecast.growth_rate * 100.0,
    storage_forecast.recommended_capacity_gb
));
```

### Multi-Cloud Storage Management

```rust
use dms::prelude::*;
use serde_json::json;

// Configure multi-cloud storage providers
let multi_cloud_config = DMSCMultiCloudConfig {
    providers: vec![
        DMSCCloudProvider {
            name: "aws-s3".to_string(),
            provider_type: DMSCCloudProviderType::AWSS3,
            region: "us-east-1".to_string(),
            bucket_name: "company-storage-bucket".to_string(),
            priority: 1,
            cost_per_gb: 0.023,
            encryption_enabled: true,
            versioning_enabled: true,
        },
        DMSCCloudProvider {
            name: "azure-blob".to_string(),
            provider_type: DMSCCloudProviderType::AzureBlob,
            region: "East US".to_string(),
            bucket_name: "company-storage-container".to_string(),
            priority: 2,
            cost_per_gb: 0.0208,
            encryption_enabled: true,
            versioning_enabled: true,
        },
        DMSCCloudProvider {
            name: "gcp-storage".to_string(),
            provider_type: DMSCCloudProviderType::GCPStorage,
            region: "us-central1".to_string(),
            bucket_name: "company-storage-bucket".to_string(),
            priority: 3,
            cost_per_gb: 0.020,
            encryption_enabled: true,
            versioning_enabled: true,
        },
    ],
    load_balancing: DMSCCloudLoadBalancing::CostOptimized,
    failover_enabled: true,
    data_replication: DMSCCloudDataReplication::CrossProvider,
    cost_optimization: true,
    provider_failover_timeout: Duration::from_secs(300),
};

ctx.storage().init_multi_cloud(multi_cloud_config).await?;

// Select optimal cloud provider to upload file
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

// Get multi-cloud storage cost analysis
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

## Summary

</div>

This example demonstrates the powerful storage management capabilities of the DMSC framework, helping you build efficient, secure, and scalable file storage systems. Through core features such as file upload/download, metadata management, storage encryption, compression, lifecycle management, version control, and monitoring statistics, you can easily manage enterprise-level file storage requirements.

### Core Features

1. **File Upload/Download**: Support for single file, multiple file, and chunked uploads, resumable downloads
2. **Metadata Management**: Flexible metadata operations, tag management, file search
3. **Storage Encryption**: AES256-GCM encryption, key rotation, client-side encryption support
4. **Storage Compression**: Intelligent compression algorithms, support for multiple file types
5. **Lifecycle Management**: Automatic archiving, deletion policies, tiered storage
6. **Version Control**: File version management, version comparison, label tagging
7. **Monitoring Statistics**: Storage usage statistics, performance monitoring, error statistics
8. **Error Handling**: Comprehensive error handling mechanisms, automatic retry strategies
9. **Temporary Links**: Secure one-time download links, access control
10. **File Organization**: Folder structure, batch operations, recursive processing

### Advanced Features

1. **Distributed Cluster**: Multi-node storage cluster, load balancing, failover
2. **Intelligent Tiering**: Automatic storage tiering based on access frequency
3. **Real-time Analysis**: Storage anomaly detection, demand forecasting, cost optimization
4. **Multi-Cloud Management**: Multi-cloud provider support, cost optimization, failover
5. **Access Control**: Fine-grained permission control, IP restrictions, user group management
6. **Performance Optimization**: Caching mechanisms, concurrent processing, network optimization

### Best Practices

- Use meaningful file naming conventions and organizational structures
- Fully utilize metadata and tags for file classification management
- Configure appropriate lifecycle policies based on business requirements
- Enable version control and automatic backup for important files
- Enable encryption and access control for sensitive files
- Enable compression for compressible files to save storage space
- Regularly monitor storage usage and performance metrics
- Implement comprehensive error handling and retry mechanisms
- Use chunked upload for large files to improve reliability
- Use temporary links to protect sensitive file downloads
- Implement storage tiering strategies based on access frequency
- Regularly perform capacity planning and cost optimization
- Ensure storage solution complies with relevant regulations

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2, and RBAC authentication and authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, learn how to use the caching module to improve application performance
- [database](./database.md): Database examples, learn database connections and query operations
- [http](./http.md): HTTP service examples, build web applications and RESTful APIs
- [mq](./mq.md): Message queue examples, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing, and security best practices

- [validation](./validation.md): Validation examples, data validation and sanitization operations
