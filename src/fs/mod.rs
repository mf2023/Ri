//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#![allow(non_snake_case)]

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::OpenOptions;
use std::time::SystemTime;

use crate::core::DMSResult;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Internal filesystem implementation.
#[derive(Clone)]
struct _CFileSystemImpl {
    project_root: PathBuf,
    app_data_root: PathBuf,
}

impl _CFileSystemImpl {
    fn _Fnew_with_roots(project_root: PathBuf, app_data_root: PathBuf) -> Self {
        _CFileSystemImpl { project_root, app_data_root }
    }

    fn _Fnew_with_root(project_root: PathBuf) -> Self {
        // Default app data root under project root; can be overridden by core/config.
        let app_data_root = project_root.join(".dms");
        _CFileSystemImpl::_Fnew_with_roots(project_root, app_data_root)
    }

    fn _Fproject_root(&self) -> &Path {
        &self.project_root
    }

    fn _Fsafe_mkdir(&self, path: &Path) -> DMSResult<PathBuf> {
        fs::create_dir_all(path).map_err(|e| crate::core::DMSError::Other(format!("safe_mkdir failed: {}", e)))?;
        Ok(path.to_path_buf())
    }

    fn _Fensure_parent_dir(&self, path: &Path) -> DMSResult<PathBuf> {
        if let Some(parent) = path.parent() {
            self._Fsafe_mkdir(parent)
        } else {
            Ok(self.project_root.clone())
        }
    }

    fn _Fatomic_write_text(&self, path: &Path, text: &str) -> DMSResult<()> {
        self._Fensure_parent_dir(path)?;
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {}", e)))?;
        let tmp_name = format!(".tmp_{}_{}", ts.as_millis(), path.file_name().and_then(|s| s.to_str()).unwrap_or("tmp"));
        let tmp_path = dir.join(tmp_name);

        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text open tmp failed: {}", e)))?;
            file.write_all(text.as_bytes())
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text write failed: {}", e)))?;
            file.sync_all()
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text sync failed: {}", e)))?;
        }

        fs::rename(&tmp_path, path)
            .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_text rename failed: {}", e)))?;

        Ok(())
    }

    fn _Fatomic_write_bytes(&self, path: &Path, data: &[u8]) -> DMSResult<()> {
        self._Fensure_parent_dir(path)?;
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {}", e)))?;
        let tmp_name = format!(".tmp_{}_{}", ts.as_millis(), path.file_name().and_then(|s| s.to_str()).unwrap_or("tmp"));
        let tmp_path = dir.join(tmp_name);

        {
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&tmp_path)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes open tmp failed: {}", e)))?;
            file.write_all(data)
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes write failed: {}", e)))?;
            file.sync_all()
                .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes sync failed: {}", e)))?;
        }

        fs::rename(&tmp_path, path)
            .map_err(|e| crate::core::DMSError::Other(format!("atomic_write_bytes rename failed: {}", e)))?;

        Ok(())
    }

    fn _Fread_text(&self, path: &Path) -> DMSResult<String> {
        let mut file = OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(|e| crate::core::DMSError::Other(format!("read_text open failed: {}", e)))?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|e| crate::core::DMSError::Other(format!("read_text read failed: {}", e)))?;
        Ok(buf)
    }

    fn _Fapp_dir(&self) -> PathBuf {
        let _ = fs::create_dir_all(&self.app_data_root);
        self.app_data_root.clone()
    }

    fn _Fcategory_dir(&self, name: &str) -> PathBuf {
        let dir = self._Fapp_dir().join(name);
        let _ = fs::create_dir_all(&dir);
        dir
    }
}

/// Public-facing filesystem class.
#[derive(Clone)]
pub struct DMSFileSystem {
    inner: _CFileSystemImpl,
}

impl DMSFileSystem {
    pub fn _Fnew_with_root(project_root: PathBuf) -> Self {
        let inner = _CFileSystemImpl::_Fnew_with_root(project_root);
        DMSFileSystem { inner }
    }

    pub fn _Fnew_with_roots(project_root: PathBuf, app_data_root: PathBuf) -> Self {
        let inner = _CFileSystemImpl::_Fnew_with_roots(project_root, app_data_root);
        DMSFileSystem { inner }
    }

    pub fn _Fnew_auto_root() -> DMSResult<Self> {
        let cwd = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {}", e)))?;
        Ok(Self::_Fnew_with_root(cwd))
    }

    pub fn _Fproject_root(&self) -> &Path {
        self.inner._Fproject_root()
    }

    pub fn _Fsafe_mkdir<P: AsRef<Path>>(&self, path: P) -> DMSResult<PathBuf> {
        self.inner._Fsafe_mkdir(path.as_ref())
    }

    pub fn _Fensure_parent_dir<P: AsRef<Path>>(&self, path: P) -> DMSResult<PathBuf> {
        self.inner._Fensure_parent_dir(path.as_ref())
    }

    pub fn _Fatomic_write_text<P: AsRef<Path>>(&self, path: P, text: &str) -> DMSResult<()> {
        self.inner._Fatomic_write_text(path.as_ref(), text)
    }

    pub fn _Fatomic_write_bytes<P: AsRef<Path>>(&self, path: P, data: &[u8]) -> DMSResult<()> {
        self.inner._Fatomic_write_bytes(path.as_ref(), data)
    }

    pub fn _Fread_text<P: AsRef<Path>>(&self, path: P) -> DMSResult<String> {
        self.inner._Fread_text(path.as_ref())
    }

    pub fn _Fread_json<P: AsRef<Path>, T: DeserializeOwned>(&self, path: P) -> DMSResult<T> {
        let text = self._Fread_text(path)?;
        serde_json::from_str(&text)
            .map_err(|e| crate::core::DMSError::Other(format!("json read failed: {}", e)))
    }

    pub fn _Fexists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }

    pub fn _Fremove_file<P: AsRef<Path>>(&self, path: P) -> DMSResult<()> {
        let p = path.as_ref();
        match fs::remove_file(p) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(crate::core::DMSError::Other(format!("remove_file failed: {}", e))),
        }
    }

    pub fn _Fremove_dir_all<P: AsRef<Path>>(&self, path: P) -> DMSResult<()> {
        let p = path.as_ref();
        match fs::remove_dir_all(p) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(crate::core::DMSError::Other(format!("remove_dir_all failed: {}", e))),
        }
    }

    pub fn _Fcopy_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, from: P, to: Q) -> DMSResult<()> {
        let src = from.as_ref();
        let dst = to.as_ref();
        if let Some(parent) = dst.parent() {
            self._Fsafe_mkdir(parent)?;
        }
        fs::copy(src, dst)
            .map_err(|e| crate::core::DMSError::Other(format!("copy_file failed: {}", e)))?;
        Ok(())
    }

    pub fn _Fappend_text<P: AsRef<Path>>(&self, path: P, text: &str) -> DMSResult<()> {
        use std::io::Write as _;

        let path_ref = path.as_ref();
        self._Fensure_parent_dir(path_ref)?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_ref)
            .map_err(|e| crate::core::DMSError::Other(format!("append_text open failed: {}", e)))?;
        file.write_all(text.as_bytes())
            .map_err(|e| crate::core::DMSError::Other(format!("append_text write failed: {}", e)))?;
        file.flush()
            .map_err(|e| crate::core::DMSError::Other(format!("append_text flush failed: {}", e)))?;
        Ok(())
    }

    pub fn _Fwrite_json<P: AsRef<Path>, T: Serialize>(&self, path: P, value: &T) -> DMSResult<()> {
        let text = serde_json::to_string_pretty(value)
            .map_err(|e| crate::core::DMSError::Other(format!("json serialize failed: {}", e)))?;
        self._Fatomic_write_text(path, &text)
    }

    pub fn _Fapp_dir(&self) -> PathBuf {
        self.inner._Fapp_dir()
    }

    pub fn _Flogs_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("logs")
    }

    pub fn _Fcache_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("cache")
    }

    pub fn _Freports_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("reports")
    }

    pub fn _Fobservability_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("observability")
    }

    pub fn _Ftemp_dir(&self) -> PathBuf {
        self.inner._Fcategory_dir("tmp")
    }

    pub fn _Fensure_category_path<S: AsRef<str>, P: AsRef<Path>>(&self, category: S, path_or_name: P) -> PathBuf {
        let base = match category.as_ref() {
            "logs" => self._Flogs_dir(),
            "cache" => self._Fcache_dir(),
            "reports" => self._Freports_dir(),
            "observability" => self._Fobservability_dir(),
            "tmp" => self._Ftemp_dir(),
            _ => self._Fapp_dir(),
        };

        let target = base.join(path_or_name.as_ref());
        let _ = fs::create_dir_all(target.parent().unwrap_or(&base));
        target
    }

    pub fn _Fnormalize_under_category<S: AsRef<str>, P: AsRef<Path>>(&self, category: S, path_or_name: P) -> PathBuf {
        let name = path_or_name.as_ref().file_name().unwrap_or_else(|| std::ffi::OsStr::new(""));
        self._Fensure_category_path(category, PathBuf::from(name))
    }
}
