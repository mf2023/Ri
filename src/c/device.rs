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

use crate::device::{DMSCDevice, DMSCDeviceController, DMSCDeviceScheduler, DMSCDeviceType};
use std::ffi::c_char;

c_wrapper!(CDMSCDevice, DMSCDevice);
c_wrapper!(CDMSCDeviceController, DMSCDeviceController);
c_wrapper!(CDMSCDeviceScheduler, DMSCDeviceScheduler);

// DMSCDevice constructors and destructors
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
c_destructor!(dmsc_device_free, CDMSCDevice);
