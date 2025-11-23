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

// Logging module for DMS.
// This is a first-stage implementation using std only; can be extended later.

use std::fmt::Debug;

use crate::core::DMSResult;
use crate::fs::DMSFileSystem;
use rand;
use serde_json::json;
use std::fs as stdfs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
mod context;
pub use context::DMSLogContext;

// Log level definition.
#[derive(Clone, Copy, Debug)]
pub enum DMSLogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl DMSLogLevel {
    pub fn _Fas_str(&self) -> &'static str {
        match self {
            DMSLogLevel::Debug => "DEBUG",
            DMSLogLevel::Info => "INFO",
            DMSLogLevel::Warn => "WARN",
            DMSLogLevel::Error => "ERROR",
        }
    }
}

// Public logging configuration class.
pub struct DMSLogConfig {
    pub level: DMSLogLevel,
    pub console_enabled: bool,
    pub file_enabled: bool,
    pub sampling_default: f32,
    pub file_name: String,
    pub json_format: bool,
    pub rotate_when: String,   // e.g. "size" | "none" (currently only size supported)
    pub max_bytes: u64,        // used when rotate_when == "size"
}

impl DMSLogConfig {
    pub fn _Fdefault() -> Self {
        DMSLogConfig {
            level: DMSLogLevel::Info,
            console_enabled: true,
            file_enabled: true,
            sampling_default: 1.0,
            file_name: "dms.log".to_string(),
            json_format: false,
            rotate_when: "size".to_string(),
            max_bytes: 10 * 1024 * 1024,
        }
    }

    pub fn _Ffrom_config(config: &crate::config::DMSConfig) -> Self {
        let mut base = DMSLogConfig::_Fdefault();

        if let Some(level_str) = config._Fget_str("log.level") {
            base.level = match level_str {
                "DEBUG" | "debug" => DMSLogLevel::Debug,
                "INFO" | "info" => DMSLogLevel::Info,
                "WARN" | "warn" | "WARNING" | "warning" => DMSLogLevel::Warn,
                "ERROR" | "error" => DMSLogLevel::Error,
                _ => base.level,
            };
        }

        if let Some(v) = config._Fget_f32("log.sampling_default") {
            base.sampling_default = v.clamp(0.0, 1.0);
        }

        if let Some(file_name) = config._Fget_str("log.file_name") {
            if !file_name.is_empty() {
                base.file_name = file_name.to_string();
            }
        }

        if let Some(fmt) = config._Fget_str("log.file_format") {
            // Accept "json"/"JSON" to enable JSON file output, others default to text
            if fmt.eq_ignore_ascii_case("json") {
                base.json_format = true;
            }
        }

        if let Some(rotate) = config._Fget_str("log.rotate_when") {
            if !rotate.is_empty() {
                base.rotate_when = rotate.to_string();
            }
        }

        if let Some(v) = config._Fget_u64("log.max_bytes") {
            if v > 0 {
                base.max_bytes = v;
            }
        }

        if let Some(v) = config._Fget_bool("log.console_enabled") {
            base.console_enabled = v;
        }

        if let Some(v) = config._Fget_bool("log.file_enabled") {
            base.file_enabled = v;
        }

        base
    }
}

// Internal logger implementation.
struct _CLoggerImpl {
    level: DMSLogLevel,
    fs: DMSFileSystem,
    sampling_default: f32,
    console_enabled: bool,
    file_enabled: bool,
    file_name: String,
    json_format: bool,
    rotate_when: String,
    max_bytes: u64,
}

impl _CLoggerImpl {
    fn _Fnew(config: &DMSLogConfig, fs: DMSFileSystem) -> Self {
        _CLoggerImpl {
            level: config.level,
            fs,
            sampling_default: config.sampling_default,
            console_enabled: config.console_enabled,
            file_enabled: config.file_enabled,
            file_name: config.file_name.clone(),
            json_format: config.json_format,
            rotate_when: config.rotate_when.clone(),
            max_bytes: config.max_bytes,
        }
    }

    fn _Fshould_log(&self, level: DMSLogLevel) -> bool {
        (level as u8) >= (self.level as u8)
    }

    fn _Fshould_log_event(&self, _event: &str) -> bool {
        // Placeholder: simple sampling based on sampling_default; can be extended per-event.
        if self.sampling_default >= 1.0 {
            true
        } else if self.sampling_default <= 0.0 {
            false
        } else {
            let r = rand::random::<f32>();
            r < self.sampling_default
        }
    }

    fn _Fnow_timestamp() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                let secs = dur.as_secs();
                let millis = dur.subsec_millis();
                format!("{}.{:03}Z", secs, millis)
            }
            Err(_) => "0.000Z".to_string(),
        }
    }

    fn _Flog_message<T: Debug>(&self, level: DMSLogLevel, target: &str, message: T) -> DMSResult<()> {
        if !self._Fshould_log(level) {
            return Ok(());
        }

        let event = target; // simple default; can be extended to accept explicit event names.
        if !self._Fshould_log_event(event) {
            return Ok(());
        }

        let ts = Self::_Fnow_timestamp();
        let ctx_kv = DMSLogContext::_Fget_all();

        if self.console_enabled {
            let line = format!(
                "{} [{}] {} event={} - {:?}{}",
                ts,
                level._Fas_str(),
                target,
                event,
                message,
                if ctx_kv.is_empty() {
                    String::new()
                } else {
                    let parts: Vec<String> = ctx_kv
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    format!(" ctx={{ {} }}", parts.join(", "))
                },
            );
            println!("{}", line);
        }

        if self.file_enabled {
            let log_file = self.fs._Flogs_dir().join(&self.file_name);

            // Simple size-based rotation if enabled
            if self.rotate_when.eq_ignore_ascii_case("size") && self.max_bytes > 0 {
                if let Ok(meta) = stdfs::metadata(&log_file) {
                    if meta.len() >= self.max_bytes {
                        if let Some(parent) = log_file.parent() {
                            let base = log_file.file_name().and_then(|s| s.to_str()).unwrap_or("dms.log");
                            let ts = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {}", e)))?;
                            let rotated = parent.join(format!("{}.{}", base, ts.as_millis()));
                            let _ = stdfs::rename(&log_file, &rotated);
                        }
                    }
                }
            }

            let line = if self.json_format {
                let mut obj = json!({
                    "timestamp": ts,
                    "level": level._Fas_str(),
                    "target": target,
                    "event": event,
                    "message": format!("{:?}", message),
                });

                if !ctx_kv.is_empty() {
                    if let Some(map) = obj.as_object_mut() {
                        map.insert("ctx".to_string(), json!(ctx_kv));
                    }
                }

                obj.to_string()
            } else {
                let ctx_str = if ctx_kv.is_empty() {
                    String::new()
                } else {
                    let parts: Vec<String> = ctx_kv
                        .iter()
                        .map(|(k, v)| format!("{}={}", k, v))
                        .collect();
                    format!(" ctx={{ {} }}", parts.join(", "))
                };

                format!(
                    "{} [{}] {} event={} - {:?}{}",
                    ts,
                    level._Fas_str(),
                    target,
                    event,
                    message,
                    ctx_str,
                )
            };

            self.fs._Fappend_text(&log_file, &format!("{}\n", line))?;
        }

        Ok(())
    }
}

// Public-facing logger class.
pub struct DMSLogger {
    inner: _CLoggerImpl,
}

impl DMSLogger {
    pub fn _Fnew(config: &DMSLogConfig, fs: DMSFileSystem) -> Self {
        let inner = _CLoggerImpl::_Fnew(config, fs);
        DMSLogger { inner }
    }

    pub fn _Fdebug<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Debug, target, message)
    }

    pub fn _Finfo<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Info, target, message)
    }

    pub fn _Fwarn<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Warn, target, message)
    }

    pub fn _Ferror<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Error, target, message)
    }
}
