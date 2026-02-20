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

//! # Java JNI Bindings Module
//!
//! This module provides Java JNI bindings for DMSC, enabling Java applications
//! to use DMSC functionality through native method calls.
//!
//! ## Key Components
//!
//! - **jvm**: JVM lifecycle management
//! - **converter**: Rust-Java type conversion utilities
//! - **exception**: Java exception handling
//! - **classes**: JNI bindings for all DMSC classes
//!
//! ## Design Principles
//!
//! 1. **API Consistency**: Java API matches Rust API exactly
//! 2. **Memory Safety**: Proper handling of native pointers
//! 3. **Error Handling**: Rust errors converted to Java exceptions
//! 4. **Thread Safety**: Safe for concurrent access from Java

pub mod jvm;
pub mod converter;
pub mod exception;
pub mod classes;

pub use jvm::DMSCJavaContext;
pub use converter::{JavaConvertible, ToJava, FromJava};
pub use exception::{DMSCJavaException, throw_exception};
