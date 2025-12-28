<div align="center">

# 存储功能使用示例

**版本：1.0.0**

**最后修改日期：2025-12-12**

本示例展示如何使用DMSC的storage模块进行文件上传下载、对象存储、本地文件系统操作、云存储集成等。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 文件上传和下载
- 对象存储管理
- 本地文件系统操作
- 云存储服务集成（AWS S3、阿里云OSS等）
- 文件压缩和解压
- 文件加密存储
- 存储配额管理
- 文件版本控制

## 环境准备

### 依赖安装

```bash
pip install dmsc[storage,compression,crypto]
```

### 配置文件

```yaml
# config.yaml
storage:
  default:
    type: local
    base_path: ./data/storage
    max_file_size: 104857600  # 100MB
    allowed_extensions: [".jpg", ".png", ".pdf", ".txt", ".docx"]
  
  s3:
    type: s3
    bucket: dmsc-demo-bucket
    region: us-east-1
    access_key_id: ${AWS_ACCESS_KEY_ID}
    secret_access_key: ${AWS_SECRET_ACCESS_KEY}
    endpoint_url: https://s3.amazonaws.com
    max_file_size: 524288000  # 500MB
  
  oss:
    type: oss
    bucket: dmsc-demo-oss
    endpoint: https://oss-cn-hangzhou.aliyuncs.com
    access_key_id: ${ALI_ACCESS_KEY_ID}
    access_key_secret: ${ALI_ACCESS_KEY_SECRET}
    max_file_size: 1048576000  # 1GB
  
  compression:
    enabled: true
    algorithms: ["gzip", "brotli", "lz4"]
    default_level: 6
  
  encryption:
    enabled: true
    algorithm: AES-256-GCM
    key_derivation: PBKDF2
  
  versioning:
    enabled: true
    max_versions: 10
    retention_days: 90
```

## 完整代码

```python
import asyncio
import os
import json
import time
import random
import tempfile
from datetime import datetime
from pathlib import Path
from typing import Dict, List, Optional, Any, BinaryIO
from dataclasses import dataclass, asdict
from dmsc import DMSC, DMSCContext
from dmsc.storage import (
    StorageManager, FileMetadata, StorageConfig,
    CompressionManager, EncryptionManager
)


@dataclass
class FileUploadRequest:
    """文件上传请求"""
    filename: str
    content_type: str
    size: int
    data: bytes
    metadata: Optional[Dict[str, Any]] = None
    compression_enabled: bool = True
    encryption_enabled: bool = True


@dataclass
class FileDownloadResponse:
    """文件下载响应"""
    filename: str
    content_type: str
    size: int
    data: bytes
    metadata: Optional[Dict[str, Any]] = None
    compression_applied: bool = False
    encryption_applied: bool = False


@dataclass
class StorageQuota:
    """存储配额信息"""
    total_space: int
    used_space: int
    available_space: int
    file_count: int
    max_file_size: int


class LocalStorageExample:
    """本地存储示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.storage = StorageManager()
        self.logger = ctx.logger.getChild("local_storage_example")
    
    async def basic_file_operations(self):
        """基础文件操作"""
        self.logger.info("=== 本地存储基础操作 ===")
        
        # 创建测试文件
        test_files = [
            {
                "filename": "document.txt",
                "content_type": "text/plain",
                "content": "This is a test document content.\nIt contains multiple lines of text."
            },
            {
                "filename": "data.json",
                "content_type": "application/json",
                "content": json.dumps({"name": "test", "value": 123, "timestamp": datetime.now().isoformat()})
            },
            {
                "filename": "image.png",
                "content_type": "image/png",
                "content": b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x01\x00\x00\x00\x01\x08\x02\x00\x00\x00\x90wS\xde\x00\x00\x00\x0cIDATx\x9cc\xf8\x0f\x00\x00\x01\x01\x00\x05\x18\xd4c\x00\x00\x00\x00IEND\xaeB`\x82"
            }
        ]
        
        uploaded_files = []
        
        # 上传文件
        for file_data in test_files:
            upload_request = FileUploadRequest(
                filename=file_data["filename"],
                content_type=file_data["content_type"],
                size=len(file_data["content"]),
                data=file_data["content"] if isinstance(file_data["content"], bytes) else file_data["content"].encode(),
                metadata={"uploaded_by": "system", "source": "demo"}
            )
            
            file_id = await self.storage.upload_file(upload_request)
            uploaded_files.append(file_id)
            
            self.logger.info("file_uploaded", 
                           filename=file_data["filename"],
                           file_id=file_id,
                           size=upload_request.size)
        
        # 列出文件
        file_list = await self.storage.list_files()
        self.logger.info("file_list", count=len(file_list), files=[f["filename"] for f in file_list])
        
        # 下载文件
        for file_id in uploaded_files:
            download_response = await self.storage.download_file(file_id)
            
            self.logger.info("file_downloaded",
                           filename=download_response.filename,
                           size=download_response.size,
                           content_type=download_response.content_type)
            
            # 验证文件内容
            original_file = next(f for f in test_files if f["filename"] == download_response.filename)
            original_content = original_file["content"] if isinstance(original_file["content"], bytes) else original_file["content"].encode()
            
            content_match = original_content == download_response.data
            self.logger.info("content_verification", filename=download_response.filename, match=content_match)
        
        # 获取文件元数据
        for file_id in uploaded_files:
            metadata = await self.storage.get_file_metadata(file_id)
            self.logger.info("file_metadata", file_id=file_id, metadata=metadata)
        
        # 删除文件
        for file_id in uploaded_files[:2]:  # 删除前两个文件
            success = await self.storage.delete_file(file_id)
            self.logger.info("file_deleted", file_id=file_id, success=success)
        
        # 验证删除
        remaining_files = await self.storage.list_files()
        self.logger.info("remaining_files", count=len(remaining_files))
        
        return uploaded_files
    
    async def directory_operations(self):
        """目录操作"""
        self.logger.info("=== 目录操作 ===")
        
        # 创建目录结构
        directories = [
            "documents/2024",
            "documents/2024/reports",
            "images/products",
            "data/export",
            "temp/processing"
        ]
        
        for directory in directories:
            created = await self.storage.create_directory(directory)
            self.logger.info("directory_created", path=directory, created=created)
        
        # 在目录中创建文件
        directory_files = [
            ("documents/2024/annual_report.pdf", "application/pdf", b"PDF content here"),
            ("documents/2024/reports/monthly_01.pdf", "application/pdf", b"January report"),
            ("images/products/product_001.jpg", "image/jpeg", b"JPEG image data"),
            ("data/export/customers_2024.csv", "text/csv", b"name,email\nJohn,john@example.com"),
        ]
        
        file_ids = []
        for filepath, content_type, content in directory_files:
            upload_request = FileUploadRequest(
                filename=filepath,
                content_type=content_type,
                size=len(content),
                data=content,
                metadata={"directory": True, "created_at": datetime.now().isoformat()}
            )
            
            file_id = await self.storage.upload_file(upload_request)
            file_ids.append(file_id)
        
        # 列出目录内容
        for directory in ["documents/2024", "images/products", "data/export"]:
            contents = await self.storage.list_directory(directory)
            self.logger.info("directory_contents", directory=directory, files=len(contents))
        
        # 获取目录统计
        for directory in ["documents", "images", "data"]:
            stats = await self.storage.get_directory_stats(directory)
            self.logger.info("directory_stats", directory=directory, stats=stats)
        
        return file_ids
    
    async def file_search_demo(self):
        """文件搜索示例"""
        self.logger.info("=== 文件搜索示例 ===")
        
        # 创建测试文件用于搜索
        search_files = [
            ("report_2024_q1.pdf", "application/pdf", b"Q1 2024 Financial Report"),
            ("report_2024_q2.pdf", "application/pdf", b"Q2 2024 Financial Report"),
            ("presentation_sales.pptx", "application/vnd.openxmlformats-officedocument.presentationml.presentation", b"PPTX data"),
            ("meeting_notes_2024.txt", "text/plain", b"Meeting notes from 2024 planning session"),
            ("budget_2024.xlsx", "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", b"Excel budget data"),
        ]
        
        uploaded_ids = []
        for filename, content_type, content in search_files:
            upload_request = FileUploadRequest(
                filename=filename,
                content_type=content_type,
                size=len(content),
                data=content,
                metadata={"year": 2024, "type": "document", "searchable": True}
            )
            
            file_id = await self.storage.upload_file(upload_request)
            uploaded_ids.append(file_id)
        
        # 按文件名搜索
        search_results = await self.storage.search_files("report")
        self.logger.info("search_by_name", query="report", results=len(search_results))
        
        # 按元数据搜索
        metadata_search = await self.storage.search_by_metadata({"year": 2024, "type": "document"})
        self.logger.info("search_by_metadata", results=len(metadata_search))
        
        # 按文件类型过滤
        pdf_files = await self.storage.search_by_type("application/pdf")
        self.logger.info("search_by_type", type="application/pdf", results=len(pdf_files))
        
        # 按日期范围搜索
        date_range_results = await self.storage.search_by_date_range(
            start_date=datetime(2024, 1, 1),
            end_date=datetime(2024, 12, 31)
        )
        self.logger.info("search_by_date_range", results=len(date_range_results))
        
        return uploaded_ids


class CompressionExample:
    """压缩解压示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.compression = CompressionManager()
        self.logger = ctx.logger.getChild("compression_example")
    
    async def compression_demo(self):
        """压缩示例"""
        self.logger.info("=== 文件压缩示例 ===")
        
        # 创建测试数据
        test_data = {
            "users": [
                {"id": i, "name": f"User {i}", "email": f"user{i}@example.com", "data": "x" * 1000}
                for i in range(100)
            ],
            "metadata": {
                "version": "1.0",
                "created_at": datetime.now().isoformat(),
                "total_records": 100,
                "description": "This is a large dataset for compression testing"
            }
        }
        
        json_data = json.dumps(test_data, indent=2)
        original_size = len(json_data.encode())
        
        self.logger.info("original_data", size=original_size, records=len(test_data["users"]))
        
        # 使用不同压缩算法
        compression_algorithms = ["gzip", "brotli", "lz4", "zstd"]
        compression_results = {}
        
        for algorithm in compression_algorithms:
            # 压缩数据
            compressed_data = await self.compression.compress(json_data.encode(), algorithm=algorithm)
            compressed_size = len(compressed_data)
            compression_ratio = (1 - compressed_size / original_size) * 100
            
            # 解压数据
            decompressed_data = await self.compression.decompress(compressed_data, algorithm=algorithm)
            decompressed_text = decompressed_data.decode()
            
            # 验证数据完整性
            is_intact = json_data == decompressed_text
            
            compression_results[algorithm] = {
                "original_size": original_size,
                "compressed_size": compressed_size,
                "compression_ratio": compression_ratio,
                "is_intact": is_intact
            }
            
            self.logger.info("compression_result",
                           algorithm=algorithm,
                           ratio=f"{compression_ratio:.1f}%",
                           intact=is_intact)
        
        # 压缩级别对比
        self.logger.info("=== 压缩级别对比 ===")
        
        for level in [1, 3, 6, 9]:  # gzip压缩级别
            compressed = await self.compression.compress(json_data.encode(), algorithm="gzip", level=level)
            ratio = (1 - len(compressed) / original_size) * 100
            
            self.logger.info("compression_level_test", level=level, ratio=f"{ratio:.1f}%")
        
        return compression_results
    
    async def archive_demo(self):
        """归档示例"""
        self.logger.info("=== 文件归档示例 ===")
        
        # 创建临时文件用于归档
        temp_files = []
        
        with tempfile.TemporaryDirectory() as temp_dir:
            # 创建多个文件
            file_contents = [
                ("document1.txt", "This is the first document content.\n" * 100),
                ("document2.txt", "This is the second document content.\n" * 150),
                ("data.json", json.dumps({"items": [{"id": i, "value": f"item_{i}"} for i in range(50)]})),
                ("README.md", "# Archive Demo\n\nThis is a demonstration of file archiving.\n" * 20),
            ]
            
            for filename, content in file_contents:
                filepath = Path(temp_dir) / filename
                filepath.write_text(content)
                temp_files.append(str(filepath))
            
            # 创建归档文件
            archive_formats = ["zip", "tar", "tar.gz"]
            archive_results = {}
            
            for archive_format in archive_formats:
                archive_path = Path(temp_dir) / f"archive.{archive_format}"
                
                # 创建归档
                await self.compression.create_archive(temp_files, str(archive_path), format=archive_format)
                
                # 获取归档信息
                archive_info = await self.compression.get_archive_info(str(archive_path))
                
                archive_results[archive_format] = {
                    "size": archive_info["size"],
                    "file_count": archive_info["file_count"],
                    "compression_ratio": archive_info.get("compression_ratio", 0)
                }
                
                self.logger.info("archive_created",
                               format=archive_format,
                               size=archive_info["size"],
                               files=archive_info["file_count"])
                
                # 解压归档
                extract_dir = Path(temp_dir) / f"extracted_{archive_format}"
                await self.compression.extract_archive(str(archive_path), str(extract_dir))
                
                # 验证解压结果
                extracted_files = list(extract_dir.rglob("*")) if extract_dir.exists() else []
                self.logger.info("archive_extracted",
                               format=archive_format,
                               extracted_files=len([f for f in extracted_files if f.is_file()]))
            
            return archive_results


class EncryptionStorageExample:
    """加密存储示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.storage = StorageManager()
        self.logger = ctx.logger.getChild("encryption_storage_example")
    
    async def encrypted_storage_demo(self):
        """加密存储示例"""
        self.logger.info("=== 加密存储示例 ===")
        
        # 生成加密密钥
        encryption_key = await self.storage.generate_encryption_key()
        
        # 敏感数据
        sensitive_documents = [
            {
                "filename": "financial_report_2024.pdf",
                "content_type": "application/pdf",
                "content": b"CONFIDENTIAL: Financial report for Q4 2024\nRevenue: $1,234,567\nExpenses: $987,654",
                "classification": "confidential"
            },
            {
                "filename": "employee_data.json",
                "content_type": "application/json",
                "content": json.dumps({
                    "employees": [
                        {"name": "John Doe", "ssn": "123-45-6789", "salary": 75000},
                        {"name": "Jane Smith", "ssn": "987-65-4321", "salary": 85000}
                    ],
                    "classification": "restricted"
                }).encode(),
                "classification": "restricted"
            },
            {
                "filename": "customer_database.csv",
                "content_type": "text/csv",
                "content": b"name,email,credit_card\nJohn,john@example.com,1234-5678-9012-3456\nJane,jane@example.com,9876-5432-1098-7654",
                "classification": "confidential"
            }
        ]
        
        encrypted_files = []
        
        # 上传加密文件
        for doc in sensitive_documents:
            upload_request = FileUploadRequest(
                filename=doc["filename"],
                content_type=doc["content_type"],
                size=len(doc["content"]),
                data=doc["content"],
                metadata={
                    "classification": doc["classification"],
                    "encrypted": True,
                    "uploaded_at": datetime.now().isoformat()
                },
                encryption_enabled=True
            )
            
            file_id = await self.storage.upload_encrypted_file(upload_request, encryption_key)
            encrypted_files.append(file_id)
            
            self.logger.info("encrypted_file_uploaded",
                           filename=doc["filename"],
                           file_id=file_id,
                           classification=doc["classification"])
        
        # 下载并解密文件
        for file_id in encrypted_files:
            download_response = await self.storage.download_encrypted_file(file_id, encryption_key)
            
            self.logger.info("encrypted_file_downloaded",
                           filename=download_response.filename,
                           size=download_response.size,
                           encryption_applied=download_response.encryption_applied)
            
            # 验证解密结果
            original_doc = next(d for d in sensitive_documents if d["filename"] == download_response.filename)
            decryption_successful = original_doc["content"] == download_response.data
            
            self.logger.info("decryption_verification",
                           filename=download_response.filename,
                           successful=decryption_successful)
        
        # 管理加密密钥
        key_info = await self.storage.get_encryption_key_info(encryption_key)
        self.logger.info("encryption_key_info", key_info=key_info)
        
        return encrypted_files, encryption_key


class CloudStorageExample:
    """云存储示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.storage = StorageManager()
        self.logger = ctx.logger.getChild("cloud_storage_example")
    
    async def cloud_storage_demo(self):
        """云存储操作示例"""
        self.logger.info("=== 云存储操作示例 ===")
        
        # 创建测试文件
        cloud_files = [
            {
                "filename": "website_assets/logo.png",
                "content_type": "image/png",
                "content": b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x00\x80\x00\x00\x00\x20\x08\x02\x00\x00\x00\x90\x91\x68\x05\x00\x00\x00\x0cIDATx\x9cc\xf8\x0f\x00\x00\x01\x01\x00\x05\x18\xd4c\x00\x00\x00\x00IEND\xaeB`\x82",
                "tags": ["website", "logo", "assets"],
                "cache_control": "public, max-age=31536000"
            },
            {
                "filename": "backups/database_backup_2024.sql",
                "content_type": "application/sql",
                "content": b"-- Database backup\nCREATE TABLE users (id INT, name VARCHAR(255));\nINSERT INTO users VALUES (1, 'John'), (2, 'Jane');",
                "tags": ["backup", "database", "sql"],
                "storage_class": "STANDARD_IA"  # 低频访问存储
            },
            {
                "filename": "public/downloads/software_v1.0.exe",
                "content_type": "application/x-msdownload",
                "content": b"MZ\x90\x00\x03" + b"\x00" * 1000,  # 模拟可执行文件
                "tags": ["software", "download", "public"],
                "cache_control": "public, max-age=604800"
            }
        ]
        
        uploaded_files = []
        
        # 上传到云存储
        for file_data in cloud_files:
            upload_request = FileUploadRequest(
                filename=file_data["filename"],
                content_type=file_data["content_type"],
                size=len(file_data["content"]),
                data=file_data["content"],
                metadata={
                    "tags": file_data["tags"],
                    "cache_control": file_data.get("cache_control"),
                    "storage_class": file_data.get("storage_class", "STANDARD"),
                    "uploaded_to": "cloud"
                }
            )
            
            # 选择存储类型
            if "website" in file_data["tags"]:
                storage_type = "s3"  # 使用S3存储网站资源
            elif "backup" in file_data["tags"]:
                storage_type = "oss"  # 使用OSS存储备份
            else:
                storage_type = "s3"  # 默认使用S3
            
            file_id = await self.storage.upload_to_cloud(upload_request, storage_type)
            uploaded_files.append(file_id)
            
            self.logger.info("cloud_file_uploaded",
                           filename=file_data["filename"],
                           file_id=file_id,
                           storage_type=storage_type)
        
        # 获取云存储信息
        for file_id in uploaded_files:
            cloud_info = await self.storage.get_cloud_storage_info(file_id)
            self.logger.info("cloud_storage_info", file_id=file_id, info=cloud_info)
        
        # 生成预签名URL
        for file_id in uploaded_files:
            if "public" in cloud_files[uploaded_files.index(file_id)]["tags"]:
                # 生成下载URL
                download_url = await self.storage.generate_presigned_url(file_id, operation="download", expiry=3600)
                self.logger.info("presigned_download_url", file_id=file_id, url=download_url[:50] + "...")
                
                # 生成上传URL（如果需要客户端上传）
                upload_url = await self.storage.generate_presigned_url(file_id, operation="upload", expiry=1800)
                self.logger.info("presigned_upload_url", file_id=file_id, url=upload_url[:50] + "...")
        
        # 云存储管理操作
        storage_stats = await self.storage.get_cloud_storage_stats()
        self.logger.info("cloud_storage_stats", stats=storage_stats)
        
        return uploaded_files


class StorageQuotaExample:
    """存储配额示例"""
    
    def __init__(self, ctx: DMSCContext):
        self.ctx = ctx
        self.storage = StorageManager()
        self.logger = ctx.logger.getChild("quota_example")
    
    async def quota_management_demo(self):
        """配额管理示例"""
        self.logger.info("=== 存储配额管理示例 ===")
        
        # 设置用户配额
        user_quotas = {
            "user_1": {"max_space": 100 * 1024 * 1024, "max_files": 1000},  # 100MB, 1000文件
            "user_2": {"max_space": 500 * 1024 * 1024, "max_files": 5000},  # 500MB, 5000文件
            "user_3": {"max_space": 1024 * 1024 * 1024, "max_files": 10000},  # 1GB, 10000文件
        }
        
        for user_id, quota in user_quotas.items():
            await self.storage.set_user_quota(user_id, quota)
            self.logger.info("user_quota_set", user_id=user_id, quota=quota)
        
        # 模拟用户上传文件
        test_uploads = [
            ("user_1", "document_1.pdf", 1024 * 1024),  # 1MB
            ("user_1", "image_1.jpg", 2 * 1024 * 1024),  # 2MB
            ("user_2", "video_1.mp4", 50 * 1024 * 1024),  # 50MB
            ("user_3", "backup_1.zip", 200 * 1024 * 1024),  # 200MB
        ]
        
        upload_results = []
        
        for user_id, filename, file_size in test_uploads:
            # 检查配额
            quota_info = await self.storage.check_user_quota(user_id, file_size)
            
            if quota_info["can_upload"]:
                # 模拟上传文件
                upload_request = FileUploadRequest(
                    filename=filename,
                    content_type="application/octet-stream",
                    size=file_size,
                    data=b"x" * file_size,  # 模拟文件内容
                    metadata={"user_id": user_id, "uploaded_at": datetime.now().isoformat()}
                )
                
                file_id = await self.storage.upload_file(upload_request)
                
                # 更新用户配额使用
                await self.storage.update_user_quota_usage(user_id, file_size, 1)
                
                upload_results.append({
                    "user_id": user_id,
                    "filename": filename,
                    "file_id": file_id,
                    "size": file_size,
                    "status": "success"
                })
                
                self.logger.info("file_uploaded_within_quota",
                               user_id=user_id,
                               filename=filename,
                               size=file_size)
            else:
                upload_results.append({
                    "user_id": user_id,
                    "filename": filename,
                    "size": file_size,
                    "status": "quota_exceeded",
                    "reason": quota_info["reason"]
                })
                
                self.logger.warning("quota_exceeded",
                                  user_id=user_id,
                                  filename=filename,
                                  reason=quota_info["reason"])
        
        # 获取用户配额使用情况
        for user_id in ["user_1", "user_2", "user_3"]:
            quota_usage = await self.storage.get_user_quota_usage(user_id)
            self.logger.info("quota_usage", user_id=user_id, usage=quota_usage)
        
        # 配额超限测试
        large_file_upload = FileUploadRequest(
            filename="large_file.iso",
            content_type="application/octet-stream",
            size=150 * 1024 * 1024,  # 150MB
            data=b"x" * (150 * 1024 * 1024),
            metadata={"user_id": "user_1"}
        )
        
        quota_check = await self.storage.check_user_quota("user_1", large_file_upload.size)
        self.logger.info("large_file_quota_check", 
                        user_id="user_1", 
                        can_upload=quota_check["can_upload"],
                        reason=quota_check.get("reason"))
        
        return upload_results


async def main():
    """主函数"""
    # 创建DMSC应用
    dmsc = DMSC()
    
    # 启动应用
    async with dmsc.run() as ctx:
        # 本地存储示例
        local_storage = LocalStorageExample(ctx)
        await local_storage.basic_file_operations()
        await local_storage.directory_operations()
        await local_storage.file_search_demo()
        
        # 压缩示例
        compression = CompressionExample(ctx)
        await compression.compression_demo()
        await compression.archive_demo()
        
        # 加密存储示例
        encryption_storage = EncryptionStorageExample(ctx)
        await encryption_storage.encrypted_storage_demo()
        
        # 云存储示例
        cloud_storage = CloudStorageExample(ctx)
        await cloud_storage.cloud_storage_demo()
        
        # 配额管理示例
        quota_management = StorageQuotaExample(ctx)
        await quota_management.quota_management_demo()
        
        ctx.logger.info("storage_demo", "所有存储示例执行完成！")


if __name__ == "__main__":
    # 运行应用
    asyncio.run(main())
```

## 运行示例

### 基本运行

```bash
python storage_example.py
```

### 使用配置文件

```bash
python storage_example.py --config config.yaml
```

### 云存储配置

```bash
# 配置AWS S3
export AWS_ACCESS_KEY_ID=your_access_key
export AWS_SECRET_ACCESS_KEY=your_secret_key

# 配置阿里云OSS
export ALI_ACCESS_KEY_ID=your_access_key
export ALI_ACCESS_KEY_SECRET=your_secret_key

python storage_example.py --config config.yaml
```

### 后台运行

```bash
# 作为后台服务运行
nohup python storage_example.py > storage_demo.log 2>&1 &

# 查看日志
tail -f storage_demo.log
```

## 关键特性

### 1. 多存储后端
- 本地文件系统存储
- AWS S3集成
- 阿里云OSS支持
- 统一API接口

### 2. 文件管理
- 文件上传下载
- 目录操作
- 文件搜索
- 元数据管理

### 3. 数据压缩
- 多种压缩算法
- 压缩级别配置
- 归档文件支持
- 自动压缩检测

### 4. 安全存储
- 文件加密
- 密钥管理
- 访问控制
- 安全传输

### 5. 云存储功能
- 预签名URL
- 存储类别选择
- CDN集成
- 跨区域复制

### 6. 配额管理
- 用户配额设置
- 使用情况统计
- 超限检测
- 自动清理

## 最佳实践

### 1. 文件组织
- 合理的目录结构
- 文件命名规范
- 元数据标记
- 版本控制

### 2. 性能优化
- 大文件分片上传
- 并发下载
- 缓存策略
- CDN使用

### 3. 安全策略
- 文件类型限制
- 大小限制
- 访问权限
- 审计日志

### 4. 成本控制
- 存储类别选择
- 生命周期管理
- 清理策略
- 监控告警

## 故障排除

### 常见问题

1. **上传失败**
   - 检查文件大小限制
   - 验证存储后端连接
   - 确认权限配置

2. **下载速度慢**
   - 检查网络连接
   - 使用CDN加速
   - 考虑就近存储

3. **存储空间不足**
   - 检查配额限制
   - 清理过期文件
   - 升级存储计划

4. **加密失败**
   - 验证密钥有效性
   - 检查加密算法支持
   - 确认数据完整性

### 调试技巧

```python
# 启用存储调试日志
import logging
logging.getLogger("dmsc.storage").setLevel(logging.DEBUG)

# 检查存储配置
storage_config = await ctx.storage.get_storage_config()
print(f"Storage type: {storage_config['type']}")

# 验证存储连接
connection_test = await ctx.storage.test_connection()
print(f"Connection status: {connection_test['status']}")

# 查看存储统计
storage_stats = await ctx.storage.get_storage_stats()
print(f"Total files: {storage_stats['file_count']}")
print(f"Total size: {storage_stats['total_size']}")
```

## 相关链接

- [存储API参考](../04-api-reference/storage.md)
- [安全功能使用示例](security.md)
- [配置管理使用示例](config.md)
- [DMSC官方文档](https://dmsc.org/docs)