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

//! # JVM Lifecycle Management
//!
//! Provides utilities for managing the Java Virtual Machine lifecycle
//! and obtaining JNI environment pointers.

use jni::JavaVM;
use jni::JNIEnv;
use std::sync::OnceLock;

static JVM_INSTANCE: OnceLock<JavaVM> = OnceLock::new();

/// DMSC Java context for managing JVM interactions
pub struct DMSCJavaContext {
    jvm: JavaVM,
}

impl DMSCJavaContext {
    /// Initialize the Java context from an existing JNIEnv
    pub fn init(env: JNIEnv) -> Self {
        let jvm = env.get_java_vm().expect("Failed to get JavaVM");
        let ctx = Self { jvm };
        JVM_INSTANCE.get_or_init(|| jvm);
        ctx
    }

    /// Get the current JNIEnv
    pub fn get_env(&self) -> JNIEnv {
        self.jvm
            .attach_current_thread()
            .expect("Failed to attach current thread to JVM")
    }

    /// Get the global JVM instance
    pub fn get_jvm() -> Option<&'static JavaVM> {
        JVM_INSTANCE.get()
    }
}

/// Get the current JNIEnv from the global JVM instance
pub fn get_env() -> Option<JNIEnv> {
    JVM_INSTANCE.get().map(|jvm| {
        jvm.attach_current_thread()
            .expect("Failed to attach current thread to JVM")
    })
}
