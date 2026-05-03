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

//! # C API Macros
//!
//! This module provides procedural macros for generating C FFI (Foreign Function Interface)
//! boilerplate code. These macros simplify the creation of C-compatible interfaces for Rust
//! types, enabling seamless interoperability between Rust implementations and C/C++ consumers.
//!
//! The macro system addresses the fundamental impedance mismatch between Rust's ownership-based
//! memory model and C's manual memory management paradigm. By generating appropriate wrapper
//! structures, constructor functions, destructor functions, and property accessors, these
//! macros reduce boilerplate while maintaining safety and correctness guarantees.
//!
//! ## Design Philosophy
//!
//! The macro system follows several key design principles:
//!
//! - **Opaque Pointers**: All wrapped types use opaque pointer patterns where C code cannot
//!   access internal fields directly, preventing invalid memory access and ensuring encapsulation.
//!
//! - **Memory Safety**: Despite operating across language boundaries, the generated code
//!   maintains Rust's memory safety guarantees through careful ownership management and
//!   destruction semantics.
//!
//! - **Error Handling**: The generated functions return integer status codes (0 for success,
//!   negative values for errors) following C conventions for error reporting.
//!
//! - **Null Safety**: All generated functions handle NULL pointers gracefully, returning
//!   error codes or NULL outputs as appropriate rather than causing undefined behavior.
//!
//! - **Resource Cleanup**: Automatic resource cleanup through destructor functions prevents
//!   memory leaks when C code properly releases allocated objects.
//!
//! ## Available Macros
//!
//! This module provides five primary macros:
//!
//! 1. **c_wrapper!**: Generates a C-compatible wrapper structure for Rust types
//! 2. **c_constructor!**: Generates constructor functions for creating wrapped instances
//! 3. **c_destructor!**: Generates destructor functions for cleaning up wrapped instances
//! 4. **c_string_getter!**: Generates getter functions for string properties
//! 5. **c_string_setter!**: Generates setter functions for string properties
//!
//! ## Memory Management Model
//!
//! The generated code follows a consistent memory management pattern:
//!
//! - **Allocation**: Constructor functions allocate objects on the heap using Box::into_raw
//! - **Access**: C code receives raw pointers to the allocated objects
//! - **Deallocation**: Destructor functions free heap allocations using Box::from_raw
//!
//! C code using these generated APIs must:
//!
//! 1. Call constructor functions to create instances
//! 2. Pass returned pointers to all subsequent API calls
//! 3. Call destructor functions before pointers go out of scope
//!
//! ## Thread Safety Considerations
//!
//! The generated wrapper structures themselves do not provide thread synchronization.
//! Thread safety depends on the underlying Rust type implementations:
//!
//! - Types implementing Send + Sync can be safely shared across threads
//! - Types without these traits may require additional synchronization in C code
//! - Concurrent access to wrapped objects should be coordinated by the C caller
//!
//! ## Usage Pattern
//!
//! The typical usage pattern for wrapping a Rust type involves four steps:
//!
//! 1. Define the wrapper structure using c_wrapper!
//!
//! 2. Implement the Rust type with appropriate FFI-safe methods
//!
//! 3. Generate constructor using c_constructor!
//!
//! 4. Generate destructor using c_destructor!
//!
//! ## Example: Wrapping a Custom Type
//!
//! ```rust,ignore
//! use crate::c::macros::{c_wrapper, c_constructor, c_destructor};
//!
//! // Step 1: Define the Rust type
//! pub struct MyResource {
//!     handle: i32,
//!     name: String,
//! }
//!
//! // Step 2: Generate C wrapper
//! c_wrapper!(CMyResource, MyResource);
//!
//! // Step 3: Generate constructor (implemented separately)
//! #[no_mangle]
//! pub extern "C" fn my_resource_new() -> *mut CMyResource {
//!     let resource = MyResource {
//!         handle: 0,
//!         name: String::new(),
//!     };
//!     Box::into_raw(Box::new(CMyResource::new(resource)))
//! }
//!
//! // Step 4: Generate destructor
//! c_destructor!(my_resource_free, CMyResource);
//!
//! // C code can now use:
//! // - my_resource_create() to allocate
//! // - my_resource_*() functions to operate
//! // - my_resource_free() to deallocate
//! ```
//!
//! ## Performance Characteristics
//!
//! The generated code has minimal performance overhead:
//!
//! - Wrapper structures are zero-cost abstractions (single pointer indirection)
//! - Constructor/destructor calls are single heap allocations
//! - Getter/setter operations are direct method calls on wrapped types
//!
//! The performance considerations primary are:
//!
//! - Heap allocation for object creation (amortized O(1))
//! - Reference counting overhead for Rc/Arc types
//! - Synchronization costs for thread-safe types
//!
//! ## Limitations and Constraints
//!
//! These macros have certain limitations:
//!
//! - Cannot generate wrappers for generic types directly
//! - String getters assume UTF-8 encoding
//! - Error handling is limited to integer return codes
//! - No support for complex return types (use output parameters)
//!
//! For complex types requiring multiple return values, use output parameters:
//!
//! ```rust,ignore
//! // Instead of returning complex types:
//! // fn get_result() -> Result<ComplexType, Error>
//!
//! // Use output parameters:
//! fn get_result(output: &mut ComplexType) -> c_int {
//!     // fill in output parameter
//!     0 // success
//! }
//! ```
//!
//! ## Dependencies
//!
//! This module is self-contained within the Ri C API layer:
//!
//! - No external crate dependencies
//! - Uses only standard library types (Box, std::ffi)
//! - Compatible with no_std environments with allocator
//!
//! ## Feature Flags
//!
//! This module has no feature flags as it provides core FFI infrastructure
//! that is always required for C API generation.

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

/// Macro to generate C constructor function with pointer registry support
#[macro_export]
macro_rules! c_constructor {
    ($fn_name:ident, $c_type:ty, $rust_type:ty, $new_expr:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name() -> *mut $c_type {
            let obj = $new_expr;
            let ptr = Box::into_raw(Box::new(<$c_type>::new(obj)));
            $crate::c::register_ptr(ptr as usize);
            ptr
        }
    };
}

/// Macro to generate C destructor function with pointer registry support
/// 
/// # Security
/// 
/// This macro generates a destructor that:
/// 1. Checks if the pointer is null
/// 2. Validates the pointer is registered (prevents double-free)
/// 3. Unregisters the pointer before freeing (prevents use-after-free)
#[macro_export]
macro_rules! c_destructor {
    ($fn_name:ident, $c_type:ty) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(obj: *mut $c_type) {
            if obj.is_null() {
                return;
            }
            
            if !$crate::c::unregister_ptr(obj as usize) {
                log::warn!(
                    "[Ri.C] Attempted to free unregistered or already freed pointer: {:?}",
                    obj
                );
                return;
            }
            
            unsafe {
                let _ = Box::from_raw(obj);
            }
        }
    };
}

/// Macro to generate C getter for string with pointer validation
#[macro_export]
macro_rules! c_string_getter {
    ($fn_name:ident, $c_type:ty, $getter:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(obj: *mut $c_type) -> *mut std::ffi::c_char {
            if obj.is_null() || !$crate::c::is_valid_ptr(obj as usize) {
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

/// Macro to generate C setter for string with pointer validation
#[macro_export]
macro_rules! c_string_setter {
    ($fn_name:ident, $c_type:ty, $setter:expr) => {
        #[no_mangle]
        pub extern "C" fn $fn_name(
            obj: *mut $c_type,
            value: *const std::ffi::c_char,
        ) -> std::ffi::c_int {
            if obj.is_null() || !$crate::c::is_valid_ptr(obj as usize) || value.is_null() {
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
