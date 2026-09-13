// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Global Tokio runtime shared by language bindings and sync-async bridges.
//!
//! Python extension modules are loaded into whatever interpreter thread the
//! host application happens to be on; there is no guarantee that a Tokio
//! reactor is running (and `Handle::current()` panics when there is none).
//! The same holds for synchronous constructors and `Drop` implementations
//! in core code that need to drive async Ri APIs. This module provides a
//! lazily-initialized, process-wide multi-thread runtime for those cases.

use std::sync::OnceLock;
use tokio::runtime::Runtime;

static PY_RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Returns the process-wide Tokio runtime used by PyO3 bindings.
///
/// The runtime is created on first use and shared for the lifetime of the
/// process. Runtime creation failure is extremely unlikely (it only happens
/// when the OS refuses to spawn worker threads); in that case we panic since
/// there is no meaningful way to continue.
pub fn py_runtime() -> &'static Runtime {
    PY_RUNTIME.get_or_init(|| {
        Runtime::new().expect("Ri: failed to create global pyo3 tokio runtime")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn py_runtime_is_shared() {
        let a = py_runtime() as *const Runtime;
        let b = py_runtime() as *const Runtime;
        assert_eq!(a, b);
    }

    #[test]
    fn py_runtime_can_block_on() {
        let value = py_runtime().block_on(async { 41 + 1 });
        assert_eq!(value, 42);
    }
}
