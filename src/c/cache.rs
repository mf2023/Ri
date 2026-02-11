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

//! # Cache Module C API

use crate::cache::{DMSCCacheConfig, DMSCCacheManager, DMSCMemoryCache};
use std::ffi::{c_char, c_int};

c_wrapper!(CDMSCCacheConfig, DMSCCacheConfig);
c_wrapper!(CDMSCCacheManager, DMSCCacheManager);
c_wrapper!(CDMSCMemoryCache, DMSCMemoryCache);

// DMSCCacheConfig constructors and destructors
c_constructor!(dmsc_cache_config_new, CDMSCCacheConfig, DMSCCacheConfig, DMSCCacheConfig::default());
c_destructor!(dmsc_cache_config_free, CDMSCCacheConfig);

// DMSCMemoryCache constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_memory_cache_new() -> *mut CDMSCMemoryCache {
    let cache = DMSCMemoryCache::new();
    Box::into_raw(Box::new(CDMSCMemoryCache::new(cache)))
}
c_destructor!(dmsc_memory_cache_free, CDMSCMemoryCache);
