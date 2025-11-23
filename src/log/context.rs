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

// Logging context for DMS, similar to MDC.

use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static _CLOG_CONTEXT: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
}

pub struct DMSLogContext;

impl DMSLogContext {
    pub fn _Fput(key: impl Into<String>, value: impl Into<String>) {
        let k = key.into();
        let v = value.into();
        _CLOG_CONTEXT.with(|ctx| {
            ctx.borrow_mut().insert(k, v);
        });
    }

    pub fn _Fget_all() -> HashMap<String, String> {
        _CLOG_CONTEXT.with(|ctx| ctx.borrow().clone())
    }

    pub fn _Fclear() {
        _CLOG_CONTEXT.with(|ctx| ctx.borrow_mut().clear());
    }
}
