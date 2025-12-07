// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
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

extern crate dms;

use dms::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities};
use dms::device::discovery_scheduler::{DMSDeviceDiscoveryEngine, DMSResourceScheduler, DeviceScanResult, ResourceRequest};

#[test]
fn test_device_discovery_engine() {
    let mut engine = DMSDeviceDiscoveryEngine::new();
    
    let scan_results = vec![
        DeviceScanResult {
            device_id: "gpu_1".to_string(),
            device_name: "NVIDIA GPU".to_string(),
            device_info: [
                ("device_name".to_string(), "NVIDIA GeForce RTX 3080".to_string()),
                ("driver".to_string(), "CUDA 11.4".to_string()),
            ].iter().cloned().collect(),
        },
    ];
    
    let devices = engine.discover_devices(scan_results);
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_type(), DMSDeviceType::GPU);
}

#[test]
fn test_resource_scheduler() {
    let mut scheduler = DMSResourceScheduler::new();
    
    let request = ResourceRequest {
        request_id: "req_1".to_string(),
        required_memory_gb: Some(8.0),
        required_compute_units: Some(256),
        required_bandwidth_gbps: Some(100.0),
        required_custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
        priority: 5,
        deadline: None,
    };
    
    let devices = vec![
        DMSDevice::new(
            "GPU 1".to_string(),
            DMSDeviceType::GPU,
        ).with_capabilities(DMSDeviceCapabilities {
            memory_gb: Some(16.0),
            compute_units: Some(512),
            storage_gb: Some(1000.0),
            bandwidth_gbps: Some(900.0),
            custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
        }),
    ];
    
    let selected_device = scheduler.schedule_resource(&request, &devices);
    assert_eq!(selected_device, Some(devices[0].id().to_string()));
}
