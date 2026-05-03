//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

//! # Java JNI Bindings Module
//!
//! This module provides Java JNI bindings for Ri, enabling Java applications
//! to use Ri functionality through native method calls.
//!
//! ## Key Components
//!
//! - **jvm**: JVM lifecycle management
//! - **converter**: Rust-Java type conversion utilities
//! - **exception**: Java exception handling
//! - **classes**: JNI bindings for all Ri classes
//!
//! ## Design Principles
//!
//! 1. **API Consistency**: Java API matches Rust API exactly
//! 2. **Memory Safety**: Proper handling of native pointers
//! 3. **Error Handling**: Rust errors converted to Java exceptions
//! 4. **Thread Safety**: Safe for concurrent access from Java

use std::collections::HashSet;
use std::sync::OnceLock;
use std::sync::Mutex;

pub mod jvm;
pub mod converter;
pub mod exception;
pub mod classes;

pub use jvm::RiJavaContext;
pub use converter::{JavaConvertible, ToJava, FromJava};
pub use exception::{RiJavaException, throw_exception};

/// Global pointer registry for JNI bindings
static JNI_POINTER_REGISTRY: OnceLock<Arc<Mutex<HashSet<usize>>>> = OnceLock::new();

/// Get the pointer registry
fn get_jni_pointer_registry() -> &'static Arc<Mutex<HashSet<usize>>> {
    JNI_POINTER_REGISTRY.get_or_init(|| Arc::new(Mutex::new(HashSet::new())))
}

/// Register a pointer in the registry
pub fn register_jni_ptr(ptr: usize) {
    let registry = get_jni_pointer_registry();
    let mut set = registry.lock().unwrap();
    set.insert(ptr);
}

/// Unregister a pointer from the registry
pub fn unregister_jni_ptr(ptr: usize) {
    let registry = get_jni_pointer_registry();
    let mut set = registry.lock().unwrap();
    set.remove(&ptr);
}

/// Check if a pointer is valid (registered and not null)
pub fn is_jni_ptr_valid(ptr: usize) -> bool {
    if ptr == 0 {
        return false;
    }
    let registry = get_jni_pointer_registry();
    let set = registry.lock().unwrap();
    set.contains(&ptr)
}

use std::sync::Arc;
