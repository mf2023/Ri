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

use DMS::prelude::*;
use DMS::log::DMSLogContext;

fn main() -> DMSResult<()> {
    // Build service context via core runtime path
    let ctx = DMSServiceContext::_Fnew_default()?;

    // FS: write & read a small JSON file under cache
    let fs = ctx._Ffs();
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct SmokeData { message: String, value: i32 }

    let data = SmokeData { message: "hello_dms".to_string(), value: 42 };
    let path = fs._Fcache_dir().join("smoke.json");
    fs._Fwrite_json(&path, &data)?;
    let read_back: SmokeData = fs._Fread_json(&path)?;

    // Log: write a few log lines
    let logger = ctx._Flogger();
    DMSLogContext::_Fput("component", "smoke_test");
    DMSLogContext::_Fput("file", path.to_string_lossy().to_string());
    logger._Finfo("DMS.Smoke", format!("read_back={:?}", read_back))?;

    // Hooks: emit startup/shutdown manually via runtime
    // (runtime already does this, but here we just ensure API compiles)
    let _ = ctx._Fhooks();

    Ok(())
}
