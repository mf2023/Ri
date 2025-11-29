// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate dms;

use dms::fs::DMSFileSystem;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_fs_new_with_root() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    assert_eq!(fs._Fproject_root(), temp_dir.path());
}

#[test]
fn test_fs_new_auto_root() {
    let fs = DMSFileSystem::_Fnew_auto_root().unwrap();
    assert!(fs._Fproject_root().exists());
}

#[test]
fn test_fs_safe_mkdir() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let new_dir = temp_dir.path().join("test_dir");
    let result = fs._Fsafe_mkdir(&new_dir).unwrap();
    assert_eq!(result, new_dir);
    assert!(new_dir.exists());
}

#[test]
fn test_fs_ensure_parent_dir() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("parent_dir").join("child_file.txt");
    let result = fs._Fensure_parent_dir(&file_path).unwrap();
    assert_eq!(result, temp_dir.path().join("parent_dir"));
    assert!(result.exists());
}

#[test]
fn test_fs_atomic_write_text() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content = "Hello, DMS!";
    fs._Fatomic_write_text(&file_path, content).unwrap();
    let read_content = fs._Fread_text(&file_path).unwrap();
    assert_eq!(read_content, content);
}

#[test]
fn test_fs_atomic_write_bytes() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_bytes.bin");
    let content = b"Hello, DMS in bytes!";
    fs._Fatomic_write_bytes(&file_path, content).unwrap();
    let read_content = fs._Fread_text(&file_path).unwrap();
    assert_eq!(read_content, String::from_utf8_lossy(content));
}

#[test]
fn test_fs_read_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    let json_str = serde_json::to_string(&test_data).unwrap();
    fs._Fatomic_write_text(&file_path, &json_str).unwrap();
    
    let read_data: TestData = fs._Fread_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
fn test_fs_exists() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    assert!(!fs._Fexists(&file_path));
    fs._Fatomic_write_text(&file_path, "test").unwrap();
    assert!(fs._Fexists(&file_path));
}

#[test]
fn test_fs_remove_file() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    fs._Fatomic_write_text(&file_path, "test").unwrap();
    assert!(fs._Fexists(&file_path));
    fs._Fremove_file(&file_path).unwrap();
    assert!(!fs._Fexists(&file_path));
}

#[test]
fn test_fs_remove_dir_all() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let dir_path = temp_dir.path().join("test_dir");
    fs._Fsafe_mkdir(&dir_path).unwrap();
    let file_path = dir_path.join("test_file.txt");
    fs._Fatomic_write_text(&file_path, "test").unwrap();
    assert!(fs._Fexists(&dir_path));
    assert!(fs._Fexists(&file_path));
    fs._Fremove_dir_all(&dir_path).unwrap();
    assert!(!fs._Fexists(&dir_path));
    assert!(!fs._Fexists(&file_path));
}

#[test]
fn test_fs_copy_file() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let src_path = temp_dir.path().join("src.txt");
    let dst_path = temp_dir.path().join("dst.txt");
    let content = "Hello, DMS!";
    fs._Fatomic_write_text(&src_path, content).unwrap();
    fs._Fcopy_file(&src_path, &dst_path).unwrap();
    let dst_content = fs._Fread_text(&dst_path).unwrap();
    assert_eq!(dst_content, content);
}

#[test]
fn test_fs_append_text() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test_file.txt");
    let content1 = "Hello, ";
    let content2 = "DMS!";
    fs._Fatomic_write_text(&file_path, content1).unwrap();
    fs._Fappend_text(&file_path, content2).unwrap();
    let read_content = fs._Fread_text(&file_path).unwrap();
    assert_eq!(read_content, content1.to_owned() + content2);
}

#[test]
fn test_fs_write_json() {
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let file_path = temp_dir.path().join("test.json");
    let test_data = TestData { name: "test".to_string(), value: 42 };
    fs._Fwrite_json(&file_path, &test_data).unwrap();
    
    let read_data: TestData = fs._Fread_json(&file_path).unwrap();
    assert_eq!(read_data, test_data);
}

#[test]
fn test_fs_category_dirs() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    assert!(fs._Fapp_dir().exists());
    assert!(fs._Flogs_dir().exists());
    assert!(fs._Fcache_dir().exists());
    assert!(fs._Freports_dir().exists());
    assert!(fs._Fobservability_dir().exists());
    assert!(fs._Ftemp_dir().exists());
}

#[test]
fn test_fs_ensure_category_path() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    let path = fs._Fensure_category_path("logs", "test.log");
    assert!(path.parent().unwrap().exists());
    assert!(path.starts_with(fs._Flogs_dir()));
}

#[test]
fn test_fs_normalize_under_category() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    let path = fs._Fnormalize_under_category("cache", "subdir/test.cache");
    assert!(path.starts_with(fs._Fcache_dir()));
    assert_eq!(path.file_name().unwrap(), "test.cache");
}
