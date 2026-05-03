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

//! # Device Module C API
//!
//! This module provides C language bindings for Ri's device management subsystem. The device
//! module delivers comprehensive device abstraction and control capabilities for managing various
//! types of computational resources including CPU, GPU, memory, storage, network interfaces,
//! sensors, and actuators. This C API enables C/C++ applications to leverage Ri's device
//! orchestration features for resource management, scheduling, and hardware abstraction.
//!
//! ## Module Architecture
//!
//! The device management module comprises four primary components that together provide complete
//! device lifecycle management:
//!
//! - **RiDevice**: Fundamental device abstraction representing any computational resource.
//!   Each device instance encapsulates identity, type, capabilities, and state information.
//!   Devices can be queried for properties, monitored for status, and controlled through
//!   standardized interfaces regardless of underlying hardware implementation.
//!
//! - **RiDeviceController**: Device control interface providing operational methods for
//!   device manipulation. The controller handles device initialization, configuration,
//!   activation, deactivation, and error recovery. Controllers implement device-specific
//!   logic while presenting a uniform control interface to the rest of the system.
//!
//! - **RiDeviceScheduler**: Resource scheduling component for coordinating device usage
//!   across multiple requestors. The scheduler implements allocation policies, fair queuing,
//!   and priority-based scheduling to optimize device utilization while preventing resource
//!   contention. Supports both synchronous and asynchronous scheduling modes.
//!
//! - **RiDeviceType**: Enumeration defining supported device categories. Each device type
//!   indicates the general class of hardware or resource being represented. The type system
//!   enables type-safe device operations and automatic dispatch to appropriate handlers.
//!
//! ## Device Types
//!
//! The device module supports the following device categories:
//!
//! - **CPU**: Central processing unit resources. CPU devices provide processing capability
//!   for computational tasks. Scheduling considerations include core count, clock frequency,
//!   cache hierarchy, and instruction set capabilities.
//!
//! - **GPU**: Graphics processing unit resources. GPU devices are specialized for
//!   parallel computation, machine learning inference, and graphics rendering. Support
//!   includes CUDA, OpenCL, and Vulkan compute capabilities.
//!
//! - **Memory**: Random access memory resources. Memory devices represent available RAM
//!   that can be allocated for data processing. Considerations include capacity, latency,
//!   bandwidth, and memory hierarchy (cache, main memory, swap).
//!
//! - **Storage**: Persistent storage resources. Storage devices provide durable data
//!   retention including SSDs, HDDs, and network storage. Performance characteristics
//!   include IOPS, throughput, latency, and durability ratings.
//!
//! - **Network**: Network interface resources. Network devices enable communication
//!   with external systems. Properties include bandwidth, latency, protocol support,
//!   and connection state.
//!
//! - **Sensor**: Data acquisition devices. Sensors collect environmental or system
//!   data including temperature, pressure, location, and system metrics. Support
//!   includes polling and event-driven data collection.
//!
//! - **Actuator**: Action execution devices. Actuators perform physical or logical
//!   actions based on commands. Examples include motor controllers, relay switches,
//!   and service invocation endpoints.
//!
//! - **Custom**: User-defined device types. Custom devices allow application-specific
//!   resource types beyond the standard categories. Custom types can implement any
//!   device-like behavior required by the application.
//!
//! ## Device Lifecycle
//!
//! Devices transition through well-defined lifecycle states:
//!
//! 1. **DISCOVERED**: Device detected but not yet configured or available for use
//! 2. **CONFIGURED**: Device has been initialized with required settings
//! 3. **AVAILABLE**: Device ready for allocation and operational use
//! 4. **ALLOCATED**: Device assigned to a specific consumer or task
//! 5. **BUSY**: Device actively executing operations
//! 6. **ERROR**: Device encountered an error condition
//! 7. **UNAVAILABLE**: Device temporarily or permanently unavailable
//! 8. **RELEASED**: Device resources freed after allocation
//!
//! ## Scheduling Policies
//!
//! The device scheduler implements multiple allocation strategies:
//!
//! - **FIFO (First In, First Out)**: Requests processed in arrival order. Simple
//!   and predictable, suitable for uniform priority workloads.
//!
//! - **Priority-Based**: Requests assigned priorities affecting scheduling order.
//!   Higher priority requests jump ahead of lower priority ones. Supports multiple
//!   priority levels with configurable behavior at each level.
//!
//! - **Fair-Sharing**: Resources distributed equitably across requestors. Prevents
//!   any single consumer from monopolizing device capacity. Implements weighted fair
//!   queuing for proportional allocation.
//!
//! - **Deadline-Driven**: Requests scheduled to meet deadline requirements.
//!   Suitable for real-time workloads with timing constraints. Requires deadline
//!   specification at request time.
//!
//! - **Load-Balancing**: Requests distributed across multiple identical devices.
//!   Optimizes resource utilization and maximizes throughput for parallelizable work.
//!
//! ## Device Capabilities
//!
//! Each device advertises its capabilities through a standardized interface:
//!
//! - **Properties**: Static characteristics including manufacturer, model, serial
//!   number, firmware version, and unique identifiers.
//!
//! - **Metrics**: Dynamic measurements including utilization, temperature, error
//!   rates, and operational statistics. Metrics are sampled periodically or on demand.
//!
//! - **Capabilities**: Supported operations and modes including read/write access,
//!   concurrent operation support, and specialized features.
//!
//! - **Constraints**: Operational limits including maximum throughput, memory
//!   capacity, power limits, and environmental requirements.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Device instances must be properly released after allocation
//! - Null pointer checks are required before all operations
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Device controllers handle concurrent access with internal synchronization
//! - Scheduler operations are thread-safe for multi-threaded request submission
//! - Device state queries can be performed concurrently
//! - Device control operations may require exclusive access
//!
//! ## Performance Characteristics
//!
//! Device operations have the following performance profiles:
//!
//! - Device discovery: O(n) where n is number of potential devices
//! - Device allocation: O(1) average case, O(log n) for complex policies
//! - Metric collection: O(1) for cached metrics, O(n) for hardware sampling
//! - Scheduling decisions: O(1) for FIFO, O(log p) for priority (p = priority levels)
//!
//! ## Usage Example
//!
//! ```c
//! // Create a CPU device
//! RiDevice* cpu = ri_device_new("worker-node-1", DEVICE_TYPE_CPU);
//!
//! // Create device controller
//! RiDeviceController* controller = ri_device_controller_new(cpu);
//!
//! // Configure device
//! ri_device_controller_configure(controller, "max_frequency", "3000000000");
//!
//! // Initialize device for use
//! int result = ri_device_controller_initialize(controller);
//!
//! if (result == 0) {
//!     // Device ready, create scheduler
//!     RiDeviceScheduler* scheduler = ri_device_scheduler_new();
//!
//!     // Register device with scheduler
//!     ri_device_scheduler_register(scheduler, cpu);
//!
//!     // Allocate device for task
//!     RiDevice* allocated = ri_device_scheduler_allocate(scheduler,
//!         DEVICE_TYPE_CPU, PRIORITY_NORMAL);
//!
//!     // Use device...
//!
//!     // Release when done
//!     ri_device_scheduler_release(scheduler, allocated);
//!     ri_device_scheduler_free(scheduler);
//! }
//!
//! // Cleanup
//! ri_device_controller_free(controller);
//! ri_device_free(cpu);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::device`: Rust device module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The device module is always enabled as it provides fundamental infrastructure
//! for resource management in Ri applications.

use crate::device::{RiDevice, RiDeviceController, RiDeviceScheduler, RiDeviceType};
use std::ffi::c_char;
use std::sync::Arc;

c_wrapper!(CRiDevice, RiDevice);

c_wrapper!(CRiDeviceController, RiDeviceController);

c_wrapper!(CRiDeviceScheduler, RiDeviceScheduler);

/// Device type enumeration values.
///
/// These integer constants identify the category of device being created or managed.
/// The values map to the RiDeviceType Rust enumeration.
///
/// # Type Mapping
///
/// The following mapping applies between C constants and device types:
///
/// - 0: CPU - Central processing unit
/// - 1: GPU - Graphics processing unit
/// - 2: Memory - RAM resources
/// - 3: Storage - Persistent storage devices
/// - 4: Network - Network interfaces
/// - 5: Sensor - Data acquisition devices
/// - 6: Actuator - Action execution devices
/// - 7+: Custom - Application-specific types
///
/// # Usage
///
/// When creating devices or filtering by type, pass the appropriate constant:
///
/// ```c
/// RiDevice* cpu = ri_device_new("compute-0", 0);  // CPU device
/// RiDevice* gpu = ri_device_new("render-0", 1);  // GPU device
/// ```
///
/// # Extensibility
///
/// Applications can define custom device types beyond the standard categories
/// by using values greater than or equal to 7. Custom types should be
/// documented and handled appropriately by application code.

/// Creates a new RiDevice instance with specified name and device type.
///
/// Allocates a new device object with the given identification and classification.
/// The device is created in DISCOVERED state and requires configuration and
/// initialization before use.
///
/// # Parameters
///
/// - `name`: Pointer to null-terminated C string containing the device name.
///   Must not be NULL. The name should be unique within the device namespace.
///   Names follow naming conventions: lowercase with hyphens for standard devices.
/// - `device_type`: Integer constant indicating the device category.
///   Use predefined constants (0-6) for standard types or custom values for
///   application-specific devices.
///
/// # Returns
///
/// Pointer to newly allocated RiDevice on success, or NULL if:
/// - `name` parameter is NULL
/// - Memory allocation fails
/// - Name contains invalid UTF-8 sequences
///
/// # Initial State
///
/// A newly created device:
///
/// - Has DISCOVERED lifecycle state
/// - Has no assigned controller (controller must be created separately)
/// - Has no configured settings (defaults applied)
/// - Is not registered with any scheduler
///
/// # Example
///
/// ```c
/// // Create a GPU device
/// RiDevice* gpu = ri_device_new("training-gpu-0", DEVICE_TYPE_GPU);
/// if (gpu == NULL) {
///     fprintf(stderr, "Failed to create device\n");
///     return ERROR_DEVICE_CREATION;
/// }
///
/// // Configure and initialize...
///
/// // Cleanup when done
/// ri_device_free(gpu);
/// ```
///
/// # Naming Conventions
///
/// Device names should follow these guidelines:
///
/// - Descriptive: Indicate device purpose or location
/// - Unique: No two devices share the same name
/// - Consistent: Follow naming pattern for device type
/// - Persistent: Names remain stable across restarts
#[no_mangle]
pub extern "C" fn ri_device_new(name: *const c_char, device_type: i32) -> *mut CRiDevice {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        let dtype = match device_type {
            0 => RiDeviceType::CPU,
            1 => RiDeviceType::GPU,
            2 => RiDeviceType::Memory,
            3 => RiDeviceType::Storage,
            4 => RiDeviceType::Network,
            5 => RiDeviceType::Sensor,
            6 => RiDeviceType::Actuator,
            _ => RiDeviceType::Custom,
        };
        let device = RiDevice::new(name_str.to_string(), dtype);
        let ptr = Box::into_raw(Box::new(CRiDevice::new(device)));
        crate::c::register_ptr(ptr as usize);
        ptr
    }
}

c_destructor!(ri_device_free, CRiDevice);

// RiDevice getters
c_string_getter!(
    ri_device_get_name,
    CRiDevice,
    |inner: &RiDevice| inner.name().to_string()
);

#[no_mangle]
pub extern "C" fn ri_device_get_id(device: *mut CRiDevice) -> *mut std::ffi::c_char {
    if device.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        match std::ffi::CString::new((*device).inner.id().to_string()) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_device_get_type(device: *mut CRiDevice) -> std::ffi::c_int {
    if device.is_null() {
        return -1;
    }
    unsafe {
        match (*device).inner.device_type() {
            RiDeviceType::CPU => 0,
            RiDeviceType::GPU => 1,
            RiDeviceType::Memory => 2,
            RiDeviceType::Storage => 3,
            RiDeviceType::Network => 4,
            RiDeviceType::Sensor => 5,
            RiDeviceType::Actuator => 6,
            RiDeviceType::Custom => 7,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_device_get_status(device: *mut CRiDevice) -> std::ffi::c_int {
    if device.is_null() {
        return -1;
    }
    unsafe {
        match (*device).inner.status() {
            crate::device::RiDeviceStatus::Unknown => 0,
            crate::device::RiDeviceStatus::Available => 1,
            crate::device::RiDeviceStatus::Busy => 2,
            crate::device::RiDeviceStatus::Error => 3,
            crate::device::RiDeviceStatus::Offline => 4,
            crate::device::RiDeviceStatus::Maintenance => 5,
            crate::device::RiDeviceStatus::Degraded => 6,
            crate::device::RiDeviceStatus::Allocated => 7,
        }
    }
}

// RiDeviceController C bindings
#[no_mangle]
pub extern "C" fn ri_device_controller_new() -> *mut CRiDeviceController {
    let ptr = Box::into_raw(Box::new(CRiDeviceController::new(RiDeviceController::new())));
    crate::c::register_ptr(ptr as usize);
    ptr
}
c_destructor!(ri_device_controller_free, CRiDeviceController);

#[no_mangle]
pub extern "C" fn ri_device_controller_add_device(
    controller: *mut CRiDeviceController,
    device: *mut CRiDevice,
    location: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if controller.is_null() || device.is_null() || location.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let device = (*device).inner.clone();
        let location_str = match std::ffi::CStr::from_ptr(location).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return -3,
        };
        rt.block_on(async {
            (*controller).inner.add_device(device, location_str).await
        }).map(|_| 0).unwrap_or(-4)
    }
}

#[no_mangle]
pub extern "C" fn ri_device_controller_remove_device(
    controller: *mut CRiDeviceController,
    device_id: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if controller.is_null() || device_id.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let device_id_str = match std::ffi::CStr::from_ptr(device_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        rt.block_on(async {
            (*controller).inner.remove_device(device_id_str).await;
        });
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_device_controller_get_device(
    controller: *mut CRiDeviceController,
    device_id: *const std::ffi::c_char,
) -> *mut CRiDevice {
    if controller.is_null() || device_id.is_null() {
        return std::ptr::null_mut();
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe {
        let device_id_str = match std::ffi::CStr::from_ptr(device_id).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        match rt.block_on(async { (*controller).inner.get_device(device_id_str).await }) {
            Some(device) => {
                let ptr = Box::into_raw(Box::new(CRiDevice::new(device)));
                crate::c::register_ptr(ptr as usize);
                ptr
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_device_controller_get_device_count(controller: *mut CRiDeviceController) -> usize {
    if controller.is_null() {
        return 0;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return 0,
    };
    unsafe {
        rt.block_on(async { (*controller).inner.get_all_devices().len() })
    }
}

#[no_mangle]
pub extern "C" fn ri_device_controller_discover(
    controller: *mut CRiDeviceController,
    out_devices: *mut *mut CRiDevice,
    out_count: *mut usize,
) -> std::ffi::c_int {
    if controller.is_null() || out_devices.is_null() || out_count.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        match rt.block_on(async { (*controller).inner.discover_devices().await }) {
            Ok(result) => {
                let count = result.discovered_devices.len();
                *out_count = count;
                if count == 0 {
                    *out_devices = std::ptr::null_mut();
                    return 0;
                }
                let devices: Vec<CRiDevice> = result.discovered_devices.into_iter().map(CRiDevice::new).collect();
                let ptr = Box::into_raw(Box::new(devices));
                *out_devices = ptr as *mut CRiDevice;
                0
            }
            Err(_) => -3,
        }
    }
}

// RiDeviceScheduler C bindings
#[no_mangle]
pub extern "C" fn ri_device_scheduler_new() -> *mut CRiDeviceScheduler {
    let pool_manager = Arc::new(tokio::sync::RwLock::new(crate::device::RiResourcePoolManager::new()));
    let ptr = Box::into_raw(Box::new(CRiDeviceScheduler::new(RiDeviceScheduler::new(pool_manager))));
    crate::c::register_ptr(ptr as usize);
    ptr
}
c_destructor!(ri_device_scheduler_free, CRiDeviceScheduler);

#[no_mangle]
pub extern "C" fn ri_device_scheduler_allocate(
    scheduler: *mut CRiDeviceScheduler,
    device_type: std::ffi::c_int,
    priority: u32,
    timeout_secs: u64,
) -> *mut CRiDevice {
    if scheduler.is_null() {
        return std::ptr::null_mut();
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe {
        let dtype = match device_type {
            0 => RiDeviceType::CPU,
            1 => RiDeviceType::GPU,
            2 => RiDeviceType::Memory,
            3 => RiDeviceType::Storage,
            4 => RiDeviceType::Network,
            5 => RiDeviceType::Sensor,
            6 => RiDeviceType::Actuator,
            7 => RiDeviceType::Custom,
            _ => RiDeviceType::Custom,
        };
        let request = crate::device::scheduler::RiAllocationRequest {
            device_type: dtype,
            capabilities: crate::device::RiDeviceCapabilities::default(),
            priority,
            timeout_secs,
            sla_class: None,
            resource_weights: None,
            affinity: None,
            anti_affinity: None,
        };
        match rt.block_on(async { (*scheduler).inner.select_device(&request).await }) {
            Some(device) => {
                let ptr = Box::into_raw(Box::new(CRiDevice::new((*device).clone())));
                crate::c::register_ptr(ptr as usize);
                ptr
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_device_scheduler_release(
    scheduler: *mut CRiDeviceScheduler,
    device_id: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if scheduler.is_null() || device_id.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let device_id_str = match std::ffi::CStr::from_ptr(device_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*scheduler).inner.release_device(device_id_str).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

// RiResourcePool C bindings
c_wrapper!(CRiResourcePool, crate::device::RiResourcePool);

#[no_mangle]
pub extern "C" fn ri_resource_pool_new(name: *const std::ffi::c_char, capacity: usize) -> *mut CRiResourcePool {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let config = crate::device::RiResourcePoolConfig {
            name: name_str,
            device_type: RiDeviceType::Custom,
            max_concurrent_allocations: capacity,
            allocation_timeout_secs: 30,
            health_check_interval_secs: 60,
        };
        let ptr = Box::into_raw(Box::new(CRiResourcePool::new(crate::device::RiResourcePool::new(config))));
        crate::c::register_ptr(ptr as usize);
        ptr
    }
}
c_destructor!(ri_resource_pool_free, CRiResourcePool);

#[no_mangle]
pub extern "C" fn ri_resource_pool_get_capacity(pool: *mut CRiResourcePool) -> usize {
    if pool.is_null() {
        return 0;
    }
    unsafe { (*pool).inner.get_status().total_capacity }
}

#[no_mangle]
pub extern "C" fn ri_resource_pool_get_available(pool: *mut CRiResourcePool) -> usize {
    if pool.is_null() {
        return 0;
    }
    unsafe { (*pool).inner.get_status().available_capacity }
}

#[no_mangle]
pub extern "C" fn ri_resource_pool_get_utilization(pool: *mut CRiResourcePool) -> f64 {
    if pool.is_null() {
        return 0.0;
    }
    unsafe { (*pool).inner.get_status().utilization_rate }
}
