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

//! # Service Mesh Module C API

use crate::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig, DMSCServiceEndpoint};
use std::ffi::{c_char, c_int};

c_wrapper!(CDMSCServiceMesh, DMSCServiceMesh);
c_wrapper!(CDMSCServiceMeshConfig, DMSCServiceMeshConfig);
c_wrapper!(CDMSCServiceEndpoint, DMSCServiceEndpoint);

// DMSCServiceMeshConfig constructors and destructors
c_constructor!(dmsc_service_mesh_config_new, CDMSCServiceMeshConfig, DMSCServiceMeshConfig, DMSCServiceMeshConfig::default());
c_destructor!(dmsc_service_mesh_config_free, CDMSCServiceMeshConfig);
