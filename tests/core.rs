// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

use dmsc::core::{DMSCError, DMSCServiceContext, DMSCAppBuilder};

#[test]
fn test_dms_error() {
    let error = DMSCError::Other("Test error message".to_string());
    assert!(matches!(error, DMSCError::Other(msg) if msg == "Test error message"));
}

#[test]
fn test_service_context_new() {
    let ctx = DMSCServiceContext::new_default().unwrap();
    assert!(ctx.fs().project_root().exists());
}

#[tokio::test]
async fn test_app_builder_new() {
    let builder = DMSCAppBuilder::new();
    let runtime = builder.build().unwrap();
    let result = runtime.run(|_ctx| async move { Ok(()) }).await;
    assert!(result.is_ok());
}
