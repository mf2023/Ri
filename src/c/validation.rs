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

//! # Validation Module C API

use crate::validation::{DMSCValidationResult, DMSCValidatorBuilder, DMSCSanitizer};
use std::ffi::{c_char, c_int};

c_wrapper!(CDMSCValidationResult, DMSCValidationResult);
c_wrapper!(CDMSCValidatorBuilder, DMSCValidatorBuilder);
c_wrapper!(CDMSCSanitizer, DMSCSanitizer);

// DMSCValidationResult constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_validation_result_valid() -> *mut CDMSCValidationResult {
    let result = DMSCValidationResult::valid();
    Box::into_raw(Box::new(CDMSCValidationResult::new(result)))
}
c_destructor!(dmsc_validation_result_free, CDMSCValidationResult);
