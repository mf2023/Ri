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

//! # Device Module C API
//!
//! This module provides C language bindings for DMSC's device management subsystem. The device
//! module delivers comprehensive device abstraction and control capabilities for managing various
//! types of computational resources including CPU, GPU, memory, storage, network interfaces,
//! sensors, and actuators. This C API enables C/C++ applications to leverage DMSC's device
//! orchestration features for resource management, scheduling, and hardware abstraction.
//!
//! ## Module Architecture
//!
//! The device management module comprises four primary components that together provide complete
//! device lifecycle management:
//!
//! - **DMSCDevice**: Fundamental device abstraction representing any computational resource.
//!   Each device instance encapsulates identity, type, capabilities, and state information.
//!   Devices can be queried for properties, monitored for status, and controlled through
//!   standardized interfaces regardless of underlying hardware implementation.
//!
//! - **DMSCDeviceController**: Device control interface providing operational methods for
//!   device manipulation. The controller handles device initialization, configuration,
//!   activation, deactivation, and error recovery. Controllers implement device-specific
//!   logic while presenting a uniform control interface to the rest of the system.
//!
//! - **DMSCDeviceScheduler**: Resource scheduling component for coordinating device usage
//!   across multiple requestors. The scheduler implements allocation policies, fair queuing,
//!   and priority-based scheduling to optimize device utilization while preventing resource
//!   contention. Supports both synchronous and asynchronous scheduling modes.
//!
//! - **DMSCDeviceType**: Enumeration defining supported device categories. Each device type
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
!
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
//! DMSCDevice* cpu = dmsc_device_new("worker-node-1", DEVICE_TYPE_CPU);
//!
//! // Create device controller
//! DMSCDeviceController* controller = dmsc_device_controller_new(cpu);
//!
//! // Configure device
//! dmsc_device_controller_configure(controller, "max_frequency", "3000000000");
//!
//! // Initialize device for use
//! int result = dmsc_device_controller_initialize(controller);
//!
//! if (result == 0) {
//!     // Device ready, create scheduler
//!     DMSCDeviceScheduler* scheduler = dmsc_device_scheduler_new();
//!
//!     // Register device with scheduler
//!     dmsc_device_scheduler_register(scheduler, cpu);
//!
//!     // Allocate device for task
//!     DMSCDevice* allocated = dmsc_device_scheduler_allocate(scheduler,
//!         DEVICE_TYPE_CPU, PRIORITY_NORMAL);
//!
//!     // Use device...
//!
//!     // Release when done
//!     dmsc_device_scheduler_release(scheduler, allocated);
//!     dmsc_device_scheduler_free(scheduler);
//! }
//!
//! // Cleanup
//! dmsc_device_controller_free(controller);
//! dmsc_device_free(cpu);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::device`: Rust device module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The device module is always enabled as it provides fundamental infrastructure
//! for resource management in DMSC applications.

use crate::device::{DMSCDevice, DMSCDeviceController, DMSCDeviceScheduler, DMSCDeviceType};
use std::ffi::c_char;

/// Opaque C wrapper structure for DMSCDevice.
///
/// Fundamental device abstraction representing any computational resource. Each device
/// instance encapsulates identity, type, capabilities, and state information.
///
/// # Device Properties
///
/// Each device maintains the following core properties:
///
/// - **Name**: Unique identifier for the device within the system namespace.
///   Names should be descriptive and follow naming conventions for the device type.
///
/// - **Type**: Category of device as defined in DMSCDeviceType enumeration.
///   Determines available operations and expected behavior patterns.
///
/// - **State**: Current operational state including availability, allocation status,
///   and error conditions.
///
/// - **Capabilities**: Set of supported operations and features advertised by the device.
///
/// - **Metrics**: Dynamic performance and operational measurements updated periodically.
///
/// # Device Identification
///
/// Devices are identified through multiple mechanisms:
///
/// - **Name**: Human-readable identifier used for application-level reference
/// - **UUID**: Globally unique identifier suitable for persistent references
/// - **Physical Address**: Hardware-specific address or path for physical devices
/// - **Logical ID**: System-assigned identifier for internal tracking
///
/// # Lifetime Management
///
/// Device instances have the following lifecycle:
///
/// 1. Created via dmsc_device_new() or discovered through device enumeration
/// 2. Configured with required settings before first use
/// 3. Initialized to prepare for operational use
/// 4. Allocated to consumers for exclusive or shared access
/// 5. Used for device-specific operations
/// 6. Released back to available pool
/// 7. Cleaned up via dmsc_device_free() when no longer needed
///
/// # Thread Safety
///
/// Device objects are thread-safe for read operations:
///
/// - Property queries can be performed concurrently
/// - Metric sampling supports concurrent access
/// - Control operations require external synchronization
/// - Consider using device controller for coordinated access
c_wrapper!(CDMSCDevice, DMSCDevice);

/// Opaque C wrapper structure for DMSCDeviceController.
///
/// Device control interface providing operational methods for device manipulation.
/// Controllers handle device initialization, configuration, and lifecycle management.
///
/// # Controller Responsibilities
///
/// The device controller manages:
///
/// - **Initialization**: Prepare device for operational use including resource
///   allocation, driver loading, and health verification.
/// - **Configuration**: Apply runtime settings including operating modes, power
///   management, and feature enablement.
/// - **Activation/Deactivation**: Transition device between operational and standby
///   states for power management and resource optimization.
/// - **Error Recovery**: Detect, report, and optionally recover from error
///   conditions including timeout, hardware failure, and resource exhaustion.
/// - **Monitoring**: Track device health and operational metrics for observability
///   and alerting.
///
/// # Control Operations
///
/// Controllers provide standardized operations across device types:
///
/// - initialize(): Prepare device for first use
/// - configure(key, value): Apply configuration setting
/// - activate(): Transition to operational state
/// - deactivate(): Transition to standby state
/// - reset(): Return to known-good state
/// - shutdown(): Gracefully power down
///
/// # Error Handling
///
/// Controller operations return error codes indicating outcome:
///
/// - 0: Success
/// - Negative: System error (specific codes by device type)
/// - Positive: Warning condition (operation succeeded with notes)
///
/// # Thread Safety
///
/// Controllers are not thread-safe:
///
/// - Single controller instance should not be used concurrently
•   - Use separate controllers for multi-threaded access
/// - Consider device scheduler for coordinated sharing
c_wrapper!(CDMSCDeviceController, DMSCDeviceController);

/// Opaque C wrapper structure for DMSCDeviceScheduler.
///
/// Resource scheduling component for coordinating device usage across multiple requestors.
/// The scheduler implements allocation policies and prioritization.
///
/// # Scheduling Responsibilities
///
/// The device scheduler handles:
///
/// - **Request Queuing**: Accept and queue device allocation requests with
///   associated priority and requirements.
/// - **Resource Matching**: Match requests to suitable devices based on type,
///   capabilities, and availability.
/// - **Policy Enforcement**: Apply configured scheduling policies including
///   fairness, priority, and deadline constraints.
/// - **Allocation Management**: Track device allocation state and ensure proper
///   release when requests complete or are cancelled.
/// - **Metrics Collection**: Record scheduling statistics including wait times,
///   utilization, and queue depths.
///
/// # Allocation Flow
///
/// The typical allocation flow proceeds as follows:
///
/// 1. Submit allocation request with device type, priority, and requirements
/// 2. Request enters queue based on priority and scheduling policy
/// 3. Scheduler matches request to available device when possible
/// 4. Device transitions to ALLOCATED state
/// 5. Device reference returned to requestor
/// 6. Requestor uses device for desired operations
/// 7. Requestor releases device when complete
/// 8. Device returns to AVAILABLE state
///
/// # Policy Configuration
///
/// The scheduler supports multiple configuration options:
///
/// - Default scheduling policy (FIFO, priority, fair-share, deadline)
/// - Queue limits and overflow handling
/// - Timeout behavior for waiting requests
/// - Preemption settings for priority scheduling
/// - Fair-share weights per consumer
///
/// # Thread Safety
///
/// The scheduler is fully thread-safe:
///
/// - Concurrent request submission supported
/// - Multiple threads can wait for allocation
/// - Internal synchronization handles race conditions
/// - Safe for use in high-concurrency scenarios
c_wrapper!(CDMSCDeviceScheduler, DMSCDeviceScheduler);

/// Device type enumeration values.
///
/// These integer constants identify the category of device being created or managed.
/// The values map to the DMSCDeviceType Rust enumeration.
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
/// DMSCDevice* cpu = dmsc_device_new("compute-0", 0);  // CPU device
/// DMSCDevice* gpu = dmsc_device_new("render-0", 1);  // GPU device
/// ```
///
/// # Extensibility
///
/// Applications can define custom device types beyond the standard categories
/// by using values greater than or equal to 7. Custom types should be
/// documented and handled appropriately by application code.

/// Creates a new DMSCDevice instance with specified name and device type.
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
/// Pointer to newly allocated DMSCDevice on success, or NULL if:
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
/// DMSCDevice* gpu = dmsc_device_new("training-gpu-0", DEVICE_TYPE_GPU);
/// if (gpu == NULL) {
///     fprintf(stderr, "Failed to create device\n");
///     return ERROR_DEVICE_CREATION;
/// }
///
/// // Configure and initialize...
///
/// // Cleanup when done
/// dmsc_device_free(gpu);
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
pub extern "C" fn dmsc_device_new(name: *const c_char, device_type: i32) -> *mut CDMSCDevice {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        let dtype = match device_type {
            0 => DMSCDeviceType::CPU,
            1 => DMSCDeviceType::GPU,
            2 => DMSCDeviceType::Memory,
            3 => DMSCDeviceType::Storage,
            4 => DMSCDeviceType::Network,
            5 => DMSCDeviceType::Sensor,
            6 => DMSCDeviceType::Actuator,
            _ => DMSCDeviceType::Custom,
        };
        let device = DMSCDevice::new(name_str.to_string(), dtype);
        Box::into_raw(Box::new(CDMSCDevice::new(device)))
    }
}

/// Frees a previously allocated DMSCDevice instance.
///
/// Releases all memory associated with the device including any allocated resources,
/// cached metrics, or internal state. The device must not be in ALLOCATED state
/// when freed.
///
/// # Parameters
///
/// - `device`: Pointer to DMSCDevice to free. If NULL, the function returns
///   immediately without error.
///
/// # Preconditions
///
/// Before freeing a device:
///
/// 1. Ensure device is not currently allocated to a consumer
/// 2. Release from any scheduler if registered
/// 3. Destroy associated controller if created
/// 4. Complete any pending operations
///
/// # Safety
///
/// This function is safe to call with NULL. Calling with a pointer that has
/// already been freed results in undefined behavior. Ensure proper synchronization
/// when freeing devices accessed from multiple threads.
c_destructor!(dmsc_device_free, CDMSCDevice);
