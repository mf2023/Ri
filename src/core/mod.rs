//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

#![allow(non_snake_case)]

// Core runtime, error model, and service context for DMS.

mod error;
mod context;
mod module;
mod runtime;
mod lifecycle;
mod analytics;

pub use error::{DMSError, DMSResult};
pub use context::DMSServiceContext;
pub use module::{_CServiceModule, _CAsyncServiceModule, DMSModule};
pub use runtime::{DMSAppBuilder, DMSAppRuntime};

// Re-export additional error variants
pub use error::DMSError::{DeviceNotFound, DeviceAllocationFailed, AllocationNotFound};
