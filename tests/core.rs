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

use dms::core::{DMSAppBuilder, DMSServiceContext, DMSError, DMSResult};

#[test]
fn test_dms_error() {
    let error = DMSError::new("Test error message");
    assert_eq!(error.message, "Test error message");
}

#[test]
fn test_service_context_new() {
    let ctx = DMSServiceContext::new_default().unwrap();
    assert!(ctx.fs().project_root().exists());
}

#[test]
fn test_app_builder_new() {
    let builder = DMSAppBuilder::new();
    let runtime = builder.build().unwrap();
    assert!(runtime.run().await.is_ok());
}
