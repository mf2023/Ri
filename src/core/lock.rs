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

//! # Safe Lock Utilities
//!
//! This module provides safe abstractions for lock acquisition and management,
//! eliminating the need for `.unwrap()` and `.expect()` calls when working with
//! synchronization primitives. These utilities ensure robust error handling
//! in concurrent scenarios without risking thread panics.
//!
//! ## Key Components
//!
//! - **DMSCLockError**: Specialized error type for lock-related failures
//! - **DMSCLockResult**: Result type alias for lock operations
//! - **RwLockExtensions**: Extension traits for standard `RwLock` types
//! - **MutexExtensions**: Extension traits for standard `Mutex` types
//!
//! ## Design Principles
//!
//! 1. **Never Panic**: All lock operations return `Result` instead of panicking
//! 2. **Poison Error Handling**: Properly handles poisoned locks without panicking
//! 3. **Consistent API**: Uniform error handling across all lock types
//! 4. **Zero-Cost Abstraction**: Extension traits add no overhead when unused
//!
//! ## Usage
//!
//! ```rust
//! use std::sync::Arc;
//! use dmsc::core::lock::{RwLockExtensions, DMSCLockResult};
//!
//! struct SharedState {
//!     counter: u64,
//! }
//!
//! impl SharedState {
//!     fn increment(&mut self) {
//!         self.counter += 1;
//!     }
//!
//!     fn get_value(&self) -> u64 {
//!         self.counter
//!     }
//! }
//!
//! fn example() -> DMSCLockResult<()> {
//!     let state = Arc::new(RwLock::new(SharedState { counter: 0 }));
//!
//!     // Write lock with error handling
//!     {
//!         let mut guard = state.write_safe()?;
//!         guard.increment();
//!     }
//!
//!     // Read lock with error handling
//!     {
//!         let guard = state.read_safe()?;
//!         assert_eq!(guard.get_value(), 1);
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::sync::{RwLock, Mutex, PoisonError, RwLockReadGuard, RwLockWriteGuard, MutexGuard};
use std::fmt;

#[cfg(feature = "pyo3")]
use pyo3::pyclass;

/// Specialized error type for lock-related failures.
///
/// This error type provides detailed information about lock acquisition failures,
/// including whether the lock was poisoned or simply contested. The error includes
/// context about the lock's purpose for better debugging.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct DMSCLockError {
    context: String,
    is_poisoned: bool,
}

impl DMSCLockError {
    /// Creates a new lock error with the given context.
    ///
    /// # Arguments
    ///
    ///     context: Human-readable description of what was being locked
    ///
    /// # Returns
    ///
    ///     A new `DMSCLockError` instance
    pub fn new(context: &str) -> Self {
        Self {
            context: context.to_string(),
            is_poisoned: false,
        }
    }

    /// Creates a poisoned lock error.
    ///
    /// # Arguments
    ///
    ///     context: Human-readable description of what was being locked
    ///
    /// # Returns
    ///
    ///     A new `DMSCLockError` instance marked as poisoned
    pub fn poisoned(context: &str) -> Self {
        Self {
            context: context.to_string(),
            is_poisoned: true,
        }
    }

    /// Gets the context message for this lock error.
    pub fn get_context(&self) -> &str {
        &self.context
    }
}

impl fmt::Display for DMSCLockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_poisoned {
            write!(f, "Lock poisoned during acquisition: {}", self.context)
        } else {
            write!(f, "Lock acquisition failed: {}", self.context)
        }
    }
}

impl std::error::Error for DMSCLockError {}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCLockError {
    #[new]
    fn new_py(context: String, is_poisoned: bool) -> Self {
        Self {
            context,
            is_poisoned,
        }
    }

    #[staticmethod]
    fn create_from_context(context: String) -> Self {
        Self {
            context,
            is_poisoned: false,
        }
    }

    #[staticmethod]
    fn create_poisoned(context: String) -> Self {
        Self {
            context,
            is_poisoned: true,
        }
    }

    #[getter]
    fn is_poisoned(&self) -> bool {
        self.is_poisoned
    }

    #[getter]
    fn context(&self) -> String {
        self.context.clone()
    }

    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("DMSCLockError {{ context: {:?}, is_poisoned: {} }}", self.context, self.is_poisoned)
    }
}

/// Result type alias for lock operations.
///
/// This type alias simplifies error handling for lock-related operations,
/// providing a consistent return type for all lock acquisitions.
pub type DMSCLockResult<T> = Result<T, DMSCLockError>;

/// Extension trait providing safe read lock acquisition for `RwLock`.
///
/// This trait adds a `read_safe` method to `RwLock` that returns a `Result`
/// instead of panicking when the lock is poisoned or cannot be acquired.
pub trait RwLockExtensions<T: Send + Sync> {
    /// Acquires a read lock safely, returning a Result instead of panicking.
    ///
    /// This method attempts to acquire a read lock on the `RwLock`. If the lock
    /// is held by a writer or if a previous holder panicked (poisoned), this
    /// method returns an error instead of panicking.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(RwLockReadGuard<T>)` if the lock was acquired successfully
    /// - `Err(DMSCLockError)` if the lock could not be acquired
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use std::sync::RwLock;
    /// use dmsc::core::lock::RwLockExtensions;
    ///
    /// let lock = RwLock::new(42);
    ///
    /// match lock.read_safe("counter") {
    ///     Ok(guard) => println!("Value: {}", *guard),
    ///     Err(e) => println!("Failed to acquire lock: {}", e),
    /// }
    /// ```
    fn read_safe(&self, context: &str) -> DMSCLockResult<RwLockReadGuard<'_, T>>;
    
    /// Acquires a write lock safely, returning a Result instead of panicking.
    ///
    /// This method attempts to acquire a write lock on the `RwLock`. If the lock
    /// is currently held by any readers or if a previous holder panicked (poisoned),
    /// this method returns an error instead of panicking.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(RwLockWriteGuard<T>)` if the lock was acquired successfully
    /// - `Err(DMSCLockError)` if the lock could not be acquired
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use std::sync::RwLock;
    /// use dmsc::core::lock::RwLockExtensions;
    ///
    /// let lock = RwLock::new(42);
    ///
    /// match lock.write_safe("counter") {
    ///     Ok(mut guard) => {
    ///         *guard += 1;
    ///         println!("New value: {}", *guard);
    ///     }
    ///     Err(e) => println!("Failed to acquire lock: {}", e),
    /// }
    /// ```
    fn write_safe(&self, context: &str) -> DMSCLockResult<RwLockWriteGuard<'_, T>>;
    
    /// Attempts to acquire a read lock, returning immediately if unavailable.
    ///
    /// This method tries to acquire a read lock without blocking. If the lock
    /// is held by a writer, it returns an error immediately.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(RwLockReadGuard<T>))` if the lock was acquired
    /// - `Ok(None)` if the lock is held by a writer
    /// - `Err(DMSCLockError)` if the lock is poisoned
    fn try_read_safe(&self, context: &str) -> DMSCLockResult<Option<RwLockReadGuard<'_, T>>>;
    
    /// Attempts to acquire a write lock, returning immediately if unavailable.
    ///
    /// This method tries to acquire a write lock without blocking. If the lock
    /// is held by any readers, it returns an error immediately.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(RwLockWriteGuard<T>))` if the lock was acquired
    /// - `Ok(None)` if the lock is held by readers or a writer
    /// - `Err(DMSCLockError)` if the lock is poisoned
    fn try_write_safe(&self, context: &str) -> DMSCLockResult<Option<RwLockWriteGuard<'_, T>>>;
}

impl<T: Send + Sync> RwLockExtensions<T> for RwLock<T> {
    fn read_safe(&self, context: &str) -> DMSCLockResult<RwLockReadGuard<'_, T>> {
        RwLock::read(self).map_err(|_| {
            DMSCLockError::poisoned(context)
        })
    }
    
    fn write_safe(&self, context: &str) -> DMSCLockResult<RwLockWriteGuard<'_, T>> {
        RwLock::write(self).map_err(|_| {
            DMSCLockError::poisoned(context)
        })
    }
    
    fn try_read_safe(&self, context: &str) -> DMSCLockResult<Option<RwLockReadGuard<'_, T>>> {
        match RwLock::try_read(self) {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(DMSCLockError::poisoned(context))
            }
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
        }
    }
    
    fn try_write_safe(&self, context: &str) -> DMSCLockResult<Option<RwLockWriteGuard<'_, T>>> {
        match RwLock::try_write(self) {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(DMSCLockError::poisoned(context))
            }
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
        }
    }
}

/// Extension trait providing safe lock acquisition for `Mutex`.
///
/// This trait adds a `lock_safe` method to `Mutex` that returns a `Result`
/// instead of panicking when the lock is poisoned.
pub trait MutexExtensions<T: Send> {
    /// Acquires a lock safely, returning a Result instead of panicking.
    ///
    /// This method attempts to acquire the mutex lock. If a previous holder
    /// panicked while holding the lock (poisoned), this method returns an
    /// error instead of panicking.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(MutexGuard<T>)` if the lock was acquired successfully
    /// - `Err(DMSCLockError)` if the lock could not be acquired
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use std::sync::Mutex;
    /// use dmsc::core::lock::MutexExtensions;
    ///
    /// let mutex = Mutex::new(42);
    ///
    /// match mutex.lock_safe("important_data") {
    ///     Ok(guard) => println!("Value: {}", *guard),
    ///     Err(e) => println!("Failed to acquire lock: {}", e),
    /// }
    /// ```
    fn lock_safe(&self, context: &str) -> DMSCLockResult<MutexGuard<'_, T>>;
    
    /// Attempts to acquire the lock, returning immediately if unavailable.
    ///
    /// This method tries to acquire the mutex lock without blocking. If the
    /// lock is held by another thread, it returns an error immediately.
    ///
    /// ## Arguments
    ///
    /// - `context`: A description of what is being locked for error messages
    ///
    /// ## Returns
    ///
    /// - `Ok(Some(MutexGuard<T>))` if the lock was acquired
    /// - `Ok(None)` if the lock is held by another thread
    /// - `Err(DMSCLockError)` if the lock is poisoned
    fn try_lock_safe(&self, context: &str) -> DMSCLockResult<Option<MutexGuard<'_, T>>>;
}

impl<T: Send> MutexExtensions<T> for Mutex<T> {
    fn lock_safe(&self, context: &str) -> DMSCLockResult<MutexGuard<'_, T>> {
        Mutex::lock(self).map_err(|_| {
            DMSCLockError::poisoned(context)
        })
    }
    
    fn try_lock_safe(&self, context: &str) -> DMSCLockResult<Option<MutexGuard<'_, T>>> {
        match Mutex::try_lock(self) {
            Ok(guard) => Ok(Some(guard)),
            Err(std::sync::TryLockError::Poisoned(_)) => {
                Err(DMSCLockError::poisoned(context))
            }
            Err(std::sync::TryLockError::WouldBlock) => Ok(None),
        }
    }
}

/// Utility function to convert from `PoisonError` to `DMSCLockError`.
///
/// This conversion is useful when working with standard library types that
/// return `PoisonError` and you want to convert to our custom lock error type.
///
/// ## Arguments
///
/// - `error`: The poison error to convert
/// - `context`: Description of what was being locked
///
/// ## Returns
///
/// A `DMSCLockError` with the appropriate context and poisoned flag
pub fn from_poison_error<T>(_error: PoisonError<T>, context: &str) -> DMSCLockError {
    DMSCLockError::poisoned(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_read_safe_success() {
        let lock = RwLock::new(42);
        let result = lock.read_safe("test counter");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_write_safe_success() {
        let lock = RwLock::new(42);
        let result = lock.write_safe("test counter");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_try_read_safe_available() {
        let lock = RwLock::new(42);
        let result = lock.try_read_safe("test counter");
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn test_try_write_safe_unavailable() {
        let lock = RwLock::new(42);
        let _write_guard = lock.write_safe("test counter").unwrap();
        
        let result = lock.try_write_safe("test counter");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_mutex_lock_safe() {
        let mutex = Mutex::new(42);
        let result = mutex.lock_safe("test mutex");
        assert!(result.is_ok());
        assert_eq!(*result.unwrap(), 42);
    }

    #[test]
    fn test_lock_error_display() {
        let error = DMSCLockError::new("test context");
        assert_eq!(error.to_string(), "Lock acquisition failed: test context");
        
        let poisoned = DMSCLockError::poisoned("poisoned lock");
        assert_eq!(poisoned.to_string(), "Lock poisoned during acquisition: poisoned lock");
        assert!(poisoned.is_poisoned());
    }

    #[test]
    fn test_concurrent_reads() {
        let lock = Arc::new(RwLock::new(0));
        let num_threads = 10;
        let iterations = 1000;
        
        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let lock = Arc::clone(&lock);
                thread::spawn(move || {
                    for _ in 0..iterations {
                        let _guard = lock.read_safe("concurrent read").unwrap();
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*lock.read_safe("final read").unwrap(), 0);
    }

    #[test]
    fn test_concurrent_writes() {
        let lock = Arc::new(RwLock::new(AtomicU64::new(0)));
        let num_threads = 4;
        let iterations = 100;
        
        let handles: Vec<_> = (0..num_threads)
            .map(|_i| {
                let lock = Arc::clone(&lock);
                thread::spawn(move || {
                    for _ in 0..iterations {
                        let guard = lock.write_safe("concurrent write").unwrap();
                        guard.fetch_add(1, Ordering::SeqCst);
                    }
                })
            })
            .collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        let final_value = lock.read_safe("final value").unwrap().load(Ordering::SeqCst);
        assert_eq!(final_value, (num_threads * iterations) as u64);
    }
}
