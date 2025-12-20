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

use dms::core::error_chain::*;
use std::io;

#[test]
fn test_error_chain_creation() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::new(io_err);
    assert!(chain.get_context().is_empty());
    assert!(chain.previous().is_none());
}

#[test]
fn test_error_chain_with_context() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::with_context(io_err, "Failed to load configuration");
    assert_eq!(chain.get_context(), "Failed to load configuration");
}

#[test]
fn test_error_chain_iteration() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::with_context(io_err, "Level 1")
        .context("Level 2")
        .context("Level 3");

    let contexts: Vec<String> = chain.chain().map(|e| e.get_context().to_string()).collect();
    assert_eq!(contexts, vec!["Level 3", "Level 2", "Level 1"]);
}

#[test]
fn test_result_error_context() {
    let result: Result<i32, io::Error> = Err(io::Error::new(io::ErrorKind::Other, "test error"));
    let chained = result.chain_context("Operation failed");
    assert!(chained.is_err());
    
    let err = chained.unwrap_err();
    assert_eq!(err.get_context(), "Operation failed");
}

#[test]
fn test_error_chain_contains() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let chain = ErrorChain::new(io_err).context("Failed to load config");
    
    assert!(chain.contains::<io::Error>());
    assert!(!chain.contains::<std::fmt::Error>());
}