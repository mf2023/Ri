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

//! # C/C++ API Module
//!
//! This module provides C/C++ bindings for DMSC.

use std::ffi::{c_char, c_int, CString};

#[macro_use]
pub mod macros;
pub mod core;
pub mod auth;
pub mod cache;
pub mod database;
pub mod device;
pub mod fs;
pub mod gateway;
pub mod grpc;
pub mod hooks;
pub mod log;
pub mod module_rpc;
pub mod observability;
pub mod protocol;
pub mod queue;
pub mod service_mesh;
pub mod validation;
pub mod ws;

/// Initialize the DMSC library
#[no_mangle]
pub extern "C" fn dmsc_init() -> c_int {
    0
}

/// Cleanup the DMSC library
#[no_mangle]
pub extern "C" fn dmsc_cleanup() {
}

/// Get DMSC version
#[no_mangle]
pub extern "C" fn dmsc_version() -> *mut c_char {
    let c_str = CString::new(env!("CARGO_PKG_VERSION")).unwrap();
    c_str.into_raw()
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
