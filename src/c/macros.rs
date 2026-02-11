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

//! # C API Macros
//!
//! This module provides macros for generating C API boilerplate code.

/// Macro to generate C wrapper struct for a Rust type
#[macro_export]
macro_rules! c_wrapper {
    ($c_name:ident, $rust_type:ty) => {
        #[repr(C)]
        pub struct $c_name {
            inner: $rust_type,
        }
        
        impl $c_name {
            pub fn new(inner: $rust_type) -> Self {
                Self { inner }
            }
            
            pub fn inner(&self) -> &$rust_type {
                &self.inner
            }
            
            pub fn inner_mut(&mut self) -> &mut $rust_type {
                &mut self.inner
            }
        }
    };
}

/// Macro to generate C constructor function
#[macro_export]
macro_rules! c_constructor {
    ($fn_name:ident, $c_type:ty, $rust_type:ty, $new_expr:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name() -> *mut $c_type {
            let obj = $new_expr;
            Box::into_raw(Box::new(<$c_type>::new(obj)))
        }
    };
}

/// Macro to generate C destructor function
#[macro_export]
macro_rules! c_destructor {
    ($fn_name:ident, $c_type:ty) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(obj: *mut $c_type) {
            if !obj.is_null() {
                unsafe {
                    let _ = Box::from_raw(obj);
                }
            }
        }
    };
}

/// Macro to generate C getter for string
#[macro_export]
macro_rules! c_string_getter {
    ($fn_name:ident, $c_type:ty, $getter:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(
            obj: *mut $c_type,
        ) -> *mut std::ffi::c_char {
            if obj.is_null() {
                return std::ptr::null_mut();
            }
            unsafe {
                let val = $getter(&(*obj).inner);
                match std::ffi::CString::new(val) {
                    Ok(c_str) => c_str.into_raw(),
                    Err(_) => std::ptr::null_mut(),
                }
            }
        }
    };
}

/// Macro to generate C setter for string
#[macro_export]
macro_rules! c_string_setter {
    ($fn_name:ident, $c_type:ty, $setter:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(
            obj: *mut $c_type,
            value: *const std::ffi::c_char,
        ) -> std::ffi::c_int {
            if obj.is_null() || value.is_null() {
                return -1;
            }
            unsafe {
                let val_str = match std::ffi::CStr::from_ptr(value).to_str() {
                    Ok(s) => s,
                    Err(_) => return -1,
                };
                $setter(&mut (*obj).inner, val_str);
                0
            }
        }
    };
}
