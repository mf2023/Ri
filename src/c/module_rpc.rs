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

//! # Module RPC C API

use crate::module_rpc::{DMSCModuleRPC, DMSCModuleClient, DMSCModuleEndpoint};
use std::ffi::{c_char, c_int};

c_wrapper!(CDMSCModuleRPC, DMSCModuleRPC);
c_wrapper!(CDMSCModuleClient, DMSCModuleClient);
c_wrapper!(CDMSCModuleEndpoint, DMSCModuleEndpoint);

// DMSCModuleRPC constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_module_rpc_new() -> *mut CDMSCModuleRPC {
    let rpc = DMSCModuleRPC::new();
    Box::into_raw(Box::new(CDMSCModuleRPC::new(rpc)))
}
c_destructor!(dmsc_module_rpc_free, CDMSCModuleRPC);
