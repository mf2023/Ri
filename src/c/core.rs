//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Core Module C API

use crate::prelude::{DMSCAppBuilder, DMSCConfig};
use std::ffi::{c_char, c_int, CString};

/// Opaque wrapper for DMSCAppBuilder
#[repr(C)]
pub struct CDMSCAppBuilder {
    inner: DMSCAppBuilder,
}

/// Opaque wrapper for DMSCConfig
#[repr(C)]
pub struct CDMSCConfig {
    inner: DMSCConfig,
}

/// Create a new application builder
#[no_mangle]
pub extern "C" fn dmsc_app_builder_new() -> *mut CDMSCAppBuilder {
    let builder = CDMSCAppBuilder {
        inner: DMSCAppBuilder::new(),
    };
    Box::into_raw(Box::new(builder))
}

/// Free an application builder
#[no_mangle]
pub extern "C" fn dmsc_app_builder_free(builder: *mut CDMSCAppBuilder) {
    if !builder.is_null() {
        unsafe {
            let _ = Box::from_raw(builder);
        }
    }
}

/// Create a new configuration
#[no_mangle]
pub extern "C" fn dmsc_config_new() -> *mut CDMSCConfig {
    let config = CDMSCConfig {
        inner: DMSCConfig::new(),
    };
    Box::into_raw(Box::new(config))
}

/// Free a configuration
#[no_mangle]
pub extern "C" fn dmsc_config_free(config: *mut CDMSCConfig) {
    if !config.is_null() {
        unsafe {
            let _ = Box::from_raw(config);
        }
    }
}

/// Get a string value from configuration
#[no_mangle]
pub extern "C" fn dmsc_config_get_string(
    config: *mut CDMSCConfig,
    key: *const c_char,
) -> *mut c_char {
    if config.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }
    
    unsafe {
        let c = &(*config).inner;
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        
        match c.get_str(key_str) {
            Some(val) => {
                match CString::new(val) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
            None => std::ptr::null_mut(),
        }
    }
}

/// Set a string value in configuration
#[no_mangle]
pub extern "C" fn dmsc_config_set_string(
    config: *mut CDMSCConfig,
    key: *const c_char,
    value: *const c_char,
) -> c_int {
    if config.is_null() || key.is_null() || value.is_null() {
        return -1;
    }
    
    unsafe {
        let c = &mut (*config).inner;
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };
        let value_str = match std::ffi::CStr::from_ptr(value).to_str() {
            Ok(s) => s,
            Err(_) => return -1,
        };
        
        c.set(key_str, value_str);
        0
    }
}

/// Free a string returned by DMSC
#[no_mangle]
pub extern "C" fn dmsc_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
