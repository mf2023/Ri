//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # JNI Runtime Helpers
//!
//! A shared Tokio runtime for JNI entry points, plus panic-safe wrappers
//! that convert Rust panics into Java exceptions instead of aborting the JVM.

use std::future::Future;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::OnceLock;
use futures::executor::block_on;
use jni::sys::jlong;
use jni::JNIEnv;
use tokio::runtime::Runtime;
use crate::java::exception::throw_ri_error;

static JNI_RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get (or lazily create) the process-wide JNI Tokio runtime.
pub fn jni_runtime() -> &'static Runtime {
    JNI_RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Failed to build JNI Tokio runtime")
    })
}

/// Convert a panic payload into a human-readable message.
fn payload_to_string(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "Ri JNI: unknown panic payload".to_string()
    }
}

/// Block on a future from a JNI entry point, panic-safe.
///
/// - If a Tokio runtime is already active on this thread, `fut` is polled to
///   completion inside that runtime (spawn + block_on the JoinHandle).
/// - Otherwise the future runs on the process-wide JNI runtime.
/// - Any panic inside `fut` is caught and re-thrown to Java as a
///   `com.dunimd.ri.RiError` instead of aborting the JVM.
///
/// Returns `None` when a panic was caught (a Java exception is pending).
pub fn block_on_jni<T: Send + 'static>(
    env: &mut JNIEnv,
    _context: &str,
    fut: impl Future<Output = T> + Send + 'static,
) -> Option<T> {
    let result = catch_unwind(AssertUnwindSafe(|| {
        match tokio::runtime::Handle::try_current() {
            Ok(handle) => {
                // Already inside a Tokio runtime thread: use the current one.
                let join = handle.spawn(fut);
                match block_on(join) {
                    Ok(res) => res,
                    Err(join_err) => {
                        // The spawned task panicked; forward the panic.
                        std::panic::resume_unwind(join_err.into_panic())
                    }
                }
            }
            Err(_) => jni_runtime().block_on(fut),
        }
    }));

    match result {
        Ok(value) => Some(value),
        Err(payload) => {
            throw_ri_error(env, &format!("Ri native panic: {}", payload_to_string(payload)));
            None
        }
    }
}

/// Convert a Java TTL (jlong) into an optional TTL in seconds.
/// Negative values mean "no TTL" per the Java binding contract.
pub fn ttl_from_jlong(ttl_secs: jlong) -> Option<u64> {
    if ttl_secs >= 0 {
        Some(ttl_secs as u64)
    } else {
        None
    }
}

/// Block on a future that borrows local (non-'static) data, panic-safe.
///
/// Unlike [`block_on_jni`], the future may borrow from the caller's stack;
/// it runs on the shared JNI runtime via `Handle::block_on` semantics.
/// Must NOT be called from within an existing Tokio runtime thread
/// (that would deadlock); in that case it falls back to a fresh
/// single-threaded runtime.
pub fn block_on_local<T>(
    env: &mut JNIEnv,
    context: &str,
    fut: impl Future<Output = T>,
) -> Option<T> {
    let result = catch_unwind(AssertUnwindSafe(|| {
        if tokio::runtime::Handle::try_current().is_ok() {
            // Calling Handle::block_on from inside a runtime would deadlock;
            // run the future on a throwaway single-thread runtime instead.
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Failed to build fallback JNI runtime")
                .block_on(fut)
        } else {
            jni_runtime().block_on(fut)
        }
    }));

    match result {
        Ok(value) => Some(value),
        Err(payload) => {
            throw_ri_error(env, &format!("Ri native panic ({context}): {}", payload_to_string(payload)));
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jni_runtime_is_reusable() {
        let rt = jni_runtime();
        let rt2 = jni_runtime();
        assert!(std::ptr::eq(rt, rt2));
    }

    #[test]
    fn block_on_jni_runs_futures_off_thread() {
        let rt = jni_runtime();
        let val = rt.block_on(async { 41 + 1 });
        assert_eq!(val, 42);
    }

    #[test]
    fn ttl_from_jlong_maps_negative_to_none() {
        assert_eq!(ttl_from_jlong(-1), None);
        assert_eq!(ttl_from_jlong(0), Some(0));
        assert_eq!(ttl_from_jlong(3600), Some(3600));
    }
}
