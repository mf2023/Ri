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

//! # JVM Lifecycle Management
//!
//! Provides utilities for managing the Java Virtual Machine lifecycle
//! and obtaining JNI environment pointers.

use jni::JNIEnv;
use jni::JavaVM;
use jni::AttachGuard;
use std::sync::{Arc, OnceLock};

static JVM_INSTANCE: OnceLock<Arc<JavaVM>> = OnceLock::new();

/// Ri Java context for managing JVM interactions
pub struct RiJavaContext {
    jvm: Arc<JavaVM>,
}

impl RiJavaContext {
    /// Initialize the Java context from an existing JNIEnv
    pub fn init(env: JNIEnv) -> Self {
        let jvm = env.get_java_vm().expect("Failed to get JavaVM");
        let jvm_arc = Arc::new(jvm);
        let _ = JVM_INSTANCE.set(jvm_arc.clone());
        Self { jvm: jvm_arc }
    }

    /// Get the current JNIEnv
    pub fn get_env(&self) -> AttachGuard<'_> {
        self.jvm
            .attach_current_thread()
            .expect("Failed to attach current thread to JVM")
    }

    /// Get the global JVM instance
    pub fn get_jvm() -> Option<Arc<JavaVM>> {
        JVM_INSTANCE.get().cloned()
    }
}

/// Get the current JNIEnv from the global JVM instance
pub fn get_env() -> Option<AttachGuard<'static>> {
    JVM_INSTANCE.get().and_then(|jvm| {
        jvm.attach_current_thread().ok()
    })
}
