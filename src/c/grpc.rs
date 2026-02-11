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

//! # gRPC Module C API

use crate::grpc::{DMSCGrpcConfig, DMSCGrpcServer, DMSCGrpcClient};
use std::ffi::{c_char, c_int};

c_wrapper!(CDMSCGrpcConfig, DMSCGrpcConfig);
c_wrapper!(CDMSCGrpcServer, DMSCGrpcServer);
c_wrapper!(CDMSCGrpcClient, DMSCGrpcClient);

// DMSCGrpcConfig constructors and destructors
c_constructor!(dmsc_grpc_config_new, CDMSCGrpcConfig, DMSCGrpcConfig, DMSCGrpcConfig::default());
c_destructor!(dmsc_grpc_config_free, CDMSCGrpcConfig);
